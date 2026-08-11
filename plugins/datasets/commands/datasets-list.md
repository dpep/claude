---
description: List all datasets at ~/.claude/datasets/ with their frontmatter metadata
---

List every dataset under `~/.claude/datasets/` (recursive).

If `~/.claude/datasets/` does not exist, say so and suggest `/datasets-new`. Stop.

1. Enumerate dataset files (excluding sidecars and hidden files):

   ```
   find ~/.claude/datasets/ -type f ! -name '*.meta.yml' ! -name '.*' 2>/dev/null
   ```

2. For each, read its frontmatter:
   - For `.md`: top of the file, between `---` delimiters.
   - For other formats: sibling `<file>.meta.yml` if present.

3. Present as a table sorted by `last_updated_at` (most recent first):

   | path | name | description | last_updated_at |

   Path is relative to `~/.claude/datasets/`. If a file has no frontmatter, mark it `(no metadata)` and include it anyway.
