//! The `generator` module is responsible for generating rust code that handles
//! decoding and encoding of the parsed and validated ASN1 data elements.
//! The `generator` uses string templates for generating rust code.

use crate::intermediate::*;

pub(crate) mod builder;
pub(crate) mod error;
pub(crate) mod template;
pub(crate) mod utils;

use self::{builder::*, error::GeneratorError};

pub fn generate<'a>(
    tld: ToplevelDeclaration,
) -> Result<std::string::String, GeneratorError> {
    match tld {
        ToplevelDeclaration::Type(t) => match t.r#type {
            ASN1Type::Null => generate_null(t),
            ASN1Type::Boolean => generate_boolean(t),
            ASN1Type::Integer(_) => generate_integer(t),
            ASN1Type::Enumerated(_) => generate_enumerated(t),
            ASN1Type::BitString(_) => generate_bit_string(t),
            ASN1Type::CharacterString(_) => generate_character_string(t),
            ASN1Type::Sequence(_) | ASN1Type::Set(_) => generate_sequence_or_set(t),
            ASN1Type::SequenceOf(_) => generate_sequence_of(t),
            ASN1Type::ElsewhereDeclaredType(_) => generate_typealias(t),
            ASN1Type::Choice(_) => generate_choice(t),
            ASN1Type::OctetString(_) => generate_octet_string(t),
            ASN1Type::Real(_) => todo!(),
            ASN1Type::ObjectIdentifier(_) => generate_oid(t),
            ASN1Type::InformationObjectFieldReference(_) => generate_any(t),
            ASN1Type::GeneralizedTime(_) => generate_generalized_time(t),
            ASN1Type::UTCTime(_) => generate_utc_time(t),
        },
        ToplevelDeclaration::Value(v) => match v.value {
            ASN1Value::Null => generate_null_value(v),
            ASN1Value::Boolean(_) => todo!(),
            ASN1Value::Integer(_) => generate_integer_value(v),
            ASN1Value::String(_) => todo!(),
            ASN1Value::BitString(_) => todo!(),
            ASN1Value::EnumeratedValue {
                enumerated: _,
                enumerable: _,
            } => todo!(),
            ASN1Value::ElsewhereDeclaredValue(_) => todo!(),
            ASN1Value::All => todo!(),
            // ASN1Value::Choice(_, _) => generate_choice_value(v),
            // ASN1Value::Sequence(_) => generate_sequence_value(v),
            ASN1Value::Real(_) => todo!(),
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
