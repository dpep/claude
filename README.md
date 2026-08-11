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

`find-skill` and `find-gem` are built and installed from this repo:

```sh
git clone https://github.com/dpep/claude && cd claude
make install     # builds rust/, symlinks both into ~/.claude/bin
```

Add `~/.claude/bin` to your `PATH` if it isn't already — `make install` says so
if it's missing.

## Building

Skills are markdown and need no build. The CLIs a few of them drive live in
`rust/`, as one Cargo workspace:

```sh
make build     # cargo build --release --workspace
make check     # fmt-check + clippy -D warnings + tests — the gate
make install   # build, then symlink binaries onto PATH
make uninstall
```

Small one-off tools live here rather than in repos of their own; a tool doesn't
each earn a repo, a release pipeline and a formula. Substantial standalone ones
do — [rq](https://github.com/dpep/rq) and
[gqls](https://github.com/dpep/gqls) have their own, and ship their own skills.

Every crate here builds from a clone of *this* repo alone: no path
dependencies pointing outside the workspace. That constraint is what keeps
these plugins genuinely installable by someone who isn't me.

Both are open source ([rq](https://github.com/dpep/rq),
[gqls](https://github.com/dpep/gqls)) and each ships its own copy of its skill,
so the versions here track the tools.

The `git` skill encodes *my* conventions — branch prefixes, where repos live —
and says so where it does. Adapt those bits or ignore them; the rest is general.

### `personas`

Agents with a point of view, summonable in any project. Two groups:

**Craft and analysis** — `analyst`, `librarian`, `rubyist`, `scribe`.

**A decision council** — `product-manager`, `hacker`, `staff-engineer`,
`production-engineer`, `platform-expert`, `skeptic`, chaired by a `moderator`
that sequences them to avoid anchoring and forces the tradeoffs into the open.

## License

MIT — see [LICENSE](LICENSE).
