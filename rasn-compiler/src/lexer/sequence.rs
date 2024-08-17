use nom::{
    bytes::complete::tag,
    character::complete::{char, i128},
    combinator::{into, opt, recognize},
    multi::{many0, separated_list0, separated_list1},
    sequence::{terminated, tuple},
};

use crate::intermediate::{types::*, *};

use super::{common::optional_comma, constraint::constraint, *};

pub fn sequence_value(input: Span) -> LexerResult<ASN1Value> {
    map(
        in_braces(separated_list0(
            skip_ws_and_comments(char(',')),
            skip_ws_and_comments(pair(
                opt(value_identifier),
                skip_ws_and_comments(asn1_value),
            )),
        )),
        |fields| {
            ASN1Value::SequenceOrSet(
                fields
                    .into_iter()
                    .map(|(id, val)| (id.map(|span| span.to_string()), Box::new(val)))
                    .collect(),
            )
        },
    )(input)
}

/// Tries to parse an ASN1 SEQUENCE
///
/// *`input` - string slice to be matched against
///
/// `sequence` will try to match an SEQUENCE declaration in the `input` string.
/// If the match succeeds, the lexer will consume the match and return the remaining string
/// and a wrapped `Sequence` value representing the ASN1 declaration. If the defined SEQUENCE
/// contains anonymous SEQUENCEs as members, these nested SEQUENCEs will be represented as
/// structs within the same global scope.
/// If the match fails, the lexer will not consume the input and will return an error.
pub fn sequence(input: Span) -> LexerResult<ASN1Type> {
    map(
        preceded(
            skip_ws_and_comments(tag(SEQUENCE)),
            pair(
                in_braces(tuple((
                    many0(terminated(
                        skip_ws_and_comments(sequence_component),
                        optional_comma,
                    )),
                    opt(terminated(extension_marker, opt(char(COMMA)))),
                    opt(many0(terminated(
                        skip_ws_and_comments(alt((extension_group, sequence_component))),
                        optional_comma,
                    ))),
                ))),
                opt(constraint),
            ),
        ),
        |m| ASN1Type::Sequence(m.into()),
    )(input)
}

fn extension_group(input: Span) -> LexerResult<SequenceComponent> {
    map(
        in_version_brackets(preceded(
            opt(pair(
                skip_ws_and_comments(i128),
                skip_ws_and_comments(char(':')),
            )),
            skip_ws_and_comments(many1(terminated(
                skip_ws_and_comments(sequence_component),
                optional_comma,
            ))),
        )),
        |ext_group| {
            let mut components_of = vec![];
            let mut members = vec![];
            for comp in ext_group {
                match comp {
                    SequenceComponent::Member(m) => members.push(m),
                    SequenceComponent::ComponentsOf(c) => components_of.push(c),
                }
            }
            SequenceComponent::Member(SequenceOrSetMember {
                name: String::from("ext_group_") + &members.first().unwrap().name,
                tag: None,
                ty: ASN1Type::Sequence(SequenceOrSet {
                    components_of,
                    extensible: None,
                    constraints: vec![],
                    members,
                }),
                default_value: None,
                is_optional: false,
                constraints: vec![],
            })
        },
    )(input)
}

pub fn sequence_component(input: Span) -> LexerResult<SequenceComponent> {
    skip_ws_and_comments(alt((
        map(
            preceded(
                tag(COMPONENTS_OF),
                skip_ws_and_comments(alt((
                    recognize(separated_list1(tag(".&"), identifier)),
                    title_case_identifier,
                ))),
            ),
            |id| SequenceComponent::ComponentsOf(id.to_string()),
        ),
        map(sequence_or_set_member, SequenceComponent::Member),
    )))(input)
}

