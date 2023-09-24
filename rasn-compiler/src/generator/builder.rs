use crate::intermediate::{
    utils::{to_rust_const_case, to_rust_title_case},
    ASN1Type, ASN1Value, ToplevelDeclaration, ToplevelTypeDeclaration, ToplevelValueDeclaration,
    INTEGER,
};

use super::{
    error::{GeneratorError, GeneratorErrorType},
    generate,
    template::*,
    utils::*,
};

pub fn generate_time_value(tld: ToplevelValueDeclaration) -> Result<String, GeneratorError> {
    if let ASN1Value::Time(_) = &tld.value {
        Ok(time_value_template(
            format_comments(&tld.comments),
            to_rust_const_case(&tld.name),
            tld.type_name.clone(),
            tld.value.value_as_string(Some(&tld.type_name))?,
        ))
    } else {
        Err(GeneratorError::new(
            Some(ToplevelDeclaration::Value(tld)),
            "Expected GeneralizedTime or UTCTime value top-level declaration",
            GeneratorErrorType::Asn1TypeMismatch,
        ))
    }
}

pub fn generate_typealias(tld: ToplevelTypeDeclaration) -> Result<String, GeneratorError> {
    if let ASN1Type::ElsewhereDeclaredType(dec) = &tld.r#type {
        Ok(typealias_template(
            format_comments(&tld.comments),
            to_rust_title_case(&tld.name),
            to_rust_title_case(&dec.identifier),
            format_tag(tld.tag.as_ref(), String::new()),
            format_range_annotations(true, &dec.constraints)?,
        ))
    } else {
        Err(GeneratorError::new(
            Some(ToplevelDeclaration::Type(tld)),
            "Expected type alias top-level declaration",
            GeneratorErrorType::Asn1TypeMismatch,
        ))
    }
}

pub fn generate_integer_value(tld: ToplevelValueDeclaration) -> Result<String, GeneratorError> {
    if let ASN1Value::Integer(i) = tld.value {
        if tld.type_name == INTEGER {
            Ok(integer_value_template(
                format_comments(&tld.comments),
                to_rust_const_case(&tld.name),
                int_type_token(Some(i), Some(i)),
                i.to_string(),
            ))
        } else {
            Ok(integer_value_template(
                format_comments(&tld.comments),
                to_rust_const_case(&tld.name),
                tld.type_name.as_str(),
                format!("{}({})", tld.type_name, i),
            ))
        }
    } else {
        Err(GeneratorError::new(
            Some(ToplevelDeclaration::Value(tld)),
            "Expected INTEGER value top-level declaration",
            GeneratorErrorType::Asn1TypeMismatch,
        ))
    }
}

pub fn generate_integer(tld: ToplevelTypeDeclaration) -> Result<String, GeneratorError> {
    if let ASN1Type::Integer(ref int) = tld.r#type {
        Ok(integer_template(
            format_comments(&tld.comments),
            to_rust_title_case(&tld.name),
            format_range_annotations(true, &int.constraints)?,
            format_tag(tld.tag.as_ref(), String::new()),
            int.type_token(),
        ))
    } else {
        Err(GeneratorError::new(
            Some(ToplevelDeclaration::Type(tld)),
            "Expected INTEGER top-level declaration",
            GeneratorErrorType::Asn1TypeMismatch,
        ))
    }
}

pub fn generate_bit_string(tld: ToplevelTypeDeclaration) -> Result<String, GeneratorError> {
    if let ASN1Type::BitString(ref bitstr) = tld.r#type {
        Ok(bit_string_template(
            format_comments(&tld.comments),
            to_rust_title_case(&tld.name),
            format_range_annotations(true, &bitstr.constraints)?,
            format_tag(tld.tag.as_ref(), String::new()),
        ))
    } else {
        Err(GeneratorError::new(
            Some(ToplevelDeclaration::Type(tld)),
            "Expected BIT STRING top-level declaration",
            GeneratorErrorType::Asn1TypeMismatch,
        ))
    }
}

pub fn generate_octet_string(tld: ToplevelTypeDeclaration) -> Result<String, GeneratorError> {
    if let ASN1Type::OctetString(ref oct_str) = tld.r#type {
        Ok(octet_string_template(
            format_comments(&tld.comments),
            to_rust_title_case(&tld.name),
            format_range_annotations(true, &oct_str.constraints)?,
            format_tag(tld.tag.as_ref(), String::new()),
        ))
    } else {
        Err(GeneratorError::new(
            Some(ToplevelDeclaration::Type(tld)),
            "Expected OCTET STRING top-level declaration",
            GeneratorErrorType::Asn1TypeMismatch,
        ))
    }
}

