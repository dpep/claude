# Trees and Graphs

Reference for the `algorithmist` persona. Most "graph problems" in dev tools and services are a handful of classics in disguise: dependency order, reachability, shortest path, connected groups, cycles. Recognize the classic, then call the library. A textbook algorithm re-typed from memory is where the off-by-ones and stack overflows live.

## When you see this → reach for

| Problem shape | Reach for | Cost | Use |
|---|---|---|---|
| Ordered map in memory | B-tree (not a binary tree) | O(log n), cache-friendly | Rust `BTreeMap`; see [searching-and-indexing.md](./searching-and-indexing.md) |
| Ordered map where the stdlib gives you a red-black tree | red-black tree | O(log n), pointer per node | Java `TreeMap`, C++ `std::map` (typically) |
| Prefix lookup over keys | trie / radix tree / FST | O(key length) | see [string-matching.md](./string-matching.md) |
| Next-highest priority | heap | O(log n) | see [queues-and-scheduling.md](./queues-and-scheduling.md) |
| Prefix sums or range sums with point updates | Fenwick (binary indexed) tree | O(log n), tiny code | a few lines; cp-algorithms has a reference |
| Range min/max/sum with range updates | segment tree (lazy propagation) | O(log n), 2–4n memory | same |
| Which intervals overlap this point or range? | interval tree, or sort + sweep | O(log n + hits) | PostgreSQL range types with GiST; sweep for batch work |
| Immutable, cheaply-copied collections | persistent structures (HAMT, RRB vector, persistent B-tree) | O(log₃₂ n)-ish, structural sharing | `imbl`, `rpds` (Rust); Clojure, Scala, Immutable.js |
| Build order, task DAG, "what must run first" | topological sort (Kahn's or DFS) | O(V + E) | petgraph `toposort`, Python `graphlib`, `tsort` |
| Does this dependency graph have a cycle? which one? | DFS with colours / SCCs | O(V + E) | petgraph `is_cyclic_directed`, `tarjan_scc` |
| Mutually dependent groups (cycle clusters) | strongly connected components | O(V + E) | Tarjan or Kosaraju (petgraph, networkx) |
| Are x and y connected? merge groups as edges arrive | union-find (disjoint sets) | ~O(1) amortized | petgraph `UnionFind` |
| Reachable from here, fewest hops | BFS | O(V + E) | any graph library; easy to write iteratively |
| Cheapest path, non-negative weights | Dijkstra (binary heap) | O((V + E) log V) | petgraph `dijkstra`, networkx |
| Cheapest path with a good distance estimate | A* | depends on the heuristic | petgraph `astar` |
| Negative edge weights, or detecting negative cycles | Bellman-Ford | O(V·E) | petgraph `bellman_ford` |
| All pairs, small dense graph | Floyd-Warshall | O(V³) | petgraph `floyd_warshall` |
| Cheapest way to connect everything | MST: Kruskal (sort + union-find) or Prim | O(E log E) | petgraph `min_spanning_tree` |
| Assign jobs to workers, capacities, bottlenecks | max-flow / bipartite matching | polynomial | petgraph `dinics`, `maximum_matching`; networkx; OR-Tools |
| Rank nodes by link structure | PageRank (power iteration) | a few passes over E | petgraph `page_rank`, networkx |
| Choose package versions that satisfy constraints | version solving (SAT-like) | NP-hard in general; fast in practice | PubGrub (`pubgrub` crate; used by uv) |

## Reach-for notes

- **Balanced BSTs vs B-trees:** AVL trees are more rigidly balanced (faster lookups, more rotations); red-black trees rotate less on writes. In memory, a B-tree usually beats both on cache behaviour. Don't implement any of them — use the stdlib's.
- **Representation decides the constants:**
  - An adjacency list (vector of vectors) is the default for sparse graphs.
  - An adjacency matrix suits dense or small graphs where the question is "is there an edge?".
  - CSR (compressed sparse row: one offsets array, one targets array) is compact and scan-fast for static graphs.
  - Nodes as integer indices into `Vec`s beat pointer-linked nodes.
  - petgraph offers `Graph`, `StableGraph` (indices survive removal), `GraphMap`, `MatrixGraph` and `Csr` ([docs](https://docs.rs/petgraph)).
- **Topological sort** doubles as cycle detection: petgraph's `toposort` returns `Err(Cycle)`, and Python's `graphlib.TopologicalSorter` raises `CycleError`. Report the cycle, since users need it to fix the graph. Kahn's algorithm also gives "ready now" batches for parallel scheduling (`graphlib` has `get_ready()`).
- **Union-find** with path compression and union by rank is effectively constant time per operation. Reach for it when connectivity is built up incrementally and never cut.
- **Dijkstra** is wrong with negative weights. Use lazy deletion instead of decrease-key. **A\*** is optimal when the heuristic never overestimates; with a zero heuristic it is just Dijkstra.
- **Persistent structures** (Bagwell's hash array mapped tries; RRB vectors) give cheap snapshots and undo, and share across threads safely. They cost more per operation than a mutable `Vec` or `HashMap`. Copy-on-write B-trees give databases the same property: LMDB and `redb` use them for MVCC snapshots.
- **Hard problems:** graph colouring, TSP, max clique, optimal scheduling and exact version solving are NP-hard. Recognize them and reach for a heuristic, an approximation, or a solver (SAT, MIP, CP — e.g. OR-Tools) instead of searching for a clever exact algorithm.

## Classic failure modes

- **Recursion depth:** recursive DFS on a deep graph (a long dependency chain, a linked list of commits) overflows the stack. Python's default recursion limit is 1,000. Use an explicit stack.
- **Cycles you didn't expect:** traversal without a visited set loops forever, and "it's a tree" assumptions break on real data (symlinks, re-exports, mutual imports).
- **Dense vs sparse mismatch:** a V×V matrix for a million-node sparse graph, or edge lists scanned repeatedly for adjacency checks on a dense one.
- **Negative weights fed to Dijkstra:** silently wrong answers, not an error.
- **Non-deterministic order:** hash-map-backed graphs iterate differently per run. Sort neighbours when output must be stable.
- **Rebuilding the whole graph per query** when it changes rarely. Cache it, and invalidate it on change.

**Depth:** Sedgewick and Wayne ch. 4 and §1.5 (union-find); CLRS part VI; Skiena's catalog: "Graph Problems: Polynomial-time" and "Graph Problems: Hard"; [cp-algorithms.com](https://cp-algorithms.com/) for Fenwick and segment trees; Bagwell, *Ideal Hash Trees* (2001).
