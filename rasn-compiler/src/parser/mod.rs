//! The `parser` module contains the parser combinator
//! responsible for interpreting the input as ASN1 notation.
//! The parser is made up of a number of sub-parsers that
//! interpret single elements of ASN1 syntax.SS
//!
//! The `parser` submodules provide parsers for their
//! respective eponymous ASN1 type, with the exception
//! of `common`, which contains parsers for the more
//! generic elements of ASN1 syntax, and `util`, which
//! contains helper parsers not specific to ASN1's notation.
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::multispace1,
    combinator::{into, map, opt, recognize},
    multi::{many0, many1},
    sequence::{delimited, pair, preceded, terminated, tuple},
    IResult,
};

use crate::intermediate::{information_object::*, *};

use self::{
    bit_string::*, boolean::*, character_string::*, choice::*, common::*, constraint::*,
    embedded_pdv::*, enumerated::*, error::ParserError, external::*, information_object_class::*,
    integer::*, module_reference::*, null::*, object_identifier::*, octet_string::*,
    parameterization::*, real::*, sequence::*, sequence_of::*, set::*, set_of::*, time::*,
};

mod bit_string;
mod boolean;
mod character_string;
mod choice;
mod common;
mod constraint;
mod embedded_pdv;
mod enumerated;
mod error;
mod external;
mod information_object_class;
mod integer;
mod module_reference;
mod null;
mod object_identifier;
mod octet_string;
mod parameterization;
mod real;
mod sequence;
mod sequence_of;
mod set;
mod set_of;
mod time;
mod util;

pub fn asn_spec<'a>(
    input: &'a str,
) -> Result<Vec<(ModuleReference, Vec<ToplevelDeclaration>)>, ParserError> {
    many1(pair(
        module_reference,
        terminated(
            many0(skip_ws(alt((
                map(top_level_information_declaration,ToplevelDeclaration::Information),
                map(top_level_type_declaration, ToplevelDeclaration::Type),
                map(top_level_value_declaration, ToplevelDeclaration::Value),
            )))),
            skip_ws_and_comments(alt((encoding_control, end))),
        ),
    ))(input)
    .map(|(_, res)| res)
    .map_err(|e| e.into())
}

fn encoding_control<'a>(input: &'a str) -> IResult<&'a str, &'a str> {
    delimited(
        skip_ws_and_comments(tag("ENCODING-CONTROL")),
        take_until(END),
        end,
    )(input)
}

fn end<'a>(input: &'a str) -> IResult<&'a str, &'a str> {
    skip_ws_and_comments(preceded(
        tag(END),
        recognize(many0(alt((comment, multispace1)))),
    ))(input)
}

pub fn top_level_type_declaration<'a>(input: &'a str) -> IResult<&'a str, ToplevelTypeDeclaration> {
    into(tuple((
        skip_ws(many0(comment)),
        skip_ws(title_case_identifier),
        opt(parameterization),
        preceded(assignment, pair(opt(asn_tag), asn1_type)),
    )))(input)
}

pub fn top_level_information_declaration<'a>(
    input: &'a str,
) -> IResult<&'a str, ToplevelInformationDeclaration> {
    skip_ws(alt((
        top_level_information_object_declaration,
        top_level_object_set_declaration,
        top_level_object_class_declaration,
    )))(input)
}

pub fn asn1_type<'a>(input: &'a str) -> IResult<&'a str, ASN1Type> {
    alt((
        alt((
            null,
            selection_type_choice,
            object_identifier,
            sequence_of,
            sequence,
            set_of,
            set,
            utc_time,
            external,
            embedded_pdv,
            instance_of,
            generalized_time,
            real,
        )),
        alt((
            choice,
            integer,
            enumerated,
            boolean,
            bit_string,
            time,
            octet_string,
            character_string,
            map(information_object_field_reference, |i| {
                ASN1Type::InformationObjectFieldReference(i)
            }),
            elsewhere_declared_type,
        )),
    ))(input)
}

