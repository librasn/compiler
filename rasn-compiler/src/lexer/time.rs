use nom::{
    bytes::complete::tag,
    character::complete::{char, one_of},
    combinator::{map, map_res, opt, recognize},
    error::Error,
    multi::many1,
    sequence::{delimited, preceded},
};

use crate::intermediate::{
    types::{GeneralizedTime, UTCTime},
    ASN1Type, ASN1Value, GENERALIZED_TIME, TIME, UTC_TIME,
};

use super::{common::skip_ws_and_comments, constraint::constraint, LexerResult, Span};

pub fn time_value(input: Span) -> LexerResult<ASN1Value> {
    map(skip_ws_and_comments(t_string), |t_string| {
        ASN1Value::Time(t_string.to_string())
    })(input)
}

pub fn time(input: Span) -> LexerResult<ASN1Type> {
    map(
        skip_ws_and_comments(preceded(tag(TIME), opt(constraint))),
        |t| ASN1Type::Time(t.into()),
    )(input)
}

pub fn generalized_time(input: Span) -> LexerResult<ASN1Type> {
    map(
        skip_ws_and_comments(preceded(tag(GENERALIZED_TIME), opt(constraint))),
        |cnst| {
            ASN1Type::GeneralizedTime(GeneralizedTime {
                constraints: cnst.unwrap_or_default(),
            })
        },
    )(input)
}

pub fn utc_time(input: Span) -> LexerResult<ASN1Type> {
    map(
        skip_ws_and_comments(preceded(tag(UTC_TIME), opt(constraint))),
        |cnst| {
            ASN1Type::UTCTime(UTCTime {
                constraints: cnst.unwrap_or_default(),
            })
        },
    )(input)
}

const NON_NUMERIC_TIME_CHARS: [char; 17] = [
    '+', '-', ':', '.', ',', '/', 'C', 'D', 'H', 'M', 'R', 'P', 'S', 'T', 'W', 'Y', 'Z',
];

/// Parses a time value character string
/// ### X680
/// _A "tstring" shall consist of one or more of the characters:_
/// _0 1 2 3 4 5 6 7 8 9 + - : . , / C D H M R P S T W Y Z_
/// _preceded and followed by a QUOTATION MARK (34) character (")._
fn t_string(input: Span) -> LexerResult<Span> {
    delimited(
        char('"'),
        map_res(
            recognize(many1(one_of("0123456789+-:.,/CDHMRPSTWYZ"))),
            |tstring: Span| {
                if tstring.contains(char::is_numeric)
                    && tstring.contains(|c| NON_NUMERIC_TIME_CHARS.contains(&c))
                {
                    Ok(tstring)
                } else {
                    Err(nom::Err::Error(Error {
                        input,
                        code: nom::error::ErrorKind::Satisfy,
                    }))
                }
            },
        ),
        char('"'),
    )(input)
}
