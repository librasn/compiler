//! The `intermediate` module describes the single elements of the ASN1 notation.
//! It includes constants for the various ASN1 keywords and types to represent the
//! single ASN1 data elements in an intermediate representation from which the
//! generator module produces de-/encodable types.
//! The intermediate representation aims to preserve as much information as possible
//! from the original specification, even though some of that information might not actually
//! be relevant for decoding and encoding in any of the common encoding rules
//! (inner type constraints are such an example).

pub mod constraints;
pub mod encoding_rules;
pub mod error;
pub mod information_object;
pub mod parameterization;
pub mod types;
pub mod utils;

use std::collections::BTreeMap;

use constraints::Constraint;
use error::{GrammarError, GrammarErrorType};
use information_object::{
    InformationObjectClass, InformationObjectFieldReference, ObjectFieldIdentifier,
    ToplevelInformationDeclaration,
};
use parameterization::Parameterization;
use types::*;
use utils::*;

// Comment tokens
pub const BLOCK_COMMENT_START: &'static str = "/*";
pub const BLOCK_COMMENT_END: &'static str = "*/";
pub const LINE_COMMENT: &'static str = "--";

// Bracket tokens
pub const LEFT_PARENTHESIS: char = '(';
pub const RIGHT_PARENTHESIS: char = ')';
pub const LEFT_BRACKET: char = '[';
pub const RIGHT_BRACKET: char = ']';
pub const LEFT_BRACE: char = '{';
pub const RIGHT_BRACE: char = '}';
pub const LEFT_CHEVRON: char = '<';
pub const RIGHT_CHEVRON: char = '>';

// Type tokens
pub const NULL: &'static str = "NULL";
pub const BOOLEAN: &'static str = "BOOLEAN";
pub const INTEGER: &'static str = "INTEGER";
pub const REAL: &'static str = "REAL";
pub const BIT_STRING: &'static str = "BIT STRING";
pub const OCTET_STRING: &'static str = "OCTET STRING";
pub const IA5_STRING: &'static str = "IA5String";
pub const UTF8_STRING: &'static str = "UTF8String";
pub const NUMERIC_STRING: &'static str = "NumericString";
pub const VISIBLE_STRING: &'static str = "VisibleString";
pub const TELETEX_STRING: &'static str = "TeletexString";
pub const VIDEOTEX_STRING: &'static str = "VideotexString";
pub const GRAPHIC_STRING: &'static str = "GraphicString";
pub const GENERAL_STRING: &'static str = "GeneralString";
pub const UNIVERSAL_STRING: &'static str = "UniversalString";
pub const BMP_STRING: &'static str = "BMPString";
pub const PRINTABLE_STRING: &'static str = "PrintableString";
pub const ENUMERATED: &'static str = "ENUMERATED";
pub const CHOICE: &'static str = "CHOICE";
pub const SEQUENCE: &'static str = "SEQUENCE";
pub const OF: &'static str = "OF";
pub const ALL: &'static str = "ALL";
pub const SET: &'static str = "SET";
pub const SET_OF: &'static str = "SET OF";
pub const OBJECT_IDENTIFIER: &'static str = "OBJECT IDENTIFIER";

// Tagging tokens
pub const UNIVERSAL: &'static str = "UNIVERSAL";
pub const PRIVATE: &'static str = "PRIVATE";
pub const APPLICATION: &'static str = "APPLICATION";

// Value tokens
pub const TRUE: &'static str = "TRUE";
pub const FALSE: &'static str = "FALSE";

// Header tokens
pub const BEGIN: &'static str = "BEGIN";
pub const END: &'static str = "END";
pub const DEFINITIONS: &'static str = "DEFINITIONS";
pub const AUTOMATIC: &'static str = "AUTOMATIC";
pub const EXPLICIT: &'static str = "EXPLICIT";
pub const IMPLICIT: &'static str = "IMPLICIT";
pub const IMPORTS: &'static str = "IMPORTS";
pub const FROM: &'static str = "FROM";
pub const INSTRUCTIONS: &'static str = "INSTRUCTIONS";
pub const TAGS: &'static str = "TAGS";
pub const EXTENSIBILITY_IMPLIED: &'static str = "EXTENSIBILITY IMPLIED";
pub const WITH_SUCCESSORS: &'static str = "WITH SUCCESSORS";
pub const SEMICOLON: char = ';';

// Information Object Class tokens
pub const AMPERSAND: char = '&';
pub const CLASS: &'static str = "CLASS";
pub const UNIQUE: &'static str = "UNIQUE";
pub const WITH_SYNTAX: &'static str = "WITH SYNTAX";
pub const AT: char = '@';
pub const DOT: char = '.';

