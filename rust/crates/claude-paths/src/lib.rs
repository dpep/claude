//! XDG-style path helpers shared by this workspace's binaries.
//!
//! `config_dir(tool)` → `$XDG_CONFIG_HOME/claude/<tool>`, else
//! `~/.config/claude/<tool>`. `cache_dir(tool)` is the same shape under
//! `$XDG_CACHE_HOME` / `~/.cache`.
//!
//! Deliberately only these two. They were inlined into find-skill at first —
//! right for one consumer — and became a crate when statusbar needed them too.
//! It is not a general "common" crate: the private workspace has one of those,
//! and depending on it is what kept both tools unpublishable.

use std::env;
use std::path::PathBuf;

fn under(env_var: &str, fallback: &[&str], tool: &str) -> PathBuf {
    if let Ok(d) = env::var(env_var) {
        if !d.is_empty() {
            return PathBuf::from(d).join("claude").join(tool);
        }
    }
    let mut p = PathBuf::from(env::var("HOME").unwrap_or_default());
    for part in fallback {
        p.push(part);
    }
    p.join("claude").join(tool)
}

/// Where this tool keeps user configuration.
pub fn config_dir(tool: &str) -> PathBuf {
    under("XDG_CONFIG_HOME", &[".config"], tool)
}

/// Where this tool keeps regenerable state.
pub fn cache_dir(tool: &str) -> PathBuf {
    under("XDG_CACHE_HOME", &[".cache"], tool)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn honors_xdg_then_falls_back_to_home() {
        // A set XDG var wins; the fallback nests under the conventional dir.
        // Both land in a `claude/<tool>` subdir so tools don't collide.
        unsafe { env::set_var("XDG_CONFIG_HOME", "/xdg") };
        assert_eq!(
            config_dir("find-skill"),
            PathBuf::from("/xdg/claude/find-skill")
        );
        unsafe { env::remove_var("XDG_CONFIG_HOME") };
        unsafe { env::set_var("HOME", "/home/x") };
        assert_eq!(
            config_dir("find-skill"),
            PathBuf::from("/home/x/.config/claude/find-skill")
        );
        assert_eq!(
            cache_dir("find-skill"),
            PathBuf::from("/home/x/.cache/claude/find-skill")
        );
    }
}
