# Git Optimize Subskill

Run this when git feels slow, disk is filling, or branches and worktrees are
stale. Covers pack consolidation, worktree and branch cleanup, and fetch refspec
narrowing.

## 1. Sync main

Always sync `origin/main` before anything else. Stale main makes "merged into
main" checks miss recently-merged branches, so cleanup decisions go wrong.

```bash
git rev-parse --abbrev-ref HEAD   # which branch am I on?
```

- On `main` and behind → pull:

  ```bash
  git pull --ff-only origin main
  ```

- On a feature branch → at minimum refresh `origin/main`:

  ```bash
  git fetch origin main
  ```

## 2. Diagnose

```bash
git count-objects -v       # check packs: count, size-pack
du -sh .git/               # total repo size
time git status            # baseline speed
git fsmonitor--daemon status 2>&1  # check if fsmonitor is running
git config --get maintenance.auto  # false on a bloated repo = almost certainly the cause
```

Red flags:
- `packs:` > 10 → consolidate
- `git status` > 0.3s → investigate
- fsmonitor daemon running but causing problems → disable it
- `maintenance.auto` is `false` on a bloated repo → almost certainly the cause (see step 3)

**Trap:** repos registered via `git maintenance start` get a *local* `maintenance.auto=false`
on purpose (to defer to the scheduled run). To inherit the global `true`, you must
`git config --unset maintenance.auto` locally — setting global true alone won't override it.

### Partial-clone fsck false positives

On a partial clone (`--filter=blob:...`), `git fsck` reports expected noise — don't
mistake it for corruption:

- `broken link from tree to blob` / `missing blob` → **expected**: the blob was filtered
  at clone time and not yet fetched. Not damage.

Filter for *real* damage only:

```bash
git fsck --no-dangling 2>&1 | awk '/to    tree/{tt++} /bad /{bad++} /failed to parse/{fp++} END{print "tree->tree:",tt+0,"bad:",bad+0,"failed-parse:",fp+0}'
```

- `bad` or `failed-parse` non-zero = real object-DB corruption (see step 3 recovery).
- `tree->tree` non-zero = **maybe recoverable, not necessarily corruption.** On a partial
  clone the tree may simply have been filtered/lazy and never promisor-fetched. Try the
  lazy-fetch recovery below *before* declaring corruption — it's a few minutes and avoids a
  re-clone.

### Lazy-fetch recovery (try before re-clone)

Touching each missing tree with `git cat-file -t` + `git ls-tree` triggers an on-demand
promisor fetch. If the remote still has the object, gc then succeeds without re-cloning.

```bash
git fsck --no-dangling 2>&1 \
  | awk '/^broken link from/{getline; if($0 ~ /to    tree/) print $NF}' \
  > /tmp/missing-trees.txt

while read sha; do
  git cat-file -t "$sha" > /dev/null 2>&1 && git ls-tree "$sha" > /dev/null 2>&1
  echo "$sha: $?"
done < /tmp/missing-trees.txt

# Re-run the fsck filter above — if tree->tree is now 0, proceed to gc (step 3).
```

If lazy-fetch clears it, gc succeeds and re-clone is avoided. Only after this fails should
`tree->tree` be treated as real corruption.

## 3. Consolidate Pack Files

257 packs → 1 makes a big difference for `git log`, `git diff`, rebase, object lookups.

```bash
git gc --prune=now
```

This runs in the foreground and takes several minutes on large repos (7GB+). Run in
background if needed — on a 62GB `.git` with 65 packs of ~2GB each, `git gc --prune=now`
took ~15 minutes.

**Never `git gc --aggressive` on a partial clone** — it can trigger refetches of filtered
blobs and undo the savings.

Re-enable auto-maintenance so packs stay consolidated over time:

```bash
git config maintenance.auto true
```

### When gc fails with "in the commit graph file but not in the object database"

This is a known partial-clone corruption mode: the commit-graph references commits orphaned
by force-pushes that the partial clone can't refetch (the promisor remote rejects them).

