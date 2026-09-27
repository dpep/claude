# Machine Learning (a practitioner's reach-for map)

Reference for the `algorithmist` persona. ML is a tool for problems where the rule can't be written down but examples can be collected. It is not a default. Start with the rule, the heuristic, or the search, and measure it. Reach for a model when that baseline plateaus *and* you have labelled data and an evaluation you trust.

## When you see this → reach for

| Problem shape | Reach for | Trade-off | Use |
|---|---|---|---|
| You can write the rule down | the rule, a heuristic, or search | brittle at the edges, but explainable | code; see the other maps |
| Rank text documents by a keyword query | BM25 | lexical only — no synonyms | Lucene, `tantivy`, SQLite FTS5 `bm25()` |
| "Find things that mean something similar" | embeddings + nearest-neighbour search | model cost; opaque failures | an embedding model + a vector index |
| Nearest vectors, up to ~100k–1M | exact (brute-force) search | O(n·d) per query, perfect recall | NumPy/BLAS, pgvector without an index, `sqlite-vec` |
| Nearest vectors at larger scale | ANN: HNSW (graph), IVF (clusters), PQ (compression) | recall < 100%; memory and build time | faiss, hnswlib, usearch; pgvector `hnsw`/`ivfflat` indexes |
| Predict a number or class from tabular features | linear or logistic regression first, then gradient-boosted trees | GBTs need tuning; less explainable | scikit-learn, XGBoost, LightGBM |
| Group similar items without labels | k-means (pick k), or HDBSCAN (density) | k-means assumes round clusters | scikit-learn |
| Rank results by several signals | a hand-tuned additive scorer first; learning-to-rank once you have judgments | learned rankers need labelled data and drift monitoring | LambdaMART (XGBoost/LightGBM rankers) |
| Fuzzy, open-ended language tasks (summarize, classify free text, extract) | an LLM call | cost, latency, non-determinism | a model API; cache and evaluate |
| Unsure which estimator fits | scikit-learn's estimator map | — | [choosing the right estimator](https://scikit-learn.org/stable/machine_learning_map.html) |

## Reach-for notes

- **Baseline first, always.** Ship the rule or BM25 and measure it. A model must beat it on the same evaluation, by enough to pay for training, serving, monitoring and explaining it.
- **Vector search:** start exact. Brute force over a few hundred thousand vectors is often inside budget and has perfect recall. pgvector does exact search by default and approximate search once you add an HNSW or IVFFlat index; its docs warn that results change when you do ([README](https://github.com/pgvector/pgvector)). HNSW (Malkov and Yashunin) has the best recall/latency trade-off in memory. IVF with product quantization (Jégou et al.) trades recall for memory at very large n. Measure recall@k against exact search on your own data. ann-benchmarks is no longer maintained, and its README points to newer benchmarks.
- **Embeddings** fix the question the index answers. Changing the model means re-embedding everything, so version the model alongside the vectors.
- **Hybrid retrieval** — BM25 plus vector, fused — usually beats either alone for search over code and docs. Evaluate before assuming it.
- **Additive, explainable scoring** (a sum of named features) is the right call while you have few judgments and need to debug rankings. Learning-to-rank earns its place once you have thousands of labelled query-result pairs and a harness that measures ranking quality offline.
- **Evaluation:**
  - Hold out a test set before looking at results.
  - Guard against leakage: time-based splits for anything temporal, no duplicates across splits.
  - Choose metrics that match the job — precision and recall, recall@k, MRR or NDCG for ranking.
  - Keep a fixed query set with known answers (a recall harness) so every change is measured the same way.
  - Offline metrics are proxies; check them against real outcomes.
- **LLMs as a component:** use one when the input is open-ended language and a small error rate is tolerable or checked downstream. Otherwise prefer the algorithm: cheaper, faster, deterministic and testable. Cache responses, pin model versions, validate structured output, and evaluate on a fixed set like any other model.

## Classic failure modes

- ML where a rule would do; no baseline to compare against.
- Evaluating on training data, or leaking the future into the past.
- ANN recall never measured; results quietly missing.
- An embedding model swapped without re-embedding the corpus.
- A learned ranker nobody can explain when it regresses.
- LLM output trusted without validation or a regression set.

**Depth:** Manning, Raghavan and Schütze, [*Introduction to Information Retrieval*](https://nlp.stanford.edu/IR-book/) (free online); the [scikit-learn user guide](https://scikit-learn.org/stable/user_guide.html); Malkov and Yashunin, HNSW ([arXiv:1603.09320](https://arxiv.org/abs/1603.09320)); CLRS 4th ed. ch. "Machine-Learning Algorithms".
