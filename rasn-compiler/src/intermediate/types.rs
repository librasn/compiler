//! `types` contains representations for ASN.1's basic types, such as `BOOLEAN`s
//! or `SEQUENCE`s.
#[cfg(test)]
use internal_macros::EnumDebug;
use std::vec;

#[cfg(doc)]
use crate::Backend;

use super::{constraints::*, *};

/// Trait shared by ASN1 `SET`, `SEQUENCE`, AND `CHOICE` that allows iterating
/// over their field types.
pub trait IterNameTypes {
    fn iter_name_types(&self) -> impl Iterator<Item = (&str, &ASN1Type)>;
}

/// Convenience trait for processing members of constructed types (`SEQUENCE`, `SET`) and `CHOICE`s.
pub trait MemberOrOption {
    const IS_CHOICE_OPTION: bool;
    fn name(&self) -> &str;
    fn ty(&self) -> &ASN1Type;
    fn constraints(&self) -> &[Constraint];
    fn is_recursive(&self) -> bool;
    fn tag(&self) -> Option<&AsnTag>;
}

/// Trait shared by all ASN1 types that can be constrained a.k.a subtyped.
/// *See also Rec. ITU-T X.680 (02/2021) §49 - §51*
pub trait Constrainable {
    /// returns a reference to the type's constraints
    fn constraints(&self) -> &Vec<Constraint>;
    /// returns a mutable reference to the type's constraints
    fn constraints_mut(&mut self) -> &mut Vec<Constraint>;
}

macro_rules! constrainable {
    ($typ:ty) => {
        impl Constrainable for $typ {
            fn constraints(&self) -> &Vec<Constraint> {
                &self.constraints
            }

            fn constraints_mut(&mut self) -> &mut Vec<Constraint> {
                &mut self.constraints
            }
        }
    };
}

constrainable!(Boolean);
constrainable!(Integer);
constrainable!(BitString);
constrainable!(OctetString);
constrainable!(CharacterString);
constrainable!(Real);
constrainable!(SequenceOrSet);
constrainable!(SequenceOrSetOf);
constrainable!(Choice);
constrainable!(Enumerated);
constrainable!(DeclarationElsewhere);
constrainable!(InformationObjectFieldReference);
constrainable!(Time);

/// Representation of an ASN1 BOOLEAN data element
/// with corresponding constraints.
/// *As defined in Rec. ITU-T X.680 (02/2021) §18*
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Boolean {
    pub constraints: Vec<Constraint>,
}

impl From<Option<Vec<Constraint>>> for Boolean {
    fn from(value: Option<Vec<Constraint>>) -> Self {
        Self {
            constraints: value.unwrap_or_default(),
        }
    }
}

/// Representation of an ASN1 INTEGER data element
/// with corresponding constraints and distinguished values.
/// *As defined in Rec. ITU-T X.680 (02/2021) §19*
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Integer {
    pub constraints: Vec<Constraint>,
    pub distinguished_values: Option<Vec<DistinguishedValue>>,
}

impl Integer {
    /// Returns the [IntegerType] of `self`.
    /// The [IntegerType] describes the absolute range of an integer
    pub fn int_type(&self) -> IntegerType {
        self.constraints
            .iter()
            .fold(IntegerType::Unbounded, |acc, c| {
                c.integer_constraints().max_restrictive(acc)
            })
    }
}

impl From<(i128, i128, bool)> for Integer {
    fn from(value: (i128, i128, bool)) -> Self {
        Self {
            constraints: vec![Constraint::SubtypeConstraint(ElementSet {
                set: ElementOrSetOperation::Element(SubtypeElement::ValueRange {
                    min: Some(ASN1Value::Integer(value.0)),
                    max: Some(ASN1Value::Integer(value.1)),
                    extensible: value.2,
                }),
                extensible: value.2,
            })],
            distinguished_values: None,
        }
    }
}

