@task_prose part=session
The acting verbs' caused answer now carries each record's text: message, file, line, column, error chain and boundary for panics and errors, message and scrubbed fields for logs, from the app, the framework and the engine; the closed-words caused shape (a count per kind and level) and the capture's count-only contract are retired.
The capture keeps text, admits app targets at every level, framework and engine targets at warn and above (engine records newly admitted), never escher targets, and scrubs every source by field name with the sink's content-named set; the message is kept as its author wrote it, an engine record's message included.
The capture installs once per process a chaining panic hook that keeps a panic's payload and location on the capture open on the panicking thread and always runs the hook it replaced; it prints, logs and fields nothing.
Bounds measured and pinned: a text is cut at 1024 characters and says how much it cut; 8 fields, 8 chain links and 16 caused items are kept, the rest counted (fields_past, chain_past, and a new caused_past field on click, type, press, advance and scroll, by kind and source); the content-named set gains password (11 to 12 names).
Six engine warn and error sites now write fixed messages with their values in named fields (href, url, src, text, request), and the engine exposes whether tracing was compiled in; every error boundary, root or own, is registered so a step reads errors new since its marks.
A new founder check installs escher's own sink in process over an in-memory writer at trace and finds no payload or raw href on its lines; CI's Windows job first failed an MCP comparison of caused and passed once the result text was decoded whole by a shared test helper.
@task_file n=1 path=packages/escher-telemetry/src/capture.rs
what: edited — the scoped telemetry capture behind the driver's caused answer: which targets it admits per level, the kept records, fields and panics with their bounds, the field-name scrub, and its chaining panic hook
@task_file n=2 path=packages/escher-telemetry/src/lib.rs
what: edited — escher-telemetry's sink entry points: the subscriber the sink installs, now also a public uninstalled builder at a level, and the capture's re-exports
@task_file n=3 path=packages/escher-telemetry/src/format.rs
what: edited — escher's sink log line format and its scrub set, the content-named field set (CONTENT_FIELDS), now holding password
@task_file n=4 path=packages/blitz-dom/src/events/pointer.rs
what: edited — the engine's link click, its warning on an unparseable href now a fixed message with href and url fields
@task_file n=5 path=packages/blitz-dom/src/form.rs
what: edited — the engine's form submission, its unimplemented-scheme warning now carrying the scheme in a url field
@task_file n=6 path=packages/blitz-dom/src/mutator.rs
what: edited — the engine's iframe load, its unresolvable-src warning now a fixed message with a src field
@task_file n=7 path=packages/blitz-dom/src/layout/mod.rs
what: edited — the engine's layout, its text-node error's field renamed to text
@task_file n=8 path=packages/blitz-dom/src/lib.rs
what: edited — the engine crate root, now exposing whether engine tracing is compiled in (TRACING_COMPILED)
@task_file n=9 path=packages/dioxus-native/src/assets.rs
what: edited — the framework's asset loading, its two warnings now fixed messages with a request field
@task_file n=10 path=packages/dioxus-native-dom/src/boundary.rs
what: edited — the error boundary marks: the root error mark and the new registry of every error boundary, root or own, whose errors a step reads with their message and chain
@task_file n=11 path=packages/dioxus-native-dom/src/dioxus_document.rs
what: edited — the Dioxus document, providing the boundary registry before its first build
@task_file n=12 path=packages/dioxus-native-dom/src/lib.rs
what: edited — the dioxus-native-dom crate root, exporting the boundary types
@task_file n=13 path=packages/escher-driver/src/caused.rs
what: edited — the driver's caused answer: one item per panic, error and log with its text, location, chain, boundary and fields, the 16 kept and the rest counted by kind and source
@task_file n=14 path=packages/escher-driver/src/execute.rs
what: edited — the driver's acting verbs' settled step and their outcomes, now reading every boundary's marks and carrying caused_past
@task_file n=15 path=packages/escher-driver/src/json.rs
what: edited — the driver's command answer writer: the caused item shape and the caused_past field
@task_file n=16 path=packages/escher-driver/src/schema.rs
what: edited — the driver's answer schema and its help: caused rewritten, caused_past added on the five acting verbs
@task_file n=17 path=packages/escher-driver/src/schema_tests.rs
what: edited — the answer schema's field-kind table check
@task_file n=18 path=packages/escher-driver/src/lib.rs
what: edited — the escher-driver crate root, exporting the caused types
@task_file n=19 path=packages/escher-driver/src/fixture_tests.rs
what: edited — the driver's fixture unit checks, following the acting outcomes' new field
@task_file n=20 path=packages/escher-driver/src/recording_tests.rs
what: edited — the session recording unit checks, following the acting outcomes' new field
@task_file n=21 path=packages/escher-driver/src/replay.rs
what: edited — the kept check replay, following the acting outcomes' new field
@task_file n=22 path=tests/blitz-tests/tests/stand_act_error_text.rs
what: added — the founder check of caused's text on every carrier (in process, serve_mcp, the session socket, a kept check), its bounds, its scrub, and that no needle reaches the sink's lines at trace
@task_file n=23 path=tests/blitz-tests/tests/capture_panic_hook.rs
what: added — the check that the capture's panic hook installs once and a panic outside a capture reads as before
@task_file n=24 path=tests/blitz-tests/tests/stand_act_errors.rs
what: edited — the headless stand's check of errors an acting step caused, the root boundary's Err on a session of its own, and caused crossing serve_mcp
@task_file n=25 path=tests/blitz-tests/tests/session_common/mod.rs
what: edited — the stand's shared session helpers, now holding the one decoder of an MCP tool result's text (mcp_result_text)
@task_item n=1 part=touches
decision/capture-framework-targets-warn-up: the capture now admits engine targets at warn and above too, app targets at every level, never escher targets, and keeps each record's text rather than counting it.
@task_item n=2 part=touches
decision/headless-stand-no-sink: a new stand check installs escher's own sink in process over an in-memory writer at trace — an exception the row does not state.
@task_item n=3 part=touches
contract/telemetry-scrub-set/error: an error's text now crosses in the caused answer, so a no-other-carrier statement is stale; the sink's lines still hold none of it.
@task_item n=4 part=touches
contract/telemetry-scrub-set/url: the engine's link click, form submission and asset warnings now carry their values in url, href, src and request fields, redacted by name.
@task_item n=5 part=touches
decision/escher-target-content-named-set and contract/escher-log-line/scrub and contract/span-line/scrub: the content-named set gains password, 11 to 12 names, on the sink and the capture alike.
@task_item n=6 part=touches
decision/scrub-at-sink-not-call-sites and decision/scrub-sink-reach: the capture now scrubs by the same field-name set, a second place the scrub runs beside the sink.
@task_item n=7 part=touches
decision/engine-target-safe-fields: the sink still redacts an engine record's message on its lines, but the capture keeps that message whole in the answer; six engine sites stopped interpolating values into messages.
@task_item n=8 part=touches
decision/fixed-diagnostics and decision/escher-session-fixed-diagnostics: the six engine and framework warn and error sites now write fixed messages (6 to 0 interpolating of 43).
@task_item n=9 part=touches
contract/escher-telemetry-panic-hook/previous-hook and contract/escher-telemetry-events/panic: a second, chaining panic hook installed by the capture runs before the sink's and always calls the hook it replaced; a panic outside a capture reads as before.
@task_item n=10 part=touches
decision/driver-library-installs-no-subscriber and decision/driver-emits-no-event: the driver still installs no subscriber and emits no event, but opening the capture now installs a process-wide panic hook, a stated exception.
@task_item n=11 part=touches
decision/escher-crates-always-trace and decision/escher-crates-tracing-ungated: the boundary registry is ungated, and escher-telemetry now offers a public uninstalled subscriber builder at a level filter.
@task_item n=12 part=touches
decision/engine-tracing-by-invocation and decision/engine-tracing-feature-gated: the engine now exposes whether tracing was compiled in, read by the founder check per build; the gating itself is unchanged.
@task_item n=13 part=touches
decision/answers-echo-nothing-typed and decision/session-logs-carry-no-page-content: a caused message holds what its author wrote; the typed password and the raw href appear 0 times in the answer and no sink line holds the error text.
@task_item n=14 part=touches
decision/debug-types-never-logged: the capture's kept text, field, record and panic types print no text in Debug, and the capture's Debug prints the level counts alone.
@task_item n=15 part=touches
contract/kept-check/answer and contract/kept-check/answer-digest and contract/recording-file/answer: a kept or recorded answer now holds caused's text, and a replay with a changed cut stops naming caused.
@task_item n=16 part=touches
decision/exploration-catches-panics: a caught panic the capture's hook did not see still answers a panic item with message <payload unread>, so the exploration's panic count loses no item.
@task_item n=17 part=touches
contract/command-span/field-count and decision/command-span-no-call-content and decision/driver-spans-carry-no-content: the command span keeps its eight fields and holds no caused, asserted anew.
@task_item n=18 part=touches
decision/sink-target-allowlist: the capture reuses the framework and engine target prefix sets to classify a record's source.
@task_item n=19 part=touches
decision/capability-statement-updated-by-hand: the capability statement owes an account of caused and its text (operator-approved proposal).
@task_item n=20 part=touches
decision/sink-installed-first-in-main: a test installs the sink's subscriber in process, outside any native main, through the new public builder.
@task_fact n=1 slug=6-log-coverage
fact: The capture keeps each admitted record's message, target, level, file, line and up to 8 fields; a text is cut at 1024 characters and says how much it cut; up to 16 records per source are kept and the rest counted per source.
@task_fact n=2 slug=8-pii-scrubbing-compliance
fact: The capture scrubs every source by field name with the sink's content-named set, a hidden value reading [redacted], the message and other fields kept as written — a message holds what its author wrote.
@task_fact n=3 slug=7-error-capture-reporting
fact: The capture installs once per process a chaining panic hook that keeps a panic's payload and location on the capture open on the panicking thread, does nothing when none is open, always runs the hook it replaced, and prints, logs and fields nothing.
@task_fact n=4 slug=7-error-capture-reporting
fact: The caused answer of click, type, press, advance and scroll carries one item per panic, boundary error and log, in that order, with message, location, an error's chain (8 links kept, chain_past) and boundary root or own; the first 16 are kept and the rest counted in caused_past by kind and source.
@task_fact n=5 slug=7-error-capture-reporting
fact: Every error boundary, root or own, is registered for the document's life, and a step reads the errors new since its marks from all of them; a dropped boundary cannot be told, so an app remounting many boundaries grows the registry.
@task_fact n=6 slug=3-observability-harness-contract
fact: The headless stand runs with no sink, except the error-text founder check, which installs escher's own sink in process over an in-memory writer at trace to prove no needle reaches its lines.
@task_fact n=7 slug=8-pii-scrubbing-compliance
fact: Engine and framework warn and error sites write fixed messages and carry their values in named fields (href, url, src, text, request), so the field-name scrub reaches them.
@task_fact n=8 slug=2-telemetry-strategy
fact: The engine exposes whether its tracing feature was compiled in, so a check can tell a tracing build from a plain one.
@task_item n=1 part=untouched
Telemetry backends and metrics: no exporter, OTel SDK, error-reporting service or metric emission was added; the absences stand.
@task_item n=2 part=untouched
Cold-agent and agent-run harness rows: their events, verdicts, logs, transcripts and cleanup are not reached.
@task_item n=3 part=untouched
Script error surfaces: the window error event, window.onerror, script error source labels, the script diagnostics drain and WPT reports are unchanged.
@task_item n=4 part=untouched
CI integration rows: the GitHub workflows, their uploaded artifacts and the package-install diagnostic are unchanged; the CI red was a test comparison fixed in the test.
@task_item n=5 part=untouched
Session recording and state directory privacy, recording bounds and the session's idle expiry are not reached; no new IPC method, socket, env var, verb, refusal cause, session error or exit was added.
