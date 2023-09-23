use nom::{
    bytes::complete::tag,
    character::complete::{char, one_of},
    combinator::{map, opt, recognize},
    multi::many1,
    sequence::{delimited, preceded},
    IResult,
};

use crate::intermediate::{types::{GeneralizedTime, UTCTime}, ASN1Type, ASN1Value, GENERALIZED_TIME, UTC_TIME};

use super::{common::skip_ws_and_comments, constraint::constraint};

pub fn time_value<'a>(input: &'a str) -> IResult<&'a str, ASN1Value> {
    map(skip_ws_and_comments(t_string), |t_string| {
        ASN1Value::Time(t_string.to_owned())
    })(input)
}

pub fn generalized_time<'a>(input: &'a str) -> IResult<&'a str, ASN1Type> {
    map(
        skip_ws_and_comments(preceded(tag(GENERALIZED_TIME), opt(constraint))),
        |cnst| {
            ASN1Type::GeneralizedTime(GeneralizedTime {
                constraints: cnst.unwrap_or_default(),
            })
        },
    )(input)
}

pub fn utc_time<'a>(input: &'a str) -> IResult<&'a str, ASN1Type> {
    map(
        skip_ws_and_comments(preceded(tag(UTC_TIME), opt(constraint))),
        |cnst| {
            ASN1Type::UTCTime(UTCTime {
                constraints: cnst.unwrap_or_default(),
            })
        },
    )(input)
}

/// Parses a time value character string
/// ### X680
/// _A "tstring" shall consist of one or more of the characters:_
/// _0 1 2 3 4 5 6 7 8 9 + - : . , / C D H M R P S T W Y Z_
/// _preceded and followed by a QUOTATION MARK (34) character (")._
fn t_string<'a>(input: &'a str) -> IResult<&'a str, &'a str> {
    delimited(
        char('"'),
        recognize(many1(one_of("0123456789+-:.,/CDHMRPSTWYZ"))),
        char('"'),
    )(input)
}