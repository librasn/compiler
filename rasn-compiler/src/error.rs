use nom::error::{ContextError, ErrorKind, ParseError};

use crate::input::Input;

pub trait CompilerError: std::fmt::Debug {
    fn as_report(&self, input: &str) -> String;
    fn as_styled_report(&self) -> String;
}
