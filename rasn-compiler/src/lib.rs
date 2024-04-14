#![allow(clippy::needless_doctest_main)]
//! The `rasn-compiler` library is a parser combinator that parses ASN1 specifications and outputs
//! encoding-rule-agnotic bindings for the ASN.1 data elements to be used with the `rasn` crate.
//! The compiler heavily relies on the great library [nom](https://docs.rs/nom/latest/nom/) for its basic parsers.
//!
//! ## Example
//!
//! In order to compile ASN.1 in your build process, invoke the rasn compiler in your [`build.rs` build script](https://doc.rust-lang.org/cargo/reference/build-scripts.html).
//!
//! ```rust
//! // build.rs build script
//! use std::path::PathBuf;
//! use rasn_compiler::prelude::*;
//!
//! fn main() {
//!   struct CustomBackend;
//!
//!   impl Backend for CustomBackend {
//!     fn generate_module(
//!          &self,
//!          top_level_declarations: Vec<ToplevelDefinition>,
//!     ) -> Result<GeneratedModule, GeneratorError> {
//!         Ok(GeneratedModule::empty())
//!     }
//!   }
//!
//!   // Initialize the compiler
//!   match Compiler::new()
//!     // optionally provide a custom backend
//!     .with_backend(CustomBackend)
//!     // add a single ASN1 source file
//!     .add_asn_by_path(PathBuf::from("spec_1.asn"))
//!     // add several ASN1 source files
//!     .add_asn_sources_by_path(vec![
//!         PathBuf::from("spec_2.asn"),
//!         PathBuf::from("spec_3.asn"),
//!     ].iter())
//!     // set an output path for the generated rust code
//!     .set_output_path(PathBuf::from("./asn/generated.rs"))
//!     // you may also compile literal ASN1 snippets
//!     .add_asn_literal(format!(
//!         "TestModule DEFINITIONS AUTOMATIC TAGS::= BEGIN {} END",
//!         "My-test-integer ::= INTEGER (1..128)"
//!     ))
//!     .compile() {
//!     Ok(warnings /* Vec<Box<dyn Error>> */) => { /* handle compilation warnings */ }
//!     Err(error /* Box<dyn Error> */) => { /* handle unrecoverable compilation error */ }
//!   }
//! }
//! ```
pub(crate) mod common;
mod generator;
pub mod intermediate;
mod parser;
mod validator;

use std::{
    collections::BTreeMap,
    env::{self},
    error::Error,
    fs::{self, read_to_string},
    io::{self, Write},
    path::PathBuf,
    process::{Command, Stdio},
    rc::Rc,
    vec,
};

use generator::{rasn::Rust, Backend};
use intermediate::ToplevelDefinition;
use parser::asn_spec;
use validator::Validator;

pub mod prelude {
    //! Convenience module that collects all necessary imports for
    //! using and customizing the compiler.
    pub use super::{
        CompileResult, Compiler, CompilerMissingParams, CompilerOutputSet, CompilerReady,
        CompilerSourcesSet,
    };
    pub use crate::generator::{error::*, Backend, GeneratedModule};
    pub use crate::intermediate::ToplevelDefinition;
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
pub fn compile(asn1: &str) -> Result<Generated, JsValue> {
    Compiler::new()
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
    output_path: PathBuf,
}

/// Typestate representing compiler that has the output path set, but is missing ASN1 sources
pub struct CompilerOutputSet {
    output_path: PathBuf,
}

/// Typestate representing compiler that knows about ASN1 sources, but doesn't have an output path set
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
    pub warnings: Vec<Box<dyn Error>>,
}

impl CompileResult {
    fn rust_fmt(mut self) -> Self {
        self.generated = format_bindings(&self.generated).unwrap_or(self.generated);
        self
    }
}

#[derive(Debug, PartialEq)]
enum AsnSource {
    Path(PathBuf),
    Literal(String),
}

