use nom::{bytes::complete::tag, combinator::value, IResult};

use crate::intermediate::*;

use super::{common::skip_ws_and_comments, input::{with_parser, Input}};

/// Tries to parse an ASN1 EXTERNAL
///
/// *`input` - [Input]-wrapped string slice to be matched against
///
/// `external` will try to match an EXTERNAL declaration in the `input` string.
/// If the match succeeds, the lexer will consume the match and return the remaining string
/// and an `ASN1Type::External` value representing the ASN1 declaration.
/// If the match fails, the lexer will not consume the input and will return an error.
pub fn external(input: Input<'_>) -> IResult<Input<'_>, ASN1Type> {
    with_parser("ExternalType", value(ASN1Type::External, skip_ws_and_comments(tag(EXTERNAL))))(input)
}
