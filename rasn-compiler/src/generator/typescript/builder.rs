use crate::intermediate::{ASN1Type, TypeAssignment, ValueAssignment};

use super::{template::*, utils::*, Typescript};
use crate::generator::error::{GeneratorError, GeneratorErrorType};

impl Typescript {
    pub(crate) fn generate_typealias<'a>(
        &self,
        tld: TypeAssignment<'a>,
    ) -> Result<String, GeneratorError> {
        if let ASN1Type::ElsewhereDeclaredType(dec) = &tld.ty {
            Ok(typealias_template(
                &format_comments(&tld.comments),
                &to_jer_identifier(&tld.name),
                &to_jer_reference(&dec.identifier),
            ))
        } else {
            Err(GeneratorError::new(
                Some(tld.name.to_string()),
                "Expected type alias top-level declaration",
                GeneratorErrorType::Asn1TypeMismatch,
            ))
        }
    }

    pub(crate) fn generate_number_like<'a>(
        &self,
        tld: TypeAssignment<'a>,
    ) -> Result<String, GeneratorError> {
        if let ASN1Type::Integer(_) = tld.ty {
            Ok(number_like_template(
                &format_comments(&tld.comments),
                &to_jer_identifier(&tld.name),
            ))
        } else {
            Err(GeneratorError::new(
                Some(tld.name.to_string()),
                "Expected INTEGER top-level declaration",
                GeneratorErrorType::Asn1TypeMismatch,
            ))
        }
    }

    pub(crate) fn generate_bit_string<'a>(
        &self,
        tld: TypeAssignment<'a>,
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
                Some(tld.name.to_string()),
                "Expected BIT STRING top-level declaration",
                GeneratorErrorType::Asn1TypeMismatch,
            ))
        }
    }

    pub(crate) fn generate_octet_string<'a>(
        &self,
        tld: TypeAssignment<'a>,
    ) -> Result<String, GeneratorError> {
        if let ASN1Type::OctetString(_) = tld.ty {
            Ok(octet_string_template(
                &format_comments(&tld.comments),
                &to_jer_identifier(&tld.name),
            ))
        } else {
            Err(GeneratorError::new(
                Some(tld.name.to_string()),
                "Expected OCTET STRING top-level declaration",
                GeneratorErrorType::Asn1TypeMismatch,
            ))
        }
    }

    pub(crate) fn generate_boolean<'a>(
        &self,
        tld: TypeAssignment<'a>,
    ) -> Result<String, GeneratorError> {
        if let ASN1Type::Boolean(_) = tld.ty {
            Ok(boolean_template(
                &format_comments(&tld.comments),
                &to_jer_identifier(&tld.name),
            ))
        } else {
            Err(GeneratorError::new(
                Some(tld.name.to_string()),
                "Expected BOOLEAN top-level declaration",
                GeneratorErrorType::Asn1TypeMismatch,
            ))
        }
    }

    pub(crate) fn generate_value<'a>(&self, tld: ValueAssignment) -> String {
        //Result<String, GeneratorError> {
        value_to_tokens(&tld.value)
            .map(|v| {
                value_template(
                    &format_comments(&tld.comments),
                    &to_jer_identifier(&tld.name),
                    &v,
                )
            })
            .unwrap() // TODO: Do not unwrap
    }

    pub(crate) fn generate_any<'a>(
        &self,
        tld: TypeAssignment<'a>,
    ) -> Result<String, GeneratorError> {
        Ok(any_template(
            &format_comments(&tld.comments),
            &to_jer_identifier(&tld.name),
        ))
    }

    pub(crate) fn generate_string_like<'a>(
        &self,
        tld: TypeAssignment<'a>,
    ) -> Result<String, GeneratorError> {
        Ok(string_like_template(
            &format_comments(&tld.comments),
            &to_jer_identifier(&tld.name),
        ))
    }

    pub(crate) fn generate_null<'a>(
        &self,
        tld: TypeAssignment<'a>,
    ) -> Result<String, GeneratorError> {
        if let ASN1Type::Null = tld.ty {
            Ok(null_template(
                &format_comments(&tld.comments),
                &to_jer_identifier(&tld.name),
            ))
        } else {
            Err(GeneratorError::new(
                Some(tld.name.to_string()),
                "Expected NULL top-level declaration",
                GeneratorErrorType::Asn1TypeMismatch,
            ))
        }
    }

    pub(crate) fn generate_enumerated<'a>(
        &self,
        tld: TypeAssignment<'a>,
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
                            en.description
                                .map_or(String::default(), |d| format!("//{d}")),
                        ));
                        acc
                    }),
            ))
        } else {
            Err(GeneratorError::new(
                Some(tld.name.to_string()),
                "Expected ENUMERATED top-level declaration",
                GeneratorErrorType::Asn1TypeMismatch,
            ))
        }
    }

    pub(crate) fn generate_choice<'a>(
        &self,
        tld: TypeAssignment<'a>,
    ) -> Result<String, GeneratorError> {
        if let ASN1Type::Choice(choice) = tld.ty {
            Ok(choice_template(
                &format_comments(&tld.comments),
                &to_jer_identifier(&tld.name),
                &format_choice_options(&choice),
            ))
        } else {
            Err(GeneratorError::new(
                Some(tld.name.to_string()),
                "Expected CHOICE top-level declaration",
                GeneratorErrorType::Asn1TypeMismatch,
            ))
        }
    }

    pub(crate) fn generate_sequence_or_set<'a>(
        &self,
        tld: TypeAssignment<'a>,
    ) -> Result<String, GeneratorError> {
        match tld.ty {
            ASN1Type::Sequence(ref seq) | ASN1Type::Set(ref seq) => Ok(sequence_or_set_template(
                &format_comments(&tld.comments),
                &to_jer_identifier(&tld.name),
                &format_sequence_or_set_members(seq),
            )),
            _ => Err(GeneratorError::new(
                Some(tld.name.to_string()),
                "Expected SEQUENCE top-level declaration",
                GeneratorErrorType::Asn1TypeMismatch,
            )),
        }
    }

    pub(crate) fn generate_sequence_or_set_of<'a>(
        &self,
        tld: TypeAssignment<'a>,
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
                Some(tld.name.to_string()),
                "Expected SEQUENCE OF top-level declaration",
                GeneratorErrorType::Asn1TypeMismatch,
            )),
        }
    }
}
