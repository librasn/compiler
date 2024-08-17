//! The `intermediate` module provides an intermediate representation for ASN.1 notation.
//! It includes constants for the various ASN.1 keywords and types to represent the
//! single ASN.1 data elements in an intermediate representation from which the
//! generator module produces bindings.
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

use std::{borrow::Cow, cell::RefCell, collections::BTreeMap, ops::Add, rc::Rc};

use crate::{common::INTERNAL_IO_FIELD_REF_TYPE_NAME_PREFIX, lexer::Span};
use constraints::Constraint;
use error::{GrammarError, GrammarErrorType};
use information_object::{InformationObjectFieldReference, ToplevelInformationDefinition};
#[cfg(test)]
use internal_macros::EnumDebug;
use parameterization::Parameterization;
use quote::{quote, ToTokens, TokenStreamExt};
use types::*;

// Comment tokens
pub const BLOCK_COMMENT_START: &str = "/*";
pub const BLOCK_COMMENT_END: &str = "*/";
pub const LINE_COMMENT: &str = "--";

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
pub const NULL: &str = "NULL";
pub const BOOLEAN: &str = "BOOLEAN";
pub const INTEGER: &str = "INTEGER";
pub const REAL: &str = "REAL";
pub const BIT_STRING: &str = "BIT STRING";
pub const OCTET_STRING: &str = "OCTET STRING";
pub const IA5_STRING: &str = "IA5String";
pub const UTF8_STRING: &str = "UTF8String";
pub const NUMERIC_STRING: &str = "NumericString";
pub const VISIBLE_STRING: &str = "VisibleString";
pub const TELETEX_STRING: &str = "TeletexString";
pub const VIDEOTEX_STRING: &str = "VideotexString";
pub const GRAPHIC_STRING: &str = "GraphicString";
pub const GENERAL_STRING: &str = "GeneralString";
pub const UNIVERSAL_STRING: &str = "UniversalString";
pub const BMP_STRING: &str = "BMPString";
pub const PRINTABLE_STRING: &str = "PrintableString";
pub const GENERALIZED_TIME: &str = "GeneralizedTime";
pub const UTC_TIME: &str = "UTCTime";
pub const ENUMERATED: &str = "ENUMERATED";
pub const CHOICE: &str = "CHOICE";
pub const SEQUENCE: &str = "SEQUENCE";
pub const SEQUENCE_OF: &str = "SEQUENCE OF";
pub const SET_OF: &str = "SET OF";
pub const OF: &str = "OF";
pub const ALL: &str = "ALL";
pub const SET: &str = "SET";
pub const OBJECT_IDENTIFIER: &str = "OBJECT IDENTIFIER";
pub const COMPONENTS_OF: &str = "COMPONENTS OF";

// Tagging tokens
pub const UNIVERSAL: &str = "UNIVERSAL";
pub const PRIVATE: &str = "PRIVATE";
pub const APPLICATION: &str = "APPLICATION";

// Value tokens
pub const TRUE: &str = "TRUE";
pub const FALSE: &str = "FALSE";

// Header tokens
pub const BEGIN: &str = "BEGIN";
pub const END: &str = "END";
pub const DEFINITIONS: &str = "DEFINITIONS";
pub const AUTOMATIC: &str = "AUTOMATIC";
pub const EXPLICIT: &str = "EXPLICIT";
pub const IMPLICIT: &str = "IMPLICIT";
pub const IMPORTS: &str = "IMPORTS";
pub const EXPORTS: &str = "EXPORTS";
pub const FROM: &str = "FROM";
pub const INSTRUCTIONS: &str = "INSTRUCTIONS";
pub const TAGS: &str = "TAGS";
pub const EXTENSIBILITY_IMPLIED: &str = "EXTENSIBILITY IMPLIED";
pub const WITH_SUCCESSORS: &str = "WITH SUCCESSORS";
pub const WITH_DESCENDANTS: &str = "WITH DESCENDANTS";
pub const SEMICOLON: char = ';';

