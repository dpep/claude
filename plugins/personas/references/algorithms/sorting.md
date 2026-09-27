# Sorting and Selection

Reference for the `algorithmist` persona. You almost never write a sort. You choose one, and more often you choose not to sort at all. The standard library's sort beats a hand-rolled one; the real decisions are whether you need a total order, whether ties must keep their order, and whether the data fits in memory.

## When you see this → reach for

| Problem shape | Reach for | Trade-off | Use |
|---|---|---|---|
| Need sorted order, ties don't matter | stdlib unstable sort | in place, fastest; ties reorder | Rust `sort_unstable`, C++ `std::sort`, Go `slices.Sort` |
| Equal keys must keep input order | stdlib stable sort | allocates up to n | Rust `sort`, Python `sorted`, C++ `std::stable_sort` |
| Many integers or fixed-width keys | radix / counting sort | O(n·key width); loses on small n and long keys | measure against the stdlib sort first |
| k largest or smallest, k ≪ n | bounded heap of size k | O(n log k), streams | `BinaryHeap` + `Reverse`, Python `heapq.nlargest` |
| The k-th element, or an unordered top k | quickselect | O(n), reorders the input | Rust `select_nth_unstable`, C++ `nth_element` |
| Sorted top k | select, then sort the k | O(n + k log k) | C++ `partial_sort` |
| `ORDER BY … LIMIT k` in SQL | let the DB do a bounded sort, or index the order | — | PostgreSQL shows `top-N heapsort`; a matching index skips the sort |
| More data than RAM | external merge sort | I/O dominates | the database, Unix `sort`, DuckDB |
| Grouping or dedup, order irrelevant | hash map, not a sort | O(n) expected, no order | stdlib map |
| Build once, look up many | sort once into an array, then binary search | cheap to build, O(n) insert | `Vec` + `binary_search` |

## What the stdlibs ship (check your version)

- **Rust:** `sort_unstable` is ipnsort (quicksort with a heapsort fallback, linear on sorted input). `sort` is driftsort (stable, with run detection; allocates up to `len`, clamped to `len/2` for large slices). `select_nth_unstable` is introselect with a median-of-medians fallback, so it is linear in the worst case ([slice docs](https://doc.rust-lang.org/std/primitive.slice.html)).
- **Go:** pattern-defeating quicksort since 1.19. **Python:** Timsort, with Powersort's merge policy since 3.11. **Java:** dual-pivot quicksort for primitives, a stable Timsort-style merge for objects.

These detect runs, guard against quadratic inputs, and special-case tiny partitions. Hand-rolled quicksorts do none of that.

## Reach-for notes

- **Unstable by default.** Stable only when equal keys must keep input order: multi-pass sorts, UI lists, deterministic output.
- **An expensive key?** Compute it once: `sort_by_cached_key`, Python `key=`, decorate-sort-undecorate.
- **Large elements?** Sort indices or `(key, index)` pairs, then permute once.
- **Floats?** Use `total_cmp` or equivalent. NaN breaks `<` as a total order.
- **Sort vs hash vs index:** hash for one-off equality work; sort when the output must be ordered or you'll merge-join; a B-tree or sorted structure when order is needed *repeatedly* under updates (see [searching-and-indexing.md](./searching-and-indexing.md)).
- **External sort in PostgreSQL:** a sort over `work_mem` spills to disk (`Sort Method: external merge`). Raise `work_mem` for that query or add a matching index ([EXPLAIN docs](https://www.postgresql.org/docs/current/using-explain.html)).

## Classic failure modes

- Sorting to get a max, a min or the top 10.
- Sorting inside a loop.
- A comparator that isn't a total order: undefined results, and a panic in Rust.
- Relying on stability from an unstable sort, or on hash map iteration order.
- Collation mismatch between the sort and the search or merge that relies on it.
- Benchmarking on random data only. Real data is often nearly sorted, and adaptive sorts exploit that.

**Depth:** Sedgewick and Wayne, *Algorithms* ch. 2; CLRS part II; Orson Peters' pdqsort paper (arXiv:2106.05123).
