use core::fmt::{Display, Formatter, Result};
use std::{error::Error, io};

use colored::Colorize;
use nom::{
    error::{ContextError, FromExternalError, ParseError},
    IResult, Slice,
};

use crate::{
    error::CompilerError,
    input::{Input, TrackedParser},
};

use super::util::until_next_unindented;

pub type ParserResult<'a, O> = IResult<Input<'a>, O, ErrorTree<'a>>;

#[derive(Debug, Clone)]
pub struct LexerError {
    pub details: String,
    pub kind: LexerErrorType,
    report_data: Option<ReportData>,
}

impl CompilerError for LexerError {
    fn as_report(&self, input: &str) -> String {
        if let Some(report_data) = &self.report_data {
            let error = "Error:";
            let line = report_data.line;
            let context = until_next_unindented(
                &input[report_data.context_start_line..],
                report_data.context_start_offset,
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
                    let line_no = format!("{:0>digits$}", (start_line + i).to_string());
                    format!("{acc}\n {line_no} │  {l}")
                });
            let mut tried_parsers = String::from("   │");
            if !report_data.tracked_parsers.is_empty() {
                tried_parsers += &format!(
                    "\n{indentation}   │  Applied the following parsers:\n{indentation}   │"
                );
                report_data
                    .tracked_parsers
                    .iter()
                    .for_each(|tp: &TrackedParser| {
                        let name = tp.name;
                        let tabbing = 40 - tp.name.len();
                        let status = if tp.success {
                            "successful"
                        } else {
                            "unsuccessful"
                        };
                        tried_parsers +=
                            &format!("\n{indentation}   │  {name}{indentation:tabbing$}{status}")
                    });
            }
            format!(
                r#"
{error} 
{indentation}   ╭─[line {line}, column {column}]
{indentation}   │
{indentation}   │ {pdu}
{indentation}   │
{indentation}{tried_parsers}
{spacer}───╯
        "#
            )
        } else {
            self.details.clone()
        }
    }

    fn as_styled_report(&self) -> String {
        if let Some(report_data) = &self.report_data {
            let error = "Error:".red().bold();
            let line = report_data.line;
            let digits = report_data.line.checked_ilog10().unwrap_or(0) as usize;
            let spacer = "─".repeat(digits);
            let column = report_data.column;
            let snippet = &report_data.context_start_line;
            let ws = " ";
            let arrow_head = "┬".red().bold();
            let arrow_tail = "╰── failed to parse input from here on".red().bold();
            let mut tried_parsers = String::from("   │");
            if !report_data.tracked_parsers.is_empty() {
                tried_parsers += "\n   │  Applied the following parsers:\n   │";
                report_data
                    .tracked_parsers
                    .iter()
                    .for_each(|tp: &TrackedParser| {
                        let name = tp.name.italic();
                        let indentation = 40 - tp.name.len();
                        let status = if tp.success {
                            "successful".green()
                        } else {
                            "unsuccessful".red()
                        };
                        tried_parsers += &format!("\n  │  {name}{ws:indentation$}{status}")
                    });
            }
            format!(
                r#"
{error} 
{ws:digits$}  ╭─[line {line}, column {column}]
{ws:digits$}  │
{ws:digits$}{line} │ {ws:column$}{snippet}
{ws:digits$}  │ {ws:column$}{arrow_head}
{ws:digits$}  │ {ws:column$}{arrow_tail}
{ws:digits$}{tried_parsers}
{spacer}───╯
        "#
            )
        } else {
            self.details.clone()
        }
    }
}