impl From<(Option<i128>, Option<i128>, bool)> for Integer {
    fn from(value: (Option<i128>, Option<i128>, bool)) -> Self {
        Self {
            constraints: vec![Constraint::SubtypeConstraint(ElementSet {
                set: ElementOrSetOperation::Element(SubtypeElement::ValueRange {
                    min: value.0.map(ASN1Value::Integer),
                    max: value.1.map(ASN1Value::Integer),
                    extensible: value.2,
                }),
                extensible: value.2,
            })],
            distinguished_values: None,
        }
    }
}

impl
    From<(
        &str,
        Option<Vec<DistinguishedValue>>,
        Option<Vec<Constraint>>,
    )> for Integer
{
    fn from(
        value: (
            &str,
            Option<Vec<DistinguishedValue>>,
            Option<Vec<Constraint>>,
        ),
    ) -> Self {
        Self {
            constraints: value.2.unwrap_or_default(),
            distinguished_values: value.1,
        }
    }
}

/// Representation of an ASN1 REAL data element
/// with corresponding constraints.
/// *As defined in Rec. ITU-T X.680 (02/2021) §21*
#[derive(Debug, Clone, PartialEq)]
pub struct Real {
    pub constraints: Vec<Constraint>,
}

impl From<Option<Vec<Constraint>>> for Real {
    fn from(value: Option<Vec<Constraint>>) -> Self {
        Self {
            constraints: value.unwrap_or_default(),
        }
    }
}

/// Representation of an ASN1 GeneralizedTime data element
/// with corresponding constraints.
/// *As defined in Rec. ITU-T X.680 (02/2021) §46*
#[derive(Debug, Clone, PartialEq)]
pub struct GeneralizedTime {
    pub constraints: Vec<Constraint>,
}

/// Representation of an ASN1 Universal time (a.k.a UTCTime)
/// data element with corresponding constraints.
/// *As defined in Rec. ITU-T X.680 (02/2021) §47*
#[derive(Debug, Clone, PartialEq)]
pub struct UTCTime {
    pub constraints: Vec<Constraint>,
}

/// Representation of an ASN1 OCTET STRING data element
/// with corresponding constraints.
/// *As defined in Rec. ITU-T X.680 (02/2021) §23*
#[derive(Debug, Clone, PartialEq)]
pub struct OctetString {
    pub constraints: Vec<Constraint>,
}

impl From<Option<Vec<Constraint>>> for OctetString {
    fn from(value: Option<Vec<Constraint>>) -> Self {
        OctetString {
            constraints: value.unwrap_or_default(),
        }
    }
}

/// Representation of an ASN1 BIT STRING data element
/// with corresponding constraints and distinguished values
/// defining the individual bits.
/// *As defined in Rec. ITU-T X.680 (02/2021) §22*
#[derive(Debug, Clone, PartialEq)]
pub struct BitString {
    pub constraints: Vec<Constraint>,
    pub distinguished_values: Option<Vec<DistinguishedValue>>,
}

impl From<(Option<Vec<DistinguishedValue>>, Option<Vec<Constraint>>)> for BitString {
    fn from(value: (Option<Vec<DistinguishedValue>>, Option<Vec<Constraint>>)) -> Self {
        BitString {
            constraints: value.1.unwrap_or_default(),
            distinguished_values: value.0,
        }
    }
}

/// Representation of an ASN1 OBJECT IDENTIFIER data element
/// with corresponding constraints.
/// *As defined in Rec. ITU-T X.680 (02/2021) §32*
#[derive(Debug, Clone, PartialEq)]
pub struct ObjectIdentifier {
    pub constraints: Vec<Constraint>,
}

impl From<Option<Vec<Constraint>>> for ObjectIdentifier {
    fn from(value: Option<Vec<Constraint>>) -> Self {
        ObjectIdentifier {
            constraints: value.unwrap_or_default(),
        }
    }
}

/// Representation of an ASN1 TIME data element
/// with corresponding constraints.
/// *As defined in Rec. ITU-T X.680 (02/2021) §38*
#[derive(Debug, Clone, PartialEq)]
pub struct Time {
    pub constraints: Vec<Constraint>,
}

