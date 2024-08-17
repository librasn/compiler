use std::cmp::min;

use nom::{
    bytes::complete::tag,
    error::{Error, ErrorKind, ParseError},
    Err, FindSubstring, IResult, InputLength, InputTake, Parser, Slice as _,
};

use super::{LexerResult, Span};

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
pub fn take_until_and_not<'a, Error: ParseError<Span<'a>>>(
    end_tag: &'a str,
    however_tag: &'a str,
) -> impl Fn(Span<'a>) -> IResult<Span<'a>, Span<'a>, Error> {
    move |i: Span| {
        fn recursive_until<'a, Error: ParseError<Span<'a>>>(
            i: Span<'a>,
            index: usize,
            t1: &'a str,
            t2: &'a str,
        ) -> IResult<Span<'a>, Span<'a>, Error> {
            match (
                i.slice(index..).find_substring(t1),
                i.slice(index..).find_substring(t2),
            ) {
                (None, _) => Err(Err::Error(Error::from_error_kind(i, ErrorKind::TakeUntil))),
                (Some(offset), None) => Ok(i.take_split(index + offset)),
                (Some(_), Some(offset)) => recursive_until(i, index + offset + 2, t1, t2),
            }
        }
        let res: IResult<_, _, Error> = recursive_until(i, 0, end_tag, however_tag);
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
) -> impl Fn(Span<'a>) -> LexerResult<Span> {
    move |i: Span| {
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
                return Ok((i.slice(index..), i.slice(0..index)));
            };
        }

        if bracket_counter == 0 {
            Ok((i.slice(i.len()..), i))
        } else {
            Err(Err::Error(Error::from_error_kind(i, ErrorKind::TakeUntil)))
        }
    }
}

