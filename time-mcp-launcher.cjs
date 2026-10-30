#!/usr/bin/env node
"use strict";

/**
 * time-mcp launcher for OpenCode on Termux.
 *
 * time-mcp は起動時に "Starting Time MCP server..." を stdout へ書くが、
 * MCP stdio トランスポートでは stdout は JSON-RPC 専用。非 JSON 行を
 * 除去してからクライアントへ中継する。
 *
 * 解決順:
 *   1. TIME_MCP_ENTRY (opencode.jsonc から渡す)
 *   2. OPENCODE_NODE_MODULES/time-mcp/dist/index.js
 *   3. Termux のグローバル node_modules
 */

const { spawn } = require("node:child_process");
const path = require("node:path");
const readline = require("node:readline");

const nodeModules =
  process.env.OPENCODE_NODE_MODULES ||
  "/data/data/com.termux/files/usr/lib/node_modules";
const entry =
  process.env.TIME_MCP_ENTRY ||
  path.join(nodeModules, "time-mcp", "dist", "index.js");

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
  // MCP の JSON-RPC は 1 行 1 メッセージ。バナーやログは転送しない。
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
