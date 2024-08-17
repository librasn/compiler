use nom::{bytes::complete::tag, combinator::value};

use crate::intermediate::*;

use super::{common::skip_ws_and_comments, LexerResult, Span};

/// Tries to parse an ASN1 EMBEDDED PDV
///
/// *`input` - string slice to be matched against
///
/// `embedded_pdv` will try to match an EMBEDDED PDV declaration in the `input` string.
/// If the match succeeds, the lexer will consume the match and return the remaining string
/// and an `ASN1Type::EmbeddedPdv` value representing the ASN1 declaration.
/// If the match fails, the lexer will not consume the input and will return an error.
/// ##_X680_:
/// _The term "Embedded PDV" means an abstract value from a possibly_
/// _different abstract syntax (essentially, the value and encoding_
/// _of a message defined in a separate – and identified – protocol)_
/// _that is embedded in a message.  Historically, it meant_
/// _"Embedded Presentation Data Value" from its use in the OSI_
/// _Presentation Layer, but this expansion is not used today,_
/// _and it should be interpreted as "embedded value"._
pub fn embedded_pdv(input: Span) -> LexerResult<ASN1Type> {
    value(
        ASN1Type::EmbeddedPdv,
        skip_ws_and_comments(tag(EMBEDDED_PDV)),
    )(input)
}
