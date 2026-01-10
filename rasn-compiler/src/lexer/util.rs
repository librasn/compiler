use std::{cmp::min, fmt::Debug};

use nom::{
    bytes::complete::tag,
    error::{Error, ErrorKind, ParseError},
    Err, FindSubstring, IResult, Input as _, Offset, Parser,
};

use crate::input::Input;

use super::error::{ErrorTree, ParserResult};

#[allow(dead_code)]
pub fn debug_result<'a, F>(
    mut parser: F,
) -> impl Parser<Input<'a>, Output = F::Output, Error = F::Error>
where
    F: Parser<Input<'a>>,
    F::Output: Debug,
    F::Error: Debug,
{
    move |input: Input<'a>| {
        let result = parser.parse(input.clone());
        match &result {
            Ok((rest, value)) => {
                let bytes_matched = Offset::offset(&input, rest);
                let matched = input.slice(..bytes_matched);
                eprintln!("VALUE  : {value:#?}");
                eprintln!("MATCHED: {:#?}", matched.inner());
                eprintln!("REST   : {rest:?}");
            }
            Err(err) => {
                eprintln!("ERROR  : {err:#?}");
            }
        }
        result
    }
}

pub fn until_next_unindented(input: &str, at_least_until: usize, fallback_len: usize) -> &str {
    let mut prev_was_newline = false;
    for (idx, ch) in input[at_least_until..].char_indices() {
        if prev_was_newline && ch.is_ascii_alphanumeric() {
            // Found "\n[A-Za-z0-9]" pattern, return up to the newline
            return &input[..(idx - 1 + at_least_until)];
        }
        prev_was_newline = ch == '\n';
    }

    // No match found, use fallback
    input[..input.len().min(fallback_len)].trim()
}

pub fn hex_to_bools(c: char) -> [bool; 4] {
    match c {
        '1' => [false, false, false, true],
        '2' => [false, false, true, false],
        '3' => [false, false, true, true],
        '4' => [false, true, false, false],
        '5' => [false, true, false, true],
        '6' => [false, true, true, false],
        '7' => [false, true, true, true],
        '8' => [true, false, false, false],
        '9' => [true, false, false, true],
        'A' => [true, false, true, false],
        'B' => [true, false, true, true],
        'C' => [true, true, false, false],
        'D' => [true, true, false, true],
        'E' => [true, true, true, false],
        'F' => [true, true, true, true],
        _ => [false, false, false, false],
    }
}

pub fn map_into<I, O, F>(mut parser: F) -> impl Parser<I, Output = O, Error = F::Error>
where
    F: Parser<I>,
    F::Output: Into<O>,
{
    move |input: I| {
        let (input, o) = parser.parse(input)?;
        Ok((input, o.into()))
    }
}

pub fn take_until_or<T, Input, Error: ParseError<Input>>(
    tag1: T,
    tag2: T,
) -> impl Parser<Input, Output = Input, Error = Error>
where
    Input: nom::Input + FindSubstring<T>,
    T: Clone,
{
    move |i: Input| {
        let t1 = tag1.clone();
        let t2 = tag2.clone();
        let res: IResult<_, _, Error> = match (i.find_substring(t1), i.find_substring(t2)) {
            (None, None) => Err(Err::Error(Error::from_error_kind(i, ErrorKind::TakeUntil))),
            (None, Some(index)) | (Some(index), None) => Ok(i.take_split(index)),
            (Some(i1), Some(i2)) => Ok(i.take_split(min(i1, i2))),
        };
        res
    }
}

