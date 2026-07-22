**English** | [日本語版](../ja/TESTING.md)

# Testing Policy & Guide (TESTING.md)

Guidelines for automated testing and local verification procedures for `bunka`.

---

## 1. Unit Testing Policy

Core algorithm tests are maintained in `src/lib.rs` (`tests` module).

## 2. Running Local Verification

```powershell
cargo test
cargo clippy --all-targets -- -D warnings
cargo fmt --check
cargo doc --no-deps --document-private-items
```

*(Local verification commands can be skipped for Markdown-only edits).*
