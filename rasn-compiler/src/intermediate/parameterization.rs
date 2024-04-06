use std::default;

use super::{information_object::InformationObjectClass, ASN1Type};

#[derive(Debug, Clone, PartialEq)]
pub struct Parameterization {
    pub parameters: Vec<ParameterizationArgument>,
}

impl From<Vec<ParameterizationArgument>> for Parameterization {
    fn from(value: Vec<ParameterizationArgument>) -> Self {
        Self { parameters: value }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParameterizationArgument {
    pub dummy_reference: String,
    pub param_governor: ParameterGovernor,
}

impl From<&str> for ParameterizationArgument {
    fn from(value: &str) -> Self {
        Self {
            dummy_reference: value.to_owned(),
            param_governor: ParameterGovernor::None,
        }
    }
}

impl From<(ASN1Type, &str)> for ParameterizationArgument {
    fn from(value: (ASN1Type, &str)) -> Self {
        Self {
            dummy_reference: value.1.to_owned(),
            param_governor: ParameterGovernor::TypeOrClass(value.0),
        }
    }
}

impl From<(&str, &str)> for ParameterizationArgument {
    fn from(value: (&str, &str)) -> Self {
        Self {
            dummy_reference: value.1.to_owned(),
            param_governor: ParameterGovernor::Class(value.0.to_owned()),
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub enum ParameterGovernor {
    #[default]
    None,
    TypeOrClass(ASN1Type),
    Class(String),
}
