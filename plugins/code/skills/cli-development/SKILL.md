---
name: cli-development
description: Use when building a new CLI or library, or hardening an existing one toward production — deciding output formats, flags, exit codes, stdin and streaming, data paths, cache lifecycle, TTY behavior, logging, observability, or shell completion. Also for "is this tool agent-friendly", "what's missing before I ship this", or reviewing a tool someone else wrote. Encodes the house conventions shared by rq, ae, and vocab.
---

# CLI development: getting a tool production-ready

## Overview

A tool that works is not yet a tool that ships. The gap is mostly a checklist,
its items cheap at the start and awkward to retrofit — `--json` on one command
rather than all of them, logging that went to stdout and now can't move without
breaking a consumer. Shared across `rq`, `ae`, and `vocab`: the consistency is
the point, so the second tool costs nothing to learn.

## The flag spine

Every tool carries these, with the same letters:

| Flag | Meaning |
|---|---|
| `-j, --json` | pretty JSON — an array of results, or an object for a command |
| `-J, --ndjson` | one compact object per line, for streaming |
| `-v, --verbose` | telemetry to **stderr** |
| `-q, --quiet` | suppress stdout; the work still happens |
| `-f, --file` | read input from a file, where input is read at all |
| `--db` / `--config` | override the data path; also honors an env var |
| `-h, --help` / `-V, --version` | table stakes |
| `--completions [SHELL]` | print a completion script, defaulting to `$SHELL` |
| `--profile` | phase timings and counters to stderr |
| `--dry-run` | for anything that writes or deletes |

## Structured output is not a feature of one command

**Every command honors the format flags, not just the main one** — status
messages, reports, and errors included. A consumer that has to parse `list`
with JSON and `sync` by scraping text will scrape everything. Resolve the
format once, render through one module, and keep field names stable: consumers
parse them, so a rename is a breaking change and belongs in the changelog as
one. New commands and fields get structured output in the same change.

## stdout is data, stderr is everything else

Logging, progress, warnings, and telemetry go to stderr through a logger, never
`println!`. The test is whether `tool … | jq` works while `-v` is on.
`--profile` is held to the same rule, so a profiled run still pipes cleanly.

## Exit codes should mean something

Pick the convention that matches the command's *job*, and document it:

- **Checking / linting** — clean exits `0`, findings exit `1`, so it drops into
  a pre-commit hook or CI with no wrapper.
- **Querying** — results exit `0`, empty exits `1`, so `tool list foo && …`
  reads as "if any" — grep's convention, and users expect it.
- **Reserve a code for "no answer yet, ask again."** A tool with a cache, an
  index, or a background build has three states, not two, and the third is
  neither a verdict nor a failure. `rq` exits `2` for `warming` so a caller can
  retry rather than treat an incomplete index as "no match" — which is the
  wrong answer, confidently given.
- **An operational error gets its own code**, never shared with a verdict.
  Where there is no retryable state, `2` is the natural home for it; where
  there is, move it up. One meaning per code.

Both verdict conventions in one binary is fine and often correct — just say
that `1` reads two ways, or someone will file it as a bug.

**Put the table in `--help`, not only the README.** Whoever is debugging an
exit code at 2am is at a terminal.

## Reading input: stdin is not a fallback

One item per line, and the positional argument optional when input is piped.
`tool foo` and `echo foo | tool` should reach the same code path — one loop
over an iterator of lines, not the same logic written once per source, which is
how the three drift apart.

**Then check that a line format actually streams** — but only where input is
a stream. For a request/response tool `-J` is just a compact format and there
is nothing to get wrong. A format that collects
everything and prints at EOF is line-*shaped*, not a pipe, and the bytes are
identical either way — only the timing differs, which is why it survives
review. `tail -f log | tool -J` prints nothing.

The cause is almost always **control flow, not buffering** — results collected
into a vector and rendered after the loop, or input drained to EOF before any
work starts. Both bugs were in our tools and both looked like buffering. Fix
the shape: read lazily, emit per item.

Buffering is the second-order problem, and language-dependent. Rust's `stdout`
is a `LineWriter`, so `println!` reaches a pipe on every newline; C stdio and
Python block-buffer into an 8K buffer when stdout is not a tty. The Rust trap
is the *optimization* — wrapping stdout in a `BufWriter` for speed silently
converts line buffering into block buffering. Check, rather than assume, which
you have; an explicit flush is cheap insurance either way.

`-j` deliberately cannot stream — a pretty array is a single document — which
is exactly why both formats exist; say so where they are documented, or the
asymmetry reads as an oversight.

**The test asserts timing, not bytes**: write one item, leave stdin open, and
require output before EOF. Nothing else catches this, because the output is
byte-identical either way — which is how it survives review, and how it sat in
two of our tools for months.

## Where its data lives

Three ways in, in this order of precedence — flag, environment variable, XDG
default:

```rust
#[arg(long, env = "AE_DB", global = true)]
db: Option<PathBuf>,     // else $XDG_DATA_HOME/ae/…, else ~/.local/share/ae/…
```

The flag is for a one-off, the env var is what makes the tool testable (every
e2e test points it at a temp dir) and scriptable, and the XDG default keeps it
from scattering dotfiles. Add the env var late and every existing test has
already grown its own `--db`. Print the resolved path in `--status` with `~`
unexpanded — the reader wants to recognize it, not paste it.

## Long-lived state needs lifecycle commands

