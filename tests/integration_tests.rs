//! Integration tests for envcraft.
//!
//! These tests verify the complete workflow of each command.

use std::fs;
use std::process::Command;

use tempfile::TempDir;

/// Helper to create a temp directory with files.
fn setup_test_files(files: &[(&str, &str)]) -> TempDir {
    let dir = TempDir::new().expect("Failed to create temp dir");
    for (name, content) in files {
        let path = dir.path().join(name);
        fs::write(&path, content).expect("Failed to write test file");
    }
    dir
}

/// Get the path to the envcraft binary.
fn envcraft_bin() -> std::path::PathBuf {
    let mut path = std::env::current_exe().expect("Failed to get current exe");
    path.pop(); // Remove test binary name
    path.pop(); // Remove deps
    path.push("envcraft");
    
    // On Windows, add .exe extension
    #[cfg(windows)]
    {
        path.set_extension("exe");
    }
    
    path
}

#[test]
fn test_check_valid_env() {
    let dir = setup_test_files(&[
        ("schema.yml", "PORT: int\nDEBUG: bool"),
        (".env", "PORT=8080\nDEBUG=true"),
    ]);

    let output = Command::new(envcraft_bin())
        .args(["check", "schema.yml", ".env"])
        .current_dir(dir.path())
        .output()
        .expect("Failed to run envcraft");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("validation passed"));
    assert!(output.status.success());
}

#[test]
fn test_check_missing_key() {
    let dir = setup_test_files(&[
        ("schema.yml", "PORT: int\nDEBUG: bool\nMISSING: string"),
        (".env", "PORT=8080\nDEBUG=true"),
    ]);

    let output = Command::new(envcraft_bin())
        .args(["check", "schema.yml", ".env"])
        .current_dir(dir.path())
        .output()
        .expect("Failed to run envcraft");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("missing required key: MISSING"));
    assert!(stdout.contains("validation failed"));
}

#[test]
fn test_check_type_error() {
    let dir = setup_test_files(&[
        ("schema.yml", "PORT: int"),
        (".env", "PORT=not_a_number"),
    ]);

    let output = Command::new(envcraft_bin())
        .args(["check", "schema.yml", ".env"])
        .current_dir(dir.path())
        .output()
        .expect("Failed to run envcraft");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("invalid value"));
    assert!(stdout.contains("validation failed"));
}

#[test]
fn test_check_extra_key_warning() {
    let dir = setup_test_files(&[
        ("schema.yml", "PORT: int"),
        (".env", "PORT=8080\nEXTRA=value"),
    ]);

    let output = Command::new(envcraft_bin())
        .args(["check", "schema.yml", ".env"])
        .current_dir(dir.path())
        .output()
        .expect("Failed to run envcraft");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("extra key not in schema: EXTRA"));
    assert!(stdout.contains("validation passed")); // Extra keys are warnings, not errors
}

#[test]
fn test_diff_identical_files() {
    let dir = setup_test_files(&[
        ("a.env", "KEY=value"),
        ("b.env", "KEY=value"),
    ]);

    let output = Command::new(envcraft_bin())
        .args(["diff", "a.env", "b.env"])
        .current_dir(dir.path())
        .output()
        .expect("Failed to run envcraft");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("identical"));
}

#[test]
fn test_diff_added_removed_changed() {
    let dir = setup_test_files(&[
        ("a.env", "REMOVED=old\nCHANGED=before"),
        ("b.env", "ADDED=new\nCHANGED=after"),
    ]);

    let output = Command::new(envcraft_bin())
        .args(["diff", "a.env", "b.env"])
        .current_dir(dir.path())
        .output()
        .expect("Failed to run envcraft");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("+ ADDED=new"));
    assert!(stdout.contains("- REMOVED=old"));
    assert!(stdout.contains("~ CHANGED: before → after"));
}

#[test]
fn test_diff_redact() {
    let dir = setup_test_files(&[
        ("a.env", "SECRET=old_secret"),
        ("b.env", "SECRET=new_secret"),
    ]);

    let output = Command::new(envcraft_bin())
        .args(["diff", "a.env", "b.env", "--redact"])
        .current_dir(dir.path())
        .output()
        .expect("Failed to run envcraft");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("~ SECRET"));
    assert!(!stdout.contains("old_secret"));
    assert!(!stdout.contains("new_secret"));
}

#[test]
fn test_format_stdout() {
    let dir = setup_test_files(&[
        (".env", "  zebra = z  \napple=a"),
    ]);

    let output = Command::new(envcraft_bin())
        .args(["format", ".env"])
        .current_dir(dir.path())
        .output()
        .expect("Failed to run envcraft");

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Keys should be uppercase and sorted
    let apple_pos = stdout.find("APPLE=").expect("APPLE not found");
    let zebra_pos = stdout.find("ZEBRA=").expect("ZEBRA not found");
    assert!(apple_pos < zebra_pos, "Keys should be sorted alphabetically");
    
    // Values should be trimmed
    assert!(stdout.contains("APPLE=a"));
    assert!(stdout.contains("ZEBRA=z"));
}

#[test]
fn test_format_in_place() {
    let dir = setup_test_files(&[
        (".env", "  lower_key = value  "),
    ]);

    let env_path = dir.path().join(".env");

    Command::new(envcraft_bin())
        .args(["format", ".env", "--in-place"])
        .current_dir(dir.path())
        .output()
        .expect("Failed to run envcraft");

    let content = fs::read_to_string(&env_path).expect("Failed to read file");
    assert!(content.contains("LOWER_KEY=value"));
}

#[test]
fn test_format_preserves_comments() {
    let dir = setup_test_files(&[
        (".env", "# Important comment\nKEY=value"),
    ]);

    let output = Command::new(envcraft_bin())
        .args(["format", ".env"])
        .current_dir(dir.path())
        .output()
        .expect("Failed to run envcraft");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("# Important comment"));
}

#[test]
fn test_version_flag() {
    let output = Command::new(envcraft_bin())
        .args(["--version"])
        .output()
        .expect("Failed to run envcraft");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("envcraft"));
    assert!(stdout.contains("0.1.0"));
}

#[test]
fn test_help_flag() {
    let output = Command::new(envcraft_bin())
        .args(["--help"])
        .output()
        .expect("Failed to run envcraft");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("check"));
    assert!(stdout.contains("diff"));
    assert!(stdout.contains("format"));
}
