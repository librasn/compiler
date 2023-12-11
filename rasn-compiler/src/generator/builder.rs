use std::{collections::BTreeMap, ops::Not};

use proc_macro2::TokenStream;
use quote::quote;

use crate::intermediate::{
    information_object::{
        ASN1Information, ClassLink, InformationObjectClass, InformationObjectFields,
        ObjectSetValue, ToplevelInformationDeclaration,
    },
    utils::{to_rust_const_case, to_rust_title_case, to_rust_snake_case},
    ASN1Type, ASN1Value, ToplevelDeclaration, ToplevelTypeDeclaration, ToplevelValueDeclaration,
    INTEGER, constraints::Constraint,
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
                "i64",
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
            string_type(&char_str.r#type)?,
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
    // TODO: process boolean constraints
    if let ASN1Type::Boolean(_) = tld.r#type {
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

pub fn generate_choice_value(tld: ToplevelValueDeclaration) -> Result<String, GeneratorError> {
    if let ASN1Value::Choice(ref choice, inner) = tld.value {
        let name = to_rust_const_case(&tld.name).parse::<TokenStream>().unwrap();
        let type_id = tld.type_name.parse::<TokenStream>().unwrap();
        let choice_name = to_rust_title_case(choice).parse::<TokenStream>().unwrap();
        let inner_decl = inner.value_as_string(None)?.parse::<TokenStream>().unwrap();
        Ok(quote! {
            const #name: #type_id = #type_id :: #choice_name (#inner_decl);
        }.to_string())
    } else {
        Err(GeneratorError::new(
            Some(ToplevelDeclaration::Value(tld)),
            "Expected CHOICE top-level declaration",
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
            let name = to_rust_title_case(&tld.name).parse::<TokenStream>().unwrap();
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
            let class_fields = seq.members.iter().fold(String::new(), |mut acc, m| { [m.constraints.clone(), m.r#type.constraints()].concat().iter().for_each(|c| {
                let decode_fn = format!("decode_{}", to_rust_snake_case(&m.name)).parse::<TokenStream>().unwrap();
                let open_field_name = to_rust_snake_case(&m.name).parse::<TokenStream>().unwrap();
                match (c, &m.r#type) {
                    (Constraint::TableConstraint(t), ASN1Type::InformationObjectFieldReference(iofr)) => {
                        let identifier = t.linked_fields.iter().map(|l| to_rust_snake_case(&l.field_name)).collect::<Vec<String>>().join(".").parse::<TokenStream>().unwrap();
                        let field_name = iofr.field_path.last().unwrap().identifier().replace("&", "");
                        let obj_set_name = match t.object_set.values.first() {
                            Some(ObjectSetValue::Reference(s)) => s,
                            _ => todo!()
                        };
                        let field_enum_name = format!("{obj_set_name}_{field_name}").parse::<TokenStream>().unwrap();
                        let input = m.is_optional.then(|| quote!(self. #open_field_name .as_ref())).unwrap_or(quote!(Some(&self. #open_field_name)));
                        acc += &quote! {

                            impl #name {
                                pub fn #decode_fn<D: Decoder>(&self, decoder: &mut D) -> Result<#field_enum_name, D::Error> {
                                    #field_enum_name ::decode(decoder, #input, &self. #identifier)
                                }
                            }
                        }.to_string()
                    },
                    _ => ()
                };
            });
            acc
            });
            let (declaration, name_types) = format_sequence_or_set_members(seq, &name.to_string())?;
            Ok(sequence_or_set_template(
                format_comments(&tld.comments),
                name.to_string(),
                extensible,
                declaration,
                format_nested_sequence_members(seq, &name.to_string())? + &class_fields,
                format_tag(tld.tag.as_ref(), String::from("automatic_tags")),
                set_annotation.into(),
                format_default_methods(&seq.members, &name.to_string())?,
                format_new_impl(&name.to_string(), name_types),
            ))
        }
        _ => Err(GeneratorError::new(
            Some(ToplevelDeclaration::Type(tld)),
            "Expected SEQUENCE top-level declaration",
            GeneratorErrorType::Asn1TypeMismatch,
        )),
    }
}

pub fn generate_sequence_or_set_of(tld: ToplevelTypeDeclaration) -> Result<String, GeneratorError> {
    let (is_set_of, seq_or_set_of) = match &tld.r#type {
        ASN1Type::SetOf(se_of) => (true, se_of),
        ASN1Type::SequenceOf(se_of) => (false, se_of),
        _ => {
            return Err(GeneratorError::new(
                Some(ToplevelDeclaration::Type(tld)),
                "Expected SEQUENCE OF top-level declaration",
                GeneratorErrorType::Asn1TypeMismatch,
            ))
        }
    };
    let name = to_rust_title_case(&tld.name);
    let anonymous_item = match seq_or_set_of.r#type.as_ref() {
        ASN1Type::ElsewhereDeclaredType(_) => None,
        n => Some(generate(ToplevelDeclaration::Type(
            ToplevelTypeDeclaration {
                parameterization: None,
                comments: format!(
                    " Anonymous {} OF member ",
                    if is_set_of { "SET" } else { "SEQUENCE" }
                ),
                name: String::from("Anonymous") + &name,
                r#type: n.clone(),
                tag: None,
                index: None,
            },
        ))?),
    }
    .unwrap_or_default();
    let member_type = match seq_or_set_of.r#type.as_ref() {
        ASN1Type::ElsewhereDeclaredType(d) => to_rust_title_case(&d.identifier),
        _ => String::from("Anonymous") + &name,
    };
    Ok(sequence_or_set_of_template(
        is_set_of,
        format_comments(&tld.comments),
        name,
        anonymous_item,
        member_type,
        format_range_annotations(true, &seq_or_set_of.constraints)?,
        format_tag(tld.tag.as_ref(), String::new()),
    ))
}

