# Profiling

## Reading a flamegraph

- **Width is time, height is call depth.** Left-to-right order is alphabetical
  in most renderers, not chronological, so don't read a flamegraph as a
  timeline.
- **Self time** (a frame's top edge) says where the work *is*. **Total time**
  says who *caused* it. When a function is cheap per call but called a million
  times, fix the caller.
- **Wide plateaus near the top** are hot leaves: parsing, hashing, allocation,
  regex. **Many thin towers** under one parent mean death by a thousand calls,
  usually an N+1 or per-item setup that should be hoisted.
- **Compare like with like.** Diff a slow profile against a fast one with the
  same build, workload, and machine class. A differential flamegraph shows
  what grew; two screenshots side by side don't.

## Profile types

| Type | Answers | Misleads when |
|---|---|---|
| CPU | Where cycles go | The process is mostly waiting |
| Wall | Where elapsed time goes, waiting included | Threads are idle by design (pools, event loops) |
| Allocation | Who allocates, and how much | GC pressure isn't the problem |
| Heap / live | What is retained | You need churn, not footprint |
| Lock / contention | Who waits on whom | Contention is outside the process (DB, pool) |

Slow on wall but cheap on CPU means it's waiting: look at I/O, locks, and pool
checkout. Hot on both CPU and allocation usually means GC. Fix the allocation,
not the collector.

## Local profilers

Reach for one when production profiles don't exist, or when a fix needs
confirming before it ships.

| Language | Sampling profiler | Notes |
|---|---|---|
| Ruby | `vernier`, `stackprof`, `rbspy` (attach to a live pid) | vernier sees GVL and GC waits; singed scopes any of them to one block, spec or request ([local-ruby.md](local-ruby.md)) |
| Rust | `samply`, `cargo flamegraph` | Profile a release build with debug symbols |

A debug build, a cold cache, or a noisy laptop each make the profile describe
something other than production. Say what conditions you measured under.
