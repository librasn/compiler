use std::collections::BTreeMap;

use crate::intermediate::{constraints::*, error::*, *};

impl Constraint {
    pub(super) fn link_cross_reference(
        &mut self,
        identifier: &String,
        tlds: &BTreeMap<String, ToplevelDefinition>,
    ) -> Result<(), GrammarError> {
        match self {
            Constraint::SubtypeConstraint(t) => t.set.link_cross_reference(identifier, tlds),
            _ => Ok(()),
        }
    }

    pub(super) fn has_cross_reference(&self) -> bool {
        match self {
            Self::SubtypeConstraint(c) => c.set.has_cross_reference(),
            Self::Parameter(_) => true,
            _ => false,
        }
    }
}

impl SubtypeElement {
    pub(super) fn link_cross_reference(
        &mut self,
        identifier: &String,
        tlds: &BTreeMap<String, ToplevelDefinition>,
    ) -> Result<(), GrammarError> {
        match self {
            SubtypeElement::SingleValue {
                value,
                extensible: _,
            } => {
                value.link_elsewhere_declared(identifier, tlds)?;
            }
            SubtypeElement::PermittedAlphabet(e) => {
                e.link_cross_reference(identifier, tlds)?;
            }
            SubtypeElement::ContainedSubtype {
                subtype,
                extensible: _,
            } => {
                subtype.link_subtype_constraint(tlds)?;
            }
            SubtypeElement::ValueRange {
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
            SubtypeElement::SizeConstraint(s) => {
                s.link_cross_reference(identifier, tlds)?;
            }
            SubtypeElement::TypeConstraint(t) => {
                t.link_constraint_reference(identifier, tlds)?;
            }
            SubtypeElement::SingleTypeConstraint(constraints) => {
                for c in constraints {
                    c.link_cross_reference(identifier, tlds)?;
                }
            }
            SubtypeElement::MultipleTypeConstraints(s) => {
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
            SubtypeElement::SingleValue {
                value,
                extensible: _,
            } => value.is_elsewhere_declared(),
            SubtypeElement::PatternConstraint(_) => false,
            SubtypeElement::UserDefinedConstraint(_) => false,
            SubtypeElement::PropertySettings(_) => false,
            SubtypeElement::PermittedAlphabet(e) => e.has_cross_reference(),
            SubtypeElement::ContainedSubtype {
                subtype,
                extensible: _,
            } => subtype.contains_constraint_reference(),
            SubtypeElement::ValueRange {
                min,
                max,
                extensible: _,
            } => {
                min.as_ref().map_or(false, |s| s.is_elsewhere_declared())
                    || max.as_ref().map_or(false, |s| s.is_elsewhere_declared())
            }
            SubtypeElement::SizeConstraint(s) => s.has_cross_reference(),
            SubtypeElement::TypeConstraint(t) => t.references_class_by_name(),
            SubtypeElement::SingleTypeConstraint(c) => {
                c.iter().any(|constraint| constraint.has_cross_reference())
            }
            SubtypeElement::MultipleTypeConstraints(s) => s
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