pub fn asn1_value<'a>(input: &'a str) -> IResult<&'a str, ASN1Value> {
    alt((
        all_value,
        null_value,
        map(object_identifier_value, ASN1Value::ObjectIdentifier),
        choice_value,
        real_value,
        sequence_value,
        sequence_or_set_of_value,
        time_value,
        bit_string_value,
        boolean_value,
        integer_value,
        character_string_value,
        elsewhere_declared_value,
    ))(input)
}

pub fn elsewhere_declared_value<'a>(input: &'a str) -> IResult<&'a str, ASN1Value> {
    map(
        pair(
            opt(skip_ws_and_comments(recognize(many1(pair(
                identifier,
                tag(".&"),
            ))))),
            value_identifier,
        ),
        |(p, id)| ASN1Value::ElsewhereDeclaredValue {
            parent: p.map(ToString::to_string),
            identifier: id.into(),
        },
    )(input)
}

pub fn elsewhere_declared_type<'a>(input: &'a str) -> IResult<&'a str, ASN1Type> {
    map(
        tuple((
            opt(skip_ws_and_comments(recognize(many1(pair(
                identifier,
                tag(".&"),
            ))))),
            skip_ws_and_comments(title_case_identifier),
            opt(skip_ws_and_comments(constraint)),
        )),
        |m| ASN1Type::ElsewhereDeclaredType(m.into()),
    )(input)
}

fn top_level_value_declaration<'a>(input: &'a str) -> IResult<&'a str, ToplevelValueDeclaration> {
    alt((
        into(tuple((
            skip_ws(many0(comment)),
            skip_ws(value_identifier),
            skip_ws(alt((
                // Cover built-in types with spaces
                tag(OBJECT_IDENTIFIER),
                tag(OCTET_STRING),
                tag(BIT_STRING),
                identifier,
            ))),
            preceded(assignment, skip_ws_and_comments(asn1_value)),
        ))),
        enumerated_value,
    ))(input)
}

fn top_level_information_object_declaration<'a>(
    input: &'a str,
) -> IResult<&'a str, ToplevelInformationDeclaration> {
    into(tuple((
        skip_ws(many0(comment)),
        skip_ws(identifier),
        skip_ws(uppercase_identifier),
        preceded(assignment, information_object),
    )))(input)
}

fn top_level_object_set_declaration<'a>(
    input: &'a str,
) -> IResult<&'a str, ToplevelInformationDeclaration> {
    into(tuple((
        skip_ws(many0(comment)),
        skip_ws(identifier),
        skip_ws(uppercase_identifier),
        preceded(assignment, object_set),
    )))(input)
}

fn top_level_object_class_declaration<'a>(
    input: &'a str,
) -> IResult<&'a str, ToplevelInformationDeclaration> {
    into(tuple((
        skip_ws(many0(comment)),
        skip_ws(uppercase_identifier),
        preceded(assignment, alt((type_identifier, information_object_class))),
    )))(input)
}

#[cfg(test)]
mod tests {
    use core::panic;
    use std::vec;

    use crate::{
        intermediate::{
            constraints::*,
            parameterization::{Parameterization, ParameterizationArgument},
            types::*,
        },
        parser::end,
    };

    use crate::parser::top_level_information_declaration;

    use super::*;

