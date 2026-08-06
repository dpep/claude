# personas

A home for reusable agent **personas** — standalone personalities you can summon in any project. Each is a self-contained system prompt (its own frameworks, posture, and output format). Install once, use everywhere.

Personas are Claude Code subagents: invoke them by intent ("analyze this dynamic," "where should this file go," "pressure-test this plan") or by name via the Agent tool. They run in their own context and report back.

Two families live here:

- **Craft & analysis personas** — deep expertise or a specific analytical lens you summon to do or examine work (`analyst`, `librarian`, `rubyist`).
- **The decision council** — a set of role-based viewpoints you convene to pressure-test a decision, plus a `moderator` who chairs them.

Most personas work in **two modes**: *critique* (examine a plan/design/codebase from this viewpoint) and *do* (produce the artifact or write the code in this voice). The `analyst` is the deliberate exception — descriptive and read-only by design.

## Craft & analysis

- **`analyst`** — descriptive analysis of a conversation, interaction, or body of text when you want to understand what's going on underneath: group dynamics, communication patterns, role-induction, boundary state, the gap between what's said and what's happening. Applies SAVI, SCT phase/subphase indicators, a Shannon-derived noise lens (signal-to-noise, per participant and overall), and language/rhetoric markers; produces a structured, **read-only** report. Trigger on intent ("analyze this," "what's really going on"). The frameworks it applies are summarised inline; the source literature is cited for anyone who wants the full grids.
- **`librarian`** — understand and organize a body of information: files, notes, docs, datasets, a codebase's layout. Builds and critiques taxonomies, says where something should live and where you'd look for it, identifies what should co-locate and what should split. Surveys and proposes a plan first; only moves or edits on explicit approval. Trigger on intent ("organize this," "where should this go," "what's the right taxonomy").
- **`rubyist`** — a senior Ruby/Rails expert with taste. Writes idiomatic, expressive Ruby and reviews it for idiom, clarity, and smells (nested conditionals, N+1, the wrong abstraction, premature service objects, over-metaprogramming). Knows the Rails Doctrine and its pitfalls, Sandi Metz, and RSpec discipline. Reviews Ruby *or* writes it. Trigger on intent ("write this in Ruby," "make this more idiomatic," "is this the Rails way").
- **`scribe`** — a sharp technical writer and editor. Drafts notes, docs, JIRA tickets, PR descriptions, commit messages, and messages — or fine-combs an existing draft, tightening without flattening the author's voice. Writes concise, direct, semi-casual prose with apt analogies and the occasional dry bit of levity (BLUF, omit-needless-words, curse-of-knowledge). Drafts from scratch *or* wordsmiths. Trigger on intent ("write this up," "draft a PR description / ticket," "tighten this," "make this read better").

## The decision council

Role-based viewpoints you convene — together or singly — to pressure-test a plan, design, or idea. Each is a **legitimate optimization function** that could exist inside a real startup: its own mission, incentives, mental models, heuristics, common objections, and failure modes (including its own). Summon several for a decision and read where they pull against each other — *the tension is the value*. Each can also do the work in its voice, not only critique.

- **`product-manager`** — "are we building something people want?" Reframes around the customer's job-to-be-done, names the riskiest assumption and the cheapest test, distinguishes outcome from output. (JTBD, the four product risks, discovery, MVP-as-experiment.)
- **`hacker`** — "what's the shortest path to production?" Maximizes learning per unit time by getting something real in front of usage fast: hardcode, stub, defer, find the creative shortcut — taking debt as a *tracked* loan on *reversible* decisions, and refusing to cut corners on auth, payments, data loss, or anything irreversible. Make-it-happen, tinkerer. (Two-way doors, YAGNI, spike-and-stabilize.)
- **`staff-engineer`** — "is there a simpler model that ages well?" Guards conceptual integrity and long-term velocity: the core abstraction, the deep module, the leaking assumption, the special case to define out of existence — and distrusts its own urge to over-build. (Brooks, Ousterhout, Metz's wrong-abstraction cost.)
- **`production-engineer`** — "will it survive reality?" SRE operability: how it fails, blast radius, observability, safe rollback, expand-contract migrations, tested recovery, cost at scale — right-sized, not gold-plated. (SLOs/error budgets, observability vs monitoring, design-for-failure.)
- **`platform-expert`** — "is the contract sound for the people who build on it?" Guards the *external* interface — DX, consistency, backwards-compatibility, useful errors, docs-as-interface, transparency. Additive evolution, honest versioning, "never surprise." (Hyrum's Law, Problem Details, Stripe/Bloch; see [`references/great-public-api.md`](./references/great-public-api.md).)
- **`skeptic`** — "how do we know?" Protects the team from self-deception: load-bearing assumptions, steelman before rebut, premortem, risk ranked by likelihood × impact — productively, always proposing the cheap test and saying when the evidence is enough. Not a blocker. (Popper, Klein's premortem, Feynman.)
- **`moderator`** — chairs the council. Part tech lead, part manager, optimizing for **decision quality per unit time**: frames the decision, sequences who speaks (anti-anchoring), names conflicts, forces implicit tradeoffs into the open, and produces a recommendation with an owner and a reversal trigger. Right-sizes the process — no full council for a two-way door. Trigger on intent ("facilitate this," "we can't agree," "make the call").

## Adding a persona

Drop a `<name>.md` under `agents/` with frontmatter (`name`, `description`, optional `tools`, `model`) and a system-prompt body — Claude Code auto-discovers it. Omit `tools` to let the persona both critique and *do* (it inherits the full toolset); restrict `tools` only when read-only is the point (see `analyst`). Keep each persona **self-contained** (soft "if installed" pointers to another plugin's references are fine). Add an entry above when you do.

## Install

```
/plugin marketplace add dpep/claude
/plugin install personas@dpep
```
