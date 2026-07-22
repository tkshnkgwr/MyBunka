**English** | [日本語版](../ja/ARCHITECTURE.md)

# System Architecture (ARCHITECTURE.md)

This document describes the system architecture, structural design intents, technology stack, and data flow across modules for `bunka`.

---

## 1. System Overview & Objectives

`bunka` (分化) is an application and library designed to approximate floating-point numbers (e.g., `0.142857`) or percentage notations (e.g., `10%`) into optimal reduced fraction representations (e.g., `1/7`, `1/10`).

### Core Objectives
- **High-Precision Rational Approximation**: Rapid and precise conversion from decimal numbers to intuitive fractional expressions.
- **Multi-Interface Support**: 
  - A lightweight **CLI Tool** suitable for scripting and automated pipelines.
  - An interactive **Desktop GUI App** that stays always-on-top with semi-transparent aesthetics.
- **High Efficiency & Resource Optimization**: Fast continued fraction expansion algorithm minimizing heap allocations and memory footprint.

---

## 2. Technology Stack

### 2.1 Language
- **Rust (Edition 2024)**: Offers memory safety, zero-cost abstractions, and high execution speed.

### 2.2 Frameworks & Libraries
- **eframe / egui (v0.35.0)** (when `gui` feature is enabled):
  - Immediate-mode GUI library for Rust.
  - Enables frameless (`decorated: false`), transparent (`transparent: true`), and always-on-top window controls.
- **windows (v0.62.2) & winapi (v0.3.9)** (when `gui` feature is enabled):
  - Provides Win32 API access for Windows-specific single instance prevention via Named Mutex.
- **common_lib** (local crate at `path = "../common_lib"`, optional with `gui` feature):
  - Shared desktop utility library providing single instance mutex controls.

---

## 3. Directory Structure & Architecture Intent

```text
bunka/
├── .agents/
│   ├── AGENTS.md               # AI development guidelines & rules
│   └── INSTRUCTIONS.md         # AI coding style instructions
├── .github/
│   └── workflows/
│       ├── ci.yml              # CI workflow for Windows builds & testing
│       └── release.yml         # CD workflow for release automation
├── docs/
│   ├── en/                     # English documentation
│   │   ├── ARCHITECTURE.md     # This document
│   │   ├── DIAGRAM.md          # Flowcharts & diagrams
│   │   ├── FOOTPRINTS.md       # Binary footprint & performance metrics
│   │   ├── PROJECT_TEMPLATE_GUIDE.md # Template setup guide
│   │   ├── SPEC.md             # Functional specifications
│   │   ├── TEST_REPORT.md      # Test results report
│   │   └── TODO.md             # Task roadmap
│   ├── ja/                     # Japanese documentation
│   │   └── ...
│   └── images/                 # Visual assets
├── src/
│   ├── main.rs                 # Feature-based entry point
│   ├── lib.rs                  # Core rational algorithm & parsing
│   ├── cli.rs                  # CLI argument parsing & I/O
│   └── gui.rs                  # egui rendering & window controls
├── Cargo.toml                  # Dependencies & release profiles
├── README.md                   # English overview guide
├── README_JA.md                # Japanese overview guide
└── CHANGELOG.md                # Changelog redirect page
```

---

## 4. Data Flow & Module Cooperation

### 4.1 CLI Data Flow
1. `main.rs` invokes `cli::run_cli()`.
2. `cli.rs` parses arguments from `std::env::args()`.
3. Passes input string to `lib::parse_decimal_or_percent()` to parse percentages or decimals into an `f64`.
4. Executes `lib::approximate_fraction()` with configured parameters (`-d`, `-t`).
5. Prints the result (`numerator/denominator`) to `stdout` and exits.

### 4.2 GUI Data Flow
1. `main.rs` invokes `gui::run_gui()`.
2. Checks Named Mutex via `common_lib::desktop::acquire_single_instance()`. Exits if an instance is already running.
3. Initializes `BunkaGuiApp` via `eframe::run_native()`.
4. Updates input and triggers `recalculate()` on user input changes.
5. Updates numerator and denominator in component state upon successful calculation.
6. Renders state in `eframe::App::ui()` and supports one-click copying to clipboard.
