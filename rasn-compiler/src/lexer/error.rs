use core::fmt::{Display, Formatter, Result};
use std::error::Error;

#[derive(Debug, Clone)]
pub struct LexerError {
    pub details: String,
    pub kind: LexerErrorType,
}

impl<'a> From<nom::Err<nom::error::Error<&'a str>>> for LexerError {
    fn from(value: nom::Err<nom::error::Error<&'a str>>) -> Self {
        match value {
            nom::Err::Incomplete(_) => Self {
                details: "Unexpected end of input!".into(),
                kind: LexerErrorType::NotEnoughData,
            },
            nom::Err::Error(e) => Self {
                details: "Error matching ASN syntax while parsing:".to_owned() + e.input,
                kind: LexerErrorType::MatchingError(e.code),
            },
            nom::Err::Failure(e) => Self {
                details: "Unrecoverable error while parsing:".to_owned() + e.input,
                kind: LexerErrorType::Failure(e.code),
            },
        }
    }
}

#[derive(Debug, Clone)]
pub enum LexerErrorType {
    NotEnoughData,
    MatchingError(nom::error::ErrorKind),
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
