use core::fmt::{Display, Formatter, Result};
use std::error::Error;

use crate::intermediate::error::GrammarError;

#[derive(Debug, Clone, PartialEq)]
pub struct LinkerError {
    pub data_element: Option<String>,
    pub details: String,
    pub kind: LinkerErrorType,
}

impl LinkerError {
    pub fn new(data_element: Option<String>, details: &str, kind: LinkerErrorType) -> Self {
        LinkerError {
            data_element,
            details: details.into(),
            kind,
        }
    }

    pub fn specify_data_element(&mut self, data_element: String) {
        self.data_element = Some(data_element)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LinkerErrorType {
    MissingDependency,
    InvalidConstraintsError,
    Unknown,
}

impl Error for LinkerError {}

impl Display for LinkerError {
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

impl From<GrammarError> for LinkerError {
    fn from(value: GrammarError) -> Self {
        Self {
            data_element: None,
            details: value.details,
            kind: LinkerErrorType::Unknown,
        }
    }
}
