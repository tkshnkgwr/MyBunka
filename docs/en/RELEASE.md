**English** | [日本語版](../ja/RELEASE.md)

# Release Procedures Guide (RELEASE.md)

This document outlines versioning and release procedures for `bunka`.

---

## 1. Automated Release Workflow (Recommended)

GitHub Actions workflow (`.github/workflows/release.yml`) monitors `Cargo.toml` on `main` branch.

1. **Bump Version in `Cargo.toml`**:
   ```toml
   [package]
   name = "bunka"
   version = "0.4.7"
   ```

2. **Update Changelog**:
   Add release notes to `docs/en/CHANGELOG.md` and `docs/ja/CHANGELOG.md`.

3. **Commit & Push to `main`**:
   ```bash
   git commit -am "chore: bump version to 0.4.7"
   git push origin main
   ```

4. **Automated GitHub Release**:
   Actions will automatically detect the new tag `v0.4.7`, build production binary (CLI), and publish the GitHub release asset (`bunka-windows-x64.zip`).
