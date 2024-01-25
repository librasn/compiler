use std::collections::BTreeMap;

use crate::intermediate::{
    error::{GrammarError, GrammarErrorType},
    information_object::*,
    *,
};

use self::types::*;

pub(crate) fn find_tld_or_enum_value_by_name(
    type_name: &String,
    name: &String,
    tlds: &BTreeMap<String, ToplevelDeclaration>,
) -> Option<ASN1Value> {
    if let Some(ToplevelDeclaration::Value(v)) = tlds.get(name) {
        return Some(v.value.clone());
    } else {
        for (_, tld) in tlds.iter() {
            if let Some(value) = tld.get_distinguished_or_enum_value(Some(type_name), name) {
                return Some(value);
            }
        }
        // Make second attempt without requiring a matching type name
        // This is the current best shot at linking inner subtypes
        for (_, tld) in tlds.iter() {
            if let Some(value) = tld.get_distinguished_or_enum_value(None, name) {
                return Some(value);
            }
        }
    }
    None
}

pub(crate) fn octet_string_to_bit_string(bytes: &[u8]) -> Vec<bool> {
    let mut bits = vec![];
    for byte in bytes {
        is_bit_set(*byte, 128, &mut bits);
    }
    bits
}

fn is_bit_set(rem: u8, limit: u8, bits: &mut Vec<bool>) {
    bits.push(rem >= limit);
    if limit >= 2 {
        is_bit_set(rem % limit, limit / 2, bits)
    }
}

pub(crate) fn bit_string_to_octet_string(bits: &[bool]) -> Result<Vec<u8>, GrammarError> {
    let mut octets = vec![];
    for byte in bits.chunks(8) {
        if byte.len() != 8 {
            return Err(GrammarError {
                details: "Binary octet string value needs to be a multiple of 8 bits!".into(),
                kind: GrammarErrorType::LinkerError,
            });
        }
        octets.push(byte.iter().enumerate().fold(0u8, |acc, (i, bit)| {
            acc + if *bit { 2u8.pow(7 - i as u32) } else { 0 }
        }));
    }
    Ok(octets)
}

pub(crate) fn walk_object_field_ref_path<'a>(
    fields: &'a Vec<InformationObjectClassField>,
    path: &'a Vec<ObjectFieldIdentifier>,
    mut index: usize,
) -> Option<&'a InformationObjectClassField> {
    fields
        .iter()
        .find_map(|f| {
            path.get(index).and_then(|id| {
                (&f.identifier == id).then(|| {
                    if path.len() == (index + 1) {
                        Some(f)
                    } else {
                        index += 1;
                        walk_object_field_ref_path(fields, path, index)
                    }
                })
            })
        })
        .flatten()
}

/// Resolves the custom syntax declared in an information object class' WITH SYNTAX clause
pub fn resolve_custom_syntax(
    fields: &mut InformationObjectFields,
    class: &InformationObjectClass,
) -> Result<(), GrammarError> {
    if let InformationObjectFields::CustomSyntax(application) = fields {
        let tokens = match &class.syntax {
            Some(s) => s.flatten(),
            None => {
                return Err(GrammarError {
                    details: "No syntax definition for information object class found!".into(),
                    kind: GrammarErrorType::LinkerError,
                })
            }
        };

        let mut unsorted_default_syntax = Vec::<(usize, InformationObjectField)>::new();

        let mut application_index = 0;
        'syntax_matching: for (required, token) in &tokens {
            if let Some(expr) = application.get(application_index) {
                if expr.matches(token, &tokens) {
                    match expr {
                        SyntaxApplication::ObjectSetDeclaration(o) => {
                            if let Some(index) =
                                class.fields.iter().enumerate().find_map(|(i, v)| {
                                    (v.identifier
                                        == ObjectFieldIdentifier::MultipleValue(
                                            token.name_or_empty().to_owned(),
                                        ))
                                    .then_some(i)
                                })
                            {
                                unsorted_default_syntax.push((
                                    index,
                                    InformationObjectField::ObjectSetField(ObjectSetField {
                                        identifier: token.name_or_empty().to_owned(),
                                        value: o.clone(),
                                    }),
                                ));
                            }
                        }
                        SyntaxApplication::LiteralOrTypeReference(t) => {
                            if let Some(index) =
                                class.fields.iter().enumerate().find_map(|(i, v)| {
                                    (v.identifier
                                        == ObjectFieldIdentifier::MultipleValue(
                                            token.name_or_empty().to_owned(),
                                        ))
                                    .then_some(i)
                                })
                            {
                                unsorted_default_syntax.push((
                                    index,
                                    InformationObjectField::TypeField(TypeField {
                                        identifier: token.name_or_empty().to_owned(),
                                        r#type: ASN1Type::ElsewhereDeclaredType(t.clone()),
                                    }),
                                ));
                            }
                        }
                        SyntaxApplication::TypeReference(t) => {
                            if let Some(index) =
                                class.fields.iter().enumerate().find_map(|(i, v)| {
                                    (v.identifier
                                        == ObjectFieldIdentifier::MultipleValue(
                                            token.name_or_empty().to_owned(),
                                        ))
                                    .then_some(i)
                                })
                            {
                                unsorted_default_syntax.push((
                                    index,
                                    InformationObjectField::TypeField(TypeField {
                                        identifier: token.name_or_empty().to_owned(),
                                        r#type: t.clone(),
                                    }),
                                ));
                            }
                        }
                        SyntaxApplication::ValueReference(v) => {
                            if let Some(index) =
                                class.fields.iter().enumerate().find_map(|(i, v)| {
                                    (v.identifier
                                        == ObjectFieldIdentifier::SingleValue(
                                            token.name_or_empty().to_owned(),
                                        ))
                                    .then_some(i)
                                })
                            {
                                unsorted_default_syntax.push((
                                    index,
                                    InformationObjectField::FixedValueField(FixedValueField {
                                        identifier: token.name_or_empty().to_owned(),
                                        value: v.clone(),
                                    }),
                                ));
                            }
                        }
                        _ => continue 'syntax_matching,
                    }
                    application_index += 1;
                } else if *required {
                    return Err(GrammarError {
                        details: "Syntax mismatch while resolving information object.".to_string(),
                        kind: GrammarErrorType::LinkerError,
                    });
                } else {
                    continue 'syntax_matching;
                }
            } else if *required {
                return Err(GrammarError {
                    details: "Syntax mismatch while resolving information object.".to_string(),
                    kind: GrammarErrorType::LinkerError,
                });
            } else {
                continue 'syntax_matching;
            }
        }
        unsorted_default_syntax.sort_by(|&(a, _), &(b, _)| a.cmp(&b));
        *fields = InformationObjectFields::DefaultSyntax(
            unsorted_default_syntax
                .into_iter()
                .map(|(_, field)| field)
                .collect(),
        );
    }
    Ok(())
}

