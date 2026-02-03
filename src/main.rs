//! envcraft - Precise tools for .env files
//!
//! A safe, deterministic CLI tool for validating, comparing, and formatting
//! environment configuration files.

mod cli;
mod diff;
mod error;
mod format;
mod parser;
mod schema;

use std::process::ExitCode;

use cli::{Cli, Commands};
use error::EnvcraftError;

fn main() -> ExitCode {
    let cli = Cli::parse_args();

    let result: Result<bool, EnvcraftError> = match cli.command {
        Commands::Check { schema, envfile } => {
            schema::run_check(&schema, &envfile).map_err(EnvcraftError::from)
        }
        Commands::Diff {
            file1,
            file2,
            redact,
        } => diff::run_diff(&file1, &file2, redact).map_err(EnvcraftError::from),
        Commands::Format { file, in_place } => {
            format::run_format(&file, in_place).map_err(EnvcraftError::from)
        }
    };

    match result {
        Ok(success) => {
            if success {
                ExitCode::SUCCESS
            } else {
                ExitCode::from(1)
            }
        }
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::from(2)
        }
    }
}
