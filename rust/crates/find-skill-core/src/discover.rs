//! Discover local working repos that contain skills, by scanning the
//! registered search paths. A search path may be a single repo or a
//! workspace holding many repos; either way we find every skill and
//! command, walk up to its enclosing git repo, and record that repo
//! once with its origin remote (the bridge key back to installed
//! skills).

use crate::index::RepoRoot;
use crate::remote;
use crate::scan;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

/// Walk `search_paths` and return the distinct git repos that contain
/// at least one skill, each with its origin slug (when it has one).
pub fn discover(search_paths: &[PathBuf]) -> Vec<RepoRoot> {
    // BTreeMap keyed on repo root → stable, deduped, sorted output.
    let mut roots: BTreeMap<PathBuf, Option<String>> = BTreeMap::new();
    for base in search_paths {
        if !base.is_dir() {
            continue;
        }
        for entry in scan::walk(base, 12).flatten() {
            if !scan::is_skill_or_command(entry.path()) {
                continue;
            }
            if let Some(root) = git_root(entry.path(), base) {
                roots
                    .entry(root)
                    .or_insert_with_key(|r| remote::origin_slug(r));
            }
        }
    }
    roots
        .into_iter()
        .map(|(path, remote)| RepoRoot { path, remote })
        .collect()
}

/// Walk up from `start` looking for the nearest ancestor containing a
/// `.git`, not going above `boundary`. Returns None if the file isn't
/// inside a git repo within the search path.
fn git_root(start: &Path, boundary: &Path) -> Option<PathBuf> {
    let mut cur = start.parent();
    while let Some(dir) = cur {
        if dir.join(".git").exists() {
            return Some(dir.to_path_buf());
        }
        if dir == boundary {
            break;
        }
        cur = dir.parent();
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn write(path: &Path, body: &str) {
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, body).unwrap();
    }

    #[test]
    fn finds_repos_containing_skills() {
        let ws = tempfile::tempdir().unwrap();
        // A workspace with two repos, one holding a skill.
        let repo_a = ws.path().join("claude");
        write(
            &repo_a.join(".git/config"),
            "[remote \"origin\"]\n\turl = git@github.com:dpep/claude.git\n",
        );
        write(
            &repo_a.join("plugins/code/skills/git/SKILL.md"),
            "---\nname: git\n---\n",
        );
        let repo_b = ws.path().join("empty");
        write(
            &repo_b.join(".git/config"),
            "[remote \"origin\"]\n\turl = git@github.com:dpep/empty.git\n",
        );

        let roots = discover(&[ws.path().to_path_buf()]);
        assert_eq!(roots.len(), 1);
        assert!(roots[0].path.ends_with("claude"));
        assert_eq!(roots[0].remote.as_deref(), Some("dpep/claude"));
    }

    /// A project whose only skills live under `.claude/` still counts —
    /// the walk descends into it despite the leading dot.
    #[test]
    fn finds_repo_whose_skills_are_under_dot_claude() {
        let ws = tempfile::tempdir().unwrap();
        let repo = ws.path().join("proj");
        write(
            &repo.join(".git/config"),
            "[remote \"origin\"]\n\turl = git@github.com:rewild/proj.git\n",
        );
        write(
            &repo.join(".claude/commands/release.md"),
            "Cut a release.\n",
        );

        let roots = discover(&[ws.path().to_path_buf()]);
        assert_eq!(roots.len(), 1);
        assert_eq!(roots[0].remote.as_deref(), Some("rewild/proj"));
    }

    #[test]
    fn skill_outside_git_is_skipped() {
        let ws = tempfile::tempdir().unwrap();
        write(
            &ws.path().join("loose/skills/x/SKILL.md"),
            "---\nname: x\n---\n",
        );
        assert!(discover(&[ws.path().to_path_buf()]).is_empty());
    }
}