// Subtyping tokens
pub const SIZE: &'static str = "SIZE";
pub const CONSTRAINED_BY: &'static str = "CONSTRAINED BY";
pub const PATTERN: &'static str = "PATTERN";
pub const DEFAULT: &'static str = "DEFAULT";
pub const OPTIONAL: &'static str = "OPTIONAL";
pub const WITH_COMPONENTS: &'static str = "WITH COMPONENTS";
pub const WITH_COMPONENT: &'static str = "WITH COMPONENT";
pub const UNION: &'static str = "UNION";
pub const EXCEPT: &'static str = "EXCEPT";
pub const INTERSECTION: &'static str = "INTERSECTION";
pub const ABSENT: &'static str = "ABSENT";
pub const PRESENT: &'static str = "PRESENT";
pub const INCLUDES: &'static str = "INCLUDES";
pub const MIN: &'static str = "MIN";
pub const MAX: &'static str = "MAX";
pub const LESS_THAN: char = '<';
pub const GREATER_THAN: char = '>';
pub const PIPE: &'static str = "|";
pub const CARET: &'static str = "^";

pub const ASSIGN: &'static str = "::=";
pub const RANGE: &'static str = "..";
pub const ELLIPSIS: &'static str = "...";
pub const COMMA: char = ',';
pub const COLON: char = ':';
pub const SINGLE_QUOTE: char = '\'';

// invalid syntax word tokens
pub const ABSTRACT_SYNTAX: &'static str = "ABSTRACT-SYNTAX";
pub const BIT: &'static str = "BIT";
pub const CHARACTER: &'static str = "CHARACTER";
pub const CONTAINING: &'static str = "CONTAINING";
pub const DATE: &'static str = "DATE";
pub const DATE_TIME: &'static str = "DATE-TIME";
pub const DURATION: &'static str = "DURATION";
pub const EMBEDDED: &'static str = "EMBEDDED";
pub const EXTERNAL: &'static str = "EXTERNAL";
pub const INSTANCE: &'static str = "INSTANCE";
pub const MINUS_INFINITY: &'static str = "MINUS-INFINITY";
pub const NOT_A_NUMBER: &'static str = "NOT-A-NUMBER";
pub const OBJECT: &'static str = "OBJECT";
pub const OCTET: &'static str = "OCTET";
pub const OID_IRI: &'static str = "OID-IRI";
pub const PLUS_INFINITY: &'static str = "PLUS-INFINITY";
pub const RELATIVE_OID: &'static str = "RELATIVE-OID";
pub const RELATIVE_OID_IRI: &'static str = "RELATIVE-OID-IRI";
pub const TIME: &'static str = "TIME";
pub const TIME_OF_DAY: &'static str = "TIME-OF-DAY";
pub const TYPE_IDENTIFIER: &'static str = "TYPE-IDENTIFIER";

pub const ASN1_KEYWORDS: [&str; 63] = [
    ABSTRACT_SYNTAX,
    BIT,
    CHARACTER,
    CONTAINING,
    DATE,
    DATE_TIME,
    DURATION,
    EMBEDDED,
    EXTERNAL,
    INSTANCE,
    MINUS_INFINITY,
    NOT_A_NUMBER,
    OBJECT,
    OCTET,
    OID_IRI,
    PLUS_INFINITY,
    RELATIVE_OID,
    RELATIVE_OID_IRI,
    TIME,
    TIME_OF_DAY,
    TYPE_IDENTIFIER,
    SIZE,
    DEFAULT,
    OPTIONAL,
    WITH_COMPONENTS,
    WITH_COMPONENT,
    UNION,
    EXCEPT,
    INTERSECTION,
    ABSENT,
    PRESENT,
    INCLUDES,
    MIN,
    MAX,
    CLASS,
    UNIQUE,
    WITH_SYNTAX,
    NULL,
    BOOLEAN,
    INTEGER,
    REAL,
    ENUMERATED,
    CHOICE,
    SEQUENCE,
    OF,
    ALL,
    SET,
    OBJECT_IDENTIFIER,
    UNIVERSAL,
    PRIVATE,
    APPLICATION,
    TRUE,
    FALSE,
    BEGIN,
    END,
    DEFINITIONS,
    AUTOMATIC,
    EXPLICIT,
    IMPLICIT,
    IMPORTS,
    FROM,
    INSTRUCTIONS,
    TAGS,
];

#[derive(Debug, Clone, PartialEq)]
pub struct EncodingReferenceDefault(pub String);

