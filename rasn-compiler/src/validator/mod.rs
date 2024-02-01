//! The `validator` module ensures that the
//! parsed data elements of the ASN1 specification
//! are valid declarations that can be further
//! processed. Among other things, the `validator`
//! assures that all dependencies of the individual
//! data elements resolve, and checks for conflicting
//! constraints and value definitions.
pub(crate) mod error;
mod linking;

use std::{collections::BTreeMap, error::Error, ops::Not};

use crate::intermediate::{constraints::*, types::*, *, information_object::{ClassLink, ToplevelInformationDefinition}};

use self::error::{ValidatorError, ValidatorErrorType};

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
        let mut keys = self.tlds.iter()
            .filter_map(|(k, v)| matches![v, ToplevelDefinition::Value(_)].then_some(k.clone()))
            .chain(
                self.tlds.iter()
                .filter_map(|(k, v)| matches![v, ToplevelDefinition::Value(_)].not().then_some(k.clone()))
            ).collect::<Vec<String>>();
        while let Some(key) = keys.pop() {
            if self.references_class_by_name(&key) {
                match self.tlds.remove(&key) {
                    Some(ToplevelDefinition::Type(mut tld)) => {
                        tld.ty = tld.ty.resolve_class_reference(&self.tlds);
                        self.tlds
                        .insert(tld.name.clone(), ToplevelDefinition::Type(tld));
                        },
                    Some(ToplevelDefinition::Information(mut tld)) => {
                        tld = tld.resolve_class_reference(&self.tlds);
                        self.tlds
                        .insert(tld.name.clone(), ToplevelDefinition::Information(tld));
                    },
                    _ => ()
                }
            }
            if self.has_components_of_notation(&key) {
                if let Some(ToplevelDefinition::Type(mut tld)) = self.tlds.remove(&key) {
                    tld.ty.link_components_of_notation(&self.tlds);
                    self.tlds
                        .insert(tld.name.clone(), ToplevelDefinition::Type(tld));
                }
            } 
            if self.has_choice_selection_type(&key) {
                if let Some(ToplevelDefinition::Type(mut tld)) = self.tlds.remove(&key) {
                    tld.ty.link_choice_selection_type(&self.tlds)?;
                    self.tlds
                        .insert(tld.name.clone(), ToplevelDefinition::Type(tld));
                }
            }
            if self.references_object_set_by_name(&key) {
                if let Some(ToplevelDefinition::Information(mut tld)) = self.tlds.remove(&key) {
                    tld.value.link_object_set_reference(&self.tlds);
                    self.tlds
                        .insert(tld.name.clone(), ToplevelDefinition::Information(tld));
                }
            }
            if self.has_constraint_reference(&key) {
                let mut tld = self.tlds.remove(&key).ok_or(ValidatorError {
                    data_element: Some(key.clone()),
                    details: "Could not find toplevel declaration to remove!".into(),
                    kind: ValidatorErrorType::MissingDependency,
                })?;
                if !tld.link_constraint_reference(&self.tlds)? {
                    warnings.push(
                        Box::new(
                            ValidatorError { 
                                data_element: Some(tld.name().to_string()), 
                                details: format!(
                                    "Failed to link cross-reference to elsewhere defined value in constraint of {}", 
                                    tld.name()),
                                kind: ValidatorErrorType::MissingDependency
                            }
                        )
                    )
                }
                self.tlds.insert(tld.name().clone(), tld);
            }
            if let Some(mut tld) = self.tlds.remove(&key) {
                tld.collect_supertypes(&self.tlds)?;
                self.tlds.insert(key, tld);
            }
        }

        Ok((self, warnings))
    }

    fn has_constraint_reference(&mut self, key: &String) -> bool {
        self.tlds
            .get(key)
            .map(ToplevelDefinition::has_constraint_reference)
            .unwrap_or(false)
    }

    fn references_class_by_name(&mut self, key: &String) -> bool {
        self.tlds
            .get(key)
            .map(|t| match t {
                ToplevelDefinition::Type(t) => t.ty.references_class_by_name(),
                ToplevelDefinition::Information(i) => i.class.as_ref().map_or(false, |c| match c {
                    ClassLink::ByReference(_) => false,
                    ClassLink::ByName(_) => true
                }),
                _ => false,
            })
            .unwrap_or(false)
    }

    fn references_object_set_by_name(&mut self, key: &String) -> bool {
        self.tlds
            .get(key)
            .map(|t|
                match t {
                ToplevelDefinition::Information(ToplevelInformationDefinition { value, .. }) => value.references_object_set_by_name(),
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
