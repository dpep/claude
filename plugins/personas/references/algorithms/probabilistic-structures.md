# Probabilistic Structures

Reference for the `algorithmist` persona. A sketch trades a bounded, known error for orders of magnitude less memory. It's acceptable when you can state the error, the consumer tolerates it (or an exact check follows), and you've measured the bound on real data.

## When you see this → reach for

| Problem shape | Reach for | Error / cost | Use |
|---|---|---|---|
| Skip expensive lookups for keys that definitely aren't there | Bloom filter | false positives only; ~1.44·log2(1/ε) bits per item | RocksDB/LevelDB per-file filters, PostgreSQL `bloom`, many crates |
| Same, with deletes | cuckoo filter | smaller than Bloom when ε < ~3%; inserts can fail near full | reference implementation and ports |
| Same, for a set rebuilt in batches | binary fuse / xor filter | immutable; within ~13% / ~23% of the space lower bound | `xorf` (Rust) |
| Count distinct items | HyperLogLog(++) | ~1.04/√m relative error; mergeable | Redis `PFADD`/`PFCOUNT`, analytics databases |
| Frequency of an item; heavy hitters | count-min sketch (+ a top-k heap) | overestimates only, by ε·(total count) | Redis Stack, streaming libraries |
| Percentiles of latencies or values | quantile sketch | bounded rank or relative error; mergeable | HdrHistogram, t-digest, DDSketch; see [numerical-methods.md](./numerical-methods.md) |
| Uniform sample of a stream of unknown length | reservoir sampling | exact uniformity; O(k) memory | Algorithm R / L; most stats libraries |
| Similar sets / near-duplicate documents | MinHash + LSH banding | estimates Jaccard similarity | datasketch (Python) and many ports |
| Compact fingerprint, near-duplicate text | SimHash | Hamming distance tracks cosine similarity | 64-bit fingerprints (Manku et al., WWW 2007) |

## Reach-for notes

- **Safe shapes:** a filter in front of an exact check, where an error costs only a wasted lookup; or an estimate that feeds humans or thresholds. **Unsafe:** billing, access control, uniqueness — anywhere an error becomes a wrong action with no verification.
- **Bloom math:** `ε ≈ (1 − e^(−kn/m))^k`, best at `k = (m/n)·ln 2`. 10 bits per item with k = 7 gives ~0.8%. Size for the *final* n. Derive k positions from two hashes as `h1 + i·h2` (Kirsch and Mitzenmacher). Blocked Bloom filters keep an item in one cache line. Check whether your storage engine already has one: RocksDB does, and so does SQLite's planner for large joins, since 3.38.0.
- **Cuckoo filters** (Fan et al., CoNEXT 2014) support deletes, but deleting a never-inserted item can cause false negatives ([paper](https://dl.acm.org/doi/10.1145/2674005.2674994)). **Xor and binary fuse filters** (Graf and Lemire 2020, 2022) are static and the smallest at a given false-positive rate ([paper](https://arxiv.org/abs/2201.01174)).
- **HyperLogLog** (Flajolet et al. 2007; HLL++ from Heule et al., EDBT 2013): unions are free (register-wise max). Intersections via inclusion-exclusion are poor. Redis uses up to 12 KB for 0.81% standard error ([docs](https://redis.io/docs/latest/develop/data-types/probabilistic/hyperloglogs/)). Report the estimate as a band.
- **Count-min** (Cormode and Muthukrishnan 2005): the error is relative to the *total*, so it is good for the frequent and useless for the rare. Conservative update and periodic halving (aging) help; TinyLFU uses both.
- **Reservoir sampling:** Algorithm R (Vitter, ACM TOMS 1985); Algorithm L (Li 1994) skips ahead. Weighted sampling needs Efraimidis-Spirakis keys (u^(1/w)), not a tweaked Algorithm R.
- **MinHash** (Broder 1997) estimates Jaccard similarity, with standard error at most 1/(2√k). LSH banding finds candidate pairs; verify them exactly.

## How to verify a sketch

- Compare against exact answers on a real sample, across tiny, typical and extreme sizes. A result much worse than the theoretical bound usually means a weak or correlated hash.
- Measure a filter's false-positive rate at the load you'll actually run, not the load it was sized for.
- Say the error in the output ("~12,400 ±1.6%"), and keep an exact path for audits and small inputs.

**Depth:** Cormode, Garofalakis, Haas and Jermaine, *Synopses for Massive Data* (Foundations and Trends in Databases, 2012); the papers linked above; the Apache DataSketches documentation.
