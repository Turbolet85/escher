@task_prose part=chunk
An acting answer's caused (click, type, press, advance, scroll) gains the text of each record the step caused: a panic's payload and source location, an error's message and whole cause chain from the app's root error boundary, and a log record's message, target, level, location and fields from the app, the UI framework and the engine.
Every field passes the log sink's existing scrub by field name; a hidden field shows its name and the redaction marker, the rest of the message still crosses, and the ratified mask of password and file input values is unmoved.
Each message is cut at a fixed length and an answer carries a fixed count of records; a cut says it was cut and by how much, and records past the count are counted by source.
The text rides every carrier caused rides today (in-process Outcome, command line JSON line, session socket call, MCP tool result, kept check) inside the existing MCP and socket answer bounds, and describe() names the new fields and the cut and count meanings.
Research first measures real message lengths on the stand and fixtures to set both bounds, and whether an app's own error boundaries can be read at all; an unread boundary is said in the tool's help.
@task_file n=1 path=packages/escher-driver/src/caused.rs mark=edits
what: the acting answer's caused, the panic, error or log its step caused, by handler — edits: caused, which gains each record's text, the cut and the count past the bound
@task_file n=2 path=packages/escher-telemetry/src/panic.rs
what: the panic hook, which reads a panic's payload and its source location (file, line, column)
@task_file n=3 path=packages/dioxus-native-dom/src/boundary.rs
what: the app's root error boundary and its RootErrorMark, which holds the error the boundary took
@task_file n=4 path=packages/escher-telemetry/src/capture.rs
what: the capture of log records a step causes, today counting records by level and admitting no engine or escher target
@task_file n=5 path=packages/escher-telemetry/src/format.rs
what: the log sink's scrub by field name: the content field names and the engine's safe-field rule
@task_item n=1 part=needs
The acting verbs' answers (click, type, press, advance) whose caused gains per-record text: a new field on each verb's answer, with no new verb, refusal cause or exit.
@task_item n=2 part=needs
The escher-session MCP role's other tool results: an acting tool's result now carries the error text inside the 1,048,576-byte MCP result bound, and a kept check records and replays that answer whole.
@task_item n=3 part=needs
The escher-session command line output on stdout: the one JSON line of an acting command carries the new text of caused, and the session socket's call answer stays inside its answer bound.
@task_item n=4 part=needs
The escher-session help and describe() self-description: it names the new fields of caused, what a cut message says, what the count of records past the bound means, and states which error boundaries are read and which are not.
@task_item n=5 part=needs
Driver output is read by agents, not people: the new text is structured so an agent reads from the answer alone what failed and where, with no colour or terminal formatting.
@task_item n=6 part=needs
The exit statuses and exit codes stay as they are: a caused panic, error or log record changes no exit status, and none is added.
@task_item n=7 part=needs
The session error object, its remedy and the stderr session and usage errors stay unchanged: the error text crosses only as the command's answer, never as a session error.
@task_item n=8 part=needs
The bounds: one fixed per-message cut length and one fixed per-answer record count, both set from the measured distribution of real message lengths so the usual message arrives whole; a cut is never silent and overflow is counted by source.
@task_item n=9 part=needs
The scrub carve-out: a message is never blanked because one field in it is hidden; the hidden field shows its name and the redaction marker; a password's or a file input's value never crosses.
@task_item n=10 part=needs
The log sink's own lines are unchanged: no message, payload or field value reaches a log line or a span field that does not already, and the command and tool_call spans keep their fixed fields.
@task_item n=11 part=needs
Capture widens to the UI framework's and the engine's log targets beside the app's, carrying each record's target, level, bridged location (log.file, log.line) and fields.
@task_item n=12 part=needs
The error a root error boundary takes is read for its message and every source() in its cause chain, not its top line alone.
@task_item n=13 part=needs
The app's own error boundaries inside the app are measured for whether the driver can read the error they catch; what is not read is said in help, never silent.
@task_item n=14 part=needs
The acceptance fixture with a real failure (a panic in a real code path, an Err with a cause chain, a framework or engine record) asserts the message, the source location and the cause chain are in the answer, in both layout modes, in process, through the command line and over MCP.
@task_item n=15 part=needs
The stand's tasks the research measures on (counter, crud, cells, flight-booker, timer, load): the engine and framework records an acting step causes on them give the length distribution.
@task_standing n=1
convention: each new answer field on the escher-session command line and MCP output is written for an agent reader, in the one stdout JSON line, with no colour and no terminal test
home: surface · surface-cli
@task_item n=1 part=not_needed
Colour palette, typography, spacing, border radius and depth rows: the chunk paints nothing and changes no colour, font, spacing or stacking.
@task_item n=2 part=not_needed
Select list rows, events, accessibility and painting: the chunk touches no select element or its open list.
@task_item n=3 part=not_needed
The WPT runner's output and result colours: the chunk changes no conformance runner output.
@task_item n=4 part=not_needed
Shell provider, cursor, favicon and window controls (maximize, minimize, decorations, drag): the chunk touches no window or shell behaviour.
@task_item n=5 part=not_needed
The screenshot and snapshot verbs and their results, and font availability refusals: the chunk changes no read verb, only acting answers.
@task_item n=6 part=not_needed
The actions, coverage and UI graph lines and their arguments: the chunk adds nothing to those read answers.
@task_item n=7 part=not_needed
The JS window object, the viewport's colour scheme and the dioxus-native configuration values: the chunk changes no page-facing value or config.
@task_item n=8 part=not_needed
Animation, smooth scroll, scrollbar fade and virtual-clock timing decisions: the chunk changes no clock or scroll behaviour, only what an acting answer reports.
