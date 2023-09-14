use core::fmt::{Display, Formatter, Result};
use std::error::Error;

#[derive(Debug, Clone)]
pub struct ParserError {
    pub details: String,
    pub kind: ParserErrorType,
}

impl<'a> From<nom::Err<nom::error::Error<&'a str>>> for ParserError {
    fn from(value: nom::Err<nom::error::Error<&'a str>>) -> Self {
        match value {
            nom::Err::Incomplete(_) => Self {
                details: "Unexpected end of input!".into(),
                kind: ParserErrorType::NotEnoughData,
            },
            nom::Err::Error(e) => Self {
                details: "Error matching ASN syntax while parsing:".to_owned() + e.input,
                kind: ParserErrorType::MatchingError(e.code),
            },
            nom::Err::Failure(e) => Self {
                details: "Unrecoverable error while parsing:".to_owned() + e.input,
                kind: ParserErrorType::Failure(e.code),
            },
        }
    }
}

#[derive(Debug, Clone)]
pub enum ParserErrorType {
    NotEnoughData,
    MatchingError(nom::error::ErrorKind),
    Failure(nom::error::ErrorKind),
}

impl Error for ParserError {}

impl Display for ParserError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "Encountered error while parsing {:?} - {}",
            self.kind,
            self.details
        )
    }
}
