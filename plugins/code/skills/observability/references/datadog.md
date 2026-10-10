# Datadog

Org specifics live in the notes doc (`~/.claude/docs/observability.md`): site,
service names, dashboards, and which env vars hold keys. This file covers only
what's true for every Datadog org.

## Access, in order of preference

1. **Datadog's MCP server**, if connected. It handles auth (OAuth, as you) and
   has tools the API alone doesn't give you: a flamegraph explorer, bottleneck
   summaries, and SQL over logs and spans. See [MCP server](#mcp-server).
2. **The API**, with an API key and an application key from the environment
   (conventionally `DD_API_KEY` / `DD_APP_KEY`, plus `DD_SITE` for non-US1
   orgs). The host is `api.<site>`, e.g. `api.datadoghq.com` or
   `api.datadoghq.eu`.
3. **No access.** Write the query and a UI link for the user to open, and say
   plainly that you haven't seen the data.

## MCP server

Datadog runs an official remote MCP server ([docs](https://docs.datadoghq.com/mcp_server/),
[tools](https://docs.datadoghq.com/mcp_server/tools/)). The tool names below
were current when this was written, but Datadog says they're still changing.
**Go by the tools actually loaded in the session.** If a name here is missing,
search the loaded tools for the closest match rather than falling back to the
API.

**Setup:** in Claude Code, `/plugin install datadog@claude-plugins-official`,
then `/ddsetup`. Or `claude mcp add --transport http datadog-mcp <endpoint>`
with your site's endpoint. Tools come in **toolsets**, selected with
`?toolsets=a,b` on the endpoint; only `core` loads by default. For this skill,
add `apm` (Preview), `profiling`, `watchdog`, `error-tracking`, and `dbm` (if you have
Database Monitoring). `apm` and `live-debugger` are Preview, and
`toolsets=all` doesn't include them. Each toolset costs context, so don't
enable them all.

| Question | Tool (toolset) |
|---|---|
| What is this service, who owns it, what does it call | `search_datadog_entities` (core) |
| What changed: deploys, alerts, config | `search_datadog_events` (core), `search_dora_deployments` (software-delivery) |
| How much / trending | `get_datadog_metric`; check tags first with `get_datadog_metric_context` (core) |
| Percentiles or counts over spans, as a timeseries | `aggregate_spans` (core) |
| Example slow or failing requests | `search_datadog_spans`, then `get_datadog_trace` (core) |
| Where in a trace the time goes | `apm_latency_bottleneck_summary`, `apm_query_trace` (SQL over one trace's spans) (apm) |
| Which span tags exist to split by | `apm_discover_span_tags` (apm) |
| Log examples / log analytics | `search_datadog_logs` / `analyze_datadog_logs` (SQL-style) (core) |
| Where the CPU / alloc / lock time goes | `get_profiling_services` → `get_profiling_profile_types` → `explore_profiling_flame_graph` / `explore_profiling_call_graph`, `get_profiling_service_insights` (profiling) |
| What Datadog already flagged as anomalous | `search_watchdog_stories`, `find_new_jumps` (watchdog) |
| Which tags explain a change | `get_influential_tags` (watchdog) |
| Grouped errors across logs, traces, RUM | `search_datadog_error_tracking_issues`, `analyze_datadog_error_tracking_errors` (error-tracking) |
| Slow queries, explain plans | `find_datadog_database_instances` first, then `get_datadog_database_query_performance`, `get_datadog_database_explain_plans` (dbm) |
| What a proposed metric tag would cost | `estimate_datadog_metric_cardinality` (metrics-governance) |
| Is this signal monitored at all | `get_monitor_coverage` (alerting) |

Using it well:

- **Responses are truncated** to an estimated size. Most tools accept
  `max_tokens`. A truncated list is a sample, so use the aggregate tools for
  counts.
- **Rate limits:** 50 calls per 10 seconds, plus a monthly cap. Aggregate on
  the server; don't fetch a thousand spans to count them yourself.
- **It runs as you**, so it sees only what your Datadog role sees. An empty
  result may mean a missing permission or a log restriction query, not
  missing data.
- **Write tools change shared state:** creating monitors, editing dashboards,
  changing sampling rules, and so on. Confirm with the user before every
  write, even where the tool doesn't ask.
- **Live Debugger logpoints** (`live-debugger`, Preview) add a log line to
  running code without a deploy: scaffolding in minutes, not days. It still
  touches production, so propose it and let the user approve.
- **Bits AI investigations** (`investigator`, Preview) run Datadog's own
  agent. Treat their findings as leads to verify, not evidence.
- **Link every finding.** Where a tool returns a UI URL, pass it on.

## Unified service tagging

`service`, `env`, and `version` are reserved tags that join metrics, traces,
logs, and profiles. Without `version`, deploy comparisons fall back to reading
timestamps by eye, which makes it usually the first scaffolding item to fix.

## Adding instrumentation (Ruby, dd-trace-rb)

```ruby
Datadog::Tracing.trace("fraud.check", resource: provider) do |span|
  span.set_tag("payment.provider", provider)   # bounded: fine on a span, and on a metric
  span.set_tag("rows.scanned", rows.size)       # counters explain durations
  check!
end

Datadog::Tracing.active_span&.set_tag("cart.items", cart.items.size)   # tag the span you're already in
```

For log ↔ trace correlation, make sure log injection is on
(`c.tracing.log_injection`) and the logger emits structured JSON. Otherwise the
trace id is just part of a string.

## Log search syntax

```
service:checkout env:prod status:error
service:checkout @http.status_code:>=500 -@http.url_details.path:/health*
@duration:>2000000000                  # duration attributes are usually nanoseconds
"connection reset"                      # free text matches the message only
trace_id:<id>                           # a trace's logs (needs log injection)
@usr.id:12345                           # one user's requests
```

- `@` addresses an attribute; a bare word addresses a reserved field or tag.
- Range and attribute queries generally need a **facet** or **measure** on
  that attribute. A query on an unfaceted `@field` can come back empty even
  though matching logs exist, so check the facet before concluding anything is
  absent.
- You can search only what was **indexed**. Exclusion filters and index
  retention can drop logs that live tail or archives still had.

## Metric query syntax

```
avg:trace.rack.request.duration{service:web,env:prod} by {resource_name}
p99:trace.rack.request{service:web,env:prod}                # percentiles come from the distribution metric, not .duration
sum:trace.rack.request.hits{service:web}.as_count()
sum:trace.rack.request.errors{service:web}.as_count() / sum:trace.rack.request.hits{service:web}.as_count()
max:system.cpu.user{service:web} by {host}.rollup(max, 60)
```

- **Trace metrics** (`trace.<operation>.hits|errors|duration…`, plus the
  `trace.<operation>` distribution for percentiles) cover all
  traffic, before ingestion sampling. Use them for rates and latency, not
  counts of retained spans. The operation name depends on the integration
  (`rack.request`, `http.request`, `grpc.server`, …), so look it up rather
  than guessing.
- `.as_count()` vs `.as_rate()` changes the meaning of `count`-type metrics.
  Say which one you used.
- Wide windows roll up automatically. Set `.rollup(<agg>, <seconds>)` when a
  spike matters.
- Custom metric cost scales with unique tag combinations; see the cardinality
  rule in SKILL.md.

## Span / trace search

APM span queries use the log syntax: `service:checkout env:prod
resource_name:"POST /orders" @duration:>1s status:error`. Indexed spans are a
sample kept by retention filters, so treat them as examples, not a census.

From a slow trace:

- **Read the critical path** before the longest span. Parallel children don't
  add up.
- **Check for gaps** between spans. Untraced time is usually in-process work,
  a lock, or an uninstrumented client, and those gaps are where you add spans.
- **Code Hotspots** (endpoint profiling) links a span to the profile captured
  during it, where the profiler is enabled.

## API sketches

v1 metrics take epoch **seconds**. v2 search accepts ISO strings or
`now-1h`-style math.

```
# metrics
GET  /api/v1/query?from=<epoch_s>&to=<epoch_s>&query=<urlencoded query>

# logs: search, then aggregate
POST /api/v2/logs/events/search
  {"filter":{"query":"service:checkout status:error","from":"now-1h","to":"now"},
   "sort":"-timestamp","page":{"limit":50}}
POST /api/v2/logs/analytics/aggregate
  {"filter":{...},"compute":[{"aggregation":"count"}],"group_by":[{"facet":"@http.status_code"}]}

# spans
POST /api/v2/spans/events/search
  {"data":{"type":"search_request","attributes":{
     "filter":{"query":"service:checkout @duration:>1s","from":"now-1h","to":"now"},
     "page":{"limit":25}}}}
```

Auth headers are `DD-API-KEY` and `DD-APPLICATION-KEY`. Results are paginated
with a cursor (`meta.page.after`). One page is a sample, not a total; use the
aggregate endpoint for counts.

## UI links

When handing a query to the user, give a link, not just the text:
`https://app.<site>/logs?query=<urlencoded>&from_ts=<epoch_ms>&to_ts=<epoch_ms>`.
The UI takes epoch **milliseconds**; the v1 API takes seconds.
