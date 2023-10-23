//! The `generator` module is responsible for generating rust code that handles
//! decoding and encoding of the parsed and validated ASN1 data elements.
//! The `generator` uses string templates for generating rust code.

use crate::intermediate::*;

pub(crate) mod builder;
pub(crate) mod error;
pub(crate) mod template;
pub(crate) mod utils;

use self::{builder::*, error::{GeneratorError, GeneratorErrorType}};

pub fn generate<'a>(
    tld: ToplevelDeclaration,
) -> Result<std::string::String, GeneratorError> {
    match tld {
        ToplevelDeclaration::Type(t) => match t.r#type {
            ASN1Type::Null => generate_null(t),
            ASN1Type::Boolean(_) => generate_boolean(t),
            ASN1Type::Integer(_) => generate_integer(t),
            ASN1Type::Enumerated(_) => generate_enumerated(t),
            ASN1Type::BitString(_) => generate_bit_string(t),
            ASN1Type::CharacterString(_) => generate_character_string(t),
            ASN1Type::Sequence(_) | ASN1Type::Set(_) => generate_sequence_or_set(t),
            ASN1Type::SequenceOf(_) |
            ASN1Type::SetOf(_) => generate_sequence_or_set_of(t),
            ASN1Type::ElsewhereDeclaredType(_) => generate_typealias(t),
            ASN1Type::Choice(_) => generate_choice(t),
            ASN1Type::OctetString(_) => generate_octet_string(t),
            ASN1Type::Real(_) => Err(GeneratorError {
                kind: GeneratorErrorType::NotYetInplemented,
                details: "Real types are currently unsupported!".into(),
                top_level_declaration: None,
            }),
            ASN1Type::ObjectIdentifier(_) => generate_oid(t),
            ASN1Type::InformationObjectFieldReference(_) | ASN1Type::EmbeddedPdv | ASN1Type::External => generate_any(t),
            ASN1Type::GeneralizedTime(_) => generate_generalized_time(t),
            ASN1Type::UTCTime(_) => generate_utc_time(t),
            ASN1Type::ChoiceSelectionType(_) => unreachable!()
        },
        ToplevelDeclaration::Value(v) => match v.value {
            ASN1Value::Null => generate_null_value(v),
            ASN1Value::Boolean(_) => Err(GeneratorError {
            kind: GeneratorErrorType::NotYetInplemented,
            details: "Boolean values are currently unsupported!".into(),
            top_level_declaration: None,
        }),
            ASN1Value::Integer(_) => generate_integer_value(v),
            ASN1Value::String(_) => Err(GeneratorError {
            kind: GeneratorErrorType::NotYetInplemented,
            details: "String values are currently unsupported!".into(),
            top_level_declaration: None,
        }),
            ASN1Value::BitString(_) => Err(GeneratorError {
            kind: GeneratorErrorType::NotYetInplemented,
            details: "BitString values are currently unsupported!".into(),
            top_level_declaration: None,
        }),
            ASN1Value::EnumeratedValue {
                enumerated: _,
                enumerable: _,
            } => Err(GeneratorError {
            kind: GeneratorErrorType::NotYetInplemented,
            details: "EnumeratedValue values are currently unsupported!".into(),
            top_level_declaration: None,
        }),
            ASN1Value::ElsewhereDeclaredValue(_) => Err(GeneratorError {
            kind: GeneratorErrorType::NotYetInplemented,
            details: "Value cross references are currently unsupported!".into(),
            top_level_declaration: None,
        }),
            ASN1Value::All => Err(GeneratorError {
            kind: GeneratorErrorType::NotYetInplemented,
            details: "ALL values are currently unsupported!".into(),
            top_level_declaration: None,
        }),
            // ASN1Value::Choice(_, _) => generate_choice_value(v),
            // ASN1Value::Sequence(_) => generate_sequence_value(v),
            ASN1Value::Real(_) => Err(GeneratorError {
                kind: GeneratorErrorType::NotYetInplemented,
                details: "Real values are currently unsupported!".into(),
                top_level_declaration: None,
            }),
            ASN1Value::ObjectIdentifier(_) => generate_object_identifier_value(v),
            ASN1Value::Time(_) => generate_time_value(v),
            _ => Ok("".into()),
        },
        ToplevelDeclaration::Information(i) => match i.value {
            // ASN1Information::ObjectClass(_) => {
            //     generate_information_object_class(i)
            // }
            // ASN1Information::ObjectSet(_) => {
            //   generate_information_object_set(i)
            // }
            _ => Ok("".into()),
        },
    }
}
