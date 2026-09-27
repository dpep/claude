# Resident processes: daemons, language servers, background workers

The SKILL covers a single invocation. This covers what outlives one: a process
that stays up, work that finishes after the answer, and state that several
processes read at once. Worked examples are from rq, trekr, ae and contour
(github.com/dpep); the lessons are general.

## Climb the process ladder one measured rung at a time

Each rung buys latency with lifecycle you now own. Start at the bottom and move
up only when a number says the rung below can't meet the budget.

| rung | example | what it costs you |
|---|---|---|
| one process per call, state on disk | rq, `trekr --def` | startup per call — often a few ms |
| small work amortized *after* the output | rq's usage count, staleness check | a slower exit nobody waits on… unless they do (see measuring) |
| a detached, single-flighted background child | rq's `--warm`, trekr's LSP-spawned `--index` | a lock, niceness, a skip rule |
| an editor-owned server | `trekr --lsp` | protocol, cancellation, hot reload, memory held for a session |
| a true daemon (socket, idle timeout) | ae, contour's MCP server | start/stop, stale binaries, idle memory, multi-client state, a second failure mode for every command |

Before climbing, ask whether a **persisted, mmap-able snapshot** gets the win
instead: if the per-call cost is rebuilding the same structure every time,
building it once and mapping it back is most of a daemon's speed with none of
its lifecycle — and mapped pages are shared by every process that reads them.

A resident front stays a *thin layer over on-disk state*, never its owner. The
moment the server holds the only copy of something, a crash or an upgrade
loses it and every other front (the CLI, a second editor) disagrees with it.

## Upgrades must reach processes that are already running

A server that outlives `brew upgrade` answers with the old code indefinitely.

- **Watch the path you were launched as (`argv[0]`), not `current_exe()`.** A
  Homebrew upgrade re-points `bin/tool` at a new Cellar directory and leaves the
  old file alone; on Linux `current_exe()` reads `/proc/self/exe`, which already
  resolved the symlink, so it never sees the change. Follow the symlink when
  *stamping*, so a rebuild in place and a relink are the same event.
- **Stamp by device, inode, size, mtime — and mode.** Mode, so `chmod +x` on a
  file you refused as non-executable counts as a change. A stat through the
  symlink costs microseconds; poll at idle moments.
- **A daemon can simply step down** on a changed stamp (ae): the next client
  call starts the new build. Cheap, and correct when clients reconnect.
- **A stdio server can't step down without dropping its client**, so swap in
  place:
  1. Wait for a safe moment: inbox drained, nothing in flight, no background
     job whose progress the new process couldn't finish.
  2. **Probe the new binary first** (run it in a probe mode that reports which
     handoff format it reads). An `exec` into a binary that then dies takes the
     client's connection with it and there is no way back. Three outcomes:
     reads our format → exec; runs but can't resume → exit so the client
     restarts it; doesn't run → keep serving, log it, don't retry until the
     stamp moves again. Treat "text file busy" as "install in progress", retry.
  3. Hand off in a 0600 file, created fresh and deleted once read: the
     `initialize` params verbatim, registrations the client holds, open buffers
     with version and full text, and any partly read message bytes.
  4. `exec` the new binary with the same argv plus a resume marker. stdio file
     descriptors survive `exec`; the connection continues.
- **Own the stdin reader.** A library reader that runs on its own thread and
  buffers ahead has already taken bytes off the pipe that die with the process
  at `exec`. Read on the loop's own thread and carry unread bytes in the handoff.
- **Log every reload** (from/to version, ms, what was carried) — "is it running
  the new build?" must be answerable from the log.

## Shared on-disk state

- **Content-address anything derived from a file.** Key facts by a hash of the
  bytes (a git blob OID works), never by path, and keep a small per-checkout
  `path → content` map. N worktrees of one repo then cost one index, each sees
  its own local edits, and a branch switch parses only new content. Keying by
  `(repo, path)` looks simpler and makes the last checkout to index a path win
  for everyone.
- **Never modify a file another process has mapped.** Write immutable,
  content-keyed files: temp → fsync → atomic rename. Readers keep a valid view
  of the old file even after it is replaced or unlinked, and switch when their
  own key moves. Racing builders produce identical bytes, so the rename is
  idempotent and needs no lock.
