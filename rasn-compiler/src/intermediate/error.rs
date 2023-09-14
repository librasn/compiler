use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone)]
pub struct GrammarError {
    pub details: String,
    pub kind: GrammarErrorType,
}

impl GrammarError {
    pub fn new(data_details: &str, kind: GrammarErrorType) -> Self {
      GrammarError { details: data_details.into(), kind }
    }
}

#[derive(Debug, Clone)]
pub enum GrammarErrorType {
    UnpackingError,
    LinkerError,
    PerVisibleConstraintError,
}

impl Display for GrammarError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "{:?} in ASN grammar: {}",
            self.kind, self.details
        )
    }
}
