**English** | [日本語版](../ja/PROJECT_TEMPLATE_GUIDE.md)

# Project Template Guide (PROJECT_TEMPLATE_GUIDE.md)

This guide provides setup templates and workflows for standardizing Rust CLI applications.

---

## 1. Editor Configuration

### 1.1 `.editorconfig`
```ini
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
  }
}
```

---

## 2. Release Optimization Profile (`Cargo.toml`)

```toml
[profile.release]
opt-level = 'z'
lto = true
codegen-units = 1
panic = 'abort'
strip = true
```
