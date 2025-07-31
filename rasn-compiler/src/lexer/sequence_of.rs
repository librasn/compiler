use crate::{input::Input, intermediate::*};
use nom::{
    bytes::complete::tag,
    combinator::{map, opt},
    sequence::{pair, preceded},
    Parser,
};

use super::{
    asn1_type,
    common::{asn_tag, opt_parentheses, skip_ws_and_comments, value_reference},
    constraint::constraints,
    error::ParserResult,
};

/// Tries to parse an ASN1 SEQUENCE OF
///
/// *`input` - [Input]-wrapped string slice to be matched against
///
/// `sequence_of` will try to match an SEQUENCE OF declaration in the `input` string.
/// If the match succeeds, the lexer will consume the match and return the remaining string
/// and a wrapped `SequenceOf` type representing the ASN1 declaration.
/// If the match fails, the lexer will not consume the input and will return an error.
pub fn sequence_of(input: Input<'_>) -> ParserResult<'_, ASN1Type> {
    map(
        pair(
            preceded(
                skip_ws_and_comments(tag(SEQUENCE)),
                opt(opt_parentheses(constraints)),
            ),
            preceded(
                skip_ws_and_comments(pair(tag(OF), opt(skip_ws_and_comments(value_reference)))),
                skip_ws_and_comments(pair(opt(asn_tag), skip_ws_and_comments(asn1_type))),
            ),
        ),
        |m| ASN1Type::SequenceOf(m.into()),
    )
    .parse(input)
}

#[cfg(test)]
mod tests {
    use crate::intermediate::{
        constraints::*,
        information_object::{ObjectSet, ObjectSetValue},
        types::*,
        *,
    };

    use crate::lexer::sequence_of;

    #[test]
    fn parses_simple_sequence_of() {
        assert_eq!(
            sequence_of("SEQUENCE OF BOOLEAN".into()).unwrap().1,
            ASN1Type::SequenceOf(SequenceOrSetOf {
                element_tag: None,
                is_recursive: false,
                constraints: vec![],
                element_type: Box::new(ASN1Type::Boolean(Boolean {
                    constraints: vec![]
                }))
            })
        );
    }

    #[test]
    fn parses_simple_sequence_of_elsewhere_declared_type() {
        assert_eq!(
            sequence_of("SEQUENCE OF Things".into()).unwrap().1,
            ASN1Type::SequenceOf(SequenceOrSetOf {
                element_tag: None,
                is_recursive: false,
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
    fn parses_constraint_sequence_of_elsewhere_declared_type() {
        assert_eq!(
            sequence_of("SEQUENCE SIZE (1..13,...) OF CorrelationCellValue  ".into())
                .unwrap()
                .1,
            ASN1Type::SequenceOf(SequenceOrSetOf {
                element_tag: None,
                is_recursive: false,
                constraints: vec![Constraint::Subtype(ElementSetSpecs {
                    set: ElementOrSetOperation::Element(SubtypeElements::SizeConstraint(Box::new(
                        ElementOrSetOperation::Element(SubtypeElements::ValueRange {
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
    fn parses_constraint_sequence_of_with_extra_parentheses() {
        assert_eq!(
            sequence_of("SEQUENCE (SIZE (1..13, ...)) OF CorrelationCellValue  ".into())
                .unwrap()
                .1,
            ASN1Type::SequenceOf(SequenceOrSetOf {
                element_tag: None,
                is_recursive: false,
                constraints: vec![Constraint::Subtype(ElementSetSpecs {
                    set: ElementOrSetOperation::Element(SubtypeElements::SizeConstraint(Box::new(
                        ElementOrSetOperation::Element(SubtypeElements::ValueRange {
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
    fn parses_constraint_sequence_of_constraint_integer() {
        assert_eq!(
            sequence_of(
                r#"SEQUENCE SIZE (1..13,...) OF INTEGER {
              one-distinguished-value (12)
            } (1..13,...) "#
                    .into()
            )
            .unwrap()
            .1,
            ASN1Type::SequenceOf(SequenceOrSetOf {
                element_tag: None,
                is_recursive: false,
                constraints: vec![Constraint::Subtype(ElementSetSpecs {
                    set: ElementOrSetOperation::Element(SubtypeElements::SizeConstraint(Box::new(
                        ElementOrSetOperation::Element(SubtypeElements::ValueRange {
                            min: Some(ASN1Value::Integer(1)),
                            max: Some(ASN1Value::Integer(13)),
                            extensible: true
                        })
                    ))),
                    extensible: false
                })],
                element_type: Box::new(ASN1Type::Integer(Integer {
                    constraints: vec![Constraint::Subtype(ElementSetSpecs {
                        set: ElementOrSetOperation::Element(SubtypeElements::ValueRange {
                            min: Some(ASN1Value::Integer(1)),
                            max: Some(ASN1Value::Integer(13)),
                            extensible: true
                        }),
                        extensible: false
                    })],
                    distinguished_values: Some(vec![DistinguishedValue {
                        name: "one-distinguished-value".into(),
                        value: 12
                    }])
                }))
            })
        );
    }

    #[test]
    fn parses_parameterized_constrained_sequence_of() {
        assert_eq!(
            sequence_of(
                r#"SEQUENCE (SIZE(1..4)) OF
        RegionalExtension {{Reg-MapData}} OPTIONAL,"#
                    .into()
            )
            .unwrap()
            .1,
            ASN1Type::SequenceOf(SequenceOrSetOf {
                element_tag: None,
                is_recursive: false,
                constraints: vec![Constraint::Subtype(ElementSetSpecs {
                    set: ElementOrSetOperation::Element(SubtypeElements::SizeConstraint(Box::new(
                        ElementOrSetOperation::Element(SubtypeElements::ValueRange {
                            min: Some(ASN1Value::Integer(1)),
                            max: Some(ASN1Value::Integer(4)),
                            extensible: false
                        })
                    ))),
                    extensible: false
                })],
                element_type: Box::new(ASN1Type::ElsewhereDeclaredType(DeclarationElsewhere {
                    parent: None,
                    constraints: Vec::new(),
                    identifier: DefinedType::ParameterizedTypeOrValueSetType {
                        simple_defined_type: Box::new("RegionalExtension".into()),
                        actual_parameter_list: vec![Parameter::ObjectSetParameter(ObjectSet {
                            values: vec![ObjectSetValue::Reference("Reg-MapData".into())],
                            extensible: None
                        })]
                    }
                }))
            })
        );
    }

    #[test]
    fn handles_object_field_ref() {
        println!(
            "{:?}",
            sequence_of(
                r#"SEQUENCE (SIZE(1..MAX)) OF
        IEEE1609DOT2-HEADERINFO-CONTRIBUTED-EXTENSION.&Extn({
        Ieee1609Dot2HeaderInfoContributedExtensions
      }{@.contributorId})"#
                    .into()
            )
            .unwrap()
            .1
        )
    }
}
