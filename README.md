---
type: Reference
title: "opencode グローバル設定"
description: "Termux・Kubuntu・Windows で共有する opencode 設定と MCP 構成"
tags:
  - opencode
  - configuration
  - mcp
generated:
  by: opencode
  at: "2026-10-24T23:58:31+09:00"
verified:
  by: opencode
  at: "2026-11-03T18:01:15+09:00"
---

## 概要

`opencode.jsonc` は全プラットフォームで同一内容を保ち、環境依存の差は
次の 3 層で吸収する。パス置換スクリプトや git フックは使わない。

- Node 実行ファイル: PATH 上の `node` を使う
- モジュール解決: `mcp-paths.cjs` が候補ルートを順に探索する
- ブラウザ: ランチャーが Termux の Chromium の有無で引数を切り替える

シェルは PATH 上の `bash` を指定する（`"shell": "bash"`）。Termux の
`termux-exec` は opencode 起動時の環境から子プロセスへ継承される。

## 前提

| 項目 | 要件 |
| --- | --- |
| `node` | PATH から解決できること |
| `bash` | PATH から解決できること（`shell` 設定） |
| MCP モジュール | 下記いずれかの `node_modules` に導入済みであること |
| `OPENCODE_LD_PRELOAD` | Termux の `libtermux-exec.so`（任意） |

`mcp-paths.cjs` は次の順でモジュールを探索する。

1. `OPENCODE_NODE_MODULES`（明示的な上書き用）
2. `$HOME/.local/share/opencode/mcp/node_modules`（推奨の固定ルート）
3. `$HOME/.config/opencode/node_modules`
4. `$HOME/.npm-global/lib/node_modules`
5. `/usr/local/lib/node_modules` と `/usr/lib/node_modules`
6. `/data/data/com.termux/files/usr/lib/node_modules`（Termux のグローバル）

既存のグローバル導入は 4〜6 で解決されるため、固定ルートへの再導入は
必須ではない。

## MCP モジュールの導入

Termux・Kubuntu とも同じコマンドで固定ルートへ導入できる。

```bash
mkdir -p "$HOME/.local/share/opencode/mcp"
npm install --prefix "$HOME/.local/share/opencode/mcp" \
  chrome-devtools-mcp@latest \
  @playwright/mcp@latest \
  @modelcontextprotocol/server-memory@latest \
  time-mcp@latest
```

`package.json` と `package-lock.json` が生成され、バージョンが固定される。

### Termux

```bash
pkg install -y nodejs-lts bash chromium
export OPENCODE_LD_PRELOAD="/data/data/com.termux/files/usr/lib/libtermux-exec.so"
```

`OPENCODE_LD_PRELOAD` は必須ではない。未設定の場合は空文字として展開され、
プリロードは行われない。

### Kubuntu

追加の環境変数は不要。`node` と `bash` が PATH にあり、Google Chrome が
導入済みであれば chrome-devtools はそのまま動作する。Playwright は
`~/.cache/ms-playwright` のブラウザを使うため、未導入なら次を実行する。

```bash
npx playwright install chromium
```

### Windows (PowerShell)

`{env:HOME}` を使うため、`HOME` をユーザープロファイルに設定する。

```powershell
setx HOME $env:USERPROFILE
```

`setx` で設定した値は、新しく起動したシェルから有効になる。`bash` と
`node` が PATH に必要である。

## MCP 構成

| サーバ | 起動方法 | 備考 |
| --- | --- | --- |
| chrome-devtools | `node` + `chrome-devtools-mcp-launcher.cjs` | Termux はシステム Chromium、Kubuntu は Chrome を自動検出 |
| playwright | `node` + `playwright-mcp-launcher.cjs` | Termux は platform を `linux` に提示し Chromium を使用 |
| memory | `node` + `mcp-node-launcher.cjs` | モジュールは `mcp-paths.cjs` が解決 |
| timemcp | `node` + `time-mcp-launcher.cjs` | 起動バナーを除去して JSON-RPC のみ転送 |
| serena | `{env:HOME}/.local/bin/serena` | 未導入環境では接続失敗（他へ影響なし） |

ランチャーはすべて `{env:HOME}/.config/opencode/` 配下にあり、
`mcp-paths.cjs` を共有する。

## Termux のヘッドレスブラウザ

Termux には Playwright の Linux ARM64 ビルドを取得できるが、
`playwright-core` は `process.platform === "android"` を未サポートとして
読み込み時に例外を投げる。`playwright-mcp-launcher.cjs` が `linux` として
提示してから読み込むことで回避する。

ブラウザは両 MCP とも `pkg install chromium` のシステム Chromium (Bionic) を
使う。glibc 版 Playwright ブラウザの構築手順は `scripts/termux-glibc/` に
残してあり、必要時に `--executable-path` を切り替えられる。

- 実行ファイル: `/data/data/com.termux/files/usr/lib/chromium/chrome`
- ランチャーが Chromium の存在を検出し、`--executablePath` と
  `--executable-path` を自動で付与する
- Android では sandbox と `/dev/shm` が使えないため、
  `--no-sandbox` と `--disable-dev-shm-usage` を自動で渡す
- `playwright-mcp-launcher.cjs` が `process.platform` を `linux` として提示する
- Kubuntu ではこれらの上書きを行わず、通常のブラウザ検出に任せる

glibc 版は `scripts/termux-glibc/README.md` の手順で再現できる。
`patchelf` でインタプリタを差し替え、`LD_PRELOAD` を外す必要がある。

Playwright の永続プロファイルは
`~/.cache/ms-playwright/mcp-chromium-<workspace-hash>` に作成され、
自動出力は `~/.cache/playwright-mcp/output` に退避する。

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
