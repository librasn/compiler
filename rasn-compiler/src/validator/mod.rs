//! The `validator` module ensures that the
//! parsed data elements of the ASN1 specification
//! are valid declarations that can be further
//! processed. Among other things, the `validator`
//! assures that all dependencies of the individual
//! data elements resolve, and checks for conflicting
//! constraints and value definitions.
pub(crate) mod error;
mod linking;
#[cfg(test)]
mod tests;

use std::{
    cell::RefCell,
    collections::{BTreeMap, HashSet},
    error::Error,
    ops::Not,
    rc::Rc,
};

use crate::intermediate::{
    constraints::*,
    information_object::{ClassLink, ToplevelInformationDefinition},
    types::*,
    *,
};

use self::{
    error::{ValidatorError, ValidatorErrorType},
    information_object::{
        ASN1Information, InformationObjectClass, InformationObjectClassField, ObjectSet,
    },
};

pub struct Validator {
    tlds: BTreeMap<String, ToplevelDefinition>,
}

impl Validator {
    pub fn new(tlds: Vec<ToplevelDefinition>) -> Validator {
        Self {
            tlds: tlds
                .into_iter()
                .map(|tld| (tld.name().to_owned(), tld))
                .collect(),
        }
    }

    fn link(mut self) -> Result<(Self, Vec<Box<dyn Error>>), ValidatorError> {
        let mut warnings: Vec<Box<dyn Error>> = vec![];
        // Linking of ASN1 values depends on linked ASN1 types, so we order the key colelction accordingly (note that we pop keys)
        let mut keys = self
            .tlds
            .iter()
            .filter_map(|(k, v)| matches![v, ToplevelDefinition::Value(_)].then_some(k.clone()))
            .chain(self.tlds.iter().filter_map(|(k, v)| {
                matches![v, ToplevelDefinition::Value(_)]
                    .not()
                    .then_some(k.clone())
            }))
            .collect::<Vec<String>>();
        let mut visited_headers = HashSet::<String>::new();
        while let Some(key) = keys.pop() {
            if matches![
                self.tlds.get(&key),
                Some(ToplevelDefinition::Information(
                    ToplevelInformationDefinition {
                        value: ASN1Information::ObjectSet(ObjectSet { .. }),
                        ..
                    }
                ))
            ] {
                let mut item = self.tlds.remove_entry(&key);
                if let Some((
                    _,
                    ToplevelDefinition::Information(ToplevelInformationDefinition {
                        value: ASN1Information::ObjectSet(set),
                        ..
                    }),
                )) = &mut item
                {
                    if let Err(e) = set.resolve_object_set_references(&self.tlds) {
                        warnings.push(Box::new(e))
                    }
                }
                if let Some((k, tld)) = item {
                    self.tlds.insert(k, tld);
                }
            }
            if self.references_class_by_name(&key) {
                match self.tlds.remove_entry(&key) {
                    Some((k, ToplevelDefinition::Type(mut tld))) => {
                        tld.ty = tld.ty.resolve_class_reference(&self.tlds);
                        self.tlds.insert(k, ToplevelDefinition::Type(tld));
                    }
                    Some((k, ToplevelDefinition::Information(mut tld))) => {
                        tld = tld.resolve_class_reference(&self.tlds);
                        self.tlds.insert(k, ToplevelDefinition::Information(tld));
                    }
                    _ => (),
                }
            }
            // if self.is_parameterized(&key) {
            //     if let Some((k, mut tld)) = self.tlds.remove_entry(&key) {
            //         if let Err(e) = tld.resolve_parameterization(&self.tlds) {
            //             warnings.push(Box::new(e));
            //         }
            //         self.tlds.insert(k, tld);
            //     }
            // }
            if self.has_components_of_notation(&key) {
                if let Some((k, ToplevelDefinition::Type(mut tld))) = self.tlds.remove_entry(&key) {
                    tld.ty.link_components_of_notation(&self.tlds);
                    self.tlds.insert(k, ToplevelDefinition::Type(tld));
                }
            }
            if self.has_choice_selection_type(&key) {
                if let Some((k, ToplevelDefinition::Type(mut tld))) = self.tlds.remove_entry(&key) {
                    if let Err(e) = tld.ty.link_choice_selection_type(&self.tlds) {
                        warnings.push(Box::new(e));
                    }
                    self.tlds.insert(k, ToplevelDefinition::Type(tld));
                }
            }
            if self.references_object_set_by_name(&key) {
                if let Some((k, ToplevelDefinition::Information(mut tld))) =
                    self.tlds.remove_entry(&key)
                {
                    tld.value.link_object_set_reference(&self.tlds);
                    self.tlds.insert(k, ToplevelDefinition::Information(tld));
                }
            }
            if self.has_constraint_reference(&key) {
                match self.tlds.remove(&key).ok_or_else(|| ValidatorError {
                    data_element: Some(key.clone()),
                    details: "Could not find toplevel declaration to remove!".into(),
                    kind: ValidatorErrorType::MissingDependency,
                }) {
                    Ok(mut tld) => {
                        if let Err(e) = tld.link_constraint_reference(&self.tlds) {
                            warnings.push(Box::new(e));
                        }
                        self.tlds.insert(tld.name().clone(), tld);
                    }
                    Err(e) => {
                        warnings.push(Box::new(e));
                    }
                };
            }
            if let Some((k, mut tld)) = self.tlds.remove_entry(&key) {
                if let Err(e) = tld.collect_supertypes(&self.tlds) {
                    warnings.push(Box::new(e));
                }
                self.tlds.insert(k, tld);
            }
            if self
                .tlds
                .get(&key)
                .and_then(ToplevelDefinition::get_module_reference)
                .map_or(false, |m| visited_headers.contains(&m.borrow().name).not())
            {
                self.fill_in_associated_type_imports(key, &mut visited_headers);
            }
        }

        Ok((self, warnings))
    }

