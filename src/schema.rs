//! Schema validation for .env files.
//!
//! Validates environment files against YAML schema definitions.
//! Supports string, int, and bool types.

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use thiserror::Error;

use crate::parser::{EnvFile, ParseError};

/// Errors that can occur during schema validation.
#[derive(Error, Debug)]
pub enum SchemaError {
    #[error("failed to read schema file: {0}")]
    IoError(#[from] std::io::Error),

    #[error("failed to parse schema YAML: {0}")]
    YamlError(#[from] serde_yaml::Error),

    #[error("invalid type '{0}' for key '{1}' (expected: string, int, bool)")]
    InvalidType(String, String),

    #[error("env file error: {0}")]
    EnvParseError(#[from] ParseError),
}

/// Supported value types in schema.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValueType {
    String,
    Int,
    Bool,
}

impl ValueType {
    /// Parse a type string into a ValueType.
    fn from_str(s: &str, key: &str) -> Result<Self, SchemaError> {
        match s.to_lowercase().as_str() {
            "string" => Ok(ValueType::String),
            "int" | "integer" => Ok(ValueType::Int),
            "bool" | "boolean" => Ok(ValueType::Bool),
            _ => Err(SchemaError::InvalidType(s.to_string(), key.to_string())),
        }
    }

    /// Validate a value against this type.
    fn validate(&self, value: &str) -> bool {
        match self {
            ValueType::String => true,
            ValueType::Int => value.parse::<i64>().is_ok(),
            ValueType::Bool => {
                let lower = value.to_lowercase();
                lower == "true" || lower == "false"
            }
        }
    }

    /// Get a human-readable description of valid values.
    fn description(&self) -> &'static str {
        match self {
            ValueType::String => "any string",
            ValueType::Int => "an integer (e.g., 42, -10)",
            ValueType::Bool => "true or false",
        }
    }
}

/// A parsed schema definition.
#[derive(Debug)]
pub struct Schema {
    /// Map of key names to their expected types
    pub fields: BTreeMap<String, ValueType>,
}

impl Schema {
    /// Load a schema from a YAML file.
    pub fn from_path(path: &Path) -> Result<Self, SchemaError> {
        let content = fs::read_to_string(path)?;
        Self::from_str(&content)
    }

    /// Parse a schema from a YAML string.
    pub fn from_str(content: &str) -> Result<Self, SchemaError> {
        let raw: BTreeMap<String, String> = serde_yaml::from_str(content)?;
        let mut fields = BTreeMap::new();

        for (key, type_str) in raw {
            let value_type = ValueType::from_str(&type_str, &key)?;
            fields.insert(key, value_type);
        }

        Ok(Self { fields })
    }
}

/// Result of validating an env file against a schema.
#[derive(Debug)]
pub struct ValidationResult {
    /// Keys that are missing from the env file
    pub missing: Vec<String>,
    /// Keys that are in the env file but not in the schema
    pub extra: Vec<String>,
    /// Keys with type validation errors (key, expected_type, actual_value)
    pub type_errors: Vec<(String, ValueType, String)>,
}

impl ValidationResult {
    /// Check if validation passed (no errors).
    /// Extra keys are warnings, not errors.
    pub fn is_valid(&self) -> bool {
        self.missing.is_empty() && self.type_errors.is_empty()
    }

