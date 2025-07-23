//! The `linker` module links the individual symbols of an ASN.1 spec.
//! It resolves references and enriches the internal representations with
//! metadata that may be required by the generator module.
use crate::linker::symbol_table::SymbolTable;
pub(crate) mod error;
mod symbol_table;
#[cfg(test)]
mod tests;

pub struct Linker<'a> {
    symbol_table: SymbolTable<'a>
}