Cache and index state goes wrong, and the user needs to act on it without first
hunting for a directory to delete:

| Command | For |
|---|---|
| `--index` / `--refresh` | rebuild from source |
| `--warm` | build ahead of the first real call |
| `--clear-cache` | drop derived data, keep the real data |
| `--drop` | remove everything, for starting over |
| `--status` | what exists, how big, how stale |

`--status` doubles as the **health check**: read-only, non-zero when something
is wrong, so it drops into a container probe or a `&&` chain. Have it report
what the tool integrates with — which editors have the exported dictionary,
whether a config was found — or "did it install?" is unanswerable without going
to look.

## Non-blocking is a mode, not a timeout

A tool that waits on a warming index gets killed rather than waited for; an
agent cannot sit at a spinner. Give it `--no-wait`: return what is known now,
with the state named in the output, and let the caller decide whether to ask
again. This is the other half of the retryable exit code — ship either alone
and the caller is still guessing.

## Behave differently for a terminal than for a pipe

`is_terminal` on stdout is the one branch worth making: colour, progress bars,
and interactive prompts when a human is watching, none of it when the output is
parsed. On **stdin** it tells `tool` with no arguments to print help rather
than block forever on input that is never coming — hanging on a bare invocation
reads as broken.

Never let it change the *data*, only the presentation. A pipeline that gives
different results when run by hand is unrepeatable — and by hand is the first
way anyone runs it when debugging.

## `--profile` earns its place, but only if it covers the work

The trap: timing only *setup* leaves the hot path hiding between the named
phases and the total. If they do not roughly sum, the gap is where the time
goes, and it is usually the answer. Counters matter as much as timings:
"307,485 candidates scanned for 3 suggestions" says more than any duration.

## Help should answer the question that was asked

Built-in `help` in most frameworks walks subcommands only, so `tool help
--some-flag` answers "unrecognized subcommand" — true and useless. Resolve the
topic against options too, and when it matches nothing, **list what was
available** rather than just failing.

Watch for the inverse trap: if the tool takes free text as a positional
argument, `tool status` may *silently succeed* by treating `status` as input. A
command that does not exist should never report success.

Help drifts, and only a test notices. Assert that every subcommand appears in
`--help` — one that exists but isn't listed is one nobody finds. Then pull the
example lines out of your after-help block and run them back through the
parser: the examples you wrote three releases ago should still be valid syntax.
It is the in-binary version of running every README example, and it costs one
test.

## Completion is a flag, it runs before your data exists, and it drifts

`--completions [SHELL]`, not a `completion` subcommand: it is meta-output about
the tool, like `--version`, rather than a verb the tool performs — and a
subcommand squats in the noun-space the tool's own vocabulary needs (`ae
completion` reads like it completes acronyms). Default the shell to `$SHELL` so
a human can run it bare; still take it as an argument, because packaging asks
for each shell by name.

Two rules you only learn by breaking them:

**Nothing but the script may reach stdout.** People `eval "$(tool --completions
zsh)"` from a shell rc, so a stray log line, warning, or progress bar gets
executed at their login. Emit the script, exit 0, put everything else on stderr.

**It must work with no data, no config, and no network.** Homebrew runs it in a
clean sandbox at install time, so if generating completions opens the database,
loads a model, or migrates a schema, the *install* fails — on a machine where
none of that exists yet. Generate from the parser and nothing else.

Wire the packaging in the same change:

```ruby
generate_completions_from_executable(bin/"tool", "--completions", shells: [:bash, :zsh, :fish])
```

Brew literally runs `tool --completions bash`, which is a cross-repo coupling
with nothing holding it together: rename the flag, or stop defaulting an
argument, and the *formula* breaks — in another repository, at install time,
for someone else. Pin it:

- each shell emits a non-empty script carrying that shell's marker (`complete
  -F`, `compdef`, `complete -c`)
- the bare form honors `$SHELL`, and an unset or unrecognized one fails
  usefully instead of panicking
- the invocation the formula uses is a test case, verbatim
- `brew test` asserts it post-install — a broken completion script fails
  silently, as "tab does nothing", which nobody reports as a bug

**Static completion is free; value completion is the point and isn't.**
`clap_complete` derives flags and subcommands from the parser for nothing, and
that generated floor never drifts. What people actually want is the *argument*
completed — `ae define <TAB>` offering acronyms it already knows. That needs a
hidden subcommand printing candidates one per line plus a shell function that
calls it, and *that* drifts, because now a human wrote something. Worth it when
the argument is drawn from a set the tool knows and the user doesn't; skip it
for paths, where the shell is already better at this than you are.

**It is for humans.** Agents do not press tab, so completion never substitutes
for help that answers the question or output that parses. It earns its place on
the human side of a tool that has both.

The house is not currently consistent, which is the argument for writing it
down: `rq`, `gqls` and `vocab` take the flag (vocab's optional argument is the
shape to copy), `contextdb` and `iriq` took a subcommand, `rwr` depends on
`clap_complete` while exposing neither, and `ae`, `inception` and `navi` have
none. Converge when you next touch one.

## Before it ships

- **Run every example in the README** and diff against actual output — they
  drift silently and compound across releases.
- **Make the dry run readable** — it is what people trust before letting the
  real thing run.
- **Write the CLAUDE.md**: what the tool is for, its first principles, the
  layout, how to run the gate. Future work is only as good as that file.
- **A changelog entry in the same change that earns it**, while the reasoning
  is fresh. Say what a user must *do*.
