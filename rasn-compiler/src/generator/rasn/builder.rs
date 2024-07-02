use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens, TokenStreamExt};
use std::collections::BTreeMap;

use crate::intermediate::{
    constraints::Constraint,
    information_object::{
        ASN1Information, ClassLink, InformationObjectClass, InformationObjectFields,
        ObjectSetValue, ToplevelInformationDefinition,
    },
    ASN1Type, ASN1Value, ToplevelDefinition, ToplevelTypeDefinition, ToplevelValueDefinition,
    BIT_STRING, BOOLEAN, GENERALIZED_TIME, INTEGER, NULL, OCTET_STRING, UTC_TIME,
};

use super::{
    information_object::InformationObjectClassField, template::*, Rasn, BMP_STRING, GENERAL_STRING,
    IA5_STRING, NUMERIC_STRING, OBJECT_IDENTIFIER, PRINTABLE_STRING, SEQUENCE_OF, SET_OF,
    UTF8_STRING, VISIBLE_STRING,
};
use crate::generator::error::{GeneratorError, GeneratorErrorType};

pub(crate) const INNER_ARRAY_LIKE_PREFIX: &str = "Anonymous_";

macro_rules! call_template {
    ($this:ident, $fn:ident, $tld:ident, $($args:expr),*) => {
        Ok($fn(
            $this.format_comments(&$tld.comments)?,
            $this.to_rust_const_case(&$tld.name),
            $($args),*
        ))
    };
}

