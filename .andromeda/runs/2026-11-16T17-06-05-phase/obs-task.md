@task_prose part=chunk
An acting answer's caused (click, type, press, advance, scroll), today closed words, ids and counts alone, gains the text of what each step caused, per record, from every source: a panic's payload and location, an error's message and whole cause chain from the root error boundary, and a log record's message, target, level, location and fields from the app, the UI framework and the engine.
Every field passes the log sink's existing scrub by field name; a hidden field is shown hidden and the rest of the message still crosses; the ratified mask is unmoved.
Each message is cut at a fixed length and each answer carries a fixed count of records, both set from measured message lengths; a cut is said with its size and records past the count are counted by source.
The text rides caused on every carrier caused rides today, inside the MCP and socket answer bounds, and describe() names the new fields, the cut and the count.
Research first measures real message lengths on the stand and fixtures, and whether an error an app's own error boundary catches can be read at all.
@task_file n=1 path=packages/escher-driver/src/caused.rs mark=edits
what: the acting answer's caused — what a step caused, by handler: panic, error and log records as closed words, ids and counts — edits: each record's text, the cut and the count
@task_file n=2 path=packages/escher-telemetry/src/capture.rs mark=edits
what: the capture of log records during an acting step, counted by level and admitting no engine or escher target — edits: what it admits and keeps of each record
@task_file n=3 path=packages/dioxus-native-dom/src/boundary.rs mark=edits
what: the app's root error boundary and its RootErrorMark, holding the caught error by identity — edits: RootErrorMark, to read the error's message and source chain
@task_file n=4 path=packages/escher-telemetry/src/panic.rs
what: escher-telemetry's panic hook, which reads a panic's payload and source location (file, line, column)
@task_file n=5 path=packages/escher-telemetry/src/format.rs
what: the log sink's line format and its field scrub by field name: the content-named field set and the engine targets' safe-field rule
@task_item n=1 part=needs
caused on every acting verb (click, type, press, advance, scroll) carries per-record text: a panic's payload and its file, line and column; an error's message and each source() of its cause chain; a log record's message, target, level, bridged log.file and log.line, and fields.
@task_item n=2 part=needs
The step's log capture changes: it admits the UI framework's, the engine's and escher's targets beside the app's and keeps each record's text, where today it counts records by level alone and counts framework targets at warn and above.
@task_item n=3 part=needs
The root error boundary's mark changes from holding the error by identity to reading its message and walking its cause chain.
@task_item n=4 part=needs
The panic hook's payload and location reach caused as the hook already reads them; the hook still chains to the previous hook.
@task_item n=5 part=needs
Every field of every record passes the sink's existing scrub by field name (the content-named set for escher targets, the safe-field rule for engine targets), applied at that one scrub and not re-invented at call sites; a hidden field crosses as its name and the redaction marker, and the rest of the message crosses whole, never blanked.
@task_item n=6 part=needs
The ratified mask stays: a password's or a file input's value, and the text a caller typed, never cross in caused.
@task_item n=7 part=needs
The bounds: one fixed per-message cut and one fixed per-answer record count, both numbers set from the measured lengths so the usual message arrives whole; a cut message says it was cut and by how much; records past the count are counted by source.
@task_item n=8 part=needs
The carriers: the text rides caused in the in-process Outcome, the command line's JSON line on escher-session stdout, the session socket's call answer, the MCP tool result, and a kept check's recorded answer, replayed and kept whole with its answer digest; the MCP result bound of 1,048,576 bytes and the socket answer bound hold over it.
@task_item n=9 part=needs
describe()'s self-description names caused's new fields, the meaning of the cut and of the count, and says plainly which error boundaries are not read.
@task_item n=10 part=needs
The log sink's lines and the spans are unchanged: no message, payload or field value of a caused record reaches a log line or a span field it does not already reach; the command and tool_call spans keep their fixed closed fields, and the text crosses only as the command's answer.
@task_item n=11 part=needs
No new verb, carrier, exit code, refusal cause, session error, socket or listener is added.
@task_item n=12 part=needs
The rules that an answer, a session log or a recording carries no page or app content are widened for caused's text alone: error text is page- and app-authored content crossing onto the ratified carriers, owed its security-plan and architecture amendments at the wrap.
@task_item n=13 part=needs
Research measures real message lengths on the stand and fixtures: a panic with its location, an error with its source chain, and the engine's and framework's records an acting step causes on the stand's tasks; the distribution is read before the cut and the count are set.
@task_item n=14 part=needs
Research measures whether the error an app's own error boundary (an ErrorBoundary inside the app, not the root one) catches can be read by the driver at all, and with what; the result states what is read and what is not, and an unread boundary is never silent.
@task_item n=15 part=needs
Acceptance: a fixture with a real failure (a panic in a real code path, an Err with a cause chain, a framework or engine record) whose answer alone tells what failed and where; the test asserts the message, the source location and the cause chain are in the answer, not only that the fields are present.
@task_item n=16 part=needs
Acceptance: a record with a secret-named field crosses with that field hidden and the rest of its text whole; a message past the cut arrives cut with the cut said; records past the count are counted by source.
@task_item n=17 part=needs
Acceptance runs in both layout modes, and in process, through the command line and over MCP.
@task_item n=18 part=needs
Not taken: the whole unscrubbed text, and the scrub with no cut.
@task_standing n=1
convention: each new check is silent on success and shares its modules with the existing checks
home: 3-observability-harness-contract
@task_standing n=2
convention: each check that asserts an empty stderr unsets RUST_LOG
home: 3-observability-harness-contract
@task_standing n=3
convention: a failing log check names a kind and a count, never the content it read
home: 3-observability-harness-contract
@task_standing n=4
convention: answer-bearing types are never logged, in tests as in the code
home: 3-observability-harness-contract
@task_standing n=5
convention: a committed kept check or fixture holds no secret or private data
home: ci-integration · 9-ci-integration
@task_item n=1 part=not_needed
Metric emission, metrics exporters and the WPT scores' publishing: the chunk adds no counter, gauge, histogram or published figure.
@task_item n=2 part=not_needed
The cold-agent pipe and the agent-run harness: their events, verdict fields and reasons, counted verbs, transcripts and cleanup; the chunk changes neither harness.
@task_item n=3 part=not_needed
The page script error surfaces: the window error event, window.onerror, the script error source labels and the WPT report messages; the chunk carries Rust-side panics, errors and log records, not a page's script errors.
@task_item n=4 part=not_needed
The snapshot, graph, actions and shot answers and the select, optgroup and form-entry rows: their content and shape are untouched; only caused changes.
@task_item n=5 part=not_needed
The coverage role and its coverage line, and the session lifecycle's idle expiry: neither is touched.
@task_item n=6 part=not_needed
The WASM console panic hook: the chunk's panics are read on the native headless build.
@task_item n=7 part=not_needed
CI platform, CI artifact uploads and the package-install diagnostic line: the chunk adds no CI job or artifact.
@task_item n=8 part=not_needed
The failed-stylesheet pending set and file: loads from non-file documents: unrelated error handling in the engine's loading.
