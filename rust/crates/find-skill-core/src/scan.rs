//! The three source scans that turn directories into `Skill` entries:
//! personal (`~/.claude`), installed marketplace checkouts, and local
//! working repos. Each yields both skills (`<name>/SKILL.md`) and
//! commands (`commands/*.md`) — Claude Code invokes them alike.

use crate::frontmatter;
use crate::types::{Skill, Source};
use ignore::WalkBuilder;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Read a `SKILL.md` and work out how to label and describe it, the way
/// Claude Code does: every frontmatter field is optional, `name`
/// defaults to the directory name, and an absent `description` falls
/// back to the first paragraph of the body. `None` only when there's no
/// readable file — what makes a skill is where it lives, not what its
/// frontmatter says.
fn load_fields(path: &Path, dir_name: &str) -> Option<(String, String)> {
    let body = std::fs::read_to_string(path).ok()?;
    let fm = frontmatter::parse(&body);
    let label = fm
        .name
        .filter(|s| !s.trim().is_empty())
        .unwrap_or_else(|| dir_name.to_string());
    let description = fm
        .description
        .filter(|s| !s.trim().is_empty())
        .unwrap_or_else(|| frontmatter::first_paragraph(&body));
    Some((label, description))
}

/// The skill directory's own name — the label and identity to fall back
/// on.
fn dir_name(skill_dir: &Path) -> String {
    skill_dir
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string()
}

fn subdirs(path: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    if let Ok(rd) = std::fs::read_dir(path) {
        for e in rd.flatten() {
            if e.file_type().map(|t| t.is_dir()).unwrap_or(false) {
                out.push(e.path());
            }
        }
    }
    out
}

/// Every `*.md` under a `commands/` directory, nested ones included.
/// Custom commands are skills — `commands/deploy.md` and
/// `skills/deploy/SKILL.md` both give you `/deploy` — so a tool that
/// finds skills has to find these too.
fn command_files(commands_dir: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    if !commands_dir.is_dir() {
        return out;
    }
    for entry in WalkBuilder::new(commands_dir)
        .max_depth(Some(4))
        .build()
        .flatten()
    {
        let path = entry.path();
        if path.is_file() && path.extension().and_then(|e| e.to_str()) == Some("md") {
            out.push(path.to_path_buf());
        }
    }
    out
}

/// A command's name is its file stem — the `/deploy` you type.
fn file_stem(path: &Path) -> String {
    path.file_stem()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string()
}

/// `~/.claude/` — one `<name>/SKILL.md` per skill under `skills/`, plus
/// every `commands/*.md`. A loose top-level `skills/*.md` is not a
/// layout Claude Code loads, so it isn't one we report.
pub fn scan_personal(claude_dir: &Path) -> Vec<Skill> {
    let mut skills = Vec::new();
    let personal = |label: String, description: String, path: PathBuf, identity: String| Skill {
        label,
        description,
        path,
        source: Source::Personal,
        editable: true,
        remote_url: None,
        plugin_ref: None,
        enabled: None,
        identity,
    };

    for skill_dir in subdirs(&claude_dir.join("skills")) {
        let md = skill_dir.join("SKILL.md");
        let name = dir_name(&skill_dir);
        let Some((label, description)) = load_fields(&md, &name) else {
            continue;
        };
        skills.push(personal(label, description, md, format!("personal:{name}")));
    }

    for md in command_files(&claude_dir.join("commands")) {
        let name = file_stem(&md);
        let Some((label, description)) = load_fields(&md, &name) else {
            continue;
        };
        let identity = format!("personal-command:{name}");
        skills.push(personal(label, description, md, identity));
    }
    skills
}

/// Path of a skill dir relative to its repo/marketplace root, as a
/// forward-slash string — the identity key that bridges an installed
/// skill to its working-repo source.
fn relpath_key(root: &Path, skill_dir: &Path) -> String {
    skill_dir
        .strip_prefix(root)
        .unwrap_or(skill_dir)
        .to_string_lossy()
        .replace('\\', "/")
}

