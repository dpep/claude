---
name: find-gem
description: Locate an installed Ruby gem's source on disk and search inside it — "where is gem X installed", "read the source of gem Y", "how does gem Z implement ...", or any dive into a dependency's internals. Uses the `find-gem` CLI (bundle-aware). Not for finding code in the current repo — use rg/rq for that.
---

# find-gem

`find-gem` resolves where an installed gem's source lives and can search it in
one step — no `bundle show` plus hand-composed absolute path.

Use it to read a *dependency's* internals. For code in the current repo, use
`rg` (text) or `rq` (definitions).

```
find-gem <gem>                    print the gem's source dir
find-gem <gem> <pattern> [args]   rg <pattern> inside the gem; extra args go to rg
find-gem <gem> -- [args]          hand everything after `--` to rg verbatim
```

Resolution is bundle-aware: run it from a project with a Gemfile (or with
`BUNDLE_GEMFILE` set) to get the bundled version; anywhere else it falls back
to the newest installed version.

## Output

`-j/--json` and `-J/--ndjson` work on both forms. Locating emits
`{"gem":…,"dir":…}`; searching emits `{"file":…,"line":…,"text":…}` per
match — an array under `--json`, one per line under `--ndjson`.

Those four flags are find-gem's wherever they appear, so rg's colliding short
`-j` (`--threads`) never reaches rg — use rg's long forms. Past a `--` nothing
counts as a flag, which is how a dash-leading pattern gets through.

## Typical use

```sh
# where does the bundled graphql-client live?
BUNDLE_GEMFILE=<project>/Gemfile find-gem graphql-client

# find where a method is defined inside a gem
find-gem graphql-client "def define_class"

# case-insensitive, with rg flags passed through
find-gem activerecord -i "connection_pool" -g "*.rb"

# a pattern that starts with a dash
find-gem activerecord -- --frozen-string-literal

# structured hits to act on
find-gem graphql-client "def define_class" --json
```

Then Read specific files from the printed paths as usual.

## Installing the CLI

Inside Claude Code sessions the script is already on PATH (it ships in this
plugin's `bin/`). For regular shells, symlink it into `~/.claude/bin`:

```sh
make -C ~/.claude/plugins/marketplaces/dpep install
```

That links `find-gem` alongside the repo's other binaries. Working from your
own clone instead? `make install` from its root.
