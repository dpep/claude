# What Makes a Great Test

Reference for the `test-engineer` persona. The throughline: **a test is worth keeping when it fails for a real regression and stays green through a refactor.** Those are the two halves of its value, and most bad tests fail one of them. A test that runs the code but can't see the change is a false sense of safety; a test that breaks whenever the internals move is a tax on every future change, and teams learn to "fix" it by updating the expectation, which is how it stops catching anything. **Fewer tests that each prove something beat many that each prove a little.**

## 1. The two properties that make a test valuable

Kent Beck's *Test Desiderata* lists twelve properties a test can have. Two of them define its value and the rest are about cost:

- **Behavioral** — the result changes when the code's behaviour changes.
- **Structure-insensitive** — the result stays the same when the code's structure changes and its behaviour doesn't.

Vladimir Khorikov names the same pair as the first two of his *four pillars* of a good test: **protection against regressions** and **resistance to refactoring**, alongside **fast feedback** and **maintainability**. A test's value is roughly the product of the four, so a zero on any one zeroes the test. Khorikov treats resistance to refactoring as the one not to trade away: a test either couples to implementation or it doesn't, and one that does raises false alarms forever. The real trade is between regression protection and speed, which is where test size comes in (§7).

Kent C. Dodds puts the two failure modes in plain terms. A test coupled to implementation details gives a **false negative** when a behaviour-preserving refactor breaks it ("a test failure, but it was because of a broken test, not broken app code") and a **false positive** when it passes while the software is broken. The same coupling usually causes both, because a test that watches the internals isn't watching the behaviour.

**Resistant doesn't mean never edited.** Renaming a public method, or changing what the system is supposed to do, rightly changes its tests. The standard is that a refactor *which leaves the observable contract alone* doesn't. Google's *Software Engineering at Google* says it directly: "the ideal test is unchanging" — after it's written, it changes only when the requirements do.

## 2. Test behaviour through the surface a caller uses

The rule that delivers both properties at once: **exercise the code the way its callers do, and assert on what they can observe.** Google's testing chapter frames it as testing via public APIs, testing state rather than interactions, and writing a test per *behaviour* rather than per method — one method can have several behaviours, and one behaviour can span several methods.

"Public" means the boundary a caller crosses, not every method that happens to be visible. A helper class that exists only to serve one module is that module's implementation detail, even if the language makes it public; test it through the module unless it carries enough logic to deserve its own contract. A behaviour you can't reach from the caller's side is design feedback — either it isn't a behaviour anyone depends on, or the interface is missing something.

Dodds's version, from UI testing, generalizes: code has users — the people who call it and the people who see its output — and a test that pokes internals invents a third user whose needs nobody else shares.

## 3. Would it fail if the change were reverted?

The sharpest question to ask of a test written for a change: **if this change were reverted, would this test fail, and on which assertion?** It turns "is it tested" from a judgement into an experiment. A test exercises the change only when it calls the changed code on an input where the old and new code behave differently, *and* asserts on the part of the result that differs.

The standard earned its place in a review experiment. A general reviewer called a change "covered" whenever a test ran the changed code; only 4 of the 16 tests it named would actually have failed with the change reverted. A reviewer prompted to apply the revert standard named 15 tests, and 12 of them would have failed. The same reviewer also correctly said *no new tests needed* on three pure refactors, where the general reviewer had called them covered. In one real library change, 17 existing examples executed the changed line and none failed on the old code: a genuine gap that coverage reported as fully covered.

Three limits to keep in mind:

- **Failing isn't enough; it has to fail on an assertion.** A new method reverted produces `NoMethodError` or a compile error, which any test that merely calls it will hit. That proves the method exists, not that it behaves. Ask which assertion would catch a *wrong* implementation.
- **A refactor should pass the revert check both ways.** For a change that claims no behaviour change, the existing tests passing on old and new code is the evidence the claim is true. The question there is whether those tests are strong enough to have caught a slip, which is what mutation testing answers (§4).
- **It is scoped to the change.** It says nothing about older, untested behaviour next door. That's a follow-up, not a gap in this change.

