use core::fmt::{Display, Formatter, Result};
use std::error::Error;

use proc_macro2::LexError;

use crate::{
    error::CompilerError,
    intermediate::{error::GrammarError, ToplevelDefinition},
};

#[derive(Debug, Clone)]
pub struct GeneratorError {
    pub top_level_declaration: Option<ToplevelDefinition>,
    pub details: String,
    pub kind: GeneratorErrorType,
}

impl GeneratorError {
    pub fn new(tld: Option<ToplevelDefinition>, details: &str, kind: GeneratorErrorType) -> Self {
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
    FormattingError,
    IO,
    NotYetInplemented,
}

impl Error for GeneratorError {}

impl CompilerError for GeneratorError {
    fn as_report(&self, input: &str) -> String {
        format!("{self:#?}")
    }

    fn as_styled_report(&self) -> String {
        format!("{self:#?}")
    }
}

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
            Some(ToplevelDefinition::Type(t)) => &t.name,
            Some(ToplevelDefinition::Value(v)) => &v.name,
            Some(ToplevelDefinition::Information(i)) => &i.name,
            None => "",
        };
        write!(
            f,
            "{:?} generating bindings for {name}: {}",
            self.kind, self.details
        )
    }
}
