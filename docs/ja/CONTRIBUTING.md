[English](../en/CONTRIBUTING.md) | **日本語版**

# 貢献ガイドライン (CONTRIBUTING.md)

`MyBunka` プロジェクトへの貢献に関心を持っていただき、ありがとうございます！

## 1. 開発環境のセットアップ

1. **Rust ツールのインストール**:
   最新の Stable ツールチェーンをインストールしてください。

   ```bash
   rustup update stable
   ```

2. **リポジトリのクローン**:

   ```bash
   git clone https://github.com/tkshnkgwr/MyBunka.git
   cd MyBunka
   ```

## 2. コミット規約

コミットメッセージは以下の Conventional Commits 形式に従ってください：

- `feat:` 新機能追加
- `fix:` バグ修正
- `docs:` ドキュメントの変更
- `refactor:` リファクタリング
- `perf:` パフォーマンス最適化
- `test:` テストの追加・修正
- `chore:` ビルド・維持管理タスク

## 3. テストとローカル検証

プルリクエストを送信する前に、以下のコマンドをローカルで順次実行し、エラーおよび警告がゼロであることを確認してください。

```bash
cargo test
cargo clippy --all-targets -- -D warnings
cargo fmt --check
cargo doc --no-deps --document-private-items
```

※ Markdown ファイル（`.md`）のみの変更の場合は、事前検証コマンドの実行はスキップ可能です。
