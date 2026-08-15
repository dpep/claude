//! The pure render: session JSON + `Env` + config → the ANSI status line.
//!
//! Segment order, left to right:
//!   cwd · wt:<worktree> · <ref> · #<pr> · [session] · <model> · ctx:N% · rate:N% (Nm)
//!
//! `<ref>` is the branch, or the PR number when `pr.prefer_over_branch`
//! is set and a PR is present. Each segment is independently gated by
//! config and by whether its data exists; absent segments don't join in.

use crate::config::{BranchConfig, Config, CwdConfig};
use crate::input::{Pr, Session};

// ANSI SGR codes. Kept literal (not a dependency) — the bash original
// used the same escapes and Claude Code renders them verbatim.
const CYAN: &str = "\x1b[36m";
const GREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";
const RED: &str = "\x1b[31m";
const DIM: &str = "\x1b[2m";
const RESET: &str = "\x1b[0m";

/// The impure inputs the CLI resolves and hands to the render: the git
/// branch (from `git symbolic-ref`), `$HOME`, and the current time.
pub struct Env<'a> {
    pub branch: Option<&'a str>,
    /// The `gh` CLI's logged-in user, used to strip a `<handle>/` branch
    /// prefix (see `branch.strip_handle`). `None` when `gh` is unconfigured.
    pub github_handle: Option<&'a str>,
    pub home: &'a str,
    pub now_unix: i64,
}

/// Render the full status line. Never panics; missing data yields fewer
/// segments, not an error.
pub fn render(s: &Session, env: &Env, cfg: &Config) -> String {
    let mut parts: Vec<String> = Vec::new();

    // Working directory (cyan).
    if let Some(dir) = s.workspace.current_dir.as_deref() {
        if !dir.is_empty() {
            parts.push(format!(
                "{CYAN}{}{RESET}",
                cwd_display(dir, env.home, &cfg.cwd)
            ));
        }
    }

    // Worktree (dim) — only when it differs from the branch.
    if cfg.worktree {
        if let Some(wt) = s.workspace.git_worktree.as_deref() {
            if !wt.is_empty() && Some(wt) != env.branch {
                parts.push(format!("{DIM}wt:{wt}{RESET}"));
            }
        }
    }

    // Ref slot: the PR (when preferred) stands in for the branch, since
    // Claude Code derives the PR from the current branch.
    let pr = s.pr.as_ref().filter(|p| p.number > 0 && cfg.pr.enabled);
    let pr_in_ref = pr.is_some() && cfg.pr.prefer_over_branch;

    if !pr_in_ref && cfg.branch.enabled {
        if let Some(b) = env.branch {
            if !b.is_empty() && !cfg.branch.hide_on.iter().any(|h| h == b) {
                parts.push(branch_display(b, &cfg.branch, env.github_handle).to_string());
            }
        }
    }
    if let Some(pr) = pr {
        parts.push(pr_segment(pr));
    }

    // Session name (dim).
    if cfg.session {
        if let Some(name) = s.session_name.as_deref() {
            if !name.is_empty() {
                parts.push(format!("{DIM}[{name}]{RESET}"));
            }
        }
    }

    // Model display name (dim, opt-in), hidden when it's an everyday default.
    if cfg.model.enabled {
        if let Some(name) = s.model.as_ref().and_then(|m| m.display_name.as_deref()) {
            let hidden = cfg.model.hide.iter().any(|h| h.eq_ignore_ascii_case(name));
            if !name.is_empty() && !hidden {
                parts.push(format!("{DIM}{name}{RESET}"));
            }
        }
    }

    // Context-window usage — dim, yellow past the warn threshold, hidden
    // until it crosses `show_at`.
    if cfg.context_window.enabled {
        if let Some(cw) = &s.context_window {
            let pct = cw.used_percentage;
            if pct >= cfg.context_window.show_at {
                let color = if pct >= cfg.context_window.warn_at {
                    YELLOW
                } else {
                    DIM
                };
                parts.push(format!("{color}ctx:{}%{RESET}", pct.round() as i64));
            }
        }
    }

    // Rate limit (5-hour window) — hidden below warn, yellow, then red.
    if cfg.rate_limit.enabled {
        if let Some(w) = s.rate_limits.as_ref().and_then(|r| r.five_hour.as_ref()) {
            let pct = w.used_percentage;
            if pct >= cfg.rate_limit.warn_at {
                let color = if pct >= cfg.rate_limit.danger_at {
                    RED
                } else {
                    YELLOW
                };
                let mut seg = format!("{color}rate:{}%{RESET}", pct.round() as i64);
                if w.resets_at > env.now_unix {
                    let mins = (w.resets_at - env.now_unix) / 60;
                    if mins > 0 {
                        seg.push_str(&format!(" {DIM}({mins}m){RESET}"));
                    }
                }
                parts.push(seg);
            }
        }
    }

    parts.join(&cfg.separator)
}

