//! Support for parsing conventions used in SNMP MIBs

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::{cut, map, opt};
use nom::multi::{many0, many1, separated_list1};
use nom::sequence::{pair, preceded, tuple};

use crate::input::Input;
use crate::intermediate::{
    ASN1Type, ASN1Value, MANDATORY_GROUPS, MODULE_COMPLIANCE, NOTIFICATION_GROUP,
    NOTIFICATION_TYPE, OBJECT_GROUP, OBJECT_TYPE,
};
use crate::lexer::character_string::character_string_value;
use crate::lexer::common::{
    in_braces, skip_ws_and_comments, title_case_identifier, value_identifier,
};
use crate::lexer::error::ParserResult;
use crate::lexer::{asn1_type, asn1_value};

// MODULE-IDENTITY MACRO ::=
// BEGIN
//     TYPE NOTATION ::=
//                   "LAST-UPDATED" value(Update ExtUTCTime)
//                   "ORGANIZATION" Text
//                   "CONTACT-INFO" Text
//                   "DESCRIPTION" Text
//                   RevisionPart
//
//     VALUE NOTATION ::=
//                   value(VALUE OBJECT IDENTIFIER)
//
//     RevisionPart ::=
//                   Revisions
//                 | empty
//     Revisions ::=
//                   Revision
//                 | Revisions Revision
//     Revision ::=
//                   "REVISION" value(Update ExtUTCTime)
//                   "DESCRIPTION" Text
//
//     -- a character string as defined in section 3.1.1
//     Text ::= value(IA5String)
// END

/// SNMPv2-SMI MODULE-IDENTITY MACRO
#[derive(Debug, Clone, PartialEq)]
pub struct ModuleIdentity {
    // "LAST-UPDATED" value(Update ExtUTCTime)
    pub last_updated: String,
    // "ORGANIZATION" value(IA5String)
    pub organization: String,
    // "CONTACT-INFO" value(IA5String)
    pub contact_info: String,
    // "DESCRIPTION" value(IA5String)
    pub description: String,
    // revision +
    pub revisions: Vec<ModuleIdentityRevision>,
}

/// REVISION in SNMPv2-SMI MODULE-IDENTITY MACRO
#[derive(Debug, Clone, PartialEq)]
pub struct ModuleIdentityRevision {
    // "REVISION" value(Update ExtUTCTime)
    revision: String,
    // "DESCRIPTION" value(IA5String)
    description: String,
}

