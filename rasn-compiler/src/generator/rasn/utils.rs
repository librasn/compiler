use std::str::FromStr;

use proc_macro2::{Ident, Literal, Punct, Spacing, Span, TokenStream};
use quote::{format_ident, quote, ToTokens, TokenStreamExt};
use utils::types::SequenceOrSetOf;

use crate::{
    common::INTERNAL_NESTED_TYPE_NAME_PREFIX,
    intermediate::{
        constraints::Constraint,
        encoding_rules::per_visible::{
            per_visible_range_constraints, CharsetSubset, PerVisibleAlphabetConstraints,
        },
        information_object::{InformationObjectClass, InformationObjectField},
        types::{Choice, ChoiceOption, Enumerated, SequenceOrSet, SequenceOrSetMember},
        ASN1Type, ASN1Value, AsnTag, CharacterStringType, IntegerType, TagClass,
        TaggingEnvironment, ToplevelDefinition, ToplevelTypeDefinition,
    },
};

use crate::generator::error::{GeneratorError, GeneratorErrorType};

macro_rules! error {
    ($kind:ident, $($arg:tt)*) => {
        GeneratorError {
            details: format!($($arg)*),
            top_level_declaration: None,
            kind: GeneratorErrorType::$kind,
        }
    };
}

pub(crate) use error;

use self::types::{CharacterString, Constrainable};

use super::*;

impl IntegerType {
    fn to_token_stream(self) -> TokenStream {
        match self {
            IntegerType::Int8 => quote!(i8),
            IntegerType::Uint8 => quote!(u8),
            IntegerType::Int16 => quote!(i16),
            IntegerType::Uint16 => quote!(u16),
            IntegerType::Int32 => quote!(i32),
            IntegerType::Uint32 => quote!(u32),
            IntegerType::Int64 => quote!(i64),
            IntegerType::Uint64 => quote!(u64),
            IntegerType::Unbounded => quote!(Integer),
        }
    }
}

pub struct NameType {
    name: Ident,
    typ: TokenStream,
}

#[cfg(test)]
macro_rules! assert_eq_ignore_ws {
    ($left:expr, $right:expr) => {
        assert_eq!(
            $left.replace(|c: char| c.is_whitespace(), ""),
            String::from($right).replace(|c: char| c.is_whitespace(), "")
        )
    };
}

impl Rasn {
    pub(crate) fn inner_name(&self, name: &str, parent_name: &str) -> Ident {
        format_ident!(
            "{}{}",
            parent_name,
            self.to_rust_title_case(name).to_string()
        )
    }

    pub(crate) fn int_type_token(
        &self,
        opt_min: Option<i128>,
        opt_max: Option<i128>,
        is_extensible: bool,
    ) -> Ident {
        if let (Some(min), Some(max)) = (opt_min, opt_max) {
            let as_str = if is_extensible {
                "Integer"
            } else if min >= 0 {
                match max {
                    r if r <= u8::MAX.into() => "u8",
                    r if r <= u16::MAX.into() => "u16",
                    r if r <= u32::MAX.into() => "u32",
                    r if r <= u64::MAX.into() => "u64",
                    _ => "Integer",
                }
            } else {
                match (min, max) {
                    (mi, ma) if mi >= i8::MIN.into() && ma <= i8::MAX.into() => "i8",
                    (mi, ma) if mi >= i16::MIN.into() && ma <= i16::MAX.into() => "i16",
                    (mi, ma) if mi >= i32::MIN.into() && ma <= i32::MAX.into() => "i32",
                    (mi, ma) if mi >= i64::MIN.into() && ma <= i64::MAX.into() => "i64",
                    _ => "Integer",
                }
            };
            format_ident!("{as_str}")
        } else {
            format_ident!("Integer")
        }
    }

    pub(crate) fn format_comments(&self, comments: &str) -> Result<TokenStream, GeneratorError> {
        if comments.is_empty() {
            Ok(TokenStream::new())
        } else {
            let joined = String::from("///") + &comments.replace('\n', "\n ///") + "\n";
            Ok(TokenStream::from_str(&joined)?)
        }
    }

