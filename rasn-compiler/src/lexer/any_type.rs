use nom::{
    bytes::complete::tag,
    combinator::{opt, value},
    sequence::preceded,
    Parser,
};

use crate::{
    input::Input,
    intermediate::{ASN1Type, ANY, BY, DEFINED},
    lexer::common::identifier,
};

use super::{common::skip_ws_and_comments, error::ParserResult};

/// Parses an AnyType
///
/// Note: "DEFINED BY identifier" is currently discarded.
///
/// # Syntax
///
/// ```text
/// AnyType ::=
///     ANY |
///     ANY DEFINED BY identifier
/// ```
pub fn any_type(input: Input<'_>) -> ParserResult<'_, ASN1Type> {
    value(
        ASN1Type::Any,
        preceded(
            skip_ws_and_comments(tag(ANY)),
            opt(preceded(
                (
                    skip_ws_and_comments(tag(DEFINED)),
                    skip_ws_and_comments(tag(BY)),
                ),
                skip_ws_and_comments(identifier),
            )),
        ),
    )
    .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_any() {
        let input = Input::from("  ANY ");
        let output = any_type(input).unwrap().1;
        assert_eq!(output, ASN1Type::Any)
    }

    #[test]
    fn parses_any_defined_by() {
        let input = Input::from(" ANY DEFINED BY ident ");
        let output = any_type(input).unwrap().1;
        assert_eq!(output, ASN1Type::Any)
    }
}
