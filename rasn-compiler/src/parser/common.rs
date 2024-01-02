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
    IResult,
};

use crate::intermediate::{constraints::*, types::*, *};

use super::{
    asn1_value,
    util::{map_into, opt_delimited, take_until_or, take_until_unbalanced},
};

/// Parses an ASN1 comment.
///
/// * `input` string slice reference used as an input for the parser
///
/// returns a `Result` yielding a tuple containing a reference to the remaining string slice
/// and the parsed comment in case of sucess, or a parsing error if unsuccessful.
///
/// #### X680
/// _The lexical item "comment" can have two forms:_
///    * _One-line comments which begin with "--" as defined in 12.6.3;_
///    * _Multiple-line comments which begin with "/*" as defined in 12.6.4._
pub fn comment<'a>(input: &'a str) -> IResult<&'a str, &'a str> {
    skip_ws(alt((block_comment, line_comment)))(input)
}

pub fn line_comment<'a>(input: &'a str) -> IResult<&'a str, &'a str> {
    delimited(
        tag(LINE_COMMENT),
        take_until_or("\n", LINE_COMMENT),
        opt(tag(LINE_COMMENT)),
    )(input)
}

pub fn block_comment<'a>(input: &'a str) -> IResult<&'a str, &'a str> {
    delimited(
        tag(BLOCK_COMMENT_START),
        take_until_unbalanced(BLOCK_COMMENT_START, BLOCK_COMMENT_END),
        tag(BLOCK_COMMENT_END),
    )(input)
}

/// Parses an ASN1 identifier.
///
/// * `input` string slice reference used as an input for the parser
///
/// returns a `Result` yielding a tuple containing a reference to the remaining string slice
/// and the parsed identifier in case of sucess, or a parsing error if unsuccessful.
///
/// #### X.680
/// _12.3 An "identifier" shall consist of an arbitrary number (one or more) of letters, digits,
/// and hyphens. The initial character shall be a lower-case letter. A hyphen shall not be the
/// last character. A hyphen shall not be immediately followed by another hyphen._
pub fn identifier<'a>(input: &'a str) -> IResult<&'a str, &'a str> {
    skip_ws_and_comments(recognize(pair(
        alpha1,
        many0(alt((preceded(char('-'), alphanumeric1), alphanumeric1))),
    )))(input)
}

pub fn title_case_identifier<'a>(input: &'a str) -> IResult<&'a str, &'a str> {
    map_res(
        recognize(pair(
            one_of("ABCDEFGHIJKLMNOPQRSTUVWXYZ01234567890"),
            many0(alt((preceded(char('-'), alphanumeric1), alphanumeric1))),
        )),
        |identifier| {
            if ASN1_KEYWORDS.contains(&identifier) {
                Err(nom::Err::Error(Error {
                    input: input,
                    code: nom::error::ErrorKind::Tag,
                }))
            } else {
                Ok(identifier)
            }
        },
    )(input)
}

pub fn value_identifier<'a>(input: &'a str) -> IResult<&'a str, &'a str> {
    recognize(pair(
        one_of("abcdefghijklmnopqrstuvwxyz"),
        many0(alt((preceded(char('-'), alphanumeric1), alphanumeric1))),
    ))(input)
}

pub fn skip_ws<'a, F, O>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O>
where
    F: FnMut(&'a str) -> IResult<&'a str, O>,
{
    preceded(multispace0, inner)
}

pub fn skip_ws_and_comments<'a, F, O>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O>
where
    F: FnMut(&'a str) -> IResult<&'a str, O>,
{
    preceded(many0(alt((comment, multispace1))), inner)
}

pub fn in_parentheses<'a, F, O>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O>
where
    F: FnMut(&'a str) -> IResult<&'a str, O>,
{
    delimited(
        skip_ws_and_comments(char(LEFT_PARENTHESIS)),
        skip_ws_and_comments(inner),
        skip_ws_and_comments(char(RIGHT_PARENTHESIS)),
    )
}

pub fn in_brackets<'a, F, O>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O>
where
    F: FnMut(&'a str) -> IResult<&'a str, O>,
{
    delimited(
        skip_ws_and_comments(char(LEFT_BRACKET)),
        skip_ws_and_comments(inner),
        skip_ws_and_comments(char(RIGHT_BRACKET)),
    )
}

pub fn in_version_brackets<'a, F, O>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O>
where
    F: FnMut(&'a str) -> IResult<&'a str, O>,
{
    delimited(
        skip_ws_and_comments(tag("[[")),
        skip_ws_and_comments(inner),
        skip_ws_and_comments(tag("]]")),
    )
}

pub fn opt_parentheses<'a, F, O>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O>
where
    F: FnMut(&'a str) -> IResult<&'a str, O>,
{
    opt_delimited::<char, O, char, Error<&str>, _, _, _>(
        skip_ws_and_comments(char(LEFT_BRACKET)),
        skip_ws_and_comments(inner),
        skip_ws_and_comments(char(RIGHT_BRACKET)),
    )
}

pub fn in_braces<'a, F, O>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O>
where
    F: FnMut(&'a str) -> IResult<&'a str, O>,
{
    delimited(
        skip_ws_and_comments(char(LEFT_BRACE)),
        skip_ws_and_comments(inner),
        skip_ws_and_comments(char(RIGHT_BRACE)),
    )
}

pub fn all_value<'a>(input: &'a str) -> IResult<&'a str, ASN1Value> {
    value(ASN1Value::All, skip_ws_and_comments(tag(ALL)))(input)
}

pub fn asn_tag<'a>(input: &'a str) -> IResult<&'a str, AsnTag> {
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

pub fn range_seperator<'a>(input: &'a str) -> IResult<&'a str, RangeSeperator> {
    skip_ws_and_comments(tag(RANGE))(input).map(|(remaining, _)| (remaining, RangeSeperator()))
}

pub fn extension_marker<'a>(input: &'a str) -> IResult<&'a str, ExtensionMarker> {
    skip_ws_and_comments(tag(ELLIPSIS))(input).map(|(remaining, _)| (remaining, ExtensionMarker()))
}

