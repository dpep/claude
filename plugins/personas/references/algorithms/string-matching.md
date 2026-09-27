# String Matching

Reference for the `algorithmist` persona. "Find strings like this" is a dozen different questions — exact, any of many patterns, prefix, regex, substring-in-a-corpus, within k edits, abbreviation, what changed — and each has a structure that answers it and several that only look like they do. Pin down what the consumer accepts first.

## When you see this → reach for

| Problem shape | Reach for | Trade-off | Use |
|---|---|---|---|
| Does this needle occur in this text? | SIMD substring search | O(n + m) worst case | `memchr::memmem` (Rust), libc `memmem` |
| Which of many fixed patterns occur? | Aho-Corasick | one pass, whatever the pattern count | `aho-corasick` (Rust), Hyperscan |
| Regex over untrusted pattern or input | automaton engine | no backreferences or look-around | Rust `regex`, RE2, Go `regexp` |
| Need backreferences or look-around | backtracking engine, trusted input only | exponential worst case (ReDoS) | PCRE2, `fancy-regex` |
| Keys with a prefix, or in a range | trie / FST / sorted range | FSTs are immutable | `fst`, or a B-tree range |
| Rows containing a substring, many rows | trigram index + verify | index can outweigh the text; queries under 3 chars get no help | PostgreSQL `pg_trgm`, SQLite FTS5 `trigram` |
| Many substring queries over one big static text | suffix array / FM-index | memory, or compression complexity | libdivsufsort, sdsl-lite |
| Distance between two strings | Levenshtein / Damerau, bit-parallel | O(⌈m/w⌉·n) | `strsim`, `triple_accel`, RapidFuzz |
| Dictionary words within k edits | Levenshtein automaton over a trie or FST | best for k ≤ 2 | Lucene `FuzzyQuery`, `levenshtein_automata`, `fst` |
| Spelling suggestions, huge dictionary, small k | SymSpell (symmetric delete) | large precomputed table | SymSpell and its ports |
| Near matches in a metric space, larger keys | BK-tree | needs a true metric; weak pruning on short keys | several small crates; measure |
| Any approximate query at scale | filter then verify | the filter must be a superset | q-grams, pigeonhole, length and count gates |
| Type-a-few-letters picker | subsequence scoring (Smith-Waterman-style) | O(n·m) per candidate | fzf, `nucleo`, `fuzzy-matcher` |
| What changed between two versions? | Myers diff; patience or histogram for readable code diffs | O((N+M)·D) | `git diff --diff-algorithm`, `similar`, `imara-diff` |
| Combine two edits of one base | three-way merge (diff3) | conflicts need a human | `git merge-file`, diff3 |

## Reach-for notes

