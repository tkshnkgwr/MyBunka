**English** | [日本語版](../ja/CHANGELOG.md)

# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased] - 2026-07-21

### Changed
- **Separation of shared library dependency via Cargo Features**:
  - Made `common_lib` dependency optional in `Cargo.toml`.
  - Loaded `common_lib` only when `gui` Cargo feature is enabled, achieving an ultra-lightweight CLI build with zero GUI/Windows dependencies by default.

### Added
- **1000 Line Limit Rule & Code Scale Verification**:
  - Added 1,000 line file limit guideline to `.agents/AGENTS.md` and `.agents/INSTRUCTIONS.md`.
- **Expanded RustDoc Documentation**:
  - Added module-level RustDoc comments to `src/lib.rs`, `src/cli.rs`, and `src/gui.rs`.

## [0.4.6] - 2026-07-13

### Added
- **Percentage Notation Support**:
  - Added support for percentage inputs like `bunka 10%`.
  - Percentage inputs work in both CLI and GUI versions.
- **GUI Screenshots in README**:
  - Added screenshots to `README.md` and `README_JA.md`.

## [0.4.5] - 2026-07-08

### Added
- **Automated GitHub Tag & Release Creation**:
  - Integrated release workflow to detect `Cargo.toml` version changes on `main` branch push and trigger GitHub Releases.

## [0.4.4] - 2026-07-03

### Added
- **Integration with `common_lib`**:
  - Delegated Windows desktop Named Mutex single-instance logic to `common_lib`.

## [0.4.3] - 2026-07-01

### Added
- **Status Badges**:
  - Added Rust edition and platform badges to `README.md`.

## [0.4.0] - 2026-06-26

### Added
- **GUI Desktop Overlay**:
  - Introduced desktop GUI app powered by `eframe`/`egui`.
  - Features custom dark theme, semi-transparent background, always-on-top mode, and copy-to-clipboard button.