pub fn assignment<'a>(input: &'a str) -> IResult<&'a str, &'a str> {
    skip_ws_and_comments(tag(ASSIGN))(input)
}

pub fn distinguished_values<'a>(input: &'a str) -> IResult<&'a str, Vec<DistinguishedValue>> {
    delimited(
        skip_ws_and_comments(char(LEFT_BRACE)),
        many0(terminated(
            skip_ws_and_comments(distinguished_val),
            opt(skip_ws_and_comments(char(COMMA))),
        )),
        skip_ws_and_comments(char(RIGHT_BRACE)),
    )(input)
}

pub fn distinguished_val<'a>(input: &'a str) -> IResult<&'a str, DistinguishedValue> {
    map_into(pair(skip_ws_and_comments(identifier), in_parentheses(i128)))(input)
}

pub fn optional_comma<'a>(input: &'a str) -> IResult<&'a str, Option<char>> {
    skip_ws_and_comments(opt(char(COMMA)))(input)
}

pub fn optional_marker<'a>(input: &'a str) -> IResult<&'a str, Option<OptionalMarker>> {
    opt(into(skip_ws_and_comments(tag(OPTIONAL))))(input)
}

pub fn default<'a>(input: &'a str) -> IResult<&'a str, Option<ASN1Value>> {
    opt(preceded(
        skip_ws_and_comments(tag(DEFAULT)),
        skip_ws_and_comments(asn1_value),
    ))(input)
}

pub fn uppercase_identifier<'a>(input: &'a str) -> IResult<&'a str, &'a str> {
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

    use super::*;

    #[test]
    fn parses_line_comment() {
        let line = r#"-- Test, one, two, three/
"#;
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
            )
            .unwrap()
            .1
        );
        assert_eq!(
            " Very annoying! ",
            comment("-- Very annoying! --").unwrap().1
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
    StartOfDeclaration"#).unwrap().1,
      "* \n      * This DE indicates a change of acceleration.\n      *\n      * The value shall be set to:\n      * - 0 - `accelerate` - if the magnitude of the horizontal velocity vector increases.\n      * - 1 - `decelerate` - if the magnitude of the horizontal velocity vector decreases.\n      *\n      * @category: Kinematic information\n      * @revision: Created in V2.1.1\n     "
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
            ),
            Ok(("\n      unavailable", " This means backward",),)
        );
        assert_eq!(
            comment(
                r#"-- This means forward
        backward    (2), -- This means backward"#
            ),
            Ok((
                "\n        backward    (2), -- This means backward",
                " This means forward",
            ),)
        )
    }

    #[test]
    fn parses_valid_identifiers() {
        assert_eq!(identifier("EEE-DDD"), Ok(("", "EEE-DDD")));
        assert_eq!(identifier("GenericLane "), Ok((" ", "GenericLane")));
        assert_eq!(identifier("regional "), Ok((" ", "regional")));
        assert_eq!(identifier("NodeXY64"), Ok(("", "NodeXY64")));
        assert_eq!(identifier("Sub-Cause-Code  "), Ok(("  ", "Sub-Cause-Code")));
    }

    #[test]
    fn handles_invalid_identifiers() {
        assert_eq!(identifier("EEE--DDD"), Ok(("--DDD", "EEE")));
        assert!(identifier("-GenericLane").is_err());
        assert!(identifier("64NodeXY").is_err());
        assert!(identifier("&regional").is_err());
        assert_eq!(identifier("Sub-Cause-Code-"), Ok(("-", "Sub-Cause-Code")));
    }

    #[test]
    fn discards_whitespace() {
        assert_eq!(skip_ws(identifier)(" EEE-DDD"), Ok(("", "EEE-DDD")));
        assert_eq!(
            skip_ws(identifier)("\nGenericLane "),
            Ok((" ", "GenericLane"))
        );
        assert_eq!(skip_ws(identifier)("\t regional "), Ok((" ", "regional")));
        assert_eq!(skip_ws(identifier)("   NodeXY64"), Ok(("", "NodeXY64")));
        assert_eq!(
            skip_ws(identifier)("\r\n\nSub-Cause-Code  "),
            Ok(("  ", "Sub-Cause-Code"))
        );
    }

    #[test]
    fn discards_whitespace_and_comments() {
        assert_eq!(
            skip_ws_and_comments(identifier)(" -- comment --EEE-DDD"),
            Ok(("", "EEE-DDD"))
        );
    }

    #[test]
    fn parses_distinguished_values() {
        let sample = r#"{
    positiveOutOfRange (160),
    unavailable        (161)  
}"#;
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
}"#;
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
