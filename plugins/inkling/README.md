# inkling

Tells a Claude Code session when a pull request, Jira issue, Slack message or
Slack channel it cares about changes, so the session never polls. CI goes red,
a review lands, someone replies: one line arrives in the session, mid-turn.

```
inkling: github.com/owner/repo#12 | checks pending → failure: lint (job 1234567890)
```

This plugin is four hooks and a skill that wire sessions to the
[`inkling`](https://github.com/dpep/inkling) CLI. macOS and Linux.

## Install

```sh
cargo install inkling-dev
```

```
/plugin marketplace add dpep/claude
/plugin install inkling@dpep
```

`inkling` must be on `PATH`. Without it the hooks do nothing, and SessionStart
says so once.

Each source uses a tool you probably already have, with no config file:

- GitHub: `gh`, logged in.
- Jira: `acli`, logged in.
- Slack: Claude Code's Slack connector. Each Slack read is a `claude -p` call,
  about 2¢, so a Slack watch is read at most every 5 minutes.

## What the hooks do

| Hook | Runs | Effect |
| --- | --- | --- |
| SessionStart | `inkling hook session-start` | Attaches the session's inbox, so news can be pushed to it. |
| SessionEnd | `inkling hook session-end` | Detaches it. Watches and the cursor live on, so `claude --resume` gets the net change. |
| UserPromptSubmit | `inkling hook prompt` | Adds anything that moved and wasn't already pushed. Prints nothing when nothing did. |
| PostToolUse (async) | `inkling hook post-tool` | Watches PRs touched by `gh pr …` or `git push`, issues passed to `acli jira workitem …`, and Slack messages the session posts. |

A resident `inkling` process starts itself when a subscribed session has
something to watch, pushes each change to the sessions watching it, and exits
when nothing is left. There's nothing to start or stop. A hook never fails a
session: on any error it logs and exits 0. `inkling status` and `inkling logs`
are where to look.

The skill teaches the session `watch`, `events` and `wait`, and how to read a
pushed line.

## If you installed the hooks by hand first

Remove the `inkling hook …` entries from `~/.claude/settings.json` once this
plugin is enabled, or every hook fires twice.
