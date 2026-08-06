//! Plugin enablement — whether an installed plugin is actually turned
//! on. A marketplace plugin can be downloaded and fully readable on disk
//! while absent from `enabledPlugins`, in which case Claude Code answers
//! `Unknown skill` for everything it ships. We read the same settings
//! files Claude Code merges, so a located skill can say which it is.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Merged `enabledPlugins`, keyed `<plugin>@<marketplace>`.
///
/// `None` when no settings file declares the key at all — we then know
/// nothing about enablement and say nothing, rather than reporting every
/// plugin as disabled.
pub fn load(claude_dir: &Path, cwd: &Path) -> Option<HashMap<String, bool>> {
    let mut merged: Option<HashMap<String, bool>> = None;
    for path in settings_files(claude_dir, cwd) {
        if let Some(entries) = read_enabled_plugins(&path) {
            merged.get_or_insert_with(HashMap::new).extend(entries);
        }
    }
    merged
}

/// User settings first, then the enclosing project's shared and local
/// settings — later files win, matching Claude Code's precedence.
fn settings_files(claude_dir: &Path, cwd: &Path) -> Vec<PathBuf> {
    let mut files = vec![claude_dir.join("settings.json")];
    if let Some(project) = project_claude_dir(claude_dir, cwd) {
        files.push(project.join("settings.json"));
        files.push(project.join("settings.local.json"));
    }
    files
}

/// The nearest `.claude/` at or above `cwd` — find-skill may be run from
/// anywhere inside a project. The user's own `~/.claude` is not a
/// project: reached from any cwd under `$HOME`, it would let a stray
/// `settings.local.json` there outrank the real user settings.
fn project_claude_dir(claude_dir: &Path, cwd: &Path) -> Option<PathBuf> {
    let user = resolved(claude_dir);
    cwd.ancestors()
        .map(|dir| dir.join(".claude"))
        .find(|dir| dir.is_dir() && resolved(dir) != user)
}

/// Compare directories by resolved path: `cwd` arrives with symlinks
/// followed (on macOS `/var` is really `/private/var`) while the claude
/// dir doesn't, so the same directory reaches us spelled two ways.
fn resolved(dir: &Path) -> PathBuf {
    std::fs::canonicalize(dir).unwrap_or_else(|_| dir.to_path_buf())
}

/// Pull `enabledPlugins` out of one settings file. Missing file, bad
/// JSON, or no such key all yield `None` — find-skill degrades quietly.
fn read_enabled_plugins(path: &Path) -> Option<HashMap<String, bool>> {
    let raw = std::fs::read_to_string(path).ok()?;
    let doc: serde_json::Value = serde_json::from_str(&raw).ok()?;
    let obj = doc.get("enabledPlugins")?.as_object()?;
    Some(
        obj.iter()
            .filter_map(|(key, val)| val.as_bool().map(|on| (key.clone(), on)))
            .collect(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn write(path: &Path, body: &str) {
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, body).unwrap();
    }

    #[test]
    fn reads_user_settings() {
        let dir = tempfile::tempdir().unwrap();
        let claude = dir.path().join("claude");
        write(
            &claude.join("settings.json"),
            r#"{"enabledPlugins":{"code@dpep":true,"datasets@dpep":false}}"#,
        );
        let map = load(&claude, dir.path()).unwrap();
        assert_eq!(map.get("code@dpep"), Some(&true));
        assert_eq!(map.get("datasets@dpep"), Some(&false));
    }

    #[test]
    fn project_settings_win_and_local_wins_over_shared() {
        let dir = tempfile::tempdir().unwrap();
        let claude = dir.path().join("claude");
        write(
            &claude.join("settings.json"),
            r#"{"enabledPlugins":{"code@dpep":true,"azimuth@dpep":true}}"#,
        );
        let project = dir.path().join("work/proj");
        write(
            &project.join(".claude/settings.json"),
            r#"{"enabledPlugins":{"code@dpep":false}}"#,
        );
        write(
            &project.join(".claude/settings.local.json"),
            r#"{"enabledPlugins":{"azimuth@dpep":false}}"#,
        );

        // Run from a subdirectory: the nearest `.claude/` still applies.
        let nested = project.join("src/deep");
        std::fs::create_dir_all(&nested).unwrap();
        let map = load(&claude, &nested).unwrap();
        assert_eq!(map.get("code@dpep"), Some(&false));
        assert_eq!(map.get("azimuth@dpep"), Some(&false));
    }

    #[test]
    fn the_users_own_claude_dir_is_not_a_project() {
        let dir = tempfile::tempdir().unwrap();
        let claude = dir.path().join(".claude");
        write(
            &claude.join("settings.json"),
            r#"{"enabledPlugins":{"code@dpep":true}}"#,
        );
        // A leftover local file next to the user settings — Claude Code
        // never merges this one, and neither do we.
        write(
            &claude.join("settings.local.json"),
            r#"{"enabledPlugins":{"code@dpep":false}}"#,
        );
        let map = load(&claude, dir.path()).unwrap();
        assert_eq!(map.get("code@dpep"), Some(&true));
    }

    #[test]
    fn no_settings_is_unknown_not_disabled() {
        let dir = tempfile::tempdir().unwrap();
        assert!(load(&dir.path().join("claude"), dir.path()).is_none());
        // A settings file without the key is equally uninformative.
        let claude = dir.path().join("claude");
        write(&claude.join("settings.json"), r#"{"theme":"dark"}"#);
        assert!(load(&claude, dir.path()).is_none());
    }
}
