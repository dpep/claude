# quiet-mode

A Claude Code mod that quiets the terminal UI:

| Where | Before | After |
|---|---|---|
| spinner | `✻ Schlepping… (12s, 300 tokens)` | `✻ (12s, 300 tokens)` |
| end of each turn | `Baked for 5s` | nothing |
| footer hint | `(shift+tab to cycle) · /tasks to see subagents · ← 1 agent` | the last turn's duration, e.g. `5s` |

A spinner showing a real status (compacting, say) keeps its words. Durations
truncate below an hour (`5s`, `4min`) and round to the half hour above it
(`1h`, `1.5h`). Only the main conversation's turns count; a subagent finishing
doesn't change the number.

Requires a Claude Code build that loads mods (hooks modules with `ui.render`).

## Install

```
/plugin marketplace add dpep/claude
/plugin install quiet-mode@dpep
```

## What it can't reach

Some chrome is drawn outside anything a mod can redraw:

- the `⏵⏵ auto mode on ·` indicator and its separator
- `⎿ Allowed by auto mode classifier` under tool calls
- the list of running agents below the prompt

## Developing

```
claude --plugin-dir plugins/quiet-mode    # live reload while editing
claude plugin test plugins/quiet-mode
```
