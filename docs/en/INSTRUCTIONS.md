**English** | [日本語版](../ja/INSTRUCTIONS.md)

# AI Coding Instructions (INSTRUCTIONS.md)

This document outlines coding conventions, design policies, and response formats that AI agents (Daikenja) and human developers must follow when modifying code in `MyBunka`.

---

## 1. Naming Conventions

Follow standard Rust API Guidelines:

- **Variables, Functions, Modules, Macros**: `snake_case` (e.g., `approximate_fraction`, `parse_decimal_or_percent`)
- **Structs, Traits, Enums, Type Aliases**: `UpperCamelCase` (e.g., `CliOptions`)
- **Constants, Statics**: `SCREAMING_SNAKE_CASE`
- **File Names**:
  - Source files (`.rs`): `snake_case` (`cli.rs`)
  - Documentation (`.md`): `UPPER_SNAKE_CASE` (`ARCHITECTURE.md`, `INSTRUCTIONS.md`)
    - *Exceptions: `README.md`, `LICENSE`, `CHANGELOG.md`*

---

## 2. Error Handling Policy

- **Library Core (`lib.rs`)**: Avoid `panic!`, `unwrap()`, or `expect()`. Return `Result<T, E>` or `Option<T>` for fallible operations.
- **CLI (`cli.rs`)**: Print clean error messages to `eprintln!` and exit with code `1`.

---

## 3. Component & Module Division

- **Pure Logic vs. I/O**: Keep core fraction expansion algorithms in `lib.rs` independent of I/O.
- **Presentation Logic**: Encapsulate CLI presentation logic in `cli.rs`.
- **1,000 Line File Limit**: Propose refactoring/submodule division when a single source file approaches 1,000 lines.
