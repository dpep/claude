---
description: Find datasets whose frontmatter matches a query (metadata-only first pass)
argument-hint: <query>
---

Search for `$ARGUMENTS` in dataset **frontmatter only** — fast, cheap on context, and high-signal for "find the right dataset" intent. Content search is a deeper pass available on request.

If `$ARGUMENTS` is empty, ask the user what to search for and stop.

## First pass: metadata search

Frontmatter only lives in two places: inline at the top of `.md` files, and inside `*.meta.yml` sidecars. Search those directly — no need to crawl raw data files (csv/tsv/json/yaml).

1. **Search for `$ARGUMENTS`** (case-insensitive, return filenames) inside `~/.claude/datasets/` restricted to files matching `*.md` or `*.meta.yml` (skip dotfiles). Use whichever search tool fits your environment best — Claude Code's built-in Grep tool, `rg`, `grep`, `ag`, anything that respects the glob and case-insensitivity. The goal is the list of matching paths; the underlying tool is up to you.

2. For each match, resolve to the **dataset's data file path**:
   - If the match is a `.md` file → the dataset path *is* the file itself.
   - If the match is a `*.meta.yml` sidecar → strip the `.meta.yml` suffix to get the data file path (e.g., `pantry.csv.meta.yml` → `pantry.csv`).

3. Read the matched frontmatter and extract `name`, `description`, `last_updated_at`.

4. Present matching datasets sorted by `last_updated_at` (most recent first):

   | path | name | description | last_updated_at |

   Use the relative path from `~/.claude/datasets/`.

## Fallback: content search

If the metadata pass returns **zero matches**, tell the user and ask:

> No metadata hits for `<query>`. Want me to search the actual contents of the datasets? Heavier on context but more thorough.

Wait for confirmation. If they say yes:

**Search for `$ARGUMENTS`** (case-sensitive this time — content searches benefit from precision) inside `~/.claude/datasets/` across all files **except** `*.meta.yml` sidecars and dotfiles. Same tool freedom: pick whatever fits.

For each match, show the relative path, frontmatter `name` and `description`, and 1–3 best snippets — a couple of lines of surrounding context per hit, formatted however your search tool surfaces them.

If still nothing matches, say so plainly — don't fabricate.
