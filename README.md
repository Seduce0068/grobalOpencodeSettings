---
type: Reference
title: "opencode グローバル設定"
description: "Termux・Kubuntu・Windows で共有する opencode 設定の環境変数と MCP 構成"
tags:
  - opencode
  - configuration
  - mcp
generated:
  by: opencode
  at: "2026-10-24T23:58:31+09:00"
---

## 概要

`opencode.jsonc` は全プラットフォームで同一内容を保ち、環境依存のパスは
opencode の `{env:VAR}` 展開で吸収する。パス置換スクリプトや git フックは使わない。

## 必須の環境変数

| 変数 | 用途 | 未設定時 |
| --- | --- | --- |
| `OPENCODE_NODE` | memory MCP を起動する Node.js 実行ファイル | memory MCP 起動失敗 |
| `OPENCODE_NODE_MODULES` | グローバル npm モジュールのルート | memory MCP 起動失敗 |
| `OPENCODE_LD_PRELOAD` | Termux の `libtermux-exec.so`（任意） | プリロードなし |

### Termux

```bash
export OPENCODE_NODE="$(command -v node)"
export OPENCODE_NODE_MODULES="/data/data/com.termux/files/usr/lib/node_modules"
export OPENCODE_LD_PRELOAD="/data/data/com.termux/files/usr/lib/libtermux-exec.so"
```

### Kubuntu

```bash
export OPENCODE_NODE="$(command -v node)"
export OPENCODE_NODE_MODULES="$(npm root -g)"
```

`OPENCODE_LD_PRELOAD` は設定しない。未設定の場合は空文字として展開され、
プリロードは行われない。

### Windows (PowerShell)

```powershell
setx OPENCODE_NODE (Get-Command node).Source
setx OPENCODE_NODE_MODULES (npm root -g)
```

`setx` で設定した値は、新しく起動したシェルから有効になる。

## MCP 構成

| サーバ | 起動方法 | 備考 |
| --- | --- | --- |
| memory | `{env:OPENCODE_NODE}` + モジュール絶対パス | `@modelcontextprotocol/server-memory` をグローバル導入 |
| serena | `{env:HOME}/.local/bin/serena` | 未導入環境では接続失敗（他へ影響なし） |
| chrome-devtools / playwright | `{env:OPENCODE_NODE}` + グローバルモジュール | `npm i -g chrome-devtools-mcp @playwright/mcp` が必要 |

## Serena の導入 (Termux)

Termux では `psutil` の公式 sdist が Android を拒否し、`cryptography` の
ソースビルドは `dlopen` に失敗する。次の手順で導入する。

### 1. psutil の patched wheel

```bash
mkdir -p /data/data/com.termux/files/usr/tmp/serena && cd /data/data/com.termux/files/usr/tmp/serena
URL=$(curl -s https://pypi.org/pypi/psutil/7.0.0/json \
  | python3 -c "import sys,json; print([u['url'] for u in json.load(sys.stdin)['urls'] if u['packagetype']=='sdist'][0])")
curl -sL -o psutil.tar.gz "$URL" && tar xf psutil.tar.gz
sed -i 's/LINUX = sys.platform.startswith("linux")/LINUX = sys.platform.startswith(("linux", "android"))/' \
  psutil-7.0.0/psutil/_common.py
uv build --wheel --out-dir wheels ./psutil-7.0.0
```

### 2. serena-agent の導入

```bash
pkg install -y python-cryptography
uv tool install -p "$(command -v python3)" --find-links wheels serena-agent
```

`cryptography` は Termux パッケージ版に差し替える。

```bash
SYS=/data/data/com.termux/files/usr/lib/python3.14/site-packages
VENV=$HOME/.local/share/uv/tools/serena-agent/lib/python3.14/site-packages
rm -rf "$VENV/cryptography" && cp -a "$SYS/cryptography" "$VENV/cryptography"
```

### 3. pywebview の Android 対応

`pywebview` は Android で `ANDROID_APP_PATH` を参照し、未設定だと
`os.path.exists(None)` で落ちる。serena の MCP 環境変数に
`ANDROID_APP_PATH={env:HOME}` を設定済み。

## 廃止した方式

- `scripts/materialize-paths.rs` によるプレースホルダー置換
- `.githooks/post-checkout` と `post-merge` による自動マテリアライズ

working tree とコミット内容が一致するため、`git status` に差分が出ない。
