use core::fmt::{Display, Formatter, Result};
use std::{collections::VecDeque, error::Error, io};

use nom::{
    error::{ContextError, FromExternalError, ParseError},
    IResult,
};

use crate::input::Input;

use super::util::until_next_unindented;

pub type ParserResult<'a, O> = IResult<Input<'a>, O, ErrorTree<'a>>;

#[derive(Debug, Clone, PartialEq)]
pub struct LexerError {
    pub kind: LexerErrorType,
}

impl LexerError {
    pub fn contextualize(&self, input: &str) -> String {
        match &self.kind {
            LexerErrorType::MatchingError(report_data) => {
                let line = report_data.line;
                let context = until_next_unindented(
                    &input[report_data.context_start_offset..],
                    report_data.offset - report_data.context_start_offset + 1,
                    300,
                );
                let pdu_lines = context.match_indices('\n').count();
                let start_line = report_data.context_start_line;
                let end_line = report_data.context_start_line + pdu_lines;
                let column = report_data.column;
                let n = end_line.checked_ilog10().unwrap_or(0) as usize;
                let digits = n + 1;
                let spacer = "─".repeat(n);
                let indentation = " ".repeat(n);
                let pdu = context
                    .lines()
                    .enumerate()
                    .fold(String::new(), |acc, (i, l)| {
                        if l.trim().is_empty() {
                            return acc;
                        }
                        let line_no = format!("{:0>digits$}", (start_line + i).to_string());
                        let mut ln = format!("{acc}\n {line_no} │  {}", l.trim_end());
                        if i + start_line == line {
                            ln += " ◀▪▪▪▪▪▪▪▪▪▪ FAILED AT THIS LINE";
                        }
                        ln
                    });

                format!(
                    r#"
Error matching ASN syntax at while parsing:
{indentation}   ╭─[line {line}, column {column}]
{indentation}   │
{indentation}   │ {pdu}
{indentation}   │
{spacer}───╯
        "#
                )
            }
            _ => format!("{self}"),
        }
    }
}

impl<'a> From<nom::Err<ErrorTree<'a>>> for LexerError {
    fn from(value: nom::Err<ErrorTree<'a>>) -> Self {
        match value {
            nom::Err::Incomplete(needed) => Self {
                kind: LexerErrorType::NotEnoughData(match needed {
                    nom::Needed::Unknown => None,
                    nom::Needed::Size(i) => Some(i.get()),
                }),
            },
            nom::Err::Error(e) | nom::Err::Failure(e) => Self {
                kind: LexerErrorType::MatchingError(e.into()),
            },
        }
    }
}

