//! `types` contains representations for ASN.1's basic types, such as `BOOLEAN`s
//! or `SEQUENCE`s.
#[cfg(test)]
use internal_macros::EnumDebug;
use std::vec;

#[cfg(doc)]
use crate::Backend;

use super::{constraints::*, *};

/// Defines the optionality of a field.
#[derive(Debug, Clone, PartialEq)]
pub enum Optionality<T> {
    /// All definitions are required to specify this field.
    Required,
    /// The field can be left undefined.
    Optional,
    /// Default if the field is omitted.
    Default(T),
}

impl<T> Optionality<T> {
    /// Get a reference to the default `T`, or None if there is no default.
    pub fn default(&self) -> Option<&T> {
        match self {
            Optionality::Required | Optionality::Optional => None,
            Optionality::Default(d) => Some(d),
        }
    }

    /// Get a mutable reference to the default `T`, or None if there is no default.
    pub fn default_mut(&mut self) -> Option<&mut T> {
        match self {
            Optionality::Required | Optionality::Optional => None,
            Optionality::Default(d) => Some(d),
        }
    }
}

/// Trait shared by ASN1 `SET`, `SEQUENCE`, AND `CHOICE` that allows iterating
/// over their field types.
pub trait IterNameTypes {
    fn iter_name_types(&self) -> impl Iterator<Item = (&str, &ASN1Type)>;
}

pub trait IterMembers<'a> {
    type Member: MemberOrOption<'a>;
    fn iter_members<'b>(&'b self) -> impl Iterator<Item = &'b Self::Member>
    where
        'a: 'b;
    fn iter_mut_members<'b>(&'b mut self) -> impl Iterator<Item = &'b mut Self::Member>
    where
        'a: 'b;
}

/// Convenience trait for processing members of constructed types (`SEQUENCE`, `SET`) and `CHOICE`s.
pub trait MemberOrOption<'a> {
    const IS_CHOICE_OPTION: bool;
    fn name(&self) -> Cow<'a, str>;
    fn ty(&self) -> &ASN1Type<'a>;
    fn ty_mut(&mut self) -> &mut ASN1Type<'a>;
    fn is_recursive(&self) -> bool;
    fn tag(&self) -> Option<&AsnTag>;
}

/// Trait shared by all ASN1 types that can be constrained a.k.a subtyped.
/// *See also Rec. ITU-T X.680 (02/2021) §49 - §51*
pub trait Constrainable<'a> {
    /// returns a reference to the type's constraints
    fn constraints(&self) -> &Vec<Constraint<'a>>;
    /// returns a mutable reference to the type's constraints
    fn constraints_mut(&mut self) -> &mut Vec<Constraint<'a>>;
}

macro_rules! constrainable {
    ($typ:ty) => {
        impl<'a> Constrainable<'a> for $typ {
            fn constraints(&self) -> &Vec<Constraint<'a>> {
                &self.constraints
            }

            fn constraints_mut(&mut self) -> &mut Vec<Constraint<'a>> {
                &mut self.constraints
            }
        }
    };
}

