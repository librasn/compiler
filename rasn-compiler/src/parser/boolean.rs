use nom::{bytes::complete::tag, combinator::value, IResult, branch::alt};

use crate::intermediate::{ASN1Type, ASN1Value, BOOLEAN, FALSE, TRUE};

use super::common::skip_ws_and_comments;

pub fn boolean_value<'a>(input: &'a str) -> IResult<&'a str, ASN1Value> {
    alt((
        value(ASN1Value::Boolean(true), skip_ws_and_comments(tag(TRUE))),
        value(ASN1Value::Boolean(false), skip_ws_and_comments(tag(FALSE))),
    ))(input)
}

/// Tries to parse an ASN1 BOOLEAN
/// 
/// *`input` - string slice to be matched against
/// 
/// `boolean` will try to match an BOOLEAN declaration in the `input` string.
/// If the match succeeds, the parser will consume the match and return the remaining string
/// and an `ASN1Type::Boolean` value representing the ASN1 declaration.
/// If the match fails, the parser will not consume the input and will return an error.
pub fn boolean<'a>(input: &'a str) -> IResult<&'a str, ASN1Type> {
    value(ASN1Type::Boolean, skip_ws_and_comments(tag(BOOLEAN)))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_boolean() {
        assert_eq!(
            boolean(" --who would put a comment here?--BOOLEAN")
                .unwrap()
                .1,
            ASN1Type::Boolean
        )
    }
}
