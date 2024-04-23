use std::{
    env,
    error::Error,
    io::{self, Write},
    path::PathBuf,
    process::{Command, Stdio},
    str::FromStr,
};

use self::information_object::ASN1Information;
use crate::intermediate::*;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

use super::{
    error::{GeneratorError, GeneratorErrorType},
    Backend, GeneratedModule,
};

mod builder;
mod template;
mod utils;

#[derive(Debug, Default)]
/// A compiler backend that generates bindings to be used with
/// the `rasn` framework for rust. 
pub struct Rasn {
    config: Config,
}

#[derive(Debug, Default)]
/// A configuration for the [Rasn] backend
pub struct Config {
    /// ASN.1 Open Types are represented as the `rasn::types::Any` type,
    /// which holds a binary `content`. If `opaque_open_types` is `false`,
    /// the compiler will generate de-/encode methods for all rust types 
    /// that hold an open type. In this way, for example a SEQUENCE field
    /// of an Open Type can be completely decoded to a rust type instance.
    /// While with `opaque_open_type == true`, the same SEQUENCE field would
    /// be represented as an `Any`-wrapped `Vec<u8>` containing the encoded
    /// actual value of the Open Type.
    pub opaque_open_types: bool,
    /// The compiler will try to match module import dependencies of the ASN.1
    /// module as close as possible, importing only those types from other modules
    /// that are imported in the ASN.1 module. If the `default_wildcard_imports`
    /// is set to `true` , the compiler will import the entire module using 
    /// the wildcard `*` for each module that the input ASN.1 module imports from.
    pub default_wildcard_imports: bool,
}

impl Backend for Rasn {
    type Config = Config;

    fn from_config(config: Self::Config) -> Self {
        Self { config }
    }

    fn config(&self) -> &Self::Config {
        &self.config
    }