impl From<Option<Vec<Constraint>>> for Time {
    fn from(value: Option<Vec<Constraint>>) -> Self {
        Time {
            constraints: value.unwrap_or_default(),
        }
    }
}

/// Representation of an ASN1 Character String type data element
/// with corresponding constraints. ASN1 Character String types
/// include IA5String, UTF8String, VideotexString.
/// *As defined in Rec. ITU-T X.680 (02/2021) §39-*§44
#[derive(Debug, Clone, PartialEq)]
pub struct CharacterString {
    pub constraints: Vec<Constraint>,
    pub ty: CharacterStringType,
}

impl From<(&str, Option<Vec<Constraint>>)> for CharacterString {
    fn from(value: (&str, Option<Vec<Constraint>>)) -> Self {
        CharacterString {
            constraints: value.1.unwrap_or_default(),
            ty: value.0.into(),
        }
    }
}

/// Representation of an ASN1 SEQUENCE OF and SET OF data element
/// with corresponding constraints and element type info
/// Whether the struct describes a SEQUENCE OF or a SET OF
/// is identified by the `ASN1Type` enum variant that
/// holds this struct as a value (i.e. `ASN1Type::SetOf(SequenceOrSetOf { .. })`
/// or `ASN1Type::SequenceOf(SequenceOrSetOf { .. })`).
/// *As defined in Rec. ITU-T X.680 (02/2021) §26 and §28*
#[derive(Debug, Clone, PartialEq)]
pub struct SequenceOrSetOf {
    pub constraints: Vec<Constraint>,
    /// [ASN.1 type](ASN1Type) of the individual elements of the collection
    /// ### Example
    /// The ASN.1 type
    /// ```ignore
    /// Sequence-of-booleans ::= SEQUENCE OF BOOLEAN
    /// ```
    /// will have an `element_type` field of
    /// ```
    /// # use rasn_compiler::prelude::ir::*;
    /// # let test =
    /// Box::new(ASN1Type::Boolean(Boolean { constraints: vec![] } ))
    /// # ;
    /// ```
    pub element_type: Box<ASN1Type>,
    pub element_tag: Option<AsnTag>,
    pub is_recursive: bool,
}

impl From<(Option<Vec<Constraint>>, (Option<AsnTag>, ASN1Type))> for SequenceOrSetOf {
    fn from(value: (Option<Vec<Constraint>>, (Option<AsnTag>, ASN1Type))) -> Self {
        Self {
            constraints: value.0.unwrap_or_default(),
            element_type: Box::new(value.1 .1),
            element_tag: value.1 .0,
            is_recursive: false,
        }
    }
}

/// Representation of an ASN1 SEQUENCE or SET data element
/// with corresponding members and extension information.
/// Whether the struct describes a SEQUENCE or a SET
/// is identified by the `ASN1Type` enum variant that
/// holds this struct as a value (i.e. `ASN1Type::Set(SequenceOrSet { .. })`
/// or `ASN1Type::Sequence(SequenceOrSet { .. })`).
/// *As defined in Rec. ITU-T X.680 (02/2021) §25 and §27*
#[derive(Debug, Clone, PartialEq)]
pub struct SequenceOrSet {
    pub fixed_components: Vec<SequenceComponent>,
    pub extension_components: Vec<SequenceComponent>,
    pub suffix_components: Vec<SequenceComponent>,
    pub extensible: bool,
    pub constraints: Vec<Constraint>,
}

impl SequenceOrSet {
    pub fn direct_members(&self) -> impl Iterator<Item=&SequenceOrSetMember> {
        [&self.fixed_components, &self.extension_components, &self.suffix_components]
            .into_iter()
            .flat_map(|x| x)
            .flat_map(|x| {
                if let SequenceComponent::Member(m) = x {
                    Some(m)
                } else {
                    None
                }
            })
    }

    pub fn direct_members_mut(&mut self) -> impl Iterator<Item=&mut SequenceOrSetMember> {
        [&mut self.fixed_components, &mut self.extension_components, &mut self.suffix_components]
            .into_iter()
            .flat_map(|x| x)
            .flat_map(|x| {
                if let SequenceComponent::Member(m) = x {
                    Some(m)
                } else {
                    None
                }
            })
    }

