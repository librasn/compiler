use super::{constraints::*, utils::walk_object_field_ref_path, *};

#[derive(Debug, Clone, PartialEq)]
pub struct ToplevelInformationDeclaration {
    pub comments: String,
    pub name: String,
    pub class: Option<ClassLink>,
    pub value: ASN1Information,
    pub index: Option<(Rc<ModuleReference>, usize)>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ClassLink {
    ByName(String),
    ByReference(InformationObjectClass),
}

impl From<(Vec<&str>, &str, &str, InformationObjectFields)> for ToplevelInformationDeclaration {
    fn from(value: (Vec<&str>, &str, &str, InformationObjectFields)) -> Self {
        Self {
            comments: value.0.join("\n"),
            name: value.1.into(),
            class: Some(ClassLink::ByName(value.2.into())),
            value: ASN1Information::Object(InformationObject {
                supertype: value.2.into(),
                fields: value.3,
            }),
            index: None,
        }
    }
}

impl From<(Vec<&str>, &str, &str, ObjectSet)> for ToplevelInformationDeclaration {
    fn from(value: (Vec<&str>, &str, &str, ObjectSet)) -> Self {
        Self {
            comments: value.0.join("\n"),
            name: value.1.into(),
            class: Some(ClassLink::ByName(value.2.into())),
            value: ASN1Information::ObjectSet(value.3),
            index: None,
        }
    }
}

impl From<(Vec<&str>, &str, InformationObjectClass)> for ToplevelInformationDeclaration {
    fn from(value: (Vec<&str>, &str, InformationObjectClass)) -> Self {
        Self {
            comments: value.0.join("\n"),
            name: value.1.into(),
            class: None,
            value: ASN1Information::ObjectClass(value.2),
            index: None,
        }
    }
}

/// The possible types of an ASN1 information object.
#[derive(Debug, Clone, PartialEq)]
pub enum ASN1Information {
    ObjectClass(InformationObjectClass),
    ObjectSet(ObjectSet),
    Object(InformationObject),
}

#[derive(Debug, Clone, PartialEq)]
pub enum SyntaxExpression {
    Required(SyntaxToken),
    Optional(Vec<SyntaxExpression>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum SyntaxApplication {
    ObjectSetDeclaration(ObjectSet),
    ValueReference(ASN1Value),
    TypeReference(ASN1Type),
    Comma,
    Literal(String),
    LiteralOrTypeReference(DeclarationElsewhere)
}

#[derive(Debug, Clone, PartialEq)]
pub enum SyntaxToken {
    Literal(String),
    Comma,
    Field(ObjectFieldIdentifier),
}

impl SyntaxToken {
    pub fn _name_or_empty(&self) -> &str {
        match self {
            SyntaxToken::Field(ObjectFieldIdentifier::SingleValue(v))
            | SyntaxToken::Field(ObjectFieldIdentifier::MultipleValue(v)) => v.as_str(),
            _ => "",
        }
    }
}

impl From<ObjectFieldIdentifier> for SyntaxToken {
    fn from(value: ObjectFieldIdentifier) -> Self {
        Self::Field(value)
    }
}

impl From<&str> for SyntaxToken {
    fn from(value: &str) -> Self {
        if value == "," {
            Self::Comma
        } else {
            Self::Literal(value.into())
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct InformationObjectSyntax {
    pub expressions: Vec<SyntaxExpression>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct InformationObjectClass {
    pub fields: Vec<InformationObjectClassField>,
    pub syntax: Option<InformationObjectSyntax>,
}

impl InformationObjectClass {
    pub fn get_field<'a>(
        &'a self,
        path: &'a Vec<ObjectFieldIdentifier>,
    ) -> Option<&InformationObjectClassField> {
        walk_object_field_ref_path(&self.fields, path, 0)
    }
}

impl
    From<(
        Vec<InformationObjectClassField>,
        Option<Vec<SyntaxExpression>>,
    )> for InformationObjectClass
{
    fn from(
        value: (
            Vec<InformationObjectClassField>,
            Option<Vec<SyntaxExpression>>,
        ),
    ) -> Self {
        Self {
            fields: value.0,
            syntax: value
                .1
                .map(|expr| InformationObjectSyntax { expressions: expr }),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct InformationObjectClassField {
    pub identifier: ObjectFieldIdentifier,
    pub r#type: Option<ASN1Type>,
    pub is_optional: bool,
    pub default: Option<ASN1Value>,
    pub is_unique: bool,
}

impl
    From<(
        ObjectFieldIdentifier,
        Option<ASN1Type>,
        Option<&str>,
        Option<OptionalMarker>,
        Option<ASN1Value>,
    )> for InformationObjectClassField
{
    fn from(
        value: (
            ObjectFieldIdentifier,
            Option<ASN1Type>,
            Option<&str>,
            Option<OptionalMarker>,
            Option<ASN1Value>,
        ),
    ) -> Self {
        Self {
            identifier: value.0,
            r#type: value.1,
            is_unique: value.2.is_some(),
            is_optional: value.3.is_some() || value.4.is_some(),
            default: value.4,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ObjectFieldIdentifier {
    SingleValue(String),
    MultipleValue(String),
}

impl ObjectFieldIdentifier {
    pub fn identifier(&self) -> String {
        match self {
            ObjectFieldIdentifier::SingleValue(s) => s.clone(),
            ObjectFieldIdentifier::MultipleValue(s) => s.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct InformationObject {
    pub supertype: String,
    pub fields: InformationObjectFields,
}

#[derive(Debug, Clone, PartialEq)]
pub enum InformationObjectFields {
    DefaultSyntax(Vec<InformationObjectField>),
    CustomSyntax(Vec<SyntaxApplication>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ObjectSetValue {
    Reference(String),
    Inline(InformationObjectFields),
}

impl From<&str> for ObjectSetValue {
    fn from(value: &str) -> Self {
        Self::Reference(value.into())
    }
}

impl From<InformationObjectFields> for ObjectSetValue {
    fn from(value: InformationObjectFields) -> Self {
        Self::Inline(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ObjectSet {
    pub values: Vec<ObjectSetValue>,
    pub extensible: Option<usize>,
}

impl
    From<(
        Vec<ObjectSetValue>,
        Option<ExtensionMarker>,
        Option<Vec<ObjectSetValue>>,
    )> for ObjectSet
{
    fn from(
        mut value: (
            Vec<ObjectSetValue>,
            Option<ExtensionMarker>,
            Option<Vec<ObjectSetValue>>,
        ),
    ) -> Self {
        let index_of_first_extension = value.0.len();
        value.0.append(&mut value.2.unwrap_or(vec![]));
        ObjectSet {
            values: value.0,
            extensible: value.1.map(|_| index_of_first_extension),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum InformationObjectField {
    TypeField(TypeField),
    FixedValueField(FixedValueField),
    ObjectSetField(ObjectSetField),
}

#[derive(Debug, Clone, PartialEq)]
pub struct FixedValueField {
    pub identifier: String,
    pub value: ASN1Value,
}

impl From<(ObjectFieldIdentifier, ASN1Value)> for InformationObjectField {
    fn from(value: (ObjectFieldIdentifier, ASN1Value)) -> Self {
        Self::FixedValueField(FixedValueField {
            identifier: value.0.identifier(),
            value: value.1,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeField {
    pub identifier: String,
    pub r#type: ASN1Type,
}

impl From<(ObjectFieldIdentifier, ASN1Type)> for InformationObjectField {
    fn from(value: (ObjectFieldIdentifier, ASN1Type)) -> Self {
        Self::TypeField(TypeField {
            identifier: value.0.identifier(),
            r#type: value.1,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ObjectSetField {
    pub identifier: String,
    pub value: ObjectSet,
}

impl From<(ObjectFieldIdentifier, ObjectSet)> for InformationObjectField {
    fn from(value: (ObjectFieldIdentifier, ObjectSet)) -> Self {
        Self::ObjectSetField(ObjectSetField {
            identifier: value.0.identifier(),
            value: value.1,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct InformationObjectFieldReference {
    pub class: String,
    pub field_path: Vec<ObjectFieldIdentifier>,
    pub constraints: Vec<Constraint>,
}

impl From<(&str, Vec<ObjectFieldIdentifier>, Option<Vec<Constraint>>)>
    for InformationObjectFieldReference
{
    fn from(value: (&str, Vec<ObjectFieldIdentifier>, Option<Vec<Constraint>>)) -> Self {
        Self {
            class: value.0.into(),
            field_path: value.1,
            constraints: value.2.unwrap_or_default(),
        }
    }
}
