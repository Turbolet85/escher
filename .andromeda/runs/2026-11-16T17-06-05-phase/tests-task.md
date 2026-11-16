@task_prose part=chunk
An acting answer's caused (click, type, press, advance, scroll), today closed words, ids and counts alone, gains the text of each record the step caused, from every source: a panic's payload and source location, a root-boundary error's message and whole cause chain, and a log record's message, target, level, location and fields from the app, the UI framework and the engine.
Every field passes the log sink's existing scrub by field name, a hidden field shown hidden while the rest of the message crosses; each message is cut at a fixed length and an answer holds a fixed count of records, a cut said with its size and the records past the count counted by source.
The text rides every carrier caused rides today, within the MCP result bound and the socket answer bound, and the self-description names the new fields and the meanings of the cut and the count.
Research measures first the real message lengths on the stand and fixtures, to set the cut and the count, and whether an error an app's own error boundary catches can be read by the driver at all.
Acceptance: a fixture with a real failure whose answer alone says what failed and where, the scrub, the bounds, in both layout modes, in process, through the command line and over MCP.
@task_file n=1 path=packages/escher-driver/src/caused.rs mark=edits
what: the driver's caused record of an acting answer, the panic, error or log its step caused by handler — edits: caused, gaining each record's text, the cut and the count past the bound
@task_file n=2 path=packages/escher-telemetry/src/panic.rs mark=edits
what: the panic hook, reading a panic's payload and its source location (file, line, column)
@task_file n=3 path=packages/dioxus-native-dom/src/boundary.rs mark=edits
what: the app's root error boundary and its mark of the error it took — edits: RootErrorMark, reading the error's message and its cause chain
@task_file n=4 path=packages/escher-telemetry/src/capture.rs mark=edits
what: the log capture of the records an acting step caused, today counting records by level alone and admitting no engine or escher target — edits: the capture, keeping each record's message, target, level, location and fields
@task_file n=5 path=packages/escher-telemetry/src/format.rs
what: the log sink's format and its scrub by field name, the content field names and the engine's safe-field rule
@task_item n=1 part=needs
The acting call result's caused, per record and per source: a panic's payload with its file, line and column; an error's message with each source in its cause chain; a log record's message, target, level, location and fields.
@task_item n=2 part=needs
The root error boundary's taken error read for its message and cause chain, where today it is held by identity and read for nothing.
@task_item n=3 part=needs
The log capture behind caused admits the app's, the UI framework's and the engine's records, engine and escher targets included, while the log sink's own target allowlist and the lines it writes stay as they are.
@task_item n=4 part=needs
The scrub by field name over every field of every record in caused: a field the content field names or the safe-field rule hide is shown by its name and the redaction marker, and the rest of the message still crosses, never blanked whole.
@task_item n=5 part=needs
The ratified mask is unmoved: a typed password's or a file input's value never crosses in caused text, on any carrier.
@task_item n=6 part=needs
The message cut: each message cut at a fixed length set from the measured lengths, a cut message saying it was cut and by how much; never a silent cut.
@task_item n=7 part=needs
The record count: an answer holds at most a fixed count of records, set from measurement, the records past it counted by source.
@task_item n=8 part=needs
Research measures real message lengths on the stand and fixtures, a panic with its location, an error with its source chain, the engine's and the framework's records a stand task's acting step causes, before the cut and the count are fixed.
@task_item n=9 part=needs
Research reads whether an error an app's own error boundary, not the root one, catches can be read by the driver, and with what; an unread boundary is said in the tool's own help, never silent.
@task_item n=10 part=needs
The carriers of caused: the in-process Outcome, the command line's JSON line, the session socket's call answer, the MCP tool result, and a kept check recorded and replayed with its answer kept whole, all carrying the same text.
@task_item n=11 part=needs
The answer bounds hold over the new text: the MCP tool result bound of 1,048,576 bytes and the session wire's answer bound, an oversize answer staying the existing oversize outcome.
@task_item n=12 part=needs
The help: the self-description and the MCP and command-line help name the new fields of caused and what the cut and the count mean, held in parity across surfaces.
@task_item n=13 part=needs
Acceptance on a fixture with a real failure, a panic in a real code path, an Err with a cause chain, a framework or engine record: the test asserts the message, the source location and the cause chain are in the answer, not only that the fields are present.
@task_item n=14 part=needs
Acceptance of the scrub: a record with a secret-named field crosses with that field hidden and the rest of its text whole; of the bounds: a message past the cut arrives cut with the cut said, records past the count counted by source.
@task_item n=15 part=needs
The acceptance runs in both layout modes and on every surface: in process, through the command line's own binary, and over MCP.
@task_item n=16 part=needs
No new verb, carrier, exit code, refusal cause, session error, socket or listener: the existing sets, their order and their counts stay as they are.
@task_item n=17 part=needs
The log sink's lines are unchanged: no message, payload or field value reaches a log line, stderr line or span field; the command span and tool_call span keep their fixed fields, the driver log line keeps fixed words and counts, and the text crosses only as the command's answer.
@task_item n=18 part=needs
A root-boundary Err fixture runs on its own session, the error the boundary took not leaking into another check's answer.
@task_item n=19 part=needs
Committed recorded checks whose kept answers carry caused stay byte-equal to the binary's recording and hold no private data, re-recorded where the answer's shape grows.
@task_item n=20 part=needs
Test failures over caused text print no argument value, and a test reading the session host's or MCP child's output finds no screen content or error text on stderr.
@task_standing n=1
convention: each new behaviour gets its own integration test target, one test target per file, shared helpers in a shared module that is no target
home: test-strategy
@task_standing n=2
convention: each new stand check runs in both layout modes, on the bundled font, with no skip
home: test-scope-summary
@task_standing n=3
convention: each new unit test sits in an inline test module of its source file
home: unit-test-strategy
@task_standing n=4
convention: each new test is controlled red-before-green, and a table-driven test asserts its row count
home: unit-test-strategy
@task_standing n=5
convention: each new fixture or test datum is stated in code, nothing seeded, the stand booting offline with hand-written stand-ins
home: test-data-fixtures
@task_standing n=6
convention: each new session check owns its host, is unix-gated where it uses the socket, and reads no sleep or wall clock, time moved by advance alone
home: integration-test-strategy
@task_standing n=7
convention: each new command-line check runs the package's own binary, and each MCP check runs the role as a child and in process
home: e2e-test-strategy
@task_item n=1 part=not_needed
WPT conformance rows, the runner, its suites, expectations, status set and report: the chunk changes no rendering or conformance result.
@task_item n=2 part=not_needed
Text contrast, lint defects and lint findings: the chunk reads no colour or geometry of the screen.
@task_item n=3 part=not_needed
Screenshot, pixel and PNG rows, the shot's determinism and fonts-unavailable refusal: caused text takes no shot.
@task_item n=4 part=not_needed
The cold-agent pipe, its verbs, events, verdict, exit codes and isolation: no cold-agent run is part of this chunk.
@task_item n=5 part=not_needed
Agent-run verbs, selections, events and exit codes: the chunk adds no agent-run selection or event.
@task_item n=6 part=not_needed
Handler coverage, its coverage line and limits: the chunk changes no listener read or replay count.
@task_item n=7 part=not_needed
Layout, painting and engine-behaviour rows: buffered render, select list painting, scrollbar thumbs and fade, the open pseudo-class, password caret, incremental layout oracle.
@task_item n=8 part=not_needed
The graph verb, its answer and reserved owner-key id form, and snapshot text budget and part rows: the chunk changes no reading verb's answer.
@task_item n=9 part=not_needed
Session state directory length bound, idle expiry and owner-only state files: the chunk adds no state file or lifecycle step.
@task_item n=10 part=not_needed
Upstream-tree rows, the blitz-tests manifest, the upstream buffered-render and mounted-flush tests, and the blitz-dom request constructor: the chunk takes nothing from another tree.
