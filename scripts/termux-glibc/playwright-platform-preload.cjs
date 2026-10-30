"use strict";
// playwright-core は process.platform === "android" を拒否する。
// NODE_OPTIONS=--require で CLI とその子プロセスの両方に適用する。
Object.defineProperty(process, "platform", {
  value: "linux",
  writable: false,
  enumerable: true,
  configurable: true,
});
if (!process.env.PLAYWRIGHT_HOST_PLATFORM_OVERRIDE) {
  process.env.PLAYWRIGHT_HOST_PLATFORM_OVERRIDE = "ubuntu24.04-arm64";
}
