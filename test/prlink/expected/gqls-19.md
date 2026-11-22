## Summary

- Adds a fourth, **adversarial** eval set — [`script/eval/hasura.tsv`](https://github.com/dpep/gqls/blob/2504f78/script/eval/hasura.tsv) — against PokeAPI's public Hasura endpoint (`https://beta.pokeapi.co/graphql/v1beta`), and registers it in [`script/eval.sh`](https://github.com/dpep/gqls/blob/2504f78/script/eval.sh).
- Unlike `holdout.tsv`, it's labelled ADVERSARIAL, not held-out — it targets a known-weak surface (the README's own Hasura/PostGraphile example) on purpose, so it's fair game for tuning.
- Measures and reports what d0639e2 ("Let a description cover a phrase's words") does on this schema, and why.

## The schema, measured before writing any queries

26218 records total. Of the ones that matter for search:

| | count |
|---|---|
| root query fields | 459 (100% share the `pokemon_v2_` prefix) |
| object types | 1991 (1989 share the prefix) |
| records with *any* description | 7062 / 26218 |

The 7062 descriptions are **100% machine-generated Hasura templates** — `"fetch data from the table: \"X\""`, `"An array relationship"`, `"column name"` (685 occurrences) — that echo the record's own name and add zero vocabulary a name doesn't already have. Raw scalar columns (`weight`, `height`, `name`, ...) carry no description at all. So this schema tests both the documented shared-prefix weakness (README: "On a schema where every name starts the same way, name the type.") and d0639e2's description mechanism at the same time, in the place it structurally cannot help.

## The 20 queries

8 O (word overlap — the query reads as the record's name in English word order), 8 V (vocabulary mismatch — a content word has no counterpart in the name), 4 S (suffix-sibling disambiguation — the base table, `_aggregate` and `_by_pk` query fields share one identical stem, so only intent, not words, picks the right sibling). All 20 target paths verified to exist against the live introspection dump before committing.

## Results against the current binary (d0639e2)

```
category     hit@1   hit@5     MRR
O             0.00    0.00    0.00
V             0.00    0.00    0.00
S             0.00    0.00    0.01
overall       0.00    0.00    0.00
```

19 of 20 queries don't match at all inside the top 50; one S query (`how many pokemon are there in total` → the `_aggregate` field) ranks 24th.

## Diffed against a d0639e2^ binary (built via `git archive` into a scratch dir)

Zero movement on any of the 20 hasura queries — identical ranks (mostly "miss", one at 24) on both sides. Confirmed directly: none of these queries' words appear anywhere in this schema's boilerplate description text, so `described_by` never fires for any of them. This is exactly the "little or no movement" the mechanism predicts on a schema without real descriptions — the github set (hand-authored, human prose) still measures MRR 0.43 → 0.47 as documented in d0639e2's own commit message; this new set reconfirms that with an honest zero rather than assuming it.

## The more specific, unexpected finding

The near-zero *baseline* itself is not (only) the prefix collision — it's a plural/singular gap in the word scorer that this schema exposes cleanly. gqls's word match is a DP subsequence aligner with a typo fallback bounded to a length delta of 1–2 chars **against the whole record name** (`typo_distance`). Hasura names root query fields after the singular table (`pokemon_v2_type`, not `pokemon_v2_types`), while a developer naturally asks in the plural ("list of egg groups", "pokemon types"). The plural's trailing character has no letter to land on as a subsequence, and the whole name is far too long (prefixed) for the typo fallback to engage — so most of this set's queries fail to *match at all*, not just rank low. Verified directly: `type` ranks `pokemon_v2_type` 1st of 30 matches; `types` matches 3 records, none of them it. GitHub/AniList schemas don't expose this because their hand-written list fields are already named in the plural (`followers`, `labels`, `topics`).

This is a real, separate ranking gap worth its own follow-up — not something this PR fixes, since I own only the eval harness and this TSV, not `src/`.

## What this set can and cannot tell us

It's a clean, honest confirmation that the description mechanism does nothing on a schema whose descriptions carry no real vocabulary — useful as a permanent regression guard against a future change that assumes "has a description" implies "has *useful* prose." It is not, on its own, a measurement of gqls's general Hasura-schema quality: the near-zero score here is dominated by the plural/singular subsequence gap above, which the phrase-coverage/description work was never meant to fix, so a reader shouldn't conclude "gqls is unusable on Hasura schemas" from this MRR — only that *this specific* ranking change doesn't move the needle here, and that there's a separate, now-documented gap worth a dedicated fix.

## Test plan

- [x] `script/eval.sh --selftest` passes
- [x] [`script/eval.sh`](https://github.com/dpep/gqls/blob/2504f78/script/eval.sh) runs all four sets, hasura fetches and caches correctly, report labels it ADVERSARIAL
- [x] All 20 hasura.tsv target paths verified to exist against the live schema before committing
- [x] [`script/check.sh`](https://github.com/dpep/gqls/blob/d0639e2/script/check.sh) — all green (fmt, clippy, tests)
- [x] Diff confirmed against a `d0639e2^` binary built via `git archive`

🤖 Generated with Claude Code