    pub fn extension_range(&self) -> std::ops::Range<usize> {
        self.fixed_components.len() .. self.fixed_components.len() + self.extension_components.len()
    }
}

impl IterNameTypes for SequenceOrSet {
    fn iter_name_types(&self) -> impl Iterator<Item = (&str, &ASN1Type)> {
        self.direct_members()
            .map(|m| (m.name.as_str(), &m.ty))
    }
}

impl
    From<(
        (
            Vec<SequenceComponent>,
            Option<(
                Vec<SequenceComponent>,
                Vec<SequenceComponent>,
            )>,
        ),
        Option<Vec<Constraint>>,
    )> for SequenceOrSet
{
    fn from(
        mut value: (
            (
                Vec<SequenceComponent>,
                Option<(
                    Vec<SequenceComponent>,
                    Vec<SequenceComponent>,
                )>,
            ),
            Option<Vec<Constraint>>,
        ),
    ) -> Self {
        let extensible = value.0 .1.is_some();
        let (extension_components, suffix_components) = value.0 .1.unwrap_or_default();

        SequenceOrSet {
            fixed_components: value.0 .0,
            extension_components,
            suffix_components,
            constraints: value.1.unwrap_or_default(),
            extensible,
        }
    }
}

impl
    From<(
        (
            Vec<SequenceComponent>,
            Option<ExtensionMarker>,
            Option<Vec<SequenceComponent>>,
        ),
        Option<Vec<Constraint>>,
    )> for SequenceOrSet
{
    fn from(
        mut value: (
            (
                Vec<SequenceComponent>,
                Option<ExtensionMarker>,
                Option<Vec<SequenceComponent>>,
            ),
            Option<Vec<Constraint>>,
        ),
    ) -> Self {
        SequenceOrSet {
            fixed_components: value.0 .0.into_iter().collect(),
            extension_components: value.0 .2.into_iter().flat_map(|x| x).collect(),
            suffix_components: vec![],
            constraints: value.1.unwrap_or_default(),
            extensible: value.0 .1.is_some(),
        }
    }
}

/// Intermediate parsing type to parse COMPONENTS OF notation.
/// `SequenceComponent` is an intermediary type that implementors of
/// a [Backend] will usually not interact with.
/// When parsing the body of an ASN.1 SEQUENCE or SET, the lexer
/// distinguishes between a group of members (`SequenceComponent::ComponentsOf`) that is imnported from
/// another ASN.1 data element using the `COMPONENTS OF` notation
/// (i.e. `Extending-Sequence ::= SEQUENCE { COMPONENTS OF Another-Sequence, added-field BOOLEAN }`)
/// and the regular member declaration (`SequenceComponent::Member`)
/// (i.e. `Simple-Sequence ::= SEQUENCE { field BOOLEAN }`).
/// When the lexer assembles the complete [SequenceOrSet] struct,
/// it groups the parsed `SequenceComponent` items into the `members`
/// and `components_of` fields of the [SequenceOrSet] struct. The linker
/// will subsequently try to resolve the `components_of` identifiers.
#[cfg_attr(test, derive(EnumDebug))]
#[cfg_attr(not(test), derive(Debug))]
#[derive(Clone, PartialEq)]
pub enum SequenceComponent {
    Member(SequenceOrSetMember),
    ComponentsOf(String),
}

