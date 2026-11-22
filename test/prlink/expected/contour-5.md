`a_server_restarts_into_a_contour_installed_underneath_it` failed once on a
Dependabot PR (run 35385316982,
toml 1.1.4→1.1.6) and passed on a re-run. Branch protection requires that check,
so a random failure blocks auto-merge.

## The mechanism

The panic was `left: Number(7), right: 6` at
[`Session::request`](https://github.com/dpep/contour/blob/b1bff58/tests/mcp_e2e.rs#L105) —
the reply to the `ping` sent as id 6 came back as id 7. Only one thing in the
test emits an unsolicited id 7: the shell stand-in installed for the second
upgrade, which hard-coded `"id":7` in every reply. So the stand-in answered
request 6.

It could, because the reader
restats the binary after **every** line,
not only after the line that follows an install. The install happens while the
reader sits between two lines, so whether it is seen by the stat after the
earlier request or the stat after the later one depends purely on whether the
reader's `stat` beats the installer's `rename`. Both orderings are legal. When
the earlier stat wins, the `exec` happens *before* the next request is read, and
the replacement program answers it.

The test assumed the other ordering twice: it read exactly one line per request
and required it to be that request's reply, and its stand-in assumed it would
only ever be asked id 7.

**This is not a server bug.** A JSON-RPC server may volunteer a notification at
any moment — `serve` says so itself,
and real clients match on `id` — and an installer never coordinates with the
request stream. Restarting before reading the next line is the *safe* moment;
deferring it would be worse.

## The fix

- [`Session::reply`](https://github.com/dpep/contour/blob/b897140/tests/mcp_e2e.rs#L122)
  reads until the id matches, collecting anything unsolicited into
  `volunteered` — the harness becomes a correct client, and every test gets the
  property.
- The `tools/list_changed` assertion now asks `was_volunteered(…)` after the
  ping it used to read past, so it no longer cares which side of that reply the
  notification landed on — and still fails on its own line rather than blocking.
- The stand-in echoes the id it was asked under, since which request reaches it
  isn't the test's to decide.

## Proving it

macOS couldn't reproduce it by brute force (0 failures in 600 runs at up to 128
concurrent, debug and release) — `std::fs::copy` clones on APFS, so the install
is near-instant and the window is much narrower than on a 2-core Linux runner.

So the window was opened deliberately instead: a temporary `sleep` in the reader
between dispatching a line and the restat, which is exactly "the reader lost its
slice here". **Not part of this PR** — it was reverted before committing.

| | stall after the line before install #2 | stall after every line | no stall, 128 concurrent |
|---|---|---|---|
| before | 20/20 fail (`left: 7, right: 6`, the CI signature) | 20/20 fail (`left: Null, right: 3`) | 0/600 |
| after | 0/20 fail | 0/20 fail | 0/500 |

No CHANGELOG entry: nothing a user does or sees changes.

## Decided against

- **A retry, a longer sleep, or `#[ignore]`** — the ordering is knowable, so
  making the test tolerate both orderings costs less than any of them.
- **Shipping the stall knob** as a permanent test hook — a `sleep` in `serve()`
  that exists only for one test is production complexity for a race the fix
  already closes by construction.

🤖 Generated with Claude Code
