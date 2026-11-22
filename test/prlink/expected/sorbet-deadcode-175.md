## What's wrong

`detect_column` ends its `def` pattern with `\b`, which can't match after `?`, `!` or `=`. So `def ok?`, `def save!(x)` and `def name=(v)` fall back to column 0, and the LSP query asks about whatever sits there: Sorbet may return unrelated references (kept as live) or none (kept as dead). [`HybridFinder`](https://github.com/corsonknowles/sorbet-deadcode/blob/a417b2d/lib/sorbet_deadcode/lsp/hybrid_finder.rb#L5) has the same pattern.

## The fix

Replace the trailing `\b` with `(?![\w?!=])` in both finders. `foo` still doesn't match `foo_bar`, `foo?`, `foo!` or `foo=`, and `foo?` now matches itself.

## Tests

New cases in [`dead_code_finder_spec.rb`](https://github.com/dpep/sorbet-deadcode/blob/00510d7/spec/sorbet_deadcode/lsp/dead_code_finder_spec.rb) and [`hybrid_finder_spec.rb`](https://github.com/dpep/sorbet-deadcode/blob/00510d7/spec/sorbet_deadcode/lsp/hybrid_finder_spec.rb) check the column for `?`, `!` and `=` methods; they fail on main. `bundle exec rake` passes: 100% line and branch coverage, RuboCop clean.