/// Representation of an single ASN1 SEQUENCE or SET member.
/// ### Example
/// The ASN.1 SEQUENCE defined as
/// ```ignore
/// Test-Sequence ::= SEQUENCE {
///     int-member [0] INTEGER (0..2) DEFAULT 1
/// }
/// ```
/// defines one member, which is representated as follows
/// ```
/// # use rasn_compiler::prelude::ir::*;
/// # let test =
/// SequenceOrSetMember {
///     is_recursive: false,
///     name: String::from("int-member"),
///     tag: Some(AsnTag {
///         environment: TaggingEnvironment::Automatic,
///         tag_class: TagClass::ContextSpecific,
///         id: 0,
///     }),
///     ty: ASN1Type::Integer(Integer {
///         constraints: vec![
///             Constraint::SubtypeConstraint(ElementSet {
///                 set: ElementOrSetOperation::Element(SubtypeElement::ValueRange {
///                     min: Some(ASN1Value::Integer(0)),
///                     max: Some(ASN1Value::Integer(2)),
///                     extensible: false
///                 }),
///                 extensible: false
///            })
///         ],
///         distinguished_values: None,
///     }),
///     default_value: Some(ASN1Value::Integer(1)),
///     is_optional: true,
///     constraints: vec![]
/// }
/// # ;
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct SequenceOrSetMember {
    pub name: String,
    pub tag: Option<AsnTag>,
    pub ty: ASN1Type,
    pub default_value: Option<ASN1Value>,
    pub is_optional: bool,
    pub is_recursive: bool,
    pub constraints: Vec<Constraint>,
}

impl MemberOrOption for SequenceOrSetMember {
    fn name(&self) -> &str {
        &self.name
    }

    fn ty(&self) -> &ASN1Type {
        &self.ty
    }

    fn constraints(&self) -> &[Constraint] {
        &self.constraints
    }

    fn is_recursive(&self) -> bool {
        self.is_recursive
    }

    fn tag(&self) -> Option<&AsnTag> {
        self.tag.as_ref()
    }

    const IS_CHOICE_OPTION: bool = false;
}

impl
    From<(
        &str,
        Option<AsnTag>,
        ASN1Type,
        Option<Vec<Constraint>>,
        Option<OptionalMarker>,
        Option<ASN1Value>,
    )> for SequenceOrSetMember
{
    fn from(
        value: (
            &str,
            Option<AsnTag>,
            ASN1Type,
            Option<Vec<Constraint>>,
            Option<OptionalMarker>,
            Option<ASN1Value>,
        ),
    ) -> Self {
        SequenceOrSetMember {
            name: value.0.into(),
            tag: value.1,
            ty: value.2,
            is_optional: value.4.is_some() || value.5.is_some(),
            default_value: value.5,
            is_recursive: false,
            constraints: value.3.unwrap_or_default(),
        }
    }
}

/// Representation of an ASN1 CHOICE data element
/// with corresponding members and extension information.
/// *As defined in Rec. ITU-T X.680 (02/2021) §29*
#[derive(Debug, Clone, PartialEq)]
pub struct Choice {
    pub extensible: Option<usize>,
    pub options: Vec<ChoiceOption>,
    pub constraints: Vec<Constraint>,
}

impl IterNameTypes for Choice {
    fn iter_name_types(&self) -> impl Iterator<Item = (&str, &ASN1Type)> {
        self.options.iter().map(|o| (o.name.as_str(), &o.ty))
    }
}

impl
    From<(
        Vec<ChoiceOption>,
        Option<ExtensionMarker>,
        Option<Vec<ChoiceOption>>,
    )> for Choice
{
    fn from(
        mut value: (
            Vec<ChoiceOption>,
            Option<ExtensionMarker>,
            Option<Vec<ChoiceOption>>,
        ),
    ) -> Self {
        let index_of_first_extension = value.0.len();
        value.0.append(&mut value.2.unwrap_or_default());
        Choice {
            extensible: value.1.map(|_| index_of_first_extension),
            options: value.0,
            constraints: vec![],
        }
    }
}

/// Representation of an single ASN1 CHOICE option.
/// ### Example
/// The ASN.1 CHOICE defined as
/// ```ignore
/// Test-Choice ::= CHOICE {
///     boolean-option [0] BOOLEAN
/// }
/// ```
/// defines one option, which is representated as follows
/// ```
/// # use rasn_compiler::prelude::ir::*;
/// # let test =
/// ChoiceOption {
///     name: String::from("boolean-option"),
///     is_recursive: false,
///     tag: Some(AsnTag {
///         environment: TaggingEnvironment::Automatic,
///         tag_class: TagClass::ContextSpecific,
///         id: 0,
///     }),
///     ty: ASN1Type::Boolean(Boolean {
///         constraints: vec![]
///     }),
///     constraints: vec![]
/// }
/// # ;
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct ChoiceOption {
    pub name: String,
    pub tag: Option<AsnTag>,
    pub ty: ASN1Type,
    pub constraints: Vec<Constraint>,
    pub is_recursive: bool,
}