The revert check can be done by reading, which is cheap and usually right, or by running the change's tests against head's tests over the base's code, which is definitive and costs about two runs of the affected spec files.

## 4. Mutation testing measures test strength

Coverage says code was *executed*. Mutation testing says whether the tests would *notice* if it were wrong: it makes small deliberate changes (flip a comparison, delete a statement, return a constant) and reports each one no test caught. A surviving mutant is a concrete claim — "this line can be wrong and nothing fails."

The evidence that it measures something real: Just et al. (FSE 2014) found that a suite's mutant detection rate correlates with its detection of real faults, more strongly than statement coverage does, and that about 73% of real faults were coupled to common mutants. Inozemtseva and Holmes used mutants as the yardstick for exactly that reason.

The costs are real too:

- **It's expensive.** Every mutant is a test run. Run it on the diff, not the codebase: `mutant --since main` (Ruby) and `cargo mutants --in-diff` (Rust) select only changed subjects. Google runs it only on changed lines during code review, after dropping lines without coverage and "arid" code (logging, plumbing) where a mutant tells you nothing.
- **Many survivors aren't worth a test.** When Google first surfaced mutants in review, developers rated about 85% of them not useful; filtering and per-line caps were what made the tool worth having. Equivalent mutants (a change that can't alter behaviour) and mutants in code no one cares about are noise.
- **A survivor has two honest answers.** Add the test that pins the behaviour, or accept that the original code was redundant and simplify it. mutant's own docs say this. A third answer — "not worth pinning" — is fine when said out loud with a reason.

Use it when skeptical: a test suite you inherited and don't trust, a refactor that rests on "the tests are green," a bit of logic (money, permissions, date maths, parsers) where a silent slip is expensive. Don't use it to chase a score.

## 5. Coverage is a floor, not a goal

Inozemtseva and Holmes (ICSE 2014, a distinguished paper) found coverage only weakly to moderately correlated with suite effectiveness once suite size is controlled for, and stronger coverage criteria added no insight. Coverage is good at one thing: **finding code no test touches.** Uncovered changed lines are a certain gap. Covered ones are not a certain pass.

Google's 2020 guidance is the sane middle: there is no ideal number; 60% is acceptable, 75% commendable, 90% exemplary; project-wide goals above 90% are probably not worth it — and **per-commit coverage of changed lines is the number to watch**, where a high bar (90–99%) is reasonable. Beck compared pride in 100% coverage to pride in reading every word of the newspaper. Dodds keeps his small published libraries at 100% because breakage hits many consumers and the code is small; applications get diminishing returns long before that.

Make coverage diff-scoped and per-test where possible:

- **Ruby:** SimpleCov (1.2+) can record which example ran each line (`track_tests`), answer `simplecov tests FILE:LINE`, score only a change's lines with `simplecov patch --base REF`, and select affected tests. `undercover --compare origin/main` flags changed methods with no coverage.
- **Rust:** `cargo llvm-cov` produces lcov; `diff-cover` (or a CI service) scopes it to the diff.

Know the blind spots of line coverage: code inside strings (a Lua script in a heredoc, `class_eval` text) maps to the line that defines the string; a changed argument on a multi-line call maps to the call's first line. And coverage + the revert check together give the precise gap: "executed by N tests, none fail on the old code."

## 6. Doubles: real when you can, mock at the edge

Google's order of preference for a collaborator: **real implementation, then a fake, then a stub, then interaction testing (mocks)** as the last resort. Stubs leak implementation details and can't keep state; mocks verify that a call happened, not that the code works, and produce what Google calls change-detector tests.

