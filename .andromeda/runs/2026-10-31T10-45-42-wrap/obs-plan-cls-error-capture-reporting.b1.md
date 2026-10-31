# Classified — obs-plan · error-capture-reporting.b1

## Answers
error-capture-reporting#1.1 · no · no manifest was edited and the only dependency move is a pulldown-cmark lock bump, and the added lines add no listener, log site or print, so no analytics or gtag client enters the changed files
error-capture-reporting#1.2 · no · cells.rs gains only a Tab stop, key handler, handles and outline, all instrumentation n/a, and the escher_telemetry init sites are not reached, so the absence holds on the changed files
error-capture-reporting#1.3 · no · the only dependency move is a pulldown-cmark lock bump and not a telemetry crate, and the new code is instrumentation n/a, so no sentry or opentelemetry client enters the changed files
error-capture-reporting#1.4 · no · no panic path was added, the panic hook section is untouched and escher-telemetry's chaining hook is not reached, so no panic::set_hook or catch_unwind enters the listed files
error-capture-reporting#1.5 · no · the two info_span! sites are unmoved and the new code adds no span, so the span-free reading holds on the changed files
error-capture-reporting#1.6 · no · metric coverage is untouched (debug_timer, the resolve phase timings and blitz-paint's counters are not reached, and blitz-paint shows no diff) and the new code is instrumentation n/a, so no counter, histogram or metric enters