macro_rules! assignment {
    ($this:ident, $unformatted:expr, $inner:expr) => {{
        let ty = $this.to_rust_title_case($unformatted);
        let inner = $inner;
        quote!(#ty(#inner))
    }};
}

impl Rasn {
    pub(crate) fn generate_tld(
        &self,
        tld: ToplevelDefinition,
    ) -> Result<TokenStream, GeneratorError> {
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

    pub(crate) fn generate_typealias(
        &self,
        tld: ToplevelTypeDefinition,
    ) -> Result<TokenStream, GeneratorError> {
        if let ASN1Type::ElsewhereDeclaredType(dec) = &tld.ty {
            let name = self.to_rust_title_case(&tld.name);
            let mut annotations = vec![
                quote!(delegate),
                self.format_tag(tld.tag.as_ref(), false),
                self.format_range_annotations(true, &dec.constraints)?,
            ];
            if name.to_string() != tld.name {
                annotations.push(self.format_identifier_annotation(
                    &tld.name,
                    &tld.comments,
                    &tld.ty,
                ));
            }
            Ok(typealias_template(
                self.format_comments(&tld.comments)?,
                name,
                self.to_rust_title_case(&dec.identifier),
                self.join_annotations(annotations),
            ))
        } else {
            Err(GeneratorError::new(
                Some(ToplevelDefinition::Type(tld)),
                "Expected type alias top-level declaration",
                GeneratorErrorType::Asn1TypeMismatch,
            ))
        }
    }

    pub(crate) fn generate_integer_value(
        &self,
        tld: ToplevelValueDefinition,
    ) -> Result<TokenStream, GeneratorError> {
        if let ASN1Value::LinkedIntValue { integer_type, .. } = tld.value {
            let formatted_value = self.value_to_tokens(&tld.value, None)?;
            let ty = self.to_rust_title_case(&tld.associated_type.as_str());
            if tld.associated_type.as_str() == INTEGER {
                Ok(lazy_static_value_template(
                    self.format_comments(&tld.comments)?,
                    self.to_rust_const_case(&tld.name),
                    integer_type.to_token_stream(),
                    formatted_value,
                ))
            } else if integer_type.is_unbounded() {
                Ok(lazy_static_value_template(
                    self.format_comments(&tld.comments)?,
                    self.to_rust_const_case(&tld.name),
                    ty.clone(),
                    quote!(#ty(#formatted_value)),
                ))
            } else {
                Ok(integer_value_template(
                    self.format_comments(&tld.comments)?,
                    self.to_rust_const_case(&tld.name),
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

    pub(crate) fn generate_integer(
        &self,
        tld: ToplevelTypeDefinition,
    ) -> Result<TokenStream, GeneratorError> {
        if let ASN1Type::Integer(ref int) = tld.ty {
            let name = self.to_rust_title_case(&tld.name);
            let mut annotations = vec![
                quote!(delegate),
                self.format_range_annotations(true, &int.constraints)?,
                self.format_tag(tld.tag.as_ref(), false),
            ];
            if name.to_string() != tld.name {
                annotations.push(self.format_identifier_annotation(
                    &tld.name,
                    &tld.comments,
                    &tld.ty,
                ));
            }
            Ok(integer_template(
                self.format_comments(&tld.comments)?,
                name,
                self.join_annotations(annotations),
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

    pub(crate) fn generate_bit_string(
        &self,
        tld: ToplevelTypeDefinition,
    ) -> Result<TokenStream, GeneratorError> {
        if let ASN1Type::BitString(ref bitstr) = tld.ty {
            let name = self.to_rust_title_case(&tld.name);
            let mut annotations = vec![
                quote!(delegate),
                self.format_range_annotations(true, &bitstr.constraints)?,
                self.format_tag(tld.tag.as_ref(), false),
            ];
            if name.to_string() != tld.name {
                annotations.push(self.format_identifier_annotation(
                    &tld.name,
                    &tld.comments,
                    &tld.ty,
                ));
            }
            Ok(bit_string_template(
                self.format_comments(&tld.comments)?,
                name,
                self.join_annotations(annotations),
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
    ) -> Result<TokenStream, GeneratorError> {
        if let ASN1Type::OctetString(ref oct_str) = tld.ty {
            let name = self.to_rust_title_case(&tld.name);
            let mut annotations = vec![
                quote!(delegate),
                self.format_range_annotations(true, &oct_str.constraints)?,
                self.format_tag(tld.tag.as_ref(), false),
            ];
            if name.to_string() != tld.name {
                annotations.push(self.format_identifier_annotation(
                    &tld.name,
                    &tld.comments,
                    &tld.ty,
                ));
            }
            Ok(octet_string_template(
                self.format_comments(&tld.comments)?,
                name,
                self.join_annotations(annotations),
            ))
        } else {
            Err(GeneratorError::new(
                Some(ToplevelDefinition::Type(tld)),
                "Expected OCTET STRING top-level declaration",
                GeneratorErrorType::Asn1TypeMismatch,
            ))
        }
    }

    pub(crate) fn generate_character_string(
        &self,
        tld: ToplevelTypeDefinition,
    ) -> Result<TokenStream, GeneratorError> {
        if let ASN1Type::CharacterString(ref char_str) = tld.ty {
            let name = self.to_rust_title_case(&tld.name);
            let mut annotations = vec![
                quote!(delegate),
                self.format_range_annotations(true, &char_str.constraints)?,
                self.format_alphabet_annotations(char_str.ty, &char_str.constraints)?,
                self.format_tag(tld.tag.as_ref(), false),
            ];
            if name.to_string() != tld.name {
                annotations.push(self.format_identifier_annotation(
                    &tld.name,
                    &tld.comments,
                    &tld.ty,
                ));
            }
            Ok(char_string_template(
                self.format_comments(&tld.comments)?,
                name,
                self.string_type(&char_str.ty)?,
                self.join_annotations(annotations),
            ))
        } else {
            Err(GeneratorError::new(
                Some(ToplevelDefinition::Type(tld)),
                "Expected Character String top-level declaration",
                GeneratorErrorType::Asn1TypeMismatch,
            ))
        }
    }

    pub(crate) fn generate_boolean(
        &self,
        tld: ToplevelTypeDefinition,
    ) -> Result<TokenStream, GeneratorError> {
        // TODO: process boolean constraints
        let name = self.to_rust_title_case(&tld.name);
        let mut annotations = vec![quote!(delegate), self.format_tag(tld.tag.as_ref(), false)];
        if name.to_string() != tld.name {
            annotations.push(self.format_identifier_annotation(&tld.name, &tld.comments, &tld.ty));
        }
        if let ASN1Type::Boolean(_) = tld.ty {
            Ok(boolean_template(
                self.format_comments(&tld.comments)?,
                name,
                self.join_annotations(annotations),
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
    ) -> Result<TokenStream, GeneratorError> {
        let ty = tld.associated_type.as_str();
        match &tld.value {
            ASN1Value::Null if ty == NULL => {
                call_template!(self, primitive_value_template, tld, quote!(()), quote!(()))
            }
            ASN1Value::Null => call_template!(
                self,
                primitive_value_template,
                tld,
                self.to_rust_title_case(&tld.associated_type.as_str()),
                assignment!(self, &tld.associated_type.as_str(), quote!(()))
            ),
            ASN1Value::Boolean(b) if ty == BOOLEAN => call_template!(
                self,
                primitive_value_template,
                tld,
                quote!(bool),
                b.to_token_stream()
            ),
            ASN1Value::Boolean(b) => call_template!(
                self,
                primitive_value_template,
                tld,
                self.to_rust_title_case(&tld.associated_type.as_str()),
                assignment!(self, &tld.associated_type.as_str(), b.to_token_stream())
            ),
            ASN1Value::LinkedIntValue { .. } => self.generate_integer_value(tld),
            ASN1Value::BitString(_) if ty == BIT_STRING => call_template!(
                self,
                lazy_static_value_template,
                tld,
                quote!(BitString),
                self.value_to_tokens(&tld.value, None)?
            ),
            ASN1Value::OctetString(_) if ty == OCTET_STRING => call_template!(
                self,
                lazy_static_value_template,
                tld,
                quote!(OctetString),
                self.value_to_tokens(&tld.value, None)?
            ),
            ASN1Value::Choice {
                variant_name,
                inner_value,
                ..
            } => {
                if inner_value.is_const_type() {
                    call_template!(
                        self,
                        const_choice_value_template,
                        tld,
                        self.to_rust_title_case(&tld.associated_type.as_str()),
                        self.to_rust_enum_identifier(variant_name),
                        self.value_to_tokens(inner_value, None)?
                    )
                } else {
                    call_template!(
                        self,
                        choice_value_template,
                        tld,
                        self.to_rust_title_case(&tld.associated_type.as_str()),
                        self.to_rust_enum_identifier(variant_name),
                        self.value_to_tokens(inner_value, None)?
                    )
                }
            }
            ASN1Value::EnumeratedValue {
                enumerated,
                enumerable,
            } => call_template!(
                self,
                enum_value_template,
                tld,
                self.to_rust_title_case(enumerated),
                self.to_rust_enum_identifier(enumerable)
            ),
            ASN1Value::Time(_) if ty == GENERALIZED_TIME => call_template!(
                self,
                lazy_static_value_template,
                tld,
                quote!(GeneralizedTime),
                self.value_to_tokens(&tld.value, Some(&quote!(GeneralizedTime)))?
            ),
            ASN1Value::Time(_) if ty == UTC_TIME => call_template!(
                self,
                lazy_static_value_template,
                tld,
                quote!(UtcTime),
                self.value_to_tokens(&tld.value, Some(&quote!(UtcTime)))?
            ),
            ASN1Value::LinkedStructLikeValue(s) => {
                let members = s
                    .iter()
                    .map(|(_, ty, val)| {
                        self.value_to_tokens(val.value(), self.type_to_tokens(ty).ok().as_ref())
                    })
                    .collect::<Result<Vec<TokenStream>, _>>()?;
                call_template!(
                    self,
                    sequence_or_set_value_template,
                    tld,
                    self.to_rust_title_case(&tld.associated_type.as_str()),
                    quote!(#(#members),*)
                )
            }
            ASN1Value::LinkedNestedValue { supertypes, value } => {
                let parent = supertypes.last().map(|s| self.to_rust_title_case(s));
                if value.is_const_type() {
                    call_template!(
                        self,
                        primitive_value_template,
                        tld,
                        self.to_rust_title_case(&tld.associated_type.as_str()),
                        assignment!(
                            self,
                            &tld.associated_type.as_str(),
                            self.value_to_tokens(&tld.value, parent.as_ref())?
                        )
                    )
                } else {
                    call_template!(
                        self,
                        lazy_static_value_template,
                        tld,
                        self.to_rust_title_case(&tld.associated_type.as_str()),
                        assignment!(
                            self,
                            &tld.associated_type.as_str(),
                            self.value_to_tokens(&tld.value, parent.as_ref())?
                        )
                    )
                }
            }
            ASN1Value::ObjectIdentifier(_) if ty == OBJECT_IDENTIFIER => call_template!(
                self,
                lazy_static_value_template,
                tld,
                quote!(ObjectIdentifier),
                self.value_to_tokens(&tld.value, None)?
            ),
            ASN1Value::LinkedCharStringValue(_, _) if ty == NUMERIC_STRING => call_template!(
                self,
                lazy_static_value_template,
                tld,
                quote!(NumericString),
                self.value_to_tokens(&tld.value, None)?
            ),
            ASN1Value::LinkedCharStringValue(_, _) if ty == VISIBLE_STRING => call_template!(
                self,
                lazy_static_value_template,
                tld,
                quote!(VisibleString),
                self.value_to_tokens(&tld.value, None)?
            ),
            ASN1Value::LinkedCharStringValue(_, _) if ty == IA5_STRING => call_template!(
                self,
                lazy_static_value_template,
                tld,
                quote!(IA5String),
                self.value_to_tokens(&tld.value, None)?
            ),
            ASN1Value::LinkedCharStringValue(_, _) if ty == UTF8_STRING => call_template!(
                self,
                lazy_static_value_template,
                tld,
                quote!(UTF8String),
                self.value_to_tokens(&tld.value, None)?
            ),
            ASN1Value::LinkedCharStringValue(_, _) if ty == BMP_STRING => call_template!(
                self,
                lazy_static_value_template,
                tld,
                quote!(BMPString),
                self.value_to_tokens(&tld.value, None)?
            ),
            ASN1Value::LinkedCharStringValue(_, _) if ty == PRINTABLE_STRING => call_template!(
                self,
                lazy_static_value_template,
                tld,
                quote!(PrintableString),
                self.value_to_tokens(&tld.value, None)?
            ),
            ASN1Value::LinkedCharStringValue(_, _) if ty == GENERAL_STRING => call_template!(
                self,
                lazy_static_value_template,
                tld,
                quote!(GeneralString),
                self.value_to_tokens(&tld.value, None)?
            ),
            ASN1Value::LinkedArrayLikeValue(s) if ty.contains(SEQUENCE_OF) => {
                let item_type = self.format_sequence_or_set_of_item_type(
                    match tld.associated_type {
                        ASN1Type::SequenceOf(seq) => seq.element_type.as_str().into_owned(),
                        _ => unreachable!(),
                    },
                    s.first().map(|i| &**i),
                );
                call_template!(
                    self,
                    lazy_static_value_template,
                    tld,
                    quote!(Vec<#item_type>),
                    self.value_to_tokens(&tld.value, None)?
                )
            }
            ASN1Value::LinkedArrayLikeValue(s) if ty.contains(SET_OF) => {
                let item_type = self.format_sequence_or_set_of_item_type(
                    match tld.associated_type {
                        ASN1Type::SetOf(set) => set.element_type.as_str().into_owned(),
                        _ => unreachable!(),
                    },
                    s.first().map(|i| &**i),
                );
                call_template!(
                    self,
                    lazy_static_value_template,
                    tld,
                    quote!(Vec<#item_type>),
                    self.value_to_tokens(&tld.value, None)?
                )
            }
            ASN1Value::BitString(_)
            | ASN1Value::Time(_)
            | ASN1Value::LinkedCharStringValue(_, _)
            | ASN1Value::ObjectIdentifier(_)
            | ASN1Value::LinkedArrayLikeValue(_)
            | ASN1Value::ElsewhereDeclaredValue { .. }
            | ASN1Value::OctetString(_) => call_template!(
                self,
                lazy_static_value_template,
                tld,
                self.to_rust_title_case(&tld.associated_type.as_str()),
                assignment!(
                    self,
                    &tld.associated_type.as_str(),
                    self.value_to_tokens(&tld.value, None)?
                )
            ),
            _ => Ok(TokenStream::new()),
        }
    }

    pub(crate) fn generate_any(
        &self,
        tld: ToplevelTypeDefinition,
    ) -> Result<TokenStream, GeneratorError> {
        let name = self.to_rust_title_case(&tld.name);
        let mut annotations = vec![quote!(delegate), self.format_tag(tld.tag.as_ref(), false)];
        if name.to_string() != tld.name {
            annotations.push(self.format_identifier_annotation(&tld.name, &tld.comments, &tld.ty));
        }
        Ok(any_template(
            self.format_comments(&tld.comments)?,
            name,
            self.join_annotations(annotations),
        ))
    }

    pub(crate) fn generate_generalized_time(
        &self,
        tld: ToplevelTypeDefinition,
    ) -> Result<TokenStream, GeneratorError> {
        if let ASN1Type::GeneralizedTime(_) = &tld.ty {
            let name = self.to_rust_title_case(&tld.name);
            let mut annotations = vec![quote!(delegate), self.format_tag(tld.tag.as_ref(), false)];
            if name.to_string() != tld.name {
                annotations.push(self.format_identifier_annotation(
                    &tld.name,
                    &tld.comments,
                    &tld.ty,
                ));
            }
            Ok(generalized_time_template(
                self.format_comments(&tld.comments)?,
                name,
                self.join_annotations(annotations),
            ))
        } else {
            Err(GeneratorError::new(
                Some(ToplevelDefinition::Type(tld)),
                "Expected GeneralizedTime top-level declaration",
                GeneratorErrorType::Asn1TypeMismatch,
            ))
        }
    }

    pub(crate) fn generate_utc_time(
        &self,
        tld: ToplevelTypeDefinition,
    ) -> Result<TokenStream, GeneratorError> {
        if let ASN1Type::UTCTime(_) = &tld.ty {
            let name = self.to_rust_title_case(&tld.name);
            let mut annotations = vec![quote!(delegate), self.format_tag(tld.tag.as_ref(), false)];
            if name.to_string() != tld.name {
                annotations.push(self.format_identifier_annotation(
                    &tld.name,
                    &tld.comments,
                    &tld.ty,
                ));
            }
            Ok(utc_time_template(
                self.format_comments(&tld.comments)?,
                name,
                self.join_annotations(annotations),
            ))
        } else {
            Err(GeneratorError::new(
                Some(ToplevelDefinition::Type(tld)),
                "Expected UTCTime top-level declaration",
                GeneratorErrorType::Asn1TypeMismatch,
            ))
        }
    }

    pub(crate) fn generate_oid(
        &self,
        tld: ToplevelTypeDefinition,
    ) -> Result<TokenStream, GeneratorError> {
        if let ASN1Type::ObjectIdentifier(oid) = &tld.ty {
            let name = self.to_rust_title_case(&tld.name);
            let mut annotations = vec![
                quote!(delegate),
                self.format_tag(tld.tag.as_ref(), false),
                self.format_range_annotations(false, &oid.constraints)?,
            ];
            if name.to_string() != tld.name {
                annotations.push(self.format_identifier_annotation(
                    &tld.name,
                    &tld.comments,
                    &tld.ty,
                ));
            }
            Ok(oid_template(
                self.format_comments(&tld.comments)?,
                name,
                self.join_annotations(annotations),
            ))
        } else {
            Err(GeneratorError::new(
                Some(ToplevelDefinition::Type(tld)),
                "Expected OBJECT IDENTIFIER top-level declaration",
                GeneratorErrorType::Asn1TypeMismatch,
            ))
        }
    }

    pub(crate) fn generate_null(
        &self,
        tld: ToplevelTypeDefinition,
    ) -> Result<TokenStream, GeneratorError> {
        if let ASN1Type::Null = tld.ty {
            let name = self.to_rust_title_case(&tld.name);
            let mut annotations = vec![quote!(delegate), self.format_tag(tld.tag.as_ref(), false)];
            if name.to_string() != tld.name {
                annotations.push(self.format_identifier_annotation(
                    &tld.name,
                    &tld.comments,
                    &tld.ty,
                ));
            }
            Ok(null_template(
                self.format_comments(&tld.comments)?,
                name,
                self.join_annotations(annotations),
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
    ) -> Result<TokenStream, GeneratorError> {
        if let ASN1Type::Enumerated(ref enumerated) = tld.ty {
            let extensible = enumerated
                .extensible
                .map(|_| {
                    quote! {
                    #[non_exhaustive]}
                })
                .unwrap_or_default();
            let name = self.to_rust_title_case(&tld.name);
            let mut annotations =
                vec![quote!(enumerated), self.format_tag(tld.tag.as_ref(), false)];
            if name.to_string() != tld.name {
                annotations.push(self.format_identifier_annotation(
                    &tld.name,
                    &tld.comments,
                    &tld.ty,
                ));
            }
            Ok(enumerated_template(
                self.format_comments(&tld.comments)?,
                name,
                extensible,
                self.format_enum_members(enumerated),
                self.join_annotations(annotations),
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
    ) -> Result<TokenStream, GeneratorError> {
        if let ASN1Type::Choice(ref choice) = tld.ty {
            let name = self.to_rust_title_case(&tld.name);
            let inner_options = self.format_nested_choice_options(choice, &name.to_string())?;
            let extensible = choice
                .extensible
                .map(|_| {
                    quote! {
                    #[non_exhaustive]}
                })
                .unwrap_or_default();
            let mut annotations = vec![quote!(choice), self.format_tag(tld.tag.as_ref(), true)];
            if name.to_string() != tld.name {
                annotations.push(self.format_identifier_annotation(
                    &tld.name,
                    &tld.comments,
                    &tld.ty,
                ));
            }
            Ok(choice_template(
                self.format_comments(&tld.comments)?,
                name.clone(),
                extensible,
                self.format_choice_options(choice, &name.to_string())?,
                inner_options,
                self.join_annotations(annotations),
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
    ) -> Result<TokenStream, GeneratorError> {
        match tld.ty {
            ASN1Type::Sequence(ref seq) | ASN1Type::Set(ref seq) => {
                let name = self.to_rust_title_case(&tld.name);
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
                let class_fields = if self.config.opaque_open_types {
                    TokenStream::new()
                } else {
                    seq.members.iter().fold(
                    TokenStream::new(),
                    |mut acc, m| {
                        [
                            m.constraints.clone(),
                            m.ty.constraints().map_or(vec![], |c| c.to_vec())
                        ]
                        .concat()
                        .iter()
                        .for_each(|c| {
                            if let (Constraint::TableConstraint(t), ASN1Type::InformationObjectFieldReference(iofr)) = (c, &m.ty) {
                                let decode_fn = format_ident!("decode_{}", self.to_rust_snake_case(&m.name));
                                let open_field_name = self.to_rust_snake_case(&m.name);
                                let identifier = t.linked_fields.iter().map(|l|
                                    self.to_rust_snake_case(&l.field_name)
                                );
                                let field_name = iofr.field_path.last().unwrap().identifier().replace('&', "");
                                if field_name.starts_with(|initial: char| initial.is_lowercase()) {
                                    // Fixed-value fields of Information Object usages should have been resolved at this point
                                    return;
                                }
                                let obj_set_name = match t.object_set.values.first() {
                                    Some(ObjectSetValue::Reference(s)) => self.to_rust_title_case(s),
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
                    })
                };
                let (declaration, name_types) =
                    self.format_sequence_or_set_members(seq, &name.to_string())?;
                let mut annotations = vec![set_annotation, self.format_tag(tld.tag.as_ref(), true)];
                if name.to_string() != tld.name {
                    annotations.push(self.format_identifier_annotation(
                        &tld.name,
                        &tld.comments,
                        &tld.ty,
                    ));
                }
                Ok(sequence_or_set_template(
                    self.format_comments(&tld.comments)?,
                    name.clone(),
                    extensible,
                    declaration,
                    self.format_nested_sequence_members(seq, &name.to_string())?,
                    self.join_annotations(annotations),
                    self.format_default_methods(&seq.members, &name.to_string())?,
                    self.format_new_impl(&name, name_types),
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

    pub(crate) fn generate_sequence_or_set_of(
        &self,
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
        let name = self.to_rust_title_case(&tld.name);
        let anonymous_item = match seq_or_set_of.element_type.as_ref() {
            ASN1Type::ElsewhereDeclaredType(_) => None,
            n => Some(
                self.generate_tld(ToplevelDefinition::Type(ToplevelTypeDefinition {
                    parameterization: None,
                    comments: format!(
                        " Anonymous {} OF member ",
                        if is_set_of { "SET" } else { "SEQUENCE" }
                    ),
                    name: String::from(INNER_ARRAY_LIKE_PREFIX) + &name.to_string(),
                    ty: n.clone(),
                    tag: None,
                    index: None,
                }))?,
            ),
        }
        .unwrap_or_default();
        let member_type = match seq_or_set_of.element_type.as_ref() {
            ASN1Type::ElsewhereDeclaredType(d) => self.to_rust_title_case(&d.identifier),
            _ => format_ident!("Anonymous{}", &name.to_string()).to_token_stream(),
        };
        let mut annotations = vec![
            quote!(delegate),
            self.format_range_annotations(true, &seq_or_set_of.constraints)?,
            self.format_tag(tld.tag.as_ref(), false),
        ];
        if name.to_string() != tld.name {
            annotations.push(self.format_identifier_annotation(&tld.name, &tld.comments, &tld.ty));
        }
        Ok(sequence_or_set_of_template(
            is_set_of,
            self.format_comments(&tld.comments)?,
            name,
            anonymous_item,
            member_type,
            self.join_annotations(annotations),
        ))
    }

    pub(crate) fn generate_information_object_set(
        &self,
        tld: ToplevelInformationDefinition,
    ) -> Result<TokenStream, GeneratorError> {
        if self.config.opaque_open_types {
            return Ok(TokenStream::new());
        }
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
                        self.resolve_standard_syntax(class, s)
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
                            top_level_declaration: Some(ToplevelDefinition::Information(
                                tld.clone(),
                            )),
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

            if choices.is_empty() {
                for InformationObjectClassField { identifier, .. } in &class.fields {
                    choices.insert(identifier.identifier().clone(), Vec::new());
                }
            }

            let name = self.to_rust_title_case(&tld.name);
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
            let class_unique_id_type_name = self.type_to_tokens(&class_unique_id_type)?;

            let mut field_enums = vec![];
            for (field_name, fields) in choices.iter() {
                let field_enum_name = format_ident!("{name}_{}", field_name.replace('&', ""));
                let (mut ids, mut inner_types) = (vec![], vec![]);
                for (index, (id, ty)) in fields.iter().enumerate() {
                    let identifier_value = match id {
                        ASN1Value::LinkedElsewhereDefinedValue {
                            can_be_const: false,
                            ..
                        } => {
                            let tokenized_value =
                                self.value_to_tokens(id, Some(&class_unique_id_type_name))?;
                            quote!(*#tokenized_value)
                        }
                        ASN1Value::LinkedNestedValue { value, .. }
                            if matches![
                                &**value,
                                ASN1Value::LinkedElsewhereDefinedValue {
                                    can_be_const: false,
                                    ..
                                }
                            ] =>
                        {
                            let tokenized_value =
                                self.value_to_tokens(value, Some(&class_unique_id_type_name))?;
                            quote!(*#tokenized_value)
                        }
                        ASN1Value::LinkedNestedValue { value, .. }
                            if matches![
                                &**value,
                                ASN1Value::LinkedElsewhereDefinedValue { .. }
                            ] =>
                        {
                            self.value_to_tokens(value, Some(&class_unique_id_type_name))?
                        }
                        _ => self.value_to_tokens(id, Some(&class_unique_id_type_name))?,
                    };
                    let type_id = self.type_to_tokens(ty).unwrap_or(quote!(Option<()>));
                    let variant_name = match id {
                        ASN1Value::LinkedElsewhereDefinedValue {
                            identifier: ref_id, ..
                        }
                        | ASN1Value::ElsewhereDeclaredValue {
                            identifier: ref_id, ..
                        } => self.to_rust_title_case(ref_id),
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
                        let range_constraints = self
                            .format_range_annotations(
                                signed_range,
                                ty.constraints().unwrap_or(&Vec::<_>::new()),
                            )
                            .unwrap();
                        let alphabet_constraints = character_string_type
                            .and_then(|c| {
                                self.format_alphabet_annotations(
                                    c,
                                    ty.constraints().unwrap_or(&Vec::<_>::new()),
                                )
                                .ok()
                            })
                            .unwrap_or_default();
                        let annotations = self.join_annotations(vec![
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
}
