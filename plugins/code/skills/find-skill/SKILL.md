---
name: find-skill
description: Locate where a Claude Code skill lives on disk so you can open, read, or edit its SKILL.md — "where is the X skill", "open the Y skill", "show me the git skill's instructions", or finding a skill that isn't loaded this session. Uses the `find-skill` CLI. Not for choosing among already-active skills (those are in context).
---

# find-skill

`find-skill` fuzzy-matches Claude Code skills by name and description and
prints where each one lives — an editable local path when the skill's
working repo is checked out, otherwise its GitHub URL.

Use it to **locate, open, or inspect** a skill's file, including skills not
loaded this session. Not for choosing among the skills already active — those
are in context already.

It searches three sources: personal (`~/.claude`), installed marketplace
plugins, and local working repos you register — including each one's
`.claude/` directory.

Skills are counted the way Claude Code counts them: a `<name>/SKILL.md`, or a
`commands/*.md` file (custom commands are skills — both give you `/name`). A
loose `~/.claude/skills/*.md` is neither. Frontmatter is optional and falls
back the same way: `name` to the directory or file name, `description` to the
body's first paragraph.

## Commands

Every command takes `-j/--json` and `-J/--ndjson`; prefer `--json` when you
need to act on the result.

```
find-skill <pattern>       fuzzy match over name + description
find-skill -1 <pattern>    print ONLY the best match's path
find-skill -a <pattern>    list EVERY copy (personal + local repos + installed),
                           instead of collapsing each skill to its editable best
find-skill                 list every skill, grouped by source
find-skill --paths         show registered search paths + discovered repos
find-skill --add <dir>     register a repo or workspace to search
find-skill --remove <dir>  unregister a search path
find-skill --refresh       re-discover local repos, ignoring the cache
find-skill --reset         delete config + cache (clean slate)
```

## Typical use

```sh
find-skill -1 git          # -> <checkout>/plugins/code/skills/git/SKILL.md
find-skill --json dataviz  # path + description + source, as JSON
```

Then Read the printed path. Once the working repo is registered (below), the
match resolves to that editable checkout rather than an installed clone.

## Cached vs. usable

A marketplace plugin can be downloaded but not switched on, in which case its
skills sit on disk and invoking one fails with `Unknown skill`. Those results
carry `"enabled": false` (text output: `(disabled)`) alongside `"plugin_ref"`
— the `<plugin>@<marketplace>` key to add:

```json
{"enabledPlugins": {"code@dpep": true}}
```

in `~/.claude/settings.json` (or the project's `.claude/settings.json`). The
change takes effect in a **new session**. A missing `enabled` field means
there was nothing to check — a personal skill, or no settings file declaring
`enabledPlugins`.

## Search space

Personal and installed skills are always scanned. Local working repos are
**opt-in** — register the repos (or a workspace holding several) you edit
skills in:

```sh
find-skill --add <dir>     # a single repo, or a workspace of repos
```

Registering the checkout is what buys the editable source path (matched to the
installed skill by the repo's GitHub remote) instead of a read-only installed
copy — the difference between opening a file you can change and one you can't.

Repo discovery is cached; skill *contents* are read live, so edits show up
immediately. If a newly-checked-out repo's skills aren't appearing, run
`find-skill --refresh`. `find-skill --paths` shows what's registered and which
repos are currently discovered.

## Installing / updating the binary

If `find-skill` isn't on PATH, build it from your checkout of
<https://github.com/dpep/claude> (needs the Rust toolchain), then retry:

```sh
make -C <repo> install-find-skill   # builds find-skill, links into ~/.claude/bin
export PATH="$HOME/.claude/bin:$PATH"
```

To update, `git pull` in the checkout and re-run the make line (the install is
a symlink, so a plain rebuild also refreshes what's on PATH).