impl MemberOrOption for ChoiceOption {
    fn name(&self) -> &str {
        &self.name
    }

    fn ty(&self) -> &ASN1Type {
        &self.ty
    }

    fn constraints(&self) -> &[Constraint] {
        &self.constraints
    }

    fn is_recursive(&self) -> bool {
        self.is_recursive
    }

    fn tag(&self) -> Option<&AsnTag> {
        self.tag.as_ref()
    }

    const IS_CHOICE_OPTION: bool = true;
}

impl From<(&str, Option<AsnTag>, ASN1Type, Option<Vec<Constraint>>)> for ChoiceOption {
    fn from(value: (&str, Option<AsnTag>, ASN1Type, Option<Vec<Constraint>>)) -> Self {
        ChoiceOption {
            name: value.0.into(),
            tag: value.1,
            ty: value.2,
            constraints: value.3.unwrap_or_default(),
            is_recursive: false,
        }
    }
}

/// Representation of an ASN1 ENUMERATED data element
/// with corresponding enumerals and extension information.
/// *As defined in Rec. ITU-T X.680 (02/2021) §20*
#[derive(Debug, Clone, PartialEq)]
pub struct Enumerated {
    pub members: Vec<Enumeral>,
    pub extensible: Option<usize>,
    pub constraints: Vec<Constraint>,
}

impl
    From<(
        Vec<Enumeral>,
        Option<ExtensionMarker>,
        Option<Vec<Enumeral>>,
    )> for Enumerated
{
    fn from(
        mut value: (
            Vec<Enumeral>,
            Option<ExtensionMarker>,
            Option<Vec<Enumeral>>,
        ),
    ) -> Self {
        let index_of_first_extension = value.0.len();
        value.0.append(&mut value.2.unwrap_or_default());
        Enumerated {
            members: value.0,
            extensible: value.1.map(|_| index_of_first_extension),
            constraints: vec![],
        }
    }
}

/// Representation of a single member/enumeral of an ASN1
/// ENUMERATED data element.
/// ### Example
/// The ASN.1 ENUMERATED defined as
/// ```ignore
/// Test-Enum ::= ENUMERATED {
///     first-item(7) -- This is the first item of Test-Enum
/// }
/// ```
/// defines one option, which is representated as follows
/// ```
/// # use rasn_compiler::prelude::ir::*;
/// # let test =
/// Enumeral {
///     name: String::from("first-item"),
///     description: Some(String::from(" This is the first item of Test-Enum")),
///     index: 7
/// }
/// # ;
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Enumeral {
    pub name: String,
    pub description: Option<String>,
    pub index: i128,
}

/// Representation of a ASN1 distinguished value,
/// as seen in some INTEGER and BIT STRING declarations
/// *As defined in Rec. ITU-T X.680 (02/2021) §19.5 and §22.4*
#[derive(Debug, Clone, PartialEq)]
pub struct DistinguishedValue {
    pub name: String,
    pub value: i128,
}

impl From<(&str, i128)> for DistinguishedValue {
    fn from(value: (&str, i128)) -> Self {
        Self {
            name: value.0.into(),
            value: value.1,
        }
    }
}

/// Representation of a ASN1 selection type as used with ASN1 CHOICEs
/// *As defined in Rec. ITU-T X.680 (02/2021) §30*
#[derive(Debug, Clone, PartialEq)]
pub struct ChoiceSelectionType {
    pub choice_name: String,
    pub selected_option: String,
}

impl From<(&str, &str)> for ChoiceSelectionType {
    fn from(value: (&str, &str)) -> Self {
        Self {
            choice_name: value.1.into(),
            selected_option: value.0.into(),
        }
    }
}
