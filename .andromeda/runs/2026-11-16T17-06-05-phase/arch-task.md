@task_prose part=chunk
An acting answer's caused (click, type, press, advance, scroll) gains the text of each record its step caused, from every source: a panic's payload and source location, an error's message and cause chain from the app's root error boundary, and a log record's message, target, level, location and fields from the app, the UI framework and the engine.
Every field of every record passes the telemetry sink's existing scrub by field name; a hidden field shows its name and the redaction marker and the rest of the message still crosses; password and file input values never cross.
Each message is cut at a fixed length and an answer carries at most a fixed count of records, both set from measured message lengths; a cut says it was cut and by how much, and records past the count are counted by source.
The text rides caused on every carrier caused already rides (in-process Outcome, command line JSON line, session socket call, MCP tool result, kept check), inside the MCP result bound and the socket answer bound; describe() names the new fields and the cut and count meanings.
Research first measures real message lengths on the stand and fixtures, and whether the error an app's own error boundary catches can be read by the driver at all; an unread boundary is stated in the tool's own help.
@task_file n=1 path=packages/escher-driver/src/caused.rs mark=edits
what: the acting verb results' caused field — what an acting step caused, by handler, today closed words, ids and counts alone — edits: the caused record, gaining per-record text, the cut marker and the past-count tally by source
@task_file n=2 path=packages/escher-telemetry/src/capture.rs mark=edits
what: the capture's process-wide dispatcher pair that collects the log records a step causes — edits: the capture, which today counts records by level alone and admits no engine or escher target
@task_file n=3 path=packages/dioxus-native-dom/src/boundary.rs mark=edits
what: the dioxus-native-dom public API's root_error_mark and RootErrorMark, the app's root error boundary — edits: RootErrorMark, which today holds the error by identity and reads nothing of it
@task_file n=4 path=packages/escher-telemetry/src/panic.rs
what: the escher-telemetry bootstrap's panic hook and the telemetry panic event, which read a panic's payload and source location
@task_file n=5 path=packages/escher-telemetry/src/format.rs
what: the telemetry scrub on the log sink — the content field names and the engine's safe-field rule by which every printed field is scrubbed
@task_item n=1 part=needs
The acting verb results' caused contract changes: each caused record gains its message text, source location, cause chain, target, level and fields, a cut marker and amount, and a past-count tally by source.
@task_item n=2 part=needs
The answer JSON's outcome and its string escaping: the new text fields are written by the driver's own hand-written JSON writer, with no serde_json types on the driver's public API.
@task_item n=3 part=needs
The root error boundary mark (RootErrorMark, root_error_mark) changes from holding an error by identity to reading its message and each source() in its cause chain.
@task_item n=4 part=needs
The panic event and panic hook: a panic's payload and its file, line and column reach caused as the hook already reads them.
@task_item n=5 part=needs
The capture's dispatcher pair widens from counting records by level to admitting the app's, the UI framework's and the engine's records with message, target, level, log.file, log.line and fields; the telemetry target filter and prefix sets bear on which targets it admits.
@task_item n=6 part=needs
The telemetry scrub (content span fields, engine targets, escher targets, other targets, the engine's safe-field rule) is applied to every field of every caused record, a hidden field shown by its name and the redaction marker, the rest of the message never blanked.
@task_item n=7 part=needs
Masking holds: password and file input values never cross on caused, through is_masked_input and the kept check's masking.
@task_item n=8 part=needs
The fixed message cut and fixed record count are constants set from measured lengths; a cut is never silent, and records past the count are counted by source.
@task_item n=9 part=needs
Measured limits stated where an agent reads the tool: describe()'s one self-description for command line and MCP names caused's new fields, the cut and the count, and says which error boundaries are not read.
@task_item n=10 part=needs
Carriers carry caused unchanged in kind: the in-process Outcome, the escher-session command line answer line, the session socket's call and its answer bound, the MCP tool result and its result bound of 1,048,576 bytes; an oversize answer stays inside the existing ok oversize and answer-too-large paths.
@task_item n=11 part=needs
Kept checks: a recorded check keeps the answer with its caused text whole, under the kept check file's size bounds and digest, and replay compares it through the one run path.
@task_item n=12 part=needs
Logs stay content-free: the driver logs no content, the command span and the tool_call span keep their fixed fields, and no message, payload or field value reaches a log line or span field; the text crosses only as the command's answer.
@task_item n=13 part=needs
The new text is page- and app-authored content crossing the ratified carriers, a boundary widening beside the program's fixed text on carriers, owed its architecture amendment at the wrap.
@task_item n=14 part=needs
Research measures real message lengths on the seven_guis headless stand and fixtures (a panic with location, an error with a source chain, engine and framework records an acting step causes), with the stand booting offline and its readings identical across CI operating systems.
@task_item n=15 part=needs
Research reads whether an ErrorBoundary inside the app can be read by the driver, beside the root boundary, through the Dioxus document's handle_ui_event and poll and the reached record by handler.
@task_item n=16 part=needs
Acceptance fixture with a real failure (a panic in a real code path, an Err with a cause chain, a framework or engine record): the test asserts the message, the source location and the cause chain are in the answer, not only that fields are present.
@task_item n=17 part=needs
Acceptance tests for the scrub (a secret-named field hidden, the rest whole) and for the bounds (a message cut with the cut said, records past the count counted by source).
@task_item n=18 part=needs
Acceptance runs in both layout modes, in process, through the command line and over MCP.
@task_item n=19 part=needs
No new verb, carrier, exit status, refusal cause, session error, socket or listener: the validated verb set, the eight refusal causes, the session error set and the one local Unix listener stay as they are.
@task_standing n=1
convention: each new driver scenario test runs in both layout modes
home: conventions
@task_standing n=2
convention: each new test asserts its fixture's condition first and has a descriptive name, one behavior per integration test file
home: conventions
@task_standing n=3
convention: each new constant carries its rationale in a doc comment, and each new public item and module is documented, rustdoc warnings denied
home: conventions
@task_standing n=4
convention: each new fixture uses numbered cases and relative URLs, and a captured fixture is kept as captured
home: conventions
@task_standing n=5
convention: each new expect or unwrap carries a justification and each new error type is an enum
home: conventions
@task_standing n=6
convention: each new driver unit test lives in a path child of its module, at the end of the file
home: existing-scopes · conventions
@task_item n=1 part=not_needed
Layout, style, paint and text-shaping engine rows (Taffy, Stylo, Parley, scrollbars, transforms, tables, writing modes): the chunk changes no engine layout or painting.
@task_item n=2 part=not_needed
The select list, form controls, range inputs and their default actions and keyboard models: no control behaviour or event dispatch order changes.
@task_item n=3 part=not_needed
The WPT runner, its result kinds, statuses, outputs and CI artifacts: the chunk adds no WPT test and reads no WPT result.
@task_item n=4 part=not_needed
The accessibility tree, its ARIA states and the AccessKit adapter: caused text does not come from or change the accessibility tree.
@task_item n=5 part=not_needed
The snapshot, its node keys, text form and diff, the UI graph and the actions verb's answer: their fields are unchanged; only acting answers' caused gains text.
@task_item n=6 part=not_needed
The lint verb, its reason keys and findings, and the screenshot verb and its pixels: neither carries caused.
@task_item n=7 part=not_needed
The browser app, its history store, about pages and persistence, and the rdme and example CLIs: none is a driver carrier.
@task_item n=8 part=not_needed
Networking, script fetching and the Boa script globals, and the HTTP cache: the chunk adds no fetch and no script surface.
@task_item n=9 part=not_needed
The cold-agent and agent-run scripts, their events, verdicts and exit codes: no run script changes.
@task_item n=10 part=not_needed
Upstream sync and cherry-pick rows and upstream behaviour ratification: the chunk takes nothing from another tree.
@task_item n=11 part=not_needed
Stable element id tiers and their persistence: ids in caused stay as they are.
