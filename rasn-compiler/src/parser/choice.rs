use nom::{
    bytes::complete::tag,
    character::complete::char,
    combinator::{into, opt},
    multi::many0,
    sequence::{separated_pair, terminated, tuple},
    IResult,
};

use crate::intermediate::{types::*, *};

use super::{constraint::constraint, *};

pub fn choice_value<'a>(input: &'a str) -> IResult<&'a str, ASN1Value> {
    map(
        skip_ws_and_comments(separated_pair(identifier, char(':'), asn1_value)),
        |(id, val)| ASN1Value::Choice(id.to_owned(), Box::new(val)),
    )(input)
}

/// Tries to parse an ASN1 CHOICE
///
/// *`input` - string slice to be matched against
///
/// `sequence` will try to match an CHOICE declaration in the `input` string.
/// If the match succeeds, the parser will consume the match and return the remaining string
/// and a wrapped `Choice` value representing the ASN1 declaration. If the defined CHOICE
/// contains anonymous members, these nested members will be represented as
/// structs within the same global scope.
/// If the match fails, the parser will not consume the input and will return an error.
pub fn choice<'a>(input: &'a str) -> IResult<&'a str, ASN1Type> {
    map(
        preceded(
            skip_ws_and_comments(tag(CHOICE)),
            in_braces(tuple((
                many0(terminated(
                    skip_ws_and_comments(choice_option),
                    optional_comma,
                )),
                opt(terminated(
                    extension_marker,
                    opt(skip_ws_and_comments(char(COMMA))),
                )),
                opt(many0(terminated(
                    skip_ws_and_comments(choice_option),
                    optional_comma,
                ))),
            ))),
        ),
        |m| ASN1Type::Choice(m.into()),
    )(input)
}

fn choice_option<'a>(input: &'a str) -> IResult<&'a str, ChoiceOption> {
    into(tuple((
        skip_ws_and_comments(identifier),
        opt(asn_tag),
        skip_ws_and_comments(asn1_type),
        opt(skip_ws_and_comments(constraint)),
    )))(input)
}

#[cfg(test)]
mod tests {
    use crate::intermediate::{
        types::{Choice, ChoiceOption},
        ASN1Type,
    };

    use crate::parser::choice;

    #[test]
    fn parses_extensible_choice() {
        assert_eq!(
            choice(
                r#"CHOICE
    {normal NULL,
    high NULL,
    ...,
    medium NULL }"#
            )
            .unwrap()
            .1,
            ASN1Type::Choice(Choice {
                extensible: Some(2),
                options: vec![
                    ChoiceOption {
                        name: "normal".into(),
                        tag: None,
                        r#type: ASN1Type::Null,
                        constraints: vec![]
                    },
                    ChoiceOption {
                        name: "high".into(),
                        tag: None,
                        r#type: ASN1Type::Null,
                        constraints: vec![]
                    },
                    ChoiceOption {
                        name: "medium".into(),
                        tag: None,
                        r#type: ASN1Type::Null,
                        constraints: vec![]
                    }
                ],
                constraints: vec![]
            })
        )
    }
}
