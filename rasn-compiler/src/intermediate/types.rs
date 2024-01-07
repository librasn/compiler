use std::vec;

use super::{constraints::*, *};

/// Trait shared by all ASN1 types that can be constrained a.k.a subtyped
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
constrainable!(ChoiceOption);
constrainable!(SequenceOrSetMember);
constrainable!(Time);

/// Representation of an ASN1 BOOLEAN data element
/// with corresponding constraints
#[derive(Debug, Clone, PartialEq)]
pub struct Boolean {
    pub constraints: Vec<Constraint>,
}

impl Default for Boolean {
    fn default() -> Self {
        Self {
            constraints: vec![],
        }
    }
}

impl From<Option<Vec<Constraint>>> for Boolean {
    fn from(value: Option<Vec<Constraint>>) -> Self {
        Self {
            constraints: value.unwrap_or_default(),
        }
    }
}

/// Representation of an ASN1 INTEGER data element
/// with corresponding constraints and distinguished values
#[derive(Debug, Clone, PartialEq)]
pub struct Integer {
    pub constraints: Vec<Constraint>,
    pub distinguished_values: Option<Vec<DistinguishedValue>>,
}

impl Integer {
    pub fn type_token(&self) -> String {
        let (min, max, extensible) = self.constraints.iter().fold(
            (i128::MAX, i128::MIN, false),
            |(mut min, mut max, mut ext), c| {
                if let Ok((cmin, cmax, extensible)) = c.unpack_as_value_range() {
                    ext = ext || extensible;
                    if let Some(ASN1Value::Integer(i)) = cmin {
                        min = (*i).min(min);
                    };
                    if let Some(ASN1Value::Integer(i)) = cmax {
                        max = (*i).max(max);
                    };
                } else if let Ok((val, extensible)) = c.unpack_as_strict_value() {
                    ext = ext || extensible;
                    if let ASN1Value::Integer(i) = val {
                        min = (*i).min(min);
                        max = (*i).max(max);
                    };
                };
                (min, max, ext)
            },
        );
        if min > max {
            "Integer".to_owned()
        } else {
            int_type_token(min, max, extensible).to_owned()
        }
    }
}

impl Default for Integer {
    fn default() -> Self {
        Self {
            constraints: vec![],
            distinguished_values: None,
        }
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
                    min: value.0.map(|v| ASN1Value::Integer(v)),
                    max: value.1.map(|v| ASN1Value::Integer(v)),
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
            constraints: value.2.unwrap_or(vec![]),
            distinguished_values: value.1,
        }
    }
}

/// Representation of an ASN1 REAL data element
/// with corresponding constraints
#[derive(Debug, Clone, PartialEq)]
pub struct Real {
    pub constraints: Vec<Constraint>,
}

impl From<Option<Vec<Constraint>>> for Real {
    fn from(value: Option<Vec<Constraint>>) -> Self {
        Self {
            constraints: value.unwrap_or(vec![]),
        }
    }
}

/// Representation of an ASN1 GeneralizedTime data element
/// with corresponding constraints
#[derive(Debug, Clone, PartialEq)]
pub struct GeneralizedTime {
    pub constraints: Vec<Constraint>,
}

/// Representation of an ASN1 UTCTime data element
/// with corresponding constraints
#[derive(Debug, Clone, PartialEq)]
pub struct UTCTime {
    pub constraints: Vec<Constraint>,
}

/// Representation of an ASN1 OCTET STRING data element
/// with corresponding constraints
#[derive(Debug, Clone, PartialEq)]
pub struct OctetString {
    pub constraints: Vec<Constraint>,
}

impl From<Option<Vec<Constraint>>> for OctetString {
    fn from(value: Option<Vec<Constraint>>) -> Self {
        OctetString {
            constraints: value.unwrap_or(vec![]),
        }
    }
}

