//! find-skill-core — locate Claude Code skills across personal,
//! installed, and local-working-repo sources; dedupe the same skill
//! found in more than one place (preferring the copy you can edit);
//! and rank the result against a fuzzy query.
//!
//! The `-cli` crate is thin glue over the functions here.

pub mod config;
pub mod discover;
pub mod enablement;
pub mod frontmatter;
pub mod index;
pub mod marketplaces;
pub mod matcher;
pub mod paths;
pub mod remote;
pub mod scan;
pub mod types;

pub use index::RepoRoot;
pub use types::{Skill, Source};

use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Re-discover local repos if the cache is older than a week (or the
/// registered search paths changed). Skill *contents* are always read
/// live, so this only bounds how stale the repo *list* can get.
pub const DEFAULT_TTL_SECS: i64 = 7 * 24 * 3600;

/// Injected environment — real paths in production, temp dirs in tests.
pub struct Env {
    pub claude_dir: PathBuf,
    pub config_path: PathBuf,
    pub cache_path: PathBuf,
    /// Where we're being run from — locates the project settings that
    /// may enable or disable a plugin for this directory.
    pub cwd: PathBuf,
    /// Unix seconds "now" — passed in so discovery timestamps and cache
    /// staleness are deterministic under test.
    pub now: i64,
}

impl Env {
    /// Production environment. `CLAUDE_DIR` overrides `~/.claude`
    /// (mainly for tests / alternate homes).
    pub fn real() -> Self {
        let claude_dir = std::env::var("CLAUDE_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(|_| {
                PathBuf::from(std::env::var("HOME").unwrap_or_default()).join(".claude")
            });
        Env {
            claude_dir,
            config_path: config::config_path(),
            cache_path: index::cache_path(),
            cwd: std::env::current_dir().unwrap_or_default(),
            now: now_unix(),
        }
    }
}

fn now_unix() -> i64 {
    time::OffsetDateTime::now_utc().unix_timestamp()
}

/// Full find pipeline: scan all sources, rank against `query`.
/// `refresh` forces local-repo re-discovery regardless of cache age.
/// `all` lists every copy from every source (personal + local repos +
/// installed) instead of collapsing each skill to its editable best.
pub fn find(env: &Env, query: &str, refresh: bool, all: bool) -> Vec<Skill> {
    let cfg = config::load(&env.config_path);
    let roots = resolve_roots(env, &cfg, refresh);
    let repos = marketplaces::load(&env.claude_dir);

    let mut skills = scan::scan_personal(&env.claude_dir);
    skills.extend(scan::scan_installed(&env.claude_dir, &repos));
    for root in &roots {
        skills.extend(scan::scan_repo(&root.path, root.remote.as_deref()));
    }
    mark_enablement(&mut skills, &enablement::load(&env.claude_dir, &env.cwd));
    if !all {
        skills = dedupe(skills);
    }
    matcher::rank(skills, query)
}

/// Registered search paths plus the repos discovery currently sees.
pub fn paths(env: &Env, refresh: bool) -> (Vec<PathBuf>, Vec<RepoRoot>) {
    let cfg = config::load(&env.config_path);
    let roots = resolve_roots(env, &cfg, refresh);
    (cfg.search_paths, roots)
}

/// Outcome of registering/unregistering a search path.
pub struct PathChange {
    pub changed: bool,
    pub roots: Vec<RepoRoot>,
}

/// Register a search path (repo or workspace of repos) and re-discover.
pub fn add_path(env: &Env, dir: &Path) -> anyhow::Result<PathChange> {
    let mut cfg = config::load(&env.config_path);
    let changed = config::add(&mut cfg, dir);
    config::save(&env.config_path, &cfg)?;
    let roots = rediscover(env, &cfg);
    Ok(PathChange { changed, roots })
}

/// Unregister a search path and re-discover.
pub fn remove_path(env: &Env, dir: &Path) -> anyhow::Result<PathChange> {
    let mut cfg = config::load(&env.config_path);
    let changed = config::remove(&mut cfg, dir);
    config::save(&env.config_path, &cfg)?;
    let roots = rediscover(env, &cfg);
    Ok(PathChange { changed, roots })
}

/// Delete the config and cache files, returning the paths that existed
/// and were removed. Restores find-skill to a clean slate (empty search
/// paths, no discovery cache).
pub fn reset(env: &Env) -> Vec<PathBuf> {
    let mut removed = Vec::new();
    for path in [&env.config_path, &env.cache_path] {
        if path.exists() && std::fs::remove_file(path).is_ok() {
            removed.push(path.clone());
        }
    }
    removed
}