1. Delete the commit-graph cache first — it auto-regenerates:

   ```bash
   trash <repo>/.git/objects/info/commit-graphs   # or: rm -rf
   ```

   The new chained format lives in the `commit-graphs/` subdir. The old format put files at
   `info/commit-graph` and `info/graph-*.graph` — different paths, delete whichever exists.

2. Re-run `git gc --prune=now`. If it now fails with `bad tree object <sha>`, that's *real*
   object-DB corruption — gc cannot fix it on a partial clone (the promisor can't refetch the
   lost tree). **Re-clone is the only path** (see below).

### Re-clone (preserve the original partial-clone filter)

Match the filter the repo was originally cloned with — check it first:

```bash
git config --get-all remote.origin.partialclonefilter   # e.g. blob:limit=100k
git clone --filter=blob:limit=100k <remote-url> <new-path>
```

## 4. Disable fsmonitor (if causing problems)

fsmonitor can cause issues on some setups. To disable:

```bash
git fsmonitor--daemon stop
git config core.fsmonitor false
```

Note: `~/.gitconfig` has `core.fsmonitor=false` globally. Local repo config overrides this,
so check `git config --local core.fsmonitor` if behavior is unexpected.

### fsmonitor daemon hanging another tool (e.g. `brew update`)

Symptom: `brew update` (or any tool that takes a flock) hangs forever with no brew/git
process obviously running. Cause: when git spawns `git fsmonitor--daemon`, the daemon
**inherits all open file descriptors** from its parent — including a lock fd the caller was
holding. The daemon outlives the caller and keeps the `flock` held indefinitely, so every
later run blocks waiting on it.

Diagnose — find who actually holds the lock:

```bash
lsof /opt/homebrew/var/homebrew/locks/update   # COMMAND is `git`, FD like `200w`
ps -o pid,ppid,lstart,command -p <PID>         # reveals a fsmonitor--daemon, PPID 1
```

Fix:

```bash
kill <PID>                                         # releases the inherited lock fd
git -C /opt/homebrew config core.fsmonitor false   # stop the repo respawning it
```

A stale daemon may ignore SIGTERM, and `git fsmonitor--daemon stop` won't recognize it once
config is `false` — escalate to `kill -9 <PID>`. fsmonitor is a per-repo speed optimization
(FSEvents-backed `git status`) that only pays off on large repos (`git status` > 0.3s); it's
safe to leave disabled on small ones. To sweep orphaned daemons across all repos:
`pgrep -fl fsmonitor--daemon`, then kill the ones whose repos don't need it.

## 5. Garbage-Collect Worktrees

Parallel background agents leave worktrees behind, and every one is a full
checkout — a dozen abandoned ones will fill a disk while `.git` itself looks
fine. Do this **before** step 6: `git branch -D` refuses a branch that is still
checked out in a worktree.

Removing worktrees reclaims *working-tree* bytes only. They share one object DB,
so a bloated `.git` is step 3's problem, not this one — check both before
concluding which is eating the disk.

```bash
git worktree list --porcelain     # path, HEAD, branch, and bare/detached/locked/prunable
du -sh <path> <path> …            # one call, all the paths — where the bytes actually are
```

Start with the free win — records whose directory is already gone:

```bash
git worktree prune -v --dry-run
git worktree prune -v
```

That only drops administrative records; it never reclaims disk for a worktree
that still exists. Its `--expire <time>` is no shortcut either — it bounds which
*missing* worktrees get pruned, and ignores live ones however old. The rest need
classifying.

### Classify each remaining worktree

Skip two that are never candidates: the **primary** worktree (first in the list,
the repo root) and the one you are **currently running in**.

For each of the others, gather the age and the two containment refs:

```bash
git -C <wt> log -1 --format=%ct                 # last commit, epoch seconds
git -C <wt> status --porcelain                  # any output = dirty
git -C <wt> rev-parse HEAD
git -C <wt> rev-parse --verify -q origin/<branch>   # exit 1, no output = no remote branch
gh pr list --head <branch> --state all --json number,state,headRefOid
```

