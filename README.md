# dpep/claude

A small [Claude Code](https://claude.com/claude-code) plugin marketplace: the
parts of my setup that are useful to someone who isn't me.

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
| `git` | branches, PRs, worktrees, rebases |
| `rq` | find a definition by name, across Ruby/Rust/Go/Python/TypeScript |
| `gqls` | search a GraphQL schema by name or meaning, or jump to a resolver |
| `find-skill` | locate where a skill is defined on disk |
| `find-gem` | locate and search an installed Ruby gem's source |

The `rq` and `gqls` skills drive CLIs of the same name — install them first, or
those two skills have nothing to call:

```sh
brew install dpep/tools/rq dpep/tools/gqls
```

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

Each is a self-contained system prompt. Most will either critique a plan or do
the work, so they're useful as reviewers *and* as implementers — a staff
engineer that only ever has opinions isn't much help.

## Why so few?

Because the rest is personal. My full setup lives in a private repo and holds
memory, goals and org-specific vocabulary that would be noise to anyone else.
These two carry no data — just skills and prompts — which is what makes them
shareable.

## License

MIT — see [LICENSE](LICENSE).
