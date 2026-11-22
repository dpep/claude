## What's wrong

`--history` and `--dead-since` match a candidate's name as a substring.

- `added:` ([`History#introductions`](https://github.com/corsonknowles/sorbet-deadcode/blob/a417b2d/lib/sorbet_deadcode/git/history.rb#L112)) takes the first added line that `include?`s the name, so `name` is reported as introduced by `def display_name`, and `ready` by `def ready?`.
- `dead_since:` ([`History#pickaxe_commit_lines`](https://github.com/corsonknowles/sorbet-deadcode/blob/a417b2d/lib/sorbet_deadcode/git/history.rb#L98)) runs `git log -S name`, which counts substrings. A commit that adds a call to `lonely_method_helper` turns `lonely_method` from "dead-on-arrival" into "dead since <that commit>".

## The fix

Match the name as a whole Ruby identifier: no word character before it, and no word character or `?`/`!`/`=` after it, since `foo?`, `foo!` and `foo=` are different methods.

- `dead_since:` uses `git log -S <pattern> --pickaxe-regex` ([`pickaxe_pattern`](https://github.com/dpep/sorbet-deadcode/blob/e0474ec/lib/sorbet_deadcode/git/history.rb#L111)). That keeps `-S`'s "reference count changed" meaning; `-G` would also match a commit that only moves or reformats a caller. git's regex is POSIX ERE, with no lookaround or portable `\b`, so the boundaries are bracket expressions.
- `added:` uses the Ruby equivalent ([`identifier_pattern`](https://github.com/dpep/sorbet-deadcode/blob/e0474ec/lib/sorbet_deadcode/git/history.rb#L116)), and scrubs each line first, since `match?` raises on invalid UTF-8 where `include?` didn't.

One tradeoff: the ERE pattern consumes the character after the name, so adjacent occurrences like `foo(foo)` count once. That seemed worth a portable pattern.

## Tests

New cases in [`history_spec.rb`](https://github.com/dpep/sorbet-deadcode/blob/e0474ec/spec/sorbet_deadcode/git/history_spec.rb) for longer identifiers and `?` suffixes (fail on main), literal matching of `ready?`, and invalid UTF-8. `bundle exec rake` passes: 100% line and branch coverage, RuboCop clean.
