use crate::intermediate::{constraints::*, *};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::char,
    combinator::{into, map, map_res, opt, value},
    multi::{many0_count, many1, separated_list0, separated_list1},
    sequence::{delimited, pair, preceded, separated_pair, terminated, tuple},
};

use super::{
    asn1_type, asn1_value,
    common::{
        extension_marker, identifier, in_braces, in_parentheses, range_seperator,
        skip_ws_and_comments,
    },
    information_object_class::object_set,
    parameterization::parameters,
    skip_ws,
    util::{opt_delimited, take_until_and_not, take_until_unbalanced},
    LexerResult, Span,
};

pub fn constraint(input: Span) -> LexerResult<Vec<Constraint>> {
    skip_ws_and_comments(many1(alt((
        single_constraint,
        // Handle SIZE constraint without external parentheses
        map(size_constraint, |c| {
            Constraint::SubtypeConstraint(ElementSet {
                set: ElementOrSetOperation::Element(c),
                extensible: false,
            })
        }),
        map(parameters, Constraint::Parameter),
    ))))(input)
}

pub fn single_constraint(input: Span) -> LexerResult<Constraint> {
    skip_ws_and_comments(in_parentheses(alt((
        map(content_constraint, Constraint::ContentConstraint),
        map(table_constraint, Constraint::TableConstraint),
        map(element_set, Constraint::SubtypeConstraint),
    ))))(input)
}

pub fn set_operator(input: Span) -> LexerResult<SetOperator> {
    skip_ws_and_comments(alt((
        value(SetOperator::Intersection, tag(INTERSECTION)),
        value(SetOperator::Intersection, tag(CARET)),
        value(SetOperator::Union, tag(UNION)),
        value(SetOperator::Union, tag(PIPE)),
        value(SetOperator::Except, tag(EXCEPT)),
    )))(input)
}

fn element_set(input: Span) -> LexerResult<ElementSet> {
    into(pair(
        alt((
            map(set_operation, ElementOrSetOperation::SetOperation),
            map(subtype_element, ElementOrSetOperation::Element),
        )),
        opt(skip_ws_and_comments(preceded(
            char(COMMA),
            extension_marker,
        ))),
    ))(input)
}

fn set_operation(input: Span) -> LexerResult<SetOperation> {
    into(tuple((
        subtype_element,
        set_operator,
        alt((
            map(set_operation, ElementOrSetOperation::SetOperation),
            map(subtype_element, ElementOrSetOperation::Element),
        )),
    )))(input)
}

fn subtype_element(input: Span) -> LexerResult<SubtypeElement> {
    alt((
        single_type_constraint,
        multiple_type_constraints,
        size_constraint,
        pattern_constraint,
        user_defined_constraint,
        property_settings_constraint,
        permitted_alphabet_constraint,
        value_range,
        single_value,
        contained_subtype,
    ))(input)
}

fn extension_additions(input: Span) -> LexerResult<()> {
    value(
        (),
        opt(pair(
            skip_ws_and_comments(char(COMMA)),
            skip_ws_and_comments(separated_list0(
                skip_ws_and_comments(char(COMMA)),
                skip_ws_and_comments(alt((
                    value(
                        0,
                        pair(
                            terminated(
                                alt((value(None, tag(MIN)), map(asn1_value, Some))),
                                skip_ws_and_comments(opt(char(GREATER_THAN))),
                            ),
                            preceded(
                                range_seperator,
                                preceded(
                                    opt(char(LESS_THAN)),
                                    skip_ws_and_comments(alt((
                                        value(None, tag(MAX)),
                                        map(asn1_value, Some),
                                    ))),
                                ),
                            ),
                        ),
                    ),
                    value(0, asn1_value),
                ))),
            )),
        )),
    )(input)
}

fn single_value(input: Span) -> LexerResult<SubtypeElement> {
    opt_delimited::<char, SubtypeElement, char, _, _, _>(
        skip_ws_and_comments(char(LEFT_PARENTHESIS)),
        skip_ws_and_comments(into(pair(
            asn1_value,
            opt(skip_ws_and_comments(delimited(
                char(COMMA),
                extension_marker,
                extension_additions,
            ))),
        ))),
        skip_ws_and_comments(char(RIGHT_PARENTHESIS)),
    )(input)
}

fn contained_subtype(input: Span) -> LexerResult<SubtypeElement> {
    opt_delimited::<char, SubtypeElement, char, _, _, _>(
        skip_ws_and_comments(char(LEFT_PARENTHESIS)),
        skip_ws_and_comments(map(
            pair(
                preceded(opt(tag(INCLUDES)), skip_ws_and_comments(asn1_type)),
                opt(skip_ws_and_comments(preceded(
                    char(COMMA),
                    extension_marker,
                ))),
            ),
            |(t, ext)| SubtypeElement::ContainedSubtype {
                subtype: t,
                extensible: ext.is_some(),
            },
        )),
        skip_ws_and_comments(char(RIGHT_PARENTHESIS)),
    )(input)
}