pub fn sequence_or_set_member(input: Span) -> LexerResult<SequenceOrSetMember> {
    into(tuple((
        skip_ws_and_comments(identifier),
        opt(asn_tag),
        skip_ws_and_comments(asn1_type),
        opt(constraint),
        optional_marker,
        default,
    )))(input)
}

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::intermediate::constraints::*;

    use super::*;

    #[test]
    fn parses_optional_marker() {
        assert_eq!(
            optional_marker(Span::new("\n\tOPTIONAL")).unwrap().1,
            Some(OptionalMarker())
        );
        assert_eq!(optional_marker(Span::new("DEFAULT")).unwrap().1, None);
    }

    #[test]
    fn parses_default_int() {
        assert_eq!(
            default(Span::new("\n\tDEFAULT\t-1")).unwrap().1,
            Some(ASN1Value::Integer(-1))
        );
    }

    #[test]
    fn parses_default_boolean() {
        assert_eq!(
            default(Span::new("  DEFAULT   TRUE")).unwrap().1,
            Some(ASN1Value::Boolean(true))
        );
    }

    #[test]
    fn parses_default_bitstring() {
        assert_eq!(
            default(Span::new("  DEFAULT '001010011'B")).unwrap().1,
            Some(ASN1Value::BitString(vec![
                false, false, true, false, true, false, false, true, true
            ]))
        );
        assert_eq!(
            default(Span::new("DEFAULT 'F60E'H")).unwrap().1,
            Some(ASN1Value::BitString(vec![
                true, true, true, true, false, true, true, false, false, false, false, false, true,
                true, true, false
            ]))
        );
    }

    #[test]
    fn parses_default_enumeral() {
        assert_eq!(
            default(Span::new("  DEFAULT enumeral1")).unwrap().1,
            Some(ASN1Value::ElsewhereDeclaredValue {
                identifier: "enumeral1".into(),
                parent: None
            })
        );
        assert_eq!(
            default(Span::new("DEFAULT enumeral1")).unwrap().1,
            Some(ASN1Value::ElsewhereDeclaredValue {
                identifier: "enumeral1".into(),
                parent: None
            })
        );
    }

    #[test]
    fn parses_subtyped_sequence() {
        assert_eq!(
        sequence(
            Span::new(r#"SEQUENCE {
              clusterBoundingBoxShape    Shape (WITH COMPONENTS{..., elliptical ABSENT, radial ABSENT, radialShapes ABSENT}) OPTIONAL,
              ...
           }"#)
        )
        .unwrap()
        .1,
        ASN1Type::Sequence(SequenceOrSet {
            components_of: vec![],extensible: Some(1),
            constraints: vec![],
            members: vec![
                SequenceOrSetMember {
                    name: "clusterBoundingBoxShape".into(),
                    tag: None,
                    ty: ASN1Type::ElsewhereDeclaredType(DeclarationElsewhere { parent: None,
                        identifier: "Shape".into(), constraints: vec![Constraint::SubtypeConstraint(ElementSet { set: ElementOrSetOperation::Element(SubtypeElement::SingleTypeConstraint(InnerTypeConstraint { is_partial: true, constraints: vec![ConstrainedComponent { identifier: "elliptical".into(), constraints: vec![], presence: ComponentPresence::Absent },ConstrainedComponent { identifier: "radial".into(), constraints: vec![], presence: ComponentPresence::Absent },ConstrainedComponent { identifier: "radialShapes".into(), constraints: vec![], presence: ComponentPresence::Absent }] })), extensible: false })
                     ]}),
                    default_value: None,
                    is_optional: true,
                    constraints: vec![],
                }
            ]
        })
    )
    }

    #[test]
    fn parses_simple_sequence() {
        assert_eq!(
            sequence(Span::new(
                r#"SEQUENCE {
        value         AccelerationValue,
        confidence    AccelerationConfidence
    }"#
            ))
            .unwrap()
            .1,
            ASN1Type::Sequence(SequenceOrSet {
                components_of: vec![],
                extensible: None,
                constraints: vec![],
                members: vec![
                    SequenceOrSetMember {
                        name: "value".into(),

                        tag: None,
                        ty: ASN1Type::ElsewhereDeclaredType(DeclarationElsewhere {
                            parent: None,
                            identifier: "AccelerationValue".into(),
                            constraints: vec![]
                        }),
                        default_value: None,
                        is_optional: false,
                        constraints: vec![]
                    },
                    SequenceOrSetMember {
                        name: "confidence".into(),

                        tag: None,
                        ty: ASN1Type::ElsewhereDeclaredType(DeclarationElsewhere {
                            parent: None,
                            identifier: "AccelerationConfidence".into(),
                            constraints: vec![]
                        }),
                        default_value: None,
                        is_optional: false,
                        constraints: vec![],
                    }
                ]
            })
        )
    }

    #[test]
    fn parses_sequence_with_optionals() {
        assert_eq!(
            sequence(Span::new(
                r#"SEQUENCE{
                  xCoordinate    CartesianCoordinateWithConfidence,
                  --x
                  yCoordinate    CartesianCoordinateWithConfidence, -- y --
                  zCoordinate    CartesianCoordinateWithConfidence OPTIONAL -- this is optional
              }"#
            ))
            .unwrap()
            .1,
            ASN1Type::Sequence(SequenceOrSet {
                components_of: vec![],
                extensible: None,
                constraints: vec![],
                members: vec![
                    SequenceOrSetMember {
                        name: "xCoordinate".into(),

                        tag: None,
                        ty: ASN1Type::ElsewhereDeclaredType(DeclarationElsewhere {
                            parent: None,
                            identifier: "CartesianCoordinateWithConfidence".into(),
                            constraints: vec![]
                        }),
                        default_value: None,
                        is_optional: false,
                        constraints: vec![],
                    },
                    SequenceOrSetMember {
                        name: "yCoordinate".into(),
                        tag: None,
                        ty: ASN1Type::ElsewhereDeclaredType(DeclarationElsewhere {
                            parent: None,
                            identifier: "CartesianCoordinateWithConfidence".into(),
                            constraints: vec![]
                        }),
                        default_value: None,
                        is_optional: false,
                        constraints: vec![],
                    },
                    SequenceOrSetMember {
                        name: "zCoordinate".into(),
                        tag: None,
                        ty: ASN1Type::ElsewhereDeclaredType(DeclarationElsewhere {
                            parent: None,
                            identifier: "CartesianCoordinateWithConfidence".into(),
                            constraints: vec![]
                        }),
                        default_value: None,
                        is_optional: true,
                        constraints: vec![],
                    }
                ]
            })
        )
    }

    #[test]
    fn parses_extended_sequence_with_default() {
        assert_eq!(
            sequence(Span::new(
                r#"SEQUENCE {
                  horizontalPositionConfidence  PosConfidenceEllipse OPTIONAL,
                  deltaAltitude -- COMMENT --   DeltaAltitude DEFAULT unavailable,
                  altitudeConfidence            AltitudeConfidence DEFAULT unavailable,
                  -- Attention: Extension!
                  ...
                }"#
            ))
            .unwrap()
            .1,
            ASN1Type::Sequence(SequenceOrSet {
                components_of: vec![],
                extensible: Some(3),
                constraints: vec![],
                members: vec![
                    SequenceOrSetMember {
                        name: "horizontalPositionConfidence".into(),
                        tag: None,
                        ty: ASN1Type::ElsewhereDeclaredType(DeclarationElsewhere {
                            parent: None,
                            identifier: "PosConfidenceEllipse".into(),
                            constraints: vec![]
                        }),
                        default_value: None,
                        is_optional: true,
                        constraints: vec![],
                    },
                    SequenceOrSetMember {
                        name: "deltaAltitude".into(),
                        tag: None,
                        ty: ASN1Type::ElsewhereDeclaredType(DeclarationElsewhere {
                            parent: None,
                            identifier: "DeltaAltitude".into(),
                            constraints: vec![]
                        }),
                        default_value: Some(ASN1Value::ElsewhereDeclaredValue {
                            identifier: "unavailable".into(),
                            parent: None
                        }),
                        is_optional: true,
                        constraints: vec![],
                    },
                    SequenceOrSetMember {
                        name: "altitudeConfidence".into(),
                        tag: None,
                        ty: ASN1Type::ElsewhereDeclaredType(DeclarationElsewhere {
                            parent: None,
                            identifier: "AltitudeConfidence".into(),
                            constraints: vec![]
                        }),
                        default_value: Some(ASN1Value::ElsewhereDeclaredValue {
                            identifier: "unavailable".into(),
                            parent: None
                        }),
                        is_optional: true,
                        constraints: vec![],
                    }
                ]
            })
        )
    }

    #[test]
    fn parses_sequence_with_primitives() {
        assert_eq!(
            sequence(Span::new(
                r#"SEQUENCE {
                  unNumber                INTEGER (0..9999),
                  limitedQuantity         BOOLEAN DEFAULT FALSE,
                  emergencyActionCode     OCTET STRING (SIZE (1..24)) OPTIONAL,
                  ...
              }"#
            ))
            .unwrap()
            .1,
            ASN1Type::Sequence(SequenceOrSet {
                components_of: vec![],
                extensible: Some(3),
                constraints: vec![],
                members: vec![
                    SequenceOrSetMember {
                        name: "unNumber".into(),
                        tag: None,
                        ty: ASN1Type::Integer(Integer {
                            constraints: vec![Constraint::SubtypeConstraint(ElementSet {
                                set: ElementOrSetOperation::Element(SubtypeElement::ValueRange {
                                    min: Some(ASN1Value::Integer(0)),
                                    max: Some(ASN1Value::Integer(9999)),
                                    extensible: false
                                }),
                                extensible: false
                            })],
                            distinguished_values: None,
                        }),
                        default_value: None,
                        is_optional: false,
                        constraints: vec![],
                    },
                    SequenceOrSetMember {
                        name: "limitedQuantity".into(),
                        tag: None,
                        ty: ASN1Type::Boolean(Boolean {
                            constraints: vec![]
                        }),
                        default_value: Some(ASN1Value::Boolean(false)),
                        is_optional: true,
                        constraints: vec![],
                    },
                    SequenceOrSetMember {
                        name: "emergencyActionCode".into(),
                        tag: None,
                        ty: ASN1Type::OctetString(OctetString {
                            constraints: vec![Constraint::SubtypeConstraint(ElementSet {
                                set: ElementOrSetOperation::Element(
                                    SubtypeElement::SizeConstraint(Box::new(
                                        ElementOrSetOperation::Element(
                                            SubtypeElement::ValueRange {
                                                min: Some(ASN1Value::Integer(1)),
                                                max: Some(ASN1Value::Integer(24)),
                                                extensible: false
                                            }
                                        )
                                    ))
                                ),
                                extensible: false
                            })],
                        }),
                        default_value: None,
                        is_optional: true,
                        constraints: vec![],
                    }
                ]
            })
        )
    }

    #[test]
    fn parses_nested_sequence() {
        assert_eq!(
            sequence(Span::new(
                r#"SEQUENCE {
                  nested                SEQUENCE {
                    wow         Wow -- WOW!
                    this-is-annoying BOOLEAN DEFAULT TRUE,
                    another
                    SEQUENCE
                    {
                      inner BIT STRING (SIZE(1,...)) DEFAULT '0'B
                    } OPTIONAL,
                    ...
                  },
                  ...
              }"#
            ))
            .unwrap()
            .1,
            ASN1Type::Sequence(SequenceOrSet {
                components_of: vec![],
                extensible: Some(1),
                constraints: vec![],
                members: vec![SequenceOrSetMember {
                    name: "nested".into(),

                    tag: None,
                    ty: ASN1Type::Sequence(SequenceOrSet {
                        components_of: vec![],
                        extensible: Some(3),
                        constraints: vec![],
                        members: vec![
                            SequenceOrSetMember {
                                name: "wow".into(),

                                tag: None,
                                ty: ASN1Type::ElsewhereDeclaredType(DeclarationElsewhere {
                                    parent: None,
                                    identifier: "Wow".into(),
                                    constraints: vec![]
                                }),
                                default_value: None,
                                is_optional: false,
                                constraints: vec![],
                            },
                            SequenceOrSetMember {
                                name: "this-is-annoying".into(),

                                tag: None,
                                ty: ASN1Type::Boolean(Boolean {
                                    constraints: vec![]
                                }),
                                default_value: Some(ASN1Value::Boolean(true)),
                                is_optional: true,
                                constraints: vec![],
                            },
                            SequenceOrSetMember {
                                name: "another".into(),

                                tag: None,
                                ty: ASN1Type::Sequence(SequenceOrSet {
                                    components_of: vec![],
                                    extensible: None,
                                    constraints: vec![],
                                    members: vec![SequenceOrSetMember {
                                        name: "inner".into(),

                                        tag: None,
                                        ty: ASN1Type::BitString(BitString {
                                            constraints: vec![Constraint::SubtypeConstraint(
                                                ElementSet {
                                                    set: ElementOrSetOperation::Element(
                                                        SubtypeElement::SizeConstraint(Box::new(
                                                            ElementOrSetOperation::Element(
                                                                SubtypeElement::SingleValue {
                                                                    value: ASN1Value::Integer(1),
                                                                    extensible: true
                                                                }
                                                            )
                                                        ))
                                                    ),
                                                    extensible: false
                                                }
                                            )],
                                            distinguished_values: None
                                        }),
                                        default_value: Some(ASN1Value::BitString(vec![false])),
                                        is_optional: true,
                                        constraints: vec![],
                                    }]
                                }),
                                default_value: None,
                                is_optional: true,
                                constraints: vec![],
                            }
                        ]
                    }),
                    default_value: None,
                    is_optional: false,
                    constraints: vec![],
                }]
            })
        )
    }

    #[test]
    fn parses_sequence_value() {
        assert_eq!(
            sequence_value(Span::new("{itsaid content:0, ctx c-ctxRefNull}"))
                .unwrap()
                .1,
            ASN1Value::SequenceOrSet(vec![
                (
                    Some("itsaid".into()),
                    Box::new(ASN1Value::Choice {
                        type_name: None,
                        variant_name: "content".into(),
                        inner_value: Box::new(ASN1Value::Integer(0))
                    })
                ),
                (
                    Some("ctx".into()),
                    Box::new(ASN1Value::ElsewhereDeclaredValue {
                        identifier: "c-ctxRefNull".into(),
                        parent: None
                    })
                )
            ])
        )
    }

    #[test]
    fn parses_sequence_with_extension_group() {
        assert_eq!(
            sequence(Span::new(
                "SEQUENCE {item-code INTEGER (0..254),
                ...,
                [[ alternate-item-code INTEGER (0..254),
                    and-another BOOLEAN DEFAULT TRUE
                 ]] }"
            ))
            .unwrap()
            .1,
            ASN1Type::Sequence(SequenceOrSet {
                components_of: vec![],
                extensible: Some(1),
                constraints: vec![],
                members: vec![
                    SequenceOrSetMember {
                        name: "item-code".into(),
                        tag: None,
                        ty: ASN1Type::Integer(Integer {
                            constraints: vec![Constraint::SubtypeConstraint(ElementSet {
                                set: ElementOrSetOperation::Element(SubtypeElement::ValueRange {
                                    min: Some(ASN1Value::Integer(0)),
                                    max: Some(ASN1Value::Integer(254)),
                                    extensible: false
                                }),
                                extensible: false
                            })],
                            distinguished_values: None,
                        }),
                        default_value: None,
                        is_optional: false,
                        constraints: vec![]
                    },
                    SequenceOrSetMember {
                        name: "ext_group_alternate-item-code".into(),
                        tag: None,
                        ty: ASN1Type::Sequence(SequenceOrSet {
                            components_of: vec![],
                            extensible: None,
                            constraints: vec![],
                            members: vec![
                                SequenceOrSetMember {
                                    name: "alternate-item-code".into(),
                                    tag: None,
                                    ty: ASN1Type::Integer(Integer {
                                        constraints: vec![Constraint::SubtypeConstraint(
                                            ElementSet {
                                                set: ElementOrSetOperation::Element(
                                                    SubtypeElement::ValueRange {
                                                        min: Some(ASN1Value::Integer(0)),
                                                        max: Some(ASN1Value::Integer(254)),
                                                        extensible: false
                                                    }
                                                ),
                                                extensible: false
                                            }
                                        )],
                                        distinguished_values: None,
                                    }),
                                    default_value: None,
                                    is_optional: false,
                                    constraints: vec![]
                                },
                                SequenceOrSetMember {
                                    name: "and-another".into(),
                                    tag: None,
                                    ty: ASN1Type::Boolean(Boolean {
                                        constraints: vec![]
                                    }),
                                    default_value: Some(ASN1Value::Boolean(true)),
                                    is_optional: true,
                                    constraints: vec![]
                                }
                            ]
                        }),
                        default_value: None,
                        is_optional: false,
                        constraints: vec![]
                    }
                ]
            })
        )
    }

    #[test]
    fn parses_sequence_with_components_of_notation() {
        assert_eq!(
            sequence(Span::new(
                r#"SEQUENCE {
            COMPONENTS OF TypeA,
            bilateral-information TypeB
          }"#
            ))
            .unwrap()
            .1,
            ASN1Type::Sequence(SequenceOrSet {
                components_of: vec!["TypeA".into()],
                extensible: None,
                constraints: vec![],
                members: vec![SequenceOrSetMember {
                    name: "bilateral-information".into(),
                    tag: None,
                    ty: ASN1Type::ElsewhereDeclaredType(DeclarationElsewhere {
                        parent: None,
                        identifier: "TypeB".into(),
                        constraints: vec![]
                    }),
                    default_value: None,
                    is_optional: false,
                    constraints: vec![]
                }]
            })
        )
    }

    #[test]
    fn parse_x284() {
        println!(
            "{:?}",
            sequence(Span::new(
                r#"SEQUENCE --(GRJ)
        {
requestSeqNum           RequestSeqNum,
protocolIdentifier      ProtocolIdentifier,
nonStandardData         NonStandardParameter OPTIONAL,
gatekeeperIdentifier    GatekeeperIdentifier OPTIONAL,
rejectReason            GatekeeperRejectReason,
...,
altGKInfo               AltGKInfo OPTIONAL,
tokens                  SEQUENCE OF ClearToken OPTIONAL,
cryptoTokens            SEQUENCE OF CryptoH323Token OPTIONAL,
integrityCheckValue     ICV OPTIONAL
}"#
            ))
        )
    }
}