// Information Object Class tokens
pub const AMPERSAND: char = '&';
pub const CLASS: &str = "CLASS";
pub const UNIQUE: &str = "UNIQUE";
pub const WITH_SYNTAX: &str = "WITH SYNTAX";
pub const AT: char = '@';
pub const DOT: char = '.';

// Subtyping tokens
pub const SIZE: &str = "SIZE";
pub const CONSTRAINED_BY: &str = "CONSTRAINED BY";
pub const PATTERN: &str = "PATTERN";
pub const DEFAULT: &str = "DEFAULT";
pub const CONTAINING: &str = "CONTAINING";
pub const ENCODED_BY: &str = "ENCODED BY";
pub const OPTIONAL: &str = "OPTIONAL";
pub const WITH_COMPONENTS: &str = "WITH COMPONENTS";
pub const WITH_COMPONENT: &str = "WITH COMPONENT";
pub const UNION: &str = "UNION";
pub const EXCEPT: &str = "EXCEPT";
pub const INTERSECTION: &str = "INTERSECTION";
pub const ABSENT: &str = "ABSENT";
pub const PRESENT: &str = "PRESENT";
pub const INCLUDES: &str = "INCLUDES";
pub const MIN: &str = "MIN";
pub const MAX: &str = "MAX";
pub const LESS_THAN: char = '<';
pub const GREATER_THAN: char = '>';
pub const PIPE: &str = "|";
pub const CARET: &str = "^";

pub const ASSIGN: &str = "::=";
pub const RANGE: &str = "..";
pub const ELLIPSIS: &str = "...";
pub const COMMA: char = ',';
pub const COLON: char = ':';
pub const SINGLE_QUOTE: char = '\'';

// invalid syntax word tokens
pub const ABSTRACT_SYNTAX: &str = "ABSTRACT-SYNTAX";
pub const BIT: &str = "BIT";
pub const CHARACTER: &str = "CHARACTER";
pub const DATE: &str = "DATE";
pub const DATE_TIME: &str = "DATE-TIME";
pub const DURATION: &str = "DURATION";
pub const EMBEDDED_PDV: &str = "EMBEDDED PDV";
pub const EXTERNAL: &str = "EXTERNAL";
pub const INSTANCE_OF: &str = "INSTANCE OF";
pub const MINUS_INFINITY: &str = "MINUS-INFINITY";
pub const NOT_A_NUMBER: &str = "NOT-A-NUMBER";
pub const OBJECT: &str = "OBJECT";
pub const OCTET: &str = "OCTET";
pub const OID_IRI: &str = "OID-IRI";
pub const PLUS_INFINITY: &str = "PLUS-INFINITY";
pub const RELATIVE_OID: &str = "RELATIVE-OID";
pub const RELATIVE_OID_IRI: &str = "RELATIVE-OID-IRI";
pub const TIME: &str = "TIME";
pub const TIME_OF_DAY: &str = "TIME-OF-DAY";
pub const TYPE_IDENTIFIER: &str = "TYPE-IDENTIFIER";

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

impl From<Span<'_>> for EncodingReferenceDefault {
    fn from(value: Span) -> Self {
        Self(value.to_string())
    }
}

#[cfg_attr(test, derive(EnumDebug))]
#[cfg_attr(not(test), derive(Debug))]
#[derive(Clone, Copy, PartialEq)]
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

/// Represents the extensibility environment as specified in
/// Rec. ITU-T X.680 (02/2021) § 13.4
#[cfg_attr(test, derive(EnumDebug))]
#[cfg_attr(not(test), derive(Debug))]
#[derive(Clone, PartialEq)]
pub enum ExtensibilityEnvironment {
    Implied,
    Explicit,
}

/// Represents compatibility selectors as specified in
/// Rec. ITU-T X.680 (02/2021) § 13.16 (f)
#[cfg_attr(test, derive(EnumDebug))]
#[cfg_attr(not(test), derive(Debug))]
#[derive(Clone, PartialEq)]
pub enum With {
    Successors,
    Descendants,
}

/// Represents a global module reference as specified in
/// Rec. ITU-T X.680 (02/2021)
#[derive(Debug, Clone, PartialEq)]
pub struct ExternalValueReference {
    pub module_reference: String,
    pub value_reference: String,
}

