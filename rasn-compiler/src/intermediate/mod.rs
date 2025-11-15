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
pub mod macros;
pub mod parameterization;
pub mod types;
pub mod utils;

use std::{borrow::Cow, cell::RefCell, collections::BTreeMap, ops::Add, rc::Rc, usize};

use crate::{common::INTERNAL_IO_FIELD_REF_TYPE_NAME_PREFIX, prelude::ir::Parameter};
use constraints::Constraint;
use error::{GrammarError, GrammarErrorType};
use information_object::{
    ObjectClassAssignment, ObjectClassFieldType, ObjectOrObjectSetAssignment,
};
#[cfg(test)]
use internal_macros::EnumDebug;
use macros::MacroDefinition;
use parameterization::Parameterization;
use quote::{quote, ToTokens, TokenStreamExt};
use types::*;

#[cfg(doc)]
use crate::Backend;

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

// Macro tokens
pub const MACRO: &str = "MACRO";

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
pub const ENCODING_CONTROL: &str = "ENCODING-CONTROL";

pub const ASN1_KEYWORDS: [&str; 64] = [
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
    MACRO,
];

macro_rules! grammar_error {
    ($kind:ident, $($arg:tt)*) => {
        GrammarError::new(&format!($($arg)*),GrammarErrorType::$kind)
    };
}

#[derive(Debug, Clone, PartialEq)]
pub struct EncodingReferenceDefault<'a>(pub Cow<'a, str>);

impl<'a> From<&'a str> for EncodingReferenceDefault<'a> {
    fn from(value: &'a str) -> Self {
        Self(Cow::Borrowed(value))
    }
}

#[cfg_attr(test, derive(EnumDebug))]
#[cfg_attr(not(test), derive(Debug))]
#[derive(Clone, Copy, PartialEq, Default)]
pub enum TaggingEnvironment {
    Automatic,
    #[default]
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
#[derive(Clone, Copy, PartialEq, Default)]
pub enum ExtensibilityEnvironment {
    Implied,
    #[default]
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
pub struct ExternalValueReference<'a> {
    pub module_reference: Cow<'a, str>,
    pub value_reference: Cow<'a, str>,
}

/// Represents a global module reference as specified in
/// Rec. ITU-T X.680 (02/2021)
#[derive(Debug, Clone, PartialEq)]
pub struct GlobalModuleReference<'a> {
    pub module_reference: Cow<'a, str>,
    pub assigned_identifier: AssignedIdentifier<'a>,
}

impl<'a> From<(&'a str, AssignedIdentifier<'a>)> for GlobalModuleReference<'a> {
    fn from(value: (&'a str, AssignedIdentifier<'a>)) -> Self {
        Self {
            module_reference: Cow::Borrowed(value.0),
            assigned_identifier: value.1,
        }
    }
}

/// Represents an assigned identifier as specified in
/// Rec. ITU-T X.680 (02/2021)
#[cfg_attr(test, derive(EnumDebug))]
#[cfg_attr(not(test), derive(Debug))]
#[derive(Clone, PartialEq)]
pub enum AssignedIdentifier<'a> {
    ObjectIdentifierValue(ObjectIdentifierValue),
    ExternalValueReference(ExternalValueReference<'a>),
    ValueReference(Cow<'a, str>),
    ParameterizedValue {
        value_reference: Cow<'a, str>,
        actual_parameter_list: Cow<'a, str>,
    },
    Empty,
}

/// Represents a module import as specified in
/// Rec. ITU-T X.680 (02/2021) § 13.16
#[derive(Debug, Clone, PartialEq)]
pub struct Import<'a> {
    pub types: Vec<Cow<'a, str>>,
    pub global_module_reference: GlobalModuleReference<'a>,
    pub with: Option<With>,
}

