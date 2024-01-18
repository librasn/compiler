//! The `linking` module contains methods to link different tokens of the parsed AST
//! in order to generate correct rust representations.

mod constraints;
mod information_object;
mod utils;

use std::{borrow::BorrowMut, collections::BTreeMap};

use crate::{
    intermediate::{error::*, information_object::*, types::*, utils::*, *},
    validator::linking::utils::bit_string_to_octet_string,
};

use self::utils::{find_tld_or_enum_value_by_name, octet_string_to_bit_string};

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
                    return e
                        .members
                        .iter()
                        .find_map(|m| (&m.name == identifier).then(|| ASN1Value::Integer(m.index)))
                }
                ASN1Type::Integer(i) => {
                    return i.distinguished_values.as_ref().and_then(|dv| {
                        dv.iter().find_map(|d| {
                            (&d.name == identifier).then(|| ASN1Value::Integer(d.value))
                        })
                    })
                }
                _ => (),
            }
        }
        None
    }

    pub fn is_class_with_name(&self, name: &String) -> Option<&InformationObjectClass> {
        match self {
            ToplevelDeclaration::Information(info) => match &info.value {
                ASN1Information::ObjectClass(class) => (&info.name == name).then_some(class),
                _ => None,
            },
            _ => None,
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

    /// Collects supertypes of ASN1 values.
    pub fn collect_supertypes(
        &mut self,
        tlds: &BTreeMap<String, ToplevelDeclaration>,
    ) -> Result<(), GrammarError> {
        match self {
            ToplevelDeclaration::Type(t) => t.collect_supertypes(tlds),
            ToplevelDeclaration::Value(v) => v.collect_supertypes(tlds),
            ToplevelDeclaration::Information(i) => i.collect_supertypes(tlds),
        }
    }
}

impl ToplevelTypeDeclaration {
    /// Collects supertypes of ASN1 values.
    /// In `ToplevelTypeDeclaration`s, values will appear only as `DEFAULT`
    /// values in `SET`s or `SEQUENCE`s.
    pub fn collect_supertypes(
        &mut self,
        tlds: &BTreeMap<String, ToplevelDeclaration>,
    ) -> Result<(), GrammarError> {
        match self.r#type {
            ASN1Type::Set(ref mut s) | ASN1Type::Sequence(ref mut s) => {
                s.members.iter_mut().try_for_each(|m| {
                    m.default_value
                        .as_mut()
                        .map(|d| d.link_with_type(tlds, &m.r#type))
                        .unwrap_or(Ok(()))
                })
            }
            _ => Ok(()),
        }
    }
}

impl ToplevelValueDeclaration {
    /// Collects supertypes and implicit supertypes of an ASN1 value
    /// that are not straightforward to parse on first pass
    /// ### Example
    /// `exmpleValue`'s supertypes would be "ExampleType", "OuterExampleType", and "RootType"
    /// ```ignore
    /// ExampleType ::= OuterExampleType (2..8)
    /// OuterExampleType ::= RootType
    /// RootType ::= INTEGER
    /// exampleValue ExampleType ::= 6
    /// ```
    /// The supertypes are recorded in a `LinkedASN1Value`
    pub fn collect_supertypes(
        &mut self,
        tlds: &BTreeMap<String, ToplevelDeclaration>,
    ) -> Result<(), GrammarError> {
        if let Some(ToplevelDeclaration::Type(tld)) = tlds.get(&self.associated_type) {
            self.value.link_with_type(tlds, &tld.r#type)
        } else {
            let ty = match self.associated_type.as_str() {
                INTEGER => ASN1Type::Integer(Integer {
                    constraints: vec![],
                    distinguished_values: None,
                }),
                BIT_STRING => ASN1Type::BitString(BitString {
                    constraints: vec![],
                    distinguished_values: None,
                }),
                OCTET_STRING => ASN1Type::OctetString(OctetString {
                    constraints: vec![],
                }),
                GENERALIZED_TIME => ASN1Type::GeneralizedTime(GeneralizedTime {
                    constraints: vec![],
                }),
                UTC_TIME => ASN1Type::UTCTime(UTCTime {
                    constraints: vec![],
                }),
                BOOLEAN => ASN1Type::Boolean(Boolean {
                    constraints: vec![],
                }),
                OBJECT_IDENTIFIER => ASN1Type::ObjectIdentifier(ObjectIdentifier {
                    constraints: vec![],
                }),
                _ => return Ok(()),
            };
            self.value.link_with_type(tlds, &ty)
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
                .any(|b| b),
            ASN1Type::Choice(c) => c
                .options
                .iter()
                .map(|o| o.r#type.has_choice_selection_type())
                .any(|b| b),
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
                .try_for_each(|m| m.r#type.link_choice_selection_type(tlds)),
            ASN1Type::Choice(c) => c
                .options
                .iter_mut()
                .try_for_each(|o: &mut ChoiceOption| o.r#type.link_choice_selection_type(tlds)),
            ASN1Type::SequenceOf(s) | ASN1Type::SetOf(s) => {
                s.r#type.link_choice_selection_type(tlds)
            }
            _ => Ok(()),
        }
    }

    pub fn contains_components_of_notation(&self) -> bool {
        match self {
            ASN1Type::Choice(c) => c
                .options
                .iter()
                .any(|o| o.r#type.contains_components_of_notation()),
            ASN1Type::Set(s) | ASN1Type::Sequence(s) => {
                !s.components_of.is_empty()
                    || s.members
                        .iter()
                        .any(|m| m.r#type.contains_components_of_notation())
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
            ASN1Type::Choice(c) => c
                .options
                .iter_mut()
                .any(|o| o.r#type.link_components_of_notation(tlds)),
            ASN1Type::Set(s) | ASN1Type::Sequence(s) => {
                let mut member_linking = s
                    .members
                    .iter_mut()
                    .any(|m| m.r#type.link_components_of_notation(tlds));
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
                matches!(
                    io_ref.field_path.last(),
                    Some(ObjectFieldIdentifier::SingleValue(_))
                )
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
                .and_then(|class_field| class_field.r#type.clone())
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
    pub fn link_with_type(
        &mut self,
        tlds: &BTreeMap<String, ToplevelDeclaration>,
        ty: &ASN1Type,
    ) -> Result<(), GrammarError> {
        #[allow(clippy::useless_asref)] // false positive
        match (ty, self.as_mut()) {
            (
                ASN1Type::ElsewhereDeclaredType(e),
                ASN1Value::LinkedNestedValue { supertypes, value },
            ) => {
                supertypes.push(e.identifier.clone());
                if let ASN1Value::LinkedIntValue { integer_type, .. } = value.borrow_mut() {
                    let int_type = e.constraints.iter().fold(IntegerType::Unbounded, |acc, c| {
                        c.integer_constraints().max_restrictive(acc)
                    });
                    *integer_type = int_type;
                }
                if let Some(ToplevelDeclaration::Type(t)) = tlds.get(&e.identifier) {
                    self.link_with_type(tlds, &t.r#type)
                } else {
                    Err(GrammarError {
                        details: format!("Failed to link value with '{}'", e.identifier),
                        kind: GrammarErrorType::LinkerError,
                    })
                }
            }
            (ASN1Type::ElsewhereDeclaredType(_), ASN1Value::ElsewhereDeclaredValue { .. }) => {
                Ok(())
            }
            (ASN1Type::ElsewhereDeclaredType(e), val) => {
                if let ASN1Value::Integer(value) = *val {
                    let int_type = e.constraints.iter().fold(IntegerType::Unbounded, |acc, c| {
                        c.integer_constraints().max_restrictive(acc)
                    });
                    *val = ASN1Value::LinkedIntValue {
                        integer_type: int_type,
                        value,
                    };
                }
                *self = ASN1Value::LinkedNestedValue {
                    supertypes: vec![e.identifier.clone()],
                    value: Box::new((*val).clone()),
                };
                if let Some(ToplevelDeclaration::Type(t)) = tlds.get(&e.identifier) {
                    self.link_with_type(tlds, &t.r#type)
                } else {
                    Err(GrammarError {
                        details: format!("Failed to link value with '{}'", e.identifier),
                        kind: GrammarErrorType::LinkerError,
                    })
                }
            }
            (ASN1Type::Choice(c), ASN1Value::Choice(opt, val)) => {
                if let Some(option) = c.options.iter().find(|o| &o.name == opt) {
                    val.link_with_type(tlds, &option.r#type)
                } else {
                    Err(GrammarError {
                        details: format!("Failed to link value with '{}'", opt),
                        kind: GrammarErrorType::LinkerError,
                    })
                }
            }
            (ASN1Type::Choice(c), ASN1Value::LinkedNestedValue { supertypes, value })
                if matches![**value, ASN1Value::Choice(_, _)] =>
            {
                supertypes.pop();
                if let ASN1Value::Choice(opt, val) = &mut **value {
                    if let Some(option) = c.options.iter().find(|o| &o.name == opt) {
                        val.link_with_type(tlds, &option.r#type)
                    } else {
                        Err(GrammarError {
                            details: format!("Failed to link value with '{}'", opt),
                            kind: GrammarErrorType::LinkerError,
                        })
                    }
                } else {
                    Ok(())
                }
            }
            (ASN1Type::Set(s), ASN1Value::SequenceOrSet(val))
            | (ASN1Type::Sequence(s), ASN1Value::SequenceOrSet(val)) => {
                *self = Self::link_struct_like(val, s, tlds)?;
                Ok(())
            }
            (ASN1Type::Set(s), ASN1Value::LinkedNestedValue { value, .. })
            | (ASN1Type::Sequence(s), ASN1Value::LinkedNestedValue { value, .. })
                if matches![**value, ASN1Value::SequenceOrSet(_)] =>
            {
                if let ASN1Value::SequenceOrSet(val) = &mut **value {
                    *value = Box::new(Self::link_struct_like(val, s, tlds)?);
                }
                Ok(())
            }
            (ASN1Type::SetOf(s), ASN1Value::SequenceOrSetOf(val))
            | (ASN1Type::SequenceOf(s), ASN1Value::SequenceOrSetOf(val)) => val
                .iter_mut()
                .try_for_each(|v| v.link_with_type(tlds, &s.r#type)),
            (ASN1Type::Integer(i), ASN1Value::Integer(val)) => {
                *self = ASN1Value::LinkedIntValue {
                    integer_type: i.int_type(),
                    value: *val,
                };
                Ok(())
            }
            (ASN1Type::CharacterString(t), ASN1Value::String(s)) => {
                *self = ASN1Value::LinkedCharStringValue(t.r#type, s.clone());
                Ok(())
            }
            (ASN1Type::CharacterString(t), ASN1Value::LinkedNestedValue { value, .. })
                if matches![**value, ASN1Value::String(_)] =>
            {
                if let ASN1Value::String(s) = &**value {
                    *value = Box::new(ASN1Value::LinkedCharStringValue(t.r#type, s.clone()));
                }
                Ok(())
            }
            (ASN1Type::BitString(_), ASN1Value::OctetString(o)) => {
                *self = ASN1Value::BitString(octet_string_to_bit_string(o));
                Ok(())
            }
            (ASN1Type::BitString(_), ASN1Value::LinkedNestedValue { value, .. })
                if matches![**value, ASN1Value::OctetString(_)] =>
            {
                if let ASN1Value::OctetString(o) = &**value {
                    *value = Box::new(ASN1Value::BitString(octet_string_to_bit_string(o)));
                }
                Ok(())
            }
            (ASN1Type::OctetString(_), ASN1Value::BitString(b)) => {
                *self = ASN1Value::OctetString(bit_string_to_octet_string(b)?);
                Ok(())
            }
            (ASN1Type::OctetString(_), ASN1Value::LinkedNestedValue { value, .. })
                if matches![**value, ASN1Value::BitString(_)] =>
            {
                if let ASN1Value::BitString(b) = &**value {
                    *value = Box::new(ASN1Value::OctetString(bit_string_to_octet_string(b)?));
                }
                Ok(())
            }
            (ASN1Type::Integer(i), ASN1Value::LinkedIntValue { integer_type, .. }) => {
                let int_type = i.int_type().max_restrictive(*integer_type);
                *integer_type = int_type;
                Ok(())
            }
            (ASN1Type::Integer(i), ASN1Value::LinkedNestedValue { value, .. })
                if matches![**value, ASN1Value::ElsewhereDeclaredValue { .. }] =>
            {
                if let ASN1Value::ElsewhereDeclaredValue { identifier, .. } = &**value {
                    if let Some(distinguished_value) =
                        i.distinguished_values.as_ref().and_then(|dist_vals| {
                            dist_vals
                                .iter()
                                .find_map(|d| (&d.name == identifier).then_some(d.value))
                        })
                    {
                        *value = Box::new(ASN1Value::LinkedIntValue {
                            integer_type: i.int_type(),
                            value: distinguished_value,
                        });
                    }
                }
                Ok(())
            }
            (ASN1Type::Integer(i), ASN1Value::ElsewhereDeclaredValue { identifier, .. }) => {
                if let Some(value) = i.distinguished_values.as_ref().and_then(|dist_vals| {
                    dist_vals
                        .iter()
                        .find_map(|d| (&d.name == identifier).then_some(d.value))
                }) {
                    *self = ASN1Value::LinkedIntValue {
                        integer_type: i.int_type(),
                        value,
                    };
                }
                Ok(())
            }
            (ASN1Type::Enumerated(_), ASN1Value::LinkedNestedValue { value, .. })
                if matches![**value, ASN1Value::ElsewhereDeclaredValue { .. }] =>
            {
                if let ASN1Value::ElsewhereDeclaredValue { identifier, .. } = &**value {
                    if let Some((_, tld)) = tlds
                        .iter()
                        .find(|(_, tld)| tld.has_enum_value(None, identifier))
                    {
                        *value = Box::new(ASN1Value::EnumeratedValue {
                            enumerated: tld.name().clone(),
                            enumerable: identifier.clone(),
                        });
                    }
                }
                Ok(())
            }
            (ASN1Type::Enumerated(_), ASN1Value::ElsewhereDeclaredValue { identifier, .. }) => {
                if let Some((_, tld)) = tlds
                    .iter()
                    .find(|(_, tld)| tld.has_enum_value(None, identifier))
                {
                    *self = ASN1Value::EnumeratedValue {
                        enumerated: tld.name().clone(),
                        enumerable: identifier.clone(),
                    };
                }
                Ok(())
            }
            _ => Ok(()),
        }
    }

    fn link_struct_like(
        val: &mut [(String, Box<ASN1Value>)],
        s: &SequenceOrSet,
        tlds: &BTreeMap<String, ToplevelDeclaration>,
    ) -> Result<ASN1Value, GrammarError> {
        val.iter_mut().try_for_each(|v| {
            if let Some(member) = s.members.iter().find(|m| m.name == v.0) {
                v.1.link_with_type(tlds, &member.r#type)
            } else {
                Err(GrammarError {
                    details: format!("Failed to link value with '{}'", v.0),
                    kind: GrammarErrorType::LinkerError,
                })
            }
        })?;

        s.members
            .iter()
            .map(|member| {
                val.iter()
                    .find_map(|(name, value)| {
                        (name == &member.name)
                            .then_some(StructLikeFieldValue::Explicit(value.clone()))
                    })
                    .or(member
                        .default_value
                        .as_ref()
                        .map(|d| StructLikeFieldValue::Implicit(Box::new(d.clone()))))
                    .ok_or_else(|| GrammarError {
                        details: format!("No value for field {} found!", member.name),
                        kind: GrammarErrorType::LinkerError,
                    })
                    .map(|field_value| (member.name.clone(), field_value))
            })
            .collect::<Result<Vec<_>, _>>()
            .map(ASN1Value::LinkedStructLikeValue)
    }

    pub fn is_elsewhere_declared(&self) -> bool {
        let is = matches!(
            self,
            Self::ElsewhereDeclaredValue { .. }
                | Self::EnumeratedValue {
                    enumerated: _,
                    enumerable: _,
                }
        );
        is
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
            if object_name.contains('.') {
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
                        if let Some(SyntaxApplication::ValueReference(val)) = c.get(i) {
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
            Self::ElsewhereDeclaredValue {
                parent: Some(_), ..
            } => {
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
                if let Some(v) = find_tld_or_enum_value_by_name(identifier, e, tlds) {
                    *self = v;
                    return Ok(true);
                }
            }
            _ => {}
        }
        Ok(false)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use crate::intermediate::{types::*, *};

    macro_rules! tld {
        ($name:literal, $ty:expr) => {
            ToplevelTypeDeclaration {
                comments: String::new(),
                tag: None,
                index: None,
                name: $name.into(),
                r#type: $ty,
                parameterization: None,
            }
        };
    }

    #[test]
    fn links_asn1_value() {
        let tlds: BTreeMap<String, ToplevelDeclaration> = {
            let mut map = BTreeMap::new();
            map.insert(
                "RootBool".into(),
                ToplevelDeclaration::Type(tld!(
                    "RootBool",
                    ASN1Type::Boolean(Boolean {
                        constraints: vec![]
                    })
                )),
            );
            map.insert(
                "IntermediateBool".into(),
                ToplevelDeclaration::Type(tld!(
                    "IntermediateBool",
                    ASN1Type::ElsewhereDeclaredType(DeclarationElsewhere {
                        parent: None,
                        identifier: String::from("RootBool"),
                        constraints: vec![]
                    })
                )),
            );
            map.insert(
                "BaseChoice".into(),
                ToplevelDeclaration::Type(tld!(
                    "BaseChoice",
                    ASN1Type::Choice(Choice {
                        extensible: None,
                        constraints: vec![],
                        options: vec![ChoiceOption {
                            name: String::from("first"),
                            constraints: vec![],
                            tag: None,
                            r#type: ASN1Type::ElsewhereDeclaredType(DeclarationElsewhere {
                                parent: None,
                                identifier: String::from("IntermediateBool"),
                                constraints: vec![]
                            })
                        }]
                    })
                )),
            );
            map
        };
        let mut example_value = ToplevelValueDeclaration {
            comments: String::new(),
            name: "exampleValue".into(),
            associated_type: "BaseChoice".into(),
            index: None,
            value: ASN1Value::Choice(String::from("first"), Box::new(ASN1Value::Boolean(true))),
        };
        example_value.collect_supertypes(&tlds).unwrap();
        assert_eq!(
            example_value,
            ToplevelValueDeclaration {
                comments: "".into(),
                name: "exampleValue".into(),
                associated_type: "BaseChoice".into(),
                value: ASN1Value::Choice(
                    "first".into(),
                    Box::new(ASN1Value::LinkedNestedValue {
                        supertypes: vec!["IntermediateBool".into(), "RootBool".into()],
                        value: Box::new(ASN1Value::Boolean(true))
                    })
                ),
                index: None
            }
        )
    }
}
