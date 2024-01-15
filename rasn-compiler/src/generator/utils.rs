use std::str::FromStr;

use proc_macro2::{Ident, Literal, Punct, Spacing, TokenStream};
use quote::{format_ident, quote, ToTokens, TokenStreamExt};

use crate::intermediate::{
    constraints::Constraint,
    encoding_rules::per_visible::{
        per_visible_range_constraints, CharsetSubset, PerVisibleAlphabetConstraints,
    },
    information_object::{
        InformationObjectClass, InformationObjectField, ObjectFieldIdentifier, SyntaxApplication,
    },
    types::{Choice, ChoiceOption, Enumerated, SequenceOrSet, SequenceOrSetMember},
    utils::{to_rust_const_case, to_rust_enum_identifier, to_rust_snake_case, to_rust_title_case},
    ASN1Type, ASN1Value, AsnTag, CharacterStringType, IntegerType, TagClass, TaggingEnvironment,
    ToplevelDeclaration, ToplevelTypeDeclaration,
};

use crate::generator::{error::GeneratorError, generate};

use super::error::GeneratorErrorType;

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

impl IntegerType {
    pub fn to_token_stream(self) -> TokenStream {
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

pub fn inner_name(name: &String, parent_name: &String) -> Ident {
    format_ident!("{}{}", parent_name, to_rust_title_case(name).to_string())
}

pub fn int_type_token(opt_min: Option<i128>, opt_max: Option<i128>, is_extensible: bool) -> Ident {
    if let (Some(min), Some(max)) = (opt_min, opt_max) {
        format_ident!(
            "{}",
            crate::intermediate::utils::int_type_token(min, max, is_extensible)
        )
    } else {
        format_ident!("Integer")
    }
}

pub fn format_comments(comments: &String) -> Result<TokenStream, GeneratorError> {
    if comments.is_empty() {
        Ok(TokenStream::new())
    } else {
        let joined = String::from("///") + &comments.replace('\n', "\n ///") + "\n";
        Ok(TokenStream::from_str(&joined)?)
    }
}

pub fn format_range_annotations(
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

pub fn format_alphabet_annotations(
    string_type: CharacterStringType,
    constraints: &Vec<Constraint>,
) -> Result<TokenStream, GeneratorError> {
    if constraints.is_empty() {
        return Ok(TokenStream::new());
    }
    let mut permitted_alphabet = PerVisibleAlphabetConstraints::default_for(string_type);
    for c in constraints {
        PerVisibleAlphabetConstraints::try_new(c, string_type)?
            .map(|mut p| permitted_alphabet += &mut p);
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

pub fn format_enum_members(enumerated: &Enumerated) -> TokenStream {
    let first_extension_index = enumerated.extensible;
    let enumerals = enumerated.members.iter().enumerate().map(|(i, e)| {
        let name = to_rust_enum_identifier(&e.name);
        let index = Literal::i128_unsuffixed(e.index);
        let extension = if i >= first_extension_index.unwrap_or(usize::MAX) {
            quote!(#[rasn(extension_addition)])
        } else {
            TokenStream::new()
        };
        quote!(
            #extension
            #name = #index,
        )
    });
    quote!(#(#enumerals)*)
}

pub fn format_tag(tag: Option<&AsnTag>, fallback_to_automatic: bool) -> TokenStream {
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

pub fn format_sequence_or_set_members(
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
            format_sequence_member(m, parent_name, extension_annotation).map(
                |(declaration, name_type)| {
                    acc.0.append_all([declaration, quote!(, )]);
                    acc.1.push(name_type);
                    acc
                },
            )
        },
    )
}

fn format_sequence_member(
    member: &SequenceOrSetMember,
    parent_name: &String,
    extension_annotation: TokenStream,
) -> Result<(TokenStream, NameType), GeneratorError> {
    let name = to_rust_snake_case(&member.name);
    let (mut all_constraints, mut formatted_type_name) =
        constraints_and_type_name(&member.r#type, &member.name, parent_name)?;
    all_constraints.append(&mut member.constraints.clone());
    if member.is_optional && member.default_value.is_none() {
        formatted_type_name = quote!(Option<#formatted_type_name>);
    }
    let default_annotation = member
        .default_value
        .as_ref()
        .map(|_| {
            let default_fn = default_method_name(parent_name, &member.name);
            quote!(default = #default_fn)
        })
        .unwrap_or_default();
    let range_annotations = format_range_annotations(
        matches!(member.r#type, ASN1Type::Integer(_)),
        &all_constraints,
    )?;
    let alphabet_annotations = if let ASN1Type::CharacterString(c_string) = &member.r#type {
        format_alphabet_annotations(c_string.r#type, &all_constraints)?
    } else {
        TokenStream::new()
    };
    let annotations = join_annotations(vec![
        extension_annotation,
        range_annotations,
        alphabet_annotations,
        format_tag(member.tag.as_ref(), false),
        default_annotation,
    ]);
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

pub fn format_choice_options(
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
            let name = to_rust_enum_identifier(&o.name);
            format_choice_option(name, o, parent_name, extension_annotation)
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(quote!(#(#options)*))
}

fn format_choice_option(
    name: Ident,
    member: &ChoiceOption,
    parent_name: &String,
    extension_annotation: TokenStream,
) -> Result<TokenStream, GeneratorError> {
    let (mut all_constraints, formatted_type_name) =
        constraints_and_type_name(&member.r#type, &member.name, parent_name)?;
    all_constraints.append(&mut member.constraints.clone());
    let range_annotations = format_range_annotations(
        matches!(member.r#type, ASN1Type::Integer(_)),
        &all_constraints,
    )?;
    let alphabet_annotations = if let ASN1Type::CharacterString(c_string) = &member.r#type {
        format_alphabet_annotations(c_string.r#type, &all_constraints)?
    } else {
        TokenStream::new()
    };
    let annotations = join_annotations(vec![
        extension_annotation,
        range_annotations,
        alphabet_annotations,
        format_tag(member.tag.as_ref(), false),
    ]);
    Ok(quote! {
            #annotations
            #name(#formatted_type_name),
    })
}

fn constraints_and_type_name(
    ty: &ASN1Type,
    name: &String,
    parent_name: &String,
) -> Result<(Vec<Constraint>, TokenStream), GeneratorError> {
    Ok(match ty {
        ASN1Type::Null => (vec![], quote!(())),
        ASN1Type::Boolean(b) => (b.constraints.clone(), quote!(bool)),
        ASN1Type::Integer(i) => {
            let per_constraints = per_visible_range_constraints(true, &i.constraints)?;
            (
                i.constraints.clone(),
                int_type_token(
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
        ASN1Type::CharacterString(c) => (c.constraints.clone(), string_type(&c.r#type)?),
        ASN1Type::Enumerated(_)
        | ASN1Type::Choice(_)
        | ASN1Type::Sequence(_)
        | ASN1Type::SequenceOf(_)
        | ASN1Type::SetOf(_)
        | ASN1Type::Set(_) => (vec![], inner_name(name, parent_name).to_token_stream()),
        ASN1Type::ElsewhereDeclaredType(e) => (
            e.constraints.clone(),
            to_rust_title_case(&e.identifier).to_token_stream(),
        ),
        ASN1Type::InformationObjectFieldReference(_)
        | ASN1Type::EmbeddedPdv
        | ASN1Type::External => (vec![], quote!(Any)),
        ASN1Type::ChoiceSelectionType(_) => unreachable!(),
    })
}

pub fn string_type(c_type: &CharacterStringType) -> Result<TokenStream, GeneratorError> {
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

pub fn join_annotations(elements: Vec<TokenStream>) -> TokenStream {
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

pub fn default_method_name(parent_name: &String, field_name: &String) -> String {
    format!(
        "{}_{}_default",
        to_rust_snake_case(parent_name),
        to_rust_snake_case(field_name)
    )
}

pub fn format_default_methods(
    members: &Vec<SequenceOrSetMember>,
    parent_name: &String,
) -> Result<TokenStream, GeneratorError> {
    let mut output = TokenStream::new();
    for member in members {
        if let Some(value) = member.default_value.as_ref() {
            let val = value_to_tokens(
                value,
                Some(&to_rust_title_case(
                    &type_to_tokens(&member.r#type)?.to_string(),
                )),
            )?;
            let ty = type_to_tokens(&member.r#type)?;
            let method_name =
                TokenStream::from_str(&default_method_name(parent_name, &member.name))?;
            output.append_all(quote! {
                fn #method_name() -> #ty {
                    #val
                }
            });
        }
    }
    Ok(output)
}

pub fn type_to_tokens(ty: &ASN1Type) -> Result<TokenStream, GeneratorError> {
    match ty {
        ASN1Type::Null => Ok(quote!(Asn1Null)),
        ASN1Type::Boolean(_) => Ok(quote!(bool)),
        ASN1Type::Integer(i) => Ok(i.int_type().to_token_stream()),
        ASN1Type::Real(_) => Ok(quote!(f64)),
        ASN1Type::BitString(_) => Ok(quote!(BitString)),
        ASN1Type::OctetString(_) => Ok(quote!(OctetString)),
        ASN1Type::CharacterString(_) => Ok(quote!(Utf8String)),
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
            let inner = type_to_tokens(&so.r#type)?;
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
        ASN1Type::ElsewhereDeclaredType(e) => Ok(to_rust_title_case(&e.identifier)),
        ASN1Type::InformationObjectFieldReference(_) => todo!(),
        ASN1Type::Time(_) => todo!(),
        ASN1Type::GeneralizedTime(_) => Ok(quote!(GeneralizedTime)),
        ASN1Type::UTCTime(_) => Ok(quote!(UtcTime)),
        ASN1Type::EmbeddedPdv | ASN1Type::External => Ok(quote!(Any)),
        ASN1Type::ChoiceSelectionType(c) => {
            let choice = to_rust_title_case(&c.choice_name);
            let option = to_rust_enum_identifier(&c.selected_option);
            Ok(quote!(#choice::#option))
        }
    }
}

pub fn value_to_tokens(
    value: &ASN1Value,
    type_name: Option<&TokenStream>,
) -> Result<TokenStream, GeneratorError> {
    match value {
        ASN1Value::All => Err(error!(
            NotYetInplemented,
            "rasn does not support ALL values."
        )),
        ASN1Value::Null => Ok(quote!(())),
        ASN1Value::Choice(i, v) => {
            if let Some(ty_n) = type_name {
                let option = to_rust_enum_identifier(i);
                let inner = value_to_tokens(v, None)?;
                Ok(quote!(#ty_n::#option(#inner)))
            } else {
                Err(error!(
                    Unidentified,
                    "A type name is needed to stringify choice value {:?}", value
                ))
            }
        },
        ASN1Value::OctetString(o) => {
            let bytes = o.iter().map(|byte| Literal::u8_unsuffixed(*byte));
            Ok(quote!(<OctetString as From<&'static [u8]>>::from(&[#(#bytes),*])))
        },
        ASN1Value::SequenceOrSet(fields) => {
            if let Some(ty_n) = type_name {
                let tokenized_fields = fields
                    .iter()
                    .map(|(id, val)| {
                        let ident = format_ident!("{id}");
                        value_to_tokens(val, None).map(|field_val| {
                            quote! {
                                #ident: #field_val
                            }
                        })
                    })
                    .collect::<Result<Vec<TokenStream>, _>>()?;
                Ok(quote!(#ty_n { #(#tokenized_fields),* }))
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
            let enum_name = to_rust_title_case(enumerated);
            let enumerable_id = to_rust_enum_identifier(enumerable);
            Ok(quote!(#enum_name::#enumerable_id))
        }
        ASN1Value::ElsewhereDeclaredValue { identifier: e, .. } => {
            Ok(to_rust_const_case(e).to_token_stream())
        }
        ASN1Value::ObjectIdentifier(oid) => {
            let arcs = oid
                .0
                .iter()
                .filter_map(|arc| arc.number.map(|id| id.to_token_stream()));
            Ok(quote!([#(#arcs),*]))
        }
        ASN1Value::Time(t) => match type_name {
            Some(time_type) => Ok(quote!(#t.parse::<#time_type>().unwrap())),
            None => Ok(quote!(#t.parse::<_>().unwrap()))
        },
        ASN1Value::SequenceOrSetOf(seq) => {
            let elems = seq
                .iter()
                .map(|v| value_to_tokens(v, None))
                .collect::<Result<Vec<_>, _>>()?;
            Ok(quote!(&'static [#(#elems),*].into_iter().collect()))
        }
        ASN1Value::LinkedASN1Value { supertypes, value } => {
            fn nester(s: TokenStream, mut types: Vec<String>) -> TokenStream {
                match types.pop() {
                    Some(t) => {
                        let ident = to_rust_title_case(&t);
                        nester(quote!(#ident(#s)), types)
                    }
                    None => s,
                }
            }
            Ok(nester(
                value_to_tokens(value, type_name)?,
                supertypes.clone(),
            ))
        }
        ASN1Value::LinkedASN1IntValue {
            integer_type,
            value,
        } => {
            let val = Literal::i128_unsuffixed(*value);
            match integer_type {
                IntegerType::Unbounded => Ok(quote!(Integer::from(#val))),
                _ => Ok(val.to_token_stream()),
            }
        }
    }
}

pub fn format_nested_sequence_members(
    sequence_or_set: &SequenceOrSet,
    parent_name: &String,
) -> Result<Vec<TokenStream>, GeneratorError> {
    sequence_or_set
        .members
        .iter()
        .filter(|m| {
            matches!(
                m.r#type,
                ASN1Type::Enumerated(_)
                    | ASN1Type::Choice(_)
                    | ASN1Type::Sequence(_)
                    | ASN1Type::SequenceOf(_)
                    | ASN1Type::Set(_)
            )
        })
        .map(|m| {
            generate(ToplevelDeclaration::Type(ToplevelTypeDeclaration {
                parameterization: None,
                comments: " Inner type ".into(),
                name: inner_name(&m.name, parent_name).to_string(),
                r#type: m.r#type.clone(),
                tag: None,
                index: None,
            }))
        })
        .collect::<Result<Vec<_>, _>>()
}

pub fn format_nested_choice_options(
    choice: &Choice,
    parent_name: &String,
) -> Result<Vec<TokenStream>, GeneratorError> {
    choice
        .options
        .iter()
        .filter(|m| {
            matches!(
                m.r#type,
                ASN1Type::Enumerated(_)
                    | ASN1Type::Choice(_)
                    | ASN1Type::Sequence(_)
                    | ASN1Type::SequenceOf(_)
                    | ASN1Type::Set(_)
            )
        })
        .map(|m| {
            generate(ToplevelDeclaration::Type(ToplevelTypeDeclaration {
                parameterization: None,
                comments: " Inner type ".into(),
                name: inner_name(&m.name, parent_name).to_string(),
                r#type: m.r#type.clone(),
                tag: None,
                index: None,
            }))
        })
        .collect::<Result<Vec<_>, _>>()
}

pub fn format_new_impl(name: &TokenStream, name_types: Vec<NameType>) -> TokenStream {
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

/// Resolves the custom syntax declared in an information object class' WITH SYNTAX clause
pub fn resolve_standard_syntax(
    class: &InformationObjectClass,
    application: &[InformationObjectField],
) -> Result<(ASN1Value, Vec<(usize, ASN1Type)>), GeneratorError> {
    let mut key = None;
    let mut field_index_map = Vec::<(usize, ASN1Type)>::new();

    let mut appl_iter = application.iter().enumerate();
    'syntax_matching: for class_field in &class.fields {
        if let Some((index, field)) = appl_iter.next() {
            if class_field.identifier.identifier() == field.identifier() {
                match field {
                    InformationObjectField::TypeField(f) => {
                        field_index_map.push((index, f.r#type.clone()));
                    }
                    InformationObjectField::FixedValueField(f) => {
                        key = Some(f.value.clone());
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

/// Resolves the custom syntax declared in an information object class' WITH SYNTAX clause
pub fn resolve_custom_syntax(
    class: &InformationObjectClass,
    application: &[SyntaxApplication],
) -> Result<(ASN1Value, Vec<(usize, ASN1Type)>), GeneratorError> {
    let tokens = match &class.syntax {
        Some(s) => s.flatten(),
        None => {
            return Err(GeneratorError {
                top_level_declaration: None,
                details: "No syntax definition for information object class found!".into(),
                kind: GeneratorErrorType::MissingCustomSyntax,
            })
        }
    };

    let mut key = None;
    let mut field_index_map = Vec::<(usize, ASN1Type)>::new();

    let mut application_index = 0;
    'syntax_matching: for (required, token) in &tokens {
        if let Some(expr) = application.get(application_index) {
            if expr.matches(token, &tokens) {
                match expr {
                    SyntaxApplication::ObjectSetDeclaration(_) => todo!(),
                    SyntaxApplication::LiteralOrTypeReference(t) => {
                        if let Some(index) = class.fields.iter().enumerate().find_map(|(i, v)| {
                            (v.identifier
                                == ObjectFieldIdentifier::MultipleValue(
                                    token.name_or_empty().to_owned(),
                                ))
                            .then_some(i)
                        }) {
                            field_index_map
                                .push((index, ASN1Type::ElsewhereDeclaredType(t.clone())))
                        }
                    }
                    SyntaxApplication::TypeReference(t) => {
                        if let Some(index) = class.fields.iter().enumerate().find_map(|(i, v)| {
                            (v.identifier
                                == ObjectFieldIdentifier::MultipleValue(
                                    token.name_or_empty().to_owned(),
                                ))
                            .then_some(i)
                        }) {
                            field_index_map.push((index, t.clone()))
                        }
                    }
                    SyntaxApplication::ValueReference(v) => {
                        if class
                            .fields
                            .iter()
                            .any(|field| {
                                field.identifier
                                    == ObjectFieldIdentifier::SingleValue(
                                        token.name_or_empty().to_owned(),
                                    )
                                    && field.is_unique
                            })
                        {
                            key = Some(v.clone())
                        }
                    }
                    _ => continue 'syntax_matching,
                }
                application_index += 1;
            } else if *required {
                return Err(GeneratorError {
                    top_level_declaration: None,
                    details: "Syntax mismatch while resolving information object.".to_string(),
                    kind: GeneratorErrorType::SyntaxMismatch,
                });
            } else {
                continue 'syntax_matching;
            }
        } else if *required {
            return Err(GeneratorError {
                top_level_declaration: None,
                details: "Syntax mismatch while resolving information object.".to_string(),
                kind: GeneratorErrorType::SyntaxMismatch,
            });
        } else {
            continue 'syntax_matching;
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

impl ASN1Value {
    pub fn is_const_type(&self) -> bool {
        match self {
            ASN1Value::Null |
            ASN1Value::Boolean(_) |
            ASN1Value::EnumeratedValue { .. } => true,
            ASN1Value::Choice(_, v) => v.is_const_type(),
            ASN1Value::LinkedASN1IntValue { integer_type, .. } => integer_type != &IntegerType::Unbounded,
            ASN1Value::LinkedASN1Value { value, .. } => value.is_const_type(),
            _ => false
        }
    }
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

#[cfg(test)]
mod tests {
    use quote::quote;

    use crate::{
        generator::utils::format_tag,
        intermediate::{
            constraints::ElementSet,
            types::{Boolean, Enumeral, Integer},
            AsnTag,
        },
    };

    use super::*;

    #[test]
    fn joins_annotations() {
        assert_eq_ignore_ws!(
            join_annotations(vec![
                quote!(delegate),
                format_tag(
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
        assert_eq_ignore_ws!(
            format_sequence_or_set_members(
                &SequenceOrSet {
                    components_of: vec![],
                    extensible: Some(1),
                    constraints: vec![],
                    members: vec![
                        SequenceOrSetMember {
                            name: "testMember0".into(),
                            tag: None,
                            r#type: ASN1Type::Boolean(Boolean {
                                constraints: vec![]
                            }),
                            default_value: None,
                            is_optional: true,
                            constraints: vec![]
                        },
                        SequenceOrSetMember {
                            name: "testMember1".into(),
                            tag: None,
                            r#type: ASN1Type::Integer(Integer {
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
                pub test_member0: Option<bool>,
                #[rasn(extension_addition, value("4", extensible), default = "parent_test_member1_default")]
                pub test_member1: Integer,
            "#
        );
    }

    #[test]
    fn formats_enum_members() {
        assert_eq_ignore_ws!(
            format_enum_members(&Enumerated {
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
            test_option_1=0,
            test_option_2=2,
            #[rasn(extension_addition)]
            test_option_3=5,
            "#
        )
    }

    #[test]
    fn formats_choice_options() {
        assert_eq_ignore_ws!(
            format_choice_options(
                &Choice {
                    extensible: Some(1),
                    constraints: vec![],
                    options: vec![
                        ChoiceOption {
                            name: "testMember0".into(),
                            tag: None,
                            r#type: ASN1Type::Boolean(Boolean {
                                constraints: vec![]
                            }),
                            constraints: vec![]
                        },
                        ChoiceOption {
                            name: "testMember1".into(),
                            tag: None,
                            r#type: ASN1Type::Integer(Integer {
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
        assert_eq!(
            value_to_tokens(
                &ASN1Value::LinkedASN1Value {
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
}