/// Represents a global module reference as specified in
/// Rec. ITU-T X.680 (02/2021)
#[derive(Debug, Clone, PartialEq)]
pub struct GlobalModuleReference {
    pub module_reference: String,
    pub assigned_identifier: AssignedIdentifier,
}

impl From<(Span<'_>, AssignedIdentifier)> for GlobalModuleReference {
    fn from(value: (Span, AssignedIdentifier)) -> Self {
        Self {
            module_reference: value.0.to_string(),
            assigned_identifier: value.1,
        }
    }
}

/// Represents an assigned identifier as specified in
/// Rec. ITU-T X.680 (02/2021)
#[cfg_attr(test, derive(EnumDebug))]
#[cfg_attr(not(test), derive(Debug))]
#[derive(Clone, PartialEq)]
pub enum AssignedIdentifier {
    ObjectIdentifierValue(ObjectIdentifierValue),
    ExternalValueReference(ExternalValueReference),
    ValueReference(String),
    ParameterizedValue {
        value_reference: String,
        actual_parameter_list: String,
    },
    Empty,
}

/// Represents a module import as specified in
/// Rec. ITU-T X.680 (02/2021) § 13.16
#[derive(Debug, Clone, PartialEq)]
pub struct Import {
    pub types: Vec<String>,
    pub global_module_reference: GlobalModuleReference,
    pub with: Option<With>,
}

impl From<(Vec<Span<'_>>, (GlobalModuleReference, Option<Span<'_>>))> for Import {
    fn from(value: (Vec<Span>, (GlobalModuleReference, Option<Span>))) -> Self {
        Self {
            types: value.0.iter().map(Span::to_string).collect(),
            global_module_reference: value.1 .0,
            with: value.1 .1.map(|with| {
                if *with == WITH_SUCCESSORS {
                    With::Successors
                } else {
                    With::Descendants
                }
            }),
        }
    }
}

/// Represents a module export as specified in
/// Rec. ITU-T X.680 (02/2021) § 13.13
#[cfg_attr(test, derive(EnumDebug))]
#[cfg_attr(not(test), derive(Debug))]
#[derive(Clone, PartialEq)]
pub enum Exports {
    Identifier(Vec<String>),
    All,
}

impl From<Vec<Span<'_>>> for Exports {
    fn from(value: Vec<Span>) -> Self {
        Self::Identifier(value.iter().map(ToString::to_string).collect())
    }
}

/// Represents a module header's definitive identifier as specified in
/// Rec. ITU-T X.680 (02/2021) § 13.8
#[cfg_attr(test, derive(EnumDebug))]
#[cfg_attr(not(test), derive(Debug))]
#[derive(Clone, PartialEq)]
pub enum DefinitiveIdentifier {
    DefinitiveOID(ObjectIdentifierValue),
    DefinitiveOIDandIRI {
        oid: ObjectIdentifierValue,
        iri: String,
    },
}

impl From<(ObjectIdentifierValue, Option<Span<'_>>)> for DefinitiveIdentifier {
    fn from(value: (ObjectIdentifierValue, Option<Span>)) -> Self {
        if let Some(iri_value) = value.1 {
            Self::DefinitiveOIDandIRI {
                oid: value.0,
                iri: iri_value.to_string(),
            }
        } else {
            Self::DefinitiveOID(value.0)
        }
    }
}

/// Represents a module header as specified in
/// Rec. ITU-T X.680 (02/2021) § 13
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
    /// Returns an import that matches a given identifier, if present.
    pub fn find_import(&self, identifier: &str) -> Option<&String> {
        self.imports
            .iter()
            .find_map(|i| i.types.iter().find(|id| *id == identifier))
    }
}

impl
    From<(
        Span<'_>,
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
            Span,
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
            name: value.0.to_string(),
            module_identifier: value.1,
            encoding_reference_default,
            tagging_environment,
            extensibility_environment,
            exports: value.3,
            imports: value.4.unwrap_or_default(),
        }
    }
}

/// Represents an object identifier value as specified in
/// Rec. ITU-T X.680 (02/2021) §32
#[derive(Debug, Clone, PartialEq)]
pub struct ObjectIdentifierValue(pub Vec<ObjectIdentifierArc>);

