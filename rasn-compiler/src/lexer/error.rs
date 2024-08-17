use core::fmt::{Display, Formatter, Result};
use std::error::Error;

use super::Span;

#[derive(Debug, Clone)]
pub struct LexerError {
    pub details: String,
    pub kind: LexerErrorType,
}

impl From<nom::Err<nom::error::Error<Span<'_>>>> for LexerError {
    fn from(value: nom::Err<nom::error::Error<Span>>) -> Self {
        match value {
            nom::Err::Incomplete(_) => Self {
                details: "Unexpected end of input!".into(),
                kind: LexerErrorType::NotEnoughData,
            },
            nom::Err::Error(e) => Self {
                details: format!("Error matching ASN syntax while parsing: {}", e.input),
                kind: LexerErrorType::MatchingError(e.code),
            },
            nom::Err::Failure(e) => Self {
                details: format!("Unrecoverable error while parsing: {}", e.input),
                kind: LexerErrorType::Failure(e.code),
            },
        }
    }
}

#[derive(Debug, Clone)]
pub enum LexerErrorType {
    NotEnoughData,
    #[allow(dead_code)]
    MatchingError(nom::error::ErrorKind),
    #[allow(dead_code)]
    Failure(nom::error::ErrorKind),
}

impl Error for LexerError {}

impl Display for LexerError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "Encountered error while parsing {:?} - {}",
            self.kind, self.details
        )
    }
}
