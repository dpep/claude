# Where the Time Goes

Reference for the `performance-engineer` persona. The throughline: **your intuition about why something is slow is wrong often enough that acting on it, rather than measuring, is the main way performance effort gets wasted.** Not occasionally wrong — routinely, and in a patterned way. The patterns are learnable, which is what this document is for.

## 1. The obvious cause is usually wrong

Three diagnoses from one project, each confidently reasoned and each incorrect:

| Symptom | The confident guess | What it actually was |
|---|---|---|
| A pipeline stage pinned at 100% CPU for minutes per file | that stage is the bottleneck | orphaned workers from a killed run competing for the same disk — the same stage did the work in 12 seconds alone |
| Suggestion lookup cost 19.7ms each | the edit-distance algorithm | five heap allocations per candidate, half a million per call; the arithmetic was never the problem |
| Loading a lookup table cost 8ms on every invocation | allocating 102,495 strings from static text | *hashing* 102,495 keys. Borrowing instead of allocating barely moved it; a sorted `Vec` with binary search was 36× cheaper to build |

The pattern: **the expensive thing is usually structural, not algorithmic.** Allocation in a loop, contention, setup repeated per invocation, a data structure chosen for the wrong access pattern. The algorithm is what everyone stares at because it is the interesting part, and it is usually fine.

A corollary worth internalizing: **fixing the wrong thing can look like success.** Adding a fast prefilter to a stage that was starved by contention "worked" in the sense that nothing got worse. It cost real time and taught nothing, because the bottleneck was never there.

## 2. A profile that covers only setup hides the answer

The trap is subtle. Instrument the phases you already suspect — loading, opening, parsing — and the report looks thorough while the hot path sits in the residue between the named phases and the total.

In one case the named phases summed to 18ms of a 53ms run. The remaining 35ms was the actual work, entirely unattributed, and it was where every real finding lived.

**If the named phases don't roughly sum to the total, the gap is the finding.** Add a timer around the work itself before believing any breakdown.

Counters deserve equal weight with timings. *"307,485 candidates scanned to produce 3 suggestions"* diagnoses the problem in a way no duration does.

Pair every timing with what it returned, because **the pass that returns nothing can be the expensive one** and a duration alone will never say so. One query pipeline had a stage costing a flat 5ms on every call that contributed *zero rows* to every query measured. It was invisible in the total and obvious the moment the counter sat next to the timer.

## 3. Measure the slope, not the total

A total conflates fixed and variable cost, and they want different fixes.

Run the workload with 0, 1, 3, and 6 of whatever scales, and read it:

```
0 unknown words: 12.12ms      <- fixed cost, paid every invocation
1 unknown words: 17.62ms
3 unknown words: 29.10ms      <- +5.6ms each, dead linear
6 unknown words: 46.96ms
```

That single table says: there is a 12ms floor worth attacking separately, and a 5.6ms per-unit cost worth attacking differently. Neither is visible in "the run took 47ms".

## 4. The environment is part of the measurement

A number is only comparable to another number taken the same way. Three ways that quietly breaks:

**Contention.** A number taken while something else runs is not a number. The failure that cost the most: killing a script left its `xargs` workers alive, so the next run put two sets of readers on the same files. The symptom was a load average of 19 on eight cores with **every process at 0% CPU** — everything apparently slow, nothing actually progressing. Reap strays, check the load average, re-measure. A stage that "takes minutes" often takes seconds alone.

**Build profile.** A debug build and a release build differ by ~10× on allocation-heavy code, which is most code that matters here. One project compared a debug measurement against a release baseline taken the same morning, concluded a 5× per-candidate regression had shipped, and published that in a changelog. Measured like for like, both release, the two versions were 5-6ms and 7ms — there was no regression at all. **Whenever two numbers disagree, check they came from the same profile before believing the difference.** "Measured today" is not enough; it has to be measured the same way.

**Cache warmth.** The first run touches cold pages and can read 10× high. One per-phase breakdown showed a pass at 50ms that was really 5.5ms warm — and it was not even the pass worth attacking. Run the workload two or three times and read the steady-state number, or you will optimize whichever phase happened to fault first.

## 5. Price the fancy fix before building it

The interesting optimization is frequently a pessimization for the common case, and this is checkable in minutes rather than days.

A trigram index over a 102k-word list would have cut per-lookup cost from 5.6ms to roughly 1ms. But building the index costs tens of milliseconds, and the common case — a single short input with zero or one unknown word — spends less than that in total. The index would have made the typical invocation slower while making a rare one faster.

Two minutes of measurement settled it: *a `Vec` is 36× cheaper to build than a `HashMap` here.* That number decided the design without writing either version.

**Record the rejection.** "Considered, measured, rejected because construction dominates the common case" is worth more than silence, because otherwise the same clever idea returns every few months with the same confidence.

## 6. When the fix changes nothing, suspect the fix

A change that produces no measurable difference has two explanations, and the flattering one — "my hypothesis was wrong" — is the second thing to check, not the first.

**Did it land?** An edit script that asserts partway through can discard its own writes; a build can fail silently in a piped grep. The tell in one case was a stray "unused function" warning next to a number that refused to move: the helper had been added and the call site it was meant for had not. Confirm the change is in the binary you are timing before drawing any conclusion from it.

**Is it where you think?** A cheap guard only saves what it sits above. One "cheap prefilter", added specifically to avoid a per-candidate allocation, was placed *after* the allocation and bought exactly nothing — for two rounds of measurement, because the reasoning was sound and only the position was wrong. **A gate below an allocation is not a gate.**

And when a plan or a benchmark is the evidence, run it on the *real* statement. A query plan taken on a simplified `SELECT id FROM t WHERE ...` said one thing; the actual query, with its joins, planned differently. The simplification was the bug.

## 7. A speedup that changes the output is not a speedup

Every optimization ships with an equivalence check. In practice this means a fixed corpus and a fixed set of metrics run before and after, with the expectation that they are *identical* — not "close", identical, unless the change was supposed to alter behaviour.

The discipline pays twice: it catches the subtle break, and it makes the win reportable. "19.7ms → 5.6ms per call, output identical: recall 72.0%, precision 95.1%" is a claim someone can act on.

## 8. Know the budget, and stop when you're inside it

"Fast enough" is a number. For a hook that runs on every keystroke it is single-digit milliseconds; for a batch job it may be minutes. Without it, optimization has no terminating condition and the work continues until someone gets bored.

Saying "this is already inside budget, here is the measurement, we should stop" is a real deliverable. It is also the one least likely to be volunteered, because it looks like doing nothing.
