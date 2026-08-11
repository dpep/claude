---
name: datasets
description: User-curated reference data stored at ~/.claude/datasets/ (markdown, csv, tsv, json, yaml). Activate when the user mentions "dataset" or "datasets" — to create, update, list, search, or read. Never write to a dataset without an explicit user request; the user saying "dataset" is the consent signal.
---

# Datasets

The user keeps personal reference data in `~/.claude/datasets/` — markdown, CSV, TSV, JSON, YAML. Examples: weeknight recipes, pantry inventory, a book reading log, garden journal. Datasets are user-curated (never auto-generated) and persist across sessions.

## Activation

Only act on this skill when the user **explicitly mentions** "dataset" or "datasets" (or invokes a `/datasets-*` slash command). Cues like "remember this list" or "save these notes" are too ambiguous — wait for the user to use the dataset vocabulary, which signals intent for persistent storage.

## Storage location

Default: `~/.claude/datasets/`. Subdirectories are encouraged for organization (e.g., `recipes/weeknight.md`, `pantry/staples.csv`). Filenames and structure are user-chosen — do not impose a rigid convention.

## Frontmatter (required on every dataset)

Every dataset carries frontmatter so it's self-describing:

- **Markdown** (`.md`): YAML frontmatter inline at the top, between `---` delimiters.
- **Other formats** (`.csv`, `.tsv`, `.json`, `.yml`, `.yaml`): a sidecar metadata file at `<dataset-path>.meta.yml`.

### Required fields

| field | example |
|---|---|
| `name` | `"Weeknight dinner recipes"` |
| `description` | one line summarizing what's in here |
| `created_at` | ISO 8601 datetime, UTC: `2026-04-15T08:30:00Z` |
| `last_updated_at` | ISO 8601 datetime, UTC; refresh on every change |

Frontmatter is YAML, so users can add custom fields per-dataset (e.g., `tags`, `source`, `owner`) if useful — the convention only requires the four above.

### Maintaining frontmatter

When you modify a dataset's contents in any meaningful way:

- Set `last_updated_at` to the current UTC datetime (e.g., output of `date -u +%Y-%m-%dT%H:%M:%SZ`).
- Refine `description` if the dataset's purpose has shifted.

Update frontmatter **in the same write operation** that changes the data, so frontmatter never lags behind reality.

## Operations

- **Read / search** — free to do anytime once the skill has activated. Use `/datasets-list` and `/datasets-search` or read files directly with the Read tool.
- **Write / update** — only on explicit user instruction. Always update `last_updated_at` (and `description`/`tags` if relevant) alongside the data change.
- **Delete** — only on explicit user instruction with confirmation.

## Slash commands

For atomic operations the user can invoke directly:

- `/datasets-list` — list all datasets with their frontmatter
- `/datasets-search <query>` — search across datasets (frontmatter first, content on confirmation); tool-agnostic, returns matches with metadata
- `/datasets-show <path-or-name>` — display a specific dataset
- `/datasets-new <name>` — scaffold a new dataset with correct frontmatter
