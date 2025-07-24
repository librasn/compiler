use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::combinator::{cut, map, map_res, not, opt, peek, value};
use nom::multi::{many0, many1, separated_list1};
use nom::sequence::{delimited, pair, preceded, separated_pair, terminated};
use nom::Parser;

use crate::input::{context_boundary, Input};
use crate::intermediate::{
    ASN1Type, ASN1Value, DeclarationElsewhere, ASN1_KEYWORDS, ASSIGN, BEGIN, DOT, END,
    GREATER_THAN, LEFT_PARENTHESIS, LESS_THAN, MACRO, PIPE, RIGHT_PARENTHESIS,
};
use crate::lexer::common::{
    in_parentheses, module_reference, skip_ws_and_comments, type_reference, uppercase_identifier,
    value_reference,
};
use crate::lexer::error::{MiscError, ParserResult};
use crate::lexer::{asn1_type, asn1_value};

use super::character_string::raw_string_literal;

#[derive(Debug, Clone, PartialEq)]
pub struct MacroDefinition<'i> {
    pub name: &'i str,
    pub substance: MacroSubstance<'i>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MacroSubstance<'i> {
    Body(MacroBody<'i>),
    Reference(&'i str),
    ExternalReference {
        module_reference: &'i str,
        macro_reference: &'i str,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct MacroBody<'i> {
    pub type_production: Vec<Vec<SymbolElement<'i>>>,
    pub value_production: Vec<Vec<SymbolElement<'i>>>,
    pub supporting_productions: Vec<Production<'i>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Production<'i> {
    name: &'i str,
    alternatives: Vec<Vec<SymbolElement<'i>>>,
}

/// Parse a macro definition.
///
/// # Syntax
///
/// ```text
/// MacroDefinition ::=
///     macroreference MACRO "::=" MacroSubstance
/// ```
pub fn macro_definition(input: Input<'_>) -> ParserResult<'_, MacroDefinition> {
    map(
        separated_pair(
            skip_ws_and_comments(macro_reference),
            pair(
                skip_ws_and_comments(tag(MACRO)),
                skip_ws_and_comments(tag(ASSIGN)),
            ),
            context_boundary(cut(macro_substance)),
        ),
        |v| MacroDefinition {
            name: v.0,
            substance: v.1,
        },
    )
    .parse(input)
}

/// Parse a macro substance.
///
/// # Syntax
///
/// ```text
/// MacroSubstance ::=
///     BEGIN MacroBody END      |
///     macroreference           |
///     Externalmacroreference
/// ```
fn macro_substance(input: Input<'_>) -> ParserResult<'_, MacroSubstance> {
    skip_ws_and_comments(alt((
        map(
            delimited(tag(BEGIN), cut(macro_body), skip_ws_and_comments(tag(END))),
            MacroSubstance::Body,
        ),
        map(macro_reference, MacroSubstance::Reference),
        map(external_macro_reference, |v| {
            MacroSubstance::ExternalReference {
                module_reference: v.0,
                macro_reference: v.1,
            }
        }),
    )))
    .parse(input)
}

/// Parse a macro body.
///
/// # Syntax
///
/// ```text
/// MacroBody ::=
///     TypeProduction
///     ValueProduction
///     SupportingProductions
///
/// TypeProduction ::=
///     TYPE NOTATION "::=" MacroAlternativeList
///
/// ValueProduction ::=
///     VALUE PRODUCTION "::=" MacroAlternativeList
///
/// ```
fn macro_body(input: Input<'_>) -> ParserResult<'_, MacroBody> {
    map(
        (
            preceded(
                pair(
                    skip_ws_and_comments(tag("TYPE NOTATION")),
                    skip_ws_and_comments(tag(ASSIGN)),
                ),
                skip_ws_and_comments(macro_alternative_list),
            ),
            preceded(
                pair(
                    skip_ws_and_comments(tag("VALUE NOTATION")),
                    skip_ws_and_comments(tag(ASSIGN)),
                ),
                skip_ws_and_comments(macro_alternative_list),
            ),
            supporting_productions,
        ),
        |v| MacroBody {
            type_production: v.0,
            value_production: v.1,
            supporting_productions: v.2,
        },
    )
    .parse(input)
}

/// Parse supporting productions.
///
/// # Syntax
///
/// ```text
/// SupportingProductions ::=
///     ProductionList  |
///     empty
///
/// ProductionList ::=
///     Production  |
///     ProductionList Production
///
/// Production ::=
///     productionreference "::=" MacroAlternativeList
/// ```
fn supporting_productions(input: Input<'_>) -> ParserResult<'_, Vec<Production>> {
    many0(map(
        separated_pair(
            skip_ws_and_comments(production_reference),
            skip_ws_and_comments(tag(ASSIGN)),
            skip_ws_and_comments(macro_alternative_list),
        ),
        |v| Production {
            name: v.0,
            alternatives: v.1,
        },
    ))
    .parse(input)
}

/// Parse an external macro reference.
///
/// # Syntax
///
/// ```text
/// Externalmacroreference ::=
///     modulereference "." macroreference
/// ```
fn external_macro_reference(input: Input<'_>) -> ParserResult<'_, (&'_ str, &'_ str)> {
    separated_pair(
        module_reference,
        skip_ws_and_comments(char(DOT)),
        skip_ws_and_comments(macro_reference),
    )
    .parse(input)
}

#[derive(Debug, Clone, PartialEq)]
pub enum SymbolElement<'i> {
    SymbolDefn(SymbolDefn<'i>),
    EmbeddedDefinitions(Vec<EmbeddedDefinition<'i>>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum SymbolDefn<'i> {
    /// A string literal
    AString(String),
    ProductionReference(&'i str),
    /// "string"
    String,
    /// "identifier"
    Identifier,
    /// "number"
    Number,
    /// "empty"
    Empty,
    /// type | type(localtypereference)
    Type {
        local_type_reference: Option<&'i str>,
    },
    /// value(MacroType)
    ValueMacroType(ASN1Type),
    /// value(VALUE MacroType)
    ValueVALUEMacroType(ASN1Type),
    /// value(localvaluereference MacroType)
    ValueLocalvaluereferenceMacroType {
        local_value_reference: &'i str,
        ty: ASN1Type,
    },
}

/// Parse a macro alternative list.
///
/// # Syntax
///
/// ```text
/// MacroAlternativeList ::=
///     MacroAlternative  |
///     MacroAlternativeList "|" MacroAlternative
///
/// MacroAlternative ::= SymbolList
/// ```
fn macro_alternative_list(input: Input<'_>) -> ParserResult<'_, Vec<Vec<SymbolElement>>> {
    separated_list1(skip_ws_and_comments(tag(PIPE)), symbol_list).parse(input)
}

/// Parse a symbol list.
///
/// # Syntax
///
/// ```text
/// SymbolList ::=
///     SymbolElement  |
///     SymbolList SymbolElement
///
/// SymbolElement
///     SymbolDefn  |
///     EmbeddedDefinitions
/// ```
fn symbol_list(input: Input<'_>) -> ParserResult<'_, Vec<SymbolElement>> {
    many1(terminated(
        skip_ws_and_comments(alt((
            map(symbol_defn, SymbolElement::SymbolDefn),
            map(embedded_definitions, SymbolElement::EmbeddedDefinitions),
        ))),
        skip_ws_and_comments(not(peek(tag(ASSIGN)))),
    ))
    .parse(input)
}

/// Parse a symbol definition.
///
/// # Syntax
///
/// ```text
/// SymbolDefn ::=
///     astring                    |
///     productionreference        |
///     "string"                   |
///     "identifier"               |
///     "number"                   |
///     "empty"                    |
///     type                       |
///     type(localtypereference)   |
///     value(MacroType)           |
///     value(VALUE MacroType)     |
///     value(localvaluereference MacroType)
/// ```
fn symbol_defn(input: Input<'_>) -> ParserResult<'_, SymbolDefn<'_>> {
    skip_ws_and_comments(alt((
        map(astring, SymbolDefn::AString),
        map(production_reference, SymbolDefn::ProductionReference),
        value(SymbolDefn::String, tag("string")),
        value(SymbolDefn::Identifier, tag("identifier")),
        value(SymbolDefn::Number, tag("number")),
        value(SymbolDefn::Empty, tag("empty")),
        map(
            preceded(tag("type"), opt(in_parentheses(cut(local_type_reference)))),
            |v| SymbolDefn::Type {
                local_type_reference: v,
            },
        ),
        preceded(
            (tag("value"), skip_ws_and_comments(char(LEFT_PARENTHESIS))),
            cut(alt((
                map(
                    terminated(macro_type, skip_ws_and_comments(char(RIGHT_PARENTHESIS))),
                    SymbolDefn::ValueMacroType,
                ),
                map(
                    delimited(
                        tag("VALUE"),
                        skip_ws_and_comments(macro_type),
                        skip_ws_and_comments(char(RIGHT_PARENTHESIS)),
                    ),
                    SymbolDefn::ValueVALUEMacroType,
                ),
                map(
                    terminated(
                        (
                            alt((
                                local_value_reference,
                                // Type references are not allowed here, parse it anyway for
                                // compatibility with for example SNMPv2-SMI.
                                local_type_reference,
                            )),
                            skip_ws_and_comments(macro_type),
                        ),
                        skip_ws_and_comments(char(RIGHT_PARENTHESIS)),
                    ),
                    |v| SymbolDefn::ValueLocalvaluereferenceMacroType {
                        local_value_reference: v.0,
                        ty: v.1,
                    },
                ),
            ))),
        ),
    )))
    .parse(input)
}

/// Parse astring, that is, a string literal.
///
/// String literals in MACROs are a bit simpler than regular string literals. The only
/// transformation is to replace any escaped double quotes `""` with a single `"`.
fn astring(input: Input<'_>) -> ParserResult<'_, String> {
    map(raw_string_literal, |s| s.replace("\"\"", "\"")).parse(input)
}

/// Parse a macro type.
///
/// # Syntax
///
/// ```text
/// MacroType ::=
///     localtypereference  |
///     Type
/// ```
fn macro_type(input: Input<'_>) -> ParserResult<'_, ASN1Type> {
    // A localtypeference will be stored as ASN1Type::ElsewhereDeclaredType
    skip_ws_and_comments(map_res(asn1_type, |v| match v {
        ASN1Type::ElsewhereDeclaredType(DeclarationElsewhere { ref identifier, .. })
            if ADDITIONAL_KEYWORDS.contains(&identifier.as_str()) =>
        {
            Err(MiscError(
                "Type reference can not be a keyword when used in ASN.1 MACRO.",
            ))
        }
        _ => Ok(v),
    }))
    .parse(input)
}

#[derive(Debug, Clone, PartialEq)]
pub enum EmbeddedDefinition<'i> {
    Type(LocalTypeassignment<'i>),
    Value(LocalValueassignment<'i>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct LocalTypeassignment<'i> {
    name: &'i str,
    ty: ASN1Type,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LocalValueassignment<'i> {
    name: &'i str,
    ty: ASN1Type,
    value: ASN1Value,
}

/// Parse embedded definitions.
///
/// # Syntax
///
/// ```text
/// EmbeddedDefinitions ::= "<" EmbeddedDefinitionList ">"
///
/// EmbeddedDefinitionList ::=
///     EmbeddedDefinition |
///     EmbeddedDefinitionList EmbeddedDefinition
/// ```
fn embedded_definitions(input: Input<'_>) -> ParserResult<'_, Vec<EmbeddedDefinition>> {
    delimited(
        skip_ws_and_comments(char(LESS_THAN)),
        cut(many1(skip_ws_and_comments(embedded_definition))),
        skip_ws_and_comments(char(GREATER_THAN)),
    )
    .parse(input)
}

/// Parse an embedded definition.
///
/// # Syntax
///
/// ```text
/// EmbeddedDefinition ::=
///     LocalTypeassignment |
///     LocalValueassignment
/// ```
fn embedded_definition(input: Input<'_>) -> ParserResult<'_, EmbeddedDefinition> {
    alt((
        map(local_type_assignement, EmbeddedDefinition::Type),
        map(local_value_assignement, EmbeddedDefinition::Value),
    ))
    .parse(input)
}

/// Parse a local type assignement.
///
/// # Syntax
///
/// ```text
/// LocalTypeReference ::=
///     localtypereference "::=" MacroType
/// ```
fn local_type_assignement(input: Input<'_>) -> ParserResult<'_, LocalTypeassignment> {
    map(
        separated_pair(
            local_type_reference,
            skip_ws_and_comments(tag(ASSIGN)),
            skip_ws_and_comments(macro_type),
        ),
        |v| LocalTypeassignment { name: v.0, ty: v.1 },
    )
    .parse(input)
}

/// Parse a local value assignement.
///
/// # Syntax
///
/// ```text
/// LocalValueassignment ::=
///     localvaluereference MacroType "::=" MacroValue
/// ```
fn local_value_assignement(input: Input<'_>) -> ParserResult<'_, LocalValueassignment> {
    map(
        (
            skip_ws_and_comments(local_value_reference),
            skip_ws_and_comments(macro_type),
            skip_ws_and_comments(preceded(tag(ASSIGN), skip_ws_and_comments(macro_value))),
        ),
        |v| LocalValueassignment {
            name: v.0,
            ty: v.1,
            value: v.2,
        },
    )
    .parse(input)
}

/// Parse a macro value.
///
/// # Syntax
///
/// ```text
/// MacroValue ::=
///     Value      |
///     localvaluereference
/// ```
fn macro_value(input: Input<'_>) -> ParserResult<'_, ASN1Value> {
    // A localvaluereference will be stored as ASN1Value::ElsewhereDeclaredValue
    skip_ws_and_comments(map_res(asn1_value, |v| match v {
        ASN1Value::ElsewhereDeclaredValue { ref identifier, .. }
            if ADDITIONAL_KEYWORDS.contains(&identifier.as_str()) =>
        {
            Err(MiscError(
                "Value reference can not be a keyword when used in ASN.1 MACRO.",
            ))
        }
        _ => Ok(v),
    }))
    .parse(input)
}

/// Keywords not allowed for some MACRO items.
///
/// The keywords are not allowed in the following items:
/// - Productionreference
/// - Localtypereference
/// - Localvaluereference
const ADDITIONAL_KEYWORDS: [&str; 6] = ["MACRO", "TYPE", "NOTATION", "VALUE", "value", "type"];

/// Parse a macro reference.
fn macro_reference(input: Input<'_>) -> ParserResult<'_, &'_ str> {
    map_res(uppercase_identifier, |v| {
        if ASN1_KEYWORDS.contains(&v) {
            Err(MiscError("Macro reference can not be a ASN.1 keyword."))
        } else if ADDITIONAL_KEYWORDS.contains(&v) {
            Err(MiscError(
                "Macro reference can not be a keyword when used in ASN.1 MACRO.",
            ))
        } else {
            Ok(v)
        }
    })
    .parse(input)
}

/// Parse a production reference.
///
/// #### X.680 1994 J.2.2 Productionreference
/// _A "productionreference" shall consist of the sequence of characters specified for a
/// "typereference" in 9.2._
fn production_reference(input: Input<'_>) -> ParserResult<'_, &'_ str> {
    map_res(type_reference, |v| {
        if ADDITIONAL_KEYWORDS.contains(&v) {
            Err(MiscError(
                "Production reference can not be a keyword when used in ASN.1 MACRO.",
            ))
        } else {
            Ok(v)
        }
    })
    .parse(input)
}

/// Parse a local type reference.
///
/// #### X.680 1994 J.2.3 Localtypereference
/// _A "localtypereference" shall consist of the sequence of characters specified for a
/// "typereference" in 9.2. A "localtypereference" is used as an identifier for types which are
/// recognized during syntax analysis of an instance of the new type or value notation._
fn local_type_reference(input: Input<'_>) -> ParserResult<'_, &'_ str> {
    map_res(type_reference, |v| {
        if ADDITIONAL_KEYWORDS.contains(&v) {
            Err(MiscError(
                "Type reference can not be a keyword when used in ASN.1 MACRO.",
            ))
        } else {
            Ok(v)
        }
    })
    .parse(input)
}

/// Parse a local value reference
fn local_value_reference(input: Input<'_>) -> ParserResult<'_, &'_ str> {
    map_res(value_reference, |v| {
        if ADDITIONAL_KEYWORDS.contains(&v) {
            Err(MiscError(
                "Value reference can not be a keyword when used in ASN.1 MACRO.",
            ))
        } else {
            Ok(v)
        }
    })
    .parse(input)
}

#[cfg(test)]
mod tests {
    use crate::input::Input;
    use crate::intermediate::types::ObjectIdentifier;
    use crate::intermediate::{ASN1Type, ASN1Value, DeclarationElsewhere};
    use crate::lexer::macros::{
        embedded_definitions, local_type_assignement, local_value_assignement, macro_body,
        macro_definition, supporting_productions, symbol_list, EmbeddedDefinition,
        LocalTypeassignment, LocalValueassignment, MacroBody, MacroDefinition, MacroSubstance,
        Production, SymbolDefn, SymbolElement,
    };
    use crate::lexer::types::{Boolean, Integer};

    #[test]
    fn parses_snmp_trap_type() {
        let input = Input::from(
            r#"
            TRAP-TYPE MACRO ::=
            BEGIN
                TYPE NOTATION ::= "ENTERPRISE" value
                                (enterprise OBJECT IDENTIFIER)
                                VarPart
                                DescrPart
                                ReferPart
                VALUE NOTATION ::= value (VALUE INTEGER)
                VarPart ::=
                        "VARIABLES" "{" VarTypes "}"
                        | empty
                VarTypes ::=
                        VarType | VarTypes "," VarType
                VarType ::=
                        value (vartype ObjectName)
                DescrPart ::=
                        "DESCRIPTION" value (description DisplayString)
                        | empty
                ReferPart ::=
                        "REFERENCE" value (reference DisplayString)
                        | empty
            END
            "#,
        );

        let (_, result) = macro_definition(input).unwrap();

        assert_eq!(
            result,
            MacroDefinition {
                name: "TRAP-TYPE",
                substance: MacroSubstance::Body(MacroBody {
                    type_production: vec![vec![
                        SymbolElement::SymbolDefn(SymbolDefn::AString("ENTERPRISE".to_string())),
                        SymbolElement::SymbolDefn(SymbolDefn::ValueLocalvaluereferenceMacroType {
                            local_value_reference: "enterprise",
                            ty: ASN1Type::ObjectIdentifier(ObjectIdentifier {
                                constraints: vec![]
                            })
                        }),
                        SymbolElement::SymbolDefn(SymbolDefn::ProductionReference("VarPart")),
                        SymbolElement::SymbolDefn(SymbolDefn::ProductionReference("DescrPart")),
                        SymbolElement::SymbolDefn(SymbolDefn::ProductionReference("ReferPart"))
                    ]],
                    value_production: vec![vec![SymbolElement::SymbolDefn(
                        SymbolDefn::ValueVALUEMacroType(ASN1Type::Integer(Integer {
                            constraints: vec![],
                            distinguished_values: None
                        }))
                    )]],
                    supporting_productions: vec![
                        Production {
                            name: "VarPart",
                            alternatives: vec![
                                vec![
                                    SymbolElement::SymbolDefn(SymbolDefn::AString(
                                        "VARIABLES".to_string()
                                    )),
                                    SymbolElement::SymbolDefn(SymbolDefn::AString("{".to_string())),
                                    SymbolElement::SymbolDefn(SymbolDefn::ProductionReference(
                                        "VarTypes"
                                    )),
                                    SymbolElement::SymbolDefn(SymbolDefn::AString("}".to_string()))
                                ],
                                vec![SymbolElement::SymbolDefn(SymbolDefn::Empty)]
                            ]
                        },
                        Production {
                            name: "VarTypes",
                            alternatives: vec![
                                vec![SymbolElement::SymbolDefn(SymbolDefn::ProductionReference(
                                    "VarType"
                                ))],
                                vec![
                                    SymbolElement::SymbolDefn(SymbolDefn::ProductionReference(
                                        "VarTypes"
                                    )),
                                    SymbolElement::SymbolDefn(SymbolDefn::AString(",".to_string())),
                                    SymbolElement::SymbolDefn(SymbolDefn::ProductionReference(
                                        "VarType"
                                    ))
                                ]
                            ]
                        },
                        Production {
                            name: "VarType",
                            alternatives: vec![vec![SymbolElement::SymbolDefn(
                                SymbolDefn::ValueLocalvaluereferenceMacroType {
                                    local_value_reference: "vartype",
                                    ty: ASN1Type::ElsewhereDeclaredType(DeclarationElsewhere {
                                        parent: None,
                                        module: None,
                                        identifier: "ObjectName".to_string(),
                                        constraints: vec![]
                                    })
                                }
                            )]]
                        },
                        Production {
                            name: "DescrPart",
                            alternatives: vec![
                                vec![
                                    SymbolElement::SymbolDefn(SymbolDefn::AString(
                                        "DESCRIPTION".to_string()
                                    )),
                                    SymbolElement::SymbolDefn(
                                        SymbolDefn::ValueLocalvaluereferenceMacroType {
                                            local_value_reference: "description",
                                            ty: ASN1Type::ElsewhereDeclaredType(
                                                DeclarationElsewhere {
                                                    parent: None,
                                                    module: None,
                                                    identifier: "DisplayString".to_string(),
                                                    constraints: vec![]
                                                }
                                            )
                                        }
                                    )
                                ],
                                vec![SymbolElement::SymbolDefn(SymbolDefn::Empty)]
                            ]
                        },
                        Production {
                            name: "ReferPart",
                            alternatives: vec![
                                vec![
                                    SymbolElement::SymbolDefn(SymbolDefn::AString(
                                        "REFERENCE".to_string()
                                    )),
                                    SymbolElement::SymbolDefn(
                                        SymbolDefn::ValueLocalvaluereferenceMacroType {
                                            local_value_reference: "reference",
                                            ty: ASN1Type::ElsewhereDeclaredType(
                                                DeclarationElsewhere {
                                                    parent: None,
                                                    module: None,
                                                    identifier: "DisplayString".to_string(),
                                                    constraints: vec![]
                                                }
                                            )
                                        }
                                    )
                                ],
                                vec![SymbolElement::SymbolDefn(SymbolDefn::Empty)]
                            ]
                        }
                    ]
                })
            }
        );
    }

    #[test]
    fn parses_macro_body() {
        let input = Input::from(
            r#"
            TYPE NOTATION ::= string number | empty
            VALUE NOTATION ::= value(INTEGER) | Name
            Name ::= "NAME" string
            "#,
        );

        let (rest, result) = macro_body(input).unwrap();

        assert_eq!(rest.inner().trim(), "");
        assert_eq!(
            result,
            MacroBody {
                type_production: vec![
                    vec![
                        SymbolElement::SymbolDefn(SymbolDefn::String),
                        SymbolElement::SymbolDefn(SymbolDefn::Number)
                    ],
                    vec![SymbolElement::SymbolDefn(SymbolDefn::Empty)],
                ],
                value_production: vec![
                    vec![SymbolElement::SymbolDefn(SymbolDefn::ValueMacroType(
                        ASN1Type::Integer(Integer {
                            constraints: vec![],
                            distinguished_values: None,
                        })
                    ))],
                    vec![SymbolElement::SymbolDefn(SymbolDefn::ProductionReference(
                        "Name"
                    ))]
                ],
                supporting_productions: vec![Production {
                    name: "Name",
                    alternatives: vec![vec![
                        SymbolElement::SymbolDefn(SymbolDefn::AString("NAME".to_string())),
                        SymbolElement::SymbolDefn(SymbolDefn::String)
                    ]],
                }],
            }
        );
    }

    #[test]
    fn parses_supporting_productions() {
        let input = Input::from(
            r#"
            Where ::=
                "LOCATION" string Who
            Who   ::=
                "NAME" string number  |
                empty
            "#,
        );

        let (rest, result) = supporting_productions(input).unwrap();

        assert_eq!(rest.inner().trim(), "");
        assert_eq!(
            result,
            vec![
                Production {
                    name: "Where",
                    alternatives: vec![vec![
                        SymbolElement::SymbolDefn(SymbolDefn::AString("LOCATION".to_string())),
                        SymbolElement::SymbolDefn(SymbolDefn::String),
                        SymbolElement::SymbolDefn(SymbolDefn::ProductionReference("Who")),
                    ]],
                },
                Production {
                    name: "Who",
                    alternatives: vec![
                        vec![
                            SymbolElement::SymbolDefn(SymbolDefn::AString("NAME".to_string())),
                            SymbolElement::SymbolDefn(SymbolDefn::String),
                            SymbolElement::SymbolDefn(SymbolDefn::Number),
                        ],
                        vec![SymbolElement::SymbolDefn(SymbolDefn::Empty)]
                    ],
                }
            ]
        );
    }

    #[test]
    fn parses_symbol_list() {
        let input = Input::from(
            r#"
            "astring"
            ProductionRef
            string
            identifier
            number
            empty
            type
            type(LocalTypeRef)
            value(BOOLEAN)
            value(VALUE INTEGER)
            value(local-val-ref BOOLEAN)
            "#,
        );

        let (rest, result) = symbol_list(input).unwrap();

        assert_eq!(rest.inner().trim(), "");

        use SymbolDefn as SD;
        use SymbolElement as SE;

        assert_eq!(
            result,
            vec![
                SE::SymbolDefn(SD::AString("astring".to_string())),
                SE::SymbolDefn(SD::ProductionReference("ProductionRef")),
                SE::SymbolDefn(SD::String),
                SE::SymbolDefn(SD::Identifier),
                SE::SymbolDefn(SD::Number),
                SE::SymbolDefn(SD::Empty),
                SE::SymbolDefn(SD::Type {
                    local_type_reference: None
                }),
                SE::SymbolDefn(SD::Type {
                    local_type_reference: Some("LocalTypeRef")
                }),
                SE::SymbolDefn(SD::ValueMacroType(ASN1Type::Boolean(Boolean {
                    constraints: vec![]
                }))),
                SE::SymbolDefn(SD::ValueVALUEMacroType(ASN1Type::Integer(Integer {
                    constraints: vec![],
                    distinguished_values: None,
                }))),
                SE::SymbolDefn(SD::ValueLocalvaluereferenceMacroType {
                    local_value_reference: "local-val-ref",
                    ty: ASN1Type::Boolean(Boolean {
                        constraints: vec![]
                    })
                }),
            ]
        );
    }

    #[test]
    fn parses_embedded_definitions() {
        let input = Input::from(
            "
            < ALocalType ::= BOOLEAN >
            ",
        );

        let (_, result) = embedded_definitions(input).unwrap();

        assert_eq!(
            result,
            vec![EmbeddedDefinition::Type(LocalTypeassignment {
                name: "ALocalType",
                ty: ASN1Type::Boolean(Boolean {
                    constraints: vec![]
                })
            })]
        );
    }

    #[test]
    fn parses_local_type_assignement() {
        let input = Input::from("ALocalType ::= BOOLEAN");

        let (_, result) = local_type_assignement(input).unwrap();

        assert_eq!(
            result,
            LocalTypeassignment {
                name: "ALocalType",
                ty: ASN1Type::Boolean(Boolean {
                    constraints: vec![]
                }),
            }
        );
    }

    #[test]
    fn parses_local_value_assignement() {
        let input = Input::from("a-local-value BOOLEAN ::= TRUE");

        let (_, result) = local_value_assignement(input).unwrap();

        assert_eq!(
            result,
            LocalValueassignment {
                name: "a-local-value",
                ty: ASN1Type::Boolean(Boolean {
                    constraints: vec![],
                }),
                value: ASN1Value::Boolean(true),
            }
        );
    }
}
