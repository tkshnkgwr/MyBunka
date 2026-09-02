**English** | [日本語版](../ja/FOOTPRINTS.md)

# Performance & Footprint Metrics (FOOTPRINTS.md)

This document records empirical measurement data regarding binary file size, memory utilization, and execution performance for `MyBunka`.

---

## 1. Overview of Metrics

Metrics measured on Windows 11 x64 using Rust 1.96.0 (`x86_64-pc-windows-msvc` target).

### 1.1 Binary File Size

- **CLI Version File Size**: `138,752 Bytes` (~135.5 KB)
- **Evaluation**: The CLI version is ultra-compact with zero external dependencies, containing only pure rational logic and argument parsing.

### 1.2 Memory Usage (RAM)

- **Peak Physical Working Set**: Estimated `1.2 MB – 2.0 MB` during execution.
- **Allocation Strategy**: Continued fraction iterations operate completely on stack-allocated state transitions with zero heap allocations (`Vec`, `String`) inside the main loop.

### 1.3 Execution Time

- **Startup & Calculation Time**: ~20 ms (measured via PowerShell `Measure-Command`).
- **Algorithm Iteration Cycles**: Max 50 steps (typically converges within 3 to 10 cycles for standard decimals).

---

## 2. Release Optimization Profile

Applied in [Cargo.toml](../../Cargo.toml):

```toml
[profile.release]
opt-level = 'z'       # Optimize for size
lto = true            # Link-time optimization
codegen-units = 1     # Single codegen unit
panic = 'abort'       # Abort on panic
strip = true          # Strip symbols
```

---

## 3. Reproduction Command

Run in **PowerShell 7 (pwsh)**:

```powershell
# Measure CLI Binary Size
cargo build --release
(Get-Item target/release/MyBunka.exe).Length

# Measure Execution Speed
Measure-Command { target/release/MyBunka.exe 0.142857 }
```
