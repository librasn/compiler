//! provides a [custom input type](https://github.com/rust-bakery/nom/blob/54557471141b73ca3b2d07be88d6709a43495b10/doc/custom_input_types.md)
//! for parsing ASN.1 sources with [`nom``](https://github.com/rust-bakery/nom).
//! The `Input` type is a thin wrapper around a string slice, with additional
//! data for debugging purposes.
//! This module owes a lot to fflorent's [`nom_locate`](https://github.com/fflorent/nom_locate).

use std::{
    fmt::Debug,
    ops::RangeTo,
    str::{CharIndices, Chars, FromStr},
};

use nom::{
    AsBytes, Compare, ExtendInto, FindSubstring, FindToken, IResult, InputIter, InputLength,
    InputTake, InputTakeAtPosition, Offset, ParseTo, Slice,
};
use rand::{rngs::SmallRng, RngCore, SeedableRng};

/// Tracks the parser name and success in the parser `Input`.
pub fn with_parser<'a, F, O: Debug>(
    parser_name: &'static str,
    mut inner: F,
) -> impl FnMut(Input<'a>) -> IResult<Input<'a>, O>
where
    F: FnMut(Input<'a>) -> IResult<Input<'a>, O>,
{
    move |mut input| {
        let uuid = input.track_parser(parser_name, false);
        match inner(input) {
            Ok((mut rem, res)) => {
                rem.track_parser_success(uuid);
                Ok((rem, res))
            }
            res => res,
        }
    }
}

/// Delimits a parser or set of parsers that are tracked as one operation
/// in the parser's `Input`. In particular, `tracking_boundary` resets the `Input`'s tracking arrays.
pub fn tracking_boundary<'a, F, O>(mut inner: F) -> impl FnMut(Input<'a>) -> IResult<Input<'a>, O>
where
    F: FnMut(Input<'a>) -> IResult<Input<'a>, O>,
{
    move |mut input| {
        let tracked = input.drain_tracked_parsers();
        inner(input).map(|(mut res, o)| {
            res.reset_parser_tracking();
            res.add_untracked(&tracked);
            (res, o)
        })
    }
}

/// Success status of an applied parser. Used for tracking the parser's progress within a tracking boundary.
#[derive(Debug, Clone, PartialEq)]
pub struct TrackedParser {
    pub name: &'static str,
    pub success: bool,
    // UUID for tracking a specific parser call
    pub uuid: u32,
}

impl TrackedParser {
    pub fn new(name: &'static str, success: bool, uuid: u32) -> Self {
        Self {
            name,
            success,
            uuid,
        }
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
    /// current column position of parser, starts at 1
    column: usize,
    /// current offset of parser from start of initial input, starts at 0
    offset: usize,
    /// tracks the applied parsers within an tracking boundary
    tracked_parsers: Vec<TrackedParser>,
}

impl Input<'_> {
    pub fn tracked_parsers(&self) -> &[TrackedParser] {
        &self.tracked_parsers
    }

    pub fn drain_tracked_parsers(&mut self) -> Vec<TrackedParser> {
        self.tracked_parsers.drain(..).collect()
    }

    pub fn tracked_parser_name_success_at(&self, index: usize) -> Option<(&'static str, bool)> {
        self.tracked_parsers.get(index).map(|p| (p.name, p.success))
    }

    pub fn line(&self) -> usize {
        self.line
    }

    pub fn column(&self) -> usize {
        self.column
    }

    pub fn offset(&self) -> usize {
        self.offset
    }

    pub fn add_untracked(&mut self, parsers: &[TrackedParser]) {
        let mut i = 0;
        while let (own, Some(other)) = (self.tracked_parsers.get(i), parsers.get(i)) {
            if own != Some(other) {
                self.tracked_parsers.push(other.clone());
            }
            i += 1;
        }
    }

    fn track_parser(&mut self, parser: &'static str, success: bool) -> u32 {
        let uuid = SmallRng::from_entropy().next_u32();
        self.tracked_parsers
            .push(TrackedParser::new(parser, success, uuid));
        uuid
    }

    fn track_parser_success(&mut self, uuid: u32) {
        if let Some(parser) = self.tracked_parsers.iter_mut().find(|p| p.uuid == uuid) {
            parser.success = true;
        }
    }

    fn reset_parser_tracking(&mut self) {
        self.tracked_parsers.clear();
    }

    fn with_line_column_and_offset(&self, line: usize, column: usize, offset: usize) -> Self {
        Self {
            inner: self.inner,
            line,
            column,
            offset,
            tracked_parsers: self.tracked_parsers.clone(),
        }
    }
}

impl<'a> From<&'a str> for Input<'a> {
    fn from(value: &'a str) -> Self {
        Self {
            inner: value,
            line: 1,
            column: 1,
            offset: 0,
            tracked_parsers: Vec::new(),
        }
    }
}

impl AsBytes for Input<'_> {
    fn as_bytes(&self) -> &[u8] {
        self.inner.as_bytes()
    }
}