impl<'a> From<nom::Err<ErrorTree<'a>>> for LexerError {
    fn from(value: nom::Err<ErrorTree<'a>>) -> Self {
        match value {
            nom::Err::Incomplete(_) => Self {
                details: "Unexpected end of input!".into(),
                kind: LexerErrorType::NotEnoughData,
                report_data: None,
            },
            nom::Err::Error(e) | nom::Err::Failure(e) => {
                println!("{e:#?}");
                let mut input = e.into_input();
                Self {
                    details: format!(
                        "Error matching ASN syntax at while parsing: {}",
                        &input.slice(..(input.len().min(300))).into_inner()
                    ),
                    kind: LexerErrorType::MatchingError(nom::error::ErrorKind::Alpha),
                    report_data: Some(ReportData {
                        context_start_line: input.context_start_line(),
                        context_start_offset: input.context_start_offset(),
                        line: input.line(),
                        offset: input.offset(),
                        column: input.column(),
                        tracked_parsers: input.drain_tracked_parsers(),
                    }),
                }
            }
        }
    }
}

impl From<io::Error> for LexerError {
    fn from(value: io::Error) -> Self {
        LexerError {
            details: value.to_string(),
            kind: LexerErrorType::IO,
            report_data: None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum LexerErrorType {
    NotEnoughData,
    #[allow(dead_code)]
    MatchingError(nom::error::ErrorKind),
    #[allow(dead_code)]
    Failure(nom::error::ErrorKind),
    IO,
}

impl Error for LexerError {}

impl Display for LexerError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "Encountered error while parsing {:?} - {}",
            self.kind, self.details
        )
    }
}