impl From<Vec<ObjectIdentifierArc>> for ObjectIdentifierValue {
    fn from(value: Vec<ObjectIdentifierArc>) -> Self {
        Self(value)
    }
}

/// Represents a single arc of an object identifier value
/// as specified in Rec. ITU-T X.680 (02/2021) §32
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

impl From<Span<'_>> for ObjectIdentifierArc {
    fn from(value: Span) -> Self {
        Self {
            name: Some(value.to_string()),
            number: None,
        }
    }
}

impl From<(Span<'_>, u128)> for ObjectIdentifierArc {
    fn from(value: (Span, u128)) -> Self {
        Self {
            name: Some(value.0.to_string()),
            number: Some(value.1),
        }
    }
}

/// Represents a top-level ASN.1 definition.
/// The compiler distinguished three different variants of top-level definitions.
/// * `Type` definitions define custom types based on ASN.1's built-in types
/// * `Value` definitions define values using custom ot built-in types
/// * `Information` definitions define abstraction concepts introduced in ITU-T X.681
///
/// The linker and any [Backend] for this compiler consumes top-level definitions in
/// order to generate bindings.
#[cfg_attr(test, derive(EnumDebug))]
#[cfg_attr(not(test), derive(Debug))]
#[derive(Clone, PartialEq)]
pub enum ToplevelDefinition {
    Type(ToplevelTypeDefinition),
    Value(ToplevelValueDefinition),
    Information(ToplevelInformationDefinition),
}

