use crate::intermediate::*;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::char,
    combinator::{into, map, opt, recognize, value},
    multi::{many0, separated_list1},
    sequence::{delimited, pair, preceded, terminated, tuple},
    IResult,
};

use super::{
    common::{identifier, skip_ws, skip_ws_and_comments, value_identifier},
    object_identifier::object_identifier_value,
};

pub fn module_reference<'a>(input: &'a str) -> IResult<&'a str, ModuleReference> {
    skip_ws_and_comments(into(tuple((
        identifier,
        opt(skip_ws(definitive_identification)),
        skip_ws_and_comments(delimited(
            tag(DEFINITIONS),
            opt(environments),
            skip_ws_and_comments(pair(tag(ASSIGN), skip_ws_and_comments(tag(BEGIN)))),
        )),
        opt(exports),
        opt(imports),
    ))))(input)
}

fn definitive_identification<'a>(input: &'a str) -> IResult<&'a str, DefinitiveIdentifier> {
    into(pair(object_identifier_value, opt(iri_value)))(input)
}

fn iri_value<'a>(input: &'a str) -> IResult<&'a str, &'a str> {
    skip_ws_and_comments(delimited(
        tag("\"/"),
        recognize(separated_list1(char('/'), identifier)),
        char('"'),
    ))(input)
}

fn exports<'a>(input: &'a str) -> IResult<&'a str, Exports> {
    skip_ws_and_comments(delimited(
        tag(EXPORTS),
        skip_ws(alt((
            value(Exports::All, tag(ALL)),
            into(separated_list1(
                skip_ws(char(COMMA)),
                skip_ws(alt((parameterized_identifier, identifier))),
            )),
        ))),
        char(SEMICOLON),
    ))(input)
}

fn imports<'a>(input: &'a str) -> IResult<&'a str, Vec<Import>> {
    skip_ws_and_comments(delimited(
        tag(IMPORTS),
        skip_ws_and_comments(many0(import)),
        skip_ws_and_comments(char(SEMICOLON)),
    ))(input)
}

fn parameterized_identifier<'a>(input: &'a str) -> IResult<&'a str, &'a str> {
    terminated(identifier, tag("{}"))(input)
}

fn import<'a>(input: &'a str) -> IResult<&'a str, Import> {
    into(skip_ws_and_comments(pair(
        separated_list1(
            skip_ws(char(COMMA)),
            skip_ws(alt((parameterized_identifier, identifier))),
        ),
        preceded(
            skip_ws_and_comments(tag(FROM)),
            skip_ws_and_comments(tuple((
                identifier,
                alt((
                    value(None, skip_ws_and_comments(value_identifier)),
                    opt(skip_ws_and_comments(object_identifier_value)),
                )),
                opt(skip_ws_and_comments(alt((
                    tag(WITH_SUCCESSORS),
                    tag(WITH_DESCENDANTS),
                )))),
            ))),
        ),
    )))(input)
}

fn environments<'a>(
    input: &'a str,
) -> IResult<
    &'a str,
    (
        Option<EncodingReferenceDefault>,
        TaggingEnvironment,
        ExtensibilityEnvironment,
    ),
