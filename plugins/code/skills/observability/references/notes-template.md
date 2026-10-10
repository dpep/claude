---
name: observability
description: Private observability notes for the observability skill — telemetry access, services, dashboards, monitors, saved queries, tag conventions, profiling setup, gotchas, and an investigation log.
---

# Observability notes

Private: the org names, links, and conventions the public `observability`
skill must not carry. Each entry says what question it answers and when it was
last verified. Edit entries in place; delete stale ones.

## Access

<!-- Backend + site/org, how to authenticate (env var NAMES, never values), MCP server + enabled toolsets, envs that exist. -->

## Services

<!-- service name → what it does, owning team, upstream/downstream, APM operation name, notable tags. -->

## Dashboards

<!-- [name](link) — the question it answers. verified YYYY-MM-DD -->

## Monitors & SLOs

<!-- [name](link) — what fires it, what it means, runbook. verified YYYY-MM-DD -->

## Saved queries

<!-- question → query (metric / log / span), plus a UI link when useful. verified YYYY-MM-DD -->

## Tags & fields

<!-- tag/attribute conventions, custom facets, where cardinality limits bite. -->

## Profiling

<!-- which services have continuous profiling on, profile types enabled, how to get one locally. -->

## Gotchas

<!-- traps specific to this org's setup: sampling rates, exclusion filters, retention, misleading metrics. -->

## Investigations

<!-- one line each: YYYY-MM-DD — question → answer → where the evidence is. -->
