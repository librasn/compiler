use std::path::PathBuf;
use std::process::ExitCode;

use clap::{arg, command, Parser};
use colored::Colorize;
use rasn_compiler::{OutputMode, RasnCompiler, TsCompiler};
use walkdir::WalkDir;

#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CompilerArgs {
    #[clap(flatten, next_help_heading = "Input")]
    source: SourceArgsGroup,

    #[clap(flatten, next_help_heading = "Output")]
    output: OutputArgGroup,

    /// Specify which compiler backend to use
    #[arg(short, long, default_value = "rasn")]
    backend: BackendArg,
}

#[derive(clap::Args, Debug)]
#[group(required = true, multiple = true)]
pub struct SourceArgsGroup {
    /// Specify a directory for the compiler to search for ASN1 modules.
    /// The compiler will search recursively for `.asn` and `.asn1` files
    #[arg(short, long, value_name = "DIR")]
    directory: Option<PathBuf>,

    /// Add an ASN1 module by path. Multiple modules can be added by appending "-m PATH_TO_MODULE"
    #[arg(short, long = "module", value_name="FILE", num_args(0..))]
    module_files: Vec<PathBuf>,
}

#[derive(clap::Args, Debug)]
#[group(required = false, multiple = false)]
pub struct OutputArgGroup {
    /// Write all compiled modules to a single file at PATH.
    ///
    /// If PATH is a directory, a default filename will be used in that directory.
    ///
    /// If no output is specified, a default filename in current directory will be used.
    #[arg(short, long, value_name = "PATH")]
    output_path: Option<PathBuf>,

    /// Write all compiled modules to stdout.
    #[arg(long)]
    stdout: bool,

    /// Do not write anything, only check.
    #[arg(long)]
    no_output: bool,
}

#[derive(clap::ValueEnum, Debug, Clone, Copy, PartialEq, Eq)]
enum BackendArg {
    /// Generate rust-bindings for the rasn framework
    Rasn,
    /// Generate typescript type definitions
    Typescript,
}

fn main() -> ExitCode {
    let args = CompilerArgs::parse();

    // Read module paths
    let mut modules = args.source.module_files;

    // Scan directory, if given
    if let Some(dir) = &args.source.directory {
        let mut module_found = false;
        for entry in WalkDir::new(dir).follow_links(true) {
            let entry = match entry {
                Ok(entry) => entry,
                Err(err) => {
                    eprintln!("{}: {err}", "warning".yellow());
                    continue;
                }
            };
            let file_name = entry.file_name().to_string_lossy();

            if file_name.ends_with(".asn") || file_name.ends_with(".asn1") {
                eprintln!("{}: Found ASN1 module {}", "info".blue(), file_name);
                modules.push(entry.into_path());
                module_found = true;
            }
        }

        if !module_found {
            eprintln!(
                "{}: No modules where found in '{}'",
                "warning".yellow(),
                dir.display(),
            );
        }
    }

    if modules.is_empty() {
        eprintln!("{}: No modules", "error".red());
        return ExitCode::FAILURE;
    }

    let output = make_output_mode(args.output);
    let results = match args.backend {
        BackendArg::Rasn => RasnCompiler::new()
            .add_asn_sources_by_path(modules.into_iter())
            .set_output_mode(output)
            .compile(),
        BackendArg::Typescript => TsCompiler::new()
            .add_asn_sources_by_path(modules.into_iter())
            .set_output_mode(output)
            .compile(),
    };

    match results {
        Ok(warnings) => {
            for warning in warnings {
                eprintln!("{}: {warning}", "warning".yellow())
            }
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("{}: {error}", "error".red());
            ExitCode::FAILURE
        }
    }
}

/// Create an [OutputConf] from command arguments, that can be used with
/// [RasnCompiler::set_output].
fn make_output_mode(args: OutputArgGroup) -> OutputMode {
    // Only zero or one output argument is allowed, and enforced by Clap.
    if let Some(v) = args.output_path {
        OutputMode::SingleFile(v)
    } else if args.stdout {
        OutputMode::Stdout
    } else if args.no_output {
        OutputMode::NoOutput
    } else {
        OutputMode::SingleFile(".".into())
    }
}
