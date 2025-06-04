use std::process::ExitCode;

fn main() -> ExitCode {
    rasn_compiler::cli::CompilerArgs::drive()
}
