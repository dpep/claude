---
name: pull-requests
description: Open, describe, update, and shepherd GitHub pull requests — always as a draft, with a short title, a description that links every code reference to a pinned permalink, a quality checklist, the CONTRIBUTING/AI-policy/CLA check before contributing upstream, updating with upstream without losing approvals, and replying to review comments. Drives `prlink`, which finds and links the code references in a PR body. Use for "open a PR", "create a pull request", "write the PR description", "update the PR", "link the references", "add permalinks", "rebase the PR", "respond to review comments", "address the feedback", or contributing to a repo you don't own. Branching, commits, and rebasing themselves are the git skill.
---

# Pull requests

Opening, describing, and updating PRs. Git-side mechanics (branching, commits,
rebasing) live in the `git` skill.

## Prerequisites

- `gh`, authenticated: creating and editing PRs, and `prlink --pr`.
  `prlink --body FILE` runs without it.
- `prlink` ships in this plugin's `bin/`, so it is on PATH inside Claude Code.
  For a regular shell, `make install` at the plugin repo's root links it into
  `~/.claude/bin`. Ruby 2.6+, stdlib only.
- Optional, for resolving code references:
  - `trekr` for Ruby repos: `brew install dpep/tools/trekr` (or `cargo install trekr`)
  - `rq` for Rust, Go, Python, TypeScript and JavaScript: `brew install dpep/tools/rq`
    (or `cargo install reference-query`)

  Without them prlink still links file paths and verifies existing GitHub
  links, which need only git. Names it would resolve with a missing tool come
  back `unresolved` with cause `tool_missing`, and the report names the tool.

## Creating

- **Always open as a draft:** `gh pr create --draft`, most of all on repos you
  don't own. Mark it ready once CI is green; a PR isn't ready for review
  before that.
- Keep the change small and focused: easier to review, faster feedback.
- After creating, share the PR URL as a link.

**Write the body to a file, never inline.** Inline bodies hit shell quoting:

```bash
gh pr create --draft --body-file /tmp/pr-<branch>.md
gh pr edit <num> --body-file /tmp/pr-<num>.md
```

**Check where a new branch's first push goes.** With `push.default=upstream`,
a branch cut from `main` inherits `origin/main` as its upstream, so `git push
-u origin feature` pushes to `main` and skips the PR's CI. Check `git config
push.default`, or push with an explicit refspec: `git push -u origin
HEAD:refs/heads/feature`.

## Title

- Under 50 characters; GitHub truncates around 70.
- Prefix with the ticket key when there is one: `[PROJ-123] Fix validation`.
- Say what changed, not how. Bad: "Replace sleep 0.1 with proper Capybara wait
  in onboarding spec". Good: "Fix flaky onboarding spec".

## Description

- Use `.github/pull_request_template.md` when the repo has one.
- No AI-attribution footer ("Generated with …").
- Link resources that help a reviewer, or a reader of the history later.

**Link every reference, in the same pass as the prose.** A bare name reads
fine, but the reviewer can't jump to it, and links promised for later don't
get added.

| Reference | Link as |
|-----------|---------|
| Method / class / file | `https://github.com/<org>/<repo>/blob/<short_sha>/<path>#L<line>` |
| Test shared context or example | its definition file |
| Ticket | your tracker's issue URL |
| PR | `https://github.com/<org>/<repo>/pull/<number>` |
| GitHub user | `@<handle>` |

Pin code permalinks to a **short commit SHA, never a branch**: a branch moves
and its line numbers rot.