pub(crate) fn built_in_type(associated_type: &str) -> Option<ASN1Type> {
    match associated_type {
        INTEGER => Some(ASN1Type::Integer(Integer {
            constraints: vec![],
            distinguished_values: None,
        })),
        BIT_STRING => Some(ASN1Type::BitString(BitString {
            constraints: vec![],
            distinguished_values: None,
        })),
        OCTET_STRING => Some(ASN1Type::OctetString(OctetString {
            constraints: vec![],
        })),
        GENERALIZED_TIME => Some(ASN1Type::GeneralizedTime(GeneralizedTime {
            constraints: vec![],
        })),
        UTC_TIME => Some(ASN1Type::UTCTime(UTCTime {
            constraints: vec![],
        })),
        BOOLEAN => Some(ASN1Type::Boolean(Boolean {
            constraints: vec![],
        })),
        OBJECT_IDENTIFIER => Some(ASN1Type::ObjectIdentifier(ObjectIdentifier {
            constraints: vec![],
        })),
        ty if ty.contains(SEQUENCE_OF) => {
            let identifier = ty.replace(SEQUENCE_OF, "").trim().to_string();
            Some(ASN1Type::SequenceOf(SequenceOrSetOf {
                constraints: vec![],
                r#type: Box::new(built_in_type(&identifier).unwrap_or(
                    ASN1Type::ElsewhereDeclaredType(DeclarationElsewhere {
                        parent: None,
                        identifier,
                        constraints: vec![],
                    }),
                )),
            }))
        }
        ty if ty.contains(SET_OF) => {
            let identifier = ty.replace(SET_OF, "").trim().to_string();
            Some(ASN1Type::SetOf(SequenceOrSetOf {
                constraints: vec![],
                r#type: Box::new(built_in_type(&identifier).unwrap_or(
                    ASN1Type::ElsewhereDeclaredType(DeclarationElsewhere {
                        parent: None,
                        identifier,
                        constraints: vec![],
                    }),
                )),
            }))
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use crate::validator::linking::utils::octet_string_to_bit_string;

    #[test]
    fn converts_octet_to_bit_string() {
        assert_eq!(
            octet_string_to_bit_string(&[76]),
            vec![false, true, false, false, true, true, false, false]
        );
        assert_eq!(
            octet_string_to_bit_string(&[129]),
            vec![true, false, false, false, false, false, false, true]
        );
        assert_eq!(
            octet_string_to_bit_string(&[128]),
            vec![true, false, false, false, false, false, false, false]
        );
        assert_eq!(
            octet_string_to_bit_string(&[0]),
            vec![false, false, false, false, false, false, false, false]
        );
        assert_eq!(
            octet_string_to_bit_string(&[32]),
            vec![false, false, true, false, false, false, false, false]
        );
        assert_eq!(
            octet_string_to_bit_string(&[89, 45]),
            vec![
                false, true, false, true, true, false, false, true, false, false, true, false,
                true, true, false, true
            ]
        );
    }
}
