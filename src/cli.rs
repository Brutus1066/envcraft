//! Command-line interface definition for envcraft.
//!
//! Uses clap with derive macros for a clean, type-safe CLI structure.

use std::path::PathBuf;

use clap::{Parser, Subcommand};

/// envcraft - Precise tools for .env files
///
/// A safe, deterministic CLI tool for validating, comparing, and formatting
/// environment configuration files. No AI, no heuristics, no network access.
#[derive(Parser, Debug)]
#[command(name = "envcraft")]
#[command(author = "LazyFrog <support@kindware.dev>")]
#[command(version)]
#[command(about = "Precise tools for .env files", long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

impl Cli {
    /// Parse command-line arguments and return the CLI structure.
    pub fn parse_args() -> Self {
        Self::parse()
    }
}

/// Available subcommands for envcraft.
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Validate a .env file against a YAML schema
    ///
    /// The schema file defines expected keys and their types.
    /// Supported types: string, int, bool
    Check {
        /// Path to the YAML schema file
        #[arg(value_name = "SCHEMA")]
        schema: PathBuf,

        /// Path to the .env file to validate
        #[arg(value_name = "ENVFILE")]
        envfile: PathBuf,
    },

    /// Show semantic differences between two .env files
    ///
    /// Output shows added (+), removed (-), and changed (~) keys.
    /// Results are sorted alphabetically by key name.
    Diff {
        /// Path to the first .env file
        #[arg(value_name = "FILE1")]
        file1: PathBuf,

        /// Path to the second .env file
        #[arg(value_name = "FILE2")]
        file2: PathBuf,

        /// Hide values in output (show only key names)
        #[arg(long, default_value_t = false)]
        redact: bool,
    },

    /// Normalize and format a .env file
    ///
    /// Applies consistent formatting: trims whitespace, uppercases keys,
    /// normalizes to KEY=VALUE format, and sorts alphabetically.
    /// Comments are preserved.
    Format {
        /// Path to the .env file to format
        #[arg(value_name = "FILE")]
        file: PathBuf,

        /// Modify the file in place instead of printing to stdout
        #[arg(long, default_value_t = false)]
        in_place: bool,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        Cli::command().debug_assert();
    }
}
