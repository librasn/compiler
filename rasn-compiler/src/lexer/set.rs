use nom::{
    bytes::complete::tag,
    character::complete::char,
    combinator::opt,
    multi::many0,
    sequence::{terminated, tuple},
};

use crate::intermediate::*;

use super::{common::optional_comma, constraint::constraint, sequence::sequence_component, *};

/// Tries to parse an ASN1 SET
///
/// *`input` - string slice to be matched against
///
/// `set` will try to match an SET declaration in the `input` string.
/// If the match succeeds, the lexer will consume the match and return the remaining string
/// and a wrapped `Set` value representing the ASN1 declaration. If the defined SET
/// contains anonymous built-in types as members, these nested built-in types will be represented as
/// structs within the same global scope.
/// If the match fails, the lexer will not consume the input and will return an error.
pub fn set(input: Span) -> LexerResult<ASN1Type> {
    map(
        preceded(
            skip_ws_and_comments(tag(SET)),
            pair(
                in_braces(tuple((
                    many0(terminated(
                        skip_ws_and_comments(sequence_component),
                        optional_comma,
                    )),
                    opt(terminated(extension_marker, opt(char(COMMA)))),
                    opt(many0(terminated(
                        skip_ws_and_comments(sequence_component),
                        optional_comma,
                    ))),
                ))),
                opt(constraint),
            ),
        ),
        |m| ASN1Type::Set(m.into()),
    )(input)
}

#[cfg(test)]
mod tests {
    use set::types::{CharacterString, SequenceOrSet, SequenceOrSetMember, SequenceOrSetOf};

    use super::*;

    #[test]
    fn issue_2_test() {
        assert_eq!(
            set(Span::new(
                r#"SET {
            title VisibleString,
            children SEQUENCE OF VisibleString DEFAULT {}
        }"#
            ))
            .unwrap()
            .1,
            ASN1Type::Set(SequenceOrSet {
                components_of: vec![],
                extensible: None,
                constraints: vec![],
                members: vec![
                    SequenceOrSetMember {
                        name: "title".into(),
                        tag: None,
                        ty: ASN1Type::CharacterString(CharacterString {
                            constraints: vec![],
                            ty: CharacterStringType::VisibleString
                        }),
                        default_value: None,
                        is_optional: false,
                        constraints: vec![],
                    },
                    SequenceOrSetMember {
                        name: "children".into(),
                        tag: None,
                        ty: ASN1Type::SequenceOf(SequenceOrSetOf {
                            constraints: vec![],
                            element_type: Box::new(ASN1Type::CharacterString(CharacterString {
                                constraints: vec![],
                                ty: CharacterStringType::VisibleString
                            }))
                        }),
                        default_value: Some(ASN1Value::SequenceOrSet(vec![])),
                        is_optional: true,
                        constraints: vec![]
                    }
                ]
            })
        );
    }
}