**Stale = no commit for two weeks.** That is the whole test, and it only asks one
question: is anyone still using this worktree. Treat the threshold as a dial, not
a law — say which one you used.

Staleness makes a worktree a *candidate*. What decides how you remove it is
whether its work exists anywhere else:

| Is local `HEAD` contained off-machine, tree clean? | Action |
| --- | --- |
| Yes — in a PR head or in `origin/<branch>` | remove; nothing is lost |
| No | **confirm**, and offer to push it somewhere first |

**Containment is about reconstructability, not merge status.** Both refs count and
for the same reason — the commits survive the worktree. A remote branch proves it
as well as a PR does; don't wait for git's merge detection to agree either, since
a squash-merged or rebased PR leaves no ancestry for `--merged` to find, which is
exactly the case that strands worktrees for months.

Check both refs, because the common end state has only one of them. Merged PR plus
`delete_branch_on_merge` (the repo convention — see the `github` skill) means
`origin/<branch>` is gone while the PR still holds the commits; a pushed branch
with no PR yet is the mirror case.

```bash
git -C <wt> merge-base --is-ancestor HEAD <pr_head_oid>       # exit 0 = contained
git -C <wt> merge-base --is-ancestor HEAD origin/<branch>     # either one suffices
```

Test containment against those rather than `@{upstream}`, which gets two cases
wrong: the branch was advanced from another worktree (local is *behind* — still
contained, still safe) and there is no upstream ref at all (it just errors). If
local is **ahead** of both, it carries commits nothing else has — that is the
confirm row.

Nothing contained is ever unsafe to remove, stale or not. So in an actual space
crunch, containment alone licenses removal — a contained worktree costs one
`git worktree add` to bring back. Staleness is just what makes it uncontroversial.

### Remove

Present the table — path, size, branch, PR or remote ref, last activity — then:

```bash
git worktree remove <path>            # refuses on dirty or untracked files, or if locked
git worktree unlock <path>            # only if it is locked and you mean it
```

Remove contained worktrees without asking; get explicit sign-off for anything in
the confirm row. `--force` overrides both the dirty-tree refusal and the lock, so it
throws away exactly the evidence the classification depends on — never reach for it on a
worktree you have not classified, and never on a dirty one that failed the
containment check.

Removal leaves the branch alone; step 6 decides that separately. If the worktree
had a row in `~/.claude/worktrees.md` (see [worktrees](./worktrees.md)), drop it
in the same pass.

## 6. Prune Stale Branches

### Fetch and prune deleted remote branches

```bash
git fetch --prune
```

### Categorize each local branch by its PR

A `: gone]` remote does **not** reliably mean "merged" — a PR closed without
merging also deletes the remote, and deleting the local branch then loses the
work. Look up each branch's PR before deciding.

For every local branch other than `main`:

```bash
gh pr list --state all --head <branch> --json number,state,title,mergedAt,closedAt
```

Categorize:

| PR state | Action |
| --- | --- |
| OPEN | keep |
| MERGED | safe to delete (`git branch -D`) |
| CLOSED (not merged) | **confirm** before delete — work would be lost |
| No PR found + remote gone | **confirm** before delete |

Build a table of branch, remote status (`: gone]` or tracked), PR number/state,
and proposed action. Present it and **wait for confirmation** before any
destructive `git branch -D`. Only delete MERGED branches without asking; for
CLOSED and no-PR branches, get explicit sign-off first.

## 7. Narrow Fetch Refspec

By default git fetches all remote branches. Narrow to only your own branches:

```bash
git config remote.origin.fetch "+refs/heads/main:refs/remotes/origin/main"
git config --add remote.origin.fetch "+refs/heads/dp/*:refs/remotes/origin/dp/*"
git config --add remote.origin.fetch "+refs/heads/dpep/*:refs/remotes/origin/dpep/*"
```

