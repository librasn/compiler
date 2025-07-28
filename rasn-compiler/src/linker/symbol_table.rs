use std::{borrow::Cow, collections::HashMap, fmt::Display, ops::Add};

use crate::{
    intermediate::{macros::MacroDefinition, AsnModule, TypeAssignment, ValueAssignment},
    linker::{error::LinkerError, unnest::{Unnest, Unnested}},
    prelude::{
        ir::{ObjectClassAssignment, ObjectOrObjectSetAssignment},
        Assignment,
    },
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(super) struct SymbolId<'a> {
    pub module_reference: Cow<'a, str>,
    pub type_reference: Cow<'a, str>,
    pub scope: Scope<'a>,
}

impl<'a> SymbolId<'a> {
    pub(super) fn locally_scoped(&self, scope_id: Cow<'a, str>) -> Self {
        let mut new_id = self.clone();
        new_id.scope = new_id.scope + Scope::Local(scope_id);
        new_id
    }
}

impl Display for SymbolId<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Scope::Local(id) = &self.scope {
            write!(f, "{}.{}.{id}", self.module_reference, self.type_reference)
        } else {
            write!(f, "{}.{}", self.module_reference, self.type_reference)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(super) enum Scope<'a> {
    Module,
    Local(Cow<'a, str>),
}

impl<'a> Add<Scope<'a>> for Scope<'a> {
    type Output = Scope<'a>;

    fn add(self, rhs: Scope<'a>) -> Self::Output {
        match (self, rhs) {
            (Self::Module, Self::Module) => Self::Module,
            (Self::Local(p), Self::Module) | (Self::Module, Self::Local(p)) => Self::Local(p),
            (Self::Local(p1), Self::Local(p2)) => Self::Local(Cow::Owned(format!("{p1}.{p2}"))),
        }
    }
}

pub(super) struct SymbolTable<'a>(HashMap<SymbolId<'a>, Assignment<'a>>);

impl<'a> SymbolTable<'a> {
    pub(super) fn new(modules: Vec<AsnModule<'a>>) -> Result<Self, LinkerError> {
        let mut symbol_table = HashMap::new();
        for module in modules {
            for tld in module.top_level_definitions {
                let symbol_id = SymbolId {
                    module_reference: module.module_header.name.clone(),
                    type_reference: tld.name(),
                    scope: Scope::Module,
                };
                symbol_table.insert(symbol_id, tld);
            }
        }
        let mut slf = Self(symbol_table);
        slf.unnest()?;
        Ok(slf)
    }

    fn unnest(&mut self) -> Result<(), LinkerError> {
        let mut all_unnested = Vec::new();
        for (id, symbol) in &mut self.0 {
            if let Some(mut unnested) = symbol.unnest(id.clone(), ()) {
                all_unnested.append(&mut unnested);
            }
        }
        for Unnested { id, assignment } in all_unnested {
            self.0.insert(id, assignment);
        }
        Ok(())
    }

    pub(super) fn as_top_level_type(&self, id: &SymbolId<'a>) -> Option<&TypeAssignment<'a>> {
        self.0.get(id).and_then(|elem| match elem {
            Assignment::Type(t) => Some(t),
            _ => None,
        })
    }
    pub(super) fn as_top_level_value(&self, id: &SymbolId<'a>) -> Option<&ValueAssignment<'a>> {
        self.0.get(id).and_then(|elem| match elem {
            Assignment::Value(t) => Some(t),
            _ => None,
        })
    }
    pub(super) fn as_top_level_object(
        &self,
        id: &SymbolId<'a>,
    ) -> Option<&ObjectOrObjectSetAssignment<'a>> {
        self.0.get(id).and_then(|elem| match elem {
            Assignment::Object(t) => Some(t),
            _ => None,
        })
    }
    pub(super) fn as_top_level_class(
        &self,
        id: &SymbolId<'a>,
    ) -> Option<&ObjectClassAssignment<'a>> {
        self.0.get(id).and_then(|elem| match elem {
            Assignment::Class(t) => Some(t),
            _ => None,
        })
    }
    pub(super) fn as_top_level_macro(&self, id: &SymbolId<'a>) -> Option<&MacroDefinition<'a>> {
        self.0.get(id).and_then(|elem| match elem {
            Assignment::Macro(t) => Some(t),
            _ => None,
        })
    }
}
