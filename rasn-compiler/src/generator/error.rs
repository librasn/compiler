use core::fmt::{Display, Formatter, Result};
use std::error::Error;

use proc_macro2::LexError;

use crate::intermediate::{error::GrammarError, ToplevelDefinition};

#[derive(Debug, Clone, PartialEq)]
pub struct GeneratorError {
    pub top_level_declaration: Option<Box<ToplevelDefinition>>,
    pub details: String,
    pub kind: GeneratorErrorType,
}

impl GeneratorError {
    pub fn new(tld: Option<ToplevelDefinition>, details: &str, kind: GeneratorErrorType) -> Self {
        GeneratorError {
            top_level_declaration: tld.map(Box::new),
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

impl Display for GeneratorErrorType {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            GeneratorErrorType::Asn1TypeMismatch => f.write_str("ASN1 type mismatch"),
            GeneratorErrorType::EmptyChoiceType => f.write_str("Empty CHOICE type"),
            GeneratorErrorType::MissingCustomSyntax => f.write_str("Missing custom syntax"),
            GeneratorErrorType::SyntaxMismatch => f.write_str("Syntax mismatch"),
            GeneratorErrorType::MissingClassKey => f.write_str("Missing CLASS key"),
            GeneratorErrorType::Unidentified => f.write_str("Unidentified error"),
            GeneratorErrorType::LexerError => f.write_str("Lexer error"),
            GeneratorErrorType::FormattingError => f.write_str("Formatting error"),
            GeneratorErrorType::IO => f.write_str("IO error"),
            GeneratorErrorType::NotYetInplemented => f.write_str("Not yet implemented error"),
            GeneratorErrorType::Unsupported => f.write_str("Unsupported error"),
        }
    }
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
        write!(f, "{} while generating bindings", self.kind)?;
        if let Some(tld) = &self.top_level_declaration {
            write!(f, " for {}", tld.name())?;
        }
        write!(f, ": {}", self.details)
    }
}
