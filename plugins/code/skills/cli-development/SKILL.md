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

**`--ndjson` means one row per line, for every row set.** A command whose
answer is a list (references, candidates, symbols) writes each row as its own
line — the same object as its element in the `--json` array — and ends with one
summary line carrying the rest of the answer. One line holding a nested array
is JSON with the newlines removed, not NDJSON: `jq -c` per line and streaming
consumers both break on it.

## stdout is data, stderr is everything else

Logging, progress, warnings, and telemetry go to stderr through a logger, never
`println!`. The test is whether `tool … | jq` works while `-v` is on.
`--profile` is held to the same rule, so a profiled run still pipes cleanly.

**A process that detaches needs a file, or it has no voice at all.** Send a
daemon's stderr to `/dev/null` and it cannot be debugged at any verbosity, from
anywhere: the caller's `-v`, `RUST_LOG` and `RUST_BACKTRACE` are all still set,
still inherited, and still going nowhere. Put it beside whatever the process is
named by — the socket, the pidfile — and name that path in the error the caller
*does* see, because "it exited" is not a diagnosis.

Four things decide whether that file is worth having:

- **Append, don't truncate.** A crash loop is what the log is most needed for,
  and truncating per start erases it on the way in. One file, many lives — so
  each run must name its version and pid as it starts, or the file is a blur.
- **Cap it, and keep enforcing the cap.** Checking only at startup bounds a
  crash loop but not a long life: at debug level a watchdog thread can log twice
  a second, and nothing ever re-checks. Trim on a timer the process already has.
  Keep the *newest* half — during a loop the oldest lines are the least
  interesting — and resume at a line boundary so the first entry isn't a
  fragment. Trimming under a live writer is safe when it holds the file
  `O_APPEND`: the next line goes to the end of whatever is there now.
- **Default it, then make it configurable** the way every other knob is, and put
  a floor on it. A cap below one entry turns every write into a rewrite.
- **Log routine events routinely.** The first thing reading a new log will show
  you is what you got wrong about it: ours logged `WARN connection error` for
  every liveness probe — connect, see an answer, drop — which is once per call
  in the workload that produced 500 a day. A log that is mostly warnings about
  nothing is one you stop reading, which is worse than not having it.

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

**Refuse a relative path.** A relative `$TOOL_DB` resolves against each process's
working directory, so every directory silently gets its own empty store —
including the detached child a query spawns. Exit 64 naming the variable and
suggesting `$PWD/…`; treat an empty value as unset; reject a trailing `/`.

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

Where the tool has a fast path and a slow fallback, **`--status` must be able to
tell you which one is actually being used.** A client that quietly gives up on a
warm daemon and loads its own copy of a model is ten times the memory and
invisible from outside — no flag, no log line, nothing. A count of work served,
checked against how often the caller thinks it ran, is the whole diagnosis; the
mismatch is the signal, so don't inflate it with probes and status calls.

`--status` doubles as the **health check**: read-only, non-zero when something
is wrong, so it drops into a container probe or a `&&` chain. Have it report
what the tool integrates with — which editors have the exported dictionary,
whether a config was found — or "did it install?" is unanswerable without going
to look.

## State survives upgrades, downgrades and damage

A store that outlives the binary will meet a binary that doesn't match it: an
upgrade, a downgrade, two versions installed side by side (a package manager's
and a dev build), a crash mid-write, a full disk. The user must never be left
holding an error only a manual `rm` fixes.

- **Stamp two versions.** The schema version (SQLite's `user_version`) says what
  the file holds; the tool version that last wrote it — and created it — says who.
  The second is what turns "no such column" into "written by rq 0.61.0".
- **Forward: migrate, and fall back to rebuilding.** Migrate in one transaction,
  re-reading the version under the write lock so concurrent openers don't both
  migrate. On *any* failure — a migration error, "file is not a database",
  "malformed", a failed `quick_check` — move the file aside (with its `-wal` and
  `-shm`; keep the latest copy only) and rebuild. Say it once, on stderr:
  what failed, that it's rebuilding, where the old file went. A derived index
  can always be regenerated; a broken one can't be reasoned with.
- **Backward: never destroy a newer store.** An older binary that wipes what it
  doesn't understand starts a ping-pong with the newer one, each rebuilding
  over the other. Refuse clearly, or better, give the older binary its own
  side file (`tool.v22.db`) and warn once. `--status` shows side files; `--gc`
  removes them.
- **Say what a long upgrade is doing.** A one-time migration on a large store
  takes seconds and holds the write lock: other processes should wait for it
  (a longer busy timeout while the version is behind), and one of them should
  print "upgrading the index (one-time)…" — not exit with "database is locked".
- **Resident processes recover too.** A language server or background worker
  holding the old store must notice it changed underneath (re-check the version
  per write), refuse to write old-format data into a new store, and reopen or
  hand off — not answer errors until restarted. See
  [resident-processes.md](references/resident-processes.md).
- **Test every path.** A corrupt file, a truncated one, a migration that fails
  midway (inject it), a newer-schema store, two versions alternating on one
  path, and several openers of a broken store at once.
- **Guard the version against the code that fills the store.** A cache keyed on
  its input but not on the code that produced it goes stale silently when that
  code changes: an extractor fix ships, existing stores keep the old rows, and
  nothing says so. "Remember to bump the schema" is a rule nothing enforces —
  rq shipped several extraction fixes that reached existing indexes only by
  luck. Add a golden test from day one: hash the stored output over the
  fixtures, record the schema version beside it, and fail with "output changed:
  bump the version" when the hash moves and the version didn't. Hash only what
  the store keeps, in a stable order, per language or kind so the failure names
  which; regenerate with an env var.

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

## Processes that outlive one call

Daemons, language and MCP servers, detached background workers, and state
several processes share — choosing the process model, getting upgrades into
running servers (hot reload), content-addressed and mmap-shared state, garbage
collection, background-work hygiene, measuring what stays up, and VS Code
provider integration — are in
[references/resident-processes.md](references/resident-processes.md). Read it
before adding a daemon, a `--watch`/`--serve`/`--lsp` mode, or a cache that
another process reads.

## Before it ships

- **Sort before you cut.** A list truncated in hash-map order differs between
  two runs of the same binary, which makes every output diff noise.
- **Tests that shell out to git clear `GIT_DIR`, `GIT_WORK_TREE` and
  `GIT_INDEX_FILE`**, or a gate run under `git rebase --exec` writes fixtures
  into the real repository. So does the tool, if it picked its repository by
  walking up to `.git`: an inherited `GIT_DIR` must not send its own git calls
  somewhere else. Prove it by running the suite with `GIT_DIR` pointed at a
  path that doesn't exist.

- **Run every example in the README** and diff against actual output — they
  drift silently and compound across releases.
- **Make the dry run readable** — it is what people trust before letting the
  real thing run.
- **Write the CLAUDE.md**: what the tool is for, its first principles, the
  layout, how to run the gate. Future work is only as good as that file.
- **A changelog entry in the same change that earns it**, while the reasoning
  is fresh. Say what a user must *do*.