    fn fill_in_associated_type_imports(
        &mut self,
        key: String,
        visited_headers: &mut HashSet<String>,
    ) {
        let tld = self.tlds.remove(&key).unwrap();
        {
            let mod_ref = tld.get_module_reference().unwrap();

            let mut associated_type_imports = Vec::new();
            if let ToplevelDefinition::Information(ToplevelInformationDefinition {
                class: Some(ClassLink::ByReference(ref class_ref)),
                ..
            }) = tld
            {
                for field in &class_ref.fields {
                    self.associated_import_type_class_field(
                        field,
                        mod_ref.clone(),
                        &mut associated_type_imports,
                    );
                }
            }
            for import_modules in &mod_ref.borrow().imports {
                for import in &import_modules.types {
                    if import.starts_with(|c: char| c.is_lowercase()) {
                        match self.tlds.get(import) {
                            Some(ToplevelDefinition::Information(
                                ToplevelInformationDefinition {
                                    class: Some(ClassLink::ByReference(class_ref)),
                                    ..
                                },
                            )) => {
                                for field in &class_ref.fields {
                                    self.associated_import_type_class_field(
                                        field,
                                        mod_ref.clone(),
                                        &mut associated_type_imports,
                                    );
                                }
                            }
                            Some(ToplevelDefinition::Value(ToplevelValueDefinition {
                                associated_type,
                                ..
                            })) => {
                                self.associated_import_type(
                                    &associated_type.as_str().into_owned(),
                                    mod_ref.clone(),
                                    &mut associated_type_imports,
                                );
                            }
                            _ => (),
                        }
                    }
                }
            }
            for mut import in associated_type_imports {
                let mut mut_mod_ref = mod_ref.borrow_mut();
                if let Some(mod_imports) = mut_mod_ref
                    .imports
                    .iter_mut()
                    .find(|i| i.global_module_reference == import.global_module_reference)
                {
                    if mod_imports.types.contains(&import.types[0]).not() {
                        mod_imports.types.push(std::mem::take(&mut import.types[0]));
                    }
                } else {
                    mut_mod_ref.imports.push(import);
                }
            }

            visited_headers.insert(mod_ref.borrow().name.clone());
        }
        self.tlds.insert(key, tld);
    }

    fn associated_import_type(
        &self,
        associated_type: &String,
        mod_ref: Rc<RefCell<ModuleReference>>,
        associated_type_imports: &mut Vec<Import>,
    ) {
        if let Some(ToplevelDefinition::Type(ToplevelTypeDefinition {
            name,
            parameterization,
            index: Some((m_ref, _)),
            ..
        })) = self.tlds.get(associated_type)
        {
            let v_type_name = format!("{}{}", name, parameterization.as_ref().map_or("", |_| "{}"));
            let v_type_mod_name = &m_ref.borrow().name;
            if v_type_mod_name != &mod_ref.borrow().name
                && mod_ref.borrow().find_import(&v_type_name).is_none()
            {
                associated_type_imports.push(Import {
                    types: vec![v_type_name],
                    global_module_reference: GlobalModuleReference {
                        module_reference: m_ref.borrow().name.clone(),
                        assigned_identifier: match &m_ref.borrow().module_identifier {
                            Some(DefinitiveIdentifier::DefinitiveOID(oid))
                            | Some(DefinitiveIdentifier::DefinitiveOIDandIRI { oid, .. }) => {
                                AssignedIdentifier::ObjectIdentifierValue(oid.clone())
                            }
                            None => AssignedIdentifier::Empty,
                        },
                    },
                    with: None,
                })
            }
        }
    }

