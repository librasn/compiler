use num::pow::Pow;

use crate::generator::error::GeneratorError;

use super::{
    types::{BitString, Choice, SequenceOrSet},
    ASN1Type, ASN1Value,
};

const JSON_NULL: &str = "null";

pub fn to_jer_identifier(identifier: &str) -> String {
    identifier.replace('-', "_")
}

pub fn type_to_tokens(ty: &ASN1Type) -> String {
    match ty {
        ASN1Type::Null => String::from("null"),
        ASN1Type::Boolean(_) => String::from("boolean"),
        ASN1Type::Real(_) | ASN1Type::Integer(_) => String::from("number"),
        ASN1Type::BitString(b) if !is_fixed_size(b) => {
            String::from("{ value: string, length: number }")
        }
        ASN1Type::BitString(_)
        | ASN1Type::OctetString(_)
        | ASN1Type::CharacterString(_)
        | ASN1Type::Time(_)
        | ASN1Type::UTCTime(_)
        | ASN1Type::GeneralizedTime(_)
        | ASN1Type::ObjectIdentifier(_) => String::from("string"),
        ASN1Type::Enumerated(e) => e
            .members
            .iter()
            .map(|m| format!(r#""{}""#, m.name))
            .collect::<Vec<_>>()
            .join(" | "),
        ASN1Type::Choice(c) => format_choice_options(c),
        ASN1Type::Set(se) | ASN1Type::Sequence(se) => format_sequence_or_set_members(se),
        ASN1Type::SetOf(s) | ASN1Type::SequenceOf(s) => type_to_tokens(&s.element_type) + "[]",
        ASN1Type::ElsewhereDeclaredType(e) => to_jer_identifier(&e.identifier),
        _ => String::from("any"),
    }
}

pub fn format_choice_options(choice: &Choice) -> String {
    choice
        .options
        .iter()
        .map(|m| {
            format!(
                r#"{{{}: {}}}"#,
                to_jer_identifier(&m.name),
                type_to_tokens(&m.ty)
            )
        })
        .collect::<Vec<_>>()
        .join(" | ")
}

pub fn format_sequence_or_set_members(se: &SequenceOrSet) -> String {
    format!(
        r#"{{
            {}{}
        }}"#,
        se.members
            .iter()
            .map(|m| format!(
                r#"{}: {},"#,
                to_jer_identifier(&m.name),
                type_to_tokens(&m.ty)
            ))
            .collect::<Vec<_>>()
            .join("\n"),
        se.extensible
            .map_or(String::new(), |_| String::from("\n\t[key: string]: any"))
    )
}

pub fn value_to_tokens(value: &ASN1Value) -> Result<String, GeneratorError> {
    match value {
        ASN1Value::Null => Ok(JSON_NULL.to_owned()),
        ASN1Value::Choice {
            type_name: _,
            variant_name,
            inner_value,
        } => value_to_tokens(inner_value).map(|inner| {
            format!(
                r#"{{
                            {variant_name}: {inner}
                        }}"#
            )
        }),
        ASN1Value::OctetString(o) => Ok(o.iter().fold(String::from("\""), |mut acc, byte| {
            acc.push_str(&format!("{byte:02X}"));
            acc
        }) + "\""),
        ASN1Value::SequenceOrSet(_) => Err(GeneratorError {
            details: "Unexpectedly encountered unlinked struct-like ASN1 value!".into(),
            ..Default::default()
        }),
        ASN1Value::LinkedStructLikeValue(fields) => fields
            .iter()
            .try_fold(String::from("{"), |mut acc, (field, _, val)| {
                acc.push_str("\n\t");
                value_to_tokens(val.value()).map(|tokenized| {
                    acc.push_str(&format!("{field}: {tokenized},"));
                    acc
                })
            })
            .map(|mut s| {
                s.push_str("\n}");
                s
            }),
        ASN1Value::Boolean(b) => Ok(String::from(if *b { "true" } else { "false" })),
        ASN1Value::Integer(i) => Ok(i.to_string()),
        ASN1Value::String(s) => Ok(format!(r#""{s}""#)),
        ASN1Value::Real(r) => Ok(r.to_string()),
        ASN1Value::BitString(b) => {
            let value = b.chunks(8).fold(String::new(), |mut value, bits| {
                let mut bits = bits.to_vec();
                bits.resize(8, false);
                bits.reverse();
                let byte = bits.into_iter().enumerate().fold(0, |mut acc, (i, bit)| {
                    if bit {
                        acc += 2.pow(i);
                    }
                    acc
                });
                value.push_str(&format!("{byte:02X?}"));
                value
            });
            let length = b.len();
            Ok(format!(
                r#"{{
                value: "{value}",
                length: {length},
            }}"#
            ))
        }
        ASN1Value::EnumeratedValue {
            enumerated,
            enumerable,
        } => {
            let enum_name = to_jer_identifier(enumerated);
            let enumerable_id = to_jer_identifier(enumerable);
            Ok(format!("{enum_name}.{enumerable_id}"))
        }
        ASN1Value::LinkedElsewhereDefinedValue { identifier: e, .. }
        | ASN1Value::ElsewhereDeclaredValue { identifier: e, .. } => Ok(to_jer_identifier(e)),
        ASN1Value::ObjectIdentifier(oid) => oid
            .0
            .iter()
            .try_fold(String::from("\""), |mut acc, arc| {
                arc.number
                    .ok_or(GeneratorError {
                        details: "Missing Object Identifier arc number.".into(),
                        ..Default::default()
                    })
                    .map(|i| {
                        acc.push_str(&i.to_string());
                        acc.push('.');
                        acc
                    })
            })
            .map(|mut s| {
                s.pop();
                s + "\""
            }),
        ASN1Value::Time(_) => todo!(),
        ASN1Value::LinkedArrayLikeValue(seq) => seq
            .iter()
            .try_fold(String::from("["), |mut acc, v| {
                value_to_tokens(v).map(|v| {
                    acc.push_str(&v);
                    acc.push(',');
                    acc
                })
            })
            .map(|mut s| {
                s.pop();
                s + "]"
            }),
        ASN1Value::LinkedNestedValue {
            supertypes: _,
            value,
        } => value_to_tokens(value),
        ASN1Value::LinkedIntValue {
            integer_type: _,
            value,
        } => Ok(value.to_string()),
        ASN1Value::LinkedCharStringValue(_, value) => Ok(format!(r#""{value}""#)),
        ASN1Value::All => todo!(),
    }
}

