//! The `generator` module is responsible for generating rust code that handles
//! decoding and encoding of the parsed and validated ASN1 data elements.
//! The `generator` uses string templates for generating rust code.

use std::error::Error;

use crate::intermediate::ToplevelDeclaration;

use self::error::GeneratorError;

pub mod error;
pub mod rasn;

/// Implementors of the `Backend` trait can be used 
/// as a backend to the compiler in order to create bindings
/// for other frameworks and languages than the default backend.
pub trait Backend: Sized {
    /// generates bindings for an ASN.1 module
    /// ### Params
    /// - `top_level_declarations` vector of [TopLevelDeclaration]s that are defined in the ASN.1 module
    fn generate_module(
        &self,
        top_level_declarations: Vec<ToplevelDeclaration>,
    ) -> Result<GeneratedModule, GeneratorError>;
}

pub struct GeneratedModule {
    pub generated: Option<String>,
    pub warnings: Vec<Box<dyn Error>>,
}

impl GeneratedModule {
    pub fn empty() -> Self {
        Self {
            generated: None,
            warnings: vec![],
        }
    }
}
