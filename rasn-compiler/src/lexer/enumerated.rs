use nom::{
    bytes::complete::tag,
    character::complete::{char, i128},
    combinator::{map, opt},
    multi::{fold_many0, many0},
    sequence::{preceded, terminated},
    Parser,
};

use crate::{
    input::Input,
    intermediate::{constraints::*, types::*, *},
    lexer::{asn1_type, error::ErrorTree, parameterization},
};

use super::{common::*, error::ParserResult};

type EnumeralInput<'s> = (&'s str, Option<i128>, Option<char>, Option<&'s str>);
type EnumeralBody = (
    Vec<Enumeral>,
    Option<ExtensionMarker>,
    Option<Vec<Enumeral>>,
);

pub fn enumerated_value(input: Input<'_>) -> ParserResult<'_, ToplevelValueDefinition> {
    map(
        (
            skip_ws(many0(comment)),
            skip_ws(value_reference),
            skip_ws_and_comments(opt(parameterization)),
            skip_ws_and_comments(asn1_type),
            preceded(assignment, skip_ws_and_comments(value_reference)),
        ),
        |(c, n, params, p, e)| ToplevelValueDefinition {
            comments: c.into_iter().fold(String::new(), |mut acc, s| {
                acc = acc + "\n" + s;
                acc
            }),
            name: n.to_string(),
            parameterization: params,
            associated_type: p.clone(),
            value: ASN1Value::EnumeratedValue {
                enumerated: p.as_str().into_owned(),
                enumerable: e.to_string(),
            },
            index: None,
        },
    )
    .parse(input)
}

/// Tries to parse an ASN1 ENUMERATED
///
/// *`input` - [Input]-wrapped string slice to be matched against
///
/// `enumerated` will try to match an ENUMERATED declaration in the `input` string.
/// If the match succeeds, the lexer will consume the match and return the remaining string
/// and a wrapped `Enumerated` value representing the ASN1 declaration.
/// If the match fails, the lexer will not consume the input and will return an error.
pub fn enumerated(input: Input<'_>) -> ParserResult<'_, ASN1Type> {
    map(
        preceded(skip_ws_and_comments(tag(ENUMERATED)), enumerated_body),
        |m| ASN1Type::Enumerated(m.into()),
    )
    .parse(input)
}

fn enumeral(input: Input<'_>) -> ParserResult<'_, EnumeralInput<'_>> {
    skip_ws_and_comments((
        skip_ws(identifier),
        skip_ws(opt(in_parentheses(skip_ws_and_comments(i128)))),
        opt(skip_ws_and_comments(char(COMMA))),
        skip_ws(opt(comment)),
    ))
    .parse(input)
}

fn enumerals<'a>(
    start_index: usize,
) -> impl Parser<Input<'a>, Output = Vec<Enumeral>, Error = ErrorTree<'a>> {
    fold_many0(
        enumeral,
        Vec::<Enumeral>::new,
        move |mut acc, (name, index, _, comments)| {
            acc.push(Enumeral {
                name: name.into(),
                description: comments.map(|c| c.into()),
                index: index.unwrap_or((acc.len() + start_index) as i128),
            });
            acc
        },
    )
}

fn enumerated_body(input: Input<'_>) -> ParserResult<'_, EnumeralBody> {
    in_braces(|input| {
        let (input, root_enumerals) = enumerals(0).parse(input)?;
        let (input, ext_marker) =
            opt(terminated(extension_marker, opt(char(COMMA)))).parse(input)?;
        let (input, ext_enumerals) = opt(enumerals(root_enumerals.len())).parse(input)?;
        Ok((input, (root_enumerals, ext_marker, ext_enumerals)))
    })
    .parse(input)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn parses_enumerals_with_line_comments() {
        assert_eq!(
            enumerals(0)
                .parse(
                    r#"forward     (1), -- This means forward
      backward    (2), -- This means backward
      unavailable (3)  -- This means nothing
      "#
                    .into()
                )
                .unwrap()
                .1,
            [
                Enumeral {
                    name: "forward".into(),
                    description: Some(" This means forward".into(),),
                    index: 1,
                },
                Enumeral {
                    name: "backward".into(),
                    description: Some(" This means backward".into(),),
                    index: 2,
                },
                Enumeral {
                    name: "unavailable".into(),
                    description: Some(" This means nothing".into(),),
                    index: 3,
                },
            ],
        )
    }

    #[test]
    fn parses_enumerated() {
        assert_eq!(
            enumerated(
                r#"ENUMERATED {
      onePerMeter-0-1,
      outOfRange,
      unavailable
  }"#
                .into()
            )
            .unwrap()
            .1,
            ASN1Type::Enumerated(Enumerated {
                constraints: vec![],
                members: vec![
                    Enumeral {
                        name: "onePerMeter-0-1".into(),
                        description: None,
                        index: 0
                    },
                    Enumeral {
                        name: "outOfRange".into(),
                        description: None,
                        index: 1
                    },
                    Enumeral {
                        name: "unavailable".into(),
                        description: None,
                        index: 2
                    }
                ],
                extensible: None
            })
        )
    }

    #[test]
    fn parses_extended_enumerated() {
        assert_eq!(
            enumerated(
                "ENUMERATED {m1, m2, m3 -- another annoying comment we'll ignore --,...}".into()
            )
            .unwrap()
            .1,
            ASN1Type::Enumerated(Enumerated {
                constraints: vec![],
                members: vec![
                    Enumeral {
                        name: "m1".into(),
                        description: None,
                        index: 0
                    },
                    Enumeral {
                        name: "m2".into(),
                        description: None,
                        index: 1
                    },
                    Enumeral {
                        name: "m3".into(),
                        description: None,
                        index: 2
                    }
                ],
                extensible: Some(3)
            })
        )
    }

    #[test]
    fn parses_extended_enumerated_without_indices() {
        assert_eq!(
            enumerated(r#"ENUMERATED { One, ..., Three }"#.into())
                .unwrap()
                .1,
            ASN1Type::Enumerated(Enumerated {
                constraints: vec![],
                members: vec![
                    Enumeral {
                        name: "One".into(),
                        description: None,
                        index: 0
                    },
                    Enumeral {
                        name: "Three".into(),
                        description: None,
                        index: 1
                    }
                ],
                extensible: Some(1)
            })
        )
    }

    #[test]
    fn parses_enumerated_with_ellipsis() {
        assert_eq!(
            enumerated(
                r#"ENUMERATED {
                permanentCenDsrcTolling (0),
                ...,
                temporaryCenDsrcTolling (1)
            }"#
                .into()
            )
            .unwrap()
            .1,
            ASN1Type::Enumerated(Enumerated {
                constraints: vec![],
                members: vec![
                    Enumeral {
                        name: "permanentCenDsrcTolling".into(),
                        description: None,
                        index: 0
                    },
                    Enumeral {
                        name: "temporaryCenDsrcTolling".into(),
                        description: None,
                        index: 1
                    }
                ],
                extensible: Some(1)
            })
        )
    }

    #[test]
    fn parses_indexed_enumerated() {
        assert_eq!(
            enumerated(
                r#"ENUMERATED {
          forward     (1),--This means forward
          -- Annoyance
          -- another annoyance -- backward    (2), --This means backward
          unavailable (3)--This means nothing
      }"#
                .into()
            )
            .unwrap()
            .1,
            ASN1Type::Enumerated(Enumerated {
                constraints: vec![],
                members: vec![
                    Enumeral {
                        name: "forward".into(),
                        description: Some("This means forward".into()),
                        index: 1
                    },
                    Enumeral {
                        name: "backward".into(),
                        description: Some("This means backward".into()),
                        index: 2
                    },
                    Enumeral {
                        name: "unavailable".into(),
                        description: Some("This means nothing".into()),
                        index: 3
                    }
                ],
                extensible: None
            })
        )
    }

    #[test]
    fn parses_indexed_extended_enumerated() {
        assert_eq!(
            enumerated(
                r#"ENUMERATED {
          forward  -- this, too, ignored --   (1),
          -- let's consider this a comment concerning 'forward' -- ...
      }"#
                .into()
            )
            .unwrap()
            .1,
            ASN1Type::Enumerated(Enumerated {
                constraints: vec![],
                members: vec![Enumeral {
                    name: "forward".into(),
                    description: Some(
                        " let's consider this a comment concerning 'forward' ".into()
                    ),
                    index: 1
                },],
                extensible: Some(1)
            })
        )
    }

    #[test]
    fn parses_enumerated_value() {
        assert_eq!(
            enumerated_value(
                r#"-- Alias of another enumeral
            enumeral-alias Test-Enum ::= enumeral"#
                    .into()
            )
            .unwrap()
            .1,
            ToplevelValueDefinition {
                comments: String::from("\n Alias of another enumeral"),
                name: String::from("enumeral-alias"),
                associated_type: ASN1Type::ElsewhereDeclaredType(DeclarationElsewhere {
                    parent: None,
                    module: None,
                    identifier: String::from("Test-Enum"),
                    constraints: vec![],
                }),
                parameterization: None,
                value: ASN1Value::EnumeratedValue {
                    enumerated: String::from("Test-Enum"),
                    enumerable: String::from("enumeral")
                },
                index: None
            }
        )
    }
}