- **Validate on open** with a header carrying format version, key and checksum;
  a truncated, stale or old-format file is rebuilt, never trusted. This is also
  what makes an upgrade that changes the format safe.
- **Collect by reachability, not by age alone.** "Never garbage collect" gets
  decided early and outlives its reason; "delete the old versions" is also
  wrong when different projects pin different versions. Keep what something
  live can still reach (a checkout on disk, a lockfile that names it), add a
  short `last_seen` window for things that flip back (a branch switch), and
  make collection explicit with a dry run that *is* the real run rolled back.
- **An extraction change needs a re-extraction path.** New facts from existing
  files appear only when those files are re-parsed; either migrate the store to
  mark them stale and re-parse in the background, or (pre-1.0) bump the store
  version and rebuild — deliberately, and said plainly in the changelog.

## Background work

- **Detached children run niced with throttled I/O**, stdio pointed at a log
  file (see the SKILL on detached processes having a voice), single-flighted
  per repo by a pid-stamped lock.
- **Don't spawn when there's provably nothing to do.** Record the child's
  "nothing moved" verdict with a stamp of the state it checked (HEAD, the index
  file's mtime) and skip the spawn while the stamp matches — within a short
  window, because some changes (an unstaged edit) touch nothing you can stamp.
  Say which changes are caught immediately and which only after the window.
- **Read the stamp *after* the check that can rewrite it** (`git status`
  refreshes `.git/index`), or the next call always looks changed.
- **Cancellation must be read ahead of the queue.** A withdrawn request that
  waits behind itself in the inbox is never cancelled; long scans should check
  for it partway through.
- **A cache of file contents must revalidate** (mtime + length) or an agent
  reads its own file as it was before its own edit.

## Measuring a process that stays up

- **Measure to where the caller stops waiting.** An agent's shell waits for
  exit, not for the answer. Work after the output — a join, a spawn, a
  write — is invisible to an in-process timer and was the largest remaining
  cost more than once.
- **When nothing makes an operation cheaper, move it.** A per-search usage
  write that no tuning could speed up stopped delaying the answer the moment it
  ran after the output.
- **Memory: measure private/dirty, not RSS.** Mapped files inflate RSS with
  shared, evictable pages; macOS's phys_footprint is the number that costs
  something. Freed memory the allocator keeps looks like a leak — purge after
  big requests or use an allocator that returns it. Per-request structures
  (parse trees for a references scan) are dropped when the request ends.
- **Measure N concurrent servers on the same repo** before deciding whether to
  share one: with a shared mapped snapshot, the duplicate cost may be small.
- **Deterministic before diffable.** Sort before you cut a list: a result
  truncated in hash-map order differs between two runs of the same binary, and
  every before/after comparison then reports noise. A comparison harness must
  diff to zero against itself first; pin clock-dependent signals and parallel
  ordering until it does.
- **On a loaded machine, interleave A and B** run by run and compare only
  within a run.

## Editor integration (VS Code)

- VS Code **merges every provider's answers** for definitions, references and
  workspace symbols. Two providers for one language mean every answer twice and
  a click that opens a list instead of jumping.
- A secondary provider can **defer**: call `vscode.executeDefinitionProvider`
  (or the workspace-symbol equivalent) itself and answer only when the others
  found nothing, or add only what they didn't return. That call re-enters your
  own provider — guard it with a set keyed by position (or query) and return
  nothing from the inner call.
- Answer in the **client's path spelling**: a workspace opened through a
  symlink (or macOS `/var`) must get locations under that spelling, or the
  editor opens the same file twice.
- LSP positions are **UTF-16 code units**; a column computed in bytes or chars
  is wrong after the first multibyte character.
- `editor.multiCursorModifier: ctrlCmd` moves Go to Definition to Alt/Option-
  click — check the user's settings before debugging "Cmd-click doesn't work".

## Test hygiene for tools that shell out to git

- **Clear `GIT_DIR`, `GIT_WORK_TREE` and `GIT_INDEX_FILE`** on every git command
  a test fixture runs, and on every copy of the tool the tests spawn. Run under
  `git rebase --exec`, the gate inherits `GIT_DIR`, and a fixture's `git init`
  then writes into the real repository — it set `core.bare=true` on one.
- **Never run the test gate under `rebase --exec`** anyway: rebase, then run it.
