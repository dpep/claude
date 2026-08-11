# datasets

User-curated reference data for Claude. Files at `~/.claude/datasets/` (markdown, CSV, TSV, JSON, YAML) with YAML frontmatter for discovery. A skill plus four slash commands so Claude can find, read, and update your datasets when you mention them.

## What it does

You keep personal reference data in `~/.claude/datasets/` — weeknight recipes, pantry inventory, a book reading log, garden journal, anything you want Claude to recall later. This plugin gives Claude:

- A skill that activates when you say "dataset" or "datasets" — Claude knows where to look without being told each session.
- Four slash commands for atomic operations.
- A frontmatter convention so each dataset is self-describing and discoverable.

No silent writes. Claude only persists data when you explicitly request it; the activation cue ("dataset") is the consent signal.

## Install

```
/plugin marketplace add dpep/claude
/plugin install datasets@dpep
```

## Recommended permissions

This plugin reads, writes, and searches inside `~/.claude/datasets/` constantly. To avoid Claude Code prompting you on every operation, pre-approve these patterns in your **user-level** `~/.claude/settings.json`:

```json
{
  "permissions": {
    "allow": [
      "Read(~/.claude/datasets/**)",
      "Edit(~/.claude/datasets/**)",
      "Write(~/.claude/datasets/**)",
      "Bash(find ~/.claude/datasets/:*)",
      "Bash(mkdir -p ~/.claude/datasets/:*)"
    ]
  }
}
```

If you prefer not to edit settings.json by hand, the first time Claude Code prompts you for a dataset operation, choose "Always allow" — it'll add the equivalent rule for you.

These rules are scoped to `~/.claude/datasets/`; nothing outside that directory is silently allowed.

## Storage

`~/.claude/datasets/`. Subdirectories may be used for organization (`recipes/`, `meals/`, `hobbies/`). Filenames and structure are your choice.

## Frontmatter

Every dataset carries frontmatter that Claude maintains as the data evolves.

For markdown, frontmatter is inline:

```md
---
name: Weeknight dinner recipes
description: Quick recipes I actually cook on weeknights, with notes on substitutions
created_at: 2026-04-15T08:30:00Z
last_updated_at: 2026-05-03T11:42:00Z
---

# Weeknight dinner recipes

## Pasta aglio e olio
...
```

For other formats, frontmatter lives in a sidecar `<file>.meta.yml`:

```yaml
# ~/.claude/datasets/pantry.csv.meta.yml
name: Pantry inventory
description: Staple ingredients and spices I keep on hand, with rough quantities
created_at: 2026-03-01T09:00:00Z
last_updated_at: 2026-05-03T11:42:00Z
```

### Required fields

| field | notes |
|---|---|
| `name` | human-readable |
| `description` | one-line summary |
| `created_at` | ISO 8601 UTC datetime |
| `last_updated_at` | ISO 8601 UTC datetime; refreshed by Claude on every change |

Frontmatter is YAML — add custom fields per-dataset (e.g., `tags`, `source`, `owner`) if useful. The convention only requires the four above.

## Slash commands

- `/datasets-list` — list all datasets with metadata, sorted by most recently updated
- `/datasets-search <query>` — ripgrep across datasets; returns file paths, frontmatter, and snippets
- `/datasets-show <path-or-name>` — display a specific dataset
- `/datasets-new <name>` — scaffold a new dataset with correct frontmatter

## Skills

- `datasets` — teaches Claude the convention and activates on explicit "dataset" mentions.

## Privacy

The plugin reads/writes only inside `~/.claude/datasets/`. Writes only happen on explicit user instruction (creating, updating, or deleting a specific dataset). The skill won't activate on ambiguous cues like "remember this" — you have to use the word "dataset" to opt in to persistence.

## Development

See the repo-level [CLAUDE.md](../../CLAUDE.md).
