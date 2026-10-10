---
name: observability
description: Use when digging through production telemetry for evidence — traces, logs, metrics, continuous profiles, Datadog — or when that evidence doesn't exist yet and you need to say what instrumentation would produce it. Triggers on "observability", "o11y", "why is X slow/failing in prod", "dig into Datadog", "find the trace/logs for", "read this flamegraph", "is this happening in prod", "how would we know if", "add logging/tracing/metrics for", "what should we instrument". Keeps a private notes doc of services, dashboards, saved queries and gotchas, and adds to it as it learns.
---

# Observability: finding evidence in telemetry, or building the scaffolding that produces it

## Overview

Telemetry answers questions you can phrase. Most wasted investigation time goes
to browsing dashboards and hoping something jumps out. Instead, start from a
hypothesis, work out what it would leave behind, and go check. If the evidence
can't exist because nothing records it, the finding is the instrumentation that
*would* record it. That's progress, not a dead end.

## Before you start: read the notes

Org-specific knowledge lives in a private notes doc,
**`~/.claude/docs/observability.md`**, not in this skill: service names,
dashboard links, saved queries, tag conventions, and gotchas learned the hard
way. Read it first. It often turns a twenty-minute hunt into one reusable query.

If it doesn't exist, create it from [references/notes-template.md](references/notes-template.md).
[Keeping the notes](#keeping-the-notes) covers how to keep it useful.

## The loop

0. **Ask what changed.** Deploys, flag flips, config, traffic mix, upstream
   incidents in the window. It's the cheapest query there is, and it explains
   most production surprises before any theory does.
1. **State the hypothesis as a claim that can be false.** "Checkout p99
   regressed because the fraud call got slower after Tuesday's deploy," not
   "something's off with checkout."
2. **Say what it would leave behind:** the signal, where it lives, and its
   shape. For the example above: `fraud.check` span duration steps up at the
   deploy's `version` tag, its share of checkout's total time rises, and other
   downstream spans stay flat.
3. **Look, and look for the opposite.** Check the counter-hypothesis in the
   same pass. If fraud is flat and the DB spans moved, you've learned more
   than a confirmation would have told you.
4. **Narrow.** Split by the dimension that separates good from bad: version,
   host, endpoint, customer tier, region, payload size. Compare a slow trace
   with a fast one for the same endpoint before reading either closely.
5. **If the evidence can't exist, design the scaffolding.** Name the exact
   span, tag, log field, or metric, and the question it settles. See
   [Scaffolding](#scaffolding).

Report each claim with what backs it: a query, a trace link, a count, a time
window. "p99 went 210ms → 480ms at 14:05 PDT, `version:a1b2c3d`, fraud span
accounts for 230ms of it ([trace](…))" is evidence. "Looks like fraud is slow" is
not.

## Which signal answers which question

| Question | Reach for | Why not the others |
|---|---|---|
| How often / how much / is it trending | **metrics** (incl. trace metrics) | Cheap, unsampled, long retention, but can't show you *one* example |
| Where did *this* request spend its time | **traces** | The only signal with causal structure across services |
| What exactly happened, with what values | **logs** | Arbitrary detail, but expensive and often sampled or excluded |
| Where does the CPU / memory / lock time go *inside* a process | **profiles** | Traces stop at the span boundary; profiles see inside it |
| Did it change at a deploy | any of the above, **split by `version`** | Without a version tag you're eyeballing timestamps |

Move between them: a metric finds the window, a trace finds the slow span, and
the profile (or a log correlated by trace id) explains it.

## Reading the data honestly

The traps that turn telemetry into confident wrong answers:

- **Sampled ≠ complete.** APM keeps a sample of traces, so a missing trace
  doesn't prove something didn't happen. Get rates and latency from trace
  *metrics*, which most APM backends compute before sampling, and use spans
  for examples.
- **Logs can be dropped before indexing.** Exclusion filters, log-level
  thresholds, and retention windows all remove data. If a query returns
  nothing, find out whether the log *could* have been there before concluding
  it wasn't.
- **A query that matches nothing may just be the wrong query.** Check the
  field name, the facet, `env`, and the time zone first. Test the query
  against a case you *know* exists.
- **Percentiles don't average.** The average of per-host p99s is not the
  fleet p99. Percentiles need distribution-type data aggregated across the
  fleet.
- **Long windows hide spikes.** Wide time ranges roll up to coarse intervals,
  so a 30-second stall vanishes inside a 5-minute average. Zoom in, or roll up
  by `max`.
- **Averages hide bimodality.** A 200ms mean can be 95% at 50ms plus 5% at 3s.
  Check the distribution or a heatmap before trusting the mean.
- **Correlating across services needs the same clock and the same id.**
  Without a trace id in the logs, you're joining on timestamps and guessing.
- **Convert times before you say them.** Backends store UTC, but the UI shows
  the browser's zone. Say which one you mean.

## Profiles

Read [references/profiling.md](references/profiling.md) when reading a
flamegraph or choosing a local profiler. In short: self time says where the
work is, total time says who caused it, and wall time ≠ CPU time. A request
that's slow on wall time but cheap on CPU is waiting on something: I/O, a
lock, or a pool.

When you can reproduce the behavior locally in Ruby, read
[references/local-ruby.md](references/local-ruby.md). It covers singed for a
flamegraph scoped to one block, spec, or request; TracePoint for what actually
runs and what raised and got swallowed; and the debug gem's non-interactive
breakpoints for state at a point.

## Scaffolding

When the evidence doesn't exist, propose the smallest instrumentation that
settles the question, specific enough to paste into a PR:

- **Name the question it answers.** "Tag `checkout.submit` with
  `payment.provider` so we can split p99 by provider," not "add more
  tracing."
- **High cardinality goes on spans and logs, never on metrics.** Every unique
  tag combination on a custom metric is a separate time series, stored and
  billed. `user_id`, `request_id`, and raw URLs belong on spans and log
  attributes; metrics get bounded tags like `endpoint`, `status_class`, and
  `tier`.
- **Wide events beat scattered log lines.** One structured record per unit of
  work, carrying every useful field, answers questions nobody has thought of
  yet. Twelve `info` lines answer only the ones they were written for.
- **Structured fields, not interpolated strings.** `amount_cents=1200` can be
  queried; `"charged $12.00"` can only be grepped.
- **Correlation first.** Put the trace id in every log line and tag every
  signal with `service`, `env`, and `version`. Without those, nothing joins
  and you can't split by deploy.
- **Count what you're about to measure.** Record "rows scanned: 307,485,
  returned: 3" as span tags. Counters often explain a duration better than the
  duration does.
- **Histograms (distributions) for latency, counters for events.** Gauges for
  neither.
- **Never instrument sensitive data:** card or account numbers, emails,
  tokens, request bodies. Log an id or a bucketed value instead. Logs get
  copied, and they outlive the data they describe.
- **Say how you'll know it worked.** Give the query that will return a result
  once the instrumentation ships.

## Datadog

Read [references/datadog.md](references/datadog.md) for query syntax (logs,
metrics, spans), the API, and Datadog-specific traps. If Datadog's MCP server
is connected (look for `datadog` in the tool names), prefer it over raw API
calls; the reference maps each question to its tool. If it isn't connected,
check the notes doc for how this org accesses Datadog and mention once that
the MCP server would help. Then write the query and the UI link for the user
to run, and say that's what you did.

## Keeping the notes

The notes doc is what makes the next investigation faster than this one.
Update it **during** the work, as soon as something proves useful, not in a
wrap-up that never comes.

- **Add** an entry when a query answered something, when you find a dashboard
  or monitor worth returning to, when you work out what a service is, or when
  you hit a gotcha.
- **File it under its section** (see the template). Edit an existing entry
  rather than adding a near-duplicate. If an entry fits no section, the
  sections are wrong: add one rather than making a junk drawer.
- **Each entry says what question it answers**, with the link or query and a
  `verified YYYY-MM-DD` date. A dashboard link with no stated purpose gets
  ignored.
- **Fix or delete** what turns out to be stale (a dead link, a renamed metric,
  a gotcha that's been fixed) instead of adding a correction next to it.
- **Investigations** get one line each: question → answer → where the
  evidence is. No narrative.
- **Nothing secret.** No API keys or tokens, and no customer data copied out
  of logs. Record the *name* of the env var holding a key, never its value.
