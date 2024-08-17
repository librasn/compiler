//! The `linking` module contains methods to link different tokens of the parsed AST
//! in order to generate correct rust representations.

mod constraints;
mod information_object;
mod types;
mod utils;

use std::{borrow::BorrowMut, collections::BTreeMap};

use crate::{
    common::INTERNAL_NESTED_TYPE_NAME_PREFIX,
    intermediate::{error::*, information_object::*, types::*, utils::*, *},
    lexer::Span,
    validator::{
        linking::utils::bit_string_to_octet_string,
        parameterization::{Parameterization, ParameterizationArgument},
    },
};

use self::{
    parameterization::ParameterGovernor,
    utils::{find_tld_or_enum_value_by_name, octet_string_to_bit_string},
};

use super::{Constraint, Parameter, TableConstraint};

macro_rules! error {
    ($kind:ident, $($arg:tt)*) => {
        GrammarError {
            details: format!($($arg)*),
            kind: GrammarErrorType::$kind,
        }
    };
}

impl ToplevelDefinition {
    pub(crate) fn is_parameterized(&self) -> bool {
        match self {
            ToplevelDefinition::Information(ToplevelInformationDefinition {
                parameterization: Some(_),
                ..
            })
            | ToplevelDefinition::Type(ToplevelTypeDefinition {
                parameterization: Some(_),
                ..
            })
            | ToplevelDefinition::Value(ToplevelValueDefinition {
                parameterization: Some(_),
                ..
            }) => true,
            ToplevelDefinition::Type(ToplevelTypeDefinition {
                ty: ASN1Type::Sequence(s),
                ..
            })
            | ToplevelDefinition::Type(ToplevelTypeDefinition {
                ty: ASN1Type::Set(s),
                ..
            }) => s.members.iter().any(|m| {
                m.constraints
                    .iter()
                    .any(|c| matches!(c, Constraint::Parameter(_)))
            }),
            ToplevelDefinition::Type(ToplevelTypeDefinition {
                ty: ASN1Type::SequenceOf(s),
                ..
            })
            | ToplevelDefinition::Type(ToplevelTypeDefinition {
                ty: ASN1Type::SetOf(s),
                ..
            }) => s.element_type.constraints().map_or(false, |constraints| {
                constraints
                    .iter()
                    .any(|c| matches!(c, Constraint::Parameter(_)))
            }),
            _ => false,
        }
    }

