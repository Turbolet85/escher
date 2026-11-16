@task_prose part=chunk
An acting answer's caused item keeps the text of a record's field named exactly error; today the per-step capture judges fields by the log sink's own content-field set, which holds error, so it reads [redacted].
The answer gets a scrub set of its own, the sink's set without error; every other content name (url, href, src, html, text, value, attrs, path, request, panic.payload, password) still reads [redacted] in a caused item.
The log sink's stderr lines, its content-field set and its two readers of that set do not change; a unit test pins the sink's error rule under an escher_* target.
The tool's self-description of caused follows: caused's result-field help in the driver's schema (served by help and as the MCP tools' descriptions) and caused's module and fields docs say error crosses as its author wrote it.
It applies on every carrier that carries caused: the command line, the socket's call, MCP and a kept check's kept answer; no new verb, argument, node key, cause, session error, exit, carrier or log field.
@task_file n=1 path=packages/escher-telemetry/src/capture.rs mark=edits
what: the per-step capture of records for the acting answer's caused item, judging which fields read [redacted] — edits: Values::keep, the KeptField doc, the answer's own scrub set beside the sink's, its capture unit tests
@task_file n=2 path=packages/escher-telemetry/src/format.rs mark=edits
what: the log sink's stderr line format and its content-field set CONTENT_FIELDS (unchanged) — edits: its unit test pinning the sink's error rule under an escher_* target
@task_file n=3 path=packages/escher-driver/src/schema.rs mark=edits
what: the driver's schema, the result-field help served by help and as the MCP tools' descriptions — edits: caused's result-field help
@task_file n=4 path=packages/escher-driver/src/caused.rs mark=edits
what: the caused item of an acting answer, its fields carried per record — edits: its module doc, the fields doc
@task_file n=5 path=tests/blitz-tests/tests/stand_act_error_text.rs mark=edits
what: the error-text stand check, holding the founder check that installs escher's sink over an in-memory writer (a named exception) — edits: the check that a real failure's error field reads its text in caused while the sink's line still prints error=[redacted] and another content field still reads [redacted]
@task_item n=1 part=needs
The escher-session actions answer: an acting answer's caused item now carries the text of a field named error, on the command line, the socket's call, MCP and a kept check's kept answer.
@task_item n=2 part=needs
Every other content-named field in a caused item still reads [redacted]: url, href, src, html, text, value, attrs, path, request, panic.payload, password.
@task_item n=3 part=needs
The escher-session help answer: caused's result-field help changes to say error crosses as its author wrote it, no longer listing it among the names that read [redacted].
@task_item n=4 part=needs
The escher-session MCP server's tools/list: the tools' descriptions are served from the same schema help and carry the changed caused wording; the result bound is unchanged.
@task_item n=5 part=needs
The escher-session recording: a kept check's kept answer (check-{n}.json) carries caused items whose error field now holds the error text.
@task_item n=6 part=needs
The host log on stderr stays as it is: a field named error still prints [redacted] under every target, including escher_* targets, and the sink drops an app target's record whole.
@task_item n=7 part=needs
The text cut (1024 bytes), the fields kept per record and the records kept per capture stay unchanged; the opened error text stays inside the existing text cut.
@task_item n=8 part=needs
The item's message, location, boundary and cause chain are untouched; they already cross whole.
@task_item n=9 part=needs
The answer's command form, exit codes, refusals and node keys gain nothing: no new verb, argument, node key, cause, session error, exit, carrier or log field.
@task_item n=10 part=needs
Acceptance check: a real failure whose text sits in a field named error, logged from a handler a step reaches, is read from the acting answer's caused alone, holding the error's text; a control shows the sink's line for an engine record (the engine's Resource load failed warn, fields url, waiting_nodes, error) still prints error=[redacted] and url=[redacted], and the app record's error text occurs 0 times on the sink's lines.
@task_item n=11 part=needs
The stand's load task and its failed fetch: a failed resource load is the real failure whose error field the check reads, and the load page is the one with a network provider.
@task_item n=12 part=needs
The stand check drives the served stand through the driver like every stand check: the shared stand boot, the pinned headless viewport, the stand's keyed controls and stable element ids.
@task_item n=13 part=needs
The opened error text may hold what its author put into it (a failed fetch's URL); that is the founder's stated price, and a field named url or path on the same record still reads [redacted].
@task_standing n=1
convention: a new or changed stand check boots the stand the way every stand check does: shared stand boot, pinned headless viewport, lints clean at boot, controls reached by their stable element ids
home: surface-desktop-native
@task_standing n=2
convention: a new or changed check of the session's answer reads it through the escher-session command line or MCP surface as the answer contracts state it, and is replayable as a session recording
home: surface-cli
@task_item n=1 part=not_needed
The ShellProvider window chrome controls (drag, maximise, minimise, decorations, close): the chunk opens no window and touches no chrome.
@task_item n=2 part=not_needed
The agent-run and cold-agent pipe contracts (usage, stdout, exits, run log, record directory, Windows pass-through) and the cold-agent decisions: no pipe or cold-agent run changes.
@task_item n=3 part=not_needed
The multi_doc and paint_bench command lines: no benchmark or multi-document tool is touched.
@task_item n=4 part=not_needed
Layout and interaction decisions of the stand's other tasks: CRUD list keyboard region, flight type select boot reading, open list rows' own bounds, into-view scrolling, visible region through clipping boxes, accent colour.
@task_item n=5 part=not_needed
Arbitrary index.html templates for a document, and the fixed document skeleton: the chunk changes no document markup.
@task_item n=6 part=not_needed
The mobile-native and web-spa surfaces and the decisions log: not measured and not reached by this chunk.
@task_item n=7 part=not_needed
The snapshot, screenshot, graph, lint and coverage answers and the served tasks other than load: none of their content changes.