/// Representation of an ASN1 BIT STRING data element
/// with corresponding constraints and distinguished values
/// defining the individual bits
#[derive(Debug, Clone, PartialEq)]
pub struct BitString {
    pub constraints: Vec<Constraint>,
    pub distinguished_values: Option<Vec<DistinguishedValue>>,
}

impl From<(Option<Vec<DistinguishedValue>>, Option<Vec<Constraint>>)> for BitString {
    fn from(value: (Option<Vec<DistinguishedValue>>, Option<Vec<Constraint>>)) -> Self {
        BitString {
            constraints: value.1.unwrap_or(vec![]),
            distinguished_values: value.0,
        }
    }
}

/// Representation of an ASN1 OBJECT IDENTIFIER data element
/// with corresponding constraints
#[derive(Debug, Clone, PartialEq)]
pub struct ObjectIdentifier {
    pub constraints: Vec<Constraint>,
}

impl From<Option<Vec<Constraint>>> for ObjectIdentifier {
    fn from(value: Option<Vec<Constraint>>) -> Self {
        ObjectIdentifier {
            constraints: value.unwrap_or(vec![]),
        }
    }
}

/// Representation of an ASN1 TIME data element
/// with corresponding constraints
#[derive(Debug, Clone, PartialEq)]
pub struct Time {
    pub constraints: Vec<Constraint>,
}

impl From<Option<Vec<Constraint>>> for Time {
    fn from(value: Option<Vec<Constraint>>) -> Self {
        Time {
            constraints: value.unwrap_or(vec![]),
        }
    }
}

/// Representation of an ASN1 Character String type data element
/// with corresponding constraints. ASN1 Character String types
/// include IA5String, UTF8String, VideotexString
#[derive(Debug, Clone, PartialEq)]
pub struct CharacterString {
    pub constraints: Vec<Constraint>,
    pub r#type: CharacterStringType,
}

impl From<(&str, Option<Vec<Constraint>>)> for CharacterString {
    fn from(value: (&str, Option<Vec<Constraint>>)) -> Self {
        CharacterString {
            constraints: value.1.unwrap_or(vec![]),
            r#type: value.0.into(),
        }
    }
}

/// Representation of an ASN1 SEQUENCE OF data element
/// with corresponding constraints and element type info
#[derive(Debug, Clone, PartialEq)]
pub struct SequenceOrSetOf {
    pub constraints: Vec<Constraint>,
    pub r#type: Box<ASN1Type>,
}

impl From<(Option<Vec<Constraint>>, ASN1Type)> for SequenceOrSetOf {
    fn from(value: (Option<Vec<Constraint>>, ASN1Type)) -> Self {
        Self {
            constraints: value.0.unwrap_or(vec![]),
            r#type: Box::new(value.1),
        }
    }
}

/// Representation of an ASN1 SEQUENCE data element
/// with corresponding members and extension information
#[derive(Debug, Clone, PartialEq)]
pub struct SequenceOrSet {
    pub components_of: Vec<String>,
    pub extensible: Option<usize>,
    pub constraints: Vec<Constraint>,
    pub members: Vec<SequenceOrSetMember>,
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
        let index_of_first_extension = value.0 .0.len();
        value.0 .0.append(&mut value.0 .2.unwrap_or(vec![]));
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
            constraints: value.1.unwrap_or(vec![]),
            extensible: value.0 .1.map(|_| index_of_first_extension),
            members,
        }
    }
}

impl
    From<(
        (
            Vec<SequenceOrSetMember>,
            Option<ExtensionMarker>,
            Option<Vec<SequenceOrSetMember>>,
        ),
        Option<Vec<Constraint>>,
    )> for SequenceOrSet
{
    fn from(
        mut value: (
            (
                Vec<SequenceOrSetMember>,
                Option<ExtensionMarker>,
                Option<Vec<SequenceOrSetMember>>,
            ),
            Option<Vec<Constraint>>,
        ),
    ) -> Self {
        let index_of_first_extension = value.0 .0.len();
        value.0 .0.append(&mut value.0 .2.unwrap_or(vec![]));
        SequenceOrSet {
            components_of: vec![],
            constraints: value.1.unwrap_or(vec![]),
            extensible: value.0 .1.map(|_| index_of_first_extension),
            members: value.0 .0,
        }
    }
}

