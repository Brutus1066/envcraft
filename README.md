# envcraft

```
╔════════════════════════════════════════════════════════════════════════════╗
║                                                                            ║
║   ███████╗███╗   ██╗██╗   ██╗ ██████╗██████╗  █████╗ ███████╗████████╗     ║
║   ██╔════╝████╗  ██║██║   ██║██╔════╝██╔══██╗██╔══██╗██╔════╝╚══██╔══╝     ║
║   █████╗  ██╔██╗ ██║██║   ██║██║     ██████╔╝███████║█████╗     ██║        ║
║   ██╔══╝  ██║╚██╗██║╚██╗ ██╔╝██║     ██╔══██╗██╔══██║██╔══╝     ██║        ║
║   ███████╗██║ ╚████║ ╚████╔╝ ╚██████╗██║  ██║██║  ██║██║        ██║        ║
║   ╚══════╝╚═╝  ╚═══╝  ╚═══╝   ╚═════╝╚═╝  ╚═╝╚═╝  ╚═╝╚═╝        ╚═╝        ║
║                                                                            ║
║                      Precise tools for .env files                          ║
║                                                                            ║
║                      🐸 LazyFrog | kindware.dev                            ║
║                                                                            ║
╚════════════════════════════════════════════════════════════════════════════╝
```

