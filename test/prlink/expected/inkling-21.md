## Why

#18 made the resident server stand down when a new build is installed over it ([`Exit::Replaced`](https://github.com/dpep/inkling/blob/9f40601/src/serve.rs#L180)). That worked live, but then nothing polled until some hook happened to spawn a server, and the session most exposed is an idle one waiting on CI: it fires no hooks, which is exactly the case push exists for. [`Exit::MaxLifetime`](https://github.com/dpep/inkling/blob/9f40601/src/serve.rs#L177) (24h) had the same hole.

## What

- **Hand off instead of going quiet.** When a server exits [`Replaced`](https://github.com/dpep/inkling/blob/9f40601/src/serve.rs#L180) or [`MaxLifetime`](https://github.com/dpep/inkling/blob/9f40601/src/serve.rs#L177) and its last tick was still `Busy` (the same `should_serve` that keeps it alive), [`serve::run`](https://github.com/dpep/inkling/blob/9f40601/src/serve.rs#L632) spawns the executable at its launch path with its own arguments ([`hand_off`](https://github.com/dpep/inkling/blob/4046648/src/serve.rs#L686)), waits for the successor's record, logs the handoff, and exits. The decision is [`Report::hands_off`](https://github.com/dpep/inkling/blob/4046648/src/serve.rs#L211). With nothing to serve, it exits as before.
- **`make install` rings the server** through a hidden `inkling serve --ring` (Makefile), so the running server notices within a second instead of at its next wake, which can be an hour away. It runs before the config load, so a broken config can't fail the install, and it's a silent no-op with no server.
- Design notes are in watching.md, "A new build takes over".

### Spawn, not exec

The brief preferred `exec` (keep the pid). I went with spawn-then-exit:

- **Exec doesn't keep the lock either.** `serve.lock`'s descriptor is close-on-exec, so exec opens the same gap between releasing the lock and taking it again. The only thing it keeps is the pid, and nothing reads that: `serve.json` is rewritten by the new image either way.
- **Exec loses the log lines that explain it.** Logging goes through `tracing_appender::non_blocking`; exec kills the writer thread and whatever it hasn't flushed yet, which is the "standing down" and "serve finished" lines. You can only flush by dropping the guard, and after that an exec that *failed* would have nowhere to log.
- **Spawn reuses what exists.** [`spawn_detached`](https://github.com/dpep/inkling/blob/4046648/src/serve.rs#L330) plus `claimed` is the path [`ensure_running`](https://github.com/dpep/inkling/blob/9f40601/src/serve.rs#L264) already uses, and it lets the old server report whether the successor actually took the lock.

**The lock race.** Between release and the successor's claim, a hook's [`ensure_serving`](https://github.com/dpep/inkling/blob/9f40601/src/main.rs#L1363) may spawn a server too. Both try the flock and the loser logs "another inkling is already serving; standing down" and exits 0. That's the arbitration [`ensure_running`](https://github.com/dpep/inkling/blob/9f40601/src/serve.rs#L264) already depends on, not a new case. Rings in the gap aren't lost: the old server still holds the FIFO open, and a named pipe's buffer is shared by everyone who opens it.

**No crash loop.** A handoff is tried once and the old server exits whatever happens. A successor that can't be spawned costs one warning. One that starts and dies costs a warning that it never claimed the lock. Neither is retried. A successor can't immediately hand off again, because its build and start time are its own.

**The takeoff window.** I found this in the scratch run: the first exec of a just-copied binary takes ~1s here (macOS assessing it; measured 1217/1249/1258ms first run vs ~230ms second), against `claimed`'s 250ms window. So every handoff after an install logged a false warning. Handoffs now wait up to 5s ([`HANDOFF_LOOKS`](https://github.com/dpep/inkling/blob/4046648/src/serve.rs#L309)). Nobody is waiting on a server that's leaving. The hooks' 250ms is unchanged.

## Proof

Tests (watched failing first): a busy server that is replaced hands off, an idle one doesn't, a busy one at its lifetime cap does, idle or signalled exits never do, a handoff to a build that can't run returns an error once, and `serve --ring` succeeds with no server and an unparseable config, and doesn't create the FIFO.

End to end, all on scratch paths: a debug build copied to a scratch bin, a fake session socket, a live watch on a probe PR (dpep/inkling-ci-probe#6, opened through `gh api`), and the `make install` sequence (copy, rename over, `serve --ring`) run by hand:

| Case | What the scratch log says |
| --- | --- |
| New build, live watch | `standing down` at the ring → `serving pid=73832` → `handed off to a new server from=73320 to=73832 reason="replaced"`, ~1.1s after the install. `lsof` shows 73832 running the new inode. It polled on schedule 48s later (`polled watches why="serve"`), and `inkling status` showed `serving yes (pid 73832)` |
| New build, nothing watched | `serve finished reason="replaced"`, no successor, `serving no`, no process |
| Installed file can't be spawned (mode 644) | `could not hand off; the next hook will start a server error=… Permission denied`, exits, no process |
| Installed file starts and dies (+x text, which BSD `execvp` hands to `/bin/sh`) | `handed off, but the new server has not claimed the lock from=82041 to=82884` after 5s, exits, nothing respawns |
| Lifetime cap (20s in scratch config), live watch | `serve finished reason="max_lifetime" uptime_seconds=30` → `serving pid=89937` → `handed off to a new server from=87504 to=89937 reason="max_lifetime"`, 61ms later with a warm binary (30s rather than 20s because the cap is checked after a tick, and naps are at least the 15s interval) |

`make install` itself was not run, since it goes live behind every session's hooks. The Makefile recipe was checked with `make -n` against a scratch `BIN_DIR`.

**Upgrade note:** the install that brings this in is still handled by the old server, which stands down without a successor. It's in the CHANGELOG.

## Dogfood

- Rung 1: `inkling wait --until checks dpep/inkling#21` started as a background task at 03:54:15Z, right after `gh pr create`. It exited 0 at 03:55:08Z with `success … met after 52s (4 polls)`. I could act on that without looking anything else up.
- Push (rung 3+) couldn't be measured from here: this lane shares its parent's session, so anything pushed for #21 went to the parent, not to this agent.
- The handoff can't be dogfooded on its own PR until it's installed. The first `make install` after merging is handled by the old server (see the upgrade note), so the install after that is the first live test. Look for `handed off to a new server` in `inkling logs -g serve`.
- [`docs/dogfood.md`](https://github.com/dpep/inkling/blob/4046648/docs/dogfood.md) has two new lines: the stand-down-then-silence finding, and the false "not claimed" warning caused by macOS's first-exec cost.

🤖 Generated with Claude Code
