# dpep/claude

A [Claude Code](https://claude.com/claude-code) plugin marketplace: code
navigation, a bench of opinionated agents, a status line, and a couple of small
conveniences.

```
/plugin marketplace add dpep/claude
```

It shows up as **`dpep`** — install what you want from `/plugin`.

## Plugins

### `code`

Skills for the two questions that come up constantly while programming — *where
is this defined?* and *how do I do this in git?* — plus a couple of narrower
lookups.

| skill | for |
|---|---|
| `git` | branches, PRs, worktrees, rebases, optimizations |
| `rq` | find code definitions, across Ruby/Rust/Go/Python/TypeScript/JavaScript |
| `gqls` | search a GraphQL schema by name or meaning, or jump to a resolver |
| `find-skill` | locate a Claude skill definition on disk |
| `find-gem` | locate a Ruby gem's source |

`rq` and `gqls` drive CLIs of the same name. Without those, the two skills have
nothing to call:

```sh
brew install dpep/tools/rq dpep/tools/gqls
```

`find-skill`, `find-gem` and `statusbar` come from this repo, and adding the
marketplace already cloned it — nothing to fetch:

```sh
make -C ~/.claude/plugins/marketplaces/dpep install
```

That builds `rust/` and symlinks the binaries into `~/.claude/bin` — make sure
that's on your `PATH`. Working from your own clone? Run `make install` from its
root.

### `personas`

Agents with a point of view. Each is a self-contained system prompt: summon one
for a critique, or hand it the work.

**Craft and analysis** — `analyst`, `librarian`, `rubyist`, `scribe`.

**A decision council** — `product-manager`, `hacker`, `staff-engineer`,
`production-engineer`, `platform-expert`, `skeptic`, chaired by a `moderator`.

### `statusbar`

A compact status line: working directory, git branch or PR, session, model,
context-window use and rate-limit, configurable per segment.

It renders from a small Rust binary — the same `make install` as above — wired
into `settings.json` as a `statusLine` command. `/statusbar-install` does the
whole thing: builds it, writes the settings block, and smoke-tests the render.

### `datasets`

Curated reference data Claude can find: markdown, CSV, TSV, JSON or YAML under
`~/.claude/datasets`, each with frontmatter describing what it is and when to
use it. Slash commands and a skill to create, list, search and show them.

Nothing to build — the plugin is the whole thing.

## Building

The `make install` above is one of several targets. They all run from the repo
root and operate on the CLIs in `rust/`:

```sh
make build     # cargo build --release --workspace
make check     # fmt-check + clippy -D warnings + tests — the gate
make install   # build, then symlink binaries onto PATH
make uninstall
```

## License

MIT — see [LICENSE](LICENSE).
