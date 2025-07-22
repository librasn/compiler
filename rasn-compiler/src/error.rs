use std::error::Error;
use std::fmt::Display;

use crate::{
    lexer::error::LexerError,
    prelude::{ir::GrammarError, GeneratorError},
    validator::error::LinkerError,
};

#[derive(Debug, Clone, PartialEq)]
pub enum CompilerError {
    Lexer(LexerError),
    Grammar(GrammarError),
    Linker(LinkerError),
    Generator(String),
}

impl CompilerError {
    pub fn contextualize(&self, input: &str) -> String {
        match self {
            CompilerError::Lexer(lexer_error) => lexer_error.contextualize(input),
            CompilerError::Generator(e) => e.clone(),
            e => e.to_string(),
        }
    }
}

impl Display for CompilerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompilerError::Lexer(lexer_error) => Display::fmt(lexer_error, f),
            CompilerError::Grammar(grammar_error) => Display::fmt(grammar_error, f),
            CompilerError::Linker(linker_error) => Display::fmt(linker_error, f),
            CompilerError::Generator(generator_error) => Display::fmt(generator_error, f),
        }
    }
}

impl Error for CompilerError {}

impl From<LexerError> for CompilerError {
    fn from(value: LexerError) -> Self {
        Self::Lexer(value)
    }
}

impl From<LinkerError> for CompilerError {
    fn from(value: LinkerError) -> Self {
        Self::Linker(value)
    }
}

impl From<GeneratorError> for CompilerError {
    fn from(value: GeneratorError) -> Self {
        Self::Generator(value.to_string())
    }
}

impl From<GrammarError> for CompilerError {
    fn from(value: GrammarError) -> Self {
        Self::Grammar(value)
    }
}