/// `#<number>`, colored by review state: green when approved, red when
/// changes are requested, dim otherwise.
fn pr_segment(pr: &Pr) -> String {
    let color = match pr.review_state.as_deref() {
        Some("approved") => GREEN,
        Some("changes_requested") => RED,
        _ => DIM,
    };
    format!("{color}#{}{RESET}", pr.number)
}

/// Trim configured prefixes off the working dir (a leading `~` in a
/// prefix expands to `$HOME`), else collapse a leading `$HOME` to `~`,
/// then collapse deep paths to `…/<parent>/<current>`.
fn cwd_display(dir: &str, home: &str, cfg: &CwdConfig) -> String {
    let mut d = dir.to_string();
    let mut stripped = false;

    for prefix in &cfg.strip_prefixes {
        let expanded = expand_tilde(prefix, home);
        if let Some(rest) = d.strip_prefix(&expanded) {
            d = rest.trim_start_matches('/').to_string();
            stripped = true;
            break;
        }
    }

    if !stripped && !home.is_empty() {
        if let Some(rest) = d.strip_prefix(home) {
            d = format!("~{rest}");
        }
    }

    collapse(&d, cfg.collapse_depth)
}

/// Collapse a path to its last two components when it runs deeper than
/// `depth` components; otherwise leave it as-is.
fn collapse(path: &str, depth: usize) -> String {
    let comps: Vec<&str> = path.split('/').filter(|c| !c.is_empty()).collect();
    if comps.len() > depth && comps.len() >= 2 {
        let n = comps.len();
        format!("…/{}/{}", comps[n - 2], comps[n - 1])
    } else {
        path.trim_end_matches('/').to_string()
    }
}

fn expand_tilde(prefix: &str, home: &str) -> String {
    if let Some(rest) = prefix.strip_prefix('~') {
        format!("{home}{rest}")
    } else {
        prefix.to_string()
    }
}

/// The branch name with a leading prefix trimmed. An explicit
/// `strip_prefixes` match wins; when none matches and `strip_handle` is
/// on, a `<github-handle>/` prefix (derived from `gh`) is trimmed instead.
fn branch_display<'a>(branch: &'a str, cfg: &BranchConfig, handle: Option<&str>) -> &'a str {
    for p in &cfg.strip_prefixes {
        if let Some(rest) = branch.strip_prefix(p) {
            return rest;
        }
    }
    if cfg.strip_handle {
        if let Some(h) = handle.filter(|h| !h.is_empty()) {
            if let Some(rest) = branch.strip_prefix(h).and_then(|r| r.strip_prefix('/')) {
                return rest;
            }
        }
    }
    branch
}

