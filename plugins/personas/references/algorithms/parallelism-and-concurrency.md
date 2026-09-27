# Parallelism and Concurrency

Reference for the `algorithmist` persona. Parallel speedup is bounded by the serial fraction and eaten by contention. The usual bottleneck isn't the algorithm; it's a shared lock, a shared cache line, or a queue with no bound. Reach for the highest-level tool that fits, and keep output deterministic.

## When you see this → reach for

| Problem shape | Reach for | Trade-off | Use |
|---|---|---|---|
| Same operation over a big collection (map, filter, reduce) | data parallelism via parallel iterators | needs enough work per item | `rayon` `par_iter`; Java parallel streams; shell: `xargs -P`, GNU `parallel` |
| Recursive divide-and-conquer, irregular task trees | fork-join over work stealing | tiny tasks drown in overhead | `rayon::join` and `scope`, Java `ForkJoinPool` |
| Stages that run at different speeds | pipeline of bounded channels | a stage's slowness backs up upstream (by design) | `crossbeam-channel`, `tokio::sync::mpsc`; see [queues-and-scheduling.md](./queues-and-scheduling.md) |
| Fan out N requests, gather the results | fan-out/fan-in with a concurrency limit | unbounded fan-out overwhelms the target | Tokio `JoinSet` + `Semaphore`, `futures::stream::buffer_unordered` |
| Thousands of concurrent I/O waits (sockets, HTTP) | async I/O | colored functions; blocking calls stall the executor | Tokio; move blocking work to `spawn_blocking` |
| A few CPU-bound jobs | OS threads or a thread pool | per-thread stack and scheduling cost | `std::thread::scope`, `rayon` |
| State owned by one place, many writers | message passing (an actor owns the state) | extra hops; bounded mailboxes needed | a task + channel; Erlang/OTP, Akka |
| Small shared state, short critical sections | `Mutex` | contention at high core counts | `std::sync::Mutex`, `parking_lot` |
| Read-mostly shared data, occasional swaps | `RwLock`, or atomic pointer swap | `RwLock` writers can starve or stall | `arc-swap` for read-mostly config |
| Concurrent map, many writers | sharded map | no global snapshot | `dashmap`, Java `ConcurrentHashMap` |
| Counters, flags, one-time init | atomics | ordering rules | `AtomicU64`, `OnceLock` |
| A lock-free structure | a library's | memory reclamation is the hard part | `crossbeam` (epoch GC), `crossbeam-skiplist` |
| Parallel sort / prefix sum | library parallel primitives | only pays at large n | `rayon` `par_sort_unstable`; parallel scan in GPU/array libraries |
| Stop work that is no longer needed | cooperative cancellation + timeouts | every loop must check | Tokio `CancellationToken` (`tokio-util`), dropping futures, `select!` with a timer |

## Reach-for notes

- **Amdahl's law:** with a parallel fraction p on N cores, speedup ≤ 1 / ((1 − p) + p/N). 95% parallel caps at 20×, whatever N is. **Gustafson** (CACM 1988): grow the problem with the cores and the scaled speedup is (1 − p) + p·N. Know which regime you're in. Measure speedup at 1, 2, 4, 8 threads; the curve tells you where the serial part and the contention are.
- **Granularity:** batch tiny items into chunks (`with_min_len`, `par_chunks`) so scheduling overhead stays small next to the work.
- **Contention and false sharing** are the usual real bottleneck. A single hot lock, a shared atomic counter, or two threads writing different fields on one cache line serializes the cores through cache coherence traffic. Fixes: per-thread accumulators merged at the end, sharding, and padding hot fields to a cache line (`crossbeam_utils::CachePadded`).
- **Locks:**
  - An uncontended mutex costs an atomic operation or two, so a mutex is fine until a profile says otherwise.
  - Hold locks briefly, never across I/O or `.await`.
  - Tokio's docs recommend the ordinary `std::sync::Mutex` in async code unless the guard must be held across an `.await` ([docs](https://docs.rs/tokio/latest/tokio/sync/struct.Mutex.html)).
- **Atomics at a glance:**
  - `Relaxed` for independent counters.
  - `Release` on the write that publishes data, paired with `Acquire` on the read that consumes it.
  - `SeqCst` when unsure.
  - Most "lock-free" bugs are a missing Acquire/Release pair or the ABA problem. Mara Bos's book is the practical guide.
- **Async vs threads:** async wins when you're waiting on many things at once. Threads (or `rayon`) win for CPU work. Mixing them means `spawn_blocking` for CPU or blocking calls inside async, and bounded concurrency for outbound calls.
- **Determinism:** parallel completion order is random. Collect in input order (`rayon`'s `collect` into a `Vec` preserves it for indexed iterators), or **sort before you truncate**, with a total tie-break key. Seed RNGs per task, not from a shared global.
- **Deadlock avoidance:** acquire locks in one global order; never call unknown code (callbacks, trait objects) while holding a lock; use `try_lock` or timeouts where a cycle is possible. `parking_lot` has an experimental deadlock detector. **Livelock:** retries that collide forever; add jitter. **Starvation:** unfair locks and strict priorities; prefer fair queues or aging.

## Classic failure modes

- Parallelizing something that is I/O-bound on one disk, or already memory-bandwidth-bound. More threads just add contention.
- One global `Mutex<HashMap>` behind every request.
- Blocking calls on an async executor thread.
- Unbounded spawning: a task per item with no limit.
- Non-deterministic output from parallel stages, leaking into tests, caches or diffs.
- Measuring speedup on a busy machine; interleave runs, as in the performance-engineer's method.

**Depth:** Herlihy and Shavit, *The Art of Multiprocessor Programming*; McKenney, [*Is Parallel Programming Hard, And, If So, What Can You Do About It?*](https://mirrors.edge.kernel.org/pub/linux/kernel/people/paulmck/perfbook/perfbook.html) (free); Bos, [*Rust Atomics and Locks*](https://marabos.nl/atomics/) (free online); CLRS ch. "Parallel Algorithms".
