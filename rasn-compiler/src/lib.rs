#[doc = include_str!("../README.md")]
pub(crate) mod common;
mod error;
mod generator;
mod input;
pub mod intermediate;
mod lexer;
#[cfg(test)]
mod tests;
mod validator;

use std::{
    borrow::Cow,
    cell::RefCell,
    collections::BTreeMap,
    fs::{self, read_to_string},
    io::Write,
    path::{Path, PathBuf},
    rc::Rc,
    vec,
};

use error::CompilerError;
use generator::Backend;
use intermediate::ToplevelDefinition;
use lexer::{asn_spec, error::LexerError};
use prelude::{GeneratorError, GeneratorErrorType};
use validator::Validator;

pub type RasnCompiler<S> = Compiler<generator::rasn::Rasn, S>;
pub type TsCompiler<S> = Compiler<generator::typescript::Typescript, S>;

pub mod prelude {
    //! Convenience module that collects all necessary imports for
    //! using and customizing the compiler.
    pub use super::{
        error::CompilerError, CompileResult, Compiler, CompilerMissingParams, CompilerOutputSet,
        CompilerReady, CompilerSourcesSet,
    };
    pub use crate::generator::{
        error::*,
        rasn::{Config as RasnConfig, Rasn as RasnBackend},
        typescript::{Config as TsConfig, Typescript as TypescriptBackend},
        Backend, GeneratedModule,
    };

    pub use crate::intermediate::{
        ExtensibilityEnvironment, TaggingEnvironment, ToplevelDefinition,
    };

    pub use crate::lexer::error::{LexerError, LexerErrorType, ReportData};
    pub use crate::validator::error::{LinkerError, LinkerErrorType};

    pub mod ir {
        pub use crate::intermediate::{
            constraints::*,
            encoding_rules::{per_visible::*, *},
            error::*,
            information_object::*,
            parameterization::*,
            types::*,
            *,
        };
    }
}

#[cfg(target_family = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg(target_family = "wasm")]
#[wasm_bindgen(inspectable, getter_with_clone)]
pub struct Generated {
    pub rust: String,
    pub warnings: String,
}

#[cfg(target_family = "wasm")]
#[wasm_bindgen]
pub fn compile_to_typescript(asn1: &str) -> Result<Generated, JsValue> {
    Compiler::<crate::prelude::TypescriptBackend, _>::new()
        .add_asn_literal(asn1)
        .compile_to_string()
        .map(|result| Generated {
            rust: result.generated,
            warnings: result
                .warnings
                .into_iter()
                .fold(String::new(), |mut acc, w| {
                    acc += &w.to_string();
                    acc += "\n";
                    acc
                }),
        })
        .map_err(|e| JsValue::from(e.to_string()))
}

#[cfg(target_family = "wasm")]
#[wasm_bindgen]
pub fn compile_to_rust(
    asn1: &str,
    config: crate::prelude::RasnConfig,
) -> Result<Generated, JsValue> {
    Compiler::<crate::prelude::RasnBackend, _>::new_with_config(config)
        .add_asn_literal(asn1)
        .compile_to_string()
        .map(|result| Generated {
            rust: result.generated,
            warnings: result
                .warnings
                .into_iter()
                .fold(String::new(), |mut acc, w| {
                    acc += &w.to_string();
                    acc += "\n";
                    acc
                }),
        })
        .map_err(|e| JsValue::from(e.to_string()))
}

/// The rasn compiler
pub struct Compiler<B: Backend, S: CompilerState> {
    state: S,
    backend: B,
}

/// Typestate representing compiler with missing parameters
pub struct CompilerMissingParams;

impl Default for CompilerMissingParams {
    fn default() -> Self {
        Self
    }
}

/// Typestate representing compiler that is ready to compile
pub struct CompilerReady {
    sources: Vec<AsnSource>,
    output_mode: OutputMode,
}

/// Typestate representing compiler that has the output mode set, but is missing ASN1 sources
pub struct CompilerOutputSet {
    output_mode: OutputMode,
}

