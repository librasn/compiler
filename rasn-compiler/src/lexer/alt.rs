//! contains a modified version of `nom::branch::alt` that allows for
//! tracking the tried branches of an `alt` parser.
use nom::{error::ParseError, IResult, Parser};

use super::input::Input;

pub trait Alt<'a, O> {
    fn choice(&mut self, input: Input<'a>) -> IResult<Input<'a>, O>;
}

pub fn alt<'a, O, List: Alt<'a, O>>(mut l: List) -> impl FnMut(Input<'a>) -> IResult<Input<'a>, O> {
    move |i: Input<'a>| l.choice(i)
}

macro_rules! succ (
    (0, $submac:ident ! ($($rest:tt)*)) => ($submac!(1, $($rest)*));
    (1, $submac:ident ! ($($rest:tt)*)) => ($submac!(2, $($rest)*));
    (2, $submac:ident ! ($($rest:tt)*)) => ($submac!(3, $($rest)*));
    (3, $submac:ident ! ($($rest:tt)*)) => ($submac!(4, $($rest)*));
    (4, $submac:ident ! ($($rest:tt)*)) => ($submac!(5, $($rest)*));
    (5, $submac:ident ! ($($rest:tt)*)) => ($submac!(6, $($rest)*));
    (6, $submac:ident ! ($($rest:tt)*)) => ($submac!(7, $($rest)*));
    (7, $submac:ident ! ($($rest:tt)*)) => ($submac!(8, $($rest)*));
    (8, $submac:ident ! ($($rest:tt)*)) => ($submac!(9, $($rest)*));
    (9, $submac:ident ! ($($rest:tt)*)) => ($submac!(10, $($rest)*));
    (10, $submac:ident ! ($($rest:tt)*)) => ($submac!(11, $($rest)*));
    (11, $submac:ident ! ($($rest:tt)*)) => ($submac!(12, $($rest)*));
    (12, $submac:ident ! ($($rest:tt)*)) => ($submac!(13, $($rest)*));
    (13, $submac:ident ! ($($rest:tt)*)) => ($submac!(14, $($rest)*));
    (14, $submac:ident ! ($($rest:tt)*)) => ($submac!(15, $($rest)*));
    (15, $submac:ident ! ($($rest:tt)*)) => ($submac!(16, $($rest)*));
    (16, $submac:ident ! ($($rest:tt)*)) => ($submac!(17, $($rest)*));
    (17, $submac:ident ! ($($rest:tt)*)) => ($submac!(18, $($rest)*));
    (18, $submac:ident ! ($($rest:tt)*)) => ($submac!(19, $($rest)*));
    (19, $submac:ident ! ($($rest:tt)*)) => ($submac!(20, $($rest)*));
    (20, $submac:ident ! ($($rest:tt)*)) => ($submac!(21, $($rest)*));
  );

macro_rules! alt_trait(
    ($first:ident $second:ident $($id: ident)+) => (
      alt_trait!(__impl $first $second; $($id)+);
    );
    (__impl $($current:ident)*; $head:ident $($id: ident)+) => (
      alt_trait_impl!($($current)*);

      alt_trait!(__impl $($current)* $head; $($id)+);
    );
    (__impl $($current:ident)*; $head:ident) => (
      alt_trait_impl!($($current)*);
      alt_trait_impl!($($current)* $head);
    );
  );

macro_rules! alt_trait_impl(
    ($($id:ident)+) => (
      impl<
        'a,
        Output,
        $($id: Parser<Input<'a>, Output, nom::error::Error<Input<'a>>>),+
      > Alt<'a, Output> for ( $($id),+ ) {

        fn choice(&mut self, mut input: Input<'a>) -> IResult<Input<'a>, Output> {
          match self.0.parse(input.clone()) {
            Err(nom::Err::Error(e)) => {
                input.add_untracked(e.input.tracked_parsers());
                alt_trait_inner!(1, self, input, e, $($id)+)
            },
            res => res,
          }
        }
      }
    );
  );

macro_rules! alt_trait_inner(
    ($it:tt, $self:expr, $input:expr, $err:expr, $head:ident $($id:ident)+) => (
      match $self.$it.parse($input.clone()) {
        Err(nom::Err::Error(e)) => {
          let err = $err.or(e);
          $input.add_untracked(err.input.tracked_parsers());
          succ!($it, alt_trait_inner!($self, $input, err, $($id)+))
        }
        res => res,
      }
    );
    ($it:tt, $self:expr, $input:expr, $err:expr, $head:ident) => (
      Err(nom::Err::Error(nom::error::Error::append($input, nom::error::ErrorKind::Alt, $err)))
    );
  );

alt_trait!(A B C D E F G H I J K L M N O P Q R S T U);

impl<'a, Output, A: Parser<Input<'a>, Output, nom::error::Error<Input<'a>>>> Alt<'a, Output>
    for (A,)
{
    fn choice(&mut self, input: Input<'a>) -> IResult<Input<'a>, Output> {
        self.0.parse(input)
    }
}
