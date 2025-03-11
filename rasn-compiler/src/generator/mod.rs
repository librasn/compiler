//! The `generator` module is responsible for generating rust code that handles
//! decoding and encoding of the parsed and validated ASN1 data elements.
//! The `generator` uses string templates for generating rust code.

use std::fmt::Debug;

use crate::{
    error::CompilerError,
    intermediate::{ExtensibilityEnvironment, TaggingEnvironment, ToplevelDefinition},
};

use self::error::GeneratorError;

pub mod error;
pub mod rasn;
pub mod typescript;

/// Implementors of the `Backend` trait can be used
/// as a backend to the compiler in order to create bindings
/// for other frameworks and languages than the default backend.
pub trait Backend: Default {
    type Config: Default + Debug;

    /// File extension that should be used for output file containing the generated bindings.
    /// For example: `.ts` for Typescript, `.rs` for Rasn bindings.
    const FILE_EXTENSION: &'static str;

    /// generates bindings for an ASN.1 module
    /// ### Params
    /// - `top_level_declarations` vector of [TopLevelDeclaration]s that are defined in the ASN.1 module
    fn generate_module(
        &mut self,
        top_level_declarations: Vec<ToplevelDefinition>,
    ) -> Result<GeneratedModule, GeneratorError>;

    /// generates bindings for a single ASN.1 item
    /// ### Params
    /// - `tld` [TopLevelDeclaration] for which the bindings should be generated
    fn generate(&self, tld: ToplevelDefinition) -> Result<String, GeneratorError>;

    /// Formats the bindings using the language- or framework-specific linters.
    /// For example, the Rust backend uses rustfmt for formatting bindings.
    fn format_bindings(bindings: &str) -> Result<String, CompilerError> {
        Ok(bindings.to_owned())
    }

    /// Returns a reference to the backend's config
    fn config(&self) -> &Self::Config;

    /// Creates a backend from its config
    fn from_config(config: Self::Config) -> Self;

    /// Creates a backend from its fields.
    /// Usually, the tagging and extensibility environments do not
    /// have to be set manually, but will follow the respective module header.
    fn new(
        config: Self::Config,
        tagging_environment: TaggingEnvironment,
        extensibility_environment: ExtensibilityEnvironment,
    ) -> Self;
}

pub struct GeneratedModule {
    pub generated: Option<String>,
    pub warnings: Vec<CompilerError>,
}

impl GeneratedModule {
    pub fn empty() -> Self {
        Self {
            generated: None,
            warnings: vec![],
        }
    }
}
