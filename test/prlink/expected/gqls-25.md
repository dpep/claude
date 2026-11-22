A docs pass before 0.27.0. Seven ranking changes have merged since 0.26.1, each editing the docs in isolation, and the central claim had quietly gone half-false.

## What was wrong

README.md and [`claude/gqls-skill.md`](https://github.com/dpep/gqls/blob/71402aa/claude/gqls-skill.md) — and the skill's frontmatter `description:`, which is what makes an agent reach for the tool at all — all said fuzzy matching **"bridges spelling, not vocabulary"**, and that a synonym "needs you to try the other word — gqls won't". Since 0.26.1 that stopped being true in three ways: a phrase's words are looked for in a record's description (#17), a word matching nothing is retried as its singular (#21), and question words are dropped before scoring (#20). Generated plumbing also now ranks below what it wraps (#22, #24), which the Hasura paragraph still described the old way.

## What it says now

The user-facing docs stay **high level** — no taxonomy of which rule bridges what, since that reads as a spec and rots on the same schedule the old claim did. Both files say the two things that change what someone does:

- A phrase in your own words works. Describe the thing; you don't have to trim a question down to keywords first.
- gqls can't invent a word the schema never writes down. Search a word you're sure of.

[`CHANGELOG.md`](https://github.com/dpep/gqls/blob/71402aa/CHANGELOG.md) is the exception and keeps every mechanism and measured number, because that's who reads it. Its eight `## Unreleased` entries fold into two — one for the phrase rules, one for the demotion — with each metric stated once. The `## Unreleased` heading is untouched for the release script.

## Verified

Every behavioural sentence kept or written was checked against `cargo build --release`, not assumed. Notable: two examples from the original brief did **not** hold and are not in the docs — `number of forks` does not reach `Repository.forkCount` (not in top 50), and `current user` reaches `Query.viewer` at rank 8, not at the top. [`script/eval.sh`](https://github.com/dpep/gqls/blob/5321a23/script/eval.sh) agrees. Details in the thread below.

[`script/check.sh`](https://github.com/dpep/gqls/blob/5321a23/script/check.sh): all green.
