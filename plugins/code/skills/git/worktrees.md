# Worktrees Subskill

Multi-agent coordination using git worktrees.

## Overview

Multiple Claude agents run in parallel, each in its own worktree. This requires coordination to avoid conflicts.

## Key Constraint

**Cannot checkout the same branch in multiple worktrees.** If a branch is checked out elsewhere, you'll get:
```
fatal: '<branch>' is already used by worktree at '/path/to/other/worktree'
```

Solution: Work in the worktree where the branch is checked out, or use a different branch.

## Coordination File

Location: `~/.claude/worktrees.md`

Tracks active work across all agents:

```markdown
| Remote Branch | Local Branch | Worktree | Task | Status | PR |
|---------------|--------------|----------|------|--------|-----|
| dpep/feature | dpep/feature-claude3 | claude3 | Add pagination | in progress | |
```

## Identifying Current Worktree

Derive from working directory basename:
- `~/code/myrepo-claude` → `claude`
- `~/code/myrepo-claude3` → `claude3`

## Workflow

### Before Starting Git Work

1. Read `~/.claude/worktrees.md` to see what's in flight
2. Either pick up an existing branch or create a new one
3. Update the file with your branch and task

### When Finishing or Pausing

1. Push changes to origin first
2. Update status in `worktrees.md` (completed, paused, waiting for review)
3. Include PR link if created

### Branch Naming with Worktrees

- Local branches can include worktree suffix: `dpep/feature-claude3`
- Remote branches use common name: `dpep/feature`
- Push with mapping: `git push origin dpep/feature-claude3:dpep/feature`
- Pull before starting, push when pausing - like two humans on the same PR

## Cleanup

After opening/updating a PR:

1. Check if any PRs in `worktrees.md` for this worktree have been merged
2. Check PR state: `gh pr view <url> --json state --jq '.state'`
   - Returns: `MERGED`, `OPEN`, or `CLOSED`
3. For merged PRs:
   - Delete local branch: `git branch -d <branch>`
   - Remove record from `worktrees.md`
4. Each agent only cleans up its own worktree's branches

## Working on Another Worktree's Branch

If you need to work on a branch checked out in another worktree:

Option 1: Navigate to that worktree
```bash
cd ~/code/myrepo-claude3
# work there
```

Option 2: Use GitHub API (for simple updates like rebasing)
```
mcp__github__update_pull_request_branch
```

Option 3: Ask the user to switch worktrees
