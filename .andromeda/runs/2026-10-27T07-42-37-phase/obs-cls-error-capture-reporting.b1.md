# Classified — obs · error-capture-reporting.b1

## Answers
error-capture-reporting#1.1 · no · an absence of an error-reporting or analytics client; nothing in the task's Needs adds one, so no code or test of the chunk needs to honour it
error-capture-reporting#1.2 · no · an absence of a Sentry capture service in the examples slice; nothing in the task's Needs adds one, and the reached host_log and mcp_log checks need nothing from it
error-capture-reporting#1.3 · no · an absence of Sentry or OpenTelemetry exporters in the s05 files; the task's Not needed excludes any exporter the take could add
error-capture-reporting#1.4 · no · an absence of a panic hook or catch_unwind in the s06 files; the take reaches no panic hook, as the task's Not needed excludes escher-telemetry's, the WPT runner's and the WASM builds' hooks
error-capture-reporting#1.5 · no · an absence of Sentry, OpenTelemetry or span! sites over 16 listed files; the span! pattern would match the driver's info_span! sites, so they lie outside those files and the chunk's re-count is not what this states
error-capture-reporting#1.6 · no · an absence of metric, counter and exporter emissions over 21 s11 files; the task's Not needed excludes metric coverage, and the take adds no exporter, counter or metric emission
