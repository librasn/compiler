use std::cmp::min;

use nom::{
    bytes::complete::tag,
    error::{Error, ErrorKind, ParseError},
    Err, FindSubstring, IResult, InputLength, InputTake, Parser,
};

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

pub fn map_into<I, O1, O2, E, F>(mut parser: F) -> impl FnMut(I) -> IResult<I, O2, E>
where
    F: Parser<I, O1, E>,
    O1: Into<O2>,
{
    move |input: I| {
        let (input, o1) = parser.parse(input)?;
        Ok((input, o1.into()))
    }
}

pub fn take_until_or<T, Input, Error: ParseError<Input>>(
    tag1: T,
    tag2: T,
) -> impl Fn(Input) -> IResult<Input, Input, Error>
where
    Input: InputTake + FindSubstring<T>,
    T: InputLength + Clone,
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
pub fn take_until_and_not<'a, Error: ParseError<&'a str>>(
    end_tag: &'a str,
    however_tag: &'a str,
) -> impl Fn(&'a str) -> IResult<&'a str, &'a str, Error> {
    move |i: &str| {
        let t1 = end_tag.clone();
        let t2 = however_tag.clone();
        fn recursive_until<'a, Error: ParseError<&'a str>>(
            i: &'a str,
            index: usize,
            t1: &'a str,
            t2: &'a str,
        ) -> IResult<&'a str, &'a str, Error> {
            match (
                (&i[index..]).find_substring(t1),
                (&i[index..]).find_substring(t2),
            ) {
                (None, _) => Err(Err::Error(Error::from_error_kind(i, ErrorKind::TakeUntil))),
                (Some(offset), None) => Ok(i.take_split(index + offset)),
                (Some(_), Some(offset)) => recursive_until(i, index + offset + 2, t1, t2),
            }
        }
        let res: IResult<_, _, Error> = recursive_until(i, 0, t1, t2);
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
) -> impl Fn(&'a str) -> IResult<&str, &str> {
    move |i: &str| {
        let mut index = 0;
        let mut bracket_counter = 0;
        'consume: loop {
            let input = &i[index..];

            if tag::<&str, &str, Error<&str>>(opening_tag)(input).is_ok() {
                bracket_counter += 1;
                index += opening_tag.len();
            } else if tag::<&str, &str, Error<&str>>(closing_tag)(input).is_ok() {
                bracket_counter -= 1;
                index += closing_tag.len();
            } else if index == i.len() - 1 {
                break 'consume;
            } else {
                let c = i[index..].chars().next().unwrap_or_default();
                index += c.len_utf8();
            }

            // We found the unmatched closing bracket.
            if bracket_counter == -1 {
                // We do not consume it.
                index -= closing_tag.len();
                return Ok((&i[index..], &i[0..index]));
            };
        }

        if bracket_counter == 0 {
            Ok(("", i))
        } else {
            Err(Err::Error(Error::from_error_kind(i, ErrorKind::TakeUntil)))
        }
    }
}

pub fn opt_delimited<'a, O1, O2, O3, E: ParseError<&'a str>, F, G, H>(
    mut first: F,
    mut second: G,
    mut third: H,
) -> impl FnMut(&'a str) -> IResult<&'a str, O2, Error<&'a str>>
where
    F: Parser<&'a str, O1, Error<&'a str>>,
    G: Parser<&'a str, O2, Error<&'a str>>,
    H: Parser<&'a str, O3, Error<&'a str>>,
    O1: std::fmt::Debug,
{
    move |input| {
        let (input, expect_closing) = match first.parse(input) {
            Ok((i, _)) => (i, true),
            Err(Err::Error(e)) => (e.input, false),
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

    use crate::parser::asn1_value;
    use crate::parser::common::{in_parentheses, skip_ws_and_comments};

    use crate::parser::util::{opt_delimited, take_until_and_not};

    use crate::intermediate::{ASN1Value, LEFT_PARENTHESIS, RIGHT_PARENTHESIS};
    use nom::character::streaming::char;

    use nom::multi::many1;

    use nom::{bytes::complete::tag, error::Error};

    #[test]
    fn optional_delimiter() {
        assert_eq!(
            opt_delimited::<&str, &str, &str, Error<&str>, _, _, _>(
                skip_ws_and_comments(tag("1")),
                skip_ws_and_comments(tag("ab")),
                skip_ws_and_comments(tag("2"))
            )("1ab2"),
            Ok(("", "ab"))
        );
        assert_eq!(
            opt_delimited::<char, &str, char, Error<&str>, _, _, _>(
                skip_ws_and_comments(char('(')),
                skip_ws_and_comments(tag("ab")),
                skip_ws_and_comments(char(')'))
            )("ab"),
            Ok(("", "ab"))
        );
        assert_eq!(
            opt_delimited::<char, &str, char, Error<&str>, _, _, _>(
                skip_ws_and_comments(char('(')),
                skip_ws_and_comments(tag("ab")),
                skip_ws_and_comments(char(')'))
            )("( abc"),
            Err(nom::Err::Error(Error {
                input: "c",
                code: nom::error::ErrorKind::Char
            }))
        );
        assert_eq!(
            opt_delimited::<char, &str, char, Error<&str>, _, _, _>(
                skip_ws_and_comments(char('(')),
                skip_ws_and_comments(tag("ab")),
                skip_ws_and_comments(char(')'))
            )(" ab )"),
            Ok((" )", "ab"))
        );
        assert_eq!(
            in_parentheses(opt_delimited::<char, &str, char, Error<&str>, _, _, _>(
                skip_ws_and_comments(char('(')),
                skip_ws_and_comments(tag("ab")),
                skip_ws_and_comments(char(')'))
            ))("(( ab ))"),
            Ok(("", "ab"))
        );
        assert_eq!(
            many1(in_parentheses(opt_delimited::<
                char,
                ASN1Value,
                char,
                Error<&str>,
                _,
                _,
                _,
            >(
                skip_ws_and_comments(char(LEFT_PARENTHESIS)),
                skip_ws_and_comments(asn1_value),
                skip_ws_and_comments(char(RIGHT_PARENTHESIS))
            )))("((5))"),
            Ok(("", vec![ASN1Value::Integer(5)]))
        );
    }

    #[test]
    fn takes_until_and_not() {
        assert_eq!(
            take_until_and_not::<nom::error::Error<&str>>("\"", "\"\"")(
                r#"[a-zA-Z]#""(1,8)""(-[a-zA-Z0-9]#(1,8))*""#
            )
            .unwrap()
            .1,
            r#"[a-zA-Z]#""(1,8)""(-[a-zA-Z0-9]#(1,8))*"#
        );
        assert_eq!(
            take_until_and_not::<nom::error::Error<&str>>("\"", "\"\"")(
                r#"[a-zA-Z]#(1,8)""(-[a-zA-Z0-9]#(1,8))*""#
            )
            .unwrap()
            .1,
            r#"[a-zA-Z]#(1,8)""(-[a-zA-Z0-9]#(1,8))*"#
        );
        assert_eq!(
            take_until_and_not::<nom::error::Error<&str>>("\"", "\"\"")(
                r#"[a-zA-Z]#(1,8)(-[a-zA-Z0-9]#(1,8))*""#
            )
            .unwrap()
            .1,
            r#"[a-zA-Z]#(1,8)(-[a-zA-Z0-9]#(1,8))*"#
        )
    }
}
