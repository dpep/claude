# Git Optimize Subskill

Run this when git feels slow or branches are stale. Covers pack consolidation,
branch cleanup, and fetch refspec narrowing.

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

## 5. Prune Stale Branches

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

## 6. Narrow Fetch Refspec

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

## Applying to Other Repos

Run steps 1-5 in each clone under `~/code/`. Step 6 (refspec) is per-repo
and needs to be applied to each clone separately.

```bash
cd ~/code/<repo>
git pull --ff-only origin main   # sync main first (step 1)
git gc --prune=now
git config maintenance.auto true
git fetch --prune
# Branch cleanup: use the PR-aware flow in step 5 — do NOT blanket-delete
# ": gone]" branches. Look up each branch's PR and confirm before deleting
# anything not cleanly merged.
```
