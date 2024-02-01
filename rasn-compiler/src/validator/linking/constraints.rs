use std::collections::BTreeMap;

use crate::intermediate::{constraints::*, error::*, *};

impl Constraint {
    pub(super) fn link_cross_reference(
        &mut self,
        identifier: &String,
        tlds: &BTreeMap<String, ToplevelDefinition>,
    ) -> Result<bool, GrammarError> {
        match self {
            Constraint::SubtypeConstraint(t) => t.set.link_cross_reference(identifier, tlds),
            _ => Ok(false),
        }
    }

    pub(super) fn has_cross_reference(&self) -> bool {
        if let Self::SubtypeConstraint(c) = self {
            c.set.has_cross_reference()
        } else {
            false
        }
    }
}

impl SubtypeElement {
    pub(super) fn link_cross_reference(
        &mut self,
        identifier: &String,
        tlds: &BTreeMap<String, ToplevelDefinition>,
    ) -> Result<bool, GrammarError> {
        match self {
            SubtypeElement::SingleValue {
                value,
                extensible: _,
            } => value.link_elsewhere_declared(identifier, tlds),
            SubtypeElement::PermittedAlphabet(e) => e.link_cross_reference(identifier, tlds),
            SubtypeElement::ContainedSubtype {
                subtype,
                extensible: _,
            } => subtype.link_subtype_constraint(tlds),
            SubtypeElement::ValueRange {
                min,
                max,
                extensible: _,
            } => {
                let a = min
                    .as_mut()
                    .map_or(Ok(false), |m| m.link_elsewhere_declared(identifier, tlds))?;
                let b = max
                    .as_mut()
                    .map_or(Ok(false), |m| m.link_elsewhere_declared(identifier, tlds))?;
                Ok(a || b)
            }
            SubtypeElement::SizeConstraint(s) => s.link_cross_reference(identifier, tlds),
            SubtypeElement::TypeConstraint(t) => t.link_constraint_reference(identifier, tlds),
            SubtypeElement::SingleTypeConstraint(s)
            | SubtypeElement::MultipleTypeConstraints(s) => s
                .constraints
                .iter_mut()
                .flat_map(|cc| &mut cc.constraints)
                .try_fold(false, |acc, b| {
                    b.link_cross_reference(identifier, tlds)
                        .map(|res| res || acc)
                }),
            _ => Ok(false),
        }
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
            SubtypeElement::MultipleTypeConstraints(s)
            | SubtypeElement::SingleTypeConstraint(s) => s
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
    ) -> Result<bool, GrammarError> {
        match self {
            ElementOrSetOperation::Element(e) => e.link_cross_reference(identifier, tlds),
            ElementOrSetOperation::SetOperation(s) => {
                let a = s.base.link_cross_reference(identifier, tlds)?;
                let b = s.operant.link_cross_reference(identifier, tlds)?;
                Ok(a || b)
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
