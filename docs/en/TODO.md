**English** | [日本語版](../ja/TODO.md)

# Task & Roadmap Management (TODO.md)

Current implementation status (Done), active tasks (In Progress), and roadmap proposals (Backlog) for `MyBunka`.

---

## 1. Implemented Features (Done)

- [x] Continued fraction expansion algorithm for rational approximation (`lib.rs`).
- [x] Parsing support for percentage inputs (e.g. `10%` -> `0.1` -> `1/10`).
- [x] CLI interface with options for max denominator (`-d`) and tolerance (`-t`).
- [x] Comprehensive CLI argument parsing unit tests (`cli.rs`).
- [x] Multi-language documentation structure (`docs/en/` & `docs/ja/`).
- [x] Renamed project and CLI binary to `MyBunka` with full documentation and CI synchronization.

---

## 2. Active Tasks (In Progress / Todo)

- All immediate tasks completed.

---

## 3. Backlog Proposals

- [ ] **Standard Input (stdin / Pipeline) Support**
  - Accept streams from standard input (e.g., `echo "0.142857" | MyBunka` or `cat numbers.txt | MyBunka`) to facilitate shell script pipelines and batch processing.
- [ ] **Output Format Options (JSON / CSV / Mixed Fraction)**
  - Structured output with `--json` (e.g., `{"input": 0.142857, "numerator": 1, "denominator": 7, "error": 0.00000014}`).
  - Mixed fraction output with `--mixed` / `-m` (e.g., `3 1/7` for `3.141592`).
- [ ] **Continued Fraction Step Visualization (Verbose Mode)**
  - Detailed step-by-step table output of convergent coefficients `[a0; a1, a2, ...]` and intermediate approximations via `--verbose`.
- [ ] **Expression Parser**
  - Parse simple arithmetic expressions (e.g., `0.5 + 1/3` or `10% * 3`) and approximate their resulting values.
- [ ] **Interactive REPL Mode**
  - Interactive shell session to repeatedly input and convert numbers without restarting the binary.
- [ ] **Cross-Platform Automated Releases (Linux / macOS)**
  - Expand `.github/workflows/release.yml` with a build matrix supporting Linux (x86_64, aarch64) and macOS (Apple Silicon / Intel) target binaries.
- [ ] **Shell Auto-Completion Scripts (Shell Completions)**
  - Generate shell completions via `--completions bash|zsh|powershell|fish`.
