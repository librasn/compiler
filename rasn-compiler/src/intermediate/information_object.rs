use crate::lexer::asn1_value;

use super::{constraints::*, *};

#[derive(Debug, Clone, PartialEq)]
pub struct ToplevelInformationDefinition {
    pub comments: String,
    pub name: String,
    pub parameterization: Option<Parameterization>,
    pub class: Option<ClassLink>,
    pub value: ASN1Information,
    pub index: Option<(Rc<RefCell<ModuleReference>>, usize)>,
}

impl From<(Span<'_>, ASN1Information, &str)> for ToplevelInformationDefinition {
    fn from(value: (Span, ASN1Information, &str)) -> Self {
        Self {
            comments: String::new(),
            name: value.0.to_string(),
            parameterization: None,
            class: Some(ClassLink::ByName(value.2.to_owned())),
            value: value.1,
            index: None,
        }
    }
}

#[cfg_attr(test, derive(EnumDebug))]
#[cfg_attr(not(test), derive(Debug))]
#[derive(Clone, PartialEq)]
pub enum ClassLink {
    ByName(String),
    ByReference(InformationObjectClass),
}

impl ToplevelInformationDefinition {
    pub fn pdu(&self) -> &ASN1Information {
        &self.value
    }
}

impl
    From<(
        Vec<Span<'_>>,
        Span<'_>,
        Option<Parameterization>,
        Span<'_>,
        InformationObjectFields,
    )> for ToplevelInformationDefinition
{
    fn from(
        value: (
            Vec<Span>,
            Span,
            Option<Parameterization>,
            Span,
            InformationObjectFields,
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
            class: Some(ClassLink::ByName(value.3.to_string())),
            parameterization: value.2,
            value: ASN1Information::Object(InformationObject {
                class_name: value.3.to_string(),
                fields: value.4,
            }),
            index: None,
        }
    }
}

impl
    From<(
        Vec<Span<'_>>,
        Span<'_>,
        Option<Parameterization>,
        Span<'_>,
        ObjectSet,
    )> for ToplevelInformationDefinition
{
    fn from(value: (Vec<Span>, Span, Option<Parameterization>, Span, ObjectSet)) -> Self {
        Self {
            comments: value
                .0
                .into_iter()
                .map(Span::into_fragment)
                .collect::<Vec<&str>>()
                .join("\n"),
            name: value.1.to_string(),
            parameterization: value.2,
            class: Some(ClassLink::ByName(value.3.to_string())),
            value: ASN1Information::ObjectSet(value.4),
            index: None,
        }
    }
}

impl
    From<(
        Vec<Span<'_>>,
        Span<'_>,
        Option<Parameterization>,
        InformationObjectClass,
    )> for ToplevelInformationDefinition
{
    fn from(
        value: (
            Vec<Span>,
            Span,
            Option<Parameterization>,
            InformationObjectClass,
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
            class: None,
            value: ASN1Information::ObjectClass(value.3),
            index: None,
        }
    }
}

/// The possible types of an ASN1 information object.
#[cfg_attr(test, derive(EnumDebug))]
#[cfg_attr(not(test), derive(Debug))]
#[derive(Clone, PartialEq)]
pub enum ASN1Information {
    ObjectClass(InformationObjectClass),
    ObjectSet(ObjectSet),
    Object(InformationObject),
}

#[cfg_attr(test, derive(EnumDebug))]
#[cfg_attr(not(test), derive(Debug))]
#[derive(Clone, PartialEq)]
pub enum SyntaxExpression {
    Required(SyntaxToken),
    Optional(Vec<SyntaxExpression>),
}

#[cfg_attr(test, derive(EnumDebug))]
#[cfg_attr(not(test), derive(Debug))]
#[derive(Clone, PartialEq)]
pub enum SyntaxApplication {
    ObjectSetDeclaration(ObjectSet),
    ValueReference(ASN1Value),
    TypeReference(ASN1Type),
    Comma,
    Literal(String),
    LiteralOrTypeReference(DeclarationElsewhere),
}

impl SyntaxApplication {
    /// Checks if a token of a syntactic expression matches a given syntax token,
    /// considering the entire syntax (in form of a flattened SyntaxExpression Vec), in order to reliably match Literals
    pub fn matches(
        &self,
        next_token: &SyntaxToken,
        syntax: &[(bool, SyntaxToken)],
        current_index: usize,
    ) -> bool {
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
                for (required, token) in &syntax[current_index + 1..] {
                    if token.as_str() == identifier {
                        return false;
                    } else if *required {
                        return true;
                    }
                }
                true
            }
            (
                SyntaxToken::Field(ObjectFieldIdentifier::SingleValue(_)),
                SyntaxApplication::ValueReference(_),
            ) => true,
            (
                SyntaxToken::Field(ObjectFieldIdentifier::SingleValue(_)),
                SyntaxApplication::LiteralOrTypeReference(DeclarationElsewhere {
                    identifier: lit,
                    ..
                }),
            )
            | (
                SyntaxToken::Field(ObjectFieldIdentifier::SingleValue(_)),
                SyntaxApplication::Literal(lit),
            ) => {
                let val = asn1_value(Span::new(lit));
                match val {
                    Ok((_, ASN1Value::ElsewhereDeclaredValue { .. })) => false,
                    Ok((_, _)) => true,
                    _ => false,
                }
            }
            _ => false,
        }
    }

    pub(crate) fn as_str_or_none(&self) -> Option<&str> {
        match self {
            SyntaxApplication::ObjectSetDeclaration(_) => None,
            SyntaxApplication::ValueReference(ASN1Value::ElsewhereDeclaredValue {
                parent: None,
                identifier,
            })
            | SyntaxApplication::LiteralOrTypeReference(DeclarationElsewhere {
                parent: None,
                identifier,
                ..
            })
            | SyntaxApplication::TypeReference(ASN1Type::ElsewhereDeclaredType(
                DeclarationElsewhere {
                    parent: None,
                    identifier,
                    ..
                },
            )) => Some(identifier),
            SyntaxApplication::Literal(l) => Some(l),
            _ => None,
        }
    }
}

