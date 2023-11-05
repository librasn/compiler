use core::fmt::{Display, Formatter, Result};
use std::error::Error;

use crate::intermediate::error::GrammarError;

#[derive(Debug, Clone)]
pub struct ValidatorError {
    pub data_element: Option<String>,
    pub details: String,
    pub kind: ValidatorErrorType,
}

impl ValidatorError {
    pub fn new(data_element: Option<String>, details: &str, kind: ValidatorErrorType) -> Self {
        ValidatorError {
            data_element,
            details: details.into(),
            kind,
        }
    }

    pub fn specify_data_element(&mut self, data_element: String) {
        self.data_element = Some(data_element)
    }
}

#[derive(Debug, Clone)]
pub enum ValidatorErrorType {
    MissingDependency,
    InvalidConstraintsError,
    Unknown,
}

impl Error for ValidatorError {}

impl Display for ValidatorError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "{:?} validating parsed data element {}: {}",
            self.kind,
            self.data_element.as_ref().unwrap_or(&"".into()),
            self.details
        )
    }
}

impl From<GrammarError> for ValidatorError {
    fn from(value: GrammarError) -> Self {
        Self {
            data_element: None,
            details: value.details,
            kind: ValidatorErrorType::Unknown,
        }
    }
}
