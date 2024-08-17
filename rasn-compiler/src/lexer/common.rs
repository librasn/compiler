use nom::{
    branch::alt,
    bytes::complete::{is_not, tag},
    character::complete::{
        alpha1, alphanumeric1, char, i128, multispace0, multispace1, one_of, u64,
    },
    combinator::{into, map_res, opt, peek, recognize, value},
    error::Error,
    multi::{many0, many1},
    sequence::{delimited, pair, preceded, terminated},
};

use crate::intermediate::{constraints::*, types::*, *};

use super::{
    asn1_value,
    util::{opt_delimited, take_until_or, take_until_unbalanced},
    LexerResult, Span,
};

/// Parses an ASN1 comment.
///
/// * `input` string slice reference used as an input for the lexer
///
/// returns a `Result` yielding a tuple containing a reference to the remaining string slice
/// and the parsed comment in case of sucess, or a parsing error if unsuccessful.
///
/// #### X680
/// _The lexical item "comment" can have two forms:_
///    * _One-line comments which begin with "--" as defined in 12.6.3;_
///    * _Multiple-line comments which begin with "/*" as defined in 12.6.4._
pub fn comment(input: Span) -> LexerResult<Span> {
    skip_ws(alt((block_comment, line_comment)))(input)
}

pub fn line_comment(input: Span) -> LexerResult<Span> {
    delimited(
        tag(LINE_COMMENT),
        take_until_or("\n", LINE_COMMENT),
        opt(tag(LINE_COMMENT)),
    )(input)
}

pub fn block_comment(input: Span) -> LexerResult<Span> {
    delimited(
        tag(BLOCK_COMMENT_START),
        take_until_unbalanced(BLOCK_COMMENT_START, BLOCK_COMMENT_END),
        tag(BLOCK_COMMENT_END),
    )(input)
}

/// Parses an ASN1 identifier.
///
/// * `input` string slice reference used as an input for the lexer
///
/// returns a `Result` yielding a tuple containing a reference to the remaining string slice
/// and the parsed identifier in case of sucess, or a parsing error if unsuccessful.
///
/// #### X.680
/// _12.3 An "identifier" shall consist of an arbitrary number (one or more) of letters, digits,
/// and hyphens. The initial character shall be a lower-case letter. A hyphen shall not be the
/// last character. A hyphen shall not be immediately followed by another hyphen._
pub fn identifier(input: Span) -> LexerResult<Span> {
    skip_ws_and_comments(recognize(pair(
        alpha1,
        many0(alt((preceded(char('-'), alphanumeric1), alphanumeric1))),
    )))(input)
}

pub fn title_case_identifier(input: Span) -> LexerResult<Span> {
    map_res(
        recognize(pair(
            one_of("ABCDEFGHIJKLMNOPQRSTUVWXYZ01234567890"),
            many0(alt((preceded(char('-'), alphanumeric1), alphanumeric1))),
        )),
        |identifier: Span| {
            if ASN1_KEYWORDS.contains(&*identifier) {
                Err(nom::Err::Error(Error {
                    input,
                    code: nom::error::ErrorKind::Tag,
                }))
            } else {
                Ok(identifier)
            }
        },
    )(input)
}

pub fn value_identifier(input: Span) -> LexerResult<Span> {
    recognize(pair(
        one_of("abcdefghijklmnopqrstuvwxyz"),
        many0(alt((preceded(char('-'), alphanumeric1), alphanumeric1))),
    ))(input)
}

