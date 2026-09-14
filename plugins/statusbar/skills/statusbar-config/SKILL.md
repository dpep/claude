---
name: statusbar-config
description: Change what the statusbar status line shows — "add the model to my status line", "show/hide the branch, PR, context %, worktree, session", "my status line is too noisy", "shorten the path", "strip the prefix off my branch name", "change the separator", or any "can the statusbar show X". Edits ~/.config/claude/statusbar/config.json, NOT settings.json, and verifies the new bar by rendering it. For first-time setup (building the binary and wiring settings.json) use /statusbar-install instead.
---

# statusbar config

Everything the bar shows is driven by one optional file:

```
~/.config/claude/statusbar/config.json
```

(`$XDG_CONFIG_HOME/claude/statusbar/config.json` when that variable is set.)

**It usually does not exist — create it.** The binary ships with defaults and
every section is `#[serde(default)]`, so **write only the keys being changed**.
Copying the full default block into the file freezes today's defaults as the
user's own settings, and future default improvements stop reaching them.

**This is not `~/.claude/settings.json`.** That file holds only the wiring —
the `statusLine` command and its refresh interval — and says nothing about
which segments render. Editing it to change the bar's contents does nothing.

## What the user asked for → what to set

| The ask | Set |
|---|---|
| show the model | `model.enabled: true` |
| …but not my everyday model | `model.hide: ["Opus"]` — case-insensitive, matches the family too |
| the model name is too long | already abbreviated to the family by default; `model.abbreviate: false` restores the full `Opus 5 (1M context)` |
| path is too long / too short | `cwd.collapse_depth` (default 3) |
| drop a workspace prefix from the path | `cwd.strip_prefixes: ["~/work/"]` — first match wins |
| my handle clutters the branch | `branch.strip_handle` is already on; add `branch.strip_prefixes: ["dp/"]` for other namespaces |
| hide the branch on trunk | `branch.hide_on: ["main", "master"]` |
| show the branch *and* the PR | `pr.prefer_over_branch: false` |
| stop showing PRs | `pr.enabled: false` |
| always show context %, not just when high | `context_window.show_at: 0` |
| context / rate warnings are too noisy | raise `context_window.show_at`, `rate_limit.warn_at` |
| the session name / branch eats the bar | both cap at 24 by default; tune `session.max_len`, `branch.max_len`, or `0` to uncap |
| drop the worktree or session tag | `worktree: false` / `session: false` |
| different separator | `separator` |

The session name is whatever Claude Code derived when the session started, so
on a long session it goes stale — it describes the first task, not the current
one. Capping it limits the damage; it does not fix it.

A minimal file that turns on the model and nothing else:

```json
{ "model": { "enabled": true } }
```

Full segment list, colors, and defaults: [the plugin README](../../README.md).

## Verify it, don't guess

The live bar refreshes every ~10s on its own — no restart, no reload. But you
don't have to wait, and you shouldn't: pipe a session payload straight in.

```bash
echo '{"workspace":{"current_dir":"/tmp/demo"},"model":{"display_name":"Opus 5"}}' | statusbar
statusbar --json    # {"statusline": "<plain>", "rendered": "<ansi>"} — statusline is easiest to read
```

The input mimics what Claude Code pipes in, so add whatever fields the segment
under test needs. Show the user the before and after line.

If `statusbar` isn't on PATH it lives at `~/.claude/bin/statusbar`; if that's
missing too, the plugin was never installed — run `/statusbar-install`.

## When a change doesn't take

The two failure modes look nothing alike, and the difference tells you which
one you have:

- **One setting didn't apply, everything else did** → that key is misspelled or
  nested at the wrong level. Unknown keys are ignored silently, so there is no
  error to find.
- **Every customization reverted at once** → the file failed to parse, and the
  whole thing was discarded for defaults. Either malformed JSON (a trailing
  comma) or a key with the wrong type — `"model": true` where an object belongs
  is the easy mistake, since `"worktree": true` right above it *is* a bare bool.

That second behaviour is deliberate: a broken config must never break a
session, so the bar fails open rather than erroring. The cost is silence, which
is why you render after every edit rather than trusting the write.

`python3 -m json.tool < ~/.config/claude/statusbar/config.json` catches the
syntax half. It won't catch a wrong type or a misspelled key, so still re-render.
