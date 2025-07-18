//! The `object_identifier` module contains lexers for
//! parsing ASN1 OBJECT IDENTIFIERs in a specification.
//! OBJECT IDENTIFIERs serve to uniquely and globally (really globally!)
//! identify a so-called _information object_.
use crate::{
    input::Input,
    intermediate::{ASN1Type, ObjectIdentifierArc, ObjectIdentifierValue, OBJECT_IDENTIFIER},
};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::u128,
    combinator::{into, map, opt},
    multi::many1,
    sequence::{pair, preceded},
    Parser,
};

use super::{
    common::{in_braces, in_parentheses, skip_ws, skip_ws_and_comments, value_reference},
    constraint::constraints,
    error::ParserResult,
    RELATIVE_OID,
};

/// Tries to parse an ASN1 OBJECT IDENTIFIER
/// As opposed to other ASN1 "types", an OBJECT IDENTIFIER is always a value.
/// In some places of an ASN1 spec, we do not encounter the `OBJECT IDENTIFIER` keyword
/// before an OBJECT IDENTIFIER value, such as in the header's module identifier.
///
/// *`input` - [Input]-wrapped string slice to be matched against
///
/// `object_identifier` will try to match an OBJECT IDENTIFIER declaration in the `input` string.
/// If the match succeeds, the lexer will consume the match and return the remaining string
/// and an `ObjectIdentifier` value representing the ASN1 declaration.
/// If the match fails, the lexer will not consume the input and will return an error.
pub fn object_identifier_value(input: Input<'_>) -> ParserResult<'_, ObjectIdentifierValue> {
    into(skip_ws_and_comments(preceded(
        // TODO: store info whether the object id is relative
        opt(alt((tag(OBJECT_IDENTIFIER), tag(RELATIVE_OID)))),
        in_braces(many1(skip_ws(object_identifier_arc))),
    )))
    .parse(input)
}

pub fn object_identifier(input: Input<'_>) -> ParserResult<'_, ASN1Type> {
    map(
        into(preceded(
            skip_ws_and_comments(alt((tag(OBJECT_IDENTIFIER), tag(RELATIVE_OID)))),
            opt(skip_ws_and_comments(constraints)),
        )),
        ASN1Type::ObjectIdentifier,
    )
    .parse(input)
}

fn object_identifier_arc(input: Input<'_>) -> ParserResult<'_, ObjectIdentifierArc> {
    skip_ws(alt((
        numeric_id,
        into(pair(value_reference, skip_ws(in_parentheses(u128)))),
        into(value_reference),
    )))
    .parse(input)
}

fn numeric_id(input: Input<'_>) -> ParserResult<'_, ObjectIdentifierArc> {
    map(u128, |i| i.into()).parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_ansi_x9_62() {
        assert_eq!(
            object_identifier_value(r#"{iso(1) member-body(2) us(840) 10045 modules(0) 2}"#.into())
                .unwrap()
                .1,
            ObjectIdentifierValue(vec![
                ObjectIdentifierArc {
                    name: Some("iso".into()),
                    number: Some(1)
                },
                ObjectIdentifierArc {
                    name: Some("member-body".into()),
                    number: Some(2)
                },
                ObjectIdentifierArc {
                    name: Some("us".into()),
                    number: Some(840)
                },
                ObjectIdentifierArc {
                    name: None,
                    number: Some(10045)
                },
                ObjectIdentifierArc {
                    name: Some("modules".into()),
                    number: Some(0)
                },
                ObjectIdentifierArc {
                    name: None,
                    number: Some(2)
                },
            ])
        )
    }
}
