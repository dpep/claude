//! Parse `~/.claude/plugins/known_marketplaces.json` into a
//! `marketplace name → owner/repo` map. Used to turn an installed
//! skill's marketplace into a GitHub link and to match it against a
//! local working checkout by remote slug.

use serde::Deserialize;
use std::collections::HashMap;
use std::path::Path;

#[derive(Deserialize)]
struct Entry {
    source: SourceField,
}

#[derive(Deserialize)]
struct SourceField {
    #[serde(default)]
    repo: Option<String>,
}

/// Load `name → repo-slug`. Missing file / malformed entries yield an
/// empty map rather than an error — find-skill degrades gracefully.
pub fn load(claude_dir: &Path) -> HashMap<String, String> {
    let path = claude_dir.join("plugins").join("known_marketplaces.json");
    let Ok(raw) = std::fs::read_to_string(path) else {
        return HashMap::new();
    };
    let Ok(map) = serde_json::from_str::<HashMap<String, Entry>>(&raw) else {
        return HashMap::new();
    };
    map.into_iter()
        .filter_map(|(name, e)| e.source.repo.map(|r| (name, r)))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_repo_slugs() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::create_dir(dir.path().join("plugins")).unwrap();
        std::fs::write(
            dir.path().join("plugins").join("known_marketplaces.json"),
            r#"{"dpep":{"source":{"source":"github","repo":"dpep/claude"},"installLocation":"/x"},
                "official":{"source":{"source":"github","repo":"anthropics/claude-plugins-official"}}}"#,
        )
        .unwrap();
        let m = load(dir.path());
        assert_eq!(m.get("dpep").map(String::as_str), Some("dpep/claude"));
        assert_eq!(
            m.get("official").map(String::as_str),
            Some("anthropics/claude-plugins-official")
        );
    }

    #[test]
    fn missing_file_is_empty() {
        let dir = tempfile::tempdir().unwrap();
        assert!(load(dir.path()).is_empty());
    }
}