/// Variant of `nom`'s `take_until` that only breaks off ingest at the `end_tag`
/// if it does not also match the `however_tag`.
///
/// __Example__: In an ASN1 PATTERN constraint, the following input could be found:
///
/// ```"[a-zA-Z]#""(1,8)""(-[a-zA-Z0-9]#(1,8))*"```
///
/// `take_until("\"")` would match only `[a-zA-Z]#`, until the next `"`.
/// `take_unitl_and_not("\"","\"\"")` will match the entire pattern
/// `[a-zA-Z]#""(1,8)""(-[a-zA-Z0-9]#(1,8))*`
pub fn take_until_and_not<'a>(
    end_tag: &'a str,
    however_tag: &'a str,
) -> impl Parser<Input<'a>, Output = &'a str, Error = ErrorTree<'a>> {
    move |i: Input<'a>| {
        fn recursive_until<'a>(
            i: Input<'a>,
            index: usize,
            t1: &'a str,
            t2: &'a str,
        ) -> ParserResult<'a, &'a str> {
            match (
                (i.slice(index..)).find_substring(t1),
                (i.slice(index..)).find_substring(t2),
            ) {
                (None, _) => Err(Err::Error(ErrorTree::from_error_kind(
                    i,
                    ErrorKind::TakeUntil,
                ))),
                (Some(offset), None) => {
                    Ok(i.take_split(index + offset)).map(|(rem, res)| (rem, res.into_inner()))
                }
                (Some(_), Some(offset)) => recursive_until(i, index + offset + 2, t1, t2),
            }
        }
        let res: ParserResult<'_, _> = recursive_until(i, 0, end_tag, however_tag);
        res
    }
}

/// A recursive variant of `nom::bytes::complete::take_until()` for nested delimiters.
/// Takes an opening and a closing tag and returns the input up to the point where the
/// parser hits an unbalanced closing tag. It is designed to work inside the
/// `nom::sequence::delimited()` parser.
///
/// ### Params
/// * `opening_tag` - Opening tag of the delimited sequence. When the parser meets an opening tag, it increases the number of closing tags that need to be matched before returning.
/// * `closing_tag` - Closing tag of the delimited sequence. The parser will consume all balanced closing tags and returns once the first unbalanced closing tag is met. It does not consume the unbalanced tag.
pub fn take_until_unbalanced<'a>(
    opening_tag: &'a str,
    closing_tag: &'a str,
) -> impl Parser<Input<'a>, Output = &'a str, Error = ErrorTree<'a>> {
    move |i: Input<'a>| {
        let mut index = 0;
        let mut bracket_counter = 0;
        'consume: loop {
            let input = i.slice(index..);

            if tag::<&str, Input<'_>, Error<Input<'_>>>(opening_tag)(input.clone()).is_ok() {
                bracket_counter += 1;
                index += opening_tag.len();
            } else if tag::<&str, Input<'_>, Error<Input<'_>>>(closing_tag)(input).is_ok() {
                bracket_counter -= 1;
                index += closing_tag.len();
            } else if index == i.len() - 1 {
                break 'consume;
            } else {
                let c = i.slice(index..).inner().chars().next().unwrap_or_default();
                index += c.len_utf8();
            }

            // We found the unmatched closing bracket.
            if bracket_counter == -1 {
                // We do not consume it.
                index -= closing_tag.len();
                return Ok((i.slice(index..), i.slice(0..index).into_inner()));
            };
        }

        if bracket_counter == 0 {
            Ok(("".into(), i.into_inner()))
        } else {
            Err(Err::Error(ErrorTree::from_error_kind(
                i,
                ErrorKind::TakeUntil,
            )))
        }
    }
}

