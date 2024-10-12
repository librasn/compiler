use crate::{input::Input, intermediate::*};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, i32, i64, u64},
    combinator::{map, opt, value},
    error::context,
    sequence::{delimited, preceded, separated_pair, tuple},
    IResult,
};

use super::{
    common::{in_braces, skip_ws_and_comments},
    constraint::constraint,
    error::ParserResult,
};

pub fn real_value(input: Input<'_>) -> ParserResult<'_, ASN1Value> {
    map(
        skip_ws_and_comments(alt((dot_notation, mbe_notation))),
        ASN1Value::Real,
    )(input)
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
    context(
        "RealType",
        map(
            preceded(
                skip_ws_and_comments(tag(REAL)),
                opt(skip_ws_and_comments(constraint)),
            ),
            |m| ASN1Type::Real(m.into()),
        ),
    )(input)
}

fn dot_notation(input: Input<'_>) -> ParserResult<'_, f64> {
    map(
        skip_ws_and_comments(separated_pair(i64, char('.'), u64)),
        |(wholes, decimals)| {
            let mut decimals_f64 = decimals as f64;
            while decimals_f64 > 1. {
                decimals_f64 /= 10.
            }
            if wholes >= 0 {
                wholes as f64 + decimals_f64
            } else {
                wholes as f64 - decimals_f64
            }
        },
    )(input)
}

fn mbe_notation(input: Input<'_>) -> ParserResult<'_, f64> {
    map(
        in_braces(tuple((
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
        ))),
        |(mantissa, base, exponent)| mantissa as f64 * (base as f64).powf(exponent as f64),
    )(input)
}

#[cfg(test)]
mod tests {
    use crate::intermediate::{
        constraints::{
            ComponentPresence, ConstrainedComponent, Constraint, ElementOrSetOperation, ElementSet,
            InnerTypeConstraint, SubtypeElement,
        },
        types::Real,
        ASN1Type, ASN1Value,
    };

    use crate::lexer::real::real_value;

    use super::real;

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
                constraints: vec![Constraint::SubtypeConstraint(ElementSet {
                    set: ElementOrSetOperation::Element(SubtypeElement::SingleTypeConstraint(
                        InnerTypeConstraint {
                            is_partial: false,
                            constraints: vec![
                                ConstrainedComponent {
                                    identifier: "mantissa".into(),
                                    constraints: vec![Constraint::SubtypeConstraint(ElementSet {
                                        set: ElementOrSetOperation::Element(
                                            SubtypeElement::ValueRange {
                                                min: Some(ASN1Value::Integer(-16777215)),
                                                max: Some(ASN1Value::Integer(16777215)),
                                                extensible: false
                                            }
                                        ),
                                        extensible: false
                                    })],
                                    presence: ComponentPresence::Unspecified
                                },
                                ConstrainedComponent {
                                    identifier: "base".into(),
                                    constraints: vec![Constraint::SubtypeConstraint(ElementSet {
                                        set: ElementOrSetOperation::Element(
                                            SubtypeElement::SingleValue {
                                                value: ASN1Value::Integer(2),
                                                extensible: false
                                            }
                                        ),
                                        extensible: false
                                    })],
                                    presence: ComponentPresence::Unspecified
                                },
                                ConstrainedComponent {
                                    identifier: "exponent".into(),
                                    constraints: vec![Constraint::SubtypeConstraint(ElementSet {
                                        set: ElementOrSetOperation::Element(
                                            SubtypeElement::ValueRange {
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
    fn parses_dot_notation_real_value() {
        assert_eq!(
            real_value("2.23412".into()).unwrap().1,
            ASN1Value::Real(2.23412)
        );
        assert_eq!(
            real_value("-12.23412".into()).unwrap().1,
            ASN1Value::Real(-12.23412)
        )
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
