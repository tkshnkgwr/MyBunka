# MyBunka (分数近似ツール)

[![Latest Release](https://img.shields.io/github/v/release/tkshnkgwr/MyBunka)](https://github.com/tkshnkgwr/MyBunka/releases)
[![CI](https://github.com/tkshnkgwr/MyBunka/actions/workflows/ci.yml/badge.svg)](https://github.com/tkshnkgwr/MyBunka/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/license-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Rust-2024-orange?logo=rust)](https://www.rust-lang.org/)
[![Platform](https://img.shields.io/badge/Platform-Windows%20%7C%20macOS%20%7C%20Linux-blue)](https://github.com/tkshnkgwr/MyBunka)

[English](README.md) | **日本語版**

`MyBunka` は、与えられた小数点数やパーセント表記（例：`10%`）を**連分数展開（Continued Fraction Expansion）アルゴリズム**を用いて、近似的な分数（分子/分母）に変換する軽量な Rust 製コマンドラインツールです。

## 特徴

- **高い近似精度**: 最大分母制限および許容誤差を設定することで、実数値を高精度に分数へと近似します。
- **極小のフットプリント**: 外部依存ライブラリを一切持たず、リリースビルド時のバイナリサイズは約138.5 KBと非常に軽量です。
- **数理的なアプローチ**: 連分数展開の性質を利用し、規定の分母範囲内で最も誤差が小さくなる最適な有理数を探索します。

## クイックスタート

### 前提条件

- [Rust ツールチェーン](https://rustup.rs/) (Edition 2024 対応)

### ビルド手順

リポジトリをクローンし、リリースビルドを実行します：

```bash
cargo build --release
```

最適化されたバイナリが `target/release/MyBunka.exe` (Linux/macOS の場合は `target/release/MyBunka`) に生成されます。

## 使い方

コマンドライン引数に変換したい小数点数とオプションを指定して実行します：

```bash
MyBunka <小数点数> [オプション]
```

### オプション

- `-d, --max-den <値>`: 近似計算に使用する最大分母 (デフォルト: 100,000)
- `-t, --tolerance <値>`: 近似計算の許容誤差 (デフォルト: 1e-6)
- `-h, --help`: このヘルプメッセージを表示して終了します
- `-v, -V, --version`: バージョン情報を表示して終了します

### 実行例

```bash
$ MyBunka 0.142857
1/7

# 最大分母を100に制限する
$ MyBunka 3.14159265 -d 100
22/7

# 許容誤差を極めて小さくし、最大分母を拡張する
$ MyBunka 0.142857 -t 1e-10 -d 10000000
142857/1000000

# パーセント表記の入力（自動で100で除算されます）
$ MyBunka 10%
1/10

# スペースや負の数を含むパーセント表記の入力
$ MyBunka "  -5.5 % "
-11/200
```

### エラーハンドリング

引数が指定されていない場合や、無効な文字列が渡された場合は、標準エラー出力（`stderr`）にヘルプまたはエラーメッセージを出力し、終了コード `1` で終了します。

```bash
$ MyBunka
使用方法: MyBunka <小数点数> [オプション]
例) MyBunka 0.142857  ->  1/7

$ MyBunka invalid
エラー: 'invalid' は無効な数値またはパーセント表記です
```

## 📚 ドキュメント一覧

詳細なドキュメントは `docs/ja/` 配下の各ファイルを参照してください：

- [システム仕様書 (docs/ja/SPEC.md)](docs/ja/SPEC.md) - アルゴリズムの詳細および引数仕様。
- [システム設計書 (docs/ja/ARCHITECTURE.md)](docs/ja/ARCHITECTURE.md) - システムの全体設計、技術スタック、ディレクトリ構造、データフロー。
- [構成図・フローチャート (docs/ja/DIAGRAM.md)](docs/ja/DIAGRAM.md) - 処理フローと構造図。
- [開発指示書 (docs/ja/INSTRUCTIONS.md)](docs/ja/INSTRUCTIONS.md) - AIや開発者がコードを修正・追加する際のコーディングスタイル、命名規則、エラーハンドリング規約。
- [開発タスク管理 (docs/ja/TODO.md)](docs/ja/TODO.md) - 実装済み機能、直近のタスク、バックログの管理。
- [フットプリント記録 (docs/ja/FOOTPRINTS.md)](docs/ja/FOOTPRINTS.md) - バイナリサイズとメモリ使用量の実測データ。
- [テスト方針・ガイド (docs/ja/TESTING.md)](docs/ja/TESTING.md) - 自動テスト方針とテスト検証手順。
- [テスト検証報告書 (docs/ja/TEST_REPORT.md)](docs/ja/TEST_REPORT.md) - テストケースと実行結果。
- [リリース手順書 (docs/ja/RELEASE.md)](docs/ja/RELEASE.md) - バージョン更新およびリリース作業の手順。
- [貢献ガイドライン (docs/ja/CONTRIBUTING.md)](docs/ja/CONTRIBUTING.md) - PRルールやコミット規約。
- [セキュリティポリシー (docs/ja/SECURITY.md)](docs/ja/SECURITY.md) - 脆弱性報告手順とサポートバージョン方針。
- [プロジェクト初期設定テンプレートガイド (docs/ja/PROJECT_TEMPLATE_GUIDE.md)](docs/ja/PROJECT_TEMPLATE_GUIDE.md) - エディタ、CI/CD、Dependabotの設定テンプレート。
- [変更履歴 (docs/ja/CHANGELOG.md)](docs/ja/CHANGELOG.md) - プロジェクトの変更履歴。

## 開発 (Development)

本リポジトリには、エディタ設定の統一や自動化ワークフローが組み込まれています：

- **エディタ設定の統一**: コードの書式を一貫させるため、[.editorconfig](.editorconfig) および VS Code 用設定 [.vscode/settings.json](.vscode/settings.json) を提供しています。
- **CI/CD**: プルリクエストやメインブランチへのプッシュ時に [.github/workflows/ci.yml](.github/workflows/ci.yml) による自動テストを実行します。また、リリースタグ（`v*`）のプッシュ時には、Windows 向け CLI バイナリをビルドし、zip アーカイブにまとめて GitHub Releases に自動デプロイする [.github/workflows/release.yml](.github/workflows/release.yml) を設定しています。
- **自動アップデート (Dependabot)**: 依存ライブラリや GitHub Actions の更新を週次でチェックし、PR を自動作成する [.github/dependabot.yml](.github/dependabot.yml) を設定しています。

## ライセンス

本プロジェクトは [MIT ライセンス](LICENSE) の下でオープンソースとして公開されています。詳細については [LICENSE](LICENSE) ファイルを参照してください。

