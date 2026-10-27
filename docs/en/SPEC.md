**English** | [日本語版](../ja/SPEC.md)

# Technical Specification (SPEC.md)

This document describes the software requirements, interface definitions, and detailed algorithm specifications for `MyBunka`.

---

## 1. CLI System Specifications

### 1.1 Interface Definition

`MyBunka` operates as a command-line interface (CLI) tool. It accepts a decimal number (or percentage notation) as an argument, along with optional parameters.

- **Command Syntax**:
  - Number conversion: `MyBunka <decimal or percentage> [options]` (e.g., `MyBunka 0.142857` / `MyBunka 10%`)
  - Version display: `MyBunka --version` (or `-v`, `-V`)
  - Help display: `MyBunka --help` (or `-h`)
- **Standard Output (stdout)**:
  - Number conversion: Reduced fraction formatted as `<numerator>/<denominator>` (e.g., `1/7`, `1/10`).
  - Version display: Multi-line string in the format `MyBunka v<version>\nCopyright (c) 2026 tkshnkgwr\nLicense: GPL-3.0-or-later` (e.g., `MyBunka v1.1.0`).
  - Help display: Usage instructions, arguments, options, and practical examples.
- **Standard Error (stderr)**: Outputs error messages and usage hints upon execution failure or invalid arguments.
- **Exit Codes**:
  - `0`: Success (conversion completed, version info shown, or help displayed).
  - `1`: Failure (missing arguments, invalid numeric/percentage format, or unknown option).

### 1.2 Parameter Options

The following parameter options can be specified after the input value to control the fraction approximation algorithm:

| Long Flag     | Short Flag | Type                     | Default   | Description                              |
| :------------ | :--------- | :----------------------- | :-------- | :--------------------------------------- |
| `--max-den`   | `-d`       | Positive Integer (`u64`) | `100,000` | Maximum denominator limit.               |
| `--tolerance` | `-t`       | Positive Float (`f64`)   | `1e-6`    | Approximation error tolerance threshold. |

#### Command Example

```bash
$ MyBunka 0.142857 -d 1000 -t 1e-4
1/7
```

### 1.3 Algorithm Specification (Continued Fraction Expansion)

To approximate a real number $x$ into a fraction $h/k$, `MyBunka` employs the **Continued Fraction Expansion** algorithm:

1. **Sign Handling**: Determine and extract the sign of $x$. Calculate the fraction for absolute value $|x|$, and restore the sign to the numerator at the end.
2. **Expansion Loop**:
   - Let $a_n$ be the integer part of the residual $r_n$ (initial value is $|x|$).
   - Dynamically compute convergent fractions using the recurrence relations:
     $$h_n = a_n h_{n-1} + h_{n-2}$$
     $$k_n = a_n k_{n-1} + k_{n-2}$$
     Initial state: $(h_{-1}, k_{-1}) = (1, 0)$ and $(h_{-2}, k_{-2}) = (0, 1)$.
3. **Termination Criteria**:
   - If denominator $k_n > max\_denominator$, terminate loop and adopt the previous convergent $h_{n-1}/k_{n-1}$.
   - If absolute error $|x - h_n/k_n| \le tolerance$, terminate loop immediately.
   - If the fractional part of residual ($r_n - a_n$) is nearly zero ($< 10^{-11}$), terminate loop.
   - If iteration count reaches `50`, force termination to prevent runaway loops.
   - Update residual: $r_{n+1} = 1 / (r_n - a_n)$.

### 1.4 Input Parsing Specification

In addition to standard decimal notation (e.g., `0.142857`), `MyBunka` parses percentage notation (e.g., `10%`):

1. **Whitespace Trimming**: Trim leading and trailing whitespace from the input string.
2. **Percentage Sign Check**: Detect whether the trimmed string ends with `%`.
3. **Parsing & Division**:
   - If ending with `%`, strip the `%` symbol, trim surrounding spaces, parse the remainder as a floating-point number (`f64`), and divide by `100.0` (e.g., `10%` -> `0.1`, `  -5.5 % ` -> `-0.055`).
   - If not ending with `%`, parse directly as a floating-point number.
4. **Error Handling**: Non-numeric or invalid formatting (e.g., `abc` or `10%%`) produces an error and terminates execution with exit code `1`.

---

## 2. Development, CI/CD & Maintenance

### 2.1 Editor Configuration

- **[.editorconfig](../../.editorconfig)**: Unifies line endings (`LF`), encoding (`UTF-8` without BOM), and indentation (`4` spaces for Rust, `2` spaces for Markdown/YAML). See [PROJECT_TEMPLATE_GUIDE.md](PROJECT_TEMPLATE_GUIDE.md) for shared templates.

### 2.2 CI/CD & Automated Release Workflow

- **Continuous Integration (CI)**:
  - Runs automated builds and tests on Windows on push and pull requests ([.github/workflows/ci.yml](../../.github/workflows/ci.yml)).
- **Continuous Deployment (CD) & Automated Releases**:
  - Automatically triggered upon push to `main` branch ([.github/workflows/release.yml](../../.github/workflows/release.yml)).
  - Reads `version` from `Cargo.toml` and compares it against remote Git tags.
  - If a new unreleased version is detected, automatically tags the commit and generates a GitHub Release.
  - Compiles the release binary for Windows and packages it into `MyBunka-windows-x64.zip`.

### 2.3 Automated Dependency Monitoring

- **[dependabot.yml](../../.github/dependabot.yml)**:
  - Weekly checks for GitHub Actions actions and Cargo dependencies, generating automated pull requests for updates.
