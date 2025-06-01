use std::path::PathBuf;

use crate::prelude::*;

use clap::{arg, command, Parser};
use colored::Colorize;
use walkdir::WalkDir;

#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CompilerArgs {
    #[clap(flatten)]
    source: SourceArgsGroup,

    /// Set the output path for the generated rust module
    #[arg(short, long, default_value = ".")]
    output_path: PathBuf,

    /// Specify which compiler backend to use:
    ///  - "rasn" [DEFAULT]: generates rust-bindings for the rasn framework
    ///  - "typescript": generates typescript type definitions
    #[arg(short, long, default_value = "rasn")]
    backend: String,
}

#[derive(clap::Args, Debug)]
#[group(required = true, multiple = true)]
pub struct SourceArgsGroup {
    /// Specify a directory for the compiler to search for ASN1 modules.
    /// The compiler will search recursively for `.asn` and `.asn1` files
    #[arg(short, long)]
    directory: Option<PathBuf>,

    /// Add an ASN1 module by path. Multiple modules can be added by appending "-m PATH_TO_MODULE"
    #[arg(short, long = "module", num_args(0..))]
    module_files: Vec<PathBuf>,
}

impl CompilerArgs {
    pub fn drive() {
        let args = CompilerArgs::parse();

        // Read module paths
        let mut modules = args.source.module_files;

        // Scan directory, if given
        if let Some(dir) = &args.source.directory {
            for entry in WalkDir::new(dir)
                .follow_links(true)
                .into_iter()
                .filter_map(|e| e.ok())
            {
                let file_name = entry.file_name().to_string_lossy();

                if file_name.ends_with(".asn") || file_name.ends_with(".asn1") {
                    println!("Found ASN1 module {} in directory", file_name);
                    modules.push(entry.into_path());
                }
            }
        }

        if modules.is_empty() {
            println!("{}", "No modules found".red());
            return;
        }

        let results = if args.backend == "typescript" {
            Compiler::<TypescriptBackend, _>::new()
                .add_asn_sources_by_path(modules.into_iter())
                .set_output_path(args.output_path)
                .compile()
        } else {
            Compiler::<RasnBackend, _>::new()
                .add_asn_sources_by_path(modules.into_iter())
                .set_output_path(args.output_path)
                .compile()
        };

        match results {
            Ok(warnings) => {
                for warning in warnings {
                    println!(
                        "{}\n{}",
                        "Rasn compiler warning:".yellow(),
                        warning.to_string().yellow()
                    )
                }
            }
            Err(error) => {
                println!(
                    "{}\n{}",
                    "Rasn compiler error:".red(),
                    error.to_string().red()
                )
            }
        }
    }
}
