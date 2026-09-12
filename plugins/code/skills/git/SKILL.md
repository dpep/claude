---
name: git
description: Git operations, branching, commits, and rebasing. Activate when starting new git work, committing, rebasing a branch onto main, or resolving merge conflicts. Pull requests are the github skill, not this one.
allowed-tools: Bash(git *), Bash(gh *), Bash(open *), ToolSearch, mcp__github__*
---

# Git Skill

Git operations, branching, commits, and rebasing.

## Trigger

Use when:
- Starting new git work or creating branches
- Committing, rebasing a branch onto main
- Resolving merge conflicts

Opening, describing, or updating a **pull request** is the `github` skill — this
one stops at the push.

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
4. Open the PR — see the `github` skill

## Rebasing a Branch

```bash
git fetch origin main <branch-name>
git checkout <branch-name>
git rebase origin/main
# Resolve conflicts if any
git push --force-with-lease origin <branch-name>
```

For a branch that already has an **open PR**, a local rebase isn't always the right
move — it invalidates approvals. See the `github` skill.

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
