# Data Layout, Allocation and Compression

Reference for the `algorithmist` persona. Representation is often worth more than algorithm. The same O(n) scan runs an order of magnitude apart depending on whether the bytes it touches are contiguous, compact and allocation-free. Before choosing a clever structure, check whether a better layout of a dumb one is enough.

## When you see this → reach for

| Problem shape | Reach for | Trade-off | Use |
|---|---|---|---|
| Hot loop reads one or two fields of many records | struct-of-arrays (columnar in memory) | awkward to pass a "record" around | parallel `Vec`s; Arrow arrays |
| Many small objects with one shared lifetime | arena / bump allocation | free all at once, not individually | `bumpalo`, `typed-arena` |
| Graph or tree with lots of cross-references | indices into a `Vec` instead of pointers | manual bounds; stale indices | plain `u32` ids; `slotmap` (generational keys catch staleness) |
| The same strings repeated many times | interning (store once, pass an id) | an interner to own and share | `lasso`, `string-interner`; `Arc<str>` for a few repeats |
| Collections that are usually tiny | small-vector optimization | larger inline size | `smallvec`, `arrayvec` |
| Read a big file randomly without loading it | memory mapping | page faults; SIGBUS if the file shrinks | `memmap2`; the OS page cache does the caching |
| Load serialized data without parsing it | zero-copy formats | layout constraints; validation cost | FlatBuffers, Cap'n Proto, `rkyv` |
| Sorted integer ids (postings, timestamps) | delta + varint, or block bit-packing | sequential decode | `integer-encoding` (varint/zigzag), `bitpacking` (SIMD) |
| Signed small integers | ZigZag, then varint | — | protobuf `sint` encoding |
| Long runs of repeated values | run-length encoding | bad on noisy data | Parquet/Arrow encodings, Roaring run containers |
| Low-cardinality strings in a column | dictionary encoding | the dictionary itself | Parquet, DuckDB, Arrow dictionary arrays |
| General-purpose compression, speed first | LZ4 | modest ratio | `lz4_flex`, liblz4 |
| General-purpose compression, balanced; tunable levels | Zstandard | slower at high levels | `zstd`, libzstd |
| Many small similar payloads (records, messages) | Zstandard with a trained dictionary | a dictionary to version and ship | `zstd --train` |
| Compatibility with everything | gzip / deflate | slower, weaker than zstd | zlib, `flate2` |
| Static web assets | Brotli | slow to compress | CDN or build-step compression |
| Bitvectors with fast rank/select; compressed sets of sorted ints | succinct structures (rank/select, Elias-Fano, wavelet trees) | complex; near-entropy space | `sucds` (Rust), sdsl-lite (C++) |

## Reach-for notes

- **Cache lines, not bytes.** Memory moves in cache lines (commonly 64 bytes). A pointer hop to a random location costs a potential miss; a sequential scan is prefetched. Shrinking a hot struct so more of them fit per line, or splitting hot fields from cold ones, is often the cheapest big win (see [where-the-time-goes.md](../where-the-time-goes.md)).
- **Allocation is a cost centre.** Per-item allocation in a hot loop routinely dominates the arithmetic. Hoist buffers, reuse them, borrow instead of cloning, and allocate from an arena when lifetimes are shared.
- **Indices over pointers** make graphs cheap to build, serialize and share across threads, and sidestep Rust's borrow-checker friction with cyclic structures. Generational indices (`slotmap`) turn a use-after-free into a failed lookup.
- **Compression can make things faster:** when data is I/O- or memory-bandwidth-bound, decoding a compact block is cheaper than moving the bytes it replaces. Compressed postings and columnar encodings rely on this.
- **Small payloads compress poorly** because there is no history to learn from. Zstandard's training mode builds a dictionary from samples to fix that ([README](https://github.com/facebook/zstd)). Version the dictionary with the data.
- **Zero-copy and mmap** move cost from load time to access time and push validation onto you. Treat mapped or untrusted bytes as untrusted input.

## Classic failure modes

- Array-of-structs with fat records scanned for one field.
- A `Vec<Box<T>>` or linked nodes where a flat `Vec<T>` would do.
- Interning without bounds, which is a leak for long-running processes fed unbounded input.
- Compressing already-compressed data (images, archives), or compressing tiny messages without a dictionary.
- A custom on-disk format with no version field, which becomes impossible to evolve. Prefer a standard container format.
- Benchmarking layout changes on data that fits in L1 when production doesn't.

**Depth:** Drepper, *What Every Programmer Should Know About Memory* (2007); Sedgewick and Wayne §5.5 (data compression); Navarro, *Compact Data Structures* (2016); the Parquet and Arrow format specifications.
