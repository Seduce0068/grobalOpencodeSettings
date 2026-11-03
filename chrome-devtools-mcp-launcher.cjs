#!/usr/bin/env node
/**
 * Cross-platform launcher for chrome-devtools-mcp.
 *
 * The package is resolved from mcp-paths.cjs instead of a hard-coded npm
 * prefix, so the same configuration works on Termux and desktop Linux.
 *
 * On Termux/Android the MCP cannot download a usable Chrome build, so the
 * system Chromium is selected explicitly and launched with the flags Android
 * requires (no sandbox, no /dev/shm). On desktop Linux the executable is left
 * to chrome-devtools-mcp's normal Chrome discovery.
 */
"use strict";

const fs = require("node:fs");
const { resolveModule, spawnNode } = require("./mcp-paths.cjs");

const ENTRY = "chrome-devtools-mcp/build/src/bin/chrome-devtools-mcp.js";
const TERMUX_CHROMIUM = "/data/data/com.termux/files/usr/lib/chromium/chrome";

function hasFlag(args, ...names) {
  return args.some((arg) =>
    names.some((name) => arg === name || arg.startsWith(`${name}=`)),
  );
}

const entry = resolveModule(ENTRY);
if (!entry) {
  process.stderr.write(
    `chrome-devtools-mcp not found. Install it with:\n` +
      `  npm install --prefix "$HOME/.local/share/opencode/mcp" chrome-devtools-mcp@latest\n`,
  );
  process.exit(1);
}

const args = process.argv.slice(2);

if (fs.existsSync(TERMUX_CHROMIUM) && !hasFlag(args, "--executablePath", "--executable-path")) {
  args.push(
    "--executablePath",
    TERMUX_CHROMIUM,
    "--chromeArg=--no-sandbox",
    "--chromeArg=--disable-setuid-sandbox",
    "--chromeArg=--disable-dev-shm-usage",
  );
}

spawnNode(entry, args);
