use nom::{
    bytes::complete::tag,
    character::complete::i128,
    combinator::{map, opt},
    sequence::tuple,
    IResult,
};

use crate::intermediate::{ASN1Type, ASN1Value, INTEGER};

use super::{constraint::*, *};

pub fn integer_value<'a>(input: &'a str) -> IResult<&'a str, ASN1Value> {
    map(skip_ws_and_comments(i128), |m| ASN1Value::Integer(m))(input)
}

/// Tries to parse an ASN1 INTEGER
///
/// *`input` - string slice to be matched against
///
/// `integer` will try to match an INTEGER declaration in the `input` string.
/// If the match succeeds, the lexer will consume the match and return the remaining string
/// and a wrapped `Integer` value representing the ASN1 declaration.
/// If the match fails, the lexer will not consume the input and will return an error.
pub fn integer<'a>(input: &'a str) -> IResult<&'a str, ASN1Type> {
    map(
        tuple((
            skip_ws_and_comments(tag(INTEGER)),
            opt(skip_ws_and_comments(distinguished_values)),
            opt(skip_ws_and_comments(constraint)),
        )),
        |m| ASN1Type::Integer(m.into()),
    )(input)
}

#[cfg(test)]
mod tests {

    use crate::intermediate::{constraints::*, types::*, *};

    use super::*;

    #[test]
    fn parses_integer() {
        assert_eq!(
            integer("INTEGER"),
            Ok(("", ASN1Type::Integer(Integer::default())))
        );
        assert_eq!(
            integer("INTEGER  (-9..-4, ...)").unwrap().1,
            ASN1Type::Integer(Integer {
                constraints: vec![Constraint::SubtypeConstraint(ElementSet {
                    set: ElementOrSetOperation::Element(SubtypeElement::ValueRange {
                        min: Some(ASN1Value::Integer(-9)),
                        max: Some(ASN1Value::Integer(-4)),
                        extensible: true
                    }),
                    extensible: false
                })],
                distinguished_values: None,
            })
        );
        assert_eq!(
            integer("\r\nINTEGER(-9..-4)").unwrap().1,
            ASN1Type::Integer(Integer {
                constraints: vec![Constraint::SubtypeConstraint(ElementSet {
                    set: ElementOrSetOperation::Element(SubtypeElement::ValueRange {
                        min: Some(ASN1Value::Integer(-9)),
                        max: Some(ASN1Value::Integer(-4)),
                        extensible: false
                    }),
                    extensible: false
                })],
                distinguished_values: None,
            })
        );
    }
}