To fetch a teammate's branch explicitly when needed:

```bash
git fetch origin their-branch
git checkout their-branch
```

To verify current refspecs:

```bash
git config --get-all remote.origin.fetch
```

### Clean up tracking refs the narrowed refspec no longer matches

Narrowing the refspec does **not** remove existing tracking refs. `git fetch --prune` only
drops refs that were fetched before and are now gone upstream — it leaves refs that simply
stopped *matching* the new refspec. After narrowing, thousands of stale
`refs/remotes/origin/*` can linger and keep `packed-refs` huge. Bulk-delete the ones outside
the new refspec:

```bash
# Adjust the regex to your kept prefixes (main, HEAD, your branch namespace).
git for-each-ref --format='%(refname)' refs/remotes/origin/ \
  | grep -v -E '^refs/remotes/origin/(main|HEAD|dpep/)' \
  | sed 's/^/delete /' \
  | git update-ref --stdin
```

`git update-ref --stdin` is the right batch interface — `xargs -n1 git update-ref -d` works
but spawns one process per ref (tens of thousands of them).

### Prune `git maintenance` prefetch refs too

`maintenance.auto=true` populates `refs/prefetch/remotes/origin/*` as a fetch cache. These
are **not** pruned to match a narrowed refspec either, and they're invisible in normal
`git branch` / `git for-each-ref` output unless you look under `refs/prefetch/` — so you
won't notice them without checking. Same fix:

```bash
git for-each-ref --format='%(refname)' refs/prefetch/ \
  | grep -v -E '^refs/prefetch/remotes/origin/(main|dpep/)' \
  | sed 's/^/delete /' \
  | git update-ref --stdin
```

### Compact `packed-refs` after any bulk ref delete

`git update-ref --stdin delete …` removes refs but doesn't compact `packed-refs` — after
deleting tens of thousands of refs the file can still be megabytes. One pass shrinks it by
an order of magnitude:

```bash
git pack-refs --all --prune
```

Run this as the final step after any bulk-ref-delete operation above.

## Partial Clones Used by Parallel Claude Agents

Running multiple Claude agents against the same `.git` invites races during on-demand
partial-clone blob fetches, which is how these repos get corrupted in the first place.

### Recommended baseline config

```bash
git config --global maintenance.auto true
git config --global core.commitGraph false
```

`commitGraph false` dodges the corruption mode in step 3 — the commit-graph references
commits orphaned by force-pushes that a partial clone cannot refetch, after which gc fails
with "in the commit graph file but not in the object database" pointing at a SHA the
promisor remote rejects.

### Auto vs scheduled maintenance: pick one

Running both collides — the scheduled incremental-repack races foreground fetches driven by
agents. For daily-use repos, **auto alone is sufficient**. To disable scheduled maintenance:

```bash
for r in <each-repo>; do git -C "$r" maintenance unregister; done
git maintenance stop
```

### Isolation and cattle

- **One clone per agent** isolates the blast radius. Worktrees do **not** help here — they
  share the object DB, so a race corrupts all of them.
- Treat agent clones as **cattle, not pets**: re-clone proactively every few weeks rather
  than waiting for corruption to force the issue.
- Cattle still need culling. Background agents abandon worktrees faster than anyone
  notices, and each is a full checkout — sweep them with step 5 on a schedule, not
  once the disk is full.

## Applying to Other Repos

Run steps 1-6 in each clone under `~/code/`. Step 7 (refspec) is per-repo
and needs to be applied to each clone separately.

```bash
cd ~/code/<repo>
git pull --ff-only origin main   # sync main first (step 1)
git gc --prune=now
git config maintenance.auto true
git fetch --prune
git worktree prune -v            # then classify what is left — step 5
# Branch cleanup: use the PR-aware flow in step 6 — do NOT blanket-delete
# ": gone]" branches. Look up each branch's PR and confirm before deleting
# anything not cleanly merged.
```
