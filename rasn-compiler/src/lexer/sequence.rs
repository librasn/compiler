use nom::{
    bytes::complete::tag,
    character::complete::{char, i128},
    combinator::{into, opt, recognize},
    multi::{many0, separated_list0, separated_list1},
    sequence::terminated,
};

use crate::{
    common::INTERNAL_EXTENSION_GROUP_NAME_PREFIX,
    intermediate::{types::*, *},
};

use super::{common::optional_comma, constraint::constraints, *};

pub fn sequence_value(input: Input<'_>) -> ParserResult<'_, ASN1Value> {
    map(
        in_braces(separated_list0(
            skip_ws_and_comments(char(',')),
            skip_ws_and_comments(alt((
                pair(opt(value_reference), skip_ws_and_comments(asn1_value)),
                map(skip_ws_and_comments(asn1_value), |v| (None, v)),
            ))),
        )),
        |fields| {
            ASN1Value::SequenceOrSet(
                fields
                    .into_iter()
                    .map(|(id, val)| (id.map(|str| str.to_owned()), Box::new(val)))
                    .collect(),
            )
        },
    )
    .parse(input)
}

/// Tries to parse an ASN1 SEQUENCE
///
/// *`input` - [Input]-wrapped string slice to be matched against
///
/// `sequence` will try to match an SEQUENCE declaration in the `input` string.
/// If the match succeeds, the lexer will consume the match and return the remaining string
/// and a wrapped `Sequence` value representing the ASN1 declaration. If the defined SEQUENCE
/// contains anonymous SEQUENCEs as members, these nested SEQUENCEs will be represented as
/// structs within the same global scope.
/// If the match fails, the lexer will not consume the input and will return an error.
pub fn sequence(input: Input<'_>) -> ParserResult<'_, ASN1Type> {
    map(
        preceded(
            skip_ws_and_comments(tag(SEQUENCE)),
            pair(
                in_braces((
                    many0(terminated(
                        skip_ws_and_comments(sequence_component),
                        optional_comma,
                    )),
                    opt(terminated(extension_marker, opt(char(COMMA)))),
                    opt(many0(terminated(
                        skip_ws_and_comments(alt((extension_group, sequence_component))),
                        optional_comma,
                    ))),
                )),
                opt(constraints),
            ),
        ),
        |m| ASN1Type::Sequence(m.into()),
    )
    .parse(input)
}

fn extension_group(input: Input<'_>) -> ParserResult<'_, SequenceComponent> {
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
                is_recursive: false,
                name: String::from(INTERNAL_EXTENSION_GROUP_NAME_PREFIX)
                    + &members.first().unwrap().name,
                tag: None,
                ty: ASN1Type::Sequence(SequenceOrSet {
                    components_of,
                    extensible: None,
                    constraints: vec![],
                    members,
                }),
                optionality: Optionality::Required,
                constraints: vec![],
            })
        },
    )
    .parse(input)
}

