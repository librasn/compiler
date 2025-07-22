use std::borrow::Cow;

use super::ASN1Type;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Parameterization<'a> {
    pub parameters: Vec<ParameterizationArgument<'a>>,
}

impl<'a> From<Vec<ParameterizationArgument<'a>>> for Parameterization<'a> {
    fn from(value: Vec<ParameterizationArgument<'a>>) -> Self {
        Self { parameters: value }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParameterizationArgument<'a> {
    pub dummy_reference: Cow<'a, str>,
    pub param_governor: ParameterGovernor<'a>,
}

impl<'a> From<&'a str> for ParameterizationArgument<'a> {
    fn from(value: &'a str) -> Self {
        Self {
            dummy_reference: Cow::Borrowed(value),
            param_governor: ParameterGovernor::None,
        }
    }
}

impl<'a> From<(ASN1Type<'a>, &'a str)> for ParameterizationArgument<'a> {
    fn from(value: (ASN1Type<'a>, &'a str)) -> Self {
        Self {
            dummy_reference: Cow::Borrowed(value.1),
            param_governor: ParameterGovernor::TypeOrClass(value.0),
        }
    }
}

impl<'a> From<(&'a str, &'a str)> for ParameterizationArgument<'a> {
    fn from(value: (&'a str, &'a str)) -> Self {
        Self {
            dummy_reference: Cow::Borrowed(value.1),
            param_governor: ParameterGovernor::Class(value.0.to_owned()),
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub enum ParameterGovernor<'a> {
    #[default]
    None,
    TypeOrClass(ASN1Type<'a>),
    Class(String),
}
