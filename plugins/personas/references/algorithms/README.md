# Algorithms

Reference for the `algorithmist` persona: a map of what's out there and when to reach for it, not a textbook. Each page leads with a *when you see this → reach for* table — the problem shape, the standard answer, what it trades, and the library that already implements it — then short notes, failure modes, and pointers to canonical sources.

The stance throughout is the persona's own:

- Name the question the consumer actually asks.
- Measure the naive baseline.
- Prefer the library over the hand-rolled version.
- Prove a fast path equivalent to ground truth before trusting its speed.

## The maps

| Page | Reach for it when you're… |
|---|---|
| [sorting.md](./sorting.md) | ordering, top-k, selection, external sorts, or deciding sort vs hash vs index |
| [searching-and-indexing.md](./searching-and-indexing.md) | looking things up: hash tables, B-trees, LSMs, inverted indexes, bitmaps, columnar, spatial indexes |
| [string-matching.md](./string-matching.md) | matching text: exact, multi-pattern, regex, n-gram and suffix indexes, edit distance, fuzzy pickers, diff and merge |
| [trees-and-graphs.md](./trees-and-graphs.md) | working with trees (balanced, Fenwick, segment, interval, persistent) and graphs (dependency order, cycles, paths, components, flow) |
| [hashing-and-caching.md](./hashing-and-caching.md) | choosing a hash function, spreading keys over nodes, eviction, memoization, invalidation, content addressing |
| [probabilistic-structures.md](./probabilistic-structures.md) | trading exactness for memory: Bloom/cuckoo/xor filters, HyperLogLog, count-min, reservoir sampling, MinHash |
| [queues-and-scheduling.md](./queues-and-scheduling.md) | moving work between producers and consumers: bounds, backpressure, priorities, timers, retries, rate limits |
| [parallelism-and-concurrency.md](./parallelism-and-concurrency.md) | using more cores or more concurrent I/O: data and task parallelism, locks, atomics, async, determinism, deadlock |
| [distributed-systems.md](./distributed-systems.md) | coordinating machines: consensus, leases and fencing, CRDTs, logical clocks, IDs, idempotency |
| [data-layout-and-compression.md](./data-layout-and-compression.md) | making data smaller or closer: struct-of-arrays, arenas, interning, encodings, compression, succinct structures |
| [cryptography.md](./cryptography.md) | protecting data: AEAD, password hashing, MACs, signatures, key exchange, randomness — never rolling your own |
| [numerical-methods.md](./numerical-methods.md) | computing with floats and statistics: precision, linear algebra, solvers, randomness, percentiles, A/B comparisons |
| [machine-learning.md](./machine-learning.md) | deciding whether a model beats a rule: vector search, classic models, ranking, evaluation, LLMs as a component |

For *where the time actually goes* — measurement, not algorithm choice — see the performance engineer's [where-the-time-goes.md](../where-the-time-goes.md).

## Ways to think: design paradigms and their tells

Most new problems are an old paradigm in disguise. Recognize the shape and the method follows.

| Paradigm | The tell-tale shape | Examples |
|---|---|---|
| Brute force, made cheap | small n, or a filter that makes each check nearly free | a flat scan over a compact array; SIMD comparison |
| Divide and conquer | the answer for the whole combines answers for independent halves | merge sort, binary search, parallel reduction |
| Dynamic programming | overlapping subproblems with optimal substructure: "best way to reach state i" | edit distance, diff (LCS), knapsack, shortest paths on DAGs, parsing |
| Greedy | a locally best choice provably never hurts (an exchange argument) | Dijkstra, Kruskal/Prim, interval scheduling, Huffman coding |
| Backtracking / branch and bound | build a solution choice by choice, abandoning dead prefixes | constraint puzzles, dependency resolution, generating permutations or subsets |
| Filter then verify | an exact check is expensive; a necessary condition is cheap | Bloom filters, q-gram filters, index candidates plus a real matcher |
| Sweep line / two pointers | sorted events or intervals processed in order | interval overlap, merge joins, windowed stats |
| Precompute / index | the same question asked many times over slow-changing data | every index; memoization; prefix sums |
| Randomization | adversarial or unknown inputs; approximate answers suffice | quicksort pivots, hashing, sketches, sampling |
| Reduce to a solved problem | the problem is a known one wearing a costume | bipartite matching for assignment; SAT or MIP solvers for configuration |

**Recognize hard problems early.** Scheduling with constraints, graph colouring, TSP, set cover, bin packing and exact version solving are NP-hard. Reach for a heuristic, an approximation, or an off-the-shelf solver (SAT or SMT, MIP, constraint programming — e.g. OR-Tools; PubGrub for package versions) before hunting for a clever exact algorithm.

## Canonical sources

- Skiena, *The Algorithm Design Manual* — its problem catalog is the best practitioner's index, organized by "I have this problem".
- Cormen, Leiserson, Rivest and Stein, *Introduction to Algorithms* (CLRS) — the reference for correctness and complexity.
- Sedgewick and Wayne, *Algorithms* (4th ed.) and [algs4.cs.princeton.edu](https://algs4.cs.princeton.edu/) — clear implementations of the core.
- [cp-algorithms.com](https://cp-algorithms.com/) — concise write-ups of specific algorithms and structures.
- Kleppmann, *Designing Data-Intensive Applications* — storage, replication and distributed trade-offs.
- Each page lists its own deeper sources.
