**English** | [日本語版](../ja/TEST_REPORT.md)

# Test Verification Report (TEST_REPORT.md)

This document records the test execution environment, test cases, and empirical results verifying the mathematical correctness and robustness of `MyBunka`.

---

## 1. Test Environment

- **Operating System**: Windows 11 Home x64
- **Rust Version**: `rustc 1.96.0` (edition 2024)
- **Shell**: PowerShell 7 (`pwsh`)
- **Target Profile**: Release Build (`target/release/MyBunka.exe`)

---

## 2. Test Cases & Execution Results

### 2.1 Positive Cases (Conversions & Options)

| Test ID | Input Argument                  | Expected Output  | Actual Output    | Status   | Notes / Mathematical Details                                            |
| :------ | :------------------------------ | :--------------- | :--------------- | :------- | :---------------------------------------------------------------------- |
| TC-01   | `0.142857`                      | `1/7`            | `1/7`            | **PASS** | Approximates $1/7 \approx 0.14285714...$                                |
| TC-02   | `0.333333`                      | `1/3`            | `1/3`            | **PASS** | Approximates $1/3 \approx 0.33333333...$                                |
| TC-03   | `3.14159265`                    | `355/113`        | `355/113`        | **PASS** | Standard Milü approximation for $\pi$                                   |
| TC-04   | `0.0`                           | `0/1`            | `0/1`            | **PASS** | Boundary case for zero input                                            |
| TC-05   | `-0.5`                          | `-1/2`           | `-1/2`           | **PASS** | Negative decimal input handling                                         |
| TC-08   | `--version`                     | `MyBunka v1.0.0` | `MyBunka v1.0.0` | **PASS** | Version, copyright, and license display (Exit Code 0)                   |
| TC-09   | `--help`                        | Detailed Help    | Detailed Help    | **PASS** | Help message display (Exit Code 0)                                      |
| TC-10   | `3.14159265 -d 100`             | `22/7`           | `22/7`           | **PASS** | Max denominator constrained to 100 yields Archimedes approximation $22/7$ |
| TC-11   | `0.142857 -t 1e-10 -d 10000000` | `142857/1000000` | `142857/1000000` | **PASS** | Tight tolerance and expanded denominator preserve literal decimal value |
| TC-16   | `10%`                           | `1/10`           | `1/10`           | **PASS** | Percentage notation approximation ($10\% = 0.1$)                        |
| TC-17   | `  -5.5 % `                     | `-11/200`        | `-11/200`        | **PASS** | Negative percentage with surrounding whitespace                         |

#### CLI Command Execution Examples

```powershell
PS MyBunka> target/release/MyBunka.exe 0.142857
1/7
PS MyBunka> target/release/MyBunka.exe --version
MyBunka v1.0.0
Copyright (c) 2026 tkshnkgwr
License: MIT
PS MyBunka> target/release/MyBunka.exe --help
MyBunka - 小数点数から分数への近似変換ツール

使用方法:
    MyBunka <小数点数> [オプション]

引数:
    <小数点数>        分数に近似変換したい浮動小数点数

オプション:
    -d, --max-den <値>  近似計算に使用する最大分母 (デフォルト: 100,000)
    -t, --tolerance <値> 近似計算の許容誤差 (デフォルト: 1e-6)
    -h, --help        このヘルプメッセージを表示して終了します
    -v, -V, --version バージョン情報を表示して終了します

使用例:
    MyBunka 0.142857   -> 1/7
    MyBunka 3.14159265 -d 1000 -> 355/113
```

---

### 2.2 Error Handling & Input Validation Cases

#### TC-06: Missing Arguments
- **Input**: None (executed with no parameters)
- **Expected**: Exit code `1`, usage printed to `stderr`.
- **Status**: **PASS**

#### TC-07: Invalid String Input
- **Input**: `invalid`
- **Expected**: Exit code `1`, invalid format error printed to `stderr`.
- **Status**: **PASS**

#### TC-12: Invalid Max Denominator Value
- **Input**: `0.142857 -d 0`
- **Expected**: Exit code `1`, invalid denominator error printed to `stderr`.
- **Status**: **PASS**

#### TC-13: Missing Option Value
- **Input**: `0.142857 -d`
- **Expected**: Exit code `1`, missing argument error printed to `stderr`.
- **Status**: **PASS**

#### TC-14: Unknown Option
- **Input**: `0.142857 --unknown`
- **Expected**: Exit code `1`, unknown option error printed to `stderr`.
- **Status**: **PASS**

#### TC-18: Invalid Percentage Format
- **Input**: `10%%`
- **Expected**: Exit code `1`, invalid format error printed to `stderr`.
- **Status**: **PASS**

---

## 3. Automated Continuous Testing

All automated tests are executed via `cargo test`:

```powershell
PS MyBunka> cargo test
```

### Result Summary

```text
running 8 tests
test cli::tests::test_parse_cli_args_basic ... ok
test cli::tests::test_parse_cli_args_invalid ... ok
test cli::tests::test_parse_cli_args_options ... ok
test cli::tests::test_parse_cli_args_help_version ... ok
test tests::test_approximate_fraction_negative ... ok
test tests::test_approximate_fraction_positive ... ok
test tests::test_approximate_fraction_zero ... ok
test tests::test_parse_decimal_or_percent ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests mybunka

running 2 tests
test src/lib.rs - approximate_fraction (line 29) ... ok
test src/lib.rs - parse_decimal_or_percent (line 111) ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.08s
```

All 8 unit tests and 2 doctests pass cleanly with zero warnings.
