# Hashing and Caching

Reference for the `algorithmist` persona. Choose a hash function for a threat model and a stability requirement, not for speed alone. A cache is a second copy of the truth, and the invalidation is the cost you take on with it.

## When you see this → reach for

| Problem shape | Reach for | Trade-off | Use |
|---|---|---|---|
| Hash map, keys from untrusted input | keyed, randomly seeded hash | slower on small keys | Rust default (SipHash-1-3), `ahash` |
| Hash map, trusted keys, hashing shows in the profile | fast non-DoS-resistant hash | HashDoS exposure | `rustc-hash` (FxHash), `foldhash` |
| Checksums, persisted fingerprints, stable shard keys | fast hash with stable output | not collision-resistant | xxHash (XXH3/XXH64) |
| Integrity, dedup, or naming across trust boundaries | cryptographic hash | slower | BLAKE3, SHA-256; see [cryptography.md](./cryptography.md) |
| Spread keys over nodes that come and go | rendezvous hashing, or a consistent-hash ring with virtual nodes | O(nodes) per lookup / ring upkeep | most client libraries |
| Shard over numbered buckets that only grow | jump consistent hash | buckets must be sequential | ~5 lines (Lamping and Veach 2014) |
| Bounded in-memory cache, general workload | W-TinyLFU or S3-FIFO | more machinery than LRU | Caffeine (Java), `moka`, `quick_cache` (Rust) |
| Tiny single-threaded cache | LRU | a single scan flushes it | `lru` crate, Python `functools.lru_cache` |
| Repeated calls to a pure function | memoization, bounded | wrong if the function isn't pure | `lru_cache`; `moka` with per-key `get_with` |
| Hot key expiring under load | single-flight, serve-stale, or probabilistic early refresh | — | XFetch (Vattani et al., VLDB 2015) |
| Immutable blobs that dedupe themselves | content addressing | the name→hash pointer needs its own consistency | git, OCI images, Nix |

## Reach-for notes

- **Rust's `HashMap`** defaults to seeded SipHash-1-3. The docs say faster hashes win on small and large keys, but typically without HashDoS protection ([docs](https://doc.rust-lang.org/std/collections/struct.HashMap.html)).
  - `foldhash` is `hashbrown`'s default and "minimally DoS-resistant" ([README](https://github.com/orlp/foldhash)).
  - `ahash` is keyed and intended only for in-memory maps ([README](https://github.com/tkaitchuck/aHash)).
  - FxHash is what `rustc` uses on non-adversarial input ([README](https://github.com/rust-lang/rustc-hash)).
- **Python** hashes `str` and `bytes` with SipHash-1-3 since 3.11.
- **Never persist a hash map hasher's output.** Seeds vary per process and algorithms change between releases. xxHash's outputs are stable across platforms and releases ([README](https://github.com/Cyan4973/xxHash)).
- **Consistent hashing** (Karger et al. 1997) moves about 1/n of the keys when a node changes. Without many virtual nodes, the load is uneven. **Rendezvous** (highest random weight) needs no ring and gives top-r replicas naturally. **Jump hash** needs no storage but can only add or remove the last bucket ([paper](https://arxiv.org/abs/1406.2294)). **Maglev** (NSDI 2016) is the load-balancer variant.
- **Eviction policies:**
  - **LRU/CLOCK** favour recency and are flushed by scans.
  - **ARC** (FAST 2003) adapts between recency and frequency.
  - **TinyLFU** adds *admission*: a count-min-style frequency sketch decides whether a newcomer beats the eviction victim ([paper](https://arxiv.org/abs/1512.00727)).
  - **S3-FIFO** (SOSP 2023) and **SIEVE** (NSDI 2024) are recent FIFO-based policies with strong hit ratios.
  - Replay a real access trace before choosing on published benchmarks.
- **Size by weight**, not entry count, when entries vary in size.
- **Memoize only pure functions.** Key on every input, including a version for the code or rules that compute the result. Bound the table.
- **Invalidation patterns:**
  - **TTL:** bounded staleness, no coordination.
  - **Cache-aside:** on write, delete the entry rather than updating it, so racing writers can't leave an old value.
  - **Write-through:** consistent; slower writes.
  - **Write-behind:** fast; loses data on a crash.
  - **Versioned keys** (`user:42:v7`): no delete races.
  - **Change-stream invalidation:** precise, but only as reliable as event delivery.
- **Content addressing** makes caches immutable and dedup free. It needs a cryptographic hash when content crosses trust boundaries. Content-defined chunking keeps dedup stable across inserts in large files.

## Classic failure modes

- A fast unseeded hash on attacker-controlled keys (HashDoS).
- Persisting a process-seeded or version-unstable hash.
- A cache with no invalidation story; staleness bugs look like data corruption.
- Unbounded caches and memo tables — memory leaks with good intentions.
- Caching errors or empty results with the success TTL.
- Hit ratio measured on uniform synthetic keys; real access is skewed.
- A cache hiding a slow path that becomes the outage on a cold start.

**Depth:** Kleppmann, *Designing Data-Intensive Applications* (partitioning, caching); Caffeine's [efficiency wiki](https://github.com/ben-manes/caffeine/wiki/Efficiency); Skiena's catalog: "Dictionaries".
