---
description: Display a specific dataset (frontmatter + contents)
argument-hint: <path-or-name>
---

Show the dataset identified by `$ARGUMENTS`. The argument may be:

- An absolute path: `~/.claude/datasets/recipes/pasta.md`
- A path relative to `~/.claude/datasets/`: `recipes/pasta.md`
- Just a filename if unique: `pasta.md`
- A frontmatter `name` match (case-insensitive substring)

Resolve in that order. If multiple candidates result, list them and ask — never pick silently.

Once resolved:

1. Print frontmatter (inline YAML for `.md`, or the sibling `<file>.meta.yml` for other formats).
2. Print contents. For tabular formats (`.csv`, `.tsv`), render as a markdown table if small (under ~50 rows); for larger files, show first 20 + last 5 rows with `...` between.