fn value_range(input: Span) -> LexerResult<SubtypeElement> {
    opt_delimited::<char, SubtypeElement, char, _, _, _>(
        skip_ws_and_comments(char(LEFT_PARENTHESIS)),
        skip_ws_and_comments(map(
            tuple((
                terminated(
                    alt((value(None, tag(MIN)), map(asn1_value, Some))),
                    skip_ws_and_comments(opt(char(GREATER_THAN))),
                ),
                preceded(
                    range_seperator,
                    preceded(
                        opt(char(LESS_THAN)),
                        skip_ws_and_comments(alt((value(None, tag(MAX)), map(asn1_value, Some)))),
                    ),
                ),
                opt(skip_ws_and_comments(delimited(
                    char(COMMA),
                    extension_marker,
                    extension_additions,
                ))),
            )),
            |(min, max, ext)| SubtypeElement::ValueRange {
                min,
                max,
                extensible: ext.is_some(),
            },
        )),
        skip_ws_and_comments(char(RIGHT_PARENTHESIS)),
    )(input)
}

fn size_constraint(input: Span) -> LexerResult<SubtypeElement> {
    opt_delimited::<char, SubtypeElement, char, _, _, _>(
        skip_ws_and_comments(char(LEFT_PARENTHESIS)),
        skip_ws_and_comments(into(preceded(tag(SIZE), single_constraint))),
        skip_ws_and_comments(char(RIGHT_PARENTHESIS)),
    )(input)
}

fn pattern_constraint(input: Span) -> LexerResult<SubtypeElement> {
    map(
        opt_delimited::<char, PatternConstraint, char, _, _, _>(
            skip_ws_and_comments(char(LEFT_PARENTHESIS)),
            skip_ws_and_comments(into(preceded(
                tag(PATTERN),
                skip_ws_and_comments(delimited(
                    char('"'),
                    take_until_and_not("\"", "\"\""),
                    char('"'),
                )),
            ))),
            skip_ws_and_comments(char(RIGHT_PARENTHESIS)),
        ),
        SubtypeElement::PatternConstraint,
    )(input)
}

fn user_defined_constraint(input: Span) -> LexerResult<SubtypeElement> {
    map(
        opt_delimited::<char, UserDefinedConstraint, char, _, _, _>(
            skip_ws_and_comments(char(LEFT_PARENTHESIS)),
            skip_ws_and_comments(into(preceded(
                tag(CONSTRAINED_BY),
                skip_ws_and_comments(delimited(
                    char(LEFT_BRACE),
                    take_until_unbalanced("{", "}"),
                    char(RIGHT_BRACE),
                )),
            ))),
            skip_ws_and_comments(char(RIGHT_PARENTHESIS)),
        ),
        SubtypeElement::UserDefinedConstraint,
    )(input)
}

/// Parses a PermittedAlphabet constraint.
/// ### Reference in X680
/// >* _51.7.1 The "PermittedAlphabet" notation shall be: `PermittedAlphabet ::= FROM Constraint`_
/// >* _51.7.2 A "PermittedAlphabet" specifies all values which can be constructed using a sub-alphabet of the parent string. This notation can only be applied to restricted character string types._
/// >* _51.7.3 The "Constraint" shall use the "SubtypeConstraint" alternative of "ConstraintSpec". Each "SubtypeElements" within that "SubtypeConstraint" shall be one of the four alternatives "SingleValue", "ContainedSubtype", "ValueRange", and "SizeConstraint". The sub-alphabet includes precisely those characters which appear in one or more of the values of the parent string type which are allowed by the "Constraint"._
/// >* _51.7.4 If "Constraint" is extensible, then the set of values selected by the permitted alphabet constraint is extensible. The set of values in the root are those permitted by the root of "Constraint", and the extension additions are those values permitted by the root together with the extension-additions of "Constraint", excluding those values already in the root._
fn permitted_alphabet_constraint(input: Span) -> LexerResult<SubtypeElement> {
    opt_delimited::<char, SubtypeElement, char, _, _, _>(
        skip_ws_and_comments(char(LEFT_PARENTHESIS)),
        skip_ws_and_comments(map(
            preceded(
                tag(FROM),
                in_parentheses(alt((
                    map(set_operation, ElementOrSetOperation::SetOperation),
                    map(subtype_element, ElementOrSetOperation::Element),
                ))),
            ),
            |i| SubtypeElement::PermittedAlphabet(Box::new(i)),
        )),
        skip_ws_and_comments(char(RIGHT_PARENTHESIS)),
    )(input)
}

fn single_type_constraint(input: Span) -> LexerResult<SubtypeElement> {
    opt_delimited::<char, SubtypeElement, char, _, _, _>(
        skip_ws_and_comments(char(LEFT_PARENTHESIS)),
        skip_ws_and_comments(into(preceded(
            tag(WITH_COMPONENTS),
            in_braces(pair(
                opt(skip_ws_and_comments(terminated(
                    value(ExtensionMarker(), tag(ELLIPSIS)),
                    skip_ws_and_comments(char(COMMA)),
                ))),
                many1(terminated(
                    subset_member,
                    opt(skip_ws_and_comments(char(COMMA))),
                )),
            )),
        ))),
        skip_ws_and_comments(char(RIGHT_PARENTHESIS)),
    )(input)
}

