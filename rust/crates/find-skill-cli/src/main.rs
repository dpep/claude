//! `find-skill` — locate Claude Code skills by fuzzy match and print
//! where they live so you can open or edit them. A skill is a
//! `<name>/SKILL.md` or a `commands/*.md`, the way Claude Code counts
//! them — a loose `~/.claude/skills/*.md` is neither.
//!
//! Flat CLI (agent-friendly, no subcommand tree):
//!   find-skill [PATTERN]      fuzzy match over name + description
//!   find-skill -1 PATTERN     print only the best match's path
//!   find-skill -a PATTERN     list every copy (don't collapse to best)
//!   find-skill --add DIR      register a repo/workspace search path
//!   find-skill --remove DIR   unregister a search path
//!   find-skill --paths        show search paths + discovered repos
//!   find-skill --refresh      force re-discovery of local repos
//!   find-skill --reset        delete config + cache (clean slate)
//! `-j/--json` and `-J/--ndjson` work on every one of them.

mod output;

use clap::Parser;
use find_skill_core::{add_path, find, paths, remove_path, Env, RepoRoot, Skill, Source};
use output::Mode;
use serde::Serialize;
use std::path::{Path, PathBuf};

#[derive(Parser)]
#[command(
    name = "find-skill",
    about = "Fuzzy-find Claude Code skills and print where each one lives.",
    version
)]
struct Cli {
    /// Fuzzy pattern matched against skill name + description
    pattern: Option<String>,

    /// Print only the best match's path (feed it to $EDITOR or Read)
    #[arg(short = '1', long)]
    first: bool,

    /// List every copy of each skill (personal, local repo, installed)
    #[arg(short = 'a', long)]
    all: bool,

    /// Register a repo (or workspace of repos) to search
    #[arg(long, value_name = "DIR")]
    add: Option<PathBuf>,

    /// Unregister a search path
    #[arg(long, value_name = "DIR")]
    remove: Option<PathBuf>,

    /// Show registered search paths and discovered repos
    #[arg(long)]
    paths: bool,

    /// Re-discover local repos, ignoring the cache
    #[arg(long)]
    refresh: bool,

    /// Delete config + discovery cache (clean slate)
    #[arg(long)]
    reset: bool,

    /// Emit a single JSON document
    #[arg(short = 'j', long)]
    json: bool,

    /// Emit newline-delimited JSON (one object per line)
    #[arg(short = 'J', long, conflicts_with = "json")]
    ndjson: bool,
}

fn main() {
    let cli = Cli::parse();
    let mode = Mode::new(cli.json, cli.ndjson);
    let env = Env::real();
    let code = run(&cli, &env, mode);
    std::process::exit(code);
}

fn run(cli: &Cli, env: &Env, mode: Mode) -> i32 {
    if cli.reset {
        return act_reset(env, mode);
    }
    if let Some(dir) = &cli.add {
        return act_change(add_path(env, dir), dir, "added", "already registered", mode);
    }
    if let Some(dir) = &cli.remove {
        return act_change(
            remove_path(env, dir),
            dir,
            "removed",
            "not registered",
            mode,
        );
    }
    if cli.paths || (cli.refresh && cli.pattern.is_none() && !cli.first) {
        return act_paths(env, cli.refresh, mode);
    }
    act_find(cli, env, mode)
}

// ---- find ----------------------------------------------------------

fn act_find(cli: &Cli, env: &Env, mode: Mode) -> i32 {
    let query = cli.pattern.as_deref().unwrap_or("");
    let skills = find(env, query, cli.refresh, cli.all);

    if cli.first {
        return match skills.first() {
            Some(s) if mode.structured() => {
                output::emit_one(mode, s);
                0
            }
            Some(s) => {
                println!("{}", s.path.display());
                0
            }
            None => 1,
        };
    }

    if mode.structured() {
        output::emit_list(mode, &skills);
    } else {
        render_find(&skills, query.trim().is_empty() || cli.all);
    }
    if skills.is_empty() {
        1
    } else {
        0
    }
}

fn render_find(skills: &[Skill], listing_all: bool) {
    if skills.is_empty() {
        return;
    }
    let width = skills
        .iter()
        .map(|s| s.label.chars().count())
        .max()
        .unwrap_or(0);

    if listing_all {
        for (title, want) in [
            ("personal", Category::Personal),
            ("local repos", Category::Repo),
            ("installed", Category::Installed),
        ] {
            let group: Vec<&Skill> = skills.iter().filter(|s| category(s) == want).collect();
            if group.is_empty() {
                continue;
            }
            println!("{title}:");
            for s in group {
                println!("  {:<width$}  {}{}", s.label, location(s), marker(s));
            }
            println!();
        }
    } else {
        for s in skills {
            println!("{:<width$}  {}{}", s.label, location(s), marker(s));
        }
    }
    render_disabled_note(skills);
}

