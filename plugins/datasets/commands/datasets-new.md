---
description: Scaffold a new dataset with correct frontmatter
argument-hint: <name-or-path>
---

Create a new dataset at `~/.claude/datasets/`.

1. **Resolve target path** from `$ARGUMENTS`. Markdown is the default if no extension; honor any extension the user specifies. Confirm the path; if non-markdown, confirm the format.

2. **Gather inputs** — ask the user (or infer from context):
   - `name` (defaults to a humanized version of the filename)
   - `description` (one line)

3. **Compute timestamps**: `date -u +%Y-%m-%dT%H:%M:%SZ` for both `created_at` and `last_updated_at`.

4. **Create parent directories** if needed: `mkdir -p $(dirname <path>)`.

5. **Write the dataset**:
   - For `.md`: inline YAML frontmatter at the top:
     ```
     ---
     name: <name>
     description: <description>
     created_at: <iso-utc>
     last_updated_at: <iso-utc>
     ---

     # <Title>
     ```
   - For other formats: create the data file (header row for csv/tsv, `[]` for json, etc.) AND a sidecar `<file>.meta.yml` with the same fields.

6. Confirm and ask if the user wants to populate it now.
