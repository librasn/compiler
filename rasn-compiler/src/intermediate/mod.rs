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

use std::{
    collections::BTreeMap,
    ops::{Add, AddAssign},
    rc::Rc, f32::consts::E,
};

use constraints::Constraint;
use error::{GrammarError, GrammarErrorType};
use information_object::{InformationObjectFieldReference, ToplevelInformationDeclaration};
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
pub const GENERALIZED_TIME: &'static str = "GeneralizedTime";
pub const UTC_TIME: &'static str = "UTCTime";
pub const ENUMERATED: &'static str = "ENUMERATED";
pub const CHOICE: &'static str = "CHOICE";
pub const SEQUENCE: &'static str = "SEQUENCE";
pub const OF: &'static str = "OF";
pub const ALL: &'static str = "ALL";
pub const SET: &'static str = "SET";
pub const OBJECT_IDENTIFIER: &'static str = "OBJECT IDENTIFIER";
pub const COMPONENTS_OF: &'static str = "COMPONENTS OF";

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
pub const EXPORTS: &'static str = "EXPORTS";
pub const FROM: &'static str = "FROM";
pub const INSTRUCTIONS: &'static str = "INSTRUCTIONS";
pub const TAGS: &'static str = "TAGS";
pub const EXTENSIBILITY_IMPLIED: &'static str = "EXTENSIBILITY IMPLIED";
pub const WITH_SUCCESSORS: &'static str = "WITH SUCCESSORS";
pub const WITH_DESCENDANTS: &'static str = "WITH DESCENDANTS";
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
pub const EMBEDDED_PDV: &'static str = "EMBEDDED PDV";
pub const EXTERNAL: &'static str = "EXTERNAL";
pub const INSTANCE_OF: &'static str = "INSTANCE OF";
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
    EMBEDDED_PDV,
    EXTERNAL,
    INSTANCE_OF,
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

macro_rules! error {
    ($kind:ident, $($arg:tt)*) => {
        GrammarError {
            details: format!($($arg)*),
            kind: GrammarErrorType::$kind,
        }
    };
}

#[derive(Debug, Clone, PartialEq)]
pub struct EncodingReferenceDefault(pub String);

