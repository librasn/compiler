use core::fmt::{Display, Formatter, Result};
use std::error::Error;

use proc_macro2::LexError;

use crate::intermediate::error::GrammarError;

#[derive(Debug, Clone, PartialEq)]
pub struct GeneratorError {
    pub top_level_declaration: Option<String>,
    pub details: String,
    pub kind: GeneratorErrorType,
}

impl GeneratorError {
    pub fn new(tld: Option<String>, details: &str, kind: GeneratorErrorType) -> Self {
        GeneratorError {
            top_level_declaration: tld,
            details: details.into(),
            kind,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum GeneratorErrorType {
    Asn1TypeMismatch,
    EmptyChoiceType,
    MissingCustomSyntax,
    SyntaxMismatch,
    MissingClassKey,
    Unidentified,
    LexerError,
    FormattingError,
    IO,
    NotYetInplemented,
    Unsupported,
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
        write!(f, "{:?} generating bindings", self.kind);
        if let Some(tld) = &self.top_level_declaration {
            write!(f, " for {tld}");
        }
        write!(f, ": {}", self.details)
    }
}
