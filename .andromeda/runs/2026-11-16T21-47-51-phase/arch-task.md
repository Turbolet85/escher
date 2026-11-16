@task_prose part=chunk
An acting answer's caused item stops hiding a record's field named exactly error: the per-step capture gets a scrub set of its own, the log sink's content-field set without error, so that field carries the text its author wrote.
Every other content name (url, href, src, html, text, value, attrs, path, request, panic.payload, password) still reads [redacted] in the answer, and the log sink's stderr lines, its content-field set and its two readers of that set do not change.
The tool's self-description of caused (its result-field help in the driver schema, served by help and as the MCP tools' descriptions) and the caused module's docs are made to say that error crosses as written.
It proves this in the existing error-text stand check and escher-telemetry's capture and format unit tests: an app record logged with error = %e reads its text in caused, another content field on it stays [redacted], and the sink's line for an engine failure record still prints error=[redacted].
@task_file n=1 path=packages/escher-telemetry/src/capture.rs mark=edits
what: escher-telemetry capture, the per-step capture of records that feeds the acting answer's caused items, with its bounds and its unit tests — edits: Values::keep, the KeptField doc, the answer's own scrub set beside the sink's, the capture unit tests
@task_file n=2 path=packages/escher-telemetry/src/format.rs mark=edits
what: the escher telemetry sink's line format and its scrub by field name, holding the content-field set CONTENT_FIELDS, the event rule and the span-field store — edits: its unit test pinning the sink's error rule under an escher_* target
@task_file n=3 path=packages/escher-driver/src/schema.rs mark=edits
what: the driver's command and result schema, the one self-description served by help and as the MCP tools' descriptions — edits: caused's result-field help
@task_file n=4 path=packages/escher-driver/src/caused.rs mark=edits
what: the driver's caused item, the acting verb result that carries each captured record's message, location, boundary, cause chain and fields — edits: its module doc, the fields doc
@task_file n=5 path=tests/blitz-tests/tests/stand_act_error_text.rs mark=edits
what: the error-text stand check, the blitz-tests integration check of an acting answer's caused text, holding the founder check that installs escher's sink over an in-memory writer — edits: the check that a real failure's error field crosses whole while the sink line stays scrubbed
@task_item n=1 part=needs
The acting verb results' caused item: a kept field named exactly error now carries its text (still cut at the text cut) instead of [redacted]; this is a behaviour change of the answer on every carrier that carries caused — the command line, the socket's call, MCP and a kept check's kept answer.
@task_item n=2 part=needs
The answer's scrub by field name: the answer reads its own set, the sink's content-field set minus error, and every other content name of the sink's set still reads [redacted] in a caused item's fields.
@task_item n=3 part=needs
The telemetry sink's scrub stays as it is: under escher targets, engine targets and every other target a field named error still prints [redacted] on stderr; the content-field set itself, the event rule and the span-field store stay on it; the sink still drops an app target's record whole.
@task_item n=4 part=needs
escher-telemetry capture bounds stay unchanged: the 1024-byte text cut, the fields kept per record and the records kept per capture.
@task_item n=5 part=needs
The parts of a caused item other than its fields — message, location, boundary and cause chain — are untouched; they already cross whole.
@task_item n=6 part=needs
Closed sets stay closed: no new verb, argument, node key, refusal cause, session error, exit status, carrier or log field; the change rides the carriers already ratified for caused.
@task_item n=7 part=needs
The caused help text in the driver schema is fixed program text riding the ratified carriers and the one self-description shared by the command line and MCP (help and the MCP tool listing's descriptions); it must say error crosses as its author wrote it, and the capability statement held to the driver schema must not be left contradicting it.
@task_item n=8 part=needs
No content reaches a log line or span through this change: the driver logs no content, opens spans only and never fields error text; the opened text exists only in the answer.
@task_item n=9 part=needs
The price the founder accepted: error text can hold what its author put into it (a failed fetch's URL); a kept check's kept answer therefore may carry it, and kept checks stay on the owner-only medium, never logged, with their masking unchanged.
@task_item n=10 part=needs
Answer size bounds the opened text now counts against: the MCP result bound, the socket's reply and answer bound and answer-too-large; an error field is bounded by the unchanged text cut.
@task_item n=11 part=needs
The acceptance check: a real failure whose text sits in a field named error, logged with error = %e from a handler the step reaches, is read from the acting answer's caused alone; another content field on the same record (url or path) still reads [redacted]; the app record's error text occurs 0 times on the sink's lines.
@task_item n=12 part=needs
The control on the sink: the engine's Resource load failed warn (fields url, waiting_nodes, error) is written only in a build that compiled the engine's tracing (engine tracing gated per call site, the crate root's tracing-compiled flag); there its answer item reads the error text while the sink line reads error=[redacted] and url=[redacted]; on every build the sink's escher_* rule for error is pinned by escher-telemetry's own unit test.
@task_item n=13 part=needs
The founder check installs escher's sink over an in-memory writer inside a test, a named exception to the one telemetry install site per binary; the capture's process-wide dispatcher pair and capture opened before a check's own recorder still bind how that check is set up.
@task_item n=14 part=needs
Docs that state the old rule are rewritten, not left behind: the caused module doc and fields doc and the KeptField doc; rustdoc warnings are errors and the answer's new set needs its rationale stated where it is declared.
@task_item n=15 part=needs
The answer's set lives with its one reader in the capture, not in the sink's format; crate internals stay restricted, so the new set is not exposed beyond what its reader and tests need.
@task_item n=16 part=needs
The plan's statements of the scrub (the caused rows, the scrub by field name) are amended at the wrap as a boundary widening, not by this chunk's code; v030-10 already reads verified and the chunk claims no capability of its own.
@task_standing n=1
convention: Unit tests sit at the end of their source file, with descriptive test names, and each test asserts its fixture's condition first.
home: conventions
@task_standing n=2
convention: Every constant states its rationale in its doc, and module and public items carry docs.
home: conventions
@task_standing n=3
convention: expect and unwrap in new code or tests carry a justification; clippy allows stay local.
home: conventions
@task_standing n=4
convention: Stand-check helpers live in non-target modules and assertion helpers use track-caller; integration tests stay one behaviour per blitz-tests target file.
home: conventions
@task_standing n=5
convention: A stand check's readings are identical on every CI OS, the stand boots offline in a pinned environment, and the stand-repeat run holds a zero-failure budget.
home: standard-contracts
@task_item n=1 part=not_needed
Engine layout, style, paint, scrolling, hit testing and text rows: the chunk changes no engine behaviour; the engine record it reads is an existing warn.
@task_item n=2 part=not_needed
Select list, form controls, accessibility tree and ARIA rows: no control, role or state reading changes.
@task_item n=3 part=not_needed
Snapshot, node keys, snapshot diff, UI graph, actions verb and lint verb rows: the chunk widens only caused's fields, no other answer part.
@task_item n=4 part=not_needed
WPT runner, its outputs, statuses and CI artifacts: no WPT test or report is touched.
@task_item n=5 part=not_needed
Browser app rows (history, persistence, about pages, toolbar, favicon cache): the browser is not reached.
@task_item n=6 part=not_needed
Script runtime rows (Boa, script globals, script fetcher, script events): no script behaviour changes.
@task_item n=7 part=not_needed
Stable element id tiers, explore and the recording directory's layout: no id, path expansion or recording layout changes.
@task_item n=8 part=not_needed
Upstream sync, cherry-pick and upstream take rows, and dependency version pins: the chunk takes nothing from another tree and adds no dependency.
@task_item n=9 part=not_needed
Cold-agent and agent-run script contracts (events, exit codes, verdict fields, state directory): no script or verdict changes.
@task_item n=10 part=not_needed
The name register's crates, binaries and other names: no new name is added.