/// Typestate representing compiler that knows about ASN1 sources, but doesn't have an output mode set
pub struct CompilerSourcesSet {
    sources: Vec<AsnSource>,
}

/// State of the rasn compiler
pub trait CompilerState {}
impl CompilerState for CompilerReady {}
impl CompilerState for CompilerOutputSet {}
impl CompilerState for CompilerSourcesSet {}
impl CompilerState for CompilerMissingParams {}

#[derive(Debug)]
pub struct CompileResult {
    pub generated: String,
    pub warnings: Vec<CompilerError>,
}

impl CompileResult {
    fn fmt<B: Backend>(mut self) -> Self {
        self.generated = B::format_bindings(&self.generated).unwrap_or(self.generated);
        self
    }
}

#[derive(Debug, PartialEq)]
enum AsnSource {
    Path(PathBuf),
    Literal(String),
}

impl<B: Backend> Default for Compiler<B, CompilerMissingParams> {
    fn default() -> Self {
        Self::new()
    }
}

impl<B: Backend, S: CompilerState> Compiler<B, S> {
    pub fn with_backend<B2: Backend>(self, backend: B2) -> Compiler<B2, S> {
        Compiler {
            state: self.state,
            backend,
        }
    }
}

impl<B: Backend> Compiler<B, CompilerMissingParams> {
    /// Provides a Builder for building rasn compiler commands
    pub fn new() -> Compiler<B, CompilerMissingParams> {
        Compiler {
            state: CompilerMissingParams,
            backend: B::default(),
        }
    }

    /// Provides a Builder for building rasn compiler commands
    pub fn new_with_config(config: B::Config) -> Compiler<B, CompilerMissingParams> {
        Compiler {
            state: CompilerMissingParams,
            backend: B::from_config(config),
        }
    }
}

impl<B: Backend> Compiler<B, CompilerMissingParams> {
    /// Add an ASN1 source to the compile command by path
    /// * `path_to_source` - path to ASN1 file to include
    pub fn add_asn_by_path(
        self,
        path_to_source: impl Into<PathBuf>,
    ) -> Compiler<B, CompilerSourcesSet> {
        Compiler {
            state: CompilerSourcesSet {
                sources: vec![AsnSource::Path(path_to_source.into())],
            },
            backend: self.backend,
        }
    }

    /// Add several ASN1 sources by path to the compile command
    /// * `path_to_source` - iterator of paths to the ASN1 files to be included
    pub fn add_asn_sources_by_path(
        self,
        paths_to_sources: impl Iterator<Item = impl Into<PathBuf>>,
    ) -> Compiler<B, CompilerSourcesSet> {
        Compiler {
            state: CompilerSourcesSet {
                sources: paths_to_sources
                    .map(|p| AsnSource::Path(p.into()))
                    .collect(),
            },
            backend: self.backend,
        }
    }

    /// Add a literal ASN1 source to the compile command
    /// * `literal` - literal ASN1 statement to include
    /// ```rust
    /// # use rasn_compiler::prelude::*;
    /// Compiler::<RasnBackend, _>::new().add_asn_literal(format!(
    ///     "TestModule DEFINITIONS AUTOMATIC TAGS::= BEGIN {} END",
    ///     "My-test-integer ::= INTEGER (1..128)"
    /// )).compile_to_string();
    /// ```
    pub fn add_asn_literal(self, literal: impl Into<String>) -> Compiler<B, CompilerSourcesSet> {
        Compiler {
            state: CompilerSourcesSet {
                sources: vec![AsnSource::Literal(literal.into())],
            },
            backend: self.backend,
        }
    }

    /// Set the output path for the generated bindings.
    /// * `output_path` - path to an output file or directory, if path indicates a directory, the
    ///   output file is named `generated` with the extension used by the Backend.
    #[deprecated = "Use `Self::set_output_mode` instead"]
    pub fn set_output_path(
        self,
        output_path: impl Into<PathBuf>,
    ) -> Compiler<B, CompilerOutputSet> {
        Compiler {
            state: CompilerOutputSet {
                output_mode: OutputMode::SingleFile(output_path.into()),
            },
            backend: self.backend,
        }
    }

