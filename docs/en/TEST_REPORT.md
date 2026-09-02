**English** | [日本語版](../ja/TEST_REPORT.md)

# Test Verification Report (TEST_REPORT.md)

Verification environment, test cases, and empirical test results for `MyBunka`.

---

## 1. Test Environment

- **OS**: Windows 11 Home x64
- **Rust Version**: `rustc 1.96.0` (edition 2024)
- **Shell**: PowerShell 7 (`pwsh`)

---

## 2. Test Cases & Execution Results

| Test ID | Input        | Expected Output | Actual Output | Status   |
| :------ | :----------- | :-------------- | :------------ | :------- |
| TC-01   | `0.142857`   | `1/7`           | `1/7`         | **PASS** |
| TC-02   | `0.333333`   | `1/3`           | `1/3`         | **PASS** |
| TC-03   | `3.14159265` | `355/113`       | `355/113`     | **PASS** |
| TC-04   | `0.0`        | `0/1`           | `0/1`         | **PASS** |
| TC-05   | `-0.5`       | `-1/2`          | `-1/2`        | **PASS** |
| TC-16   | `10%`        | `1/10`          | `1/10`        | **PASS** |

All unit test suites (`cargo test`) pass cleanly (`ok. 8 passed; 0 failed`).
