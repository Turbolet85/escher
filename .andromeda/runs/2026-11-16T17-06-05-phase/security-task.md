@task_prose part=chunk
An acting answer's caused (click, type, press, advance, scroll) gains the text of each record the step caused, from every source: a panic's payload and source location, a root-boundary error's message and its whole cause chain, and a log record's message, target, level, bridged location and fields from the app, the UI framework and the engine.
Every field of every record passes the log sink's existing scrub by field name; a hidden field is shown by name and redaction marker while the rest of the message crosses; the password and file input value mask is unmoved.
Each message is cut at a fixed length and an answer carries a fixed count of records, both set from measured message lengths; a cut says it was cut and by how much, and records past the count are counted by source.
The text rides caused on every carrier it rides today (in-process Outcome, command-line JSON line, session socket call, MCP tool result, kept check) inside the existing MCP result and socket answer bounds, and describe() names the new fields, the cut and the count.
Research first measures real message lengths on the stand and fixtures, and whether an error an app's own error boundary catches can be read by the driver at all; an unread boundary is said in the tool's help.
@task_file n=1 path=packages/escher-driver/src/caused.rs mark=edits
what: the caused answer of an acting command, today closed words, ids and counts alone — edits: the caused record, gaining per-record message text, location, cause chain, target, level and fields, the cut marker and the per-source overflow count
@task_file n=2 path=packages/escher-telemetry/src/panic.rs mark=edits
what: the panic hook that reads a panic's payload and source location (file, line, column) — edits: handing the payload and location on to caused
@task_file n=3 path=packages/dioxus-native-dom/src/boundary.rs mark=edits
what: the app's root error boundary and its RootErrorMark, which today holds the error by identity and reads nothing of it — edits: RootErrorMark, reading the error's message and its source() chain
@task_file n=4 path=packages/escher-telemetry/src/capture.rs mark=edits
what: the capture of log records a step causes, today counting records by level alone and admitting no engine or escher target — edits: the capture, keeping each record's message, target, level, log.file and log.line and fields, and admitting engine and escher targets
@task_file n=5 path=packages/escher-telemetry/src/format.rs
what: the log sink's scrub by field name — the content field names and the engine's safe-field rule — reused unmoved as the scrub every caused field passes
@task_item n=1 part=needs
The caused answer crossing: caused now carries page- and app-authored error text (panic payloads, error messages and cause chains, log messages and fields) beside its closed words, ids and counts, a boundary widening inside the founder's answer that is owed security-plan and architecture amendments at the wrap.
@task_item n=2 part=needs
The scrub on the crossing: every field of every caused record passes the log sink's own field-name scrub (content-named fields withheld, the engine's safe-field allowlist); a hidden field shows its name and the redaction marker, and the rest of the message still crosses, never blanked whole.
@task_item n=3 part=needs
The password and file input value mask holds on every carrier: no masked control's value, nor the masked stand-in's underlying text, reaches caused through a log record, an error message or a panic payload, including in a recorded or replayed kept check.
@task_item n=4 part=needs
The bounds on caused: each message cut at a fixed length and a fixed count of records per answer, both set from measured lengths; a cut message says it was cut and by how much, and records past the count are counted by source, never silently dropped.
@task_item n=5 part=needs
The MCP tool result size bound (1,048,576 bytes) and the session socket answer bound hold over a caused answer carrying text; the command line's answer bounds and the MCP request and result bounds are unchanged.
@task_item n=6 part=needs
The root-boundary error: the error the app's root error boundary takes now crosses as its message and its source() cause chain in caused, beyond crossing as screen content only; the root boundary must read it without a panic.
@task_item n=7 part=needs
Error boundaries inside the app (an ErrorBoundary component under the root): research measures whether the error such a boundary catches can be read by the driver and with what; what is read and what is not is stated, and an unread boundary is said in the tool's help, never silent.
@task_item n=8 part=needs
The panic hook's reading: a panic's payload and its file, line and column cross into caused as the hook already reads them; the hook's behaviour toward the stderr sink is unchanged.
@task_item n=9 part=needs
The log capture's admitted targets: the capture admits app, UI framework, engine and escher target records into caused with message, target, level, log.file and log.line and fields, while the stderr sink's own target allowlist and its dropping of other targets stay as they are.
@task_item n=10 part=needs
The log sink's lines are unchanged: no message, payload or field value reaches a log line or span field that does not already; the command span and the tool_call span keep their fixed, content-free fields; the text crosses only as the command's answer, and driver outcomes are still never logged.
@task_item n=11 part=needs
No new verb, carrier, exit code, refusal cause, session error, socket or listener: the closed driver verb schema, the closed refusal cause set, the fixed session error table, the CLI and escher-session exit codes and the one Unix socket listener stay as they are.
@task_item n=12 part=needs
The session socket reply and the command line's JSON line carry caused with its new text through the existing ok v3 replies and the existing stdout command result line, with no new reply form; escher-session stdout and stderr outputs otherwise unchanged.
@task_item n=13 part=needs
MCP tool results carry caused with its text inside the existing tool result shape; MCP protocol errors stay fixed and carry no data, and the MCP listing and help stay fixed program text apart from describe()'s new field descriptions.
@task_item n=14 part=needs
Kept checks: a recorded check keeps each answer whole, so the caused text is recorded and replayed in kept check files, which stay owner-only, uncommitted and read strictly as untrusted on replay.
@task_item n=15 part=needs
The tool's self-description: describe() names caused's new fields, the meaning of the cut marker and of the per-source overflow count, and what an app's own error boundary does and does not let the driver read.
@task_item n=16 part=needs
Untrusted text on the crossing: messages, payloads and fields are page- and app-authored content; reading them (including a panic payload of any type and a cause chain of any depth) must not panic, and the output must not let an ESC byte or other raw caller-controlled control bytes reach the terminal through the command line beyond what the existing escaping rule permits.
@task_item n=17 part=needs
The research measurement: real message lengths of panics with locations, errors with source chains and the engine's and framework's records on the stand's tasks and the fixtures, the distribution read before the cut and the count are fixed.
@task_item n=18 part=needs
The acceptance: a fixture with a real failure (a panic in a real code path, an Err with a cause chain, a framework or engine record) whose answer alone lets an agent say what failed and where — the test asserts the message, the source location and the cause chain are present in the answer, plus a secret-named field crossing hidden with the rest whole, a message past the cut arriving cut and said, and records past the count counted by source, in both layout modes, in process, through the command line and over MCP.
@task_standing n=1
convention: Each new test or fixture holds no real credential or secret; a secret-named field used to prove the scrub is a fixture stand-in, and live evidence holds no credentials.
home: secret-management
@task_standing n=2
convention: Each new test proving a crossing of content asserts the value mask holds on that carrier (a masked control's value never appears) and that no content reaches a log line or span field.
home: input-validation · logging-monitoring
@task_standing n=3
convention: Each new kept check or recorded check produced by a test stays owner-only and uncommitted, and its replay reads it strictly as untrusted.
home: data-protection
@task_standing n=4
convention: Each new CI job or test leg runs locked builds and keeps the per-crate line coverage floors.
home: dependency-security · input-validation
@task_item n=1 part=not_needed
Outbound networking: outbound HTTP, per-host concurrency, request timeouts, response size limits, CORS and same-origin checks and the script fetcher's URL rules — the chunk fetches nothing.
@task_item n=2 part=not_needed
Authentication and authorization, CI tokens, publish secrets and the release publishing guards — the chunk adds no credential, job permission or publish path.
@task_item n=3 part=not_needed
The select list, range input, password painting, accessibility adapter and stable element id rules — the chunk changes no control behaviour, painting or id derivation.
@task_item n=4 part=not_needed
Browser storage: history in sqlite, the HTTP cache, cookies, clipboard and file dialog — the chunk stores nothing of the browser app.
@task_item n=5 part=not_needed
CSS and HTML parsing limits: import and iframe nesting depth, invalid declarations and selectors, presentational hints and serialization escaping — the chunk parses no page or stylesheet.
@task_item n=6 part=not_needed
The WPT runner and its post-results workflow, quarantine entries and result codes — the chunk adds nothing to the WPT harness.
@task_item n=7 part=not_needed
The cold-agent and agent-run harness scripts, their prompts, transcripts and event logs — the chunk's acceptance runs as tests, not through those scripts.
@task_item n=8 part=not_needed
The coverage, graph and actions answers and the lint findings — their crossings are unchanged; only caused gains text.
