use std::{
    borrow::Cow,
    collections::HashMap,
    fmt::Display,
    marker::PhantomData,
    ops::{Add, AddAssign},
};

use chrono::format::Item;

use crate::{
    intermediate::{
        macros::MacroDefinition, ASN1Type, AsnModule, DeclarationElsewhere, DefinedType,
        TypeAssignment, ValueAssignment,
    },
    linker::{
        error::LinkerError,
        resolve_parameters::{self, SymbolTableExtraParams},
        unnest::Unnest,
    },
    prelude::{
        ir::{ObjectClassAssignment, ObjectOrObjectSetAssignment},
        Assignment,
    },
};

#[derive(Debug, Clone, PartialEq)]
pub(super) struct SymbolTableEntry<'a> {
    pub id: SymbolId<'a>,
    pub assignment: Assignment<'a>,
}

#[derive(Debug, Clone, PartialEq)]
pub(super) struct GeneratedSymbols<'a>(pub Option<Vec<SymbolTableEntry<'a>>>);

impl<'a> AddAssign for GeneratedSymbols<'a> {
    fn add_assign(&mut self, rhs: Self) {
        match (&mut self.0, rhs.0) {
            (Some(e1), Some(mut e2)) => {
                e1.append(&mut e2);
            }
            (s @ None, rhs) => {
                *s = rhs;
            }
            (Some(_), None) => (),
        }
    }
}

impl<'a> IntoIterator for GeneratedSymbols<'a> {
    type Item = SymbolTableEntry<'a>;

    type IntoIter = std::vec::IntoIter<SymbolTableEntry<'a>>;

    fn into_iter(self) -> Self::IntoIter {
        match self.0 {
            Some(s) => s.into_iter(),
            None => Vec::with_capacity(0).into_iter(),
        }
    }
}

impl<'a> FromIterator<GeneratedSymbols<'a>> for GeneratedSymbols<'a> {
    fn from_iter<T: IntoIterator<Item = GeneratedSymbols<'a>>>(iter: T) -> Self {
        iter.into_iter()
            .fold(GeneratedSymbols::empty(), |mut acc, g| {
                acc += g;
                acc
            })
    }
}

impl<'a> FromIterator<SymbolTableEntry<'a>> for GeneratedSymbols<'a> {
    fn from_iter<T: IntoIterator<Item = SymbolTableEntry<'a>>>(iter: T) -> Self {
        let vec = Vec::from_iter(iter);
        if vec.is_empty() {
            GeneratedSymbols::empty()
        } else {
            GeneratedSymbols(Some(vec))
        }
    }
}

impl<'a> GeneratedSymbols<'a> {
    pub(super) fn new(symbols: Vec<SymbolTableEntry<'a>>) -> Self {
        Self(Some(symbols))
    }

    pub(super) fn single(symbol: SymbolTableEntry<'a>) -> Self {
        Self(Some(vec![symbol]))
    }

    pub(super) fn empty() -> Self {
        Self(None)
    }

    pub(super) fn iter_mut(&mut self) -> impl Iterator<Item = &mut SymbolTableEntry<'a>> {
        match &mut self.0 {
            Some(s) => s.iter_mut(),
            None => [].iter_mut(),
        }
    }

    pub(super) fn is_empty(&self) -> bool {
        match &self.0 {
            None => true,
            Some(e) => e.is_empty(),
        }
    }
}

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

pub(super) trait SymbolTableState {}
pub(super) struct LexedSymbols;
impl SymbolTableState for LexedSymbols {}
pub(super) struct UnnestedSymbols;
impl SymbolTableState for UnnestedSymbols {}
pub(super) struct ResolvedParameters;
impl SymbolTableState for ResolvedParameters {}
pub(super) struct ResolvedConstraintReferences;
impl SymbolTableState for ResolvedConstraintReferences {}

pub(super) struct SymbolTable<'a, S: SymbolTableState>(
    HashMap<SymbolId<'a>, Assignment<'a>>,
    PhantomData<S>,
);

impl<'a> SymbolTable<'a, LexedSymbols> {
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
        Ok(Self(symbol_table, PhantomData))
    }

    fn unnest(mut self) -> Result<SymbolTable<'a, UnnestedSymbols>, LinkerError> {
        let mut all_unnested = GeneratedSymbols::empty();
        for (id, symbol) in &mut self.0 {
            all_unnested += symbol.unnest(id.clone(), ());
        }
        for SymbolTableEntry { id, assignment } in all_unnested {
            self.0.insert(id, assignment);
        }
        Ok(SymbolTable(self.0, PhantomData))
    }
}

impl<'a> SymbolTable<'a, UnnestedSymbols> {
    pub(super) fn resolve_parameters(
        mut self,
    ) -> Result<SymbolTable<'a, ResolvedParameters>, LinkerError> {
        for (id, symbol) in &mut self.0 {
            match symbol {
                Assignment::Type(TypeAssignment { ty, .. }) => {
                    ty.apply_resursively::<_, SymbolTableExtraParams>(&mut |ty, extra| {
                        resolve_parameters::resolve_parameters(ty, extra)
                    })?;
                }
                _ => todo!(),
            }
        }
        Ok(SymbolTable(self.0, PhantomData))
    }
}

impl<'a, S: SymbolTableState> SymbolTable<'a, S> {
    pub(super) fn get(&self, id: &SymbolId<'a>) -> Option<&Assignment<'a>> {
        self.0.get(id)
    }
    /// Traverses a dependency graph and returns the leaf type.
    /// Caution: The returned type is a reference to the leaf type
    /// and does not take into account any contraints that are applied along the dependency chain to the root.
    /// ```ignore
    /// TypeRefA ::= TypeRefB (1..4) /* <--- these constraints are not considered in the return type */
    /// TypeRefB ::= INTEGER (0..8) /* <--- a reference to this type will be returned */
    /// ```
    pub(super) fn get_root_type(&self, id: &SymbolId<'a>) -> Option<&ASN1Type<'a>> {
        self.as_top_level_type(id).and_then(|ty| {
            match ty {
                // types have been unnested, so we do not need to traverse constructed types
                TypeAssignment {
                    ty: ASN1Type::ElsewhereDeclaredType(DeclarationElsewhere { identifier, .. }),
                    ..
                } => {
                    todo!()
                }
                t => Some(&t.ty),
            }
        })
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