fn multiple_type_constraints(input: Span) -> LexerResult<SubtypeElement> {
    opt_delimited::<char, SubtypeElement, char, _, _, _>(
        skip_ws_and_comments(char(LEFT_PARENTHESIS)),
        skip_ws_and_comments(preceded(
            tag(WITH_COMPONENT),
            skip_ws_and_comments(alt((
                map(single_type_constraint, |s| {
                    if let SubtypeElement::SingleTypeConstraint(c) = s {
                        SubtypeElement::MultipleTypeConstraints(c)
                    } else {
                        s
                    }
                }),
                multiple_type_constraints,
            ))),
        )),
        skip_ws_and_comments(char(RIGHT_PARENTHESIS)),
    )(input)
}

fn subset_member(
    input: Span,
) -> LexerResult<(Span, Option<Vec<Constraint>>, Option<ComponentPresence>)> {
    skip_ws_and_comments(tuple((
        identifier,
        opt(skip_ws_and_comments(constraint)),
        opt(skip_ws_and_comments(alt((
            value(ComponentPresence::Present, tag(PRESENT)),
            value(ComponentPresence::Absent, tag(ABSENT)),
        )))),
    )))(input)
}

fn content_constraint(input: Span) -> LexerResult<ContentConstraint> {
    opt_delimited::<char, ContentConstraint, char, _, _, _>(
        skip_ws_and_comments(char(LEFT_PARENTHESIS)),
        skip_ws_and_comments(alt((
            into(pair(
                preceded(skip_ws_and_comments(tag(CONTAINING)), skip_ws(asn1_type)),
                preceded(skip_ws_and_comments(tag(ENCODED_BY)), skip_ws(asn1_value)),
            )),
            into(preceded(
                skip_ws_and_comments(tag(CONTAINING)),
                skip_ws(asn1_type),
            )),
            into(preceded(
                skip_ws_and_comments(tag(ENCODED_BY)),
                skip_ws(asn1_value),
            )),
        ))),
        skip_ws_and_comments(char(RIGHT_PARENTHESIS)),
    )(input)
}

fn table_constraint(input: Span) -> LexerResult<TableConstraint> {
    opt_delimited::<char, TableConstraint, char, _, _, _>(
        skip_ws_and_comments(char(LEFT_PARENTHESIS)),
        skip_ws_and_comments(into(pair(
            object_set,
            opt(in_braces(separated_list1(
                skip_ws_and_comments(char(COMMA)),
                relational_constraint,
            ))),
        ))),
        skip_ws_and_comments(char(RIGHT_PARENTHESIS)),
    )(input)
}

fn relational_constraint(input: Span) -> LexerResult<RelationalConstraint> {
    into(skip_ws_and_comments(preceded(
        char(AT),
        pair(many0_count(char(DOT)), identifier),
    )))(input)
}

fn property_settings_constraint(input: Span) -> LexerResult<SubtypeElement> {
    preceded(
        skip_ws_and_comments(tag("SETTINGS")),
        map_res(
            skip_ws_and_comments(delimited(
                char('"'),
                many1(skip_ws_and_comments(separated_pair(
                    settings_identifier,
                    char('='),
                    identifier,
                ))),
                char('"'),
            )),
            |res| {
                res.into_iter()
                    .map(PropertyAndSettingsPair::try_from)
                    .collect::<Result<Vec<PropertyAndSettingsPair>, _>>()
                    .map(|settings| {
                        SubtypeElement::PropertySettings(PropertySettings {
                            property_settings_list: settings,
                        })
                    })
            },
        ),
    )(input)
}

fn settings_identifier(input: Span) -> LexerResult<Span> {
    alt((
        tag(BasicSettings::NAME),
        tag(DateSettings::NAME),
        tag(YearSettings::NAME),
        tag(TimeSettings::NAME),
        tag(LocalOrUtcSettings::NAME),
        tag(IntervalTypeSettings::NAME),
        tag(StartEndPointSettings::NAME),
        tag(RecurrenceSettings::NAME),
        tag(MidnightSettings::NAME),
    ))(input)
}

#[cfg(test)]
mod tests {
    use crate::intermediate::{information_object::*, types::*};

    use crate::lexer::constraint::*;