/// Intermediate parsing type to parse COMPONENTS OF notation
#[derive(Debug, Clone, PartialEq)]
pub enum SequenceComponent {
    Member(SequenceOrSetMember),
    ComponentsOf(String),
}

/// Helper enum to express `DEFAULT` values
#[derive(Debug, Clone, PartialEq)]
pub enum DefaultValue {
    None,
    WithTypereference {
        typereference: String,
        value: ASN1Value,
    },
    WithTypereferenceChain {
        typereferences: Vec<String>,
        base_type: ASN1Type,
        value: ASN1Value,
    },
}

impl DefaultValue {
    fn from_type_and_value(ty: &ASN1Type, value: Option<ASN1Value>) -> Self {
        match (ty, value) {
            (ASN1Type::ElsewhereDeclaredType(e), Some(v)) => DefaultValue::WithTypereference {
                typereference: e.identifier.clone(),
                value: v,
            },
            (t, Some(v)) => DefaultValue::WithTypereferenceChain {
                typereferences: vec![],
                base_type: t.clone(),
                value: v,
            },
            (_, None) => DefaultValue::None
        }
    }

    pub fn is_none(&self) -> bool {
        self == &DefaultValue::None
    }
}

/// Representation of an single ASN1 SEQUENCE member
#[derive(Debug, Clone, PartialEq)]
pub struct SequenceOrSetMember {
    pub name: String,
    pub tag: Option<AsnTag>,
    pub r#type: ASN1Type,
    pub default_value: DefaultValue,
    pub is_optional: bool,
    pub constraints: Vec<Constraint>,
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
            default_value: DefaultValue::from_type_and_value(&value.2, value.5),
            r#type: value.2,
            is_optional: value.4.is_some() || value.5.is_some(),
            constraints: value.3.unwrap_or(vec![]),
        }
    }
}

/// Representation of an ASN1 CHOICE data element
/// with corresponding members and extension information
#[derive(Debug, Clone, PartialEq)]
pub struct Choice {
    pub extensible: Option<usize>,
    pub options: Vec<ChoiceOption>,
    pub constraints: Vec<Constraint>,
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
        value.0.append(&mut value.2.unwrap_or(vec![]));
        Choice {
            extensible: value.1.map(|_| index_of_first_extension),
            options: value.0,
            constraints: vec![],
        }
    }
}

/// Representation of an single ASN1 CHOICE option
#[derive(Debug, Clone, PartialEq)]
pub struct ChoiceOption {
    pub name: String,
    pub tag: Option<AsnTag>,
    pub r#type: ASN1Type,
    pub constraints: Vec<Constraint>,
}

impl From<(&str, Option<AsnTag>, ASN1Type, Option<Vec<Constraint>>)> for ChoiceOption {
    fn from(value: (&str, Option<AsnTag>, ASN1Type, Option<Vec<Constraint>>)) -> Self {
        ChoiceOption {
            name: value.0.into(),
            tag: value.1,
            r#type: value.2,
            constraints: value.3.unwrap_or(vec![]),
        }
    }
}

/// Representation of an ASN1 ENUMERATED data element
/// with corresponding enumerals and extension information
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
        value.0.append(&mut value.2.unwrap_or(vec![]));
        Enumerated {
            members: value.0,
            extensible: value.1.map(|_| index_of_first_extension),
            constraints: vec![],
        }
    }
}

/// Representation of a single member/enumeral of an ASN1
/// ENUMERATED data element
#[derive(Debug, Clone, PartialEq)]
pub struct Enumeral {
    pub name: String,
    pub description: Option<String>,
    pub index: i128,
}

/// Representation of a ASN1 distinguished value,
/// as seen in some INTEGER and BIT STRING declarations
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