pub fn opt_delimited<'a, O1, O2, O3, F, G, H>(
    mut first: F,
    mut second: G,
    mut third: H,
) -> impl FnMut(Span<'a>) -> LexerResult<O2>
where
    F: Parser<Span<'a>, O1, Error<Span<'a>>>,
    G: Parser<Span<'a>, O2, Error<Span<'a>>>,
    H: Parser<Span<'a>, O3, Error<Span<'a>>>,
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

    use crate::lexer::common::{in_parentheses, skip_ws_and_comments};
    use crate::lexer::{asn1_value, Span};

    use crate::lexer::util::{opt_delimited, take_until_and_not};

    use crate::intermediate::{ASN1Value, LEFT_PARENTHESIS, RIGHT_PARENTHESIS};
    use nom::character::streaming::char;

    use nom::multi::many1;

    use nom::{bytes::complete::tag, Slice as _};

    #[test]
    fn optional_delimiter_simple() {
        let input = Span::new("1ab2");
        let result = opt_delimited(
            skip_ws_and_comments(tag("1")),
            skip_ws_and_comments(tag("ab")),
            skip_ws_and_comments(tag("2")),
        )(input);
        let expect_remainder = input.slice(4..);
        let expect_value = input.slice(1..3);
        assert_eq!(*expect_remainder, "");
        assert_eq!(*expect_value, "ab");
        assert_eq!(result, Ok((expect_remainder, expect_value)));
    }

    #[test]
    fn optional_delimiter_unmatched_delimiter() {
        let input = Span::new("ab");
        let result = opt_delimited::<char, Span, char, _, _, _>(
            skip_ws_and_comments(char('(')),
            skip_ws_and_comments(tag("ab")),
            skip_ws_and_comments(char(')')),
        )(input);
        let expect_remainder = input.slice(2..);
        let expect_value = input.slice(0..2);
        assert_eq!(*expect_remainder, "");
        assert_eq!(*expect_value, "ab");
        assert_eq!(result, Ok((expect_remainder, expect_value)));
    }

    #[test]
    fn optional_delimiter_missing_end_delimiter() {
        let input = Span::new("( abc");
        let result = opt_delimited(
            skip_ws_and_comments(char('(')),
            skip_ws_and_comments(tag("ab")),
            skip_ws_and_comments(char(')')),
        )(input);
        let expect_remainder = input.slice(4..);
        assert_eq!(*expect_remainder, "c");
        assert_eq!(
            result,
            Err(nom::Err::Error(nom::error::Error {
                input: expect_remainder,
                code: nom::error::ErrorKind::Char
            }))
        );
    }

    #[test]
    fn optional_delimiter_missing_start_delimiter() {
        let input = Span::new(" ab )");
        let result = opt_delimited(
            skip_ws_and_comments(char('(')),
            skip_ws_and_comments(tag("ab")),
            skip_ws_and_comments(char(')')),
        )(input);
        let expect_remainder = input.slice(3..);
        let expect_value = input.slice(1..3);
        assert_eq!(*expect_remainder, " )");
        assert_eq!(*expect_value, "ab");
        assert_eq!(result, Ok((expect_remainder, expect_value)));
    }

    #[test]
    fn optional_delimiter_in_parentheses() {
        let input = Span::new("(( ab ))");
        let result = in_parentheses(opt_delimited(
            skip_ws_and_comments(char('(')),
            skip_ws_and_comments(tag("ab")),
            skip_ws_and_comments(char(')')),
        ))(input);
        let expect_remainder = input.slice(8..);
        let expect_value = input.slice(3..5);
        assert_eq!(*expect_remainder, "");
        assert_eq!(*expect_value, "ab");
        assert_eq!(result, Ok((expect_remainder, expect_value)));
    }

    #[test]
    fn optional_delimiter_parenthesised_list() {
        let input = Span::new("((5))");
        let result = many1(in_parentheses(opt_delimited(
            skip_ws_and_comments(char(LEFT_PARENTHESIS)),
            skip_ws_and_comments(asn1_value),
            skip_ws_and_comments(char(RIGHT_PARENTHESIS)),
        )))(input);
        let expect_remainder = input.slice(5..);
        let expect_value = vec![ASN1Value::Integer(5)];
        assert_eq!(*expect_remainder, "");
        assert_eq!(result, Ok((expect_remainder, expect_value)));
    }

    #[test]
    fn takes_until_and_not_two_however_tag() {
        let input = Span::new(r#"[a-zA-Z]#""(1,8)""(-[a-zA-Z0-9]#(1,8))*""#);
        let result = take_until_and_not::<nom::error::Error<Span>>("\"", "\"\"")(input);
        let expect_remainder = input.slice(39..);
        let expect_value = input.slice(..39);
        assert_eq!(*expect_remainder, "\"");
        assert_eq!(*expect_value, r#"[a-zA-Z]#""(1,8)""(-[a-zA-Z0-9]#(1,8))*"#);
        assert_eq!(result, Ok((expect_remainder, expect_value)));
    }

    #[test]
    fn takes_until_and_not_one_however_tag() {
        let input = Span::new(r#"[a-zA-Z]#(1,8)""(-[a-zA-Z0-9]#(1,8))*""#);
        let result = take_until_and_not::<nom::error::Error<Span>>("\"", "\"\"")(input);
        let expect_remainder = input.slice(37..);
        let expect_value = input.slice(..37);
        assert_eq!(*expect_remainder, "\"");
        assert_eq!(*expect_value, r#"[a-zA-Z]#(1,8)""(-[a-zA-Z0-9]#(1,8))*"#);
        assert_eq!(result, Ok((expect_remainder, expect_value)));
    }

    #[test]
    fn takes_until_and_not_zero_however_tag() {
        let input = Span::new(r#"[a-zA-Z]#(1,8)(-[a-zA-Z0-9]#(1,8))*""#);
        let result = take_until_and_not::<nom::error::Error<Span>>("\"", "\"\"")(input);
        let expect_remainder = input.slice(35..);
        let expect_value = input.slice(..35);
        assert_eq!(*expect_remainder, "\"");
        assert_eq!(*expect_value, r#"[a-zA-Z]#(1,8)(-[a-zA-Z0-9]#(1,8))*"#);
        assert_eq!(result, Ok((expect_remainder, expect_value)));
    }
}