constrainable!(Boolean<'a>);
constrainable!(Integer<'a>);
constrainable!(BitString<'a>);
constrainable!(OctetString<'a>);
constrainable!(CharacterString<'a>);
constrainable!(Real<'a>);
constrainable!(SequenceOrSet<'a>);
constrainable!(SequenceOrSetOf<'a>);
constrainable!(Choice<'a>);
constrainable!(Enumerated<'a>);
constrainable!(DeclarationElsewhere<'a>);
constrainable!(ObjectClassFieldType<'a>);
constrainable!(Time<'a>);

/// Representation of an ASN1 BOOLEAN data element
/// with corresponding constraints.
/// *As defined in Rec. ITU-T X.680 (02/2021) §18*
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Boolean<'a> {
    pub constraints: Vec<Constraint<'a>>,
}

impl<'a> From<Option<Vec<Constraint<'a>>>> for Boolean<'a> {
    fn from(value: Option<Vec<Constraint<'a>>>) -> Self {
        Self {
            constraints: value.unwrap_or_default(),
        }
    }
}

/// Representation of an ASN1 INTEGER data element
/// with corresponding constraints and distinguished values.
/// *As defined in Rec. ITU-T X.680 (02/2021) §19*
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Integer<'a> {
    pub constraints: Vec<Constraint<'a>>,
    pub distinguished_values: Option<Vec<DistinguishedValue<'a>>>,
}

impl<'a> Integer<'a> {
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

impl<'a> From<(i128, i128, bool)> for Integer<'a> {
    fn from(value: (i128, i128, bool)) -> Self {
        Self {
            constraints: vec![Constraint::Subtype(ElementSetSpecs {
                set: ElementOrSetOperation::Element(SubtypeElements::ValueRange {
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

impl<'a> From<(Option<i128>, Option<i128>, bool)> for Integer<'a> {
    fn from(value: (Option<i128>, Option<i128>, bool)) -> Self {
        Self {
            constraints: vec![Constraint::Subtype(ElementSetSpecs {
                set: ElementOrSetOperation::Element(SubtypeElements::ValueRange {
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

impl<'a>
    From<(
        &str,
        Option<Vec<DistinguishedValue<'a>>>,
        Option<Vec<Constraint<'a>>>,
    )> for Integer<'a>
{
    fn from(
        value: (
            &str,
            Option<Vec<DistinguishedValue<'a>>>,
            Option<Vec<Constraint<'a>>>,
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
pub struct Real<'a> {
    pub constraints: Vec<Constraint<'a>>,
}

impl<'a> From<Option<Vec<Constraint<'a>>>> for Real<'a> {
    fn from(value: Option<Vec<Constraint<'a>>>) -> Self {
        Self {
            constraints: value.unwrap_or_default(),
        }
    }
}

/// Representation of an ASN1 GeneralizedTime data element
/// with corresponding constraints.
/// *As defined in Rec. ITU-T X.680 (02/2021) §46*
#[derive(Debug, Clone, PartialEq)]
pub struct GeneralizedTime<'a> {
    pub constraints: Vec<Constraint<'a>>,
}

/// Representation of an ASN1 Universal time (a.k.a UTCTime)
/// data element with corresponding constraints.
/// *As defined in Rec. ITU-T X.680 (02/2021) §47*
#[derive(Debug, Clone, PartialEq)]
pub struct UTCTime<'a> {
    pub constraints: Vec<Constraint<'a>>,
}

/// Representation of an ASN1 OCTET STRING data element
/// with corresponding constraints.
/// *As defined in Rec. ITU-T X.680 (02/2021) §23*
#[derive(Debug, Clone, PartialEq)]
pub struct OctetString<'a> {
    pub constraints: Vec<Constraint<'a>>,
}

impl<'a> From<Option<Vec<Constraint<'a>>>> for OctetString<'a> {
    fn from(value: Option<Vec<Constraint<'a>>>) -> Self {
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
pub struct BitString<'a> {
    pub constraints: Vec<Constraint<'a>>,
    pub distinguished_values: Option<Vec<DistinguishedValue<'a>>>,
}

impl<'a>
    From<(
        Option<Vec<DistinguishedValue<'a>>>,
        Option<Vec<Constraint<'a>>>,
    )> for BitString<'a>
{
    fn from(
        value: (
            Option<Vec<DistinguishedValue<'a>>>,
            Option<Vec<Constraint<'a>>>,
        ),
    ) -> Self {
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
pub struct ObjectIdentifier<'a> {
    pub constraints: Vec<Constraint<'a>>,
}

impl<'a> From<Option<Vec<Constraint<'a>>>> for ObjectIdentifier<'a> {
    fn from(value: Option<Vec<Constraint<'a>>>) -> Self {
        ObjectIdentifier {
            constraints: value.unwrap_or_default(),
        }
    }
}

/// Representation of an ASN1 TIME data element
/// with corresponding constraints.
/// *As defined in Rec. ITU-T X.680 (02/2021) §38*
#[derive(Debug, Clone, PartialEq)]
pub struct Time<'a> {
    pub constraints: Vec<Constraint<'a>>,
}

impl<'a> From<Option<Vec<Constraint<'a>>>> for Time<'a> {
    fn from(value: Option<Vec<Constraint<'a>>>) -> Self {
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
pub struct CharacterString<'a> {
    pub constraints: Vec<Constraint<'a>>,
    pub ty: CharacterStringType,
}

impl<'a> From<(&str, Option<Vec<Constraint<'a>>>)> for CharacterString<'a> {
    fn from(value: (&str, Option<Vec<Constraint<'a>>>)) -> Self {
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
pub struct SequenceOrSetOf<'a> {
    pub constraints: Vec<Constraint<'a>>,
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
    pub element_type: Box<ASN1Type<'a>>,
    pub element_tag: Option<AsnTag>,
    pub is_recursive: bool,
}

impl<'a> From<(Option<Vec<Constraint<'a>>>, (Option<AsnTag>, ASN1Type<'a>))>
    for SequenceOrSetOf<'a>
{
    fn from(value: (Option<Vec<Constraint<'a>>>, (Option<AsnTag>, ASN1Type<'a>))) -> Self {
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
pub struct SequenceOrSet<'a> {
    pub components_of: Vec<Cow<'a, str>>,
    pub extensible: Option<usize>,
    pub constraints: Vec<Constraint<'a>>,
    pub members: Vec<SequenceOrSetMember<'a>>,
}

impl<'a> IterNameTypes for SequenceOrSet<'a> {
    fn iter_name_types(&self) -> impl Iterator<Item = (&str, &ASN1Type)> {
        self.members.iter().map(|m| (&*m.name, &m.ty))
    }
}

impl<'a> IterMembers<'a> for SequenceOrSet<'a> {
    type Member = SequenceOrSetMember<'a>;

    fn iter_members<'b>(&'b self) -> impl Iterator<Item = &'b Self::Member>
    where
        'a: 'b,
    {
        self.members.iter()
    }

    fn iter_mut_members<'b>(&'b mut self) -> impl Iterator<Item = &'b mut Self::Member>
    where
        'a: 'b,
    {
        self.members.iter_mut()
    }
}

impl<'a>
    From<(
        (
            Vec<SequenceComponent<'a>>,
            Option<ExtensionMarker>,
            Option<Vec<SequenceComponent<'a>>>,
        ),
        Option<Vec<Constraint<'a>>>,
    )> for SequenceOrSet<'a>
{
    fn from(
        mut value: (
            (
                Vec<SequenceComponent<'a>>,
                Option<ExtensionMarker>,
                Option<Vec<SequenceComponent<'a>>>,
            ),
            Option<Vec<Constraint<'a>>>,
        ),
    ) -> Self {
        let index_of_first_extension = value.0 .0.len();
        value.0 .0.append(&mut value.0 .2.unwrap_or_default());
        let mut components_of = vec![];
        let mut members = vec![];
        for comp in value.0 .0 {
            match comp {
                SequenceComponent::Member(m) => members.push(m),
                SequenceComponent::ComponentsOf(c) => components_of.push(c),
            }
        }
        SequenceOrSet {
            components_of,
            constraints: value.1.unwrap_or_default(),
            extensible: value.0 .1.map(|_| index_of_first_extension),
            members,
        }
    }
}

impl<'a>
    From<(
        (
            Vec<SequenceOrSetMember<'a>>,
            Option<ExtensionMarker>,
            Option<Vec<SequenceOrSetMember<'a>>>,
        ),
        Option<Vec<Constraint<'a>>>,
    )> for SequenceOrSet<'a>
{
    fn from(
        mut value: (
            (
                Vec<SequenceOrSetMember<'a>>,
                Option<ExtensionMarker>,
                Option<Vec<SequenceOrSetMember<'a>>>,
            ),
            Option<Vec<Constraint<'a>>>,
        ),
    ) -> Self {
        let index_of_first_extension = value.0 .0.len();
        value.0 .0.append(&mut value.0 .2.unwrap_or_default());
        SequenceOrSet {
            components_of: vec![],
            constraints: value.1.unwrap_or_default(),
            extensible: value.0 .1.map(|_| index_of_first_extension),
            members: value.0 .0,
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
#[allow(clippy::large_enum_variant)]
#[derive(Clone, PartialEq)]
pub enum SequenceComponent<'a> {
    Member(SequenceOrSetMember<'a>),
    ComponentsOf(Cow<'a, str>),
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
///     name: Cow::from("int-member"),
///     tag: Some(AsnTag {
///         environment: TaggingEnvironment::Automatic,
///         tag_class: TagClass::ContextSpecific,
///         id: 0,
///     }),
///     ty: ASN1Type::Integer(Integer {
///         constraints: vec![
///             Constraint::Subtype(ElementSetSpecs {
///                 set: ElementOrSetOperation::Element(SubtypeElements::ValueRange {
///                     min: Some(ASN1Value::Integer(0)),
///                     max: Some(ASN1Value::Integer(2)),
///                     extensible: false
///                 }),
///                 extensible: false
///            })
///         ],
///         distinguished_values: None,
///     }),
///     optionality: Optionality::Default(ASN1Value::Integer(1)),
///     constraints: vec![]
/// }
/// # ;
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct SequenceOrSetMember<'a> {
    pub name: Cow<'a, str>,
    pub tag: Option<AsnTag>,
    pub ty: ASN1Type<'a>,
    pub optionality: Optionality<ASN1Value<'a>>,
    pub is_recursive: bool,
}

impl<'a> MemberOrOption<'a> for SequenceOrSetMember<'a> {
    fn name(&self) -> Cow<'a, str> {
        self.name.clone()
    }

    fn ty(&self) -> &ASN1Type<'a> {
        &self.ty
    }

    fn is_recursive(&self) -> bool {
        self.is_recursive
    }

    fn tag(&self) -> Option<&AsnTag> {
        self.tag.as_ref()
    }

    const IS_CHOICE_OPTION: bool = false;

    fn ty_mut(&mut self) -> &mut ASN1Type<'a> {
        &mut self.ty
    }
}

impl<'a>
    From<(
        &'a str,
        Option<AsnTag>,
        ASN1Type<'a>,
        Optionality<ASN1Value<'a>>,
    )> for SequenceOrSetMember<'a>
{
    fn from(
        value: (
            &'a str,
            Option<AsnTag>,
            ASN1Type<'a>,
            Optionality<ASN1Value<'a>>,
        ),
    ) -> Self {
        SequenceOrSetMember {
            name: Cow::Borrowed(value.0),
            tag: value.1,
            ty: value.2,
            optionality: value.3,
            is_recursive: false,
        }
    }
}

/// Representation of an ASN1 CHOICE data element
/// with corresponding members and extension information.
/// *As defined in Rec. ITU-T X.680 (02/2021) §29*
#[derive(Debug, Clone, PartialEq)]
pub struct Choice<'a> {
    pub extensible: Option<usize>,
    pub options: Vec<ChoiceOption<'a>>,
    pub constraints: Vec<Constraint<'a>>,
}

impl<'a> IterNameTypes for Choice<'a> {
    fn iter_name_types(&self) -> impl Iterator<Item = (&str, &ASN1Type)> {
        self.options.iter().map(|o| (&*o.name, &o.ty))
    }
}

impl<'a> IterMembers<'a> for Choice<'a> {
    type Member = ChoiceOption<'a>;

    fn iter_members<'b>(&'b self) -> impl Iterator<Item = &'b Self::Member>
    where
        'a: 'b,
    {
        self.options.iter()
    }

    fn iter_mut_members<'b>(&'b mut self) -> impl Iterator<Item = &'b mut Self::Member>
    where
        'a: 'b,
    {
        self.options.iter_mut()
    }
}

impl<'a>
    From<(
        Vec<ChoiceOption<'a>>,
        Option<ExtensionMarker>,
        Option<Vec<ChoiceOption<'a>>>,
    )> for Choice<'a>
{
    fn from(
        mut value: (
            Vec<ChoiceOption<'a>>,
            Option<ExtensionMarker>,
            Option<Vec<ChoiceOption<'a>>>,
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
///     name: Cow::from("boolean-option"),
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
pub struct ChoiceOption<'a> {
    pub name: Cow<'a, str>,
    pub tag: Option<AsnTag>,
    pub ty: ASN1Type<'a>,
    pub is_recursive: bool,
}

impl<'a> MemberOrOption<'a> for ChoiceOption<'a> {
    fn name(&self) -> Cow<'a, str> {
        self.name.clone()
    }

    fn ty(&self) -> &ASN1Type<'a> {
        &self.ty
    }

    fn is_recursive(&self) -> bool {
        self.is_recursive
    }

    fn tag(&self) -> Option<&AsnTag> {
        self.tag.as_ref()
    }

    const IS_CHOICE_OPTION: bool = true;

    fn ty_mut(&mut self) -> &mut ASN1Type<'a> {
        &mut self.ty
    }
}

impl<'a> From<(&'a str, Option<AsnTag>, ASN1Type<'a>)> for ChoiceOption<'a> {
    fn from(value: (&'a str, Option<AsnTag>, ASN1Type<'a>)) -> Self {
        ChoiceOption {
            name: value.0.into(),
            tag: value.1,
            ty: value.2,
            is_recursive: false,
        }
    }
}

/// Representation of an ASN1 ENUMERATED data element
/// with corresponding enumerals and extension information.
/// *As defined in Rec. ITU-T X.680 (02/2021) §20*
#[derive(Debug, Clone, PartialEq)]
pub struct Enumerated<'a> {
    pub members: Vec<Enumeral<'a>>,
    pub extensible: Option<usize>,
    pub constraints: Vec<Constraint<'a>>,
}

impl<'a>
    From<(
        Vec<Enumeral<'a>>,
        Option<ExtensionMarker>,
        Option<Vec<Enumeral<'a>>>,
    )> for Enumerated<'a>
{
    fn from(
        mut value: (
            Vec<Enumeral<'a>>,
            Option<ExtensionMarker>,
            Option<Vec<Enumeral<'a>>>,
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
///     name: Cow::from("first-item"),
///     description: Some(Cow::from(" This is the first item of Test-Enum")),
///     index: 7
/// }
/// # ;
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Enumeral<'a> {
    pub name: Cow<'a, str>,
    pub description: Option<Cow<'a, str>>,
    pub index: i128,
}

/// Representation of a ASN1 distinguished value,
/// as seen in some INTEGER and BIT STRING declarations
/// *As defined in Rec. ITU-T X.680 (02/2021) §19.5 and §22.4*
#[derive(Debug, Clone, PartialEq)]
pub struct DistinguishedValue<'a> {
    pub name: Cow<'a, str>,
    pub value: i128,
}

impl<'a> From<(&'a str, i128)> for DistinguishedValue<'a> {
    fn from(value: (&'a str, i128)) -> Self {
        Self {
            name: Cow::Borrowed(value.0),
            value: value.1,
        }
    }
}

/// Representation of a ASN1 selection type as used with ASN1 CHOICEs
/// *As defined in Rec. ITU-T X.680 (02/2021) §30*
#[derive(Debug, Clone, PartialEq)]
pub struct ChoiceSelectionType<'a> {
    pub choice_name: Cow<'a, str>,
    pub selected_option: Cow<'a, str>,
}

impl<'a> From<(&'a str, &'a str)> for ChoiceSelectionType<'a> {
    fn from(value: (&'a str, &'a str)) -> Self {
        Self {
            choice_name: Cow::Borrowed(value.1),
            selected_option: Cow::Borrowed(value.0),
        }
    }
}
