"use strict";

/**
 * Cross-platform module resolution for Node-based MCP servers.
 *
 * The same opencode.jsonc is shared by Termux (Android) and desktop Linux.
 * Node's absolute path and the npm global prefix differ per platform, so the
 * launchers resolve packages from a list of candidate roots instead of
 * embedding a platform-specific absolute path in the configuration.
 *
 * Resolution order:
 *   1. OPENCODE_NODE_MODULES (explicit override, kept for compatibility)
 *   2. $HOME/.local/share/opencode/mcp/node_modules (documented fixed root)
 *   3. $HOME/.config/opencode/node_modules (legacy per-config root)
 *   4. $HOME/.npm-global/lib/node_modules (npm prefix used on Kubuntu)
 *   5. /usr/local/lib/node_modules and /usr/lib/node_modules (system-wide)
 *   6. /data/data/com.termux/files/usr/lib/node_modules (Termux global)
 */

const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");

const home = process.env.HOME || os.homedir();

const moduleRoots = [
  process.env.OPENCODE_NODE_MODULES,
  path.join(home, ".local/share/opencode/mcp/node_modules"),
  path.join(home, ".config/opencode/node_modules"),
  path.join(home, ".npm-global/lib/node_modules"),
  "/usr/local/lib/node_modules",
  "/usr/lib/node_modules",
  "/data/data/com.termux/files/usr/lib/node_modules",
].filter((root) => typeof root === "string" && root.length > 0);

/**
 * Resolve a package entry point relative to one of the known module roots.
 *
 * @param {string} relativeEntry Path under node_modules, e.g.
 *   "@playwright/mcp/cli.js".
 * @returns {string | null} Absolute path when it exists, otherwise null.
 */
function resolveModule(relativeEntry) {
  for (const root of moduleRoots) {
    const candidate = path.join(root, relativeEntry);
    if (fs.existsSync(candidate)) return candidate;
  }
  return null;
}

/**
 * Spawn a Node entry point with inherited stdio and signal forwarding.
 *
 * @param {string} entry Absolute path to the entry point.
 * @param {string[]} args Arguments passed to the entry point.
 * @returns {void}
 */
function spawnNode(entry, args) {
  const { spawn } = require("node:child_process");
  const child = spawn(process.execPath, [entry, ...args], {
    stdio: "inherit",
    env: process.env,
  });

  for (const signal of ["SIGINT", "SIGTERM", "SIGHUP"]) {
    process.on(signal, () => child.kill(signal));
  }

  child.on("error", (error) => {
    process.stderr.write(`mcp launcher: ${error.message}\n`);
    process.exitCode = 1;
  });

  child.on("close", (code, signal) => {
    process.exitCode = signal ? 1 : (code ?? 1);
  });
}

module.exports = { moduleRoots, resolveModule, spawnNode };
