use std::borrow::Cow;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, one_of},
    combinator::{map, opt},
    multi::{fold_many0, separated_list0},
    sequence::{delimited, pair, preceded},
    Parser,
};

use crate::{input::Input, intermediate::*};

use super::{common::*, constraint::constraints, error::ParserResult, util::hex_to_bools};

/// Parses a BIT STRING value. Currently, the lexer only supports parsing binary and
/// hexadecimal values, but not the named bit notation in curly braces.
pub fn bit_string_value(input: Input<'_>) -> ParserResult<'_, ASN1Value> {
    alt((
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
        ),
        map(
            skip_ws_and_comments(delimited(
                char(LEFT_BRACE),
                separated_list0(char(','), skip_ws_and_comments(value_reference)),
                char(RIGHT_BRACE),
            )),
            |named_bits| {
                ASN1Value::BitStringNamedBits(named_bits.into_iter().map(Cow::Borrowed).collect())
            },
        ),
    ))
    .parse(input)
}

/// Tries to parse an ASN1 BIT STRING
///
/// *`input` - [Input]-wrapped string slice to be matched against
///
/// `bit_string` will try to match an BIT STRING declaration in the `input` string.
/// If the match succeeds, the lexer will consume the match and return the remaining string
/// and a wrapped `BitString` value representing the ASN1 declaration.
/// If the match fails, the lexer will not consume the input and will return an error.
pub fn bit_string(input: Input<'_>) -> ParserResult<'_, ASN1Type> {
    map(
        preceded(
            skip_ws_and_comments(tag(BIT_STRING)),
            pair(opt(distinguished_values), opt(constraints)),
        ),
        |m| ASN1Type::BitString(m.into()),
    )
    .parse(input)
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use crate::{
        intermediate::{constraints::*, types::*, *},
        lexer::{bit_string, bit_string_value},
    };

    #[test]
    fn parses_unconfined_bitstring() {
        let sample = "  BIT STRING".into();
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
        let sample = "  BIT STRING(SIZE (8))".into();
        assert_eq!(
            bit_string(sample).unwrap().1,
            ASN1Type::BitString(BitString {
                distinguished_values: None,
                constraints: vec![Constraint::Subtype(ElementSetSpecs {
                    set: ElementOrSetOperation::Element(SubtypeElements::SizeConstraint(Box::new(
                        ElementOrSetOperation::Element(SubtypeElements::SingleValue {
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
        let sample = "  BIT STRING -- even here?!?!? -- (SIZE (8 ..18))".into();
        assert_eq!(
            bit_string(sample).unwrap().1,
            ASN1Type::BitString(BitString {
                distinguished_values: None,
                constraints: vec![Constraint::Subtype(ElementSetSpecs {
                    set: ElementOrSetOperation::Element(SubtypeElements::SizeConstraint(Box::new(
                        ElementOrSetOperation::Element(SubtypeElements::ValueRange {
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
        let sample = "  BIT STRING (SIZE (2, ...))".into();
        assert_eq!(
            bit_string(sample).unwrap().1,
            ASN1Type::BitString(BitString {
                distinguished_values: None,
                constraints: vec![Constraint::Subtype(ElementSetSpecs {
                    set: ElementOrSetOperation::Element(SubtypeElements::SizeConstraint(Box::new(
                        ElementOrSetOperation::Element(SubtypeElements::SingleValue {
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
        let sample = "  BIT STRING (SIZE (8 -- comment -- .. 18, ...))".into();
        assert_eq!(
            bit_string(sample).unwrap().1,
            ASN1Type::BitString(BitString {
                distinguished_values: None,
                constraints: vec![Constraint::Subtype(ElementSetSpecs {
                    set: ElementOrSetOperation::Element(SubtypeElements::SizeConstraint(Box::new(
                        ElementOrSetOperation::Element(SubtypeElements::ValueRange {
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
        let sample = r#"BIT STRING {
          heavyLoad    (0),
          excessWidth  (1),  -- this is excessive
          excessLength (2),  -- this, too
          excessHeight (3) -- and this
      } (SIZE(4))"#
            .into();
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
                constraints: vec![Constraint::Subtype(ElementSetSpecs {
                    set: ElementOrSetOperation::Element(SubtypeElements::SizeConstraint(Box::new(
                        ElementOrSetOperation::Element(SubtypeElements::SingleValue {
                            value: ASN1Value::Integer(4),
                            extensible: false
                        })
                    ))),
                    extensible: false
                })]
            })
        )
    }

    #[test]
    fn parses_named_bits() {
        assert_eq!(
            ASN1Value::BitStringNamedBits(vec![Cow::from("blue"), Cow::from("yellow")]),
            bit_string_value(r#"{blue, yellow}"#.into()).unwrap().1
        )
    }
}
