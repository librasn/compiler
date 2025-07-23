use std::{borrow::Cow, collections::HashMap};

use crate::{
    intermediate::{macros::ToplevelMacroDefinition, AsnModule, ToplevelTypeDefinition, ToplevelValueDefinition}, linker::error::LinkerError, prelude::{ir::{ObjectClassAssignment, ToplevelInformationDefinition}, ToplevelDefinition}
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct SymbolId<'a> {
    module_reference: Cow<'a, str>,
    type_reference: Cow<'a, str>,
}

pub(super) struct SymbolTable<'a>(HashMap<SymbolId<'a>, ToplevelDefinition<'a>>);

impl<'a> SymbolTable<'a> {
    pub(super) fn new(modules: Vec<AsnModule<'a>>) -> Result<Self, LinkerError> {
        let mut symbol_table = HashMap::new();
        for module in modules {
            for tld in module.top_level_definitions {
                let symbol_id = SymbolId {
                    module_reference: module.module_header.name.clone(),
                    type_reference: tld.name(),
                };
                symbol_table.insert(symbol_id, tld);
            }
        }
        let mut slf = Self(symbol_table);
        slf.resolve_references()?;
        Ok(slf)
    }

    fn resolve_references(&mut self) -> Result<(), LinkerError> {
        
    }

    pub(super) fn as_top_level_type(
        &self,
        id: &SymbolId<'a>,
    ) -> Option<&ToplevelTypeDefinition<'a>> {
        self.0.get(id).and_then(|elem| match elem {
            ToplevelDefinition::Type(t) => Some(t),
            _ => None
        })
    }
    pub(super) fn as_top_level_value(
        &self,
        id: &SymbolId<'a>,
    ) -> Option<&ToplevelValueDefinition<'a>> {
        self.0.get(id).and_then(|elem| match elem {
            ToplevelDefinition::Value(t) => Some(t),
            _ => None
        })
    }
    pub(super) fn as_top_level_object(
        &self,
        id: &SymbolId<'a>,
    ) -> Option<&ToplevelInformationDefinition<'a>> {
        self.0.get(id).and_then(|elem| match elem {
            ToplevelDefinition::Object(t) => Some(t),
            _ => None
        })
    }
    pub(super) fn as_top_level_class(
        &self,
        id: &SymbolId<'a>,
    ) -> Option<&ObjectClassAssignment<'a>> {
        self.0.get(id).and_then(|elem| match elem {
            ToplevelDefinition::Class(t) => Some(t),
            _ => None
        })
    }
    pub(super) fn as_top_level_macro(
        &self,
        id: &SymbolId<'a>,
    ) -> Option<&ToplevelMacroDefinition<'a>> {
        self.0.get(id).and_then(|elem| match elem {
            ToplevelDefinition::Macro(t) => Some(t),
            _ => None
        })
    }
}
