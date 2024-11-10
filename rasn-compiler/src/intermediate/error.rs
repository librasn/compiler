use std::{
    error::Error,
    fmt::{Display, Formatter, Result},
};

#[derive(Debug, Clone, PartialEq)]
pub struct GrammarError {
    pub details: String,
    pub kind: GrammarErrorType,
    pub pdu: Option<String>,
}

impl GrammarError {
    pub fn new(data_details: &str, kind: GrammarErrorType) -> Self {
        GrammarError {
            details: data_details.into(),
            kind,
            pdu: None,
        }
    }

    pub fn todo() -> Self {
        GrammarError {
            details: "Not yet implemented!".into(),
            kind: GrammarErrorType::NotYetInplemented,
            pdu: None,
        }
    }

    pub fn contextualize(&mut self, pdu: &str) {
        self.pdu = Some(pdu.into());
    }
}

impl Error for GrammarError {}

#[derive(Debug, Clone, PartialEq)]
pub enum GrammarErrorType {
    UnpackingError,
    LinkerError,
    PerVisibleConstraintError,
    NotYetInplemented,
    SyntaxMismatch,
}

impl Display for GrammarError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?} in ASN grammar: {}", self.kind, self.details)
    }
}
