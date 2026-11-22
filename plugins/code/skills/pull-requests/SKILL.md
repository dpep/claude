---
name: pull-requests
description: Open, describe, update, and merge GitHub pull requests — open as a draft and mark ready on green CI, write a short title and a description that links every code reference to a pinned permalink, self-review against a quality checklist, read CONTRIBUTING, the AI policy, and any CLA before contributing to a repo you don't own, update with upstream without dismissing approvals, reply to review comments, and merge a stacked PR's base without closing the child. Drives `prlink`, which finds and links the code references in a PR body. Use for "open a PR", "create a draft PR", "mark ready for review", "PR title", "write the PR description", "link the references", "add permalinks", "update the PR", "update branch", "rebase the PR", "keep the approvals", "respond to review comments", "address the feedback", "stacked PR", "retarget the PR", "contribute upstream", or a PR to a repo or fork you don't own. Branching, commits, and rebasing a branch with no PR are the git skill.
allowed-tools: Bash(gh *), Bash(git *), Bash(prlink *)
---

# Pull requests

Check, open, describe, update, and merge GitHub PRs. Needs `gh`, authenticated.
Branching, commits, and rebasing a branch with no PR live in the `git` skill.

## Before opening: the change

- **Small and focused beats complete.** Smaller PRs get reviewed sooner and
  more carefully. On a repo you don't own, go further: see
  [Contributing upstream](#contributing-upstream).
- Change behavior *or* structure, not both at once.
- Handle similar cases the same way, and match the codebase's existing
  conventions unless the deviation is intentional.
- Prefer standard patterns over custom implementations.
- Avoid duplication; reuse or extract a helper.
- Limit hardcoded values: use a constant, config, or a dynamic lookup.
- Question necessity: remove dead code and unnecessary abstractions. When
  deleting code, hunt the orphans it leaves: unused translation keys,
  unreachable methods, constants nobody references.
- Test quality over quantity: don't test one thing twice; spend the effort on
  edge cases and error handling.
- Keep test descriptions simple and resilient ("raises an error", not "raises
  SpecificError").

## Open a draft

On a repo you don't own, read [Contributing upstream](#contributing-upstream)
before preparing the PR.

1. **Check where a new branch's first push goes.** With
   `push.default=upstream`, a branch cut from `main` inherits `origin/main` as
   its upstream, so `git push -u origin feature` pushes to `main` and skips the
   PR's CI. Check `git config push.default`, or push with an explicit refspec:
   `git push -u origin HEAD:refs/heads/feature`.
2. **Open as a draft**, most of all on repos you don't own. Write the body to a
   scratch file, e.g. `/tmp/pr-<branch>.md`, never inline: inline bodies hit
   shell quoting.

   ```bash
   gh pr create --draft --body-file /tmp/pr-<branch>.md
   gh pr edit <num> --body-file /tmp/pr-<branch>.md
   ```

3. **Mark it ready once CI is green** (`gh pr ready <num>`). A PR isn't ready
   for review before that.
4. Share the PR URL as a link.

## Title

- Aim for under 50 characters; GitHub truncates around 70.
- Prefix with the ticket key when there is one: `[PROJ-123] Fix validation`.
- Say what changed, not how. Bad: "Replace sleep 0.1 with proper Capybara wait
  in onboarding spec". Good: "Fix flaky onboarding spec".

## Description

- Use `.github/pull_request_template.md` when the repo has one.
- No AI-attribution footer ("Generated with …") unless the repo's AI policy
  asks for disclosure.

## Link every reference

Link the resources that help a reviewer, or a later reader of the history, and
do it in the same pass as the prose. A bare name reads fine, but the reviewer
can't jump to it, and links promised for later don't get added.

| Reference | Link as |
|-----------|---------|
| Method / class / file | `https://github.com/<org>/<repo>/blob/<short_sha>/<path>#L<line>` |
| Test shared context or example | its definition file |
| Ticket | your tracker's issue URL |
| PR | `https://github.com/<org>/<repo>/pull/<number>` |
| GitHub user | `@<handle>`, which notifies that person: use it only for someone you mean to notify |

Pin code permalinks to a **short commit SHA, never a branch**: a branch moves
and its line numbers rot.

Link **dependency code** too: a framework method, a gem's class. Pin to the
version actually resolved (the lockfile's git revision or release tag), and
take line numbers from the installed source (the plugin's `find-gem` for Ruby
gems), not from memory.

### With prlink

`prlink` does the linking. It finds every code reference in the body
(backticked paths, constants, `Owner#method`, `Mod::fn`, bare names,
`path:line` tokens, existing GitHub links), resolves each with trekr (Ruby) or
rq (other languages), and pins it at the base sha when the line is unchanged
there, else at the head (fork) sha. Run it from the clone, or pass `-C DIR`.
`prlink --help` lists every flag, including `--ndjson` and `--explain`.

1. **Report.** Before `gh pr create`, run `prlink --body /tmp/pr-<branch>.md
   --json` on the draft body (`--base REV` when the base isn't `origin/HEAD`;
   any rev works, e.g. `HEAD~1`). After, run `prlink --pr <num> --json` on the
   open PR. Both are read-only: one row per ref with its `status`, `reason`,
   `candidates` (url, side, path, line, tool, and the line's text as
   `evidence`) and the `chosen` link.
2. **Review** the rows that aren't `linked` or `existing_ok`:
   - `ambiguous` lists candidates: pick one, or write the ref more precisely.
     A plain word (`price`) is never linked on its own; `price()` or
     `Item#price` is.
   - `unresolved` says `external` (a dependency, core, or stdlib: link the
     installed source by hand, see [Link every reference](#link-every-reference)),
     `not_found`, `tool_missing`, or `warming` (rq's index is still building:
     rerun).
   - `existing_wrong` carries a fix when prlink is sure; `existing_stale` is a
     sha the clone lacks; `existing_unpinned` is a branch link.
   - `left_alone` says why (a path in plain text: backtick it).
3. **Apply.** Rerun without `--json` for the linked body on stdout (report on
   stderr), or with `--write` to save it (`gh pr edit` for `--pr`). Then add
   the links you chose by hand. `head_pushed: false` means the head's links
   would 404: push first (`--write` refuses without `--allow-unpushed`).
4. **Verify.** Scan the body for backticked `Const` or `#method` names that
   still aren't links.

Exit 0: nothing to review. 1: refs to review, or an unpushed head. 2: a usage
or operational error. A rerun on its own output changes nothing.

### Setup

- `prlink` ships in this plugin's `bin/`, so it is on PATH inside Claude Code.
  Outside Claude Code, see the plugin README's install notes. Ruby 2.6+,
  stdlib only.
- `prlink --pr` needs `gh`; `prlink --body FILE` runs without it.
- Optional, for resolving code references:
  - `trekr` for Ruby repos: `brew install dpep/tools/trekr` (or `cargo install trekr`)
  - `rq` for Rust, Go, Python, TypeScript and JavaScript: `brew install dpep/tools/rq`
    (or `cargo install reference-query`)

  Without them, prlink still links file paths and verifies existing GitHub
  links, which need only git. A name it would resolve with a missing tool
  comes back `unresolved` with cause `tool_missing`, and the report names the
  tool.

## Contributing upstream

External repos, plugins, shared tooling, another team's code:

- **Read the rules first.** Before preparing the PR, read CONTRIBUTING, the AI
  policy (the repo's and the org's `.github` repo), and any CLA. Some projects
  forbid agent-submitted PRs or AI-written descriptions; some refuse
  refactor-only PRs.
- **Minimize the diff** to what the change strictly needs. Don't refactor or
  tidy the surroundings in the same PR; save those observations for a
  follow-up PR or a comment.

## Update with upstream

For a **non-draft PR in review or queued to merge**, use GitHub's server-side
update. It creates a merge commit and keeps existing approvals, which a local
rebase and force-push would dismiss:

```bash
gh pr update-branch <num>
```

It only handles a clean merge, and it moves the remote ahead of your local
copy. So:

- **Clean:** after the server-side update, sync locally: `git pull --rebase`.
- **Conflicts:** there is no server-side update. Rebase locally, resolve, and
  push; the push is the update:

  ```bash
  git fetch origin main
  git rebase origin/main
  # resolve, git rebase --continue
  git push --force-with-lease
  ```

For a **draft PR** there are no approvals to protect: rebase locally and
`git push --force-with-lease`.

## Respond to review comments

When you act on an inline review comment, reply to that comment saying what
changed.

## Merge a stacked PR's base

`gh pr merge --delete-branch` on a PR that another PR is stacked on deletes
the child's base branch, and GitHub then *closes* the child. A closed PR can't
be retargeted, nor reopened once its head has been force-pushed. Retarget the
child first: `gh pr edit <child> --base main`.

To recover: push the old base branch back, reset the child's head to its
pre-close sha, reopen, retarget, then push the new head.
