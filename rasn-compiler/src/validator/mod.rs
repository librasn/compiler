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
    collections::{HashMap, HashSet},
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
    tlds: HashMap<String, ToplevelDefinition>,
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
        let mut keys: Vec<_> = self.tlds.values().collect();
        // Linking of ASN1 values depends on linked ASN1 types, so we order the key collection
        // accordingly (note that we pop keys).
        keys.sort_unstable_by_key(|tld| (!matches!(tld, ToplevelDefinition::Value(_)), tld.name()));
        let mut keys: Vec<_> = keys.into_iter().map(|tld| tld.name().to_owned()).collect();
        let mut visited_headers = HashSet::<String>::new();
        while let Some(key) = keys.pop() {
            let Some((name, mut tld)) = self.tlds.remove_entry(&key) else {
                continue;
            };
            if let ToplevelDefinition::Object(ToplevelInformationDefinition {
                value: ASN1Information::ObjectSet(set),
                ..
            }) = &mut tld
            {
                if let Err(mut e) = set.resolve_object_set_references(&self.tlds) {
                    e.contextualize(&key);
                    warnings.push(e.into())
                }
            }
            if self.references_class_by_name(&tld) {
                tld = match tld {
                    ToplevelDefinition::Type(mut tld) => {
                        tld.ty = tld.ty.resolve_class_reference(&self.tlds);
                        ToplevelDefinition::Type(tld)
                    }
                    ToplevelDefinition::Object(mut tld) => {
                        tld = tld.resolve_class_reference(&self.tlds);
                        ToplevelDefinition::Object(tld)
                    }
                    _ => tld,
                }
            }
            if self.has_components_of_notation(&tld) {
                if let ToplevelDefinition::Type(tld) = &mut tld {
                    tld.ty.link_components_of_notation(&self.tlds);
                }
            }
            if self.has_choice_selection_type(&tld) {
                if let ToplevelDefinition::Type(tld) = &mut tld {
                    if let Err(mut e) = tld.ty.link_choice_selection_type(&self.tlds) {
                        e.contextualize(&key);
                        warnings.push(e.into());
                    }
                }
            }
            if self.references_object_set_by_name(&tld) {
                if let ToplevelDefinition::Object(tld) = &mut tld {
                    tld.value.link_object_set_reference(&self.tlds);
                }
            }
            if self.has_constraint_reference(&tld) {
                if let Err(mut e) = tld.link_constraint_reference(&self.tlds) {
                    e.contextualize(&key);
                    warnings.push(e.into());
                }
            }
            if let Err(mut e) = tld.collect_supertypes(&self.tlds) {
                e.contextualize(&key);
                warnings.push(e.into());
            }
            if let Err(mut e) = tld.mark_recursive(&self.tlds) {
                e.contextualize(&key);
                warnings.push(e.into());
            }
            if let Some(header) = tld.get_module_header() {
                if !visited_headers.contains(&header.borrow().name) {
                    self.fill_in_associated_type_imports(&mut tld, &header, &mut visited_headers);
                }
            }
            self.tlds.insert(name, tld);
        }

        Ok((self, warnings))
    }

    fn fill_in_associated_type_imports(
        &mut self,
        tld: &mut ToplevelDefinition,
        module_header: &Rc<RefCell<ModuleHeader>>,
        visited_headers: &mut HashSet<String>,
    ) {
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

    fn associated_import_type(
        &self,
        associated_type: &str,
        module_header: Rc<RefCell<ModuleHeader>>,
        associated_type_imports: &mut Vec<Import>,
    ) {
        if let Some(ToplevelDefinition::Type(ToplevelTypeDefinition {
            name,
            parameterization,
            module_header: Some(m_hdr),
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

    fn has_constraint_reference(&mut self, tld: &ToplevelDefinition) -> bool {
        tld.is_parameterized() || tld.has_constraint_reference()
    }

    fn references_class_by_name(&mut self, tld: &ToplevelDefinition) -> bool {
        match tld {
            ToplevelDefinition::Type(t) => t.ty.references_class_by_name(),
            ToplevelDefinition::Object(i) => match i.class {
                ClassLink::ByReference(_) => false,
                ClassLink::ByName(_) => true,
            },
            _ => false,
        }
    }

    fn references_object_set_by_name(&mut self, tld: &ToplevelDefinition) -> bool {
        match tld {
            ToplevelDefinition::Object(ToplevelInformationDefinition { value, .. }) => {
                value.references_object_set_by_name()
            }
            _ => false,
        }
    }

    fn has_choice_selection_type(&mut self, tld: &ToplevelDefinition) -> bool {
        match tld {
            ToplevelDefinition::Type(t) => t.ty.has_choice_selection_type(),
            _ => false,
        }
    }

    fn has_components_of_notation(&mut self, tld: &ToplevelDefinition) -> bool {
        match tld {
            ToplevelDefinition::Type(t) => t.ty.contains_components_of_notation(),
            _ => false,
        }
    }

    pub fn validate(
        mut self,
    ) -> Result<(Vec<ToplevelDefinition>, Vec<CompilerError>), CompilerError> {
        let warnings: Vec<CompilerError>;
        (self, warnings) = self.link()?;
        let (mut tlds, warnings) = self.tlds.into_iter().fold(
            (Vec::<ToplevelDefinition>::new(), warnings),
            |(mut tlds, mut errors), (_, tld)| {
                match tld.validate() {
                    Ok(_) => tlds.push(tld),
                    Err(e) => errors.push(e.into()),
                }
                (tlds, errors)
            },
        );
        tlds.sort_unstable_by(|tld1, tld2| tld1.name().cmp(tld2.name()));
        Ok((tlds, warnings))
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
