use crate::intermediate::{
    ASN1Type, ToplevelDefinition, ToplevelTypeDefinition, ToplevelValueDefinition,
};

use super::{template::*, utils::*, Typescript};
use crate::generator::error::{GeneratorError, GeneratorErrorType};

impl Typescript {
    pub(crate) fn generate_typealias(
        &self,
        tld: ToplevelTypeDefinition,
    ) -> Result<String, GeneratorError> {
        if let ASN1Type::ElsewhereDeclaredType(dec) = &tld.ty {
            Ok(typealias_template(
                &format_comments(&tld.comments),
                &to_jer_identifier(&tld.name),
                &to_jer_identifier(&dec.identifier),
            ))
        } else {
            Err(GeneratorError::new(
                Some(ToplevelDefinition::Type(tld)),
                "Expected type alias top-level declaration",
                GeneratorErrorType::Asn1TypeMismatch,
            ))
        }
    }

    pub(crate) fn generate_number_like(
        &self,
        tld: ToplevelTypeDefinition,
    ) -> Result<String, GeneratorError> {
        if let ASN1Type::Integer(_) = tld.ty {
            Ok(number_like_template(
                &format_comments(&tld.comments),
                &to_jer_identifier(&tld.name),
            ))
        } else {
            Err(GeneratorError::new(
                Some(ToplevelDefinition::Type(tld)),
                "Expected INTEGER top-level declaration",
                GeneratorErrorType::Asn1TypeMismatch,
            ))
        }
    }

    pub(crate) fn generate_bit_string(
        &self,
        tld: ToplevelTypeDefinition,
    ) -> Result<String, GeneratorError> {
        if let ASN1Type::BitString(ref bit_str) = tld.ty {
            Ok(bit_string_template(
                &format_comments(&tld.comments),
                &to_jer_identifier(&tld.name),
                if is_fixed_size(bit_str) {
                    "string"
                } else {
                    "{ value: string, length: number }"
                },
            ))
        } else {
            Err(GeneratorError::new(
                Some(ToplevelDefinition::Type(tld)),
                "Expected BIT STRING top-level declaration",
                GeneratorErrorType::Asn1TypeMismatch,
            ))
        }
    }

    pub(crate) fn generate_octet_string(
        &self,
        tld: ToplevelTypeDefinition,
    ) -> Result<String, GeneratorError> {
        if let ASN1Type::OctetString(_) = tld.ty {
            Ok(octet_string_template(
                &format_comments(&tld.comments),
                &to_jer_identifier(&tld.name),
            ))
        } else {
            Err(GeneratorError::new(
                Some(ToplevelDefinition::Type(tld)),
                "Expected OCTET STRING top-level declaration",
                GeneratorErrorType::Asn1TypeMismatch,
            ))
        }
    }

    pub(crate) fn generate_boolean(
        &self,
        tld: ToplevelTypeDefinition,
    ) -> Result<String, GeneratorError> {
        if let ASN1Type::Boolean(_) = tld.ty {
            Ok(boolean_template(
                &format_comments(&tld.comments),
                &to_jer_identifier(&tld.name),
            ))
        } else {
            Err(GeneratorError::new(
                Some(ToplevelDefinition::Type(tld)),
                "Expected BOOLEAN top-level declaration",
                GeneratorErrorType::Asn1TypeMismatch,
            ))
        }
    }

    pub(crate) fn generate_value(
        &self,
        tld: ToplevelValueDefinition,
    ) -> Result<String, GeneratorError> {
        value_to_tokens(&tld.value).map(|v| {
            value_template(
                &format_comments(&tld.comments),
                &to_jer_identifier(&tld.name),
                &v,
            )
        })
    }

    pub(crate) fn generate_any(
        &self,
        tld: ToplevelTypeDefinition,
    ) -> Result<String, GeneratorError> {
        Ok(any_template(
            &format_comments(&tld.comments),
            &to_jer_identifier(&tld.name),
        ))
    }

    pub(crate) fn generate_string_like(
        &self,
        tld: ToplevelTypeDefinition,
    ) -> Result<String, GeneratorError> {
        Ok(string_like_template(
            &format_comments(&tld.comments),
            &to_jer_identifier(&tld.name),
        ))
    }

    pub(crate) fn generate_null(
        &self,
        tld: ToplevelTypeDefinition,
    ) -> Result<String, GeneratorError> {
        if let ASN1Type::Null = tld.ty {
            Ok(null_template(
                &format_comments(&tld.comments),
                &to_jer_identifier(&tld.name),
            ))
        } else {
            Err(GeneratorError::new(
                Some(ToplevelDefinition::Type(tld)),
                "Expected NULL top-level declaration",
                GeneratorErrorType::Asn1TypeMismatch,
            ))
        }
    }

    pub(crate) fn generate_enumerated(
        &self,
        tld: ToplevelTypeDefinition,
    ) -> Result<String, GeneratorError> {
        if let ASN1Type::Enumerated(enumerated) = tld.ty {
            Ok(enumerated_template(
                &format_comments(&tld.comments),
                &to_jer_identifier(&tld.name),
                &enumerated
                    .members
                    .into_iter()
                    .fold(String::new(), |mut acc, en| {
                        acc.push_str(&format!(
                            r#"{} = "{}", {}
                        "#,
                            to_jer_identifier(&en.name),
                            en.name,
                            en.description.map_or(String::default(), |d| format!("//{}", d)),
                        ));
                        acc
                    }),
            ))
        } else {
            Err(GeneratorError::new(
                Some(ToplevelDefinition::Type(tld)),
                "Expected ENUMERATED top-level declaration",
                GeneratorErrorType::Asn1TypeMismatch,
            ))
        }
    }

    pub(crate) fn generate_choice(
        &self,
        tld: ToplevelTypeDefinition,
    ) -> Result<String, GeneratorError> {
        if let ASN1Type::Choice(choice) = tld.ty {
            Ok(choice_template(
                &format_comments(&tld.comments),
                &to_jer_identifier(&tld.name),
                &format_choice_options(&choice),
            ))
        } else {
            Err(GeneratorError::new(
                Some(ToplevelDefinition::Type(tld)),
                "Expected CHOICE top-level declaration",
                GeneratorErrorType::Asn1TypeMismatch,
            ))
        }
    }

    pub(crate) fn generate_sequence_or_set(
        &self,
        tld: ToplevelTypeDefinition,
    ) -> Result<String, GeneratorError> {
        match tld.ty {
            ASN1Type::Sequence(ref seq) | ASN1Type::Set(ref seq) => Ok(sequence_or_set_template(
                &format_comments(&tld.comments),
                &to_jer_identifier(&tld.name),
                &format_sequence_or_set_members(seq),
            )),
            _ => Err(GeneratorError::new(
                Some(ToplevelDefinition::Type(tld)),
                "Expected SEQUENCE top-level declaration",
                GeneratorErrorType::Asn1TypeMismatch,
            )),
        }
    }

    pub(crate) fn generate_sequence_or_set_of(
        &self,
        tld: ToplevelTypeDefinition,
    ) -> Result<String, GeneratorError> {
        match tld.ty {
            ASN1Type::SetOf(se_of) | ASN1Type::SequenceOf(se_of) => {
                Ok(sequence_or_set_of_template(
                    &format_comments(&tld.comments),
                    &to_jer_identifier(&tld.name),
                    &type_to_tokens(&se_of.element_type),
                ))
            }
            _ => Err(GeneratorError::new(
                Some(ToplevelDefinition::Type(tld)),
                "Expected SEQUENCE OF top-level declaration",
                GeneratorErrorType::Asn1TypeMismatch,
            )),
        }
    }
}
