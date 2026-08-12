[English](../en/RELEASE.md) | **日本語版**

# リリース手順書 (RELEASE.md)

このドキュメントでは、`bunka` の新バージョンリリース手順について説明します。

---

## 1. 自動リリース手順 (推奨)

本プロジェクトの GitHub Actions ワークフロー（`.github/workflows/release.yml`）は、`main` ブランチの `Cargo.toml` のバージョン表記を自動検知してリリースを行うように自動化されています。

1. **`Cargo.toml` のバージョンカウントアップ**:
   `Cargo.toml` 内の `version` フィールドを更新します。
   ```toml
   [package]
   name = "bunka"
   version = "0.4.7" # 例: 0.4.6 -> 0.4.7
   ```

2. **変更内容のドキュメント更新**:
   - `docs/ja/CHANGELOG.md` および `docs/en/CHANGELOG.md` に最新バージョンの変更ログを追記します。

3. **コミットおよび `main` ブランチへのプッシュ**:
   ```bash
   git commit -am "chore: bump version to 0.4.7"
   git push origin main
   ```

4. **自動化ワークフローの実行**:
   - GitHub Actions が自動でトリガーされ、`v0.4.7` タグの有無をチェックします。
   - タグが存在しない場合、CLI版バイナリがビルド・パッケージングされ、GitHub Release が自動生成されます。

---

## 2. 手動リリースビルドの手順

ローカル環境でリリース用バイナリを直接ビルドする場合は、PowerShell 7 (pwsh) で以下を実行します：

```powershell
# CLI版のリリースビルド
cargo build --release
```
生成されたバイナリは `target/release/bunka.exe` です。
