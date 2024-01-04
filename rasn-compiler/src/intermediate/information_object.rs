use std::ptr::eq;

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

impl ToplevelInformationDeclaration {
    pub fn pdu(&self) -> &ASN1Information {
        &self.value
    }

    pub fn resolve_class_reference(mut self, tlds: &BTreeMap<String, ToplevelDeclaration>) -> Self {
        if let Some(ClassLink::ByName(name)) = &self.class {
            if let Some(ToplevelDeclaration::Information(ToplevelInformationDeclaration {
                value: ASN1Information::ObjectClass(c),
                ..
            })) = tlds.get(name)
            {
                self.class = Some(ClassLink::ByReference(c.clone()));
            }
        }
        self
    }
}

impl From<(Vec<&str>, &str, &str, InformationObjectFields)> for ToplevelInformationDeclaration {
    fn from(value: (Vec<&str>, &str, &str, InformationObjectFields)) -> Self {
        Self {
            comments: value.0.join("\n"),
            name: value.1.into(),
            class: Some(ClassLink::ByName(value.2.into())),
            value: ASN1Information::Object(InformationObject {
                class_name: value.2.into(),
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

impl ASN1Information {
    pub fn link_object_set_reference(
        &mut self,
        tlds: &BTreeMap<String, ToplevelDeclaration>,
    ) -> bool {
        match self {
            ASN1Information::ObjectSet(s) => s.link_object_set_reference(tlds),
            ASN1Information::Object(o) => o.link_object_set_reference(tlds),
            ASN1Information::ObjectClass(_) => false,
        }
    }

    pub fn references_object_set_by_name(&self) -> bool {
        match self {
            ASN1Information::ObjectSet(s) => s.references_object_set_by_name(),
            ASN1Information::Object(o) => o.references_object_set_by_name(),
            ASN1Information::ObjectClass(_) => false,
        }
    }
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
    LiteralOrTypeReference(DeclarationElsewhere),
}

impl SyntaxApplication {
    pub fn link_object_set_reference(
        &mut self,
        tlds: &BTreeMap<String, ToplevelDeclaration>,
    ) -> bool {
        match self {
            SyntaxApplication::ObjectSetDeclaration(o) => o.link_object_set_reference(tlds),
            _ => false,
        }
    }

    pub fn references_object_set_by_name(&self) -> bool {
        match self {
            SyntaxApplication::ObjectSetDeclaration(o) => o.references_object_set_by_name(),
            _ => false,
        }
    }

    /// Checks if a token of a syntactic expression matches a given syntax token,
    /// considering the entire syntax (in form of a flattened SyntaxExpression Vec), in order to reliably match Literals
    pub fn matches(&self, next_token: &SyntaxToken, syntax: &Vec<(bool, SyntaxToken)>) -> bool {
        match (next_token, self) {
            (SyntaxToken::Comma, SyntaxApplication::Comma) => true,
            (SyntaxToken::Literal(t), SyntaxApplication::Literal(a)) if t == a => true,
            (
                SyntaxToken::Literal(t),
                SyntaxApplication::LiteralOrTypeReference(DeclarationElsewhere {
                    identifier, ..
                }),
            ) if t == identifier => true,
            (
                SyntaxToken::Field(ObjectFieldIdentifier::MultipleValue(_)),
                SyntaxApplication::ObjectSetDeclaration(_),
            ) => true,
            (
                SyntaxToken::Field(ObjectFieldIdentifier::MultipleValue(_)),
                SyntaxApplication::TypeReference(_),
            ) => true,
            (
                SyntaxToken::Field(ObjectFieldIdentifier::MultipleValue(_)),
                SyntaxApplication::LiteralOrTypeReference(DeclarationElsewhere {
                    identifier, ..
                }),
            ) => {
                syntax.iter().find(|(_, t)| match t {
                    SyntaxToken::Literal(lit) => lit == identifier,
                    _ => false,
                }).is_none()
            }
            (
                SyntaxToken::Field(ObjectFieldIdentifier::SingleValue(_)),
                SyntaxApplication::ValueReference(_),
            ) => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SyntaxToken {
    Literal(String),
    Comma,
    Field(ObjectFieldIdentifier),
}

impl SyntaxToken {
    pub fn name_or_empty(&self) -> &str {
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

impl InformationObjectSyntax {
    /// Information object syntax consists of mandatory and optional expressions.
    /// Optional expressions may be nested without limit.
    /// Declarations do not have this nested structure, but are always a sequence of
    /// tokens, so in order to check whether an expression follows a given syntax we need to
    /// flatten the nested structure into a sequence of tokens with a `required` marker.
    pub fn flatten(&self) -> Vec<(bool, SyntaxToken)> {
        fn iter_expressions(
            expressions: &Vec<SyntaxExpression>,
            optional_recursion: bool,
        ) -> Vec<(bool, &SyntaxExpression)> {
            expressions
                .iter()
                .flat_map(|x| match x {
                    SyntaxExpression::Optional(o) => iter_expressions(o, true),
                    r => vec![(!optional_recursion, r)],
                })
                .collect()
        }

        iter_expressions(&self.expressions, false)
            .into_iter()
            .map(|x| match x {
                (is_required, SyntaxExpression::Required(r)) => (is_required, r.clone()),
                _ => unreachable!(),
            })
            .collect()
    }
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
    pub fn identifier(&self) -> &String {
        match self {
            ObjectFieldIdentifier::SingleValue(s) => &s,
            ObjectFieldIdentifier::MultipleValue(s) => &s,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct InformationObject {
    pub class_name: String,
    pub fields: InformationObjectFields,
}

impl InformationObject {
    pub fn link_object_set_reference(
        &mut self,
        tlds: &BTreeMap<String, ToplevelDeclaration>,
    ) -> bool {
        match &mut self.fields {
            InformationObjectFields::DefaultSyntax(d) => d.iter_mut().fold(false, |acc, field| {
                acc || field.link_object_set_reference(tlds)
            }),
            InformationObjectFields::CustomSyntax(c) => c.iter_mut().fold(false, |acc, field| {
                acc || field.link_object_set_reference(tlds)
            }),
        }
    }

    pub fn references_object_set_by_name(&self) -> bool {
        match &self.fields {
            InformationObjectFields::DefaultSyntax(d) => d.iter().fold(false, |acc, field| {
                acc || field.references_object_set_by_name()
            }),
            InformationObjectFields::CustomSyntax(c) => c.iter().fold(false, |acc, field| {
                acc || field.references_object_set_by_name()
            }),
        }
    }
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

impl ObjectSetValue {
    pub fn link_object_set_reference(
        &mut self,
        tlds: &BTreeMap<String, ToplevelDeclaration>,
    ) -> bool {
        match self {
            ObjectSetValue::Reference(id) => {
                if let Some(ToplevelDeclaration::Information(ToplevelInformationDeclaration {
                    value: ASN1Information::Object(obj),
                    ..
                })) = tlds.get(id)
                {
                    *self = ObjectSetValue::Inline(obj.fields.clone());
                    true
                } else {
                    false
                }
            }
            ObjectSetValue::Inline(InformationObjectFields::CustomSyntax(c)) => {
                c.iter_mut().fold(false, |acc, field| {
                    acc || field.link_object_set_reference(tlds)
                })
            }
            ObjectSetValue::Inline(InformationObjectFields::DefaultSyntax(d)) => {
                d.iter_mut().fold(false, |acc, field| {
                    acc || field.link_object_set_reference(tlds)
                })
            }
        }
    }

    pub fn references_object_set_by_name(&self) -> bool {
        match self {
            ObjectSetValue::Reference(_) => true,
            ObjectSetValue::Inline(InformationObjectFields::CustomSyntax(c)) => {
                c.iter().fold(false, |acc, field| {
                    acc || field.references_object_set_by_name()
                })
            }
            ObjectSetValue::Inline(InformationObjectFields::DefaultSyntax(d)) => {
                d.iter().fold(false, |acc, field| {
                    acc || field.references_object_set_by_name()
                })
            }
        }
    }
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

impl ObjectSet {
    pub fn link_object_set_reference(
        &mut self,
        tlds: &BTreeMap<String, ToplevelDeclaration>,
    ) -> bool {
        self.values
            .iter_mut()
            .fold(false, |acc, val| acc || val.link_object_set_reference(tlds))
    }

    pub fn references_object_set_by_name(&self) -> bool {
        self.values
            .iter()
            .fold(false, |acc, val| acc || val.references_object_set_by_name())
    }
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

impl InformationObjectField {
    /// Returns the identifier of an InformationObjectField
    pub fn identifier(&self) -> &String {
        match self {
            InformationObjectField::TypeField(f) => &f.identifier,
            InformationObjectField::FixedValueField(f) => &f.identifier,
            InformationObjectField::ObjectSetField(f) => &f.identifier,
        }
    }

    pub fn link_object_set_reference(
        &mut self,
        tlds: &BTreeMap<String, ToplevelDeclaration>,
    ) -> bool {
        match self {
            InformationObjectField::ObjectSetField(ObjectSetField { value, .. }) => {
                value.link_object_set_reference(tlds)
            }
            _ => false,
        }
    }

    pub fn references_object_set_by_name(&self) -> bool {
        match self {
            InformationObjectField::ObjectSetField(ObjectSetField { value, .. }) => {
                value.references_object_set_by_name()
            }
            _ => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FixedValueField {
    pub identifier: String,
    pub value: ASN1Value,
}

impl From<(ObjectFieldIdentifier, ASN1Value)> for InformationObjectField {
    fn from(value: (ObjectFieldIdentifier, ASN1Value)) -> Self {
        Self::FixedValueField(FixedValueField {
            identifier: value.0.identifier().clone(),
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
            identifier: value.0.identifier().clone(),
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
            identifier: value.0.identifier().clone(),
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