/// Load-or-discover the local repo roots, honoring the cache unless
/// `refresh` is set or the cache is stale.
fn resolve_roots(env: &Env, cfg: &config::Config, refresh: bool) -> Vec<RepoRoot> {
    if !refresh {
        if let Some(idx) = index::load(&env.cache_path) {
            if !idx.is_stale(&cfg.search_paths, DEFAULT_TTL_SECS, env.now) {
                // A cached root may have been moved/deleted since.
                return idx.roots.into_iter().filter(|r| r.path.is_dir()).collect();
            }
        }
    }
    rediscover(env, cfg)
}

fn rediscover(env: &Env, cfg: &config::Config) -> Vec<RepoRoot> {
    let roots = discover::discover(&cfg.search_paths);
    index::save(
        &env.cache_path,
        &index::Index::new(cfg.search_paths.clone(), roots.clone(), env.now),
    );
    roots
}

/// Record, on every skill that comes from an installed plugin, whether
/// that plugin is enabled. A `None` map means no settings file said
/// anything, so we leave the question open.
fn mark_enablement(skills: &mut [Skill], enabled: &Option<HashMap<String, bool>>) {
    let Some(map) = enabled else { return };
    for skill in skills.iter_mut() {
        if let Some(key) = &skill.plugin_ref {
            skill.enabled = Some(map.get(key).copied().unwrap_or(false));
        }
    }
}

