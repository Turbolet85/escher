@task_prose part=chunk
An acting answer's caused (click, type, press, advance, scroll) gains the text of each record its step caused, from every source: a panic's payload and location, an error's message and cause chain, and a log record's message, target, level, location and fields from the app, the UI framework and the engine.
Every field passes the log sink's existing scrub by field name, and a hidden field is shown as hidden while the rest of the message still crosses. Each message is cut at a fixed length and each answer carries a fixed count of records. A cut is always said, and the records past the count are counted by source.
The text rides every carrier caused already rides, and the help names the new fields.
Research first measures real message lengths on the stand and fixtures, and that sets the cut and the count. It also measures whether the error an app's own error boundary catches can be read at all.
@task_file n=1 path=packages/escher-driver/src/caused.rs mark=edits
what: the acting answer's caused record, today closed words, ids and counts alone — edits: caused, which gains per-record text, the cut marker and the overflow count by source
@task_file n=2 path=packages/escher-telemetry/src/panic.rs
what: the panic hook that reads a panic's payload and its source location (file, line, column)
@task_file n=3 path=packages/dioxus-native-dom/src/boundary.rs mark=edits
what: the app's root error boundary — edits: RootErrorMark, which today holds the error by identity and now reads its message and cause chain
@task_file n=4 path=packages/escher-telemetry/src/capture.rs mark=edits
what: the log record capture that counts records by level alone and admits no engine or escher target — edits: the capture, which now keeps the message, target, level, location and fields and admits engine and framework targets
@task_file n=5 path=packages/escher-telemetry/src/format.rs
what: the log sink's scrub by field name (the content field names, the engine's safe-field rule) that every crossing field passes
@task_item n=1 part=needs
The caused record of an acting answer (click, type, press, advance, scroll) carries per-record text from every source: panic, error boundary, and log records of the app, the UI framework and the engine.
@task_item n=2 part=needs
A panic record carries its payload and its source location (file, line, column), as the panic hook reads them.
@task_item n=3 part=needs
An error the root error boundary took carries its message and its whole cause chain (each source in turn), not its top line alone.
@task_item n=4 part=needs
A log record carries its message, its target (which source said it), its level, the location a bridged log record carries, and its fields. The capture admits engine and escher targets beside the app.
@task_item n=5 part=needs
The scrub: every field of every record passes the log sink's scrub by field name. A hidden field shows its name and the redaction marker, and the rest of the message still crosses, so a message is never blanked because one of its fields is.
@task_item n=6 part=needs
The ratified mask is unmoved: a password's or a file input's value never crosses in the error text.
@task_item n=7 part=needs
Indirect, through the caller-input rule: the new text in caused must not become a path by which caller input is written back beyond what the scrub and the mask allow.
@task_item n=8 part=needs
The bounds: each message is cut at a fixed length and an answer carries at most a fixed count of records. Both numbers come from the measured lengths, so the usual message arrives whole.
@task_item n=9 part=needs
A cut is never silent: a cut message says it was cut and by how much, and the records past the count are counted by source.
@task_item n=10 part=needs
Every carrier carries the text: the in-process Outcome, the command line's JSON line, the session socket's call, an MCP tool result, and a kept check, which is recorded, replayed and keeps its answer whole.
@task_item n=11 part=needs
The MCP result bound (1,048,576 bytes) and the socket answer bound still hold with the text inside caused.
@task_item n=12 part=needs
The help: describe()'s self-description names the new fields of caused and what the cut and the count mean.
@task_item n=13 part=needs
An error boundary that cannot be read (an app's own ErrorBoundary inside the app) is said by the tool's own help and is never silent. The research states what is read and what is not.
@task_item n=14 part=needs
Research, before the bounds are set: it measures real message lengths on the stand's served tasks and on fixtures (a panic with its location, an error with its source chain, the engine's and the framework's records an acting step causes) and reads their distribution.
@task_item n=15 part=needs
Acceptance: a fixture with a real failure (a panic in a real code path, an Err with a cause chain, a framework or engine record). It asserts that the message, the source location and the cause chain are in the answer, not only that the fields are present.
@task_item n=16 part=needs
Acceptance of the scrub and the bounds: a record with a secret-named field crosses with that field hidden and its other text whole. A message longer than the cut arrives cut, with the cut said. Records past the count are counted by source.
@task_item n=17 part=needs
The acceptance runs in both layout modes: in process, through the command line and over MCP.
@task_item n=18 part=needs
No new verb, carrier, exit, refusal cause, session error, socket or listener: the existing exit codes, refusals and MCP tools list stay as they are.
@task_item n=19 part=needs
The log sink's own lines are unchanged: no message, payload or field value reaches a log line or span field it did not already reach. The command and tool_call spans keep their fixed fields, and the text crosses only as the command's answer.
@task_item n=20 part=needs
Not taken: the whole unscrubbed text, and the scrub with no cut.
@task_standing n=1
convention: each new acceptance test of a session answer runs in both layout modes and on each carrier it names: in process, the command line and MCP
home: surface-cli
@task_standing n=2
convention: each new fixture control a test drives is keyed and carries its id as an attribute alone, and the fixture lints clean at boot
home: surface-desktop-native
@task_item n=1 part=not_needed
ShellProvider window chrome controls (drag, maximize, minimize, close, decorations): the chunk touches no window chrome.
@task_item n=2 part=not_needed
The agent-run.sh test contract and the cold-agent.sh run pipe contracts and decisions (usage, stdout, exits, record dir, Windows pass-through, prompt and build rules): the chunk changes neither script.
@task_item n=3 part=not_needed
The multi_doc and paint_bench command lines (arguments, environment variables, backend, stdout): these are benchmark tools the chunk does not reach.
@task_item n=4 part=not_needed
Layout geometry decisions (scrolling into view, visible region through clipping boxes, open list rows' bounds, the pinned viewport) and the stand's accent colour: the chunk changes no layout or paint.
@task_item n=5 part=not_needed
The screenshot answer and result, the snapshot answer and its part option, and the graph, lint and coverage role answers: none of these carries caused.
@task_item n=6 part=not_needed
Arbitrary index.html templates and the fixed document skeleton with its main mount: the chunk changes no document shape.