pub fn opt_delimited<'a, F, G, H>(
    mut first: F,
    mut second: G,
    mut third: H,
) -> impl Parser<Input<'a>, Output = G::Output, Error = ErrorTree<'a>>
where
    F: Parser<Input<'a>, Error = ErrorTree<'a>>,
    G: Parser<Input<'a>, Error = ErrorTree<'a>>,
    H: Parser<Input<'a>, Error = ErrorTree<'a>>,
{
    move |input| {
        let (input, expect_closing) = match first.parse(input) {
            Ok((i, _)) => (i, true),
            Err(Err::Error(e)) => (e.into_input(), false),
            Err(e) => return Err(e),
        };
        let (input, o2) = second.parse(input)?;
        if expect_closing {
            third.parse(input).map(|(i, _)| (i, o2))
        } else {
            Ok((input, o2))
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::lexer::asn1_value;
    use crate::lexer::common::{in_parentheses, skip_ws_and_comments};

    use super::*;
    use crate::lexer::util::{opt_delimited, take_until_and_not};

    use crate::intermediate::{ASN1Value, LEFT_PARENTHESIS, RIGHT_PARENTHESIS};
    use nom::character::streaming::char;

    use nom::multi::many1;

    use nom::bytes::complete::tag;

    #[test]
    fn optional_delimiter() {
        assert_eq!(
            opt_delimited(
                skip_ws_and_comments(tag("1")),
                skip_ws_and_comments(tag("ab")),
                skip_ws_and_comments(tag("2"))
            )
            .parse("1ab2".into())
            .map(|(i, o)| (i.into_inner(), o.into_inner())),
            Ok(("", "ab"))
        );
        assert_eq!(
            opt_delimited(
                skip_ws_and_comments(char('(')),
                skip_ws_and_comments(tag("ab")),
                skip_ws_and_comments(char(')'))
            )
            .parse("ab".into())
            .map(|(i, o)| (i.into_inner(), o.into_inner())),
            Ok(("", "ab"))
        );
        assert!(opt_delimited(
            skip_ws_and_comments(char('(')),
            skip_ws_and_comments(tag("ab")),
            skip_ws_and_comments(char(')'))
        )
        .parse("( abc".into())
        .is_err());
        assert_eq!(
            opt_delimited(
                skip_ws_and_comments(char('(')),
                skip_ws_and_comments(tag("ab")),
                skip_ws_and_comments(char(')'))
            )
            .parse(" ab )".into())
            .map(|(i, o)| (i.into_inner(), o.into_inner())),
            Ok((" )", "ab"))
        );
        assert_eq!(
            in_parentheses(opt_delimited(
                skip_ws_and_comments(char('(')),
                skip_ws_and_comments(tag("ab")),
                skip_ws_and_comments(char(')'))
            ))
            .parse("(( ab ))".into())
            .map(|(i, o)| (i.into_inner(), o.into_inner())),
            Ok(("", "ab"))
        );
        assert_eq!(
            many1(in_parentheses(opt_delimited(
                skip_ws_and_comments(char(LEFT_PARENTHESIS)),
                skip_ws_and_comments(asn1_value),
                skip_ws_and_comments(char(RIGHT_PARENTHESIS))
            )))
            .parse("((5))".into())
            .map(|(i, o)| (i.into_inner(), o)),
            Ok(("", vec![ASN1Value::Integer(5)]))
        );
    }

    #[test]
    fn takes_until_and_not() {
        assert_eq!(
            take_until_and_not("\"", "\"\"")
                .parse(r#"[a-zA-Z]#""(1,8)""(-[a-zA-Z0-9]#(1,8))*""#.into())
                .unwrap()
                .1,
            r#"[a-zA-Z]#""(1,8)""(-[a-zA-Z0-9]#(1,8))*"#
        );
        assert_eq!(
            take_until_and_not("\"", "\"\"")
                .parse(r#"[a-zA-Z]#(1,8)""(-[a-zA-Z0-9]#(1,8))*""#.into())
                .unwrap()
                .1,
            r#"[a-zA-Z]#(1,8)""(-[a-zA-Z0-9]#(1,8))*"#
        );
        assert_eq!(
            take_until_and_not("\"", "\"\"")
                .parse(r#"[a-zA-Z]#(1,8)(-[a-zA-Z0-9]#(1,8))*""#.into())
                .unwrap()
                .1,
            r#"[a-zA-Z]#(1,8)(-[a-zA-Z0-9]#(1,8))*"#
        )
    }
}
