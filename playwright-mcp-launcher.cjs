#!/usr/bin/env node
/**
 * Cross-platform launcher for Playwright MCP.
 *
 * Termux/Android caveats:
 * - playwright-core throws at load time when process.platform is "android".
 *   The property is redefined to "linux" before the CLI is loaded.
 * - /etc/os-release is absent, so the host platform is pinned explicitly.
 * - The system Chromium is used instead of a downloaded Playwright browser,
 *   because Android has no usable glibc browser build.
 *
 * On desktop Linux none of the overrides are applied: Playwright resolves the
 * platform and the installed browser normally.
 */
"use strict";

const fs = require("node:fs");
const { resolveModule } = require("./mcp-paths.cjs");

const ENTRY = "@playwright/mcp/cli.js";
const TERMUX_CHROMIUM = "/data/data/com.termux/files/usr/lib/chromium/chrome";

function hasFlag(args, ...names) {
  return args.some((arg) =>
    names.some((name) => arg === name || arg.startsWith(`${name}=`)),
  );
}

const cli = resolveModule(ENTRY);
if (!cli) {
  process.stderr.write(
    `Playwright MCP not found. Install it with:\n` +
      `  npm install --prefix "$HOME/.local/share/opencode/mcp" @playwright/mcp@latest\n`,
  );
  process.exit(1);
}

if (process.platform === "android") {
  Object.defineProperty(process, "platform", {
    value: "linux",
    writable: false,
    enumerable: true,
    configurable: true,
  });

  if (!process.env.PLAYWRIGHT_HOST_PLATFORM_OVERRIDE) {
    process.env.PLAYWRIGHT_HOST_PLATFORM_OVERRIDE = "ubuntu24.04-arm64";
  }
}

const args = process.argv.slice(2);

if (fs.existsSync(TERMUX_CHROMIUM) && !hasFlag(args, "--executable-path")) {
  args.push("--no-sandbox", "--executable-path", TERMUX_CHROMIUM);
}

process.argv = [process.argv[0], process.argv[1], ...args];
require(cli);
