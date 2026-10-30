#!/usr/bin/env node
/**
 * Termux/Android 向け Playwright MCP ランチャー。
 *
 * playwright-core は process.platform === "android" を未サポートとして
 * 読み込み時に例外を投げる。Node の process.platform は configurable な
 * データプロパティなので、linux として提示してから CLI を読み込む。
 *
 * 併せて PLAYWRIGHT_HOST_PLATFORM_OVERRIDE を ubuntu24.04-arm64 に固定し、
 * /etc/os-release を持たない Termux でも hostPlatform 判定を安定させる。
 * ブラウザは --executable-path で Termux の Chromium を明示する前提。
 */
"use strict";

Object.defineProperty(process, "platform", {
  value: "linux",
  writable: false,
  enumerable: true,
  configurable: true,
});

if (!process.env.PLAYWRIGHT_HOST_PLATFORM_OVERRIDE) {
  process.env.PLAYWRIGHT_HOST_PLATFORM_OVERRIDE = "ubuntu24.04-arm64";
}

// glibc 版 Chromium は Bionic 用の LD_PRELOAD を受け付けない。
// Node.js の起動後なので、ここで外しても MCP サーバー自体には影響しない。
delete process.env.LD_PRELOAD;

const path = require("path");
const bases = [
  process.env.OPENCODE_NODE_MODULES,
  path.join(path.dirname(process.execPath), "..", "lib", "node_modules"),
  path.join(process.env.HOME || "", ".config/opencode/node_modules"),
].filter(Boolean);

let cli;
for (const base of bases) {
  const candidate = path.join(base, "@playwright/mcp/cli.js");
  try {
    require.resolve(candidate);
    cli = candidate;
    break;
  } catch {
    // 次の候補へ
  }
}

if (!cli) {
  console.error(
    "Playwright MCP not found. Install it with: npm i -g @playwright/mcp",
  );
  process.exit(1);
}

require(cli);
