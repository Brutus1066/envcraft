//! Parser for .env files.
//!
//! Provides deterministic parsing of environment files with support for
//! comments and standard KEY=VALUE format.

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use thiserror::Error;

/// Errors that can occur during .env file parsing.
#[derive(Error, Debug)]
pub enum ParseError {
    #[error("failed to read file: {0}")]
    IoError(#[from] std::io::Error),

    #[error("invalid line format at line {line}: {content}")]
    InvalidLine { line: usize, content: String },
}

/// Represents a parsed line from a .env file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EnvLine {
    /// A comment line (starts with #)
    Comment(String),
    /// A blank/empty line
    Blank,
    /// A key-value pair
    KeyValue { key: String, value: String },
}

/// Represents a fully parsed .env file.
#[derive(Debug, Clone)]
pub struct EnvFile {
    /// All lines in order, preserving structure
    pub lines: Vec<EnvLine>,
    /// Key-value pairs for quick lookup (keys are stored as-is)
    pub entries: BTreeMap<String, String>,
}

impl EnvFile {
    /// Parse a .env file from a path.
    pub fn from_path(path: &Path) -> Result<Self, ParseError> {
        let content = fs::read_to_string(path)?;
        Self::from_str(&content)
    }

    /// Parse a .env file from a string.
    pub fn from_str(content: &str) -> Result<Self, ParseError> {
        let mut lines = Vec::new();
        let mut entries = BTreeMap::new();

        for (line_num, line) in content.lines().enumerate() {
            let parsed = parse_line(line, line_num + 1)?;

            if let EnvLine::KeyValue { ref key, ref value } = parsed {
                entries.insert(key.clone(), value.clone());
            }

            lines.push(parsed);
        }

        Ok(Self { lines, entries })
    }

    /// Get the value for a key, if it exists.
    pub fn get(&self, key: &str) -> Option<&String> {
        self.entries.get(key)
    }

    /// Check if a key exists.
    #[allow(dead_code)]
    pub fn contains_key(&self, key: &str) -> bool {
        self.entries.contains_key(key)
    }

    /// Get all keys in sorted order.
    pub fn keys(&self) -> impl Iterator<Item = &String> {
        self.entries.keys()
    }
}

/// Parse a single line from a .env file.
fn parse_line(line: &str, line_num: usize) -> Result<EnvLine, ParseError> {
    let trimmed = line.trim();

    // Empty line
    if trimmed.is_empty() {
        return Ok(EnvLine::Blank);
    }

    // Comment line
    if trimmed.starts_with('#') {
        return Ok(EnvLine::Comment(line.to_string()));
    }

    // Key-value line
    if let Some(eq_pos) = line.find('=') {
        let key = line[..eq_pos].trim().to_string();
        let value = line[eq_pos + 1..].trim().to_string();

        // Validate key is not empty
        if key.is_empty() {
            return Err(ParseError::InvalidLine {
                line: line_num,
                content: line.to_string(),
            });
        }

        // Remove surrounding quotes from value if present
        let value = strip_quotes(&value);

        return Ok(EnvLine::KeyValue { key, value });
    }

    // Invalid line (no = sign and not a comment or blank)
    Err(ParseError::InvalidLine {
        line: line_num,
        content: line.to_string(),
    })
}

/// Remove surrounding quotes from a value if they match.
fn strip_quotes(value: &str) -> String {
    let trimmed = value.trim();

    if trimmed.len() >= 2 {
        let first = trimmed.chars().next();
        let last = trimmed.chars().next_back();

        if (first == Some('"') && last == Some('"'))
            || (first == Some('\'') && last == Some('\''))
        {
            return trimmed[1..trimmed.len() - 1].to_string();
        }
    }

    trimmed.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_env() {
        let content = r#"
# Database config
DATABASE_URL=postgres://localhost/db
PORT=8080

# Debug mode
DEBUG=true
"#;
        let env = EnvFile::from_str(content).unwrap();

        assert_eq!(env.get("DATABASE_URL"), Some(&"postgres://localhost/db".to_string()));
        assert_eq!(env.get("PORT"), Some(&"8080".to_string()));
        assert_eq!(env.get("DEBUG"), Some(&"true".to_string()));
    }

    #[test]
    fn test_parse_quoted_values() {
        let content = r#"
SINGLE='single quoted'
DOUBLE="double quoted"
NONE=no quotes
"#;
        let env = EnvFile::from_str(content).unwrap();

        assert_eq!(env.get("SINGLE"), Some(&"single quoted".to_string()));
        assert_eq!(env.get("DOUBLE"), Some(&"double quoted".to_string()));
        assert_eq!(env.get("NONE"), Some(&"no quotes".to_string()));
    }

    #[test]
    fn test_parse_whitespace_handling() {
        let content = r#"
  KEY1  =  value1  
KEY2=   value2
KEY3=value3   
"#;
        let env = EnvFile::from_str(content).unwrap();

        assert_eq!(env.get("KEY1"), Some(&"value1".to_string()));
        assert_eq!(env.get("KEY2"), Some(&"value2".to_string()));
        assert_eq!(env.get("KEY3"), Some(&"value3".to_string()));
    }

    #[test]
    fn test_parse_empty_value() {
        let content = "EMPTY=\n";
        let env = EnvFile::from_str(content).unwrap();

        assert_eq!(env.get("EMPTY"), Some(&"".to_string()));
    }

    #[test]
    fn test_parse_preserves_line_structure() {
        let content = "# Comment\nKEY=value\n\n# Another";
        let env = EnvFile::from_str(content).unwrap();

        assert_eq!(env.lines.len(), 4);
        assert!(matches!(env.lines[0], EnvLine::Comment(_)));
        assert!(matches!(env.lines[1], EnvLine::KeyValue { .. }));
        assert!(matches!(env.lines[2], EnvLine::Blank));
        assert!(matches!(env.lines[3], EnvLine::Comment(_)));
    }

    #[test]
    fn test_invalid_line() {
        let content = "VALID=ok\nINVALID_NO_EQUALS\n";
        let result = EnvFile::from_str(content);

        assert!(result.is_err());
        if let Err(ParseError::InvalidLine { line, .. }) = result {
            assert_eq!(line, 2);
        } else {
            panic!("Expected InvalidLine error");
        }
    }

    #[test]
    fn test_empty_key_rejected() {
        let content = "=value\n";
        let result = EnvFile::from_str(content);

        assert!(result.is_err());
    }
}
