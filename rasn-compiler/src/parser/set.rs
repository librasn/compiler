use nom::{
    bytes::complete::tag,
    character::complete::char,
    combinator::opt,
    multi::many0,
    sequence::{terminated, tuple},
    IResult,
};

use crate::intermediate::*;

use super::{common::optional_comma, constraint::constraint, *, sequence::sequence_or_set_member};

/// Tries to parse an ASN1 SET
///
/// *`input` - string slice to be matched against
///
/// `set` will try to match an SET declaration in the `input` string.
/// If the match succeeds, the parser will consume the match and return the remaining string
/// and a wrapped `Set` value representing the ASN1 declaration. If the defined SET
/// contains anonymous built-in types as members, these nested built-in types will be represented as
/// structs within the same global scope.
/// If the match fails, the parser will not consume the input and will return an error.
pub fn set<'a>(input: &'a str) -> IResult<&'a str, ASN1Type> {
    map(
        preceded(
            skip_ws_and_comments(tag(SET)),
            pair(
                in_braces(tuple((
                    many0(terminated(
                        skip_ws_and_comments(sequence_or_set_member),
                        optional_comma,
                    )),
                    opt(terminated(extension_marker, opt(char(COMMA)))),
                    opt(many0(terminated(
                        skip_ws_and_comments(sequence_or_set_member),
                        optional_comma,
                    ))),
                ))),
                opt(constraint),
            ),
        ),
        |m| ASN1Type::Set(m.into()),
    )(input)
}

#[cfg(test)]
mod tests {
    }
