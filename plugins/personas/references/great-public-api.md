# What Makes a Great Public API?

Reference for the `platform-expert` persona. The throughline: **a great API makes developers successful.** Everything else — reliability, versioning, performance, observability, documentation — exists to support that. Great APIs minimize surprises: developers should be able to *predict* how the API behaves before they read the docs, *trust* it before they integrate, and *forget* about it after they deploy.

## 1. Developer experience is the product

Most API discussions start with architecture, protocols, or performance. But developers don't choose APIs because they're REST or GraphQL — they choose APIs that are easy to understand, easy to integrate, and easy to trust. A great API minimizes cognitive load. A developer should be able to answer quickly:

- What can I do?
- How do I authenticate?
- How do I make my first successful request?
- What errors should I expect?
- How do I recover when things go wrong?

Everything below supports this goal.

## 2. Predictability beats cleverness

The best APIs are boring. Consistent naming, resource models, pagination, error handling, and authentication patterns. Developers should be able to *guess* how a new endpoint works before reading its documentation.

## 3. Reliability is a feature

An API is a dependency — every outage becomes your customer's outage. Cover availability targets, latency expectations, graceful degradation, backward compatibility, and incident communication. An unreliable API creates more developer pain than an awkward one.

## 4. Stable contracts

Inputs and outputs should behave consistently over time: schema stability, backward compatibility, versioning, deprecation policies, long support windows. The goal isn't "never change" — it's **"never surprise."**

## 5. Errors are part of the API

Many APIs spend enormous effort on success responses and almost none on failures — yet developers often spend more time handling failures than successes. Good errors:

- Use standard HTTP semantics
- Have a consistent structure
- Explain what happened
- Explain how to fix it
- Include identifiers for support/debugging

## 6. Performance with guardrails

Developers want flexibility; operators want predictability. The API must provide both: rate limits, query-complexity limits, pagination requirements, request-size limits, timeouts. The best APIs establish clear boundaries and communicate them upfront.

## 7. Documentation is part of the interface

Documentation isn't a separate product — it *is* the product. Great docs include a quick-start guide, copy-paste examples, reference documentation, common workflows, migration guides, and changelogs. If developers need support before making their first request, the documentation has failed.

## 8. Observability creates trust

You can't improve what you can't see. Internally: latency, errors, availability, usage patterns. Externally: a status page, incident history, deprecation notices, usage metrics. Transparency builds confidence.

## Conclusion

A great API isn't defined by REST vs GraphQL, JSON vs protobuf, or any technology choice. It makes developers successful — and everything else exists to support that. Minimize surprises: predict before reading, trust before integrating, forget after deploying.
