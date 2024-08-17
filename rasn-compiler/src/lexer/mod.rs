//! The `lexer` module contains the lexer combinator
//! responsible for interpreting the input as ASN1 notation.
//! The lexer is made up of a number of sub-lexers that
//! interpret single elements of ASN1 syntax.SS
//!
//! The `lexer` submodules provide lexers for their
//! respective eponymous ASN1 type, with the exception
//! of `common`, which contains lexers for the more
//! generic elements of ASN1 syntax, and `util`, which
//! contains helper lexers not specific to ASN1's notation.
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::multispace1,
    combinator::{into, map, opt, recognize},
    multi::{many0, many1},
    sequence::{delimited, pair, preceded, terminated, tuple},
};

use crate::intermediate::{information_object::*, *};

use self::{
    bit_string::*, boolean::*, character_string::*, choice::*, common::*, constraint::*,
    embedded_pdv::*, enumerated::*, error::LexerError, external::*, information_object_class::*,
    integer::*, module_reference::*, null::*, object_identifier::*, octet_string::*,
    parameterization::*, real::*, sequence::*, sequence_of::*, set::*, set_of::*, time::*,
};

mod bit_string;
mod boolean;
mod character_string;
mod choice;
mod common;
mod constraint;
mod embedded_pdv;
mod enumerated;
mod error;
mod external;
mod information_object_class;
mod integer;
mod module_reference;
mod null;
mod object_identifier;
mod octet_string;
mod parameterization;
mod real;
mod sequence;
mod sequence_of;
mod set;
mod set_of;
mod time;
mod util;

pub(crate) type Span<'i> = nom_locate::LocatedSpan<&'i str, ()>;
pub(crate) type LexerResult<'i, O> = nom::IResult<Span<'i>, O, nom::error::Error<Span<'i>>>;

#[cfg(test)]
mod tests;

pub fn asn_spec(
    input: Span,
) -> Result<Vec<(ModuleReference, Vec<ToplevelDefinition>)>, LexerError> {
    many1(pair(
        module_reference,
        terminated(
            many0(skip_ws(alt((
                map(
                    top_level_information_declaration,
                    ToplevelDefinition::Information,
                ),
                map(top_level_type_declaration, ToplevelDefinition::Type),
                map(top_level_value_declaration, ToplevelDefinition::Value),
            )))),
            skip_ws_and_comments(alt((encoding_control, end))),
        ),
    ))(input)
    .map(|(_, res)| res)
    .map_err(|e| e.into())
}

fn encoding_control(input: Span) -> LexerResult<Span> {
    delimited(
        skip_ws_and_comments(tag("ENCODING-CONTROL")),
        take_until(END),
        end,
    )(input)
}

fn end(input: Span) -> LexerResult<Span> {
    skip_ws_and_comments(preceded(
        tag(END),
        recognize(many0(alt((comment, multispace1)))),
    ))(input)
}

pub fn top_level_type_declaration(input: Span) -> LexerResult<ToplevelTypeDefinition> {
    into(tuple((
        skip_ws(many0(comment)),
        skip_ws(title_case_identifier),
        opt(parameterization),
        preceded(assignment, pair(opt(asn_tag), asn1_type)),
    )))(input)
}

pub fn top_level_information_declaration(
    input: Span,
) -> LexerResult<ToplevelInformationDefinition> {
    skip_ws(alt((
        top_level_information_object_declaration,
        top_level_object_set_declaration,
        top_level_object_class_declaration,
    )))(input)
}

pub fn asn1_type(input: Span) -> LexerResult<ASN1Type> {
    alt((
        alt((
            null,
            selection_type_choice,
            object_identifier,
            sequence_of,
            sequence,
            set_of,
            set,
            utc_time,
            external,
            embedded_pdv,
            instance_of,
            generalized_time,
            real,
        )),
        alt((
            choice,
            integer,
            enumerated,
            boolean,
            bit_string,
            time,
            octet_string,
            character_string,
            map(information_object_field_reference, |i| {
                ASN1Type::InformationObjectFieldReference(i)
            }),
            elsewhere_declared_type,
        )),
    ))(input)
}

pub fn asn1_value(input: Span) -> LexerResult<ASN1Value> {
    alt((
        all_value,
        null_value,
        map(object_identifier_value, ASN1Value::ObjectIdentifier),
        choice_value,
        real_value,
        sequence_value,
        time_value,
        bit_string_value,
        boolean_value,
        integer_value,
        character_string_value,
        elsewhere_declared_value,
    ))(input)
}

pub fn elsewhere_declared_value(input: Span) -> LexerResult<ASN1Value> {
    map(
        pair(
            opt(skip_ws_and_comments(recognize(many1(pair(
                identifier,
                tag(".&"),
            ))))),
            value_identifier,
        ),
        |(p, id)| ASN1Value::ElsewhereDeclaredValue {
            parent: p.map(|span| span.to_string()),
            identifier: id.to_string(),
        },
    )(input)
}

pub fn elsewhere_declared_type(input: Span) -> LexerResult<ASN1Type> {
    map(
        tuple((
            opt(skip_ws_and_comments(recognize(many1(pair(
                identifier,
                tag(".&"),
            ))))),
            skip_ws_and_comments(title_case_identifier),
            opt(skip_ws_and_comments(constraint)),
        )),
        |(parent, id, constraints)| ASN1Type::builtin_or_elsewhere(parent, id, constraints),
    )(input)
}

fn top_level_value_declaration(input: Span) -> LexerResult<ToplevelValueDefinition> {
    alt((
        into(tuple((
            skip_ws(many0(comment)),
            skip_ws(value_identifier),
            skip_ws_and_comments(opt(parameterization)),
            skip_ws_and_comments(asn1_type),
            preceded(assignment, skip_ws_and_comments(asn1_value)),
        ))),
        enumerated_value,
    ))(input)
}

fn top_level_information_object_declaration(
    input: Span,
) -> LexerResult<ToplevelInformationDefinition> {
    into(tuple((
        skip_ws(many0(comment)),
        skip_ws(identifier),
        skip_ws(opt(parameterization)),
        skip_ws(uppercase_identifier),
        preceded(assignment, information_object),
    )))(input)
}

fn top_level_object_set_declaration(input: Span) -> LexerResult<ToplevelInformationDefinition> {
    into(tuple((
        skip_ws(many0(comment)),
        skip_ws(identifier),
        skip_ws(opt(parameterization)),
        skip_ws(uppercase_identifier),
        preceded(assignment, object_set),
    )))(input)
}

fn top_level_object_class_declaration(input: Span) -> LexerResult<ToplevelInformationDefinition> {
    into(tuple((
        skip_ws(many0(comment)),
        skip_ws(uppercase_identifier),
        skip_ws(opt(parameterization)),
        preceded(assignment, alt((type_identifier, information_object_class))),
    )))(input)
}
