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
    ops::Not,
    rc::Rc,
};

use crate::{
    error::CompilerError,
    intermediate::{
        constraints::*,
        information_object::{ClassLink, ToplevelInformationDefinition},
        types::*,
        *,
    },
};

use self::{
    error::{LinkerError, LinkerErrorType},
    information_object::{ASN1Information, InformationObjectClassField},
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

    fn link(mut self) -> Result<(Self, Vec<CompilerError>), LinkerError> {
        let mut warnings: Vec<CompilerError> = vec![];
        // Linking of ASN1 values depends on linked ASN1 types, so we order the key collection accordingly (note that we pop keys)
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
                Some(ToplevelDefinition::Object(ToplevelInformationDefinition {
                    value: ASN1Information::ObjectSet(_),
                    ..
                }))
            ] {
                let mut item = self.tlds.remove_entry(&key);
                if let Some((
                    _,
                    ToplevelDefinition::Object(ToplevelInformationDefinition {
                        value: ASN1Information::ObjectSet(set),
                        ..
                    }),
                )) = &mut item
                {
                    if let Err(mut e) = set.resolve_object_set_references(&self.tlds) {
                        e.contextualize(&key);
                        warnings.push(e.into())
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
                    Some((k, ToplevelDefinition::Object(mut tld))) => {
                        tld = tld.resolve_class_reference(&self.tlds);
                        self.tlds.insert(k, ToplevelDefinition::Object(tld));
                    }
                    _ => (),
                }
            }
            if self.has_components_of_notation(&key) {
                if let Some((k, ToplevelDefinition::Type(mut tld))) = self.tlds.remove_entry(&key) {
                    tld.ty.link_components_of_notation(&self.tlds);
                    self.tlds.insert(k, ToplevelDefinition::Type(tld));
                }
            }
            if self.has_choice_selection_type(&key) {
                if let Some((k, ToplevelDefinition::Type(mut tld))) = self.tlds.remove_entry(&key) {
                    if let Err(mut e) = tld.ty.link_choice_selection_type(&self.tlds) {
                        e.contextualize(&key);
                        warnings.push(e.into());
                    }
                    self.tlds.insert(k, ToplevelDefinition::Type(tld));
                }
            }
            if self.references_object_set_by_name(&key) {
                if let Some((k, ToplevelDefinition::Object(mut tld))) = self.tlds.remove_entry(&key)
                {
                    tld.value.link_object_set_reference(&self.tlds);
                    self.tlds.insert(k, ToplevelDefinition::Object(tld));
                }
            }
            if self.has_constraint_reference(&key) {
                match self.tlds.remove(&key).ok_or_else(|| LinkerError {
                    pdu: Some(key.clone()),
                    details: "Could not find toplevel declaration to remove!".into(),
                    kind: LinkerErrorType::MissingDependency,
                }) {
                    Ok(mut tld) => {
                        if let Err(mut e) = tld.link_constraint_reference(&self.tlds) {
                            e.contextualize(&key);
                            warnings.push(e.into());
                        }
                        self.tlds.insert(tld.name().clone(), tld);
                    }
                    Err(mut e) => {
                        e.contextualize(&key);
                        warnings.push(e.into());
                    }
                };
            }
            if let Some((k, mut tld)) = self.tlds.remove_entry(&key) {
                if let Err(mut e) = tld.collect_supertypes(&self.tlds) {
                    e.contextualize(&key);
                    warnings.push(e.into());
                }
                self.tlds.insert(k, tld);
            }
            if let Some((k, mut tld)) = self.tlds.remove_entry(&key) {
                if let Err(mut e) = tld.mark_recursive(&self.tlds) {
                    e.contextualize(&key);
                    warnings.push(e.into());
                }
                self.tlds.insert(k, tld);
            }
            if self
                .tlds
                .get(&key)
                .and_then(ToplevelDefinition::get_module_header)
                .is_some_and(|m| visited_headers.contains(&m.borrow().name).not())
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
            let module_header = tld.get_module_header().unwrap();

            let mut associated_type_imports = Vec::new();
            if let ToplevelDefinition::Object(ToplevelInformationDefinition {
                class: ClassLink::ByReference(ref class_ref),
                ..
            }) = tld
            {
                for field in &class_ref.fields {
                    self.associated_import_type_class_field(
                        field,
                        module_header.clone(),
                        &mut associated_type_imports,
                    );
                }
            }
            for import_modules in &module_header.borrow().imports {
                for import in &import_modules.types {
                    if import.starts_with(|c: char| c.is_lowercase()) {
                        match self.tlds.get(import) {
                            Some(ToplevelDefinition::Object(ToplevelInformationDefinition {
                                class: ClassLink::ByReference(class_ref),
                                ..
                            })) => {
                                for field in &class_ref.fields {
                                    self.associated_import_type_class_field(
                                        field,
                                        module_header.clone(),
                                        &mut associated_type_imports,
                                    );
                                }
                            }
                            Some(ToplevelDefinition::Value(ToplevelValueDefinition {
                                associated_type,
                                ..
                            })) => {
                                self.associated_import_type(
                                    associated_type.as_str().as_ref(),
                                    module_header.clone(),
                                    &mut associated_type_imports,
                                );
                            }
                            _ => (),
                        }
                    }
                }
            }
            for mut import in associated_type_imports {
                let mut mut_module_header = module_header.borrow_mut();
                if let Some(mod_imports) = mut_module_header
                    .imports
                    .iter_mut()
                    .find(|i| i.global_module_reference == import.global_module_reference)
                {
                    if mod_imports.types.contains(&import.types[0]).not() {
                        mod_imports.types.push(std::mem::take(&mut import.types[0]));
                    }
                } else {
                    mut_module_header.imports.push(import);
                }
            }

            visited_headers.insert(module_header.borrow().name.clone());
        }
        self.tlds.insert(key, tld);
    }

    fn associated_import_type(
        &self,
        associated_type: &str,
        module_header: Rc<RefCell<ModuleHeader>>,
        associated_type_imports: &mut Vec<Import>,
    ) {
        if let Some(ToplevelDefinition::Type(ToplevelTypeDefinition {
            name,
            parameterization,
            index: Some((m_hdr, _)),
            ..
        })) = self.tlds.get(associated_type)
        {
            let v_type_name = format!("{}{}", name, parameterization.as_ref().map_or("", |_| "{}"));
            let v_type_mod_name = &m_hdr.borrow().name;
            if v_type_mod_name != &module_header.borrow().name
                && module_header.borrow().find_import(&v_type_name).is_none()
            {
                associated_type_imports.push(Import {
                    types: vec![v_type_name],
                    global_module_reference: GlobalModuleReference {
                        module_reference: m_hdr.borrow().name.clone(),
                        assigned_identifier: match &m_hdr.borrow().module_identifier {
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
        module_header: Rc<RefCell<ModuleHeader>>,
        associated_type_imports: &mut Vec<Import>,
    ) {
        if let Some(ASN1Type::ElsewhereDeclaredType(DeclarationElsewhere {
            parent: Some(class_id),
            identifier,
            ..
        })) = &field.ty
        {
            if let Some(ToplevelDefinition::Class(class)) = self.tlds.get(class_id) {
                if let Some(field) = class
                    .definition
                    .fields
                    .iter()
                    .find(|f| f.identifier.identifier() == identifier)
                {
                    self.associated_import_type_class_field(
                        field,
                        module_header,
                        associated_type_imports,
                    );
                }
            }
        } else if let Some(ASN1Type::ElsewhereDeclaredType(DeclarationElsewhere {
            identifier,
            ..
        })) = &field.ty
        {
            self.associated_import_type(identifier, module_header, associated_type_imports)
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
                ToplevelDefinition::Object(i) => match i.class {
                    ClassLink::ByReference(_) => false,
                    ClassLink::ByName(_) => true,
                },
                _ => false,
            })
            .unwrap_or(false)
    }

    fn references_object_set_by_name(&mut self, key: &String) -> bool {
        self.tlds
            .get(key)
            .map(|t| match t {
                ToplevelDefinition::Object(ToplevelInformationDefinition { value, .. }) => {
                    value.references_object_set_by_name()
                }
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
    ) -> Result<(Vec<ToplevelDefinition>, Vec<CompilerError>), CompilerError> {
        let warnings: Vec<CompilerError>;
        (self, warnings) = self.link()?;
        Ok(self.tlds.into_iter().fold(
            (Vec::<ToplevelDefinition>::new(), warnings),
            |(mut tlds, mut errors), (_, tld)| {
                match tld.validate() {
                    Ok(_) => tlds.push(tld),
                    Err(e) => errors.push(e.into()),
                }
                (tlds, errors)
            },
        ))
    }
}

pub trait Validate {
    fn validate(&self) -> Result<(), LinkerError>;
}

impl Validate for ToplevelDefinition {
    fn validate(&self) -> Result<(), LinkerError> {
        match self {
            ToplevelDefinition::Type(t) => {
                if let Err(mut e) = t.ty.validate() {
                    e.contextualize(&t.name);
                    return Err(e);
                }
                Ok(())
            }
            ToplevelDefinition::Value(_v) => Ok(()),
            ToplevelDefinition::Class(_c) => Ok(()),
            ToplevelDefinition::Object(_o) => Ok(()),
            ToplevelDefinition::Macro(_m) => Ok(()),
        }
    }
}

impl Validate for ASN1Type {
    fn validate(&self) -> Result<(), LinkerError> {
        match self {
            ASN1Type::Integer(ref i) => i.validate(),
            ASN1Type::BitString(ref b) => b.validate(),
            ASN1Type::CharacterString(ref o) => o.validate(),
            _ => Ok(()),
        }
    }
}

impl Validate for Integer {
    fn validate(&self) -> Result<(), LinkerError> {
        for c in &self.constraints {
            c.validate()?;
        }
        Ok(())
    }
}

impl Validate for BitString {
    fn validate(&self) -> Result<(), LinkerError> {
        for c in &self.constraints {
            c.validate()?;
        }
        Ok(())
    }
}

impl Validate for CharacterString {
    fn validate(&self) -> Result<(), LinkerError> {
        for c in &self.constraints {
            c.validate()?;
        }
        Ok(())
    }
}

impl Validate for Constraint {
    fn validate(&self) -> Result<(), LinkerError> {
        if let Constraint::Subtype(c) = self {
            if let ElementOrSetOperation::Element(SubtypeElements::ValueRange {
                min,
                max,
                extensible: _,
            }) = &c.set
            {
                if let Some((ASN1Value::Integer(min), ASN1Value::Integer(max))) =
                    min.as_ref().zip(max.as_ref())
                {
                    if min > max {
                        return Err(LinkerError::new(
                            None,
                            "Mininum value exceeds maximum value!",
                            LinkerErrorType::InvalidConstraintsError,
                        ));
                    }
                }
            }
        }
        Ok(())
    }
}
