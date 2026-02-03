//! Semantic diff between two .env files.
//!
//! Provides deterministic comparison showing added, removed, and changed keys.

use std::collections::BTreeSet;
use std::path::Path;

use thiserror::Error;

use crate::parser::{EnvFile, ParseError};

/// Errors that can occur during diff operation.
#[derive(Error, Debug)]
pub enum DiffError {
    #[error("failed to parse env file: {0}")]
    ParseError(#[from] ParseError),
}

/// A single difference entry.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum DiffEntry {
    /// Key only exists in the second file
    Added { key: String, value: String },
    /// Key only exists in the first file
    Removed { key: String, value: String },
    /// Key exists in both but values differ
    Changed {
        key: String,
        old_value: String,
        new_value: String,
    },
}

impl DiffEntry {
    /// Get the key for this entry.
    pub fn key(&self) -> &str {
        match self {
            DiffEntry::Added { key, .. } => key,
            DiffEntry::Removed { key, .. } => key,
            DiffEntry::Changed { key, .. } => key,
        }
    }

    /// Format this entry for display.
    pub fn format(&self, redact: bool) -> String {
        match self {
            DiffEntry::Added { key, value } => {
                if redact {
                    format!("+ {key}")
                } else {
                    format!("+ {key}={value}")
                }
            }
            DiffEntry::Removed { key, value } => {
                if redact {
                    format!("- {key}")
                } else {
                    format!("- {key}={value}")
                }
            }
            DiffEntry::Changed {
                key,
                old_value,
                new_value,
            } => {
                if redact {
                    format!("~ {key}")
                } else {
                    format!("~ {key}: {old_value} → {new_value}")
                }
            }
        }
    }
}

/// Result of comparing two env files.
#[derive(Debug)]
pub struct DiffResult {
    /// All differences, sorted alphabetically by key
    pub entries: Vec<DiffEntry>,
}

impl DiffResult {
    /// Check if the files are identical.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Get the number of differences.
    pub fn len(&self) -> usize {
        self.entries.len()
    }
}

/// Compare two env files and return their differences.
pub fn diff(file1: &EnvFile, file2: &EnvFile) -> DiffResult {
    let mut entries = Vec::new();

    // Collect all keys from both files
    let keys1: BTreeSet<&String> = file1.keys().collect();
    let keys2: BTreeSet<&String> = file2.keys().collect();

    // Find removed keys (in file1 but not file2)
    for key in keys1.difference(&keys2) {
        let value = file1.get(key).unwrap().clone();
        entries.push(DiffEntry::Removed {
            key: (*key).clone(),
            value,
        });
    }

    // Find added keys (in file2 but not file1)
    for key in keys2.difference(&keys1) {
        let value = file2.get(key).unwrap().clone();
        entries.push(DiffEntry::Added {
            key: (*key).clone(),
            value,
        });
    }

    // Find changed keys (in both but different values)
    for key in keys1.intersection(&keys2) {
        let value1 = file1.get(key).unwrap();
        let value2 = file2.get(key).unwrap();

        if value1 != value2 {
            entries.push(DiffEntry::Changed {
                key: (*key).clone(),
                old_value: value1.clone(),
                new_value: value2.clone(),
            });
        }
    }

    // Sort by key for deterministic output
    entries.sort_by(|a, b| a.key().cmp(b.key()));

    DiffResult { entries }
}

/// Run the diff command.
pub fn run_diff(path1: &Path, path2: &Path, redact: bool) -> Result<bool, DiffError> {
    let file1 = EnvFile::from_path(path1)?;
    let file2 = EnvFile::from_path(path2)?;
    let result = diff(&file1, &file2);

    if result.is_empty() {
        println!("Files are identical");
        return Ok(true);
    }

    for entry in &result.entries {
        println!("{}", entry.format(redact));
    }

    println!();
    println!("{} difference(s) found", result.len());

    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diff_identical() {
        let env1 = EnvFile::from_str("A=1\nB=2").unwrap();
        let env2 = EnvFile::from_str("A=1\nB=2").unwrap();
        let result = diff(&env1, &env2);

        assert!(result.is_empty());
    }

    #[test]
    fn test_diff_added() {
        let env1 = EnvFile::from_str("A=1").unwrap();
        let env2 = EnvFile::from_str("A=1\nB=2").unwrap();
        let result = diff(&env1, &env2);

        assert_eq!(result.len(), 1);
        assert!(matches!(
            &result.entries[0],
            DiffEntry::Added { key, value } if key == "B" && value == "2"
        ));
    }

    #[test]
    fn test_diff_removed() {
        let env1 = EnvFile::from_str("A=1\nB=2").unwrap();
        let env2 = EnvFile::from_str("A=1").unwrap();
        let result = diff(&env1, &env2);

        assert_eq!(result.len(), 1);
        assert!(matches!(
            &result.entries[0],
            DiffEntry::Removed { key, value } if key == "B" && value == "2"
        ));
    }

    #[test]
    fn test_diff_changed() {
        let env1 = EnvFile::from_str("A=1").unwrap();
        let env2 = EnvFile::from_str("A=2").unwrap();
        let result = diff(&env1, &env2);

        assert_eq!(result.len(), 1);
        assert!(matches!(
            &result.entries[0],
            DiffEntry::Changed { key, old_value, new_value }
            if key == "A" && old_value == "1" && new_value == "2"
        ));
    }

    #[test]
    fn test_diff_complex() {
        let env1 = EnvFile::from_str("A=1\nB=2\nC=3").unwrap();
        let env2 = EnvFile::from_str("A=1\nB=changed\nD=4").unwrap();
        let result = diff(&env1, &env2);

        // Should have: B changed, C removed, D added
        assert_eq!(result.len(), 3);

        // Sorted alphabetically
        assert!(matches!(&result.entries[0], DiffEntry::Changed { key, .. } if key == "B"));
        assert!(matches!(&result.entries[1], DiffEntry::Removed { key, .. } if key == "C"));
        assert!(matches!(&result.entries[2], DiffEntry::Added { key, .. } if key == "D"));
    }

    #[test]
    fn test_diff_format_normal() {
        let added = DiffEntry::Added {
            key: "KEY".to_string(),
            value: "value".to_string(),
        };
        let removed = DiffEntry::Removed {
            key: "KEY".to_string(),
            value: "value".to_string(),
        };
        let changed = DiffEntry::Changed {
            key: "KEY".to_string(),
            old_value: "old".to_string(),
            new_value: "new".to_string(),
        };

        assert_eq!(added.format(false), "+ KEY=value");
        assert_eq!(removed.format(false), "- KEY=value");
        assert_eq!(changed.format(false), "~ KEY: old → new");
    }

    #[test]
    fn test_diff_format_redacted() {
        let added = DiffEntry::Added {
            key: "KEY".to_string(),
            value: "secret".to_string(),
        };
        let removed = DiffEntry::Removed {
            key: "KEY".to_string(),
            value: "secret".to_string(),
        };
        let changed = DiffEntry::Changed {
            key: "KEY".to_string(),
            old_value: "old_secret".to_string(),
            new_value: "new_secret".to_string(),
        };

        assert_eq!(added.format(true), "+ KEY");
        assert_eq!(removed.format(true), "- KEY");
        assert_eq!(changed.format(true), "~ KEY");
    }
}