    #[test]
    fn parses_value_constraint() {
        assert_eq!(
            constraint(Span::new("(5)")).unwrap().1,
            vec![Constraint::SubtypeConstraint(ElementSet {
                set: ElementOrSetOperation::Element(SubtypeElement::SingleValue {
                    value: ASN1Value::Integer(5),
                    extensible: false
                }),
                extensible: false
            })]
        );
        assert_eq!(
            constraint(Span::new("(5..9)")).unwrap().1,
            vec![Constraint::SubtypeConstraint(ElementSet {
                set: ElementOrSetOperation::Element(SubtypeElement::ValueRange {
                    min: Some(ASN1Value::Integer(5)),
                    max: Some(ASN1Value::Integer(9)),
                    extensible: false
                }),
                extensible: false
            })]
        );
        assert_eq!(
            constraint(Span::new("(-5..9)")).unwrap().1,
            vec![Constraint::SubtypeConstraint(ElementSet {
                set: ElementOrSetOperation::Element(SubtypeElement::ValueRange {
                    min: Some(ASN1Value::Integer(-5)),
                    max: Some(ASN1Value::Integer(9)),
                    extensible: false
                }),
                extensible: false
            })]
        );
        assert_eq!(
            constraint(Span::new("(-9..-4,...)")).unwrap().1,
            vec![Constraint::SubtypeConstraint(ElementSet {
                set: ElementOrSetOperation::Element(SubtypeElement::ValueRange {
                    min: Some(ASN1Value::Integer(-9)),
                    max: Some(ASN1Value::Integer(-4)),
                    extensible: true
                }),
                extensible: false
            })]
        );
    }

    #[test]
    fn handles_added_extension_values() {
        assert_eq!(
            constraint(Span::new("(1..32767,..., 8388607)")).unwrap().1,
            vec![Constraint::SubtypeConstraint(ElementSet {
                set: ElementOrSetOperation::Element(SubtypeElement::ValueRange {
                    min: Some(ASN1Value::Integer(1)),
                    max: Some(ASN1Value::Integer(32767)),
                    extensible: true
                }),
                extensible: false
            })]
        )
    }

    #[test]
    fn handles_redundant_parentheses() {
        assert_eq!(
            constraint(Span::new("((5..9))")).unwrap().1,
            vec![Constraint::SubtypeConstraint(ElementSet {
                set: ElementOrSetOperation::Element(SubtypeElement::ValueRange {
                    min: Some(ASN1Value::Integer(5)),
                    max: Some(ASN1Value::Integer(9)),
                    extensible: false
                }),
                extensible: false
            })]
        );
    }

    #[test]
    fn parses_value_constraint_with_inserted_comment() {
        assert_eq!(
            constraint(Span::new("(-9..-4, -- Very annoying! -- ...)"))
                .unwrap()
                .1,
            vec![Constraint::SubtypeConstraint(ElementSet {
                set: ElementOrSetOperation::Element(SubtypeElement::ValueRange {
                    min: Some(ASN1Value::Integer(-9)),
                    max: Some(ASN1Value::Integer(-4)),
                    extensible: true
                }),
                extensible: false
            })]
        );
        assert_eq!(
            constraint(Span::new("(-9-- Very annoying! --..-4,  ...)"))
                .unwrap()
                .1,
            vec![Constraint::SubtypeConstraint(ElementSet {
                set: ElementOrSetOperation::Element(SubtypeElement::ValueRange {
                    min: Some(ASN1Value::Integer(-9)),
                    max: Some(ASN1Value::Integer(-4)),
                    extensible: true
                }),
                extensible: false
            })]
        );
    }

    #[test]
    fn parses_size_constraint() {
        assert_eq!(
            constraint(Span::new("(SIZE(3..16, ...))")).unwrap().1,
            vec![Constraint::SubtypeConstraint(ElementSet {
                set: ElementOrSetOperation::Element(SubtypeElement::SizeConstraint(Box::new(
                    ElementOrSetOperation::Element(SubtypeElement::ValueRange {
                        min: Some(ASN1Value::Integer(3)),
                        max: Some(ASN1Value::Integer(16)),
                        extensible: true
                    })
                ))),
                extensible: false
            })]
        )
    }