    fn generate_module(
        &self,
        tlds: Vec<ToplevelDefinition>,
    ) -> Result<GeneratedModule, GeneratorError> {
        if let Some((module_ref, _)) = tlds.first().and_then(|tld| tld.get_index().cloned()) {
            let module = module_ref.borrow();
            let name = self.to_rust_snake_case(&module.name);
            let imports = module.imports.iter().map(|import| {
                let module =
                    self.to_rust_snake_case(&import.global_module_reference.module_reference);
                let mut usages = Some(vec![]);
                'imports: for usage in &import.types {
                    if usage.contains("{}") || usage.chars().all(|c| c.is_uppercase() || c == '-') {
                        usages = None;
                        break 'imports;
                    } else if usage.starts_with(|c: char| c.is_lowercase()) {
                        if let Some(us) = usages.as_mut() {
                            us.push(self.to_rust_const_case(usage).to_token_stream())
                        }
                    } else if usage.starts_with(|c: char| c.is_uppercase()) {
                        if let Some(us) = usages.as_mut() {
                            us.push(self.to_rust_title_case(usage).to_token_stream())
                        }
                    }
                }
                let used_imports = if self.config.default_wildcard_imports {
                    vec![TokenStream::from_str("*").unwrap()]
                } else {
                    usages.unwrap_or(vec![TokenStream::from_str("*").unwrap()])
                };
                quote!(use super:: #module::{ #(#used_imports),* };)
            });
            let (pdus, warnings): (Vec<TokenStream>, Vec<Box<dyn Error>>) =
                tlds.into_iter()
                    .fold((vec![], vec![]), |mut acc, tld| match self.generate(tld) {
                        Ok(s) => {
                            acc.0.push(s);
                            acc
                        }
                        Err(e) => {
                            acc.1.push(Box::new(e));
                            acc
                        }
                    });
            Ok(GeneratedModule {
                generated: Some(quote! {
                #[allow(non_camel_case_types, non_snake_case, non_upper_case_globals, unused)]
                pub mod #name {
                    extern crate alloc;

                    use core::borrow::Borrow;
                    use rasn::prelude::*;
                    use lazy_static::lazy_static;

                    #(#imports)*

                    #(#pdus)*
                }
            }.to_string()), warnings})
        } else {
            Ok(GeneratedModule::empty())
        }
    }

    fn format_bindings(bindings: &str) -> Result<String, Box<dyn Error>> {
        let mut rustfmt = PathBuf::from(env::var("CARGO_HOME")?);
        rustfmt.push("bin/rustfmt");
        let mut cmd = Command::new(&*rustfmt);

        cmd.stdin(Stdio::piped()).stdout(Stdio::piped());

        let mut child = cmd.spawn()?;
        let mut child_stdin = child.stdin.take().unwrap();
        let mut child_stdout = child.stdout.take().unwrap();

        // Write to stdin in a new thread, so that we can read from stdout on this
        // thread. This keeps the child from blocking on writing to its stdout which
        // might block us from writing to its stdin.
        let bindings = bindings.to_owned();
        let stdin_handle = ::std::thread::spawn(move || {
            let _ = child_stdin.write_all(bindings.as_bytes());
            bindings
        });

        let mut output = vec![];
        io::copy(&mut child_stdout, &mut output)?;

        let status = child.wait()?;
        let bindings = stdin_handle.join().expect(
            "The thread writing to rustfmt's stdin doesn't do \
             anything that could panic",
        );

        match String::from_utf8(output) {
            Ok(bindings) => match status.code() {
                Some(0) => Ok(bindings),
                Some(2) => Err(Box::new(io::Error::new(
                    io::ErrorKind::Other,
                    "Rustfmt parsing errors.".to_string(),
                ))),
                Some(3) => Ok(bindings),
                _ => Err(Box::new(io::Error::new(
                    io::ErrorKind::Other,
                    "Internal rustfmt error".to_string(),
                ))),
            },
            _ => Ok(bindings),
        }
    }

    fn generate(&self, tld: ToplevelDefinition) -> Result<TokenStream, GeneratorError> {
        match tld {
            ToplevelDefinition::Type(t) => {
                if t.parameterization.is_some() {
                    return Ok(TokenStream::new());
                }
                match t.ty {
                    ASN1Type::Null => self.generate_null(t),
                    ASN1Type::Boolean(_) => self.generate_boolean(t),
                    ASN1Type::Integer(_) => self.generate_integer(t),
                    ASN1Type::Enumerated(_) => self.generate_enumerated(t),
                    ASN1Type::BitString(_) => self.generate_bit_string(t),
                    ASN1Type::CharacterString(_) => self.generate_character_string(t),
                    ASN1Type::Sequence(_) | ASN1Type::Set(_) => self.generate_sequence_or_set(t),
                    ASN1Type::SequenceOf(_) | ASN1Type::SetOf(_) => {
                        self.generate_sequence_or_set_of(t)
                    }
                    ASN1Type::ElsewhereDeclaredType(_) => self.generate_typealias(t),
                    ASN1Type::Choice(_) => self.generate_choice(t),
                    ASN1Type::OctetString(_) => self.generate_octet_string(t),
                    ASN1Type::Time(_) => unimplemented!("rasn does not support TIME types yet!"),
                    ASN1Type::Real(_) => Err(GeneratorError {
                        kind: GeneratorErrorType::NotYetInplemented,
                        details: "Real types are currently unsupported!".into(),
                        top_level_declaration: None,
                    }),
                    ASN1Type::ObjectIdentifier(_) => self.generate_oid(t),
                    ASN1Type::InformationObjectFieldReference(_)
                    | ASN1Type::EmbeddedPdv
                    | ASN1Type::External => self.generate_any(t),
                    ASN1Type::GeneralizedTime(_) => self.generate_generalized_time(t),
                    ASN1Type::UTCTime(_) => self.generate_utc_time(t),
                    ASN1Type::ChoiceSelectionType(_) => Err(GeneratorError {
                        kind: GeneratorErrorType::Asn1TypeMismatch,
                        details: "Choice selection type should have been resolved at this point!"
                            .into(),
                        top_level_declaration: None,
                    }),
                }
            }
            ToplevelDefinition::Value(v) => self.generate_value(v),
            ToplevelDefinition::Information(i) => match i.value {
                ASN1Information::ObjectSet(_) => self.generate_information_object_set(i),
                _ => Ok(TokenStream::new()),
            },
        }
    }
}