    /// Set the output destination for the generated bindings.
    pub fn set_output_mode(self, output_mode: OutputMode) -> Compiler<B, CompilerOutputSet> {
        Compiler {
            state: CompilerOutputSet { output_mode },
            backend: self.backend,
        }
    }
}

impl<B: Backend> Compiler<B, CompilerOutputSet> {
    /// Add an ASN1 source to the compile command by path
    /// * `path_to_source` - path to ASN1 file to include
    pub fn add_asn_by_path(self, path_to_source: impl Into<PathBuf>) -> Compiler<B, CompilerReady> {
        Compiler {
            state: CompilerReady {
                sources: vec![AsnSource::Path(path_to_source.into())],
                output_mode: self.state.output_mode,
            },
            backend: self.backend,
        }
    }

    /// Add several ASN1 sources by path to the compile command
    /// * `path_to_source` - iterator of paths to the ASN1 files to be included
    pub fn add_asn_sources_by_path(
        self,
        paths_to_sources: impl Iterator<Item = impl Into<PathBuf>>,
    ) -> Compiler<B, CompilerReady> {
        Compiler {
            state: CompilerReady {
                sources: paths_to_sources
                    .map(|p| AsnSource::Path(p.into()))
                    .collect(),
                output_mode: self.state.output_mode,
            },
            backend: self.backend,
        }
    }

    /// Add a literal ASN1 source to the compile command
    /// * `literal` - literal ASN1 statement to include
    /// ```rust
    /// # use rasn_compiler::prelude::*;
    /// Compiler::<RasnBackend, _>::new().add_asn_literal(format!(
    ///     "TestModule DEFINITIONS AUTOMATIC TAGS::= BEGIN {} END",
    ///     "My-test-integer ::= INTEGER (1..128)"
    /// )).compile_to_string();
    /// ```
    pub fn add_asn_literal(self, literal: impl Into<String>) -> Compiler<B, CompilerReady> {
        Compiler {
            state: CompilerReady {
                sources: vec![AsnSource::Literal(literal.into())],
                output_mode: self.state.output_mode,
            },
            backend: self.backend,
        }
    }
}

impl<B: Backend> Compiler<B, CompilerSourcesSet> {
    /// Add an ASN1 source to the compile command by path
    /// * `path_to_source` - path to ASN1 file to include
    pub fn add_asn_by_path(
        self,
        path_to_source: impl Into<PathBuf>,
    ) -> Compiler<B, CompilerSourcesSet> {
        let mut sources: Vec<AsnSource> = self.state.sources;
        sources.push(AsnSource::Path(path_to_source.into()));
        Compiler {
            state: CompilerSourcesSet { sources },
            backend: self.backend,
        }
    }

    /// Add several ASN1 sources by path to the compile command
    /// * `path_to_source` - iterator of paths to the ASN1 files to be included
    pub fn add_asn_sources_by_path(
        self,
        paths_to_sources: impl Iterator<Item = impl Into<PathBuf>>,
    ) -> Compiler<B, CompilerSourcesSet> {
        let mut sources: Vec<AsnSource> = self.state.sources;
        sources.extend(paths_to_sources.map(|p| AsnSource::Path(p.into())));
        Compiler {
            state: CompilerSourcesSet { sources },
            backend: self.backend,
        }
    }

    /// Add a literal ASN1 source to the compile command
    /// * `literal` - literal ASN1 statement to include
    /// ```rust
    /// # use rasn_compiler::prelude::*;
    /// Compiler::<RasnBackend, _>::new().add_asn_literal(format!(
    ///     "TestModule DEFINITIONS AUTOMATIC TAGS::= BEGIN {} END",
    ///     "My-test-integer ::= INTEGER (1..128)"
    /// )).compile_to_string();
    /// ```
    pub fn add_asn_literal(self, literal: impl Into<String>) -> Compiler<B, CompilerSourcesSet> {
        let mut sources: Vec<AsnSource> = self.state.sources;
        sources.push(AsnSource::Literal(literal.into()));
        Compiler {
            state: CompilerSourcesSet { sources },
            backend: self.backend,
        }
    }

