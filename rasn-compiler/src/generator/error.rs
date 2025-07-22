use core::fmt::{Display, Formatter, Result};
use std::error::Error;

use proc_macro2::LexError;

use crate::intermediate::{error::GrammarError, ToplevelDefinition};

#[derive(Debug, Clone, PartialEq)]
pub struct GeneratorError<'a> {
    pub top_level_declaration: Option<Box<ToplevelDefinition<'a>>>,
    pub details: String,
    pub kind: GeneratorErrorType,
}

impl<'a> GeneratorError<'a> {
    pub fn new(tld: Option<ToplevelDefinition<'a>>, details: &str, kind: GeneratorErrorType) -> Self {
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

impl<'a> Error for GeneratorError<'a> {}

impl<'a> Default for GeneratorError<'a> {
    fn default() -> Self {
        Self {
            top_level_declaration: Default::default(),
            details: Default::default(),
            kind: GeneratorErrorType::Unidentified,
        }
    }
}

impl<'a> From<GrammarError> for GeneratorError<'a> {
    fn from(value: GrammarError) -> Self {
        Self {
            details: value.details,
            top_level_declaration: None,
            kind: GeneratorErrorType::Unidentified,
        }
    }
}

impl<'a> From<LexError> for GeneratorError<'a> {
    fn from(value: LexError) -> Self {
        Self {
            details: value.to_string(),
            top_level_declaration: None,
            kind: GeneratorErrorType::LexerError,
        }
    }
}

impl<'a> Display for GeneratorError<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let name = match self.top_level_declaration.as_deref() {
            Some(ToplevelDefinition::Type(t)) => &t.name,
            Some(ToplevelDefinition::Value(v)) => &v.name,
            Some(ToplevelDefinition::Class(c)) => &c.name,
            Some(ToplevelDefinition::Object(o)) => &o.name,
            Some(ToplevelDefinition::Macro(m)) => &m.name,
            None => "",
        };
        write!(
            f,
            "{:?} generating bindings for {name}: {}",
            self.kind, self.details
        )
    }
}
