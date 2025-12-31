use nom::{
    branch::alt,
    bytes::complete::{is_not, tag},
    character::complete::{
        alpha1, alphanumeric1, char, i128, multispace0, multispace1, one_of, u64,
    },
    combinator::{cut, into, map, map_res, opt, peek, recognize, rest, success, value},
    multi::{many0, many1},
    sequence::{delimited, pair, preceded, terminated},
    Parser,
};

use crate::{
    input::Input,
    intermediate::{constraints::*, types::*, *},
    lexer::error::ErrorTree,
};

use super::{
    error::{MiscError, ParserResult},
    util::{map_into, opt_delimited, take_until_or, take_until_unbalanced},
};

/// Parses an ASN1 comment.
///
/// * `input` [Input]-wrapped string slice reference used as an input for the lexer
///
/// returns a `Result` yielding a tuple containing a reference to the remaining [Input]-wrapped string slice
/// and the parsed comment in case of sucess, or a parsing error if unsuccessful.
///
/// #### X680
/// _The lexical item "comment" can have two forms:_
///    * _One-line comments which begin with "--" as defined in 12.6.3;_
///    * _Multiple-line comments which begin with "/*" as defined in 12.6.4._
pub fn comment(input: Input<'_>) -> ParserResult<'_, &str> {
    skip_ws(alt((block_comment, line_comment))).parse(input)
}

pub fn line_comment(input: Input<'_>) -> ParserResult<'_, &str> {
    delimited(
        tag(LINE_COMMENT),
        alt((
            into_inner(take_until_or("\n", LINE_COMMENT)),
            into_inner(rest),
        )),
        opt(tag(LINE_COMMENT)),
    )
    .parse(input)
}

pub fn block_comment(input: Input<'_>) -> ParserResult<'_, &str> {
    delimited(
        tag(BLOCK_COMMENT_START),
        take_until_unbalanced(BLOCK_COMMENT_START, BLOCK_COMMENT_END),
        tag(BLOCK_COMMENT_END),
    )
    .parse(input)
}

/// Parses a typereference lexical item.
///
/// #### X.680 12.2  Type references
/// _A "typereference" shall consist of an arbitrary number (one or more) of letters, digits, and
/// hyphens. The initial character shall be an upper-case letter. A hyphen shall not be the last
/// character. A hyphen shall not be immediately followed by another hyphen. NOTE – The rules
/// concerning hyphen are designed to avoid ambiguity with (possibly following) comment. 12.2.2A
/// "typereference" shall not be one of the reserved character sequences listed in 12.38._
pub fn type_reference(input: Input<'_>) -> ParserResult<'_, &'_ str> {
    map_res(
        recognize(pair(
            one_of("ABCDEFGHIJKLMNOPQRSTUVWXYZ"),
            many0(alt((
                preceded(char('-'), into_inner(alphanumeric1)),
                into_inner(alphanumeric1),
            ))),
        )),
        |identifier| {
            if ASN1_KEYWORDS.contains(&identifier.inner()) {
                Err(MiscError("Identifier is ASN.1 keyword."))
            } else {
                Ok(identifier.into_inner())
            }
        },
    )
    .parse(input.clone())
}

/// Parses an ASN1 identifier.
///
/// * `input` [Input]-wrapped string slice reference used as an input for the lexer
///
/// returns a `Result` yielding a tuple containing a reference to the remaining [Input]-wrapped string slice
/// and the parsed identifier in case of sucess, or a parsing error if unsuccessful.
///
/// #### X.680
/// _12.3 An "identifier" shall consist of an arbitrary number (one or more) of letters, digits,
/// and hyphens. The initial character shall be a lower-case letter. A hyphen shall not be the
/// last character. A hyphen shall not be immediately followed by another hyphen._
pub fn identifier(input: Input<'_>) -> ParserResult<'_, &str> {
    skip_ws_and_comments(into_inner(recognize(pair(
        alpha1,
        many0(alt((
            preceded(char('-'), into_inner(alphanumeric1)),
            into_inner(alphanumeric1),
        ))),
    ))))
    .parse(input)
}

/// Parses a valuereference lexical item.
///
/// #### X.680 12.4  Value references
/// _A "valuereference" shall consist of the sequence of characters specified for an "identifier" in
/// 12.3. In analyzing an instance of use of this notation, a "valuereference" is distinguished
/// from an "identifier" by the context in which it appears._
pub fn value_reference(input: Input<'_>) -> ParserResult<'_, &'_ str> {
    into_inner(recognize(pair(
        one_of("abcdefghijklmnopqrstuvwxyz"),
        many0(alt((
            preceded(char('-'), into_inner(alphanumeric1)),
            into_inner(alphanumeric1),
        ))),
    )))
    .parse(input)
}

/// Parses a modulereference lexical item.
///
/// #### X.680 12.5 Module references
/// _A "modulereference" shall consist of the sequence of characters specified for a
/// "typereference" in 12.2. In analyzing an instance of use of this notation, a "modulereference"
/// is distinguished from a "typereference" by the context in which it appears._
pub fn module_reference(input: Input<'_>) -> ParserResult<'_, &'_ str> {
    type_reference(input)
}

pub fn into_inner<'a, F>(parser: F) -> impl Parser<Input<'a>, Output = &'a str, Error = F::Error>
where
    F: Parser<Input<'a>, Output = Input<'a>>,
{
    map(parser, Input::into_inner)
}

pub fn skip_ws<'a, F>(inner: F) -> impl Parser<Input<'a>, Output = F::Output, Error = F::Error>
where
    F: Parser<Input<'a>>,
{
    preceded(multispace0, inner)
}

pub fn skip_ws_and_comments<'a, F>(
    inner: F,
) -> impl Parser<Input<'a>, Output = F::Output, Error = F::Error>
where
    F: Parser<Input<'a>, Error = ErrorTree<'a>>,
{
    preceded(many0(alt((comment, into_inner(multispace1)))), inner)
}

pub fn in_parentheses<'a, F>(
    inner: F,
) -> impl Parser<Input<'a>, Output = F::Output, Error = F::Error>
where
    F: Parser<Input<'a>, Error = ErrorTree<'a>>,
{
    delimited(
        skip_ws_and_comments(char(LEFT_PARENTHESIS)),
        skip_ws_and_comments(inner),
        skip_ws_and_comments(char(RIGHT_PARENTHESIS)),
    )
}

pub fn in_brackets<'a, F>(inner: F) -> impl Parser<Input<'a>, Output = F::Output, Error = F::Error>
where
    F: Parser<Input<'a>, Error = ErrorTree<'a>>,
{
    delimited(
        skip_ws_and_comments(char(LEFT_BRACKET)),
        skip_ws_and_comments(inner),
        skip_ws_and_comments(char(RIGHT_BRACKET)),
    )
}

pub fn in_version_brackets<'a, F>(
    inner: F,
) -> impl Parser<Input<'a>, Output = F::Output, Error = F::Error>
where
    F: Parser<Input<'a>, Error = ErrorTree<'a>>,
{
    delimited(
        skip_ws_and_comments(tag("[[")),
        skip_ws_and_comments(inner),
        skip_ws_and_comments(tag("]]")),
    )
}

pub fn opt_parentheses<'a, F>(
    inner: F,
) -> impl Parser<Input<'a>, Output = F::Output, Error = F::Error>
where
    F: Parser<Input<'a>, Error = ErrorTree<'a>>,
{
    opt_delimited(
        skip_ws_and_comments(char(LEFT_BRACKET)),
        skip_ws_and_comments(inner),
        skip_ws_and_comments(char(RIGHT_BRACKET)),
    )
}

