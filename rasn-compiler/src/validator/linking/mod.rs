//! The `linking` module contains methods to link different tokens of the parsed AST
//! in order to generate correct rust representations.

mod constraints;
mod information_object;
pub(self) mod utils;

use std::collections::BTreeMap;

use crate::intermediate::{*, types::*, utils::*, error::*, information_object::*};

use self::utils::find_tld_or_enum_value_by_name;

macro_rules! error {
    ($kind:ident, $($arg:tt)*) => {
        GrammarError {
            details: format!($($arg)*),
            kind: GrammarErrorType::$kind,
        }
    };
}

impl ToplevelDeclaration {
    pub(crate) fn get_distinguished_or_enum_value(
        &self,
        type_name: Option<&String>,
        identifier: &String,
    ) -> Option<ASN1Value> {
        if let ToplevelDeclaration::Type(t) = self {
            if type_name.is_some() && Some(&t.name) != type_name {
                return None;
            }
            match &t.r#type {
                ASN1Type::Enumerated(e) => {
                    return e.members.iter().find_map(|m| {
                        (&m.name == identifier).then(|| ASN1Value::Integer(m.index as i128))
                    })
                }
                ASN1Type::Integer(i) => {
                    return i
                        .distinguished_values
                        .as_ref()
                        .map(|dv| {
                            dv.iter().find_map(|d| {
                                (&d.name == identifier).then(|| ASN1Value::Integer(d.value))
                            })
                        })
                        .flatten()
                }
                _ => (),
            }
        }
        None
    }

    pub fn is_class_with_name(&self, name: &String) -> Option<&InformationObjectClass> {
        match self {
            ToplevelDeclaration::Information(info) => match &info.value {
                ASN1Information::ObjectClass(class) => {
                    (&info.name == name).then(|| class)
                }
                _ => None,
            },
            _ => None,
        }
    }

    /// Traverses a top-level declaration to check for references to other top-level declarations
    /// in a SEQUENCE's or SET's DEFAULT values.
    pub fn has_default_reference(&self) -> bool {
        match self {
            ToplevelDeclaration::Type(t) => match &t.r#type {
                ASN1Type::Sequence(s) | ASN1Type::Set(s) => {
                    s.members.iter().fold(false, |acc, m| {
                        acc || m
                            .default_value
                            .as_ref()
                            .map_or(false, |d| d.is_elsewhere_declared())
                    })
                }
                _ => false,
            },
            _ => false,
        }
    }

    /// Traverses a top-level declaration to replace references to other top-level declarations
    /// in a SEQUENCE's or SET's DEFAULT values.
    pub fn link_default_reference(&mut self, tlds: &BTreeMap<String, ToplevelDeclaration>) -> bool {
        match self {
            ToplevelDeclaration::Type(t) => match &mut t.r#type {
                ASN1Type::Sequence(s) | ASN1Type::Set(s) => {
                    s.members.iter_mut().fold(false, |acc, m| {
                        if let Some(default) = m.default_value.as_mut() {
                            let maybe_id =
                                if let ASN1Value::ElsewhereDeclaredValue { identifier, .. } =
                                    default
                                {
                                    Some(identifier.clone())
                                } else {
                                    None
                                };
                            if let Some(ToplevelDeclaration::Value(id)) =
                                tlds.get(&maybe_id.clone().unwrap_or_default())
                            {
                                *default = id.value.clone();
                                return true;
                            }
                            let enumerated_id = match &m.r#type {
                                ASN1Type::Enumerated(_) => format!(
                                    "{}{}",
                                    to_rust_title_case(&t.name),
                                    to_rust_title_case(&m.name)
                                ),
                                ASN1Type::ElsewhereDeclaredType(e) => {
                                    if let Some(tld) = e.find_root_id(tlds) {
                                        tld.name().clone()
                                    } else {
                                        return acc;
                                    }
                                }
                                _ => return acc,
                            };
                            maybe_id.map_or(acc, |id| {
                                *default = ASN1Value::EnumeratedValue {
                                    enumerated: enumerated_id,
                                    enumerable: id,
                                };
                                true
                            })
                        } else {
                            acc
                        }
                    })
                }
                _ => false,
            },
            _ => false,
        }
    }

    /// Traverses a top-level declaration to check for references to other top-level declarations
    /// in a constraint. An example would be the constraint of the `intercontinental` field in the
    /// following example.
    /// ```ignore
    /// fifteen INTEGER = 15
    ///
    /// Departures ::= SEQUENCE {
    ///   local SEQUENCE (SIZE(0..999)) OF Local,
    ///   continental SEQUENCE (SIZE(0..99)) OF Continental,
    ///   intercontinental SEQUENCE (SIZE(0..fifteen)) OF Intercontinental
    /// }
    /// ```
    pub fn has_constraint_reference(&self) -> bool {
        match self {
            ToplevelDeclaration::Type(t) => t.r#type.contains_constraint_reference(),
            // TODO: Cover constraint references in other types of top-level declarations
            _ => false,
        }
    }

    /// Traverses a top-level declaration to replace references to other top-level declarations
    /// in a constraint. An example would be the constraint of the `intercontinental` field in the
    /// following example.
    /// ```ignore
    /// fifteen INTEGER = 15
    ///
    /// Departures ::= SEQUENCE {
    ///   local SEQUENCE (SIZE(0..999)) OF Local,
    ///   continental SEQUENCE (SIZE(0..99)) OF Continental,
    ///   intercontinental SEQUENCE (SIZE(0..fifteen)) OF Intercontinental
    /// }
    /// ```
    /// The method handles linking of multiple constraint references within a top-level declaration.
    /// ### Params
    ///  * `tlds` - vector of other top-level declarations that will be searched as the method resolves a reference
    /// returns `true` if the reference was resolved successfully.
    pub fn link_constraint_reference(
        &mut self,
        tlds: &BTreeMap<String, ToplevelDeclaration>,
    ) -> Result<bool, GrammarError> {
        match self {
            ToplevelDeclaration::Type(t) => t.r#type.link_constraint_reference(&t.name, tlds),
            // TODO: Cover constraint references in other types of top-level declarations
            _ => Ok(false),
        }
    }
}

