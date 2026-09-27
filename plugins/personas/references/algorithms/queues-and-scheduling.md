# Queues, Scheduling and Rate Limiting

Reference for the `algorithmist` persona. A queue doesn't make work faster. It moves where the waiting happens, and an unbounded one hides overload until memory runs out. The real decisions are bounds, backpressure and fairness. The data structure is usually `VecDeque` or a channel from the standard library.

## When you see this → reach for

| Problem shape | Reach for | Trade-off | Use |
|---|---|---|---|
| FIFO in one thread | ring buffer / deque | O(1); contiguous | Rust `VecDeque`, Python `collections.deque`, C++ `std::deque` |
| Highest priority next | binary heap (4-ary for big heaps) | O(log n); no in-place priority change | Rust `BinaryHeap` + `Reverse`, Python `heapq` |
| Hand work between threads | bounded channel | a lock or atomics per operation | `std::sync::mpsc`, `crossbeam-channel`, `flume`, Go channels |
| Hand work between async tasks | bounded async channel | `send` waits when full | `tokio::sync::mpsc` |
| One producer, one consumer, real-time | lock-free SPSC ring | wait-free, fixed size | `rtrb`, `ringbuf` |
| Many small CPU tasks | work-stealing pool | overhead if tasks are tiny | `rayon`, Tokio, Java `ForkJoinPool`; see [parallelism-and-concurrency.md](./parallelism-and-concurrency.md) |
| Producer faster than consumer | bound + backpressure, reject, or drop-oldest | latency vs loss | bounded channels; CoDel for time-in-queue limits |
| One tenant starving the others | per-tenant queues, round-robin or weighted fair | more queues to manage | deficit round-robin |
| Many timers, mostly cancelled (timeouts) | hierarchical timing wheel | O(1) start/stop; tick resolution | Tokio's timer, Netty `HashedWheelTimer` |
| Few timers, exact order | heap of deadlines | O(log n) | `BinaryHeap<Reverse<Instant>>` |
| Retry a failing call | capped exponential backoff + full jitter + a retry budget | slower recovery for the unlucky | client library retry policies |
| Allow N requests per period, with bursts | token bucket / GCRA | burst size is a parameter | `governor` (Rust, GCRA), API gateways |
| Smooth output to a fixed rate | leaky bucket (as a queue) | adds delay | nginx `limit_req`, traffic shapers |
| Rate limit shared across servers | a counter in shared storage (sliding window or GCRA) | a round-trip per check | Redis-based limiters |
| How many workers, how deep a queue? | Little's law: L = λW | — | arithmetic |

## Reach-for notes

- **Linked-list queues** allocate per node and chase pointers. A ring buffer is almost always better.
- **Heaps:** mutating an item's priority in place corrupts the heap; use lazy deletion (push the new entry, skip stale ones on pop). Fibonacci heaps lose to binary heaps on constants. Ties pop in unspecified order; add a sequence number to break them.
- **Bound every queue that crosses a rate boundary.** Capacity ÷ throughput = worst-case wait, so size queues from the latency you can tolerate. Rejecting fast beats finishing work after the caller has timed out.
- **A `Mutex<VecDeque>` plus a condvar is a fine MPMC queue** until a profile shows contention. Rust's `std::sync::mpsc` has been built on a port of `crossbeam-channel` since 1.67. Never hand-roll a lock-free MPMC queue.
- **Timing wheels** (Varghese and Lauck, SOSP 1987) give O(1) timers at tick resolution. Tokio uses six levels of 64 slots ([source](https://github.com/tokio-rs/tokio/blob/master/tokio/src/runtime/time/wheel/mod.rs)).
- **Backoff:** `sleep = random(0, min(cap, base·2^attempt))`. In Brooker's analysis, full jitter did the least total work; equal jitter was worst of the jittered variants ([AWS blog](https://aws.amazon.com/blogs/architecture/exponential-backoff-and-jitter/)). Retry only idempotent operations. Retries multiply across layers (3 × 3 × 3 = 27×).
- **Rate limiting:** a token bucket allows bursts up to the bucket size, then the refill rate. GCRA gives the same behaviour with a single timestamp per key, which is why `governor` uses it ([docs](https://docs.rs/governor)). Fixed windows allow 2× bursts at window edges; sliding windows or GCRA avoid that.
- **Queueing basics:** Little's law holds for any stable system: 200 requests/s at 50 ms means ~10 in flight. In an M/M/1 queue, time in system is service time ÷ (1 − utilization): 2× at 50%, 10× at 90%, 100× at 99%. Keep headroom.

## Classic failure modes

- Unbounded queues as the default.
- Retry storms: no jitter, no cap, retries at every layer.
- Head-of-line blocking: one slow item stalls a shared FIFO. Split queues by class.
- Blocking inside an async executor or a work-stealing pool.
- Strict priorities without aging, which starve the low classes.
- Load-testing below the knee; users feel behaviour near saturation.

**Depth:** Harchol-Balter, *Performance Modeling and Design of Computer Systems*; Nichols and Jacobson, "Controlling Queue Delay" (ACM Queue, 2012); Sedgewick and Wayne §1.3 and §2.4.
