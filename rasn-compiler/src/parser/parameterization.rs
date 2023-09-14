use nom::{
    branch::alt,
    character::complete::char,
    combinator::{into, map, opt},
    multi::separated_list1,
    sequence::{pair, preceded},
    IResult,
};

use crate::intermediate::{constraints::Parameter, parameterization::*, *};

use super::{
    asn1_type, asn1_value,
    common::{identifier, in_braces, skip_ws_and_comments},
    information_object_class::{information_object, object_set},
};

pub fn parameterization<'a>(input: &'a str) -> IResult<&'a str, Parameterization> {
    into(in_braces(separated_list1(
        char(COMMA),
        skip_ws_and_comments(pair(
            identifier,
            opt(preceded(
                skip_ws_and_comments(char(COLON)),
                skip_ws_and_comments(identifier),
            )),
        )),
    )))(input)
}

pub fn parameters<'a>(input: &'a str) -> IResult<&'a str, Vec<Parameter>> {
    into(in_braces(separated_list1(
        char(COMMA),
        skip_ws_and_comments(alt((
            map(asn1_type, |t| Parameter::TypeParameter(t)),
            map(asn1_value, |v| Parameter::ValueParameter(v)),
            map(object_set, |o| Parameter::ObjectSetParameter(o)),
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
        parameterization::{Parameterization, ParameterizationArgument},
    };

    use crate::parser::parameterization::{parameterization, parameters};

    #[test]
    fn parses_class_parameterization_param() {
        assert_eq!(
            parameterization(r#"{REG-EXT-ID-AND-TYPE : Set}"#)
                .unwrap()
                .1,
            Parameterization {
                parameters: vec![ParameterizationArgument {
                    r#type: "REG-EXT-ID-AND-TYPE".into(),
                    name: Some("Set".into())
                }]
            }
        )
    }

    #[test]
    fn parses_object_set_parameter() {
        assert_eq!(
            parameters("{{Reg-MapData}}").unwrap().1,
            vec![Parameter::ObjectSetParameter(ObjectSet {
                values: vec![ObjectSetValue::Reference("Reg-MapData".into())],
                extensible: None
            })]
        )
    }
}