/// Installed marketplace skills: `~/.claude/plugins/marketplaces/<mp>/
/// {plugins,external_plugins}/<plugin>/skills/<skill>/SKILL.md`.
///
/// `repos` maps marketplace name → `owner/repo` (for links + bridge
/// identity). We read the current checkout only, never the versioned
/// `cache/` copies — so there's no version to dedupe.
pub fn scan_installed(claude_dir: &Path, repos: &HashMap<String, String>) -> Vec<Skill> {
    let mp_root = claude_dir.join("plugins").join("marketplaces");
    let mut skills = Vec::new();
    for mp_dir in subdirs(&mp_root) {
        let marketplace = mp_dir
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();
        let repo = repos.get(&marketplace).cloned();
        for group in ["plugins", "external_plugins"] {
            for plugin_dir in subdirs(&mp_dir.join(group)) {
                let plugin = plugin_dir
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string();
                // A skill is a directory holding SKILL.md; a command is
                // a lone markdown file. Both land here the same way,
                // keyed on the path that identifies them.
                let units = subdirs(&plugin_dir.join("skills"))
                    .into_iter()
                    .map(|dir| (dir.join("SKILL.md"), dir))
                    .chain(
                        command_files(&plugin_dir.join("commands"))
                            .into_iter()
                            .map(|md| (md.clone(), md)),
                    );

                for (md, unit) in units {
                    let name = if unit == md {
                        file_stem(&unit)
                    } else {
                        dir_name(&unit)
                    };
                    let Some((label, description)) = load_fields(&md, &name) else {
                        continue;
                    };
                    let rel = relpath_key(&mp_dir, &unit);
                    let kind = if unit == md { "blob" } else { "tree" };
                    let remote_url = repo
                        .as_ref()
                        .map(|r| format!("https://github.com/{r}/{kind}/main/{rel}"));
                    // Identity bridges to the working repo by remote slug
                    // + relative path; fall back to marketplace name.
                    let identity = match &repo {
                        Some(r) => format!("{r}::{rel}"),
                        None => format!("{marketplace}:{plugin}:{name}"),
                    };
                    skills.push(Skill {
                        label,
                        description,
                        path: md,
                        source: Source::Installed {
                            marketplace: marketplace.clone(),
                            plugin: plugin.clone(),
                        },
                        editable: false,
                        remote_url,
                        plugin_ref: Some(format!("{plugin}@{marketplace}")),
                        enabled: None,
                        identity,
                    });
                }
            }
        }
    }
    skills
}

/// A markdown file Claude Code would register as a command: it sits
/// under a `commands/` directory owned by a plugin (a `.claude-plugin/`
/// manifest beside it) or by a `.claude/` config dir. Without that
/// check, any repo's unrelated `commands/` notes would read as skills.
fn is_command_file(path: &Path) -> bool {
    if path.extension().and_then(|e| e.to_str()) != Some("md") {
        return false;
    }
    let mut cur = path.parent();
    while let Some(dir) = cur {
        if dir.file_name().and_then(|s| s.to_str()) == Some("commands") {
            return dir.parent().is_some_and(|owner| {
                owner.join(".claude-plugin").is_dir()
                    || owner.file_name().and_then(|s| s.to_str()) == Some(".claude")
            });
        }
        cur = dir.parent();
    }
    false
}

/// The traversal shared by repo scanning and repo discovery:
/// gitignore-aware, and `.claude/` is the one hidden directory worth
/// descending into — it's where a project keeps its own skills.
pub fn walk(root: &Path, max_depth: usize) -> ignore::Walk {
    WalkBuilder::new(root)
        .max_depth(Some(max_depth))
        .hidden(false)
        .filter_entry(|e| {
            let name = e.file_name().to_string_lossy().to_string();
            e.depth() == 0 || !name.starts_with('.') || name == ".claude"
        })
        .build()
}

/// Whether a path names something invokable — a skill or a command.
pub fn is_skill_or_command(path: &Path) -> bool {
    path.file_name().is_some_and(|n| n == "SKILL.md") || is_command_file(path)
}

/// Walk a local working repo for skills and commands. `remote` is the
/// repo's origin slug, used to build bridge identities that match
/// `scan_installed`.
pub fn scan_repo(root: &Path, remote: Option<&str>) -> Vec<Skill> {
    let mut skills = Vec::new();
    for entry in walk(root, 8).flatten() {
        let is_skill = entry.file_name() == "SKILL.md";
        if !is_skill && !is_command_file(entry.path()) {
            continue;
        }
        let md = entry.into_path();
        // A skill is identified by its directory, a command by its file.
        let unit = match md.parent() {
            Some(dir) if is_skill => dir.to_path_buf(),
            Some(_) => md.clone(),
            None => continue,
        };
        let name = if is_skill {
            dir_name(&unit)
        } else {
            file_stem(&unit)
        };
        let Some((label, description)) = load_fields(&md, &name) else {
            continue;
        };
        let rel = relpath_key(root, &unit);
        let identity = match remote {
            Some(r) => format!("{r}::{rel}"),
            None => format!("repo:{}::{rel}", root.to_string_lossy()),
        };
        skills.push(Skill {
            label,
            description,
            path: md.clone(),
            source: Source::Repo {
                remote: remote.map(str::to_string),
            },
            editable: true,
            remote_url: None,
            plugin_ref: None,
            enabled: None,
            identity,
        });
    }
    skills
}

#[cfg(test)]
mod tests {
    use super::*;

