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
use error::ParserResult;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{char, multispace1},
    combinator::{into, map, opt, recognize},
    multi::{many0, many1},
    sequence::{delimited, pair, preceded, terminated},
    Parser,
};

use crate::lexer::macros::macro_definition;
use crate::{
    input::{context_boundary, Input},
    intermediate::{information_object::*, *},
};
use crate::{intermediate::macros::ToplevelMacroDefinition, AsnSourceUnit};

use self::{
    bit_string::*, boolean::*, character_string::*, choice::*, common::*, constraint::*,
    embedded_pdv::*, enumerated::*, error::LexerError, external::*, information_object_class::*,
    integer::*, null::*, object_identifier::*, octet_string::*, parameterization::*, real::*,
    sequence::*, sequence_of::*, set::*, set_of::*, time::*,
};

mod bit_string;
mod boolean;
mod character_string;
mod choice;
mod common;
mod constraint;
mod embedded_pdv;
mod enumerated;
pub(crate) mod error;
mod external;
mod information_object_class;
mod integer;
pub(crate) mod macros;
mod module_header;
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

#[cfg(test)]
mod tests;

pub fn asn_spec(
    input: AsnSourceUnit,
) -> Result<Vec<(ModuleHeader, Vec<ToplevelDefinition>)>, LexerError> {
    let mut result = Vec::new();
    let mut remaining_input = Input::from(&input);
    loop {
        match asn_module(remaining_input) {
            Ok((remaining, res)) => {
                result.push(res);
                remaining_input = remaining;
                if remaining_input.is_empty() {
                    return Ok(result);
                }
            }
            Err(nom::Err::Error(e)) if e.is_eof_error() => {
                return Ok(result);
            }
            Err(e) => {
                return Err(e.into());
            }
        }
    }
}

pub(crate) fn asn_module(
    input: Input<'_>,
) -> ParserResult<'_, (ModuleHeader, Vec<ToplevelDefinition>)> {
    pair(
        module_header::module_header,
        terminated(
            many0(skip_ws(alt((
                map(object_class_assignement, ToplevelDefinition::Class),
                map(
                    top_level_information_declaration,
                    ToplevelDefinition::Object,
                ),
                map(top_level_type_declaration, ToplevelDefinition::Type),
                map(top_level_value_declaration, ToplevelDefinition::Value),
                map(macro_definition, |m| {
                    ToplevelDefinition::Macro(ToplevelMacroDefinition::from(m))
                }),
            )))),
            context_boundary(skip_ws_and_comments(alt((end, encoding_control)))),
        ),
    )
    .parse(input)
}

fn encoding_control(input: Input<'_>) -> ParserResult<'_, &str> {
    into_inner(delimited(
        skip_ws_and_comments(tag(ENCODING_CONTROL)),
        take_until(END),
        end,
    ))
    .parse(input)
}

fn end(input: Input<'_>) -> ParserResult<'_, &str> {
    skip_ws_and_comments(into_inner(preceded(
        tag(END),
        recognize(many0(alt((comment, into_inner(multispace1))))),
    )))
    .parse(input)
}

pub fn top_level_type_declaration(input: Input<'_>) -> ParserResult<'_, ToplevelTypeDefinition> {
    into((
        skip_ws(many0(comment)),
        skip_ws(type_reference),
        opt(parameterization),
        preceded(assignment, pair(opt(asn_tag), asn1_type)),
    ))
    .parse(input)
}

pub fn top_level_information_declaration(
    input: Input<'_>,
) -> ParserResult<'_, ToplevelInformationDefinition> {
    skip_ws(alt((
        top_level_information_object_declaration,
        top_level_object_set_declaration,
    )))
    .parse(input)
}

pub fn asn1_type(input: Input<'_>) -> ParserResult<'_, ASN1Type> {
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
            map(object_class_field_type, ASN1Type::ObjectClassField),
            elsewhere_declared_type,
        )),
    ))
    .parse(input)
}

pub fn asn1_value(input: Input<'_>) -> ParserResult<'_, ASN1Value> {
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
    ))
    .parse(input)
}

pub fn elsewhere_declared_value(input: Input<'_>) -> ParserResult<'_, ASN1Value> {
    map(
        pair(
            opt(skip_ws_and_comments(recognize(many1(pair(
                identifier,
                tag(".&"),
            ))))),
            value_reference,
        ),
        |(p, id)| ASN1Value::ElsewhereDeclaredValue {
            parent: p.map(|par| par.inner().to_string()),
            identifier: id.into(),
        },
    )
    .parse(input)
}

pub fn elsewhere_declared_type(input: Input<'_>) -> ParserResult<'_, ASN1Type> {
    map(
        (
            opt(skip_ws_and_comments(recognize(many1(pair(
                identifier,
                tag(".&"),
            ))))),
            opt(skip_ws_and_comments(terminated(
                module_reference,
                skip_ws_and_comments(char(DOT)),
            ))),
            skip_ws_and_comments(type_reference),
            opt(skip_ws_and_comments(constraints)),
        ),
        |(parent, module, id, constraints)| {
            ASN1Type::builtin_or_elsewhere(
                parent.map(|p| p.into_inner()),
                module,
                id,
                constraints.unwrap_or_default(),
            )
        },
    )
    .parse(input)
}

fn top_level_value_declaration(input: Input<'_>) -> ParserResult<'_, ToplevelValueDefinition> {
    alt((
        into((
            skip_ws(many0(comment)),
            skip_ws(context_boundary(value_reference)),
            skip_ws_and_comments(opt(parameterization)),
            skip_ws_and_comments(asn1_type),
            preceded(assignment, skip_ws_and_comments(asn1_value)),
        )),
        enumerated_value,
    ))
    .parse(input)
}

fn top_level_information_object_declaration(
    input: Input<'_>,
) -> ParserResult<'_, ToplevelInformationDefinition> {
    into((
        skip_ws(many0(comment)),
        skip_ws(context_boundary(identifier)),
        skip_ws(opt(parameterization)),
        skip_ws(uppercase_identifier),
        preceded(assignment, information_object),
    ))
    .parse(input)
}

fn top_level_object_set_declaration(
    input: Input<'_>,
) -> ParserResult<'_, ToplevelInformationDefinition> {
    into((
        skip_ws(many0(comment)),
        skip_ws(context_boundary(identifier)),
        skip_ws(opt(parameterization)),
        skip_ws(uppercase_identifier),
        preceded(assignment, object_set),
    ))
    .parse(input)
}

#[test]
fn eof_comments() {
    println!(
        "{:#?}",
        pair(
            module_header::module_header,
            terminated(
                many0(skip_ws(alt((
                    map(
                        top_level_information_declaration,
                        ToplevelDefinition::Object,
                    ),
                    map(top_level_type_declaration, ToplevelDefinition::Type),
                    map(top_level_value_declaration, ToplevelDefinition::Value),
                )))),
                context_boundary(skip_ws_and_comments(into_inner(preceded(
                    tag(END),
                    recognize(many0(alt((comment, into_inner(multispace1))))),
                ))))
            )
        )
        .parse(
            r#"
LdapSystemSchema {joint-iso-itu-t ds(5) module(1) ldapSystemSchema(38) 9}
DEFINITIONS ::=
BEGIN

id-oat-supportedFeatures                  OBJECT IDENTIFIER ::= {10 5}

END -- LdapSystemSchema"#
                .into()
        )
        .unwrap()
        .0
    );
}
