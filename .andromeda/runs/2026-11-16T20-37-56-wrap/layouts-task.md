@task_prose part=session
The session put the error text into the answer: each acting verb's caused list now holds one item per record, with its message, source, target, file, line and column, the boundary it fell to (root or own) and its cause chain, where before it held only closed words with a count per kind and level.
Every source counts: the app at every level, the framework and, new, the engine at warn and error; any field whose name is in the content-field scrub set reads [redacted], and the scrub set gains password (11 to 12 names).
Text is bounded by measurement: each text is cut at 1024 bytes and says how much was cut; 16 items, 8 chain links and 8 fields are kept, and what is past them is counted in a new caused_past field on click, type, press, advance and scroll, written only when non-empty.
The telemetry capture keeps text and installs a chaining panic hook once per process; the native document registers every error boundary so the driver reads errors from every boundary, not only the root one; six engine log sites carry their values in fields under fixed messages.
The same caused crosses every carrier: in-process session, the MCP server over stdio, the session socket to host children and kept recording checks; no new verb, refusal, exit, endpoint or environment variable.
After implement, a Windows CI red was fixed by one shared test helper that decodes an MCP tool result's text whole, so a path's backslashes match.
@task_file n=1 path=tests/blitz-tests/tests/stand_act_error_text.rs
what: the new stand check that each failure answers what failed, where and from which source, over the in-process session, the MCP server, the session socket's host children and a kept check, with a flood bound and a sink-lines check
@task_file n=2 path=tests/blitz-tests/tests/capture_panic_hook.rs
what: the new stand check that the capture's panic hook installs once and a panic outside a capture reads as before
@task_file n=3 path=tests/blitz-tests/tests/stand_act_errors.rs
what: the stand check of the root boundary's error on a session of its own and of the same caused crossing the MCP server in process
@task_file n=4 path=tests/blitz-tests/tests/session_common/mod.rs
what: the stand's shared session test helpers, now with the decoder of an MCP tool result's text
@task_file n=5 path=packages/escher-telemetry/src/capture.rs
what: the scoped telemetry capture behind each step's caused, now keeping record and panic text by source
@task_file n=6 path=packages/escher-telemetry/src/lib.rs
what: the telemetry sink's entry, now with a public subscriber builder over a writer and a level
@task_file n=7 path=packages/escher-telemetry/src/format.rs
what: the sink's line format and the content-field scrub set
@task_file n=8 path=packages/blitz-dom/src/events/pointer.rs
what: the engine's pointer events, the link click's warning
@task_file n=9 path=packages/blitz-dom/src/form.rs
what: the engine's form submission, the unimplemented-scheme warning
@task_file n=10 path=packages/blitz-dom/src/mutator.rs
what: the engine's document mutator, the iframe load warning
@task_file n=11 path=packages/blitz-dom/src/layout/mod.rs
what: the engine's layout, the text-node error log
@task_file n=12 path=packages/blitz-dom/src/lib.rs
what: the engine crate's root, now saying whether tracing is compiled in
@task_file n=13 path=packages/dioxus-native/src/assets.rs
what: the native renderer's asset loading warnings
@task_file n=14 path=packages/dioxus-native-dom/src/boundary.rs
what: the native document's error boundary reading, now a registry of every boundary with marks per boundary
@task_file n=15 path=packages/dioxus-native-dom/src/dioxus_document.rs
what: the native document, which provides the boundary registry before its first build
@task_file n=16 path=packages/dioxus-native-dom/src/lib.rs
what: the native document crate's public exports
@task_file n=17 path=packages/escher-driver/src/caused.rs
what: the driver's caused list of an acting verb's answer, its items and their bounds
@task_file n=18 path=packages/escher-driver/src/execute.rs
what: the driver's step execution and the acting verbs' outcomes
@task_file n=19 path=packages/escher-driver/src/json.rs
what: the driver's answer writer, one JSON line per answer
@task_file n=20 path=packages/escher-driver/src/schema.rs
what: the driver's verb and field schema behind help and the MCP tools list
@task_file n=21 path=packages/escher-driver/src/schema_tests.rs
what: the schema's unit tests and its field-kind table
@task_file n=22 path=packages/escher-driver/src/lib.rs
what: the driver library's public exports
@task_file n=23 path=packages/escher-driver/src/fixture_tests.rs
what: the driver's fixture unit tests of the acting verbs' outcomes
@task_file n=24 path=packages/escher-driver/src/recording_tests.rs
what: the driver's session recording unit tests
@task_file n=25 path=packages/escher-driver/src/replay.rs
what: the replay of a kept recording check against its answers
@task_item n=1 part=touches
The escher-session actions answer: every acting verb's caused list changed shape, from closed words with a count per kind and level to one item per record with message, cut, source, target, file, line, column, boundary, chain and chain_past or fields and fields_past, id, owner and event; the count member is gone.
@task_item n=2 part=touches
The escher-session actions answer gains a field, caused_past, after caused on click, type, press, advance and scroll, written only when non-empty, each entry a kind, an optional source and a count.
@task_item n=3 part=touches
The escher-session help answer and the MCP server's tools list: both read the driver's schema, whose caused help was rewritten and which gains the caused_past field on the five acting verbs.
@task_item n=4 part=touches
The escher-session MCP server's result bound: an answer line now carries text, bounded by a 1024-byte cut per text, 16 items, 8 chain links and 8 fields; the flood check holds a line under 1,048,576 bytes; any row holding a bound on the result or line size is read against these values.
@task_item n=5 part=touches
The rule that caller input is never written back: a caused message now holds what its author wrote and fields are scrubbed by name only (password and value read [redacted]); a typed password and a raw href never appear in the answer, and a masked control's value stays masked.
@task_item n=6 part=touches
Session recording as a replayable check and the recording's check file: a kept check now holds and replays the caused text, and a changed cut makes the check stop at its step naming caused.
@task_item n=7 part=touches
The MCP server local over stdio and the cold agent driving through the driver's MCP server: the agent now reads the error text, location, chain and source in caused through the tool result; the tool result carries the answer line as a JSON string, decoded whole.
@task_item n=8 part=touches
Session roles sharing the stand boot: host children of the session socket carry the same caused as the in-process session; the new check starts two ignored host children of its own.
@task_item n=9 part=touches
The escher-session host log and its service name: the new stand check installs escher's own sink in process over an in-memory writer at trace, an exception to the headless stand having no sink; no sink line holds the error text.
@task_item n=10 part=touches
The root boundary's error read on a session of its own: the driver now reads errors from every error boundary, root and own, by a registry the native document provides before its first build, and the root error's text now crosses in caused.
@task_item n=11 part=touches
The exit codes and refusals of escher-session: unchanged, no new exit, refusal cause or session error; a row listing what an acting verb's answer can hold is touched only through caused and caused_past.
@task_fact n=1 slug=surface-cli
fact: Every acting verb's answer carries caused as one item per panic, error and log record, each with its message text, source (app, framework or engine), target, location, boundary and cause chain or kept fields, a content-named field reading [redacted].
@task_fact n=2 slug=surface-cli
fact: Each text in caused is cut at 1024 bytes and says how many bytes were cut; caused keeps 16 items, 8 chain links and 8 fields an item, and caused_past counts the rest by kind and source on click, type, press, advance and scroll, written only when non-empty.
@task_fact n=3 slug=surface-cli
fact: A check comparing an MCP tool result against the in-process answer decodes the result text whole, as a JSON string, so a Windows path's backslashes match.
@task_item n=1 part=untouched
The desktop-native stand's element ids, keyboard regions, viewport, accent colour, skeleton, accessibility attributes and boot lints: the session added no stand control and moved no id or layout.
@task_item n=2 part=untouched
The window chrome controls of the shell provider: no window control or decoration changed.
@task_item n=3 part=untouched
The agent-run, cold-agent, multi_doc and paint_bench command lines, their usage, stdout, logs and exits: none changed.
@task_item n=4 part=untouched
The escher-session served tasks, the coverage role, graph, lint, screenshot and snapshot answers and the command form: none changed; only the acting verbs' answers moved.
@task_item n=5 part=untouched
The MCP server's protocol revisions, its end and its screenshot result: unchanged.
@task_item n=6 part=untouched
The surfaces not yet measured (mobile-native, web-spa) and the absence of arbitrary index.html templates: the session reaches none of them.
