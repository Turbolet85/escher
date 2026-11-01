# Classified — obs · error-capture-reporting.b1

## Answers
error-capture-reporting#1.1 · yes · the recorded absence of telemetry backends (sentry, analytics, gtag) binds where a crate enters or leaves the lock, per the Needs item on telemetry dependencies
error-capture-reporting#1.2 · yes · the recorded absence of a sentry backend over the examples slice binds where the take's examples edits land and a crate may enter
error-capture-reporting#1.3 · yes · the recorded absence of sentry and opentelemetry backends over the s05 files binds where a crate enters the lock
error-capture-reporting#1.4 · yes · no panic::set_hook or catch_unwind in the listed s06 files binds the key-press panic at position_in_containing_block, its chained hook and the raw message std's hook still prints
error-capture-reporting#1.5 · yes · the recorded absence of sentry, opentelemetry and span! binds the backend absence where a crate enters, and the escher-driver span surface read-back (2 info_span! sites, command and tool_call fields)
error-capture-reporting#1.6 · yes · the recorded absence of metric, counter!, histogram and backends over s11 binds the in-process counters a carried engine commit could reach (blitz-paint layer counters, blitz-dom resolve phases) and the backend absence where a crate enters
