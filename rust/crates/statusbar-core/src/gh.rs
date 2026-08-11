//! Deriving the active GitHub handle from the `gh` CLI's `hosts.yml`.
//!
//! The parse is a hand-rolled scan — no YAML dependency — that pulls the
//! `user:` under the top-level `github.com:` host. It stays pure and
//! testable; the CLI reads the file (`~/.config/gh/hosts.yml`) and feeds
//! the contents here. Any deviation from the expected shape yields
//! `None`, so a missing or odd file simply strips nothing.

/// The logged-in handle for `github.com`, or `None` if it can't be found.
pub fn handle_from_hosts_yml(raw: &str) -> Option<String> {
    let mut in_github = false;
    for line in raw.lines() {
        // A non-indented line is a top-level host key; it opens or closes
        // the block we care about.
        if !line.starts_with([' ', '\t']) {
            let key = line.split(':').next().unwrap_or("").trim();
            in_github = key == "github.com";
            continue;
        }
        if !in_github {
            continue;
        }
        // `user:` is a direct child of `github.com:`. The `:` in the
        // prefix means `users:` (the per-user map) never matches.
        if let Some(rest) = line.trim().strip_prefix("user:") {
            let handle = rest.trim().trim_matches('"');
            if !handle.is_empty() {
                return Some(handle.to_string());
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    const HOSTS: &str = "github.com:\n    user: dpep\n    oauth_token: gho_xxx\n    git_protocol: https\n    users:\n        dpep:\n            oauth_token: gho_xxx\n";

    #[test]
    fn extracts_the_github_user() {
        assert_eq!(handle_from_hosts_yml(HOSTS).as_deref(), Some("dpep"));
    }

    #[test]
    fn ignores_the_users_submap() {
        // The `users:` key sits at the same indent as `user:`; the `:` in
        // the prefix must keep them distinct even if `user:` came later.
        let raw =
            "github.com:\n    users:\n        dpep:\n            oauth_token: x\n    user: dpep\n";
        assert_eq!(handle_from_hosts_yml(raw).as_deref(), Some("dpep"));
    }

    #[test]
    fn only_the_github_com_host_counts() {
        let raw = "ghe.example.com:\n    user: work-account\ngithub.com:\n    user: dpep\n";
        assert_eq!(handle_from_hosts_yml(raw).as_deref(), Some("dpep"));
    }

    #[test]
    fn none_when_github_absent() {
        let raw = "ghe.example.com:\n    user: work-account\n";
        assert!(handle_from_hosts_yml(raw).is_none());
    }

    #[test]
    fn none_on_empty_or_garbage() {
        assert!(handle_from_hosts_yml("").is_none());
        assert!(handle_from_hosts_yml("not yaml at all").is_none());
    }

    #[test]
    fn tolerates_quoted_value() {
        let raw = "github.com:\n    user: \"dpep\"\n";
        assert_eq!(handle_from_hosts_yml(raw).as_deref(), Some("dpep"));
    }
}
