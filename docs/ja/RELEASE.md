[English](../en/RELEASE.md) | **日本語版**

# リリース手順書 (RELEASE.md)

このドキュメントでは、`MyBunka` の新バージョンリリース手順について説明します。

---

## 1. 自動リリース手順 (推奨)

本プロジェクトの GitHub Actions ワークフロー（`.github/workflows/release.yml`）は、`main` ブランチの `Cargo.toml` のバージョン表記を自動検知してリリースを行うように自動化されています。

1. **`Cargo.toml` のバージョンカウントアップ**:
   `Cargo.toml` 内の `version` フィールドを更新します。

   ```toml
   [package]
   name = "mybunka"
   version = "1.0.1" # 例: 1.0.0 -> 1.0.1
   ```

2. **変更内容のドキュメント更新**:
   - `docs/ja/CHANGELOG.md` および `docs/en/CHANGELOG.md` に最新バージョンの変更ログを追記します。

3. **コミットおよび `main` ブランチへのプッシュ**:

   ```bash
   git commit -am "chore: bump version to 1.0.1"
   git push origin main
   ```

4. **自動化ワークフローの実行**:
   - GitHub Actions が自動でトリガーされ、`v1.0.1` タグの有無をチェックします。
   - タグが存在しない場合、CLI版バイナリがビルド・パッケージングされ、GitHub Release が自動生成されます。

---

## 2. 手動リリースビルドの手順

ローカル環境でリリース用バイナリを直接ビルドする場合は、PowerShell 7 (pwsh) で以下を実行します：

```powershell
# CLI版のリリースビルド
cargo build --release
```

生成されたバイナリは `target/release/MyBunka.exe` です。