pub fn sequence_component(input: Input<'_>) -> ParserResult<'_, SequenceComponent> {
    skip_ws_and_comments(alt((
        map(
            preceded(
                tag(COMPONENTS_OF),
                skip_ws_and_comments(alt((
                    into_inner(recognize(separated_list1(tag(".&"), identifier))),
                    type_reference,
                ))),
            ),
            |id| SequenceComponent::ComponentsOf(id.into()),
        ),
        map(sequence_or_set_member, SequenceComponent::Member),
    )))
    .parse(input)
}

pub fn sequence_or_set_member(input: Input<'_>) -> ParserResult<'_, SequenceOrSetMember> {
    into((
        skip_ws_and_comments(identifier),
        opt(asn_tag),
        skip_ws_and_comments(asn1_type),
        opt(constraints),
        skip_ws_and_comments(optionality(asn1_value)),
    ))
    .parse(input)
}

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::intermediate::constraints::*;

    use super::*;

    #[test]
    fn parses_subtyped_sequence() {
        assert_eq!(
        sequence(
            r#"SEQUENCE {
              clusterBoundingBoxShape    Shape (WITH COMPONENTS{..., elliptical ABSENT, radial ABSENT, radialShapes ABSENT}) OPTIONAL,
              ...
           }"#.into()
        )
        .unwrap()
        .1,
        ASN1Type::Sequence(SequenceOrSet {
            components_of: vec![],extensible: Some(1),
            constraints: vec![],
            members: vec![
                SequenceOrSetMember {
is_recursive: false,
                    name: "clusterBoundingBoxShape".into(),
                    tag: None,
                    ty: ASN1Type::ElsewhereDeclaredType(DeclarationElsewhere { parent: None,
                        identifier: "Shape".into(), constraints: vec![Constraint::Subtype(ElementSetSpecs { set: ElementOrSetOperation::Element(SubtypeElements::MultipleTypeConstraints(InnerTypeConstraint { is_partial: true, constraints: vec![NamedConstraint { identifier: "elliptical".into(), constraints: vec![], presence: ComponentPresence::Absent },NamedConstraint { identifier: "radial".into(), constraints: vec![], presence: ComponentPresence::Absent },NamedConstraint { identifier: "radialShapes".into(), constraints: vec![], presence: ComponentPresence::Absent }] })), extensible: false })
                     ]}),
                    optionality: Optionality::Optional,
                    constraints: vec![],
                }
            ]
        })
    )
    }

    #[test]
    fn parses_simple_sequence() {
        assert_eq!(
            sequence(
                r#"SEQUENCE {
        value         AccelerationValue,
        confidence    AccelerationConfidence
    }"#
                .into()
            )
            .unwrap()
            .1,
            ASN1Type::Sequence(SequenceOrSet {
                components_of: vec![],
                extensible: None,
                constraints: vec![],
                members: vec![
                    SequenceOrSetMember {
                        is_recursive: false,
                        name: "value".into(),
                        tag: None,
                        ty: ASN1Type::ElsewhereDeclaredType(DeclarationElsewhere {
                            parent: None,
                            identifier: "AccelerationValue".into(),
                            constraints: vec![]
                        }),
                        optionality: Optionality::Required,
                        constraints: vec![]
                    },
                    SequenceOrSetMember {
                        is_recursive: false,
                        name: "confidence".into(),

                        tag: None,
                        ty: ASN1Type::ElsewhereDeclaredType(DeclarationElsewhere {
                            parent: None,
                            identifier: "AccelerationConfidence".into(),
                            constraints: vec![]
                        }),
                        optionality: Optionality::Required,
                        constraints: vec![],
                    }
                ]
            })
        )
    }

    #[test]
    fn parses_sequence_with_optionals() {
        assert_eq!(
            sequence(
                r#"SEQUENCE{
                  xCoordinate    CartesianCoordinateWithConfidence,
                  --x
                  yCoordinate    CartesianCoordinateWithConfidence, -- y --
                  zCoordinate    CartesianCoordinateWithConfidence OPTIONAL -- this is optional
              }"#
                .into()
            )
            .unwrap()
            .1,
            ASN1Type::Sequence(SequenceOrSet {
                components_of: vec![],
                extensible: None,
                constraints: vec![],
                members: vec![
                    SequenceOrSetMember {
                        is_recursive: false,
                        name: "xCoordinate".into(),

                        tag: None,
                        ty: ASN1Type::ElsewhereDeclaredType(DeclarationElsewhere {
                            parent: None,
                            identifier: "CartesianCoordinateWithConfidence".into(),
                            constraints: vec![]
                        }),
                        optionality: Optionality::Required,
                        constraints: vec![],
                    },
                    SequenceOrSetMember {
                        is_recursive: false,
                        name: "yCoordinate".into(),
                        tag: None,
                        ty: ASN1Type::ElsewhereDeclaredType(DeclarationElsewhere {
                            parent: None,
                            identifier: "CartesianCoordinateWithConfidence".into(),
                            constraints: vec![]
                        }),
                        optionality: Optionality::Required,
                        constraints: vec![],
                    },
                    SequenceOrSetMember {
                        is_recursive: false,
                        name: "zCoordinate".into(),
                        tag: None,
                        ty: ASN1Type::ElsewhereDeclaredType(DeclarationElsewhere {
                            parent: None,
                            identifier: "CartesianCoordinateWithConfidence".into(),
                            constraints: vec![]
                        }),
                        optionality: Optionality::Optional,
                        constraints: vec![],
                    }
                ]
            })
        )
    }

    #[test]
    fn parses_extended_sequence_with_default() {
        assert_eq!(
            sequence(
                r#"SEQUENCE {
                  horizontalPositionConfidence  PosConfidenceEllipse OPTIONAL,
                  deltaAltitude -- COMMENT --   DeltaAltitude DEFAULT unavailable,
                  altitudeConfidence            AltitudeConfidence DEFAULT unavailable,
                  -- Attention: Extension!
                  ...
                }"#
                .into()
            )
            .unwrap()
            .1,
            ASN1Type::Sequence(SequenceOrSet {
                components_of: vec![],
                extensible: Some(3),
                constraints: vec![],
                members: vec![
                    SequenceOrSetMember {
                        is_recursive: false,
                        name: "horizontalPositionConfidence".into(),
                        tag: None,
                        ty: ASN1Type::ElsewhereDeclaredType(DeclarationElsewhere {
                            parent: None,
                            identifier: "PosConfidenceEllipse".into(),
                            constraints: vec![]
                        }),
                        optionality: Optionality::Optional,
                        constraints: vec![],
                    },
                    SequenceOrSetMember {
                        is_recursive: false,
                        name: "deltaAltitude".into(),
                        tag: None,
                        ty: ASN1Type::ElsewhereDeclaredType(DeclarationElsewhere {
                            parent: None,
                            identifier: "DeltaAltitude".into(),
                            constraints: vec![]
                        }),
                        optionality: Optionality::Default(ASN1Value::ElsewhereDeclaredValue {
                            identifier: "unavailable".into(),
                            parent: None
                        }),
                        constraints: vec![],
                    },
                    SequenceOrSetMember {
                        is_recursive: false,
                        name: "altitudeConfidence".into(),
                        tag: None,
                        ty: ASN1Type::ElsewhereDeclaredType(DeclarationElsewhere {
                            parent: None,
                            identifier: "AltitudeConfidence".into(),
                            constraints: vec![]
                        }),
                        optionality: Optionality::Default(ASN1Value::ElsewhereDeclaredValue {
                            identifier: "unavailable".into(),
                            parent: None
                        }),
                        constraints: vec![],
                    }
                ]
            })
        )
    }

    #[test]
    fn parses_sequence_with_primitives() {
        assert_eq!(
            sequence(
                r#"SEQUENCE {
                  unNumber                INTEGER (0..9999),
                  limitedQuantity         BOOLEAN DEFAULT FALSE,
                  emergencyActionCode     OCTET STRING (SIZE (1..24)) OPTIONAL,
                  ...
              }"#
                .into()
            )
            .unwrap()
            .1,
            ASN1Type::Sequence(SequenceOrSet {
                components_of: vec![],
                extensible: Some(3),
                constraints: vec![],
                members: vec![
                    SequenceOrSetMember {
                        is_recursive: false,
                        name: "unNumber".into(),
                        tag: None,
                        ty: ASN1Type::Integer(Integer {
                            constraints: vec![Constraint::Subtype(ElementSetSpecs {
                                set: ElementOrSetOperation::Element(SubtypeElements::ValueRange {
                                    min: Some(ASN1Value::Integer(0)),
                                    max: Some(ASN1Value::Integer(9999)),
                                    extensible: false
                                }),
                                extensible: false
                            })],
                            distinguished_values: None,
                        }),
                        optionality: Optionality::Required,
                        constraints: vec![],
                    },
                    SequenceOrSetMember {
                        is_recursive: false,
                        name: "limitedQuantity".into(),
                        tag: None,
                        ty: ASN1Type::Boolean(Boolean {
                            constraints: vec![]
                        }),
                        optionality: Optionality::Default(ASN1Value::Boolean(false)),
                        constraints: vec![],
                    },
                    SequenceOrSetMember {
                        is_recursive: false,
                        name: "emergencyActionCode".into(),
                        tag: None,
                        ty: ASN1Type::OctetString(OctetString {
                            constraints: vec![Constraint::Subtype(ElementSetSpecs {
                                set: ElementOrSetOperation::Element(
                                    SubtypeElements::SizeConstraint(Box::new(
                                        ElementOrSetOperation::Element(
                                            SubtypeElements::ValueRange {
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
                        optionality: Optionality::Optional,
                        constraints: vec![],
                    }
                ]
            })
        )
    }

    #[test]
    fn parses_nested_sequence() {
        assert_eq!(
            sequence(
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
                .into()
            )
            .unwrap()
            .1,
            ASN1Type::Sequence(SequenceOrSet {
                components_of: vec![],
                extensible: Some(1),
                constraints: vec![],
                members: vec![SequenceOrSetMember {
                    is_recursive: false,
                    name: "nested".into(),

                    tag: None,
                    ty: ASN1Type::Sequence(SequenceOrSet {
                        components_of: vec![],
                        extensible: Some(3),
                        constraints: vec![],
                        members: vec![
                            SequenceOrSetMember {
                                is_recursive: false,
                                name: "wow".into(),

                                tag: None,
                                ty: ASN1Type::ElsewhereDeclaredType(DeclarationElsewhere {
                                    parent: None,
                                    identifier: "Wow".into(),
                                    constraints: vec![]
                                }),
                                optionality: Optionality::Required,
                                constraints: vec![],
                            },
                            SequenceOrSetMember {
                                is_recursive: false,
                                name: "this-is-annoying".into(),

                                tag: None,
                                ty: ASN1Type::Boolean(Boolean {
                                    constraints: vec![]
                                }),
                                optionality: Optionality::Default(ASN1Value::Boolean(true)),
                                constraints: vec![],
                            },
                            SequenceOrSetMember {
                                is_recursive: false,
                                name: "another".into(),

                                tag: None,
                                ty: ASN1Type::Sequence(SequenceOrSet {
                                    components_of: vec![],
                                    extensible: None,
                                    constraints: vec![],
                                    members: vec![SequenceOrSetMember {
                                        is_recursive: false,
                                        name: "inner".into(),

                                        tag: None,
                                        ty: ASN1Type::BitString(BitString {
                                            constraints: vec![Constraint::Subtype(
                                                ElementSetSpecs {
                                                    set: ElementOrSetOperation::Element(
                                                        SubtypeElements::SizeConstraint(Box::new(
                                                            ElementOrSetOperation::Element(
                                                                SubtypeElements::SingleValue {
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
                                        optionality: Optionality::Default(ASN1Value::BitString(
                                            vec![false]
                                        )),
                                        constraints: vec![],
                                    }]
                                }),
                                optionality: Optionality::Optional,
                                constraints: vec![],
                            }
                        ]
                    }),
                    optionality: Optionality::Required,
                    constraints: vec![],
                }]
            })
        )
    }

    #[test]
    fn parses_sequence_value() {
        assert_eq!(
            sequence_value("{itsaid content:0, ctx c-ctxRefNull}".into())
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
            sequence(
                "SEQUENCE {item-code INTEGER (0..254),
                ...,
                [[ alternate-item-code INTEGER (0..254),
                    and-another BOOLEAN DEFAULT TRUE
                 ]] }"
                    .into()
            )
            .unwrap()
            .1,
            ASN1Type::Sequence(SequenceOrSet {
                components_of: vec![],
                extensible: Some(1),
                constraints: vec![],
                members: vec![
                    SequenceOrSetMember {
                        is_recursive: false,
                        name: "item-code".into(),
                        tag: None,
                        ty: ASN1Type::Integer(Integer {
                            constraints: vec![Constraint::Subtype(ElementSetSpecs {
                                set: ElementOrSetOperation::Element(SubtypeElements::ValueRange {
                                    min: Some(ASN1Value::Integer(0)),
                                    max: Some(ASN1Value::Integer(254)),
                                    extensible: false
                                }),
                                extensible: false
                            })],
                            distinguished_values: None,
                        }),
                        optionality: Optionality::Required,
                        constraints: vec![]
                    },
                    SequenceOrSetMember {
                        is_recursive: false,
                        name: "ext_group_alternate-item-code".into(),
                        tag: None,
                        ty: ASN1Type::Sequence(SequenceOrSet {
                            components_of: vec![],
                            extensible: None,
                            constraints: vec![],
                            members: vec![
                                SequenceOrSetMember {
                                    is_recursive: false,
                                    name: "alternate-item-code".into(),
                                    tag: None,
                                    ty: ASN1Type::Integer(Integer {
                                        constraints: vec![Constraint::Subtype(ElementSetSpecs {
                                            set: ElementOrSetOperation::Element(
                                                SubtypeElements::ValueRange {
                                                    min: Some(ASN1Value::Integer(0)),
                                                    max: Some(ASN1Value::Integer(254)),
                                                    extensible: false
                                                }
                                            ),
                                            extensible: false
                                        })],
                                        distinguished_values: None,
                                    }),
                                    optionality: Optionality::Required,
                                    constraints: vec![]
                                },
                                SequenceOrSetMember {
                                    is_recursive: false,
                                    name: "and-another".into(),
                                    tag: None,
                                    ty: ASN1Type::Boolean(Boolean {
                                        constraints: vec![]
                                    }),
                                    optionality: Optionality::Default(ASN1Value::Boolean(true)),
                                    constraints: vec![]
                                }
                            ]
                        }),
                        optionality: Optionality::Required,
                        constraints: vec![]
                    }
                ]
            })
        )
    }

    #[test]
    fn parses_sequence_with_components_of_notation() {
        assert_eq!(
            sequence(
                r#"SEQUENCE {
            COMPONENTS OF TypeA,
            bilateral-information TypeB
          }"#
                .into()
            )
            .unwrap()
            .1,
            ASN1Type::Sequence(SequenceOrSet {
                components_of: vec!["TypeA".into()],
                extensible: None,
                constraints: vec![],
                members: vec![SequenceOrSetMember {
                    is_recursive: false,
                    name: "bilateral-information".into(),
                    tag: None,
                    ty: ASN1Type::ElsewhereDeclaredType(DeclarationElsewhere {
                        parent: None,
                        identifier: "TypeB".into(),
                        constraints: vec![]
                    }),
                    optionality: Optionality::Required,
                    constraints: vec![]
                }]
            })
        )
    }

    #[test]
    fn parse_x284() {
        assert!(sequence(
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
            .into()
        )
        .is_ok())
    }

    #[test]
    fn complex_set_of_value() {
        assert_eq!(
            sequence_value(
                r#"{ not:equalityMatch:{ attributeDesc "ABCDLMYZ", assertionValue 'A2'H }, equalityMatch:{ attributeDesc "XY", assertionValue '00'H } }"#.into()
            )
            .unwrap().1,
            ASN1Value::SequenceOrSet(vec![
                (None, Box::new(
                    ASN1Value::Choice {
                        type_name: None,
                        variant_name: "not".into(),
                        inner_value: Box::new(ASN1Value::Choice {
                            type_name: None,
                            variant_name: "equalityMatch".into(),
                            inner_value: Box::new(ASN1Value::SequenceOrSet(vec![
                                (
                                    Some("attributeDesc".into()),
                                    Box::new(ASN1Value::String("ABCDLMYZ".into())),
                                ),
                                (
                                    Some("assertionValue".into()),
                                    Box::new(ASN1Value::BitString(vec![
                                        true, false, true, false, false, false, true, false
                                    ],)),
                                ),
                            ])),
                        })
                    }
                )),
                (None, Box::new(
                    ASN1Value::Choice {
                        type_name: None,
                        variant_name: "equalityMatch".into(),
                        inner_value: Box::new(ASN1Value::SequenceOrSet(vec![
                            (
                                Some("attributeDesc".into()),
                                Box::new(ASN1Value::String("XY".into())),
                            ),
                            (
                                Some("assertionValue".into()),
                                Box::new(ASN1Value::BitString(vec![
                                    false, false, false, false, false, false, false, false,
                                ],)),
                            ),
                        ])),
                    }
                ))
            ])
        );
    }

    #[test]
    fn parses_anonymous_sequence_of_item_in_field() {
        assert_eq!(
            sequence(
                r#"
            SEQUENCE {
                ages		SEQUENCE OF INTEGER (1..5)
            }
        "#
                .into()
            )
            .unwrap()
            .1,
            ASN1Type::Sequence(SequenceOrSet {
                components_of: vec![],
                extensible: None,
                constraints: vec![],
                members: vec![SequenceOrSetMember {
                    name: "ages".into(),
                    tag: None,
                    ty: ASN1Type::SequenceOf(SequenceOrSetOf {
                        element_tag: None,
                        constraints: vec![],
                        element_type: Box::new(ASN1Type::Integer(Integer {
                            constraints: vec![Constraint::Subtype(ElementSetSpecs {
                                set: ElementOrSetOperation::Element(SubtypeElements::ValueRange {
                                    min: Some(ASN1Value::Integer(1,),),
                                    max: Some(ASN1Value::Integer(5,),),
                                    extensible: false,
                                },),
                                extensible: false,
                            },),],
                            distinguished_values: None,
                        },)),
                        is_recursive: false,
                    },),
                    optionality: Optionality::Required,
                    is_recursive: false,
                    constraints: vec![],
                }],
            },)
        )
    }

    #[test]
    fn parses_prefixed_sequence_member() {
        assert_eq!(
            sequence(
                r#"
            SEQUENCE       
            {
                member SEQUENCE OF [0] INTEGER,
            }
        "#
                .into()
            )
            .unwrap()
            .1,
            ASN1Type::Sequence(SequenceOrSet {
                components_of: vec![],
                extensible: None,
                constraints: vec![],
                members: vec![SequenceOrSetMember {
                    name: "member".into(),
                    tag: None,
                    ty: ASN1Type::SequenceOf(SequenceOrSetOf {
                        constraints: vec![],
                        element_type: Box::new(ASN1Type::Integer(Integer {
                            constraints: vec![],
                            distinguished_values: None,
                        })),
                        element_tag: Some(AsnTag {
                            environment: TaggingEnvironment::Automatic,
                            tag_class: TagClass::ContextSpecific,
                            id: 0,
                        },),
                        is_recursive: false,
                    },),
                    optionality: Optionality::Required,
                    is_recursive: false,
                    constraints: vec![],
                },],
            },)
        )
    }
}
