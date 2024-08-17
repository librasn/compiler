use core::panic;
use std::vec;

use crate::{
    intermediate::{
        constraints::*,
        parameterization::{ParameterGovernor, Parameterization, ParameterizationArgument},
        types::*,
    },
    lexer::*,
};

use crate::lexer::top_level_information_declaration;

#[test]
fn parses_toplevel_simple_integer_declaration() {
    let tld = top_level_type_declaration(Span::new(
        "/**
          * The DE represents a cardinal number that counts the size of a set.
          *
          * @category: Basic information
          * @revision: Created in V2.1.1
         */
         CardinalNumber3b ::= INTEGER(1..8)",
    ))
    .unwrap()
    .1;
    assert_eq!(tld.name, String::from("CardinalNumber3b"));
    assert!(tld.comments.contains("@revision: Created in V2.1.1"));
    if let ASN1Type::Integer(int) = tld.ty {
        assert!(!int.constraints.is_empty());
        assert_eq!(
            *int.constraints.first().unwrap(),
            Constraint::SubtypeConstraint(ElementSet {
                set: ElementOrSetOperation::Element(SubtypeElement::ValueRange {
                    min: Some(ASN1Value::Integer(1)),
                    max: Some(ASN1Value::Integer(8)),
                    extensible: false
                }),
                extensible: false
            })
        );
    } else {
        panic!("Top-level declaration contains other type than integer.")
    }
}

