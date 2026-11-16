@task_prose part=session
The acting verbs' caused answer now carries text: each record's message, file, line and column, its source (app, framework or engine) and target, an error's boundary and cause chain, and a log record's fields, in place of the retired closed-words shape that held a count per kind and level.
Kept text is scrubbed by field name alone: a field named in the content-named set reads [redacted] on every source, an engine record's message included kept whole; password joined that set (11 to 12 names), on the sink and in the capture alike.
Kept text is bounded: 16 items, 8 chain links and 8 fields per item, each text cut at 1024 characters with its cut count; the rest is counted in a new caused_past field (kind, source, count) on click, type, press, advance and scroll.
The scoped capture now admits engine records at warn and above, and installs once per process a chaining panic hook that keeps a panic's payload and location on the open capture, prints and logs nothing, and always runs the hook it replaced.
Every error boundary, root and own, is now read through a boundary registry in the native document; six engine log sites moved their values out of the message into named fields (href, url, src, text, request) with fixed messages.
No new IPC method, socket, port, env var, verb, refusal cause, session error, exit or dependency; CI green on 17 of 17 after a Windows fix to a test's decoding of the MCP result text.
@task_file n=1 path=packages/escher-telemetry/src/capture.rs
what: the scoped capture of log records and panics during a step, its admission by target and level, its scrub by content-named field, its text bounds and its process-wide chaining panic hook (edited)
@task_file n=2 path=packages/escher-telemetry/src/lib.rs
what: escher's telemetry sink entry points, now a public uninstalled subscriber builder taking a level (edited)
@task_file n=3 path=packages/escher-telemetry/src/format.rs
what: the sink's line format and its content-named field set CONTENT_FIELDS, now holding password (edited)
@task_file n=4 path=packages/blitz-dom/src/events/pointer.rs
what: the engine's pointer events, its warn when a clicked link's href does not parse, now a fixed message with href and url fields (edited)
@task_file n=5 path=packages/blitz-dom/src/form.rs
what: the engine's form submission, its warn for an unimplemented scheme, now a fixed message with a url field (edited)
@task_file n=6 path=packages/blitz-dom/src/mutator.rs
what: the engine's DOM mutator, its warn when an iframe src cannot be resolved, now a fixed message with a src field (edited)
@task_file n=7 path=packages/blitz-dom/src/layout/mod.rs
what: the engine's layout, its text-node error log whose field data is renamed text (edited)
@task_file n=8 path=packages/blitz-dom/src/lib.rs
what: the engine crate root, now exporting TRACING_COMPILED, whether the tracing feature is built in (edited)
@task_file n=9 path=packages/dioxus-native/src/assets.rs
what: the Dioxus native asset handler, its two warns now fixed messages with a request field (edited)
@task_file n=10 path=packages/dioxus-native-dom/src/boundary.rs
what: the error boundary reading of the native document: the root error mark, and the new ungated boundary registry with boundary marks reading every boundary's error, message and chain (edited)
@task_file n=11 path=packages/dioxus-native-dom/src/dioxus_document.rs
what: the Dioxus native document, which now provides the boundary registry before its first build (edited)
@task_file n=12 path=packages/dioxus-native-dom/src/lib.rs
what: the Dioxus native DOM crate root, exporting the boundary types (edited)
@task_file n=13 path=packages/escher-driver/src/caused.rs
what: the driver's caused answer builder: one item per panic, boundary error and kept record, the 16-item and 8-link bounds, and the caused_past counts (edited)
@task_file n=14 path=packages/escher-driver/src/execute.rs
what: the driver's acting verb execution and settled step, reading every boundary's mark before a step and returning caused and caused_past (edited)
@task_file n=15 path=packages/escher-driver/src/json.rs
what: the driver's answer writer, writing the caused items' text, cut, location, boundary, chain and fields, and caused_past (edited)
@task_file n=16 path=packages/escher-driver/src/schema.rs
what: the driver's closed verb schema, its caused help rewritten and the new caused_past result field on the five acting verbs (edited)
@task_file n=17 path=packages/escher-driver/src/schema_tests.rs
what: the driver's verb schema unit tests, its field-kind table (edited)
@task_file n=18 path=packages/escher-driver/src/lib.rs
what: the driver crate root, exporting the caused types (edited)
@task_file n=19 path=packages/escher-driver/src/fixture_tests.rs
what: the driver's fixture unit tests over acting verb outcomes (edited)
@task_file n=20 path=packages/escher-driver/src/recording_tests.rs
what: the driver's session recording unit tests (edited)
@task_file n=21 path=packages/escher-driver/src/replay.rs
what: the kept check replay through the session, matching each step's outcome (edited)
@task_file n=22 path=tests/blitz-tests/tests/stand_act_error_text.rs
what: the integration check that each failure's text, location, chain and source cross every carrier, are scrubbed and bounded, replay in a kept check and reach no sink line (new, then edited)
@task_file n=23 path=tests/blitz-tests/tests/capture_panic_hook.rs
what: the integration check that the capture's panic hook installs once and a panic outside a capture reads as before (new)
@task_file n=24 path=tests/blitz-tests/tests/stand_act_errors.rs
what: the integration check of a root-boundary Err on its own session and the same caused crossing every carrier (edited)
@task_file n=25 path=tests/blitz-tests/tests/session_common/mod.rs
what: the shared session test helpers, now the one decoder of an MCP tool result's text (edited)
@task_item n=1 part=touches
decision/caused-answer-crossing: caused no longer crosses as closed words and counts; it crosses each record's message, location, source, target, boundary, cause chain and kept fields as text, scrubbed by field name, bounded, with caused_past beside it.
@task_item n=2 part=touches
decision/root-boundary-err-as-screen-content: a root-boundary Err's message and cause chain now cross in caused (boundary root), not as screen content only.
@task_item n=3 part=touches
decision/mask-holds-on-every-carrier and decision/masked-input-values, contract/snapshot-value/masked-value: a masked control's value stays MASKED_VALUE and a typed password reaches the answer 0 times, but a caused message holds whatever its author wrote.
@task_item n=4 part=touches
decision/no-screen-content-in-logs, decision/driver-outcomes-never-logged, decision/command-span-content-free and the driver-command-span contract items: caused text is never logged, the command span keeps its eight fields and holds no caused.
@task_item n=5 part=touches
decision/engine-target-field-allowlist and decision/engine-target-safe-fields: six engine log sites moved their values from the message into fields href, url, src, text and request with fixed messages; the sink's engine rule (safe fields only, message redacted) is unchanged on its lines.
@task_item n=6 part=touches
decision/escher-target-content-fields-withheld, decision/escher-target-content-scrub and decision/log-sink-redacts-and-allowlists: the content-named field set gains password, 11 to 12 names, tightening the sink.
@task_item n=7 part=touches
decision/telemetry-sink-target-allowlist and decision/other-log-targets-dropped, decision/outside-target-dropped: the scoped capture (not the sink) now keeps app records at every level and framework and engine records at warn and above, escher_ targets never; the sink's own target allowlist is unchanged.
@task_item n=8 part=touches
decision/driver-installs-no-subscriber and decision/driver-no-subscriber-no-env: the driver still installs no subscriber, but the capture it opens installs a process-wide chaining panic hook once; the sink's subscriber builder is now public and built uninstalled.
@task_item n=9 part=touches
decision/mcp-result-crossing, decision/session-socket-carries-calls, decision/socket-carries-screen-reading, contract/mcp-server/tool-result and contract/escher-session-stdout/command-line: the caused text crosses Session::run, the MCP result and the session socket alike.
@task_item n=10 part=touches
decision/mcp-bounded-lines, contract/mcp-tool-result/size-bound and contract/driver-command-line/answer-bounds: caused adds bounded text to an answer line (16 items, 1024-character cut, measured flood line under 1048576 bytes).
@task_item n=11 part=touches
decision/closed-driver-verb-schema and decision/closed-value-field-no-widening, decision/boundary-widening-rule: a new result field caused_past on click, type, press, advance and scroll, and caused widened from closed values to text, a ratified widening of the carriers.
@task_item n=12 part=touches
decision/kept-check-answer-whole, decision/kept-check-call-content and contract/kept-check-file/contents: a kept check now holds the caused text in its answer and replay compares it; a changed cut stops the replay naming caused.
@task_item n=13 part=touches
decision/no-echo-of-caller-input and decision/session-output-holds-no-caller-input: an app's error or log message in caused may carry what the app wrote from page state; the typed password is asserted absent.
@task_item n=14 part=touches
decision/tracing-feature-propagates and decision/tracing-for-engine-logging: engine records reach caused only when the tracing feature is built in, now readable as TRACING_COMPILED; the engine sites' tracing gating is unchanged.
@task_item n=15 part=touches
absence/error-reporting-integration: still no integration, but each step's answer now reports the errors, panics and warnings it caused, with their text.
@task_fact n=1 slug=threat-model-summary
fact: The caused answer keeps at most 16 items, 8 chain links and 8 fields per item, each text cut at 1024 characters with its cut count, and counts the rest in caused_past by kind and source.
@task_fact n=2 slug=logging-monitoring
fact: The scoped capture scrubs every source by field name alone: a content-named field reads [redacted] while messages and other fields are kept, an engine record's message included, which the sink still redacts on its own lines.
@task_fact n=3 slug=logging-monitoring
fact: The scoped capture installs once per process a chaining panic hook that keeps a panic's payload and location on the capture open on the panicking thread, prints and logs nothing, and always runs the hook it replaced.
@task_fact n=4 slug=logging-monitoring
fact: Engine warn and error sites carry their values in named fields under fixed messages; none interpolates a value into its message (0 of 43).
@task_fact n=5 slug=logging-monitoring
fact: A field named password is content-named, redacted on the sink and in caused alike.
@task_fact n=6 slug=error-handling
fact: The native document's boundary registry keeps every error boundary built for the document's life; a dropped boundary cannot be told, so an app remounting many boundaries grows it without bound.
@task_item n=1 part=untouched
Authentication and authorization, socket ownership and permissions, and the MCP stdio transport: no IPC method, socket, port or listener was added.
@task_item n=2 part=untouched
Refusal causes, session errors, exit codes and MCP protocol errors: no cause, error or exit was added or changed.
@task_item n=3 part=untouched
Dependency security, audit, licence and pin rows: no dependency was added or bumped.
@task_item n=4 part=untouched
Secret management and environment variables: no env var is read and no secret is touched.
@task_item n=5 part=untouched
Data protection's history store, cookies, HTTP cache, TLS, clipboard and file dialog rows.
@task_item n=6 part=untouched
Select list, stable element id, accessibility adapter, snapshot state words and actions or graph answer rows.
@task_item n=7 part=untouched
CI workflow, WPT runner, release publishing and cold-agent script rows.