The classicist/mockist split (Fowler, *Mocks Aren't Stubs*) is the root of this: classicists use real objects where practical and double only the awkward; mockists double every collaborator with interesting behaviour and verify the calls. Fowler, a classicist, notes that mockist tests "are thus more coupled to the implementation of a method." That coupling is the resistance-to-refactoring cost from §1.

Where the line falls, from Khorikov: **communication inside your system is implementation detail; communication with systems outside it is behaviour.** Use real instances of *managed* dependencies (your app's own database, which nothing else reads) and mock only *unmanaged* ones whose interactions are visible to the outside world (an email gateway, a payment API, a message bus). Google agrees from another angle: interaction testing is appropriate for **state-changing** calls with effects outside the system (`send_email`, `charge`), not for queries, whose results you can assert on directly.

And for third-party code, *don't mock what you don't own* (from London-school TDD, Freeman and Pryce; restated by Test Double): wrap the library in an adapter shaped by your needs, mock the adapter in your unit tests, and test the adapter against the real thing.

**Reconciling "don't mock what you own" with "don't mock what you don't own".** They sound opposed and aren't. Don't mock your *internal* collaborators (intra-system, Khorikov); don't mock a *third-party* type directly (Freeman/Pryce). Do mock the thin adapter *you own* at the boundary to an unmanaged system — the one place both rules agree on.

DHH's *test-induced design damage* is the warning from the other side: indirection added only so something can be mocked or unit-tested in isolation makes the code worse. Hard-to-test code is sometimes a design smell and sometimes just integration code that wants an integration test.

## 7. Test size and the shapes are trade-offs

Google separates **size** (resources: small is one process and no I/O or sleeps; medium is localhost; large is anything) from **scope** (how much code is exercised). A broad-scope test can still be small if its out-of-process dependencies are faked. Size drives speed and determinism; scope drives how much one test proves.

The pyramid (many unit, fewer integration, few end-to-end — Google aims for roughly 80/15/5 by count), the trophy (Dodds: "write tests, not too many, mostly integration") and the honeycomb argue past each other mostly over what "unit" means. Fowler's resolution: a *sociable* unit test (real collaborators) is what honeycomb advocates call an integration test, and a *solitary* one (all collaborators doubled) is what they're against. His verdict on the percentages debate: "a distraction." The question that matters is whether each test has a clear boundary, runs fast and reliably, and fails only for useful reasons.

Practical reading: default to sociable tests at the boundary of a module, small enough to be fast and deterministic. Go narrower when a unit has dense logic worth exercising case by case (a parser, a pricing rule). Go wider when the risk lives in the wiring (a controller, a CLI end to end). Keep the slow, wide tests few and focused on paths whose failure you'd page someone for. Avoid the *ice-cream cone* (mostly end-to-end) and the *hourglass* (unit and end-to-end with nothing between).

## 8. Property-based tests pay where examples run out

A property test states something true for all inputs and lets the tool hunt for a counterexample, then shrink it. One property can replace a dozen examples and find the case you didn't think of. It pays most on:

- **round-trips** — parse/print, encode/decode, serialize/deserialize;
- **invariants** — sorted output stays sorted, a balance never goes negative;
- **equivalence to a model or oracle** — the fast path against the slow obvious one, the new implementation against the old (the oracle test for an optimization);
- **metamorphic relations** — when you can't state the answer but can state how it changes (adding a filter never adds results).

John Hughes's *How to Specify It!* compared these strategies by planting bugs in a correct implementation; model-based and metamorphic properties found the most. Tools: `proptest` or `quickcheck` in Rust (commit the `proptest-regressions` files so a found case stays found); `prop_check` or `rantly` in Ruby. Don't reach for it when the domain is a handful of business cases best stated as examples; a property nobody can read is worse than three examples everyone can.

## 9. Flakiness: suspect the new test first

A flaky test fails without a defect, and Google's experience is that as a suite approaches 1% flakiness, its tests begin to lose value — people stop believing red. Luo et al. (FSE 2014), from 201 flaky-test fixes across 51 projects, found the top causes were **async waits, concurrency, and test-order dependency**, and that **78% of flaky tests were flaky from the day they were written.** A new test is the prime suspect; run it several times before trusting it.

The usual sources and fixes (Fowler's *Eradicating Non-Determinism in Tests* covers each):

- **Time** — wall-clock reads, time zones, month ends, DST. Freeze or inject the clock (`travel_to`, Timecop; a clock trait in Rust).
- **Randomness** — unseeded generators, random factory data that sometimes collides. Seed it and print the seed.
- **Order and shared state** — globals, class-level caches, leaked database rows, env vars. Run in random order (RSpec `--order random`, Minitest's default; Rust's parallel test threads surface shared state the same way) and bisect with the seed (`rspec --seed N --bisect`).
- **Concurrency and async** — sleeps instead of waiting on a condition; parallel tests sharing a port, a temp dir, or a Redis. Poll with a timeout; give each test its own resource.
- **Network and external services** — anything not on localhost belongs behind a fake (§6), with an occasional contract test against the real thing.
- **Collection ordering** — hash/set iteration order, unordered SQL results asserted as ordered. Sort, or use an order-insensitive matcher (`match_array`).

Suspect flakiness when a failure doesn't reproduce, passes on rerun, depends on which other tests ran, clusters at certain times, or shows up only in CI. Retrying hides it; quarantine (with a limit on how long) buys time; the fix is removing the nondeterminism.

## 10. Over-testing smells

Each of these spends maintenance on something no regression would ever trip, or trips on changes that aren't regressions:

- **Asserting on incidental output** — the full error message when only the error class matters, the whole hash when one key is the behaviour, exact log lines, the timestamp format of a field nobody parses. Assert on what the change is about.
- **Snapshot sprawl** — a snapshot nobody reads is regenerated, not reviewed. Dodds: large snapshots get "nuked and retaken" on every change. Keep snapshots small and focused on output that *is* the contract (CLI output, generated code, error text); otherwise prefer explicit assertions.
- **One test per method** — mirrors the structure, so a refactor that moves logic between methods breaks tests without breaking anything. Test per behaviour (§2).
- **Mocks of your own internals** — verifies the conversation between your classes, not the result (§6). The tell: the test sets up three doubles and asserts they were called in order.
- **Tests that restate the implementation** — the expected value computed by the same formula as the code, or a test that mirrors each line of the method. It can only fail if the two drift apart in their typing, never because the logic is wrong. Use literal expected values worked out by hand.
- **Duplicate coverage** — the same behaviour asserted at three levels, or five examples of the same equivalence class. Keep the one at the right level and the boundaries of each class.
- **Testing the framework or the language** — that `attr_accessor` works, that `#[derive(Debug)]` prints, that ActiveRecord saves.
- **Tests that exist for the coverage number** — calls without assertions, assertions like `be_truthy` on something that's always truthy.

The cure for most of these is deletion. A deleted test that wasn't catching anything costs nothing; keeping it costs every refactor.

## 11. Test edits that hide behaviour

The other way tests stop proving things is by being edited. In a change that touches tests, ask of every changed test hunk: *does this test still fail if the behaviour it names broke?* The patterns:

- deleted or narrowed examples, or a rewritten one that no longer reaches its path;
- broadened matchers — `eq` to `include` or `be_truthy`, an exact value to `be_a`, `raise_error(Specific, msg)` to bare `raise_error`, `change { }.by(1)` to `change { }`, an exact float to `be_within(1)`;
- `skip`, `pending`, `xit`, `#[ignore]`, or a leftover focus (`fit`, `focus: true`);
- stubs of the code under test (`allow(subject).to receive(:the_changed_method)`), `allow_any_instance_of`, non-verifying `double` where `instance_double` would catch an interface change;
- setup removed or changed so the example passes vacuously — an empty collection, a `nil` that short-circuits, a `let` never referenced where `let!` was meant;
- expectations rewritten to match new output in a change that claims no behaviour change. In the review experiment this was the subtlest real catch: a spec flipped from "does not parse local config" to "does parse" and now expected the very bug the change introduced.

A table-driven rewrite can shrink a test file tenfold and keep every assertion. Read the old and new versions before calling it a weakening.

## 12. The tensions, and how to resolve them

**Granularity vs brittleness.** Fine-grained tests localize failures (Beck's *Specific*) but couple to structure; coarse ones survive refactors but fail vaguely. Resolve by choosing the *boundary*, not the size: test at the narrowest interface that callers actually depend on and that is stable. Go narrower only where the logic is dense enough that a coarse test can't enumerate its cases cheaply. A failing coarse test with a good failure message is specific enough.

**Mocks vs real collaborators.** Doubles buy speed and determinism and cost fidelity and refactor-resistance. Resolve with Khorikov's line: real for anything inside your system or managed by it, a fake where real is too slow, a mock only for outbound effects on systems outside your control, behind an adapter you own.

**Coverage vs value.** Coverage is cheap and measures execution; value is regressions caught. Resolve by using coverage to *find* untested changed lines, the revert check to *prove* a change is caught, and mutation testing to *measure* strength when it's in doubt. Never write a test to move a coverage number.

**Fewer tests vs the Beyoncé rule.** Google's "if you liked it, you should have put a test on it" pushes toward testing everything you depend on. The resolution is that both are true at different times: *for a change*, write the fewest tests that would catch its revert; *for code you're about to refactor and don't trust*, first write characterization tests that pin what it does today (Feathers), then refactor under them. The second is how the refactor earns its "no behaviour change" claim.

**TDD vs resilience.** Test-first at too fine a grain drives mock-heavy, implementation-shaped tests (DHH's design damage). Resolve by driving from the behaviour at the module boundary, and treating the tiny step-by-step tests as scaffolding: once the behaviour test covers them, delete the ones that only pinned an intermediate shape. TDD's lasting value is that every test was seen to fail first, which is the revert check done in advance.

**Readable vs DRY.** Google: tests should be DAMP (descriptive and meaningful phrases), not DRY — "a little bit of duplication is OK in tests so long as that duplication makes the test simpler and clearer." Avoid logic in tests; straight-line code can't hide a bug the way a loop with a conditional can. Table-driven cases are the exception that's both.

## 13. Ruby and Rust notes

**Ruby (RSpec / Minitest)**

- Find tests by convention (`lib/a/b.rb` → `spec/a/b_spec.rb` or `test/a/b_test.rb`) and then by searching for the changed method, constant, or error class; shared examples and helpers can hold the test that matters.
- Prefer verifying doubles (`instance_double`, `class_double`) — a plain `double` keeps passing after the real interface changes. `rubocop-rspec` cops (`RSpec/Focus`, `RSpec/AnyInstance`, `RSpec/VerifiedDoubles`, `RSpec/SubjectStub`) catch the mechanical smells if the repo runs them; don't repeat what a configured linter already reports.
- Coverage: SimpleCov, with `track_tests` on 1.2+ for per-example mapping (an untracked `.simplecov` file can enable it without a repo edit); `undercover` for changed-but-uncovered methods.
- Mutation: `mutant run --use rspec --since origin/main 'Namespace*'` (or `mutant-minitest`). Free for open source (`--usage opensource`); commercial use needs a paid licence, so check before proposing it on a private repo.
- Flakiness: `--order random`, `--seed`, `--bisect`, `--only-failures`; freeze time with `travel_to` or Timecop; Prosopite or a query counter for N+1 regressions belongs to the performance side.

**Rust**

- Unit tests in `#[cfg(test)] mod tests` beside the code; integration tests in `tests/` can reach only the public API — the natural home for behaviour tests.
- Coverage: `cargo llvm-cov` (lcov / html / `--show-missing-lines`); scope to the diff with `diff-cover` or CI.
- Mutation: `cargo mutants --in-diff <(git diff origin/main)` on a change; outcomes are *caught*, *missed* (the finding), *timeout*, *unviable* (didn't compile). Exclude arid code with `--exclude-re` or `#[mutants::skip]` and a reason.
- Flakiness: tests run in parallel threads by default; `-- --test-threads=1` to test for shared-state flakiness; `cargo nextest` reports retried passes as flaky instead of hiding them.
- `proptest` for round-trips and invariants; `insta` snapshots kept small and reviewed (`cargo insta review`), `trycmd` / `assert_cmd` for CLI contracts.

## Sources

- Kent Beck, [Test Desiderata](https://testdesiderata.com/); Beck's 2008 Stack Overflow answer, "I get paid for code that works, not for tests" ([context](https://visibletrap.blogspot.com/2012/07/i-get-paid-for-code-that-works-not-for.html)).
- Titus Winters, Tom Manshreck, Hyrum Wright (eds.), *Software Engineering at Google*: [ch. 11 Testing Overview](https://abseil.io/resources/swe-book/html/ch11.html), [ch. 12 Unit Testing](https://abseil.io/resources/swe-book/html/ch12.html), [ch. 13 Test Doubles](https://abseil.io/resources/swe-book/html/ch13.html).
- Google Testing Blog: [Code Coverage Best Practices](https://testing.googleblog.com/2020/08/code-coverage-best-practices.html) (2020); [Change-Detector Tests Considered Harmful](https://testing.googleblog.com/2015/01/testing-on-toilet-change-detector-tests.html); [Prefer Testing Public APIs Over Implementation-Detail Classes](https://testing.googleblog.com/2015/01/testing-on-toilet-prefer-testing-public.html).
- Kent C. Dodds: [Testing Implementation Details](https://kentcdodds.com/blog/testing-implementation-details); [Write tests. Not too many. Mostly integration.](https://kentcdodds.com/blog/write-tests); [Effective Snapshot Testing](https://kentcdodds.com/blog/effective-snapshot-testing).
- Martin Fowler: [Mocks Aren't Stubs](https://martinfowler.com/articles/mocksArentStubs.html); [On the Diverse and Fantastical Shapes of Testing](https://martinfowler.com/articles/2021-test-shapes.html); [Eradicating Non-Determinism in Tests](https://martinfowler.com/articles/nonDeterminism.html).
- Vladimir Khorikov, *Unit Testing: Principles, Practices, and Patterns* (Manning, 2020), ch. 4 (the four pillars; [summary](https://dzx.fr/blog/unit-testing-principles-practices-patterns/)); [When to Mock](https://enterprisecraftsmanship.com/posts/when-to-mock/).
- David Heinemeier Hansson, [Test-induced design damage](https://dhh.dk/2014/test-induced-design-damage.html).
- Test Double, [Don't mock what you don't own](https://github.com/testdouble/contributing-tests/wiki/Don't-mock-what-you-don't-own); Steve Freeman and Nat Pryce, *Growing Object-Oriented Software, Guided by Tests* (2009).
- Michael Feathers, *Working Effectively with Legacy Code* (2004), on characterization tests.
- Laura Inozemtseva and Reid Holmes, [Coverage Is Not Strongly Correlated with Test Suite Effectiveness](https://cs.ubc.ca/~rtholmes/papers/icse_2014_inozemtseva.pdf), ICSE 2014.
- René Just et al., [Are Mutants a Valid Substitute for Real Faults in Software Testing?](https://homes.cs.washington.edu/~rjust/publ/JustJIEHF2014b-abstract.html), FSE 2014.
- Goran Petrović and Marko Ivanković, [State of Mutation Testing at Google](https://research.google/pubs/state-of-mutation-testing-at-google/), ICSE-SEIP 2018; Petrović, Ivanković, Fraser, Just, [Practical Mutation Testing at Scale: A view from Google](https://arxiv.org/abs/2102.11378), TSE 2021.
- Qingzhou Luo, Farah Hariri, Lamyaa Eloussi, Darko Marinov, [An Empirical Analysis of Flaky Tests](https://huang.isis.vanderbilt.edu/cs8395/paper/flakytest.pdf), FSE 2014.
- John Hughes, [How to Specify It!](https://research.chalmers.se/publication/517894), 2020.
- Tools: [mutant](https://github.com/mbj/mutant) ([incremental](https://github.com/mbj/mutant/blob/main/docs/incremental.md)); [cargo-mutants](https://mutants.rs/) ([`--in-diff`](https://mutants.rs/in-diff.html)); [SimpleCov](https://github.com/simplecov-ruby/simplecov); [cargo-llvm-cov](https://github.com/taiki-e/cargo-llvm-cov).