pub fn in_braces<'a, F>(inner: F) -> impl Parser<Input<'a>, Output = F::Output, Error = F::Error>
where
    F: Parser<Input<'a>, Error = ErrorTree<'a>>,
{
    delimited(
        skip_ws_and_comments(char(LEFT_BRACE)),
        skip_ws_and_comments(inner),
        skip_ws_and_comments(char(RIGHT_BRACE)),
    )
}

pub fn all_value(input: Input<'_>) -> ParserResult<'_, ASN1Value> {
    value(ASN1Value::All, skip_ws_and_comments(tag(ALL))).parse(input)
}

pub fn asn_tag(input: Input<'_>) -> ParserResult<'_, AsnTag> {
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
    ))
    .parse(input)
}

pub fn range_seperator(input: Input<'_>) -> ParserResult<'_, RangeSeperator> {
    skip_ws_and_comments(tag(RANGE))
        .map(|_| RangeSeperator())
        .parse(input)
}

pub fn extension_marker(input: Input<'_>) -> ParserResult<'_, ExtensionMarker> {
    skip_ws_and_comments(tag(ELLIPSIS))
        .map(|_| ExtensionMarker())
        .parse(input)
}

pub fn assignment(input: Input<'_>) -> ParserResult<'_, &str> {
    into_inner(skip_ws_and_comments(tag(ASSIGN))).parse(input)
}

pub fn distinguished_values(input: Input<'_>) -> ParserResult<'_, Vec<DistinguishedValue>> {
    delimited(
        skip_ws_and_comments(char(LEFT_BRACE)),
        many0(terminated(
            skip_ws_and_comments(distinguished_val),
            opt(skip_ws_and_comments(char(COMMA))),
        )),
        skip_ws_and_comments(char(RIGHT_BRACE)),
    )
    .parse(input)
}

pub fn distinguished_val(input: Input<'_>) -> ParserResult<'_, DistinguishedValue> {
    map_into(pair(skip_ws_and_comments(identifier), in_parentheses(i128))).parse(input)
}

pub fn optional_comma(input: Input<'_>) -> ParserResult<'_, Option<char>> {
    skip_ws_and_comments(opt(char(COMMA))).parse(input)
}

pub fn uppercase_identifier(input: Input<'_>) -> ParserResult<'_, &str> {
    alt((
        into_inner(recognize(pair(
            one_of("ABCDEFGHIJKLMNOPQRSTUVWXYZ"),
            many1(alt((
                preceded(char('-'), one_of("ABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890")),
                one_of("ABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890"),
            ))),
        ))),
        terminated(
            into_inner(recognize(one_of("ABCDEFGHIJKLMNOPQRSTUVWXYZ"))),
            peek(is_not("abcdefghijklmnopqrstuvwxyz-")),
        ),
    ))
    .parse(input)
}

/// Parse the "optionality" of a field.
///
/// When optionality is DEFAULT, the argument `f` is used to parse the value.
///
/// # Syntax
/// ```text
/// OptionalitySpec ::= OPTIONAL | DEFAULT <f> | empty
/// ```
pub fn optionality<'a, F>(
    f: F,
) -> impl Parser<Input<'a>, Output = Optionality<F::Output>, Error = F::Error>
where
    F: Parser<Input<'a>, Error = ErrorTree<'a>>,
    F::Output: Clone,
{
    alt((
        value(Optionality::Optional, tag(OPTIONAL)),
        preceded(
            tag(DEFAULT),
            cut(skip_ws_and_comments(f.map(Optionality::Default))),
        ),
        success(Optionality::Required),
    ))
}

#[cfg(test)]
mod tests {

    use crate::lexer::asn1_value;

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
            )
            .map(|(i, o)| (i.into_inner(), o)),
            Ok(("\n      unavailable", " This means backward",),)
        );
        assert_eq!(
            comment(
                r#"-- This means forward
        backward    (2), -- This means backward"#
                    .into()
            )
            .map(|(i, o)| (i.into_inner(), o)),
            Ok((
                "\n        backward    (2), -- This means backward",
                " This means forward",
            ),)
        )
    }

    #[test]
    fn parses_valid_identifiers() {
        assert_eq!(identifier("EEE-DDD".into()).unwrap().1, "EEE-DDD");
        assert_eq!(identifier("GenericLane ".into()).unwrap().1, "GenericLane");
        assert_eq!(identifier("regional ".into()).unwrap().1, "regional");
        assert_eq!(identifier("NodeXY64".into()).unwrap().1, "NodeXY64");
        assert_eq!(
            identifier("Sub-Cause-Code  ".into()).unwrap().1,
            "Sub-Cause-Code"
        );
    }

    #[test]
    fn handles_invalid_identifiers() {
        assert_eq!(
            identifier("EEE--DDD".into())
                .map(|(i, o)| (i.into_inner(), o))
                .unwrap(),
            ("--DDD", "EEE")
        );
        assert!(identifier("-GenericLane".into()).is_err());
        assert!(identifier("64NodeXY".into()).is_err());
        assert!(identifier("&regional".into()).is_err());
        assert_eq!(
            identifier("Sub-Cause-Code-".into())
                .map(|(i, o)| (i.into_inner(), o))
                .unwrap(),
            ("-", "Sub-Cause-Code")
        );
    }

    #[test]
    fn discards_whitespace() {
        assert_eq!(
            skip_ws(identifier).parse(" EEE-DDD".into()).unwrap().1,
            "EEE-DDD"
        );
        assert_eq!(
            skip_ws(identifier)
                .parse("\nGenericLane ".into())
                .unwrap()
                .1,
            "GenericLane"
        );
        assert_eq!(
            skip_ws(identifier).parse("\t regional ".into()).unwrap().1,
            "regional"
        );
        assert_eq!(
            skip_ws(identifier).parse("   NodeXY64".into()).unwrap().1,
            "NodeXY64"
        );
        assert_eq!(
            skip_ws(identifier)
                .parse("\r\n\nSub-Cause-Code  ".into())
                .unwrap()
                .1,
            "Sub-Cause-Code"
        );
    }

    #[test]
    fn discards_whitespace_and_comments() {
        assert_eq!(
            skip_ws_and_comments(identifier)
                .parse(" -- comment --EEE-DDD".into())
                .unwrap()
                .1,
            "EEE-DDD"
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

    #[test]
    fn parses_eof_line_comment() {
        assert_eq!(
            line_comment(r#"-- LdapSystemSchema"#.into()).unwrap().1,
            " LdapSystemSchema"
        )
    }

    #[test]
    fn parses_default_int() {
        assert_eq!(
            optionality(asn1_value)
                .parse("DEFAULT\t-1".into())
                .unwrap()
                .1,
            Optionality::Default(ASN1Value::Integer(-1))
        );
    }

    #[test]
    fn parses_default_boolean() {
        assert_eq!(
            optionality(asn1_value)
                .parse("DEFAULT   TRUE".into())
                .unwrap()
                .1,
            Optionality::Default(ASN1Value::Boolean(true))
        );
    }

    #[test]
    fn parses_default_bitstring() {
        assert_eq!(
            optionality(asn1_value)
                .parse("DEFAULT '001010011'B".into())
                .unwrap()
                .1,
            Optionality::Default(ASN1Value::BitString(vec![
                false, false, true, false, true, false, false, true, true
            ]))
        );
        assert_eq!(
            optionality(asn1_value)
                .parse("DEFAULT 'F60E'H".into())
                .unwrap()
                .1,
            Optionality::Default(ASN1Value::BitString(vec![
                true, true, true, true, false, true, true, false, false, false, false, false, true,
                true, true, false
            ]))
        );
    }

    #[test]
    fn parses_default_enumeral() {
        assert_eq!(
            optionality(asn1_value)
                .parse("DEFAULT enumeral1".into())
                .unwrap()
                .1,
            Optionality::Default(ASN1Value::ElsewhereDeclaredValue {
                module: None,
                identifier: "enumeral1".into(),
                parent: None
            })
        );
        assert_eq!(
            optionality(asn1_value)
                .parse("DEFAULT enumeral1".into())
                .unwrap()
                .1,
            Optionality::Default(ASN1Value::ElsewhereDeclaredValue {
                module: None,
                identifier: "enumeral1".into(),
                parent: None
            })
        );
    }
}
