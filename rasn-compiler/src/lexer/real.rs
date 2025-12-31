use crate::{input::Input, intermediate::*};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit0, digit1, i32, i64, one_of},
    combinator::{map, not, opt, peek, recognize, value},
    sequence::{delimited, preceded},
    Parser,
};

use super::{
    common::{in_braces, into_inner, skip_ws_and_comments},
    constraint::constraints,
    error::ParserResult,
};

pub fn real_value(input: Input<'_>) -> ParserResult<'_, ASN1Value> {
    map(
        skip_ws_and_comments(alt((realnumber, mbe_notation))),
        ASN1Value::Real,
    )
    .parse(input)
}

/// Tries to parse an ASN1 REAL
///
/// *`input` - [Input]-wrapped string slice to be matched against
///
/// `real` will try to match an REAL declaration in the `input` string.
/// If the match succeeds, the lexer will consume the match and return the remaining string
/// and a wrapped `Real` value representing the ASN1 declaration.
/// If the match fails, the lexer will not consume the input and will return an error.
pub fn real(input: Input<'_>) -> ParserResult<'_, ASN1Type> {
    map(
        preceded(
            skip_ws_and_comments(tag(REAL)),
            opt(skip_ws_and_comments(constraints)),
        ),
        |m| ASN1Type::Real(m.into()),
    )
    .parse(input)
}

/// Parse a `realnumber` lexical item.
///
/// Note: Unlike the spec, this also parses negative numbers.
///
/// _X.680 12.9 Real numbers_
/// A "realnumber" shall consist of an integer part that is a series of one or more digits, and
/// optionally a decimal point (.). The decimal point can optionally be followed by a fractional
/// part which is one or more digits. The integer part, decimal point or fractional part (whichever
/// is last present) can optionally be followed by an e or E and an optionally-signed exponent
/// which is one or more digits. The leading digit of the "realnumber" shall not be zero unless it
/// is either the only digit or is immediately followed by a decimal point followed by a fractional
/// part of which at least one digit is not zero.
///
/// A "number" is also a valid instance of "realnumber". In analyzing an instance of use of this
/// notation, a "realnumber" is distinguished from a "number" by the context in which it appears.
fn realnumber(input: Input<'_>) -> ParserResult<'_, f64> {
    into_inner(recognize((
        // Optional sign
        opt(char('-')),
        // Integer part
        alt((
            // Either a single zero,
            recognize((char('0'), not(peek(digit1)))),
            // or starts with a non-zero digit.
            recognize((one_of("123456789"), digit0)),
        )),
        // Fractional part
        opt((char('.'), digit0)),
        // Exponent part
        opt((alt((char('e'), char('E'))), opt(char('-')), digit1)),
    )))
    .map_res(str::parse::<f64>)
    .parse(input)
}

fn mbe_notation(input: Input<'_>) -> ParserResult<'_, f64> {
    map(
        in_braces((
            delimited(
                tag("mantissa"),
                skip_ws_and_comments(i64),
                skip_ws_and_comments(char(COMMA)),
            ),
            delimited(
                skip_ws_and_comments(tag("base")),
                skip_ws_and_comments(alt((value(2i64, char('2')), value(10i64, tag("10"))))),
                skip_ws_and_comments(char(COMMA)),
            ),
            preceded(
                skip_ws_and_comments(tag("exponent")),
                skip_ws_and_comments(i32),
            ),
        )),
        |(mantissa, base, exponent)| mantissa as f64 * (base as f64).powf(exponent as f64),
    )
    .parse(input)
}

#[cfg(test)]
mod tests {
    use crate::intermediate::{
        constraints::{
            ComponentPresence, Constraint, ElementOrSetOperation, ElementSetSpecs,
            InnerTypeConstraint, NamedConstraint, SubtypeElements,
        },
        types::Real,
        ASN1Type, ASN1Value,
    };

    use crate::lexer::real::real_value;

    use super::{real, realnumber};

    #[test]
    fn parses_simple_real_type() {
        assert_eq!(
            real(
                r#" REAL -- Nothing here
        NextType ::= TestType"#
                    .into()
            )
            .unwrap()
            .1,
            ASN1Type::Real(Real {
                constraints: vec![]
            })
        )
    }

    #[test]
    fn parses_constraint_real_type() {
        assert_eq!(
            real(
                r#"REAL (WITH COMPONENTS {
                    mantissa (-16777215..16777215),
                    base (2),
                    exponent (-125..128) } )"#
                    .into()
            )
            .unwrap()
            .1,
            ASN1Type::Real(Real {
                constraints: vec![Constraint::Subtype(ElementSetSpecs {
                    set: ElementOrSetOperation::Element(SubtypeElements::MultipleTypeConstraints(
                        InnerTypeConstraint {
                            is_partial: false,
                            constraints: vec![
                                NamedConstraint {
                                    identifier: "mantissa".into(),
                                    constraints: vec![Constraint::Subtype(ElementSetSpecs {
                                        set: ElementOrSetOperation::Element(
                                            SubtypeElements::ValueRange {
                                                min: Some(ASN1Value::Integer(-16777215)),
                                                max: Some(ASN1Value::Integer(16777215)),
                                                extensible: false
                                            }
                                        ),
                                        extensible: false
                                    })],
                                    presence: ComponentPresence::Unspecified
                                },
                                NamedConstraint {
                                    identifier: "base".into(),
                                    constraints: vec![Constraint::Subtype(ElementSetSpecs {
                                        set: ElementOrSetOperation::Element(
                                            SubtypeElements::SingleValue {
                                                value: ASN1Value::Integer(2),
                                                extensible: false
                                            }
                                        ),
                                        extensible: false
                                    })],
                                    presence: ComponentPresence::Unspecified
                                },
                                NamedConstraint {
                                    identifier: "exponent".into(),
                                    constraints: vec![Constraint::Subtype(ElementSetSpecs {
                                        set: ElementOrSetOperation::Element(
                                            SubtypeElements::ValueRange {
                                                min: Some(ASN1Value::Integer(-125)),
                                                max: Some(ASN1Value::Integer(128)),
                                                extensible: false
                                            }
                                        ),
                                        extensible: false
                                    })],
                                    presence: ComponentPresence::Unspecified
                                }
                            ]
                        }
                    )),
                    extensible: false
                })]
            })
        )
    }

    #[test]
    fn parses_realnumber() {
        assert_eq!(realnumber("0".into()).unwrap().1, 0.0);
        assert_eq!(realnumber("-0".into()).unwrap().1, -0.0);
        assert_eq!(realnumber("0.".into()).unwrap().1, 0.0);
        assert_eq!(realnumber("-0.".into()).unwrap().1, -0.0);
        assert_eq!(realnumber("0.0".into()).unwrap().1, 0.0);
        assert_eq!(realnumber("-0.0".into()).unwrap().1, -0.0);
        assert_eq!(realnumber("0.0e0".into()).unwrap().1, 0.0);
        assert_eq!(realnumber("-0.0E0".into()).unwrap().1, -0.0);
        assert!(realnumber("0123".into()).is_err());
        assert_eq!(realnumber("1234567890".into()).unwrap().1, 1234567890.0);
        assert_eq!(realnumber("-1234567890".into()).unwrap().1, -1234567890.0);
        assert_eq!(realnumber("0.0123456789".into()).unwrap().1, 0.0123456789);
        assert_eq!(realnumber("-0.0123456789".into()).unwrap().1, -0.0123456789);
        assert_eq!(realnumber("123E10".into()).unwrap().1, 123e10);
        assert_eq!(realnumber("-123e10".into()).unwrap().1, -123e10);
        assert_eq!(realnumber("123.e10".into()).unwrap().1, 123e10);
        assert_eq!(realnumber("-123.E10".into()).unwrap().1, -123e10);
        assert_eq!(realnumber("0.0123E10".into()).unwrap().1, 0.0123e10);
        assert_eq!(realnumber("-0.0123e10".into()).unwrap().1, -0.0123e10);
        assert_eq!(realnumber("0.0123e-10".into()).unwrap().1, 0.0123e-10);
        assert_eq!(realnumber("-0.0123E-10".into()).unwrap().1, -0.0123e-10);
    }

    #[test]
    fn parses_mbe_notation_real_value() {
        if let ASN1Value::Real(r) = real_value("{mantissa 334159, base 10, exponent -5}".into())
            .unwrap()
            .1
        {
            assert!(r - 3.34159 < 0.0000001);
        } else {
            unreachable!()
        }
        if let ASN1Value::Real(r) = real_value("{mantissa 0, base 2, exponent 100}".into())
            .unwrap()
            .1
        {
            assert!(r < 0.0000001 && r > -0.000001);
        } else {
            unreachable!()
        }
    }
}
