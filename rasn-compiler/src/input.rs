//! provides a [custom input type](https://github.com/rust-bakery/nom/blob/54557471141b73ca3b2d07be88d6709a43495b10/doc/custom_input_types.md)
//! for parsing ASN.1 sources with [`nom``](https://github.com/rust-bakery/nom).
//! The `Input` type is a thin wrapper around a [Input]-wrapped string slice, with additional
//! data for debugging purposes.
//! This module owes a lot to fflorent's [`nom_locate`](https://github.com/fflorent/nom_locate).

use std::{
    fmt::Debug,
    slice::SliceIndex,
    str::{CharIndices, Chars, FromStr},
};

use nom::{AsBytes, Compare, ExtendInto, FindSubstring, FindToken, Offset, ParseTo, Parser};

/// Informs `Input` of a context switch.
pub fn context_boundary<'a, F>(
    mut inner: F,
) -> impl Parser<Input<'a>, Output = F::Output, Error = F::Error>
where
    F: Parser<Input<'a>>,
{
    move |mut input: Input<'a>| {
        input.reset_context();
        inner.parse(input)
    }
}

/// A [custom input type](https://github.com/rust-bakery/nom/blob/54557471141b73ca3b2d07be88d6709a43495b10/doc/custom_input_types.md)
/// for parsing ASN.1 sources with [nom](https://github.com/rust-bakery/nom).
/// The `Input` type is a thin wrapper around a string slice, with additional
/// data for debugging purposes.
#[derive(Debug, Clone, PartialEq)]
pub struct Input<'a> {
    inner: &'a str,
    /// current line position of parser, starts at 1
    line: usize,
    /// Starting line of the current context.
    /// The context is an excerpt of an ASN.1 source
    /// that is returned along with the line in which
    /// an error was encountered.
    context_start_line: usize,
    /// Starting offset of the current context.
    /// The context is an excerpt of an ASN.1 source
    /// that is returned along with the line in which
    /// an error was encountered.
    context_start_offset: usize,
    /// current column position of parser, starts at 1
    column: usize,
    /// current offset of parser from start of initial input, starts at 0
    offset: usize,
}

impl<'a> Input<'a> {
    pub fn into_inner(self) -> &'a str {
        self.inner
    }
}

impl Input<'_> {
    pub fn line(&self) -> usize {
        self.line
    }

    pub fn column(&self) -> usize {
        self.column
    }

    pub fn offset(&self) -> usize {
        self.offset
    }

    pub fn context_start_line(&self) -> usize {
        self.context_start_line
    }

    pub fn context_start_offset(&self) -> usize {
        self.context_start_offset
    }

    pub fn inner(&self) -> &str {
        self.inner
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn reset_context(&mut self) {
        self.context_start_line = self.line;
        self.context_start_offset = self.offset;
    }

    #[cfg(test)]
    pub fn with_line_column_and_offset(&self, line: usize, column: usize, offset: usize) -> Self {
        Self {
            inner: self.inner,
            line,
            context_start_line: self.context_start_line,
            context_start_offset: self.context_start_offset,
            column,
            offset,
        }
    }
}

impl<'a> From<&'a str> for Input<'a> {
    fn from(value: &'a str) -> Self {
        Self {
            inner: value,
            line: 1,
            context_start_line: 1,
            context_start_offset: 0,
            column: 1,
            offset: 0,
        }
    }
}

impl AsBytes for Input<'_> {
    fn as_bytes(&self) -> &[u8] {
        self.inner.as_bytes()
    }
}

impl<'b> Compare<&'b str> for Input<'_> {
    fn compare(&self, t: &'b str) -> nom::CompareResult {
        self.inner.compare(t)
    }

    fn compare_no_case(&self, t: &'b str) -> nom::CompareResult {
        self.inner.compare_no_case(t)
    }
}

impl<'b> Compare<&'b [u8]> for Input<'_> {
    fn compare(&self, t: &'b [u8]) -> nom::CompareResult {
        self.inner.compare(t)
    }

    fn compare_no_case(&self, t: &'b [u8]) -> nom::CompareResult {
        self.inner.compare_no_case(t)
    }
}

impl ExtendInto for Input<'_> {
    type Item = char;

    type Extender = String;

    fn new_builder(&self) -> Self::Extender {
        self.inner.new_builder()
    }

    fn extend_into(&self, acc: &mut Self::Extender) {
        self.inner.extend_into(acc)
    }
}

impl<'b> FindSubstring<&'b str> for Input<'_> {
    fn find_substring(&self, substr: &'b str) -> Option<usize> {
        self.inner.find_substring(substr)
    }
}

impl<'b> FindToken<&'b u8> for Input<'_> {
    fn find_token(&self, token: &'b u8) -> bool {
        self.inner.find_token(token)
    }
}

impl FindToken<u8> for Input<'_> {
    fn find_token(&self, token: u8) -> bool {
        self.inner.find_token(token)
    }
}

impl FindToken<char> for Input<'_> {
    fn find_token(&self, token: char) -> bool {
        self.inner.find_token(token)
    }
}

