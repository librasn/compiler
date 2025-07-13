use std::collections::BTreeMap;

use crate::intermediate::{constraints::*, error::*, *};

impl Constraint {
    pub(super) fn link_cross_reference(
        &mut self,
        identifier: &String,
        tlds: &BTreeMap<String, ToplevelDefinition>,
    ) -> Result<(), GrammarError> {
        match self {
            Constraint::Subtype(t) => t.set.link_cross_reference(identifier, tlds),
            _ => Ok(()),
        }
    }

    pub(super) fn has_cross_reference(&self) -> bool {
        match self {
            Self::Subtype(c) => c.set.has_cross_reference(),
            Self::Parameter(_) => true,
            _ => false,
        }
    }
}

impl SubtypeElements {
    pub(super) fn link_cross_reference(
        &mut self,
        identifier: &String,
        tlds: &BTreeMap<String, ToplevelDefinition>,
    ) -> Result<(), GrammarError> {
        match self {
            SubtypeElements::SingleValue {
                value,
                extensible: _,
            } => {
                value.link_elsewhere_declared(identifier, tlds)?;
            }
            SubtypeElements::PermittedAlphabet(e) => {
                e.link_cross_reference(identifier, tlds)?;
            }
            SubtypeElements::ContainedSubtype {
                subtype,
                extensible: _,
            } => {
                subtype.link_subtype_constraint(tlds)?;
            }
            SubtypeElements::ValueRange {
                min,
                max,
                extensible: _,
            } => {
                min.as_mut()
                    .map(|m| m.link_elsewhere_declared(identifier, tlds))
                    .transpose()?;
                max.as_mut()
                    .map(|m| m.link_elsewhere_declared(identifier, tlds))
                    .transpose()?;
            }
            SubtypeElements::SizeConstraint(s) => {
                s.link_cross_reference(identifier, tlds)?;
            }
            SubtypeElements::TypeConstraint(t) => {
                t.link_constraint_reference(identifier, tlds)?;
            }
            SubtypeElements::SingleTypeConstraint(constraints) => {
                for c in constraints {
                    c.link_cross_reference(identifier, tlds)?;
                }
            }
            SubtypeElements::MultipleTypeConstraints(s) => {
                for b in s.constraints.iter_mut().flat_map(|cc| &mut cc.constraints) {
                    b.link_cross_reference(identifier, tlds)?;
                }
            }
            _ => (),
        }
        Ok(())
    }

    pub(super) fn has_cross_reference(&self) -> bool {
        match self {
            SubtypeElements::SingleValue {
                value,
                extensible: _,
            } => value.is_elsewhere_declared(),
            SubtypeElements::PatternConstraint(_) => false,
            SubtypeElements::UserDefinedConstraint(_) => false,
            SubtypeElements::PropertySettings(_) => false,
            SubtypeElements::PermittedAlphabet(e) => e.has_cross_reference(),
            SubtypeElements::ContainedSubtype {
                subtype,
                extensible: _,
            } => subtype.contains_constraint_reference(),
            SubtypeElements::ValueRange {
                min,
                max,
                extensible: _,
            } => {
                min.as_ref().is_some_and(|s| s.is_elsewhere_declared())
                    || max.as_ref().is_some_and(|s| s.is_elsewhere_declared())
            }
            SubtypeElements::SizeConstraint(s) => s.has_cross_reference(),
            SubtypeElements::TypeConstraint(t) => t.references_class_by_name(),
            SubtypeElements::SingleTypeConstraint(c) => {
                c.iter().any(|constraint| constraint.has_cross_reference())
            }
            SubtypeElements::MultipleTypeConstraints(s) => s
                .constraints
                .iter()
                .any(|cc| cc.constraints.iter().any(|c| c.has_cross_reference())),
        }
    }
}

impl ElementOrSetOperation {
    pub(super) fn link_cross_reference(
        &mut self,
        identifier: &String,
        tlds: &BTreeMap<String, ToplevelDefinition>,
    ) -> Result<(), GrammarError> {
        match self {
            ElementOrSetOperation::Element(e) => e.link_cross_reference(identifier, tlds),
            ElementOrSetOperation::SetOperation(s) => {
                s.base.link_cross_reference(identifier, tlds)?;
                s.operant.link_cross_reference(identifier, tlds)
            }
        }
    }

    pub(super) fn has_cross_reference(&self) -> bool {
        match self {
            ElementOrSetOperation::Element(e) => e.has_cross_reference(),
            ElementOrSetOperation::SetOperation(s) => {
                s.base.has_cross_reference() || s.operant.has_cross_reference()
            }
        }
    }
}