#[test]
fn parses_toplevel_macro_integer_declaration() {
    let tld = top_level_type_declaration(Span::new(r#"/**
        * This DE represents the magnitude of the acceleration vector in a defined coordinate system.
        *
        * The value shall be set to:
        * - `0` to indicate no acceleration,
        * - `n` (`n > 0` and `n < 160`) to indicate acceleration equal to or less than n x 0,1 m/s^2, and greater than (n-1) x 0,1 m/s^2,
        * - `160` for acceleration values greater than 15,9 m/s^2,
        * - `161` when the data is unavailable.
        *
        * @unit 0,1 m/s^2
        * @category: Kinematic information
        * @revision: Created in V2.1.1
      */
      AccelerationMagnitudeValue ::= INTEGER {
          positiveOutOfRange (160),
          unavailable        (161)
      } (0.. 161, ...)"#)).unwrap().1;
    assert_eq!(tld.name, String::from("AccelerationMagnitudeValue"));
    assert!(tld.comments.contains("@unit 0,1 m/s^2"));
    if let ASN1Type::Integer(int) = tld.ty {
        assert_eq!(
            *int.constraints.first().unwrap(),
            Constraint::SubtypeConstraint(ElementSet {
                set: ElementOrSetOperation::Element(SubtypeElement::ValueRange {
                    min: Some(ASN1Value::Integer(0)),
                    max: Some(ASN1Value::Integer(161)),
                    extensible: true
                }),
                extensible: false
            })
        );

        assert_eq!(int.distinguished_values.as_ref().unwrap().len(), 2);
        assert_eq!(
            int.distinguished_values.as_ref().unwrap()[0],
            DistinguishedValue {
                name: String::from("positiveOutOfRange"),
                value: 160
            }
        );
    } else {
        panic!("Top-level declaration contains other type than integer.")
    }
}

#[test]
fn parses_toplevel_enumerated_declaration() {
    let tld = top_level_type_declaration(Span::new(
        r#"-- Coverage Enhancement level encoded according to TS 36.331 [16] --
        CE-mode-B-SupportIndicator ::= ENUMERATED {
           supported,
           ...
        }"#,
    ))
    .unwrap()
    .1;
    assert_eq!(tld.name, String::from("CE-mode-B-SupportIndicator"));
    assert_eq!(
        tld.comments,
        String::from(" Coverage Enhancement level encoded according to TS 36.331 [16] ")
    );
    if let ASN1Type::Enumerated(e) = tld.ty {
        assert_eq!(e.members.len(), 1);
        assert_eq!(
            e.members[0],
            Enumeral {
                name: "supported".into(),
                index: 0,
                description: None
            }
        );
        assert_eq!(e.extensible, Some(1));
    } else {
        panic!("Top-level declaration contains other type than integer.")
    }
}

#[test]
fn parses_toplevel_boolean_declaration() {
    let tld = top_level_type_declaration(
            Span::new(r#"/**
            * This DE indicates whether a vehicle (e.g. public transport vehicle, truck) is under the embarkation process.
            * If that is the case, the value is *TRUE*, otherwise *FALSE*.
            *
            * @category: Vehicle information
            * @revision: editorial update in V2.1.1
            */
           EmbarkationStatus ::= BOOLEAN"#,)
        )
        .unwrap()
        .1;
    assert_eq!(tld.name, String::from("EmbarkationStatus"));
    assert!(tld
        .comments
        .contains("@revision: editorial update in V2.1.1"));
    assert_eq!(
        tld.ty,
        ASN1Type::Boolean(Boolean {
            constraints: vec![]
        })
    );
}

#[test]
fn parses_toplevel_crossrefering_declaration() {
    let tld = top_level_type_declaration(Span::new(
        r#"-- Comments go here
        EventZone::= EventHistory
        ((WITH COMPONENT (WITH COMPONENTS {..., eventDeltaTime PRESENT})) |
         (WITH COMPONENT (WITH COMPONENTS {..., eventDeltaTime ABSENT})))
         }"#,
    ))
    .unwrap()
    .1;
    assert_eq!(
        tld,
        ToplevelTypeDefinition {
            parameterization: None,
            comments: " Comments go here".into(),
            name: "EventZone".into(),
            ty: ASN1Type::ElsewhereDeclaredType(DeclarationElsewhere {
                parent: None,
                identifier: "EventHistory".into(),
                constraints: vec![Constraint::SubtypeConstraint(ElementSet {
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
            }),
            tag: None,
            index: None
        }
    );
}

#[test]
fn parses_anonymous_sequence_of_declaration() {
    let tld = top_level_type_declaration(Span::new(
        r#"--Comments
        InterferenceManagementZones ::= SEQUENCE (SIZE(1..16), ...) OF InterferenceManagementZone"#,
    ))
    .unwrap()
    .1;
    assert_eq!(
        tld,
        ToplevelTypeDefinition {
            parameterization: None,
            comments: "Comments".into(),
            name: "InterferenceManagementZones".into(),
            ty: ASN1Type::SequenceOf(SequenceOrSetOf {
                constraints: vec![Constraint::SubtypeConstraint(ElementSet {
                    set: ElementOrSetOperation::Element(SubtypeElement::SizeConstraint(Box::new(
                        ElementOrSetOperation::Element(SubtypeElement::ValueRange {
                            min: Some(ASN1Value::Integer(1)),
                            max: Some(ASN1Value::Integer(16)),
                            extensible: false
                        })
                    ))),
                    extensible: true
                })],
                element_type: Box::new(ASN1Type::ElsewhereDeclaredType(DeclarationElsewhere {
                    parent: None,
                    identifier: "InterferenceManagementZone".into(),
                    constraints: vec![]
                }))
            }),
            tag: None,
            index: None
        }
    );
}

#[test]
fn parses_object_set_value() {
    assert_eq!(
        top_level_information_declaration(Span::new(
            r#"--comments
        CpmContainers CPM-CONTAINER-ID-AND-TYPE ::= {
        {OriginatingVehicleContainer IDENTIFIED BY originatingVehicleContainer} |
        {PerceivedObjectContainer IDENTIFIED BY perceivedObjectContainer},
        ...
    }"#
        ))
        .unwrap()
        .1,
        ToplevelInformationDefinition {
            comments: "comments".into(),
            name: "CpmContainers".into(),
            index: None,
            parameterization: None,
            class: Some(ClassLink::ByName("CPM-CONTAINER-ID-AND-TYPE".into())),
            value: ASN1Information::ObjectSet(ObjectSet {
                values: vec![
                    ObjectSetValue::Inline(InformationObjectFields::CustomSyntax(vec![
                        SyntaxApplication::LiteralOrTypeReference(DeclarationElsewhere {
                            parent: None,
                            identifier: "OriginatingVehicleContainer".into(),
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
                            identifier: "originatingVehicleContainer".into(),
                            parent: None
                        })
                    ])),
                    ObjectSetValue::Inline(InformationObjectFields::CustomSyntax(vec![
                        SyntaxApplication::LiteralOrTypeReference(DeclarationElsewhere {
                            parent: None,
                            identifier: "PerceivedObjectContainer".into(),
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
                            identifier: "perceivedObjectContainer".into(),
                            parent: None
                        })
                    ]))
                ],
                extensible: Some(2)
            })
        }
    )
}

#[test]
fn parses_empty_extensible_object_set() {
    assert_eq!(
        top_level_information_declaration(Span::new(
            r#"Reg-AdvisorySpeed	            REG-EXT-ID-AND-TYPE ::= { ... }"#
        ))
        .unwrap()
        .1,
        ToplevelInformationDefinition {
            comments: "".into(),
            index: None,
            parameterization: None,
            name: "Reg-AdvisorySpeed".into(),
            class: Some(ClassLink::ByName("REG-EXT-ID-AND-TYPE".into())),
            value: ASN1Information::ObjectSet(ObjectSet {
                values: vec![],
                extensible: Some(0)
            })
        }
    )
}

#[test]
fn parses_class_declaration() {
    assert_eq!(
        top_level_information_declaration(Span::new(
            r#"REG-EXT-ID-AND-TYPE ::= CLASS {
                  &id     RegionId UNIQUE,
                  &Type
                } WITH SYNTAX {&Type IDENTIFIED BY &id}"#
        ))
        .unwrap()
        .1,
        ToplevelInformationDefinition {
            comments: "".into(),
            name: "REG-EXT-ID-AND-TYPE".into(),
            class: None,
            index: None,
            parameterization: None,
            value: ASN1Information::ObjectClass(InformationObjectClass {
                fields: vec![
                    InformationObjectClassField {
                        identifier: ObjectFieldIdentifier::SingleValue("&id".into()),
                        ty: Some(ASN1Type::ElsewhereDeclaredType(DeclarationElsewhere {
                            parent: None,
                            identifier: "RegionId".into(),
                            constraints: vec![]
                        })),
                        is_optional: false,
                        default: None,
                        is_unique: true
                    },
                    InformationObjectClassField {
                        identifier: ObjectFieldIdentifier::MultipleValue("&Type".into()),
                        ty: None,
                        is_optional: false,
                        default: None,
                        is_unique: false
                    }
                ],
                syntax: Some(InformationObjectSyntax {
                    expressions: vec![
                        SyntaxExpression::Required(SyntaxToken::Field(
                            ObjectFieldIdentifier::MultipleValue("&Type".into())
                        )),
                        SyntaxExpression::Required(SyntaxToken::Literal("IDENTIFIED".into())),
                        SyntaxExpression::Required(SyntaxToken::Literal("BY".into())),
                        SyntaxExpression::Required(SyntaxToken::Field(
                            ObjectFieldIdentifier::SingleValue("&id".into())
                        ))
                    ]
                })
            })
        }
    )
}

#[test]
fn parses_parameterized_declaration() {
    assert_eq!(
        top_level_type_declaration(Span::new(
            r#"RegionalExtension {REG-EXT-ID-AND-TYPE : Set} ::= SEQUENCE {
                  regionId     REG-EXT-ID-AND-TYPE.&id( {Set} ),
                  regExtValue  REG-EXT-ID-AND-TYPE.&Type( {Set}{@regionId} )
                }"#
        ))
        .unwrap()
        .1,
        ToplevelTypeDefinition {
            comments: "".into(),
            index: None,
            name: "RegionalExtension".into(),
            ty: ASN1Type::Sequence(SequenceOrSet {
                extensible: None,
                components_of: vec![],
                constraints: vec![],
                members: vec![
                    SequenceOrSetMember {
                        name: "regionId".into(),
                        tag: None,
                        ty: ASN1Type::InformationObjectFieldReference(
                            InformationObjectFieldReference {
                                class: "REG-EXT-ID-AND-TYPE".into(),
                                field_path: vec![ObjectFieldIdentifier::SingleValue("&id".into())],
                                constraints: vec![Constraint::TableConstraint(TableConstraint {
                                    object_set: ObjectSet {
                                        values: vec![ObjectSetValue::Reference("Set".into())],
                                        extensible: None
                                    },
                                    linked_fields: vec![]
                                })]
                            }
                        ),
                        default_value: None,
                        is_optional: false,
                        constraints: vec![]
                    },
                    SequenceOrSetMember {
                        name: "regExtValue".into(),
                        tag: None,
                        ty: ASN1Type::InformationObjectFieldReference(
                            InformationObjectFieldReference {
                                class: "REG-EXT-ID-AND-TYPE".into(),
                                field_path: vec![ObjectFieldIdentifier::MultipleValue(
                                    "&Type".into()
                                )],
                                constraints: vec![Constraint::TableConstraint(TableConstraint {
                                    object_set: ObjectSet {
                                        values: vec![ObjectSetValue::Reference("Set".into())],
                                        extensible: None
                                    },
                                    linked_fields: vec![RelationalConstraint {
                                        field_name: "regionId".into(),
                                        level: 0
                                    }]
                                })]
                            }
                        ),
                        default_value: None,
                        is_optional: false,
                        constraints: vec![]
                    }
                ]
            }),
            parameterization: Some(Parameterization {
                parameters: vec![ParameterizationArgument {
                    dummy_reference: "Set".into(),
                    param_governor: ParameterGovernor::Class("REG-EXT-ID-AND-TYPE".into())
                }]
            }),
            tag: None
        }
    )
}

#[test]
fn parses_choice() {
    assert_eq!(
        top_level_type_declaration(Span::new(
            r#"Choice-example ::= CHOICE
                {normal NULL,
                high NULL,
                ...,
                medium NULL }"#
        ))
        .unwrap()
        .1,
        ToplevelTypeDefinition {
            comments: "".into(),
            index: None,
            name: "Choice-example".into(),
            ty: ASN1Type::Choice(Choice {
                extensible: Some(2),
                options: vec![
                    ChoiceOption {
                        name: "normal".into(),
                        tag: None,
                        ty: ASN1Type::Null,
                        constraints: vec![]
                    },
                    ChoiceOption {
                        name: "high".into(),
                        tag: None,
                        ty: ASN1Type::Null,
                        constraints: vec![]
                    },
                    ChoiceOption {
                        name: "medium".into(),
                        tag: None,
                        ty: ASN1Type::Null,
                        constraints: vec![]
                    }
                ],
                constraints: vec![]
            }),
            parameterization: None,
            tag: None
        }
    )
}

#[test]
fn parses_sequence_of_value() {
    println!(
        "{:?}",
        top_level_value_declaration(Span::new(
            r#"test-Sequence SEQUENCE OF INTEGER ::= { 1, 2, 3 }"#
        ))
        .unwrap()
        .1
    )
}

#[test]
fn parses_comment_after_end() {
    assert!(end(Span::new(
        r#"
        END

        -- Generated by Asnp, the ASN.1 pretty-printer of France Telecom R&D"#
    ))
    .is_ok())
}