impl From<io::Error> for LexerError {
    fn from(value: io::Error) -> Self {
        LexerError {
            kind: LexerErrorType::IO(value.to_string()),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LexerErrorType {
    NotEnoughData(Option<usize>),
    MatchingError(ReportData),
    IO(String),
}

impl Error for LexerError {}

impl Display for LexerError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match &self.kind {
            LexerErrorType::NotEnoughData(needed) => write!(
                f,
                "Unexpected end of input.{}",
                needed.map_or(String::new(), |i| format!(
                    " Need another {i} characters of input."
                ))
            ),
            LexerErrorType::MatchingError(report_data) => write!(
                f,
                "Error matching ASN syntax at while parsing line {} column {}.",
                report_data.line, report_data.column
            ),
            LexerErrorType::IO(reason) => write!(f, "Failed to read ASN.1 source. {reason}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ReportData {
    pub context_start_line: usize,
    pub context_start_offset: usize,
    pub line: usize,
    pub offset: usize,
    pub column: usize,
    pub reason: String,
    pub unexpected_eof: bool,
}

impl From<ErrorTree<'_>> for ReportData {
    fn from(value: ErrorTree<'_>) -> Self {
        match value {
            ErrorTree::Base { input, kind } => Self {
                context_start_line: input.context_start_line(),
                context_start_offset: input.context_start_offset(),
                line: input.line(),
                offset: input.offset(),
                column: input.column(),
                unexpected_eof: kind == ErrorKind::Nom(nom::error::ErrorKind::Eof),
                reason: match kind {
                    ErrorKind::Nom(e) => format!("Failed to parse next input. Code: {e:?}"),
                    ErrorKind::External(e) => e,
                },
            },
            ErrorTree::Stack { base, .. } => Self::from(*base),
            ErrorTree::Alt(mut alts) => {
                Self::from(alts.pop_front().expect("ErrorTree::Alt not to be empty."))
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MiscError(pub &'static str);

impl Error for MiscError {}

impl Display for MiscError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(&self.0, f)
    }
}

/// The [ErrorTree] tracks errors along a parsers path.
/// This error type is a simplified version of [`nom-supreme`](https://github.com/Lucretiel/nom-supreme/)'s
/// `GenericErrorTree`.
#[derive(Debug, Clone, PartialEq)]
pub enum ErrorTree<'a> {
    Base {
        input: Input<'a>,
        kind: ErrorKind,
    },
    Stack {
        base: Box<Self>,
        contexts: Vec<StackContext<'a>>,
    },
    Alt(VecDeque<Self>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ErrorKind {
    Nom(nom::error::ErrorKind),
    External(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct StackContext<'a> {
    input: Input<'a>,
    context: Context,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Context {
    Name(&'static str),
    ErrorKind(ErrorKind),
}

impl<'a> ErrorTree<'a> {
    pub fn into_input(self) -> Input<'a> {
        match self {
            ErrorTree::Base { input, .. } => input,
            ErrorTree::Stack { mut contexts, .. } => {
                contexts
                    .pop()
                    .expect("ErrorTree::Stack to have at least one context")
                    .input
            }
            ErrorTree::Alt(mut alts) => alts
                .pop_back()
                .expect("ErrorTree:Alt to have at least one alternative")
                .into_input(),
        }
    }

    pub fn is_eof_error(&self) -> bool {
        match self {
            ErrorTree::Base { kind, .. } => kind == &ErrorKind::Nom(nom::error::ErrorKind::Eof),
            ErrorTree::Stack { base, .. } => base.is_eof_error(),
            ErrorTree::Alt(alts) => alts.back().is_some_and(|b| b.is_eof_error()),
        }
    }
}

impl<'a> ParseError<Input<'a>> for ErrorTree<'a> {
    fn from_error_kind(input: Input<'a>, kind: nom::error::ErrorKind) -> Self {
        ErrorTree::Base {
            input,
            kind: ErrorKind::Nom(kind),
        }
    }

    fn append(input: Input<'a>, kind: nom::error::ErrorKind, other: Self) -> Self {
        let context = StackContext {
            input,
            context: Context::ErrorKind(ErrorKind::Nom(kind)),
        };
        match other {
            alt @ ErrorTree::Alt { .. } if kind == nom::error::ErrorKind::Alt => alt,
            ErrorTree::Stack { mut contexts, base } => {
                contexts.push(context);
                ErrorTree::Stack { base, contexts }
            }
            base => ErrorTree::Stack {
                base: Box::new(base),
                contexts: vec![context],
            },
        }
    }

    fn or(self, other: Self) -> Self {
        let alts = match (self, other) {
            (ErrorTree::Alt(mut alt1), ErrorTree::Alt(mut alt2)) => {
                match alt1.capacity() >= alt2.capacity() {
                    true => {
                        alt1.extend(alt2);
                        alt1
                    }
                    false => {
                        alt2.extend(alt1);
                        alt2
                    }
                }
            }
            (alt, ErrorTree::Alt(mut alts)) | (ErrorTree::Alt(mut alts), alt) => {
                alts.push_back(alt);
                alts
            }
            (alt1, alt2) => vec![alt1, alt2].into(),
        };

        ErrorTree::Alt(alts)
    }
}

impl<'a> ContextError<Input<'a>> for ErrorTree<'a> {
    fn add_context(input: Input<'a>, context: &'static str, other: Self) -> Self {
        let context = StackContext {
            input,
            context: Context::Name(context),
        };
        match other {
            ErrorTree::Stack { base, mut contexts } => {
                contexts.push(context);
                ErrorTree::Stack { base, contexts }
            }
            base => ErrorTree::Stack {
                base: Box::new(base),
                contexts: vec![context],
            },
        }
    }
}

impl<'a, E: Error> FromExternalError<Input<'a>, E> for ErrorTree<'a> {
    fn from_external_error(input: Input<'a>, _: nom::error::ErrorKind, e: E) -> Self {
        Self::Base {
            input,
            kind: ErrorKind::External(e.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contextualizes_error() {
        let input = r#"GAP-Test DEFINITIONS AUTOMATIC TAGS ::= BEGIN
c-CtxTypeSystemNull ItsAidCtxRef ::=  { itsaid content:0, ctx c-ctxRefNull }

ItsAidCtxRef ::= SEQUENCE {
 itsaid ITSaid,
 ctx CtxRef
 }

CtxRef ::INTEGER(0..255)
c-ctxRefNull CtxRef ::= 0

    END"#;
        let error = LexerError {
            kind: LexerErrorType::MatchingError(ReportData {
                context_start_line: 4,
                context_start_offset: 123,
                line: 6,
                column: 6,
                offset: 172,
                reason: "Test".into(),
                unexpected_eof: false,
            }),
        };
        assert_eq!(
            error.contextualize(input),
            r#"
Error matching ASN syntax at while parsing:
   ╭─[line 6, column 6]
   │
   │ 
 5 │  ItsAidCtxRef ::= SEQUENCE {
 6 │   itsaid ITSaid, ◀▪▪▪▪▪▪▪▪▪▪ FAILED AT THIS LINE
 7 │   ctx CtxRef
 8 │   }
   │
───╯
        "#
        )
    }
}
