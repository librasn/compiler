use nom::{bytes::complete::tag, combinator::value};

use crate::intermediate::*;

use super::{common::skip_ws_and_comments, LexerResult, Span};

/// Tries to parse an ASN1 EXTERNAL
///
/// *`input` - string slice to be matched against
///
/// `external` will try to match an EXTERNAL declaration in the `input` string.
/// If the match succeeds, the lexer will consume the match and return the remaining string
/// and an `ASN1Type::External` value representing the ASN1 declaration.
/// If the match fails, the lexer will not consume the input and will return an error.
pub fn external(input: Span) -> LexerResult<ASN1Type> {
    value(ASN1Type::External, skip_ws_and_comments(tag(EMBEDDED_PDV)))(input)
}