pub fn generate_character_string(tld: ToplevelTypeDeclaration) -> Result<String, GeneratorError> {
    if let ASN1Type::CharacterString(ref char_str) = tld.r#type {
        Ok(char_string_template(
            format_comments(&tld.comments),
            to_rust_title_case(&tld.name),
            string_type(&char_str.r#type),
            format_range_annotations(false, &char_str.constraints)?,
            format_alphabet_annotations(char_str.r#type, &char_str.constraints)?,
            format_tag(tld.tag.as_ref(), String::new()),
        ))
    } else {
        Err(GeneratorError::new(
            Some(ToplevelDeclaration::Type(tld)),
            "Expected Character String top-level declaration",
            GeneratorErrorType::Asn1TypeMismatch,
        ))
    }
}

pub fn generate_boolean(tld: ToplevelTypeDeclaration) -> Result<String, GeneratorError> {
    if let ASN1Type::Boolean = tld.r#type {
        Ok(boolean_template(
            format_comments(&tld.comments),
            to_rust_title_case(&tld.name),
            format_tag(tld.tag.as_ref(), String::new()),
        ))
    } else {
        Err(GeneratorError::new(
            Some(ToplevelDeclaration::Type(tld)),
            "Expected BOOLEAN top-level declaration",
            GeneratorErrorType::Asn1TypeMismatch,
        ))
    }
}

//     fn generate_typealias(
//         tld: ToplevelTypeDeclaration,
//         custom_derive: Option<&'a str>,
//     ) -> Result<String, GeneratorError> {
//         if let ASN1Type::ElsewhereDeclaredType(dec) = &tld.r#type {
//             Ok(typealias_template(
//                 format_comments(&tld.comments),
//                 custom_derive.unwrap_or(DERIVE_DEFAULT),
//                 rustify_name(&tld.name),
//                 rustify_name(&dec.identifier),
//                 tld.r#type.declare(),
//             ))
//         } else {
//             Err(GeneratorError::new(
//                 Some(ToplevelDeclaration::Type(tld)),
//                 "Expected type alias top-level declaration",
//                 GeneratorErrorType::Asn1TypeMismatch,
//             ))
//         }
//     }

pub fn generate_null_value(tld: ToplevelValueDeclaration) -> Result<String, GeneratorError> {
    if let ASN1Value::Null = tld.value {
        Ok(null_value_template(
            format_comments(&tld.comments),
            to_rust_const_case(&tld.name),
        ))
    } else {
        Err(GeneratorError::new(
            Some(ToplevelDeclaration::Value(tld)),
            "Expected NULL value top-level declaration",
            GeneratorErrorType::Asn1TypeMismatch,
        ))
    }
}

pub fn generate_any(tld: ToplevelTypeDeclaration) -> Result<String, GeneratorError> {
    Ok(any_template(
        format_comments(&tld.comments),
        to_rust_title_case(&tld.name),
        format_tag(tld.tag.as_ref(), String::new()),
    ))
}

pub fn generate_generalized_time(tld: ToplevelTypeDeclaration) -> Result<String, GeneratorError> {
    if let ASN1Type::GeneralizedTime(_) = &tld.r#type {
        Ok(generalized_time_template(
            format_comments(&tld.comments),
            to_rust_title_case(&tld.name),
            format_tag(tld.tag.as_ref(), String::new()),
        ))
    } else {
        Err(GeneratorError::new(
            Some(ToplevelDeclaration::Type(tld)),
            "Expected GeneralizedTime top-level declaration",
            GeneratorErrorType::Asn1TypeMismatch,
        ))
    }
}

pub fn generate_utc_time(tld: ToplevelTypeDeclaration) -> Result<String, GeneratorError> {
    if let ASN1Type::UTCTime(_) = &tld.r#type {
        Ok(utc_time_template(
            format_comments(&tld.comments),
            to_rust_title_case(&tld.name),
            format_tag(tld.tag.as_ref(), String::new()),
        ))
    } else {
        Err(GeneratorError::new(
            Some(ToplevelDeclaration::Type(tld)),
            "Expected UTCTime top-level declaration",
            GeneratorErrorType::Asn1TypeMismatch,
        ))
    }
}