impl<'a> Input<'a> {
    pub fn slice(&self, range: impl SliceIndex<str, Output = str>) -> Self {
        let inner = &self.inner[range];
        let consumed_len = self.inner.offset(inner);
        if consumed_len == 0 {
            Input {
                line: self.line,
                column: self.column,
                offset: self.offset,
                context_start_line: self.context_start_line,
                context_start_offset: self.context_start_offset,
                inner,
            }
        } else {
            let consumed = &self.inner[..consumed_len];
            let line_breaks = consumed.match_indices('\n');
            let last_line_break = line_breaks.clone().last();
            let column = if let Some(last) = last_line_break {
                consumed_len - last.0 + 1 // because we're 1-indexing
            } else {
                self.column + consumed_len
            };
            let line = self.line + line_breaks.count();

            Input {
                line,
                column,
                context_start_line: self.context_start_line,
                context_start_offset: self.context_start_offset,
                inner,
                offset: self.offset + consumed_len,
            }
        }
    }
}

impl<'a> nom::Input for Input<'a> {
    type Item = char;
    type Iter = Chars<'a>;
    type IterIndices = CharIndices<'a>;

    fn split_at_position<P, E: nom::error::ParseError<Self>>(
        &self,
        predicate: P,
    ) -> nom::IResult<Self, Self, E>
    where
        P: Fn(Self::Item) -> bool,
    {
        match self.inner.position(predicate) {
            Some(n) => Ok(self.take_split(n)),
            None => Err(nom::Err::Incomplete(nom::Needed::new(1))),
        }
    }

    fn split_at_position1<P, E: nom::error::ParseError<Self>>(
        &self,
        predicate: P,
        e: nom::error::ErrorKind,
    ) -> nom::IResult<Self, Self, E>
    where
        P: Fn(Self::Item) -> bool,
    {
        match self.inner.position(predicate) {
            Some(0) => Err(nom::Err::Error(E::from_error_kind(self.clone(), e))),
            Some(n) => Ok(self.take_split(n)),
            None => Err(nom::Err::Incomplete(nom::Needed::new(1))),
        }
    }

    fn split_at_position_complete<P, E: nom::error::ParseError<Self>>(
        &self,
        predicate: P,
    ) -> nom::IResult<Self, Self, E>
    where
        P: Fn(Self::Item) -> bool,
    {
        match self.split_at_position(predicate) {
            Err(nom::Err::Incomplete(_)) => Ok(self.take_split(self.input_len())),
            res => res,
        }
    }

    fn input_len(&self) -> usize {
        self.inner.len()
    }

    fn take(&self, count: usize) -> Self {
        self.slice(..count)
    }

    fn take_from(&self, index: usize) -> Self {
        self.slice(index..)
    }

    fn take_split(&self, count: usize) -> (Self, Self) {
        (self.slice(count..), self.slice(..count))
    }

    fn position<P>(&self, predicate: P) -> Option<usize>
    where
        P: Fn(Self::Item) -> bool,
    {
        self.inner.position(predicate)
    }

    fn iter_elements(&self) -> Self::Iter {
        self.inner.iter_elements()
    }

    fn iter_indices(&self) -> Self::IterIndices {
        self.inner.iter_indices()
    }

    fn slice_index(&self, count: usize) -> Result<usize, nom::Needed> {
        self.inner.slice_index(count)
    }

    fn split_at_position1_complete<P, E: nom::error::ParseError<Self>>(
        &self,
        predicate: P,
        e: nom::error::ErrorKind,
    ) -> nom::IResult<Self, Self, E>
    where
        P: Fn(Self::Item) -> bool,
    {
        match self.inner.position(predicate) {
            Some(0) => Err(nom::Err::Error(E::from_error_kind(self.clone(), e))),
            Some(n) => Ok(self.take_split(n)),
            None => {
                if self.inner.input_len() == 0 {
                    Err(nom::Err::Error(E::from_error_kind(self.clone(), e)))
                } else {
                    Ok(self.take_split(self.input_len()))
                }
            }
        }
    }
}

impl Offset for Input<'_> {
    fn offset(&self, second: &Self) -> usize {
        self.inner.offset(second.inner)
    }
}

impl<R: FromStr> ParseTo<R> for Input<'_> {
    fn parse_to(&self) -> Option<R> {
        self.inner.parse_to()
    }
}

#[cfg(test)]
mod tests {
    use nom::{bytes::complete::take_until, error::Error};

    use super::*;

    #[test]
    fn slices_complete() {
        let input = Input::from("test");
        assert_eq!(input, input.slice(..));
    }

    #[test]
    fn slices_from() {
        let input = Input::from("test").slice(1..);
        assert_eq!(input.with_line_column_and_offset(1, 2, 1), input);
    }

    #[test]
    fn slices_to() {
        let input = Input::from("test").slice(..2);
        assert_eq!(input.with_line_column_and_offset(1, 1, 0), input);
    }

    #[test]
    fn slices_range() {
        let input = Input::from("test").slice(1..2);
        assert_eq!(input.with_line_column_and_offset(1, 2, 1), input);
    }

    #[test]
    fn tracks_line_breaks() {
        let input = Input::from("test1\n  test2").slice(6..);
        assert_eq!(input.with_line_column_and_offset(2, 2, 6), input);
    }

    #[test]
    fn tracks_multiple_line_breaks() {
        let input = Input::from("test1\n  test2\n  test3").slice(14..);
        assert_eq!(input.with_line_column_and_offset(3, 2, 14), input);
    }

    #[test]
    fn tracks_line_breaks_while_parsing() {
        let (remaining, _) = take_until::<&str, Input<'_>, Error<Input<'_>>>("test3")(Input::from(
            "test1\n  test2\n  test3",
        ))
        .unwrap();
        assert_eq!(remaining.with_line_column_and_offset(3, 4, 16), remaining);
    }
}
