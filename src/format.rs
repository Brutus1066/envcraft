//! Formatting and normalization for .env files.
//!
//! Provides consistent formatting while preserving comments and
//! never modifying values except for whitespace trimming.

use std::fs;
use std::path::Path;

use thiserror::Error;

use crate::parser::{EnvFile, EnvLine, ParseError};

/// Errors that can occur during format operation.
#[derive(Error, Debug)]
pub enum FormatError {
    #[error("failed to parse env file: {0}")]
    ParseError(#[from] ParseError),

    #[error("failed to write file: {0}")]
    IoError(#[from] std::io::Error),
}

/// A formatted key-value entry.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct FormattedEntry {
    /// Uppercase key
    key: String,
    /// Original key for sorting
    original_key: String,
    /// Trimmed value (not modified except whitespace)
    value: String,
    /// Associated comments (lines before this entry)
    preceding_comments: Vec<String>,
}

/// Format an env file and return the formatted content as a string.
pub fn format_env(env: &EnvFile) -> String {
    let mut entries = Vec::new();
    let mut current_comments: Vec<String> = Vec::new();
    let mut header_comments: Vec<String> = Vec::new();
    let mut seen_first_entry = false;

    // First pass: collect entries with their preceding comments
    for line in &env.lines {
        match line {
            EnvLine::Comment(text) => {
                if seen_first_entry {
                    current_comments.push(text.clone());
                } else {
                    header_comments.push(text.clone());
                }
            }
            EnvLine::Blank => {
                // Blank lines in comments section are preserved
                if seen_first_entry {
                    current_comments.push(String::new());
                } else {
                    header_comments.push(String::new());
                }
            }
            EnvLine::KeyValue { key, value } => {
                seen_first_entry = true;
                entries.push(FormattedEntry {
                    key: key.to_uppercase(),
                    original_key: key.clone(),
                    value: value.trim().to_string(),
                    preceding_comments: std::mem::take(&mut current_comments),
                });
            }
        }
    }

    // Sort entries alphabetically by uppercase key
    entries.sort_by(|a, b| a.key.cmp(&b.key));

    // Build output
    let mut output = String::new();

    // Add header comments (before any entries)
    for comment in &header_comments {
        output.push_str(comment);
        output.push('\n');
    }

    // Add sorted entries with their comments
    for (i, entry) in entries.iter().enumerate() {
        // Add preceding comments for this entry
        for comment in &entry.preceding_comments {
            output.push_str(comment);
            output.push('\n');
        }

        // Add the key=value line
        output.push_str(&entry.key);
        output.push('=');
        output.push_str(&entry.value);
        output.push('\n');

        // Add blank line between entries for readability (except after last)
        if i < entries.len() - 1 && entry.preceding_comments.is_empty() {
            // Only add if the next entry doesn't have comments
            if entries[i + 1].preceding_comments.is_empty() {
                // Don't add extra blank lines
            }
        }
    }

    // Handle trailing comments (after all entries)
    for comment in &current_comments {
        output.push_str(comment);
        output.push('\n');
    }

    output
}

/// Run the format command.
pub fn run_format(path: &Path, in_place: bool) -> Result<bool, FormatError> {
    let env = EnvFile::from_path(path)?;
    let formatted = format_env(&env);

    if in_place {
        fs::write(path, &formatted)?;
        println!("Formatted: {}", path.display());
    } else {
        print!("{formatted}");
    }

    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_uppercase_keys() {
        let env = EnvFile::from_str("port=8080\ndebug=true").unwrap();
        let formatted = format_env(&env);

        assert!(formatted.contains("DEBUG=true"));
        assert!(formatted.contains("PORT=8080"));
        assert!(!formatted.contains("port="));
        assert!(!formatted.contains("debug="));
    }

    #[test]
    fn test_format_sorts_alphabetically() {
        let env = EnvFile::from_str("ZEBRA=z\nAPPLE=a\nMIDDLE=m").unwrap();
        let formatted = format_env(&env);

        let apple_pos = formatted.find("APPLE=").unwrap();
        let middle_pos = formatted.find("MIDDLE=").unwrap();
        let zebra_pos = formatted.find("ZEBRA=").unwrap();

        assert!(apple_pos < middle_pos);
        assert!(middle_pos < zebra_pos);
    }

    #[test]
    fn test_format_trims_whitespace() {
        let env = EnvFile::from_str("KEY=  value with spaces  ").unwrap();
        let formatted = format_env(&env);

        assert!(formatted.contains("KEY=value with spaces\n"));
    }

    #[test]
    fn test_format_preserves_comments() {
        let env = EnvFile::from_str("# Header comment\nKEY=value").unwrap();
        let formatted = format_env(&env);

        assert!(formatted.contains("# Header comment"));
    }

    #[test]
    fn test_format_preserves_values() {
        let env = EnvFile::from_str("URL=postgres://user:pass@host/db").unwrap();
        let formatted = format_env(&env);

        assert!(formatted.contains("URL=postgres://user:pass@host/db"));
    }

    #[test]
    fn test_format_empty_value() {
        let env = EnvFile::from_str("EMPTY=").unwrap();
        let formatted = format_env(&env);

        assert!(formatted.contains("EMPTY=\n"));
    }

    #[test]
    fn test_format_complex() {
        let content = r#"# Database configuration
database_url=postgres://localhost/db

# Server settings
Port=8080
DEBUG = true
"#;
        let env = EnvFile::from_str(content).unwrap();
        let formatted = format_env(&env);

        // Keys should be uppercase and sorted
        assert!(formatted.contains("DATABASE_URL="));
        assert!(formatted.contains("DEBUG=true"));
        assert!(formatted.contains("PORT=8080"));

        // Comments should be preserved
        assert!(formatted.contains("# Database configuration"));
    }

    #[test]
    fn test_format_mixed_case_key() {
        let env = EnvFile::from_str("MyKey=value\nmyOtherKey=value2").unwrap();
        let formatted = format_env(&env);

        assert!(formatted.contains("MYKEY=value"));
        assert!(formatted.contains("MYOTHERKEY=value2"));
    }
}
