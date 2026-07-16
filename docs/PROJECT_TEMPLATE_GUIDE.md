# プロジェクト初期設定テンプレートガイド (PROJECT_TEMPLATE_GUIDE.md)

このドキュメントでは、Rust プロジェクト（特に CLI / GUI デスクトップアプリ）の開発環境の標準化、自動ビルド・テスト、およびリリースデプロイメントを迅速にセットアップするための各種テンプレートファイルと設定手順について説明します。

次回以降、同様の構成でプロジェクトを立ち上げる際は、本ガイドの設定ファイルをコピーして使用してください。

---

## 1. 開発エディタ設定

### 1.1 `.editorconfig`
プロジェクト全体のコーディング規約（改行コード LF、文字コード UTF-8 BOMなし、インデント幅など）をエディタ間で統一します。

**設定パス**: `.editorconfig` (プロジェクトルート)

```ini
# EditorConfig is awesome: https://EditorConfig.org

root = true

[*]
charset = utf-8
end_of_line = lf
insert_final_newline = true
trim_trailing_whitespace = true
indent_style = space
indent_size = 4

[*.{md,yml,yaml}]
indent_size = 2

[*.rs]
indent_size = 4
```

### 1.2 `.vscode/settings.json`
VS Code 利用者向けの設定を定義します。ファイルの保存時に `rustfmt` による自動フォーマットを有効化し、文字コードや改行コードを EditorConfig と整合させます。

**設定パス**: `.vscode/settings.json`

```json
{
  "editor.formatOnSave": true,
  "editor.trimTrailingWhitespace": true,
  "editor.insertSpaces": true,
  "editor.tabSize": 4,
  "files.insertFinalNewline": true,
  "files.eol": "\n",
  "files.encoding": "utf8",
  "[rust]": {
    "editor.defaultFormatter": "rust-lang.rust-analyzer"
  },
  "[markdown]": {
    "editor.tabSize": 2,
    "editor.wordWrap": "on"
  },
  "[yaml]": {
    "editor.tabSize": 2
  }
}
```

---

## 2. GitHub Actions CI/CD ワークフロー

### 2.1 継続的インテグレーション (`ci.yml`)
プルおよびプルリクエスト発生時に Windows 環境で自動でテストとビルドを実行し、コードの健全性を検証します。外部ライブラリなどの別リポジトリに依存している場合のチェックアウト設定や、作業ディレクトリの設定が含まれています。

**設定パス**: `.github/workflows/ci.yml`

```yaml
name: Rust CI

on:
  push:
    branches: [ "main" ]
  pull_request:
    branches: [ "main" ]

env:
  CARGO_TERM_COLOR: always

jobs:
  test:
    name: Run cargo test and cargo build (Windows)
    runs-on: windows-latest

    defaults:
      run:
        working-directory: <YOUR_APP_NAME> # アプリフォルダ内で実行する場合

    steps:
    - name: Checkout repository
      uses: actions/checkout@v4
      with:
        path: <YOUR_APP_NAME>

    - name: Checkout dependent library (Optional)
      uses: actions/checkout@v4
      with:
        repository: <YOUR_GITHUB_ORG_OR_USER>/<DEPENDENT_LIB_NAME>
        path: <DEPENDENT_LIB_NAME>
        token: ${{ secrets.PAT || github.token }} # プライベートリポジトリ対応
    
    - name: Install Rust
      uses: dtolnay/rust-toolchain@stable
      
    - name: Rust cache
      uses: Swatinem/rust-cache@v2
      with:
        workspaces: <YOUR_APP_NAME>
      
    - name: Run cargo test
      run: cargo test --verbose
      
    - name: Run cargo build
      run: cargo build --release --verbose
```

### 2.2 継続的デプロイ・自動リリース (`release.yml`)
`main` ブランチへのプッシュをトリガーとして `Cargo.toml` 内のバージョンと既存のGitタグを照合し、新バージョンの場合に自動でGitタグを作成してリリースをデプロイします。パッケージング処理は `target/release` ディレクトリ内で完結するように構成されています。

**設定パス**: `.github/workflows/release.yml`

