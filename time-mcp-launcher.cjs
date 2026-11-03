#!/usr/bin/env node
"use strict";

/**
 * Cross-platform launcher for time-mcp.
 *
 * time-mcp prints a startup banner to stdout, but MCP's stdio transport
 * reserves stdout for JSON-RPC. The launcher forwards only JSON lines so the
 * transport stays clean.
 *
 * Resolution order:
 *   1. TIME_MCP_ENTRY (explicit override)
 *   2. time-mcp resolved from the shared module roots in mcp-paths.cjs
 */

const fs = require("node:fs");
const { spawn } = require("node:child_process");
const readline = require("node:readline");
const { resolveModule } = require("./mcp-paths.cjs");

const candidates = [
  process.env.TIME_MCP_ENTRY,
  resolveModule("time-mcp/dist/index.js"),
].filter((candidate) => typeof candidate === "string" && candidate.length > 0);

const entry = candidates.find((candidate) => fs.existsSync(candidate));

if (!entry) {
  process.stderr.write(
    `time-mcp not found. Install it with:\n` +
      `  npm install --prefix "$HOME/.local/share/opencode/mcp" time-mcp@latest\n`,
  );
  process.exit(1);
}

const child = spawn(process.execPath, [entry, ...process.argv.slice(2)], {
  stdio: ["pipe", "pipe", "inherit"],
  env: process.env,
});

child.stdout.setEncoding("utf8");
const lines = readline.createInterface({
  input: child.stdout,
  crlfDelay: Infinity,
});

lines.on("line", (line) => {
  // MCP JSON-RPC is one message per line. Drop banners and log output.
  if (!line.trimStart().startsWith("{")) return;
  if (!process.stdout.write(`${line}\n`)) lines.pause();
});

process.stdout.on("drain", () => lines.resume());

process.stdin.pipe(child.stdin);
process.stdin.on("error", () => {});
child.stdin.on("error", () => {});

for (const signal of ["SIGINT", "SIGTERM", "SIGHUP"]) {
  process.on(signal, () => child.kill(signal));
}

child.on("error", (error) => {
  process.stderr.write(`time-mcp launcher: ${error.message}\n`);
  process.exitCode = 1;
});

child.on("close", (code, signal) => {
  process.exitCode = signal ? 1 : (code ?? 1);
});
