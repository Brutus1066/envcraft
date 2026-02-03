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