    /// Set the output path for the generated bindings.
    /// * `output_path` - path to an output file or directory, if path indicates a directory, the
    ///   output file is named `generated` with the extension used by the Backend.
    #[deprecated = "Use `Self::set_output_mode` instead"]
    pub fn set_output_path(self, output_path: impl Into<PathBuf>) -> Compiler<B, CompilerReady> {
        Compiler {
            state: CompilerReady {
                sources: self.state.sources,
                output_mode: OutputMode::SingleFile(output_path.into()),
            },
            backend: self.backend,
        }
    }

    /// Set the output destination for the generated bindings.
    pub fn set_output_mode(self, output_mode: OutputMode) -> Compiler<B, CompilerReady> {
        Compiler {
            state: CompilerReady {
                sources: self.state.sources,
                output_mode,
            },
            backend: self.backend,
        }
    }

    /// Runs the rasn compiler command and returns stringified Rust.
    /// Returns a Result wrapping a compilation result:
    /// * _Ok_  - tuple containing the stringified bindings for the ASN1 spec as well as a vector of warnings raised during the compilation
    /// * _Err_ - Unrecoverable error, no bindings were generated
    pub fn compile_to_string(self) -> Result<CompileResult, CompilerError> {
        self.set_output_mode(OutputMode::NoOutput)
            .compile_to_string()
    }
}

impl<B: Backend> Compiler<B, CompilerReady> {
    /// Add an ASN1 source to the compile command by path
    /// * `path_to_source` - path to ASN1 file to include
    pub fn add_asn_by_path(self, path_to_source: impl Into<PathBuf>) -> Compiler<B, CompilerReady> {
        let mut sources: Vec<AsnSource> = self.state.sources;
        sources.push(AsnSource::Path(path_to_source.into()));
        Compiler {
            state: CompilerReady {
                output_mode: self.state.output_mode,
                sources,
            },
            backend: self.backend,
        }
    }

    /// Add several ASN1 sources by path to the compile command
    /// * `path_to_source` - iterator of paths to the ASN1 files to be included
    pub fn add_asn_sources_by_path(
        self,
        paths_to_sources: impl Iterator<Item = impl Into<PathBuf>>,
    ) -> Compiler<B, CompilerReady> {
        let mut sources: Vec<AsnSource> = self.state.sources;
        sources.extend(paths_to_sources.map(|p| AsnSource::Path(p.into())));
        Compiler {
            state: CompilerReady {
                sources,
                output_mode: self.state.output_mode,
            },
            backend: self.backend,
        }
    }

    /// Add a literal ASN1 source to the compile command
    /// * `literal` - literal ASN1 statement to include
    /// ```rust
    /// # use rasn_compiler::prelude::*;
    /// Compiler::<RasnBackend, _>::new().add_asn_literal(format!(
    ///     "TestModule DEFINITIONS AUTOMATIC TAGS::= BEGIN {} END",
    ///     "My-test-integer ::= INTEGER (1..128)"
    /// )).compile_to_string();
    /// ```
    pub fn add_asn_literal(self, literal: impl Into<String>) -> Compiler<B, CompilerReady> {
        let mut sources: Vec<AsnSource> = self.state.sources;
        sources.push(AsnSource::Literal(literal.into()));
        Compiler {
            state: CompilerReady {
                output_mode: self.state.output_mode,
                sources,
            },
            backend: self.backend,
        }
    }

    /// Runs the rasn compiler command and returns stringified Rust.
    /// Returns a Result wrapping a compilation result:
    /// * _Ok_  - tuple containing the stringified bindings for the ASN1 spec as well as a vector of warnings raised during the compilation
    /// * _Err_ - Unrecoverable error, no bindings were generated
    pub fn compile_to_string(mut self) -> Result<CompileResult, CompilerError> {
        self.internal_compile().map(CompileResult::fmt::<B>)
    }

    /// Runs the rasn compiler command.
    /// Returns a Result wrapping a compilation result:
    /// * _Ok_  - Vector of warnings raised during the compilation
    /// * _Err_ - Unrecoverable error, no bindings were generated
    pub fn compile(mut self) -> Result<Vec<CompilerError>, CompilerError> {
        let result = self.internal_compile()?.fmt::<B>();

        self.output_generated(&result.generated)?;

        Ok(result.warnings)
    }

