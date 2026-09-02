**English** | [日本語版](../ja/SPEC.md)

# Technical Specification (SPEC.md)

Software requirements, interface definitions, and rational algorithm specifications for `MyBunka`.

---

## 1. CLI Specifications

### 1.1 Interface Definition

- **Syntax**: `MyBunka <decimal or percentage> [options]` (e.g. `MyBunka 0.142857` / `MyBunka 10%`)
- **stdout**: Outputs reduced fraction `<numerator>/<denominator>` (e.g., `1/7`, `1/10`).
- **stderr**: Outputs help and warnings on failure.
- **Exit Codes**:
  - `0`: Success
  - `1`: Failure / Invalid arguments

### 1.2 Options

| Long Flag     | Short Flag | Type                     | Default   | Description                              |
| :------------ | :--------- | :----------------------- | :-------- | :--------------------------------------- |
| `--max-den`   | `-d`       | Positive Integer (`u64`) | `100,000` | Maximum denominator limit.               |
| `--tolerance` | `-t`       | Positive Float (`f64`)   | `1e-6`    | Approximation error tolerance threshold. |