    fn write(path: &Path, body: &str) {
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, body).unwrap();
    }

    /// What Claude Code would load: a `<name>/SKILL.md`. A loose
    /// top-level markdown file isn't one; missing frontmatter fields
    /// don't disqualify a skill, they just fall back.
    #[test]
    fn personal_takes_skill_dirs_and_fills_in_missing_fields() {
        let dir = tempfile::tempdir().unwrap();
        let claude = dir.path();
        write(
            &claude.join("skills/git/SKILL.md"),
            "---\nname: git\ndescription: Git stuff.\n---\n",
        );
        write(
            &claude.join("skills/notes.md"),
            "---\nname: notes\ndescription: Loose note.\n---\n",
        );
        write(
            &claude.join("skills/draft/SKILL.md"),
            "# Draft\n\nA rough idea,\nstill forming.\n\nMore below.\n",
        );
        write(
            &claude.join("skills/nameless/SKILL.md"),
            "---\ndescription: No name here.\n---\n",
        );
        write(&claude.join("skills/empty"), "");

        let mut got = scan_personal(claude);
        got.sort_by(|a, b| a.label.cmp(&b.label));
        assert_eq!(
            got.iter().map(|s| s.label.as_str()).collect::<Vec<_>>(),
            ["draft", "git", "nameless"],
            "loose *.md is not a skill; a dir without SKILL.md is not either"
        );
        // No frontmatter at all: labelled by directory, described by the
        // first paragraph of the body.
        assert_eq!(got[0].description, "A rough idea, still forming.");
        assert_eq!(got[1].description, "Git stuff.");
        assert_eq!(got[2].description, "No name here.");
        assert!(got[0].editable);
    }

    /// `commands/*.md` are invokable the same as skills, so they're
    /// found the same way — labelled by file stem when frontmatter
    /// doesn't say otherwise.
    #[test]
    fn personal_commands_are_skills_too() {
        let dir = tempfile::tempdir().unwrap();
        let claude = dir.path();
        write(
            &claude.join("commands/deploy.md"),
            "---\ndescription: Ship it.\n---\n",
        );
        write(
            &claude.join("commands/frontend/component.md"),
            "Scaffold a component.\n",
        );
        write(&claude.join("commands/notes.txt"), "not markdown");

        let mut got = scan_personal(claude);
        got.sort_by(|a, b| a.label.cmp(&b.label));
        assert_eq!(
            got.iter().map(|s| s.label.as_str()).collect::<Vec<_>>(),
            ["component", "deploy"]
        );
        assert_eq!(got[0].description, "Scaffold a component.");
        assert!(got[0].editable);
    }

    #[test]
    fn installed_plugin_commands_carry_the_plugin_ref() {
        let dir = tempfile::tempdir().unwrap();
        let claude = dir.path();
        write(
            &claude.join("plugins/marketplaces/dpep/plugins/azimuth/commands/focus.md"),
            "---\ndescription: Pick the next action.\n---\n",
        );
        let mut repos = HashMap::new();
        repos.insert("dpep".to_string(), "dpep/claude".to_string());

        let got = scan_installed(claude, &repos);
        assert_eq!(got.len(), 1);
        assert_eq!(got[0].label, "focus");
        assert_eq!(got[0].plugin_ref.as_deref(), Some("azimuth@dpep"));
        assert_eq!(
            got[0].identity,
            "dpep/claude::plugins/azimuth/commands/focus.md"
        );
        assert_eq!(
            got[0].remote_url.as_deref(),
            Some("https://github.com/dpep/claude/blob/main/plugins/azimuth/commands/focus.md")
        );
    }

    /// A project keeps its own under `.claude/`, which the walk has to
    /// descend into; an unrelated `commands/` elsewhere is not a plugin's.
    #[test]
    fn repo_finds_project_dot_claude_and_ignores_stray_commands() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        write(
            &root.join(".claude/skills/deploy/SKILL.md"),
            "---\ndescription: Deploy this project.\n---\n",
        );
        write(
            &root.join(".claude/commands/release.md"),
            "---\ndescription: Cut a release.\n---\n",
        );
        write(
            &root.join("docs/commands/psql.md"),
            "Handy psql commands.\n",
        );

        let mut got = scan_repo(root, None);
        got.sort_by(|a, b| a.label.cmp(&b.label));
        assert_eq!(
            got.iter().map(|s| s.label.as_str()).collect::<Vec<_>>(),
            ["deploy", "release"]
        );
    }

    #[test]
    fn installed_labels_and_identity() {
        let dir = tempfile::tempdir().unwrap();
        let claude = dir.path();
        let base = claude.join("plugins/marketplaces/dpep/plugins/code/skills/git");
        write(
            &base.join("SKILL.md"),
            "---\nname: git\ndescription: Git ops.\n---\n",
        );
        let mut repos = HashMap::new();
        repos.insert("dpep".to_string(), "dpep/claude".to_string());
        let got = scan_installed(claude, &repos);
        assert_eq!(got.len(), 1);
        assert_eq!(got[0].label, "git");
        assert_eq!(got[0].identity, "dpep/claude::plugins/code/skills/git");
        assert_eq!(
            got[0].remote_url.as_deref(),
            Some("https://github.com/dpep/claude/tree/main/plugins/code/skills/git")
        );
        assert_eq!(got[0].plugin_ref.as_deref(), Some("code@dpep"));
        assert!(!got[0].editable);
    }

    #[test]
    fn repo_identity_matches_installed_for_bridge() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        write(
            &root.join("plugins/code/skills/git/SKILL.md"),
            "---\nname: git\ndescription: Git ops.\n---\n",
        );
        let got = scan_repo(root, Some("dpep/claude"));
        assert_eq!(got.len(), 1);
        assert_eq!(got[0].identity, "dpep/claude::plugins/code/skills/git");
        assert!(got[0].editable);
    }
}
