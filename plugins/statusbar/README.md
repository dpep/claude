# statusbar

A compact, configurable Claude Code status line. Claude Code pipes session
state as JSON to a status-line command every ~10s; statusbar renders it into a
tidy bar of `·`-joined segments with ANSI color.

```
~/code/lib/claude · example-api · [review] · ctx:72% · rate:88% (14m)
```

## What it renders

Segments, left to right — each shows only when its data is present and its
config toggle is on:

| Segment | Shows | Default |
|---|---|---|
| **cwd** (cyan) | working dir; `$HOME`→`~`, deep paths collapse to `…/parent/current` | on |
| **worktree** (dim) | `wt:<name>` — only when it differs from the branch | on |
| **ref** | the git branch, or the **PR number** in its place (see below) | on |
| **pr** | `#<number>`, colored green/red by review state | on (when PR open) |
| **session** (dim) | `[<name>]` when the session is named | on |
| **model** (dim) | the model's display name, abbreviated | off |
| **context** | `ctx:N%` of the context window used; yellow near compaction | on ≥50% |
| **rate** | `rate:N% (Nm)` of the 5-hour quota + reset countdown; yellow/red | on ≥70% |

**Branch vs. PR.** When `pr.enabled` is on and the session has an open PR,
`pr.prefer_over_branch` (default) renders `#474` *in place of* the branch —
Claude Code derives the PR from the current branch, so the number is the
tighter identifier. Turn `prefer_over_branch` off to show both.

**Context vs. rate limit** — they answer different questions, so both ship on.
`ctx:` is how full *this session's* context window is (actionable: compact
soon), shown once it passes 50%. `rate:` is your rolling 5-hour usage quota
(actionable: slow down), shown only once it's high enough to matter (≥70%).

## Install

```
/plugin marketplace add dpep/claude     # one-time, adds the marketplace
/plugin install statusbar@dpep          # install the plugin
/statusbar-install                      # build the binary + wire settings.json
```

`/statusbar-install` compiles the `statusbar` binary, symlinks it into
`~/.claude/bin`, and adds a `statusLine` block to your `~/.claude/settings.json`:

```json
"statusLine": { "type": "command", "command": "~/.claude/bin/statusbar", "refreshInterval": 10 }
```

(`${CLAUDE_PLUGIN_ROOT}` isn't substituted in statusLine commands, so the bar
points at the version-stable `~/.claude/bin` path `make install` maintains.)

## Configuration

Everything is optional — the binary ships with the defaults below. To
customize, write only the keys you care about to
`~/.config/claude/statusbar/config.json`; unset keys keep their defaults. A
malformed file falls back to defaults (the status line never breaks a session).

```json
{
  "separator": " · ",
  "cwd": { "strip_prefixes": ["~/workspace/"], "collapse_depth": 3 },
  "branch": { "enabled": true, "strip_prefixes": ["dp/"], "strip_handle": true, "hide_on": ["main", "master"] },
  "pr": { "enabled": true, "prefer_over_branch": true },
  "worktree": true,
  "session": true,
  "model": { "enabled": true, "hide": ["Opus"] },
  "context_window": { "enabled": true, "show_at": 50, "warn_at": 80 },
  "rate_limit": { "enabled": true, "warn_at": 70, "danger_at": 90 }
}
```

- **`cwd.strip_prefixes`** — trimmed off the working dir before display (a
  leading `~` expands to `$HOME`); first match wins. **`collapse_depth`** — max
  path components before collapsing to `…/parent/current`.
- **`branch.strip_prefixes`** — trimmed off the branch name (e.g. a `dp/`
  namespace); first match wins. **`strip_handle`** (default on) — when no
  explicit prefix matched, trim a leading `<github-handle>/`, where the handle
  is the `gh` CLI's logged-in user (read from `~/.config/gh/hosts.yml`, no
  network call). So a `<handle>/my-feature` branch shows as `my-feature` with
  no config; set `false` to disable. **`hide_on`** — branches that render nothing.
- **`model.hide`** — model names to omit (case-insensitive) — e.g. hide your
  everyday default so the segment only appears when you're on something else.
- **`context_window.show_at` / `warn_at`** — the `ctx:` floor and the
  turns-yellow threshold. **`rate_limit.warn_at` / `danger_at`** — yellow and
  red thresholds; below `warn_at` the segment is hidden.

## CLI

The binary reads the session JSON on stdin and prints the bar:

```
echo '{"workspace":{"current_dir":"'"$PWD"'"}}' | statusbar
statusbar --json   # {"statusline": "<plain>", "rendered": "<ansi>"} — handy for debugging
```

## Design

- **No daemon.** The binary is invoked fresh each refresh (cold-start ~ms in
  Rust). No ports, no background process.
- **Fail open.** Bad JSON, missing config, or no git yields fewer segments or an
  empty line — never a crash. A broken status line must not disrupt a session.
- **Pure core.** All rendering lives in `statusbar-core` and is unit-tested; the
  branch, `$HOME`, and clock are injected so the render stays deterministic.
