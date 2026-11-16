@task_prose part=chunk
An acting answer's caused item stops reading the field named error as [redacted]: the per-step capture gets a scrub set of its own, the sink's content set without error.
Every other content name stays hidden in the answer, and the log sink's stderr lines, its content set and its two readers of that set do not change.
The tool's self-description of caused (its result-field help, served by help and as the MCP tools' descriptions) and the caused module's docs are made to say that error crosses as its author wrote it.
It proves the change with a real failure read from caused alone and re-asserts, as a control, that the sink's line still prints error=[redacted].
@task_file n=1 path=packages/escher-telemetry/src/capture.rs mark=edits
what: escher-telemetry's per-step capture of log records for the answer's caused item, which judges each kept field by a scrub set by field name — edits: Values::keep, the KeptField doc, the answer's own scrub set beside the sink's, the capture unit tests
@task_file n=2 path=packages/escher-telemetry/src/format.rs mark=edits
what: escher's sink log line format and its field scrub, the content-named scrub set and the event and span-field rules that read it — edits: its unit test pinning the sink's error rule under an escher_* target
@task_file n=3 path=packages/escher-driver/src/schema.rs mark=edits
what: the driver's command schema, the result-field help served by help and as the MCP tools' descriptions — edits: caused's result-field help
@task_file n=4 path=packages/escher-driver/src/caused.rs mark=edits
what: the driver's caused item of an acting answer, the records a step caused with their fields — edits: its module doc, the fields doc
@task_file n=5 path=tests/blitz-tests/tests/stand_act_error_text.rs mark=edits
what: the stand's error-text check of an acting answer's caused item, holding the founder check that installs escher's sink over an in-memory writer
@task_item n=1 part=needs
The caused item of an acting answer keeps the text of a record's field named exactly error: the behaviour this chunk changes, on every carrier of caused — the command line, the socket's call, MCP and a kept check's kept answer.
@task_item n=2 part=needs
The answer's scrub set is its own: the sink's content-named set without error, living beside its one reader in the capture; scrubbing by field name in the sink and the capture is widened for this one name in the capture only.
@task_item n=3 part=needs
Every other content name still reads [redacted] in a caused item's fields: url, href, src, html, text, value, attrs, path, request, panic.payload and password — no other field name opens.
@task_item n=4 part=needs
The sink's stderr lines are unchanged: a field named error under an escher_* target, and under every target, still prints [redacted]; the sink's content-named scrub set is unchanged and both its event rule and its closed-span field store stay on it — pinned by a sink unit test.
@task_item n=5 part=needs
The sink drops an app target's record whole and writes no line for it; on every build the app record's error text occurs zero times on the sink's lines, and only an engine record (the engine's resource-load warn, with url and error) has a sink line, written only in a build that compiled engine tracing.
@task_item n=6 part=needs
Acceptance: a real failure whose text sits in a field named error, logged from a handler the step reaches, is read from the acting answer's caused alone, and a field of another content name on the same record (url or path) still reads [redacted] there.
@task_item n=7 part=needs
The text cut of 1024 bytes, the fields kept per record and the records kept per capture are unchanged; the error text now crossing is bounded by that same cut.
@task_item n=8 part=needs
The message, location, boundary and cause chain of a caused item are untouched; they already cross whole.
@task_item n=9 part=needs
No new verb, argument, node key, cause, session error, exit status, carrier or log field; no span, span field or log line changes.
@task_item n=10 part=needs
The result-field help of caused, served by help and as the MCP tools' descriptions, and the caused module's doc and fields doc stop naming error among the names that read [redacted] and say it crosses as its author wrote it — fixed program text, no new carrier.
@task_item n=11 part=needs
The answer now carrying a failure's error text (which may hold what its author put there, such as a failed fetch's URL) is still never logged: answer-bearing types and a part's answer stay out of every log line and span.
@task_item n=12 part=needs
A kept check's kept answer carries the error text as the live answer does; the kept check file's answer and its digest follow from the answer, with no new key.
@task_item n=13 part=needs
The founder check that installs escher's sink over an in-memory writer is a named exception to the headless stand running with no sink; the new acceptance and control tests sit in the error-text check beside it.
@task_item n=14 part=needs
The plan rows that state the scrub — the escher-telemetry scrub set's error entry, scrubbing by field name in the sink and the capture, and the caused text crossing — are amended at the wrap as a boundary widening, not by this chunk's code.
@task_standing n=1
convention: each new check is silent and its shared modules print nothing on a pass
home: 3-observability-harness-contract
@task_standing n=2
convention: each new check that reads the sink's stderr or asserts it empty unsets RUST_LOG
home: 3-observability-harness-contract
@task_standing n=3
convention: a new log check's failure names a kind and a count, never the content it read
home: 3-observability-harness-contract
@task_item n=1 part=not_needed
Metric coverage and metric or exporter absences, and any telemetry backend or OTel export: the chunk emits no metric and adds no backend.
@task_item n=2 part=not_needed
The agent-run harness and cold-agent pipe rows — their events, verdict fields, counted verbs, transcripts, kept checks at rest and cleanup: the chunk touches neither harness.
@task_item n=3 part=not_needed
The page script error surface, the window error event and window.onerror, script error source labels and the WPT report message: the chunk changes no script-side error reporting.
@task_item n=4 part=not_needed
The select, optgroup, open list, snapshot, form entry, page event and state registry rows of the PII section: the chunk changes no snapshot or form content.
@task_item n=5 part=not_needed
The CI platform, artifact upload, package-install diagnostic and WPT publishing rows: the chunk adds no CI job or artifact.
@task_item n=6 part=not_needed
The recording file, session lifecycle, recording bounds and session state directory rows: the chunk changes no recording or session state.