    #[test]
    fn parses_toplevel_simple_integer_declaration() {
        let tld = top_level_type_declaration(
            "/**
          * The DE represents a cardinal number that counts the size of a set. 
          * 
          * @category: Basic information
          * @revision: Created in V2.1.1
         */
         CardinalNumber3b ::= INTEGER(1..8)",
        )
        .unwrap()
        .1;
        assert_eq!(tld.name, String::from("CardinalNumber3b"));
        assert!(tld.comments.contains("@revision: Created in V2.1.1"));
        if let ASN1Type::Integer(int) = tld.r#type {
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
        let tld = top_level_type_declaration(r#"/** 
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
      } (0.. 161, ...)"#).unwrap().1;
        assert_eq!(tld.name, String::from("AccelerationMagnitudeValue"));
        assert!(tld.comments.contains("@unit 0,1 m/s^2"));
        if let ASN1Type::Integer(int) = tld.r#type {
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
        let tld = top_level_type_declaration(
            r#"-- Coverage Enhancement level encoded according to TS 36.331 [16] --
        CE-mode-B-SupportIndicator ::= ENUMERATED {
           supported,
           ...
        }"#,
        )
        .unwrap()
        .1;
        assert_eq!(tld.name, String::from("CE-mode-B-SupportIndicator"));
        assert_eq!(
            tld.comments,
            String::from(" Coverage Enhancement level encoded according to TS 36.331 [16] ")
        );
        if let ASN1Type::Enumerated(e) = tld.r#type {
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
            r#"/**
            * This DE indicates whether a vehicle (e.g. public transport vehicle, truck) is under the embarkation process.
            * If that is the case, the value is *TRUE*, otherwise *FALSE*.
            *
            * @category: Vehicle information
            * @revision: editorial update in V2.1.1
            */
           EmbarkationStatus ::= BOOLEAN"#,
        )
        .unwrap()
        .1;
        assert_eq!(tld.name, String::from("EmbarkationStatus"));
        assert!(tld
            .comments
            .contains("@revision: editorial update in V2.1.1"));
        assert_eq!(
            tld.r#type,
            ASN1Type::Boolean(Boolean {
                constraints: vec![]
            })
        );
    }

    #[test]
    fn parses_toplevel_crossrefering_declaration() {
        let tld = top_level_type_declaration(
            r#"-- Comments go here
        EventZone::= EventHistory
        ((WITH COMPONENT (WITH COMPONENTS {..., eventDeltaTime PRESENT})) |
         (WITH COMPONENT (WITH COMPONENTS {..., eventDeltaTime ABSENT})))
         }"#,
        )
        .unwrap()
        .1;
        assert_eq!(
            tld,
            ToplevelTypeDeclaration {
                parameterization: None,
                comments: " Comments go here".into(),
                name: "EventZone".into(),
                r#type: ASN1Type::ElsewhereDeclaredType(DeclarationElsewhere {
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
        let tld = top_level_type_declaration(
            r#"--Comments
        InterferenceManagementZones ::= SEQUENCE (SIZE(1..16), ...) OF InterferenceManagementZone"#,
        )
        .unwrap()
        .1;
        assert_eq!(
            tld,
            ToplevelTypeDeclaration {
                parameterization: None,
                comments: "Comments".into(),
                name: "InterferenceManagementZones".into(),
                r#type: ASN1Type::SequenceOf(SequenceOrSetOf {
                    constraints: vec![Constraint::SubtypeConstraint(ElementSet {
                        set: ElementOrSetOperation::Element(SubtypeElement::SizeConstraint(
                            Box::new(ElementOrSetOperation::Element(SubtypeElement::ValueRange {
                                min: Some(ASN1Value::Integer(1)),
                                max: Some(ASN1Value::Integer(16)),
                                extensible: false
                            }))
                        )),
                        extensible: true
                    })],
                    r#type: Box::new(ASN1Type::ElsewhereDeclaredType(DeclarationElsewhere {
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
            top_level_information_declaration(
                r#"--comments
        CpmContainers CPM-CONTAINER-ID-AND-TYPE ::= {
        {OriginatingVehicleContainer IDENTIFIED BY originatingVehicleContainer} |
        {PerceivedObjectContainer IDENTIFIED BY perceivedObjectContainer},
        ...
    }"#
            )
            .unwrap()
            .1,
            ToplevelInformationDeclaration {
                comments: "comments".into(),
                name: "CpmContainers".into(),
                index: None,
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
            top_level_information_declaration(
                r#"Reg-AdvisorySpeed	            REG-EXT-ID-AND-TYPE ::= { ... }"#
            )
            .unwrap()
            .1,
            ToplevelInformationDeclaration {
                comments: "".into(),
                index: None,
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
            top_level_information_declaration(
                r#"REG-EXT-ID-AND-TYPE ::= CLASS {
                  &id     RegionId UNIQUE,
                  &Type
                } WITH SYNTAX {&Type IDENTIFIED BY &id}"#
            )
            .unwrap()
            .1,
            ToplevelInformationDeclaration {
                comments: "".into(),
                name: "REG-EXT-ID-AND-TYPE".into(),
                class: None,
                index: None,
                value: ASN1Information::ObjectClass(InformationObjectClass {
                    fields: vec![
                        InformationObjectClassField {
                            identifier: ObjectFieldIdentifier::SingleValue("&id".into()),
                            r#type: Some(ASN1Type::ElsewhereDeclaredType(DeclarationElsewhere {
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
                            r#type: None,
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
            top_level_type_declaration(
                r#"RegionalExtension {REG-EXT-ID-AND-TYPE : Set} ::= SEQUENCE {
                  regionId     REG-EXT-ID-AND-TYPE.&id( {Set} ),
                  regExtValue  REG-EXT-ID-AND-TYPE.&Type( {Set}{@regionId} )
                }"#
            )
            .unwrap()
            .1,
            ToplevelTypeDeclaration {
                comments: "".into(),
                index: None,
                name: "RegionalExtension".into(),
                r#type: ASN1Type::Sequence(SequenceOrSet {
                    extensible: None,
                    components_of: vec![],
                    constraints: vec![],
                    members: vec![
                        SequenceOrSetMember {
                            name: "regionId".into(),
                            tag: None,
                            r#type: ASN1Type::InformationObjectFieldReference(
                                InformationObjectFieldReference {
                                    class: "REG-EXT-ID-AND-TYPE".into(),
                                    field_path: vec![ObjectFieldIdentifier::SingleValue(
                                        "&id".into()
                                    )],
                                    constraints: vec![Constraint::TableConstraint(
                                        TableConstraint {
                                            object_set: ObjectSet {
                                                values: vec![ObjectSetValue::Reference(
                                                    "Set".into()
                                                )],
                                                extensible: None
                                            },
                                            linked_fields: vec![]
                                        }
                                    )]
                                }
                            ),
                            default_value: None,
                            is_optional: false,
                            constraints: vec![]
                        },
                        SequenceOrSetMember {
                            name: "regExtValue".into(),
                            tag: None,
                            r#type: ASN1Type::InformationObjectFieldReference(
                                InformationObjectFieldReference {
                                    class: "REG-EXT-ID-AND-TYPE".into(),
                                    field_path: vec![ObjectFieldIdentifier::MultipleValue(
                                        "&Type".into()
                                    )],
                                    constraints: vec![Constraint::TableConstraint(
                                        TableConstraint {
                                            object_set: ObjectSet {
                                                values: vec![ObjectSetValue::Reference(
                                                    "Set".into()
                                                )],
                                                extensible: None
                                            },
                                            linked_fields: vec![RelationalConstraint {
                                                field_name: "regionId".into(),
                                                level: 0
                                            }]
                                        }
                                    )]
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
                        r#type: "REG-EXT-ID-AND-TYPE".into(),
                        name: Some("Set".into())
                    }]
                }),
                tag: None
            }
        )
    }

    #[test]
    fn parses_choice() {
        assert_eq!(
            top_level_type_declaration(
                r#"Choice-example ::= CHOICE
                {normal NULL,
                high NULL,
                ...,
                medium NULL }"#
            )
            .unwrap()
            .1,
            ToplevelTypeDeclaration {
                comments: "".into(),
                index: None,
                name: "Choice-example".into(),
                r#type: ASN1Type::Choice(Choice {
                    extensible: Some(2),
                    options: vec![
                        ChoiceOption {
                            name: "normal".into(),
                            tag: None,
                            r#type: ASN1Type::Null,
                            constraints: vec![]
                        },
                        ChoiceOption {
                            name: "high".into(),
                            tag: None,
                            r#type: ASN1Type::Null,
                            constraints: vec![]
                        },
                        ChoiceOption {
                            name: "medium".into(),
                            tag: None,
                            r#type: ASN1Type::Null,
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
    fn parses_comment_after_end() {
        assert!(
            end(r#"
        END
        
        -- Generated by Asnp, the ASN.1 pretty-printer of France Telecom R&D"#)
            .is_ok()
        )
    }
}
