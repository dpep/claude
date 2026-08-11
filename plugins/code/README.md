# code

Programming workflow & code-navigation skills. No hooks; the `rq` and `gqls` skills drive companion CLIs, and `find-gem` ships as a script.

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
- **find-gem** — locate an installed Ruby *gem's* source and search inside it
  (`find-gem <gem> [rg pattern]`), bundle-aware, with `-j/--json` and
  `-J/--ndjson` like the other CLIs. A bash script shipped in this
  plugin's `bin/` (on PATH inside Claude Code sessions); `make install` at the
  repo root links it into `~/.claude/bin` for regular shells.

Will grow to cover review, testing, and language-specific patterns.

## Conventions

These are *my* conventions, shipped so my Claude has the same workflow on every machine. If you install this for yourself, adapt the personal bits — branch prefix (`dpep/`), repo root (`~/code/`) — to match your setup.

## Development

See the repo-level [CLAUDE.md](../../CLAUDE.md).

## Tools these skills drive

Four skills call a CLI. A missing one doesn't fail loudly — Claude reads the
skill, the command isn't found, and it falls back to grep — so a SessionStart
hook reports which are absent and how to get each:

| tool | install |
|---|---|
| `rq` | `brew install dpep/tools/rq` (or `cargo install reference-query`) |
| `gqls` | `brew install dpep/tools/gqls` (or `cargo install gqls-cli`) |
| `find-skill` | `make -C ~/.claude/plugins/marketplaces/dpep install` |
| `find-gem` | ships in `bin/` here — the same `make install` symlinks it |

The hook is quiet when everything resolves, and speaks up out loud only when
*nothing* does — a fresh install you probably meant to finish. If some tools
are present and others aren't, it tells Claude and stays silent to you: these
skills ship together and you may only want one of them.
