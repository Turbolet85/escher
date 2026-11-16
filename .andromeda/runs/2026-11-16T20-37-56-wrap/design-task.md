@task_prose part=session
The session put each failure's text into the acting verbs' answer: caused is now one item per panic, error or log record, carrying its message, its source (app, framework or engine), target, file, line and column, the error boundary that took it (root or own), its cause chain and its kept fields.
Text is cut at 1024 bytes with the amount cut stated; the first 16 items are kept and the rest are counted in a new caused_past field on click, type, press, advance and scroll, written only when there are any; a chain keeps 8 links and a log record keeps 8 fields.
The scoped capture behind caused now keeps text, not only counts; it also admits engine records at warn and above, and it installs a once-per-process chaining panic hook that prints nothing and always runs the hook it replaced.
The capture hides a field by its name on every source, and password joined the content-field set (11 to 12 names); a message stays as its author wrote it. Six engine log sites now use fixed messages and carry their values in named fields.
The answer crosses the command line, the MCP role and the session socket alike; an MCP-side check now decodes the tool result text whole because Windows paths in a panic item's file carry backslashes.
Retired: the closed-words caused shape (a count per kind and level) and the capture's count-only contract. No verb, exit status, session error, refusal, socket or env var was added.
@task_file n=1 path=tests/blitz-tests/tests/stand_act_error_text.rs
what: the founder check of the acting verbs' caused text: each failure answers what failed, where and from which source, on the command line, the MCP role and the session socket; it installs escher's own log sink over an in-memory writer, an exception to the headless stand having no sink
@task_file n=2 path=tests/blitz-tests/tests/capture_panic_hook.rs
what: the check that the capture's chaining panic hook is installed once and that a panic outside a capture reads as before
@task_file n=3 path=packages/escher-telemetry/src/capture.rs
what: the scoped capture under each driver step: the records, panics and per-source counts it keeps, its admission rule by source and level, its scrub by field name, and its chaining panic hook
@task_file n=4 path=packages/escher-telemetry/src/lib.rs
what: the telemetry crate's entry points, now with a public subscriber builder (the sink plus the capture layer, not installed)
@task_file n=5 path=packages/escher-telemetry/src/format.rs
what: the log sink's line format and its content-field scrub set, now with password
@task_file n=6 path=packages/blitz-dom/src/events/pointer.rs
what: the engine's pointer event handling; its warning for a clicked link whose href does not parse now has a fixed message, with href and url as fields
@task_file n=7 path=packages/blitz-dom/src/form.rs
what: the engine's form submission; its warning for an unimplemented submission scheme now has a fixed message and a url field
@task_file n=8 path=packages/blitz-dom/src/mutator.rs
what: the engine's DOM mutator; its warning for an iframe whose src does not resolve now has a fixed message and a src field
@task_file n=9 path=packages/blitz-dom/src/layout/mod.rs
what: the engine's layout of text nodes; the field of its text-node error was renamed from data to text
@task_file n=10 path=packages/blitz-dom/src/lib.rs
what: the engine crate's root, now exposing whether tracing is compiled in (TRACING_COMPILED)
@task_file n=11 path=packages/dioxus-native/src/assets.rs
what: the dioxus-native asset loader; both of its warnings now have fixed messages and a request field
@task_file n=12 path=packages/dioxus-native-dom/src/boundary.rs
what: the error-boundary reads of the dioxus-native document: a registry of every error boundary (root or own), a mark per boundary, and an error read back with its cause chain; the root error mark is kept
@task_file n=13 path=packages/dioxus-native-dom/src/dioxus_document.rs
what: the dioxus-native document, which now provides the boundary registry before its first build
@task_file n=14 path=packages/dioxus-native-dom/src/lib.rs
what: the dioxus-native-dom crate root and its public exports of the boundary types
@task_file n=15 path=packages/escher-driver/src/caused.rs
what: the driver's caused answer: one item per panic, error or log record with message, source, location, boundary, chain and fields; the 16-item bound and the caused_past counts
@task_file n=16 path=packages/escher-driver/src/execute.rs
what: the driver's step execution for the acting verbs: the settled step reads every boundary's marks before it acts and returns caused with caused_past
@task_file n=17 path=packages/escher-driver/src/json.rs
what: the driver's answer-line writer: the caused item members and the caused_past member
@task_file n=18 path=packages/escher-driver/src/schema.rs
what: the driver's verb schema table that help reads: caused's rewritten help and the new caused_past field on click, type, press, advance and scroll
@task_file n=19 path=packages/escher-driver/src/schema_tests.rs
what: the driver schema's field-kind table checks (30 to 31 rows)
@task_file n=20 path=packages/escher-driver/src/lib.rs
what: the driver crate's root and its public exports of the caused types
@task_file n=21 path=packages/escher-driver/src/fixture_tests.rs
what: the driver's fixture tests of the acting verbs' outcomes, now with caused_past
@task_file n=22 path=packages/escher-driver/src/recording_tests.rs
what: the driver's session-recording tests, now with the outcome's caused_past
@task_file n=23 path=packages/escher-driver/src/replay.rs
what: the replay of a kept exploration check, which compares the replayed caused
@task_file n=24 path=tests/blitz-tests/tests/stand_act_errors.rs
what: the stand check of a step's errors: a root boundary error on a session of its own, now read as crossing in caused, and the same caused crossing the MCP role, compared on the decoded result text
@task_file n=25 path=tests/blitz-tests/tests/session_common/mod.rs
what: the shared helpers of the session stand checks, now with the one decoder of an MCP tool result's text (mcp_result_text)
@task_item n=1 part=touches
The driver verbs and virtual clock contract items for click, type, press and advance: each verb's answer now has a caused_past field after caused, and caused's item shape changed from closed words with counts to one text item per record.
@task_item n=2 part=touches
The scroll verb's answer gains caused_past too; a row on what the scroll answer holds is touched, though scroll is still instant.
@task_item n=3 part=touches
The escher-session MCP role's other tool results: the result text now carries caused's text items and caused_past, and a check now compares that text after decoding the JSON string whole.
@task_item n=4 part=touches
The escher-session stdout command-line and MCP-role output lines: an acting verb's answer line now holds message, location, chain and field text, bounded by a 1024-byte cut per text and 16 items per answer.
@task_item n=5 part=touches
The escher-session help command contract: the help text for caused was rewritten and a caused_past field was added to five verbs in the schema table that help reads.
@task_item n=6 part=touches
The MCP role's Err ending and its root-boundary reading: an Err the root error boundary takes now crosses the answer in caused with its top line and cause chain, marked boundary root; an own boundary is marked own.
@task_item n=7 part=touches
The decision that driver command-line output is read by agents, and the decision that escher-session is read by an agent: caused now gives the agent the failure's text, source and place, not just counts.
@task_item n=8 part=touches
Indirect: rules over what a driver step leaves behind it. The capture a step opens now installs a process-wide chaining panic hook once, which is a stated exception to a side-effect-free driver library.
@task_item n=9 part=touches
Indirect: any rule over what crosses the answer versus the logs. The error text now crosses in the answer and is never logged; the command span keeps its eight fields and none of them holds caused.
@task_item n=10 part=touches
A name taken: caused_past, a new answer field, and the source words app, framework and engine plus the boundary words root and own, used as values in the answer.
@task_item n=11 part=touches
Values a row may hold: 1024 bytes per text, 16 caused items, 8 chain links, 8 fields per record, and 12 names in the content-field scrub set (password added).
@task_fact n=1 slug=surface
fact: An acting verb's answer (click, type, press, advance, scroll) carries caused as one item per panic, error or log record: kind, level, source (app, framework or engine), target, message, file, line, column, the boundary that took it (root or own), its cause chain with chain_past on an error, its kept fields with fields_past on a log, and the handler's id, owner and event.
@task_fact n=2 slug=surface
fact: Caused is bounded: each text is cut at 1024 bytes and says how much was cut; panics come first, then errors, then records in order; the first 16 items are kept and the rest are counted in caused_past, one entry per kind and source, and that field is written only when it is non-empty.
@task_fact n=3 slug=surface
fact: In caused, a field whose name is in the content-field set (password among them) reads [redacted] on every source; the message and the other fields are kept as their author wrote them, and a masked control's value stays masked.
@task_fact n=4 slug=surface
fact: A caught panic whose payload the capture's hook did not see still answers a panic item, with the message <payload unread>, so no panic is lost from caused.
@task_fact n=5 slug=surface-cli
fact: The same caused text crosses the command line, the MCP role and the session socket alike, and a kept exploration check replays it and stops at the step where it changes, naming caused.
@task_item n=1 part=untouched
Colour, typography, spacing, depth, border radius and iconography rows: no colour, font, stacking, hit-test or cursor rule moved.
@task_item n=2 part=untouched
The motion and virtual-clock decisions (advance moves the one clock, the load wait, settle not waiting on animation, instant scroll, smooth scroll and overlay scrollbar timings): the clock and settling rules are unchanged; only the answer's caused grew.
@task_item n=3 part=untouched
The select list rows (overlay, open gestures, events, accessibility, list colours, placement): nothing about the select changed.
@task_item n=4 part=untouched
The escher-session exit statuses and exit codes, session error remedy, stderr usage and session errors: no exit, session error or refusal was added.
@task_item n=5 part=untouched
The snapshot, screenshot, actions line, coverage line and UI graph line contracts and their arguments: these answers are not changed.
@task_item n=6 part=untouched
The escher-session task fixtures, the shell provider, the JS window object, MountedData during render, WPT runner output and colours, and the web-spa canvas surface.
