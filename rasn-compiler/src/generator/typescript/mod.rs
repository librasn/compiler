use self::utils::to_jer_identifier;
use crate::{error::CompilerError, intermediate::*};

use super::{
    error::{GeneratorError, GeneratorErrorType},
    Backend, GeneratedModule,
};

mod builder;
mod template;
mod utils;

#[derive(Debug, Default)]
/// A compiler backend that generates typescript type definitions for
/// ASN.1 data elements encoded using ITU X.697 JSON Encoding Rules
/// with default encoding instructions
pub struct Typescript {
    config: Config,
}

#[derive(Debug, Default)]
/// A configuration for the [Rasn] backend
pub struct Config {}

impl Backend for Typescript {
    type Config = Config;

    const FILE_EXTENSION: &'static str = ".ts";

    fn from_config(config: Self::Config) -> Self {
        Self { config }
    }

    fn config(&self) -> &Self::Config {
        &self.config
    }

    fn new(config: Self::Config, _: TaggingEnvironment, _: ExtensibilityEnvironment) -> Self {
        Self::from_config(config)
    }

    fn generate_module(
        &mut self,
        tlds: Vec<ToplevelDefinition>,
    ) -> Result<GeneratedModule, GeneratorError> {
        if let Some((module_ref, _)) = tlds.first().and_then(|tld| tld.get_index().cloned()) {
            let module = module_ref.borrow();
            let namespace = to_jer_identifier(&module.name);
            let imports = module
                .imports
                .iter()
                .fold(String::new(), |mut acc, import| {
                    let import_namespace =
                        to_jer_identifier(&import.global_module_reference.module_reference);
                    let mut usages = vec![];
                    for usage in &import.types {
                        if !usage.contains("{}")
                            && !usage.chars().all(|c| c.is_uppercase() || c == '-')
                        {
                            let jer_identifier = to_jer_identifier(usage);
                            usages.push(format!(
                                "import {jer_identifier} = {import_namespace}.{jer_identifier};",
                            ))
                        }
                    }
                    acc.push('\n');
                    acc.push_str(&usages.join("\n"));
                    acc
                });
            let (pdus, warnings): (String, Vec<Box<dyn CompilerError>>) =
                tlds.into_iter()
                    .fold((String::new(), vec![]), |mut acc, tld| {
                        match self.generate(tld) {
                            Ok(s) => {
                                acc.0.push('\n');
                                acc.0.push_str(&s);
                                acc
                            }
                            Err(e) => {
                                acc.1.push(Box::new(e));
                                acc
                            }
                        }
                    });
            Ok(GeneratedModule {
                generated: Some(format!(
                    r#"
                export namespace {namespace} {{
                    {imports}

                    {pdus}
                }}
                "#
                )),
                warnings,
            })
        } else {
            Ok(GeneratedModule::empty())
        }
    }

    fn format_bindings(bindings: &str) -> Result<String, Box<dyn CompilerError>> {
        Ok(bindings.to_string())
    }

    fn generate(&self, tld: ToplevelDefinition) -> Result<String, GeneratorError> {
        match tld {
            ToplevelDefinition::Type(t) => {
                if t.parameterization.is_some() {
                    return Ok(String::new());
                }
                match t.ty {
                    ASN1Type::Null => self.generate_null(t),
                    ASN1Type::Boolean(_) => self.generate_boolean(t),
                    ASN1Type::Integer(_) => self.generate_number_like(t),
                    ASN1Type::Enumerated(_) => self.generate_enumerated(t),
                    ASN1Type::BitString(_) => self.generate_bit_string(t),
                    ASN1Type::Sequence(_) | ASN1Type::Set(_) => self.generate_sequence_or_set(t),
                    ASN1Type::SequenceOf(_) | ASN1Type::SetOf(_) => {
                        self.generate_sequence_or_set_of(t)
                    }
                    ASN1Type::ElsewhereDeclaredType(_) => self.generate_typealias(t),
                    ASN1Type::Choice(_) => self.generate_choice(t),
                    ASN1Type::Time(_) => unimplemented!("rasn does not support TIME types yet!"),
                    ASN1Type::Real(_) => self.generate_number_like(t),
                    ASN1Type::InformationObjectFieldReference(_)
                    | ASN1Type::EmbeddedPdv
                    | ASN1Type::External => self.generate_any(t),
                    ASN1Type::OctetString(_) => self.generate_octet_string(t),
                    ASN1Type::ObjectIdentifier(_)
                    | ASN1Type::GeneralizedTime(_)
                    | ASN1Type::CharacterString(_)
                    | ASN1Type::UTCTime(_) => self.generate_string_like(t),
                    ASN1Type::ChoiceSelectionType(_) => Err(GeneratorError {
                        kind: GeneratorErrorType::Asn1TypeMismatch,
                        details: "Choice selection type should have been resolved at this point!"
                            .into(),
                        top_level_declaration: None,
                    }),
                }
            }
            ToplevelDefinition::Value(v) => self.generate_value(v),
            _ => Ok(String::new()),
        }
    }
}