pub fn generate_oid(tld: ToplevelTypeDeclaration) -> Result<String, GeneratorError> {
    if let ASN1Type::ObjectIdentifier(oid) = &tld.r#type {
        Ok(oid_template(
            format_comments(&tld.comments),
            to_rust_title_case(&tld.name),
            format_tag(tld.tag.as_ref(), String::new()),
            format_range_annotations(false, &oid.constraints)?,
        ))
    } else {
        Err(GeneratorError::new(
            Some(ToplevelDeclaration::Type(tld)),
            "Expected OBJECT IDENTIFIER top-level declaration",
            GeneratorErrorType::Asn1TypeMismatch,
        ))
    }
}

pub fn generate_null(tld: ToplevelTypeDeclaration) -> Result<String, GeneratorError> {
    if let ASN1Type::Null = tld.r#type {
        Ok(null_template(
            format_comments(&tld.comments),
            to_rust_title_case(&tld.name),
            format_tag(tld.tag.as_ref(), String::new()),
        ))
    } else {
        Err(GeneratorError::new(
            Some(ToplevelDeclaration::Type(tld)),
            "Expected NULL top-level declaration",
            GeneratorErrorType::Asn1TypeMismatch,
        ))
    }
}

pub fn generate_enumerated(tld: ToplevelTypeDeclaration) -> Result<String, GeneratorError> {
    if let ASN1Type::Enumerated(ref enumerated) = tld.r#type {
        let extensible = if enumerated.extensible.is_some() {
            r#"
                #[non_exhaustive]"#
        } else {
            ""
        };
        Ok(enumerated_template(
            format_comments(&tld.comments),
            to_rust_title_case(&tld.name),
            extensible,
            format_enum_members(enumerated),
            format_tag(tld.tag.as_ref(), String::new()),
        ))
    } else {
        Err(GeneratorError::new(
            Some(ToplevelDeclaration::Type(tld)),
            "Expected ENUMERATED top-level declaration",
            GeneratorErrorType::Asn1TypeMismatch,
        ))
    }
}

pub fn generate_choice(tld: ToplevelTypeDeclaration) -> Result<String, GeneratorError> {
    if let ASN1Type::Choice(ref choice) = tld.r#type {
        let name = to_rust_title_case(&tld.name);
        let inner_options = format_nested_choice_options(&choice, &name)?;
        let extensible = if choice.extensible.is_some() {
            r#"
                #[non_exhaustive]"#
        } else {
            ""
        };

        Ok(choice_template(
            format_comments(&tld.comments),
            name.clone(),
            extensible,
            format_choice_options(&choice, &name)?,
            inner_options,
            format_tag(tld.tag.as_ref(), String::from("automatic_tags")),
        ))
    } else {
        Err(GeneratorError::new(
            Some(ToplevelDeclaration::Type(tld)),
            "Expected CHOICE top-level declaration",
            GeneratorErrorType::Asn1TypeMismatch,
        ))
    }
}

pub fn generate_object_identifier_value(
    tld: ToplevelValueDeclaration,
) -> Result<String, GeneratorError> {
    if let ASN1Value::ObjectIdentifier(_) = tld.value {
        Ok(object_identifier_value_template(
            format_comments(&tld.comments),
            to_rust_const_case(&tld.name),
            tld.value.value_as_string(None)?,
        ))
    } else {
        Err(GeneratorError::new(
            Some(ToplevelDeclaration::Value(tld)),
            "Expected OBJECT IDENTIFIER top-level declaration",
            GeneratorErrorType::Asn1TypeMismatch,
        ))
    }
}

pub fn generate_sequence_or_set(tld: ToplevelTypeDeclaration) -> Result<String, GeneratorError> {
    match tld.r#type {
        ASN1Type::Sequence(ref seq) | ASN1Type::Set(ref seq) => {
            let name = to_rust_title_case(&tld.name);
            let extensible = if seq.extensible.is_some() {
                r#"
                #[non_exhaustive]"#
            } else {
                ""
            };
            let set_annotation = if let ASN1Type::Set(_) = tld.r#type {
                "set"
            } else {
                ""
            };
            let (declaration, name_types) = format_sequence_or_set_members(seq, &name)?;
            Ok(sequence_or_set_template(
                format_comments(&tld.comments),
                name.clone(),
                extensible,
                declaration,
                format_nested_sequence_members(seq, &name)?,
                format_tag(tld.tag.as_ref(), String::from("automatic_tags")),
                set_annotation.into(),
                format_default_methods(&seq.members, &name)?,
                format_new_impl(&name, name_types),
            ))
        }
        _ => Err(GeneratorError::new(
            Some(ToplevelDeclaration::Type(tld)),
            "Expected SEQUENCE top-level declaration",
            GeneratorErrorType::Asn1TypeMismatch,
        )),
    }
}