impl ASN1Type {
    pub fn has_choice_selection_type(&self) -> bool {
        match self {
            ASN1Type::ChoiceSelectionType(_) => true,
            ASN1Type::Sequence(s) | ASN1Type::Set(s) => s
                .members
                .iter()
                .map(|m| m.r#type.has_choice_selection_type())
                .fold(false, |acc, b| acc || b),
            ASN1Type::Choice(c) => c
                .options
                .iter()
                .map(|o| o.r#type.has_choice_selection_type())
                .fold(false, |acc, b| acc || b),
            ASN1Type::SequenceOf(s) | ASN1Type::SetOf(s) => s.r#type.has_choice_selection_type(),
            _ => false,
        }
    }

    pub fn link_choice_selection_type(
        &mut self,
        tlds: &BTreeMap<String, ToplevelDeclaration>,
    ) -> Result<(), GrammarError> {
        match self {
            ASN1Type::ChoiceSelectionType(c) => {
                if let Some(ToplevelDeclaration::Type(parent)) = tlds.get(&c.choice_name) {
                    *self = parent.r#type.clone();
                    Ok(())
                } else {
                    Err(error!(
                        LinkerError,
                        "Could not find Choice {} of selection type.", c.choice_name
                    ))
                }
            }
            ASN1Type::Sequence(s) | ASN1Type::Set(s) => s
                .members
                .iter_mut()
                .map(|m| m.r#type.link_choice_selection_type(tlds))
                .collect::<Result<(), _>>(),
            ASN1Type::Choice(c) => c
                .options
                .iter_mut()
                .map(|o: &mut ChoiceOption| o.r#type.link_choice_selection_type(tlds))
                .collect::<Result<(), _>>(),
            ASN1Type::SequenceOf(s) | ASN1Type::SetOf(s) => {
                s.r#type.link_choice_selection_type(tlds)
            }
            _ => Ok(()),
        }
    }

    pub fn contains_components_of_notation(&self) -> bool {
        match self {
            ASN1Type::Choice(c) => c.options.iter().fold(false, |acc, o| {
                acc || o.r#type.contains_components_of_notation()
            }),
            ASN1Type::Set(s) | ASN1Type::Sequence(s) => {
                !s.components_of.is_empty()
                    || s.members.iter().fold(false, |acc, m| {
                        acc || m.r#type.contains_components_of_notation()
                    })
            }
            ASN1Type::SequenceOf(so) => so.r#type.contains_components_of_notation(),
            _ => false,
        }
    }

    pub fn link_components_of_notation(
        &mut self,
        tlds: &BTreeMap<String, ToplevelDeclaration>,
    ) -> bool {
        match self {
            ASN1Type::Choice(c) => c.options.iter_mut().fold(false, |acc, o| {
                acc || o.r#type.link_components_of_notation(tlds)
            }),
            ASN1Type::Set(s) | ASN1Type::Sequence(s) => {
                let mut member_linking = s.members.iter_mut().fold(false, |acc, m| {
                    acc || m.r#type.link_components_of_notation(tlds)
                });
                // TODO: properly link components of in extensions
                // TODO: link components of Class field, such as COMPONENTS OF BILATERAL.&id
                for comp_link in &s.components_of {
                    if let Some(ToplevelDeclaration::Type(linked)) = tlds.get(comp_link) {
                        if let ASN1Type::Sequence(linked_seq) = &linked.r#type {
                            linked_seq
                                .members
                                .iter()
                                .enumerate()
                                .for_each(|(index, member)| {
                                    if index < linked_seq.extensible.unwrap_or(usize::MAX) {
                                        if let Some(index_of_first_ext) = s.extensible {
                                            s.extensible = Some(index_of_first_ext + 1)
                                        }
                                        s.members.push(member.clone());
                                    }
                                });
                            member_linking = true;
                        }
                    }
                }
                member_linking
            }
            ASN1Type::SequenceOf(so) => so.r#type.link_components_of_notation(tlds),
            _ => false,
        }
    }

    pub fn link_constraint_reference(
        &mut self,
        name: &String,
        tlds: &BTreeMap<String, ToplevelDeclaration>,
    ) -> Result<bool, GrammarError> {
        match self {
            ASN1Type::Null => Ok(false),
            ASN1Type::Choice(c) => {
                let constraints = c.constraints.iter_mut().try_fold(false, |acc, b| {
                    b.link_cross_reference(name, tlds).map(|res| res || acc)
                })?;
                let options = c
                    .options
                    .iter_mut()
                    .flat_map(|o| o.constraints_mut())
                    .try_fold(false, |acc, b| {
                        b.link_cross_reference(name, tlds).map(|res| res || acc)
                    })?;
                Ok(constraints || options)
            }
            ASN1Type::Sequence(s) => {
                let constraints = s.constraints.iter_mut().try_fold(false, |acc, b| {
                    b.link_cross_reference(name, tlds).map(|res| res || acc)
                })?;
                let members = s
                    .members
                    .iter_mut()
                    .flat_map(|o| o.constraints_mut())
                    .try_fold(false, |acc, b| {
                        b.link_cross_reference(name, tlds).map(|res| res || acc)
                    })?;
                Ok(constraints || members)
            }
            ASN1Type::SequenceOf(s) => {
                let a = s.constraints.iter_mut().try_fold(false, |acc, b| {
                    b.link_cross_reference(name, tlds).map(|res| res || acc)
                })?;
                let b = s.r#type.link_constraint_reference(name, tlds)?;
                Ok(a || b)
            }
            ASN1Type::ElsewhereDeclaredType(e) => {
                let id_clone = e.identifier.clone();
                e.constraints_mut().iter_mut().try_fold(false, |acc, b| {
                    b.link_cross_reference(&id_clone, tlds)
                        .map(|res| res || acc)
                })
            }
            ty => {
                if let Some(c) = ty.constraints_mut() {
                    c.iter_mut().try_fold(false, |acc, b| {
                        b.link_cross_reference(name, tlds).map(|res| res || acc)
                    })
                } else {
                    Ok(false)
                }
            }
        }
    }

    pub fn contains_constraint_reference(&self) -> bool {
        match self {
            ASN1Type::Null => false,
            ASN1Type::Boolean(b) => b.constraints.iter().any(|c| c.has_cross_reference()),
            ASN1Type::ObjectIdentifier(o) => o.constraints.iter().any(|c| c.has_cross_reference()),
            ASN1Type::Integer(i) => i.constraints.iter().any(|c| c.has_cross_reference()),
            ASN1Type::BitString(b) => b.constraints.iter().any(|c| c.has_cross_reference()),
            ASN1Type::OctetString(o) => o.constraints.iter().any(|c| c.has_cross_reference()),
            ASN1Type::CharacterString(c) => c.constraints.iter().any(|c| c.has_cross_reference()),
            ASN1Type::Enumerated(e) => e.constraints.iter().any(|c| c.has_cross_reference()),
            ASN1Type::Choice(c) => {
                c.constraints.iter().any(|c| c.has_cross_reference())
                    || c.options.iter().any(|o| {
                        o.r#type.contains_constraint_reference()
                            || o.constraints.iter().any(|c| c.has_cross_reference())
                    })
            }
            ASN1Type::Sequence(s) => {
                s.constraints.iter().any(|c| c.has_cross_reference())
                    || s.members.iter().any(|m| {
                        m.r#type.contains_constraint_reference()
                            || m.constraints.iter().any(|c| c.has_cross_reference())
                    })
            }
            ASN1Type::SequenceOf(s) => {
                s.constraints.iter().any(|c| c.has_cross_reference())
                    || s.r#type.contains_constraint_reference()
            }
            ASN1Type::ElsewhereDeclaredType(e) => {
                e.constraints.iter().any(|c| c.has_cross_reference())
            }
            _ => false,
        }
    }

    pub fn references_class_by_name(&self) -> bool {
        match self {
            ASN1Type::Choice(c) => c
                .options
                .iter()
                .any(|o| o.r#type.references_class_by_name()),
            ASN1Type::Sequence(s) => s
                .members
                .iter()
                .any(|m| m.r#type.references_class_by_name()),
            ASN1Type::SequenceOf(so) => so.r#type.references_class_by_name(),
            ASN1Type::InformationObjectFieldReference(io_ref) => {
                if let Some(ObjectFieldIdentifier::SingleValue(_)) = io_ref.field_path.last() {
                    true
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    pub fn resolve_class_reference(self, tlds: &BTreeMap<String, ToplevelDeclaration>) -> Self {
        match self {
            ASN1Type::Choice(c) => ASN1Type::Choice(Choice {
                extensible: c.extensible,
                options: c
                    .options
                    .into_iter()
                    .map(|option| ChoiceOption {
                        name: option.name,
                        tag: option.tag,
                        r#type: option.r#type.resolve_class_reference(tlds),
                        constraints: vec![],
                    })
                    .collect(),
                constraints: c.constraints,
            }),
            ASN1Type::Sequence(s) => ASN1Type::Sequence(SequenceOrSet {
                extensible: s.extensible,
                constraints: s.constraints,
                components_of: s.components_of,
                members: s
                    .members
                    .into_iter()
                    .map(|mut member| {
                        member.constraints = vec![];
                        member.r#type = member.r#type.resolve_class_reference(tlds);
                        member
                    })
                    .collect(),
            }),
            ASN1Type::InformationObjectFieldReference(_) => self.reassign_type_for_ref(tlds),
            _ => self,
        }
    }

    fn reassign_type_for_ref(mut self, tlds: &BTreeMap<String, ToplevelDeclaration>) -> Self {
        if let Self::InformationObjectFieldReference(ref ior) = self {
            if let Some(t) = tlds
                .iter()
                .find_map(|(_, c)| {
                    c.is_class_with_name(&ior.class)
                        .map(|clazz| clazz.get_field(&ior.field_path))
                })
                .flatten()
                .map(|class_field| class_field.r#type.clone())
                .flatten()
            {
                self = t;
            }
        }
        self
    }

    pub fn link_subtype_constraint(
        &mut self,
        tlds: &BTreeMap<String, ToplevelDeclaration>,
    ) -> Result<bool, GrammarError> {
        match self {
            Self::ElsewhereDeclaredType(e) => {
                if let Some(ToplevelDeclaration::Type(t)) = tlds.get(&e.identifier) {
                    *self = t.r#type.clone();
                    return Ok(true);
                }
                Ok(false)
            }
            _ => Ok(false),
        }
    }
}

impl ASN1Value {
    pub fn is_elsewhere_declared(&self) -> bool {
        match self {
            Self::ElsewhereDeclaredValue { .. }
            | Self::EnumeratedValue {
                enumerated: _,
                enumerable: _,
            } => true,
            _ => false,
        }
    }

    /// Tries to resolve an `ElsewhereDeclaredValue` that references a
    /// path instead of a simple top-level declaration.
    /// ### Example
    /// From X501 LDAP System Schema
    /// ```ignore
    /// namingContexts ATTRIBUTE ::= {
    ///     WITH SYNTAX              DistinguishedName
    ///     USAGE                    dSAOperation
    ///     LDAP-SYNTAX              dn.&id
    ///     LDAP-NAME                {"namingContexts"}
    ///     ID                       id-lat-namingContexts
    /// }
    /// ```
    /// The `LDAP-SYNTAX` field refers to a field ob an information object `dn`.
    pub fn resolve_elsewhere_with_parent(
        &mut self,
        tlds: &BTreeMap<String, ToplevelDeclaration>,
    ) -> Result<(), GrammarError> {
        if let Self::ElsewhereDeclaredValue {
            parent: Some(object_name),
            identifier,
        } = self
        {
            if object_name.contains(".") {
                return Err(error!(NotYetInplemented, "Value references of path length > 2 are not yet supported! Found reference {object_name}.{identifier}"));
            }
            let object = get_declaration![
                tlds,
                object_name,
                Information,
                ASN1Information::Object
            ]
            .ok_or_else(|| error!(LinkerError, "No information object found for identifier {object_name}, parent of {identifier}"))?;
            match &object.fields {
                InformationObjectFields::DefaultSyntax(d) => {
                    match d.iter().find(|elem| elem.identifier() == identifier) {
                        Some(InformationObjectField::FixedValueField(FixedValueField { value, .. })) => {
                            *self = value.clone();
                            return Ok(())
                        }
                        _ => return Err(error!(
                            LinkerError,
                                "No matching value field for identifier {identifier} found in object {object_name}"
                        ))
                    }
                }
                InformationObjectFields::CustomSyntax(c) => {
                    let class_name = &object.class_name;
                    let class = get_declaration![
                        tlds,
                        class_name,
                        Information,
                        ASN1Information::ObjectClass
                    ]
                    .ok_or_else(|| {
                        error!(
                            LinkerError,
                            "No information object class found for identifier {class_name}"
                        )
                    })?;
                    let syntax = class.syntax.as_ref().ok_or_else(|| {
                        error!(LinkerError, "No syntax info found for class {class_name}")
                    })?;
                    let tokens = syntax.flatten();
                    let (mut before, mut after) = (None, None);
                    'iter_syntax: for i in 0..tokens.len() {
                        let expr = tokens.get(i);
                        match expr {
                            Some((
                                _,
                                SyntaxToken::Field(ObjectFieldIdentifier::SingleValue(id)),
                            )) if id == identifier => {
                                before = tokens.get(i - 1).map(|(_, token)| token);
                                after = tokens.get(i + 1).map(|(_, token)| token);
                                break 'iter_syntax;
                            }
                            _ => {}
                        };
                    }
                    for i in 0..c.len() {
                        if let Some(SyntaxApplication::ValueReference(val)) =
                            c.get(i)
                        {
                            match (c.get(i - 1), before, c.get(i + 1), after) {
                                (Some(a), Some(b), _, _) if a.matches(b, &tokens) => {
                                    *self = val.clone();
                                    return Ok(());
                                }
                                (_, _, Some(c), Some(d)) if c.matches(d, &tokens) => {
                                    *self = val.clone();
                                    return Ok(());
                                }
                                _ => {}
                            };
                        }
                    }
                    return Err(error!(
                        LinkerError,
                        "Failed to match expression to syntax of class {class_name}"
                    ));
                }
            }
        }
        Ok(())
    }

    pub fn link_elsewhere_declared(
        &mut self,
        identifier: &String,
        tlds: &BTreeMap<String, ToplevelDeclaration>,
    ) -> Result<bool, GrammarError> {
        match self {
            Self::ElsewhereDeclaredValue { parent: Some(_), .. } => {
                return self.resolve_elsewhere_with_parent(tlds).map(|_| true);
            }
            Self::ElsewhereDeclaredValue {
                identifier: e,
                parent: _,
            }
            | Self::EnumeratedValue {
                enumerated: _,
                enumerable: e,
            } => {
                if let Some(v) = find_tld_or_enum_value_by_name(identifier, &e, tlds) {
                    *self = v;
                    return Ok(true);
                }
            }
            _ => {}
        }
        Ok(false)
    }
}

impl DeclarationElsewhere {
    pub fn find_root_id<'a>(
        &self,
        tlds: &'a BTreeMap<String, ToplevelDeclaration>,
    ) -> Option<&'a ToplevelDeclaration> {
        if let Some(ToplevelDeclaration::Type(ToplevelTypeDeclaration {
            comments: _,
            tag: _,
            name: _,
            r#type: ASN1Type::ElsewhereDeclaredType(e),
            parameterization: _,
            index: _,
        })) = tlds.get(&self.identifier)
        {
            e.find_root_id(tlds)
        } else {
            tlds.get(&self.identifier)
        }
    }
}
