# dpep/claude

A small [Claude Code](https://claude.com/claude-code) plugin marketplace to accelerate your development.

```
/plugin marketplace add dpep/claude
```

It appears as **`dpep`**. Then install what you want from `/plugin`.

## Plugins

### `code`

Skills for the two questions that come up constantly while programming — *where
is this defined?* and *how do I do this in git?* — plus a couple of narrower
lookups.

| skill | for |
|---|---|
| `git` | branches, PRs, worktrees, rebases, optimizations |
| `rq` | find code definitions, across Ruby/Rust/Go/Python/TypeScript |
| `gqls` | search a GraphQL schema by name or meaning, or jump to a resolver |
| `find-skill` | locate a Claude skill definition on disk |
| `find-gem` | locate a Ruby gem's source |

The `rq` and `gqls` skills drive CLIs of the same name — install them first, or
those two skills have nothing to call:

```sh
brew install dpep/tools/rq dpep/tools/gqls
```

`find-skill`, `find-gem` and `statusbar` are built and installed from this repo:

```sh
git clone https://github.com/dpep/claude && cd claude
make install     # builds rust/, symlinks them into ~/.claude/bin
```

Add `~/.claude/bin` to your `PATH`.

## Building

The CLIs a few of them drive live in `rust/`:

```sh
make build     # cargo build --release --workspace
make check     # fmt-check + clippy -D warnings + tests — the gate
make install   # build, then symlink binaries onto PATH
make uninstall
```

### `personas`

Agents with a point of view:

**Craft and analysis** — `analyst`, `librarian`, `rubyist`, `scribe`.

**A decision council** — `product-manager`, `hacker`, `staff-engineer`,
`production-engineer`, `platform-expert`, `skeptic`, chaired by a `moderator`.

### `statusbar`

A compact status line: working directory, git branch or PR, session, model,
context-window use and rate-limit, configurable per segment.

It renders from a small Rust binary and is wired into `settings.json` as a
`statusLine` command. `/statusbar-install` does the whole thing — fetches or
builds the binary, writes the settings block, and smoke-tests the render:

```sh
cargo install --git https://github.com/dpep/claude statusbar-cli   # no clone needed
```

### `datasets`

Curated reference data Claude can find: markdown, CSV, TSV, JSON or YAML under
`~/.claude/datasets`, each with frontmatter describing what it is and when to
use it. Slash commands and a skill to create, list, search and show them.

Nothing to build or install beyond the plugin.

## License

MIT — see [LICENSE](LICENSE).
