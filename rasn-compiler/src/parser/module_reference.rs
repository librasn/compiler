use crate::intermediate::{
    EncodingReferenceDefault, ExtensibilityEnvironment, Import, ModuleReference,
    TaggingEnvironment, ASSIGN, AUTOMATIC, BEGIN, COMMA, DEFINITIONS, EXPLICIT,
    EXTENSIBILITY_IMPLIED, FROM, IMPLICIT, IMPORTS, INSTRUCTIONS, SEMICOLON, TAGS, WITH_SUCCESSORS,
};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, alpha1, alphanumeric1},
    combinator::{into, map, opt, recognize},
    multi::{many1, separated_list1, many0},
    sequence::{delimited, pair, preceded, terminated, tuple},
    IResult,
};

use super::{
    common::{identifier, skip_ws, skip_ws_and_comments},
    object_identifier::object_identifier_value,
};

pub fn module_reference<'a>(input: &'a str) -> IResult<&'a str, ModuleReference> {
    skip_ws_and_comments(into(tuple((
        identifier,
        opt(skip_ws(object_identifier_value)),
        skip_ws_and_comments(delimited(
            tag(DEFINITIONS),
            opt(environments),
            skip_ws_and_comments(pair(tag(ASSIGN), skip_ws_and_comments(tag(BEGIN)))),
        )),
        opt(imports),
    ))))(input)
}

pub fn import_identifier<'a>(input: &'a str) -> IResult<&'a str, &'a str> {
    recognize(pair(
        alpha1,
        many0(alt((preceded(char('-'), alphanumeric1), alphanumeric1, tag("{"), tag("}")))),
    ))(input)
}

fn imports<'a>(input: &'a str) -> IResult<&'a str, Vec<Import>> {
    skip_ws_and_comments(delimited(
        tag(IMPORTS),
        many1(skip_ws_and_comments(import)),
        skip_ws_and_comments(char(SEMICOLON)),
    ))(input)
}

fn import<'a>(input: &'a str) -> IResult<&'a str, Import> {
    into(skip_ws_and_comments(pair(
        skip_ws_and_comments(separated_list1(
            char(COMMA),
            skip_ws_and_comments(import_identifier),
        )),
        preceded(
            skip_ws_and_comments(tag(FROM)),
            skip_ws_and_comments(tuple((
                identifier,
                skip_ws_and_comments(object_identifier_value),
                opt(skip_ws_and_comments(tag(WITH_SUCCESSORS))),
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
            tag(INSTRUCTIONS),
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
    use crate::intermediate::*;

    use crate::parser::module_reference::module_reference;

    #[test]
    fn parses_a_module_reference() {
        assert_eq!(module_reference(r#"--! @options: no-fields-header

    ETSI-ITS-CDD {itu-t (0) identified-organization (4) etsi (0) itsDomain (5) wg1 (1) 102894 cdd (2) major-version-3 (3) minor-version-1 (1)}
    
    DEFINITIONS AUTOMATIC TAGS ::=
    
    BEGIN
    "#).unwrap().1,
    ModuleReference {name:"ETSI-ITS-CDD".into(),module_identifier:Some(ObjectIdentifierValue(vec![ObjectIdentifierArc{name:Some("itu-t".into()),number:Some(0)},ObjectIdentifierArc{name:Some("identified-organization".into()),number:Some(4)},ObjectIdentifierArc{name:Some("etsi".into()),number:Some(0)},ObjectIdentifierArc{name:Some("itsDomain".into()),number:Some(5)},ObjectIdentifierArc{name:Some("wg1".into()),number:Some(1)},ObjectIdentifierArc{name:None,number:Some(102894)},ObjectIdentifierArc{name:Some("cdd".into()),number:Some(2)},ObjectIdentifierArc{name:Some("major-version-3".into()),number:Some(3)},ObjectIdentifierArc{name:Some("minor-version-1".into()),number:Some(1)}])),encoding_reference_default:None,tagging_environment:crate::intermediate::TaggingEnvironment::Automatic,extensibility_environment:crate::intermediate::ExtensibilityEnvironment::Explicit, imports: vec![] }
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
    ModuleReference { name: "CPM-PDU-Descriptions".into(), module_identifier: Some(ObjectIdentifierValue(vec![ObjectIdentifierArc { name: Some("itu-t".into()), number: Some(0) }, ObjectIdentifierArc { name: Some("identified-organization".into()), number: Some(4) }, ObjectIdentifierArc { name: Some("etsi".into()), number: Some(0) }, ObjectIdentifierArc { name: Some("itsDomain".into()), number: Some(5) }, ObjectIdentifierArc { name: Some("wg1".into()), number: Some(1) }, ObjectIdentifierArc { name: Some("ts".into()), number: Some(103324) }, ObjectIdentifierArc { name: Some("cpm".into()), number: Some(1) }, ObjectIdentifierArc { name: Some("major-version-1".into()), number: Some(1) }, ObjectIdentifierArc { name: Some("minor-version-1".into()), number: Some(1) }])), encoding_reference_default: None, tagging_environment: TaggingEnvironment::Automatic, extensibility_environment: ExtensibilityEnvironment::Explicit, imports: vec![Import { types: vec!["ItsPduHeader".into(), "MessageRateHz".into(), "MessageSegmentationInfo".into(), "OrdinalNumber1B".into(), "ReferencePosition".into(), "StationType".into(), "TimestampIts".into()], origin_name: "ETSI-ITS-CDD".into(), origin_identifier: ObjectIdentifierValue(vec![ObjectIdentifierArc { name: Some("itu-t".into()), number: Some(0) }, ObjectIdentifierArc { name: Some("identified-organization".into()), number: Some(4) }, ObjectIdentifierArc { name: Some("etsi".into()), number: Some(0) }, ObjectIdentifierArc { name: Some("itsDomain".into()), number: Some(5) }, ObjectIdentifierArc { name: Some("wg1".into()), number: Some(1) }, ObjectIdentifierArc { name: Some("ts".into()), number: Some(102894) }, ObjectIdentifierArc { name: Some("cdd".into()), number: Some(2) }, ObjectIdentifierArc { name: Some("major-version-3".into()), number: Some(3) }, ObjectIdentifierArc { name: Some("minor-version-1".into()), number: Some(1) }]), with_successors: true }, Import { types: vec!["OriginatingRsuContainer".into(), "OriginatingVehicleContainer".into()], origin_name: "CPM-OriginatingStationContainers".into(), origin_identifier: ObjectIdentifierValue(vec![ObjectIdentifierArc { name: Some("itu-t".into()), number: Some(0) }, ObjectIdentifierArc { name: Some("identified-organization".into()), number: Some(4) }, ObjectIdentifierArc { name: Some("etsi".into()), number: Some(0) }, ObjectIdentifierArc { name: Some("itsDomain".into()), number: Some(5) }, ObjectIdentifierArc { name: Some("wg1".into()), number: Some(1) }, ObjectIdentifierArc { name: Some("ts".into()), number: Some(103324) }, ObjectIdentifierArc { name: Some("originatingStationContainers".into()), number: Some(2) }, ObjectIdentifierArc { name: Some("major-version-1".into()), number: Some(1) }, ObjectIdentifierArc { name: Some("minor-version-1".into()), number: Some(1) }]), with_successors: true }] })
    }
}
