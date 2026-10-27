**English** | [日本語版](../ja/TESTING.md)

# Testing Policy & Guide (TESTING.md)

This document describes the automated testing policies, test case classification, and local verification procedures for `MyBunka`.

---

## 1. Automated Testing Policy

To guarantee mathematical precision and prevent regressions during refactoring, comprehensive unit tests and doctests are strictly maintained.

### 1.1 Core Logic Unit Tests (`src/lib.rs`)

- Maintained in the `tests` module in `src/lib.rs`.
- Key verification targets:
  - Typical rational and irrational approximations (e.g., `0.142857 -> 1/7`, `0.5 -> 1/2`, `3.14159265 -> 355/113`).
  - Percentage notation parsing (e.g., `10% -> 0.1`, `  -5.5 %  -> -0.055`).
  - Boundary cases: zero, negative values, and maximum denominator constraints.
  - Rustdoc doctests verifying documented public API examples.

### 1.2 CLI Argument Parsing Unit Tests (`src/cli.rs`)

- Maintained in the `tests` module in `src/cli.rs`.
- Key verification targets:
  - Basic arguments (decimal numbers and percentage notation).
  - Parameter options (`-d`, `--max-den`, `-t`, `--tolerance`) and default values.
  - Help (`-h`, `--help`) and version (`-v`, `-V`, `--version`) flag evaluation.
  - Error handling for invalid inputs, missing option arguments, and unknown flags.

---

## 2. Running Local Verification

Execute the following commands in PowerShell 7 (pwsh) to verify all tests and quality checks:

```powershell
# Run unit tests and doctests
cargo test

# Run Clippy static analysis (deny all warnings)
cargo clippy --all-targets -- -D warnings

# Check code formatting
cargo fmt --check

# Check Rustdoc documentation build
cargo doc --no-deps --document-private-items
```

*(Local verification commands can be skipped when changes are restricted strictly to Markdown files).*
