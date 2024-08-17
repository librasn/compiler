use crate::intermediate::*;
use nom::{
    bytes::complete::tag,
    combinator::{map, opt},
    sequence::{pair, preceded},
};

use super::{
    asn1_type,
    common::{opt_parentheses, skip_ws_and_comments, value_identifier},
    constraint::constraint,
    LexerResult, Span,
};

/// Tries to parse an ASN1 SET OF
///
/// *`input` - string slice to be matched against
///
/// `set_of` will try to match an SET OF declaration in the `input` string.
/// If the match succeeds, the lexer will consume the match and return the remaining string
/// and a wrapped `SetOf` value representing the ASN1 declaration.
/// If the match fails, the lexer will not consume the input and will return an error.
pub fn set_of(input: Span) -> LexerResult<ASN1Type> {
    map(
        pair(
            preceded(
                skip_ws_and_comments(tag(SET)),
                opt(opt_parentheses(constraint)),
            ),
            preceded(
                skip_ws_and_comments(pair(tag(OF), opt(skip_ws_and_comments(value_identifier)))),
                asn1_type,
            ),
        ),
        |m| ASN1Type::SetOf(m.into()),
    )(input)
}

#[cfg(test)]
mod tests {
    use crate::intermediate::{
        constraints::*,
        information_object::{ObjectSet, ObjectSetValue},
        types::*,
    };

    use super::*;

    #[test]
    fn parses_simple_set_of() {
        assert_eq!(
            set_of(Span::new("SET OF BOOLEAN")).unwrap().1,
            ASN1Type::SetOf(SequenceOrSetOf {
                constraints: vec![],
                element_type: Box::new(ASN1Type::Boolean(Boolean {
                    constraints: vec![]
                }))
            })
        );
    }

    #[test]
    fn parses_simple_set_of_elsewhere_declared_type() {
        assert_eq!(
            set_of(Span::new("SET OF Things")).unwrap().1,
            ASN1Type::SetOf(SequenceOrSetOf {
                constraints: vec![],
                element_type: Box::new(ASN1Type::ElsewhereDeclaredType(DeclarationElsewhere {
                    parent: None,
                    identifier: "Things".into(),
                    constraints: vec![]
                }))
            })
        );
    }

    #[test]
    fn parses_constraint_set_of_elsewhere_declared_type() {
        assert_eq!(
            set_of(Span::new("SET SIZE (1..13,...) OF CorrelationCellValue  "))
                .unwrap()
                .1,
            ASN1Type::SetOf(SequenceOrSetOf {
                constraints: vec![Constraint::SubtypeConstraint(ElementSet {
                    set: ElementOrSetOperation::Element(SubtypeElement::SizeConstraint(Box::new(
                        ElementOrSetOperation::Element(SubtypeElement::ValueRange {
                            min: Some(ASN1Value::Integer(1)),
                            max: Some(ASN1Value::Integer(13)),
                            extensible: true
                        })
                    ))),
                    extensible: false
                })],
                element_type: Box::new(ASN1Type::ElsewhereDeclaredType(DeclarationElsewhere {
                    parent: None,
                    identifier: "CorrelationCellValue".into(),
                    constraints: vec![]
                }))
            })
        );
    }

    #[test]
    fn parses_constraint_set_of_with_extra_parentheses() {
        assert_eq!(
            set_of(Span::new(
                "SET (SIZE (1..13, ...)) OF CorrelationCellValue  "
            ))
            .unwrap()
            .1,
            ASN1Type::SetOf(SequenceOrSetOf {
                constraints: vec![Constraint::SubtypeConstraint(ElementSet {
                    set: ElementOrSetOperation::Element(SubtypeElement::SizeConstraint(Box::new(
                        ElementOrSetOperation::Element(SubtypeElement::ValueRange {
                            min: Some(ASN1Value::Integer(1)),
                            max: Some(ASN1Value::Integer(13)),
                            extensible: true
                        })
                    ))),
                    extensible: false
                })],
                element_type: Box::new(ASN1Type::ElsewhereDeclaredType(DeclarationElsewhere {
                    parent: None,
                    identifier: "CorrelationCellValue".into(),
                    constraints: vec![]
                }))
            })
        );
    }

    #[test]
    fn parses_constraint_set_of_constraint_integer() {
        assert_eq!(
            set_of(Span::new(
                r#"SET SIZE (1..13,...) OF INTEGER {
              one-distinguished-value (12)
            } (1..13,...) "#
            ))
            .unwrap()
            .1,
            ASN1Type::SetOf(SequenceOrSetOf {
                constraints: vec![Constraint::SubtypeConstraint(ElementSet {
                    set: ElementOrSetOperation::Element(SubtypeElement::SizeConstraint(Box::new(
                        ElementOrSetOperation::Element(SubtypeElement::ValueRange {
                            min: Some(ASN1Value::Integer(1)),
                            max: Some(ASN1Value::Integer(13)),
                            extensible: true
                        })
                    ))),
                    extensible: false
                })],
                element_type: Box::new(ASN1Type::Integer(Integer {
                    constraints: vec![Constraint::SubtypeConstraint(ElementSet {
                        set: ElementOrSetOperation::Element(SubtypeElement::ValueRange {
                            min: Some(ASN1Value::Integer(1)),
                            max: Some(ASN1Value::Integer(13)),
                            extensible: true
                        }),
                        extensible: false
                    })],
                    distinguished_values: Some(vec![DistinguishedValue {
                        name: "one-distinguished-value".into(),
                        value: 12
                    }]),
                }))
            })
        );
    }

    #[test]
    fn parses_parameterized_constrained_set_of() {
        assert_eq!(
            set_of(Span::new(
                r#"SET (SIZE(1..4)) OF
      RegionalExtension {{Reg-MapData}} OPTIONAL,"#
            ))
            .unwrap()
            .1,
            ASN1Type::SetOf(SequenceOrSetOf {
                constraints: vec![Constraint::SubtypeConstraint(ElementSet {
                    set: ElementOrSetOperation::Element(SubtypeElement::SizeConstraint(Box::new(
                        ElementOrSetOperation::Element(SubtypeElement::ValueRange {
                            min: Some(ASN1Value::Integer(1)),
                            max: Some(ASN1Value::Integer(4)),
                            extensible: false
                        })
                    ))),
                    extensible: false
                })],
                element_type: Box::new(ASN1Type::ElsewhereDeclaredType(DeclarationElsewhere {
                    parent: None,
                    identifier: "RegionalExtension".into(),
                    constraints: vec![Constraint::Parameter(vec![Parameter::ObjectSetParameter(
                        ObjectSet {
                            values: vec![ObjectSetValue::Reference("Reg-MapData".into())],
                            extensible: None
                        }
                    )])]
                }))
            })
        );
    }

    #[test]
    fn handles_object_field_ref() {
        println!(
            "{:?}",
            set_of(Span::new(
                r#"SET (SIZE(1..MAX)) OF
        IEEE1609DOT2-HEADERINFO-CONTRIBUTED-EXTENSION.&Extn({
        Ieee1609Dot2HeaderInfoContributedExtensions
      }{@.contributorId})"#
            ))
            .unwrap()
            .1
        )
    }
}
