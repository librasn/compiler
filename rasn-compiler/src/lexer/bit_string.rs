use nom::{
    bytes::complete::tag,
    character::complete::{char, one_of},
    combinator::{map, opt},
    multi::fold_many0,
    sequence::{delimited, pair, preceded},
};

use crate::intermediate::*;

use super::{common::*, constraint::constraint, util::hex_to_bools, LexerResult, Span};

/// Parses a BIT STRING value. Currently, the lexer only supports parsing binary and
/// hexadecimal values, but not the named bit notation in curly braces.
pub fn bit_string_value(input: Span) -> LexerResult<ASN1Value> {
    map(
        skip_ws_and_comments(pair(
            delimited(
                char(SINGLE_QUOTE),
                fold_many0(one_of("0123456789ABCDEF"), String::new, |mut acc, curr| {
                    acc.push(curr);
                    acc
                }),
                char(SINGLE_QUOTE),
            ),
            one_of("HB"),
        )),
        |(value, encoding)| {
            if encoding == 'B' {
                ASN1Value::BitString(value.chars().map(|c| c == '1').collect())
            } else {
                ASN1Value::BitString(value.chars().flat_map(hex_to_bools).collect())
            }
        },
    )(input)
}

/// Tries to parse an ASN1 BIT STRING
///
/// *`input` - string slice to be matched against
///
/// `bit_string` will try to match an BIT STRING declaration in the `input` string.
/// If the match succeeds, the lexer will consume the match and return the remaining string
/// and a wrapped `BitString` value representing the ASN1 declaration.
/// If the match fails, the lexer will not consume the input and will return an error.
pub fn bit_string(input: Span) -> LexerResult<ASN1Type> {
    map(
        preceded(
            skip_ws_and_comments(tag(BIT_STRING)),
            pair(opt(distinguished_values), opt(constraint)),
        ),
        |m| ASN1Type::BitString(m.into()),
    )(input)
}

#[cfg(test)]
mod tests {
    use crate::{
        intermediate::{constraints::*, types::*, *},
        lexer::Span,
    };

    use super::bit_string;

    #[test]
    fn parses_unconfined_bitstring() {
        let sample = Span::new("  BIT STRING");
        assert_eq!(
            bit_string(sample).unwrap().1,
            ASN1Type::BitString(BitString {
                distinguished_values: None,
                constraints: vec![]
            })
        )
    }

    #[test]
    fn parses_strictly_constrained_bitstring() {
        let sample = Span::new("  BIT STRING(SIZE (8))");
        assert_eq!(
            bit_string(sample).unwrap().1,
            ASN1Type::BitString(BitString {
                distinguished_values: None,
                constraints: vec![Constraint::SubtypeConstraint(ElementSet {
                    set: ElementOrSetOperation::Element(SubtypeElement::SizeConstraint(Box::new(
                        ElementOrSetOperation::Element(SubtypeElement::SingleValue {
                            value: ASN1Value::Integer(8),
                            extensible: false
                        })
                    ))),
                    extensible: false
                })]
            })
        )
    }

    #[test]
    fn parses_range_constrained_bitstring() {
        let sample = Span::new("  BIT STRING -- even here?!?!? -- (SIZE (8 ..18))");
        assert_eq!(
            bit_string(sample).unwrap().1,
            ASN1Type::BitString(BitString {
                distinguished_values: None,
                constraints: vec![Constraint::SubtypeConstraint(ElementSet {
                    set: ElementOrSetOperation::Element(SubtypeElement::SizeConstraint(Box::new(
                        ElementOrSetOperation::Element(SubtypeElement::ValueRange {
                            min: Some(ASN1Value::Integer(8)),
                            max: Some(ASN1Value::Integer(18)),
                            extensible: false
                        })
                    ))),
                    extensible: false
                })]
            })
        )
    }

    #[test]
    fn parses_strictly_constrained_extended_bitstring() {
        let sample = Span::new("  BIT STRING (SIZE (2, ...))");
        assert_eq!(
            bit_string(sample).unwrap().1,
            ASN1Type::BitString(BitString {
                distinguished_values: None,
                constraints: vec![Constraint::SubtypeConstraint(ElementSet {
                    set: ElementOrSetOperation::Element(SubtypeElement::SizeConstraint(Box::new(
                        ElementOrSetOperation::Element(SubtypeElement::SingleValue {
                            value: ASN1Value::Integer(2),
                            extensible: true
                        })
                    ))),
                    extensible: false
                })]
            })
        )
    }

    #[test]
    fn parses_range_constrained_extended_bitstring() {
        let sample = Span::new("  BIT STRING (SIZE (8 -- comment -- .. 18, ...))");
        assert_eq!(
            bit_string(sample).unwrap().1,
            ASN1Type::BitString(BitString {
                distinguished_values: None,
                constraints: vec![Constraint::SubtypeConstraint(ElementSet {
                    set: ElementOrSetOperation::Element(SubtypeElement::SizeConstraint(Box::new(
                        ElementOrSetOperation::Element(SubtypeElement::ValueRange {
                            min: Some(ASN1Value::Integer(8)),
                            max: Some(ASN1Value::Integer(18)),
                            extensible: true
                        })
                    ))),
                    extensible: false
                })]
            })
        )
    }

    #[test]
    fn parses_bitstring_with_distinguished_values() {
        let sample = Span::new(
            r#"BIT STRING {
          heavyLoad    (0),
          excessWidth  (1),  -- this is excessive
          excessLength (2),  -- this, too
          excessHeight (3) -- and this
      } (SIZE(4))"#,
        );
        assert_eq!(
            bit_string(sample).unwrap().1,
            ASN1Type::BitString(BitString {
                distinguished_values: Some(vec![
                    DistinguishedValue {
                        name: "heavyLoad".into(),
                        value: 0
                    },
                    DistinguishedValue {
                        name: "excessWidth".into(),
                        value: 1
                    },
                    DistinguishedValue {
                        name: "excessLength".into(),
                        value: 2
                    },
                    DistinguishedValue {
                        name: "excessHeight".into(),
                        value: 3
                    },
                ]),
                constraints: vec![Constraint::SubtypeConstraint(ElementSet {
                    set: ElementOrSetOperation::Element(SubtypeElement::SizeConstraint(Box::new(
                        ElementOrSetOperation::Element(SubtypeElement::SingleValue {
                            value: ASN1Value::Integer(4),
                            extensible: false
                        })
                    ))),
                    extensible: false
                })]
            })
        )
    }
}