impl<'a, 'b> Compare<&'b str> for Input<'a> {
    fn compare(&self, t: &'b str) -> nom::CompareResult {
        self.inner.compare(t)
    }

    fn compare_no_case(&self, t: &'b str) -> nom::CompareResult {
        self.inner.compare_no_case(t)
    }
}

impl<'a, 'b> Compare<&'b [u8]> for Input<'a> {
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

impl<'a, 'b> FindSubstring<&'b str> for Input<'a> {
    fn find_substring(&self, substr: &'b str) -> Option<usize> {
        self.inner.find_substring(substr)
    }
}

impl<'a, 'b> FindToken<&'b u8> for Input<'a> {
    fn find_token(&self, token: &'b u8) -> bool {
        self.inner.find_token(token)
    }
}

impl<'a> FindToken<u8> for Input<'a> {
    fn find_token(&self, token: u8) -> bool {
        self.inner.find_token(token)
    }
}

impl<'a> FindToken<char> for Input<'a> {
    fn find_token(&self, token: char) -> bool {
        self.inner.find_token(token)
    }
}

impl<'a> InputIter for Input<'a> {
    type Item = char;
    type Iter = CharIndices<'a>;
    type IterElem = Chars<'a>;

    #[inline]
    fn iter_indices(&self) -> Self::Iter {
        self.inner.iter_indices()
    }

    #[inline]
    fn iter_elements(&self) -> Self::IterElem {
        self.inner.iter_elements()
    }

    #[inline]
    fn position<P>(&self, predicate: P) -> Option<usize>
    where
        P: Fn(Self::Item) -> bool,
    {
        self.inner.position(predicate)
    }

    #[inline]
    fn slice_index(&self, count: usize) -> Result<usize, nom::Needed> {
        self.inner.slice_index(count)
    }
}

impl InputLength for Input<'_> {
    fn input_len(&self) -> usize {
        self.inner.len()
    }
}

impl InputTake for Input<'_> {
    fn take(&self, count: usize) -> Self {
        self.slice(..count)
    }

    fn take_split(&self, count: usize) -> (Self, Self) {
        (self.slice(count..), self.slice(..count))
    }
}

impl<'a, R> Slice<R> for Input<'a>
where
    &'a str: Slice<R> + Slice<RangeTo<usize>>,
{
    fn slice(&self, range: R) -> Self {
        let inner = self.inner.slice(range);
        let consumed_len = self.inner.offset(inner);
        if consumed_len == 0 {
            Input {
                line: self.line,
                column: self.column,
                offset: self.offset,
                inner,
                tracked_parsers: self.tracked_parsers.clone(),
            }
        } else {
            let consumed = self.inner.slice(..consumed_len);
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
                inner,
                offset: self.offset + consumed_len,
                tracked_parsers: self.tracked_parsers.clone(),
            }
        }
    }
}

