# obs · Error Capture & Reporting — category read

## Carried
- (.andromeda/obs-plan-detail/error-capture-reporting.md:8) The sink re-check over the taken delta must find no error-reporting client or service entering any of these slices, whichever take the founder chooses; one that entered would be a new surface, named and reviewed, never absorbed silently.
  > - observed absent — an error-reporting client · searched: `sentry|analytics|gtag` over the 21 s02 files
  > - observed absent — error capture service · searched: `sentry|Sentry` over the 32 slice files (examples slice)
  > - observed absent — error reporting services · searched: `sentry|opentelemetry` over the 15 s05 files
  > - observed absent — an error reporting service or panic hook · searched: `sentry|panic::set_hook|catch_unwind` over the 17 listed s06 files
  > - observed absent — an error reporting service · searched: `sentry|opentelemetry|otel|span!` over the 16 listed files (no match)
  > - observed absent — an error-reporting service · searched: `metric|counter!|histogram|opentelemetry|sentry|prometheus` over the 21 listed s11 files
- (.andromeda/obs-plan-detail/error-capture-reporting.md:11) If the founder's P4 fork takes the `node.rs:1775` fix, the fix must repair the key-press panic at its cause and add no `panic::set_hook` or `catch_unwind` over these files to contain it; the same search must hold after the take.
  > - observed absent — an error reporting service or panic hook · searched: `sentry|panic::set_hook|catch_unwind` over the 17 listed s06 files
- (.andromeda/obs-plan.md:209) However the fork on `node.rs:1775` is answered, a panic in a sink-installing binary, the key-press panic included, must keep this one reporting path: one ERROR event with the payload redacted, the previous hook chained, the exit code unchanged. The take must leave the hook's contract as it reads here.
  > - escher-telemetry's hook, installed by `init` in `seven_guis_native` and in `escher-session`, chains: it takes the previous hook, logs one ERROR event at target `escher_telemetry::panic` with `panic.file`, `panic.line`, `panic.column` and `panic.payload` (redacted, §8), then runs the previous hook — std's default still prints the raw message to stderr and the exit code is unchanged (packages/escher-telemetry/src/panic.rs:4-24)
- (.andromeda/obs-plan.md:216) Where a take carries `f6edb9701` (a failed stylesheet no longer blocks rendering), the failed-load path must be re-read against this line, and the line and its `document.rs` citation must be corrected at the wrap if the capture or its range moves.
  > - blitz-dom: resource load errors are logged as `tracing::warn!` with `error` field and not propagated further (packages/blitz-dom/src/document.rs:1804-1823)
- (.andromeda/obs-plan.md:217) Where a take carries `f6edb9701` or `7a3dd8918`, the stylesheet parse sites must be re-read: any error reporter a taken commit adds must be recorded here, and these three citations must follow the merged lines.
  > - blitz-dom: stylesheets are parsed with no error reporter (`None, // error_reporter`) (packages/blitz-dom/src/net.rs:166; packages/blitz-dom/src/net.rs:276; packages/blitz-dom/src/document.rs:1717)
- (.andromeda/obs-plan.md:219) `layout/construct.rs` is in the predicted conflict set, and `8b1a0a3ce` brings SVG attribute damage; a take that reaches the file must keep the SVG parse error captured as stated, and the citation must follow the resolved lines.
  > - blitz-dom layout: SVG parse errors are captured into the `error` field of a warn event and not propagated (packages/blitz-dom/src/layout/construct.rs:460-470)
- (.andromeda/obs-plan.md:227) `runtime/events.rs` is a predicted add/add conflict, and these lines' citations stand in it: a resolution must keep the script error surface's intent (the event's fields, the `onerror` call, no further event from a handler's exception, the source labels), and each citation must be re-read against the merged file.
  > - The window `error` event carries message, filename "", lineno 0, colno 0 and error (packages/blitz-vibey-script/src/runtime/events.rs:58-74)
  > - `window.onerror` is called with message, source, lineno, colno, error (packages/blitz-vibey-script/src/runtime/events.rs:84-96)
  > - Exceptions thrown by error handlers are recorded but fire no further error events (packages/blitz-vibey-script/src/runtime/events.rs:34-36; packages/blitz-vibey-script/src/state.rs:93-96)
  > - Error sources labelled in reports include "timer callback", "event listener", "error event listener", "timer microtasks", "event microtasks" (packages/blitz-vibey-script/src/runtime/timers.rs:32; packages/blitz-vibey-script/src/runtime/timers.rs:35; packages/blitz-vibey-script/src/runtime/events.rs:172; packages/blitz-vibey-script/src/runtime/events.rs:278; packages/blitz-vibey-script/src/runtime/events.rs:81)
- (.andromeda/obs-plan.md:99) The same conflict must leave the uncaught-JS-error log whole: still in `report_js_error`, still feature-gated, still at the engine target the sink's allowlist admits, with this take's own census reading it again at the chosen sha.
  > the fixed-sha census over their `packages/*.rs` lines reading its two named hits, the uncaught-JS-error `tracing::error!` standing whole in `report_js_error`, still under `#[cfg(feature = "tracing")]` at the engine target `blitz_vibey_script` (packages/blitz-vibey-script/src/runtime/events.rs:20-21)
- (.andromeda/obs-plan.md:235) This broad absence must still read true after the take, as the body's statement of the detail file's six searches.
  > - Error-reporting clients and services observed absent over the reading's s02, examples, s05, s06 and s11 slices and the 16 listed files.

## Bindings
- **Panic hook ↔ §6 Log Coverage**: the hook's ERROR event (.andromeda/obs-plan.md:209) and the logged-event line naming its target and fields (.andromeda/obs-plan.md:188) must keep stating one contract; a change to either moves both.
- **Panic hook ↔ §8 PII Scrubbing**: the raw message the chained hook prints stands past the scrub (.andromeda/obs-plan.md:255), so the key-press panic, which a remote page can reach, must not be reported as scrubbed by the event's redacted payload (.andromeda/obs-plan.md:209).
- **Uncaught-JS-error log ↔ §8 Scrubbing**: the event at `blitz_vibey_script` (.andromeda/obs-plan.md:99) prints only as the engine-target rule allows (.andromeda/obs-plan.md:259); the sink allowlist re-check must read both together.
- **Absences ↔ §8 reach of the sink**: a third-party target entering with a taken crate has its errors dropped whole by the sink (.andromeda/obs-plan.md:263), so the absence searches (.andromeda/obs-plan-detail/error-capture-reporting.md:8) and the lock read for new crates must be read as one question: what would report an error, and where would it go.
- **Failed stylesheet ↔ §6 engine emitters**: a log site `f6edb9701` adds or moves on the failed-load path (.andromeda/obs-plan.md:216) must also be recorded among blitz-dom's log sites (.andromeda/obs-plan.md:192).
