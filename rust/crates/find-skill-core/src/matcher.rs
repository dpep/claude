//! Fuzzy ranking of skills against a query, over both the label and
//! the description — because "which skill does X" is usually answered
//! by the description, not the slug. A label hit outranks a
//! description-only hit for the same query.

use crate::types::Skill;
use nucleo_matcher::pattern::{CaseMatching, Normalization, Pattern};
use nucleo_matcher::{Config, Matcher, Utf32Str};

/// Filter + rank `skills` by `query`. An empty query returns everything
/// sorted by label. Non-matches are dropped.
pub fn rank(skills: Vec<Skill>, query: &str) -> Vec<Skill> {
    if query.trim().is_empty() {
        let mut all = skills;
        all.sort_by(|a, b| a.label.cmp(&b.label));
        return all;
    }

    let mut matcher = Matcher::new(Config::DEFAULT);
    let pattern = Pattern::parse(query, CaseMatching::Ignore, Normalization::Smart);
    let mut buf: Vec<char> = Vec::new();

    let mut scored: Vec<(u32, Skill)> = Vec::new();
    for skill in skills {
        let label_score = pattern.score(Utf32Str::new(&skill.label, &mut buf), &mut matcher);
        let desc_score = pattern.score(Utf32Str::new(&skill.description, &mut buf), &mut matcher);
        // Weight the label more heavily so slug hits sort first.
        let combined = match (label_score, desc_score) {
            (None, None) => continue,
            (l, d) => l.unwrap_or(0) * 2 + d.unwrap_or(0),
        };
        scored.push((combined, skill));
    }

    // Highest score first; ties broken by label for stable output.
    scored.sort_by(|a, b| b.0.cmp(&a.0).then_with(|| a.1.label.cmp(&b.1.label)));

    // A short query fuzzy-matches a long noisy tail of weak
    // subsequence hits. Keep only results scoring within a fraction of
    // the best, which trims that tail without a hard result cap.
    if let Some((top, _)) = scored.first() {
        let threshold = (*top as f32 * KEEP_RATIO) as u32;
        scored.retain(|(score, _)| *score >= threshold);
    }
    scored.into_iter().map(|(_, s)| s).collect()
}

/// Drop matches scoring below this fraction of the best match.
const KEEP_RATIO: f32 = 0.5;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Source;

    fn skill(label: &str, description: &str) -> Skill {
        Skill {
            label: label.into(),
            description: description.into(),
            path: Default::default(),
            source: Source::Personal,
            editable: true,
            remote_url: None,
            plugin_ref: None,
            enabled: None,
            identity: label.into(),
        }
    }

    #[test]
    fn empty_query_returns_all_sorted() {
        let out = rank(vec![skill("git", ""), skill("azimuth", "")], "");
        assert_eq!(
            out.iter().map(|s| s.label.clone()).collect::<Vec<_>>(),
            ["azimuth", "git"]
        );
    }

    #[test]
    fn matches_description_not_just_label() {
        let out = rank(
            vec![
                skill("azimuth", "goal clarity and prioritization"),
                skill("git", "version control"),
            ],
            "clarity",
        );
        assert_eq!(out.first().map(|s| s.label.as_str()), Some("azimuth"));
    }

    #[test]
    fn label_hit_outranks_description_hit() {
        let out = rank(
            vec![
                skill("other", "mentions git in passing"),
                skill("git", "version control"),
            ],
            "git",
        );
        assert_eq!(out.first().map(|s| s.label.as_str()), Some("git"));
    }

    #[test]
    fn non_matches_dropped() {
        let out = rank(vec![skill("git", "vcs")], "zzzzz");
        assert!(out.is_empty());
    }
}
