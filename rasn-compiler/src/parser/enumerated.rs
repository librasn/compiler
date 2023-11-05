use nom::{
    bytes::complete::tag,
    character::complete::{char, i128},
    combinator::{map, opt},
    multi::fold_many0,
    sequence::{preceded, terminated, tuple},
    IResult,
};

use crate::intermediate::{constraints::*, types::*, *};

use super::common::*;

/// Tries to parse an ASN1 ENUMERATED
///
/// *`input` - string slice to be matched against
///
/// `enumerated` will try to match an ENUMERATED declaration in the `input` string.
/// If the match succeeds, the parser will consume the match and return the remaining string
/// and a wrapped `Enumerated` value representing the ASN1 declaration.
/// If the match fails, the parser will not consume the input and will return an error.
pub fn enumerated<'a>(input: &'a str) -> IResult<&'a str, ASN1Type> {
    map(
        preceded(skip_ws_and_comments(tag(ENUMERATED)), enumerated_body),
        |m| ASN1Type::Enumerated(m.into()),
    )(input)
}

fn enumeral<'a>(
    input: &'a str,
) -> IResult<&'a str, (&str, Option<i128>, Option<char>, Option<&str>)> {
    skip_ws_and_comments(tuple((
        skip_ws_and_comments(identifier),
        skip_ws_and_comments(opt(in_parentheses(skip_ws_and_comments(i128)))),
        skip_ws(opt(char(COMMA))),
        skip_ws(opt(comment)),
    )))(input)
}

fn enumerals<'a>(start_index: usize) -> impl FnMut(&'a str) -> IResult<&'a str, Vec<Enumeral>> {
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

fn enumerated_body<'a>(
    input: &'a str,
) -> IResult<
    &'a str,
    (
        Vec<Enumeral>,
        Option<ExtensionMarker>,
        Option<Vec<Enumeral>>,
    ),
> {
    in_braces(|input| {
        let (input, root_enumerals) = enumerals(0)(input)?;
        let (input, ext_marker) = opt(terminated(extension_marker, opt(char(COMMA))))(input)?;
        let (input, ext_enumerals) = opt(enumerals(root_enumerals.len()))(input)?;
        Ok((input, (root_enumerals, ext_marker, ext_enumerals)))
    })(input)
}

#[cfg(test)]
mod tests {
    use crate::intermediate::types::*;

    use super::*;

    #[test]
    fn parses_enumerals_with_line_comments() {
        assert_eq!(
            enumerals(0)(
                r#"forward     (1), -- This means forward
      backward    (2), -- This means backward
      unavailable (3)  -- This means nothing
      "#
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
            enumerated("ENUMERATED {m1, m2, m3 -- another annoying comment we'll ignore --,...}")
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
            enumerated(r#"ENUMERATED { One, ..., Three }"#).unwrap().1,
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
}