pub fn generate_information_object_set(
    tld: ToplevelInformationDeclaration,
) -> Result<String, GeneratorError> {
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
            .map(|v| {
                match v {
                    ObjectSetValue::Reference(_) => todo!(),
                    // basically, an information object specifies a sequence implementing a class. So we sould treat information objects like sequences
                    ObjectSetValue::Inline(InformationObjectFields::CustomSyntax(s)) => {
                        resolve_syntax(class, s)
                    }
                    ObjectSetValue::Inline(InformationObjectFields::DefaultSyntax(_s)) => {
                        todo!()
                    }
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
                        top_level_declaration: Some(ToplevelDeclaration::Information(tld.clone())),
                        details: "Could not find class field for index.".into(),
                        kind: GeneratorErrorType::SyntaxMismatch,
                    })?;
                match choices.get_mut(&id) {
                    Some(entry) => entry.push((key.clone(), item)),
                    None => {
                        choices.insert(id, vec![(key.clone(), item)]);
                    }
                }
            }
        }

        let name = tld.name;
        let class_unique_id_type = class
            .fields
            .iter()
            .find_map(|f| (f.is_unique).then(|| f.r#type.clone()))
            .flatten()
            .ok_or_else(|| GeneratorError {
                top_level_declaration: None,
                details: "Could not determine unique class identifier type.".into(),
                kind: GeneratorErrorType::SyntaxMismatch,
            })?;
        let class_unique_id_type_name = class_unique_id_type.as_string()?.parse::<TokenStream>().unwrap();

        let field_enums = choices.iter().map(|(field_name, fields)| {
            let field_enum_name = format!("{name}_{field_name}").replace("&", "").parse::<TokenStream>().unwrap();
            let (ids, inner_types): (Vec<(TokenStream, TokenStream, TokenStream)>, Vec<TokenStream>) = fields.iter().enumerate().map(|(index, (id, ty))| {
                let type_id = ty.as_string().unwrap_or(String::from("Option<()>")).parse::<TokenStream>().unwrap();
                let identifier_value = id.value_as_string(Some(&class_unique_id_type_name.to_string())).unwrap().parse::<TokenStream>().unwrap();
                let variant_name = if let ASN1Value::ElsewhereDeclaredValue(ref_id) = id {
                    to_rust_title_case(ref_id)
                } else {
                    format!("{field_enum_name}_{index}")
                }.parse::<TokenStream>().unwrap();
                if ty.constraints().is_empty() {
                    ((variant_name, type_id, identifier_value), quote!())
                } else {
                    let (signed_range, character_string_type) = match ty {
                        ASN1Type::CharacterString(c) => (false, Some(c.r#type)),
                        ASN1Type::Integer(_) => (true, None),
                        ASN1Type::Real(_) => (true, None),
                        ASN1Type::BitString(_) => (false, None),
                        ASN1Type::OctetString(_) => (false, None),
                        _ => (false, None)
                        
                    };
                    let delegate_id = &format!("Inner_{field_enum_name}_{index}").parse::<TokenStream>().unwrap();
                    let range_constraints = format_range_annotations(signed_range, &ty.constraints()).unwrap();
                    let alphabet_constraints = character_string_type.map(|c| format_alphabet_annotations(c, &ty.constraints()).ok()).flatten().unwrap_or_default();
                    let annotations = [range_constraints, alphabet_constraints, String::from("delegate")].into_iter().filter(|ann| ann.is_empty().not()).collect::<Vec<String>>().join(",").parse::<TokenStream>().unwrap();
                    ((variant_name, delegate_id.clone(), identifier_value), quote!{
                        #[derive(Debug, Clone, PartialEq, AsnType, Decode, Encode)]
                        #[rasn(#annotations)]
                        pub struct #delegate_id (pub #type_id);

                    })
                }
            }).unzip();
            
            let variants = ids.iter().map(|(variant_name, type_id, _)| {
                quote!(#variant_name (#type_id),)
            });

            let de_match_arms = ids.iter().map(|(variant_name, _, identifier_value)| {
                quote!(& #identifier_value => Ok(decoder.codec().decode_from_binary(open_type_payload.ok_or_else(|| rasn::error::DecodeError::from_kind(
                    rasn::error::DecodeErrorKind::Custom {
                        msg: alloc::format!("Failed to decode open type! No input data given."),
                    },
                    decoder.codec()
                ).into())?.as_bytes()).map(Self:: #variant_name)?),)
            });

            let en_match_arms = ids.iter().map(|(variant_name, _, identifier_value)| {
                quote!((Self::#variant_name (inner), & #identifier_value) =>inner.encode(encoder),)
            });

            quote! {
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

            }
        });

        Ok(quote!(#(#field_enums)*).to_string())
    } else {
        Err(GeneratorError::new(
            Some(ToplevelDeclaration::Information(tld)),
            "Expected Object Set top-level declaration",
            GeneratorErrorType::Asn1TypeMismatch,
        ))
    }
}