    pub(crate) fn format_identifier_annotation(
        &self,
        name: &str,
        comments: &str,
        ty: &ASN1Type,
    ) -> TokenStream {
        if comments == " Inner type "
            || comments.starts_with(" Anonymous ")
            || name.starts_with("ext_group_")
        {
            let identifier = ty.as_str().replace(' ', "_");
            quote!(identifier = #identifier)
        } else {
            quote!(identifier = #name)
        }
    }

    pub(crate) fn format_range_annotations(
        &self,
        signed: bool,
        constraints: &Vec<Constraint>,
    ) -> Result<TokenStream, GeneratorError> {
        if constraints.is_empty() {
            return Ok(TokenStream::new());
        }
        let per_constraints = per_visible_range_constraints(signed, constraints)?;
        let range_prefix = if per_constraints.is_size_constraint() {
            quote!(size)
        } else {
            quote!(value)
        };
        // handle default size constraints
        if per_constraints.is_size_constraint()
            && !per_constraints.is_extensible()
            && per_constraints.min::<i128>() == Some(0)
            && per_constraints.max::<i128>().is_none()
        {
            return Ok(TokenStream::new());
        }
        Ok(
            match (
                per_constraints.min::<i128>(),
                per_constraints.max::<i128>(),
                per_constraints.is_extensible(),
            ) {
                (Some(min), Some(max), true) if min == max => {
                    let range = format!("{min}");
                    quote!(#range_prefix(#range, extensible))
                }
                (Some(min), Some(max), true) => {
                    let range = format!("{min}..={max}");
                    quote!(#range_prefix(#range, extensible))
                }
                (Some(min), Some(max), false) if min == max => {
                    let range = format!("{min}");
                    quote!(#range_prefix(#range))
                }
                (Some(min), Some(max), false) => {
                    let range = format!("{min}..={max}");
                    quote!(#range_prefix(#range))
                }
                (Some(min), None, true) => {
                    let range = format!("{min}..");
                    quote!(#range_prefix(#range, extensible))
                }
                (Some(min), None, false) => {
                    let range = format!("{min}..");
                    quote!(#range_prefix(#range))
                }
                (None, Some(max), true) => {
                    let range = format!("..={max}");
                    quote!(#range_prefix(#range, extensible))
                }
                (None, Some(max), false) => {
                    let range = format!("..={max}");
                    quote!(#range_prefix(#range))
                }
                _ => TokenStream::new(),
            },
        )
    }

    pub(crate) fn format_alphabet_annotations(
        &self,
        string_type: CharacterStringType,
        constraints: &Vec<Constraint>,
    ) -> Result<TokenStream, GeneratorError> {
        if constraints.is_empty() {
            return Ok(TokenStream::new());
        }
        let mut permitted_alphabet = PerVisibleAlphabetConstraints::default_for(string_type);
        for c in constraints {
            if let Some(mut p) = PerVisibleAlphabetConstraints::try_new(c, string_type)? {
                permitted_alphabet += &mut p
            }
        }
        permitted_alphabet.finalize();
        let alphabet_unicode = permitted_alphabet
            .charset_subsets()
            .iter()
            .map(|subset| match subset {
                CharsetSubset::Single(c) => format!("{}", c.escape_unicode()),
                CharsetSubset::Range { from, to } => format!(
                    "{}..{}",
                    from.map_or(String::from(""), |c| format!("{}", c.escape_unicode())),
                    to.map_or(String::from(""), |c| format!("{}", c.escape_unicode()))
                ),
            })
            .collect::<Vec<String>>()
            .join(", ");
        Ok(if alphabet_unicode.is_empty() {
            TokenStream::new()
        } else {
            quote!(from(#alphabet_unicode))
        })
    }

    pub(crate) fn format_enum_members(&self, enumerated: &Enumerated) -> TokenStream {
        let first_extension_index = enumerated.extensible;
        let enumerals = enumerated.members.iter().enumerate().map(|(i, e)| {
            let name = self.to_rust_enum_identifier(&e.name);
            let index = Literal::i128_unsuffixed(e.index);
            let extension_annotation = if i >= first_extension_index.unwrap_or(usize::MAX) {
                quote!(extension_addition)
            } else {
                TokenStream::new()
            };
            let identifier_annotation = if name != e.name {
                let name = &e.name;
                quote!(identifier = #name)
            } else {
                TokenStream::new()
            };
            let annotations =
                self.join_annotations(vec![extension_annotation, identifier_annotation]);
            quote!(
                #annotations
                #name = #index,
            )
        });
        quote!(#(#enumerals)*)
    }

    pub(crate) fn format_tag(
        &self,
        tag: Option<&AsnTag>,
        fallback_to_automatic: bool,
    ) -> TokenStream {
        if let Some(tag) = tag {
            let class = match tag.tag_class {
                TagClass::Universal => quote!(universal),
                TagClass::Application => quote!(application),
                TagClass::Private => quote!(private),
                TagClass::ContextSpecific => quote!(context),
            };
            let id = Literal::u64_unsuffixed(tag.id);
            if tag.environment == TaggingEnvironment::Explicit {
                quote!(tag(explicit(#class, #id)))
            } else {
                quote!(tag(#class, #id))
            }
        } else if fallback_to_automatic {
            quote!(automatic_tags)
        } else {
            TokenStream::new()
        }
    }

    pub(crate) fn format_sequence_or_set_members(
        &self,
        sequence_or_set: &SequenceOrSet,
        parent_name: &String,
    ) -> Result<(TokenStream, Vec<NameType>), GeneratorError> {
        let first_extension_index = sequence_or_set.extensible;
        sequence_or_set.members.iter().enumerate().try_fold(
            (TokenStream::new(), Vec::new()),
            |mut acc, (i, m)| {
                let extension_annotation = if i >= first_extension_index.unwrap_or(usize::MAX)
                    && m.name.starts_with("ext_group_")
                {
                    quote!(extension_addition_group)
                } else if i >= first_extension_index.unwrap_or(usize::MAX) {
                    quote!(extension_addition)
                } else {
                    TokenStream::new()
                };
                self.format_sequence_member(m, parent_name, extension_annotation)
                    .map(|(declaration, name_type)| {
                        acc.0.append_all([declaration, quote!(, )]);
                        acc.1.push(name_type);
                        acc
                    })
            },
        )
    }

    pub(crate) fn format_sequence_member(
        &self,
        member: &SequenceOrSetMember,
        parent_name: &String,
        extension_annotation: TokenStream,
    ) -> Result<(TokenStream, NameType), GeneratorError> {
        let name = self.to_rust_snake_case(&member.name);
        let (mut all_constraints, mut formatted_type_name) = self.constraints_and_type_name(
            &member.ty,
            &member.name,
            parent_name,
            member.is_recursive,
        )?;
        all_constraints.append(&mut member.constraints.clone());
        if (member.is_optional && member.default_value.is_none())
            || member.name.starts_with("ext_group_")
        {
            formatted_type_name = quote!(Option<#formatted_type_name>);
        }
        let default_annotation = member
            .default_value
            .as_ref()
            .map(|_| {
                let default_fn = self.default_method_name(parent_name, &member.name);
                quote!(default = #default_fn)
            })
            .unwrap_or_default();
        let range_annotations = self.format_range_annotations(
            matches!(member.ty, ASN1Type::Integer(_)),
            &all_constraints,
        )?;
        let alphabet_annotations = if let ASN1Type::CharacterString(c_string) = &member.ty {
            self.format_alphabet_annotations(c_string.ty, &all_constraints)?
        } else {
            TokenStream::new()
        };
        let mut annotation_items = vec![
            extension_annotation,
            range_annotations,
            alphabet_annotations,
            self.format_tag(member.tag.as_ref(), false),
            default_annotation,
        ];
        if name != member.name || member.name.starts_with("ext_group_") {
            annotation_items.push(self.format_identifier_annotation(&member.name, "", &member.ty));
        }
        let annotations = self.join_annotations(annotation_items);
        Ok((
            quote! {
                #annotations
                pub #name: #formatted_type_name
            },
            NameType {
                name,
                typ: formatted_type_name,
            },
        ))
    }

    pub(crate) fn format_choice_options(
        &self,
        choice: &Choice,
        parent_name: &String,
    ) -> Result<TokenStream, GeneratorError> {
        let first_extension_index = choice.extensible;
        let options = choice
            .options
            .iter()
            .enumerate()
            .map(|(i, o)| {
                let extension_annotation = if i >= first_extension_index.unwrap_or(usize::MAX)
                    && o.name.starts_with("ext_group_")
                {
                    quote!(extension_addition_group)
                } else if i >= first_extension_index.unwrap_or(usize::MAX) {
                    quote!(extension_addition)
                } else {
                    TokenStream::new()
                };
                let name = self.to_rust_enum_identifier(&o.name);
                self.format_choice_option(name, o, parent_name, extension_annotation)
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(quote!(#(#options)*))
    }

    pub(crate) fn format_choice_option(
        &self,
        name: Ident,
        member: &ChoiceOption,
        parent_name: &String,
        extension_annotation: TokenStream,
    ) -> Result<TokenStream, GeneratorError> {
        let (mut all_constraints, formatted_type_name) = self.constraints_and_type_name(
            &member.ty,
            &member.name,
            parent_name,
            member.is_recursive,
        )?;
        all_constraints.append(&mut member.constraints.clone());
        let range_annotations = self.format_range_annotations(
            matches!(member.ty, ASN1Type::Integer(_)),
            &all_constraints,
        )?;
        let alphabet_annotations = if let ASN1Type::CharacterString(c_string) = &member.ty {
            self.format_alphabet_annotations(c_string.ty, &all_constraints)?
        } else {
            TokenStream::new()
        };
        let mut annotation_items = vec![
            extension_annotation,
            range_annotations,
            alphabet_annotations,
            self.format_tag(member.tag.as_ref(), false),
        ];
        if name != member.name || member.name.starts_with("ext_group_") {
            annotation_items.push(self.format_identifier_annotation(&member.name, "", &member.ty));
        }
        let annotations = self.join_annotations(annotation_items);
        Ok(quote! {
                #annotations
                #name(#formatted_type_name),
        })
    }

    pub(crate) fn constraints_and_type_name(
        &self,
        ty: &ASN1Type,
        name: &String,
        parent_name: &String,
        is_recursive: bool,
    ) -> Result<(Vec<Constraint>, TokenStream), GeneratorError> {
        Ok(match ty {
            ASN1Type::Null => (vec![], quote!(())),
            ASN1Type::Boolean(b) => (b.constraints.clone(), quote!(bool)),
            ASN1Type::Integer(i) => {
                let per_constraints = per_visible_range_constraints(true, &i.constraints)?;
                (
                    i.constraints.clone(),
                    self.int_type_token(
                        per_constraints.min(),
                        per_constraints.max(),
                        per_constraints.is_extensible(),
                    )
                    .to_token_stream(),
                )
            }
            ASN1Type::Real(_) => (vec![], quote!(f64)),
            ASN1Type::ObjectIdentifier(o) => (o.constraints.clone(), quote!(ObjectIdentifier)),
            ASN1Type::BitString(b) => (b.constraints.clone(), quote!(BitString)),
            ASN1Type::OctetString(o) => (o.constraints.clone(), quote!(OctetString)),
            ASN1Type::GeneralizedTime(o) => (o.constraints.clone(), quote!(GeneralizedTime)),
            ASN1Type::UTCTime(o) => (o.constraints.clone(), quote!(UtcTime)),
            ASN1Type::Time(_) => {
                return Err(GeneratorError {
                    details: "rasn does not support TIME types yet!".into(),
                    top_level_declaration: None,
                    kind: GeneratorErrorType::NotYetInplemented,
                })
            }
            ASN1Type::CharacterString(c) => (c.constraints.clone(), self.string_type(&c.ty)?),
            ASN1Type::Enumerated(_)
            | ASN1Type::Choice(_)
            | ASN1Type::Sequence(_)
            | ASN1Type::Set(_) => {
                let mut tokenized = self.inner_name(name, parent_name).to_token_stream();
                if is_recursive {
                    tokenized = boxed_type(tokenized);
                }
                (vec![], tokenized)
            }
            ASN1Type::SequenceOf(s) => {
                let (_, inner_type) = self.constraints_and_type_name(
                    &s.element_type,
                    name,
                    parent_name,
                    s.is_recursive,
                )?;
                (s.constraints().clone(), quote!(SequenceOf<#inner_type>))
            }
            ASN1Type::SetOf(s) => {
                let (_, inner_type) = self.constraints_and_type_name(
                    &s.element_type,
                    name,
                    parent_name,
                    s.is_recursive,
                )?;
                (s.constraints().clone(), quote!(SetOf<#inner_type>))
            }
            ASN1Type::ElsewhereDeclaredType(e) => {
                let mut tokenized = self.to_rust_title_case(&e.identifier).to_token_stream();
                if is_recursive {
                    tokenized = boxed_type(tokenized);
                };
                (e.constraints.clone(), tokenized)
            }
            ASN1Type::InformationObjectFieldReference(_)
            | ASN1Type::EmbeddedPdv
            | ASN1Type::External => (vec![], quote!(Any)),
            ASN1Type::ChoiceSelectionType(_) => unreachable!(),
        })
    }

    pub(crate) fn string_type(
        &self,
        c_type: &CharacterStringType,
    ) -> Result<TokenStream, GeneratorError> {
        match c_type {
            CharacterStringType::NumericString => Ok(quote!(NumericString)),
            CharacterStringType::VisibleString => Ok(quote!(VisibleString)),
            CharacterStringType::IA5String => Ok(quote!(Ia5String)),
            CharacterStringType::TeletexString => Ok(quote!(TeletexString)),
            CharacterStringType::VideotexString => Err(GeneratorError {
                kind: GeneratorErrorType::NotYetInplemented,
                details: "VideotexString is currently unsupported!".into(),
                top_level_declaration: None,
            }),
            CharacterStringType::GraphicString => Err(GeneratorError {
                kind: GeneratorErrorType::NotYetInplemented,
                details: "GraphicString is currently unsupported!".into(),
                top_level_declaration: None,
            }),
            CharacterStringType::GeneralString => Ok(quote!(GeneralString)),
            CharacterStringType::UniversalString => Err(GeneratorError {
                kind: GeneratorErrorType::NotYetInplemented,
                details: "UniversalString is currently unsupported!".into(),
                top_level_declaration: None,
            }),
            CharacterStringType::UTF8String => Ok(quote!(Utf8String)),
            CharacterStringType::BMPString => Ok(quote!(BmpString)),
            CharacterStringType::PrintableString => Ok(quote!(PrintableString)),
        }
    }

    pub(crate) fn join_annotations(&self, elements: Vec<TokenStream>) -> TokenStream {
        let mut not_empty_exprs = elements.into_iter().filter(|ts| !ts.is_empty());
        if let Some(mut annotations) = not_empty_exprs.next() {
            for elem in not_empty_exprs {
                annotations.append(Punct::new(',', Spacing::Alone));
                annotations.append_all(elem);
            }
            quote!(#[rasn(#annotations)])
        } else {
            quote!()
        }
    }

    pub(crate) fn default_method_name(&self, parent_name: &str, field_name: &str) -> String {
        format!(
            "{}_{}_default",
            self.to_rust_snake_case(parent_name),
            self.to_rust_snake_case(field_name)
        )
    }

    pub(crate) fn format_default_methods(
        &self,
        members: &Vec<SequenceOrSetMember>,
        parent_name: &str,
    ) -> Result<TokenStream, GeneratorError> {
        let mut output = TokenStream::new();
        for member in members {
            if let Some(value) = member.default_value.as_ref() {
                let val = self.value_to_tokens(
                    value,
                    Some(&self.to_rust_title_case(&self.type_to_tokens(&member.ty)?.to_string())),
                )?;
                let ty = self.type_to_tokens(&member.ty)?;
                let method_name =
                    TokenStream::from_str(&self.default_method_name(parent_name, &member.name))?;
                output.append_all(quote! {
                    fn #method_name() -> #ty {
                        #val
                    }
                });
            }
        }
        Ok(output)
    }

    pub(crate) fn type_to_tokens(&self, ty: &ASN1Type) -> Result<TokenStream, GeneratorError> {
        match ty {
            ASN1Type::Null => Ok(quote!(())),
            ASN1Type::Boolean(_) => Ok(quote!(bool)),
            ASN1Type::Integer(i) => Ok(i.int_type().to_token_stream()),
            ASN1Type::Real(_) => Ok(quote!(f64)),
            ASN1Type::BitString(_) => Ok(quote!(BitString)),
            ASN1Type::OctetString(_) => Ok(quote!(OctetString)),
            ASN1Type::CharacterString(CharacterString { ty, .. }) => self.string_type(ty),
            ASN1Type::Enumerated(_) => Err(error!(
                NotYetInplemented,
                "Enumerated values are currently unsupported!"
            )),
            ASN1Type::Choice(_) => Err(error!(
                NotYetInplemented,
                "Choice values are currently unsupported!"
            )),
            ASN1Type::Sequence(_) => Err(error!(
                NotYetInplemented,
                "Sequence values are currently unsupported!"
            )),
            ASN1Type::SetOf(so) | ASN1Type::SequenceOf(so) => {
                let inner = self.type_to_tokens(&so.element_type)?;
                Ok(quote!(SequenceOf<#inner>))
            }
            ASN1Type::ObjectIdentifier(_) => Err(error!(
                NotYetInplemented,
                "Object Identifier values are currently unsupported!"
            )),
            ASN1Type::Set(_) => Err(error!(
                NotYetInplemented,
                "Set values are currently unsupported!"
            )),
            ASN1Type::ElsewhereDeclaredType(e) => Ok(self.to_rust_title_case(&e.identifier)),
            ASN1Type::InformationObjectFieldReference(_) => Err(error!(
                NotYetInplemented,
                "Information Object field reference values are currently unsupported!"
            )),
            ASN1Type::Time(_) => Err(error!(
                NotYetInplemented,
                "Time values are currently unsupported!"
            )),
            ASN1Type::GeneralizedTime(_) => Ok(quote!(GeneralizedTime)),
            ASN1Type::UTCTime(_) => Ok(quote!(UtcTime)),
            ASN1Type::EmbeddedPdv | ASN1Type::External => Ok(quote!(Any)),
            ASN1Type::ChoiceSelectionType(c) => {
                let choice = self.to_rust_title_case(&c.choice_name);
                let option = self.to_rust_enum_identifier(&c.selected_option);
                Ok(quote!(#choice::#option))
            }
        }
    }

    pub(crate) fn value_to_tokens(
        &self,
        value: &ASN1Value,
        type_name: Option<&TokenStream>,
    ) -> Result<TokenStream, GeneratorError> {
        match value {
            ASN1Value::All => Err(error!(
                NotYetInplemented,
                "rasn does not support ALL values."
            )),
            ASN1Value::Null => Ok(quote!(())),
            ASN1Value::Choice {
                type_name: tn,
                variant_name: i,
                inner_value: v,
            } => {
                let rust_ty_name = tn
            .as_ref()
            .map(|t| if t.starts_with(INTERNAL_NESTED_TYPE_NAME_PREFIX) {
                let split: Vec<&str> = t.split('$').collect();
                if split.len() != 3 {
                    Err(GeneratorError {
                        details: format!("Expected three parts in an internal nested choice name, found {split:?}"),
                        ..Default::default()
                    })
                } else {
                    Ok(self.inner_name(split[1], split[2]).to_token_stream())
                }
            } else {
                Ok(self.to_rust_title_case(t))
            }).transpose()?;
                if let Some(ty_n) = rust_ty_name.as_ref().or(type_name) {
                    let option = self.to_rust_enum_identifier(i);
                    let inner = self.value_to_tokens(v, None)?;
                    Ok(quote!(#ty_n::#option(#inner)))
                } else {
                    Err(error!(
                        Unidentified,
                        "A type name is needed to stringify choice value {:?}", value
                    ))
                }
            }
            ASN1Value::OctetString(o) => {
                let bytes = o.iter().map(|byte| Literal::u8_unsuffixed(*byte));
                Ok(quote!(<OctetString as From<&'static [u8]>>::from(&[#(#bytes),*])))
            }
            ASN1Value::SequenceOrSet(_) => Err(error!(
                Unidentified,
                "Unexpectedly encountered unlinked struct-like ASN1 value!"
            )),
            ASN1Value::LinkedStructLikeValue(fields) => {
                if let Some(ty_n) = type_name {
                    let tokenized_fields = fields
                        .iter()
                        .map(|(_, ty, val)| {
                            self.value_to_tokens(val.value(), self.type_to_tokens(ty).ok().as_ref())
                        })
                        .collect::<Result<Vec<TokenStream>, _>>()?;
                    Ok(quote!(#ty_n ::new(#(#tokenized_fields),*)))
                } else {
                    Err(error!(
                        Unidentified,
                        "A type name is needed to stringify sequence value {:?}", value
                    ))
                }
            }
            ASN1Value::Boolean(b) => Ok(b.to_token_stream()),
            ASN1Value::Integer(i) => Ok(Literal::i128_unsuffixed(*i).to_token_stream()),
            ASN1Value::String(s) => Ok(s.to_token_stream()),
            ASN1Value::Real(r) => Ok(r.to_token_stream()),
            ASN1Value::BitString(b) => {
                let bits = b.iter().map(|bit| bit.to_token_stream());
                Ok(quote!([#(#bits),*].into_iter().collect()))
            }
            ASN1Value::EnumeratedValue {
                enumerated,
                enumerable,
            } => {
                let enum_name = self.to_rust_title_case(enumerated);
                let enumerable_id = self.to_rust_enum_identifier(enumerable);
                Ok(quote!(#enum_name::#enumerable_id))
            }
            ASN1Value::LinkedElsewhereDefinedValue { identifier: e, .. }
            | ASN1Value::ElsewhereDeclaredValue { identifier: e, .. } => {
                Ok(self.to_rust_const_case(e).to_token_stream())
            }
            ASN1Value::ObjectIdentifier(oid) => {
                let arcs = oid
                    .0
                    .iter()
                    .filter_map(|arc| arc.number.map(|id| id.to_token_stream()));
                Ok(quote!(Oid::const_new(&[#(#arcs),*]).to_owned()))
            }
            ASN1Value::Time(t) => match type_name {
                Some(time_type) => Ok(quote!(#t.parse::<#time_type>().unwrap())),
                None => Ok(quote!(#t.parse::<_>().unwrap())),
            },
            ASN1Value::LinkedArrayLikeValue(seq) => {
                let elems = seq
                    .iter()
                    .map(|v| self.value_to_tokens(v, None))
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(quote!(alloc::vec![#(#elems),*]))
            }
            ASN1Value::LinkedNestedValue { supertypes, value } => {
                fn nester(generator: &Rasn, s: TokenStream, mut types: Vec<String>) -> TokenStream {
                    match types.pop() {
                        Some(t) => {
                            let ident = generator.to_rust_title_case(&t);
                            nester(generator, quote!(#ident(#s)), types)
                        }
                        None => s,
                    }
                }
                Ok(nester(
                    self,
                    self.value_to_tokens(value, type_name)?,
                    supertypes.clone(),
                ))
            }
            ASN1Value::LinkedIntValue {
                integer_type,
                value,
            } => {
                let val = Literal::i128_unsuffixed(*value);
                match integer_type {
                    IntegerType::Unbounded => Ok(quote!(Integer::from(#val))),
                    _ => Ok(val.to_token_stream()),
                }
            }
            ASN1Value::LinkedCharStringValue(string_type, value) => {
                let val = value.to_token_stream();
                match string_type {
                    CharacterStringType::NumericString => {
                        Ok(quote!(NumericString::try_from(#val).unwrap()))
                    }
                    CharacterStringType::VisibleString => {
                        Ok(quote!(VisibleString::try_from(#val).unwrap()))
                    }
                    CharacterStringType::IA5String => {
                        Ok(quote!(Ia5String::try_from(#val).unwrap()))
                    }
                    CharacterStringType::UTF8String => Ok(quote!(String::from(#val))),
                    CharacterStringType::BMPString => {
                        Ok(quote!(BmpString::try_from(#val).unwrap()))
                    }
                    CharacterStringType::PrintableString => {
                        Ok(quote!(PrintableString::try_from(#val).unwrap()))
                    }
                    CharacterStringType::GeneralString => {
                        Ok(quote!(GeneralString::try_from(String::from(#val)).unwrap()))
                    }
                    CharacterStringType::VideotexString
                    | CharacterStringType::GraphicString
                    | CharacterStringType::UniversalString
                    | CharacterStringType::TeletexString => Err(GeneratorError::new(
                        None,
                        &format!("{:?} values are currently unsupported!", string_type),
                        GeneratorErrorType::NotYetInplemented,
                    )),
                }
            }
        }
    }

    pub(crate) fn format_nested_sequence_members(
        &self,
        sequence_or_set: &SequenceOrSet,
        parent_name: &String,
    ) -> Result<Vec<TokenStream>, GeneratorError> {
        sequence_or_set
            .members
            .iter()
            .filter(|m| self.needs_unnesting(&m.ty))
            .map(|m| {
                self.generate_tld(ToplevelDefinition::Type(ToplevelTypeDefinition {
                    parameterization: None,
                    comments: " Inner type ".into(),
                    name: self.inner_name(&m.name, parent_name).to_string(),
                    ty: m.ty.clone(),
                    tag: None,
                    index: None,
                }))
            })
            .collect::<Result<Vec<_>, _>>()
    }

    pub(crate) fn needs_unnesting(&self, ty: &ASN1Type) -> bool {
        match ty {
            ASN1Type::Enumerated(_)
            | ASN1Type::Choice(_)
            | ASN1Type::Sequence(_)
            | ASN1Type::Set(_) => true,
            ASN1Type::SequenceOf(SequenceOrSetOf { element_type, .. })
            | ASN1Type::SetOf(SequenceOrSetOf { element_type, .. }) => {
                self.needs_unnesting(element_type)
            }
            _ => false,
        }
    }

    pub(crate) fn format_nested_choice_options(
        &self,
        choice: &Choice,
        parent_name: &str,
    ) -> Result<Vec<TokenStream>, GeneratorError> {
        choice
            .options
            .iter()
            .filter(|m| {
                matches!(
                    m.ty,
                    ASN1Type::Enumerated(_)
                        | ASN1Type::Choice(_)
                        | ASN1Type::Sequence(_)
                        | ASN1Type::Set(_)
                )
            })
            .map(|m| {
                self.generate_tld(ToplevelDefinition::Type(ToplevelTypeDefinition {
                    parameterization: None,
                    comments: " Inner type ".into(),
                    name: self.inner_name(&m.name, parent_name).to_string(),
                    ty: m.ty.clone(),
                    tag: None,
                    index: None,
                }))
            })
            .collect::<Result<Vec<_>, _>>()
    }
    

    pub(crate) fn format_new_impl(
        &self,
        name: &TokenStream,
        name_types: Vec<NameType>,
    ) -> TokenStream {
        let args = name_types.iter().map(|nt| {
            let name = &nt.name;
            let ty = &nt.typ;
            quote!(#name: #ty)
        });
        let instance = name_types.iter().map(|nt| &nt.name);
        quote! {
            impl #name {
                pub fn new(#(#args),*) -> Self {
                    Self { #(#instance),* }
                }
            }
        }
    }

    pub(crate) fn format_sequence_or_set_of_item_type(
        &self,
        ty: &ASN1Type,
        first_item: Option<&ASN1Value>,
    ) -> Result<TokenStream, GeneratorError> {
        if ty.is_builtin_type() {
            match ty {
                ASN1Type::Null => Ok(quote!(())),
                ASN1Type::Boolean(_) => Ok(quote!(bool)),
                ASN1Type::Integer(_) => {
                    match first_item {
                        Some(ASN1Value::LinkedIntValue { integer_type, .. }) => {
                            Ok(integer_type.to_token_stream())
                        }
                        _ => Ok(quote!(Integer)), // best effort
                    }
                }
                ASN1Type::BitString(_) => Ok(quote!(BitString)),
                ASN1Type::OctetString(_) => Ok(quote!(OctetString)),
                ASN1Type::GeneralizedTime(_) => Ok(quote!(GeneralizedTime)),
                ASN1Type::UTCTime(_) => Ok(quote!(UtcTime)),
                ASN1Type::ObjectIdentifier(_) => Ok(quote!(ObjectIdentifier)),
                ASN1Type::CharacterString(cs) => match cs.ty {
                    CharacterStringType::NumericString => Ok(quote!(NumericString)),
                    CharacterStringType::VisibleString => Ok(quote!(VisibleString)),
                    CharacterStringType::IA5String => Ok(quote!(IA5String)),
                    CharacterStringType::UTF8String => Ok(quote!(UTF8String)),
                    CharacterStringType::BMPString => Ok(quote!(BMPString)),
                    CharacterStringType::PrintableString => Ok(quote!(PrintableString)),
                    CharacterStringType::GeneralString => Ok(quote!(GeneralString)),
                    CharacterStringType::VideotexString
                    | CharacterStringType::GraphicString
                    | CharacterStringType::UniversalString
                    | CharacterStringType::TeletexString => Err(GeneratorError::new(
                        None,
                        &format!("{:?} values are currently unsupported!", cs.ty),
                        GeneratorErrorType::NotYetInplemented,
                    )),
                },
                _ => Ok(self.to_rust_title_case(&ty.as_str())),
            }
        } else {
            Ok(self.to_rust_title_case(&ty.as_str()))
        }
    }

    /// Resolves the custom syntax declared in an information object class' WITH SYNTAX clause
    pub(crate) fn resolve_standard_syntax(
        &self,
        class: &InformationObjectClass,
        application: &[InformationObjectField],
    ) -> Result<(ASN1Value, Vec<(usize, ASN1Type)>), GeneratorError> {
        let mut key = None;
        let mut field_index_map = Vec::<(usize, ASN1Type)>::new();

        let key_index = class
            .fields
            .iter()
            .enumerate()
            .find_map(|(i, f)| f.is_unique.then_some(i))
            .ok_or_else(|| GeneratorError {
                details: format!("Could not find key for class {class:?}"),
                kind: GeneratorErrorType::MissingClassKey,
                ..Default::default()
            })?;

        let mut appl_iter = application.iter().enumerate();
        'syntax_matching: for class_field in &class.fields {
            if let Some((index, field)) = appl_iter.next() {
                if class_field.identifier.identifier() == field.identifier() {
                    match field {
                        InformationObjectField::TypeField(f) => {
                            field_index_map.push((index, f.ty.clone()));
                        }
                        InformationObjectField::FixedValueField(f) => {
                            if index == key_index {
                                key = Some(f.value.clone());
                            }
                        }
                        InformationObjectField::ObjectSetField(_) => todo!(),
                    }
                } else if !class_field.is_optional {
                    return Err(GeneratorError {
                        top_level_declaration: None,
                        details: "Syntax mismatch while resolving information object.".to_string(),
                        kind: GeneratorErrorType::SyntaxMismatch,
                    });
                } else {
                    continue 'syntax_matching;
                }
            }
        }
        field_index_map.sort_by(|&(a, _), &(b, _)| a.cmp(&b));
        let types = field_index_map.into_iter().collect();
        match key {
            Some(k) => Ok((k, types)),
            None => Err(GeneratorError {
                top_level_declaration: None,
                details: "Could not find class key!".into(),
                kind: GeneratorErrorType::MissingClassKey,
            }),
        }
    }

    const RUST_KEYWORDS: [&'static str; 39] = [
        "as", "async", "await", "break", "const", "continue", "crate", "dyn", "else", "enum",
        "extern", "false", "final", "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod", "move",
        "mut", "pub", "ref", "return", "self", "Self", "static", "struct", "super", "trait",
        "true", "type", "unsafe", "use", "where", "while",
    ];

    pub(crate) fn to_rust_snake_case(&self, input: &str) -> Ident {
        let input = input.replace('-', "_");
        let mut lowercase = String::with_capacity(input.len());

        let peekable = &mut input.chars().peekable();
        while let Some(c) = peekable.next() {
            if c.is_lowercase() || c == '_' || c.is_numeric() {
                lowercase.push(c);
                if peekable.peek().map_or(false, |next| next.is_uppercase()) {
                    lowercase.push('_');
                }
            } else {
                lowercase.push(c.to_ascii_lowercase());
            }
        }
        let name = if Self::RUST_KEYWORDS.contains(&lowercase.as_str()) {
            String::from("r_") + &lowercase
        } else {
            lowercase
        };
        Ident::new(&name, Span::call_site())
    }

    pub(crate) fn to_rust_const_case(&self, input: &str) -> Ident {
        Ident::new(
            &self.to_rust_snake_case(input).to_string().to_uppercase(),
            Span::call_site(),
        )
    }

    pub(crate) fn to_rust_enum_identifier(&self, input: &str) -> Ident {
        let mut formatted = format_ident!("{}", input.replace('-', "_"));
        if Self::RUST_KEYWORDS.contains(&input) {
            formatted = format_ident!("R_{formatted}");
        }
        formatted
    }

    pub(crate) fn to_rust_title_case(&self, input: &str) -> TokenStream {
        let mut input = input.replace('-', "_");
        let input = input.drain(..).fold(String::new(), |mut acc, c| {
            if acc.is_empty() && c.is_lowercase() {
                acc.push(c.to_ascii_uppercase());
            } else if acc.ends_with(|last: char| last == '_') && c.is_uppercase() {
                acc.pop();
                acc.push(c);
            } else if acc.ends_with(|last: char| last == '_') {
                acc.pop();
                acc.push(c.to_ascii_uppercase());
            } else {
                acc.push(c);
            }
            acc
        });
        let name = if Self::RUST_KEYWORDS.contains(&input.as_str()) {
            String::from("R_") + &input
        } else {
            input
        };
        TokenStream::from_str(&name).unwrap()
    }
}

fn boxed_type(tokens: TokenStream) -> TokenStream {
    quote!(Box<#tokens>)
}

impl ASN1Value {
    pub(crate) fn is_const_type(&self) -> bool {
        match self {
            ASN1Value::Null | ASN1Value::Boolean(_) | ASN1Value::EnumeratedValue { .. } => true,
            ASN1Value::Choice { inner_value, .. } => inner_value.is_const_type(),
            ASN1Value::LinkedIntValue { integer_type, .. } => {
                integer_type != &IntegerType::Unbounded
            }
            ASN1Value::LinkedNestedValue { value, .. } => value.is_const_type(),
            ASN1Value::LinkedElsewhereDefinedValue { can_be_const, .. } => *can_be_const,
            _ => false,
        }
    }
}

impl ASN1Type {
    pub(crate) fn is_const_type(&self) -> bool {
        match self {
            ASN1Type::Null | ASN1Type::Enumerated(_) | ASN1Type::Boolean(_) => true,
            ASN1Type::Integer(i) => {
                i.constraints.iter().fold(IntegerType::Unbounded, |acc, c| {
                    acc.max_restrictive(c.integer_constraints())
                }) != IntegerType::Unbounded
            }
            ASN1Type::Choice(c) => c
                .options
                .iter()
                .fold(true, |acc, opt| opt.ty.is_const_type() && acc),
            ASN1Type::Set(s) | ASN1Type::Sequence(s) => s
                .members
                .iter()
                .fold(true, |acc, m| m.ty.is_const_type() && acc),
            ASN1Type::SetOf(s) | ASN1Type::SequenceOf(s) => s.element_type.is_const_type(),
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use quote::quote;

    use crate::intermediate::{
        constraints::ElementSet,
        types::{Boolean, Enumeral, Integer},
        AsnTag,
    };

    use super::*;

    #[test]
    fn determines_int_type() {
        let generator = Rasn::default();
        assert_eq!(generator.int_type_token(Some(600), Some(600), false), "u16");
        assert_eq!(generator.int_type_token(Some(0), Some(0), false), "u8");
        assert_eq!(generator.int_type_token(Some(-1), Some(1), false), "i8");
        assert_eq!(
            generator.int_type_token(Some(0), Some(124213412341389457931857915125), false),
            "Integer"
        );
        assert_eq!(
            generator.int_type_token(Some(-67463), Some(23123), false),
            "i32"
        );
        assert_eq!(generator.int_type_token(Some(255), Some(257), false), "u16");
    }

    #[test]
    fn joins_annotations() {
        let generator = Rasn::default();

        assert_eq_ignore_ws!(
            generator
                .join_annotations(vec![
                    quote!(delegate),
                    generator.format_tag(
                        Some(&AsnTag {
                            tag_class: crate::intermediate::TagClass::Application,
                            environment: crate::intermediate::TaggingEnvironment::Explicit,
                            id: 3,
                        }),
                        false,
                    ),
                ])
                .to_string(),
            "#[rasn(delegate, tag(explicit(application, 3)))]"
        )
    }

    #[test]
    fn formats_sequence_members() {
        let generator = Rasn::default();

        assert_eq_ignore_ws!(
            generator
                .format_sequence_or_set_members(
                    &SequenceOrSet {
                        components_of: vec![],
                        extensible: Some(1),
                        constraints: vec![],
                        members: vec![
                            SequenceOrSetMember {
                                is_recursive: false,
                                name: "testMember0".into(),
                                tag: None,
                                ty: ASN1Type::Boolean(Boolean {
                                    constraints: vec![]
                                }),
                                default_value: None,
                                is_optional: true,
                                constraints: vec![]
                            },
                            SequenceOrSetMember {
                                is_recursive: false,
                                name: "testMember1".into(),
                                tag: None,
                                ty: ASN1Type::Integer(Integer {
                                    distinguished_values: None,
                                    constraints: vec![Constraint::SubtypeConstraint(ElementSet {
                            extensible: false,
                            set: crate::intermediate::constraints::ElementOrSetOperation::Element(
                                crate::intermediate::constraints::SubtypeElement::SingleValue {
                                    value: ASN1Value::Integer(4),
                                    extensible: true
                                }
                            )
                        })]
                                }),
                                default_value: Some(ASN1Value::Integer(4)),
                                is_optional: true,
                                constraints: vec![]
                            }
                        ]
                    },
                    &"Parent".into(),
                )
                .unwrap()
                .0
                .to_string(),
            r#"
                #[rasn(identifier = "testMember0")]
                pub test_member0: Option<bool>,
                #[rasn(extension_addition, value("4", extensible), default = "parent_test_member1_default", identifier = "testMember1")]
                pub test_member1: Integer,
            "#
        );
    }

    #[test]
    fn formats_enum_members() {
        let generator = Rasn::default();

        assert_eq_ignore_ws!(
            generator
                .format_enum_members(&Enumerated {
                    members: vec![
                        Enumeral {
                            name: "test-option-1".into(),
                            description: Some("optional comment".into()),
                            index: 0
                        },
                        Enumeral {
                            name: "test-option-2".into(),
                            description: Some("another optional comment".into()),
                            index: 2
                        },
                        Enumeral {
                            name: "test-option-3".into(),
                            description: None,
                            index: 5
                        }
                    ],
                    extensible: Some(2),
                    constraints: vec![]
                })
                .to_string(),
            r#"
            #[rasn(identifier="test-option-1")]
            test_option_1=0,
            #[rasn(identifier="test-option-2")]
            test_option_2=2,
            #[rasn(extension_addition,identifier="test-option-3")]
            test_option_3=5,
            "#
        )
    }

    #[test]
    fn formats_choice_options() {
        let generator = Rasn::default();

        assert_eq_ignore_ws!(
            generator.format_choice_options(
                &Choice {
                    extensible: Some(1),
                    constraints: vec![],
                    options: vec![
                        ChoiceOption {
is_recursive: false,
                            name: "testMember0".into(),
                            tag: None,
                            ty: ASN1Type::Boolean(Boolean {
                                constraints: vec![]
                            }),
                            constraints: vec![]
                        },
                        ChoiceOption {
is_recursive: false,
                            name: "testMember1".into(),
                            tag: None,
                            ty: ASN1Type::Integer(Integer {
                                distinguished_values: None,
                                constraints: vec![Constraint::SubtypeConstraint(ElementSet {
                                    extensible: false,
                                    set: crate::intermediate::constraints::ElementOrSetOperation::Element(
                                        crate::intermediate::constraints::SubtypeElement::SingleValue {
                                            value: ASN1Value::Integer(4),
                                            extensible: true
                                        }
                                    )
                                })]
                            }),
                            constraints: vec![]
                        }
                    ]
                },
                &"Parent".into(),
            )
            .unwrap()
            .to_string(),
            r#"
                testMember0(bool),
                #[rasn(extension_addition, value("4", extensible))]
                testMember1(Integer),
            "#
        );
    }

    #[test]
    fn formats_linked_value() {
        let generator = Rasn::default();

        assert_eq!(
            generator
                .value_to_tokens(
                    &ASN1Value::LinkedNestedValue {
                        supertypes: vec!["Outer".into(), "Inner".into()],
                        value: Box::new(ASN1Value::Boolean(true))
                    },
                    None
                )
                .unwrap()
                .to_string(),
            "Outer (Inner (true))"
        )
    }

    #[test]
    fn formats_identifier_annotation() {
        let generator = Rasn::default();

        assert_eq!(
            generator
                .format_identifier_annotation(
                    "original-name",
                    "",
                    &ASN1Type::Boolean(Boolean::default())
                )
                .to_string(),
            quote!(identifier = "original-name").to_string()
        );
        assert_eq!(
            generator
                .format_identifier_annotation(
                    "ext_group_original-name",
                    "",
                    &ASN1Type::Boolean(Boolean::default())
                )
                .to_string(),
            quote!(identifier = "BOOLEAN").to_string()
        );
        assert_eq!(
            generator
                .format_identifier_annotation(
                    "original-name",
                    " Anonymous ",
                    &ASN1Type::Boolean(Boolean::default())
                )
                .to_string(),
            quote!(identifier = "BOOLEAN").to_string()
        );
        assert_eq!(
            generator
                .format_identifier_annotation(
                    "original-name",
                    " Inner type ",
                    &ASN1Type::Boolean(Boolean::default())
                )
                .to_string(),
            quote!(identifier = "BOOLEAN").to_string()
        );
    }

    #[test]
    fn converts_to_snake_case() {
        let generator = Rasn::default();

        assert_eq!(generator.to_rust_snake_case("HelloWorld"), "hello_world");
        assert_eq!(generator.to_rust_snake_case("helloWorld"), "hello_world");
        assert_eq!(generator.to_rust_snake_case("hello-world"), "hello_world");
        assert_eq!(generator.to_rust_snake_case("HELLOWORLD"), "helloworld");
        assert_eq!(generator.to_rust_snake_case("HelloWORLD"), "hello_world");
        assert_eq!(generator.to_rust_snake_case("HELLO-WORLD"), "hello__world");
        assert_eq!(generator.to_rust_snake_case("struct"), "r_struct");
        assert_eq!(generator.to_rust_snake_case("STRUCT"), "r_struct");
    }
}
