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
