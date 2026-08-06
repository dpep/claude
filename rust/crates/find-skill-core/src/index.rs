//! The discovered-roots cache. The expensive part of find-skill is
//! hunting the filesystem for local working repos that contain skills;
//! we cache *just that list* (paths + remotes) so subsequent runs skip
//! the hunt. Skill contents are always read live from these roots, so
//! edits show up immediately — the cache never goes stale on content,
//! only on the set of repos, which we refresh on a TTL or `--refresh`.

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// A local working repo that contains at least one skill.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepoRoot {
    pub path: PathBuf,
    /// `owner/repo` origin slug, when the repo has a GitHub remote.
    pub remote: Option<String>,
}

/// Cached discovery result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Index {
    pub version: u32,
    /// Unix seconds when discovery last ran.
    pub discovered_at: i64,
    pub search_paths: Vec<PathBuf>,
    pub roots: Vec<RepoRoot>,
}

const CURRENT_VERSION: u32 = 1;

impl Index {
    pub fn new(search_paths: Vec<PathBuf>, roots: Vec<RepoRoot>, now: i64) -> Self {
        Index {
            version: CURRENT_VERSION,
            discovered_at: now,
            search_paths,
            roots,
        }
    }

    /// True if the cache is older than `ttl_secs`, a different version,
    /// or was built for a different set of search paths.
    pub fn is_stale(&self, search_paths: &[PathBuf], ttl_secs: i64, now: i64) -> bool {
        self.version != CURRENT_VERSION
            || self.search_paths != search_paths
            || now.saturating_sub(self.discovered_at) > ttl_secs
    }
}

/// Default cache file: `$XDG_CACHE_HOME/claude/find-skill/index.json`.
pub fn cache_path() -> PathBuf {
    crate::paths::cache_dir("find-skill").join("index.json")
}

pub fn load(path: &Path) -> Option<Index> {
    let raw = std::fs::read_to_string(path).ok()?;
    serde_json::from_str(&raw).ok()
}

/// Best-effort write — discovery still works if the cache can't persist.
pub fn save(path: &Path, index: &Index) {
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if let Ok(json) = serde_json::to_string_pretty(index) {
        let _ = std::fs::write(path, json);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrips() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("index.json");
        let idx = Index::new(
            vec![PathBuf::from("/code")],
            vec![RepoRoot {
                path: PathBuf::from("/code/claude"),
                remote: Some("dpep/claude".into()),
            }],
            1000,
        );
        save(&path, &idx);
        let back = load(&path).unwrap();
        assert_eq!(back.roots, idx.roots);
        assert_eq!(back.search_paths, idx.search_paths);
    }

    #[test]
    fn staleness_by_ttl_and_paths() {
        let idx = Index::new(vec![PathBuf::from("/code")], vec![], 1000);
        assert!(!idx.is_stale(&[PathBuf::from("/code")], 3600, 1500));
        assert!(idx.is_stale(&[PathBuf::from("/code")], 3600, 5000)); // aged out
        assert!(idx.is_stale(&[PathBuf::from("/other")], 3600, 1500)); // paths changed
    }

    #[test]
    fn missing_cache_is_none() {
        assert!(load(Path::new("/no/such/index.json")).is_none());
    }
}
