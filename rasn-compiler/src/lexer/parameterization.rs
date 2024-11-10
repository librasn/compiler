use nom::{
    branch::alt,
    character::complete::char,
    combinator::{into, map},
    multi::separated_list1,
    sequence::separated_pair,
};

use crate::{
    input::Input,
    intermediate::{constraints::Parameter, parameterization::*, *},
};

use super::{
    asn1_type, asn1_value,
    common::{identifier, in_braces, skip_ws_and_comments},
    error::ParserResult,
    information_object_class::{information_object, object_set},
    value_identifier,
};

pub fn parameterization(input: Input<'_>) -> ParserResult<'_, Parameterization> {
    into(in_braces(separated_list1(
        char(COMMA),
        skip_ws_and_comments(alt((
            into(separated_pair(
                asn1_type,
                skip_ws_and_comments(char(COLON)),
                skip_ws_and_comments(value_identifier),
            )),
            into(skip_ws_and_comments(value_identifier)),
            into(skip_ws_and_comments(separated_pair(
                identifier,
                skip_ws_and_comments(char(COLON)),
                skip_ws_and_comments(identifier),
            ))),
            into(skip_ws_and_comments(identifier)),
        ))),
    )))(input)
}

pub fn parameters(input: Input<'_>) -> ParserResult<'_, Vec<Parameter>> {
    into(in_braces(separated_list1(
        char(COMMA),
        skip_ws_and_comments(alt((
            map(asn1_value, Parameter::ValueParameter),
            map(asn1_type, Parameter::TypeParameter),
            map(object_set, Parameter::ObjectSetParameter),
            map(information_object, |o| {
                Parameter::InformationObjectParameter(o)
            }),
        ))),
    )))(input)
}

#[cfg(test)]
mod tests {
    use crate::intermediate::{
        constraints::Parameter,
        information_object::{ObjectSet, ObjectSetValue},
        parameterization::{ParameterGovernor, Parameterization, ParameterizationArgument},
        types::{Boolean, Integer},
        ASN1Type,
    };

    use crate::lexer::parameterization::{parameterization, parameters};

    #[test]
    fn parses_class_parameterization_param() {
        assert_eq!(
            parameterization(r#"{REG-EXT-ID-AND-TYPE : Set}"#.into())
                .unwrap()
                .1,
            Parameterization {
                parameters: vec![ParameterizationArgument {
                    dummy_reference: "Set".into(),
                    param_governor: ParameterGovernor::Class("REG-EXT-ID-AND-TYPE".into(),)
                }]
            }
        )
    }

    #[test]
    fn parses_object_set_parameter() {
        assert_eq!(
            parameters("{{Reg-MapData}}".into()).unwrap().1,
            vec![Parameter::ObjectSetParameter(ObjectSet {
                values: vec![ObjectSetValue::Reference("Reg-MapData".into())],
                extensible: None
            })]
        )
    }

    #[test]
    fn parses_builtin_type_params() {
        assert_eq!(
            parameterization(r#"{ INTEGER: lower, BOOLEAN: flag }"#.into())
                .unwrap()
                .1,
            Parameterization {
                parameters: vec![
                    ParameterizationArgument {
                        dummy_reference: "lower".to_owned(),
                        param_governor: ParameterGovernor::TypeOrClass(ASN1Type::Integer(
                            Integer::default()
                        ))
                    },
                    ParameterizationArgument {
                        dummy_reference: "flag".to_owned(),
                        param_governor: ParameterGovernor::TypeOrClass(ASN1Type::Boolean(
                            Boolean::default()
                        ))
                    }
                ]
            }
        )
    }
}