pub fn generate_sequence_of(tld: ToplevelTypeDeclaration) -> Result<String, GeneratorError> {
    if let ASN1Type::SequenceOf(ref seq_of) = tld.r#type {
        let name = to_rust_title_case(&tld.name);
        let anonymous_item = match seq_of.r#type.as_ref() {
            ASN1Type::ElsewhereDeclaredType(_) => None,
            n => Some(generate(ToplevelDeclaration::Type(
                ToplevelTypeDeclaration {
                    parameterization: None,
                    comments: " Anonymous SEQUENCE OF member ".into(),
                    name: String::from("Anonymous") + &name,
                    r#type: n.clone(),
                    tag: None,
                    index: None
                },
            ))?),
        }
        .unwrap_or_default();
        let member_type = match seq_of.r#type.as_ref() {
            ASN1Type::ElsewhereDeclaredType(d) => to_rust_title_case(&d.identifier),
            _ => String::from("Anonymous") + &name,
        };
        Ok(sequence_of_template(
            format_comments(&tld.comments),
            name,
            anonymous_item,
            member_type,
            format_range_annotations(true, &seq_of.constraints)?,
            format_tag(tld.tag.as_ref(), String::new()),
        ))
    } else {
        Err(GeneratorError::new(
            Some(ToplevelDeclaration::Type(tld)),
            "Expected SEQUENCE OF top-level declaration",
            GeneratorErrorType::Asn1TypeMismatch,
        ))
    }
}

//     fn generate_information_object_set(
//         tld: ToplevelInformationDeclaration,
//     ) -> Result<String, GeneratorError> {
//         if let ASN1Information::ObjectSet(o) = &tld.value {
//             let class: &InformationObjectClass = match tld.class {
//                 Some(ClassLink::ByReference(ref c)) => c,
//                 _ => {
//                     return Err(GeneratorError::new(
//                         None,
//                         "Missing class link in Information Object Set",
//                         GeneratorErrorType::MissingClassLink,
//                     ))
//                 }
//             };
//             let keys_to_types = o
//                 .values
//                 .iter()
//                 .map(|v| {
//                     match v {
//                         ObjectSetValue::Reference(_) => todo!(),
//                         // basically, an information object specifies a sequence implementing a class. So we sould treat information objects like sequences
//                         ObjectSetValue::Inline(InformationObjectFields::CustomSyntax(s)) => {
//                             resolve_syntax(class, s)
//                         }
//                         ObjectSetValue::Inline(InformationObjectFields::DefaultSyntax(_s)) => {
//                             todo!()
//                         }
//                     }
//                 })
//                 .collect::<Result<Vec<(ASN1Value, Vec<ASN1Type>)>, GeneratorError>>()?;
//             let mut options = keys_to_types
//                 .iter()
//                 .map(|(k, types)| {
//                     format!(
//                         "_{}({})",
//                         k.to_string(),
//                         types
//                             .iter()
//                             .map(|t| format!("pub {}", t.to_string()))
//                             .collect::<Vec<String>>()
//                             .join(", ")
//                     )
//                 })
//                 .collect::<Vec<String>>()
//                 .join(",\n\t");
//             if o.extensible.is_some() {
//                 options.push_str(",\n\tUnknownClassImplementation(pub Vec<u8>)");
//             }
//             let key_type = match class
//                 .fields
//                 .iter()
//                 .find_map(|f| {
//                     f.is_unique
//                         .then(|| f.r#type.as_ref().map(|t| t.to_string()))
//                 })
//                 .flatten()
//             {
//                 Some(key_type) => key_type,
//                 None => {
//                     return Err(GeneratorError::new(
//                         None,
//                         "Could not determine class key type!",
//                         GeneratorErrorType::MissingClassKey,
//                     ))
//                 }
//             };
//             let mut branches = keys_to_types
//                 .iter()
//                 .map(|(k, _)| format!("{} => todo!()", k.to_string(),))
//                 .collect::<Vec<String>>()
//                 .join(",\n\t");
//             if o.extensible.is_some() {
//                 branches.push_str(",\n\t_ => todo!()");
//             }
//             Ok(information_object_set_template(
//                 format_comments(&tld.comments),
//                 rustify_name(&tld.name),
//                 options,
//                 key_type,
//                 branches,
//             ))
//         } else {
//             Err(GeneratorError::new(
//                 Some(ToplevelDeclaration::Information(tld)),
//                 "Expected Object Set top-level declaration",
//                 GeneratorErrorType::Asn1TypeMismatch,
//             ))
//         }
//     }
