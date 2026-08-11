//! The status bar's config shape — `~/.config/claude/statusbar/config.json`.
//!
//! Every field has a default (see the `Default` impls), and each struct
//! is `#[serde(default)]`, so the config file is optional and may set
//! only the keys it cares about. Committed defaults are anonymized: the
//! personal `strip_prefixes` (`dpep/`, `~/workspace/`, …) live in the
//! user's own config file, never in this repo.

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct Config {
    /// Joins rendered segments. The classic middle-dot by default.
    pub separator: String,
    pub cwd: CwdConfig,
    pub branch: BranchConfig,
    pub pr: PrConfig,
    /// Show a dim `wt:<name>` when the worktree differs from the branch.
    pub worktree: bool,
    /// Show a dim `[name]` when the session is named.
    pub session: bool,
    pub rate_limit: RateConfig,
    pub context_window: ContextConfig,
    /// Show the model's display name, abbreviated (opt-in).
    pub model: ModelConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            separator: " · ".to_string(),
            cwd: CwdConfig::default(),
            branch: BranchConfig::default(),
            pr: PrConfig::default(),
            worktree: true,
            session: true,
            rate_limit: RateConfig::default(),
            context_window: ContextConfig::default(),
            model: ModelConfig::default(),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct CwdConfig {
    /// Prefixes trimmed off the working dir before display. A leading
    /// `~` expands to `$HOME`. First match wins; if none match, `$HOME`
    /// is collapsed to `~`.
    pub strip_prefixes: Vec<String>,
    /// When the path has more than this many components, collapse it to
    /// `…/<parent>/<current>`.
    pub collapse_depth: usize,
}

impl Default for CwdConfig {
    fn default() -> Self {
        Self {
            strip_prefixes: Vec::new(),
            collapse_depth: 3,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct BranchConfig {
    pub enabled: bool,
    /// Prefixes trimmed off the branch name (e.g. `dpep/`, `dp/`).
    pub strip_prefixes: Vec<String>,
    /// When set (default), trim a leading `<github-handle>/` off the
    /// branch, where the handle is the `gh` CLI's logged-in user. An
    /// explicit `strip_prefixes` match always wins; this only fills the
    /// gap when none matched.
    pub strip_handle: bool,
    /// Branches that render nothing (the "you're on trunk" case).
    pub hide_on: Vec<String>,
}

impl Default for BranchConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            strip_prefixes: Vec::new(),
            strip_handle: true,
            hide_on: vec!["main".to_string(), "master".to_string()],
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct PrConfig {
    /// Show `#<number>` for the session's open PR (opt-in).
    pub enabled: bool,
    /// When a PR is present, render it in the branch's slot and hide the
    /// branch. Claude Code resolves the PR from the current branch, so
    /// the two are equivalent by construction — the number is the
    /// tighter identifier.
    pub prefer_over_branch: bool,
}

impl Default for PrConfig {
    fn default() -> Self {
        Self {
            // On by default: the `pr` object rides in the session JSON
            // (Claude Code resolves it — no `gh` call here), so showing
            // `#<number>` in the branch's place costs nothing and only
            // appears when there's actually an open PR.
            enabled: true,
            prefer_over_branch: true,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct RateConfig {
    pub enabled: bool,
    /// Below this the segment is hidden entirely; at/above it renders
    /// yellow, then red at `danger_at`.
    pub warn_at: f64,
    pub danger_at: f64,
}

impl Default for RateConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            warn_at: 70.0,
            danger_at: 90.0,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct ContextConfig {
    pub enabled: bool,
    /// Below this the `ctx:` segment is hidden — a quiet early session
    /// doesn't need it; it appears as the window fills.
    pub show_at: f64,
    /// At/above this the segment turns yellow (compaction is near).
    pub warn_at: f64,
}

impl Default for ContextConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            show_at: 50.0,
            warn_at: 80.0,
        }
    }
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct ModelConfig {
    pub enabled: bool,
    /// Hide the segment when the model name is in this list — e.g. your
    /// everyday default, which isn't worth the pixels. Case-insensitive.
    pub hide: Vec<String>,
}

impl Config {
    /// Parse a config document, falling back to defaults on any error so
    /// a malformed file never breaks the bar.
    pub fn parse(raw: &str) -> Self {
        serde_json::from_str(raw).unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_are_sane() {
        let c = Config::default();
        assert_eq!(c.separator, " · ");
        assert!(c.worktree && c.session && c.rate_limit.enabled);
        assert!(c.context_window.enabled);
        assert!(c.pr.enabled && c.pr.prefer_over_branch);
        assert!(!c.model.enabled);
        assert!(c.branch.strip_handle);
        assert_eq!(c.branch.hide_on, vec!["main", "master"]);
    }

    #[test]
    fn partial_config_overrides_only_named_keys() {
        let c =
            Config::parse(r#"{"pr": {"enabled": true}, "branch": {"strip_prefixes": ["dpep/"]}}"#);
        assert!(c.pr.enabled);
        assert!(c.pr.prefer_over_branch); // still defaulted
        assert_eq!(c.branch.strip_prefixes, vec!["dpep/"]);
        // untouched keys keep their defaults
        assert_eq!(c.separator, " · ");
        assert!(c.branch.enabled);
        assert_eq!(c.branch.hide_on, vec!["main", "master"]);
    }

    #[test]
    fn malformed_config_falls_back_to_defaults() {
        let c = Config::parse("{ not valid");
        assert_eq!(c.separator, " · ");
    }
}
