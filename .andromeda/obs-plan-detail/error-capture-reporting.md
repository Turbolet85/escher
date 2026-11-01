# obs-plan · Error Capture & Reporting
Read whole. The master's broad view is .andromeda/obs-plan.md; this file holds its Error Capture & Reporting detail.

## 7. Error Capture & Reporting

<!-- detail: absence · errors -->

- observed absent — an error-reporting client · searched: `sentry|analytics|gtag` over the 21 s02 files
- observed absent — error capture service · searched: `sentry|Sentry` over the 32 slice files (examples slice)
- observed absent — error reporting services · searched: `sentry|opentelemetry` over the 15 s05 files
- observed absent — an error reporting service or panic hook · searched: `sentry|panic::set_hook|catch_unwind` over the 17 listed s06 files
- observed absent — an error reporting service · searched: `sentry|opentelemetry|otel|span!` over the 16 listed files (no match)
- observed absent — an error-reporting service · searched: `metric|counter!|histogram|opentelemetry|sentry|prometheus` over the 21 listed s11 files

<!-- /detail -->

<!-- detail: values · errors -->

- blitz-dom: a failed stylesheet is captured as a typed answer: `ResourceHandler<StylesheetHandler>::error` responds `Err(error)`, so `pending_critical_resources` loses the request and a failed, empty or non-CSS stylesheet stops blocking render (packages/blitz-dom/src/net.rs:144-146; as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md)
- blitz-traits: `NetHandler::error` is a public trait method with a default that does nothing, so a handler without an override drops the error; blitz-test-harness's `CountedHandler` forwards it (packages/blitz-traits/src/net.rs:36-38; packages/blitz-test-harness/src/settle.rs:121-125; as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md)
- blitz-net: a failed fetch reaches its handler, the error arm calling `handler.error(url, e.to_string())` (packages/blitz-net/src/lib.rs:313); it has no log site of its own, the standing `Error fetching` and `Resource load failed` lines reporting it; `url` and `error` are in escher-telemetry's scrub set (packages/escher-telemetry/src/format.rs:50; packages/escher-telemetry/src/format.rs:59), so on `Resource load failed` the sink prints `waiting_nodes`, every other field, the message included, `[redacted]`, and the error text stops in `ResourceLoadResponse` and that log line, no carrier taking it (as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/evidence/sink.md)

<!-- /detail -->
