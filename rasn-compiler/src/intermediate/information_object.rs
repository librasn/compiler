use crate::lexer::asn1_value;

use super::{constraints::*, *};

/// This is either a X.681 ObjectClassAssignment or a X.683 ParameterizedObjectClassAssignment.
///
/// **X.681 9.1** _The construct "ObjectClassAssignment" is used to assign an information object
/// class to a reference name ("objectclassreference"). This construct is one of the alternatives
/// for "Assignment" in Rec. ITU-T X.680 | ISO/IEC 8824-1, clause 13._
///
/// **X.683 9.2** _Referencing parameterized definitions: ParameterizedObjectClassAssignment._
#[derive(Debug, Clone, PartialEq)]
pub struct ObjectClassAssignment<'a> {
    pub comments: Cow<'a, str>,
    /// A objectclassreference.
    pub name: Cow<'a, str>,
    pub parameterization: Parameterization<'a>,
    pub definition: ObjectClassDefn<'a>,
    pub module_header: Option<Rc<RefCell<ModuleHeader<'a>>>>,
}

impl<'a> ObjectClassAssignment<'a> {
    pub(crate) fn is_parameterized(&self) -> bool {
        !self.parameterization.parameters.is_empty()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ToplevelInformationDefinition<'a> {
    pub comments: Cow<'a, str>,
    pub name: Cow<'a, str>,
    pub parameterization: Option<Parameterization<'a>>,
    pub class: ClassLink<'a>,
    pub value: ASN1Information<'a>,
    pub module_header: Option<Rc<RefCell<ModuleHeader<'a>>>>,
}

impl<'a> From<(&'a str, ASN1Information<'a>, &str)> for ToplevelInformationDefinition<'a> {
    fn from(value: (&'a str, ASN1Information<'a>, &str)) -> Self {
        Self {
            comments: Cow::Borrowed(""),
            name: Cow::Borrowed(value.0),
            parameterization: None,
            class: ClassLink::ByName(value.2.to_owned()),
            value: value.1,
            module_header: None,
        }
    }
}

#[cfg_attr(test, derive(EnumDebug))]
#[cfg_attr(not(test), derive(Debug))]
#[derive(Clone, PartialEq)]
pub enum ClassLink<'a> {
    ByName(String),
    ByReference(ObjectClassDefn<'a>),
}

impl<'a> ToplevelInformationDefinition<'a> {
    pub fn pdu(&self) -> &ASN1Information {
        &self.value
    }
}

impl<'a>
    From<(
        Vec<&str>,
        &'a str,
        Option<Parameterization<'a>>,
        &'a str,
        InformationObjectFields<'a>,
    )> for ToplevelInformationDefinition<'a>
{
    fn from(
        value: (
            Vec<&str>,
            &'a str,
            Option<Parameterization<'a>>,
            &'a str,
            InformationObjectFields<'a>,
        ),
    ) -> Self {
        Self {
            comments: Cow::Owned(value.0.join("\n")),
            name: value.1.into(),
            class: ClassLink::ByName(value.3.into()),
            parameterization: value.2,
            value: ASN1Information::Object(InformationObject {
                class_name: value.3.into(),
                fields: value.4,
            }),
            module_header: None,
        }
    }
}

impl<'a> From<(Vec<&str>, &'a str, Option<Parameterization<'a>>, &str, ObjectSet<'a>)>
    for ToplevelInformationDefinition<'a>
{
    fn from(value: (Vec<&str>, &'a str, Option<Parameterization<'a>>, &str, ObjectSet<'a>)) -> Self {
        Self {
            comments: Cow::Owned(value.0.join("\n")),
            name: value.1.into(),
            parameterization: value.2,
            class: ClassLink::ByName(value.3.into()),
            value: ASN1Information::ObjectSet(value.4),
            module_header: None,
        }
    }
}

/// The possible types of an ASN1 information object.
#[cfg_attr(test, derive(EnumDebug))]
#[cfg_attr(not(test), derive(Debug))]
#[derive(Clone, PartialEq)]
pub enum ASN1Information<'a> {
    ObjectSet(ObjectSet<'a>),
    Object(InformationObject<'a>),
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
pub enum SyntaxApplication<'a> {
    ObjectSetDeclaration(ObjectSet<'a>),
    ValueReference(ASN1Value<'a>),
    TypeReference(ASN1Type<'a>),
    Comma,
    Literal(String),
    LiteralOrTypeReference(DeclarationElsewhere<'a>),
}

impl<'a> SyntaxApplication<'a> {
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
            ) => {
                let val = asn1_value((&**lit).into());
                match val {
                    Ok((_, ASN1Value::ElsewhereDeclaredValue { .. })) => false,
                    Ok((_, _)) => true,
                    _ => false,
                }
            },
            (
                SyntaxToken::Field(ObjectFieldIdentifier::SingleValue(_)),
                SyntaxApplication::Literal(lit),
            ) => {
                let val = asn1_value(lit.as_str().into());
                match val {
                    Ok((_, ASN1Value::ElsewhereDeclaredValue { .. })) => false,
                    Ok((_, _)) => true,
                    _ => false,
                }
            }
            _ => false,
        }
    }

    pub(crate) fn as_str_or_none(&'a self) -> Option<&'a str> {
        match self {
            SyntaxApplication::ObjectSetDeclaration(_) => None,
            SyntaxApplication::ValueReference(ASN1Value::ElsewhereDeclaredValue {
                parent: None,
                identifier,
            }) => Some(&*&identifier),
            SyntaxApplication::LiteralOrTypeReference(DeclarationElsewhere {
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

/// X.681 9.3  Every class is ultimately defined by an "ObjectClassDefn".
///
/// Allows the definer to provide the field specifications, and optionally a syntax list. The
/// definer may also specify semantics associated with the definition of the class.
#[derive(Debug, Clone, PartialEq)]
pub struct ObjectClassDefn<'a> {
    /// Named field specifications, as defined in 9.4.
    pub fields: Vec<InformationObjectClassField<'a>>,
    /// An information object definition syntax ("SyntaxList"), as defined in 10.5.
    pub syntax: Option<InformationObjectSyntax>,
}

impl<'a>
    From<(
        Vec<InformationObjectClassField<'a>>,
        Option<Vec<SyntaxExpression>>,
    )> for ObjectClassDefn<'a>
{
    fn from(
        value: (
            Vec<InformationObjectClassField<'a>>,
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
pub struct InformationObjectClassField<'a> {
    pub identifier: ObjectFieldIdentifier,
    pub ty: Option<ASN1Type<'a>>,
    pub optionality: Optionality<ASN1Value<'a>>,
    pub is_unique: bool,
}

impl<'a>
    From<(
        ObjectFieldIdentifier,
        Option<ASN1Type<'a>>,
        Option<&str>,
        Optionality<ASN1Value<'a>>,
    )> for InformationObjectClassField<'a>
{
    fn from(
        value: (
            ObjectFieldIdentifier,
            Option<ASN1Type<'a>>,
            Option<&str>,
            Optionality<ASN1Value<'a>>,
        ),
    ) -> Self {
        Self {
            identifier: value.0,
            ty: value.1,
            is_unique: value.2.is_some(),
            optionality: value.3,
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
pub struct InformationObject<'a> {
    pub class_name: String,
    pub fields: InformationObjectFields<'a>,
}

#[cfg_attr(test, derive(EnumDebug))]
#[cfg_attr(not(test), derive(Debug))]
#[derive(Clone, PartialEq)]
pub enum InformationObjectFields<'a> {
    DefaultSyntax(Vec<InformationObjectField<'a>>),
    CustomSyntax(Vec<SyntaxApplication<'a>>),
}

#[cfg_attr(test, derive(EnumDebug))]
#[cfg_attr(not(test), derive(Debug))]
#[derive(Clone, PartialEq)]
pub enum ObjectSetValue<'a> {
    Reference(Cow<'a, str>),
    Inline(InformationObjectFields<'a>),
}

impl<'a> From<&'a str> for ObjectSetValue<'a> {
    fn from(value: &'a str) -> Self {
        Self::Reference(Cow::Borrowed(value))
    }
}

impl<'a> From<InformationObjectFields<'a>> for ObjectSetValue<'a> {
    fn from(value: InformationObjectFields<'a>) -> Self {
        Self::Inline(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ObjectSet<'a> {
    pub values: Vec<ObjectSetValue<'a>>,
    pub extensible: Option<usize>,
}

impl<'a>
    From<(
        Vec<ObjectSetValue<'a>>,
        Option<ExtensionMarker>,
        Option<Vec<ObjectSetValue<'a>>>,
    )> for ObjectSet<'a>
{
    fn from(
        mut value: (
            Vec<ObjectSetValue<'a>>,
            Option<ExtensionMarker>,
            Option<Vec<ObjectSetValue<'a>>>,
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
pub enum InformationObjectField<'a> {
    TypeField(TypeField<'a>),
    FixedValueField(FixedValueField<'a>),
    ObjectSetField(ObjectSetField<'a>),
}

impl<'a> InformationObjectField<'a> {
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
pub struct FixedValueField<'a> {
    pub identifier: String,
    pub value: ASN1Value<'a>,
}

impl<'a> From<(ObjectFieldIdentifier, ASN1Value<'a>)> for InformationObjectField<'a> {
    fn from(value: (ObjectFieldIdentifier, ASN1Value<'a>)) -> Self {
        Self::FixedValueField(FixedValueField {
            identifier: value.0.identifier().clone(),
            value: value.1,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeField<'a> {
    pub identifier: String,
    pub ty: ASN1Type<'a>,
}

impl<'a> From<(ObjectFieldIdentifier, ASN1Type<'a>)> for InformationObjectField<'a> {
    fn from(value: (ObjectFieldIdentifier, ASN1Type<'a>)) -> Self {
        Self::TypeField(TypeField {
            identifier: value.0.identifier().clone(),
            ty: value.1,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ObjectSetField<'a> {
    pub identifier: String,
    pub value: ObjectSet<'a>,
}

impl<'a> From<(ObjectFieldIdentifier, ObjectSet<'a>)> for InformationObjectField<'a> {
    fn from(value: (ObjectFieldIdentifier, ObjectSet<'a>)) -> Self {
        Self::ObjectSetField(ObjectSetField {
            identifier: value.0.identifier().clone(),
            value: value.1,
        })
    }
}

/// #### X.681 14 Notation for the object class field type
/// _The type that is referenced by this notation depends on the category of the field name. For
/// the different categories of field names, 14.2 to 14.5 specify the type that is referenced._
#[derive(Debug, Clone, PartialEq)]
pub struct ObjectClassFieldType<'a> {
    pub class: String,
    pub field_path: Vec<ObjectFieldIdentifier>,
    pub constraints: Vec<Constraint<'a>>,
}

impl<'a> ObjectClassFieldType<'a> {
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

impl<'a> From<(&str, Vec<ObjectFieldIdentifier>, Option<Vec<Constraint<'a>>>)> for ObjectClassFieldType<'a> {
    fn from(value: (&str, Vec<ObjectFieldIdentifier>, Option<Vec<Constraint<'a>>>)) -> Self {
        Self {
            class: value.0.into(),
            field_path: value.1,
            constraints: value.2.unwrap_or_default(),
        }
    }
}
