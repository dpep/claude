# code

Programming workflow & code-navigation skills. No hooks; the `rq` and `find-skill` skills each drive a small companion CLI.

## Install

```
/plugin marketplace add dpep/claude
/plugin install code@dpep
```

## Skills

- **git** — branching, PRs, rebasing, repo optimization. Subskill: `optimize` (pack consolidation, branch cleanup, fetch refspec narrowing). Reference: `worktrees` (multi-agent coordination, load on demand).
- **rq** — find where a symbol is defined via the `rq` CLI; ranks the most-likely
  definition first (prefer over grep/rg for "where is X defined"). Includes
  binary install/update guidance (`brew install dpep/tools/rq`).
- **find-skill** — find where a *skill* is defined: fuzzy-match Claude Code skills
  by name/description and print the path to open or edit. Counts skills the way
  Claude Code does — a `<name>/SKILL.md` or a `commands/*.md`, with the same
  frontmatter fallbacks — resolves to the editable working-repo copy
  (matched by GitHub remote) once you register the checkout with
  `find-skill --add <dir>`, and marks a skill `(disabled)` when its plugin is
  cached but missing from `enabledPlugins`. Built from this repo —
  `make install` links the binary into `~/.claude/bin` (add it to your PATH).
- **find-gem** — locate an installed Ruby *gem's* source and search inside it
  (`find-gem <gem> [rg pattern]`), bundle-aware, with `-j/--json` and
  `-J/--ndjson` like the other CLIs. A bash script shipped in this
  plugin's `bin/` (on PATH inside Claude Code sessions); `make install-find-gem`
  links it into `~/.claude/bin` for regular shells.

Will grow to cover review, testing, and language-specific patterns.

## Conventions

These are *my* conventions, shipped so my Claude has the same workflow on every machine. If you install this for yourself, adapt the personal bits — branch prefix (`dpep/`), repo root (`~/code/`) — to match your setup.

## Development

See the repo-level [CLAUDE.md](../../CLAUDE.md).