#[derive(Debug, Clone)]
struct ReportData {
    pub context_start_line: usize,
    pub context_start_offset: usize,
    pub line: usize,
    pub offset: usize,
    pub column: usize,
    pub tracked_parsers: Vec<TrackedParser>,
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
    Leaf {
        input: Input<'a>,
        kind: ErrorKind,
    },
    Branch {
        tip: Box<Self>,
        links: Vec<ErrorLink<'a>>,
    },
    Fork {
        branches: Vec<Self>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum ErrorKind {
    Nom(nom::error::ErrorKind),
    External(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ErrorLink<'a> {
    input: Input<'a>,
    context: Context,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Context {
    Name(&'static str),
    ErrorKind(ErrorKind),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ErrorLeaf<'a> {
    input: Input<'a>,
    kind: ErrorKind,
}

impl<'a> ErrorTree<'a> {
    pub fn into_input(self) -> Input<'a> {
        match self {
            ErrorTree::Leaf { input, .. } => input,
            ErrorTree::Branch { mut links, .. } => {
                links
                    .pop()
                    .expect("error tree branch to have at least one link")
                    .input
            }
            ErrorTree::Fork { mut branches } => branches
                .pop()
                .expect("error tree branch to have at least one link")
                .into_input(),
        }
    }
}

impl<'a> ParseError<Input<'a>> for ErrorTree<'a> {
    fn from_error_kind(input: Input<'a>, kind: nom::error::ErrorKind) -> Self {
        ErrorTree::Leaf {
            input,
            kind: ErrorKind::Nom(kind),
        }
    }

    fn append(input: Input<'a>, kind: nom::error::ErrorKind, other: Self) -> Self {
        match other {
            fork @ ErrorTree::Fork { .. } if kind == nom::error::ErrorKind::Alt => fork,
            ErrorTree::Branch { mut links, tip } => {
                links.push(ErrorLink {
                    input,
                    context: Context::ErrorKind(ErrorKind::Nom(kind)),
                });
                ErrorTree::Branch { tip, links }
            }
            tip => ErrorTree::Branch {
                tip: Box::new(tip),
                links: vec![ErrorLink {
                    input,
                    context: Context::ErrorKind(ErrorKind::Nom(kind)),
                }],
            },
        }
    }

    fn or(self, other: Self) -> Self {
        let branches = match (self, other) {
            (ErrorTree::Fork { branches: mut b_1 }, ErrorTree::Fork { branches: mut b_2 }) => {
                match b_1.capacity() >= b_2.capacity() {
                    true => {
                        b_1.extend(b_2);
                        b_1
                    }
                    false => {
                        b_2.extend(b_1);
                        b_2
                    }
                }
            }
            (branch, ErrorTree::Fork { mut branches })
            | (ErrorTree::Fork { mut branches }, branch) => {
                branches.push(branch);
                branches
            }
            (branch_1, branch_2) => vec![branch_1, branch_2],
        };

        ErrorTree::Fork { branches }
    }
}

impl<'a> ContextError<Input<'a>> for ErrorTree<'a> {
    fn add_context(input: Input<'a>, context: &'static str, other: Self) -> Self {
        match other {
            ErrorTree::Branch { tip, mut links } => {
                links.push(ErrorLink {
                    input,
                    context: Context::Name(context),
                });
                ErrorTree::Branch { tip, links }
            }
            tip => ErrorTree::Branch {
                tip: Box::new(tip),
                links: vec![ErrorLink {
                    input,
                    context: Context::Name(context),
                }],
            },
        }
    }
}

impl<'a, E: Error> FromExternalError<Input<'a>, E> for ErrorTree<'a> {
    fn from_external_error(input: Input<'a>, _: nom::error::ErrorKind, e: E) -> Self {
        Self::Leaf {
            input,
            kind: ErrorKind::External(e.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    //     use super::*;

    //     #[test]
    //     fn reports_parsing_error() {
    //         assert_eq!(
    //             "\u{1b}[1;31mError:\u{1b}[0m
    //    ╭─[line 2, column 2]
    //    │
    //  2 │   IntermediateResponse ::= [APPLICATION 25] SEQUENCE {
    //    │   \u{1b}[1;31m┬\u{1b}[0m
    //    │   \u{1b}[1;31m╰── failed to parse input from here on\u{1b}[0m
    //    │
    // ───╯",
    //             LexerError::from(nom::Err::Error(nom::error::Error::new(
    //                 Input::from(
    //                     r#"  IntermediateResponse ::= [APPLICATION 25] SEQUENCE {
    //              responseName     [0] LDAPOID OPTIONAL,
    //              responseValue    [1] OCTET STRING OPTIONAL }

    //         END
    //         "#
    //                 )
    //                 .with_line_column_and_offset(2, 2, 4),
    //                 nom::error::ErrorKind::IsNot
    //             )))
    //             .as_styled_report()
    //             .trim()
    //         )
    //     }

    //     #[test]
    //     fn reports_parsing_error_with_tried_parsers() {
    //         let mut input = Input::from(
    //             r#"  IntermediateResponse ::= [APPLICATION 25] SEQUENCE {
    //  responseName     [0] LDAPOID OPTIONAL,
    //  responseValue    [1] OCTET STRING OPTIONAL }

    // END
    // "#,
    //         )
    //         .with_line_column_and_offset(2, 2, 4);
    //         input.add_untracked(&[
    //             TrackedParser::new("first", true),
    //             TrackedParser::new("second", false),
    //             TrackedParser::new("third", false),
    //         ]);
    //         assert_eq!(
    //             "\u{1b}[1;31mError:\u{1b}[0m
    //    ╭─[line 2, column 2]
    //    │
    //  2 │   IntermediateResponse ::= [APPLICATION 25] SEQUENCE {
    //    │   \u{1b}[1;31m┬\u{1b}[0m
    //    │   \u{1b}[1;31m╰── failed to parse input from here on\u{1b}[0m
    //    │
    //    │  Applied the following parsers:
    //    │
    //    │  \u{1b}[3mfirst\u{1b}[0m                                   \u{1b}[32msuccessful\u{1b}[0m
    //    │  \u{1b}[3msecond\u{1b}[0m                                  \u{1b}[31munsuccessful\u{1b}[0m
    //    │  \u{1b}[3mthird\u{1b}[0m                                   \u{1b}[31munsuccessful\u{1b}[0m
    // ───╯",
    //             LexerError::from(nom::Err::Error(nom::error::Error::new(
    //                 input,
    //                 nom::error::ErrorKind::IsNot
    //             )))
    //             .as_styled_report()
    //             .trim()
    //         )
    //     }
}
