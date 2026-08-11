//! Persistent user config: the registered search paths where local
//! working repos (or workspaces containing repos) live. Distinct from
//! the discovery *cache* — this is durable user intent ("also look
//! here"), the cache is the derived result of looking.
//!
//! `find-skill add <dir>` appends here; discovery then scans every
//! registered path for repos that contain skills.

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Config {
    /// Directories to search for local working repos with skills. Each
    /// may be a single repo or a workspace holding many repos.
    #[serde(default)]
    pub search_paths: Vec<PathBuf>,
}

/// Config file path: `$XDG_CONFIG_HOME/claude/find-skill/config.json`.
pub fn config_path() -> PathBuf {
    claude_paths::config_dir("find-skill").join("config.json")
}

/// Load config. Absent or malformed → an empty config; the user opts
/// into local-repo discovery explicitly with `find-skill --add <dir>`.
pub fn load(path: &Path) -> Config {
    std::fs::read_to_string(path)
        .ok()
        .and_then(|raw| serde_json::from_str::<Config>(&raw).ok())
        .unwrap_or_default()
}

/// Best-effort persist.
pub fn save(path: &Path, cfg: &Config) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let json = serde_json::to_string_pretty(cfg).unwrap_or_else(|_| "{}".into());
    std::fs::write(path, json)
}

/// Add a search path (canonicalized, deduped). Returns true if it was
/// newly added, false if already present.
pub fn add(cfg: &mut Config, dir: &Path) -> bool {
    let canon = dir.canonicalize().unwrap_or_else(|_| dir.to_path_buf());
    if cfg.search_paths.iter().any(|p| p == &canon) {
        return false;
    }
    cfg.search_paths.push(canon);
    true
}

/// Remove a search path. Returns true if it was present.
pub fn remove(cfg: &mut Config, dir: &Path) -> bool {
    let canon = dir.canonicalize().unwrap_or_else(|_| dir.to_path_buf());
    let before = cfg.search_paths.len();
    cfg.search_paths.retain(|p| p != &canon && p != dir);
    cfg.search_paths.len() != before
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_is_idempotent() {
        let dir = tempfile::tempdir().unwrap();
        let mut cfg = Config::default();
        assert!(add(&mut cfg, dir.path()));
        assert!(!add(&mut cfg, dir.path())); // second time: already there
        assert_eq!(cfg.search_paths.len(), 1);
    }

    #[test]
    fn remove_reports_presence() {
        let dir = tempfile::tempdir().unwrap();
        let mut cfg = Config::default();
        add(&mut cfg, dir.path());
        assert!(remove(&mut cfg, dir.path()));
        assert!(!remove(&mut cfg, dir.path()));
    }

    #[test]
    fn roundtrips_through_disk() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("config.json");
        let mut cfg = Config::default();
        cfg.search_paths.push(PathBuf::from("/code/claude"));
        save(&path, &cfg).unwrap();
        let back = load(&path);
        assert_eq!(back.search_paths, vec![PathBuf::from("/code/claude")]);
    }
}
