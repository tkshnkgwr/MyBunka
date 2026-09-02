# MyBunka

[![Latest Release](https://img.shields.io/github/v/release/tkshnkgwr/MyBunka)](https://github.com/tkshnkgwr/MyBunka/releases)
[![CI](https://github.com/tkshnkgwr/MyBunka/actions/workflows/ci.yml/badge.svg)](https://github.com/tkshnkgwr/MyBunka/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/license-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Rust-2024-orange?logo=rust)](https://www.rust-lang.org/)
[![Platform](https://img.shields.io/badge/Platform-Windows%20%7C%20macOS%20%7C%20Linux-blue)](https://github.com/tkshnkgwr/MyBunka)

**English** | [日本語版](README_JA.md)

`MyBunka` is a lightweight Rust command-line utility that approximates any decimal or percentage input (e.g. `10%`) into its fractional representation (numerator/denominator) using the **Continued Fraction Expansion** algorithm.

## Features

- **High Precision**: Custom tolerance and maximum denominator limits allow for accurate fractional approximations.
- **Ultra-lightweight**: Highly optimized release binary size (~138.5 KB) with zero external dependencies.
- **Mathematical Integrity**: Utilizes the continued fraction algorithm to find the best rational approximation.

## Getting Started

### Prerequisites

- [Rust toolchain](https://rustup.rs/) (edition 2024 supported)

### Installation & Build

Clone this repository and build the release binary:

```bash
cargo build --release
```

The optimized binary will be generated at `target/release/MyBunka.exe` (or `target/release/MyBunka` on Linux/macOS).

## Usage

Run the program by passing a decimal number as a command-line argument:

```bash
MyBunka <decimal_number> [options]
```

### Options

- `-d, --max-den <value>`: Maximum denominator allowed for approximation (default: 100,000)
- `-t, --tolerance <value>`: Tolerance criteria for the calculation (default: 1e-6)
- `-h, --help`: Prints help information
- `-v, -V, --version`: Prints version information

### Examples

```bash
$ MyBunka 0.142857
1/7

# Set maximum denominator to 100
$ MyBunka 3.14159265 -d 100
22/7

# Increase precision criteria
$ MyBunka 0.142857 -t 1e-10 -d 10000000
142857/1000000

# Parse percentage input (automatically divided by 100)
$ MyBunka 10%
1/10

# Parse negative percentage input with spaces
$ MyBunka "  -5.5 % "
-11/200
```

### Errors

If the argument is missing or not a valid floating-point number, `MyBunka` will output an error message to `stderr` and exit with code `1`:

```bash
$ MyBunka
使用方法: MyBunka <小数点数> [オプション]
例) MyBunka 0.142857  ->  1/7

$ MyBunka invalid
エラー: 'invalid' は無効な数値またはパーセント表記です
```

## 📚 Documentation

For more detailed documentation, please refer to the files in `docs/en/`:

- [Specification](docs/en/SPEC.md) - Algorithm details and CLI argument specification.
- [System Architecture](docs/en/ARCHITECTURE.md) - System design, technology stack, directory structure, and data flow.
- [System Diagrams](docs/en/DIAGRAM.md) - Flowcharts and architecture diagrams.
- [Development Instructions](docs/en/INSTRUCTIONS.md) - Coding style, naming conventions, error handling policies, and AI guidelines.
- [Task Management & Roadmap](docs/en/TODO.md) - Done tasks, current tasks, and backlog.
- [Performance & Footprints](docs/en/FOOTPRINTS.md) - Binary size and memory usage statistics.
- [Testing Policy & Guide](docs/en/TESTING.md) - Automated test policies and verification procedures.
- [Test Report](docs/en/TEST_REPORT.md) - Test cases and verification results.
- [Release Procedures](docs/en/RELEASE.md) - Versioning and release guidelines.
- [Contribution Guidelines](docs/en/CONTRIBUTING.md) - PR rules and commit conventions.
- [Security Policy](docs/en/SECURITY.md) - Vulnerability reporting and support policies.
- [Project Setup Template Guide](docs/en/PROJECT_TEMPLATE_GUIDE.md) - Standard configurations for editors, CI/CD, and Dependabot.
- [Changelog](docs/en/CHANGELOG.md) - Detailed release history.

## Development

This repository includes unified editor configurations and automated workflows:

- **Editor Configurations**: [.editorconfig](.editorconfig) and [.vscode/settings.json](.vscode/settings.json) are provided to ensure consistent code styling.
- **CI/CD**: Automatic testing is run on PRs/pushes via [.github/workflows/ci.yml](.github/workflows/ci.yml). Automatic release binaries (CLI version in a zip file) are created and uploaded to GitHub Releases when pushing a tag (`v*`) via [.github/workflows/release.yml](.github/workflows/release.yml).
- **Dependabot**: Automatically checks for dependency updates weekly ([.github/dependabot.yml](.github/dependabot.yml)).

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