pub fn format_comments(comments: &str) -> String {
    if comments.is_empty() {
        String::new()
    } else {
        String::from("//") + &comments.replace('\n', "\n //") + "\n"
    }
}

pub fn is_fixed_size(bit_str: &BitString) -> bool {
    bit_str.constraints.len() == 1
        && bit_str
            .constraints
            .first()
            .unwrap()
            .unpack_as_strict_value()
            .is_ok()
}

#[cfg(test)]
mod tests {
    use crate::intermediate::{
        types::Integer, ASN1Type, IntegerType, ObjectIdentifierArc, ObjectIdentifierValue,
        StructLikeFieldValue,
    };

    use super::*;

    fn no_ws<S: AsRef<str>>(input: S) -> String {
        input
            .as_ref()
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect()
    }

    #[test]
    fn tokenizes_values() {
        assert_eq!(value_to_tokens(&ASN1Value::Null).unwrap(), "null");
        assert_eq!(value_to_tokens(&ASN1Value::Boolean(true)).unwrap(), "true");
        assert_eq!(value_to_tokens(&ASN1Value::Integer(123)).unwrap(), "123");
        assert_eq!(
            value_to_tokens(&ASN1Value::LinkedIntValue {
                integer_type: IntegerType::Int16,
                value: 123
            })
            .unwrap(),
            "123"
        );
        assert_eq!(
            no_ws(value_to_tokens(&ASN1Value::BitString(vec![true, true, true, true])).unwrap()),
            no_ws(r#"{value:"F0",length:4,}"#)
        );
        assert_eq!(
            value_to_tokens(&ASN1Value::OctetString(vec![0, 255, 1, 254])).unwrap(),
            r#""00FF01FE""#
        );
        assert_eq!(value_to_tokens(&ASN1Value::Real(1.2)).unwrap(), r#"1.2"#);
        assert_eq!(
            value_to_tokens(&ASN1Value::ElsewhereDeclaredValue {
                parent: None,
                identifier: "other-Value".into()
            })
            .unwrap(),
            r#"other_Value"#
        );
        assert_eq!(
            value_to_tokens(&ASN1Value::EnumeratedValue {
                enumerated: "OneOfMany".into(),
                enumerable: "options".into()
            })
            .unwrap(),
            r#"OneOfMany.options"#
        );
        assert_eq!(
            no_ws(
                value_to_tokens(&ASN1Value::LinkedStructLikeValue(vec![(
                    String::from("field"),
                    ASN1Type::Integer(Integer::default()),
                    StructLikeFieldValue::Explicit(Box::new(ASN1Value::Integer(42)))
                )]))
                .unwrap()
            ),
            no_ws(r#"{field:42,}"#)
        );
        assert_eq!(
            value_to_tokens(&ASN1Value::ObjectIdentifier(ObjectIdentifierValue(vec![
                ObjectIdentifierArc {
                    number: Some(1),
                    name: None
                },
                ObjectIdentifierArc {
                    number: Some(2),
                    name: None
                },
                ObjectIdentifierArc {
                    number: Some(3),
                    name: None
                }
            ])))
            .unwrap(),
            r#""1.2.3""#
        );
        assert_eq!(
            no_ws(
                value_to_tokens(&ASN1Value::Choice {
                    type_name: None,
                    variant_name: "chosen-option".into(),
                    inner_value: Box::new(ASN1Value::Boolean(false))
                })
                .unwrap()
            ),
            no_ws(r#"{chosen_option:false}"#)
        );
    }
}