impl ToplevelDefinition {
    pub(crate) fn has_enum_value(&self, type_name: Option<&String>, identifier: &String) -> bool {
        if let ToplevelDefinition::Type(ToplevelTypeDefinition {
            name,
            ty: ASN1Type::Enumerated(e),
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

    pub(crate) fn set_index(
        &mut self,
        module_reference: Rc<RefCell<ModuleReference>>,
        item_no: usize,
    ) {
        match self {
            ToplevelDefinition::Type(ref mut t) => {
                t.index = Some((module_reference, item_no));
            }
            ToplevelDefinition::Value(ref mut v) => {
                v.index = Some((module_reference, item_no));
            }
            ToplevelDefinition::Information(ref mut i) => {
                i.index = Some((module_reference, item_no));
            }
        }
    }

    pub(crate) fn get_index(&self) -> Option<&(Rc<RefCell<ModuleReference>>, usize)> {
        match self {
            ToplevelDefinition::Type(ref t) => t.index.as_ref(),
            ToplevelDefinition::Value(ref v) => v.index.as_ref(),
            ToplevelDefinition::Information(ref i) => i.index.as_ref(),
        }
    }

    pub(crate) fn get_module_reference(&self) -> Option<Rc<RefCell<ModuleReference>>> {
        match self {
            ToplevelDefinition::Type(ref t) => t.index.as_ref().map(|(m, _)| m.clone()),
            ToplevelDefinition::Value(ref v) => v.index.as_ref().map(|(m, _)| m.clone()),
            ToplevelDefinition::Information(ref i) => i.index.as_ref().map(|(m, _)| m.clone()),
        }
    }

    pub(crate) fn apply_tagging_environment(&mut self, environment: &TaggingEnvironment) {
        if let (env, ToplevelDefinition::Type(ty)) = (environment, self) {
            ty.tag = ty.tag.as_ref().map(|t| AsnTag {
                environment: env + &t.environment,
                tag_class: t.tag_class,
                id: t.id,
            });
            match &mut ty.ty {
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

    /// Returns the name of a top-level definition.
    /// ### Example
    /// ```
    /// # use rasn_compiler::prelude::ir::*;
    /// assert_eq!(
    ///     ToplevelDefinition::Value(
    ///         ToplevelValueDefinition {
    ///             comments: String::from("Comments from the ASN.1 spec"),
    ///             parameterization: None,
    ///             name: String::from("the-answer"),
    ///             associated_type: ASN1Type::Integer(Integer {
    ///                 constraints: vec![],
    ///                 distinguished_values: None,
    ///             }),
    ///             value: ASN1Value::Integer(42),
    ///             index: None,
    ///         }
    ///     ).name(),
    ///     &String::from("the-answer")
    /// );
    /// ```
    pub fn name(&self) -> &String {
        match self {
            ToplevelDefinition::Information(i) => &i.name,
            ToplevelDefinition::Type(t) => &t.name,
            ToplevelDefinition::Value(v) => &v.name,
        }
    }
}

/// Represents a top-level definition of a value
/// using a custom or built-in ASN.1 type.
#[derive(Debug, Clone, PartialEq)]
pub struct ToplevelValueDefinition {
    pub comments: String,
    pub name: String,
    pub associated_type: ASN1Type,
    pub parameterization: Option<Parameterization>,
    pub value: ASN1Value,
    pub index: Option<(Rc<RefCell<ModuleReference>>, usize)>,
}

impl From<(Span<'_>, ASN1Value, ASN1Type)> for ToplevelValueDefinition {
    fn from(value: (Span, ASN1Value, ASN1Type)) -> Self {
        Self {
            comments: String::new(),
            name: value.0.to_string(),
            associated_type: value.2.to_owned(),
            parameterization: None,
            value: value.1,
            index: None,
        }
    }
}

impl
    From<(
        Vec<Span<'_>>,
        Span<'_>,
        Option<Parameterization>,
        ASN1Type,
        ASN1Value,
    )> for ToplevelValueDefinition
{
    fn from(
        value: (
            Vec<Span>,
            Span,
            Option<Parameterization>,
            ASN1Type,
            ASN1Value,
        ),
    ) -> Self {
        Self {
            comments: value
                .0
                .into_iter()
                .map(Span::into_fragment)
                .collect::<Vec<&str>>()
                .join("\n"),
            name: value.1.to_string(),
            parameterization: value.2,
            associated_type: value.3.into(),
            value: value.4,
            index: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ToplevelTypeDefinition {
    pub comments: String,
    pub tag: Option<AsnTag>,
    pub name: String,
    pub ty: ASN1Type,
    pub parameterization: Option<Parameterization>,
    pub index: Option<(Rc<RefCell<ModuleReference>>, usize)>,
}

impl ToplevelTypeDefinition {
    pub fn pdu(&self) -> &ASN1Type {
        &self.ty
    }
}

impl From<(Span<'_>, ASN1Type)> for ToplevelTypeDefinition {
    fn from(value: (Span, ASN1Type)) -> Self {
        Self {
            comments: String::new(),
            tag: None,
            name: value.0.to_string(),
            ty: value.1,
            parameterization: None,
            index: None,
        }
    }
}

impl
    From<(
        Vec<Span<'_>>,
        Span<'_>,
        Option<Parameterization>,
        (Option<AsnTag>, ASN1Type),
    )> for ToplevelTypeDefinition
{
    fn from(
        value: (
            Vec<Span>,
            Span,
            Option<Parameterization>,
            (Option<AsnTag>, ASN1Type),
        ),
    ) -> Self {
        Self {
            comments: value
                .0
                .into_iter()
                .map(Span::into_fragment)
                .collect::<Vec<&str>>()
                .join("\n"),
            name: value.1.to_string(),
            parameterization: value.2,
            ty: value.3 .1,
            tag: value.3 .0,
            index: None,
        }
    }
}

/// The possible types of an ASN1 data element.
/// In addition, the `ElsewhereDeclaredType` enumeral denotes an type
/// specified in the same or an imported ASN1 specification.
#[cfg_attr(test, derive(EnumDebug))]
#[cfg_attr(not(test), derive(Debug))]
#[derive(Clone, PartialEq)]
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
    pub fn as_str(&self) -> Cow<'_, str> {
        match self {
            ASN1Type::Null => Cow::Borrowed(NULL),
            ASN1Type::Boolean(_) => Cow::Borrowed(BOOLEAN),
            ASN1Type::Integer(_) => Cow::Borrowed(INTEGER),
            ASN1Type::Real(_) => Cow::Borrowed(REAL),
            ASN1Type::BitString(_) => Cow::Borrowed(BIT_STRING),
            ASN1Type::OctetString(_) => Cow::Borrowed(OCTET_STRING),
            ASN1Type::CharacterString(CharacterString {
                ty: CharacterStringType::BMPString,
                ..
            }) => Cow::Borrowed(BMP_STRING),
            ASN1Type::CharacterString(CharacterString {
                ty: CharacterStringType::UTF8String,
                ..
            }) => Cow::Borrowed(UTF8_STRING),
            ASN1Type::CharacterString(CharacterString {
                ty: CharacterStringType::PrintableString,
                ..
            }) => Cow::Borrowed(PRINTABLE_STRING),
            ASN1Type::CharacterString(CharacterString {
                ty: CharacterStringType::TeletexString,
                ..
            }) => Cow::Borrowed(TELETEX_STRING),
            ASN1Type::CharacterString(CharacterString {
                ty: CharacterStringType::IA5String,
                ..
            }) => Cow::Borrowed(IA5_STRING),
            ASN1Type::CharacterString(CharacterString {
                ty: CharacterStringType::UniversalString,
                ..
            }) => Cow::Borrowed(UNIVERSAL_STRING),
            ASN1Type::CharacterString(CharacterString {
                ty: CharacterStringType::VisibleString,
                ..
            }) => Cow::Borrowed(VISIBLE_STRING),
            ASN1Type::CharacterString(CharacterString {
                ty: CharacterStringType::GeneralString,
                ..
            }) => Cow::Borrowed(GENERAL_STRING),
            ASN1Type::CharacterString(CharacterString {
                ty: CharacterStringType::VideotexString,
                ..
            }) => Cow::Borrowed(VIDEOTEX_STRING),
            ASN1Type::CharacterString(CharacterString {
                ty: CharacterStringType::GraphicString,
                ..
            }) => Cow::Borrowed(GRAPHIC_STRING),
            ASN1Type::CharacterString(CharacterString {
                ty: CharacterStringType::NumericString,
                ..
            }) => Cow::Borrowed(NUMERIC_STRING),
            ASN1Type::Enumerated(_) => Cow::Borrowed(ENUMERATED),
            ASN1Type::Choice(_) => Cow::Borrowed(CHOICE),
            ASN1Type::Sequence(_) => Cow::Borrowed(SEQUENCE),
            ASN1Type::SequenceOf(_) => Cow::Borrowed(SEQUENCE_OF),
            ASN1Type::Set(_) => Cow::Borrowed(SET),
            ASN1Type::SetOf(_) => Cow::Borrowed(SET_OF),
            ASN1Type::Time(_) => Cow::Borrowed(TIME),
            ASN1Type::GeneralizedTime(_) => Cow::Borrowed(GENERALIZED_TIME),
            ASN1Type::UTCTime(_) => Cow::Borrowed(UTC_TIME),
            ASN1Type::ElsewhereDeclaredType(DeclarationElsewhere { identifier, .. }) => {
                Cow::Borrowed(identifier)
            }
            ASN1Type::ChoiceSelectionType(_) => todo!(),
            ASN1Type::ObjectIdentifier(_) => Cow::Borrowed(OBJECT_IDENTIFIER),
            ASN1Type::InformationObjectFieldReference(ifr) => Cow::Owned(format!(
                "{INTERNAL_IO_FIELD_REF_TYPE_NAME_PREFIX}{}${}",
                ifr.class,
                ifr.field_path_as_str()
            )),
            ASN1Type::EmbeddedPdv => Cow::Borrowed(EMBEDDED_PDV),
            ASN1Type::External => Cow::Borrowed(EXTERNAL),
        }
    }

    pub fn builtin_or_elsewhere(
        parent: Option<Span>,
        identifier: Span,
        constraints: Option<Vec<Constraint>>,
    ) -> ASN1Type {
        match (parent, identifier.as_ref()) {
            (None, NULL) => ASN1Type::Null,
            (None, BOOLEAN) => ASN1Type::Boolean(Boolean {
                constraints: constraints.unwrap_or_default(),
            }),
            (None, REAL) => ASN1Type::Real(Real {
                constraints: constraints.unwrap_or_default(),
            }),
            (None, INTEGER) => ASN1Type::Integer(Integer {
                constraints: constraints.unwrap_or_default(),
                distinguished_values: None,
            }),
            (None, BIT_STRING) => ASN1Type::BitString(BitString {
                constraints: constraints.unwrap_or_default(),
                distinguished_values: None,
            }),
            (None, OCTET_STRING) => ASN1Type::OctetString(OctetString {
                constraints: constraints.unwrap_or_default(),
            }),
            (None, GENERALIZED_TIME) => ASN1Type::GeneralizedTime(GeneralizedTime {
                constraints: constraints.unwrap_or_default(),
            }),
            (None, UTC_TIME) => ASN1Type::UTCTime(UTCTime {
                constraints: constraints.unwrap_or_default(),
            }),
            (None, OBJECT_IDENTIFIER) => ASN1Type::ObjectIdentifier(ObjectIdentifier {
                constraints: constraints.unwrap_or_default(),
            }),
            (None, BMP_STRING) => ASN1Type::CharacterString(CharacterString {
                constraints: constraints.unwrap_or_default(),
                ty: CharacterStringType::BMPString,
            }),
            (None, UTF8_STRING) => ASN1Type::CharacterString(CharacterString {
                constraints: constraints.unwrap_or_default(),
                ty: CharacterStringType::UTF8String,
            }),
            (None, PRINTABLE_STRING) => ASN1Type::CharacterString(CharacterString {
                constraints: constraints.unwrap_or_default(),
                ty: CharacterStringType::PrintableString,
            }),
            (None, TELETEX_STRING) => ASN1Type::CharacterString(CharacterString {
                constraints: constraints.unwrap_or_default(),
                ty: CharacterStringType::TeletexString,
            }),
            (None, IA5_STRING) => ASN1Type::CharacterString(CharacterString {
                constraints: constraints.unwrap_or_default(),
                ty: CharacterStringType::IA5String,
            }),
            (None, UNIVERSAL_STRING) => ASN1Type::CharacterString(CharacterString {
                constraints: constraints.unwrap_or_default(),
                ty: CharacterStringType::UniversalString,
            }),
            (None, VISIBLE_STRING) => ASN1Type::CharacterString(CharacterString {
                constraints: constraints.unwrap_or_default(),
                ty: CharacterStringType::VisibleString,
            }),
            (None, GENERAL_STRING) => ASN1Type::CharacterString(CharacterString {
                constraints: constraints.unwrap_or_default(),
                ty: CharacterStringType::GeneralString,
            }),
            (None, VIDEOTEX_STRING) => ASN1Type::CharacterString(CharacterString {
                constraints: constraints.unwrap_or_default(),
                ty: CharacterStringType::VideotexString,
            }),
            (None, GRAPHIC_STRING) => ASN1Type::CharacterString(CharacterString {
                constraints: constraints.unwrap_or_default(),
                ty: CharacterStringType::GraphicString,
            }),
            (None, NUMERIC_STRING) => ASN1Type::CharacterString(CharacterString {
                constraints: constraints.unwrap_or_default(),
                ty: CharacterStringType::NumericString,
            }),
            _ => ASN1Type::ElsewhereDeclaredType((parent, identifier, constraints).into()),
        }
    }

    pub fn is_builtin_type(&self) -> bool {
        !matches!(
            self,
            ASN1Type::ElsewhereDeclaredType(_)
                | ASN1Type::ChoiceSelectionType(_)
                | ASN1Type::InformationObjectFieldReference(_)
        )
    }

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
#[cfg_attr(test, derive(EnumDebug))]
#[cfg_attr(not(test), derive(Debug))]
#[derive(Clone, PartialEq, Copy)]
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
                .map(|i| char::from_u32(i).unwrap())
                .enumerate()
                .collect(),
            _ => (0..u16::MAX as u32)
                .filter_map(char::from_u32)
                .enumerate()
                .collect(),
        }
    }
}

impl From<Span<'_>> for CharacterStringType {
    fn from(value: Span) -> Self {
        match *value {
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

impl ToTokens for IntegerType {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            IntegerType::Int8 => tokens.append_all(quote!(i8)),
            IntegerType::Uint8 => tokens.append_all(quote!(u8)),
            IntegerType::Int16 => tokens.append_all(quote!(i16)),
            IntegerType::Uint16 => tokens.append_all(quote!(u16)),
            IntegerType::Int32 => tokens.append_all(quote!(i32)),
            IntegerType::Uint32 => tokens.append_all(quote!(u32)),
            IntegerType::Int64 => tokens.append_all(quote!(i64)),
            IntegerType::Uint64 => tokens.append_all(quote!(u64)),
            IntegerType::Unbounded => tokens.append_all(quote!(Integer)),
        }
    }
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
#[cfg_attr(test, derive(EnumDebug))]
#[cfg_attr(not(test), derive(Debug))]
#[derive(Clone, PartialEq)]
pub enum ASN1Value {
    All,
    Null,
    Boolean(bool),
    Choice {
        type_name: Option<String>,
        variant_name: String,
        inner_value: Box<ASN1Value>,
    },
    /// In ASN.1, value definitions are ambiguous between SEQUENCE, SET, SEQUENCE OF, and SET OF
    /// For example, `{ my-elem FALSE }` could be a value of all four types
    SequenceOrSet(Vec<(Option<String>, Box<ASN1Value>)>),
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
    /// the relation of the default value to `ExampleSubset` will not be picked up by the lexer.
    /// However, in some representations, this relation is critical information.
    LinkedNestedValue {
        /// typereferences of supertypes
        supertypes: Vec<String>,
        value: Box<ASN1Value>,
    },
    /// Integer values need type information that will not always be picked up by the lexer on first pass.
    LinkedIntValue {
        integer_type: IntegerType,
        value: i128,
    },
    /// Struct-like values such as SEQUENCE values need type information that will not always be picked up by the lexer on first pass.
    /// Contains a vector of the struct-like's fields, with the field name, the field type, and the field value as a tuple
    LinkedStructLikeValue(Vec<(String, ASN1Type, StructLikeFieldValue)>),
    /// Array-like values such as SEQUENCE OF values need type information that will not always be picked up by the lexer on first pass.
    LinkedArrayLikeValue(Vec<Box<ASN1Value>>),
    /// Character string values such as UTF8String values need type information that will not always be picked up by the lexer on first pass.
    LinkedCharStringValue(CharacterStringType, String),
    LinkedElsewhereDefinedValue {
        parent: Option<String>,
        identifier: String,
        can_be_const: bool,
    },
}

/// Representation of a field value of a struct-like ASN1 value
#[cfg_attr(test, derive(EnumDebug))]
#[cfg_attr(not(test), derive(Debug))]
#[derive(Clone, PartialEq)]
pub enum StructLikeFieldValue {
    Explicit(Box<ASN1Value>),
    Implicit(Box<ASN1Value>),
}

impl StructLikeFieldValue {
    pub fn into_value(self) -> ASN1Value {
        match self {
            StructLikeFieldValue::Explicit(v) | StructLikeFieldValue::Implicit(v) => *v,
        }
    }

    pub fn value(&self) -> &ASN1Value {
        match self {
            StructLikeFieldValue::Explicit(v) | StructLikeFieldValue::Implicit(v) => v,
        }
    }

    pub fn value_mut(&mut self) -> &mut ASN1Value {
        match self {
            StructLikeFieldValue::Explicit(ref mut v)
            | StructLikeFieldValue::Implicit(ref mut v) => &mut *v,
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

impl From<(Option<Span<'_>>, Span<'_>, Option<Vec<Constraint>>)> for DeclarationElsewhere {
    fn from(value: (Option<Span>, Span, Option<Vec<Constraint>>)) -> Self {
        DeclarationElsewhere {
            parent: value.0.as_ref().map(Span::to_string),
            identifier: value.1.to_string(),
            constraints: value.2.unwrap_or_default(),
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

impl From<((Option<Span<'_>>, u64), Option<TaggingEnvironment>)> for AsnTag {
    fn from(value: ((Option<Span>, u64), Option<TaggingEnvironment>)) -> Self {
        let tag_class = match value.0 .0.map(|span| *span) {
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