> {
    tuple((
        opt(skip_ws_and_comments(into(terminated(
            identifier,
            skip_ws(tag(INSTRUCTIONS)),
        )))),
        skip_ws_and_comments(terminated(
            map(
                alt((tag(AUTOMATIC), tag(IMPLICIT), tag(EXPLICIT))),
                |m| match m {
                    AUTOMATIC => TaggingEnvironment::Automatic,
                    IMPLICIT => TaggingEnvironment::Implicit,
                    _ => TaggingEnvironment::Explicit,
                },
            ),
            skip_ws(tag(TAGS)),
        )),
        skip_ws_and_comments(map(opt(tag(EXTENSIBILITY_IMPLIED)), |m| {
            if m.is_some() {
                ExtensibilityEnvironment::Implied
            } else {
                ExtensibilityEnvironment::Explicit
            }
        })),
    ))(input)
}

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::parser::module_reference::*;

    #[test]
    fn parses_a_module_reference() {
        assert_eq!(module_reference(r#"--! @options: no-fields-header

    ETSI-ITS-CDD {itu-t (0) identified-organization (4) etsi (0) itsDomain (5) wg1 (1) 102894 cdd (2) major-version-3 (3) minor-version-1 (1)}

    DEFINITIONS AUTOMATIC TAGS ::=

    BEGIN
    "#).unwrap().1,
    ModuleReference {name:"ETSI-ITS-CDD".into(),module_identifier:Some(DefinitiveIdentifier::DefinitiveOID(ObjectIdentifierValue(vec![ObjectIdentifierArc{name:Some("itu-t".into()),number:Some(0)},ObjectIdentifierArc{name:Some("identified-organization".into()),number:Some(4)},ObjectIdentifierArc{name:Some("etsi".into()),number:Some(0)},ObjectIdentifierArc{name:Some("itsDomain".into()),number:Some(5)},ObjectIdentifierArc{name:Some("wg1".into()),number:Some(1)},ObjectIdentifierArc{name:None,number:Some(102894)},ObjectIdentifierArc{name:Some("cdd".into()),number:Some(2)},ObjectIdentifierArc{name:Some("major-version-3".into()),number:Some(3)},ObjectIdentifierArc{name:Some("minor-version-1".into()),number:Some(1)}]))),encoding_reference_default:None,tagging_environment:crate::intermediate::TaggingEnvironment::Automatic,extensibility_environment:crate::intermediate::ExtensibilityEnvironment::Explicit, imports: vec![], exports: None }
  )
    }

    #[test]
    fn parses_a_module_reference_with_imports() {
        assert_eq!(module_reference(r#"CPM-PDU-Descriptions { itu-t (0) identified-organization (4) etsi (0) itsDomain (5) wg1 (1) ts (103324) cpm (1) major-version-1 (1) minor-version-1(1)}

        DEFINITIONS AUTOMATIC TAGS ::=

        BEGIN

        IMPORTS

        ItsPduHeader, MessageRateHz, MessageSegmentationInfo, OrdinalNumber1B,  ReferencePosition, StationType, TimestampIts
        FROM ETSI-ITS-CDD {itu-t (0) identified-organization (4) etsi (0) itsDomain (5) wg1 (1) ts (102894) cdd (2) major-version-3 (3) minor-version-1 (1)}
        WITH SUCCESSORS

        OriginatingRsuContainer, OriginatingVehicleContainer
        FROM CPM-OriginatingStationContainers {itu-t (0) identified-organization (4) etsi (0) itsDomain (5) wg1 (1) ts (103324) originatingStationContainers (2) major-version-1 (1) minor-version-1(1)}
        WITH SUCCESSORS;
    "#).unwrap().1,
    ModuleReference { name: "CPM-PDU-Descriptions".into(), module_identifier: Some(DefinitiveIdentifier::DefinitiveOID(ObjectIdentifierValue(vec![ObjectIdentifierArc { name: Some("itu-t".into()), number: Some(0) }, ObjectIdentifierArc { name: Some("identified-organization".into()), number: Some(4) }, ObjectIdentifierArc { name: Some("etsi".into()), number: Some(0) }, ObjectIdentifierArc { name: Some("itsDomain".into()), number: Some(5) }, ObjectIdentifierArc { name: Some("wg1".into()), number: Some(1) }, ObjectIdentifierArc { name: Some("ts".into()), number: Some(103324) }, ObjectIdentifierArc { name: Some("cpm".into()), number: Some(1) }, ObjectIdentifierArc { name: Some("major-version-1".into()), number: Some(1) }, ObjectIdentifierArc { name: Some("minor-version-1".into()), number: Some(1) }]))), encoding_reference_default: None, tagging_environment: TaggingEnvironment::Automatic, extensibility_environment: ExtensibilityEnvironment::Explicit, imports: vec![Import { types: vec!["ItsPduHeader".into(), "MessageRateHz".into(), "MessageSegmentationInfo".into(), "OrdinalNumber1B".into(), "ReferencePosition".into(), "StationType".into(), "TimestampIts".into()], origin_name: "ETSI-ITS-CDD".into(), origin_identifier: Some(ObjectIdentifierValue(vec![ObjectIdentifierArc { name: Some("itu-t".into()), number: Some(0) }, ObjectIdentifierArc { name: Some("identified-organization".into()), number: Some(4) }, ObjectIdentifierArc { name: Some("etsi".into()), number: Some(0) }, ObjectIdentifierArc { name: Some("itsDomain".into()), number: Some(5) }, ObjectIdentifierArc { name: Some("wg1".into()), number: Some(1) }, ObjectIdentifierArc { name: Some("ts".into()), number: Some(102894) }, ObjectIdentifierArc { name: Some("cdd".into()), number: Some(2) }, ObjectIdentifierArc { name: Some("major-version-3".into()), number: Some(3) }, ObjectIdentifierArc { name: Some("minor-version-1".into()), number: Some(1) }])), with: Some(With::Successors) }, Import { types: vec!["OriginatingRsuContainer".into(), "OriginatingVehicleContainer".into()], origin_name: "CPM-OriginatingStationContainers".into(), origin_identifier: Some(ObjectIdentifierValue(vec![ObjectIdentifierArc { name: Some("itu-t".into()), number: Some(0) }, ObjectIdentifierArc { name: Some("identified-organization".into()), number: Some(4) }, ObjectIdentifierArc { name: Some("etsi".into()), number: Some(0) }, ObjectIdentifierArc { name: Some("itsDomain".into()), number: Some(5) }, ObjectIdentifierArc { name: Some("wg1".into()), number: Some(1) }, ObjectIdentifierArc { name: Some("ts".into()), number: Some(103324) }, ObjectIdentifierArc { name: Some("originatingStationContainers".into()), number: Some(2) }, ObjectIdentifierArc { name: Some("major-version-1".into()), number: Some(1) }, ObjectIdentifierArc { name: Some("minor-version-1".into()), number: Some(1) }])), with: Some(With::Successors) }], exports: None } )
    }

    #[test]
    fn parses_iri_value() {
        assert_eq!(module_reference(r#"CMSCKMKeyManagement {itu-t recommendation(0) x(24) cms-profile(894) module(0) cKMKeyManagement(1) version1(1)}
        "/ITU-T/Recommendation/X/CMS-Profile/Module/CKMKeyManagement/Version1"
        DEFINITIONS ::=
        BEGIN
        EXPORTS ALL;
        IMPORTS
        ALGORITHM,AlgorithmIdentifier{}
        FROM AlgorithmInformation-2009
        {iso(1) identified-organization(3) dod(6) internet(1) security(5)
        mechanisms(5) pkix(7) id-mod(0) id-mod-algorithmInformation-02(58)} WITH DESCENDANTS;"#).unwrap().1,
        ModuleReference {
            name: "CMSCKMKeyManagement".into(),
            module_identifier: Some(DefinitiveIdentifier::DefinitiveOIDandIRI {
                oid: ObjectIdentifierValue(vec![
                    ObjectIdentifierArc { name: Some("itu-t".into()), number: None },
                    ObjectIdentifierArc { name: Some("recommendation".into()), number: Some(0) },
                    ObjectIdentifierArc { name: Some("x".into()), number: Some(24) },
                    ObjectIdentifierArc { name: Some("cms-profile".into()), number: Some(894) },
                    ObjectIdentifierArc { name: Some("module".into()), number: Some(0) },
                    ObjectIdentifierArc { name: Some("cKMKeyManagement".into()), number: Some(1) },
                    ObjectIdentifierArc { name: Some("version1".into()), number: Some(1) },
                ]),
                iri: "ITU-T/Recommendation/X/CMS-Profile/Module/CKMKeyManagement/Version1".into()
            }),
            encoding_reference_default: None,
            tagging_environment: TaggingEnvironment::Explicit,
            extensibility_environment: ExtensibilityEnvironment::Explicit,
            imports: vec![
                Import {
                    types: vec!["ALGORITHM".into(), "AlgorithmIdentifier".into()],
                    origin_name: "AlgorithmInformation-2009".into(),
                    origin_identifier: Some(ObjectIdentifierValue(vec![
                        ObjectIdentifierArc { name: Some("iso".into()), number: Some(1) },
                        ObjectIdentifierArc { name: Some("identified-organization".into()), number: Some(3) },
                        ObjectIdentifierArc { name: Some("dod".into()), number: Some(6) },
                        ObjectIdentifierArc { name: Some("internet".into()), number: Some(1) },
                        ObjectIdentifierArc { name: Some("security".into()), number: Some(5) },
                        ObjectIdentifierArc { name: Some("mechanisms".into()), number: Some(5) },
                        ObjectIdentifierArc { name: Some("pkix".into()), number: Some(7) },
                        ObjectIdentifierArc { name: Some("id-mod".into()), number: Some(0) },
                        ObjectIdentifierArc { name: Some("id-mod-algorithmInformation-02".into()), number: Some(58) },
                    ])),
                    with: Some(With::Descendants) }
            ],
            exports: Some(Exports::All)
        })
    }

    #[test]
    fn parses_imports() {
        assert_eq!(
            imports(
                r#"IMPORTS
            DomainParameters
            FROM ANSI-X9-42
            {iso(1) member-body(2) us(840) ansi-x942(10046) module(5) 1}
            ECDomainParameters
            FROM ANSI-X9-62
            {iso(1) member-body(2) us(840) 10045 modules(0) 2};"#
            )
            .unwrap()
            .1,
            vec![
                Import {
                    types: vec!["DomainParameters".into()],
                    origin_name: "ANSI-X9-42".into(),
                    origin_identifier: Some(ObjectIdentifierValue(vec![
                        ObjectIdentifierArc {
                            name: Some("iso".into()),
                            number: Some(1)
                        },
                        ObjectIdentifierArc {
                            name: Some("member-body".into()),
                            number: Some(2)
                        },
                        ObjectIdentifierArc {
                            name: Some("us".into()),
                            number: Some(840)
                        },
                        ObjectIdentifierArc {
                            name: Some("ansi-x942".into()),
                            number: Some(10046)
                        },
                        ObjectIdentifierArc {
                            name: Some("module".into()),
                            number: Some(5)
                        },
                        ObjectIdentifierArc {
                            name: None,
                            number: Some(1)
                        },
                    ])),
                    with: None
                },
                Import {
                    types: vec!["ECDomainParameters".into()],
                    origin_name: "ANSI-X9-62".into(),
                    origin_identifier: Some(ObjectIdentifierValue(vec![
                        ObjectIdentifierArc {
                            name: Some("iso".into()),
                            number: Some(1)
                        },
                        ObjectIdentifierArc {
                            name: Some("member-body".into()),
                            number: Some(2)
                        },
                        ObjectIdentifierArc {
                            name: Some("us".into()),
                            number: Some(840)
                        },
                        ObjectIdentifierArc {
                            name: None,
                            number: Some(10045)
                        },
                        ObjectIdentifierArc {
                            name: Some("modules".into()),
                            number: Some(0)
                        },
                        ObjectIdentifierArc {
                            name: None,
                            number: Some(2)
                        },
                    ])),
                    with: None
                }
            ]
        )
    }
}
