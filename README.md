# MyBunka

[![Latest Release](https://img.shields.io/github/v/release/tkshnkgwr/MyBunka)](https://github.com/tkshnkgwr/MyBunka/releases)
[![CI](https://github.com/tkshnkgwr/MyBunka/actions/workflows/ci.yml/badge.svg)](https://github.com/tkshnkgwr/MyBunka/actions/workflows/ci.yml)
[![License: GPL-3.0-or-later](https://img.shields.io/badge/License-GPL--3.0--or--later-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![Rust](https://img.shields.io/badge/Rust-2024-orange?logo=rust)](https://www.rust-lang.org/)
[![Platform](https://img.shields.io/badge/Platform-Windows%20%7C%20macOS%20%7C%20Linux-blue)](https://github.com/tkshnkgwr/MyBunka)

**English** | [日本語版](README_JA.md)

`MyBunka` is an ultra-lightweight Rust command-line utility that approximates any decimal or percentage input (e.g. `10%`) into its optimal rational fraction (numerator/denominator) using the **Continued Fraction Expansion** algorithm.

## ✨ Features

- **High Precision**: Computes the mathematically most optimal reduced fraction approximations via continued fraction expansion.
- **Percentage Support**: Accepts standard decimals (e.g., `0.142857`) as well as percentages (e.g., `10%`, `"  -5.5 % "`).
- **Ultra-lightweight & Fast**: Zero external dependencies (Rust standard library only), ~135 KB release binary size, and less than 2 MB memory footprint.
- **Configurable Parameters**: Customize maximum denominator (`--max-den`) and error tolerance (`--tolerance`).

## 🚀 Getting Started

### Prerequisites

- [Rust Toolchain](https://rustup.rs/) (edition 2024 supported)

### Build & Installation

Clone this repository and build the release binary:

```bash
cargo build --release
```

The optimized binary will be generated at `target/release/MyBunka.exe` (or `target/release/MyBunka` on Linux/macOS).

## 💻 Usage

Run the program by passing a decimal number or percentage string as a command-line argument:

```bash
MyBunka <decimal_or_percentage> [options]
```

### Options

- `-d, --max-den <value>`: Maximum denominator allowed for approximation (default: `100,000`)
- `-t, --tolerance <value>`: Error tolerance threshold for approximation (default: `1e-6`)
- `-h, --help`: Prints help information
- `-v, -V, --version`: Prints version, copyright, and license information

### Examples

```bash
# Basic rational approximation
$ MyBunka 0.142857
1/7

# Constrain maximum denominator to 100 (Archimedes approximation for pi)
$ MyBunka 3.14159265 -d 100
22/7

# Tighten tolerance and expand denominator for exact fractional representation
$ MyBunka 0.142857 -t 1e-10 -d 10000000
142857/1000000

# Percentage input (automatically divided by 100)
$ MyBunka 10%
1/10

# Percentage input with whitespace and negative value
$ MyBunka "  -5.5 % "
-11/200

# Version display
$ MyBunka -v
MyBunka v1.1.0
Copyright (c) 2026 tkshnkgwr
License: GPL-3.0-or-later
```

### Errors

If arguments are missing, or invalid numeric formats/unknown options are passed, `MyBunka` writes an error message to `stderr` and exits with code `1`:

```bash
$ MyBunka
使用方法: MyBunka <小数点数> [オプション]
例) MyBunka 0.142857  ->  1/7

$ MyBunka invalid
エラー: 'invalid' は無効な数値またはパーセント表記です
```

## 📚 Documentation

For complete specifications, architectural designs, and guides, refer to `docs/en/`:

- [Technical Specification (docs/en/SPEC.md)](docs/en/SPEC.md) - Algorithm details, arguments, and parsing specifications.
- [System Architecture (docs/en/ARCHITECTURE.md)](docs/en/ARCHITECTURE.md) - System design, technology stack, directory layout, and data flow.
- [System Diagrams (docs/en/DIAGRAM.md)](docs/en/DIAGRAM.md) - Mermaid execution flowcharts.
- [AI Coding Instructions (docs/en/INSTRUCTIONS.md)](docs/en/INSTRUCTIONS.md) - Coding standards, error handling, and documentation synchronization rules.
- [Task & Roadmap Management (docs/en/TODO.md)](docs/en/TODO.md) - Done items, active tasks, and backlog proposals.
- [Performance & Footprints (docs/en/FOOTPRINTS.md)](docs/en/FOOTPRINTS.md) - Empirical binary size, memory usage, and execution timing metrics.
- [Testing Policy & Guide (docs/en/TESTING.md)](docs/en/TESTING.md) - Automated testing policies and local verification steps.
- [Test Verification Report (docs/en/TEST_REPORT.md)](docs/en/TEST_REPORT.md) - Test cases, empirical results, and CI setup.
- [Release Procedures (docs/en/RELEASE.md)](docs/en/RELEASE.md) - Versioning and automated release procedures.
- [Contribution Guidelines (docs/en/CONTRIBUTING.md)](docs/en/CONTRIBUTING.md) - PR rules and Conventional Commits format.
- [Security Policy (docs/en/SECURITY.md)](docs/en/SECURITY.md) - Vulnerability disclosure and support matrix.
- [Project Template Guide (docs/en/PROJECT_TEMPLATE_GUIDE.md)](docs/en/PROJECT_TEMPLATE_GUIDE.md) - Standard configurations for editors, CI/CD, and Dependabot.
- [Changelog (docs/en/CHANGELOG.md)](docs/en/CHANGELOG.md) - Release history.

## 🛠️ Development & Automation

- **Editor Configuration**: Consistent styling via [.editorconfig](.editorconfig).
- **CI (Continuous Integration)**: Automated builds and testing via [.github/workflows/ci.yml](.github/workflows/ci.yml).
- **CD (Automated Release)**: Automatically detects `Cargo.toml` version increments on `main` push, creates Git tags, builds release binaries, and uploads zip archives to GitHub Releases ([.github/workflows/release.yml](.github/workflows/release.yml)).
- **Dependabot**: Weekly automated PR checks for actions and dependencies ([.github/dependabot.yml](.github/dependabot.yml)).

## 📄 License

This project is licensed under the GNU General Public License v3.0 or later (GPL-3.0-or-later) - Copyright (c) 2026 tkshnkgwr. See the [LICENSE](LICENSE) file for details.
