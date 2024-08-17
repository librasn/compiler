use nom::{bytes::complete::tag, combinator::value};

use crate::intermediate::{ASN1Type, ASN1Value, NULL};

use super::{common::skip_ws_and_comments, LexerResult, Span};

pub fn null_value(input: Span) -> LexerResult<ASN1Value> {
    value(ASN1Value::Null, skip_ws_and_comments(tag(NULL)))(input)
}

/// Tries to parse an ASN1 NULL
///
/// *`input` - string slice to be matched against
///
/// `null` will try to match an NULL declaration in the `input` string.
/// If the match succeeds, the lexer will consume the match and return the remaining string
/// and an `ASN1Type::Null` value representing the ASN1 declaration.
/// If the match fails, the lexer will not consume the input and will return an error.
pub fn null(input: Span) -> LexerResult<ASN1Type> {
    value(ASN1Type::Null, skip_ws_and_comments(tag(NULL)))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_null() {
        assert_eq!(
            null(Span::new(" --who would put a comment here?--NULL"))
                .unwrap()
                .1,
            ASN1Type::Null
        )
    }
}