    fn internal_compile(&mut self) -> Result<CompileResult, CompilerError> {
        let mut generated_modules = vec![];
        let mut warnings = Vec::<CompilerError>::new();
        let mut modules: Vec<ToplevelDefinition> = vec![];
        for src in &self.state.sources {
            let src_unit = src.try_into()?;
            modules.extend(asn_spec(src_unit)?.into_iter().flat_map(|(header, tlds)| {
                let header_ref = Rc::new(RefCell::new(header));
                tlds.into_iter().map(move |mut tld| {
                    tld.apply_tagging_environment(&header_ref.borrow().tagging_environment);
                    tld.set_module_header(header_ref.clone());
                    tld
                })
            }));
        }
        let (valid_items, mut validator_errors) = Validator::new(modules).validate()?;
        let modules = valid_items.into_iter().fold(
            BTreeMap::<String, Vec<ToplevelDefinition>>::new(),
            |mut modules, tld| {
                let key = tld
                    .get_module_header()
                    .map_or(<_>::default(), |module| module.borrow().name.clone());
                match modules.entry(key) {
                    std::collections::btree_map::Entry::Vacant(v) => {
                        v.insert(vec![tld]);
                    }
                    std::collections::btree_map::Entry::Occupied(ref mut e) => {
                        e.get_mut().push(tld)
                    }
                }
                modules
            },
        );
        for (_, module) in modules {
            let mut generated_module = self.backend.generate_module(module)?;
            if let Some(m) = generated_module.generated {
                generated_modules.push(m);
            }
            warnings.append(&mut generated_module.warnings);
        }
        warnings.append(&mut validator_errors);

        Ok(CompileResult {
            generated: generated_modules.join("\n"),
            warnings,
        })
    }

    fn output_generated(&self, generated: &str) -> Result<(), GeneratorError> {
        match &self.state.output_mode {
            OutputMode::SingleFile(path) => {
                let path = if path.is_dir() {
                    &path.join(format!("generated{}", B::FILE_EXTENSION))
                } else {
                    path
                };
                fs::write(path, generated).map_err(|e| {
                    GeneratorError::new(
                        None,
                        &format!(
                            "Failed to write generated bindings to {}: {e}",
                            path.display()
                        ),
                        GeneratorErrorType::IO,
                    )
                })
            }
            OutputMode::Stdout => {
                std::io::stdout()
                    .write_all(generated.as_bytes())
                    .map_err(|err| {
                        GeneratorError::new(
                            None,
                            &format!("Failed to write generated bindings to stdout: {err}"),
                            GeneratorErrorType::IO,
                        )
                    })
            }
            OutputMode::NoOutput => Ok(()),
        }
    }
}

/// Where the [Compiler] output should go.
#[derive(Debug)]
pub enum OutputMode {
    /// Write all compiled modules to a single file. Uses a default filename if path is a
    /// directory.
    SingleFile(PathBuf),
    /// Write all compiled modules to stdout.
    Stdout,
    /// Do not write anything, only check.
    NoOutput,
}

struct AsnSourceUnit<'a> {
    path: Option<&'a Path>,
    source: Cow<'a, str>,
}

impl<'a> From<&'a str> for AsnSourceUnit<'a> {
    fn from(value: &'a str) -> Self {
        Self {
            path: None,
            source: Cow::Borrowed(value),
        }
    }
}

impl<'a> TryFrom<&'a AsnSource> for AsnSourceUnit<'a> {
    type Error = CompilerError;

    fn try_from(value: &'a AsnSource) -> Result<Self, Self::Error> {
        match value {
            AsnSource::Path(path) => Ok(AsnSourceUnit {
                path: Some(path),
                source: Cow::Owned(read_to_string(path).map_err(LexerError::from)?),
            }),
            AsnSource::Literal(literal) => Ok(AsnSourceUnit {
                path: None,
                source: Cow::Borrowed(literal),
            }),
        }
    }
}