impl From<&str> for EncodingReferenceDefault {
    fn from(value: &str) -> Self {
        Self(value.into())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TaggingEnvironment {
    Automatic,
    Implicit,
    Explicit,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExtensibilityEnvironment {
    Implied,
    Explicit,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Import {
    pub types: Vec<String>,
    pub origin_name: String,
    pub origin_identifier: ObjectIdentifierValue,
    pub with_successors: bool,
}

impl From<(Vec<&str>, (&str, ObjectIdentifierValue, Option<&str>))> for Import {
    fn from(value: (Vec<&str>, (&str, ObjectIdentifierValue, Option<&str>))) -> Self {
        Self {
            types: value.0.into_iter().map(|s| String::from(s)).collect(),
            origin_name: value.1 .0.into(),
            origin_identifier: value.1 .1,
            with_successors: value.1 .2.is_some(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ModuleReference {
    pub name: String,
    pub module_identifier: Option<ObjectIdentifierValue>,
    pub encoding_reference_default: Option<EncodingReferenceDefault>,
    pub tagging_environment: TaggingEnvironment,
    pub extensibility_environment: ExtensibilityEnvironment,
    pub imports: Vec<Import>,
}

impl
    From<(
        &str,
        Option<ObjectIdentifierValue>,
        (
            Option<EncodingReferenceDefault>,
            TaggingEnvironment,
            ExtensibilityEnvironment,
        ),
        Option<Vec<Import>>,
    )> for ModuleReference
{
    fn from(
        value: (
            &str,
            Option<ObjectIdentifierValue>,
            (
                Option<EncodingReferenceDefault>,
                TaggingEnvironment,
                ExtensibilityEnvironment,
            ),
            Option<Vec<Import>>,
        ),
    ) -> Self {
        Self {
            name: value.0.into(),
            module_identifier: value.1,
            encoding_reference_default: value.2 .0,
            tagging_environment: value.2 .1,
            extensibility_environment: value.2 .2,
            imports: value.3.unwrap_or(vec![]),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ObjectIdentifierValue(pub Vec<ObjectIdentifierArc>);

impl From<Vec<ObjectIdentifierArc>> for ObjectIdentifierValue {
    fn from(value: Vec<ObjectIdentifierArc>) -> Self {
        Self(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ObjectIdentifierArc {
    pub name: Option<String>,
    pub number: Option<u128>,
}

impl From<u128> for ObjectIdentifierArc {
    fn from(value: u128) -> Self {
        Self {
            name: None,
            number: Some(value),
        }
    }
}

impl From<&str> for ObjectIdentifierArc {
    fn from(value: &str) -> Self {
        Self {
            name: Some(value.into()),
            number: None,
        }
    }
}

impl From<(&str, u128)> for ObjectIdentifierArc {
    fn from(value: (&str, u128)) -> Self {
        Self {
            name: Some(value.0.into()),
            number: Some(value.1),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ToplevelDeclaration {
    Type(ToplevelTypeDeclaration),
    Value(ToplevelValueDeclaration),
    Information(ToplevelInformationDeclaration),
}

impl ToplevelDeclaration {
    pub fn apply_tagging_environment(&mut self, environment: &TaggingEnvironment) {
        match (environment, self) {
            (env, ToplevelDeclaration::Type(ty)) => {
                ty.tag = ty.tag.as_ref().map(|t| AsnTag {
                    environment: env.clone(),
                    tag_class: t.tag_class,
                    id: t.id,
                });
                match &mut ty.r#type {
                    ASN1Type::Sequence(s) | ASN1Type::Set(s) => {
                        s.members.iter_mut().for_each(|m| {
                            m.tag = m.tag.as_ref().map(|t| AsnTag {
                                environment: env.clone(),
                                tag_class: t.tag_class,
                                id: t.id,
                            });
                        })
                    }
                    ASN1Type::Choice(c) => c.options.iter_mut().for_each(|o| {
                        o.tag = o.tag.as_ref().map(|t| AsnTag {
                            environment: env.clone(),
                            tag_class: t.tag_class,
                            id: t.id,
                        });
                    }),
                    _ => (),
                }
            }
            _ => (),
        }
    }

    pub fn name(&self) -> &String {
        match self {
            ToplevelDeclaration::Information(i) => &i.name,
            ToplevelDeclaration::Type(t) => &t.name,
            ToplevelDeclaration::Value(v) => &v.name,
        }
    }

    pub(crate) fn get_distinguished_or_enum_value(
        &self,
        type_name: Option<&String>,
        identifier: &String,
    ) -> Option<ASN1Value> {
        if let ToplevelDeclaration::Type(t) = self {
            if type_name.is_some() && Some(&t.name) != type_name {
                return None;
            }
            match &t.r#type {
                ASN1Type::Enumerated(e) => {
                    return e.members.iter().find_map(|m| {
                        (&m.name == identifier).then(|| ASN1Value::Integer(m.index as i128))
                    })
                }
                ASN1Type::Integer(i) => {
                    return i
                        .distinguished_values
                        .as_ref()
                        .map(|dv| {
                            dv.iter().find_map(|d| {
                                (&d.name == identifier).then(|| ASN1Value::Integer(d.value))
                            })
                        })
                        .flatten()
                }
                _ => (),
            }
        }
        None
    }

    pub fn is_class_with_name(&self, name: &String) -> Option<&InformationObjectClass> {
        match self {
            ToplevelDeclaration::Information(info) => match &info.value {
                information_object::ASN1Information::ObjectClass(class) => {
                    (&info.name == name).then(|| class)
                }
                _ => None,
            },
            _ => None,
        }
    }

    /// Traverses a top-level declaration to check for references to other top-level declarations
    /// in a SEQUENCE's or SET's DEFAULT values.
    pub fn has_default_reference(&self) -> bool {
        match self {
            ToplevelDeclaration::Type(t) => match &t.r#type {
                ASN1Type::Sequence(s) | ASN1Type::Set(s) => {
                    s.members.iter().fold(false, |acc, m| {
                        acc || m
                            .default_value
                            .as_ref()
                            .map_or(false, |d| d.is_elsewhere_declared())
                    })
                }
                _ => false,
            },
            _ => false,
        }
    }

    /// Traverses a top-level declaration to replace references to other top-level declarations
    /// in a SEQUENCE's or SET's DEFAULT values.
    pub fn link_default_reference(&mut self, tlds: &BTreeMap<String, ToplevelDeclaration>) -> bool {
        match self {
            ToplevelDeclaration::Type(t) => match &mut t.r#type {
                ASN1Type::Sequence(s) | ASN1Type::Set(s) => {
                    s.members.iter_mut().fold(false, |acc, m| {
                        if let Some(default) = m.default_value.as_mut() {
                            let maybe_id = if let ASN1Value::ElsewhereDeclaredValue(id) = default {
                                Some(id.clone())
                            } else {
                                None
                            };
                            if let Some(ToplevelDeclaration::Value(id)) = tlds.get(&maybe_id.clone().unwrap_or_default()) {
                                *default = id.value.clone();
                                return true
                            }
                            let enumerated_id = match &m.r#type {
                                ASN1Type::Enumerated(_) => format!(
                                    "{}{}",
                                    to_rust_title_case(&t.name),
                                    to_rust_title_case(&m.name)
                                ),
                                ASN1Type::ElsewhereDeclaredType(e) => {
                                    if let Some(tld) = e.find_root_id(tlds) {
                                        tld.name().clone()
                                    } else {
                                        return acc;
                                    }
                                }
                                _ => return acc,
                            };
                            maybe_id.map_or(acc, |id| {
                                *default = ASN1Value::EnumeratedValue {
                                    enumerated: enumerated_id,
                                    enumerable: id,
                                };
                                true
                            })
                        } else {
                            acc
                        }
                    })
                }
                _ => false,
            },
            _ => false,
        }
    }

    /// Traverses a top-level declaration to check for references to other top-level declarations
    /// in a constraint. An example would be the constraint of the `intercontinental` field in the
    /// following example.
    /// ```ignore
    /// fifteen INTEGER = 15
    ///
    /// Departures ::= SEQUENCE {
    ///   local SEQUENCE (SIZE(0..999)) OF Local,
    ///   continental SEQUENCE (SIZE(0..99)) OF Continental,
    ///   intercontinental SEQUENCE (SIZE(0..fifteen)) OF Intercontinental
    /// }
    /// ```
    pub fn has_constraint_reference(&self) -> bool {
        match self {
            ToplevelDeclaration::Type(t) => t.r#type.contains_constraint_reference(),
            // TODO: Cover constraint references in other types of top-level declarations
            _ => false,
        }
    }

    /// Traverses a top-level declaration to replace references to other top-level declarations
    /// in a constraint. An example would be the constraint of the `intercontinental` field in the
    /// following example.
    /// ```ignore
    /// fifteen INTEGER = 15
    ///
    /// Departures ::= SEQUENCE {
    ///   local SEQUENCE (SIZE(0..999)) OF Local,
    ///   continental SEQUENCE (SIZE(0..99)) OF Continental,
    ///   intercontinental SEQUENCE (SIZE(0..fifteen)) OF Intercontinental
    /// }
    /// ```
    /// The method handles linking of multiple constraint references within a top-level declaration.
    /// ### Params
    ///  * `tlds` - vector of other top-level declarations that will be searched as the method resolves a reference
    /// returns `true` if the reference was resolved successfully.
    pub fn link_constraint_reference(
        &mut self,
        tlds: &BTreeMap<String, ToplevelDeclaration>,
    ) -> bool {
        match self {
            ToplevelDeclaration::Type(t) => t.r#type.link_constraint_reference(&t.name, tlds),
            // TODO: Cover constraint references in other types of top-level declarations
            _ => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ToplevelValueDeclaration {
    pub comments: String,
    pub name: String,
    pub type_name: String,
    pub value: ASN1Value,
}

impl From<(Vec<&str>, &str, &str, ASN1Value)> for ToplevelValueDeclaration {
    fn from(value: (Vec<&str>, &str, &str, ASN1Value)) -> Self {
        Self {
            comments: value.0.join("\n"),
            name: value.1.into(),
            type_name: value.2.into(),
            value: value.3,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ToplevelTypeDeclaration {
    pub comments: String,
    pub tag: Option<AsnTag>,
    pub name: String,
    pub r#type: ASN1Type,
    pub parameterization: Option<Parameterization>,
}

impl
    From<(
        Vec<&str>,
        &str,
        Option<Parameterization>,
        (Option<AsnTag>, ASN1Type),
    )> for ToplevelTypeDeclaration
{
    fn from(
        value: (
            Vec<&str>,
            &str,
            Option<Parameterization>,
            (Option<AsnTag>, ASN1Type),
        ),
    ) -> Self {
        Self {
            comments: value.0.join("\n"),
            name: value.1.into(),
            parameterization: value.2,
            r#type: value.3 .1,
            tag: value.3 .0,
        }
    }
}

/// The possible types of an ASN1 data element.
/// In addition, the `ElsewhereDeclaredType` enumeral denotes an type
/// specified in the same or an imported ASN1 specification.
#[derive(Debug, Clone, PartialEq)]
pub enum ASN1Type {
    Null,
    Boolean,
    Integer(Integer),
    Real(Real),
    BitString(BitString),
    OctetString(OctetString),
    CharacterString(CharacterString),
    Enumerated(Enumerated),
    Choice(Choice),
    Sequence(SequenceOrSet),
    SequenceOf(SequenceOf),
    Set(SequenceOrSet),
    // SetOf,
    ElsewhereDeclaredType(DeclarationElsewhere),
    ObjectIdentifier(ObjectIdentifier),
    InformationObjectFieldReference(InformationObjectFieldReference),
}

impl ASN1Type {
    pub fn constraints(&self) -> Vec<Constraint> {
        match self {
            ASN1Type::Integer(i) => i.constraints.clone(),
            ASN1Type::BitString(b) => b.constraints.clone(),
            ASN1Type::OctetString(o) => o.constraints.clone(),
            ASN1Type::CharacterString(c) => c.constraints.clone(),
            ASN1Type::Enumerated(e) => e.constraints.clone(),
            ASN1Type::Choice(c) => c.constraints.clone(),
            ASN1Type::Sequence(s) => s.constraints.clone(),
            ASN1Type::SequenceOf(s) => s.constraints.clone(),
            ASN1Type::ElsewhereDeclaredType(e) => e.constraints.clone(),
            _ => vec![],
        }
    }

    pub fn link_constraint_reference(
        &mut self,
        name: &String,
        tlds: &BTreeMap<String, ToplevelDeclaration>,
    ) -> bool {
        match self {
            ASN1Type::Null => false,
            ASN1Type::Boolean => false,
            ASN1Type::ObjectIdentifier(o) => o
            .constraints
            .iter_mut()
            .map(|c| c.link_cross_reference(name, tlds))
            .fold(false, |acc, b| acc || b),
            ASN1Type::Integer(i) => i
                .constraints
                .iter_mut()
                .map(|c| c.link_cross_reference(name, tlds))
                .fold(false, |acc, b| acc || b),
            ASN1Type::BitString(b) => b
                .constraints
                .iter_mut()
                .map(|c| c.link_cross_reference(name, tlds))
                .fold(false, |acc, b| acc || b),
            ASN1Type::OctetString(o) => o
                .constraints
                .iter_mut()
                .map(|c| c.link_cross_reference(name, tlds))
                .fold(false, |acc, b| acc || b),
            ASN1Type::CharacterString(c) => c
                .constraints
                .iter_mut()
                .map(|c| c.link_cross_reference(name, tlds))
                .fold(false, |acc, b| acc || b),
            ASN1Type::Enumerated(e) => e
                .constraints
                .iter_mut()
                .map(|c| c.link_cross_reference(name, tlds))
                .fold(false, |acc, b| acc || b),
            ASN1Type::Choice(c) => {
                c.constraints
                    .iter_mut()
                    .map(|c| c.link_cross_reference(name, tlds))
                    .fold(false, |acc, b| acc || b)
                    || c.options
                        .iter_mut()
                        .map(|o| {
                            let b = o.r#type.link_constraint_reference(&o.name, tlds);
                            let a = o
                                .constraints
                                .iter_mut()
                                .map(|c| c.link_cross_reference(name, tlds))
                                .fold(false, |acc, b| acc || b);
                            a || b
                        })
                        .fold(false, |acc, b| acc || b)
            }
            ASN1Type::Sequence(s) => {
                s.constraints
                    .iter_mut()
                    .map(|c| c.link_cross_reference(name, tlds))
                    .fold(false, |acc, b| acc || b)
                    || s.members
                        .iter_mut()
                        .map(|o| {
                            let b = o.r#type.link_constraint_reference(&o.name, tlds);
                            let a = o
                                .constraints
                                .iter_mut()
                                .map(|c| c.link_cross_reference(name, tlds))
                                .fold(false, |acc, b| acc || b);
                            a || b
                        })
                        .fold(false, |acc, b| acc || b)
            }
            ASN1Type::SequenceOf(s) => {
                let a = s
                    .constraints
                    .iter_mut()
                    .map(|c| c.link_cross_reference(name, tlds))
                    .fold(false, |acc, b| acc || b);
                let b = s.r#type.link_constraint_reference(name, tlds);
                a || b
            }
            ASN1Type::ElsewhereDeclaredType(e) => e
                .constraints
                .iter_mut()
                .map(|c| c.link_cross_reference(&e.identifier, tlds))
                .fold(false, |acc, b| acc || b),
            _ => false,
        }
    }

    pub fn contains_constraint_reference(&self) -> bool {
        match self {
            ASN1Type::Null => false,
            ASN1Type::Boolean => false,
            ASN1Type::ObjectIdentifier(o) => o.constraints.iter().any(|c| c.has_cross_reference()),
            ASN1Type::Integer(i) => i.constraints.iter().any(|c| c.has_cross_reference()),
            ASN1Type::BitString(b) => b.constraints.iter().any(|c| c.has_cross_reference()),
            ASN1Type::OctetString(o) => o.constraints.iter().any(|c| c.has_cross_reference()),
            ASN1Type::CharacterString(c) => c.constraints.iter().any(|c| c.has_cross_reference()),
            ASN1Type::Enumerated(e) => e.constraints.iter().any(|c| c.has_cross_reference()),
            ASN1Type::Choice(c) => {
                c.constraints.iter().any(|c| c.has_cross_reference())
                    || c.options.iter().any(|o| {
                        o.r#type.contains_constraint_reference()
                            || o.constraints.iter().any(|c| c.has_cross_reference())
                    })
            }
            ASN1Type::Sequence(s) => {
                s.constraints.iter().any(|c| c.has_cross_reference())
                    || s.members.iter().any(|m| {
                        m.r#type.contains_constraint_reference()
                            || m.constraints.iter().any(|c| c.has_cross_reference())
                    })
            }
            ASN1Type::SequenceOf(s) => {
                s.constraints.iter().any(|c| c.has_cross_reference())
                    || s.r#type.contains_constraint_reference()
            }
            ASN1Type::ElsewhereDeclaredType(e) => {
                e.constraints.iter().any(|c| c.has_cross_reference())
            }
            _ => false,
        }
    }

    pub fn contains_class_field_reference(&self) -> bool {
        match self {
            ASN1Type::Choice(c) => c
                .options
                .iter()
                .any(|o| o.r#type.contains_class_field_reference()),
            ASN1Type::Sequence(s) => s
                .members
                .iter()
                .any(|m| m.r#type.contains_class_field_reference()),
            ASN1Type::SequenceOf(so) => so.r#type.contains_class_field_reference(),
            ASN1Type::InformationObjectFieldReference(io_ref) => {
                if let Some(ObjectFieldIdentifier::SingleValue(_)) = io_ref.field_path.last() {
                    true
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    pub fn resolve_class_field_reference(
        self,
        tlds: &BTreeMap<String, ToplevelDeclaration>,
    ) -> Self {
        match self {
            ASN1Type::Choice(c) => ASN1Type::Choice(Choice {
                extensible: c.extensible,
                options: c
                    .options
                    .into_iter()
                    .map(|option| ChoiceOption {
                        name: option.name,
                        tag: option.tag,
                        r#type: option.r#type.resolve_class_field_reference(tlds),
                        constraints: vec![],
                    })
                    .collect(),
                constraints: c.constraints,
            }),
            ASN1Type::Sequence(s) => ASN1Type::Sequence(SequenceOrSet {
                extensible: s.extensible,
                constraints: s.constraints,
                members: s
                    .members
                    .into_iter()
                    .map(|mut member| {
                        member.constraints = vec![];
                        member.r#type = member.r#type.resolve_class_field_reference(tlds);
                        member
                    })
                    .collect(),
            }),
            ASN1Type::InformationObjectFieldReference(_) => self.reassign_type_for_ref(tlds),
            _ => self,
        }
    }

    fn reassign_type_for_ref(mut self, tlds: &BTreeMap<String, ToplevelDeclaration>) -> Self {
        if let Self::InformationObjectFieldReference(ref ior) = self {
            if let Some(t) = tlds
                .iter()
                .find_map(|(_, c)| {
                    c.is_class_with_name(&ior.class)
                        .map(|clazz| clazz.get_field(&ior.field_path))
                })
                .flatten()
                .map(|class_field| class_field.r#type.clone())
                .flatten()
            {
                self = t;
            }
        }
        self
    }

    pub fn link_subtype_constraint(
        &mut self,
        tlds: &BTreeMap<String, ToplevelDeclaration>,
    ) -> bool {
        match self {
            Self::ElsewhereDeclaredType(e) => {
                if let Some(ToplevelDeclaration::Type(t)) = tlds.get(&e.identifier) {
                    *self = t.r#type.clone();
                    return true;
                }
                false
            }
            _ => false,
        }
    }
}

impl ToString for ASN1Type {
    fn to_string(&self) -> String {
        match self {
            ASN1Type::Null => "Asn1Null".to_owned(),
            ASN1Type::Boolean => "bool".to_owned(),
            ASN1Type::Integer(i) => i.type_token(),
            ASN1Type::Real(_) => "f64".to_owned(),
            ASN1Type::BitString(_) => "BitVec".to_owned(),
            ASN1Type::OctetString(_) => "Bytes".to_owned(),
            ASN1Type::CharacterString(_) => "String".to_owned(),
            ASN1Type::Enumerated(_) => todo!(),
            ASN1Type::Choice(_) => todo!(),
            ASN1Type::Sequence(_) => todo!(),
            ASN1Type::SequenceOf(_) => todo!(),
            ASN1Type::ObjectIdentifier(_) => todo!(),
            ASN1Type::Set(_) => todo!(),
            ASN1Type::ElsewhereDeclaredType(e) => e.identifier.clone(),
            ASN1Type::InformationObjectFieldReference(_) => todo!(),
        }
    }
}

pub const NUMERIC_STRING_CHARSET: [char; 11] =
    [' ', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
pub const PRINTABLE_STRING_CHARSET: [char; 74] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
    'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4',
    '5', '6', '7', '8', '9', ' ', '\'', '(', ')', '+', ',', '-', '.', '/', ':', '=', '?',
];

/// The types of an ASN1 character strings.
#[derive(Debug, Clone, PartialEq, Copy)]
pub enum CharacterStringType {
    NumericString,
    VisibleString,
    IA5String,
    TeletexString,
    VideotexString,
    GraphicString,
    GeneralString,
    UniversalString,
    UTF8String,
    BMPString,
    PrintableString,
}

impl CharacterStringType {
    pub fn character_set(&self) -> BTreeMap<usize, char> {
        match self {
            CharacterStringType::NumericString => {
                NUMERIC_STRING_CHARSET.into_iter().enumerate().collect()
            }
            CharacterStringType::VisibleString | CharacterStringType::PrintableString => {
                PRINTABLE_STRING_CHARSET.into_iter().enumerate().collect()
            }
            CharacterStringType::IA5String => (0..128u32)
                .into_iter()
                .map(|i| char::from_u32(i).unwrap())
                .enumerate()
                .collect(),
            _ => (0..u16::MAX as u32)
                .into_iter()
                .filter_map(|i| char::from_u32(i))
                .enumerate()
                .collect(),
        }
    }
}

impl From<&str> for CharacterStringType {
    fn from(value: &str) -> Self {
        match value {
            IA5_STRING => Self::IA5String,
            NUMERIC_STRING => Self::NumericString,
            VISIBLE_STRING => Self::VisibleString,
            TELETEX_STRING => Self::TeletexString,
            VIDEOTEX_STRING => Self::VideotexString,
            GRAPHIC_STRING => Self::GraphicString,
            GENERAL_STRING => Self::GeneralString,
            UNIVERSAL_STRING => Self::UniversalString,
            BMP_STRING => Self::BMPString,
            PRINTABLE_STRING => Self::PrintableString,
            _ => Self::UTF8String,
        }
    }
}

/// The possible types of an ASN1 value.
#[derive(Debug, Clone, PartialEq)]
pub enum ASN1Value {
    All,
    Null,
    Boolean(bool),
    Choice(String, Box<ASN1Value>),
    Sequence(Vec<(String, Box<ASN1Value>)>),
    Integer(i128),
    Real(f64),
    String(String),
    BitString(Vec<bool>),
    EnumeratedValue {
        enumerated: String,
        enumerable: String,
    },
    ElsewhereDeclaredValue(String),
    ObjectIdentifier(ObjectIdentifierValue),
}

impl ASN1Value {
    pub fn max(
        &self,
        other: &ASN1Value,
        char_set: Option<&BTreeMap<usize, char>>,
    ) -> Result<ASN1Value, GrammarError> {
        self.min_max(other, char_set, false)
    }

    pub fn min(
        &self,
        other: &ASN1Value,
        char_set: Option<&BTreeMap<usize, char>>,
    ) -> Result<ASN1Value, GrammarError> {
        self.min_max(other, char_set, true)
    }

    fn min_max(
        &self,
        other: &ASN1Value,
        char_set: Option<&BTreeMap<usize, char>>,
        getting_mininum: bool,
    ) -> Result<ASN1Value, GrammarError> {
        match (self, other, char_set) {
            (ASN1Value::Integer(s), ASN1Value::Integer(o), _) => {
                if getting_mininum {
                    Ok(ASN1Value::Integer(*s.min(o)))
                } else {
                    Ok(ASN1Value::Integer(*s.max(o)))
                }
            }
            (ASN1Value::String(s), ASN1Value::String(o), Some(set)) => {
                if s.len() != 1 || o.len() != 1 {
                    return Err(GrammarError {
                        details: format!(
                            "Unsupported operation for ASN1Values {:?} and {:?}",
                            self, other
                        ),
                        kind: GrammarErrorType::UnpackingError,
                    });
                }
                let s_as_char = s.chars().next().unwrap();
                let o_as_char = o.chars().next().unwrap();
                match (
                    set.iter().find(|(_, c)| s_as_char == **c),
                    set.iter().find(|(_, c)| o_as_char == **c),
                ) {
                    (Some((self_i, _)), Some((other_i, _))) => {
                        let return_self = if getting_mininum {
                            self_i <= other_i
                        } else {
                            self_i >= other_i
                        };
                        if return_self {
                            Ok(self.clone())
                        } else {
                            Ok(other.clone())
                        }
                    }
                    _ => Err(GrammarError {
                        details: format!(
                            "Failed to find ASN1Values {:?} and {:?} in character set {:?}",
                            self, other, char_set
                        ),
                        kind: GrammarErrorType::UnpackingError,
                    }),
                }
            }
            _ => Err(GrammarError {
                details: format!(
                    "Unsupported operation for ASN1Values {:?} and {:?}",
                    self, other
                ),
                kind: GrammarErrorType::UnpackingError,
            }),
        }
    }

    pub fn unwrap_as_integer(&self) -> Result<i128, GrammarError> {
        if let ASN1Value::Integer(i) = self {
            Ok(*i)
        } else {
            Err(GrammarError {
                details: format!("Cannot unwrap {:?} as integer!", self),
                kind: error::GrammarErrorType::UnpackingError,
            })
        }
    }

    pub fn is_elsewhere_declared(&self) -> bool {
        match self {
            Self::ElsewhereDeclaredValue(_)
            | Self::EnumeratedValue {
                enumerated: _,
                enumerable: _,
            } => true,
            _ => false,
        }
    }

    pub fn link_elsewhere_declared(
        &mut self,
        identifier: &String,
        tlds: &BTreeMap<String, ToplevelDeclaration>,
    ) -> bool {
        match self {
            Self::EnumeratedValue {
                enumerated: _,
                enumerable: e,
            }
            | Self::ElsewhereDeclaredValue(e) => {
                if let Some(v) = find_tld_or_enum_value_by_name(identifier, &e, tlds) {
                    *self = v;
                    return true;
                }
                false
            }
            _ => false,
        }
    }

    pub fn value_as_string(&self, type_name: Option<&str>) -> Result<String, GrammarError> {
        match self {
            ASN1Value::All => Ok("Asn1All".to_owned()),
            ASN1Value::Null => Ok("Asn1Null".to_owned()),
            ASN1Value::Choice(i, v) => {
                if let Some(ty_n) = type_name {
                    Ok(format!("{ty_n}::{i}({})", v.value_as_string(None)?))
                } else {
                    Err(GrammarError {
                        details: format!(
                            "A type name is needed to stringify choice value {:?}",
                            self
                        ),
                        kind: GrammarErrorType::UnpackingError,
                    })
                }
            }
            ASN1Value::Sequence(fields) => {
                if let Some(ty_n) = type_name {
                    let stringified_fields = fields
                        .iter()
                        .map(|(id, val)| val.value_as_string(None).map(|s| format!("{id}: {s}")))
                        .collect::<Result<Vec<String>, _>>()?
                        .join(", ");
                    Ok(format!("{ty_n} {{ {stringified_fields} }}"))
                } else {
                    Err(GrammarError {
                        details: format!(
                            "A type name is needed to stringify choice value {:?}",
                            self
                        ),
                        kind: GrammarErrorType::UnpackingError,
                    })
                }
            }
            ASN1Value::Boolean(b) => Ok(format!("{}", b)),
            ASN1Value::Integer(i) => Ok(format!("{}", i)),
            ASN1Value::String(s) => Ok(s.clone()),
            ASN1Value::Real(r) => Ok(format!("{}", r)),
            ASN1Value::BitString(b) => {
                let mut bits = b.iter().fold(String::new(), |mut acc, bit| {
                    if *bit {
                        acc.push_str("true,");
                    } else {
                        acc.push_str("false,");
                    }
                    acc
                });
                // remove the last comma
                bits.pop();
                Ok(format!("[{bits}].into_iter().collect()"))
            }
            ASN1Value::EnumeratedValue {
                enumerated,
                enumerable,
            } => Ok(format!(
                "{}::{}",
                to_rust_title_case(enumerated),
                to_rust_title_case(enumerable)
            )),
            ASN1Value::ElsewhereDeclaredValue(e) => Ok(to_rust_const_case(e)),
            ASN1Value::ObjectIdentifier(oid) => Ok(format!(
                "[{}]",
                oid.0
                    .iter()
                    .filter_map(|arc| arc.number.map(|id| id.to_string()))
                    .collect::<Vec<String>>()
                    .join(",")
            )),
        }
    }
}

/// Intermediate placeholder for a type declared in
/// some other part of the ASN1 specification that is
/// being parsed or in one of its imports.
#[derive(Debug, Clone, PartialEq)]
pub struct DeclarationElsewhere {
    pub identifier: String,
    pub constraints: Vec<Constraint>,
}

impl From<(&str, Option<Vec<Constraint>>)> for DeclarationElsewhere {
    fn from(value: (&str, Option<Vec<Constraint>>)) -> Self {
        DeclarationElsewhere {
            identifier: value.0.into(),
            constraints: value.1.unwrap_or(vec![]),
        }
    }
}

impl DeclarationElsewhere {
    pub fn find_root_id<'a>(
        &self,
        tlds: &'a BTreeMap<String, ToplevelDeclaration>,
    ) -> Option<&'a ToplevelDeclaration> {
        if let Some(ToplevelDeclaration::Type(ToplevelTypeDeclaration {
            comments: _,
            tag: _,
            name: _,
            r#type: ASN1Type::ElsewhereDeclaredType(e),
            parameterization: _,
        })) = tlds.get(&self.identifier)
        {
            e.find_root_id(tlds)
        } else {
            tlds.get(&self.identifier)
        }
    }
}

/// Tag classes
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TagClass {
    Universal,
    Application,
    Private,
    ContextSpecific,
}

/// Representation of a tag
#[derive(Debug, Clone, PartialEq)]
pub struct AsnTag {
    pub environment: TaggingEnvironment,
    pub tag_class: TagClass,
    pub id: u64,
}

impl From<(Option<&str>, u64)> for AsnTag {
    fn from(value: (Option<&str>, u64)) -> Self {
        let tag_class = match value.0 {
            Some("APPLICATION") => TagClass::Application,
            Some("UNIVERSAL") => TagClass::Universal,
            Some("PRIVATE") => TagClass::Private,
            _ => TagClass::ContextSpecific,
        };
        AsnTag {
            tag_class,
            id: value.1,
            environment: TaggingEnvironment::Automatic,
        }
    }
}
