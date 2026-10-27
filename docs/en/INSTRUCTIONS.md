**English** | [日本語版](../ja/INSTRUCTIONS.md)

# AI Coding Instructions (INSTRUCTIONS.md)

This document outlines the coding standards, design policies, quality verification processes, and documentation synchronization rules that AI agents (Daikenja) and developers must follow when maintaining `MyBunka`.

---

## 1. Naming Conventions

Follow standard Rust API Guidelines strictly:

- **Variables, Functions, Modules, Macros**: `snake_case` (e.g., `approximate_fraction`, `parse_decimal_or_percent`)
- **Structs, Traits, Enums, Type Aliases**: `UpperCamelCase` (e.g., `CliOptions`)
- **Constants, Statics**: `SCREAMING_SNAKE_CASE`
- **File Names**:
  - Source files (`.rs`): `snake_case` (e.g., `cli.rs`)
  - Documentation (`.md`): `UPPER_SNAKE_CASE` (e.g., `ARCHITECTURE.md`, `INSTRUCTIONS.md`)
    - *Exceptions: `README.md`, `README_JA.md`, `LICENSE`, `CHANGELOG.md`*

---

## 2. Error Handling Policy

- **Library Layer (`lib.rs`)**: Avoid `panic!`, `unwrap()`, or `expect()`. Return `Result<T, E>` or `Option<T>` for fallible operations.
- **CLI Layer (`cli.rs`)**: Output clear, actionable error messages to standard error (`eprintln!`), and terminate immediately with `std::process::exit(1)`.

---

## 3. Component & Module Architecture

- **Separation of Pure Logic and I/O**: Keep platform-independent mathematical algorithms and string parsing pure in `src/lib.rs`.
- **UI & Presentation Separation**: Encapsulate CLI argument parsing and terminal presentation logic in `src/cli.rs`.
- **Source Code Line Limit (1,000 Lines)**: Proactively propose modularization and submodule decomposition when any single source file exceeds 1,000 lines.

---

## 4. Documentation Synchronization Rules

When modifying code, refactoring, adding features, or updating dependencies, synchronize both `docs/en/` (English) and `docs/ja/` (Japanese) documentation folders.
*(Note: Automated verification and documentation updates can be skipped when modifying `.md` files only).*

| Target Document                  | Role & Scope                      | Synchronization & Formatting Rules                                                                      |
| :------------------------------- | :-------------------------------- | :------------------------------------------------------------------------------------------------------ |
| **`CHANGELOG.md`**               | Release & change history          | Append per completion under `## [YYYY-MM-DD]` in reverse chronological order. Categorize by type.       |
| **`SPEC.md`**                    | Specifications & tech stack       | Update when CLI arguments, continued fraction logic, precision, or OS requirements change.             |
| **`DIAGRAM.md`**                 | Architecture & process flows      | Update Mermaid diagrams whenever execution workflows change.                                           |
| **`README.md` / `README_JA.md`** | Project overview & quickstart     | Update root README files when options, build steps, or prerequisites change.                            |
| **`FOOTPRINTS.md`**              | Resource & binary size metrics    | Append when release binary sizes change or new measurements are taken.                                  |
| **`ARCHITECTURE.md`**            | System design & directory intent  | Update on structural overhaul, module decomposition, or algorithm modifications.                        |
| **`INSTRUCTIONS.md`**            | AI & developer coding guidelines  | Maintain AI guidelines, coding standards, and architectural rules.                                     |
| **`TODO.md`**                    | Task & roadmap tracking           | Keep Done, In Progress, and Backlog items in sync.                                                      |
| **`TESTING.md`**                 | Testing policies & guidelines     | Update when testing methodologies, commands, or suites are introduced.                                  |
| **`RELEASE.md`**                 | Release procedures                | Update when release workflows, build pipelines, or tagging processes change.                            |
| **`CONTRIBUTING.md`**            | Contribution guidelines           | Update commit conventions, PR workflows, or local setup steps.                                          |
| **`SECURITY.md`**                | Security policy                   | Update supported versions, reporting procedures, or security considerations.                            |
| **`TEST_REPORT.md`**             | Test execution & empirical report | Update empirical results upon new features or major updates.                                            |
| **`PROJECT_TEMPLATE_GUIDE.md`**  | Template configuration guide      | Update project template configs or CI/CD patterns.                                                      |
| **Rustdoc Comments**             | In-code API specifications        | Keep `///` and `//!` comments synchronized with code changes.                                           |

---

## 5. Quality Assurance & Local Pre-verification

- **Unit Test Coverage**: Add or extend unit tests when modifying core logic or adding features.
- **Pre-verification Commands** (Skipped for Markdown-only edits):

  ```powershell
  cargo test
  cargo clippy --all-targets -- -D warnings
  cargo fmt --check
  cargo doc --no-deps --document-private-items
  ```