/// Collapse skills that share an identity (the same skill found both
/// installed and in a local working repo). Prefer the editable copy;
/// carry the GitHub link and the plugin's enablement over from the
/// installed copy either way — a local checkout doesn't make a disabled
/// plugin's skill invokable.
fn dedupe(skills: Vec<Skill>) -> Vec<Skill> {
    let mut by_id: HashMap<String, Skill> = HashMap::new();
    for skill in skills {
        match by_id.get_mut(&skill.identity) {
            None => {
                by_id.insert(skill.identity.clone(), skill);
            }
            Some(existing) => {
                let remote = existing
                    .remote_url
                    .take()
                    .or_else(|| skill.remote_url.clone());
                let plugin_ref = existing
                    .plugin_ref
                    .take()
                    .or_else(|| skill.plugin_ref.clone());
                let enabled = existing.enabled.or(skill.enabled);
                if skill.prefers_over(existing) {
                    *existing = skill;
                }
                existing.remote_url = remote;
                existing.plugin_ref = plugin_ref;
                existing.enabled = enabled;
            }
        }
    }
    by_id.into_values().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn write(path: &Path, body: &str) {
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, body).unwrap();
    }

    /// Build an Env over a temp claude dir + temp config/cache.
    fn env_in(tmp: &Path) -> Env {
        Env {
            claude_dir: tmp.join("claude"),
            config_path: tmp.join("config.json"),
            cache_path: tmp.join("cache.json"),
            cwd: tmp.to_path_buf(),
            now: 1000,
        }
    }

    #[test]
    fn dedupe_prefers_editable_and_keeps_link() {
        let installed = Skill {
            label: "code:git".into(),
            description: "d".into(),
            path: PathBuf::from("/installed/SKILL.md"),
            source: Source::Installed {
                marketplace: "dpep".into(),
                plugin: "code".into(),
            },
            editable: false,
            remote_url: Some(
                "https://github.com/dpep/claude/tree/main/plugins/code/skills/git".into(),
            ),
            plugin_ref: Some("code@dpep".into()),
            enabled: Some(false),
            identity: "dpep/claude::plugins/code/skills/git".into(),
        };
        let repo = Skill {
            label: "code:git".into(),
            description: "d".into(),
            path: PathBuf::from("/work/SKILL.md"),
            source: Source::Repo {
                remote: Some("dpep/claude".into()),
            },
            editable: true,
            remote_url: None,
            plugin_ref: None,
            enabled: None,
            identity: "dpep/claude::plugins/code/skills/git".into(),
        };
        let out = dedupe(vec![installed, repo]);
        assert_eq!(out.len(), 1);
        assert!(out[0].editable);
        assert_eq!(out[0].path, PathBuf::from("/work/SKILL.md"));
        assert!(out[0].remote_url.is_some(), "github link carried over");
        assert!(out[0].disabled(), "enablement carried over");
        assert_eq!(out[0].plugin_ref.as_deref(), Some("code@dpep"));
    }

    #[test]
    fn find_bridges_installed_to_local_repo() {
        let tmp = tempfile::tempdir().unwrap();
        let env = env_in(tmp.path());

        // An installed dpep marketplace skill.
        write(
            &env.claude_dir
                .join("plugins/marketplaces/dpep/plugins/code/skills/git/SKILL.md"),
            "---\nname: git\ndescription: Git operations.\n---\n",
        );
        write(
            &env.claude_dir.join("plugins/known_marketplaces.json"),
            r#"{"dpep":{"source":{"source":"github","repo":"dpep/claude"}}}"#,
        );

        // The local working checkout of that same marketplace.
        let repo = tmp.path().join("code/claude");
        write(
            &repo.join(".git/config"),
            "[remote \"origin\"]\n\turl = git@github.com:dpep/claude.git\n",
        );
        write(
            &repo.join("plugins/code/skills/git/SKILL.md"),
            "---\nname: git\ndescription: Git operations.\n---\n",
        );

        // Register the workspace, then find.
        std::fs::write(
            &env.config_path,
            format!(
                r#"{{"search_paths":["{}"]}}"#,
                tmp.path().join("code").display()
            ),
        )
        .unwrap();

        let out = find(&env, "git", true, false);
        let git: Vec<_> = out.iter().filter(|s| s.label == "git").collect();
        assert_eq!(git.len(), 1, "installed + repo collapse to one");
        assert!(git[0].editable, "resolves to the editable working copy");
        assert!(git[0].path.starts_with(&repo));
        assert!(git[0].remote_url.is_some(), "keeps the github link");

        // --all lists every copy instead of collapsing.
        let all = find(&env, "git", true, true);
        let git_all: Vec<_> = all.iter().filter(|s| s.label == "git").collect();
        assert_eq!(git_all.len(), 2, "installed + repo both shown under --all");
        assert!(git_all.iter().any(|s| s.editable));
        assert!(git_all.iter().any(|s| !s.editable));
    }

    #[test]
    fn installed_skills_report_plugin_enablement() {
        let tmp = tempfile::tempdir().unwrap();
        let env = env_in(tmp.path());
        for (plugin, skill) in [("code", "git"), ("datasets", "notes")] {
            write(
                &env.claude_dir.join(format!(
                    "plugins/marketplaces/dpep/plugins/{plugin}/skills/{skill}/SKILL.md"
                )),
                &format!("---\nname: {skill}\ndescription: A skill.\n---\n"),
            );
        }
        // `code` is on; `datasets` is cached but never enabled.
        write(
            &env.claude_dir.join("settings.json"),
            r#"{"enabledPlugins":{"code@dpep":true}}"#,
        );

        let out = find(&env, "", true, false);
        let git = out.iter().find(|s| s.label == "git").unwrap();
        assert_eq!(git.plugin_ref.as_deref(), Some("code@dpep"));
        assert_eq!(git.enabled, Some(true));

        let notes = out.iter().find(|s| s.label == "notes").unwrap();
        assert_eq!(notes.plugin_ref.as_deref(), Some("datasets@dpep"));
        assert!(notes.disabled(), "absent from enabledPlugins => unusable");
    }

    #[test]
    fn enablement_is_unknown_without_settings() {
        let tmp = tempfile::tempdir().unwrap();
        let env = env_in(tmp.path());
        write(
            &env.claude_dir
                .join("plugins/marketplaces/dpep/plugins/code/skills/git/SKILL.md"),
            "---\nname: git\ndescription: Git operations.\n---\n",
        );
        let git = find(&env, "git", true, false).remove(0);
        assert_eq!(git.enabled, None, "no settings => no claim either way");
        assert!(!git.disabled());
    }

    #[test]
    fn reset_deletes_config_and_cache() {
        let tmp = tempfile::tempdir().unwrap();
        let env = env_in(tmp.path());
        std::fs::write(&env.config_path, r#"{"search_paths":[]}"#).unwrap();
        std::fs::write(&env.cache_path, "{}").unwrap();

        let removed = reset(&env);
        assert_eq!(removed.len(), 2);
        assert!(!env.config_path.exists());
        assert!(!env.cache_path.exists());
        // Idempotent: nothing left to remove.
        assert!(reset(&env).is_empty());
    }

    #[test]
    fn add_path_persists_and_discovers() {
        let tmp = tempfile::tempdir().unwrap();
        let env = env_in(tmp.path());
        std::fs::write(&env.config_path, r#"{"search_paths":[]}"#).unwrap();
        let repo = tmp.path().join("proj");
        write(
            &repo.join(".git/config"),
            "[remote \"origin\"]\n\turl = git@github.com:rewild/proj.git\n",
        );
        write(
            &repo.join("skills/thing/SKILL.md"),
            "---\nname: thing\n---\n",
        );

        let outcome = add_path(&env, &repo).unwrap();
        assert!(outcome.changed);
        assert_eq!(outcome.roots.len(), 1);
        // Persisted to config.
        let cfg = config::load(&env.config_path);
        assert_eq!(cfg.search_paths.len(), 1);
    }
}