pub fn skip_ws<'a, F, O>(inner: F) -> impl FnMut(Span<'a>) -> LexerResult<'a, O>
where
    F: FnMut(Span<'a>) -> LexerResult<'a, O>,
{
    preceded(multispace0, inner)
}

pub fn skip_ws_and_comments<'a, F, O>(inner: F) -> impl FnMut(Span<'a>) -> LexerResult<'a, O>
where
    F: FnMut(Span<'a>) -> LexerResult<'a, O>,
{
    preceded(many0(alt((comment, multispace1))), inner)
}

pub fn in_parentheses<'a, F, O>(inner: F) -> impl FnMut(Span<'a>) -> LexerResult<'a, O>
where
    F: FnMut(Span<'a>) -> LexerResult<'a, O>,
{
    delimited(
        skip_ws_and_comments(char(LEFT_PARENTHESIS)),
        skip_ws_and_comments(inner),
        skip_ws_and_comments(char(RIGHT_PARENTHESIS)),
    )
}

pub fn in_brackets<'a, F, O>(inner: F) -> impl FnMut(Span<'a>) -> LexerResult<'a, O>
where
    F: FnMut(Span<'a>) -> LexerResult<'a, O>,
{
    delimited(
        skip_ws_and_comments(char(LEFT_BRACKET)),
        skip_ws_and_comments(inner),
        skip_ws_and_comments(char(RIGHT_BRACKET)),
    )
}

pub fn in_version_brackets<'a, F, O>(inner: F) -> impl FnMut(Span<'a>) -> LexerResult<'a, O>
where
    F: FnMut(Span<'a>) -> LexerResult<'a, O>,
{
    delimited(
        skip_ws_and_comments(tag("[[")),
        skip_ws_and_comments(inner),
        skip_ws_and_comments(tag("]]")),
    )
}

pub fn opt_parentheses<'a, F, O>(inner: F) -> impl FnMut(Span<'a>) -> LexerResult<'a, O>
where
    F: FnMut(Span<'a>) -> LexerResult<'a, O>,
{
    opt_delimited::<char, O, char, _, _, _>(
        skip_ws_and_comments(char(LEFT_BRACKET)),
        skip_ws_and_comments(inner),
        skip_ws_and_comments(char(RIGHT_BRACKET)),
    )
}

pub fn in_braces<'a, F, O>(inner: F) -> impl FnMut(Span<'a>) -> LexerResult<'a, O>
where
    F: FnMut(Span<'a>) -> LexerResult<'a, O>,
{
    delimited(
        skip_ws_and_comments(char(LEFT_BRACE)),
        skip_ws_and_comments(inner),
        skip_ws_and_comments(char(RIGHT_BRACE)),
    )
}

pub fn all_value(input: Span) -> LexerResult<ASN1Value> {
    value(ASN1Value::All, skip_ws_and_comments(tag(ALL)))(input)
}

pub fn asn_tag(input: Span) -> LexerResult<AsnTag> {
    into(pair(
        in_brackets(pair(
            opt(skip_ws_and_comments(alt((
                tag(PRIVATE),
                tag(APPLICATION),
                tag(UNIVERSAL),
            )))),
            skip_ws_and_comments(u64),
        )),
        skip_ws_and_comments(opt(alt((
            value(TaggingEnvironment::Explicit, tag(EXPLICIT)),
            value(TaggingEnvironment::Implicit, tag(IMPLICIT)),
        )))),
    ))(input)
}

pub fn range_seperator(input: Span) -> LexerResult<RangeSeperator> {
    skip_ws_and_comments(tag(RANGE))(input).map(|(remaining, _)| (remaining, RangeSeperator()))
}

pub fn extension_marker(input: Span) -> LexerResult<ExtensionMarker> {
    skip_ws_and_comments(tag(ELLIPSIS))(input).map(|(remaining, _)| (remaining, ExtensionMarker()))
}

pub fn assignment(input: Span) -> LexerResult<Span> {
    skip_ws_and_comments(tag(ASSIGN))(input)
}

pub fn distinguished_values(input: Span) -> LexerResult<Vec<DistinguishedValue>> {
    delimited(
        skip_ws_and_comments(char(LEFT_BRACE)),
        many0(terminated(
            skip_ws_and_comments(distinguished_val),
            opt(skip_ws_and_comments(char(COMMA))),
        )),
        skip_ws_and_comments(char(RIGHT_BRACE)),
    )(input)
}

pub fn distinguished_val(input: Span) -> LexerResult<DistinguishedValue> {
    into(pair(skip_ws_and_comments(identifier), in_parentheses(i128)))(input)
}

pub fn optional_comma(input: Span) -> LexerResult<Option<char>> {
    skip_ws_and_comments(opt(char(COMMA)))(input)
}

pub fn optional_marker(input: Span) -> LexerResult<Option<OptionalMarker>> {
    opt(into(skip_ws_and_comments(tag(OPTIONAL))))(input)
}

pub fn default(input: Span) -> LexerResult<Option<ASN1Value>> {
    opt(preceded(
        skip_ws_and_comments(tag(DEFAULT)),
        skip_ws_and_comments(asn1_value),
    ))(input)
}

pub fn uppercase_identifier(input: Span) -> LexerResult<Span> {
    alt((
        recognize(pair(
            one_of("ABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890"),
            many1(alt((
                preceded(char('-'), one_of("ABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890")),
                one_of("ABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890"),
            ))),
        )),
        terminated(
            recognize(one_of("ABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890")),
            peek(is_not("abcdefghijklmnopqrstuvwxyz-")),
        ),
    ))(input)
}

#[cfg(test)]
mod tests {
    use nom::{error::ErrorKind, Err, Slice as _};

    use super::*;

    #[test]
    fn parses_line_comment() {
        let line = Span::new(
            r#"-- Test, one, two, three/
"#,
        );
        assert_eq!(" Test, one, two, three/", *comment(line).unwrap().1);
    }

    #[test]
    fn parses_block_comment() {
        assert_eq!(
            r#" Test, one, two, three
and one "#,
            *comment(Span::new(
                r#"/* Test, one, two, three
and one */"#
            ))
            .unwrap()
            .1
        );
        assert_eq!(
            r#"*
       * Hello
       "#,
            *comment(Span::new(
                r#"/**
       * Hello
       */"#
            ))
            .unwrap()
            .1
        );
        assert_eq!(
            " Very annoying! ",
            *comment(Span::new("-- Very annoying! --")).unwrap().1
        )
    }

    #[test]
    fn parses_multiline_block_comment() {
        assert_eq!(*comment(Span::new(r#"/**
      * This DE indicates a change of acceleration.
      *
      * The value shall be set to:
      * - 0 - `accelerate` - if the magnitude of the horizontal velocity vector increases.
      * - 1 - `decelerate` - if the magnitude of the horizontal velocity vector decreases.
      *
      * @category: Kinematic information
      * @revision: Created in V2.1.1
     */
    StartOfDeclaration"#)).unwrap().1,
      "*\n      * This DE indicates a change of acceleration.\n      *\n      * The value shall be set to:\n      * - 0 - `accelerate` - if the magnitude of the horizontal velocity vector increases.\n      * - 1 - `decelerate` - if the magnitude of the horizontal velocity vector decreases.\n      *\n      * @category: Kinematic information\n      * @revision: Created in V2.1.1\n     "
    )
    }

    /// 12.6.4. Whenever a "comment" begins with "/*", it shall end with a corresponding "*/",
    /// whether this "*/" is on the same line or not. If another "/*" is found before a "*/",
    /// then the comment terminates when a matching "*/" has been found for each "/*".
    #[test]
    fn parses_block_comment_with_nested_comments() {
        assert_eq!(
            *comment(Span::new(
                r#"/*this is a comment /*
        this is a nested comment */ this text should be parsed*/"#
            ))
            .unwrap()
            .1,
            "this is a comment /*
        this is a nested comment */ this text should be parsed"
        )
    }

    #[test]
    fn parses_ambiguous_asn1_comment() {
        {
            let input = Span::new(" -- This means backward\n      unavailable");
            let expect_rest = input.slice(23..);
            let expect_value = input.slice(3..23);
            assert_eq!(*expect_rest, "\n      unavailable");
            assert_eq!(*expect_value, " This means backward");
            assert_eq!(comment(input), Ok((expect_rest, expect_value)));
        }
        {
            let input =
                Span::new("-- This means forward\n        backward    (2), -- This means backward");
            let expect_rest = input.slice(21..);
            let expect_value = input.slice(2..21);
            assert_eq!(
                *expect_rest,
                "\n        backward    (2), -- This means backward"
            );
            assert_eq!(*expect_value, " This means forward");
            assert_eq!(comment(input), Ok((expect_rest, expect_value)))
        }
    }

    #[test]
    fn parses_valid_identifiers() {
        {
            let input = Span::new("EEE-DDD");
            let expect_rest = input.slice(7..);
            let expect_value = input.slice(0..7);
            assert_eq!(*expect_rest, "");
            assert_eq!(*expect_value, "EEE-DDD");
            assert_eq!(identifier(input), Ok((expect_rest, expect_value)));
        }
        {
            let input = Span::new("GenericLane ");
            let expect_rest = input.slice(11..);
            let expect_value = input.slice(0..11);
            assert_eq!(*expect_rest, " ");
            assert_eq!(*expect_value, "GenericLane");
            assert_eq!(identifier(input), Ok((expect_rest, expect_value)));
        }
        {
            let input = Span::new("regional ");
            let expect_rest = input.slice(8..);
            let expect_value = input.slice(0..8);
            assert_eq!(*expect_rest, " ");
            assert_eq!(*expect_value, "regional");
            assert_eq!(identifier(input), Ok((expect_rest, expect_value)));
        }
        {
            let input = Span::new("NodeXY64");
            let expect_rest = input.slice(8..);
            let expect_value = input.slice(0..8);
            assert_eq!(*expect_rest, "");
            assert_eq!(*expect_value, "NodeXY64");
            assert_eq!(identifier(input), Ok((expect_rest, expect_value)));
        }
        {
            let input = Span::new("Sub-Cause-Code  ");
            let expect_rest = input.slice(14..);
            let expect_value = input.slice(0..14);
            assert_eq!(*expect_rest, "  ");
            assert_eq!(*expect_value, "Sub-Cause-Code");
            assert_eq!(identifier(input), Ok((expect_rest, expect_value)));
        }
    }

    #[test]
    fn handles_invalid_identifiers() {
        {
            let input = Span::new("EEE--DDD");
            let expect_rest = input.slice(3..);
            let expect_value = input.slice(0..3);
            assert_eq!(*expect_rest, "--DDD");
            assert_eq!(*expect_value, "EEE");
            assert_eq!(identifier(input), Ok((expect_rest, expect_value)));
        }
        {
            let input = Span::new("-GenericLane");
            let expect_rest = input.slice(0..);
            assert_eq!(*expect_rest, "-GenericLane");
            assert_eq!(
                identifier(input),
                Err(Err::Error(Error {
                    input: expect_rest,
                    code: ErrorKind::Alpha
                }))
            );
        }
        {
            let input = Span::new("64NodeXY");
            let expect_rest = input.slice(0..);
            assert_eq!(*expect_rest, "64NodeXY");
            assert_eq!(
                identifier(Span::new("64NodeXY")),
                Err(Err::Error(Error {
                    input: expect_rest,
                    code: ErrorKind::Alpha
                }))
            );
        }
        {
            let input = Span::new("&regional");
            let expect_rest = input.slice(0..);
            assert_eq!(*expect_rest, "&regional");
            assert_eq!(
                identifier(Span::new("&regional")),
                Err(Err::Error(Error {
                    input: expect_rest,
                    code: ErrorKind::Alpha
                }))
            );
        }
        {
            let input = Span::new("Sub-Cause-Code-");
            let expect_rest = input.slice(14..);
            let expect_value = input.slice(0..14);
            assert_eq!(*expect_rest, "-");
            assert_eq!(*expect_value, "Sub-Cause-Code");
            assert_eq!(identifier(input), Ok((expect_rest, expect_value)));
        }
    }

    #[test]
    fn discards_whitespace() {
        {
            let input = Span::new(" EEE-DDD");
            let expect_rest = input.slice(8..);
            let expect_value = input.slice(1..8);
            assert_eq!(*expect_rest, "");
            assert_eq!(*expect_value, "EEE-DDD");
            assert_eq!(skip_ws(identifier)(input), Ok((expect_rest, expect_value)));
        }
        {
            let input = Span::new("\nGenericLane ");
            let expect_rest = input.slice(12..);
            let expect_value = input.slice(1..12);
            assert_eq!(*expect_rest, " ");
            assert_eq!(*expect_value, "GenericLane");
            assert_eq!(skip_ws(identifier)(input), Ok((expect_rest, expect_value)));
        }
        {
            let input = Span::new("\t regional ");
            let expect_rest = input.slice(10..);
            let expect_value = input.slice(2..10);
            assert_eq!(*expect_rest, " ");
            assert_eq!(*expect_value, "regional");
            assert_eq!(skip_ws(identifier)(input), Ok((expect_rest, expect_value)));
        }
        {
            let input = Span::new("   NodeXY64");
            let expect_rest = input.slice(11..);
            let expect_value = input.slice(3..11);
            assert_eq!(*expect_rest, "");
            assert_eq!(*expect_value, "NodeXY64");
            assert_eq!(skip_ws(identifier)(input), Ok((expect_rest, expect_value)));
        }
        {
            let input = Span::new("\r\n\nSub-Cause-Code  ");
            let expect_rest = input.slice(17..);
            let expect_value = input.slice(3..17);
            assert_eq!(*expect_rest, "  ");
            assert_eq!(*expect_value, "Sub-Cause-Code");
            assert_eq!(skip_ws(identifier)(input), Ok((expect_rest, expect_value)));
        }
    }

    #[test]
    fn discards_whitespace_and_comments() {
        let input = Span::new(" -- comment --EEE-DDD");
        let expect_rest = input.slice(21..);
        let expect_value = input.slice(14..21);
        assert_eq!(*expect_rest, "");
        assert_eq!(*expect_value, "EEE-DDD");
        assert_eq!(
            skip_ws_and_comments(identifier)(input),
            Ok((expect_rest, expect_value))
        );
    }

    #[test]
    fn parses_distinguished_values() {
        let sample = Span::new(
            r#"{
    positiveOutOfRange (160),
    unavailable        (161)
}"#,
        );
        assert_eq!(
            distinguished_values(sample).unwrap().1,
            [
                DistinguishedValue {
                    name: "positiveOutOfRange".into(),
                    value: 160,
                },
                DistinguishedValue {
                    name: "unavailable".into(),
                    value: 161,
                },
            ]
        )
    }

    #[test]
    fn parses_distinguished_values_with_line_comments() {
        let sample = Span::new(
            r#"{
    negativeOutOfRange (159), -- ignore this comment
    positiveOutOfRange (160), -- ignore this comment, too
    unavailable        (161)
}"#,
        );
        assert_eq!(
            distinguished_values(sample).unwrap().1,
            [
                DistinguishedValue {
                    name: "negativeOutOfRange".into(),
                    value: 159,
                },
                DistinguishedValue {
                    name: "positiveOutOfRange".into(),
                    value: 160,
                },
                DistinguishedValue {
                    name: "unavailable".into(),
                    value: 161,
                },
            ]
        )
    }
}
