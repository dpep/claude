# Searching and Indexing

Reference for the `algorithmist` persona. An index is a precomputed answer to one shape of question — point, range, prefix, containment, nearest — and every write pays to keep it fresh. Name the question, then check whether a flat scan already answers it inside budget.

## When you see this → reach for

| Problem shape | Reach for | Trade-off | Use |
|---|---|---|---|
| n small, or queried rarely | flat scan over a contiguous array | O(n) at memory bandwidth; no write cost | a `Vec`; struct-of-arrays for the hot field |
| Exact key → value | hash table | O(1) expected; no order or ranges | stdlib map (SwissTable in Rust and Abseil, Go 1.24+) |
| Build once, point and range lookups | sorted array + binary search | cheapest to build; O(n) insert | `binary_search`, `partition_point`, `bisect` |
| Ordered map with updates, ranges | B-tree / B+ tree | O(log n), shallow, cache-friendly | Rust `BTreeMap`; every database's default index |
| Write-heavy key-value store on disk | LSM tree | fast sequential writes; compaction and read amplification | RocksDB, LevelDB, `fjall` (Rust) |
| Ordered map, concurrent, in memory | skip list | simple concurrency; pointer-chasing | Java `ConcurrentSkipListMap`, `crossbeam-skiplist` |
| Which documents contain these terms | inverted index | postings per term; segment merges | Lucene, `tantivy`, SQLite FTS5, PostgreSQL GIN |
| Set algebra over integer ids | Roaring bitmap | compressed and fast; 32-bit ids by default | `roaring`, CRoaring |
| Aggregate a few columns over many rows | columnar layout | fast scans; slow row updates | Parquet, Arrow, DuckDB |
| A query reads only indexed columns | covering index | bigger index | SQLite: extra key columns; PostgreSQL 11+: `INCLUDE` |
| Hot query touches a small subset of rows | partial index | planner must prove the `WHERE` | SQLite 3.8.0+, PostgreSQL |
| Predicate on `lower(x)` or a JSON field | expression index | maintained on every write | SQLite 3.9.0+, PostgreSQL |
| Points near a location; boxes overlapping a box | R-tree (R*-tree) | good general spatial index | SQLite R*Tree module, PostGIS (GiST), `rstar` |
| k nearest points, low dimension | k-d tree | degrades past ~10–20 dimensions | `kiddo`, SciPy `cKDTree` |
| Nearest vectors, high dimension (embeddings) | ANN index | approximate; see ML map | [machine-learning.md](./machine-learning.md) |
| Shard or bucket locations by cell | geohash, S2, or H3 cells | cell edges distort distance | S2 (spherical cells), H3 (hexagons) |
| Huge append-only table, time-ordered | block-range summary index | tiny; coarse | PostgreSQL BRIN |

## Reach-for notes

- **Measure the scan first.** An index must beat it by enough to pay for its write cost and its code.
- **Hash tables:** pre-size when n is known; iteration order is arbitrary (per-process in Rust). For build-once/read-many, a sorted `Vec` is often cheaper to build (see [hashing-and-caching.md](./hashing-and-caching.md)).
- **B-trees:** in memory, Rust's `BTreeMap` stores B−1 to 2B−1 keys per node and searches each node linearly ([docs](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html)). On disk, the top levels stay cached, so a lookup is one or two I/Os. Prefix queries seek only as a range (`>= 'ab' AND < 'ac'`); a leading-wildcard `LIKE '%x'` can't use a B-tree.
- **PostgreSQL index-only scans** still check the visibility map, and visit the heap for recently changed pages ([docs](https://www.postgresql.org/docs/current/indexes-index-only-scans.html)). The other index types: Hash, GiST/SP-GiST (geometry, ranges, nearest, trigram similarity), GIN (arrays, JSONB, full text, trigrams), BRIN, `bloom` ([docs](https://www.postgresql.org/docs/current/indexes-types.html)).
- **LSM trees** trade write, read and space amplification: leveled compaction writes more, tiered compaction reads more. Choose one for write-heavy point lookups; prefer a B-tree for read-heavy work or predictable latency.
- **Postings:** delta-encode ids and use a block codec (see [data-layout-and-compression.md](./data-layout-and-compression.md)); intersect smallest-first with galloping search; freshness comes from immutable segments plus merge.
- **Roaring** splits ids into 2^16-value containers: arrays up to 4,096 values, bitsets above that, and run containers ([format spec](https://github.com/RoaringBitmap/RoaringFormatSpec)).
- **Spatial:** an R-tree is the default for "what overlaps this box" and "nearest to this point". Cells (geohash, S2, H3) turn spatial questions into key lookups and sharding keys; query neighbouring cells too, or you'll miss matches at the edges.

## Classic failure modes

- An index for a query that runs once: construction costs more than the scan.
- An index answering a neighbouring question: a B-tree for `%x%`, a hash for a range, a k-d tree for 768-dimensional embeddings.
- Forgetting the write side: every index is paid for on every insert, update and delete.
- The index exists and isn't used — a collation mismatch, a function around the column, a partial predicate not implied. Run `EXPLAIN` on the real statement.
- No freshness story: rebuild-only structures go stale in production.

**Depth:** Sedgewick and Wayne ch. 3 and §6.2; Petrov, *Database Internals* (B-trees, LSMs); Skiena, *Algorithm Design Manual* catalog: "Dictionaries", "Kd-Trees", "Range Search", "Nearest Neighbor Search"; Guttman (1984) for R-trees.