    fn associated_import_type_class_field(
        &self,
        field: &InformationObjectClassField,
        mod_ref: Rc<RefCell<ModuleReference>>,
        associated_type_imports: &mut Vec<Import>,
    ) {
        if let Some(ASN1Type::ElsewhereDeclaredType(DeclarationElsewhere {
            parent: Some(class_id),
            identifier,
            ..
        })) = &field.ty
        {
            if let Some(ToplevelDefinition::Information(ToplevelInformationDefinition {
                value: ASN1Information::ObjectClass(InformationObjectClass { fields, .. }),
                ..
            })) = self.tlds.get(class_id)
            {
                if let Some(field) = fields
                    .iter()
                    .find(|f| f.identifier.identifier() == identifier)
                {
                    self.associated_import_type_class_field(
                        field,
                        mod_ref,
                        associated_type_imports,
                    );
                }
            }
        } else if let Some(ASN1Type::ElsewhereDeclaredType(DeclarationElsewhere {
            identifier,
            ..
        })) = &field.ty
        {
            self.associated_import_type(identifier, mod_ref, associated_type_imports)
        }
    }

    fn has_constraint_reference(&mut self, key: &String) -> bool {
        if let Some(tld) = self.tlds.get(key) {
            tld.is_parameterized() || tld.has_constraint_reference()
        } else {
            false
        }
    }

    fn references_class_by_name(&mut self, key: &String) -> bool {
        self.tlds
            .get(key)
            .map(|t| match t {
                ToplevelDefinition::Type(t) => t.ty.references_class_by_name(),
                ToplevelDefinition::Information(i) => i.class.as_ref().map_or(false, |c| match c {
                    ClassLink::ByReference(_) => false,
                    ClassLink::ByName(_) => true,
                }),
                _ => false,
            })
            .unwrap_or(false)
    }

    fn references_object_set_by_name(&mut self, key: &String) -> bool {
        self.tlds
            .get(key)
            .map(|t| match t {
                ToplevelDefinition::Information(ToplevelInformationDefinition {
                    value, ..
                }) => value.references_object_set_by_name(),
                _ => false,
            })
            .unwrap_or(false)
    }

    fn has_choice_selection_type(&mut self, key: &String) -> bool {
        self.tlds
            .get(key)
            .map(|t| match t {
                ToplevelDefinition::Type(t) => t.ty.has_choice_selection_type(),
                _ => false,
            })
            .unwrap_or(false)
    }

    fn has_components_of_notation(&mut self, key: &String) -> bool {
        self.tlds
            .get(key)
            .map(|t| match t {
                ToplevelDefinition::Type(t) => t.ty.contains_components_of_notation(),
                _ => false,
            })
            .unwrap_or(false)
    }

    pub fn validate(
        mut self,
    ) -> Result<(Vec<ToplevelDefinition>, Vec<Box<dyn Error>>), Box<dyn Error>> {
        let warnings: Vec<Box<dyn Error>>;
        (self, warnings) = self.link()?;
        Ok(self.tlds.into_iter().fold(
            (Vec::<ToplevelDefinition>::new(), warnings),
            |(mut tlds, mut errors), (_, tld)| {
                match tld.validate() {
                    Ok(_) => tlds.push(tld),
                    Err(e) => errors.push(Box::new(e)),
                }
                (tlds, errors)
            },
        ))
    }
}

pub trait Validate {
    fn validate(&self) -> Result<(), ValidatorError>;
}

impl Validate for ToplevelDefinition {
    fn validate(&self) -> Result<(), ValidatorError> {
        match self {
            ToplevelDefinition::Type(t) => {
                if let Err(mut e) = t.ty.validate() {
                    e.specify_data_element(t.name.clone());
                    return Err(e);
                }
                Ok(())
            }
            ToplevelDefinition::Value(_v) => Ok(()),
            ToplevelDefinition::Information(_i) => Ok(()),
        }
    }
}

impl Validate for ASN1Type {
    fn validate(&self) -> Result<(), ValidatorError> {
        match self {
            ASN1Type::Integer(ref i) => i.validate(),
            ASN1Type::BitString(ref b) => b.validate(),
            ASN1Type::CharacterString(ref o) => o.validate(),
            _ => Ok(()),
        }
    }
}

impl Validate for Integer {
    fn validate(&self) -> Result<(), ValidatorError> {
        for c in &self.constraints {
            c.validate()?;
        }
        Ok(())
    }
}

impl Validate for BitString {
    fn validate(&self) -> Result<(), ValidatorError> {
        for c in &self.constraints {
            c.validate()?;
        }
        Ok(())
    }
}

impl Validate for CharacterString {
    fn validate(&self) -> Result<(), ValidatorError> {
        for c in &self.constraints {
            c.validate()?;
        }
        Ok(())
    }
}

impl Validate for Constraint {
    fn validate(&self) -> Result<(), ValidatorError> {
        if let Constraint::SubtypeConstraint(c) = self {
            if let ElementOrSetOperation::Element(SubtypeElement::ValueRange {
                min,
                max,
                extensible: _,
            }) = &c.set
            {
                if let Some((ASN1Value::Integer(min), ASN1Value::Integer(max))) =
                    min.as_ref().zip(max.as_ref())
                {
                    if min > max {
                        return Err(ValidatorError::new(
                            None,
                            "Mininum value exceeds maximum value!",
                            ValidatorErrorType::InvalidConstraintsError,
                        ));
                    }
                }
            }
        }
        Ok(())
    }
}
