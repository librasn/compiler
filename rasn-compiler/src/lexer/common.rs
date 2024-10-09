use nom::{
    branch::alt,
    bytes::complete::{is_not, tag},
    character::complete::{
        alpha1, alphanumeric1, char, i128, multispace0, multispace1, one_of, u64,
    },
    combinator::{into, map, map_res, opt, peek, recognize, value},
    error::Error,
    multi::{many0, many1},
    sequence::{delimited, pair, preceded, terminated},
    IResult,
};

use crate::intermediate::{constraints::*, types::*, *};

use super::{
    asn1_value,
    input::*,
    util::{map_into, opt_delimited, take_until_or, take_until_unbalanced},
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
pub fn comment(input: Input<'_>) -> IResult<Input<'_>, &str> {
    skip_ws(alt((block_comment, line_comment)))(input)
}

pub fn line_comment(input: Input<'_>) -> IResult<Input<'_>, &str> {
    delimited(
        tag(LINE_COMMENT),
        into_inner(take_until_or("\n", LINE_COMMENT)),
        opt(tag(LINE_COMMENT)),
    )(input)
}

pub fn block_comment(input: Input<'_>) -> IResult<Input<'_>, &str> {
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
pub fn identifier(input: Input<'_>) -> IResult<Input<'_>, &str> {
    skip_ws_and_comments(into_inner(recognize(pair(
        alpha1,
        many0(alt((
            preceded(char('-'), into_inner(alphanumeric1)),
            into_inner(alphanumeric1),
        ))),
    ))))(input)
}

pub fn into_inner<'a, F>(parser: F) -> impl FnMut(Input<'a>) -> IResult<Input<'a>, &'a str>
where
    F: FnMut(Input<'a>) -> IResult<Input<'a>, Input<'a>>,
{
    map(parser, |s: Input<'_>| s.into_inner())
}

pub fn title_case_identifier(input: Input<'_>) -> IResult<Input<'_>, &str> {
    map_res(
        recognize(pair(
            one_of("ABCDEFGHIJKLMNOPQRSTUVWXYZ01234567890"),
            many0(alt((
                preceded(char('-'), into_inner(alphanumeric1)),
                into_inner(alphanumeric1),
            ))),
        )),
        |identifier| {
            if ASN1_KEYWORDS.contains(&identifier.inner()) {
                Err(nom::Err::Error(Error {
                    input: input.clone(),
                    code: nom::error::ErrorKind::Tag,
                }))
            } else {
                Ok(identifier.into_inner())
            }
        },
    )(input.clone())
}

pub fn value_identifier(input: Input<'_>) -> IResult<Input<'_>, &str> {
    into_inner(recognize(pair(
        one_of("abcdefghijklmnopqrstuvwxyz"),
        many0(alt((
            preceded(char('-'), into_inner(alphanumeric1)),
            into_inner(alphanumeric1),
        ))),
    )))(input)
}

pub fn skip_ws<'a, F, O>(inner: F) -> impl FnMut(Input<'a>) -> IResult<Input<'a>, O>
where
    F: FnMut(Input<'a>) -> IResult<Input<'a>, O>,
{
    preceded(multispace0, inner)
}

pub fn skip_ws_and_comments<'a, F, O>(inner: F) -> impl FnMut(Input<'a>) -> IResult<Input<'a>, O>
where
    F: FnMut(Input<'a>) -> IResult<Input<'a>, O>,
{
    preceded(many0(alt((comment, into_inner(multispace1)))), inner)
}

pub fn in_parentheses<'a, F, O>(inner: F) -> impl FnMut(Input<'a>) -> IResult<Input<'a>, O>
where
    F: FnMut(Input<'a>) -> IResult<Input<'a>, O>,
{
    delimited(
        skip_ws_and_comments(char(LEFT_PARENTHESIS)),
        skip_ws_and_comments(inner),
        skip_ws_and_comments(char(RIGHT_PARENTHESIS)),
    )
}

pub fn in_brackets<'a, F, O>(inner: F) -> impl FnMut(Input<'a>) -> IResult<Input<'a>, O>
where
    F: FnMut(Input<'a>) -> IResult<Input<'a>, O>,
{
    delimited(
        skip_ws_and_comments(char(LEFT_BRACKET)),
        skip_ws_and_comments(inner),
        skip_ws_and_comments(char(RIGHT_BRACKET)),
    )
}

pub fn in_version_brackets<'a, F, O>(inner: F) -> impl FnMut(Input<'a>) -> IResult<Input<'a>, O>
where
    F: FnMut(Input<'a>) -> IResult<Input<'a>, O>,
{
    delimited(
        skip_ws_and_comments(tag("[[")),
        skip_ws_and_comments(inner),
        skip_ws_and_comments(tag("]]")),
    )
}

pub fn opt_parentheses<'a, F, O>(inner: F) -> impl FnMut(Input<'a>) -> IResult<Input<'a>, O>
where
    F: FnMut(Input<'a>) -> IResult<Input<'a>, O>,
{
    opt_delimited::<char, O, char, _, _, _>(
        skip_ws_and_comments(char(LEFT_BRACKET)),
        skip_ws_and_comments(inner),
        skip_ws_and_comments(char(RIGHT_BRACKET)),
    )
}

pub fn in_braces<'a, F, O>(inner: F) -> impl FnMut(Input<'a>) -> IResult<Input<'a>, O>
where
    F: FnMut(Input<'a>) -> IResult<Input<'a>, O>,
{
    delimited(
        skip_ws_and_comments(char(LEFT_BRACE)),
        skip_ws_and_comments(inner),
        skip_ws_and_comments(char(RIGHT_BRACE)),
    )
}

pub fn all_value(input: Input<'_>) -> IResult<Input<'_>, ASN1Value> {
    value(ASN1Value::All, skip_ws_and_comments(tag(ALL)))(input)
}

pub fn asn_tag(input: Input<'_>) -> IResult<Input<'_>, AsnTag> {
    into(pair(
        in_brackets(pair(
            opt(into_inner(skip_ws_and_comments(alt((
                tag(PRIVATE),
                tag(APPLICATION),
                tag(UNIVERSAL),
            ))))),
            skip_ws_and_comments(u64),
        )),
        skip_ws_and_comments(opt(alt((
            value(TaggingEnvironment::Explicit, tag(EXPLICIT)),
            value(TaggingEnvironment::Implicit, tag(IMPLICIT)),
        )))),
    ))(input)
}

pub fn range_seperator(input: Input<'_>) -> IResult<Input<'_>, RangeSeperator> {
    skip_ws_and_comments(tag(RANGE))(input).map(|(remaining, _)| (remaining, RangeSeperator()))
}

pub fn extension_marker(input: Input<'_>) -> IResult<Input<'_>, ExtensionMarker> {
    skip_ws_and_comments(tag(ELLIPSIS))(input).map(|(remaining, _)| (remaining, ExtensionMarker()))
}

pub fn assignment(input: Input<'_>) -> IResult<Input<'_>, &str> {
    into_inner(skip_ws_and_comments(tag(ASSIGN)))(input)
}

pub fn distinguished_values(input: Input<'_>) -> IResult<Input<'_>, Vec<DistinguishedValue>> {
    delimited(
        skip_ws_and_comments(char(LEFT_BRACE)),
        many0(terminated(
            skip_ws_and_comments(distinguished_val),
            opt(skip_ws_and_comments(char(COMMA))),
        )),
        skip_ws_and_comments(char(RIGHT_BRACE)),
    )(input)
}

pub fn distinguished_val(input: Input<'_>) -> IResult<Input<'_>, DistinguishedValue> {
    map_into(pair(skip_ws_and_comments(identifier), in_parentheses(i128)))(input)
}

pub fn optional_comma(input: Input<'_>) -> IResult<Input<'_>, Option<char>> {
    skip_ws_and_comments(opt(char(COMMA)))(input)
}

pub fn optional_marker(input: Input<'_>) -> IResult<Input<'_>, Option<OptionalMarker>> {
    opt(into(into_inner(skip_ws_and_comments(tag(OPTIONAL)))))(input)
}

pub fn default(input: Input<'_>) -> IResult<Input<'_>, Option<ASN1Value>> {
    opt(preceded(
        skip_ws_and_comments(tag(DEFAULT)),
        skip_ws_and_comments(asn1_value),
    ))(input)
}

pub fn uppercase_identifier(input: Input<'_>) -> IResult<Input<'_>, &str> {
    alt((
        into_inner(recognize(pair(
            one_of("ABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890"),
            many1(alt((
                preceded(char('-'), one_of("ABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890")),
                one_of("ABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890"),
            ))),
        ))),
        terminated(
            into_inner(recognize(one_of("ABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890"))),
            peek(is_not("abcdefghijklmnopqrstuvwxyz-")),
        ),
    ))(input)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn parses_line_comment() {
        let line = r#"-- Test, one, two, three/
"#
        .into();
        assert_eq!(" Test, one, two, three/", comment(line).unwrap().1);
    }

    #[test]
    fn parses_block_comment() {
        assert_eq!(
            r#" Test, one, two, three
and one "#,
            comment(
                r#"/* Test, one, two, three
and one */"#
                    .into()
            )
            .unwrap()
            .1
        );
        assert_eq!(
            r#"*
       * Hello
       "#,
            comment(
                r#"/**
       * Hello
       */"#
                .into()
            )
            .unwrap()
            .1
        );
        assert_eq!(
            " Very annoying! ",
            comment("-- Very annoying! --".into()).unwrap().1
        )
    }

    #[test]
    fn parses_multiline_block_comment() {
        assert_eq!(comment(r#"/**
      * This DE indicates a change of acceleration.
      *
      * The value shall be set to:
      * - 0 - `accelerate` - if the magnitude of the horizontal velocity vector increases.
      * - 1 - `decelerate` - if the magnitude of the horizontal velocity vector decreases.
      *
      * @category: Kinematic information
      * @revision: Created in V2.1.1
     */
    StartOfDeclaration"#.into()).unwrap().1,
      "*\n      * This DE indicates a change of acceleration.\n      *\n      * The value shall be set to:\n      * - 0 - `accelerate` - if the magnitude of the horizontal velocity vector increases.\n      * - 1 - `decelerate` - if the magnitude of the horizontal velocity vector decreases.\n      *\n      * @category: Kinematic information\n      * @revision: Created in V2.1.1\n     "
    )
    }

    /// 12.6.4. Whenever a "comment" begins with "/*", it shall end with a corresponding "*/",
    /// whether this "*/" is on the same line or not. If another "/*" is found before a "*/",
    /// then the comment terminates when a matching "*/" has been found for each "/*".
    #[test]
    fn parses_block_comment_with_nested_comments() {
        assert_eq!(
            comment(
                r#"/*this is a comment /*
        this is a nested comment */ this text should be parsed*/"#
                    .into()
            )
            .unwrap()
            .1,
            "this is a comment /*
        this is a nested comment */ this text should be parsed"
        )
    }

    #[test]
    fn parses_ambiguous_asn1_comment() {
        assert_eq!(
            comment(
                r#" -- This means backward
      unavailable"#
                    .into()
            ),
            Ok(("\n      unavailable".into(), " This means backward",),)
        );
        assert_eq!(
            comment(
                r#"-- This means forward
        backward    (2), -- This means backward"#
                    .into()
            ),
            Ok((
                "\n        backward    (2), -- This means backward".into(),
                " This means forward",
            ),)
        )
    }

    #[test]
    fn parses_valid_identifiers() {
        assert_eq!(
            identifier("EEE-DDD".into()),
            Ok(("".into(), "EEE-DDD"))
        );
        assert_eq!(
            identifier("GenericLane ".into()),
            Ok((" ".into(), "GenericLane"))
        );
        assert_eq!(
            identifier("regional ".into()),
            Ok((" ".into(), "regional"))
        );
        assert_eq!(
            identifier("NodeXY64".into()),
            Ok(("".into(), "NodeXY64"))
        );
        assert_eq!(
            identifier("Sub-Cause-Code  ".into()),
            Ok(("  ".into(), "Sub-Cause-Code"))
        );
    }

    #[test]
    fn handles_invalid_identifiers() {
        assert_eq!(
            identifier("EEE--DDD".into()),
            Ok(("--DDD".into(), "EEE"))
        );
        assert!(identifier("-GenericLane".into()).is_err());
        assert!(identifier("64NodeXY".into()).is_err());
        assert!(identifier("&regional".into()).is_err());
        assert_eq!(
            identifier("Sub-Cause-Code-".into()),
            Ok(("-".into(), "Sub-Cause-Code"))
        );
    }

    #[test]
    fn discards_whitespace() {
        assert_eq!(
            skip_ws(identifier)(" EEE-DDD".into()),
            Ok(("".into(), "EEE-DDD"))
        );
        assert_eq!(
            skip_ws(identifier)("\nGenericLane ".into()),
            Ok((" ".into(), "GenericLane"))
        );
        assert_eq!(
            skip_ws(identifier)("\t regional ".into()),
            Ok((" ".into(), "regional"))
        );
        assert_eq!(
            skip_ws(identifier)("   NodeXY64".into()),
            Ok(("".into(), "NodeXY64"))
        );
        assert_eq!(
            skip_ws(identifier)("\r\n\nSub-Cause-Code  ".into()),
            Ok(("  ".into(), "Sub-Cause-Code"))
        );
    }

    #[test]
    fn discards_whitespace_and_comments() {
        assert_eq!(
            skip_ws_and_comments(identifier)(" -- comment --EEE-DDD".into()),
            Ok(("".into(), "EEE-DDD"))
        );
    }

    #[test]
    fn parses_distinguished_values() {
        let sample = r#"{
    positiveOutOfRange (160),
    unavailable        (161)
}"#
        .into();
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
        let sample = r#"{
    negativeOutOfRange (159), -- ignore this comment
    positiveOutOfRange (160), -- ignore this comment, too
    unavailable        (161)
}"#
        .into();
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
