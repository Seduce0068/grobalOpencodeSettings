#!/usr/bin/env rust-script
//! ```cargo
//! [dependencies]
//! anyhow = "1.0"
//! ```

use anyhow::{Context, Result};
use std::fs;
use std::process::Command;

fn main() -> Result<()> {
    let home = std::env::var("HOME").context("HOME is not set")?;
    let repo_path = format!("{home}/OpenCodeRust");
    let target_bin = format!("{repo_path}/target/release/opencode");
    let dest_bin = format!("{home}/.local/bin/opencoderust");

    println!("1. Pulling latest OpenCodeRust from origin main...");
    let status = Command::new("git")
        .args(["-C", &repo_path, "pull", "origin", "main"])
        .status()
        .context("Failed to execute git pull")?;

    if !status.success() {
        anyhow::bail!("git pull failed");
    }

    println!("2. Building release binary with cargo...");
    let manifest_path = format!("{}/Cargo.toml", repo_path);
    let status = Command::new("cargo")
        .args([
            "build",
            "--release",
            "-p",
            "opencode-cli",
            "--manifest-path",
            &manifest_path,
        ])
        .status()
        .context("Failed to execute cargo build")?;

    if !status.success() {
        anyhow::bail!("cargo build failed");
    }

    println!("3. Installing binary to global path ({}) ...", dest_bin);
    fs::copy(&target_bin, &dest_bin).context("Failed to copy built binary to destination")?;

    println!("Success! opencoderust has been updated and installed globally.");
    Ok(())
}
