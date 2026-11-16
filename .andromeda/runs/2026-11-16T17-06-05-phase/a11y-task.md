@task_prose part=chunk
An acting answer's caused (click, type, press, advance, scroll) gains, per record, the text of what the step caused: a panic's payload and source location, an error's message and its whole cause chain, a log record's message, target, level, location and fields from the app, the UI framework and the engine.
Every field passes the log sink's existing scrub by field name; a hidden field shows its name and the redaction marker and the rest of the message still crosses; the password and file input mask stays unmoved.
Each message is cut at a fixed length and an answer carries a fixed count of records, both set from measured real message lengths; a cut says it was cut and by how much, and records past the count are counted by source.
The text rides every carrier caused rides today (in-process Outcome, command line JSON line, session socket call, MCP tool result, kept check) inside the MCP and socket answer bounds, and describe() names the new fields.
Research first measures message lengths on the stand and fixtures and whether an app's own error boundary can be read; an unread boundary is said by the tool's help.
@task_file n=1 path=packages/escher-driver/src/caused.rs mark=edits
what: the acting answer's caused, the panic, error or log its step caused, by handler, today closed words, ids and counts alone — edits: caused gains each record's text, the cut and the count past the bound
@task_file n=2 path=packages/escher-telemetry/src/panic.rs mark=edits
what: the panic hook, reading a panic's payload and its source location — edits: the payload and location carried into caused
@task_file n=3 path=packages/dioxus-native-dom/src/boundary.rs mark=edits
what: the app's root error boundary and its RootErrorMark, holding the error by identity — edits: RootErrorMark reads the error's message and its cause chain
@task_file n=4 path=packages/escher-telemetry/src/capture.rs mark=edits
what: the log capture of an acting step, counting records by level and admitting no engine or escher target — edits: the capture keeps each record's message, target, level, location and fields, from app, framework and engine
@task_file n=5 path=packages/escher-telemetry/src/format.rs
what: the log sink's format and its scrub by field name: the content field names and the engine's safe-field rule
@task_item n=1 part=needs
The acting answer's caused field on every acting verb (click, type, press, advance, scroll) gains per-record text: message, source location, cause chain, target, level and fields.
@task_item n=2 part=needs
A panic's payload and its source location (file, line, column) cross into the answer as the panic hook reads them.
@task_item n=3 part=needs
An error the app's root error boundary took crosses with its message and every source in its cause chain, not its top line alone.
@task_item n=4 part=needs
Log records from the app, the UI framework and the engine cross with message, target, level, bridged log location and fields; engine and escher targets are now admitted to the answer's capture.
@task_item n=5 part=needs
Every field of every crossing record passes the existing scrub by field name: the content field names stay redacted, the engine's safe-field rule holds, a hidden field shows its name and the redaction marker and the rest of the message still crosses.
@task_item n=6 part=needs
The ratified value mask is unmoved: a password input's value and a file input's value never cross in an answer, through error text or otherwise.
@task_item n=7 part=needs
The log sink's own lines are unchanged: the printed event and closed span line formats, the target allowlist and its per-target field rules, and the command and tool_call spans' fixed fields; no message, payload or field value reaches a log line or span field that does not already.
@task_item n=8 part=needs
The cut and the count are measured values: each message cut at a fixed length, a fixed count of records per answer, set from measured lengths on the stand and fixtures; a cut message states it was cut and by how much, and records past the count are counted by source.
@task_item n=9 part=needs
Research measures real message lengths on the stand's tasks and the fixtures (a panic with location, an error with source chain, engine and framework records) before the bound is set.
@task_item n=10 part=needs
Research measures whether an error an app's own error boundary component catches can be read by the driver at all, and with what; what is not read is stated by the tool's own help, never silent.
@task_item n=11 part=needs
The text rides every carrier caused rides: the in-process Outcome, the command line's JSON line, the session socket's call, an MCP tool result and a kept check recorded, replayed and kept whole; the MCP result bound of 1,048,576 bytes and the socket answer bound hold over it.
@task_item n=12 part=needs
describe()'s self-description names caused's new fields and the meaning of the cut and the count.
@task_item n=13 part=needs
No new verb, carrier, exit, refusal cause, session error, socket or listener; the actions answer and the driver refusal causes stay as they are.
@task_item n=14 part=needs
Acceptance: a fixture with a real failure (a panic in a real code path, an Err with a cause chain, a framework or engine record) whose answer alone asserts the message, the source location and the cause chain, not only the fields' presence.
@task_item n=15 part=needs
Acceptance: a record with a secret-named field crosses with that field hidden and the rest whole; a message past the cut arrives cut with the cut said; records past the count are counted by source.
@task_item n=16 part=needs
Acceptance runs in both layout modes, in process, through the command line and over MCP.
@task_item n=17 part=needs
The session library always enables accessibility, so the acting verbs and their answers are built with the accessibility feature on.
@task_standing n=1
convention: every new check on the stand is repeated three rounds
home: ci-integration · 9-ci-integration
@task_standing n=2
convention: new checks are Rust tests run in the tier and leg their kind belongs to, gating CI
home: 3-a11y-assertion-harness-contract · bootstrap-phases-derive-for-route-setup-project
@task_item n=1 part=not_needed
ARIA state and property reads, role mapping and the accessibility tree's states: the chunk reads no role, state or ARIA attribute.
@task_item n=2 part=not_needed
ARIA patterns and roles, hidden content and native element roles: the chunk changes nothing of what the tree holds or hides.
@task_item n=3 part=not_needed
Keyboard navigation, focus, select lists, range inputs and keyboard activation contracts: the chunk changes no input behaviour, only what an acting answer reports a step caused.
@task_item n=4 part=not_needed
Visual design verification: contrast, the lint's contrast finding, scrollbars, zoom and colour pairs are untouched.
@task_item n=5 part=not_needed
Cognitive accessibility: invalid-state presentation and stand focus order tables are not reached.
@task_item n=6 part=not_needed
Accessibility scope and platform reach: the accessibility tier, the platform adapter, AccessKit as the tree and browser feature defaults are not changed.
