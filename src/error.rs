//! Unified error handling for envcraft.
//!
//! Provides a common error type that wraps all module-specific errors.

use thiserror::Error;

use crate::diff::DiffError;
use crate::format::FormatError;
use crate::parser::ParseError;
use crate::schema::SchemaError;

/// Top-level error type for envcraft operations.
#[derive(Error, Debug)]
pub enum EnvcraftError {
    #[error("{0}")]
    Schema(#[from] SchemaError),

    #[error("{0}")]
    Diff(#[from] DiffError),

    #[error("{0}")]
    Format(#[from] FormatError),

    #[error("{0}")]
    Parse(#[from] ParseError),
}