impl Default for Compiler<Rust, CompilerMissingParams> {
    fn default() -> Self {
        Compiler::new()
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

impl Compiler<Rust, CompilerMissingParams> {
    /// Provides a Builder for building rasn compiler commands
    pub fn new() -> Compiler<Rust, CompilerMissingParams> {
        Compiler {
            state: CompilerMissingParams,
            backend: Rust,
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
    /// # use rasn_compiler::Compiler;
    /// Compiler::new().add_asn_literal(format!(
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

    /// Set the output path for the generated rust representation.
    /// * `output_path` - path to an output file or directory, if path indicates
    ///                   a directory, the output file is named `rasn_generated.rs`
    pub fn set_output_path(
        self,
        output_path: impl Into<PathBuf>,
    ) -> Compiler<B, CompilerOutputSet> {
        let mut path: PathBuf = output_path.into();
        if path.is_dir() {
            path.set_file_name("rasn_generated.rs");
        }
        Compiler {
            state: CompilerOutputSet { output_path: path },
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
                output_path: self.state.output_path,
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
                output_path: self.state.output_path,
            },
            backend: self.backend,
        }
    }

    /// Add a literal ASN1 source to the compile command
    /// * `literal` - literal ASN1 statement to include
    /// ```rust
    /// # use rasn_compiler::Compiler;
    /// Compiler::new().add_asn_literal(format!(
    ///     "TestModule DEFINITIONS AUTOMATIC TAGS::= BEGIN {} END",
    ///     "My-test-integer ::= INTEGER (1..128)"
    /// )).compile_to_string();
    /// ```
    pub fn add_asn_literal(self, literal: impl Into<String>) -> Compiler<B, CompilerReady> {
        Compiler {
            state: CompilerReady {
                sources: vec![AsnSource::Literal(literal.into())],
                output_path: self.state.output_path,
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
    /// # use rasn_compiler::Compiler;
    /// Compiler::new().add_asn_literal(format!(
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

    /// Set the output path for the generated rust representation.
    /// * `output_path` - path to an output file or directory, if path points to
    ///                   a directory, the compiler will generate a file for every ASN.1 module.
    ///                   If the path points to a file, all modules will be written to that file.
    pub fn set_output_path(self, output_path: impl Into<PathBuf>) -> Compiler<B, CompilerReady> {
        Compiler {
            state: CompilerReady {
                sources: self.state.sources,
                output_path: output_path.into(),
            },
            backend: self.backend,
        }
    }

    /// Runs the rasn compiler command and returns stringified Rust.
    /// Returns a Result wrapping a compilation result:
    /// * _Ok_  - tuple containing the stringified bindings for the ASN1 spec as well as a vector of warnings raised during the compilation
    /// * _Err_ - Unrecoverable error, no rust representations were generated
    pub fn compile_to_string(self) -> Result<CompileResult, Box<dyn Error>> {
        self.internal_compile().map(CompileResult::rust_fmt)
    }

    fn internal_compile(&self) -> Result<CompileResult, Box<dyn Error>> {
        let mut generated_modules = vec![];
        let mut warnings = Vec::<Box<dyn Error>>::new();
        let mut modules: Vec<ToplevelDefinition> = vec![];
        for src in &self.state.sources {
            let stringified_src = match src {
                AsnSource::Path(p) => read_to_string(p)?,
                AsnSource::Literal(l) => l.clone(),
            };
            modules.append(
                &mut asn_spec(&stringified_src)?
                    .into_iter()
                    .flat_map(|(header, tlds)| {
                        let header_ref = Rc::new(header);
                        tlds.into_iter().enumerate().map(move |(index, mut tld)| {
                            tld.apply_tagging_environment(&header_ref.tagging_environment);
                            tld.set_index(header_ref.clone(), index);
                            tld
                        })
                    })
                    .collect(),
            );
        }
        let (valid_items, mut validator_errors) = Validator::new(modules).validate()?;
        let modules = valid_items.into_iter().fold(
            BTreeMap::<String, Vec<ToplevelDefinition>>::new(),
            |mut modules, tld| {
                let key = tld
                    .get_index()
                    .map_or(<_>::default(), |(module, _)| module.name.clone());
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
}

impl<B: Backend> Compiler<B, CompilerReady> {
    /// Add an ASN1 source to the compile command by path
    /// * `path_to_source` - path to ASN1 file to include
    pub fn add_asn_by_path(self, path_to_source: impl Into<PathBuf>) -> Compiler<B, CompilerReady> {
        let mut sources: Vec<AsnSource> = self.state.sources;
        sources.push(AsnSource::Path(path_to_source.into()));
        Compiler {
            state: CompilerReady {
                output_path: self.state.output_path,
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
                output_path: self.state.output_path,
            },
            backend: self.backend,
        }
    }

    /// Add a literal ASN1 source to the compile command
    /// * `literal` - literal ASN1 statement to include
    /// ```rust
    /// # use rasn_compiler::Compiler;
    /// Compiler::new().add_asn_literal(format!(
    ///     "TestModule DEFINITIONS AUTOMATIC TAGS::= BEGIN {} END",
    ///     "My-test-integer ::= INTEGER (1..128)"
    /// )).compile_to_string();
    /// ```
    pub fn add_asn_literal(self, literal: impl Into<String>) -> Compiler<B, CompilerReady> {
        let mut sources: Vec<AsnSource> = self.state.sources;
        sources.push(AsnSource::Literal(literal.into()));
        Compiler {
            state: CompilerReady {
                output_path: self.state.output_path,
                sources,
            },
            backend: self.backend,
        }
    }

    /// Runs the rasn compiler command and returns stringified Rust.
    /// Returns a Result wrapping a compilation result:
    /// * _Ok_  - tuple containing the stringified bindings for the ASN1 spec as well as a vector of warnings raised during the compilation
    /// * _Err_ - Unrecoverable error, no rust representations were generated
    pub fn compile_to_string(self) -> Result<CompileResult, Box<dyn Error>> {
        Compiler {
            state: CompilerSourcesSet {
                sources: self.state.sources,
            },
            backend: self.backend,
        }
        .compile_to_string()
    }

    /// Runs the rasn compiler command.
    /// Returns a Result wrapping a compilation result:
    /// * _Ok_  - Vector of warnings raised during the compilation
    /// * _Err_ - Unrecoverable error, no rust representations were generated
    pub fn compile(self) -> Result<Vec<Box<dyn Error>>, Box<dyn Error>> {
        let result = Compiler {
            state: CompilerSourcesSet {
                sources: self.state.sources,
            },
            backend: self.backend,
        }
        .internal_compile()?
        .rust_fmt();
        fs::write(
            self.state
                .output_path
                .is_dir()
                .then(|| self.state.output_path.join("generated.rs"))
                .unwrap_or(self.state.output_path),
            result.generated,
        )?;

        Ok(result.warnings)
    }
}

fn format_bindings(bindings: &String) -> Result<String, Box<dyn Error>> {
    let mut rustfmt = PathBuf::from(env::var("CARGO_HOME")?);
    rustfmt.push("bin/rustfmt");
    let mut cmd = Command::new(&*rustfmt);

    cmd.stdin(Stdio::piped()).stdout(Stdio::piped());

    let mut child = cmd.spawn()?;
    let mut child_stdin = child.stdin.take().unwrap();
    let mut child_stdout = child.stdout.take().unwrap();

    // Write to stdin in a new thread, so that we can read from stdout on this
    // thread. This keeps the child from blocking on writing to its stdout which
    // might block us from writing to its stdin.
    let bindings = bindings.to_owned();
    let stdin_handle = ::std::thread::spawn(move || {
        let _ = child_stdin.write_all(bindings.as_bytes());
        bindings
    });

    let mut output = vec![];
    io::copy(&mut child_stdout, &mut output)?;

    let status = child.wait()?;
    let bindings = stdin_handle.join().expect(
        "The thread writing to rustfmt's stdin doesn't do \
             anything that could panic",
    );

    match String::from_utf8(output) {
        Ok(bindings) => match status.code() {
            Some(0) => Ok(bindings),
            Some(2) => Err(Box::new(io::Error::new(
                io::ErrorKind::Other,
                "Rustfmt parsing errors.".to_string(),
            ))),
            Some(3) => Ok(bindings),
            _ => Err(Box::new(io::Error::new(
                io::ErrorKind::Other,
                "Internal rustfmt error".to_string(),
            ))),
        },
        _ => Ok(bindings),
    }
}
