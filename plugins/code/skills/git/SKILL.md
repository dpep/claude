---
name: git
description: Git operations, branching, PRs, and rebasing. Activate when starting new git work, creating or updating PRs, or resolving merge conflicts.
allowed-tools: Bash(git *), Bash(gh *), Bash(open *), ToolSearch, mcp__github__*
---

# Git Skill

Git operations, branching, PRs, and rebasing.

## Trigger

Use when:
- Starting new git work or creating branches
- Working with PRs (creating, updating, rebasing)
- Resolving merge conflicts

## Subskills

- [optimize](./optimize.md) - Pack consolidation, branch cleanup, fetch refspec narrowing

## References

- [worktrees](./worktrees.md) - Multi-agent worktree coordination (load on demand)

## Command Shorthands

- `rebase <PR link>` — checkout PR branch, fetch latest main, rebase onto main, resolve conflicts, push

## Branch Naming

- Format: `dpep/<topic>`
- Keep it short and descriptive

## Using origin/main (No Local Main)

Never checkout `main` locally. Use `origin/main` for everything:

```bash
# Fetch latest
git fetch origin main

# Create branch from main
git checkout -b dpep/feature origin/main

# Rebase onto main
git rebase origin/main

# Compare with main
git diff origin/main

# Reset to main (careful!)
git reset --hard origin/main
```

## Starting New Work

1. Fetch latest: `git fetch origin main`
2. Create branch: `git checkout -b dpep/topic origin/main`
3. Make changes, commit, push
4. Create PR

## Rebasing a PR

```bash
git fetch origin main <branch-name>
git checkout <branch-name>
git rebase origin/main
# Resolve conflicts if any
git push --force-with-lease origin <branch-name>
```

Or use GitHub API for simple cases:
```
mcp__github__update_pull_request_branch
```

## Commit Granularity

Commit incrementally. When work splits into independent units (docs vs bugfix vs
feature), make each its own logical commit rather than batching them into one
tree — even when pushing straight to main. Small, logically-connected commits are
easier to review, revert, and bisect. When work has clearly separable phases, plan
the commit boundaries up front and commit as each unit lands.

## Commit Messages

- Use heredoc to avoid escaping issues:
```bash
git commit -F - <<'EOF'
Summary line

Body text here.
EOF
```

## Pull Requests

### Creating

- ALWAYS open in draft mode: `gh pr create --draft` — no exceptions, especially
  external repos (mark ready once CI passes)
- Keep changes small and focused
- Prefix with JIRA ticket if applicable: `[PROJ-123] Fix thing`
- After creating, open in browser: `open <pr_url>`

### Updating PRs with Upstream

For a **non-draft PR already in review or queued for merge**, default to GitHub's
server-side "Update branch" — one line, creates a merge commit, and preserves all
existing approvals (a local rebase + force-push would invalidate them):

```bash
gh pr update-branch <num>          # or the GitHub API/MCP equivalent
```
```
mcp__github__update_pull_request_branch
```

Rebase locally too — for two reasons: `gh pr update-branch` only fast-forwards a
clean merge (it can't resolve conflicts), and it moves the *remote* ahead, leaving
your local copy behind. Rebasing keeps your branch current and roughly in sync with
the remote:

```bash
git fetch origin main
git rebase origin/main
# If conflicts: resolve them, then continue and push the resolution back up
git push --force-with-lease        # required after a rebase rewrites history
```

When conflicts exist, resolving them locally and pushing **is** the update — there's
no server-side button for that. Push so the PR carries the resolution.

After a clean server-side `gh pr update-branch` (no conflicts), still sync your local
copy so it isn't behind the remote: `git fetch origin <branch> && git rebase` (or
`git pull --rebase`).

For a **draft PR** (no approvals to protect), a local rebase + `--force-with-lease`
keeps history linear and is fine.

## Shared Repo, Multiple Sessions

Several Claude sessions may be working in one clone at the same time, and the
user edits files by hand too. The working tree is not yours alone.

**Stage paths, never everything.** `git add -A`, `git add .`, and `git commit -a`
take whatever is in the tree, including work someone else has in flight.

```bash
git add src/thing.rs tests/thing.rs   # what you changed
git commit -F - <<'EOF'
...
EOF
```

**Then read the staged diff before committing.** Dropping a foreign *file* from
the index is the easy half; the trap is a foreign *hunk inside a file you are
also editing*. A session added its skill to a shared manifest while another
session was editing the same manifest for its own skill — unstaging the obvious
new file left the manifest lines behind, they shipped in the wrong commit, and
the other session had to back them out.

```bash
git status --short      # anything you did not touch is not yours
git diff --cached       # every hunk should be one you wrote
```

If a hunk you did not write is staged, `git restore --staged <path>` and add
your paths back explicitly.

**Never run tree-wide destructive commands.** `git stash`, `git checkout -- .`,
`git restore .`, and `git reset --hard` are repo-global — they take or destroy
every session's uncommitted work, not just yours. If you need a clean tree,
scope it to your own paths.

**Foreign changes inside your files:** read them before deciding. A "file was
modified" system reminder, or a hunk you do not recognize, is usually the user
editing by hand or a linter — often something that *should* be integrated into
your work rather than reverted. If it plainly belongs to what you are doing,
keep it and say so. **If it is ambiguous, ask** — do not guess, and do not
revert it silently.

**Expect collisions in shared files.** Version numbers, manifests, changelogs,
and lockfiles are where two sessions land on the same line. Re-read the file
immediately before editing it rather than trusting what you read earlier, and
re-check after a long-running task.

`ListAgents` shows peer sessions; a busy one in your repo is a reason to be
careful. When the work is genuinely parallel, a worktree removes the problem
entirely — see [worktrees](./worktrees.md).

## Cross-Repo Work

`cd <path> && git ...` triggers a permission prompt. Use `git -C <path>` instead so the working directory stays put:

```bash
# Bad - triggers permission prompt
cd ~/code/other-repo && git push

# Good - no cd, no prompt
git -C ~/code/other-repo push
```

`gh` commands need `--repo <owner>/<name>` plus explicit `--head <branch> --base main` when run from a different repo's working dir — otherwise it picks up the current dir's branch/SHA.

## Keeping Output Small

Large git output fills context fast. Default to scoped, summarized forms:

```bash
# Log: always limit and use oneline
git log --oneline -20
git log --oneline origin/main..HEAD   # just this branch

# Diff: stat first, full diff only if needed
git diff --stat                        # overview of what changed
git diff --stat origin/main           # vs main
git diff path/to/file.rb              # scope to one file

# Show: summarize a commit before expanding
git show --stat <sha>
```

Avoid bare `git log`, `git log -p`, or `git diff` without a path — these can return thousands of lines.

## Dangerous Operations

Always confirm before:
- `git reset --hard`
- `git push --force` (prefer `--force-with-lease`)
- `git branch -D`
- Deleting remote branches
