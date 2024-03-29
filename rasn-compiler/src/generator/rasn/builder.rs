use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens, TokenStreamExt};
use std::{collections::BTreeMap, error::Error, str::FromStr};

use crate::{
    generator::{Backend, GeneratedModule},
    intermediate::{
        constraints::Constraint,
        information_object::{
            ASN1Information, ClassLink, InformationObjectClass, InformationObjectFields,
            ObjectSetValue, ToplevelInformationDefinition,
        },
        utils::{
            to_rust_const_case, to_rust_enum_identifier, to_rust_snake_case, to_rust_title_case,
        },
        ASN1Type, ASN1Value, ToplevelDefinition, ToplevelTypeDefinition, ToplevelValueDefinition,
        BIT_STRING, BOOLEAN, GENERALIZED_TIME, INTEGER, NULL, OCTET_STRING, UTC_TIME,
    },
};

use super::{
    generate, template::*, utils::*, Rust, BMP_STRING, GENERAL_STRING, IA5_STRING, NUMERIC_STRING,
    OBJECT_IDENTIFIER, PRINTABLE_STRING, SEQUENCE_OF, SET_OF, UTF8_STRING, VISIBLE_STRING,
};
use crate::generator::error::{GeneratorError, GeneratorErrorType};

impl Backend for Rust {
    fn generate_module(
        &self,
        tlds: Vec<ToplevelDefinition>,
    ) -> Result<GeneratedModule, GeneratorError> {
        if let Some((module_ref, _)) = tlds.first().and_then(|tld| tld.get_index().cloned()) {
            let name = to_rust_snake_case(&module_ref.name);
            let imports = module_ref.imports.iter().map(|import| {
                let module = to_rust_snake_case(&import.global_module_reference.module_reference);
                let mut usages = Some(vec![]);
                'imports: for usage in &import.types {
                    if usage.contains("{}") || usage.chars().all(|c| c.is_uppercase() || c == '-') {
                        usages = None;
                        break 'imports;
                    } else if usage.starts_with(|c: char| c.is_lowercase()) {
                        if let Some(us) = usages.as_mut() {
                            us.push(to_rust_const_case(usage).to_token_stream())
                        }
                    } else if usage.starts_with(|c: char| c.is_uppercase()) {
                        if let Some(us) = usages.as_mut() {
                            us.push(to_rust_title_case(usage).to_token_stream())
                        }
                    }
                }
                let used_imports = usages.unwrap_or(vec![TokenStream::from_str("*").unwrap()]);
                quote!(use super:: #module::{ #(#used_imports),* };)
            });
            let (pdus, warnings): (Vec<TokenStream>, Vec<Box<dyn Error>>) =
                tlds.into_iter()
                    .fold((vec![], vec![]), |mut acc, tld| match generate(tld) {
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
}

pub fn generate_typealias(tld: ToplevelTypeDefinition) -> Result<TokenStream, GeneratorError> {
    if let ASN1Type::ElsewhereDeclaredType(dec) = &tld.ty {
        Ok(typealias_template(
            format_comments(&tld.comments)?,
            to_rust_title_case(&tld.name),
            to_rust_title_case(&dec.identifier),
            join_annotations(vec![
                quote!(delegate),
                format_tag(tld.tag.as_ref(), false),
                format_range_annotations(true, &dec.constraints)?,
            ]),
        ))
    } else {
        Err(GeneratorError::new(
            Some(ToplevelDefinition::Type(tld)),
            "Expected type alias top-level declaration",
            GeneratorErrorType::Asn1TypeMismatch,
        ))
    }
}

pub fn generate_integer_value(tld: ToplevelValueDefinition) -> Result<TokenStream, GeneratorError> {
    if let ASN1Value::LinkedIntValue { integer_type, .. } = tld.value {
        let formatted_value = value_to_tokens(&tld.value, None)?;
        let ty = to_rust_title_case(&tld.associated_type);
        if tld.associated_type == INTEGER {
            Ok(lazy_static_value_template(
                format_comments(&tld.comments)?,
                to_rust_const_case(&tld.name),
                quote!(Integer),
                formatted_value,
            ))
        } else if integer_type.is_unbounded() {
            Ok(lazy_static_value_template(
                format_comments(&tld.comments)?,
                to_rust_const_case(&tld.name),
                ty.clone(),
                quote!(#ty(#formatted_value)),
            ))
        } else {
            Ok(integer_value_template(
                format_comments(&tld.comments)?,
                to_rust_const_case(&tld.name),
                ty.clone(),
                quote!(#ty(#formatted_value)),
            ))
        }
    } else {
        Err(GeneratorError::new(
            Some(ToplevelDefinition::Value(tld)),
            "Expected INTEGER value top-level declaration",
            GeneratorErrorType::Asn1TypeMismatch,
        ))
    }
}

pub fn generate_integer(tld: ToplevelTypeDefinition) -> Result<TokenStream, GeneratorError> {
    if let ASN1Type::Integer(ref int) = tld.ty {
        Ok(integer_template(
            format_comments(&tld.comments)?,
            to_rust_title_case(&tld.name),
            join_annotations(vec![
                quote!(delegate),
                format_range_annotations(true, &int.constraints)?,
                format_tag(tld.tag.as_ref(), false),
            ]),
            int.int_type().to_token_stream(),
        ))
    } else {
        Err(GeneratorError::new(
            Some(ToplevelDefinition::Type(tld)),
            "Expected INTEGER top-level declaration",
            GeneratorErrorType::Asn1TypeMismatch,
        ))
    }
}

pub fn generate_bit_string(tld: ToplevelTypeDefinition) -> Result<TokenStream, GeneratorError> {
    if let ASN1Type::BitString(ref bitstr) = tld.ty {
        Ok(bit_string_template(
            format_comments(&tld.comments)?,
            to_rust_title_case(&tld.name),
            join_annotations(vec![
                quote!(delegate),
                format_range_annotations(true, &bitstr.constraints)?,
                format_tag(tld.tag.as_ref(), false),
            ]),
        ))
    } else {
        Err(GeneratorError::new(
            Some(ToplevelDefinition::Type(tld)),
            "Expected BIT STRING top-level declaration",
            GeneratorErrorType::Asn1TypeMismatch,
        ))
    }
}

pub fn generate_octet_string(tld: ToplevelTypeDefinition) -> Result<TokenStream, GeneratorError> {
    if let ASN1Type::OctetString(ref oct_str) = tld.ty {
        Ok(octet_string_template(
            format_comments(&tld.comments)?,
            to_rust_title_case(&tld.name),
            join_annotations(vec![
                quote!(delegate),
                format_range_annotations(true, &oct_str.constraints)?,
                format_tag(tld.tag.as_ref(), false),
            ]),
        ))
    } else {
        Err(GeneratorError::new(
            Some(ToplevelDefinition::Type(tld)),
            "Expected OCTET STRING top-level declaration",
            GeneratorErrorType::Asn1TypeMismatch,
        ))
    }
}

pub fn generate_character_string(
    tld: ToplevelTypeDefinition,
) -> Result<TokenStream, GeneratorError> {
    if let ASN1Type::CharacterString(ref char_str) = tld.ty {
        Ok(char_string_template(
            format_comments(&tld.comments)?,
            to_rust_title_case(&tld.name),
            string_type(&char_str.ty)?,
            join_annotations(vec![
                quote!(delegate),
                format_range_annotations(false, &char_str.constraints)?,
                format_alphabet_annotations(char_str.ty, &char_str.constraints)?,
                format_tag(tld.tag.as_ref(), false),
            ]),
        ))
    } else {
        Err(GeneratorError::new(
            Some(ToplevelDefinition::Type(tld)),
            "Expected Character String top-level declaration",
            GeneratorErrorType::Asn1TypeMismatch,
        ))
    }
}

pub fn generate_boolean(tld: ToplevelTypeDefinition) -> Result<TokenStream, GeneratorError> {
    // TODO: process boolean constraints
    if let ASN1Type::Boolean(_) = tld.ty {
        Ok(boolean_template(
            format_comments(&tld.comments)?,
            to_rust_title_case(&tld.name),
            join_annotations(vec![quote!(delegate), format_tag(tld.tag.as_ref(), false)]),
        ))
    } else {
        Err(GeneratorError::new(
            Some(ToplevelDefinition::Type(tld)),
            "Expected BOOLEAN top-level declaration",
            GeneratorErrorType::Asn1TypeMismatch,
        ))
    }
}

macro_rules! call_template {
    ($fn:ident, $tld:ident, $($args:expr),*) => {
        Ok($fn(
            format_comments(&$tld.comments)?,
            to_rust_const_case(&$tld.name),
            $($args),*
        ))
    };
}

macro_rules! assignment {
    ($unformatted:expr, $inner:expr) => {{
        let ty = to_rust_title_case($unformatted);
        let inner = $inner;
        quote!(#ty(#inner))
    }};
}

pub fn generate_value(tld: ToplevelValueDefinition) -> Result<TokenStream, GeneratorError> {
    let ty = tld.associated_type.as_str();
    match &tld.value {
        ASN1Value::Null if ty == NULL => {
            call_template!(primitive_value_template, tld, quote!(()), quote!(()))
        }
        ASN1Value::Null => call_template!(
            primitive_value_template,
            tld,
            to_rust_title_case(&tld.associated_type),
            assignment!(&tld.associated_type, quote!(()))
        ),
        ASN1Value::Boolean(b) if ty == BOOLEAN => call_template!(
            primitive_value_template,
            tld,
            quote!(bool),
            b.to_token_stream()
        ),
        ASN1Value::Boolean(b) => call_template!(
            primitive_value_template,
            tld,
            to_rust_title_case(&tld.associated_type),
            assignment!(&tld.associated_type, b.to_token_stream())
        ),
        ASN1Value::LinkedIntValue { .. } => generate_integer_value(tld),
        ASN1Value::BitString(_) if ty == BIT_STRING => call_template!(
            lazy_static_value_template,
            tld,
            quote!(BitString),
            value_to_tokens(&tld.value, None)?
        ),
        ASN1Value::OctetString(_) if ty == OCTET_STRING => call_template!(
            lazy_static_value_template,
            tld,
            quote!(OctetString),
            value_to_tokens(&tld.value, None)?
        ),
        ASN1Value::Choice(choice, inner) => {
            if inner.is_const_type() {
                call_template!(
                    const_choice_value_template,
                    tld,
                    to_rust_title_case(&tld.associated_type),
                    to_rust_enum_identifier(choice),
                    value_to_tokens(inner, None)?
                )
            } else {
                call_template!(
                    choice_value_template,
                    tld,
                    to_rust_title_case(&tld.associated_type),
                    to_rust_enum_identifier(choice),
                    value_to_tokens(inner, None)?
                )
            }
        }
        ASN1Value::EnumeratedValue {
            enumerated,
            enumerable,
        } => call_template!(
            enum_value_template,
            tld,
            to_rust_title_case(enumerated),
            to_rust_enum_identifier(enumerable)
        ),
        ASN1Value::Time(_) if ty == GENERALIZED_TIME => call_template!(
            lazy_static_value_template,
            tld,
            quote!(GeneralizedTime),
            value_to_tokens(&tld.value, Some(&quote!(GeneralizedTime)))?
        ),
        ASN1Value::Time(_) if ty == UTC_TIME => call_template!(
            lazy_static_value_template,
            tld,
            quote!(UtcTime),
            value_to_tokens(&tld.value, Some(&quote!(UtcTime)))?
        ),
        ASN1Value::LinkedStructLikeValue(s) => {
            let members = s
                .iter()
                .map(|(_, val)| value_to_tokens(val.value(), None))
                .collect::<Result<Vec<TokenStream>, _>>()?;
            call_template!(
                sequence_or_set_value_template,
                tld,
                to_rust_title_case(&tld.associated_type),
                quote!(#(#members),*)
            )
        }
        ASN1Value::LinkedNestedValue { supertypes, value } => {
            let parent = supertypes.last().map(to_rust_title_case);
            if value.is_const_type() {
                call_template!(
                    primitive_value_template,
                    tld,
                    to_rust_title_case(&tld.associated_type),
                    assignment!(
                        &tld.associated_type,
                        value_to_tokens(&tld.value, parent.as_ref())?
                    )
                )
            } else {
                call_template!(
                    lazy_static_value_template,
                    tld,
                    to_rust_title_case(&tld.associated_type),
                    assignment!(
                        &tld.associated_type,
                        value_to_tokens(&tld.value, parent.as_ref())?
                    )
                )
            }
        }
        ASN1Value::ObjectIdentifier(_) if ty == OBJECT_IDENTIFIER => call_template!(
            lazy_static_value_template,
            tld,
            quote!(ObjectIdentifier),
            value_to_tokens(&tld.value, None)?
        ),
        ASN1Value::LinkedCharStringValue(_, _) if ty == NUMERIC_STRING => call_template!(
            lazy_static_value_template,
            tld,
            quote!(NumericString),
            value_to_tokens(&tld.value, None)?
        ),
        ASN1Value::LinkedCharStringValue(_, _) if ty == VISIBLE_STRING => call_template!(
            lazy_static_value_template,
            tld,
            quote!(VisibleString),
            value_to_tokens(&tld.value, None)?
        ),
        ASN1Value::LinkedCharStringValue(_, _) if ty == IA5_STRING => call_template!(
            lazy_static_value_template,
            tld,
            quote!(IA5String),
            value_to_tokens(&tld.value, None)?
        ),
        ASN1Value::LinkedCharStringValue(_, _) if ty == UTF8_STRING => call_template!(
            lazy_static_value_template,
            tld,
            quote!(UTF8String),
            value_to_tokens(&tld.value, None)?
        ),
        ASN1Value::LinkedCharStringValue(_, _) if ty == BMP_STRING => call_template!(
            lazy_static_value_template,
            tld,
            quote!(BMPString),
            value_to_tokens(&tld.value, None)?
        ),
        ASN1Value::LinkedCharStringValue(_, _) if ty == PRINTABLE_STRING => call_template!(
            lazy_static_value_template,
            tld,
            quote!(PrintableString),
            value_to_tokens(&tld.value, None)?
        ),
        ASN1Value::LinkedCharStringValue(_, _) if ty == GENERAL_STRING => call_template!(
            lazy_static_value_template,
            tld,
            quote!(GeneralString),
            value_to_tokens(&tld.value, None)?
        ),
        ASN1Value::LinkedArrayLikeValue(s) if ty.contains(SEQUENCE_OF) => {
            let item_type = format_sequence_or_set_of_item_type(
                ty.replace(SEQUENCE_OF, "").trim().to_string(),
                s.first().map(|i| &**i),
            );
            call_template!(
                lazy_static_value_template,
                tld,
                quote!(Vec<#item_type>),
                value_to_tokens(&tld.value, None)?
            )
        }
        ASN1Value::LinkedArrayLikeValue(s) if ty.contains(SET_OF) => {
            let item_type = format_sequence_or_set_of_item_type(
                ty.replace(SET_OF, "").trim().to_string(),
                s.first().map(|i| &**i),
            );
            call_template!(
                lazy_static_value_template,
                tld,
                quote!(Vec<#item_type>),
                value_to_tokens(&tld.value, None)?
            )
        }
        ASN1Value::BitString(_)
        | ASN1Value::Time(_)
        | ASN1Value::LinkedCharStringValue(_, _)
        | ASN1Value::ObjectIdentifier(_)
        | ASN1Value::LinkedArrayLikeValue(_)
        | ASN1Value::ElsewhereDeclaredValue { .. }
        | ASN1Value::OctetString(_) => call_template!(
            lazy_static_value_template,
            tld,
            to_rust_title_case(&tld.associated_type),
            assignment!(&tld.associated_type, value_to_tokens(&tld.value, None)?)
        ),
        _ => Ok(TokenStream::new()),
    }
}

pub fn generate_any(tld: ToplevelTypeDefinition) -> Result<TokenStream, GeneratorError> {
    Ok(any_template(
        format_comments(&tld.comments)?,
        to_rust_title_case(&tld.name),
        join_annotations(vec![quote!(delegate), format_tag(tld.tag.as_ref(), false)]),
    ))
}

pub fn generate_generalized_time(
    tld: ToplevelTypeDefinition,
) -> Result<TokenStream, GeneratorError> {
    if let ASN1Type::GeneralizedTime(_) = &tld.ty {
        Ok(generalized_time_template(
            format_comments(&tld.comments)?,
            to_rust_title_case(&tld.name),
            join_annotations(vec![quote!(delegate), format_tag(tld.tag.as_ref(), false)]),
        ))
    } else {
        Err(GeneratorError::new(
            Some(ToplevelDefinition::Type(tld)),
            "Expected GeneralizedTime top-level declaration",
            GeneratorErrorType::Asn1TypeMismatch,
        ))
    }
}

pub fn generate_utc_time(tld: ToplevelTypeDefinition) -> Result<TokenStream, GeneratorError> {
    if let ASN1Type::UTCTime(_) = &tld.ty {
        Ok(utc_time_template(
            format_comments(&tld.comments)?,
            to_rust_title_case(&tld.name),
            join_annotations(vec![quote!(delegate), format_tag(tld.tag.as_ref(), false)]),
        ))
    } else {
        Err(GeneratorError::new(
            Some(ToplevelDefinition::Type(tld)),
            "Expected UTCTime top-level declaration",
            GeneratorErrorType::Asn1TypeMismatch,
        ))
    }
}

pub fn generate_oid(tld: ToplevelTypeDefinition) -> Result<TokenStream, GeneratorError> {
    if let ASN1Type::ObjectIdentifier(oid) = &tld.ty {
        Ok(oid_template(
            format_comments(&tld.comments)?,
            to_rust_title_case(&tld.name),
            join_annotations(vec![
                quote!(delegate),
                format_tag(tld.tag.as_ref(), false),
                format_range_annotations(false, &oid.constraints)?,
            ]),
        ))
    } else {
        Err(GeneratorError::new(
            Some(ToplevelDefinition::Type(tld)),
            "Expected OBJECT IDENTIFIER top-level declaration",
            GeneratorErrorType::Asn1TypeMismatch,
        ))
    }
}

pub fn generate_null(tld: ToplevelTypeDefinition) -> Result<TokenStream, GeneratorError> {
    if let ASN1Type::Null = tld.ty {
        Ok(null_template(
            format_comments(&tld.comments)?,
            to_rust_title_case(&tld.name),
            join_annotations(vec![quote!(delegate), format_tag(tld.tag.as_ref(), false)]),
        ))
    } else {
        Err(GeneratorError::new(
            Some(ToplevelDefinition::Type(tld)),
            "Expected NULL top-level declaration",
            GeneratorErrorType::Asn1TypeMismatch,
        ))
    }
}

pub fn generate_enumerated(tld: ToplevelTypeDefinition) -> Result<TokenStream, GeneratorError> {
    if let ASN1Type::Enumerated(ref enumerated) = tld.ty {
        let extensible = enumerated
            .extensible
            .map(|_| {
                quote! {
                #[non_exhaustive]}
            })
            .unwrap_or_default();
        Ok(enumerated_template(
            format_comments(&tld.comments)?,
            to_rust_title_case(&tld.name),
            extensible,
            format_enum_members(enumerated),
            join_annotations(vec![
                quote!(enumerated),
                format_tag(tld.tag.as_ref(), false),
            ]),
        ))
    } else {
        Err(GeneratorError::new(
            Some(ToplevelDefinition::Type(tld)),
            "Expected ENUMERATED top-level declaration",
            GeneratorErrorType::Asn1TypeMismatch,
        ))
    }
}

pub fn generate_choice(tld: ToplevelTypeDefinition) -> Result<TokenStream, GeneratorError> {
    if let ASN1Type::Choice(ref choice) = tld.ty {
        let name = to_rust_title_case(&tld.name);
        let inner_options = format_nested_choice_options(choice, &name.to_string())?;
        let extensible = choice
            .extensible
            .map(|_| {
                quote! {
                #[non_exhaustive]}
            })
            .unwrap_or_default();
        Ok(choice_template(
            format_comments(&tld.comments)?,
            name.clone(),
            extensible,
            format_choice_options(choice, &name.to_string())?,
            inner_options,
            join_annotations(vec![quote!(choice), format_tag(tld.tag.as_ref(), true)]),
        ))
    } else {
        Err(GeneratorError::new(
            Some(ToplevelDefinition::Type(tld)),
            "Expected CHOICE top-level declaration",
            GeneratorErrorType::Asn1TypeMismatch,
        ))
    }
}

pub fn generate_sequence_or_set(
    tld: ToplevelTypeDefinition,
) -> Result<TokenStream, GeneratorError> {
    match tld.ty {
        ASN1Type::Sequence(ref seq) | ASN1Type::Set(ref seq) => {
            let name = to_rust_title_case(&tld.name);
            let extensible = seq
                .extensible
                .map(|_| {
                    quote! {
                    #[non_exhaustive]}
                })
                .unwrap_or_default();
            let set_annotation = if let ASN1Type::Set(_) = tld.ty {
                quote!(set)
            } else {
                TokenStream::new()
            };
            let class_fields = seq.members.iter().fold(TokenStream::new(), |mut acc, m| { [m.constraints.clone(), m.ty.constraints().map_or(vec![], |c| c.to_vec())].concat().iter().for_each(|c| {
                let decode_fn = format_ident!("decode_{}", to_rust_snake_case(&m.name));
                let open_field_name = to_rust_snake_case(&m.name);
                if let (Constraint::TableConstraint(t), ASN1Type::InformationObjectFieldReference(iofr)) = (c, &m.ty) {
                        let identifier = t.linked_fields.iter().map(|l|
                            to_rust_snake_case(&l.field_name)
                        );
                        let field_name = iofr.field_path.last().unwrap().identifier().replace('&', "");
                        if field_name.starts_with(|initial: char| initial.is_lowercase()) {
                            // Fixed-value fields of Information Object usages should have been resolved at this point
                            return;
                        }
                        let obj_set_name = match t.object_set.values.first() {
                            Some(ObjectSetValue::Reference(s)) => to_rust_title_case(s),
                            _ => todo!()
                        };
                        let field_enum_name = format_ident!("{obj_set_name}_{field_name}");
                        let input = m.is_optional.then(|| quote!(self. #open_field_name .as_ref())).unwrap_or(quote!(Some(&self. #open_field_name)));
                        acc.append_all(quote! {

                            impl #name {
                                pub fn #decode_fn<D: Decoder>(&self, decoder: &mut D) -> Result<#field_enum_name, D::Error> {
                                    #field_enum_name ::decode(decoder, #input, &self. #(#identifier).*)
                                }
                            }
                        });
                };
            });
            acc
            });
            let (declaration, name_types) = format_sequence_or_set_members(seq, &name.to_string())?;
            Ok(sequence_or_set_template(
                format_comments(&tld.comments)?,
                name.clone(),
                extensible,
                declaration,
                format_nested_sequence_members(seq, &name.to_string())?,
                join_annotations(vec![set_annotation, format_tag(tld.tag.as_ref(), true)]),
                format_default_methods(&seq.members, &name.to_string())?,
                format_new_impl(&name, name_types),
                class_fields,
            ))
        }
        _ => Err(GeneratorError::new(
            Some(ToplevelDefinition::Type(tld)),
            "Expected SEQUENCE top-level declaration",
            GeneratorErrorType::Asn1TypeMismatch,
        )),
    }
}

pub fn generate_sequence_or_set_of(
    tld: ToplevelTypeDefinition,
) -> Result<TokenStream, GeneratorError> {
    let (is_set_of, seq_or_set_of) = match &tld.ty {
        ASN1Type::SetOf(se_of) => (true, se_of),
        ASN1Type::SequenceOf(se_of) => (false, se_of),
        _ => {
            return Err(GeneratorError::new(
                Some(ToplevelDefinition::Type(tld)),
                "Expected SEQUENCE OF top-level declaration",
                GeneratorErrorType::Asn1TypeMismatch,
            ))
        }
    };
    let name = to_rust_title_case(&tld.name);
    let anonymous_item = match seq_or_set_of.element_type.as_ref() {
        ASN1Type::ElsewhereDeclaredType(_) => None,
        n => Some(generate(ToplevelDefinition::Type(
            ToplevelTypeDefinition {
                parameterization: None,
                comments: format!(
                    " Anonymous {} OF member ",
                    if is_set_of { "SET" } else { "SEQUENCE" }
                ),
                name: String::from("Anonymous") + &name.to_string(),
                ty: n.clone(),
                tag: None,
                index: None,
            },
        ))?),
    }
    .unwrap_or_default();
    let member_type = match seq_or_set_of.element_type.as_ref() {
        ASN1Type::ElsewhereDeclaredType(d) => to_rust_title_case(&d.identifier),
        _ => format_ident!("Anonymous{}", &name.to_string()).to_token_stream(),
    };
    Ok(sequence_or_set_of_template(
        is_set_of,
        format_comments(&tld.comments)?,
        name,
        anonymous_item,
        member_type,
        join_annotations(vec![
            quote!(delegate),
            format_range_annotations(true, &seq_or_set_of.constraints)?,
            format_tag(tld.tag.as_ref(), false),
        ]),
    ))
}

pub fn generate_information_object_set(
    tld: ToplevelInformationDefinition,
) -> Result<TokenStream, GeneratorError> {
    if let ASN1Information::ObjectSet(o) = &tld.value {
        let class: &InformationObjectClass = match tld.class {
            Some(ClassLink::ByReference(ref c)) => c,
            _ => {
                return Err(GeneratorError::new(
                    None,
                    "Missing class link in Information Object Set",
                    GeneratorErrorType::MissingClassKey,
                ))
            }
        };
        let mut keys_to_types = o
            .values
            .iter()
            .map(|v| match v {
                ObjectSetValue::Reference(r) => Err(GeneratorError::new(
                    None,
                    &format!("Could not resolve reference of Information Object Set {r}"),
                    GeneratorErrorType::MissingClassKey,
                )),
                ObjectSetValue::Inline(InformationObjectFields::CustomSyntax(_)) => {
                    Err(GeneratorError::new(
                        Some(ToplevelDefinition::Information(tld.clone())),
                        "Unexpectedly encountered unresolved custom syntax!",
                        GeneratorErrorType::MissingClassKey,
                    ))
                }
                ObjectSetValue::Inline(InformationObjectFields::DefaultSyntax(s)) => {
                    resolve_standard_syntax(class, s)
                }
            })
            .collect::<Result<Vec<(ASN1Value, Vec<(usize, ASN1Type)>)>, _>>()?;
        let mut choices = BTreeMap::<String, Vec<(ASN1Value, ASN1Type)>>::new();
        for (key, items) in keys_to_types.drain(..) {
            for (index, item) in items {
                let id = class
                    .fields
                    .get(index)
                    .map(|f| f.identifier.identifier())
                    .ok_or_else(|| GeneratorError {
                        top_level_declaration: Some(ToplevelDefinition::Information(tld.clone())),
                        details: "Could not find class field for index.".into(),
                        kind: GeneratorErrorType::SyntaxMismatch,
                    })?;
                match choices.get_mut(id) {
                    Some(entry) => entry.push((key.clone(), item)),
                    None => {
                        choices.insert(id.clone(), vec![(key.clone(), item)]);
                    }
                }
            }
        }

        let name = to_rust_title_case(&tld.name);
        let class_unique_id_type = class
            .fields
            .iter()
            .find_map(|f| (f.is_unique).then(|| f.ty.clone()))
            .flatten()
            .ok_or_else(|| GeneratorError {
                top_level_declaration: None,
                details: "Could not determine unique class identifier type.".into(),
                kind: GeneratorErrorType::SyntaxMismatch,
            })?;
        let class_unique_id_type_name = type_to_tokens(&class_unique_id_type)?;

        let mut field_enums = vec![];
        for (field_name, fields) in choices.iter() {
            let field_enum_name = format_ident!("{name}_{}", field_name.replace('&', ""));
            let (mut ids, mut inner_types) = (vec![], vec![]);
            for (index, (id, ty)) in fields.iter().enumerate() {
                let identifier_value = if let ASN1Value::LinkedElsewhereDefinedValue {
                    can_be_const: false,
                    ..
                } = id
                {
                    let tokenized_value = value_to_tokens(id, Some(&class_unique_id_type_name))?;
                    quote!(*#tokenized_value)
                } else {
                    value_to_tokens(id, Some(&class_unique_id_type_name))?
                };
                let type_id = type_to_tokens(ty).unwrap_or(quote!(Option<()>));
                let variant_name = match id {
                    ASN1Value::LinkedElsewhereDefinedValue {
                        identifier: ref_id, ..
                    }
                    | ASN1Value::ElsewhereDeclaredValue {
                        identifier: ref_id, ..
                    } => to_rust_title_case(ref_id),
                    _ => format_ident!("{field_enum_name}_{index}").to_token_stream(),
                };
                if ty.constraints().map_or(true, |c| c.is_empty()) {
                    ids.push((variant_name, type_id, identifier_value));
                    inner_types.push(TokenStream::new());
                } else {
                    let (signed_range, character_string_type) = match ty {
                        ASN1Type::CharacterString(c) => (false, Some(c.ty)),
                        ASN1Type::Integer(_) => (true, None),
                        ASN1Type::Real(_) => (true, None),
                        ASN1Type::BitString(_) => (false, None),
                        ASN1Type::OctetString(_) => (false, None),
                        _ => (false, None),
                    };
                    let delegate_id = &format!("Inner_{field_enum_name}_{index}")
                        .parse::<TokenStream>()
                        .unwrap();
                    let range_constraints = format_range_annotations(
                        signed_range,
                        ty.constraints().unwrap_or(&Vec::<_>::new()),
                    )
                    .unwrap();
                    let alphabet_constraints = character_string_type
                        .and_then(|c| {
                            format_alphabet_annotations(
                                c,
                                ty.constraints().unwrap_or(&Vec::<_>::new()),
                            )
                            .ok()
                        })
                        .unwrap_or_default();
                    let annotations = join_annotations(vec![
                        range_constraints,
                        alphabet_constraints,
                        quote!(delegate),
                    ]);
                    ids.push((variant_name, delegate_id.clone(), identifier_value));
                    inner_types.push(quote! {
                        #[derive(Debug, Clone, PartialEq, AsnType, Decode, Encode)]
                        #annotations
                        pub struct #delegate_id (pub #type_id);

                    });
                }
            }

            let variants = ids
                .iter()
                .map(|(variant_name, type_id, _)| quote!(#variant_name (#type_id),));

            let de_match_arms = ids.iter().map(|(variant_name, _, identifier_value)| {
                quote!(i if i == &#identifier_value => Ok(decoder.codec().decode_from_binary(open_type_payload.ok_or_else(|| rasn::error::DecodeError::from_kind(
                    rasn::error::DecodeErrorKind::Custom {
                        msg: "Failed to decode open type! No input data given.".into(),
                    },
                    decoder.codec()
                ).into())?.as_bytes()).map(Self:: #variant_name)?),)
            });

            let en_match_arms = ids.iter().map(|(variant_name, _, identifier_value)| {
                quote!((Self::#variant_name (inner), i) if i == &#identifier_value =>inner.encode(encoder),)
            });

            field_enums.push(quote! {
                #(#inner_types)*

                #[derive(Debug, Clone, PartialEq)]
                pub enum #field_enum_name {
                    #(#variants)*
                }

                impl #field_enum_name {
                    pub fn decode<D: Decoder>(decoder: &mut D, open_type_payload: Option<&Any>, identifier: & #class_unique_id_type_name) -> Result<Self, D::Error> {
                        match identifier {
                            #(#de_match_arms)*
                            _ => Err(rasn::error::DecodeError::from_kind(
                                rasn::error::DecodeErrorKind::Custom {
                                    msg: alloc::format!("Unknown unique identifier for information object class instance."),
                                },
                                decoder.codec()
                            ).into())
                        }
                    }

                    pub fn encode<E: Encoder>(&self, encoder: &mut E, identifier: & #class_unique_id_type_name) -> Result<(), E::Error> {
                        match (self, identifier) {
                            #(#en_match_arms)*
                            _ => Err(rasn::error::EncodeError::from_kind(
                                rasn::error::EncodeErrorKind::Custom {
                                    msg: alloc::format!("Unknown unique identifier for information object class instance."),
                                },
                                encoder.codec()
                            ).into())
                        }
                    }
                }

            });
        }

        Ok(quote!(#(#field_enums)*))
    } else {
        Err(GeneratorError::new(
            Some(ToplevelDefinition::Information(tld)),
            "Expected Object Set top-level declaration",
            GeneratorErrorType::Asn1TypeMismatch,
        ))
    }
}
