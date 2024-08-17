use crate::lexer::Span;

use super::ASN1Type;

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

impl From<Span<'_>> for ParameterizationArgument {
    fn from(value: Span) -> Self {
        Self {
            dummy_reference: value.to_string(),
            param_governor: ParameterGovernor::None,
        }
    }
}

impl From<(ASN1Type, Span<'_>)> for ParameterizationArgument {
    fn from(value: (ASN1Type, Span)) -> Self {
        Self {
            dummy_reference: value.1.to_string(),
            param_governor: ParameterGovernor::TypeOrClass(value.0),
        }
    }
}

impl From<(Span<'_>, Span<'_>)> for ParameterizationArgument {
    fn from(value: (Span, Span)) -> Self {
        Self {
            dummy_reference: value.1.to_string(),
            param_governor: ParameterGovernor::Class(value.0.to_string()),
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