impl InputTakeAtPosition for Input<'_> {
    type Item = char;

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
    use nom::{
        bytes::complete::{tag, take_until},
        error::Error,
        sequence::{pair, tuple},
    };

    use crate::lexer::alt::alt;

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

    #[test]
    fn tracks_applied_parsers() {
        let (remaining, _) = pair(
            with_parser("first", tag("test1")),
            with_parser("second", tag("test2")),
        )(Input::from("test1test2test3"))
        .unwrap();
        assert_eq!(remaining.line(), 1);
        assert_eq!(remaining.column(), 11);
        assert_eq!(remaining.offset(), 10);
        assert_eq!(
            remaining.tracked_parser_name_success_at(0).unwrap(),
            ("first", true)
        );
        assert_eq!(
            remaining.tracked_parser_name_success_at(1).unwrap(),
            ("second", true)
        );
    }

    #[test]
    fn tracks_alt_applied_parsers() {
        let (remaining, _) = alt((
            with_parser("first", tag("not matching")),
            with_parser("second", tag("not matching either")),
            with_parser("third", take_until("test3")),
        ))(Input::from("test1\n  test2\n  test3"))
        .unwrap();
        assert_eq!(remaining.line(), 3);
        assert_eq!(remaining.column(), 4);
        assert_eq!(remaining.offset(), 16);
        println!("{:?}", remaining.tracked_parsers());

        assert_eq!(
            remaining.tracked_parser_name_success_at(0).unwrap(),
            ("first", false)
        );
        assert_eq!(
            remaining.tracked_parser_name_success_at(1).unwrap(),
            ("second", false)
        );
        assert_eq!(
            remaining.tracked_parser_name_success_at(2).unwrap(),
            ("third", true)
        );
    }

    #[test]
    fn respects_tracking_boundaries() {
        let mut parser = pair(
            tracking_boundary(alt((
                with_parser("how", tag("Hello")),
                with_parser("can", tag("stranger")),
                with_parser("it", take_until("\n")),
            ))),
            tracking_boundary(alt((
                with_parser("escape", tag("do they have")),
                with_parser("cities", tag("\nmore important things to do")),
            ))),
        );
        if let nom::Err::Error(result_1) = parser(Input::from("Hello, stranger")).unwrap_err() {
            assert_eq!(
                result_1.input.tracked_parser_name_success_at(0).unwrap(),
                ("escape", false)
            );
            assert_eq!(
                result_1.input.tracked_parser_name_success_at(1).unwrap(),
                ("cities", false)
            );
        } else {
            panic!("Expected result_1 to be recoverable error.");
        };
        if let nom::Err::Error(result_2) =
            parser(Input::from("do they have more important things to do")).unwrap_err()
        {
            assert_eq!(
                result_2.input.tracked_parser_name_success_at(0).unwrap(),
                ("how", false)
            );
            assert_eq!(
                result_2.input.tracked_parser_name_success_at(1).unwrap(),
                ("can", false)
            );
            assert_eq!(
                result_2.input.tracked_parser_name_success_at(2).unwrap(),
                ("it", false)
            );
        } else {
            panic!("Expected result_2 to be recoverable error.");
        };
        let (result_3, _) =
            parser(Input::from("do they have\nmore important things to do")).unwrap();
        assert_eq!(
            result_3.tracked_parser_name_success_at(0).unwrap(),
            ("escape", false)
        );
        assert_eq!(
            result_3.tracked_parser_name_success_at(1).unwrap(),
            ("cities", true)
        );
    }

    #[test]
    fn respects_nested_tracking_boundaries() {
        let mut parser = tuple((
            with_parser("loud", tag("he ")),
            with_parser(
                "city",
                tracking_boundary(tuple((
                    with_parser("in", tag("is ")),
                    with_parser("a", tag("running ")),
                    with_parser("suit", tag("through ")),
                ))),
            ),
            with_parser(
                "made",
                pair(
                    with_parser("out", tag("my ")),
                    with_parser("of", tag("eyes")),
                ),
            ),
            with_parser("song", take_until("\n")),
        ));
        if let nom::Err::Error(result_1) =
            parser(Input::from("he is important things")).unwrap_err()
        {
            assert_eq!(
                result_1.input.tracked_parser_name_success_at(0).unwrap(),
                ("in", true)
            );
            assert_eq!(
                result_1.input.tracked_parser_name_success_at(1).unwrap(),
                ("a", false)
            );
        } else {
            panic!("Expected result_2 to be recoverable error.");
        };
        let (result_2, _) = parser(Input::from("he is running through my eyes\neyes")).unwrap();
        assert_eq!(
            result_2
                .tracked_parsers()
                .iter()
                .filter_map(|p| p.success.then_some(p.name))
                .collect::<Vec<_>>()
                .join(" "),
            String::from("loud city made out of song")
        );
    }
}