fn marker(s: &Skill) -> &'static str {
    if s.disabled() {
        "  (disabled)"
    } else {
        ""
    }
}

/// How many plugin keys the not-enabled note spells out before it just
/// counts the rest — a bare listing can turn up dozens.
const NOTE_LIMIT: usize = 3;

/// A disabled plugin's skills are on disk but can't be invoked — say so
/// once, with the exact edit that fixes it.
fn render_disabled_note(skills: &[Skill]) {
    let mut refs: Vec<&str> = skills
        .iter()
        .filter(|s| s.disabled())
        .filter_map(|s| s.plugin_ref.as_deref())
        .collect();
    refs.sort_unstable();
    refs.dedup();
    if refs.is_empty() {
        return;
    }
    println!();
    println!("note: a (disabled) skill is on disk but can't be invoked. Add it to");
    println!("      enabledPlugins in settings.json (~/.claude, or the project's");
    println!("      .claude — whichever turned it off), then start a new session:");
    for r in refs.iter().take(NOTE_LIMIT) {
        println!("        \"{r}\": true");
    }
    if let Some(rest) = refs.len().checked_sub(NOTE_LIMIT).filter(|n| *n > 0) {
        println!("        (+{rest} more, each marked above)");
    }
}

#[derive(PartialEq, Eq)]
enum Category {
    Personal,
    Repo,
    Installed,
}

fn category(s: &Skill) -> Category {
    match s.source {
        Source::Personal => Category::Personal,
        Source::Repo { .. } => Category::Repo,
        Source::Installed { .. } => Category::Installed,
    }
}

/// The location column: an editable local path (collapsed to `~`),
/// else the GitHub link, else the installed path.
fn location(s: &Skill) -> String {
    if s.editable {
        collapse_home(&s.path)
    } else if let Some(url) = &s.remote_url {
        url.clone()
    } else {
        collapse_home(&s.path)
    }
}

// ---- --add / --remove ---------------------------------------------

#[derive(Serialize)]
struct ChangeResult<'a> {
    ok: bool,
    changed: bool,
    path: &'a Path,
    roots: Vec<RepoRoot>,
}

fn act_change(
    result: anyhow::Result<find_skill_core::PathChange>,
    dir: &Path,
    verb_done: &str,
    verb_noop: &str,
    mode: Mode,
) -> i32 {
    let change = match result {
        Ok(c) => c,
        Err(e) => {
            eprintln!("find-skill: {e}");
            return 1;
        }
    };
    if mode.structured() {
        output::emit_one(
            mode,
            &ChangeResult {
                ok: true,
                changed: change.changed,
                path: dir,
                roots: change.roots,
            },
        );
    } else {
        if change.changed {
            println!("{verb_done}: {}", collapse_home(dir));
        } else {
            println!("{verb_noop}: {}", collapse_home(dir));
        }
        println!("{} repo(s) with skills in search space", change.roots.len());
    }
    0
}

// ---- --paths -------------------------------------------------------

#[derive(Serialize)]
struct PathsResult {
    search_paths: Vec<PathBuf>,
    roots: Vec<RepoRoot>,
}

fn act_paths(env: &Env, refresh: bool, mode: Mode) -> i32 {
    let (search_paths, roots) = paths(env, refresh);
    if mode.structured() {
        output::emit_one(
            mode,
            &PathsResult {
                search_paths,
                roots,
            },
        );
    } else {
        println!("search paths:");
        if search_paths.is_empty() {
            println!("  (none — add one with `find-skill --add <dir>`)");
        }
        for p in &search_paths {
            println!("  {}", collapse_home(p));
        }
        println!();
        println!("discovered repos:");
        for r in &roots {
            println!(
                "  {}  {}",
                collapse_home(&r.path),
                r.remote.as_deref().unwrap_or("-")
            );
        }
    }
    0
}

// ---- --reset -------------------------------------------------------

#[derive(Serialize)]
struct ResetResult {
    ok: bool,
    removed: Vec<PathBuf>,
}

fn act_reset(env: &Env, mode: Mode) -> i32 {
    let removed = find_skill_core::reset(env);
    if mode.structured() {
        output::emit_one(mode, &ResetResult { ok: true, removed });
    } else if removed.is_empty() {
        println!("nothing to reset");
    } else {
        for p in &removed {
            println!("removed {}", collapse_home(p));
        }
    }
    0
}

// ---- helpers -------------------------------------------------------

fn collapse_home(path: &Path) -> String {
    let s = path.to_string_lossy().to_string();
    if let Ok(home) = std::env::var("HOME") {
        if !home.is_empty() {
            if let Some(rest) = s.strip_prefix(&home) {
                return format!("~{rest}");
            }
        }
    }
    s
}
