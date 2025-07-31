use std::borrow::Cow;

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
    into_inner, skip_ws,
    util::{opt_delimited, take_until_and_not, take_until_unbalanced},
};

pub fn constraints(input: Input<'_>) -> ParserResult<'_, Vec<Constraint>> {
    skip_ws_and_comments(many1(alt((
        constraint,
        // Handle SIZE constraint without external parentheses
        map(size_constraint, |c| {
            Constraint::Subtype(ElementSetSpecs {
                set: ElementOrSetOperation::Element(c),
                extensible: false,
            })
        }),
    ))))
    .parse(input)
}

/// Parses a Constraint.
///
/// # Syntax
///
/// ```text
/// Constraint ::=
///     "(" ConstraintSpec ExceptionSpec ")"
///
/// ConstraintSpec ::=
///     SubtypeConstraint  |
///     GeneralConstraint
///
/// SubtypeConstraint ::=
///     ElementSetSpecs
/// ```
pub fn constraint(input: Input<'_>) -> ParserResult<'_, Constraint> {
    skip_ws_and_comments(in_parentheses(alt((
        general_constraint,
        map(element_set_specs, Constraint::Subtype),
    ))))
    .parse(input)
}

/// Parses a GeneralConstraint.
///
/// # Syntax
///
/// ```text
/// GeneralConstraint ::=
///     UserDefinedConstraint  |
///     TableConstraint        |
///     ContentsConstraint
/// ```
pub fn general_constraint(input: Input<'_>) -> ParserResult<'_, Constraint> {
    alt((
        // TODO: UserDefinedConstraint
        map(content_constraint, Constraint::Content),
        map(table_constraint, Constraint::Table),
    ))
    .parse(input)
}

pub fn set_operator(input: Input<'_>) -> ParserResult<'_, SetOperator> {
    skip_ws_and_comments(alt((
        intersection_mark,
        union_mark,
        value(SetOperator::Except, tag(EXCEPT)),
    )))
    .parse(input)
}

/// Parses a ElementSetSpecs.
///
/// # Syntax
///
/// ```text
/// ElementSetSpecs ::=
///     RootElementSetSpec            |
///     RootElementSetSpec "," "..."  |
///     RootElementSetSpec "," "..." "," AdditionalElementSetSpec
///
/// RootElementSetSpec ::=
///     ElementSetSpec
///
/// AdditionalElementSetSpec ::=
///     ElementSetSpec
///
/// ElementSetSpec ::=
///     Unions  |
///     ALL Exclusions
///
/// Unions ::=
///     Intersections  |
///     UElems UnionMark Intersections
///
/// UElems ::=
///     Unions
///
/// Intersections ::=
///     IntersectionElements  |
///     IElems IntersectionMark IntersectionElements
///
/// IElems ::=
///     Intersections
///
/// IntersectionElements ::=
///     Elements  |
///     Elems Exclusions
///
/// Elems ::=
///     Elements
///
/// Exclusions ::=
///     EXCEPT Elements
/// ```
fn element_set_specs(input: Input<'_>) -> ParserResult<'_, ElementSetSpecs> {
    into(pair(
        alt((
            map(set_operation, ElementOrSetOperation::SetOperation),
            map(subtype_elements, ElementOrSetOperation::Element),
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
        subtype_elements,
        set_operator,
        alt((
            map(set_operation, ElementOrSetOperation::SetOperation),
            map(subtype_elements, ElementOrSetOperation::Element),
        )),
    ))
    .parse(input)
}

/// Parses a UnionMark.
///
/// # Syntax
///
/// ```text
/// UnionMark ::=
///     "|"  |
///     UNION
/// ```
pub fn union_mark(input: Input<'_>) -> ParserResult<'_, SetOperator> {
    value(SetOperator::Union, alt((tag(PIPE), tag(UNION)))).parse(input)
}

/// Parses a IntersectionMark.
///
/// # Syntax
///
/// ```text
/// IntersectionMark ::=
///     "^"  |
///     INTERSECTION
/// ```
pub fn intersection_mark(input: Input<'_>) -> ParserResult<'_, SetOperator> {
    value(
        SetOperator::Intersection,
        alt((tag(CARET), tag(INTERSECTION))),
    )
    .parse(input)
}

/// Parses a SubtypeElements.
///
/// # Syntax
///
/// ```text
/// SubtypeElements ::=
///     SingleValue          |
///     ContainedSubtype     |
///     ValueRange           |
///     PermittedAlphabet    |
///     SizeConstraint       |
///     TypeConstraint       |
///     InnerTypeConstraints |
///     PatternConstraint    |
///     PropertySettings     |
///     DurationRange        |
///     TimePointRange       |
///     RecurrenceRange
/// ```
fn subtype_elements(input: Input<'_>) -> ParserResult<'_, SubtypeElements> {
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

fn single_value(input: Input<'_>) -> ParserResult<'_, SubtypeElements> {
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

fn contained_subtype(input: Input<'_>) -> ParserResult<'_, SubtypeElements> {
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
            |(t, ext)| SubtypeElements::ContainedSubtype {
                subtype: t,
                extensible: ext.is_some(),
            },
        )),
        skip_ws_and_comments(char(RIGHT_PARENTHESIS)),
    )
    .parse(input)
}

fn value_range(input: Input<'_>) -> ParserResult<'_, SubtypeElements> {
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
            |(min, max, ext)| SubtypeElements::ValueRange {
                min,
                max,
                extensible: ext.is_some(),
            },
        )),
        skip_ws_and_comments(char(RIGHT_PARENTHESIS)),
    )
    .parse(input)
}

fn size_constraint(input: Input<'_>) -> ParserResult<'_, SubtypeElements> {
    opt_delimited(
        skip_ws_and_comments(char(LEFT_PARENTHESIS)),
        skip_ws_and_comments(into(preceded(tag(SIZE), constraint))),
        skip_ws_and_comments(char(RIGHT_PARENTHESIS)),
    )
    .parse(input)
}

fn pattern_constraint(input: Input<'_>) -> ParserResult<'_, SubtypeElements> {
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
        SubtypeElements::PatternConstraint,
    )
    .parse(input)
}

fn user_defined_constraint(input: Input<'_>) -> ParserResult<'_, SubtypeElements> {
    map(
        opt_delimited(
            skip_ws_and_comments(char(LEFT_PARENTHESIS)),
            skip_ws_and_comments(user_defined_constraint_real),
            skip_ws_and_comments(char(RIGHT_PARENTHESIS)),
        ),
        SubtypeElements::UserDefinedConstraint,
    )
    .parse(input)
}

/// Parses a UserDefinedConstraint.
///
/// # Syntax
///
/// ```text
/// UserDefinedConstraint ::=
///     CONSTRAINED BY "{" UserDefinedConstraintParameter "," * "}"
///
/// UserDefinedConstraintParameter ::=
///     Governor ":" Value   |
///     Governor ":" Object  |
///     DefinedObjectSet     |
///     Type                 |
///     DefinedObjectClass
/// ```
fn user_defined_constraint_real(input: Input<'_>) -> ParserResult<'_, UserDefinedConstraint> {
    skip_ws_and_comments(into(preceded(
        tag(CONSTRAINED_BY),
        skip_ws_and_comments(delimited(
            char(LEFT_BRACE),
            take_until_unbalanced("{", "}"),
            char(RIGHT_BRACE),
        )),
    )))
    .parse(input)
}

/// Parses a PermittedAlphabet constraint.
/// ### Reference in X680
/// >* _51.7.1 The "PermittedAlphabet" notation shall be: `PermittedAlphabet ::= FROM Constraint`_
/// >* _51.7.2 A "PermittedAlphabet" specifies all values which can be constructed using a sub-alphabet of the parent string. This notation can only be applied to restricted character string types._
/// >* _51.7.3 The "Constraint" shall use the "SubtypeConstraint" alternative of "ConstraintSpec". Each "SubtypeElements" within that "SubtypeConstraint" shall be one of the four alternatives "SingleValue", "ContainedSubtype", "ValueRange", and "SizeConstraint". The sub-alphabet includes precisely those characters which appear in one or more of the values of the parent string type which are allowed by the "Constraint"._
/// >* _51.7.4 If "Constraint" is extensible, then the set of values selected by the permitted alphabet constraint is extensible. The set of values in the root are those permitted by the root of "Constraint", and the extension additions are those values permitted by the root together with the extension-additions of "Constraint", excluding those values already in the root._
fn permitted_alphabet_constraint(input: Input<'_>) -> ParserResult<'_, SubtypeElements> {
    opt_delimited(
        skip_ws_and_comments(char(LEFT_PARENTHESIS)),
        skip_ws_and_comments(map(
            preceded(
                tag(FROM),
                in_parentheses(alt((
                    map(set_operation, ElementOrSetOperation::SetOperation),
                    map(subtype_elements, ElementOrSetOperation::Element),
                ))),
            ),
            |i| SubtypeElements::PermittedAlphabet(Box::new(i)),
        )),
        skip_ws_and_comments(char(RIGHT_PARENTHESIS)),
    )
    .parse(input)
}

fn single_type_constraint(input: Input<'_>) -> ParserResult<'_, SubtypeElements> {
    opt_delimited(
        skip_ws_and_comments(char(LEFT_PARENTHESIS)),
        skip_ws_and_comments(into(preceded(
            tag(WITH_COMPONENT),
            skip_ws_and_comments(map(constraints, SubtypeElements::SingleTypeConstraint)),
        ))),
        skip_ws_and_comments(char(RIGHT_PARENTHESIS)),
    )
    .parse(input)
}

/// Parses an optionally parenthesised MultipleTypeConstraints.
///
/// # Syntax
///
/// ```text
/// MultipleTypeConstraints ::=
///     FullSpecification  |
///     PartialSpecification
///
/// FullSpecification ::=
///     "{" TypeConstraints "}"
///
/// PartialSpecification ::=
///     "{" "..." "," TypeConstraints "}"
///
/// TypeConstraints ::=
///     NamedConstraint  |
///     NamedConstraint "," TypeConstraints
/// ```
fn multiple_type_constraints(input: Input<'_>) -> ParserResult<'_, SubtypeElements> {
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
                    named_constraint,
                    opt(skip_ws_and_comments(char(COMMA))),
                )),
            )),
        ))),
        skip_ws_and_comments(char(RIGHT_PARENTHESIS)),
    )
    .parse(input)
}

/// Parses a NamedConstraint.
///
/// # Syntax
///
/// ```text
/// NamedConstraint ::=
///     identifier ComponentConstraint
///
/// ComponentConstraint ::=
///     ValueConstraint PresenceConstraint
///
/// ValueConstraint ::=
///     Constraint  |
///     empty
///
/// PresenceConstraint ::=
///     PRESENT   |
///     ABSENT    |
///     OPTIONAL  |
///     empty
/// ```
fn named_constraint(input: Input<'_>) -> ParserResult<'_, NamedConstraint> {
    map(
        skip_ws_and_comments((
            identifier,
            opt(skip_ws_and_comments(constraints)),
            opt(skip_ws_and_comments(alt((
                value(ComponentPresence::Present, tag(PRESENT)),
                value(ComponentPresence::Absent, tag(ABSENT)),
            )))),
        )),
        |v| NamedConstraint {
            identifier: Cow::Borrowed(v.0),
            constraints: v.1.unwrap_or_default(),
            presence: v.2.unwrap_or(ComponentPresence::Unspecified),
        },
    )
    .parse(input)
}

fn content_constraint(input: Input<'_>) -> ParserResult<'_, ContentConstraint> {
    opt_delimited(
        skip_ws_and_comments(char(LEFT_PARENTHESIS)),
        skip_ws_and_comments(alt((
            map(
                pair(
                    preceded(skip_ws_and_comments(tag(CONTAINING)), skip_ws(asn1_type)),
                    preceded(skip_ws_and_comments(tag(ENCODED_BY)), skip_ws(asn1_value)),
                ),
                |v| ContentConstraint::ContainingEncodedBy {
                    containing: v.0,
                    encoded_by: v.1,
                },
            ),
            map(
                preceded(skip_ws_and_comments(tag(CONTAINING)), skip_ws(asn1_type)),
                ContentConstraint::Containing,
            ),
            map(
                preceded(skip_ws_and_comments(tag(ENCODED_BY)), skip_ws(asn1_value)),
                ContentConstraint::EncodedBy,
            ),
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

fn property_settings_constraint(input: Input<'_>) -> ParserResult<'_, SubtypeElements> {
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
                        SubtypeElements::PropertySettings(PropertySettings {
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
            constraints("(5)".into()).unwrap().1,
            vec![Constraint::Subtype(ElementSetSpecs {
                set: ElementOrSetOperation::Element(SubtypeElements::SingleValue {
                    value: ASN1Value::Integer(5),
                    extensible: false
                }),
                extensible: false
            })]
        );
        assert_eq!(
            constraints("(5..9)".into()).unwrap().1,
            vec![Constraint::Subtype(ElementSetSpecs {
                set: ElementOrSetOperation::Element(SubtypeElements::ValueRange {
                    min: Some(ASN1Value::Integer(5)),
                    max: Some(ASN1Value::Integer(9)),
                    extensible: false
                }),
                extensible: false
            })]
        );
        assert_eq!(
            constraints("(-5..9)".into()).unwrap().1,
            vec![Constraint::Subtype(ElementSetSpecs {
                set: ElementOrSetOperation::Element(SubtypeElements::ValueRange {
                    min: Some(ASN1Value::Integer(-5)),
                    max: Some(ASN1Value::Integer(9)),
                    extensible: false
                }),
                extensible: false
            })]
        );
        assert_eq!(
            constraints("(-9..-4,...)".into()).unwrap().1,
            vec![Constraint::Subtype(ElementSetSpecs {
                set: ElementOrSetOperation::Element(SubtypeElements::ValueRange {
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
            constraints("(1..32767,..., 8388607)".into()).unwrap().1,
            vec![Constraint::Subtype(ElementSetSpecs {
                set: ElementOrSetOperation::Element(SubtypeElements::ValueRange {
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
            constraints("((5..9))".into()).unwrap().1,
            vec![Constraint::Subtype(ElementSetSpecs {
                set: ElementOrSetOperation::Element(SubtypeElements::ValueRange {
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
            constraints("(-9..-4, -- Very annoying! -- ...)".into())
                .unwrap()
                .1,
            vec![Constraint::Subtype(ElementSetSpecs {
                set: ElementOrSetOperation::Element(SubtypeElements::ValueRange {
                    min: Some(ASN1Value::Integer(-9)),
                    max: Some(ASN1Value::Integer(-4)),
                    extensible: true
                }),
                extensible: false
            })]
        );
        assert_eq!(
            constraints("(-9-- Very annoying! --..-4,  ...)".into())
                .unwrap()
                .1,
            vec![Constraint::Subtype(ElementSetSpecs {
                set: ElementOrSetOperation::Element(SubtypeElements::ValueRange {
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
            constraints("(SIZE(3..16, ...))".into()).unwrap().1,
            vec![Constraint::Subtype(ElementSetSpecs {
                set: ElementOrSetOperation::Element(SubtypeElements::SizeConstraint(Box::new(
                    ElementOrSetOperation::Element(SubtypeElements::ValueRange {
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
            constraints(r#"(ALL EXCEPT 1)"#.into()).unwrap().1,
            vec![Constraint::Subtype(ElementSetSpecs {
                set: ElementOrSetOperation::SetOperation(SetOperation {
                    base: SubtypeElements::SingleValue {
                        value: ASN1Value::All,
                        extensible: false
                    },
                    operator: SetOperator::Except,
                    operant: Box::new(ElementOrSetOperation::Element(
                        SubtypeElements::SingleValue {
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
            constraints(
                r#"((WITH COMPONENT (WITH COMPONENTS {..., containerId (ALL EXCEPT 1)})) |
          (WITH COMPONENT (WITH COMPONENTS {..., containerId (ALL EXCEPT 2)})))"#
                    .into()
            )
            .unwrap()
            .1,
            vec![Constraint::Subtype(ElementSetSpecs {
                set: ElementOrSetOperation::SetOperation(SetOperation {
                    base: SubtypeElements::SingleTypeConstraint(vec![Constraint::Subtype(
                        ElementSetSpecs {
                            extensible: false,
                            set: ElementOrSetOperation::Element(
                                SubtypeElements::MultipleTypeConstraints(InnerTypeConstraint {
                                    is_partial: true,
                                    constraints: vec![NamedConstraint {
                                        identifier: "containerId".into(),
                                        constraints: vec![Constraint::Subtype(ElementSetSpecs {
                                            set: ElementOrSetOperation::SetOperation(
                                                SetOperation {
                                                    base: SubtypeElements::SingleValue {
                                                        value: ASN1Value::All,
                                                        extensible: false
                                                    },
                                                    operator: SetOperator::Except,
                                                    operant: Box::new(
                                                        ElementOrSetOperation::Element(
                                                            SubtypeElements::SingleValue {
                                                                value: ASN1Value::Integer(1),
                                                                extensible: false
                                                            }
                                                        )
                                                    )
                                                }
                                            ),
                                            extensible: false
                                        })],
                                        presence: ComponentPresence::Unspecified
                                    }]
                                })
                            )
                        }
                    )]),
                    operator: SetOperator::Union,
                    operant: Box::new(ElementOrSetOperation::Element(
                        SubtypeElements::SingleTypeConstraint(vec![Constraint::Subtype(
                            ElementSetSpecs {
                                extensible: false,
                                set: ElementOrSetOperation::Element(
                                    SubtypeElements::MultipleTypeConstraints(InnerTypeConstraint {
                                        is_partial: true,
                                        constraints: vec![NamedConstraint {
                                            identifier: "containerId".into(),
                                            constraints: vec![Constraint::Subtype(
                                                ElementSetSpecs {
                                                    set: ElementOrSetOperation::SetOperation(
                                                        SetOperation {
                                                            base: SubtypeElements::SingleValue {
                                                                value: ASN1Value::All,
                                                                extensible: false
                                                            },
                                                            operator: SetOperator::Except,
                                                            operant: Box::new(
                                                                ElementOrSetOperation::Element(
                                                                    SubtypeElements::SingleValue {
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
            dbg!(constraints(
                "(WITH COMPONENTS
                  {ordering ABSENT ,
                  sales (0..5) PRESENT,
                  e-cash-return ABSENT } )"
                    .into()
            ))
            .unwrap()
            .1,
            vec![Constraint::Subtype(ElementSetSpecs {
                set: ElementOrSetOperation::Element(SubtypeElements::MultipleTypeConstraints(
                    InnerTypeConstraint {
                        is_partial: false,
                        constraints: vec![
                            NamedConstraint {
                                identifier: "ordering".into(),
                                constraints: vec![],
                                presence: ComponentPresence::Absent
                            },
                            NamedConstraint {
                                identifier: "sales".into(),
                                constraints: vec![Constraint::Subtype(ElementSetSpecs {
                                    set: ElementOrSetOperation::Element(
                                        SubtypeElements::ValueRange {
                                            min: Some(ASN1Value::Integer(0)),
                                            max: Some(ASN1Value::Integer(5)),
                                            extensible: false
                                        }
                                    ),
                                    extensible: false
                                })],
                                presence: ComponentPresence::Present
                            },
                            NamedConstraint {
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
            constraints(
                "( WITH COMPONENTS
                      {... ,
                      ordering ABSENT,
                      sales (0..5) } )"
                    .into()
            )
            .unwrap()
            .1,
            vec![Constraint::Subtype(ElementSetSpecs {
                set: ElementOrSetOperation::Element(SubtypeElements::MultipleTypeConstraints(
                    InnerTypeConstraint {
                        is_partial: true,
                        constraints: vec![
                            NamedConstraint {
                                identifier: "ordering".into(),
                                constraints: vec![],
                                presence: ComponentPresence::Absent
                            },
                            NamedConstraint {
                                identifier: "sales".into(),
                                constraints: vec![Constraint::Subtype(ElementSetSpecs {
                                    set: ElementOrSetOperation::Element(
                                        SubtypeElements::ValueRange {
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
            constraints(
                "((WITH COMPONENT (WITH COMPONENTS {..., eventDeltaTime PRESENT})) |
                    (WITH COMPONENT (WITH COMPONENTS {..., eventDeltaTime ABSENT})))
                "
                .into()
            )
            .unwrap()
            .1,
            vec![Constraint::Subtype(ElementSetSpecs {
                set: ElementOrSetOperation::SetOperation(SetOperation {
                    base: SubtypeElements::SingleTypeConstraint(vec![Constraint::Subtype(
                        ElementSetSpecs {
                            set: ElementOrSetOperation::Element(
                                SubtypeElements::MultipleTypeConstraints(InnerTypeConstraint {
                                    is_partial: true,
                                    constraints: vec![NamedConstraint {
                                        identifier: "eventDeltaTime".into(),
                                        constraints: vec![],
                                        presence: ComponentPresence::Present
                                    }]
                                }),
                            ),
                            extensible: false
                        }
                    )]),
                    operator: SetOperator::Union,
                    operant: Box::new(ElementOrSetOperation::Element(
                        SubtypeElements::SingleTypeConstraint(vec![Constraint::Subtype(
                            ElementSetSpecs {
                                set: ElementOrSetOperation::Element(
                                    SubtypeElements::MultipleTypeConstraints(InnerTypeConstraint {
                                        is_partial: true,
                                        constraints: vec![NamedConstraint {
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
            constraints(
                "((WITH COMPONENTS {..., laneId PRESENT, connectionId ABSENT }) |
                    (WITH COMPONENTS {..., laneId ABSENT, connectionId PRESENT }))
                "
                .into()
            )
            .unwrap()
            .1,
            vec![Constraint::Subtype(ElementSetSpecs {
                set: ElementOrSetOperation::SetOperation(SetOperation {
                    base: SubtypeElements::MultipleTypeConstraints(InnerTypeConstraint {
                        is_partial: true,
                        constraints: vec![
                            NamedConstraint {
                                identifier: "laneId".into(),
                                constraints: vec![],
                                presence: ComponentPresence::Present
                            },
                            NamedConstraint {
                                identifier: "connectionId".into(),
                                constraints: vec![],
                                presence: ComponentPresence::Absent
                            }
                        ]
                    }),
                    operator: SetOperator::Union,
                    operant: Box::new(ElementOrSetOperation::Element(
                        SubtypeElements::MultipleTypeConstraints(InnerTypeConstraint {
                            is_partial: true,
                            constraints: vec![
                                NamedConstraint {
                                    identifier: "laneId".into(),
                                    constraints: vec![],
                                    presence: ComponentPresence::Absent
                                },
                                NamedConstraint {
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
            constraints(
                "(0..3|5..8|10)
                "
                .into()
            )
            .unwrap()
            .1,
            vec![Constraint::Subtype(ElementSetSpecs {
                set: ElementOrSetOperation::SetOperation(SetOperation {
                    base: SubtypeElements::ValueRange {
                        min: Some(ASN1Value::Integer(0)),
                        max: Some(ASN1Value::Integer(3)),
                        extensible: false
                    },
                    operator: SetOperator::Union,
                    operant: Box::new(ElementOrSetOperation::SetOperation(SetOperation {
                        base: SubtypeElements::ValueRange {
                            min: Some(ASN1Value::Integer(5)),
                            max: Some(ASN1Value::Integer(8)),
                            extensible: false
                        },
                        operator: SetOperator::Union,
                        operant: Box::new(ElementOrSetOperation::Element(
                            SubtypeElements::SingleValue {
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
            constraints(
                "(unknown   | passengerCar..tram
                  | agricultural)"
                    .into()
            )
            .unwrap()
            .1,
            vec![Constraint::Subtype(ElementSetSpecs {
                set: ElementOrSetOperation::SetOperation(SetOperation {
                    base: SubtypeElements::SingleValue {
                        value: ASN1Value::ElsewhereDeclaredValue {
                            identifier: "unknown".into(),
                            parent: None
                        },
                        extensible: false
                    },
                    operator: SetOperator::Union,
                    operant: Box::new(ElementOrSetOperation::SetOperation(SetOperation {
                        base: SubtypeElements::ValueRange {
                            min: Some(ASN1Value::ElsewhereDeclaredValue {
                                identifier: "passengerCar".into(),
                                parent: None
                            }),
                            max: Some(ASN1Value::ElsewhereDeclaredValue {
                                identifier: "tram".into(),
                                parent: None
                            }),
                            extensible: false
                        },
                        operator: SetOperator::Union,
                        operant: Box::new(ElementOrSetOperation::Element(
                            SubtypeElements::SingleValue {
                                value: ASN1Value::ElsewhereDeclaredValue {
                                    identifier: "agricultural".into(),
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
            constraints(
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
            vec![Constraint::Table(TableConstraint {
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
                                    constraints: vec![Constraint::Subtype(ElementSetSpecs {
                                        set: ElementOrSetOperation::Element(
                                            SubtypeElements::ValueRange {
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
            SubtypeElements::ValueRange {
                min: Some(ASN1Value::String("a".into())),
                max: Some(ASN1Value::String("z".into())),
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
            SubtypeElements::PermittedAlphabet(Box::new(ElementOrSetOperation::SetOperation(
                SetOperation {
                    base: SubtypeElements::ValueRange {
                        min: Some(ASN1Value::String("a".into())),
                        max: Some(ASN1Value::String("z".into())),
                        extensible: false
                    },
                    operator: SetOperator::Union,
                    operant: Box::new(ElementOrSetOperation::SetOperation(SetOperation {
                        base: SubtypeElements::ValueRange {
                            min: Some(ASN1Value::String("A".into())),
                            max: Some(ASN1Value::String("Z".into())),
                            extensible: false
                        },
                        operator: SetOperator::Union,
                        operant: Box::new(ElementOrSetOperation::SetOperation(SetOperation {
                            base: SubtypeElements::ValueRange {
                                min: Some(ASN1Value::String("0".into())),
                                max: Some(ASN1Value::String("9".into())),
                                extensible: false
                            },
                            operator: SetOperator::Union,
                            operant: Box::new(ElementOrSetOperation::Element(
                                SubtypeElements::SingleValue {
                                    value: ASN1Value::String(".-".into()),
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
            constraints(r#"(FROM ("a".."z" | "A".."Z" | "0".."9" | ".-")) (SIZE (1..255))"#.into())
                .unwrap()
                .1,
            vec![
                Constraint::Subtype(ElementSetSpecs {
                    set: ElementOrSetOperation::Element(SubtypeElements::PermittedAlphabet(
                        Box::new(ElementOrSetOperation::SetOperation(SetOperation {
                            base: SubtypeElements::ValueRange {
                                min: Some(ASN1Value::String("a".into())),
                                max: Some(ASN1Value::String("z".into())),
                                extensible: false
                            },
                            operator: SetOperator::Union,
                            operant: Box::new(ElementOrSetOperation::SetOperation(SetOperation {
                                base: SubtypeElements::ValueRange {
                                    min: Some(ASN1Value::String("A".into())),
                                    max: Some(ASN1Value::String("Z".into())),
                                    extensible: false
                                },
                                operator: SetOperator::Union,
                                operant: Box::new(ElementOrSetOperation::SetOperation(
                                    SetOperation {
                                        base: SubtypeElements::ValueRange {
                                            min: Some(ASN1Value::String("0".into())),
                                            max: Some(ASN1Value::String("9".into())),
                                            extensible: false
                                        },
                                        operator: SetOperator::Union,
                                        operant: Box::new(ElementOrSetOperation::Element(
                                            SubtypeElements::SingleValue {
                                                value: ASN1Value::String(".-".into()),
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
                Constraint::Subtype(ElementSetSpecs {
                    set: ElementOrSetOperation::Element(SubtypeElements::SizeConstraint(Box::new(
                        ElementOrSetOperation::Element(SubtypeElements::ValueRange {
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
            constraints(
                r#"(WITH COMPONENTS {
                mantissa (-16777215..16777215),
                base (2),
                exponent (-125..128) } )"#
                    .into()
            )
            .unwrap()
            .1,
            vec![Constraint::Subtype(ElementSetSpecs {
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
        )
    }

    #[test]
    fn parses_pattern_constraint() {
        assert_eq!(
            constraints(r#"(PATTERN "[a-zA-Z]#(1,8)(-[a-zA-Z0-9]#(1,8))*")"#.into())
                .unwrap()
                .1,
            vec![Constraint::Subtype(ElementSetSpecs {
                set: ElementOrSetOperation::Element(SubtypeElements::PatternConstraint(
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
            constraints(
                r#"(CONSTRAINED BY {/* XML representation of the XSD pattern "\d\d\d\d-\d\d-\d\dT\d\d:\d\d:\d\d[-,+]\d\d:\d\d" */})"#.into()
            ).unwrap().1,
            vec![
                Constraint::Subtype(
                    ElementSetSpecs {
                        set: ElementOrSetOperation::Element(
                            SubtypeElements::UserDefinedConstraint(
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
            constraints("(SIZE(1..4),...)".into()).unwrap().1,
            vec![Constraint::Subtype(ElementSetSpecs {
                set: ElementOrSetOperation::Element(SubtypeElements::SizeConstraint(Box::new(
                    ElementOrSetOperation::Element(SubtypeElements::ValueRange {
                        min: Some(ASN1Value::Integer(1)),
                        max: Some(ASN1Value::Integer(4)),
                        extensible: false
                    })
                ))),
                extensible: true
            })]
        );
        assert_eq!(
            constraints("(SIZE(1..4,...))".into()).unwrap().1,
            vec![Constraint::Subtype(ElementSetSpecs {
                set: ElementOrSetOperation::Element(SubtypeElements::SizeConstraint(Box::new(
                    ElementOrSetOperation::Element(SubtypeElements::ValueRange {
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
            constraints(r#"(SETTINGS "Midnight=Start")"#.into())
                .unwrap()
                .1,
            vec![Constraint::Subtype(ElementSetSpecs {
                set: ElementOrSetOperation::Element(SubtypeElements::PropertySettings(
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
            constraints(r#"(1..65535, ..., 65536..109999)"#.into())
                .unwrap()
                .1,
            vec![Constraint::Subtype(ElementSetSpecs {
                set: ElementOrSetOperation::Element(SubtypeElements::ValueRange {
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
            vec![Constraint::Subtype(ElementSetSpecs {
                set: ElementOrSetOperation::SetOperation(SetOperation {
                    base: SubtypeElements::SingleTypeConstraint(vec![Constraint::Subtype(
                        ElementSetSpecs {
                            set: ElementOrSetOperation::Element(
                                SubtypeElements::ContainedSubtype {
                                    subtype: ASN1Type::ElsewhereDeclaredType(
                                        DeclarationElsewhere {
                                            parent: None,
                                            identifier: "EtsiTs103097Certificate".into(),
                                            constraints: vec![],
                                        }
                                    ),
                                    extensible: false,
                                }
                            ),
                            extensible: false,
                        }
                    ),]),
                    operator: SetOperator::Intersection,
                    operant: Box::new(ElementOrSetOperation::Element(
                        SubtypeElements::SizeConstraint(Box::new(ElementOrSetOperation::Element(
                            SubtypeElements::SingleValue {
                                value: ASN1Value::Integer(1,),
                                extensible: false,
                            },
                        ))),
                    ))
                }),
                extensible: false,
            })],
            constraints(r#"((WITH COMPONENT (EtsiTs103097Certificate))^(SIZE(1)))"#.into())
                .unwrap()
                .1
        )
    }
}