    /// Check if there are any issues (errors or warnings).
    #[allow(dead_code)]
    pub fn has_issues(&self) -> bool {
        !self.missing.is_empty() || !self.extra.is_empty() || !self.type_errors.is_empty()
    }
}

/// Validate an env file against a schema.
pub fn validate(schema: &Schema, env: &EnvFile) -> ValidationResult {
    let mut missing = Vec::new();
    let mut extra = Vec::new();
    let mut type_errors = Vec::new();

    // Check for missing keys and type errors
    for (key, expected_type) in &schema.fields {
        match env.get(key) {
            Some(value) => {
                if !expected_type.validate(value) {
                    type_errors.push((key.clone(), *expected_type, value.clone()));
                }
            }
            None => {
                missing.push(key.clone());
            }
        }
    }

    // Check for extra keys
    for key in env.keys() {
        if !schema.fields.contains_key(key) {
            extra.push(key.clone());
        }
    }

    // Sort for deterministic output
    missing.sort();
    extra.sort();
    type_errors.sort_by(|a, b| a.0.cmp(&b.0));

    ValidationResult {
        missing,
        extra,
        type_errors,
    }
}

/// Run the check command.
pub fn run_check(schema_path: &Path, env_path: &Path) -> Result<bool, SchemaError> {
    let schema = Schema::from_path(schema_path)?;
    let env = EnvFile::from_path(env_path)?;
    let result = validate(&schema, &env);

    // Print missing keys (errors)
    for key in &result.missing {
        println!("error: missing required key: {key}");
    }

    // Print type errors
    for (key, expected_type, actual_value) in &result.type_errors {
        println!(
            "error: key '{key}' has invalid value '{actual_value}' (expected {})",
            expected_type.description()
        );
    }

    // Print extra keys (warnings)
    for key in &result.extra {
        println!("warning: extra key not in schema: {key}");
    }

    // Summary
    if result.is_valid() {
        if result.extra.is_empty() {
            println!("✓ validation passed");
        } else {
            println!(
                "✓ validation passed with {} warning(s)",
                result.extra.len()
            );
        }
        Ok(true)
    } else {
        let error_count = result.missing.len() + result.type_errors.len();
        println!("✗ validation failed with {error_count} error(s)");
        Ok(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_schema_parsing() {
        let yaml = r#"
PORT: int
DEBUG: bool
DATABASE_URL: string
"#;
        let schema = Schema::from_str(yaml).unwrap();

        assert_eq!(schema.fields.get("PORT"), Some(&ValueType::Int));
        assert_eq!(schema.fields.get("DEBUG"), Some(&ValueType::Bool));
        assert_eq!(schema.fields.get("DATABASE_URL"), Some(&ValueType::String));
    }

    #[test]
    fn test_schema_type_aliases() {
        let yaml = r#"
A: integer
B: boolean
"#;
        let schema = Schema::from_str(yaml).unwrap();

        assert_eq!(schema.fields.get("A"), Some(&ValueType::Int));
        assert_eq!(schema.fields.get("B"), Some(&ValueType::Bool));
    }

    #[test]
    fn test_schema_invalid_type() {
        let yaml = r#"
PORT: number
"#;
        let result = Schema::from_str(yaml);

        assert!(result.is_err());
        if let Err(SchemaError::InvalidType(type_str, key)) = result {
            assert_eq!(type_str, "number");
            assert_eq!(key, "PORT");
        } else {
            panic!("Expected InvalidType error");
        }
    }

    #[test]
    fn test_validation_success() {
        let schema = Schema::from_str("PORT: int\nDEBUG: bool").unwrap();
        let env = EnvFile::from_str("PORT=8080\nDEBUG=true").unwrap();
        let result = validate(&schema, &env);

        assert!(result.is_valid());
        assert!(result.missing.is_empty());
        assert!(result.extra.is_empty());
        assert!(result.type_errors.is_empty());
    }

    #[test]
    fn test_validation_missing_key() {
        let schema = Schema::from_str("PORT: int\nDEBUG: bool").unwrap();
        let env = EnvFile::from_str("PORT=8080").unwrap();
        let result = validate(&schema, &env);

        assert!(!result.is_valid());
        assert_eq!(result.missing, vec!["DEBUG"]);
    }

    #[test]
    fn test_validation_extra_key() {
        let schema = Schema::from_str("PORT: int").unwrap();
        let env = EnvFile::from_str("PORT=8080\nEXTRA=value").unwrap();
        let result = validate(&schema, &env);

        assert!(result.is_valid()); // Extra keys are warnings, not errors
        assert_eq!(result.extra, vec!["EXTRA"]);
    }

    #[test]
    fn test_validation_type_error_int() {
        let schema = Schema::from_str("PORT: int").unwrap();
        let env = EnvFile::from_str("PORT=not_a_number").unwrap();
        let result = validate(&schema, &env);

        assert!(!result.is_valid());
        assert_eq!(result.type_errors.len(), 1);
        assert_eq!(result.type_errors[0].0, "PORT");
    }

    #[test]
    fn test_validation_type_error_bool() {
        let schema = Schema::from_str("DEBUG: bool").unwrap();
        let env = EnvFile::from_str("DEBUG=yes").unwrap();
        let result = validate(&schema, &env);

        assert!(!result.is_valid());
        assert_eq!(result.type_errors.len(), 1);
    }

    #[test]
    fn test_validation_bool_case_insensitive() {
        let schema = Schema::from_str("A: bool\nB: bool\nC: bool\nD: bool").unwrap();
        let env = EnvFile::from_str("A=true\nB=TRUE\nC=False\nD=FALSE").unwrap();
        let result = validate(&schema, &env);

        assert!(result.is_valid());
    }
}
