# Local evidence in Ruby

Use these when you can reproduce the behavior locally. Production telemetry
tells you *that* and *where*; a local repro lets you ask *why*, line by line,
without a deploy. Reproduce first, ideally as a spec, so the evidence can be
rerun.

## singed: a flamegraph scoped to exactly the code in question

[singed](https://github.com/rubyatscale/singed) wraps stackprof (the default),
vernier, or rbspy and writes speedscope JSON. Its value is **scoping**: a
profile of just the slow path, not the whole process.

```ruby
flamegraph("checkout", open: false) { Checkout.new(cart).submit }   # one block
it "submits", flamegraph: true do … end                              # require "singed/rspec"
```
```
curl -H 'X-Singed: true' localhost:3000/orders/1     # one request, via the middleware
bundle exec singed -- bin/rails runner script.rb     # a whole command, via rbspy
```

- **Pass `open: false`.** Auto-open needs a browser on the same host. Read the
  JSON under `Singed.output_directory` (`tmp/speedscope` in Rails), rank frames
  by self time, and report the top few with their share of the total. The user
  can open the file in speedscope to see for themselves.
- **Use `profiler: :vernier`** for threaded code (Puma, Sidekiq) or when the
  wait might be the GVL. It shows `(waiting for GVL)` and idle time, which
  stackprof can't.
- **The rspec tag** turns a slow path into a test, which is what makes a
  before/after comparison honest.

## TracePoint: what actually runs

TracePoint answers questions a profile can't: does this path run at all, who
calls this, and what raised and got swallowed.

```ruby
# every exception raised inside the block, including rescued ones
TracePoint.new(:raise) { |tp| warn "#{tp.raised_exception.class} at #{tp.path}:#{tp.lineno}" }
          .enable { suspect_code }

# who calls one method (target: limits it to that method only, so it stays cheap)
TracePoint.new(:call) { |tp| warn caller(2, 3).join(" < ") }
          .enable(target: Order.instance_method(:recalculate!)) { suspect_code }
```

- **Always scope with `enable { }` or `target:`.** A global `:call` trace slows
  everything by orders of magnitude, so any timing taken under it is
  meaningless.
- **`:raise` finds a `rescue => e; nil`** hiding a failure, which telemetry
  often never sees.
- **Count, don't print.** On hot paths, tally into a hash and report the
  counts.

## debug gem: inspecting state without an interactive session

`rdbg` is interactive, and an agent can't drive a REPL. Two non-interactive
modes do work:

```ruby
binding.break(do: "info locals")          # print locals here and keep running
binding.break(pre: "bt 5")                # print a short backtrace, then stop
```
```
rdbg -e 'break Order#recalculate! do: info locals' -e c -- bin/rails runner script.rb
```

`do:` turns a breakpoint into a temporary log line: state at a point, without
editing logging into the code. Reach for it once singed or TracePoint has
narrowed things to a spot. It's not a search tool.

## Before you report

Local timings are evidence about local conditions: a dev database, a warm
cache, a single user. Use local runs to explain a mechanism, and production
telemetry to say how much it matters. Remove `binding.break`, TracePoints, and
`flamegraph` blocks before committing: `rg "binding\.break|TracePoint\.new|flamegraph" app lib`.
