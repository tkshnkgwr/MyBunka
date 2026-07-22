**English** | [日本語版](../ja/CONTRIBUTING.md)

# Contribution Guidelines (CONTRIBUTING.md)

Thank you for your interest in contributing to `bunka`!

## 1. Setup Environment

1. **Install Rust**:
   Ensure you have the latest stable Rust toolchain:
   ```bash
   rustup update stable
   ```
2. **Clone Repository**:
   ```bash
   git clone https://github.com/tkshnkgwr/bunka.git
   cd bunka
   ```

## 2. Commit Message Conventions

Please follow Conventional Commits format:
- `feat:` New features
- `fix:` Bug fixes
- `docs:` Documentation changes
- `refactor:` Code refactoring
- `perf:` Performance optimizations
- `test:` Test suites
- `chore:` Build/maintenance tasks

## 3. Local Verification

Run the following checks before submitting pull requests:

```bash
cargo test
cargo clippy --all-targets -- -D warnings
cargo fmt --check
cargo doc --no-deps --document-private-items
```

*(Verification commands can be skipped if changes are restricted strictly to Markdown files).*