impl From<&str> for EncodingReferenceDefault {
    fn from(value: &str) -> Self {
        Self(value.into())
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TaggingEnvironment {
    Automatic,
    Implicit,
    Explicit,
}

impl Add<&TaggingEnvironment> for &TaggingEnvironment {
    type Output = TaggingEnvironment;

    fn add(self, rhs: &TaggingEnvironment) -> Self::Output {
        match (self, rhs) {
            (t, TaggingEnvironment::Automatic) => *t,
            (_, t) => *t,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExtensibilityEnvironment {
    Implied,
    Explicit,
}

#[derive(Debug, Clone, PartialEq)]
pub enum With {
    Successors,
    Descendants,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Import {
    pub types: Vec<String>,
    pub origin_name: String,
    pub origin_identifier: Option<ObjectIdentifierValue>,
    pub with: Option<With>,
}

impl
    From<(
        Vec<&str>,
        (&str, Option<ObjectIdentifierValue>, Option<&str>),
    )> for Import
{
    fn from(
        value: (
            Vec<&str>,
            (&str, Option<ObjectIdentifierValue>, Option<&str>),
        ),
    ) -> Self {
        Self {
            types: value.0.into_iter().map(|s| String::from(s)).collect(),
            origin_name: value.1 .0.into(),
            origin_identifier: value.1 .1,
            with: value.1 .2.map(|with| {
                if with == WITH_SUCCESSORS {
                    With::Successors
                } else {
                    With::Descendants
                }
            }),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Exports {
    Identifier(Vec<String>),
    All,
}

impl From<Vec<&str>> for Exports {
    fn from(value: Vec<&str>) -> Self {
        Self::Identifier(value.iter().map(ToString::to_string).collect())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum DefinitiveIdentifier {
    DefinitiveOID(ObjectIdentifierValue),
    DefinitiveOIDandIRI {
        oid: ObjectIdentifierValue,
        iri: String,
    },
}

impl From<(ObjectIdentifierValue, Option<&str>)> for DefinitiveIdentifier {
    fn from(value: (ObjectIdentifierValue, Option<&str>)) -> Self {
        if let Some(iri_value) = value.1 {
            Self::DefinitiveOIDandIRI {
                oid: value.0,
                iri: iri_value.to_owned(),
            }
        } else {
            Self::DefinitiveOID(value.0)
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ModuleReference {
    pub name: String,
    pub module_identifier: Option<DefinitiveIdentifier>,
    pub encoding_reference_default: Option<EncodingReferenceDefault>,
    pub tagging_environment: TaggingEnvironment,
    pub extensibility_environment: ExtensibilityEnvironment,
    pub imports: Vec<Import>,
    pub exports: Option<Exports>,
}

impl ModuleReference {
    pub fn print(&self) -> String {
        format!(
            r#"
            // =====================================================
            // {name}
            // {oid}
            // =====================================================
            "#,
            name = self.name.clone(),
            oid = self
                .module_identifier
                .as_ref()
                .map(|oid| {
                    match oid {
                        DefinitiveIdentifier::DefinitiveOID(id)
                        | DefinitiveIdentifier::DefinitiveOIDandIRI { oid: id, iri: _ } => {
                            format!(
                                "{{ {} }}",
                                id.0.iter()
                                    .map(|arc| match (arc.name.clone(), arc.number) {
                                        (Some(name), Some(no)) => format!("{name}({no})"),
                                        (Some(name), None) => format!("{name}"),
                                        (None, Some(no)) => format!("{no}"),
                                        _ => Default::default(),
                                    })
                                    .collect::<Vec<String>>()
                                    .join(" ")
                            )
                        }
                    }
                })
                .unwrap_or_default()
        )
    }
}

impl
    From<(
        &str,
        Option<DefinitiveIdentifier>,
        Option<(
            Option<EncodingReferenceDefault>,
            TaggingEnvironment,
            ExtensibilityEnvironment,
        )>,
        Option<Exports>,
        Option<Vec<Import>>,
    )> for ModuleReference
{
    fn from(
        value: (
            &str,
            Option<DefinitiveIdentifier>,
            Option<(
                Option<EncodingReferenceDefault>,
                TaggingEnvironment,
                ExtensibilityEnvironment,
            )>,
            Option<Exports>,
            Option<Vec<Import>>,
        ),
    ) -> Self {
        let (encoding_reference_default, tagging_environment, extensibility_environment) =
            value.2.unwrap_or((
                None,
                TaggingEnvironment::Explicit,
                ExtensibilityEnvironment::Explicit,
            ));
        Self {
            name: value.0.into(),
            module_identifier: value.1,
            encoding_reference_default,
            tagging_environment,
            extensibility_environment,
            exports: value.3,
            imports: value.4.unwrap_or_default(),
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
    pub(crate) fn has_enum_value(&self, type_name: Option<&String>, identifier: &String) -> bool {
        if let ToplevelDeclaration::Type(ToplevelTypeDeclaration {
            name,
            r#type: ASN1Type::Enumerated(e),
            ..
        }) = self
        {
            if type_name.is_some() && Some(name) != type_name {
                return false;
            }
            e.members.iter().any(|m| &m.name == identifier)
        } else {
            false
        }
    }

    pub(crate) fn get_distinguished_value(
        &self,
        type_name: Option<&String>,
        identifier: &String,
    ) -> Option<i128> {
        if let ToplevelDeclaration::Type(ToplevelTypeDeclaration {
            name,
            r#type: ASN1Type::Integer(i),
            ..
        }) = self
        {
            if type_name.is_some() && Some(name) != type_name {
                return None;
            }
            i.distinguished_values
                .as_ref()
                .and_then(|dv| {
                    dv.iter()
                        .find_map(|d| (&d.name == identifier).then_some(d.value.clone()))
                })
        } else {
            None
        }
    }

    pub fn set_index(&mut self, module_reference: Rc<ModuleReference>, item_no: usize) {
        match self {
            ToplevelDeclaration::Type(ref mut t) => {
                t.index = Some((module_reference, item_no));
            }
            ToplevelDeclaration::Value(ref mut v) => {
                v.index = Some((module_reference, item_no));
            }
            ToplevelDeclaration::Information(ref mut i) => {
                i.index = Some((module_reference, item_no));
            }
        }
    }

    pub fn get_index(&self) -> Option<&(Rc<ModuleReference>, usize)> {
        match self {
            ToplevelDeclaration::Type(ref t) => t.index.as_ref(),
            ToplevelDeclaration::Value(ref v) => v.index.as_ref(),
            ToplevelDeclaration::Information(ref i) => i.index.as_ref(),
        }
    }

    pub fn apply_tagging_environment(&mut self, environment: &TaggingEnvironment) {
        if let (env, ToplevelDeclaration::Type(ty)) = (environment, self) {
            ty.tag = ty.tag.as_ref().map(|t| AsnTag {
                environment: env + &t.environment,
                tag_class: t.tag_class,
                id: t.id,
            });
            match &mut ty.r#type {
                ASN1Type::Sequence(s) | ASN1Type::Set(s) => s.members.iter_mut().for_each(|m| {
                    m.tag = m.tag.as_ref().map(|t| AsnTag {
                        environment: env + &t.environment,
                        tag_class: t.tag_class,
                        id: t.id,
                    });
                }),
                ASN1Type::Choice(c) => c.options.iter_mut().for_each(|o| {
                    o.tag = o.tag.as_ref().map(|t| AsnTag {
                        environment: env + &t.environment,
                        tag_class: t.tag_class,
                        id: t.id,
                    });
                }),
                _ => (),
            }
        }
    }

    pub fn name(&self) -> &String {
        match self {
            ToplevelDeclaration::Information(i) => &i.name,
            ToplevelDeclaration::Type(t) => &t.name,
            ToplevelDeclaration::Value(v) => &v.name,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ToplevelValueDeclaration {
    pub comments: String,
    pub name: String,
    pub associated_type: String,
    pub value: ASN1Value,
    pub index: Option<(Rc<ModuleReference>, usize)>,
}

impl From<(Vec<&str>, &str, &str, ASN1Value)> for ToplevelValueDeclaration {
    fn from(value: (Vec<&str>, &str, &str, ASN1Value)) -> Self {
        Self {
            comments: value.0.join("\n"),
            name: value.1.into(),
            associated_type: value.2.into(),
            value: value.3,
            index: None,
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
    pub index: Option<(Rc<ModuleReference>, usize)>,
}

impl ToplevelTypeDeclaration {
    pub fn pdu(&self) -> &ASN1Type {
        &self.r#type
    }
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
            index: None,
        }
    }
}

/// The possible types of an ASN1 data element.
/// In addition, the `ElsewhereDeclaredType` enumeral denotes an type
/// specified in the same or an imported ASN1 specification.
#[derive(Debug, Clone, PartialEq)]
pub enum ASN1Type {
    Null,
    Boolean(Boolean),
    Integer(Integer),
    Real(Real),
    BitString(BitString),
    OctetString(OctetString),
    CharacterString(CharacterString),
    Enumerated(Enumerated),
    Choice(Choice),
    Sequence(SequenceOrSet),
    SequenceOf(SequenceOrSetOf),
    Set(SequenceOrSet),
    SetOf(SequenceOrSetOf),
    Time(Time),
    GeneralizedTime(GeneralizedTime),
    UTCTime(UTCTime),
    ElsewhereDeclaredType(DeclarationElsewhere),
    ChoiceSelectionType(ChoiceSelectionType),
    ObjectIdentifier(ObjectIdentifier),
    InformationObjectFieldReference(InformationObjectFieldReference),
    EmbeddedPdv,
    External,
}

impl ASN1Type {
    pub fn constraints(&self) -> Option<&Vec<Constraint>> {
        match self {
            ASN1Type::Boolean(b) => Some(b.constraints()),
            ASN1Type::Real(r) => Some(r.constraints()),
            ASN1Type::Integer(i) => Some(i.constraints()),
            ASN1Type::BitString(b) => Some(b.constraints()),
            ASN1Type::OctetString(o) => Some(o.constraints()),
            ASN1Type::CharacterString(c) => Some(c.constraints()),
            ASN1Type::Enumerated(e) => Some(e.constraints()),
            ASN1Type::Time(t) => Some(t.constraints()),
            ASN1Type::Choice(c) => Some(c.constraints()),
            ASN1Type::Set(s) | ASN1Type::Sequence(s) => Some(s.constraints()),
            ASN1Type::SetOf(s) | ASN1Type::SequenceOf(s) => Some(s.constraints()),
            ASN1Type::ElsewhereDeclaredType(e) => Some(e.constraints()),
            ASN1Type::InformationObjectFieldReference(f) => Some(f.constraints()),
            _ => None,
        }
    }

    pub fn constraints_mut(&mut self) -> Option<&mut Vec<Constraint>> {
        match self {
            ASN1Type::Boolean(b) => Some(b.constraints_mut()),
            ASN1Type::Real(r) => Some(r.constraints_mut()),
            ASN1Type::Integer(i) => Some(i.constraints_mut()),
            ASN1Type::BitString(b) => Some(b.constraints_mut()),
            ASN1Type::OctetString(o) => Some(o.constraints_mut()),
            ASN1Type::CharacterString(c) => Some(c.constraints_mut()),
            ASN1Type::Enumerated(e) => Some(e.constraints_mut()),
            ASN1Type::Time(t) => Some(t.constraints_mut()),
            ASN1Type::Choice(c) => Some(c.constraints_mut()),
            ASN1Type::Set(s) | ASN1Type::Sequence(s) => Some(s.constraints_mut()),
            ASN1Type::SetOf(s) | ASN1Type::SequenceOf(s) => Some(s.constraints_mut()),
            ASN1Type::ElsewhereDeclaredType(e) => Some(e.constraints_mut()),
            ASN1Type::InformationObjectFieldReference(f) => Some(f.constraints_mut()),
            _ => None,
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

/// Representation of common integer types
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IntegerType {
    Int8,
    Uint8,
    Int16,
    Uint16,
    Int32,
    Uint32,
    Int64,
    Uint64,
    Unbounded,
}

impl IntegerType {
    pub fn is_unbounded(&self) -> bool {
        self == &IntegerType::Unbounded
    }
    /// Returns the Integer type with more restrictions
    /// - an IntegerType with a smaller set of values is considered more restrictive
    /// - an unsigned IntegerType is considered more restrictive if the size of the set of values is equal
    /// if equal, `self` is returned
    pub fn max_restrictive(self, rhs: IntegerType) -> IntegerType {
        match (self, rhs) {
            (x, y) if x == y => x,
            (IntegerType::Uint8, _) | (_, IntegerType::Uint8) => IntegerType::Uint8,
            (IntegerType::Int8, _) | (_, IntegerType::Int8) => IntegerType::Int8,
            (IntegerType::Uint16, _) | (_, IntegerType::Uint16) => IntegerType::Uint16,
            (IntegerType::Int16, _) | (_, IntegerType::Int16) => IntegerType::Int16,
            (IntegerType::Uint32, _) | (_, IntegerType::Uint32) => IntegerType::Uint32,
            (IntegerType::Int32, _) | (_, IntegerType::Int32) => IntegerType::Int32,
            (IntegerType::Uint64, _) | (_, IntegerType::Uint64) => IntegerType::Uint64,
            (IntegerType::Int64, _) | (_, IntegerType::Int64) => IntegerType::Int64,
            _ => IntegerType::Unbounded,
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
    SequenceOrSet(Vec<(String, Box<ASN1Value>)>),
    SequenceOrSetOf(Vec<ASN1Value>),
    Integer(i128),
    Real(f64),
    String(String),
    BitString(Vec<bool>),
    OctetString(Vec<u8>),
    EnumeratedValue {
        enumerated: String,
        enumerable: String,
    },
    Time(String),
    ElsewhereDeclaredValue {
        parent: Option<String>,
        identifier: String,
    },
    ObjectIdentifier(ObjectIdentifierValue),
    /// In ASN1 value declarations, the value type is not straighforward to parse.
    /// For example, in the following ASN1
    /// ```ignore
    /// ExampleInt ::= INTEGER
    /// ExampleSubset ::= ExampleInt (1..500)
    /// AnotherSubset ::= ExampleSubset (2..200)
    /// ExampleSet ::= SET {
    ///     int AnotherSubset DEFAULT 3
    /// }
    /// ```
    /// the relation of the default value to `ExampleSubset` will not be picked up by the parser.
    /// However, in some representations, this relation is critical information.  
    LinkedNestedValue {
        /// typereferences of supertypes
        supertypes: Vec<String>,
        value: Box<ASN1Value>,
    },
    /// Integer values need type information that will not always be picked up by the parser on first pass.
    LinkedIntValue {
        integer_type: IntegerType,
        value: i128,
    },
    /// Struct-like values such as SEQUENCE values need type information that will not always be picked up by the parser on first pass.
    LinkedStructLikeValue(Vec<(String, StructLikeFieldValue)>),
    /// Character string values such as UTF8String values need type information that will not always be picked up by the parser on first pass.
    LinkedCharStringValue(CharacterStringType, String),
    LinkedElsewhereDefinedValue{
        parent: Option<String>,
        identifier: String,
        can_be_const: bool,
    },
}

/// Representation of a field value of a struct-like ASN1 value
#[derive(Debug, Clone, PartialEq)]
pub enum StructLikeFieldValue {
    Explicit(Box<ASN1Value>),
    Implicit(Box<ASN1Value>)
}

impl StructLikeFieldValue {
    pub fn into_value(self) -> ASN1Value {
        match self {
            StructLikeFieldValue::Explicit(v) |
            StructLikeFieldValue::Implicit(v) => *v,
        }
    }

    pub fn value(&self) -> &ASN1Value {
        match self {
            StructLikeFieldValue::Explicit(v) |
            StructLikeFieldValue::Implicit(v) => &*v,
        }
    }
}

impl AsMut<ASN1Value> for ASN1Value {
    fn as_mut(&mut self) -> &mut ASN1Value {
        self
    }
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
                    return Err(error!(
                        UnpackingError,
                        "Unsupported operation for ASN1Values {self:?} and {other:?}"
                    ));
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
                    _ => Err(error!(
                            UnpackingError,
                            "Failed to find ASN1Values {self:?} and {other:?} in character set {char_set:?}",
                        )),
                }
            }
            _ => Err(error!(
                UnpackingError,
                "Unsupported operation for ASN1Values {self:?} and {other:?}",
            )),
        }
    }

    pub fn unwrap_as_integer(&self) -> Result<i128, GrammarError> {
        if let ASN1Value::Integer(i) = self {
            Ok(*i)
        } else {
            Err(error!(UnpackingError, "Cannot unwrap {self:?} as integer!"))
        }
    }
}

/// Intermediate placeholder for a type declared in
/// some other part of the ASN1 specification that is
/// being parsed or in one of its imports.
#[derive(Debug, Clone, PartialEq)]
pub struct DeclarationElsewhere {
    /// Chain of parent declaration leading back to a basic ASN1 type
    pub parent: Option<String>,
    pub identifier: String,
    pub constraints: Vec<Constraint>,
}

impl From<(Option<&str>, &str, Option<Vec<Constraint>>)> for DeclarationElsewhere {
    fn from(value: (Option<&str>, &str, Option<Vec<Constraint>>)) -> Self {
        DeclarationElsewhere {
            parent: value.0.map(ToString::to_string),
            identifier: value.1.into(),
            constraints: value.2.unwrap_or(vec![]),
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

impl From<((Option<&str>, u64), Option<TaggingEnvironment>)> for AsnTag {
    fn from(value: ((Option<&str>, u64), Option<TaggingEnvironment>)) -> Self {
        let tag_class = match value.0 .0 {
            Some("APPLICATION") => TagClass::Application,
            Some("UNIVERSAL") => TagClass::Universal,
            Some("PRIVATE") => TagClass::Private,
            _ => TagClass::ContextSpecific,
        };
        AsnTag {
            tag_class,
            id: value.0 .1,
            environment: value.1.unwrap_or(TaggingEnvironment::Automatic),
        }
    }
}
