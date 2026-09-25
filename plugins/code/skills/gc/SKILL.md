---
name: gc
description: Reclaim disk space from a machine full of code checkouts — Rust target/ dirs, stale node_modules and virtualenvs, abandoned agent worktrees, merged branches, and package-manager / app-updater caches. Use for "free up disk space", "my disk is full", "clean up old builds", "what's bloating my code dir", or a periodic sweep. Drives the `code-gc` script; per-repo git judgement calls hand off to the git skill's optimize subskill.
---

# gc

Most of the bytes in a code directory are *regenerable*: build output,
installed dependencies, caches. Those go without ceremony. The few things that
hold real work — worktrees and branches — get checked before they go.

`code-gc` does the finding. It is read-only unless given `--apply`.

```
code-gc survey    [ROOT...]                 one line per category, sizes only
code-gc targets   [--apply] [ROOT...]       Rust target/ dirs (Cargo.toml beside them)
code-gc deps      [--days N] [--apply] [ROOT...]
                                            node_modules / venv / .venv in projects
                                            with no source edit in N days (default 30)
code-gc worktrees [ROOT...]                 linked worktrees: size, unmerged, dirty, branch
code-gc branches  [--apply] [ROOT...]       branches merged into main/master; --apply = git branch -d
```

ROOT defaults to `.`; pass the directory your checkouts live under (e.g. `~/code`).

## 1. Measure

```bash
df -h /System/Volumes/Data      # macOS; `df -h /` elsewhere
du -sh <root>/* ~/.claude | sort -rh | head -20
code-gc survey <root>
```

Say where the bytes are before deleting anything. On a Rust-heavy machine,
`target/` dirs are usually most of it — a single crate's can reach 10GB+.

## 2. Regenerable — delete after one confirmation

Present the list with sizes, get a single yes, then apply.

| What | Find | Reclaim |
| --- | --- | --- |
| Rust build output | `code-gc targets <root>` | `code-gc targets --apply <root>` |
| Stale dependencies | `code-gc deps <root>` | `code-gc deps --apply <root>` |
| Homebrew | — | `brew cleanup --prune=all -s` |
| Language caches | `du -sh ~/Library/Caches/{pip,pipenv,Yarn} ~/.cargo/registry` | `rm -rf` the dirs |
| App self-updaters | `du -sh ~/Library/Caches/*ShipIt* ~/Library/Caches/*updater*` | `rm -rf` the dirs |

- `targets` only matches a `target/` with a `Cargo.toml` beside it, so a Maven
  or unrelated `target/` is never touched. The cost of deleting is one rebuild.
- **Rebuild every `relink` line after applying.** A tool installed as a symlink
  into `target/release/` (a `make install` that runs `ln -s`) breaks the moment
  its target goes — including a status line binary, which fails silently.
  Rerun that repo's install, or `cargo build --release` for a bare link.
- `deps` treats a project as live if *any* source file changed within
  `--days`, so a project in active use keeps its `node_modules`.
- `*.ShipIt` dirs hold downloaded app updates that were already applied.
- Leave `~/.cargo/registry` unless space is desperate — every crate re-downloads.

## 3. Worktrees — classify, then remove

```bash
code-gc worktrees <root>
```

`unmerged` counts commits whose *patch* is not in main (`git cherry`), so a
squash-merged or cherry-picked branch correctly reads 0 even though
`--merged` would miss it.

| unmerged | dirty | Action |
| --- | --- | --- |
| 0 | 0 | remove: `git -C <repo> worktree remove <path>`, then `git branch -D <branch>` |
| > 0 or dirty | | keep; reclaim its `target/` (step 2 already found it) and ask the user |

A locked worktree needs `git worktree remove -f -f`; only do that once it
classifies as 0/0. For a repo that needs the full PR-aware treatment (open vs
closed-unmerged PRs, remote containment), use the git skill's
[optimize](../git/optimize.md) subskill, steps 5–6.

## 4. Branches

```bash
code-gc branches <root>           # preview
code-gc branches --apply <root>
```

`--apply` uses `git branch -d`, which also refuses a branch that has commits
its own upstream lacks — those errors are git being careful, not failures.
Squash-merged branches do not show up here; the optimize subskill finds those
through their PRs.

## 5. Report

End with before/after free space and what was kept on purpose — worktrees with
unmerged work, branches git refused — so the user can decide on those.
