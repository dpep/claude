//! Minimal YAML-frontmatter reader for SKILL.md files.
//!
//! We only need `name` and `description`, and skill frontmatter is
//! uniformly simple `key: value` (occasionally a `>`/`|` block scalar
//! for a long description). A tiny hand parser handles that without
//! pulling a full YAML stack or its quirks.

/// Parsed frontmatter fields we care about.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Frontmatter {
    pub name: Option<String>,
    pub description: Option<String>,
}

/// Parse the leading `---` fenced block of a SKILL.md body.
pub fn parse(content: &str) -> Frontmatter {
    let mut fm = Frontmatter::default();
    let mut lines = content.lines();

    // Frontmatter must be the very first thing (allowing a UTF-8 BOM).
    let first = lines.next().map(|l| l.trim_start_matches('\u{feff}'));
    if first.map(str::trim_end) != Some("---") {
        return fm;
    }

    let mut cur_key: Option<String> = None;
    let mut block: Vec<String> = Vec::new();

    let flush = |fm: &mut Frontmatter, key: &Option<String>, block: &[String]| {
        if let Some(k) = key {
            let val = block.join(" ").trim().to_string();
            assign(fm, k, val);
        }
    };

    for line in lines {
        if line.trim_end() == "---" {
            break;
        }
        // A new top-level `key:` starts when the line is unindented.
        if !line.starts_with([' ', '\t']) {
            if let Some((key, rest)) = split_key(line) {
                flush(&mut fm, &cur_key, &block);
                block.clear();
                let rest = rest.trim();
                if rest.is_empty() || rest == ">" || rest == "|" || rest == ">-" || rest == "|-" {
                    cur_key = Some(key); // value continues on following indented lines
                } else {
                    assign(&mut fm, &key, unquote(rest));
                    cur_key = None;
                }
                continue;
            }
        }
        // Indented continuation of the current block scalar.
        if cur_key.is_some() {
            block.push(line.trim().to_string());
        }
    }
    flush(&mut fm, &cur_key, &block);
    fm
}

/// The body's first paragraph — what Claude Code describes a skill by
/// when its frontmatter omits `description`. Skips the frontmatter block
/// and any leading headings.
pub fn first_paragraph(content: &str) -> String {
    let mut lines = content.lines().peekable();

    // Step over the frontmatter block, if there is one.
    if lines
        .peek()
        .map(|l| l.trim_start_matches('\u{feff}').trim_end())
        == Some("---")
    {
        lines.next();
        for line in lines.by_ref() {
            if line.trim_end() == "---" {
                break;
            }
        }
    }

    let mut para: Vec<&str> = Vec::new();
    for line in lines {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            if para.is_empty() {
                continue; // still looking for the first prose line
            }
            break;
        }
        para.push(line);
    }
    para.join(" ")
}

fn split_key(line: &str) -> Option<(String, &str)> {
    let (k, v) = line.split_once(':')?;
    let key = k.trim();
    if key.is_empty() || key.contains(char::is_whitespace) {
        return None;
    }
    Some((key.to_string(), v))
}

fn assign(fm: &mut Frontmatter, key: &str, val: String) {
    match key {
        "name" => fm.name = Some(val),
        "description" => fm.description = Some(val),
        _ => {}
    }
}

fn unquote(s: &str) -> String {
    let s = s.trim();
    let bytes = s.as_bytes();
    if s.len() >= 2
        && ((bytes[0] == b'"' && bytes[s.len() - 1] == b'"')
            || (bytes[0] == b'\'' && bytes[s.len() - 1] == b'\''))
    {
        return s[1..s.len() - 1].to_string();
    }
    s.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_paragraph_skips_frontmatter_and_headings() {
        assert_eq!(
            first_paragraph("---\nname: git\n---\n\n# Git\n\nBranching and\nPRs.\n\nMore.\n"),
            "Branching and PRs."
        );
        // No frontmatter at all is the case that matters most.
        assert_eq!(
            first_paragraph("# Draft\n\nA rough idea.\n"),
            "A rough idea."
        );
        assert_eq!(first_paragraph("# Nothing but a heading\n"), "");
    }

    #[test]
    fn parses_simple_fields() {
        let fm = parse("---\nname: git\ndescription: Git operations and PRs.\n---\n# Body\n");
        assert_eq!(fm.name.as_deref(), Some("git"));
        assert_eq!(fm.description.as_deref(), Some("Git operations and PRs."));
    }

    #[test]
    fn strips_quotes() {
        let fm = parse("---\nname: \"quoted\"\ndescription: 'single'\n---\n");
        assert_eq!(fm.name.as_deref(), Some("quoted"));
        assert_eq!(fm.description.as_deref(), Some("single"));
    }

    #[test]
    fn folds_block_scalar_description() {
        let fm = parse("---\nname: x\ndescription: >\n  line one\n  line two\n---\n");
        assert_eq!(fm.description.as_deref(), Some("line one line two"));
    }

    #[test]
    fn no_frontmatter_returns_empty() {
        assert_eq!(parse("# Just a heading\n"), Frontmatter::default());
    }

    #[test]
    fn ignores_body_after_fence() {
        let fm = parse("---\nname: y\n---\ndescription: not-in-frontmatter\n");
        assert_eq!(fm.name.as_deref(), Some("y"));
        assert_eq!(fm.description, None);
    }
}
