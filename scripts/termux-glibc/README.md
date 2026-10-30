---
type: Playbook
title: "Termux glibc 版 Playwright ブラウザの構築"
description: "Termux の glibc 環境で NSPR/NSS をソースビルドし、Playwright の Chrome Headless Shell を動作させる手順。"
tags:
  - termux
  - glibc
  - playwright
  - nspr
  - nss
generated:
  by: opencode
  at: "2026-10-31T00:05:00+09:00"
---

## 概要

Termux の glibc リポジトリには Mozilla NSPR/NSS が無いため、
Playwright の Linux ARM64 ビルドはそのままでは起動できない。
本手順では NSPR と NSS をソースからビルドし、
`glibc-runner` ではなく `patchelf` でインタプリタを差し替えて動かす。

## 前提パッケージ

```bash
pkg install gcc-glibc make-glibc python-glibc pkgconf-glibc \
  glib-glibc glib-bin-glibc libxml2-glibc dbus-glibc libsqlite-glibc \
  libxcomposite-glibc libxfixes-glibc libxrandr-glibc libxext-glibc \
  libxrender-glibc libxi-glibc libxtst-glibc libdrm-glibc pango-glibc \
  libcairo-glibc alsa-lib-glibc libxkbcommon-glibc mesa-glibc \
  meson ninja patchelf
python3 -m pip install --user meson
```

## NSPR と NSS

```bash
mkdir -p ~/build/nss && cd ~/build/nss
curl -LO https://ftp.mozilla.org/pub/nspr/releases/v4.40/src/nspr-4.40.tar.gz
tar xf nspr-4.40.tar.gz
curl -LO https://ftp.mozilla.org/pub/security/nss/releases/NSS_3_132_RTM/src/nss-3.132.tar.gz
tar xf nss-3.132.tar.gz

glibc-runner -s 'bash ~/.config/opencode/scripts/termux-glibc/build-nspr.sh'
glibc-runner -s 'bash ~/.config/opencode/scripts/termux-glibc/build-nss.sh'
```

NSS は `USE_64=1` が必須。指定しないと `PR_STATIC_ASSERT` が
32bit 前提で失敗する。

## ATK と atspi

```bash
mkdir -p ~/build/atspi && cd ~/build/atspi
curl -LO https://download.gnome.org/sources/at-spi2-core/2.62/at-spi2-core-2.62.1.tar.xz
tar xf at-spi2-core-2.62.1.tar.xz
glibc-runner -s 'bash ~/.config/opencode/scripts/termux-glibc/build-atk.sh'
```

生成物の SONAME が `libatk-1.0.so` / `libatspi.so` になるため、
Chromium が要求する `.so.0` のリンクを補う。

```bash
ln -sf libatk-1.0.so "$PREFIX/glibc/lib/libatk-1.0.so.0"
ln -sf libatspi.so "$PREFIX/glibc/lib/libatspi.so.0"
ln -sf libatk-bridge-2.0.so "$PREFIX/glibc/lib/libatk-bridge-2.0.so.0"
```

## libXdamage と libudev-zero

```bash
mkdir -p ~/build/xdamage && cd ~/build/xdamage
curl -LO https://xorg.freedesktop.org/archive/individual/lib/libXdamage-1.1.6.tar.xz
tar xf libXdamage-1.1.6.tar.xz
glibc-runner -s 'bash ~/.config/opencode/scripts/termux-glibc/build-Xdamage.sh'

mkdir -p ~/build/udev && cd ~/build/udev
curl -L -o libudev-zero.tar.gz \
  https://github.com/illiliti/libudev-zero/archive/refs/tags/1.0.1.tar.gz
tar xf libudev-zero.tar.gz
glibc-runner -s 'bash ~/.config/opencode/scripts/termux-glibc/build-libudev-zero.sh'
```

## Playwright ブラウザ

`playwright-core` の android 判定を preload で回避して取得する。

```bash
PWCORE=$PREFIX/lib/node_modules/@playwright/mcp/node_modules/playwright-core
PRELOAD=~/.config/opencode/scripts/termux-glibc/playwright-platform-preload.cjs
NODE_OPTIONS="--require $PRELOAD" \
  node "$PWCORE/cli.js" install chromium-headless-shell
```

インタプリタを glibc の ld-linux に差し替える。
`/proc/self/exe` が Chrome 自身を指すようになり、子プロセス起動が壊れない。

```bash
CHROME_DIR=~/.cache/ms-playwright/chromium_headless_shell-1247/chrome-headless-shell-linux-arm64
cp -a "$CHROME_DIR/chrome-headless-shell" "$CHROME_DIR/chrome-headless-shell.orig"
patchelf --set-interpreter "$PREFIX/glibc/lib/ld-linux-aarch64.so.1" \
  --set-rpath "$PREFIX/glibc/lib" "$CHROME_DIR/chrome-headless-shell"
```

ICU データは `DIR_MODULE` から探索されるため、glibc 側にも置く。

```bash
cp "$CHROME_DIR/icudtl.dat" "$PREFIX/glibc/lib/icudtl.dat"
```

## 既知の回避策

- `libc.so` はリンカスクリプトなので、実行時は `libc.so.6` への
  シンボリックリンクに置き換える。
- Chromium が要求する `libxml2.so.2` に対し、glibc 版は
  `libxml2.so.16` を提供する。リンクで補う。バージョン警告は出るが動作する。
- `LD_PRELOAD` の Bionic 版 `libtermux-exec.so` は glibc と非互換。
  Playwright MCP のランチャーで削除する。
- `/dev/shm` が無いため `--disable-dev-shm-usage` を必ず渡す。
