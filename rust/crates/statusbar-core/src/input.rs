//! The session JSON Claude Code pipes to a statusLine command on stdin.
//!
//! We model only the fields the status bar renders. Every struct is
//! `#[serde(default)]` and every field optional, so a stripped-down or
//! future-extended payload still deserializes cleanly — the plugin must
//! never break a session over an unexpected shape.

use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct Session {
    pub workspace: Workspace,
    pub session_name: Option<String>,
    pub rate_limits: Option<RateLimits>,
    pub context_window: Option<ContextWindow>,
    pub pr: Option<Pr>,
    pub model: Option<Model>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct Workspace {
    pub current_dir: Option<String>,
    pub git_worktree: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct RateLimits {
    pub five_hour: Option<Window>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct Window {
    pub used_percentage: f64,
    /// Unix epoch seconds when the window resets.
    pub resets_at: i64,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct ContextWindow {
    pub used_percentage: f64,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct Pr {
    pub number: i64,
    /// e.g. "approved", "changes_requested", "pending".
    pub review_state: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct Model {
    pub display_name: Option<String>,
}

impl Session {
    /// Parse the piped payload; on any error, return an empty session so
    /// the caller renders nothing rather than crashing the status line.
    pub fn parse(raw: &str) -> Self {
        serde_json::from_str(raw).unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_full_payload() {
        let s = Session::parse(
            r#"{
              "workspace": {"current_dir": "/home/x/rewild/thicket", "git_worktree": "wt-a"},
              "session_name": "review",
              "rate_limits": {"five_hour": {"used_percentage": 82.4, "resets_at": 1000}},
              "context_window": {"used_percentage": 55.0},
              "pr": {"number": 474, "review_state": "approved"},
              "model": {"display_name": "Opus"}
            }"#,
        );
        assert_eq!(
            s.workspace.current_dir.as_deref(),
            Some("/home/x/rewild/thicket")
        );
        assert_eq!(s.session_name.as_deref(), Some("review"));
        assert_eq!(s.pr.unwrap().number, 474);
        assert_eq!(s.model.unwrap().display_name.as_deref(), Some("Opus"));
    }

    #[test]
    fn missing_fields_default_and_dont_error() {
        let s = Session::parse(r#"{"workspace": {"current_dir": "/tmp"}}"#);
        assert_eq!(s.workspace.current_dir.as_deref(), Some("/tmp"));
        assert!(s.session_name.is_none());
        assert!(s.rate_limits.is_none());
    }

    #[test]
    fn garbage_yields_empty_session() {
        let s = Session::parse("not json at all");
        assert!(s.workspace.current_dir.is_none());
    }

    #[test]
    fn ignores_unknown_fields() {
        let s = Session::parse(r#"{"version": "2.1.90", "vim": {"mode": "NORMAL"}}"#);
        assert!(s.workspace.current_dir.is_none());
    }
}
