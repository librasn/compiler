use crate::intermediate::{
    constraints::Constraint,
    encoding_rules::per_visible::{
        per_visible_range_constraints, CharsetSubset, PerVisibleAlphabetConstraints,
    },
    types::{Choice, ChoiceOption, Enumerated, SequenceOrSet, SequenceOrSetMember},
    utils::{to_rust_snake_case, to_rust_title_case},
    ASN1Type, ASN1Value, AsnTag, CharacterStringType, TagClass, TaggingEnvironment,
    ToplevelDeclaration, ToplevelTypeDeclaration,
};

use crate::generator::{error::GeneratorError, generate};

use super::error::GeneratorErrorType;

pub struct StringifiedNameType {
    name: String,
    typ: String,
}

pub fn inner_name(name: &String, parent_name: &String) -> String {
    format!("{}{}", parent_name, to_rust_title_case(&name))
}

pub fn int_type_token(opt_min: Option<i128>, opt_max: Option<i128>) -> &'static str {
    if let (Some(min), Some(max)) = (opt_min, opt_max) {
        crate::intermediate::utils::int_type_token(min, max)
    } else {
        "Integer"
    }
}

pub fn format_comments(comments: &String) -> String {
    if comments.is_empty() {
        String::from("")
    } else {
        String::from("///") + &comments.replace("\n", "\n ///") + "\n"
    }
}

