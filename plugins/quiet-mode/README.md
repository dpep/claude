# quiet-mode

A Claude Code mod that quiets the terminal UI:

| Where | Before | After |
|---|---|---|
| spinner | `✻ Schlepping… (12s, 300 tokens)` | nothing |
| end of each turn | `Baked for 5s` | nothing |
| footer hint | `esc to interrupt · (shift+tab to cycle) · /tasks to see subagents · ← 1 agent` | a ticking clock while working (`1m 4s`), then the last turn's duration (`1m`) |

The spinner's glyph and stats are one engine element, so the whole line goes
and the footer takes over the clock; the token count isn't shown. A spinner
showing a real status (compacting, say) stays. Finished durations truncate below an
hour (`5s`, `4m`) and round to the half hour above it (`1h`, `1.5h`). Only
the main conversation's turns count; a subagent finishing doesn't change the
number.

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
