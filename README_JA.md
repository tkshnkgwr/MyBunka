# MyBunka (分数近似ツール)

[![Latest Release](https://img.shields.io/github/v/release/tkshnkgwr/MyBunka)](https://github.com/tkshnkgwr/MyBunka/releases)
[![CI](https://github.com/tkshnkgwr/MyBunka/actions/workflows/ci.yml/badge.svg)](https://github.com/tkshnkgwr/MyBunka/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/license-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Rust-2024-orange?logo=rust)](https://www.rust-lang.org/)
[![Platform](https://img.shields.io/badge/Platform-Windows%20%7C%20macOS%20%7C%20Linux-blue)](https://github.com/tkshnkgwr/MyBunka)

[English](README.md) | **日本語版**

`MyBunka` は、与えられた小数点数やパーセント表記（例：`10%`）を**連分数展開（Continued Fraction Expansion）アルゴリズム**を用いて、近似的な分数（分子/分母）に変換する超軽量な Rust 製コマンドラインツールです。

## ✨ 特徴

- **高精度な近似**: 連分数展開により、数学的に最も効率的・高精度な既約分数を導出。
- **パーセント表記対応**: `0.142857` などの通常の小数表記に加え、`10%` や `"  -5.5 % "` 等のパーセント値も直接入力可能。
- **超軽量＆高速**: 外部依存ライブラリゼロ（標準ライブラリのみ）、最適化リリースバイナリサイズ約 135 KB、実行時メモリ消費 2 MB 以下の極小フットプリント。
- **柔軟な制御パラメータ**: 最大分母（`--max-den`）や許容誤差（`--tolerance`）を自在に指定可能。

## 🚀 ビルドと実行

### 前提条件

- [Rust ツールチェーン](https://rustup.rs/) (edition 2024 対応)

### ビルド手順

リポジトリをクローンし、リリースビルドを実行します：

```bash
cargo build --release
```

最適化された実行可能バイナリが `target/release/MyBunka.exe`（Linux/macOS では `target/release/MyBunka`）に生成されます。

## 💻 使い方

コマンドライン引数に変換したい小数点数（またはパーセント表記）とオプションを指定して実行します：

```bash
MyBunka <小数点数またはパーセント表記> [オプション]
```

### オプション一覧

- `-d, --max-den <値>`: 近似計算に使用する最大分母 (デフォルト: `100,000`)
- `-t, --tolerance <値>`: 近似計算の許容誤差しきい値 (デフォルト: `1e-6`)
- `-h, --help`: ヘルプメッセージを表示して終了します
- `-v, -V, --version`: バージョンおよび著作権・ライセンス情報を表示して終了します

### 実行例

```bash
# 基本的な小数近似
$ MyBunka 0.142857
1/7

# 最大分母を100に制限（円周率のアルキメデス近似）
$ MyBunka 3.14159265 -d 100
22/7

# 許容誤差を極めて小さくし、元の小数を厳密分数化
$ MyBunka 0.142857 -t 1e-10 -d 10000000
142857/1000000

# パーセント表記の入力（自動で100除算）
$ MyBunka 10%
1/10

# スペースや負の数を含むパーセント表記
$ MyBunka "  -5.5 % "
-11/200

# バージョン表示
$ MyBunka -v
MyBunka v1.0.0
Copyright (c) 2026 tkshnkgwr
License: MIT
```

### エラーハンドリング

引数が不足している場合や、無効な数値形式・オプションが指定された場合は、標準エラー出力（`stderr`）にエラーメッセージを出力し、終了コード `1` で終了します：

```bash
$ MyBunka
使用方法: MyBunka <小数点数> [オプション]
例) MyBunka 0.142857  ->  1/7

$ MyBunka invalid
エラー: 'invalid' は無効な数値またはパーセント表記です
```

## 📚 ドキュメント一覧

詳細な仕様や設計ドキュメントは `docs/ja/` 配下に格納されています：

- [システム仕様書 (docs/ja/SPEC.md)](docs/ja/SPEC.md) - アルゴリズムの詳細および引数仕様。
- [システム設計書 (docs/ja/ARCHITECTURE.md)](docs/ja/ARCHITECTURE.md) - システムの全体設計、技術スタック、ディレクトリ構造、データフロー。
- [構成図・フローチャート (docs/ja/DIAGRAM.md)](docs/ja/DIAGRAM.md) - 処理フローと構造図 (Mermaid)。
- [開発指示書 (docs/ja/INSTRUCTIONS.md)](docs/ja/INSTRUCTIONS.md) - コーディング規約、エラー方針、ドキュメント同期ルール。
- [開発タスク管理 (docs/ja/TODO.md)](docs/ja/TODO.md) - 実装済み機能、直近のタスク、将来拡張バックログ。
- [フットプリント記録 (docs/ja/FOOTPRINTS.md)](docs/ja/FOOTPRINTS.md) - バイナリサイズとメモリ使用量の実測データ。
- [テスト方針・ガイド (docs/ja/TESTING.md)](docs/ja/TESTING.md) - 自動テスト方針とテスト検証手順。
- [テスト検証報告書 (docs/ja/TEST_REPORT.md)](docs/ja/TEST_REPORT.md) - 単体テストおよび異常系テストの実行結果レポート。
- [リリース手順書 (docs/ja/RELEASE.md)](docs/ja/RELEASE.md) - バージョン更新およびリリース作業の手順。
- [貢献ガイドライン (docs/ja/CONTRIBUTING.md)](docs/ja/CONTRIBUTING.md) - PRルールやコミット規約。
- [セキュリティポリシー (docs/ja/SECURITY.md)](docs/ja/SECURITY.md) - 脆弱性報告手順とサポートバージョン方針。
- [プロジェクト初期設定テンプレートガイド (docs/ja/PROJECT_TEMPLATE_GUIDE.md)](docs/ja/PROJECT_TEMPLATE_GUIDE.md) - エディタ、CI/CD、Dependabotの設定テンプレート。
- [変更履歴 (docs/ja/CHANGELOG.md)](docs/ja/CHANGELOG.md) - プロジェクトの変更履歴。

## 🛠️ 開発・自動化 (Development)

- **エディタ設定**: プロジェクト全体のコード品質および書式の一貫性を保つため、[.editorconfig](.editorconfig) を設定しています。
- **CI (継続的インテグレーション)**: PR やプッシュ時に自動ビルド・単体テストを実行します（[.github/workflows/ci.yml](.github/workflows/ci.yml)）。
- **CD (自動リリース)**: `main` ブランチの `Cargo.toml` バージョン更新を検知し、未リリースタグの自動発行と zip 配布アセットの GitHub Release へのアップロードを完全自動化しています（[.github/workflows/release.yml](.github/workflows/release.yml)）。
- **Dependabot**: 依存関係と GitHub Actions の更新を週次で監視します（[.github/dependabot.yml](.github/dependabot.yml)）。

## 📄 ライセンス

本プロジェクトは MIT License のもとで公開されています - Copyright (c) 2026 tkshnkgwr。詳細は [LICENSE](LICENSE) をご覧ください。
