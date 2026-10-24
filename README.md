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
| chrome-devtools / playwright | `npx -y` | Termux では npx の bin 解決に失敗する場合がある |

## 廃止した方式

- `scripts/materialize-paths.rs` によるプレースホルダー置換
- `.githooks/post-checkout` と `post-merge` による自動マテリアライズ

working tree とコミット内容が一致するため、`git status` に差分が出ない。
