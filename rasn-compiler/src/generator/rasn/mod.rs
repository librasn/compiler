use std::{
    env,
    error::Error,
    io::{self, Write},
    path::PathBuf,
    process::{Command, Stdio},
    str::FromStr,
};

use crate::{error::CompilerError, intermediate::*};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

#[cfg(target_family = "wasm")]
use wasm_bindgen::prelude::*;

use super::{
    error::{GeneratorError, GeneratorErrorType},
    Backend, GeneratedModule,
};

mod builder;
mod template;
mod utils;

#[derive(Debug, Default)]
/// A compiler backend that generates bindings to be used with
/// the `rasn` framework for rust.
pub struct Rasn {
    config: Config,
    tagging_environment: TaggingEnvironment,
    extensibility_environment: ExtensibilityEnvironment,
}

#[cfg_attr(target_family = "wasm", wasm_bindgen)]
#[derive(Debug)]
/// A configuration for the [Rasn] backend
pub struct Config {
    /// ASN.1 Open Types are represented as the `rasn::types::Any` type,
    /// which holds a binary `content`. If `opaque_open_types` is `false`,
    /// the compiler will generate additional de-/encode methods for
    /// all rust types that hold an open type.
    /// For example, bindings for a `SEQUENCE` with a field of Open Type
    /// value will include a method for explicitly decoding the Open Type field.
    /// _Non-opaque open types are still experimental. If you have trouble_
    /// _generating correct bindings, switch back to opaque open types._
    pub opaque_open_types: bool,
    /// The compiler will try to match module import dependencies of the ASN.1
    /// module as close as possible, importing only those types from other modules
    /// that are imported in the ASN.1 module. If the `default_wildcard_imports`
    /// is set to `true` , the compiler will import the entire module using
    /// the wildcard `*` for each module that the input ASN.1 module imports from.
    pub default_wildcard_imports: bool,
    /// To make working with the generated types a bit more ergonomic, the compiler
    /// can generate `From` impls for the wrapper inner types in a `CHOICE`, as long
    /// as the generated impls are not ambiguous.
    /// This is disabled by default to generate less code, but can be enabled with
    /// `generate_from_impls` set to `true`.
    pub generate_from_impls: bool,
}

#[cfg(target_family = "wasm")]
#[wasm_bindgen]
impl Config {
    #[wasm_bindgen(constructor)]
    pub fn new(
        opaque_open_types: bool,
        default_wildcard_imports: bool,
        generate_from_impls: Option<bool>,
    ) -> Self {
        Self {
            opaque_open_types,
            default_wildcard_imports,
            generate_from_impls: generate_from_impls.unwrap_or(false),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            opaque_open_types: true,
            default_wildcard_imports: false,
            generate_from_impls: false,
        }
    }
}

impl Backend for Rasn {
    type Config = Config;

    const FILE_EXTENSION: &'static str = ".rs";

    fn new(
        config: Self::Config,
        tagging_environment: TaggingEnvironment,
        extensibility_environment: ExtensibilityEnvironment,
    ) -> Self {
        Self {
            config,
            extensibility_environment,
            tagging_environment,
        }
    }

    fn from_config(config: Self::Config) -> Self {
        Self {
            config,
            ..Default::default()
        }
    }

    fn config(&self) -> &Self::Config {
        &self.config
    }

    fn generate_module(
        &mut self,
        tlds: Vec<ToplevelDefinition>,
    ) -> Result<GeneratedModule, GeneratorError> {
        if let Some((module_ref, _)) = tlds.first().and_then(|tld| tld.get_index().cloned()) {
            let module = module_ref.borrow();
            self.tagging_environment = module.tagging_environment;
            self.extensibility_environment = module.extensibility_environment;
            let name = self.to_rust_snake_case(&module.name);
            let imports = module.imports.iter().map(|import| {
                let module =
                    self.to_rust_snake_case(&import.global_module_reference.module_reference);
                let mut usages = Some(vec![]);
                'imports: for usage in &import.types {
                    if usage.contains("{}") || usage.chars().all(|c| c.is_uppercase() || c == '-') {
                        usages = None;
                        break 'imports;
                    } else if usage.starts_with(|c: char| c.is_lowercase()) {
                        if let Some(us) = usages.as_mut() {
                            us.push(self.to_rust_const_case(usage).to_token_stream())
                        }
                    } else if usage.starts_with(|c: char| c.is_uppercase()) {
                        if let Some(us) = usages.as_mut() {
                            us.push(self.to_rust_title_case(usage).to_token_stream())
                        }
                    }
                }
                let used_imports = if self.config.default_wildcard_imports {
                    vec![TokenStream::from_str("*").unwrap()]
                } else {
                    usages.unwrap_or(vec![TokenStream::from_str("*").unwrap()])
                };
                quote!(use super:: #module::{ #(#used_imports),* };)
            });
            let (pdus, warnings): (Vec<TokenStream>, Vec<CompilerError>) =
                tlds.into_iter().fold((vec![], vec![]), |mut acc, tld| {
                    match self.generate_tld(tld) {
                        Ok(s) => {
                            acc.0.push(s);
                            acc
                        }
                        Err(e) => {
                            acc.1.push(e.into());
                            acc
                        }
                    }
                });
            Ok(GeneratedModule {
                generated: Some(quote! {
                #[allow(non_camel_case_types, non_snake_case, non_upper_case_globals, unused,
                        clippy::too_many_arguments,)]
                pub mod #name {
                    extern crate alloc;

                    use core::borrow::Borrow;
                    use rasn::prelude::*;
                    use lazy_static::lazy_static;

                    #(#imports)*

                    #(#pdus)*
                }
            }.to_string()), warnings})
        } else {
            Ok(GeneratedModule::empty())
        }
    }

    fn format_bindings(bindings: &str) -> Result<String, CompilerError> {
        Self::internal_fmt(bindings).map_err(|e| {
            GeneratorError {
                top_level_declaration: None,
                details: e.to_string(),
                kind: GeneratorErrorType::FormattingError,
            }
            .into()
        })
    }

    fn generate(&self, tld: ToplevelDefinition) -> Result<String, GeneratorError> {
        self.generate_tld(tld).map(|ts| ts.to_string())
    }
}

impl Rasn {
    fn get_rustfmt_path() -> Result<PathBuf, Box<dyn Error>> {
        // Try ~/.cargo/bin/rustfmt style paths first
        if let Ok(path) = env::var("CARGO_HOME")
            .map(PathBuf::from)
            .map(|mut path| {
                path.push("bin/rustfmt");
                path
            }) {
            if path.exists() {
                return Ok(path);
            }
        }

        // Alternatively, maybe rustfmt and cargo are in the same directory
        if let Ok(path) = env::var("CARGO")
            .map(PathBuf::from)
            .map(|mut path| {
                path.set_file_name("rustfmt");
                path
            }) {
            if path.exists() {
                return Ok(path);
            }
        }

        Err("No rustfmt found.".into())
    }

    fn internal_fmt(bindings: &str) -> Result<String, Box<dyn Error>> {
        let rustfmt = Self::get_rustfmt_path()?;
        let mut cmd = Command::new(rustfmt);

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
}
