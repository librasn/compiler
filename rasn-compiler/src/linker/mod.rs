//! The `linker` module links the individual symbols of an ASN.1 spec.
//! It resolves references and enriches the internal representations with
//! metadata that may be required by the generator module.
use crate::{
    linker::symbol_table::{SymbolTable, SymbolTableState},
    prelude::LinkerError,
};

pub(crate) mod error;
mod resolve_constraint_references;
mod resolve_parameters;
mod symbol_table;
#[cfg(test)]
mod tests;
mod types;
mod unnest;

pub struct Linker<'a, S: SymbolTableState> {
    symbol_table: SymbolTable<'a, S>,
}

#[derive(Debug, PartialEq, Clone, Copy, Default)]
struct RecursionGuard(u8, &'static str);

impl RecursionGuard {
    const MAX_DEPTH: u8 = 30;

    pub fn new() -> Self {
        Self::default()
    }

    pub fn inc(&mut self) -> Result<(), LinkerError> {
        self.0 += 1;
        if self.0 > Self::MAX_DEPTH {
            Err(LinkerError {
                pdu: None,
                details: format!("Recursion limit exceeded in {}", self.1),
                kind: error::LinkerErrorType::RecursionLimitExceeded,
            })
        } else {
            Ok(())
        }
    }
}

pub struct Validator;

impl Validator {
    pub fn new<'a>(_: Vec<crate::prelude::Assignment<'a>>) -> Validator {
        Self
    }

    pub fn link(self) -> Result<(Self, Vec<crate::CompilerError>), crate::prelude::LinkerError> {
        Ok((self, Vec::new()))
    }

    pub fn validate<'a>(
        &self,
    ) -> Result<
        (
            Vec<crate::prelude::Assignment<'a>>,
            Vec<crate::CompilerError>,
        ),
        crate::CompilerError,
    > {
        Ok((Vec::new(), Vec::new()))
    }
}