/// Parse an SNMPv2-SMI MODULE-IDENTITY MACRO
pub fn module_identity(input: Input<'_>) -> ParserResult<'_, ModuleIdentity> {
    preceded(
        skip_ws_and_comments(tag("MODULE-IDENTITY")),
        map(
            tuple((
                skip_ws_and_comments(preceded(tag("LAST-UPDATED"), character_string_value)),
                skip_ws_and_comments(preceded(tag("ORGANIZATION"), character_string_value)),
                skip_ws_and_comments(preceded(tag("CONTACT-INFO"), character_string_value)),
                skip_ws_and_comments(preceded(tag("DESCRIPTION"), character_string_value)),
                skip_ws_and_comments(many0(module_identity_revision)),
            )),
            |v| ModuleIdentity {
                last_updated: v.0,
                organization: v.1,
                contact_info: v.2,
                description: v.3,
                revisions: v.4,
            },
        ),
    )(input)
}

/// Parse a REVISION inside an SNMPv2-SMI MODULE-IDENTITY MACRO
pub fn module_identity_revision(input: Input<'_>) -> ParserResult<'_, ModuleIdentityRevision> {
    map(
        pair(
            preceded(
                skip_ws_and_comments(tag("REVISION")),
                character_string_value,
            ),
            preceded(
                skip_ws_and_comments(tag("DESCRIPTION")),
                character_string_value,
            ),
        ),
        |v| ModuleIdentityRevision {
            revision: v.0,
            description: v.1,
        },
    )(input)
}

// TEXTUAL-CONVENTION MACRO ::=
// BEGIN
//     TYPE NOTATION ::=
//                   DisplayPart
//                   "STATUS" Status
//                   "DESCRIPTION" Text
//                   ReferPart
//                   "SYNTAX" Syntax
//
//     VALUE NOTATION ::=
//                    value(VALUE Syntax)      -- adapted ASN.1
//
//     DisplayPart ::=
//                   "DISPLAY-HINT" Text
//                 | empty
//
//     Status ::=
//                   "current"
//                 | "deprecated"
//                 | "obsolete"
//
//     ReferPart ::=
//                   "REFERENCE" Text
//                 | empty
//
//     -- a character string as defined in [2]
//     Text ::= value(IA5String)
//
//     Syntax ::=   -- Must be one of the following:
//                        -- a base type (or its refinement), or
//                        -- a BITS pseudo-type
//                   type
//                 | "BITS" "{" NamedBits "}"
//
//     NamedBits ::= NamedBit
//                 | NamedBits "," NamedBit
//
//     NamedBit ::=  identifier "(" number ")" -- number is nonnegative
//
// END

/// SNMP TEXTUAL-CONVENTION MACRO
#[derive(Debug, Clone, PartialEq)]
pub struct TextualConvention {
    // "DISPLAY-HINT" value(IA5String) | empty
    pub display_hint: Option<String>,
    // "STATUS" Status
    pub status: Status,
    // "DESCRIPTION" value(IA5String)
    pub description: String,
    // "REFERENCE" value(IA5String) | empty
    pub reference: Option<String>,
    // "SYNTAX" (
    //      asn1-type
    //    | "BITS" "{" identifier "(" number ")" ( "," identifier "(" number ")" ) * "}"
    // )
    pub syntax: ASN1Type,
}

/// Parse a SNMP TEXTUAL-CONVENTION MACRO
pub fn textual_convention(input: Input<'_>) -> ParserResult<'_, TextualConvention> {
    preceded(
        skip_ws_and_comments(tag("TEXTUAL-CONVENTION")),
        map(
            cut(tuple((
                skip_ws_and_comments(opt(preceded(
                    tag("DISPLAY-HINT"),
                    cut(character_string_value),
                ))),
                skip_ws_and_comments(preceded(tag("STATUS"), cut(status))),
                skip_ws_and_comments(preceded(tag("DESCRIPTION"), character_string_value)),
                skip_ws_and_comments(opt(preceded(tag("REFERENCE"), cut(character_string_value)))),
                skip_ws_and_comments(preceded(tag("SYNTAX"), asn1_type)),
            ))),
            |v| TextualConvention {
                display_hint: v.0,
                status: v.1,
                description: v.2,
                reference: v.3,
                syntax: v.4,
            },
        ),
    )(input)
}

// OBJECT-TYPE MACRO ::=
// BEGIN
//     TYPE NOTATION ::=
//                   "SYNTAX" Syntax
//                   UnitsPart
//                   "MAX-ACCESS" Access
//                   "STATUS" Status
//                   "DESCRIPTION" Text
//                   ReferPart
//
//                   IndexPart
//                   DefValPart
//
//     VALUE NOTATION ::=
//                   value(VALUE ObjectName)
//
//     Syntax ::=   -- Must be one of the following:
//                        -- a base type (or its refinement),
//                        -- a textual convention (or its refinement), or
//                        -- a BITS pseudo-type
//                    type
//                 | "BITS" "{" NamedBits "}"
//
//     NamedBits ::= NamedBit
//                 | NamedBits "," NamedBit
//
//     NamedBit ::=  identifier "(" number ")" -- number is nonnegative
//
//     UnitsPart ::=
//                   "UNITS" Text
//                 | empty
//
//     Access ::=
//                   "not-accessible"
//                 | "accessible-for-notify"
//                 | "read-only"
//                 | "read-write"
//                 | "read-create"
//
//     Status ::=
//                   "current"
//                 | "deprecated"
//                 | "obsolete"
//
//     ReferPart ::=
//                   "REFERENCE" Text
//                 | empty
//
//     IndexPart ::=
//                   "INDEX"    "{" IndexTypes "}"
//                 | "AUGMENTS" "{" Entry      "}"
//                 | empty
//     IndexTypes ::=
//                   IndexType
//                 | IndexTypes "," IndexType
//     IndexType ::=
//                   "IMPLIED" Index
//                 | Index
//
//     Index ::=
//                     -- use the SYNTAX value of the
//                     -- correspondent OBJECT-TYPE invocation
//                   value(ObjectName)
//     Entry ::=
//                     -- use the INDEX value of the
//                     -- correspondent OBJECT-TYPE invocation
//                   value(ObjectName)
//
//     DefValPart ::= "DEFVAL" "{" Defvalue "}"
//                 | empty
//
//     Defvalue ::=  -- must be valid for the type specified in
//                   -- SYNTAX clause of same OBJECT-TYPE macro
//                   value(ObjectSyntax)
//                 | "{" BitsValue "}"
//
//     BitsValue ::= BitNames
//                 | empty
//
//     BitNames ::=  BitName
//                 | BitNames "," BitName
//
//     BitName ::= identifier
//
//     -- a character string as defined in section 3.1.1
//     Text ::= value(IA5String)
// END

/// SNMP OBJECT-TYPE MACRO
#[derive(Debug, Clone, PartialEq)]
pub struct ObjectType<'i> {
    // "SYNTAX" asn1-type
    pub syntax: ASN1Type,
    // "UNITS" value(IA5String) | empty
    pub units: Option<String>,
    // "MAX-ACCESS" (
    //     "not-accessible"
    //   | "accessible-for-notify"
    //   | "read-only"
    //   | "read-write"
    //   | "read-create"
    //  )
    pub max_access: &'i str,
    // "STATUS" Status
    pub status: Status,
    // "DESCRIPTION" value(IA5String)
    pub description: String,
    // "REFERENCE" value(IA5String) | empty
    pub reference: Option<String>,
    // IndexPart | empty
    pub index: Option<ObjectTypeIndex<'i>>,
    // "DEFAL" "{" asn1-value ( "," asn1-value .. ) | empty
    pub defval: Option<ASN1Value>,
}

/// Index part inside SNMP OBJECT-TYPE MACRO
#[derive(Debug, Clone, PartialEq)]
pub enum ObjectTypeIndex<'i> {
    // "INDEX" "{" "IMPLIED"? value(ObjectName) ( "," "IMPLIED"? value(ObjectName) )* "}"
    Index(Vec<(bool, &'i str)>),
    // "AUGMENTS" value(ObjectName)
    Augments(&'i str),
}

/// Parse SNMMP OBJECT-TYPE MACRO
pub fn object_type(input: Input<'_>) -> ParserResult<'_, ObjectType> {
    preceded(
        skip_ws_and_comments(tag(OBJECT_TYPE)),
        map(
            tuple((
                skip_ws_and_comments(preceded(
                    tag("SYNTAX"),
                    cut(skip_ws_and_comments(asn1_type)),
                )),
                skip_ws_and_comments(opt(preceded(tag("UNITS"), cut(character_string_value)))),
                skip_ws_and_comments(preceded(
                    tag("MAX-ACCESS"),
                    skip_ws_and_comments(value_identifier),
                )),
                skip_ws_and_comments(preceded(tag("STATUS"), cut(status))),
                skip_ws_and_comments(preceded(tag("DESCRIPTION"), character_string_value)),
                skip_ws_and_comments(opt(preceded(tag("REFERENCE"), cut(character_string_value)))),
                opt(object_type_index),
                skip_ws_and_comments(opt(preceded(tag("DEFVAL"), cut(asn1_value)))),
            )),
            |v| ObjectType {
                syntax: v.0,
                units: v.1,
                max_access: v.2,
                status: v.3,
                description: v.4,
                reference: v.5,
                index: v.6,
                defval: v.7,
            },
        ),
    )(input)
}

/// Parse index part in SNMMP OBJECT-TYPE MACRO
pub fn object_type_index(input: Input<'_>) -> ParserResult<'_, ObjectTypeIndex> {
    alt((
        preceded(
            skip_ws_and_comments(tag("INDEX")),
            map(
                cut(in_braces(separated_list1(
                    skip_ws_and_comments(tag(",")),
                    pair(
                        skip_ws_and_comments(map(opt(tag("IMPLIED")), |v| v.is_some())),
                        skip_ws_and_comments(value_identifier),
                    ),
                ))),
                ObjectTypeIndex::Index,
            ),
        ),
        preceded(
            skip_ws_and_comments(tag("AUGMENTS")),
            map(
                cut(in_braces(skip_ws_and_comments(value_identifier))),
                ObjectTypeIndex::Augments,
            ),
        ),
    ))(input)
}

// NOTIFICATION-TYPE MACRO ::=
// BEGIN
//     TYPE NOTATION ::=
//                   ObjectsPart
//                   "STATUS" Status
//                   "DESCRIPTION" Text
//                   ReferPart
//
//     VALUE NOTATION ::=
//                   value(VALUE NotificationName)
//
//     ObjectsPart ::=
//                   "OBJECTS" "{" Objects "}"
//                 | empty
//     Objects ::=
//                   Object
//
//                 | Objects "," Object
//     Object ::=
//                   value(ObjectName)
//
//     Status ::=
//                   "current"
//                 | "deprecated"
//                 | "obsolete"
//
//     ReferPart ::=
//                   "REFERENCE" Text
//                 | empty
//
//     -- a character string as defined in section 3.1.1
//     Text ::= value(IA5String)
// END

/// SNMP NOTIFICATION-TYPE MACRO
#[derive(Debug, Clone, PartialEq)]
pub struct NotificationType<'i> {
    // "OBJECTS" "{" value(ObjectName) ( "," value(ObjectName) )* "}" | empty
    pub objects: Vec<&'i str>,
    // "STATUS" Status
    pub status: Status,
    // "DESCRIPTION" value(IA5String)
    pub description: String,
    // "REFERENCE" value(IA5String) | empty
    pub reference: Option<String>,
}

/// Parse SNMP NOTIFICATION-TYPE MACRO
pub fn notification_type(input: Input<'_>) -> ParserResult<'_, NotificationType> {
    preceded(
        skip_ws_and_comments(tag(NOTIFICATION_TYPE)),
        map(
            tuple((
                skip_ws_and_comments(opt(preceded(
                    tag("OBJECTS"),
                    cut(in_braces(separated_list1(
                        skip_ws_and_comments(tag(",")),
                        skip_ws_and_comments(value_identifier),
                    ))),
                ))),
                skip_ws_and_comments(preceded(tag("STATUS"), cut(status))),
                skip_ws_and_comments(preceded(tag("DESCRIPTION"), character_string_value)),
                skip_ws_and_comments(opt(preceded(tag("REFERENCE"), cut(character_string_value)))),
            )),
            |v| NotificationType {
                objects: v.0.unwrap_or_default(),
                status: v.1,
                description: v.2,
                reference: v.3,
            },
        ),
    )(input)
}

// MODULE-COMPLIANCE MACRO ::=
// BEGIN
//     TYPE NOTATION ::=
//                   "STATUS" Status
//                   "DESCRIPTION" Text
//                   ReferPart
//                   ModulePart
//
//     VALUE NOTATION ::=
//                   value(VALUE OBJECT IDENTIFIER)
//
//     Status ::=
//                   "current"
//                 | "deprecated"
//                 | "obsolete"
//
//     ReferPart ::=
//                   "REFERENCE" Text
//                 | empty
//
//     ModulePart ::=
//                   Modules
//     Modules ::=
//                   Module
//                 | Modules Module
//     Module ::=
//                   -- name of module --
//                   "MODULE" ModuleName
//                   MandatoryPart
//                   CompliancePart
//
//     ModuleName ::=
//                   -- identifier must start with uppercase letter
//                   identifier ModuleIdentifier
//                   -- must not be empty unless contained
//                   -- in MIB Module
//                 | empty
//     ModuleIdentifier ::=
//                   value(OBJECT IDENTIFIER)
//                 | empty
//
//     MandatoryPart ::=
//                   "MANDATORY-GROUPS" "{" Groups "}"
//                 | empty
//
//     Groups ::=
//
//                   Group
//                 | Groups "," Group
//     Group ::=
//                   value(OBJECT IDENTIFIER)
//
//     CompliancePart ::=
//                   Compliances
//                 | empty
//
//     Compliances ::=
//                   Compliance
//                 | Compliances Compliance
//     Compliance ::=
//                   ComplianceGroup
//                 | Object
//
//     ComplianceGroup ::=
//                   "GROUP" value(OBJECT IDENTIFIER)
//                   "DESCRIPTION" Text
//
//     Object ::=
//                   "OBJECT" value(ObjectName)
//                   SyntaxPart
//                   WriteSyntaxPart
//                   AccessPart
//                   "DESCRIPTION" Text
//
//     -- must be a refinement for object's SYNTAX clause
//     SyntaxPart ::= "SYNTAX" Syntax
//                 | empty
//
//     -- must be a refinement for object's SYNTAX clause
//     WriteSyntaxPart ::= "WRITE-SYNTAX" Syntax
//                 | empty
//
//     Syntax ::=    -- Must be one of the following:
//                        -- a base type (or its refinement),
//                        -- a textual convention (or its refinement), or
//                        -- a BITS pseudo-type
//                   type
//                 | "BITS" "{" NamedBits "}"
//
//     NamedBits ::= NamedBit
//                 | NamedBits "," NamedBit
//
//     NamedBit ::= identifier "(" number ")" -- number is nonnegative
//
//     AccessPart ::=
//                   "MIN-ACCESS" Access
//                 | empty
//     Access ::=
//                   "not-accessible"
//                 | "accessible-for-notify"
//                 | "read-only"
//                 | "read-write"
//                 | "read-create"
//
//     -- a character string as defined in [2]
//     Text ::= value(IA5String)
// END

/// SNMP MODULE-COMPLIANCE MACRO
#[derive(Debug, Clone, PartialEq)]
pub struct ModuleCompliance<'i> {
    // "STATUS" Status
    status: Status,
    // "DESCRIPTION" value(IA5String)
    description: String,
    // "REFERENCE" value(IA5String) | empty
    reference: Option<String>,
    // "MODULE" ..
    modules: Vec<ModuleComplianceModule<'i>>,
}

/// MODULE inside a MODULE-COMPLIANCE MACRO
#[derive(Debug, Clone, PartialEq)]
pub struct ModuleComplianceModule<'i> {
    // identifier ( value(OBJECT IDENTIFIER) | empty ) | empty
    module_name: Option<&'i str>,
    module_identifier: Option<&'i str>,
    // "MANDATORY-GROUPS" "{" value(OBJECT IDENTIFIER) "," .. "}" | empty
    mandatory_groups: Vec<&'i str>,
    // Compliance .. | empty
    compliances: Vec<Compliance<'i>>,
}

/// Compliance part inside SNMP MODULE-COMPLIANCE MACRO
#[derive(Debug, Clone, PartialEq)]
pub enum Compliance<'i> {
    // "GROUP"
    Group {
        // value(OBJECT IDENTIFIER)
        object_identifier: &'i str,
        // "DESCRIPTION" value(IA5String)
        description: String,
    },
    // "OBJECT"
    Object {
        // value(ObjectName)
        name: &'i str,
        // "SYNTAX" (
        //      type
        //      | "BITS" "{" identifier "(" number ")" ( "," identifier "(" number ")" ) * "}"
        //   )
        //   | empty
        syntax: Option<ASN1Type>,
        // "WRITE-SYNTAX"
        //     type
        //   | "BITS" "{" identifier "(" number ")" ( "," identifier "(" number ")" ) * "}"
        //   | empty
        write_syntax: Option<ASN1Type>,
        // "MIN-ACCESS" identifier | empty
        min_access: Option<&'i str>,
        // "DESCRIPTION" value(IA5String)
        description: String,
    },
}

/// Parse SNMP MODULE-COMPLIANCE MACRO
pub fn module_compliance(input: Input<'_>) -> ParserResult<'_, ModuleCompliance> {
    preceded(
        skip_ws_and_comments(tag(MODULE_COMPLIANCE)),
        map(
            cut(tuple((
                preceded(skip_ws_and_comments(tag("STATUS")), cut(status)),
                preceded(
                    skip_ws_and_comments(tag("DESCRIPTION")),
                    character_string_value,
                ),
                opt(preceded(
                    skip_ws_and_comments(tag("REFERENCE")),
                    cut(character_string_value),
                )),
                many1(module_compliance_module),
            ))),
            |v| ModuleCompliance {
                status: v.0,
                description: v.1,
                reference: v.2,
                modules: v.3,
            },
        ),
    )(input)
}

/// Parse MODULE part in SNMP MODULE-COMPLIANCE MACRO
pub fn module_compliance_module(input: Input<'_>) -> ParserResult<'_, ModuleComplianceModule> {
    preceded(
        skip_ws_and_comments(tag("MODULE")),
        map(
            cut(tuple((
                opt(pair(
                    skip_ws_and_comments(title_case_identifier),
                    skip_ws_and_comments(opt(value_identifier)),
                )),
                opt(preceded(
                    skip_ws_and_comments(tag(MANDATORY_GROUPS)),
                    cut(in_braces(separated_list1(
                        skip_ws_and_comments(tag(",")),
                        skip_ws_and_comments(value_identifier),
                    ))),
                )),
                many1(compliance),
            ))),
            |v| ModuleComplianceModule {
                module_name: v.0.as_ref().map(|v0| v0.0),
                module_identifier: v.0.and_then(|v0| v0.1),
                mandatory_groups: v.1.unwrap_or_default(),
                compliances: v.2,
            },
        ),
    )(input)
}

/// Parse compliance part inside SNMP MODULE-COMPLIANCE MACRO
pub fn compliance(input: Input<'_>) -> ParserResult<'_, Compliance> {
    skip_ws_and_comments(alt((
        preceded(
            tag("GROUP"),
            map(
                cut(pair(
                    skip_ws_and_comments(value_identifier),
                    skip_ws_and_comments(preceded(tag("DESCRIPTION"), character_string_value)),
                )),
                |v| Compliance::Group {
                    object_identifier: v.0,
                    description: v.1,
                },
            ),
        ),
        preceded(
            tag("OBJECT"),
            map(
                cut(tuple((
                    skip_ws_and_comments(value_identifier),
                    opt(preceded(
                        skip_ws_and_comments(tag("SYNTAX")),
                        cut(skip_ws_and_comments(asn1_type)),
                    )),
                    skip_ws_and_comments(opt(preceded(
                        tag("WRITE-SYNTAX"),
                        cut(skip_ws_and_comments(asn1_type)),
                    ))),
                    skip_ws_and_comments(opt(preceded(
                        tag("MIN-ACCESS"),
                        cut(skip_ws_and_comments(value_identifier)),
                    ))),
                    skip_ws_and_comments(preceded(tag("DESCRIPTION"), character_string_value)),
                ))),
                |v| Compliance::Object {
                    name: v.0,
                    syntax: v.1,
                    write_syntax: v.2,
                    min_access: v.3,
                    description: v.4,
                },
            ),
        ),
    )))(input)
}

// OBJECT-GROUP MACRO ::=
// BEGIN
//     TYPE NOTATION ::=
//                   ObjectsPart
//                   "STATUS" Status
//                   "DESCRIPTION" Text
//                   ReferPart
//
//     VALUE NOTATION ::=
//                   value(VALUE OBJECT IDENTIFIER)
//
//     ObjectsPart ::=
//                   "OBJECTS" "{" Objects "}"
//     Objects ::=
//                   Object
//                 | Objects "," Object
//     Object ::=
//
//                   value(ObjectName)
//
//     Status ::=
//                   "current"
//                 | "deprecated"
//                 | "obsolete"
//
//     ReferPart ::=
//                   "REFERENCE" Text
//                 | empty
//
//     -- a character string as defined in [2]
//     Text ::= value(IA5String)
// END

/// SNMP OBJECT-GROUP MACRO
#[derive(Debug, Clone, PartialEq)]
pub struct ObjectGroup<'i> {
    // "OBJECTS" "{" value(ObjectName) ( "," value(ObjectType) )* "}"
    objects: Vec<&'i str>,
    // "STATUS" Status
    status: Status,
    // "DESCRIPTION" value(IA5String)
    description: String,
    // "REFERENCE" value(IA5String) | empty
    reference: Option<String>,
}

/// Parse SNMP OBJECT-GROUP MACRO
pub fn object_group(input: Input<'_>) -> ParserResult<'_, ObjectGroup> {
    preceded(
        skip_ws_and_comments(tag(OBJECT_GROUP)),
        map(
            cut(tuple((
                preceded(
                    skip_ws_and_comments(tag("OBJECTS")),
                    in_braces(separated_list1(
                        skip_ws_and_comments(tag(",")),
                        skip_ws_and_comments(value_identifier),
                    )),
                ),
                preceded(skip_ws_and_comments(tag("STATUS")), cut(status)),
                preceded(
                    skip_ws_and_comments(tag("DESCRIPTION")),
                    character_string_value,
                ),
                opt(preceded(
                    skip_ws_and_comments(tag("REFERENCE")),
                    cut(character_string_value),
                )),
            ))),
            |v| ObjectGroup {
                objects: v.0,
                status: v.1,
                description: v.2,
                reference: v.3,
            },
        ),
    )(input)
}

// NOTIFICATION-GROUP MACRO ::=
// BEGIN
//     TYPE NOTATION ::=
//                   NotificationsPart
//                   "STATUS" Status
//                   "DESCRIPTION" Text
//                   ReferPart
//
//     VALUE NOTATION ::=
//                   value(VALUE OBJECT IDENTIFIER)
//
//     NotificationsPart ::=
//                   "NOTIFICATIONS" "{" Notifications "}"
//     Notifications ::=
//                   Notification
//                 | Notifications "," Notification
//     Notification ::=
//                   value(NotificationName)
//
//     Status ::=
//                   "current"
//                 | "deprecated"
//                 | "obsolete"
//
//     ReferPart ::=
//                   "REFERENCE" Text
//                 | empty
//
//     -- a character string as defined in [2]
//     Text ::= value(IA5String)
// END

/// SNMP NOTIFICATION_GROUP MACRO
#[derive(Debug, Clone, PartialEq)]
pub struct NotificationGroup<'i> {
    // "NOTIFICATIONS" "{" value(ObjectName) ( "," value(ObjectType) )* "}"
    notifications: Vec<&'i str>,
    // "STATUS" Status
    status: Status,
    // "DESCRIPTION" value(IA5String)
    description: String,
    // "REFERENCE" value(IA5String) | empty
    reference: Option<String>,
}

/// Parse SNMP NOTIFICATION_GROUP MACRO
pub fn notification_group(input: Input<'_>) -> ParserResult<'_, NotificationGroup> {
    preceded(
        skip_ws_and_comments(tag(NOTIFICATION_GROUP)),
        map(
            cut(tuple((
                preceded(
                    skip_ws_and_comments(tag("NOTIFICATIONS")),
                    in_braces(separated_list1(
                        skip_ws_and_comments(tag(",")),
                        skip_ws_and_comments(value_identifier),
                    )),
                ),
                preceded(skip_ws_and_comments(tag("STATUS")), cut(status)),
                preceded(
                    skip_ws_and_comments(tag("DESCRIPTION")),
                    character_string_value,
                ),
                opt(preceded(
                    skip_ws_and_comments(tag("REFERENCE")),
                    cut(character_string_value),
                )),
            ))),
            |v| NotificationGroup {
                notifications: v.0,
                status: v.1,
                description: v.2,
                reference: v.3,
            },
        ),
    )(input)
}

#[derive(Debug, Clone, PartialEq)]
pub enum Status {
    Current,
    Deprecated,
    Obsolete,
}

/// Parse a status: "current" | "deprecated" | "obsolete"
fn status(input: Input<'_>) -> ParserResult<'_, Status> {
    skip_ws_and_comments(alt((
        map(tag("current"), |_| Status::Current),
        map(tag("deprecated"), |_| Status::Deprecated),
        map(tag("obsolete"), |_| Status::Obsolete),
    )))(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::intermediate::constraints::{
        Constraint, ElementOrSetOperation, ElementSet, SetOperation, SetOperator, SubtypeElement,
    };
    use crate::intermediate::DeclarationElsewhere;

    #[test]
    fn parses_module_identity() {
        let input = Input::from(
            r#"
            MODULE-IDENTITY
                LAST-UPDATED "200006140000Z"
                ORGANIZATION "IETF Interfaces MIB Working Group"
                CONTACT-INFO "Keith McCloghrie"
                DESCRIPTION "The MIB module"
            "#,
        );

        let mi = module_identity(input).unwrap().1;

        assert_eq!(
            mi,
            ModuleIdentity {
                last_updated: "200006140000Z".to_string(),
                organization: "IETF Interfaces MIB Working Group".to_string(),
                contact_info: "Keith McCloghrie".to_string(),
                description: "The MIB module".to_string(),
                revisions: vec![],
            }
        );
    }

    #[test]
    fn parses_textual_convention() {
        let input = Input::from(
            r#"
            TEXTUAL-CONVENTION
                DISPLAY-HINT "d"
                STATUS       current
                DESCRIPTION

                        "A unique value, greater than zero, for each interface or
                        network management system to the next re-initialization."
                SYNTAX       Integer32 (1..2147483647)
            "#,
        );

        let tc = textual_convention(input).unwrap().1;

        assert_eq!(
            tc,
            TextualConvention {
                display_hint: Some("d".to_string()),
                status: Status::Current,
                description: "A unique value, greater than zero, for each interface or
                        network management system to the next re-initialization."
                    .to_string(),
                reference: None,
                syntax: ASN1Type::ElsewhereDeclaredType(DeclarationElsewhere {
                    parent: None,
                    identifier: "Integer32".to_string(),
                    constraints: vec![Constraint::SubtypeConstraint(ElementSet {
                        set: ElementOrSetOperation::Element(SubtypeElement::ValueRange {
                            min: Some(ASN1Value::Integer(1)),
                            max: Some(ASN1Value::Integer(2147483647)),
                            extensible: false
                        }),
                        extensible: false
                    })],
                }),
            }
        );
    }

    #[test]
    fn parses_object_type() {
        let input = Input::from(
            r#"
            OBJECT-TYPE
                SYNTAX      Integer32
                MAX-ACCESS  read-only
                STATUS      current
                DESCRIPTION
                        "The number of network interfaces (regardless of their
                        current state) present on this system."
            "#,
        );

        let obj_ty = object_type(input).unwrap().1;

        assert_eq!(
            obj_ty,
            ObjectType {
                syntax: ASN1Type::ElsewhereDeclaredType(DeclarationElsewhere {
                    parent: None,
                    identifier: "Integer32".to_string(),
                    constraints: vec![],
                }),
                units: None,
                max_access: "read-only",
                status: Status::Current,
                description: "The number of network interfaces (regardless of their
                        current state) present on this system."
                    .to_string(),
                reference: None,
                index: None,
                defval: None,
            }
        );
    }

    #[test]
    fn parses_notification_type() {
        let input = Input::from(
            r#"
            NOTIFICATION-TYPE
                OBJECTS { ifIndex, ifAdminStatus, ifOperStatus }
                STATUS  current
                DESCRIPTION
                        "A linkDown trap signifies that the SNMP entity, acting in
                        an agent role, has detected that the ifOperStatus object for
                        one of its communication links is about to enter the down
                        state from some other state (but not from the notPresent
                        state).  This other state is indicated by the included value
                        of ifOperStatus."
            "#,
        );

        let notif_ty = notification_type(input).unwrap().1;

        assert_eq!(
            notif_ty,
            NotificationType {
                objects: vec!["ifIndex", "ifAdminStatus", "ifOperStatus"],
                status: Status::Current,
                description: "A linkDown trap signifies that the SNMP entity, acting in
                        an agent role, has detected that the ifOperStatus object for
                        one of its communication links is about to enter the down
                        state from some other state (but not from the notPresent
                        state).  This other state is indicated by the included value
                        of ifOperStatus."
                    .to_string(),
                reference: None,
            }
        );
    }

    #[test]
    fn parses_module_compliance() {
        let input = Input::from(
            r#"
            MODULE-COMPLIANCE
                STATUS      current
                DESCRIPTION
                    "The compliance
                    ifDescr, ifPhysAddress, implemented."

                MODULE
                    MANDATORY-GROUPS {
                        dot1dBaseBridgeGroup,
                        dot1dBasePortGroup
                    }

                GROUP   dot1dStpBridgeGroup
                DESCRIPTION
                    "Implementation of
                    Spanning Tree Protocol."

                OBJECT dot1dStpPriority
                SYNTAX Integer32 (4096
                            |24576)
                DESCRIPTION
                    "values defined IEEE 802.1t."
            "#,
        );

        let mc = module_compliance(input).unwrap().1;

        assert_eq!(
            mc,
            ModuleCompliance {
                status: Status::Current,
                description: "The compliance
                    ifDescr, ifPhysAddress, implemented."
                    .to_string(),
                reference: None,
                modules: vec![ModuleComplianceModule {
                    module_name: None,
                    module_identifier: None,
                    mandatory_groups: vec!["dot1dBaseBridgeGroup", "dot1dBasePortGroup"],
                    compliances: vec![
                        Compliance::Group {
                            object_identifier: "dot1dStpBridgeGroup",
                            description:
                                "Implementation of\n                    Spanning Tree Protocol."
                                    .to_string(),
                        },
                        Compliance::Object {
                            name: "dot1dStpPriority",
                            syntax: Some(ASN1Type::ElsewhereDeclaredType(DeclarationElsewhere {
                                parent: None,
                                identifier: "Integer32".to_string(),
                                constraints: vec![Constraint::SubtypeConstraint(ElementSet {
                                    set: ElementOrSetOperation::SetOperation(SetOperation {
                                        base: SubtypeElement::SingleValue {
                                            value: ASN1Value::Integer(4096),
                                            extensible: false,
                                        },
                                        operator: SetOperator::Union,
                                        operant: Box::new(ElementOrSetOperation::Element(
                                            SubtypeElement::SingleValue {
                                                value: ASN1Value::Integer(24576),
                                                extensible: false,
                                            }
                                        )),
                                    }),
                                    extensible: false,
                                })],
                            })),
                            write_syntax: None,
                            min_access: None,
                            description: "values defined IEEE 802.1t.".to_string(),
                        },
                    ],
                }]
            }
        );
    }

    #[test]
    fn parses_object_group() {
        let input = Input::from(
            r#"
            OBJECT-GROUP
            OBJECTS { ifIndex, ifPhysAddress,
                    ifTableLastChange }
            STATUS  current
            DESCRIPTION
                    "A collection of objects providing information applicable to
                    all network interfaces."
            "#,
        );

        let mc = object_group(input).unwrap().1;

        assert_eq!(
            mc,
            ObjectGroup {
                objects: vec!["ifIndex", "ifPhysAddress", "ifTableLastChange"],
                status: Status::Current,
                description: "A collection of objects providing information applicable to
                    all network interfaces."
                    .to_string(),
                reference: None,
            }
        );
    }

    #[test]
    fn parses_notification_group() {
        let input = Input::from(
            r#"
            NOTIFICATION-GROUP
                NOTIFICATIONS { linkUp, linkDown }
                STATUS  current
                DESCRIPTION
                        "The notifications which indicate specific changes in the
                        value of ifOperStatus."
            "#,
        );

        let mc = notification_group(input).unwrap().1;

        assert_eq!(
            mc,
            NotificationGroup {
                notifications: vec!["linkUp", "linkDown"],
                status: Status::Current,
                description: "The notifications which indicate specific changes in the
                        value of ifOperStatus."
                    .to_string(),
                reference: None,
            }
        );
    }
}
