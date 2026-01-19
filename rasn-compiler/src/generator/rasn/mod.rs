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

#[derive(Debug)]
/// A compiler backend that generates bindings to be used with
/// the `rasn` framework for rust.
pub struct Rasn {
    config: Config,
    tagging_environment: TaggingEnvironment,
    extensibility_environment: ExtensibilityEnvironment,
    /// A combination of the builtin required derives (`Rasn::REQUIRED_DERIVES`) and those supplied
    /// by the user in `Config::type_annotations`.
    required_derives: Vec<String>,
}

#[cfg_attr(target_family = "wasm", wasm_bindgen(getter_with_clone))]
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
    /// Stringified paths to items that will be imported into all generated modules with a
    /// [use declaration](https://doc.rust-lang.org/reference/items/use-declarations.html).
    /// For example `vec![String::from("my::module::*"), String::from("path::to::my::Struct")]`.
    pub custom_imports: Vec<String>,
    /// Annotations to be added to all generated rust types of the bindings. Each vector element
    /// will generate a new line of annotations. Note that the compiler will automatically add all pound-derives
    /// needed by `rasn` __except__ `Eq` and `Hash`, which are needed only when working with `SET`s.
    ///
    /// Default: `vec![String::from("#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]")]`
    pub type_annotations: Vec<String>,
    /// Create bindings for a `no_std` environment
    pub no_std_compliant_bindings: bool,
}

#[cfg(target_family = "wasm")]
#[wasm_bindgen]
impl Config {
    #[wasm_bindgen(constructor)]
    pub fn new(
        opaque_open_types: bool,
        default_wildcard_imports: bool,
        no_std_compliant_bindings: bool,
        generate_from_impls: Option<bool>,
        custom_imports: Option<Box<[String]>>,
        type_annotations: Option<Box<[String]>>,
    ) -> Self {
        Self {
            opaque_open_types,
            default_wildcard_imports,
            no_std_compliant_bindings,
            generate_from_impls: generate_from_impls.unwrap_or(false),
            custom_imports: custom_imports.map_or(Vec::new(), |c| c.into_vec()),
            type_annotations: type_annotations
                .map_or(Config::default().type_annotations, |c| c.into_vec()),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            opaque_open_types: true,
            default_wildcard_imports: false,
            generate_from_impls: false,
            no_std_compliant_bindings: false,
            custom_imports: Vec::default(),
            type_annotations: vec![String::from(
                "#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]",
            )],
        }
    }
}

impl Backend for Rasn {
    type Config = Config;

    const FILE_EXTENSION: &'static str = ".rs";

    fn new(
        mut config: Self::Config,
        tagging_environment: TaggingEnvironment,
        extensibility_environment: ExtensibilityEnvironment,
    ) -> Self {
        // Remove derive's from config.type_annotations, so that they can be combined with required
        // derives, and handled more optimally in later steps.
        let mut required_derives: Vec<_> = Self::REQUIRED_DERIVES
            .into_iter()
            .map(str::to_owned)
            .collect();
        let mut non_derive_annotations = Vec::new();
        for cfg_annotation in config.type_annotations {
            if let Ok((_, derives)) = parse_rust_derive_annotation(&cfg_annotation) {
                for derive in derives {
                    if !required_derives.iter().any(|d| d == derive) {
                        required_derives.push(derive.to_owned());
                    }
                }
            } else {
                non_derive_annotations.push(cfg_annotation);
            }
        }
        config.type_annotations = non_derive_annotations;

        Self {
            config,
            extensibility_environment,
            tagging_environment,
            required_derives,
        }
    }

    fn from_config(config: Self::Config) -> Self {
        Self::new(
            config,
            TaggingEnvironment::default(),
            ExtensibilityEnvironment::default(),
        )
    }

    fn config(&self) -> &Self::Config {
        &self.config
    }

    fn generate_module(
        &mut self,
        tlds: Vec<ToplevelDefinition>,
    ) -> Result<GeneratedModule, GeneratorError> {
        if let Some(module_ref) = tlds.first().and_then(|tld| tld.get_module_header()) {
            let module = module_ref.borrow();
            self.tagging_environment = module.tagging_environment;
            self.extensibility_environment = module.extensibility_environment;
            let name = self.to_rust_snake_case(&module.name);
            let custom_imports = self
                .config
                .custom_imports
                .iter()
                .map(|i| TokenStream::from_str(i.as_str()).map(|i| quote!(use #i;)))
                .collect::<Result<Vec<_>, _>>()
                .map_err(|e| GeneratorError {
                    details: e.to_string(),
                    ..Default::default()
                })?;
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
            let lazy_const_import = if self.config.no_std_compliant_bindings {
                quote!(lazy_static::lazy_static)
            } else {
                quote!(std::sync::LazyLock)
            };
            Ok(GeneratedModule {
                generated: Some(quote! {
                #[allow(non_camel_case_types, non_snake_case, non_upper_case_globals, unused,
                        clippy::too_many_arguments,)]
                pub mod #name {
                    extern crate alloc;

                    use core::borrow::Borrow;
                    use #lazy_const_import;
                    use rasn::prelude::*;
                    #(#custom_imports)*
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
    const REQUIRED_DERIVES: [&'static str; 6] =
        ["AsnType", "Debug", "Clone", "Decode", "Encode", "PartialEq"];

    fn get_rustfmt_path() -> Result<PathBuf, Box<dyn Error>> {
        // Try ~/.cargo/bin/rustfmt style paths first
        if let Ok(path) = env::var("CARGO_HOME").map(PathBuf::from).map(|mut path| {
            path.push("bin/rustfmt");
            path
        }) {
            if path.exists() {
                return Ok(path);
            }
        }

        // Alternatively, maybe rustfmt and cargo are in the same directory
        if let Ok(path) = env::var("CARGO").map(PathBuf::from).map(|mut path| {
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
                Some(2) => Err(Box::new(io::Error::other("Rustfmt parsing errors."))),
                Some(3) => Ok(bindings),
                _ => Err(Box::new(io::Error::other("Internal rustfmt error"))),
            },
            _ => Ok(bindings),
        }
    }
}

fn parse_rust_derive_annotation(input: &str) -> nom::IResult<&str, Vec<&str>> {
    use nom::{
        bytes::complete::tag,
        character::complete::{alphanumeric1, char, multispace0},
        multi::{many0, separated_list1},
        sequence::delimited,
        Parser as _,
    };

    delimited(
        (
            multispace0,
            char('#'),
            multispace0,
            char('['),
            multispace0,
            tag("derive"),
            multispace0,
            char('('),
            multispace0,
        ),
        separated_list1(many0((multispace0, char(','), multispace0)), alphanumeric1),
        (multispace0, char(')'), multispace0, char(']')),
    )
    .parse(input)
}

impl Default for Rasn {
    fn default() -> Self {
        Self::new(
            Config::default(),
            TaggingEnvironment::default(),
            ExtensibilityEnvironment::default(),
        )
    }
}