[![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=for-the-badge)](https://opensource.org/licenses/MIT)
[![Tests](https://img.shields.io/badge/Tests-44%20Passing-brightgreen?style=for-the-badge)](https://github.com/Brutus1066/envcraft)
[![Windows](https://img.shields.io/badge/Windows-0078D6?style=for-the-badge&logo=windows&logoColor=white)](https://www.microsoft.com/windows)
[![Linux](https://img.shields.io/badge/Linux-FCC624?style=for-the-badge&logo=linux&logoColor=black)](https://www.linux.org/)
[![macOS](https://img.shields.io/badge/macOS-000000?style=for-the-badge&logo=apple&logoColor=white)](https://www.apple.com/macos)
[![crates.io](https://img.shields.io/badge/crates.io-e6522c?style=for-the-badge&logo=rust&logoColor=white)](https://crates.io/crates/envcraft)

---

## 🎯 What is envcraft?

**envcraft** is a safe, deterministic CLI tool for working with `.env` files. It performs three operations with absolute precision:

| Command | Purpose |
|---------|---------|
| `check` | Validate a `.env` file against a YAML schema |
| `diff` | Show semantic differences between two `.env` files |
| `format` | Normalize and format a `.env` file |

## ✨ Features

- 🔒 **Safe** — Never modifies secret values (only trims whitespace)
- 🎯 **Deterministic** — Same input always produces same output
- 🚫 **No AI** — No heuristics, no guessing, no surprises
- 🌐 **Offline** — No network access, no telemetry
- 🖥️ **Cross-platform** — Works on Windows, Linux, and macOS
- 📦 **Zero unsafe code** — Pure safe Rust

---

## 📦 Installation

### From crates.io

```bash
cargo install envcraft
```

### From source

```bash
git clone https://github.com/Brutus1066/envcraft.git
cd envcraft
cargo build --release
```

The binary will be at `target/release/envcraft`.

---

## 🚀 Usage

### Check: Validate against a schema

```bash
envcraft check schema.yml .env
```

**Schema format (YAML):**

```yaml
PORT: int
DEBUG: bool
DATABASE_URL: string
API_KEY: string
```

**Supported types:**

| Type | Description | Valid examples |
|------|-------------|----------------|
| `string` | Any value | `hello`, `user@example.com` |
| `int` | Integer (i64) | `42`, `-10`, `8080` |
| `bool` | Boolean | `true`, `false`, `TRUE`, `FALSE` |

**Output:**

```
error: missing required key: API_KEY
error: key 'PORT' has invalid value 'abc' (expected an integer)
warning: extra key not in schema: LEGACY_MODE
✗ validation failed with 2 error(s)
```

### Diff: Compare two files

```bash
envcraft diff .env.production .env.staging
```

**Output:**

```
+ NEW_FEATURE=enabled
- DEPRECATED_KEY=old_value
~ DATABASE_URL: prod-db → staging-db
~ PORT: 80 → 8080

4 difference(s) found
```

**Redact sensitive values:**

```bash
envcraft diff .env.production .env.staging --redact
```

**Output:**

```
+ NEW_FEATURE
- DEPRECATED_KEY
~ DATABASE_URL
~ PORT

4 difference(s) found
```

### Format: Normalize a file

```bash
envcraft format .env
```

**What it does:**

- ✅ Trims whitespace from keys and values
- ✅ Converts keys to UPPERCASE
- ✅ Normalizes format to `KEY=VALUE`
- ✅ Sorts keys alphabetically
- ✅ Preserves comments
- ❌ Never modifies actual values (except whitespace trimming)

**Modify in place:**

```bash
envcraft format .env --in-place
```

---

## 📋 Demo

Try the included demo files to see envcraft in action:

### Validate a valid .env file

```bash
$ envcraft check demo/schema.yml demo/valid.env
✓ validation passed
```

### Validate an invalid .env file

```bash
$ envcraft check demo/schema.yml demo/invalid.env
error: missing required key: API_KEY
error: key 'DEBUG' has invalid value 'maybe' (expected true or false)
error: key 'PORT' has invalid value 'not_a_number' (expected an integer (e.g., 42, -10))
✗ validation failed with 3 error(s)
```

### Diff between environments

```bash
$ envcraft diff demo/dev.env demo/prod.env
~ API_KEY: sk_dev_key → sk_live_secret
+ CACHE_ENABLED=true
~ DATABASE_URL: postgres://localhost:5432/dev_db → postgres://prod-server:5432/prod_db
~ DEBUG: true → false
~ HOST: localhost → 0.0.0.0
- LOG_LEVEL=debug
~ PORT: 3000 → 80

7 difference(s) found
```

### Diff with redacted values

```bash
$ envcraft diff demo/dev.env demo/prod.env --redact
~ API_KEY
+ CACHE_ENABLED
~ DATABASE_URL
~ DEBUG
~ HOST
- LOG_LEVEL
~ PORT

7 difference(s) found
```

### Format a messy file

**Before (messy.env):**
```env
# Development environment

port = 3000
host=localhost
debug=true
database_url = postgres://localhost:5432/dev_db
api_key=sk_dev_xyz789
  extra_spaces  =   lots of whitespace   
```

**After:**
```bash
$ envcraft format demo/messy.env
# Development environment

API_KEY=sk_dev_xyz789
DATABASE_URL=postgres://localhost:5432/dev_db
DEBUG=true
EXTRA_SPACES=lots of whitespace
HOST=localhost
PORT=3000
```

---

## 📋 Examples

### Example `.env` file

```env
# Database configuration
DATABASE_URL=postgres://localhost:5432/myapp

# Server settings
PORT=8080
DEBUG=false

# API keys
API_KEY=sk_live_abc123xyz
```

### Example schema

```yaml
# schema.yml
DATABASE_URL: string
PORT: int
DEBUG: bool
API_KEY: string
```

---

## 🔒 Trust & Safety

**envcraft** is designed with security in mind:

| Guarantee | Description |
|-----------|-------------|
| 🚫 No network access | The tool never makes HTTP requests |
| 🚫 No telemetry | No data is collected or transmitted |
| 🚫 No AI/heuristics | Behavior is 100% deterministic |
| 🚫 No secret analysis | Values are treated as opaque strings |
| ✅ Whitespace only | The only modification to values is trimming |
| ✅ Offline operation | Works without internet connection |
| ✅ Open source | Full source code available for audit |

---

## 🛠️ Building

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release

# Run tests
cargo test

# Run with verbose test output
cargo test -- --nocapture
```

---

## 🧪 Tests

envcraft includes comprehensive test coverage:

```
running 32 tests
test cli::tests::verify_cli ... ok
test diff::tests::test_diff_added ... ok
test diff::tests::test_diff_changed ... ok
test diff::tests::test_diff_complex ... ok
test diff::tests::test_diff_format_normal ... ok
test diff::tests::test_diff_format_redacted ... ok
test diff::tests::test_diff_identical ... ok
test diff::tests::test_diff_removed ... ok
test format::tests::test_format_complex ... ok
test format::tests::test_format_empty_value ... ok
test format::tests::test_format_mixed_case_key ... ok
test format::tests::test_format_preserves_comments ... ok
test format::tests::test_format_preserves_values ... ok
test format::tests::test_format_sorts_alphabetically ... ok
test format::tests::test_format_trims_whitespace ... ok
test format::tests::test_format_uppercase_keys ... ok
test parser::tests::test_empty_key_rejected ... ok
test parser::tests::test_invalid_line ... ok
test parser::tests::test_parse_empty_value ... ok
test parser::tests::test_parse_preserves_line_structure ... ok
test parser::tests::test_parse_quoted_values ... ok
test parser::tests::test_parse_simple_env ... ok
test parser::tests::test_parse_whitespace_handling ... ok
test schema::tests::test_schema_invalid_type ... ok
test schema::tests::test_schema_parsing ... ok
test schema::tests::test_schema_type_aliases ... ok
test schema::tests::test_validation_bool_case_insensitive ... ok
test schema::tests::test_validation_extra_key ... ok
test schema::tests::test_validation_missing_key ... ok
test schema::tests::test_validation_success ... ok
test schema::tests::test_validation_type_error_bool ... ok
test schema::tests::test_validation_type_error_int ... ok

test result: ok. 32 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

running 12 tests
test test_check_extra_key_warning ... ok
test test_check_missing_key ... ok
test test_check_type_error ... ok
test test_check_valid_env ... ok
test test_diff_added_removed_changed ... ok
test test_diff_identical_files ... ok
test test_diff_redact ... ok
test test_format_in_place ... ok
test test_format_preserves_comments ... ok
test test_format_stdout ... ok
test test_help_flag ... ok
test test_version_flag ... ok

test result: ok. 12 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

**Total: 44 tests passing ✓**

---

## 📄 License

This project is licensed under the **MIT License** — see the [LICENSE](LICENSE) file for details.

---

## 🐸 About

**envcraft** is developed by **LazyFrog** at [kindware.dev](https://kindware.dev).

- 📧 **Support:** [support@kindware.dev](mailto:support@kindware.dev)
- 🐙 **GitHub:** [github.com/Brutus1066/envcraft](https://github.com/Brutus1066/envcraft)
- 🌐 **Website:** [kindware.dev](https://kindware.dev)

---

<p align="center">
  <strong>🐸 LazyFrog | kindware.dev</strong><br>
  <em>Precise tools for .env files</em>
</p>
