use crate::{
    input::Input,
    intermediate::{constraints::*, *},
};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::char,
    combinator::{into, map, map_res, opt, value},
    multi::{many0_count, many1, separated_list0, separated_list1},
    sequence::{delimited, pair, preceded, separated_pair, terminated},
    Parser,
};

use super::{
    asn1_type, asn1_value,
    common::{
        extension_marker, identifier, in_braces, in_parentheses, range_seperator,
        skip_ws_and_comments,
    },
    error::{MiscError, ParserResult},
    information_object_class::object_set,
    into_inner,
    parameterization::parameters,
    skip_ws,
    util::{opt_delimited, take_until_and_not, take_until_unbalanced},
};

type SubsetMember<'s> = (&'s str, Option<Vec<Constraint>>, Option<ComponentPresence>);

pub fn constraint(input: Input<'_>) -> ParserResult<'_, Vec<Constraint>> {
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
    ))))
    .parse(input)
}

pub fn single_constraint(input: Input<'_>) -> ParserResult<'_, Constraint> {
    skip_ws_and_comments(in_parentheses(alt((
        map(content_constraint, Constraint::ContentConstraint),
        map(table_constraint, Constraint::TableConstraint),
        map(element_set, Constraint::SubtypeConstraint),
    ))))
    .parse(input)
}

pub fn set_operator(input: Input<'_>) -> ParserResult<'_, SetOperator> {
    skip_ws_and_comments(alt((
        value(SetOperator::Intersection, tag(INTERSECTION)),
        value(SetOperator::Intersection, tag(CARET)),
        value(SetOperator::Union, tag(UNION)),
        value(SetOperator::Union, tag(PIPE)),
        value(SetOperator::Except, tag(EXCEPT)),
    )))
    .parse(input)
}

pub fn element_set(input: Input<'_>) -> ParserResult<'_, ElementSet> {
    into(pair(
        alt((
            map(set_operation, ElementOrSetOperation::SetOperation),
            map(subtype_element, ElementOrSetOperation::Element),
        )),
        opt(skip_ws_and_comments(preceded(
            char(COMMA),
            extension_marker,
        ))),
    ))
    .parse(input)
}

fn set_operation(input: Input<'_>) -> ParserResult<'_, SetOperation> {
    into((
        subtype_element,
        set_operator,
        alt((
            map(set_operation, ElementOrSetOperation::SetOperation),
            map(subtype_element, ElementOrSetOperation::Element),
        )),
    ))
    .parse(input)
}

fn subtype_element(input: Input<'_>) -> ParserResult<'_, SubtypeElement> {
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
    ))
    .parse(input)
}

fn extension_additions(input: Input<'_>) -> ParserResult<'_, ()> {
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
    )
    .parse(input)
}

fn single_value(input: Input<'_>) -> ParserResult<'_, SubtypeElement> {
    opt_delimited(
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
    )
    .parse(input)
}

fn contained_subtype(input: Input<'_>) -> ParserResult<'_, SubtypeElement> {
    opt_delimited(
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
    )
    .parse(input)
}

fn value_range(input: Input<'_>) -> ParserResult<'_, SubtypeElement> {
    opt_delimited(
        skip_ws_and_comments(char(LEFT_PARENTHESIS)),
        skip_ws_and_comments(map(
            (
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
            ),
            |(min, max, ext)| SubtypeElement::ValueRange {
                min,
                max,
                extensible: ext.is_some(),
            },
        )),
        skip_ws_and_comments(char(RIGHT_PARENTHESIS)),
    )
    .parse(input)
}

fn size_constraint(input: Input<'_>) -> ParserResult<'_, SubtypeElement> {
    opt_delimited(
        skip_ws_and_comments(char(LEFT_PARENTHESIS)),
        skip_ws_and_comments(into(preceded(tag(SIZE), single_constraint))),
        skip_ws_and_comments(char(RIGHT_PARENTHESIS)),
    )
    .parse(input)
}

fn pattern_constraint(input: Input<'_>) -> ParserResult<'_, SubtypeElement> {
    map(
        opt_delimited(
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
    )
    .parse(input)
}

fn user_defined_constraint(input: Input<'_>) -> ParserResult<'_, SubtypeElement> {
    map(
        opt_delimited(
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
    )
    .parse(input)
}

/// Parses a PermittedAlphabet constraint.
/// ### Reference in X680
/// >* _51.7.1 The "PermittedAlphabet" notation shall be: `PermittedAlphabet ::= FROM Constraint`_
/// >* _51.7.2 A "PermittedAlphabet" specifies all values which can be constructed using a sub-alphabet of the parent string. This notation can only be applied to restricted character string types._
/// >* _51.7.3 The "Constraint" shall use the "SubtypeConstraint" alternative of "ConstraintSpec". Each "SubtypeElements" within that "SubtypeConstraint" shall be one of the four alternatives "SingleValue", "ContainedSubtype", "ValueRange", and "SizeConstraint". The sub-alphabet includes precisely those characters which appear in one or more of the values of the parent string type which are allowed by the "Constraint"._
/// >* _51.7.4 If "Constraint" is extensible, then the set of values selected by the permitted alphabet constraint is extensible. The set of values in the root are those permitted by the root of "Constraint", and the extension additions are those values permitted by the root together with the extension-additions of "Constraint", excluding those values already in the root._
fn permitted_alphabet_constraint(input: Input<'_>) -> ParserResult<'_, SubtypeElement> {
    opt_delimited(
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
    )
    .parse(input)
}

fn single_type_constraint(input: Input<'_>) -> ParserResult<'_, SubtypeElement> {
    opt_delimited(
        skip_ws_and_comments(char(LEFT_PARENTHESIS)),
        skip_ws_and_comments(into(preceded(
            tag(WITH_COMPONENT),
            skip_ws_and_comments(map(constraint, SubtypeElement::SingleTypeConstraint)),
        ))),
        skip_ws_and_comments(char(RIGHT_PARENTHESIS)),
    )
    .parse(input)
}

fn multiple_type_constraints(input: Input<'_>) -> ParserResult<'_, SubtypeElement> {
    opt_delimited(
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
    )
    .parse(input)
}

fn subset_member(input: Input<'_>) -> ParserResult<'_, SubsetMember<'_>> {
    skip_ws_and_comments((
        identifier,
        opt(skip_ws_and_comments(constraint)),
        opt(skip_ws_and_comments(alt((
            value(ComponentPresence::Present, tag(PRESENT)),
            value(ComponentPresence::Absent, tag(ABSENT)),
        )))),
    ))
    .parse(input)
}

fn content_constraint(input: Input<'_>) -> ParserResult<'_, ContentConstraint> {
    opt_delimited(
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
    )
    .parse(input)
}

fn table_constraint(input: Input<'_>) -> ParserResult<'_, TableConstraint> {
    opt_delimited(
        skip_ws_and_comments(char(LEFT_PARENTHESIS)),
        skip_ws_and_comments(into(pair(
            object_set,
            opt(in_braces(separated_list1(
                skip_ws_and_comments(char(COMMA)),
                relational_constraint,
            ))),
        ))),
        skip_ws_and_comments(char(RIGHT_PARENTHESIS)),
    )
    .parse(input)
}

fn relational_constraint(input: Input<'_>) -> ParserResult<'_, RelationalConstraint> {
    into(skip_ws_and_comments(preceded(
        char(AT),
        pair(many0_count(char(DOT)), identifier),
    )))
    .parse(input)
}

fn property_settings_constraint(input: Input<'_>) -> ParserResult<'_, SubtypeElement> {
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
                    .map(|s| {
                        PropertyAndSettingsPair::try_from(s)
                            .map_err(|_| MiscError("Failed to parse property-settings pair."))
                    })
                    .collect::<Result<Vec<PropertyAndSettingsPair>, _>>()
                    .map(|settings| {
                        SubtypeElement::PropertySettings(PropertySettings {
                            property_settings_list: settings,
                        })
                    })
            },
        ),
    )
    .parse(input)
}

fn settings_identifier(input: Input<'_>) -> ParserResult<'_, &str> {
    into_inner(alt((
        tag(BasicSettings::NAME),
        tag(DateSettings::NAME),
        tag(YearSettings::NAME),
        tag(TimeSettings::NAME),
        tag(LocalOrUtcSettings::NAME),
        tag(IntervalTypeSettings::NAME),
        tag(StartEndPointSettings::NAME),
        tag(RecurrenceSettings::NAME),
        tag(MidnightSettings::NAME),
    )))
    .parse(input)
}

#[cfg(test)]
mod tests {
    use crate::intermediate::{information_object::*, types::*};

    use crate::lexer::constraint::*;

    #[test]
    fn parses_value_constraint() {
        assert_eq!(
            constraint("(5)".into()).unwrap().1,
            vec![Constraint::SubtypeConstraint(ElementSet {
                set: ElementOrSetOperation::Element(SubtypeElement::SingleValue {
                    value: ASN1Value::Integer(5),
                    extensible: false
                }),
                extensible: false
            })]
        );
        assert_eq!(
            constraint("(5..9)".into()).unwrap().1,
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
            constraint("(-5..9)".into()).unwrap().1,
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
            constraint("(-9..-4,...)".into()).unwrap().1,
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
            constraint("(1..32767,..., 8388607)".into()).unwrap().1,
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
            constraint("((5..9))".into()).unwrap().1,
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
            constraint("(-9..-4, -- Very annoying! -- ...)".into())
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
            constraint("(-9-- Very annoying! --..-4,  ...)".into())
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
            constraint("(SIZE(3..16, ...))".into()).unwrap().1,
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
            constraint(r#"(ALL EXCEPT 1)"#.into()).unwrap().1,
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
            constraint(
                r#"((WITH COMPONENT (WITH COMPONENTS {..., containerId (ALL EXCEPT 1)})) |
          (WITH COMPONENT (WITH COMPONENTS {..., containerId (ALL EXCEPT 2)})))"#
                    .into()
            )
            .unwrap()
            .1,
            vec![Constraint::SubtypeConstraint(ElementSet {
                set: ElementOrSetOperation::SetOperation(SetOperation {
                    base: SubtypeElement::SingleTypeConstraint(vec![
                        Constraint::SubtypeConstraint(ElementSet {
                            extensible: false,
                            set: ElementOrSetOperation::Element(
                                SubtypeElement::MultipleTypeConstraints(InnerTypeConstraint {
                                    is_partial: true,
                                    constraints: vec![ConstrainedComponent {
                                        identifier: "containerId".into(),
                                        constraints: vec![Constraint::SubtypeConstraint(
                                            ElementSet {
                                                set: ElementOrSetOperation::SetOperation(
                                                    SetOperation {
                                                        base: SubtypeElement::SingleValue {
                                                            value: ASN1Value::All,
                                                            extensible: false
                                                        },
                                                        operator: SetOperator::Except,
                                                        operant: Box::new(
                                                            ElementOrSetOperation::Element(
                                                                SubtypeElement::SingleValue {
                                                                    value: ASN1Value::Integer(1),
                                                                    extensible: false
                                                                }
                                                            )
                                                        )
                                                    }
                                                ),
                                                extensible: false
                                            }
                                        )],
                                        presence: ComponentPresence::Unspecified
                                    }]
                                })
                            )
                        })
                    ]),
                    operator: SetOperator::Union,
                    operant: Box::new(ElementOrSetOperation::Element(
                        SubtypeElement::SingleTypeConstraint(vec![Constraint::SubtypeConstraint(
                            ElementSet {
                                extensible: false,
                                set: ElementOrSetOperation::Element(
                                    SubtypeElement::MultipleTypeConstraints(InnerTypeConstraint {
                                        is_partial: true,
                                        constraints: vec![ConstrainedComponent {
                                            identifier: "containerId".into(),
                                            constraints: vec![Constraint::SubtypeConstraint(
                                                ElementSet {
                                                    set: ElementOrSetOperation::SetOperation(
                                                        SetOperation {
                                                            base: SubtypeElement::SingleValue {
                                                                value: ASN1Value::All,
                                                                extensible: false
                                                            },
                                                            operator: SetOperator::Except,
                                                            operant: Box::new(
                                                                ElementOrSetOperation::Element(
                                                                    SubtypeElement::SingleValue {
                                                                        value: ASN1Value::Integer(
                                                                            2
                                                                        ),
                                                                        extensible: false
                                                                    }
                                                                )
                                                            )
                                                        }
                                                    ),
                                                    extensible: false
                                                }
                                            )],
                                            presence: ComponentPresence::Unspecified
                                        }]
                                    })
                                )
                            }
                        )])
                    ))
                }),
                extensible: false
            })]
        )
    }

    #[test]
    fn parses_full_component_constraint() {
        assert_eq!(
            constraint(
                "(WITH COMPONENTS
                  {ordering ABSENT ,
                  sales (0..5) PRESENT,
                  e-cash-return ABSENT } )"
                    .into()
            )
            .unwrap()
            .1,
            vec![Constraint::SubtypeConstraint(ElementSet {
                set: ElementOrSetOperation::Element(SubtypeElement::MultipleTypeConstraints(
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
            constraint(
                "( WITH COMPONENTS
                      {... ,
                      ordering ABSENT,
                      sales (0..5) } )"
                    .into()
            )
            .unwrap()
            .1,
            vec![Constraint::SubtypeConstraint(ElementSet {
                set: ElementOrSetOperation::Element(SubtypeElement::MultipleTypeConstraints(
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
            constraint(
                "((WITH COMPONENT (WITH COMPONENTS {..., eventDeltaTime PRESENT})) |
                    (WITH COMPONENT (WITH COMPONENTS {..., eventDeltaTime ABSENT})))
                "
                .into()
            )
            .unwrap()
            .1,
            vec![Constraint::SubtypeConstraint(ElementSet {
                set: ElementOrSetOperation::SetOperation(SetOperation {
                    base: SubtypeElement::SingleTypeConstraint(vec![
                        Constraint::SubtypeConstraint(ElementSet {
                            set: ElementOrSetOperation::Element(
                                SubtypeElement::MultipleTypeConstraints(InnerTypeConstraint {
                                    is_partial: true,
                                    constraints: vec![ConstrainedComponent {
                                        identifier: "eventDeltaTime".into(),
                                        constraints: vec![],
                                        presence: ComponentPresence::Present
                                    }]
                                }),
                            ),
                            extensible: false
                        })
                    ]),
                    operator: SetOperator::Union,
                    operant: Box::new(ElementOrSetOperation::Element(
                        SubtypeElement::SingleTypeConstraint(vec![Constraint::SubtypeConstraint(
                            ElementSet {
                                set: ElementOrSetOperation::Element(
                                    SubtypeElement::MultipleTypeConstraints(InnerTypeConstraint {
                                        is_partial: true,
                                        constraints: vec![ConstrainedComponent {
                                            identifier: "eventDeltaTime".into(),
                                            constraints: vec![],
                                            presence: ComponentPresence::Absent
                                        }]
                                    })
                                ),
                                extensible: false
                            }
                        )])
                    ))
                }),
                extensible: false
            })]
        );
    }

    #[test]
    fn parses_composite_component_constraint() {
        assert_eq!(
            constraint(
                "((WITH COMPONENTS {..., laneId PRESENT, connectionId ABSENT }) |
                    (WITH COMPONENTS {..., laneId ABSENT, connectionId PRESENT }))
                "
                .into()
            )
            .unwrap()
            .1,
            vec![Constraint::SubtypeConstraint(ElementSet {
                set: ElementOrSetOperation::SetOperation(SetOperation {
                    base: SubtypeElement::MultipleTypeConstraints(InnerTypeConstraint {
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
                        SubtypeElement::MultipleTypeConstraints(InnerTypeConstraint {
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
            constraint(
                "(0..3|5..8|10)
                "
                .into()
            )
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
            constraint(
                "(unknown   | passengerCar..tram
                  | agricultural)"
                    .into()
            )
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
            constraint(
                "({
                  My-ops |
                  {
                    &id 5,
                    &Type INTEGER (1..6)
                  } |
                  {ConnectionManeuverAssist-addGrpC  IDENTIFIED BY addGrpC},
                  ...
                })"
                .into()
            )
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
            value_range(r#""a".."z""#.into()).unwrap().1,
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
            permitted_alphabet_constraint(
                r#"(FROM ("a".."z" | "A".."Z" | "0".."9" | ".-"))"#.into()
            )
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
            constraint(r#"(FROM ("a".."z" | "A".."Z" | "0".."9" | ".-")) (SIZE (1..255))"#.into())
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
            constraint(
                r#"(WITH COMPONENTS {
                mantissa (-16777215..16777215),
                base (2),
                exponent (-125..128) } )"#
                    .into()
            )
            .unwrap()
            .1,
            vec![Constraint::SubtypeConstraint(ElementSet {
                set: ElementOrSetOperation::Element(SubtypeElement::MultipleTypeConstraints(
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
            constraint(r#"(PATTERN "[a-zA-Z]#(1,8)(-[a-zA-Z0-9]#(1,8))*")"#.into())
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
                r#"(CONSTRAINED BY {/* XML representation of the XSD pattern "\d\d\d\d-\d\d-\d\dT\d\d:\d\d:\d\d[-,+]\d\d:\d\d" */})"#.into()
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
            constraint("(SIZE(1..4),...)".into()).unwrap().1,
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
            constraint("(SIZE(1..4,...))".into()).unwrap().1,
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
            constraint(r#"(SETTINGS "Midnight=Start")"#.into())
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
            constraint(r#"(1..65535, ..., 65536..109999)"#.into())
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

    #[test]
    fn with_component_intersection() {
        assert_eq!(
            vec![Constraint::SubtypeConstraint(ElementSet {
                set: ElementOrSetOperation::SetOperation(SetOperation {
                    base: SubtypeElement::SingleTypeConstraint(vec![
                        Constraint::SubtypeConstraint(ElementSet {
                            set: ElementOrSetOperation::Element(SubtypeElement::ContainedSubtype {
                                subtype: ASN1Type::ElsewhereDeclaredType(DeclarationElsewhere {
                                    parent: None,
                                    identifier: "EtsiTs103097Certificate".into(),
                                    constraints: vec![],
                                }),
                                extensible: false,
                            }),
                            extensible: false,
                        }),
                    ]),
                    operator: SetOperator::Intersection,
                    operant: Box::new(ElementOrSetOperation::Element(
                        SubtypeElement::SizeConstraint(Box::new(ElementOrSetOperation::Element(
                            SubtypeElement::SingleValue {
                                value: ASN1Value::Integer(1,),
                                extensible: false,
                            },
                        ))),
                    ))
                }),
                extensible: false,
            })],
            constraint(r#"((WITH COMPONENT (EtsiTs103097Certificate))^(SIZE(1)))"#.into())
                .unwrap()
                .1
        )
    }
}
