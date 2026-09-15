//! `statusbar` — Claude Code status line.
//!
//! Reads the session JSON piped on stdin, resolves the impure bits (git
//! branch, `$HOME`, current time), and prints the rendered bar. All
//! rendering lives in statusbar-core; this is stdin/stdout + config glue.
//!
//!   statusbar            read stdin, print the ANSI bar (default)
//!   statusbar -j         print `{"statusline": <plain>, "rendered": <ansi>}`
//!
//! Fail open: any error (bad JSON, missing config, no git) yields fewer
//! segments or an empty line, never a crash — a broken status line must
//! not disrupt the session.

mod output;

use std::io::Read;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use clap::Parser;
use claude_paths::{cache_dir, config_dir};
use output::Mode;
use serde::Serialize;
use statusbar_core::{handle_from_hosts_yml, render, strip_ansi, Config, Env, Session};

#[derive(Parser)]
#[command(
    name = "statusbar",
    about = "Render the Claude Code status line from session JSON on stdin.",
    version = env!("PLUGIN_VERSION")
)]
struct Cli {
    /// Emit a single JSON document.
    #[arg(short = 'j', long)]
    json: bool,

    /// Emit newline-delimited JSON (one object per line).
    #[arg(short = 'J', long)]
    ndjson: bool,
}

#[derive(Serialize)]
struct StatusOut {
    /// The bar with ANSI escapes stripped — stable to assert on.
    statusline: String,
    /// The bar as printed, escapes intact.
    rendered: String,
}

fn main() {
    let cli = Cli::parse();
    let mode = Mode::new(cli.json, cli.ndjson);

    let mut raw = String::new();
    let _ = std::io::stdin().read_to_string(&mut raw);
    let session = Session::parse(&raw);

    let cfg = load_config();
    let home = std::env::var("HOME").unwrap_or_default();
    let cwd = session
        .workspace
        .current_dir
        .clone()
        .or_else(|| std::env::var("PWD").ok())
        .unwrap_or_default();
    let branch = git_branch(&cwd);
    // Only touch the filesystem for the handle when the feature is on.
    let handle = cfg.branch.strip_handle.then(github_handle).flatten();
    let now_unix = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0);

    let label = load_label(
        session.session_id.as_deref(),
        session.session_name.as_deref(),
    );
    let env = Env {
        branch: branch.as_deref(),
        github_handle: handle.as_deref(),
        home: &home,
        now_unix,
        label: label.as_deref(),
    };
    let rendered = render(&session, &env, &cfg);

    if mode.structured() {
        output::emit_one(
            mode,
            &StatusOut {
                statusline: strip_ansi(&rendered),
                rendered: rendered.clone(),
            },
        );
    } else {
        // No trailing newline — Claude Code renders the line as-is.
        print!("{rendered}");
    }
}

/// A short, current label for this session, written by the `label-session`
/// Stop hook. Per-session because one machine runs many at once, and under
/// the cache dir because it is regenerable — losing it costs one refresh.
fn load_label(session_id: Option<&str>, session_name: Option<&str>) -> Option<String> {
    let id = session_id.filter(|id| {
        // It lands in a path, so refuse anything that isn't a plain id.
        !id.is_empty() && id.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
    })?;
    let path = cache_dir("statusbar").join("labels").join(id);
    let text = std::fs::read_to_string(path).ok()?;
    let mut lines = text.lines();
    let label = lines.next()?.trim();
    if label.is_empty() {
        return None;
    }
    // Line two records the generated title this label was written against. If
    // the live name has moved off it, a human renamed the session since — a
    // deliberate act, and it outranks anything we derived.
    if let (Some(recorded), Some(current)) = (lines.next().map(str::trim), session_name) {
        if !recorded.is_empty() && recorded != current {
            return None;
        }
    }
    Some(label.to_string())
}

/// Load `~/.config/claude/statusbar/config.json`, defaulting on any miss.
fn load_config() -> Config {
    let path = config_dir("statusbar").join("config.json");
    match std::fs::read_to_string(&path) {
        Ok(raw) => Config::parse(&raw),
        Err(_) => Config::default(),
    }
}

/// The `gh` CLI's logged-in GitHub handle, read from its `hosts.yml`
/// (`$XDG_CONFIG_HOME/gh` or `~/.config/gh`). A local file read, no
/// network call; `None` when `gh` is unconfigured or the file is absent.
fn github_handle() -> Option<String> {
    let base = std::env::var("XDG_CONFIG_HOME")
        .ok()
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| format!("{}/.config", std::env::var("HOME").unwrap_or_default()));
    let path = std::path::Path::new(&base).join("gh").join("hosts.yml");
    let raw = std::fs::read_to_string(path).ok()?;
    handle_from_hosts_yml(&raw)
}

/// Current branch via `git symbolic-ref`; `None` outside a repo / detached.
fn git_branch(cwd: &str) -> Option<String> {
    if cwd.is_empty() {
        return None;
    }
    let out = Command::new("git")
        .args(["-C", cwd, "symbolic-ref", "--short", "HEAD"])
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    let name = String::from_utf8_lossy(&out.stdout).trim().to_string();
    (!name.is_empty()).then_some(name)
}
