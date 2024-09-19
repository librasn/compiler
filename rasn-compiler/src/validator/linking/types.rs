use std::collections::BTreeMap;

use super::{
    ASN1Type, DeclarationElsewhere, GrammarError, ToplevelDefinition, ToplevelTypeDefinition
};

impl DeclarationElsewhere {
    pub fn root<'a>(
        &self,
        tlds: &'a BTreeMap<String, ToplevelDefinition>,
    ) -> Result<&'a ASN1Type, GrammarError> {
        match tlds.get(&self.identifier).ok_or_else(|| GrammarError {
            details: format!("Failed to resolve reference of ElsewhereDefined: {}", self.identifier),
            kind: super::GrammarErrorType::LinkerError
        })? {
            ToplevelDefinition::Type(ToplevelTypeDefinition { ty: ASN1Type::ElsewhereDeclaredType(e), .. }) => e.root(tlds),
            ToplevelDefinition::Type(ToplevelTypeDefinition { ty, .. }) => Ok(ty),
            ToplevelDefinition::Information(_) => todo!("Elsewhere Defined Information root"),
            _ => Err(GrammarError {
                details: format!("Unexpectedly found a value definition resolving reference of ElsewhereDefined: {}", self.identifier),
                kind: super::GrammarErrorType::LinkerError
            })
        }
    }
}