    #[test]
    fn parses_composite_constraint() {
        assert_eq!(
            constraint(Span::new(r#"(ALL EXCEPT 1)"#)).unwrap().1,
            vec![Constraint::SubtypeConstraint(ElementSet {
                set: ElementOrSetOperation::SetOperation(SetOperation {
                    base: SubtypeElement::SingleValue {
                        value: ASN1Value::All,
                        extensible: false
                    },
                    operator: SetOperator::Except,
                    operant: Box::new(ElementOrSetOperation::Element(
                        SubtypeElement::SingleValue {
                            value: ASN1Value::Integer(1),
                            extensible: false
                        }
                    ))
                }),
                extensible: false
            })]
        )
    }

    #[test]
    fn parses_complex_set() {
        assert_eq!(
            constraint(Span::new(
                r#"((WITH COMPONENT (WITH COMPONENTS {..., containerId (ALL EXCEPT 1)})) |
          (WITH COMPONENT (WITH COMPONENTS {..., containerId (ALL EXCEPT 2)})))"#
            ))
            .unwrap()
            .1,
            vec![Constraint::SubtypeConstraint(ElementSet {
                set: ElementOrSetOperation::SetOperation(SetOperation {
                    base: SubtypeElement::MultipleTypeConstraints(InnerTypeConstraint {
                        is_partial: true,
                        constraints: vec![ConstrainedComponent {
                            identifier: "containerId".into(),
                            constraints: vec![Constraint::SubtypeConstraint(ElementSet {
                                set: ElementOrSetOperation::SetOperation(SetOperation {
                                    base: SubtypeElement::SingleValue {
                                        value: ASN1Value::All,
                                        extensible: false
                                    },
                                    operator: SetOperator::Except,
                                    operant: Box::new(ElementOrSetOperation::Element(
                                        SubtypeElement::SingleValue {
                                            value: ASN1Value::Integer(1),
                                            extensible: false
                                        }
                                    ))
                                }),
                                extensible: false
                            })],
                            presence: ComponentPresence::Unspecified
                        }]
                    }),
                    operator: SetOperator::Union,
                    operant: Box::new(ElementOrSetOperation::Element(
                        SubtypeElement::MultipleTypeConstraints(InnerTypeConstraint {
                            is_partial: true,
                            constraints: vec![ConstrainedComponent {
                                identifier: "containerId".into(),
                                constraints: vec![Constraint::SubtypeConstraint(ElementSet {
                                    set: ElementOrSetOperation::SetOperation(SetOperation {
                                        base: SubtypeElement::SingleValue {
                                            value: ASN1Value::All,
                                            extensible: false
                                        },
                                        operator: SetOperator::Except,
                                        operant: Box::new(ElementOrSetOperation::Element(
                                            SubtypeElement::SingleValue {
                                                value: ASN1Value::Integer(2),
                                                extensible: false
                                            }
                                        ))
                                    }),
                                    extensible: false
                                })],
                                presence: ComponentPresence::Unspecified
                            }]
                        })
                    ))
                }),
                extensible: false
            })]
        )
    }

    #[test]
    fn parses_full_component_constraint() {
        assert_eq!(
            constraint(Span::new(
                "(WITH COMPONENTS
                  {ordering ABSENT ,
                  sales (0..5) PRESENT,
                  e-cash-return ABSENT } )"
            ))
            .unwrap()
            .1,
            vec![Constraint::SubtypeConstraint(ElementSet {
                set: ElementOrSetOperation::Element(SubtypeElement::SingleTypeConstraint(
                    InnerTypeConstraint {
                        is_partial: false,
                        constraints: vec![
                            ConstrainedComponent {
                                identifier: "ordering".into(),
                                constraints: vec![],
                                presence: ComponentPresence::Absent
                            },
                            ConstrainedComponent {
                                identifier: "sales".into(),
                                constraints: vec![Constraint::SubtypeConstraint(ElementSet {
                                    set: ElementOrSetOperation::Element(
                                        SubtypeElement::ValueRange {
                                            min: Some(ASN1Value::Integer(0)),
                                            max: Some(ASN1Value::Integer(5)),
                                            extensible: false
                                        }
                                    ),
                                    extensible: false
                                })],
                                presence: ComponentPresence::Present
                            },
                            ConstrainedComponent {
                                identifier: "e-cash-return".into(),
                                constraints: vec![],
                                presence: ComponentPresence::Absent
                            }
                        ]
                    }
                )),
                extensible: false
            })]
        );
    }

