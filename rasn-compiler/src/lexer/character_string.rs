use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{char, u8},
    combinator::{map, map_res, opt},
    sequence::{delimited, pair, terminated, tuple},
};

use crate::{input::Input, intermediate::*};

use super::{
    common::*,
    constraint::constraint,
    error::{MiscError, ParserResult},
};

pub fn character_string_value(input: Input<'_>) -> ParserResult<'_, String> {
    skip_ws_and_comments(alt((
        map(
            into_inner(delimited(tag("\""), take_until("\""), tag("\""))),
            ToString::to_string,
        ),
        map(quadruple, |c| c.to_string()),
    )))(input)
}

/// A ASN1 character value can be specified "by reference to a registration number in the ISO
/// International Register of Coded Character Sets (see ISO International Register of Coded Character
/// Sets to be used with Escape Sequences), or by reference to ISO/IEC 10646." The registration number
/// can be expressed as a Quadruple of "Group", "Plane", "Row", and "Cell".
///
/// _41.12 The "number" in the "Plane", "Row" and "Cell" productions shall be_
/// _less than 256, and in the "Group" production it shall be less than 128._
///
/// _41.13 The "Group" specifies a group in the coding space of the UCS,_
/// _the "Plane" specifies a plane within the group, the "Row" specifies a_
/// _row within the plane, and the "Cell" specifies a cell within the row._
/// _The abstract character identified by this notation is the abstract character_
/// _for the cell specified by the "Group", "Plane", "Row", and "Cell" values._
/// _In all cases, the set of permitted characters may be restricted by subtyping._
///
/// __Currently, the rasn compiler only supports group `0`__
fn quadruple(input: Input<'_>) -> ParserResult<'_, char> {
    map_res(
        in_braces(tuple((
            terminated(skip_ws(u8), skip_ws(char(COMMA))),
            terminated(skip_ws(u8), skip_ws(char(COMMA))),
            terminated(skip_ws(u8), skip_ws(char(COMMA))),
            skip_ws(u8),
        ))),
        |(group, plane, row, cell)| {
            if group > 0 {
                Err(MiscError("Currently, only group 0 is supported."))
            } else {
                let code_point = (plane as u32) << 16 | (row as u32) << 8 | cell as u32;
                char::from_u32(code_point).ok_or(MiscError(
                    "Failed to determine character from code point quadruple.",
                ))
            }
        },
    )(input.clone())
}

/// Tries to parse an ASN1 Character String type
///
/// *`input` - [Input]-wrapped string slice to be matched against
///
/// `character_string` will try to match an Character String type declaration in the `input`
/// string, i.e. ASN1 types such as IA5String, UTF8String, VideotexString, but also
/// OCTET STRING, which is treated like a String and not a buffer.
/// If the match succeeds, the lexer will consume the match and return the remaining string
/// and a wrapped `CharacterString` value representing the ASN1 declaration.
/// If the match fails, the lexer will not consume the input and will return an error.
pub fn character_string(input: Input<'_>) -> ParserResult<'_, ASN1Type> {
    map(
        pair(
            into_inner(skip_ws_and_comments(alt((
                tag(IA5_STRING),
                tag(UTF8_STRING),
                tag(NUMERIC_STRING),
                tag(VISIBLE_STRING),
                tag(TELETEX_STRING),
                tag(VIDEOTEX_STRING),
                tag(GRAPHIC_STRING),
                tag(GENERAL_STRING),
                tag(UNIVERSAL_STRING),
                tag(BMP_STRING),
                tag(PRINTABLE_STRING),
            )))),
            opt(constraint),
        ),
        |m| ASN1Type::CharacterString(m.into()),
    )(input)
}

#[cfg(test)]
mod tests {
    use crate::intermediate::{constraints::*, types::*, *};

    use crate::lexer::{
        asn1_value,
        character_string::{character_string_value, quadruple},
    };

    use super::character_string;

    #[test]
    fn parses_unconfined_characterstring() {
        let sample = "   IA5String".into();
        assert_eq!(
            character_string(sample).unwrap().1,
            ASN1Type::CharacterString(CharacterString {
                constraints: vec![],
                ty: CharacterStringType::IA5String
            })
        )
    }

    #[test]
    fn parses_strictly_constrained_characterstring() {
        let sample = "   IA5String(SIZE (8))".into();
        assert_eq!(
            character_string(sample).unwrap().1,
            ASN1Type::CharacterString(CharacterString {
                constraints: vec![Constraint::SubtypeConstraint(ElementSet {
                    set: ElementOrSetOperation::Element(SubtypeElement::SizeConstraint(Box::new(
                        ElementOrSetOperation::Element(SubtypeElement::SingleValue {
                            value: ASN1Value::Integer(8),
                            extensible: false
                        })
                    ))),
                    extensible: false
                })],
                ty: CharacterStringType::IA5String
            })
        )
    }

    #[test]
    fn parses_range_constrained_characterstring() {
        let sample = "   IA5String -- even here?!?!? -- (SIZE (8 ..18))".into();
        assert_eq!(
            character_string(sample).unwrap().1,
            ASN1Type::CharacterString(CharacterString {
                constraints: vec![Constraint::SubtypeConstraint(ElementSet {
                    set: ElementOrSetOperation::Element(SubtypeElement::SizeConstraint(Box::new(
                        ElementOrSetOperation::Element(SubtypeElement::ValueRange {
                            min: Some(ASN1Value::Integer(8)),
                            max: Some(ASN1Value::Integer(18)),
                            extensible: false
                        })
                    ))),
                    extensible: false
                })],
                ty: CharacterStringType::IA5String
            })
        )
    }

    #[test]
    fn parses_strictly_constrained_extended_characterstring() {
        let sample = r#"  IA5String
        (SIZE (2, ...))"#
            .into();
        assert_eq!(
            character_string(sample).unwrap().1,
            ASN1Type::CharacterString(CharacterString {
                constraints: vec![Constraint::SubtypeConstraint(ElementSet {
                    set: ElementOrSetOperation::Element(SubtypeElement::SizeConstraint(Box::new(
                        ElementOrSetOperation::Element(SubtypeElement::SingleValue {
                            value: ASN1Value::Integer(2),
                            extensible: true
                        })
                    ))),
                    extensible: false
                })],
                ty: CharacterStringType::IA5String
            })
        )
    }

    #[test]
    fn parses_range_constrained_extended_characterstring() {
        let sample = "   IA5String (SIZE (8 --  comment -- .. 18, ...))".into();
        assert_eq!(
            character_string(sample).unwrap().1,
            ASN1Type::CharacterString(CharacterString {
                constraints: vec![Constraint::SubtypeConstraint(ElementSet {
                    set: ElementOrSetOperation::Element(SubtypeElement::SizeConstraint(Box::new(
                        ElementOrSetOperation::Element(SubtypeElement::ValueRange {
                            min: Some(ASN1Value::Integer(8)),
                            max: Some(ASN1Value::Integer(18)),
                            extensible: true
                        })
                    ))),
                    extensible: false
                })],
                ty: CharacterStringType::IA5String
            })
        )
    }

    #[test]
    fn parses_character_string_value() {
        assert_eq!(character_string_value("\"a\"".into()).unwrap().1, "a")
    }

    #[test]
    fn parses_character_string_asn1_value() {
        assert_eq!(
            asn1_value("\"a\"".into()).unwrap().1,
            ASN1Value::String("a".to_owned())
        )
    }

    #[test]
    fn parses_iso_10646_quadruple() {
        assert_eq!(quadruple("{0,0,0,9}".into()).unwrap().1, '\t');
        assert_eq!(quadruple("{0,0,0,10}".into()).unwrap().1, '\n');
        assert_eq!(quadruple("{0,0,0,13}".into()).unwrap().1, '\r');
        assert_eq!(quadruple("{0,0,0,32}".into()).unwrap().1, ' ');
        assert_eq!(quadruple("{0,0,215,23}".into()).unwrap().1, '휗');
        assert_eq!(quadruple("{0,0,249,0}".into()).unwrap().1, '豈');
        assert_eq!(quadruple("{0,1,0,0}".into()).unwrap().1, '𐀀');
    }
}
