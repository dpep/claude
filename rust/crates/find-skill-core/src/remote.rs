//! Read a repo's `origin` remote from `.git/config` and normalize it
//! to a canonical `owner/repo` slug. This is the bridge key that ties
//! an installed marketplace skill back to your local working checkout.
//!
//! Pure file parsing — no `git` subprocess, so it stays fast and
//! testable.

use std::path::Path;

/// Return the `owner/repo` slug for the `origin` remote of the git repo
/// rooted at `dir`, or `None` if there's no `.git/config` / origin.
pub fn origin_slug(dir: &Path) -> Option<String> {
    let cfg = std::fs::read_to_string(dir.join(".git").join("config")).ok()?;
    let mut in_origin = false;
    for line in cfg.lines() {
        let t = line.trim();
        if t.starts_with('[') {
            in_origin = t == "[remote \"origin\"]";
            continue;
        }
        if in_origin {
            if let Some(url) = t
                .strip_prefix("url")
                .and_then(|r| r.trim().strip_prefix('='))
            {
                return normalize_slug(url.trim());
            }
        }
    }
    None
}

/// Normalize any GitHub remote URL form to `owner/repo`.
///
///   git@github.com:dpep/claude.git   → dpep/claude
///   https://github.com/dpep/claude   → dpep/claude
///   ssh://git@github.com/dpep/claude → dpep/claude
pub fn normalize_slug(url: &str) -> Option<String> {
    let s = url.trim();
    // Locate the host marker and take everything after it.
    let after = &s[s.find("github.com")? + "github.com".len()..];
    // Drop the separator between host and path (`:` for scp form, `/`).
    let path = after.trim_start_matches([':', '/']);
    let path = path.trim_end_matches('/');
    let path = path.strip_suffix(".git").unwrap_or(path);
    let mut parts = path.splitn(3, '/');
    let owner = parts.next().filter(|s| !s.is_empty())?;
    let repo = parts.next().filter(|s| !s.is_empty())?;
    Some(format!("{owner}/{repo}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizes_forms() {
        for url in [
            "git@github.com:dpep/claude.git",
            "https://github.com/dpep/claude.git",
            "https://github.com/dpep/claude",
            "ssh://git@github.com/dpep/claude",
            "git@github.com:dpep/claude",
        ] {
            assert_eq!(normalize_slug(url).as_deref(), Some("dpep/claude"), "{url}");
        }
    }

    #[test]
    fn rejects_non_github() {
        assert_eq!(normalize_slug("git@gitlab.com:foo/bar.git"), None);
    }

    #[test]
    fn reads_origin_from_git_config() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::create_dir(dir.path().join(".git")).unwrap();
        std::fs::write(
            dir.path().join(".git").join("config"),
            "[core]\n\tbare = false\n[remote \"origin\"]\n\turl = git@github.com:dpep/claude.git\n\tfetch = +refs/heads/*\n",
        )
        .unwrap();
        assert_eq!(origin_slug(dir.path()).as_deref(), Some("dpep/claude"));
    }

    #[test]
    fn missing_config_is_none() {
        let dir = tempfile::tempdir().unwrap();
        assert_eq!(origin_slug(dir.path()), None);
    }
}