    #[test]
    fn parses_partial_component_constraint() {
        assert_eq!(
            constraint(Span::new(
                "( WITH COMPONENTS
                      {... ,
                      ordering ABSENT,
                      sales (0..5) } )"
            ))
            .unwrap()
            .1,
            vec![Constraint::SubtypeConstraint(ElementSet {
                set: ElementOrSetOperation::Element(SubtypeElement::SingleTypeConstraint(
                    InnerTypeConstraint {
                        is_partial: true,
                        constraints: vec![
                            ConstrainedComponent {
                                identifier: "ordering".into(),
                                constraints: vec![],
                                presence: ComponentPresence::Absent
                            },
                            ConstrainedComponent {
                                identifier: "sales".into(),
                                constraints: vec![Constraint::SubtypeConstraint(ElementSet {
                                    set: ElementOrSetOperation::Element(
                                        SubtypeElement::ValueRange {
                                            min: Some(ASN1Value::Integer(0)),
                                            max: Some(ASN1Value::Integer(5)),
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
        );
    }

    #[test]
    fn parses_composite_array_constraint() {
        assert_eq!(
            constraint(Span::new(
                "((WITH COMPONENT (WITH COMPONENTS {..., eventDeltaTime PRESENT})) |
                    (WITH COMPONENT (WITH COMPONENTS {..., eventDeltaTime ABSENT})))
                "
            ))
            .unwrap()
            .1,
            vec![Constraint::SubtypeConstraint(ElementSet {
                set: ElementOrSetOperation::SetOperation(SetOperation {
                    base: SubtypeElement::MultipleTypeConstraints(InnerTypeConstraint {
                        is_partial: true,
                        constraints: vec![ConstrainedComponent {
                            identifier: "eventDeltaTime".into(),
                            constraints: vec![],
                            presence: ComponentPresence::Present
                        }]
                    }),
                    operator: SetOperator::Union,
                    operant: Box::new(ElementOrSetOperation::Element(
                        SubtypeElement::MultipleTypeConstraints(InnerTypeConstraint {
                            is_partial: true,
                            constraints: vec![ConstrainedComponent {
                                identifier: "eventDeltaTime".into(),
                                constraints: vec![],
                                presence: ComponentPresence::Absent
                            }]
                        })
                    ))
                }),
                extensible: false
            })]
        );
    }

    #[test]
    fn parses_composite_component_constraint() {
        assert_eq!(
            constraint(Span::new(
                "((WITH COMPONENTS {..., laneId PRESENT, connectionId ABSENT }) |
                    (WITH COMPONENTS {..., laneId ABSENT, connectionId PRESENT }))
                "
            ))
            .unwrap()
            .1,
            vec![Constraint::SubtypeConstraint(ElementSet {
                set: ElementOrSetOperation::SetOperation(SetOperation {
                    base: SubtypeElement::SingleTypeConstraint(InnerTypeConstraint {
                        is_partial: true,
                        constraints: vec![
                            ConstrainedComponent {
                                identifier: "laneId".into(),
                                constraints: vec![],
                                presence: ComponentPresence::Present
                            },
                            ConstrainedComponent {
                                identifier: "connectionId".into(),
                                constraints: vec![],
                                presence: ComponentPresence::Absent
                            }
                        ]
                    }),
                    operator: SetOperator::Union,
                    operant: Box::new(ElementOrSetOperation::Element(
                        SubtypeElement::SingleTypeConstraint(InnerTypeConstraint {
                            is_partial: true,
                            constraints: vec![
                                ConstrainedComponent {
                                    identifier: "laneId".into(),
                                    constraints: vec![],
                                    presence: ComponentPresence::Absent
                                },
                                ConstrainedComponent {
                                    identifier: "connectionId".into(),
                                    constraints: vec![],
                                    presence: ComponentPresence::Present
                                }
                            ]
                        })
                    ))
                }),
                extensible: false
            })]
        );
    }

    #[test]
    fn parses_composite_range_constraint() {
        assert_eq!(
            constraint(Span::new(
                "(0..3|5..8|10)
                "
            ))
            .unwrap()
            .1,
            vec![Constraint::SubtypeConstraint(ElementSet {
                set: ElementOrSetOperation::SetOperation(SetOperation {
                    base: SubtypeElement::ValueRange {
                        min: Some(ASN1Value::Integer(0)),
                        max: Some(ASN1Value::Integer(3)),
                        extensible: false
                    },
                    operator: SetOperator::Union,
                    operant: Box::new(ElementOrSetOperation::SetOperation(SetOperation {
                        base: SubtypeElement::ValueRange {
                            min: Some(ASN1Value::Integer(5)),
                            max: Some(ASN1Value::Integer(8)),
                            extensible: false
                        },
                        operator: SetOperator::Union,
                        operant: Box::new(ElementOrSetOperation::Element(
                            SubtypeElement::SingleValue {
                                value: ASN1Value::Integer(10),
                                extensible: false
                            }
                        ))
                    }))
                }),
                extensible: false
            })]
        );
    }

    #[test]
    fn parses_composite_range_constraint_with_elsewhere_declared_values() {
        assert_eq!(
            constraint(Span::new(
                "(unknown   | passengerCar..tram
                  | agricultural)"
            ))
            .unwrap()
            .1,
            vec![Constraint::SubtypeConstraint(ElementSet {
                set: ElementOrSetOperation::SetOperation(SetOperation {
                    base: SubtypeElement::SingleValue {
                        value: ASN1Value::ElsewhereDeclaredValue {
                            identifier: "unknown".to_string(),
                            parent: None
                        },
                        extensible: false
                    },
                    operator: SetOperator::Union,
                    operant: Box::new(ElementOrSetOperation::SetOperation(SetOperation {
                        base: SubtypeElement::ValueRange {
                            min: Some(ASN1Value::ElsewhereDeclaredValue {
                                identifier: "passengerCar".to_string(),
                                parent: None
                            }),
                            max: Some(ASN1Value::ElsewhereDeclaredValue {
                                identifier: "tram".to_string(),
                                parent: None
                            }),
                            extensible: false
                        },
                        operator: SetOperator::Union,
                        operant: Box::new(ElementOrSetOperation::Element(
                            SubtypeElement::SingleValue {
                                value: ASN1Value::ElsewhereDeclaredValue {
                                    identifier: "agricultural".to_string(),
                                    parent: None
                                },
                                extensible: false
                            }
                        ))
                    }))
                }),
                extensible: false
            })]
        );
    }

    #[test]
    fn parses_table_constraint() {
        assert_eq!(
            constraint(Span::new(
                "({
                  My-ops |
                  {
                    &id 5,
                    &Type INTEGER (1..6)
                  } |
                  {ConnectionManeuverAssist-addGrpC  IDENTIFIED BY addGrpC},
                  ...
                })"
            ))
            .unwrap()
            .1,
            vec![Constraint::TableConstraint(TableConstraint {
                object_set: ObjectSet {
                    values: vec![
                        ObjectSetValue::Reference("My-ops".into()),
                        ObjectSetValue::Inline(InformationObjectFields::DefaultSyntax(vec![
                            InformationObjectField::FixedValueField(FixedValueField {
                                identifier: "&id".into(),
                                value: ASN1Value::Integer(5)
                            }),
                            InformationObjectField::TypeField(TypeField {
                                identifier: "&Type".into(),
                                ty: ASN1Type::Integer(Integer {
                                    constraints: vec![Constraint::SubtypeConstraint(ElementSet {
                                        set: ElementOrSetOperation::Element(
                                            SubtypeElement::ValueRange {
                                                min: Some(ASN1Value::Integer(1)),
                                                max: Some(ASN1Value::Integer(6)),
                                                extensible: false
                                            }
                                        ),
                                        extensible: false
                                    })],
                                    distinguished_values: None,
                                })
                            })
                        ])),
                        ObjectSetValue::Inline(InformationObjectFields::CustomSyntax(vec![
                            SyntaxApplication::LiteralOrTypeReference(DeclarationElsewhere {
                                parent: None,
                                identifier: "ConnectionManeuverAssist-addGrpC".into(),
                                constraints: vec![]
                            }),
                            SyntaxApplication::LiteralOrTypeReference(DeclarationElsewhere {
                                parent: None,
                                identifier: "IDENTIFIED".into(),
                                constraints: vec![]
                            }),
                            SyntaxApplication::LiteralOrTypeReference(DeclarationElsewhere {
                                parent: None,
                                identifier: "BY".into(),
                                constraints: vec![]
                            }),
                            SyntaxApplication::ValueReference(ASN1Value::ElsewhereDeclaredValue {
                                identifier: "addGrpC".into(),
                                parent: None
                            })
                        ]))
                    ],
                    extensible: Some(3)
                },
                linked_fields: vec![]
            })]
        );
    }

    #[test]
    fn parses_character_value_range() {
        assert_eq!(
            value_range(Span::new(r#""a".."z""#)).unwrap().1,
            SubtypeElement::ValueRange {
                min: Some(ASN1Value::String("a".to_owned())),
                max: Some(ASN1Value::String("z".to_owned())),
                extensible: false
            }
        )
    }

    #[test]
    fn parses_permitted_alphabet_constraint() {
        assert_eq!(
            permitted_alphabet_constraint(Span::new(
                r#"(FROM ("a".."z" | "A".."Z" | "0".."9" | ".-"))"#
            ))
            .unwrap()
            .1,
            SubtypeElement::PermittedAlphabet(Box::new(ElementOrSetOperation::SetOperation(
                SetOperation {
                    base: SubtypeElement::ValueRange {
                        min: Some(ASN1Value::String("a".to_owned())),
                        max: Some(ASN1Value::String("z".to_owned())),
                        extensible: false
                    },
                    operator: SetOperator::Union,
                    operant: Box::new(ElementOrSetOperation::SetOperation(SetOperation {
                        base: SubtypeElement::ValueRange {
                            min: Some(ASN1Value::String("A".to_owned())),
                            max: Some(ASN1Value::String("Z".to_owned())),
                            extensible: false
                        },
                        operator: SetOperator::Union,
                        operant: Box::new(ElementOrSetOperation::SetOperation(SetOperation {
                            base: SubtypeElement::ValueRange {
                                min: Some(ASN1Value::String("0".to_owned())),
                                max: Some(ASN1Value::String("9".to_owned())),
                                extensible: false
                            },
                            operator: SetOperator::Union,
                            operant: Box::new(ElementOrSetOperation::Element(
                                SubtypeElement::SingleValue {
                                    value: ASN1Value::String(".-".to_owned()),
                                    extensible: false
                                }
                            ))
                        }))
                    }))
                }
            )))
        )
    }

    #[test]
    fn parses_serial_constraints() {
        assert_eq!(
            constraint(Span::new(
                r#"(FROM ("a".."z" | "A".."Z" | "0".."9" | ".-")) (SIZE (1..255))"#
            ))
            .unwrap()
            .1,
            vec![
                Constraint::SubtypeConstraint(ElementSet {
                    set: ElementOrSetOperation::Element(SubtypeElement::PermittedAlphabet(
                        Box::new(ElementOrSetOperation::SetOperation(SetOperation {
                            base: SubtypeElement::ValueRange {
                                min: Some(ASN1Value::String("a".to_owned())),
                                max: Some(ASN1Value::String("z".to_owned())),
                                extensible: false
                            },
                            operator: SetOperator::Union,
                            operant: Box::new(ElementOrSetOperation::SetOperation(SetOperation {
                                base: SubtypeElement::ValueRange {
                                    min: Some(ASN1Value::String("A".to_owned())),
                                    max: Some(ASN1Value::String("Z".to_owned())),
                                    extensible: false
                                },
                                operator: SetOperator::Union,
                                operant: Box::new(ElementOrSetOperation::SetOperation(
                                    SetOperation {
                                        base: SubtypeElement::ValueRange {
                                            min: Some(ASN1Value::String("0".to_owned())),
                                            max: Some(ASN1Value::String("9".to_owned())),
                                            extensible: false
                                        },
                                        operator: SetOperator::Union,
                                        operant: Box::new(ElementOrSetOperation::Element(
                                            SubtypeElement::SingleValue {
                                                value: ASN1Value::String(".-".to_owned()),
                                                extensible: false
                                            }
                                        ))
                                    }
                                ))
                            }))
                        }))
                    )),
                    extensible: false
                }),
                Constraint::SubtypeConstraint(ElementSet {
                    set: ElementOrSetOperation::Element(SubtypeElement::SizeConstraint(Box::new(
                        ElementOrSetOperation::Element(SubtypeElement::ValueRange {
                            min: Some(ASN1Value::Integer(1)),
                            max: Some(ASN1Value::Integer(255)),
                            extensible: false
                        })
                    ))),
                    extensible: false
                })
            ]
        )
    }

    #[test]
    fn parses_real_constraint() {
        assert_eq!(
            constraint(Span::new(
                r#"(WITH COMPONENTS {
                mantissa (-16777215..16777215),
                base (2),
                exponent (-125..128) } )"#
            ))
            .unwrap()
            .1,
            vec![Constraint::SubtypeConstraint(ElementSet {
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
        )
    }

    #[test]
    fn parses_pattern_constraint() {
        assert_eq!(
            constraint(Span::new(
                r#"(PATTERN "[a-zA-Z]#(1,8)(-[a-zA-Z0-9]#(1,8))*")"#
            ))
            .unwrap()
            .1,
            vec![Constraint::SubtypeConstraint(ElementSet {
                set: ElementOrSetOperation::Element(SubtypeElement::PatternConstraint(
                    PatternConstraint {
                        pattern: "[a-zA-Z]#(1,8)(-[a-zA-Z0-9]#(1,8))*".into()
                    }
                )),
                extensible: false
            })]
        )
    }

    #[test]
    fn parses_user_defined_constraint() {
        assert_eq!(
            constraint(
                Span::new(r#"(CONSTRAINED BY {/* XML representation of the XSD pattern "\d\d\d\d-\d\d-\d\dT\d\d:\d\d:\d\d[-,+]\d\d:\d\d" */})"#)
            ).unwrap().1,
            vec![
                Constraint::SubtypeConstraint(
                    ElementSet {
                        set: ElementOrSetOperation::Element(
                            SubtypeElement::UserDefinedConstraint(
                                UserDefinedConstraint {
                                    definition: "/* XML representation of the XSD pattern \"\\d\\d\\d\\d-\\d\\d-\\d\\dT\\d\\d:\\d\\d:\\d\\d[-,+]\\d\\d:\\d\\d\" */".into()
                                }
                                )
                            ),
                            extensible: false
                        }
                    )
                ]
            )
    }

    #[test]
    fn parses_two_variants_of_extensible_size() {
        assert_eq!(
            constraint(Span::new("(SIZE(1..4),...)")).unwrap().1,
            vec![Constraint::SubtypeConstraint(ElementSet {
                set: ElementOrSetOperation::Element(SubtypeElement::SizeConstraint(Box::new(
                    ElementOrSetOperation::Element(SubtypeElement::ValueRange {
                        min: Some(ASN1Value::Integer(1)),
                        max: Some(ASN1Value::Integer(4)),
                        extensible: false
                    })
                ))),
                extensible: true
            })]
        );
        assert_eq!(
            constraint(Span::new("(SIZE(1..4,...))")).unwrap().1,
            vec![Constraint::SubtypeConstraint(ElementSet {
                set: ElementOrSetOperation::Element(SubtypeElement::SizeConstraint(Box::new(
                    ElementOrSetOperation::Element(SubtypeElement::ValueRange {
                        min: Some(ASN1Value::Integer(1)),
                        max: Some(ASN1Value::Integer(4)),
                        extensible: true
                    })
                ))),
                extensible: false
            })]
        )
    }

    #[test]
    fn parses_property_settings_constraint() {
        assert_eq!(
            constraint(Span::new(r#"(SETTINGS "Midnight=Start")"#))
                .unwrap()
                .1,
            vec![Constraint::SubtypeConstraint(ElementSet {
                set: ElementOrSetOperation::Element(SubtypeElement::PropertySettings(
                    PropertySettings {
                        property_settings_list: vec![PropertyAndSettingsPair::Midnight(
                            MidnightSettings::StartOfDay
                        )]
                    }
                )),
                extensible: false
            })]
        );
    }

    #[test]
    fn parses_extended_range_constraint() {
        assert_eq!(
            constraint(Span::new(r#"(1..65535, ..., 65536..109999)"#))
                .unwrap()
                .1,
            vec![Constraint::SubtypeConstraint(ElementSet {
                set: ElementOrSetOperation::Element(SubtypeElement::ValueRange {
                    min: Some(ASN1Value::Integer(1)),
                    max: Some(ASN1Value::Integer(65535)),
                    extensible: true
                }),
                extensible: false
            })]
        )
    }
}