impl<'a> From<(Vec<&'a str>, (GlobalModuleReference<'a>, Option<&str>))> for Import<'a> {
    fn from(value: (Vec<&'a str>, (GlobalModuleReference<'a>, Option<&str>))) -> Self {
        Self {
            types: value.0.into_iter().map(Cow::Borrowed).collect(),
            global_module_reference: value.1 .0,
            with: value.1 .1.map(|with| {
                if with == WITH_SUCCESSORS {
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
pub enum Exports<'a> {
    Identifier(Vec<Cow<'a, str>>),
    All,
}

impl<'a> From<Vec<&'a str>> for Exports<'a> {
    fn from(value: Vec<&'a str>) -> Self {
        Self::Identifier(value.into_iter().map(Cow::Borrowed).collect())
    }
}

/// Represents a module header's definitive identifier as specified in
/// Rec. ITU-T X.680 (02/2021) § 13.8
#[cfg_attr(test, derive(EnumDebug))]
#[cfg_attr(not(test), derive(Debug))]
#[derive(Clone, PartialEq)]
pub enum DefinitiveIdentifier<'a> {
    DefinitiveOID(ObjectIdentifierValue),
    DefinitiveOIDandIRI {
        oid: ObjectIdentifierValue,
        iri: Cow<'a, str>,
    },
}

impl<'a> From<(ObjectIdentifierValue, Option<&'a str>)> for DefinitiveIdentifier<'a> {
    fn from(value: (ObjectIdentifierValue, Option<&'a str>)) -> Self {
        if let Some(iri_value) = value.1 {
            Self::DefinitiveOIDandIRI {
                oid: value.0,
                iri: Cow::Borrowed(iri_value),
            }
        } else {
            Self::DefinitiveOID(value.0)
        }
    }
}

/// Represents a module header as specified in
/// Rec. ITU-T X.680 (02/2021) § 13
#[derive(Debug, Clone, PartialEq)]
pub struct ModuleHeader<'a> {
    pub name: Cow<'a, str>,
    pub module_identifier: Option<DefinitiveIdentifier<'a>>,
    pub encoding_reference_default: Option<EncodingReferenceDefault<'a>>,
    pub tagging_environment: TaggingEnvironment,
    pub extensibility_environment: ExtensibilityEnvironment,
    pub imports: Vec<Import<'a>>,
    pub exports: Option<Exports<'a>>,
}

impl<'a> ModuleHeader<'a> {
    /// Returns an import that matches a given identifier, if present.
    pub fn find_import(&self, identifier: &str) -> Option<Cow<'a, str>> {
        self.imports
            .iter()
            .find_map(|i| i.types.iter().find(|id| *id == identifier))
            .cloned()
    }
}

impl<'a>
    From<(
        &'a str,
        Option<DefinitiveIdentifier<'a>>,
        Option<(
            Option<EncodingReferenceDefault<'a>>,
            TaggingEnvironment,
            ExtensibilityEnvironment,
        )>,
        Option<Exports<'a>>,
        Option<Vec<Import<'a>>>,
    )> for ModuleHeader<'a>
{
    fn from(
        value: (
            &'a str,
            Option<DefinitiveIdentifier<'a>>,
            Option<(
                Option<EncodingReferenceDefault<'a>>,
                TaggingEnvironment,
                ExtensibilityEnvironment,
            )>,
            Option<Exports<'a>>,
            Option<Vec<Import<'a>>>,
        ),
    ) -> Self {
        let (encoding_reference_default, tagging_environment, extensibility_environment) =
            value.2.unwrap_or((
                None,
                TaggingEnvironment::Explicit,
                ExtensibilityEnvironment::Explicit,
            ));
        Self {
            name: Cow::Borrowed(value.0),
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

impl ObjectIdentifierArc {
    const ITU_T: u128 = 0;
    const ISO: u128 = 1;
    const JOINT_ISO_ITU_T: u128 = 2;
    const JOINT_ISO_CCITT: u128 = 2;
    const RECOMMENDATION: u128 = 0;
    const QUESTION: u128 = 1;
    const ADMINISTRATION: u128 = 2;
    const NETWORK_OPERATOR: u128 = 3;
    const ITU_T_IDENTIFIED_ORGANIZATION: u128 = 4;
    const R_RECOMMENDATION: u128 = 5;
    const STANDARD: u128 = 0;
    const REGISTRATION_AUTHORITY: u128 = 1;
    const MEMBER_BODY: u128 = 2;
    const ISO_IDENTIFIED_ORGANIZATION: u128 = 3;

    pub(crate) fn well_known(name: Option<&String>, root: Option<u8>) -> Option<u128> {
        match (root, name.map(|s| s.as_str())) {
            (_, Some("itu-t")) => Some(Self::ITU_T),
            (_, Some("iso")) => Some(Self::ISO),
            (_, Some("joint-iso-itu-t")) => Some(Self::JOINT_ISO_ITU_T),
            (_, Some("joint-iso-ccitt")) => Some(Self::JOINT_ISO_CCITT),
            (Some(0), Some("recommendation")) => Some(Self::RECOMMENDATION),
            (Some(0), Some("question")) => Some(Self::QUESTION),
            (Some(0), Some("administration")) => Some(Self::ADMINISTRATION),
            (Some(0), Some("network-operator")) => Some(Self::NETWORK_OPERATOR),
            (Some(0), Some("identified-organization")) => Some(Self::ITU_T_IDENTIFIED_ORGANIZATION),
            (Some(0), Some("r-recommendation")) => Some(Self::R_RECOMMENDATION),
            (Some(1), Some("standard")) => Some(Self::STANDARD),
            (Some(1), Some("registration-authority")) => Some(Self::REGISTRATION_AUTHORITY),
            (Some(1), Some("member-body")) => Some(Self::MEMBER_BODY),
            (Some(1), Some("identified-organization")) => Some(Self::ISO_IDENTIFIED_ORGANIZATION),
            _ => None,
        }
    }
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

/// Represents an ASN.1 Module
#[derive(Debug, Clone, PartialEq)]
pub struct AsnModule<'a> {
    pub module_header: ModuleHeader<'a>,
    pub assignments: Vec<Assignment<'a>>,
}

impl<'a> From<(ModuleHeader<'a>, Vec<Assignment<'a>>)> for AsnModule<'a> {
    fn from(value: (ModuleHeader<'a>, Vec<Assignment<'a>>)) -> Self {
        AsnModule {
            module_header: value.0,
            assignments: value.1,
        }
    }
}

/// Represents a top-level ASN.1 definition.
/// The compiler distinguished three different variants of top-level definitions.
///
/// The linker and any [Backend] for this compiler consumes top-level definitions in
/// order to generate bindings.
#[cfg_attr(test, derive(EnumDebug))]
#[cfg_attr(not(test), derive(Debug))]
#[derive(Clone, PartialEq)]
pub enum Assignment<'a> {
    /// Definition for a custom type based on ASN.1's built-in type.
    Type(TypeAssignment<'a>),
    /// Definition for a value using custom or built-in type.
    Value(ValueAssignment<'a>),
    /// Definition for a object class as introduced in ITU-T X.681 9.
    Class(ObjectClassAssignment<'a>),
    /// Definition for an object or object set, as introduced in ITU-T X.681 11.
    Object(ObjectOrObjectSetAssignment<'a>),
    /// Definition for a macro.
    Macro(MacroDefinition<'a>),
}

impl<'a> Assignment<'a> {
    pub(crate) fn has_enum_value(&self, type_name: Option<&str>, identifier: &str) -> bool {
        if let Assignment::Type(TypeAssignment {
            name,
            ty: ASN1Type::Enumerated(e),
            ..
        }) = self
        {
            if type_name.is_some() && Some(&**name) != type_name {
                return false;
            }
            e.members.iter().any(|m| &m.name == identifier)
        } else {
            false
        }
    }

    pub(crate) fn set_module_header(&mut self, module_header: Rc<RefCell<ModuleHeader<'a>>>) {
        match self {
            Assignment::Type(ref mut t) => {
                t.module_header = Some(module_header);
            }
            Assignment::Value(ref mut v) => {
                v.module_header = Some(module_header);
            }
            Assignment::Class(ref mut c) => {
                c.module_header = Some(module_header);
            }
            Assignment::Object(ref mut o) => {
                o.module_header = Some(module_header);
            }
            Assignment::Macro(ref mut m) => {
                m.module_header = Some(module_header);
            }
        }
    }

    pub(crate) fn get_module_header(&self) -> Option<Rc<RefCell<ModuleHeader<'a>>>> {
        match self {
            Assignment::Type(ref t) => t.module_header.as_ref().cloned(),
            Assignment::Value(ref v) => v.module_header.as_ref().cloned(),
            Assignment::Class(ref c) => c.module_header.as_ref().cloned(),
            Assignment::Object(ref o) => o.module_header.as_ref().cloned(),
            Assignment::Macro(ref m) => m.module_header.as_ref().cloned(),
        }
    }

    pub(crate) fn apply_tagging_environment(&mut self, environment: &TaggingEnvironment) {
        if let (env, Assignment::Type(ty)) = (environment, self) {
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
    /// # use std::borrow::Cow;
    /// assert_eq!(
    ///     ToplevelDefinition::Value(
    ///         ToplevelValueDefinition {
    ///             comments: Cow::Borrowed("Comments from the ASN.1 spec"),
    ///             parameterization: None,
    ///             name: String::from("the-answer"),
    ///             associated_type: ASN1Type::Integer(Integer {
    ///                 constraints: vec![],
    ///                 distinguished_values: None,
    ///             }),
    ///             value: ASN1Value::Integer(42),
    ///             module_header: None,
    ///         }
    ///     ).name(),
    ///     &String::from("the-answer")
    /// );
    /// ```
    pub fn name(&self) -> Cow<'a, str> {
        match self {
            Assignment::Class(c) => c.name.clone(),
            Assignment::Object(o) => o.name.clone(),
            Assignment::Type(t) => t.name.clone(),
            Assignment::Value(v) => v.name.clone(),
            Assignment::Macro(v) => v.name.clone(),
        }
    }
}

/// Represents a top-level definition of a value
/// using a custom or built-in ASN.1 type.
#[derive(Debug, Clone, PartialEq)]
pub struct ValueAssignment<'a> {
    pub comments: Cow<'a, str>,
    pub name: Cow<'a, str>,
    pub associated_type: ASN1Type<'a>,
    pub parameterization: Option<Parameterization<'a>>,
    pub value: ASN1Value<'a>,
    pub module_header: Option<Rc<RefCell<ModuleHeader<'a>>>>,
}

impl<'a> ValueAssignment<'a> {
    pub fn with_name_value_type(
        name: Cow<'a, str>,
        value: ASN1Value<'a>,
        associated_type: ASN1Type<'a>,
    ) -> Self {
        Self {
            comments: Cow::Borrowed(""),
            name,
            associated_type,
            parameterization: None,
            value,
            module_header: None,
        }
    }
}

impl<'a> From<(&'a str, ASN1Value<'a>, ASN1Type<'a>)> for ValueAssignment<'a> {
    fn from(value: (&'a str, ASN1Value<'a>, ASN1Type<'a>)) -> Self {
        Self {
            comments: Cow::Borrowed(""),
            name: Cow::Borrowed(value.0),
            associated_type: value.2.to_owned(),
            parameterization: None,
            value: value.1,
            module_header: None,
        }
    }
}

impl<'a>
    From<(
        Vec<&str>,
        &'a str,
        Option<Parameterization<'a>>,
        ASN1Type<'a>,
        ASN1Value<'a>,
    )> for ValueAssignment<'a>
{
    fn from(
        value: (
            Vec<&str>,
            &'a str,
            Option<Parameterization<'a>>,
            ASN1Type<'a>,
            ASN1Value<'a>,
        ),
    ) -> Self {
        Self {
            comments: Cow::Owned(value.0.join("\n")),
            name: Cow::Borrowed(value.1),
            parameterization: value.2,
            associated_type: value.3,
            value: value.4,
            module_header: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeAssignment<'a> {
    pub comments: Cow<'a, str>,
    pub tag: Option<AsnTag>,
    pub name: Cow<'a, str>,
    pub ty: ASN1Type<'a>,
    pub parameterization: Option<Parameterization<'a>>,
    pub module_header: Option<Rc<RefCell<ModuleHeader<'a>>>>,
}

impl<'a> TypeAssignment<'a> {
    pub fn pdu(&self) -> &ASN1Type<'a> {
        &self.ty
    }

    pub fn with_name_and_type(name: Cow<'a, str>, ty: ASN1Type<'a>) -> Self {
        Self {
            comments: Cow::Borrowed(""),
            tag: None,
            name,
            ty,
            parameterization: None,
            module_header: None,
        }
    }
}

impl<'a> From<(&'a str, ASN1Type<'a>)> for TypeAssignment<'a> {
    fn from(value: (&'a str, ASN1Type<'a>)) -> Self {
        Self {
            comments: Cow::Borrowed(""),
            tag: None,
            name: Cow::Borrowed(value.0),
            ty: value.1,
            parameterization: None,
            module_header: None,
        }
    }
}

impl<'a>
    From<(
        Vec<&str>,
        &'a str,
        Option<Parameterization<'a>>,
        (Option<AsnTag>, ASN1Type<'a>),
    )> for TypeAssignment<'a>
{
    fn from(
        value: (
            Vec<&str>,
            &'a str,
            Option<Parameterization<'a>>,
            (Option<AsnTag>, ASN1Type<'a>),
        ),
    ) -> Self {
        Self {
            comments: Cow::Owned(value.0.join("\n")),
            name: Cow::Borrowed(value.1),
            parameterization: value.2,
            ty: value.3 .1,
            tag: value.3 .0,
            module_header: None,
        }
    }
}

/// The possible types of an ASN1 data element.
/// In addition, the `ElsewhereDeclaredType` enumeral denotes an type
/// specified in the same or an imported ASN1 specification.
#[cfg_attr(test, derive(EnumDebug))]
#[cfg_attr(not(test), derive(Debug))]
#[derive(Clone, PartialEq)]
pub enum ASN1Type<'a> {
    Null,
    Boolean(Boolean<'a>),
    Integer(Integer<'a>),
    Real(Real<'a>),
    BitString(BitString<'a>),
    OctetString(OctetString<'a>),
    CharacterString(CharacterString<'a>),
    Enumerated(Enumerated<'a>),
    Choice(Choice<'a>),
    Sequence(SequenceOrSet<'a>),
    SequenceOf(SequenceOrSetOf<'a>),
    Set(SequenceOrSet<'a>),
    SetOf(SequenceOrSetOf<'a>),
    Time(Time<'a>),
    GeneralizedTime(GeneralizedTime<'a>),
    UTCTime(UTCTime<'a>),
    ElsewhereDeclaredType(DeclarationElsewhere<'a>),
    ChoiceSelectionType(ChoiceSelectionType<'a>),
    ObjectIdentifier(ObjectIdentifier<'a>),
    ObjectClassField(ObjectClassFieldType<'a>),
    EmbeddedPdv,
    External,
}

impl<'a> ASN1Type<'a> {
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
                Cow::Borrowed(
                    identifier
                        .as_typereference()
                        .expect("Casting defined types as str only works with typereferences!"),
                )
            }
            ASN1Type::ChoiceSelectionType(_) => todo!(),
            ASN1Type::ObjectIdentifier(_) => Cow::Borrowed(OBJECT_IDENTIFIER),
            ASN1Type::ObjectClassField(ifr) => Cow::Owned(format!(
                "{INTERNAL_IO_FIELD_REF_TYPE_NAME_PREFIX}{}${}",
                ifr.class,
                ifr.field_path_as_str()
            )),
            ASN1Type::EmbeddedPdv => Cow::Borrowed(EMBEDDED_PDV),
            ASN1Type::External => Cow::Borrowed(EXTERNAL),
        }
    }

    pub fn builtin_or_elsewhere(
        parent: Option<&'a str>,
        identifier: DefinedType<'a>,
        constraints: Vec<Constraint<'a>>,
    ) -> ASN1Type<'a> {
        match (parent, identifier.as_typereference()) {
            (None, Some(NULL)) => ASN1Type::Null,
            (None, Some(BOOLEAN)) => ASN1Type::Boolean(Boolean { constraints }),
            (None, Some(REAL)) => ASN1Type::Real(Real { constraints }),
            (None, Some(INTEGER)) => ASN1Type::Integer(Integer {
                constraints,
                distinguished_values: None,
            }),
            (None, Some(BIT_STRING)) => ASN1Type::BitString(BitString {
                constraints,
                distinguished_values: None,
            }),
            (None, Some(OCTET_STRING)) => ASN1Type::OctetString(OctetString { constraints }),
            (None, Some(GENERALIZED_TIME)) => {
                ASN1Type::GeneralizedTime(GeneralizedTime { constraints })
            }
            (None, Some(UTC_TIME)) => ASN1Type::UTCTime(UTCTime { constraints }),
            (None, Some(OBJECT_IDENTIFIER)) => {
                ASN1Type::ObjectIdentifier(ObjectIdentifier { constraints })
            }
            (None, Some(BMP_STRING)) => ASN1Type::CharacterString(CharacterString {
                constraints,
                ty: CharacterStringType::BMPString,
            }),
            (None, Some(UTF8_STRING)) => ASN1Type::CharacterString(CharacterString {
                constraints,
                ty: CharacterStringType::UTF8String,
            }),
            (None, Some(PRINTABLE_STRING)) => ASN1Type::CharacterString(CharacterString {
                constraints,
                ty: CharacterStringType::PrintableString,
            }),
            (None, Some(TELETEX_STRING)) => ASN1Type::CharacterString(CharacterString {
                constraints,
                ty: CharacterStringType::TeletexString,
            }),
            (None, Some(IA5_STRING)) => ASN1Type::CharacterString(CharacterString {
                constraints,
                ty: CharacterStringType::IA5String,
            }),
            (None, Some(UNIVERSAL_STRING)) => ASN1Type::CharacterString(CharacterString {
                constraints,
                ty: CharacterStringType::UniversalString,
            }),
            (None, Some(VISIBLE_STRING)) => ASN1Type::CharacterString(CharacterString {
                constraints,
                ty: CharacterStringType::VisibleString,
            }),
            (None, Some(GENERAL_STRING)) => ASN1Type::CharacterString(CharacterString {
                constraints,
                ty: CharacterStringType::GeneralString,
            }),
            (None, Some(VIDEOTEX_STRING)) => ASN1Type::CharacterString(CharacterString {
                constraints,
                ty: CharacterStringType::VideotexString,
            }),
            (None, Some(GRAPHIC_STRING)) => ASN1Type::CharacterString(CharacterString {
                constraints,
                ty: CharacterStringType::GraphicString,
            }),
            (None, Some(NUMERIC_STRING)) => ASN1Type::CharacterString(CharacterString {
                constraints,
                ty: CharacterStringType::NumericString,
            }),
            _ => ASN1Type::ElsewhereDeclaredType(DeclarationElsewhere {
                parent: parent.map(Cow::Borrowed),
                identifier,
                constraints,
            }),
        }
    }

    pub fn is_builtin_type(&self) -> bool {
        !matches!(
            self,
            ASN1Type::ElsewhereDeclaredType(_)
                | ASN1Type::ChoiceSelectionType(_)
                | ASN1Type::ObjectClassField(_)
        )
    }

    pub fn is_constrained_type(&self) -> bool {
        self.constraints().map(|c| !c.is_empty()).unwrap_or(false)
    }

    pub fn is_constructed_type(&self) -> bool {
        match self {
            ASN1Type::Choice(_)
            | ASN1Type::Sequence(_)
            | ASN1Type::SequenceOf(_)
            | ASN1Type::Set(_)
            | ASN1Type::SetOf(_) => true,
            _ => false,
        }
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
            ASN1Type::ObjectClassField(f) => Some(f.constraints()),
            _ => None,
        }
    }

    pub fn constraints_mut(&mut self) -> Option<&mut Vec<Constraint<'a>>> {
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
            ASN1Type::ObjectClassField(f) => Some(f.constraints_mut()),
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
    ///   if equal, `self` is returned
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
pub enum ASN1Value<'a> {
    All,
    Null,
    Boolean(bool),
    Choice {
        type_name: Option<Cow<'a, str>>,
        variant_name: Cow<'a, str>,
        inner_value: Box<ASN1Value<'a>>,
    },
    /// In ASN.1, value definitions are ambiguous between SEQUENCE, SET, SEQUENCE OF, and SET OF
    /// For example, `{ my-elem FALSE }` could be a value of all four types
    SequenceOrSet(Vec<(Option<Cow<'a, str>>, Box<ASN1Value<'a>>)>),
    Integer(i128),
    Real(f64),
    String(Cow<'a, str>),
    BitString(Vec<bool>),
    BitStringNamedBits(Vec<Cow<'a, str>>),
    OctetString(Vec<u8>),
    EnumeratedValue {
        enumerated: Cow<'a, str>, // TODO: replace with DefinedType
        enumerable: Cow<'a, str>,
    },
    Time(Cow<'a, str>),
    ElsewhereDeclaredValue {
        parent: Option<Cow<'a, str>>,
        identifier: Cow<'a, str>,
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
        supertypes: Vec<Cow<'a, str>>,
        value: Box<ASN1Value<'a>>,
    },
    /// Integer values need type information that will not always be picked up by the lexer on first pass.
    LinkedIntValue {
        integer_type: IntegerType,
        value: i128,
    },
    /// Struct-like values such as SEQUENCE values need type information that will not always be picked up by the lexer on first pass.
    /// Contains a vector of the struct-like's fields, with the field name, the field type, and the field value as a tuple
    LinkedStructLikeValue(Vec<(Cow<'a, str>, ASN1Type<'a>, StructLikeFieldValue<'a>)>),
    /// Array-like values such as SEQUENCE OF values need type information that will not always be picked up by the lexer on first pass.
    LinkedArrayLikeValue(Vec<Box<ASN1Value<'a>>>),
    /// Character string values such as UTF8String values need type information that will not always be picked up by the lexer on first pass.
    LinkedCharStringValue(CharacterStringType, Cow<'a, str>),
    LinkedElsewhereDefinedValue {
        parent: Option<Cow<'a, str>>,
        identifier: Cow<'a, str>,
        can_be_const: bool,
    },
}

impl<'a> ASN1Value<'a> {
    pub fn is_constructed_value(&self) -> bool {
        match self {
            ASN1Value::Choice { .. }
            | ASN1Value::LinkedArrayLikeValue(_)
            | ASN1Value::LinkedStructLikeValue(_)
            | ASN1Value::SequenceOrSet(_) => true,
            _ => false,
        }
    }
}

/// Representation of a field value of a struct-like ASN1 value
#[cfg_attr(test, derive(EnumDebug))]
#[cfg_attr(not(test), derive(Debug))]
#[derive(Clone, PartialEq)]
pub enum StructLikeFieldValue<'a> {
    Explicit(Box<ASN1Value<'a>>),
    Implicit(Box<ASN1Value<'a>>),
}

impl<'a> StructLikeFieldValue<'a> {
    pub fn into_value(self) -> ASN1Value<'a> {
        match self {
            StructLikeFieldValue::Explicit(v) | StructLikeFieldValue::Implicit(v) => *v,
        }
    }

    pub fn value(&self) -> &ASN1Value {
        match self {
            StructLikeFieldValue::Explicit(v) | StructLikeFieldValue::Implicit(v) => v,
        }
    }

    pub fn value_mut(&mut self) -> &mut ASN1Value<'a> {
        match self {
            StructLikeFieldValue::Explicit(ref mut v)
            | StructLikeFieldValue::Implicit(ref mut v) => &mut *v,
        }
    }
}

impl<'a> AsMut<ASN1Value<'a>> for ASN1Value<'a> {
    fn as_mut(&mut self) -> &mut ASN1Value<'a> {
        self
    }
}

impl<'a> ASN1Value<'a> {
    pub fn max(
        &self,
        other: &ASN1Value<'a>,
        char_set: Option<&BTreeMap<usize, char>>,
    ) -> Result<ASN1Value<'a>, GrammarError> {
        self.min_max(other, char_set, false)
    }

    pub fn min(
        &self,
        other: &ASN1Value<'a>,
        char_set: Option<&BTreeMap<usize, char>>,
    ) -> Result<ASN1Value<'a>, GrammarError> {
        self.min_max(other, char_set, true)
    }

    fn min_max(
        &self,
        other: &ASN1Value<'a>,
        char_set: Option<&BTreeMap<usize, char>>,
        getting_mininum: bool,
    ) -> Result<ASN1Value<'a>, GrammarError> {
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
                    return Err(grammar_error!(
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
                    _ => Err(grammar_error!(
                            UnpackingError,
                            "Failed to find ASN1Values {self:?} and {other:?} in character set {char_set:?}",
                        )),
                }
            }
            _ => Err(grammar_error!(
                UnpackingError,
                "Unsupported operation for ASN1Values {self:?} and {other:?}",
            )),
        }
    }

    pub fn unwrap_as_integer(&self) -> Result<i128, GrammarError> {
        if let ASN1Value::Integer(i) = self {
            Ok(*i)
        } else {
            Err(grammar_error!(
                UnpackingError,
                "Cannot unwrap {self:?} as integer!"
            ))
        }
    }
}

/// Intermediate placeholder for a type declared in
/// some other part of the ASN1 specification that is
/// being parsed or in one of its imports.
#[derive(Debug, Clone, PartialEq)]
pub struct DeclarationElsewhere<'a> {
    /// Chain of parent declaration leading back to a basic ASN1 type
    pub parent: Option<Cow<'a, str>>,
    pub identifier: DefinedType<'a>,
    pub constraints: Vec<Constraint<'a>>,
}

impl<'a>
    From<(
        Option<&'a str>,
        DefinedType<'a>,
        Option<Vec<Constraint<'a>>>,
    )> for DeclarationElsewhere<'a>
{
    fn from(
        value: (
            Option<&'a str>,
            DefinedType<'a>,
            Option<Vec<Constraint<'a>>>,
        ),
    ) -> Self {
        DeclarationElsewhere {
            parent: value.0.map(Cow::Borrowed),
            identifier: value.1,
            constraints: value.2.unwrap_or_default(),
        }
    }
}

/// Representation of a DefinedType as defined in X.680 §14
/// `ParameterizedType` and `ParameterizedValueSetType` cannot be distinguished
/// by the lexer and are grouped into a single variant
#[cfg_attr(test, derive(EnumDebug))]
#[cfg_attr(not(test), derive(Debug))]
#[derive(Clone, PartialEq)]
pub enum DefinedType<'a> {
    ExternalTypeReference {
        modulereference: Cow<'a, str>,
        typereference: Cow<'a, str>,
    },
    TypeReference(Cow<'a, str>),
    ParameterizedTypeOrValueSetType {
        simple_defined_type: Box<DefinedType<'a>>,
        actual_parameter_list: Vec<Parameter<'a>>,
    },
}

impl<'a> DefinedType<'a> {
    pub fn as_typereference(&self) -> Option<&str> {
        match self {
            Self::TypeReference(s) => Some(&s),
            _ => None,
        }
    }
}

impl<'a> Into<DefinedType<'a>> for &'a str {
    fn into(self) -> DefinedType<'a> {
        DefinedType::TypeReference(Cow::Borrowed(self))
    }
}

/// Intermediate placeholder for a value declared in
/// some other part of the ASN1 specification that is
/// being parsed or in one of its imports.
#[derive(Debug, Clone, PartialEq)]
pub struct ElsewhereDefinedValue<'a> {
    /// Chain of parent declaration leading back to a basic ASN1 value
    pub parent: Option<String>,
    pub identifier: DefinedValue<'a>,
}

/// Representation of a DefinedValue as defined in X.680 §14
#[cfg_attr(test, derive(EnumDebug))]
#[cfg_attr(not(test), derive(Debug))]
#[derive(Clone, PartialEq)]
pub enum DefinedValue<'a> {
    ExternalValueReference {
        modulereference: Cow<'a, str>,
        valuereference: Cow<'a, str>,
    },
    ValueReference(Cow<'a, str>),
    ParameterizedValue {
        simple_defined_value: Box<DefinedValue<'a>>,
        actual_parameter_list: Vec<Parameter<'a>>,
    },
}

impl<'a> DefinedValue<'a> {
    pub fn as_valuereference(&self) -> Option<&str> {
        match self {
            Self::ValueReference(s) => Some(&s),
            _ => None,
        }
    }
}

impl<'a> Into<DefinedValue<'a>> for &'a str {
    fn into(self) -> DefinedValue<'a> {
        DefinedValue::ValueReference(Cow::Borrowed(self))
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
