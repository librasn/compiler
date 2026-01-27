//! The `object_identifier` module contains lexers for
//! parsing ASN1 OBJECT IDENTIFIERs in a specification.
//! OBJECT IDENTIFIERs serve to uniquely and globally (really globally!)
//! identify a so-called _information object_.
use crate::{
    input::Input,
    intermediate::{
        types::{OidIri, RelativeOidIri},
        ASN1Type, ObjectIdentifierArc, ObjectIdentifierValue, OBJECT_IDENTIFIER, OID_IRI,
        RELATIVE_OID_IRI,
    },
};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, u128},
    combinator::{into, map, not, opt},
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

/// Parse an `ObjectIdentifierValue` or an `RelativeOIDValue`.
///
/// Syntax:
///
/// ```text
/// ObjectIdentifierValue ::=
///     "{" ObjIdComponentsList "}"  |
///     "{" DefinedValue ObjIdComponentsList "}"
///
/// ObjIdComponentsList ::=
///     ObjIdComponents  |
///     ObjIdComponents ObjIdComponentsList
///
/// RelativeOIDValue ::=
///     "{" RelativeOIDComponentsList "}"
///
/// RelativeOIDComponentsList ::=
///     RelativeOIDComponents  |
///     RelativeOIDComponents RelativeOIDComponentsList
/// ```
pub fn object_identifier_value(input: Input<'_>) -> ParserResult<'_, ObjectIdentifierValue> {
    into(skip_ws_and_comments(in_braces(many1(skip_ws(
        object_identifier_arc,
    )))))
    .parse(input)
}

/// Parse an `ObjectIdentifierType` or an `RelativeOIDType`.
///
/// Syntax:
///
/// ```text
/// ObjectIdentifierType ::=
///     OBJECT IDENTIFIER
///
/// RelativeOIDType ::=
///     RELATIVE-OID
/// ```
pub fn object_identifier(input: Input<'_>) -> ParserResult<'_, ASN1Type> {
    map(
        into(preceded(
            // TODO: store info whether the object id is relative
            skip_ws_and_comments(alt((tag(OBJECT_IDENTIFIER), tag(RELATIVE_OID)))),
            opt(skip_ws_and_comments(constraints)),
        )),
        ASN1Type::ObjectIdentifier,
    )
    .parse(input)
}

/// Parse an `ObjIdComponents ` or an `RelativeOIDComponents`.
///
/// Syntax:
///
/// ```text
/// ObjIdComponents ::=
///     NameForm          |
///     NumberForm        |
///     NameAndNumberForm |
///     DefinedValue
///
/// NameForm ::=
///     identifier
///
/// NumberForm ::=
///     number  |
///     DefinedValue
///
/// NameAndNumberForm ::=
///     identifier "(" NumberForm ")"
///
/// RelativeOIDComponents ::=
///     NumberForm        |
///     NameAndNumberForm |
///     DefinedValue
/// ```
fn object_identifier_arc(input: Input<'_>) -> ParserResult<'_, ObjectIdentifierArc> {
    skip_ws(alt((
        numeric_id,
        into(pair(value_reference, skip_ws(in_parentheses(u128)))),
        into(value_reference),
    )))
    .parse(input)
}

/// Parse a `NumberForm`.
///
/// Syntax:
///
/// ```text
/// NumberForm ::=
///     number  |
///     DefinedValue
/// ```
///
/// Note: The `DefinedValue` alternative is not parsed here, since it is already parsed in
/// `object_identifier_arc`.
fn numeric_id(input: Input<'_>) -> ParserResult<'_, ObjectIdentifierArc> {
    map(u128, |i| i.into()).parse(input)
}

/// Parse a `IRIType`.
///
/// Syntax:
///
/// ```text
/// IRIType ::=
///     OID-IRI
/// ```
pub fn iri_type(input: Input<'_>) -> ParserResult<'_, OidIri> {
    map(
        preceded(
            (tag(OID_IRI), not(alt((tag("-"), alphanumeric1)))),
            opt(skip_ws_and_comments(constraints)),
        ),
        |constraints| OidIri {
            constraints: constraints.unwrap_or_default(),
        },
    )
    .parse(input)
}

/// Parse a `RelativeIRIType`.
///
/// Syntax:
///
/// ```text
/// RelativeIRIType ::=
///     RELATIVE-OID-IRI
///
/// ```
pub fn relative_iri_type(input: Input<'_>) -> ParserResult<'_, RelativeOidIri> {
    map(
        preceded(
            (tag(RELATIVE_OID_IRI), not(alt((tag("-"), alphanumeric1)))),
            opt(skip_ws_and_comments(constraints)),
        ),
        |constraints| RelativeOidIri {
            constraints: constraints.unwrap_or_default(),
        },
    )
    .parse(input)
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

    #[test]
    fn parses_absolute_oid_iri_type() {
        let input = Input::from("OID-IRI");

        let (rest, result) = iri_type(input).unwrap();

        assert_eq!(rest.inner(), "");
        assert_eq!(
            result,
            OidIri {
                constraints: Vec::new()
            }
        );
    }

    #[test]
    fn parses_relative_oid_iri_type() {
        let input = Input::from("RELATIVE-OID-IRI");

        let (rest, result) = relative_iri_type(input).unwrap();

        assert_eq!(rest.inner(), "");
        assert_eq!(
            result,
            RelativeOidIri {
                constraints: Vec::new()
            }
        );
    }
}
