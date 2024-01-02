use core::fmt::{Display, Formatter, Result};
use std::error::Error;

use proc_macro2::LexError;

use crate::intermediate::{error::GrammarError, ToplevelDeclaration};

#[derive(Debug, Clone)]
pub struct GeneratorError {
    pub top_level_declaration: Option<ToplevelDeclaration>,
    pub details: String,
    pub kind: GeneratorErrorType,
}

impl GeneratorError {
    pub fn new(tld: Option<ToplevelDeclaration>, details: &str, kind: GeneratorErrorType) -> Self {
        GeneratorError {
            top_level_declaration: tld,
            details: details.into(),
            kind,
        }
    }
}

#[derive(Debug, Clone)]
pub enum GeneratorErrorType {
    Asn1TypeMismatch,
    EmptyChoiceType,
    MissingCustomSyntax,
    SyntaxMismatch,
    MissingClassKey,
    Unidentified,
    LexerError,
    NotYetInplemented,
}

impl Error for GeneratorError {}

impl Default for GeneratorError {
    fn default() -> Self {
        Self {
            top_level_declaration: Default::default(),
            details: Default::default(),
            kind: GeneratorErrorType::Unidentified,
        }
    }
}

impl From<GrammarError> for GeneratorError {
    fn from(value: GrammarError) -> Self {
        Self {
            details: value.details,
            top_level_declaration: None,
            kind: GeneratorErrorType::Unidentified,
        }
    }
}

impl From<LexError> for GeneratorError {
    fn from(value: LexError) -> Self {
        Self {
            details: value.to_string(),
            top_level_declaration: None,
            kind: GeneratorErrorType::LexerError,
        }
    }
}

impl Display for GeneratorError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let name = match &self.top_level_declaration {
            Some(ToplevelDeclaration::Type(t)) => &t.name,
            Some(ToplevelDeclaration::Value(v)) => &v.name,
            Some(ToplevelDeclaration::Information(i)) => &i.name,
            None => "",
        };
        write!(
            f,
            "{:?} generating Rust representation for {name}: {}",
            self.kind, self.details
        )
    }
}