#[cfg_attr(test, derive(EnumDebug))]
#[cfg_attr(not(test), derive(Debug))]
#[derive(Clone, PartialEq)]
pub enum SyntaxToken {
    Literal(String),
    Comma,
    Field(ObjectFieldIdentifier),
}

impl SyntaxToken {
    pub fn as_str(&self) -> &str {
        match self {
            SyntaxToken::Literal(s) => s.as_str(),
            SyntaxToken::Comma => ",",
            SyntaxToken::Field(_) => self.name_or_empty(),
        }
    }

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

impl From<Span<'_>> for SyntaxToken {
    fn from(value: Span) -> Self {
        if *value == "," {
            Self::Comma
        } else {
            Self::Literal(value.to_string())
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
            expressions: &[SyntaxExpression],
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
    pub ty: Option<ASN1Type>,
    pub is_optional: bool,
    pub default: Option<ASN1Value>,
    pub is_unique: bool,
}

impl
    From<(
        ObjectFieldIdentifier,
        Option<ASN1Type>,
        Option<Span<'_>>,
        Option<OptionalMarker>,
        Option<ASN1Value>,
    )> for InformationObjectClassField
{
    fn from(
        value: (
            ObjectFieldIdentifier,
            Option<ASN1Type>,
            Option<Span>,
            Option<OptionalMarker>,
            Option<ASN1Value>,
        ),
    ) -> Self {
        Self {
            identifier: value.0,
            ty: value.1,
            is_unique: value.2.is_some(),
            is_optional: value.3.is_some() || value.4.is_some(),
            default: value.4,
        }
    }
}

#[cfg_attr(test, derive(EnumDebug))]
#[cfg_attr(not(test), derive(Debug))]
#[derive(Clone, PartialEq)]
pub enum ObjectFieldIdentifier {
    SingleValue(String),
    MultipleValue(String),
}

impl ObjectFieldIdentifier {
    pub fn identifier(&self) -> &String {
        match self {
            ObjectFieldIdentifier::SingleValue(s) => s,
            ObjectFieldIdentifier::MultipleValue(s) => s,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct InformationObject {
    pub class_name: String,
    pub fields: InformationObjectFields,
}

#[cfg_attr(test, derive(EnumDebug))]
#[cfg_attr(not(test), derive(Debug))]
#[derive(Clone, PartialEq)]
pub enum InformationObjectFields {
    DefaultSyntax(Vec<InformationObjectField>),
    CustomSyntax(Vec<SyntaxApplication>),
}

#[cfg_attr(test, derive(EnumDebug))]
#[cfg_attr(not(test), derive(Debug))]
#[derive(Clone, PartialEq)]
pub enum ObjectSetValue {
    Reference(String),
    Inline(InformationObjectFields),
}

impl From<Span<'_>> for ObjectSetValue {
    fn from(value: Span) -> Self {
        Self::Reference(value.to_string())
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
        value.0.append(&mut value.2.unwrap_or_default());
        ObjectSet {
            values: value.0,
            extensible: value.1.map(|_| index_of_first_extension),
        }
    }
}

#[cfg_attr(test, derive(EnumDebug))]
#[cfg_attr(not(test), derive(Debug))]
#[derive(Clone, PartialEq)]
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
    pub ty: ASN1Type,
}

impl From<(ObjectFieldIdentifier, ASN1Type)> for InformationObjectField {
    fn from(value: (ObjectFieldIdentifier, ASN1Type)) -> Self {
        Self::TypeField(TypeField {
            identifier: value.0.identifier().clone(),
            ty: value.1,
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

impl InformationObjectFieldReference {
    /// Returns the field path as string.
    /// The field path is stringified by joining
    /// the stringified `ObjectFieldIdentifier`s with
    /// the `$` character as a separator.
    pub fn field_path_as_str(&self) -> String {
        self.field_path
            .iter()
            .map(|o| o.identifier().clone())
            .collect::<Vec<_>>()
            .join("$")
    }
}

impl
    From<(
        Span<'_>,
        Vec<ObjectFieldIdentifier>,
        Option<Vec<Constraint>>,
    )> for InformationObjectFieldReference
{
    fn from(value: (Span, Vec<ObjectFieldIdentifier>, Option<Vec<Constraint>>)) -> Self {
        Self {
            class: value.0.to_string(),
            field_path: value.1,
            constraints: value.2.unwrap_or_default(),
        }
    }
}