/// Strip ANSI SGR escape sequences — used for `--json` output and tests.
/// Char-based so multi-byte separators (`·`) survive intact.
pub fn strip_ansi(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\x1b' && chars.peek() == Some(&'[') {
            chars.next(); // consume '['
            for cc in chars.by_ref() {
                if cc == 'm' {
                    break; // end of the SGR sequence
                }
            }
        } else {
            out.push(c);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn env<'a>(branch: Option<&'a str>) -> Env<'a> {
        Env {
            branch,
            github_handle: None,
            home: "/home/x",
            now_unix: 1_000,
        }
    }

    fn env_with_handle<'a>(branch: Option<&'a str>, handle: &'a str) -> Env<'a> {
        Env {
            github_handle: Some(handle),
            ..env(branch)
        }
    }

    fn plain(s: &Session, e: &Env, c: &Config) -> String {
        strip_ansi(&render(s, e, c))
    }

    #[test]
    fn cwd_only_on_trunk() {
        let s = Session::parse(r#"{"workspace": {"current_dir": "/home/x/projects"}}"#);
        let out = plain(&s, &env(Some("main")), &Config::default());
        assert_eq!(out, "~/projects");
    }

    #[test]
    fn home_collapses_to_tilde_and_deep_path_shortens() {
        let s = Session::parse(r#"{"workspace": {"current_dir": "/home/x/code/lib/claude"}}"#);
        let out = plain(&s, &env(None), &Config::default());
        assert_eq!(out, "…/lib/claude");
    }

    #[test]
    fn strip_prefix_then_collapse() {
        let cfg =
            Config::parse(r#"{"cwd": {"strip_prefixes": ["~/workspace/"], "collapse_depth": 2}}"#);
        let s =
            Session::parse(r#"{"workspace": {"current_dir": "/home/x/workspace/widget/api/v2"}}"#);
        let out = plain(&s, &env(None), &cfg);
        assert_eq!(out, "…/api/v2");
    }

    #[test]
    fn branch_shown_with_prefix_trimmed() {
        let cfg = Config::parse(r#"{"branch": {"strip_prefixes": ["dpep/", "dp/"]}}"#);
        let s = Session::parse(r#"{"workspace": {"current_dir": "/home/x/projects"}}"#);
        let out = plain(&s, &env(Some("dpep/example-api")), &cfg);
        assert_eq!(out, "~/projects · example-api");
    }

    #[test]
    fn handle_stripped_from_branch_by_default() {
        // strip_handle defaults on; the derived handle trims "<handle>/".
        let s = Session::parse(r#"{"workspace": {"current_dir": "/home/x/projects"}}"#);
        let out = plain(
            &s,
            &env_with_handle(Some("dpep/example-api"), "dpep"),
            &Config::default(),
        );
        assert_eq!(out, "~/projects · example-api");
    }

    #[test]
    fn handle_not_stripped_when_disabled() {
        let cfg = Config::parse(r#"{"branch": {"strip_handle": false}}"#);
        let s = Session::parse(r#"{"workspace": {"current_dir": "/tmp/x"}}"#);
        let out = plain(&s, &env_with_handle(Some("dpep/feat"), "dpep"), &cfg);
        assert_eq!(out, "/tmp/x · dpep/feat");
    }

    #[test]
    fn explicit_prefix_wins_over_handle() {
        // strip_prefixes matches first; the handle isn't consulted.
        let cfg = Config::parse(r#"{"branch": {"strip_prefixes": ["feature/"]}}"#);
        let s = Session::parse(r#"{"workspace": {"current_dir": "/tmp/x"}}"#);
        let out = plain(&s, &env_with_handle(Some("feature/x"), "dpep"), &cfg);
        assert_eq!(out, "/tmp/x · x");
    }

    #[test]
    fn handle_only_strips_when_it_is_the_prefix() {
        // A branch that merely contains the handle mid-name is untouched.
        let s = Session::parse(r#"{"workspace": {"current_dir": "/tmp/x"}}"#);
        let out = plain(
            &s,
            &env_with_handle(Some("dpepper-feat"), "dpep"),
            &Config::default(),
        );
        assert_eq!(out, "/tmp/x · dpepper-feat");
    }

    #[test]
    fn no_handle_leaves_branch_intact() {
        let s = Session::parse(r#"{"workspace": {"current_dir": "/tmp/x"}}"#);
        let out = plain(&s, &env(Some("dpep/feat")), &Config::default());
        assert_eq!(out, "/tmp/x · dpep/feat");
    }

    #[test]
    fn branch_hidden_on_master() {
        let s = Session::parse(r#"{"workspace": {"current_dir": "/tmp/x"}}"#);
        let out = plain(&s, &env(Some("master")), &Config::default());
        assert_eq!(out, "/tmp/x");
    }

    #[test]
    fn worktree_shown_only_when_differs_from_branch() {
        let s = Session::parse(
            r#"{"workspace": {"current_dir": "/tmp/x", "git_worktree": "feature"}}"#,
        );
        assert_eq!(
            plain(&s, &env(Some("main")), &Config::default()),
            "/tmp/x · wt:feature"
        );
        assert_eq!(
            plain(&s, &env(Some("feature")), &Config::default()),
            "/tmp/x · feature"
        );
    }

    #[test]
    fn rate_limit_hidden_below_threshold() {
        let s = Session::parse(
            r#"{"workspace": {"current_dir": "/tmp/x"}, "rate_limits": {"five_hour": {"used_percentage": 50}}}"#,
        );
        assert_eq!(plain(&s, &env(Some("main")), &Config::default()), "/tmp/x");
    }

    #[test]
    fn rate_limit_warn_with_countdown() {
        let s = Session::parse(
            r#"{"workspace": {"current_dir": "/tmp/x"}, "rate_limits": {"five_hour": {"used_percentage": 82.4, "resets_at": 4600}}}"#,
        );
        assert_eq!(
            plain(&s, &env(Some("main")), &Config::default()),
            "/tmp/x · rate:82% (60m)"
        );
    }

    #[test]
    fn rate_limit_no_countdown_when_reset_passed() {
        let s = Session::parse(
            r#"{"workspace": {"current_dir": "/tmp/x"}, "rate_limits": {"five_hour": {"used_percentage": 95, "resets_at": 500}}}"#,
        );
        assert_eq!(
            plain(&s, &env(Some("main")), &Config::default()),
            "/tmp/x · rate:95%"
        );
    }

    #[test]
    fn context_shown_by_default_past_floor() {
        let s = Session::parse(
            r#"{"workspace": {"current_dir": "/tmp/x"}, "context_window": {"used_percentage": 62}}"#,
        );
        assert_eq!(
            plain(&s, &env(Some("main")), &Config::default()),
            "/tmp/x · ctx:62%"
        );
    }

    #[test]
    fn context_hidden_below_floor() {
        let s = Session::parse(
            r#"{"workspace": {"current_dir": "/tmp/x"}, "context_window": {"used_percentage": 20}}"#,
        );
        assert_eq!(plain(&s, &env(Some("main")), &Config::default()), "/tmp/x");
    }

    #[test]
    fn branch_shown_when_pr_disabled() {
        let cfg = Config::parse(r#"{"pr": {"enabled": false}}"#);
        let s = Session::parse(r#"{"workspace": {"current_dir": "/tmp/x"}, "pr": {"number": 5}}"#);
        assert_eq!(plain(&s, &env(Some("feat")), &cfg), "/tmp/x · feat");
    }

    #[test]
    fn branch_shown_when_no_pr_present() {
        let s = Session::parse(r#"{"workspace": {"current_dir": "/tmp/x"}}"#);
        assert_eq!(
            plain(&s, &env(Some("feat")), &Config::default()),
            "/tmp/x · feat"
        );
    }

    #[test]
    fn pr_replaces_branch_by_default() {
        // prefer_over_branch defaults true → branch "feat" suppressed
        let s = Session::parse(
            r#"{"workspace": {"current_dir": "/tmp/x"}, "pr": {"number": 474, "review_state": "approved"}}"#,
        );
        assert_eq!(
            plain(&s, &env(Some("feat")), &Config::default()),
            "/tmp/x · #474"
        );
    }

    #[test]
    fn pr_alongside_branch_when_not_preferred() {
        let cfg = Config::parse(r#"{"pr": {"enabled": true, "prefer_over_branch": false}}"#);
        let s =
            Session::parse(r#"{"workspace": {"current_dir": "/tmp/x"}, "pr": {"number": 474}}"#);
        assert_eq!(plain(&s, &env(Some("feat")), &cfg), "/tmp/x · feat · #474");
    }

    #[test]
    fn model_hidden_when_in_hide_list() {
        let cfg = Config::parse(r#"{"model": {"enabled": true, "hide": ["opus"]}}"#);
        let default_model = Session::parse(
            r#"{"workspace": {"current_dir": "/tmp/x"}, "model": {"display_name": "Opus"}}"#,
        );
        assert_eq!(plain(&default_model, &env(Some("main")), &cfg), "/tmp/x");
        let other = Session::parse(
            r#"{"workspace": {"current_dir": "/tmp/x"}, "model": {"display_name": "Haiku"}}"#,
        );
        assert_eq!(plain(&other, &env(Some("main")), &cfg), "/tmp/x · Haiku");
    }

    #[test]
    fn full_bar_with_all_segments() {
        let cfg = Config::parse(
            r#"{"pr": {"enabled": true, "prefer_over_branch": false}, "model": {"enabled": true}}"#,
        );
        let s = Session::parse(
            r#"{
              "workspace": {"current_dir": "/home/x/projects", "git_worktree": "wt-a"},
              "session_name": "rv",
              "pr": {"number": 474, "review_state": "changes_requested"},
              "model": {"display_name": "Opus"},
              "context_window": {"used_percentage": 85},
              "rate_limits": {"five_hour": {"used_percentage": 92, "resets_at": 2200}}
            }"#,
        );
        assert_eq!(
            plain(&s, &env(Some("feat")), &cfg),
            "~/projects · wt:wt-a · feat · #474 · [rv] · Opus · ctx:85% · rate:92% (20m)"
        );
    }

    #[test]
    fn strip_ansi_removes_color_codes() {
        assert_eq!(strip_ansi("\x1b[36mhi\x1b[0m"), "hi");
    }
}
