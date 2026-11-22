## What's wrong

[`Client#start_server`](https://github.com/corsonknowles/sorbet-deadcode/blob/a417b2d/lib/sorbet_deadcode/lsp/client.rb#L93) stores the third value from [`Open3.popen3`](https://docs.ruby-lang.org/en/3.4/Open3.html#method-c-popen3) as `@stderr_thread`, but it's the stderr IO, not a thread. `shutdown` then calls `join` on it, so every `--lsp` / `--hybrid` run raises [`NoMethodError`](https://docs.ruby-lang.org/en/3.4/NoMethodError.html) after validating and loses its results.

## The fix

Read stderr in a thread and store that thread as `@stderr_thread`. Reading stderr also stops a server that logs a lot from filling the pipe and stalling.

## Tests

[`test_initialize_spawns_server_and_handshakes`](https://github.com/corsonknowles/sorbet-deadcode/blob/a417b2d/spec/sorbet_deadcode/lsp/client_spec.rb#L27) now passes a real stderr pipe and calls `shutdown`; it raises [`NoMethodError`](https://docs.ruby-lang.org/en/3.4/NoMethodError.html) on main. `bundle exec rake` passes: 100% line and branch coverage, RuboCop clean.
