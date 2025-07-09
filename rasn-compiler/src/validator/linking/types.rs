use std::collections::BTreeMap;

use super::{
    ASN1Type, DeclarationElsewhere, GrammarError, ToplevelDefinition, ToplevelTypeDefinition,
};

impl DeclarationElsewhere {
    pub fn root<'a>(
        &self,
        tlds: &'a BTreeMap<String, ToplevelDefinition>,
    ) -> Result<&'a ASN1Type, GrammarError> {
        match tlds.get(&self.identifier).ok_or_else(|| GrammarError::new(
            &format!("Failed to resolve reference of ElsewhereDefined: {}", self.identifier),
            super::GrammarErrorType::LinkerError
        ))? {
            ToplevelDefinition::Type(ToplevelTypeDefinition { ty: ASN1Type::ElsewhereDeclaredType(e), .. }) => e.root(tlds),
            ToplevelDefinition::Type(ToplevelTypeDefinition { ty, .. }) => Ok(ty),
            ToplevelDefinition::Class(_) => Err(GrammarError::todo()),
            ToplevelDefinition::Object(_) => Err(GrammarError::todo()),
            _ => Err(GrammarError::new(
                &format!(
                    "Unexpectedly found a value definition resolving reference of ElsewhereDefined: {}",
                    self.identifier
                ),
                super::GrammarErrorType::LinkerError
            ))
        }
    }
}
