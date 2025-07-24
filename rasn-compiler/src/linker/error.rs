use core::fmt::{Display, Formatter, Result};
use std::error::Error;

use crate::intermediate::error::GrammarError;

#[derive(Debug, Clone, PartialEq)]
pub struct LinkerError {
    pub pdu: Option<String>,
    pub details: String,
    pub kind: LinkerErrorType,
}

impl LinkerError {
    pub fn new(pdu: Option<String>, details: &str, kind: LinkerErrorType) -> Self {
        LinkerError {
            pdu,
            details: details.into(),
            kind,
        }
    }

    pub fn contextualize(&mut self, pdu: &str) {
        self.pdu = Some(pdu.into())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LinkerErrorType {
    MissingDependency,
    RecursionLimitExceeded,
    InvalidConstraintsError,
    Unknown,
}

impl Error for LinkerError {}

impl Display for LinkerError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "{:?} validating PDU {}: {}",
            self.kind,
            self.pdu.as_ref().unwrap_or(&"".into()),
            self.details
        )
    }
}

impl From<GrammarError> for LinkerError {
    fn from(value: GrammarError) -> Self {
        Self {
            pdu: None,
            details: value.details,
            kind: LinkerErrorType::Unknown,
        }
    }
}
