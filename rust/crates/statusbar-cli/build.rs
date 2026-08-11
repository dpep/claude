//! Stamp the *plugin* version into the binary.
//!
//! Crate versions here are all `0.0.1` and meaningless; the version users
//! and hooks care about lives in `plugins/<name>/.claude-plugin/plugin.json`.
//! Reading it at compile time keeps one source of truth and lets
//! `check-installed.sh` tell a stale binary from a current one — the skew
//! that let a `deny` rule reach a binary too old to understand it.
//!
//! Falls back to the crate version if the manifest can't be read: a build
//! must never fail over a version string.

use std::path::PathBuf;

const PLUGIN: &str = "statusbar";

fn main() {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../../plugins")
        .join(PLUGIN)
        .join(".claude-plugin/plugin.json");

    println!("cargo::rerun-if-changed={}", manifest.display());

    let version = std::fs::read_to_string(&manifest)
        .ok()
        .and_then(|s| parse_version(&s))
        .unwrap_or_else(|| env!("CARGO_PKG_VERSION").to_string());

    println!("cargo::rustc-env=PLUGIN_VERSION={version}");
}

/// Pull `"version": "x.y.z"` out of the manifest without a JSON dep —
/// build scripts should stay dependency-free.
fn parse_version(s: &str) -> Option<String> {
    let after = s.split("\"version\"").nth(1)?;
    let after = after.split_once(':')?.1;
    let start = after.find('"')? + 1;
    let rest = &after[start..];
    let end = rest.find('"')?;
    Some(rest[..end].to_string())
}