pub fn format_range_annotations(
    signed: bool,
    constraints: &Vec<Constraint>,
) -> Result<String, GeneratorError> {
    if constraints.is_empty() {
        return Ok(String::new());
    }
    let per_constraints = per_visible_range_constraints(signed, constraints)?;
    let range_prefix = if per_constraints.is_size_constraint() {
        "size"
    } else {
        "value"
    };
    // handle default size constraints
    if per_constraints.is_size_constraint()
        && !per_constraints.is_extensible()
        && per_constraints.min::<i128>() == Some(0)
        && per_constraints.max::<i128>().is_none()
    {
        return Ok(String::new());
    }
    Ok(
        match (
            per_constraints.min::<i128>(),
            per_constraints.max::<i128>(),
            per_constraints.is_extensible(),
        ) {
            (Some(min), Some(max), true) if min == max => {
                format!(r#"{range_prefix}("{min}", extensible)"#)
            }
            (Some(min), Some(max), true) => {
                format!(r#"{range_prefix}("{min}..={max}", extensible)"#)
            }
            (Some(min), Some(max), false) if min == max => {
                format!(r#"{range_prefix}("{min}")"#)
            }
            (Some(min), Some(max), false) => {
                format!(r#"{range_prefix}("{min}..={max}")"#)
            }
            (Some(min), None, true) => {
                format!(r#"{range_prefix}("{min}..", extensible)"#)
            }
            (Some(min), None, false) => format!(r#"{range_prefix}("{min}..")"#),
            (None, Some(max), true) => {
                format!(r#"{range_prefix}("..={max}", extensible)"#)
            }
            (None, Some(max), false) => format!(r#"{range_prefix}("..={max}")"#),
            _ => format!(r#""#),
        },
    )
}

pub fn format_alphabet_annotations(
    string_type: CharacterStringType,
    constraints: &Vec<Constraint>,
) -> Result<String, GeneratorError> {
    if constraints.is_empty() {
        return Ok(String::new());
    }
    let mut permitted_alphabet = PerVisibleAlphabetConstraints::default_for(string_type);
    for c in constraints {
        PerVisibleAlphabetConstraints::try_new(c, string_type)?
            .map(|mut p| permitted_alphabet += &mut p);
    }
    permitted_alphabet.finalize();
    let alphabet_unicode: Vec<String> = permitted_alphabet
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
        .collect();
    Ok(if alphabet_unicode.is_empty() {
        "".into()
    } else {
        String::from(", from(") + &alphabet_unicode.join(", ") + ")"
    })
}

pub fn format_enum_members(enumerated: &Enumerated) -> String {
    let first_extension_index = enumerated.extensible;
    enumerated.members.iter().fold(String::new(), |acc, e| {
        let rust_name = to_rust_title_case(&e.name);
        let name = if acc.contains(&format!(r#" {rust_name} = "#)) {
            e.name.replace("-", "_")
        } else {
            rust_name
        };
        let index = e.index;
        let extension = if index >= first_extension_index.map_or(i128::MAX, |x| x as i128) {
            r#"#[rasn(extension_addition)]
            "#
        } else {
            ""
        };
        acc + &format!(
            r#"{extension} {name} = {index},
                "#
        )
    })
}

pub fn format_tag(tag: Option<&AsnTag>, fallback: String) -> String {
    if let Some(tag) = tag {
        let class = match tag.tag_class {
            TagClass::Universal => "universal, ",
            TagClass::Application => "application, ",
            TagClass::Private => "private, ",
            TagClass::ContextSpecific => "context, ",
        };
        let (exp_pre, exp_post) = if tag.environment == TaggingEnvironment::Explicit {
            ("explicit(", ")")
        } else {
            ("", "")
        };
        let id = tag.id;
        format!("tag({exp_pre}{class}{id}{exp_post})")
    } else {
        fallback
    }
}

pub fn format_sequence_or_set_members(
    sequence_or_set: &SequenceOrSet,
    parent_name: &String,
) -> Result<(String, Vec<StringifiedNameType>), GeneratorError> {
    let first_extension_index = sequence_or_set.extensible;
    sequence_or_set.members.iter().enumerate().try_fold(
        (String::new(), Vec::new()),
        |mut acc, (i, m)| {
            let extension_annotation = if i >= first_extension_index.unwrap_or(usize::MAX)
                && m.name.starts_with("ext_group_")
            {
                "extension_addition_group"
            } else if i >= first_extension_index.unwrap_or(usize::MAX) {
                "extension_addition"
            } else {
                ""
            };
            format_sequence_member(m, parent_name, extension_annotation).map(
                |(declaration, name_type)| {
                    acc.0.push_str(&declaration);
                    acc.0.push_str(
                        r#",
                    "#,
                    );
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
    extension_annotation: &str,
) -> Result<(String, StringifiedNameType), GeneratorError> {
    let name = to_rust_snake_case(&member.name);
    let (mut all_constraints, mut formatted_type_name) =
        constraints_and_type_name(&member.r#type, &member.name, parent_name)?;
    all_constraints.append(&mut member.constraints.clone());
    if member.is_optional && member.default_value.is_none() {
        formatted_type_name = String::from("Option<") + &formatted_type_name + ">";
    }
    let default_annotation = if member.default_value.is_some() {
        format!(
            r#"default = "{}""#,
            default_method_name(parent_name, &member.name)
        )
    } else {
        String::new()
    };
    let range_annotations = format_range_annotations(
        matches!(member.r#type, ASN1Type::Integer(_)),
        &all_constraints,
    )?;
    let alphabet_annotations = if let ASN1Type::CharacterString(c_string) = &member.r#type {
        format_alphabet_annotations(c_string.r#type, &all_constraints)?
    } else {
        "".into()
    };
    let tag = format_tag(member.tag.as_ref(), String::new());
    let annotations = join_annotations(vec![
        extension_annotation.to_string(),
        range_annotations,
        alphabet_annotations,
        tag,
        default_annotation,
    ]);
    Ok((
        format!(r#"{annotations}pub {name}: {formatted_type_name}"#),
        StringifiedNameType {
            name,
            typ: formatted_type_name,
        },
    ))
}

pub fn format_choice_options(
    choice: &Choice,
    parent_name: &String,
) -> Result<String, GeneratorError> {
    let first_extension_index = choice.extensible;
    choice
        .options
        .iter()
        .enumerate()
        .try_fold(String::new(), |acc, (i, o)| {
            let extension_annotation = if i >= first_extension_index.unwrap_or(usize::MAX)
                && o.name.starts_with("ext_group_")
            {
                "extension_addition_group"
            } else if i >= first_extension_index.unwrap_or(usize::MAX) {
                "extension_addition"
            } else {
                ""
            };
            let rust_name = to_rust_title_case(&o.name);
            let name = if acc.contains(&format!(" {rust_name}(")) {
                o.name.replace("-", "_")
            } else {
                rust_name
            };
            format_choice_option(name, o, parent_name, extension_annotation).map(|opt| acc + &opt)
        })
}

fn format_choice_option(
    name: String,
    member: &ChoiceOption,
    parent_name: &String,
    extension_annotation: &str,
) -> Result<String, GeneratorError> {
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
        "".into()
    };
    let tag = format_tag(member.tag.as_ref(), String::new());
    let annotations = join_annotations(vec![
        extension_annotation.to_string(),
        range_annotations,
        alphabet_annotations,
        tag,
    ]);
    Ok(format!(
        r#"{annotations} {name}({formatted_type_name}),
    "#
    ))
}

fn constraints_and_type_name(
    ty: &ASN1Type,
    name: &String,
    parent_name: &String,
) -> Result<(Vec<Constraint>, String), GeneratorError> {
    Ok(match ty {
        ASN1Type::Null => (vec![], "()".into()),
        ASN1Type::Boolean(b) => (b.constraints.clone(), "bool".into()),
        ASN1Type::Integer(i) => {
            let per_constraints = per_visible_range_constraints(true, &i.constraints)?;
            (
                i.constraints.clone(),
                int_type_token(per_constraints.min(), per_constraints.max()).into(),
            )
        }
        ASN1Type::Real(_) => (vec![], "f64".into()),
        ASN1Type::ObjectIdentifier(o) => (o.constraints.clone(), "Oid".into()),
        ASN1Type::BitString(b) => (b.constraints.clone(), "BitString".into()),
        ASN1Type::OctetString(o) => (o.constraints.clone(), "OctetString".into()),
        ASN1Type::GeneralizedTime(o) => (o.constraints.clone(), "GeneralizedTime".into()),
        ASN1Type::UTCTime(o) => (o.constraints.clone(), "UtcTime".into()),
        ASN1Type::CharacterString(c) => (c.constraints.clone(), string_type(&c.r#type)?),
        ASN1Type::Enumerated(_)
        | ASN1Type::Choice(_)
        | ASN1Type::Sequence(_)
        | ASN1Type::SequenceOf(_)
        | ASN1Type::SetOf(_)
        | ASN1Type::Set(_) => (vec![], inner_name(name, parent_name)),
        ASN1Type::ElsewhereDeclaredType(e) => {
            (e.constraints.clone(), to_rust_title_case(&e.identifier))
        }
        ASN1Type::InformationObjectFieldReference(_)
        | ASN1Type::EmbeddedPdv
        | ASN1Type::External => (vec![], "Any".into()),
        ASN1Type::ChoiceSelectionType(_) => unreachable!()
    })
}

pub fn string_type(c_type: &CharacterStringType) -> Result<String, GeneratorError> {
    match c_type {
        CharacterStringType::NumericString => Ok("NumericString".into()),
        CharacterStringType::VisibleString => Ok("VisibleString".into()),
        CharacterStringType::IA5String => Ok("Ia5String".into()),
        CharacterStringType::TeletexString => Ok("TeletexString".into()),
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
        CharacterStringType::GeneralString => Ok("GeneralString".into()),
        CharacterStringType::UniversalString => Err(GeneratorError {
            kind: GeneratorErrorType::NotYetInplemented,
            details: "UniversalString is currently unsupported!".into(),
            top_level_declaration: None,
        }),
        CharacterStringType::UTF8String => Ok("Utf8String".into()),
        CharacterStringType::BMPString => Ok("BmpString".into()),
        CharacterStringType::PrintableString => Ok("PrintableString".into()),
    }
}

pub fn join_annotations(strings: Vec<String>) -> String {
    match strings
        .into_iter()
        .filter(|s| !s.is_empty())
        .collect::<Vec<String>>()
        .join(",")
    {
        s if s.is_empty() => String::new(),
        s => {
            String::from("#[rasn(")
                + &s
                + r#")]
        "#
        }
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
) -> Result<String, GeneratorError> {
    let mut output = String::new();
    for member in members {
        if let Some(value) = member.default_value.as_ref() {
            let (value_as_string, type_as_string) = match &member.r#type {
                ASN1Type::BitString(_) => (
                    format!("{}.iter().collect()", value.value_as_string(None)?),
                    "BitString".into(),
                ),
                ASN1Type::ElsewhereDeclaredType(_)
                    if !(matches!(
                        value,
                        ASN1Value::EnumeratedValue {
                            enumerated: _,
                            enumerable: _
                        }
                    )) =>
                {
                    let stringified_type = member.r#type.as_string()?;
                    (
                        format!("{stringified_type}({})", value.value_as_string(None)?),
                        stringified_type,
                    )
                }
                ty => (
                    value.value_as_string(Some(&to_rust_title_case(&ty.as_string()?)))?,
                    ty.as_string()?,
                ),
            };
            let method_name = default_method_name(parent_name, &member.name);
            output.push_str(&format!(
                r#"fn {method_name}() -> {type_as_string} {{
                {value_as_string}{}
            }}
            
            "#,
                if type_as_string == value_as_string {
                    ""
                } else {
                    ".into()"
                }
            ))
        }
    }
    Ok(output)
}

pub fn format_nested_sequence_members(
    sequence_or_set: &SequenceOrSet,
    parent_name: &String,
) -> Result<String, GeneratorError> {
    Ok(sequence_or_set
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
                name: inner_name(&m.name, parent_name),
                r#type: m.r#type.clone(),
                tag: None,
                index: None,
            }))
        })
        .collect::<Result<Vec<String>, _>>()?
        .join(
            r#"
    
    "#,
        ))
}

pub fn format_nested_choice_options(
    choice: &Choice,
    parent_name: &String,
) -> Result<String, GeneratorError> {
    Ok(choice
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
                name: inner_name(&m.name, parent_name),
                r#type: m.r#type.clone(),
                tag: None,
                index: None,
            }))
        })
        .collect::<Result<Vec<String>, _>>()?
        .join(
            r#"
    
    "#,
        ))
}

pub fn format_new_impl(name: &String, name_types: Vec<StringifiedNameType>) -> String {
    format!(
        r#"impl {name} {{
        pub fn new(
            {}
        ) -> Self {{
            Self {{
                {}
            }}
        }}
    }}"#,
        name_types
            .iter()
            .map(|nt| format!("{}: {},", nt.name, nt.typ))
            .collect::<Vec<String>>()
            .join("\n\t"),
        name_types
            .iter()
            .map(|nt| format!("{},", nt.name))
            .collect::<Vec<String>>()
            .join("\n\t")
    )
}