```yaml
name: Release

on:
  push:
    branches:
      - main

permissions:
  contents: write

jobs:
  build-release:
    name: Build & Release (Windows)
    runs-on: windows-latest
    defaults:
      run:
        working-directory: <YOUR_APP_NAME>

    steps:
      - name: Checkout repository
        uses: actions/checkout@v4
        with:
          path: <YOUR_APP_NAME>
          fetch-depth: 0

      - name: Checkout dependent library (Optional)
        uses: actions/checkout@v4
        with:
          repository: <YOUR_GITHUB_ORG_OR_USER>/<DEPENDENT_LIB_NAME>
          path: <DEPENDENT_LIB_NAME>
          token: ${{ secrets.PAT || github.token }}

      - name: Check version and determine release
        id: check_version
        shell: pwsh
        run: |
          # Cargo.toml からバージョンを取得
          $version = (Get-Content Cargo.toml | Select-String -Pattern '^version\s*=\s*"(.*)"').Matches.Groups[1].Value
          $tag_name = "v$version"
          Write-Output "Detected version in Cargo.toml: $version ($tag_name)"
          
          # リモートリポジトリに既存タグが完全一致で存在するか確認
          $tag_exists = git ls-remote origin refs/tags/$tag_name
          if ($tag_exists) {
            Write-Output "Tag refs/tags/$tag_name already exists. Skipping release."
            Add-Content -Path $env:GITHUB_ENV -Value "SKIP_RELEASE=true"
          } else {
            Write-Output "Tag refs/tags/$tag_name does not exist. Proceeding with release."
            Add-Content -Path $env:GITHUB_ENV -Value "SKIP_RELEASE=false"
            Add-Content -Path $env:GITHUB_ENV -Value "RELEASE_TAG=$tag_name"
          }

      - name: Install Rust
        if: env.SKIP_RELEASE != 'true'
        uses: dtolnay/rust-toolchain@stable

      # 1. CLI版のビルド
      - name: Build CLI release
        if: env.SKIP_RELEASE != 'true'
        run: cargo build --release --verbose

      # 2. CLI版バイナリの退避
      - name: Package CLI binary
        if: env.SKIP_RELEASE != 'true'
        shell: pwsh
        run: |
          Copy-Item -Path target/release/<YOUR_APP_NAME>.exe -Destination target/release/<YOUR_APP_NAME>-cli-temp.exe -Force

      # 3. GUI版のビルド (gui feature がある場合)
      - name: Build GUI release
        if: env.SKIP_RELEASE != 'true'
        run: cargo build --release --features gui --verbose

      # 4. GUI版バイナリの退避とリネーム、および CLI 版の復元
      - name: Package GUI binary
        if: env.SKIP_RELEASE != 'true'
        shell: pwsh
        run: |
          Copy-Item -Path target/release/<YOUR_APP_NAME>.exe -Destination target/release/<YOUR_APP_NAME>-gui.exe -Force
          Move-Item -Path target/release/<YOUR_APP_NAME>-cli-temp.exe -Destination target/release/<YOUR_APP_NAME>.exe -Force

      # 5. 両方のバイナリを含む zip アーカイブの作成
      - name: Archive production binaries
        if: env.SKIP_RELEASE != 'true'
        shell: pwsh
        run: |
          Compress-Archive -Path target/release/<YOUR_APP_NAME>.exe, target/release/<YOUR_APP_NAME>-gui.exe -DestinationPath target/release/<YOUR_APP_NAME>-windows-x64.zip -Force

      # 6. GitHub Release の作成とアップロード
      - name: Create GitHub Release and Upload Asset
        if: env.SKIP_RELEASE != 'true'
        uses: softprops/action-gh-release@v2
        with:
          tag_name: ${{ env.RELEASE_TAG }}
          files: <YOUR_APP_NAME>/target/release/<YOUR_APP_NAME>-windows-x64.zip
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
```
*※注: テンプレート内の `<YOUR_APP_NAME>` は、作成するアプリケーション名（例: `bunka` など、`Cargo.toml` の `name` フィールドに指定した値）に置換して使用してください。*

---

## 3. 依存ライブラリの自動アップデート設定

### 3.1 `dependabot.yml`
GitHub Actions のアクションや Cargo（Rust）の外部ライブラリを週次で自動スキャンし、更新があった場合に自動でプルリクエストを生成します。

**設定パス**: `.github/dependabot.yml`

```yaml
version: 2
updates:
  # GitHub Actions のアップデート設定
  - package-ecosystem: "github-actions"
    directory: "/"
    schedule:
      interval: "weekly"

  # Cargo (Rust) のアップデート設定
  - package-ecosystem: "cargo"
    directory: "/"
    schedule:
      interval: "weekly"
```

---

## 4. Cargo リリースバイナリの最適化設定 (`Cargo.toml`)

バイナリサイズおよびメモリフットプリントを最小限に抑えるためのリリース最適化設定です。

**設定パス**: `Cargo.toml` の末尾

```toml
[profile.release]
opt-level = 'z'       # バイナリサイズを最優先で最適化（命令の配置を圧縮）
lto = true            # リンク時最適化（デッドコードの削減をクレート間で徹底）
codegen-units = 1     # コード生成単位を1に統合（LLVMによるインライン化を最大化）
panic = 'abort'       # パニック時に即時終了（スタック展開用のメタデータと展開ロジックを排除）
strip = true          # シンボル情報とデバッグ情報を実行ファイルから完全に削除
```

---

## 5. 依存ライブラリの標準設定例 (`Cargo.toml`)

CLI / GUI 共通での起動制御（二重起動防止）や、最前面・透過・枠なしウィンドウ（eframe/egui）を利用する際の、標準的なクレート構成例です。

**設定パス**: `Cargo.toml` の `[dependencies]` / `[features]`

```toml
[dependencies]
# eframe (egui フレームワーク本体): GUI表示に使用
eframe = { version = "0.35.0", optional = true }

# windows (Windows APIの呼び出し): 名前付きMutexによる二重起動制御に使用
windows = {
    version = "0.62.0",
    features = [
        "Win32_System_Threading",
        "Win32_Foundation",
        "Win32_Security"
    ],
    optional = true
}

# winapi (他のWin32制御に使用、必要に応じて)
winapi = { version = "0.3.9", features = ["winuser", "windef"], optional = true }

[features]
default = []
gui = ["dep:eframe", "dep:windows", "dep:winapi"]
```
