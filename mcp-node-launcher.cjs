#!/usr/bin/env node
/**
 * Generic launcher for Node-based MCP servers without platform-specific
 * requirements. The package entry is resolved from the shared module roots
 * in mcp-paths.cjs, so the configuration does not embed an npm prefix.
 *
 * Usage:
 *   node mcp-node-launcher.cjs <entry-under-node_modules> [args...]
 */
"use strict";

const { resolveModule, spawnNode } = require("./mcp-paths.cjs");

const [relativeEntry, ...args] = process.argv.slice(2);

if (!relativeEntry) {
  process.stderr.write(
    "usage: node mcp-node-launcher.cjs <entry-under-node_modules> [args...]\n",
  );
  process.exit(1);
}

const entry = resolveModule(relativeEntry);
if (!entry) {
  process.stderr.write(
    `MCP module not found: ${relativeEntry}\n` +
      `Install it under $HOME/.local/share/opencode/mcp/node_modules or set OPENCODE_NODE_MODULES.\n`,
  );
  process.exit(1);
}

spawnNode(entry, args);
