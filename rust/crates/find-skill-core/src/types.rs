//! Core data types: a located `Skill` and where it came from.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Which kind of source a skill was found in.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "kind")]
pub enum Source {
    /// `~/.claude/skills/` — your own, directly editable.
    Personal,
    /// A local working git checkout that contains skills (e.g. a
    /// marketplace repo you author). Editable; the update target.
    Repo { remote: Option<String> },
    /// An installed marketplace checkout under `~/.claude/plugins`.
    /// Read-only in practice; carries the upstream repo for linking.
    Installed { marketplace: String, plugin: String },
}

/// A single located skill.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    /// Display label — the frontmatter `name`, or the skill directory's
    /// own name when that's absent.
    pub label: String,
    /// One-line description: the frontmatter `description`, else the
    /// first paragraph of the body. Empty when the body has neither.
    pub description: String,
    /// The best place to open/edit this skill — an editable local path
    /// when one exists, otherwise the installed copy.
    pub path: PathBuf,
    pub source: Source,
    /// True when `path` is something you can edit in place (personal or
    /// a local working repo) rather than an installed/managed copy.
    pub editable: bool,
    /// GitHub URL for the skill when it originates from a known
    /// marketplace — a reference link when no local checkout is found.
    pub remote_url: Option<String>,
    /// `<plugin>@<marketplace>` — how the plugin shipping this skill is
    /// keyed in `enabledPlugins`. `None` for personal/local-only skills.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plugin_ref: Option<String>,
    /// Whether that plugin is enabled. `None` when there's nothing to
    /// check (a personal skill) or nothing to check against (no settings
    /// file declares `enabledPlugins`). A cached-but-disabled plugin is
    /// on disk yet unusable — invoking its skills fails.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// Identity used to collapse the same skill found in multiple
    /// places (installed copy vs. its working-repo source). Not shown.
    #[serde(skip)]
    pub identity: String,
}

impl Skill {
    /// Rank preference when two entries share an identity: prefer the
    /// editable copy (the working repo you can actually update).
    pub fn prefers_over(&self, other: &Skill) -> bool {
        self.editable && !other.editable
    }

    /// Known to ship from a plugin that isn't enabled — the file is
    /// there, but the skill can't be invoked.
    pub fn disabled(&self) -> bool {
        self.enabled == Some(false)
    }
}