- **Exact search:** KMP, Boyer-Moore, Horspool and Two-Way are the classics. Production searchers pair a SIMD prefilter with Two-Way for the worst case. `memmem` guarantees linear time and constant space ([docs](https://docs.rs/memchr)).
- **Aho-Corasick:** choose leftmost-first semantics to match what a regex alternation would do. The Rust crate adds vectorized searchers for small pattern sets ([docs](https://docs.rs/aho-corasick)).
- **Regex:** Rust `regex` guarantees O(m·n) per search, though iterating over all matches isn't covered by that bound ([docs](https://docs.rs/regex)). RE2 guarantees time linear in the input ([README](https://github.com/google/re2)). Compile once, outside loops.
- **FSTs** compress sorted keys by sharing prefixes and suffixes, and run any automaton over them ([`fst`](https://docs.rs/fst)). **They're bad at "contains"**: when a match can start anywhere in a key, no prefix can be pruned, and the walk visits every key.
- **Trigram indexes:** `pg_trgm` with GIN or GiST accelerates unanchored `LIKE`, `ILIKE` and regex ([docs](https://www.postgresql.org/docs/current/pgtrgm.html)). FTS5 `trigram` handles substring `MATCH`, `LIKE` and `GLOB` ([docs](https://www.sqlite.org/fts5.html)). Always verify candidates. Cox, [Regular Expression Matching with a Trigram Index](https://swtch.com/~rsc/regexp/regexp4.html).
- **Edit distances:** Levenshtein; Damerau (adds transpositions); OSA ("restricted" Damerau), which is *not* a metric; Hamming. Jaro-Winkler and Dice are similarity scores for short strings like names.
- **Faster exact distance:** Ukkonen's cut-off (O(kn) expected when you only need "≤ k"); bit-parallel bitap (Baeza-Yates and Gonnet 1992; Wu and Manber 1992, the basis of `agrep`); Myers' bit-vector algorithm (JACM 1999); Hyyrö's extensions for Damerau distance and threshold tests (2003).
- **Levenshtein automata** (Schulz and Mihov 2002): a DFA for "within k of W", built in time linear in |W|. Walked with a trie, it visits only viable prefixes. Lucene caps `FuzzyQuery` at 2 edits ([docs](https://lucene.apache.org/core/9_0_0/core/org/apache/lucene/search/FuzzyQuery.html)). `fst`'s own `Levenshtein` is documented as experimental and slow to build.
- **BK-trees** (Burkhard and Keller 1973) prune with the triangle inequality. At radius 2 on short keys most of the tree is in range, and the search nears a scan.
- **SymSpell** (Wolf Garbe) precomputes deletions: fast lookups, and roughly C(L, k) stored entries per word ([README](https://github.com/wolfgarbe/SymSpell)).
- **Filter then verify:** the q-gram lemma (Ukkonen 1992): within k edits, strings share at least `max(|x|,|y|) − q + 1 − kq` q-grams. Pigeonhole: split into k+1 pieces, and one must match exactly. Length and character-count gates. **The filter must be a superset**: check it against the real verifier on a large, realistic query set, requiring zero disagreements.
  - *In practice:* the `rq` code navigator applies this to fuzzy symbol lookup, with a per-name signature derived from its matcher's rules and verified by the real scorer ([NAME_INDEX.md](https://github.com/dpep/rq/blob/main/docs/NAME_INDEX.md), decisions D21 and D23 in [DECISIONS.md](https://github.com/dpep/rq/blob/main/docs/DECISIONS.md)).
- **Fuzzy pickers:** fzf v1 is a greedy O(n) scan; v2 is a modified Smith-Waterman alignment with boundary, camelCase and consecutive-match bonuses and a gap penalty ([algo.go](https://github.com/junegunn/fzf/blob/master/src/algo/algo.go)). `nucleo` uses fzf's scoring and is faster ([README](https://github.com/helix-editor/nucleo)). Reuse a tuned scorer rather than inventing weights. Pre-check the subsequence cheaply before aligning.
- **Diff:** Myers' O(ND) algorithm (Algorithmica 1986) is git's default. `patience` and `histogram` anchor on lines that occur rarely in both inputs, which often reads better for code ([git-diff docs](https://git-scm.com/docs/git-diff)). Diffing is an LCS problem, so diff *lines* or tokens, not characters, for anything large.

## Classic failure modes

- Answering a neighbouring question: a substring index for a subsequence matcher, edit distance for abbreviations.
- A backtracking regex on untrusted input.
- Allocating per comparison in a distance function called millions of times.
- Unicode: byte-level edits miscount multi-byte characters, and case folding can change length. Keep filter and verifier on the same unit.
- A filter validated only on long queries; short queries are where q-gram and length filters are weakest.

**Depth:** Navarro, *A Guided Tour to Approximate String Matching* (ACM Computing Surveys, 2001); Gusfield, *Algorithms on Strings, Trees, and Sequences*; Sedgewick and Wayne ch. 5; Skiena's catalog: "String Matching", "Approximate String Matching", "Longest Common Substring/Subsequence".
