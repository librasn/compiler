use core::fmt::{Display, Formatter, Result};
use std::{error::Error, io};

use colored::Colorize;
use nom::Slice;

use crate::error::CompilerError;

use super::{
    input::{Input, TrackedParser},
    util::until_next_unindented,
};

#[derive(Debug, Clone)]
pub struct LexerError {
    pub details: String,
    pub kind: LexerErrorType,
    report_data: Option<ReportData>,
}

impl CompilerError for LexerError {
    fn as_report(&self) -> String {
        if let Some(report_data) = &self.report_data {
            let error = "Error:";
            let line = report_data.line;
            let column = report_data.column;
            let n = report_data.line.checked_ilog10().unwrap_or(0) as usize;
            let digits = n + 1;
            let spacer = "─".repeat(n);
            let indentation = " ".repeat(n);
            let pdu_lines = report_data.pdu.match_indices('\n').count();
            let pdu =
                &report_data
                    .pdu
                    .lines()
                    .enumerate()
                    .fold(String::new(), |acc, (i, l)| {
                        let line_no = format!(
                            "{:0>digits$}",
                            (line - pdu_lines + i - 1).to_string()
                        );
                        format!("{acc}\n {line_no} │  {l}")
                    });
            let mut tried_parsers = String::from("   │");
            if !report_data.tracked_parsers.is_empty() {
                tried_parsers += &format!(
                    "\n{indentation}   │  Applied the following parsers:\n{indentation}   │"
                );
                report_data.tracked_parsers.iter().for_each(
                    |tp: &crate::lexer::input::TrackedParser| {
                        let name = tp.name;
                        let tabbing = 40 - tp.name.len();
                        let status = if tp.success {
                            "successful"
                        } else {
                            "unsuccessful"
                        };
                        tried_parsers +=
                            &format!("\n{indentation}   │  {name}{indentation:tabbing$}{status}")
                    },
                );
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
            let snippet = &report_data.pdu;
            let ws = " ";
            let arrow_head = "┬".red().bold();
            let arrow_tail = "╰── failed to parse input from here on".red().bold();
            let mut tried_parsers = String::from("   │");
            if !report_data.tracked_parsers.is_empty() {
                tried_parsers += "\n   │  Applied the following parsers:\n   │";
                report_data.tracked_parsers.iter().for_each(
                    |tp: &crate::lexer::input::TrackedParser| {
                        let name = tp.name.italic();
                        let indentation = 40 - tp.name.len();
                        let status = if tp.success {
                            "successful".green()
                        } else {
                            "unsuccessful".red()
                        };
                        tried_parsers += &format!("\n  │  {name}{ws:indentation$}{status}")
                    },
                );
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

impl<'a> From<nom::Err<nom::error::Error<Input<'a>>>> for LexerError {
    fn from(value: nom::Err<nom::error::Error<Input<'a>>>) -> Self {
        match value {
            nom::Err::Incomplete(_) => Self {
                details: "Unexpected end of input!".into(),
                kind: LexerErrorType::NotEnoughData,
                report_data: None,
            },
            nom::Err::Error(mut e) => Self {
                details: format!(
                    "Error matching ASN syntax at while parsing: {}",
                    &e.input.slice(..(e.input.len().min(300))).into_inner()
                ),
                kind: LexerErrorType::MatchingError(e.code),
                report_data: Some(ReportData {
                    pdu: until_next_unindented(e.input.context(), 300).to_owned(),
                    line: e.input.line(),
                    column: e.input.column(),
                    tracked_parsers: e.input.drain_tracked_parsers(),
                }),
            },
            nom::Err::Failure(mut e) => Self {
                details: format!(
                    "Unrecoverable error while parsing: {}",
                    &e.input.slice(..(e.input.len().min(300))).into_inner()
                ),
                kind: LexerErrorType::Failure(e.code),
                report_data: Some(ReportData {
                    pdu: until_next_unindented(e.input.context(), 300).to_owned(),
                    line: e.input.line(),
                    column: e.input.column(),
                    tracked_parsers: e.input.drain_tracked_parsers(),
                }),
            },
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
    pub pdu: String,
    pub line: usize,
    pub column: usize,
    pub tracked_parsers: Vec<TrackedParser>,
}

#[cfg(test)]
mod tests {
    use crate::{
        error::CompilerError,
        lexer::{
            error::LexerError,
            input::{Input, TrackedParser},
        },
    };

    #[test]
    fn reports_parsing_error() {
        assert_eq!(
            "\u{1b}[1;31mError:\u{1b}[0m 
   ╭─[line 2, column 2]
   │
 2 │   IntermediateResponse ::= [APPLICATION 25] SEQUENCE {
   │   \u{1b}[1;31m┬\u{1b}[0m
   │   \u{1b}[1;31m╰── failed to parse input from here on\u{1b}[0m
   │
───╯",
            LexerError::from(nom::Err::Error(nom::error::Error::new(
                Input::from(
                    r#"  IntermediateResponse ::= [APPLICATION 25] SEQUENCE {
             responseName     [0] LDAPOID OPTIONAL,
             responseValue    [1] OCTET STRING OPTIONAL }

        END
        "#
                )
                .with_line_column_and_offset(2, 2, 4),
                nom::error::ErrorKind::IsNot
            )))
            .as_styled_report()
            .trim()
        )
    }

    #[test]
    fn reports_parsing_error_with_tried_parsers() {
        let mut input = Input::from(
            r#"  IntermediateResponse ::= [APPLICATION 25] SEQUENCE {
 responseName     [0] LDAPOID OPTIONAL,
 responseValue    [1] OCTET STRING OPTIONAL }

END
"#,
        )
        .with_line_column_and_offset(2, 2, 4);
        input.add_untracked(&[
            TrackedParser::new("first", true),
            TrackedParser::new("second", false),
            TrackedParser::new("third", false),
        ]);
        assert_eq!(
            "\u{1b}[1;31mError:\u{1b}[0m 
   ╭─[line 2, column 2]
   │
 2 │   IntermediateResponse ::= [APPLICATION 25] SEQUENCE {
   │   \u{1b}[1;31m┬\u{1b}[0m
   │   \u{1b}[1;31m╰── failed to parse input from here on\u{1b}[0m
   │
   │  Applied the following parsers:
   │
   │  \u{1b}[3mfirst\u{1b}[0m                                   \u{1b}[32msuccessful\u{1b}[0m
   │  \u{1b}[3msecond\u{1b}[0m                                  \u{1b}[31munsuccessful\u{1b}[0m
   │  \u{1b}[3mthird\u{1b}[0m                                   \u{1b}[31munsuccessful\u{1b}[0m
───╯",
            LexerError::from(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::IsNot
            )))
            .as_styled_report()
            .trim()
        )
    }
}
