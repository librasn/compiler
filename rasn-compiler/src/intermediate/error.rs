use std::{
    error::Error,
    fmt::{Display, Formatter, Result},
};

use crate::error::CompilerError;

#[derive(Debug, Clone)]
pub struct GrammarError {
    pub details: String,
    pub kind: GrammarErrorType,
}

impl GrammarError {
    pub fn new(data_details: &str, kind: GrammarErrorType) -> Self {
        GrammarError {
            details: data_details.into(),
            kind,
        }
    }

    pub fn todo() -> Self {
        GrammarError {
            details: "Not yet implemented!".into(),
            kind: GrammarErrorType::NotYetInplemented,
        }
    }
}

impl CompilerError for GrammarError {
    fn as_report(&self, input: &str) -> String {
        format!("{self:#?}")
    }

    fn as_styled_report(&self) -> String {
        format!("{self:#?}")
    }
}

impl Error for GrammarError {}

#[derive(Debug, Clone)]
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