    pub(crate) fn get_distinguished_or_enum_value(
        &self,
        type_name: Option<&String>,
        identifier: &String,
    ) -> Option<ASN1Value> {
        if let ToplevelDefinition::Type(t) = self {
            if type_name.is_some() && Some(&t.name) != type_name {
                return None;
            }
            match &t.ty {
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
            ToplevelDefinition::Information(info) => match &info.value {
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
            ToplevelDefinition::Type(t) => t.ty.contains_constraint_reference(),
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
    ///
    /// ## Parameterization
    /// This linking step also resolves implementations of parameterized types.
    /// The compiler does not create representations of abstract parameterized types
    /// but only of actual implementations. For example, no rust output
    /// will be generated for
    /// ```ignore
    /// ParamType { INTEGER: lower, BOOLEAN: flag } ::= SEQUENCE {
    ///     int-value INTEGER (lower..12),
    ///     bool-value BOOLEAN DEFAULT flag
    /// }
    /// ```
    /// but an implementing type such as
    /// ```ignore
    /// ImplType ::= ParamType { 2, TRUE }
    /// ```
    /// will be represented in the generated rust bindings.
    /// ### Params
    ///  * `tlds` - vector of other top-level declarations that will be searched as the method resolves a reference
    /// returns `true` if the reference was resolved successfully.
    pub fn link_constraint_reference(
        &mut self,
        tlds: &BTreeMap<String, ToplevelDefinition>,
    ) -> Result<bool, GrammarError> {
        match self {
            ToplevelDefinition::Type(t) => {
                if let Some(replacement) = t.ty.link_constraint_reference(&t.name, tlds)? {
                    t.ty = replacement;
                }
                Ok(true)
            }
            // TODO: Cover constraint references in other types of top-level declarations
            _ => Ok(false),
        }
    }

    /// Collects supertypes of ASN1 values.
    pub fn collect_supertypes(
        &mut self,
        tlds: &BTreeMap<String, ToplevelDefinition>,
    ) -> Result<(), GrammarError> {
        match self {
            ToplevelDefinition::Type(t) => t.ty.collect_supertypes(tlds),
            ToplevelDefinition::Value(v) => v.collect_supertypes(tlds),
            ToplevelDefinition::Information(i) => i.collect_supertypes(tlds),
        }
    }
}

impl ToplevelValueDefinition {
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
        tlds: &BTreeMap<String, ToplevelDefinition>,
    ) -> Result<(), GrammarError> {
        if let Some(ToplevelDefinition::Type(tld)) =
            tlds.get(self.associated_type.as_str().as_ref())
        {
            self.value.link_with_type(tlds, &tld.ty, Some(&tld.name))
        } else {
            self.value.link_with_type(tlds, &self.associated_type, None)
        }
    }
}

impl ASN1Type {
    /// Collects supertypes of ASN1 values.
    /// In `ToplevelTypeDefinition`s, values will appear only as `DEFAULT`
    /// values in `SET`s or `SEQUENCE`s.
    pub fn collect_supertypes(
        &mut self,
        tlds: &BTreeMap<String, ToplevelDefinition>,
    ) -> Result<(), GrammarError> {
        match self {
            ASN1Type::Set(ref mut s) | ASN1Type::Sequence(ref mut s) => {
                s.members.iter_mut().try_for_each(|m| {
                    m.default_value
                        .as_mut()
                        .map(|d| d.link_with_type(tlds, &m.ty, Some(&m.ty.as_str().into_owned())))
                        .unwrap_or(Ok(()))
                })
            }
            _ => Ok(()),
        }
    }

    pub fn has_choice_selection_type(&self) -> bool {
        match self {
            ASN1Type::ChoiceSelectionType(_) => true,
            ASN1Type::Sequence(s) | ASN1Type::Set(s) => s
                .members
                .iter()
                .map(|m| m.ty.has_choice_selection_type())
                .any(|b| b),
            ASN1Type::Choice(c) => c
                .options
                .iter()
                .map(|o| o.ty.has_choice_selection_type())
                .any(|b| b),
            ASN1Type::SequenceOf(s) | ASN1Type::SetOf(s) => {
                s.element_type.has_choice_selection_type()
            }
            _ => false,
        }
    }

    pub fn link_choice_selection_type(
        &mut self,
        tlds: &BTreeMap<String, ToplevelDefinition>,
    ) -> Result<(), GrammarError> {
        match self {
            ASN1Type::ChoiceSelectionType(c) => {
                if let Some(ToplevelDefinition::Type(parent)) = tlds.get(&c.choice_name) {
                    *self = parent.ty.clone();
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
                .try_for_each(|m| m.ty.link_choice_selection_type(tlds)),
            ASN1Type::Choice(c) => c
                .options
                .iter_mut()
                .try_for_each(|o: &mut ChoiceOption| o.ty.link_choice_selection_type(tlds)),
            ASN1Type::SequenceOf(s) | ASN1Type::SetOf(s) => {
                s.element_type.link_choice_selection_type(tlds)
            }
            _ => Ok(()),
        }
    }

    pub fn contains_components_of_notation(&self) -> bool {
        match self {
            ASN1Type::Choice(c) => c
                .options
                .iter()
                .any(|o| o.ty.contains_components_of_notation()),
            ASN1Type::Set(s) | ASN1Type::Sequence(s) => {
                !s.components_of.is_empty()
                    || s.members
                        .iter()
                        .any(|m| m.ty.contains_components_of_notation())
            }
            ASN1Type::SequenceOf(so) => so.element_type.contains_components_of_notation(),
            _ => false,
        }
    }

    pub fn link_components_of_notation(
        &mut self,
        tlds: &BTreeMap<String, ToplevelDefinition>,
    ) -> bool {
        match self {
            ASN1Type::Choice(c) => c
                .options
                .iter_mut()
                .any(|o| o.ty.link_components_of_notation(tlds)),
            ASN1Type::Set(s) | ASN1Type::Sequence(s) => {
                let mut member_linking = s
                    .members
                    .iter_mut()
                    .any(|m| m.ty.link_components_of_notation(tlds));
                // TODO: properly link components of in extensions
                // TODO: link components of Class field, such as COMPONENTS OF BILATERAL.&id
                for comp_link in &s.components_of {
                    if let Some(ToplevelDefinition::Type(linked)) = tlds.get(comp_link) {
                        if let ASN1Type::Sequence(linked_seq) = &linked.ty {
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
            ASN1Type::SequenceOf(so) => so.element_type.link_components_of_notation(tlds),
            _ => false,
        }
    }

    pub fn link_constraint_reference(
        &mut self,
        name: &String,
        tlds: &BTreeMap<String, ToplevelDefinition>,
    ) -> Result<Option<ASN1Type>, GrammarError> {
        let mut self_replacement = None;
        match self {
            ASN1Type::Null => (),
            ASN1Type::Choice(c) => {
                for b in c.constraints.iter_mut() {
                    b.link_cross_reference(name, tlds)?;
                }
                for opt in c.options.iter_mut() {
                    if let Some(replacement) = opt.ty.link_constraint_reference(name, tlds)? {
                        opt.ty = replacement;
                    }
                    for c in opt.constraints.iter_mut() {
                        c.link_cross_reference(name, tlds)?;
                    }
                    for c in opt.ty.constraints_mut().unwrap_or(&mut vec![]).iter_mut() {
                        c.link_cross_reference(name, tlds)?;
                    }
                }
            }
            ASN1Type::Set(s) | ASN1Type::Sequence(s) => {
                for b in s.constraints.iter_mut() {
                    b.link_cross_reference(name, tlds)?;
                }
                for m in s.members.iter_mut() {
                    if let Some(replacement) = m.ty.link_constraint_reference(name, tlds)? {
                        m.ty = replacement;
                    }
                }
            }
            ASN1Type::SetOf(s) | ASN1Type::SequenceOf(s) => {
                for b in s.constraints.iter_mut() {
                    b.link_cross_reference(name, tlds)?;
                }
                if let Some(replacement) = s.element_type.link_constraint_reference(name, tlds)? {
                    s.element_type = Box::new(replacement);
                }
            }
            ASN1Type::ElsewhereDeclaredType(e) => {
                if let Some(Constraint::Parameter(args)) = e
                    .constraints()
                    .iter()
                    .find(|c| matches![c, Constraint::Parameter(_)])
                {
                    self_replacement = Some(Self::resolve_parameters(
                        &e.identifier,
                        e.parent.as_ref(),
                        tlds,
                        args,
                    )?);
                } else {
                    let id_clone = e.identifier.clone();
                    for c in e.constraints_mut() {
                        c.link_cross_reference(&id_clone, tlds)?;
                    }
                }
            }
            ASN1Type::InformationObjectFieldReference(iofr) => {
                if let Some(ToplevelDefinition::Information(ToplevelInformationDefinition {
                    value: ASN1Information::ObjectClass(clazz),
                    ..
                })) = tlds.get(&iofr.class)
                {
                    if let Some(InformationObjectClassField { ty: Some(ty), .. }) =
                        clazz.get_field(&iofr.field_path)
                    {
                        self_replacement = Some(ty.clone());
                    }
                }
            }
            ty => {
                if let Some(c) = ty.constraints_mut() {
                    for c in c.iter_mut() {
                        c.link_cross_reference(name, tlds)?;
                    }
                }
            }
        }
        Ok(self_replacement)
    }

    pub(crate) fn resolve_parameters(
        identifier: &String,
        _parent: Option<&String>,
        tlds: &BTreeMap<String, ToplevelDefinition>,
        args: &[Parameter],
    ) -> Result<ASN1Type, GrammarError> {
        match tlds.get(identifier) {
            Some(ToplevelDefinition::Type(ToplevelTypeDefinition {
                ty,
                parameterization: Some(Parameterization { parameters }),
                ..
            })) => {
                let mut impl_template = ty.clone();
                let mut impl_tlds = tlds.clone();
                let mut table_constraint_replacements = BTreeMap::new();
                for (
                    index,
                    ParameterizationArgument {
                        dummy_reference,
                        param_governor,
                    },
                ) in parameters.iter().enumerate()
                {
                    let arg = args.get(index).ok_or_else(|| GrammarError {
                            details: format!("Did not find an argument for parameter {dummy_reference} of {identifier}"),
                            kind: GrammarErrorType::LinkerError,
                        })?;
                    match (arg, param_governor) {
                            (Parameter::ValueParameter(v), ParameterGovernor::TypeOrClass(gov)) => {
                                impl_tlds.insert(
                                    dummy_reference.clone(),
                                    ToplevelDefinition::Value(ToplevelValueDefinition::from((
                                        Span::new(dummy_reference),
                                        v.clone(),
                                        gov.clone(),
                                    ))),
                                );
                            }
                            (Parameter::TypeParameter(t), _) => {
                                impl_tlds.insert(
                                    dummy_reference.clone(),
                                    ToplevelDefinition::Type(ToplevelTypeDefinition::from((
                                        Span::new(dummy_reference),
                                        t.clone(),
                                    ))),
                                );
                            },
                            (Parameter::InformationObjectParameter(_), _) => todo!(),
                            (Parameter::ObjectSetParameter(o), ParameterGovernor::Class(c)) => {
                                match &o.values.first() {
                                    Some(osv) if o.values.len() == 1 => {
                                        #[allow(suspicious_double_ref_op)]
                                        table_constraint_replacements.insert(dummy_reference, osv.clone());
                                    }
                                    _ => return Err(GrammarError { details: "Expected object set value argument to contain single object set value!".to_owned(), kind: GrammarErrorType::LinkerError })
                                }
                                let mut info = ASN1Information::ObjectSet(o.clone());
                                info.link_object_set_reference(tlds);
                                let mut tld = ToplevelInformationDefinition::from((
                                    Span::new(dummy_reference),
                                    info,
                                    c.as_str()
                                ));
                                tld = tld.resolve_class_reference(tlds);
                                impl_tlds.insert(
                                    dummy_reference.clone(),
                                    ToplevelDefinition::Information(tld),
                                );
                            },
                            _ => return Err(GrammarError {
                                details: format!("Mismatching argument for parameter {dummy_reference} of {identifier}"),
                                kind: GrammarErrorType::LinkerError,
                            })
                        }
                }
                impl_template.link_elsewhere_declared(&impl_tlds)?;
                if let Some(replacement) =
                    impl_template.link_constraint_reference(identifier, &impl_tlds)?
                {
                    impl_template = replacement;
                };
                impl_template
                    .collect_supertypes(&impl_tlds)
                    .or_else(|_| impl_template.collect_supertypes(tlds))?;
                for (dummy_reference, osv) in table_constraint_replacements {
                    impl_template.reassign_table_constraint(dummy_reference, osv)?;
                }
                Ok(impl_template)
            }
            _ => Err(GrammarError {
                details: format!(
                    "Failed to resolve supertype {identifier} of parameterized implementation."
                ),
                kind: GrammarErrorType::LinkerError,
            }),
        }
    }

    /// In certain parameterization cases, the constraining object set of a table constraint
    /// has to be reassigned. Consider the following example:
    /// ```ignore
    /// ProtocolExtensionContainer {NGAP-PROTOCOL-EXTENSION : ExtensionSetParam} ::=
    ///     SEQUENCE (SIZE (1..4)) OF
    ///     ProtocolExtensionField {{ExtensionSetParam}}
    ///
    /// ProtocolExtensionField {NGAP-PROTOCOL-EXTENSION : ExtensionSetParam} ::= SEQUENCE {
    ///     id                    NGAP-PROTOCOL-EXTENSION.&id                ({ExtensionSetParam}),
    ///     extensionValue        NGAP-PROTOCOL-EXTENSION.&Extension        ({ExtensionSetParam}{@id})
    /// }
    ///
    /// ActualExtensions ::= ProtocolExtensionContainer { {ApplicableSet} }
    /// ApplicableSet NGAP-PROTOCOL-EXTENSION ::= { ... }
    /// ```
    /// Since the compiler only creates bindings for actual implementations of abstract items,
    /// the `ExtensionSetParam` references in `ProtocolExtensionField`'s table constraints need
    /// to be reassigned to the actual object sets that are passed in by the implementations of
    /// the abstract classes.
    fn reassign_table_constraint(
        &mut self,
        reference_id_before: &str,
        replacement: &ObjectSetValue,
    ) -> Result<(), GrammarError> {
        match self {
            ASN1Type::Sequence(s) | ASN1Type::Set(s) => {
                for m in &mut s.members {
                    if let Some(constraints) = m.ty.constraints_mut() {
                        for c in constraints {
                            if let Constraint::TableConstraint(TableConstraint {
                                object_set: ObjectSet { values, .. },
                                ..
                            }) = c
                            {
                                for value in values {
                                    match value {
                                        ObjectSetValue::Reference(r)
                                            if r == reference_id_before =>
                                        {
                                            *value = replacement.clone();
                                        }
                                        _ => (),
                                    }
                                }
                            }
                        }
                    }
                }
                Ok(())
            }
            ASN1Type::SequenceOf(s) | ASN1Type::SetOf(s) => s
                .element_type
                .reassign_table_constraint(reference_id_before, replacement),
            _ => Ok(()),
        }
    }

    fn link_elsewhere_declared(
        &mut self,
        tlds: &BTreeMap<String, ToplevelDefinition>,
    ) -> Result<(), GrammarError> {
        match self {
            ASN1Type::Choice(c) => c
                .options
                .iter_mut()
                .try_for_each(|o| o.ty.link_elsewhere_declared(tlds)),
            ASN1Type::Set(s) | ASN1Type::Sequence(s) => s
                .members
                .iter_mut()
                .try_for_each(|o| o.ty.link_elsewhere_declared(tlds)),
            ASN1Type::SequenceOf(s) | ASN1Type::SetOf(s) => {
                s.element_type.link_elsewhere_declared(tlds)
            }
            ASN1Type::ElsewhereDeclaredType(e) => {
                if let Some(ToplevelDefinition::Type(tld)) = tlds.get(&e.identifier) {
                    *self = tld.ty.clone();
                    Ok(())
                } else {
                    Err(GrammarError {
                        details: format!(
                            "Failed to resolve argument {} of parameterized implementation.",
                            e.identifier
                        ),
                        kind: GrammarErrorType::LinkerError,
                    })
                }
            }
            ASN1Type::InformationObjectFieldReference(iofr) => {
                if let Some(ToplevelDefinition::Information(ToplevelInformationDefinition {
                    value: ASN1Information::ObjectClass(c),
                    ..
                })) = tlds.get(&iofr.class)
                {
                    if let Some(field) = c.get_field(&iofr.field_path) {
                        if let Some(ref ty) = field.ty {
                            *self = ty.clone();
                        }
                        return Ok(());
                    }
                }
                Err(GrammarError {
                    details: format!(
                        "Failed to resolve argument {}.{} of parameterized implementation.",
                        iofr.class,
                        iofr.field_path
                            .iter()
                            .map(|f| f.identifier().clone())
                            .collect::<Vec<_>>()
                            .join(".")
                    ),
                    kind: GrammarErrorType::LinkerError,
                })
            }
            ASN1Type::ChoiceSelectionType(_) => Err(GrammarError {
                details: "Linking choice selection type is not yet supported!".to_string(),
                kind: GrammarErrorType::NotYetInplemented,
            }),
            _ => Ok(()),
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
                        o.ty.contains_constraint_reference()
                            || o.constraints.iter().any(|c| c.has_cross_reference())
                    })
            }
            ASN1Type::Set(s) | ASN1Type::Sequence(s) => {
                s.constraints.iter().any(|c| c.has_cross_reference())
                    || s.members.iter().any(|m| {
                        m.ty.contains_constraint_reference()
                            || m.default_value
                                .as_ref()
                                .map_or(false, |d| d.is_elsewhere_declared())
                            || m.constraints.iter().any(|c| c.has_cross_reference())
                    })
            }
            ASN1Type::SetOf(s) | ASN1Type::SequenceOf(s) => {
                s.constraints.iter().any(|c| c.has_cross_reference())
                    || s.element_type.contains_constraint_reference()
            }
            ASN1Type::ElsewhereDeclaredType(e) => {
                e.constraints.iter().any(|c| c.has_cross_reference())
            }
            _ => false,
        }
    }

    pub fn references_class_by_name(&self) -> bool {
        match self {
            ASN1Type::Choice(c) => c.options.iter().any(|o| o.ty.references_class_by_name()),
            ASN1Type::Sequence(s) => s.members.iter().any(|m| m.ty.references_class_by_name()),
            ASN1Type::SequenceOf(so) => so.element_type.references_class_by_name(),
            ASN1Type::InformationObjectFieldReference(io_ref) => {
                matches!(
                    io_ref.field_path.last(),
                    Some(ObjectFieldIdentifier::SingleValue(_))
                )
            }
            _ => false,
        }
    }

    pub fn resolve_class_reference(self, tlds: &BTreeMap<String, ToplevelDefinition>) -> Self {
        match self {
            ASN1Type::Choice(c) => ASN1Type::Choice(Choice {
                extensible: c.extensible,
                options: c
                    .options
                    .into_iter()
                    .map(|option| ChoiceOption {
                        name: option.name,
                        tag: option.tag,
                        ty: option.ty.resolve_class_reference(tlds),
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
                        member.ty = member.ty.resolve_class_reference(tlds);
                        member
                    })
                    .collect(),
            }),
            ASN1Type::InformationObjectFieldReference(_) => self.reassign_type_for_ref(tlds),
            _ => self,
        }
    }

    fn reassign_type_for_ref(mut self, tlds: &BTreeMap<String, ToplevelDefinition>) -> Self {
        if let Self::InformationObjectFieldReference(ref ior) = self {
            if let Some(t) = tlds
                .iter()
                .find_map(|(_, c)| {
                    c.is_class_with_name(&ior.class)
                        .map(|clazz| clazz.get_field(&ior.field_path))
                })
                .flatten()
                .and_then(|class_field| class_field.ty.clone())
            {
                self = t;
            }
        }
        self
    }

    pub fn link_subtype_constraint(
        &mut self,
        tlds: &BTreeMap<String, ToplevelDefinition>,
    ) -> Result<(), GrammarError> {
        if let Self::ElsewhereDeclaredType(e) = self {
            if let Some(ToplevelDefinition::Type(t)) = tlds.get(&e.identifier) {
                *self = t.ty.clone();
            }
        }
        Ok(())
    }
}

impl ASN1Value {
    pub fn link_with_type(
        &mut self,
        tlds: &BTreeMap<String, ToplevelDefinition>,
        ty: &ASN1Type,
        type_name: Option<&String>,
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
                if let Some(ToplevelDefinition::Type(t)) = tlds.get(&e.identifier) {
                    self.link_with_type(tlds, &t.ty, Some(&t.name))
                } else {
                    Err(GrammarError {
                        details: format!("Failed to link value with '{}'", e.identifier),
                        kind: GrammarErrorType::LinkerError,
                    })
                }
            }
            (
                ASN1Type::ElsewhereDeclaredType(e),
                ASN1Value::ElsewhereDeclaredValue { identifier, parent },
            ) => {
                if let Some(value) = Self::link_enum_or_distinguished(
                    tlds,
                    e,
                    identifier,
                    vec![e.identifier.clone()],
                )? {
                    *self = value;
                    return Ok(());
                } else if let Some((ToplevelDefinition::Type(ty), ToplevelDefinition::Value(val))) =
                    tlds.get(&e.identifier).zip(tlds.get(identifier))
                {
                    if ty.name != val.associated_type.as_str() {
                        // When it comes to `DEFAULT` values, the ASN.1 type system
                        // is more lenient than Rust's. For example, the it is acceptable
                        // to pass `int-value` as a `DEFAULT` value for `Int-Like-Type` in
                        // the following example:
                        // ```ignore
                        // int-value INTEGER ::= 600
                        // Int-Like-Type ::= INTEGER (1..605)
                        // Sequence-With-Defaults ::= SEQUENCE {
                        //     numeric Int-Like-Type DEFAULT int-value
                        // }
                        // ```
                        // Cases like these need to be explicitly cast in the rust bindings.
                        *self = val.clone().value;
                        self.link_with_type(
                            tlds,
                            &ASN1Type::ElsewhereDeclaredType(e.clone()),
                            None,
                        )?;
                        return Ok(());
                    }
                }
                *self = ASN1Value::LinkedElsewhereDefinedValue {
                    parent: parent.clone(),
                    identifier: identifier.clone(),
                    can_be_const: e.root(tlds)?.is_const_type(),
                };
                Ok(())
            }
            (ASN1Type::ElsewhereDeclaredType(e), val) => {
                *self = ASN1Value::LinkedNestedValue {
                    supertypes: vec![e.identifier.clone()],
                    value: Box::new((*val).clone()),
                };
                if let Some(ToplevelDefinition::Type(t)) = tlds.get(&e.identifier) {
                    self.link_with_type(tlds, &t.ty, Some(&t.name))
                } else {
                    Err(GrammarError {
                        details: format!("Failed to link value with '{}'", e.identifier),
                        kind: GrammarErrorType::LinkerError,
                    })
                }
            }
            (
                ASN1Type::Choice(c),
                ASN1Value::Choice {
                    type_name: tn,
                    variant_name,
                    inner_value,
                },
            ) => {
                if let Some(option) = c.options.iter().find(|o| &o.name == variant_name) {
                    *tn = type_name.cloned();
                    inner_value.link_with_type(
                        tlds,
                        &option.ty,
                        Some(&option.ty.as_str().into_owned()),
                    )
                } else {
                    Err(GrammarError {
                        details: format!("Failed to link value with '{}'", variant_name),
                        kind: GrammarErrorType::LinkerError,
                    })
                }
            }
            (ASN1Type::Choice(c), ASN1Value::LinkedNestedValue { supertypes, value })
                if matches![**value, ASN1Value::Choice { .. }] =>
            {
                let enum_name = supertypes.pop();
                if let ASN1Value::Choice {
                    type_name,
                    variant_name,
                    inner_value,
                } = &mut **value
                {
                    if let Some(option) = c.options.iter().find(|o| &o.name == variant_name) {
                        *type_name = enum_name;
                        inner_value.link_with_type(
                            tlds,
                            &option.ty,
                            Some(&option.ty.as_str().into_owned()),
                        )
                    } else {
                        Err(GrammarError {
                            details: format!("Failed to link value with '{}'", variant_name),
                            kind: GrammarErrorType::LinkerError,
                        })
                    }
                } else {
                    Ok(())
                }
            }
            (ASN1Type::Set(s), ASN1Value::SequenceOrSet(val))
            | (ASN1Type::Sequence(s), ASN1Value::SequenceOrSet(val)) => {
                *self = Self::link_struct_like(val, s, tlds, type_name)?;
                Ok(())
            }
            (ASN1Type::Set(s), ASN1Value::LinkedNestedValue { value, .. })
            | (ASN1Type::Sequence(s), ASN1Value::LinkedNestedValue { value, .. })
                if matches![**value, ASN1Value::SequenceOrSet(_)] =>
            {
                if let ASN1Value::SequenceOrSet(val) = &mut **value {
                    *value = Box::new(Self::link_struct_like(val, s, tlds, type_name)?);
                }
                Ok(())
            }
            (ASN1Type::SetOf(s), ASN1Value::SequenceOrSet(val))
            | (ASN1Type::SequenceOf(s), ASN1Value::SequenceOrSet(val)) => {
                *self = Self::link_array_like(val, s, tlds)?;
                Ok(())
            }
            (ASN1Type::SetOf(s), ASN1Value::LinkedNestedValue { value, .. })
            | (ASN1Type::SequenceOf(s), ASN1Value::LinkedNestedValue { value, .. })
                if matches![**value, ASN1Value::SequenceOrSet(_)] =>
            {
                if let ASN1Value::SequenceOrSet(val) = &mut **value {
                    *value = Box::new(Self::link_array_like(val, s, tlds)?);
                }
                Ok(())
            }
            (ASN1Type::Integer(i), ASN1Value::Integer(val)) => {
                *self = ASN1Value::LinkedIntValue {
                    integer_type: i.int_type(),
                    value: *val,
                };
                Ok(())
            }
            (ASN1Type::CharacterString(t), ASN1Value::String(s)) => {
                *self = ASN1Value::LinkedCharStringValue(t.ty, s.clone());
                Ok(())
            }
            (ASN1Type::CharacterString(t), ASN1Value::LinkedNestedValue { value, .. })
                if matches![**value, ASN1Value::String(_)] =>
            {
                if let ASN1Value::String(s) = &**value {
                    *value = Box::new(ASN1Value::LinkedCharStringValue(t.ty, s.clone()));
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
            (ASN1Type::Integer(i), ASN1Value::LinkedNestedValue { value, .. })
                if matches![**value, ASN1Value::Integer(_)] =>
            {
                if let ASN1Value::Integer(v) = &**value {
                    let int_type = i.constraints.iter().fold(IntegerType::Unbounded, |acc, c| {
                        c.integer_constraints().max_restrictive(acc)
                    });
                    *value = Box::new(ASN1Value::LinkedIntValue {
                        integer_type: int_type,
                        value: *v,
                    });
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
            (
                _,
                ASN1Value::ElsewhereDeclaredValue {
                    parent: None,
                    identifier,
                },
            ) => {
                if let Some(ToplevelDefinition::Value(tld)) = tlds.get(identifier) {
                    *self = tld.value.clone();
                    self.link_with_type(tlds, ty, type_name)?;
                }
                Ok(())
            }
            (_, ASN1Value::ElsewhereDeclaredValue { .. }) => todo!(),
            _ => Ok(()),
        }
    }

    fn link_enum_or_distinguished(
        tlds: &BTreeMap<String, ToplevelDefinition>,
        e: &DeclarationElsewhere,
        identifier: &mut String,
        mut supertypes: Vec<String>,
    ) -> Result<Option<ASN1Value>, GrammarError> {
        match tlds.get(&e.identifier) {
            Some(ToplevelDefinition::Type(ToplevelTypeDefinition {
                ty: ASN1Type::Enumerated(enumerated),
                ..
            })) => {
                if enumerated
                    .members
                    .iter()
                    .any(|enumeral| &enumeral.name == identifier)
                {
                    Ok(Some(ASN1Value::EnumeratedValue {
                        enumerated: e.identifier.clone(),
                        enumerable: identifier.clone(),
                    }))
                } else {
                    Ok(None)
                }
            }
            Some(ToplevelDefinition::Type(ToplevelTypeDefinition {
                ty:
                    ASN1Type::Integer(Integer {
                        distinguished_values: Some(distinguished),
                        constraints,
                    }),
                ..
            })) => {
                if let Some(distinguished_value) =
                    distinguished.iter().find(|d| &d.name == identifier)
                {
                    Ok(Some(ASN1Value::LinkedNestedValue {
                        supertypes,
                        value: Box::new(ASN1Value::LinkedIntValue {
                            integer_type: constraints
                                .iter()
                                .fold(IntegerType::Unbounded, |acc, c| {
                                    c.integer_constraints().max_restrictive(acc)
                                }),
                            value: distinguished_value.value,
                        }),
                    }))
                } else {
                    Ok(None)
                }
            }
            Some(ToplevelDefinition::Type(ToplevelTypeDefinition {
                ty: ASN1Type::ElsewhereDeclaredType(elsewhere),
                ..
            })) => {
                supertypes.push(elsewhere.identifier.clone());
                Self::link_enum_or_distinguished(tlds, elsewhere, identifier, supertypes)
            }
            _ => Ok(None),
        }
    }

    fn link_array_like(
        val: &mut [(Option<String>, Box<ASN1Value>)],
        s: &SequenceOrSetOf,
        tlds: &BTreeMap<String, ToplevelDefinition>,
    ) -> Result<ASN1Value, GrammarError> {
        let _ = val.iter_mut().try_for_each(|v| {
            v.1.link_with_type(
                tlds,
                &s.element_type,
                Some(&s.element_type.as_str().into_owned()),
            )
        });
        Ok(ASN1Value::LinkedArrayLikeValue(
            val.iter().map(|v| v.1.clone()).collect(),
        ))
    }

    fn link_struct_like(
        val: &mut [(Option<String>, Box<ASN1Value>)],
        s: &SequenceOrSet,
        tlds: &BTreeMap<String, ToplevelDefinition>,
        type_name: Option<&String>,
    ) -> Result<ASN1Value, GrammarError> {
        val.iter_mut().try_for_each(|v| {
            if let Some(member) = s.members.iter().find(|m| Some(&m.name) == v.0.as_ref()) {
                let type_name = match (member.ty.is_builtin_type(), type_name) {
                    (true, Some(parent)) => Some(
                        INTERNAL_NESTED_TYPE_NAME_PREFIX.to_owned() + &member.name + "$" + parent,
                    ),
                    (false, _) => Some(member.ty.as_str().into_owned()),
                    _ => {
                        return Err(GrammarError {
                            details: format!(
                                "Failed to determine parent name of field {}",
                                member.name
                            ),
                            kind: GrammarErrorType::LinkerError,
                        })
                    }
                };
                v.1.link_with_type(tlds, &member.ty, type_name.as_ref())
            } else {
                Err(GrammarError {
                    details: format!("Failed to link value with '{:?}'", v.0),
                    kind: GrammarErrorType::LinkerError,
                })
            }
        })?;

        s.members
            .iter()
            .map(|member| {
                val.iter()
                    .find_map(|(name, value)| {
                        (name.as_ref() == Some(&member.name))
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
                    .map(|field_value| (member.name.clone(), member.ty.clone(), field_value))
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
        tlds: &BTreeMap<String, ToplevelDefinition>,
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
                                (Some(a), Some(b), _, _) if a.matches(b, &tokens, i) => {
                                    *self = val.clone();
                                    return Ok(());
                                }
                                (_, _, Some(c), Some(d)) if c.matches(d, &tokens, i) => {
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
        tlds: &BTreeMap<String, ToplevelDefinition>,
    ) -> Result<(), GrammarError> {
        match self {
            Self::ElsewhereDeclaredValue {
                parent: Some(_), ..
            } => {
                return self.resolve_elsewhere_with_parent(tlds);
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
                }
            }
            _ => {}
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use crate::intermediate::{types::*, *};

    macro_rules! tld {
        ($name:literal, $ty:expr) => {
            ToplevelTypeDefinition {
                comments: String::new(),
                tag: None,
                index: None,
                name: $name.into(),
                ty: $ty,
                parameterization: None,
            }
        };
    }

    #[test]
    fn links_asn1_value() {
        let tlds: BTreeMap<String, ToplevelDefinition> = {
            let mut map = BTreeMap::new();
            map.insert(
                "RootBool".into(),
                ToplevelDefinition::Type(tld!(
                    "RootBool",
                    ASN1Type::Boolean(Boolean {
                        constraints: vec![]
                    })
                )),
            );
            map.insert(
                "IntermediateBool".into(),
                ToplevelDefinition::Type(tld!(
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
                ToplevelDefinition::Type(tld!(
                    "BaseChoice",
                    ASN1Type::Choice(Choice {
                        extensible: None,
                        constraints: vec![],
                        options: vec![ChoiceOption {
                            name: String::from("first"),
                            constraints: vec![],
                            tag: None,
                            ty: ASN1Type::ElsewhereDeclaredType(DeclarationElsewhere {
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
        let mut example_value = ToplevelValueDefinition {
            comments: String::new(),
            name: "exampleValue".into(),
            parameterization: None,
            associated_type: ASN1Type::ElsewhereDeclaredType(DeclarationElsewhere {
                parent: None,
                identifier: "BaseChoice".into(),
                constraints: vec![],
            }),
            index: None,
            value: ASN1Value::Choice {
                type_name: None,
                variant_name: "first".into(),
                inner_value: Box::new(ASN1Value::Boolean(true)),
            },
        };
        example_value.collect_supertypes(&tlds).unwrap();
        assert_eq!(
            example_value,
            ToplevelValueDefinition {
                comments: "".into(),
                name: "exampleValue".into(),
                associated_type: ASN1Type::ElsewhereDeclaredType(DeclarationElsewhere {
                    parent: None,
                    identifier: "BaseChoice".into(),
                    constraints: vec![]
                }),
                parameterization: None,
                value: ASN1Value::Choice {
                    type_name: Some("BaseChoice".into()),
                    variant_name: "first".into(),
                    inner_value: Box::new(ASN1Value::LinkedNestedValue {
                        supertypes: vec!["IntermediateBool".into(), "RootBool".into()],
                        value: Box::new(ASN1Value::Boolean(true))
                    })
                },
                index: None
            }
        )
    }
}