Link **dependency code** too: a framework method, a gem's class. Pin to the
version actually resolved (the lockfile's git revision or release tag), and
take line numbers from the installed source (`find-gem` for Ruby gems), not
from memory.

### prlink

`prlink` does the linking. It finds every code reference in the body
(backticked paths, constants, `Owner#method`, `Mod::fn`, bare names,
`path:line` tokens, existing GitHub links), resolves each with trekr (Ruby) or
rq (other languages), and pins it at the base sha when the line is unchanged
there, else at the head (fork) sha. Run it from the clone, or pass `-C DIR`;
`prlink --help` lists every flag.

1. **Report.** `prlink --body /tmp/pr-<branch>.md --json` for a draft
   (`--base REV` when the base isn't `origin/HEAD`; any rev works, e.g.
   `HEAD~1`), or `prlink --pr <num> --json` for an open PR. This is read-only:
   one row per ref with its `status`, `reason`, `candidates` (url, side, path,
   line, tool, and the line's text as `evidence`) and the `chosen` link.
2. **Review** the rows that aren't `linked` or `existing_ok`:
   - `ambiguous` lists candidates: pick one, or write the ref more precisely.
     A plain word (`price`) is never linked on its own; `price()` or
     `Item#price` is.
   - `unresolved` says `external` (a dependency, core, or stdlib: link the
     installed source by hand, as above), `not_found`, or `tool_missing`.
   - `existing_wrong` carries a fix when prlink is sure; `existing_stale` is a
     sha the clone lacks; `existing_unpinned` is a branch link.
   - `left_alone` says why (a path in plain text: backtick it).
3. **Apply.** Rerun without `--json` for the linked body on stdout (report on
   stderr), or with `--write` to save it (`gh pr edit` for `--pr`). Then add
   the links you chose by hand. `head_pushed: false` means the head's links
   would 404: push first (`--write` refuses without `--allow-unpushed`).

Exit 0: nothing to review. 1: refs to review, or an unpushed head. 2: a usage
or operational error. A rerun on its own output changes nothing.

Before posting, scan the body for backticked `Const` or `#method` names that
still aren't links.

## Quality checklist

- Small and focused beats complete.
- Favor internal consistency unless the deviation is intentional.
- Standard patterns over custom implementations.
- Limit hardcoded values: constants, config, or a dynamic lookup.
- Question necessity: remove dead code and unnecessary abstractions.
- Handle similar cases the same way.
- Change behavior *or* structure, not both at once.
- When deleting code, hunt the orphans it leaves: unused translation keys,
  unreachable methods, constants nobody references.
- Test quality over quantity: don't test one thing twice; spend the effort on
  edge cases and error handling.
- Avoid duplication; reuse or extract a helper.
- Test descriptions: simple and resilient ("raises an error", not "raises
  SpecificError").

## Contributing to code you don't own

External repos, plugins, shared tooling, another team's code:

- **Read the rules first.** Before preparing the PR, read CONTRIBUTING, the AI
  policy (the repo's and the org's `.github` repo), and any CLA. Some projects
  forbid agent-submitted PRs or AI-written descriptions; some refuse
  refactor-only PRs.
- **Minimize the diff** to what the change strictly needs. Don't refactor or
  tidy the surroundings in the same PR; save those observations for a
  follow-up PR or a comment.

## Updating a PR with upstream

For a **non-draft PR in review or queued to merge**, use GitHub's server-side
update. It creates a merge commit and keeps existing approvals, which a local
rebase and force-push would dismiss:

```bash
gh pr update-branch <num>
```

It only handles a clean merge, and it moves the remote ahead of your local
copy. So:

- **Clean:** after the server-side update, sync locally: `git pull --rebase`.
- **Conflicts:** there is no server-side button. Rebase locally, resolve, and
  push; the push is the update:

  ```bash
  git fetch origin main
  git rebase origin/main
  # resolve, git rebase --continue
  git push --force-with-lease
  ```

For a **draft PR** there are no approvals to protect: rebase locally and
`git push --force-with-lease`.

## Merging a stacked PR's base

`gh pr merge --delete-branch` on a PR that another PR is stacked on deletes
the child's base branch, and GitHub then *closes* the child. A closed PR can't
be retargeted, nor reopened once its head has been force-pushed. Retarget the
child first: `gh pr edit <child> --base main`.

To recover: push the old base branch back, reset the child's head to its
pre-close sha, reopen, retarget, then push the new head.

## Responding to feedback

When you act on an inline review comment, reply to that comment saying what
changed.
