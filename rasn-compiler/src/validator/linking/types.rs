use std::collections::BTreeMap;

use super::{ASN1Type, DeclarationElsewhere, GrammarError, ToplevelDeclaration, ToplevelTypeDeclaration};

impl DeclarationElsewhere {
    pub fn root<'a>(&self, tlds: &'a BTreeMap<String, ToplevelDeclaration>) -> Result<&'a ASN1Type, GrammarError> {
        match tlds.get(&self.identifier).ok_or_else(|| GrammarError {
            details: format!("Failed to resolve reference of ElsewhereDefined: {}", self.identifier),
            kind: super::GrammarErrorType::LinkerError
        })? {
            ToplevelDeclaration::Type(ToplevelTypeDeclaration { r#type: ASN1Type::ElsewhereDeclaredType(e), .. }) => e.root(tlds),
            ToplevelDeclaration::Type(ToplevelTypeDeclaration { r#type, .. }) => Ok(r#type),
            ToplevelDeclaration::Information(_) => todo!(),
            _ => Err(GrammarError {
                details: format!("Unexpectedly found a value definition resolving reference of ElsewhereDefined: {}", self.identifier),
                kind: super::GrammarErrorType::LinkerError
            })
        }
    }
}