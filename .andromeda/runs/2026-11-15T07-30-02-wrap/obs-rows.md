# obs-plan · the rows 2026-11-15-upstream-sync-errors-and-explanation touches — picked 1 of 276 · rule any · pin 1108

## 6-log-coverage · ## 6. Log Coverage · twin log-coverage
> NOT YET MEASURED — a log JSON schema, a log-file sink and a rotation policy for escher's own sink: the reading recorded none of them
decision/agent-run-no-captured-output · Agent-run events hold no captured output — Nothing between a failures: line and the next test result: line is read, so captured stdout and panic text never reach an agent-run event. · r1
decision/escher-session-fixed-diagnostics · escher-session diagnostics are fixed strings — Every escher-session diagnostic is a fixed string holding no path, label, id or value, and no ending holds anything the caller typed. · r1
decision/sink-target-allowlist · Sink target allowlist — escher's sink applies an allowlist of targets in its formatter: an engine target prints only the safe fields, the rest [redacted]; an escher_ target prints every field but the content-named set; any other target is dropped whole, not an empty line, at every level and whatever RUST_LOG names. · why: The accepted cost is that a third-party WARN or ERROR never reaches stderr, at any setting. · guard: telemetry_drop · except: The upstream apps' fmt::init() stdout subscribers and the WPT runner's env_logger stay unscrubbed. · r1
decision/verdict-fields-closed · Cold-agent verdict fields closed — Each cold-agent verdict field is a count, a boolean, a closed word, the task, the client's tool names, version and model, or the transcript file's path, never transcript text, a tool argument, a tool answer, a reply or the check. · r1
contract/agent-run-events/boot · agent-run harness events · boot — boot {ts, outcome, cargo_exit, head}, printed and appended. · r1
contract/agent-run-events/cleanup · agent-run harness events · cleanup — cleanup {outcome}, printed only. · r1
contract/agent-run-events/events-jsonl · agent-run harness events · events.jsonl — One JSON object per line on stdout, the same lines appended to target/agent-run/events.jsonl, recreated by boot and removed by cleanup; logs prints it verbatim. · r1
contract/agent-run-events/run-end · agent-run harness events · run.end — run.end {ts, selection, passed, failed, ignored, cargo_exit, outcome}, printed and appended. · r1
contract/agent-run-events/run-start · agent-run harness events · run.start — run.start {ts, selection, files}, printed and appended. · r1
contract/agent-run-events/status · agent-run harness events · status — status {booted, boot_ts, head, run}, printed only. · r1
contract/agent-run-events/test · agent-run harness events · test — test {file, test, outcome}, one per libtest result line, printed and appended. · r1
contract/agent-run-test-event/file · The agent-run harness test event · file — The target binary's stem with its hash stripped, or doc:{crate}. · r1
contract/cold-agent-events/cleanup · cold-agent pipe events · cleanup — cleanup {outcome: "done"}, printed only. · r1
contract/cold-agent-events/events-jsonl · cold-agent pipe events · events.jsonl — One JSON object per line on stdout, the same lines appended to target/cold-agent/events.jsonl, recreated by run and removed by cleanup; logs prints it. · r1
contract/cold-agent-events/run-end · cold-agent pipe events · run.end — run.end carrying every verdict field, printed and appended. · r1
contract/cold-agent-events/run-start · cold-agent pipe events · run.start — run.start {ts, task}, printed and appended. · r1
contract/cold-agent-events/status · cold-agent pipe events · status — status {the verdict}, or status {outcome: "none"} with exit 3, printed only. · r1
contract/cold-agent-verdict/advance-calls · The cold-agent verdict · advance_calls — The count of accepted tool_call span lines of the advance verb in the live server child's server.log. · r1
contract/cold-agent-verdict/counts-agree · The cold-agent verdict · counts_agree — Ties the server's span lines to the transcript: the tool_call lines equal the escher tool uses, and its refused and error lines equal the escher results marked is_error. · r1
contract/cold-agent-verdict/lint-calls · The cold-agent verdict · lint_calls — The count of accepted tool_call span lines of the lint verb in the live server child's server.log. · r1
contract/cold-agent-verdict/part-reads · cold-agent verdict · part_reads — Counts the paged snapshot reads. · r1
contract/cold-agent-verdict/reasons · cold-agent verdict · reasons — The reasons add capability-unused, recording-not-kept and wrong-calls-over-bar after task-unmet and before the check's. · r1
contract/cold-agent-verdict/recorded-sessions · cold-agent verdict · recorded_sessions — Reports the live child's kept checks. · r1
contract/cold-agent-verdict/recording-kept · cold-agent verdict · recording_kept — Reports whether the live child's checks were kept. · r1
contract/cold-agent-verdict/screenshot-calls · The cold-agent verdict · screenshot_calls — The count of accepted tool_call span lines of the screenshot verb in the live server child's server.log. · r1
contract/cold-agent-verdict/wrong-call-bar-pct · The cold-agent verdict · wrong_call_bar_pct — The bar on wrong calls, 5; the reason wrong-calls-over-bar is added when wrong_calls × 100 > 5 × tool_calls. · r1
contract/cold-agent-verdict/wrong-calls · The cold-agent verdict · wrong_calls — The sum of refused, error and foreign tool uses and permission denials, the refused and error counts read from the live server child's server.log, deciding the verdict against wrong_call_bar_pct. · r1
contract/escher-driver-span-lines/default-level · escher-driver span lines · default warn level — The driver's command and tool_call span lines print only in a process that installs escher's sink at info or below; at the default warn there is no line. · r1
contract/escher-log-line/bridged-log-record · escher's sink log line · bridged log record — A bridged log record is printed under its log.target, that field itself omitted, and is judged against the allowlist by its log.target. · r1
contract/escher-log-line/escaping · escher's sink log line · value escaping — Newlines and carriage returns in values are escaped as \n and \r, and string values are Debug-quoted. · r1
contract/escher-log-line/event-layout · escher's sink log line · event line layout — An event's line is {RFC 3339 UTC time} {LEVEL} {target} service.name={name} service.version={version} {field}={value}… · r1
contract/escher-log-line/scrub · escher's sink log line · field scrub — Every field of an event line and every pair of a span line passes the sink's scrub. · r1
contract/escher-log-line/span-line · escher's sink log line · closed span line — A closed span's line is {time} {LEVEL} {target} service.name=… service.version=… span={the span's name}, then the span's own fields in the order recorded, then message, time.busy and time.idle. · r1
contract/escher-log-line/span-on-close-only · escher's sink log line · span line timing — A span line is written only when a span whose target the allowlist admits closes, never when it is created, recorded to, entered or exited; an event's line is unchanged inside a span. · r1
contract/escher-log-line/stream · escher's sink log line · output stream — One line per printed event and one per closed span, on stderr, with no ANSI. · r1
contract/escher-session-mcp/stdout · escher-session mcp role · stdout — The mcp role writes protocol lines only on stdout. · r1
contract/escher-sink-env/rust-log · escher's sink configuration · RUST_LOG — Per-module levels come from RUST_LOG through EnvFilter, defaulting to warn when unset or unparsable. · r1
contract/escher-telemetry-events/panic · escher-telemetry events · panic — ERROR at target escher_telemetry::panic, message panic, fields panic.file, panic.line, panic.column and panic.payload (redacted), once per panic. · r1
contract/escher-telemetry-events/telemetry-installed · escher-telemetry events · telemetry installed — info at target escher_telemetry, message telemetry installed, once per successful init, with no argv, path or URL. · r1

## log-coverage · ## 6. Log Coverage · twin 6-log-coverage
decision/actions-read-changes-nothing · Actions read changes nothing in the document — The actions read acts on nothing and changes nothing in the document. · why: stand_actions' the_read_changes_nothing_and_holds_no_typed_text_or_mask pins no change mark, unchanged snapshot, focus and empty diff · r1 · stale
decision/capability-statement-updated-by-hand · Capability statement updated by hand — A change to the driver's span set or to the scrub's limits is made in the published capability statement by hand. · why: The statement's pin reads none of its emitter claims. · r1
decision/closed-span-lines · Closed spans as a second record class — The sink writes one line for each closed span whose target the allowlist admits, and no line when a span is created, recorded to, entered or exited. · why: Ratified as a boundary widening of the log record classes. · r1
decision/cold-agent-server-log-read-by-span-name · Cold-agent server log read by span name only — The cold-agent pipe reads the server log by span name only and never prints or copies it. · r1
decision/cold-agent-verdict-holds-no-content · Cold-agent verdict holds no content — Every field of the cold-agent verdict is a count, a boolean or a closed word, holding no transcript text, tool argument, tool answer, reply or check. · r1
decision/coverage-line-never-logged · Coverage line never logged — The handler coverage line is a command line answer only, never logged or fielded; each replayed step of a kept check writes its own command span line. · guard: a_coverage_run_logs_no_id_no_name_and_nothing_of_its_check_at_trace · r1
decision/decoded-screen-text-stays-in-the-match · Decoded screen text stays in the match — The cold-agent pipe's decoded paged screen text goes into the expect match only, never into an event or the verdict. · r1
decision/fixed-diagnostics · Fixed diagnostics — Every escher-session diagnostic is a fixed string holding no path, label, id or value, and no ending holds anything the caller typed. · r1
decision/graph-and-actions-not-counted-verbs · Graph and actions not counted for a cold agent — The `graph` and `actions` verbs are not counted verbs and are not required of a cold agent, so neither adds a verdict field or a `capability-unused` reason. · r1
decision/graph-answer-never-logged · Graph answer never logged — The graph answer is written as a command answer alone, never logged or fielded. · r1
decision/no-state-page-skips-registry · Page without registered state skips the registry — A page with no registered state reaches no signal registry: its events run their handlers and no dispatch is recorded. · why: signal_registry's a_page_with_no_registered_state_has_no_registry pins no registry context and a click that still acts; the load page's handler-coverage check relies on it · r1
decision/reached-record-ids-and-events-only · Reached record holds ids and event names only — The reached record holds stable ids and event names only, no node number, and its debug form prints its count alone. · why: the reached module's tests pin id and event rows and the_records_debug_prints_its_count_alone; handler coverage reads the record through reached_handlers · r1
contract/cold-agent-counted-verbs/advance · The cold-agent pipe's counted verbs · advance — A counted verb: its accepted `tool_call` span lines are counted from the server log. · r1
contract/cold-agent-counted-verbs/lint · The cold-agent pipe's counted verbs · lint — A counted verb: its accepted `tool_call` span lines are counted from the server log. · r1
contract/cold-agent-counted-verbs/screenshot · The cold-agent pipe's counted verbs · screenshot — A counted verb: its accepted `tool_call` span lines are counted from the server log. · r1
contract/cold-agent-verdict-reasons/capability-unused · The cold-agent verdict's reasons · capability-unused — Given when a counted verb or a part read went unused. · r1
contract/cold-agent-verdict-reasons/wrong-calls-over-bar · The cold-agent verdict's reasons · wrong-calls-over-bar — Given when `wrong_calls × 100 > 5 × tool_calls`. · r1
contract/cold-agent-verdict/reasons-order · The cold-agent verdict · reasons order — Reasons appear in the order `task-unmet`, `capability-unused`, `recording-not-kept`, `wrong-calls-over-bar`, `no-check`, `check-failed`, `check-cannot-fail`. · r1
contract/command-span/actions · The driver's command span · actions — An actions line is `span="command" verb="actions"` (with `cause` when refused, an argument refused `malformed`) and holds no node, id, owner, verb word or cause word of its answer. · r1
contract/command-span/field-count · The driver's command span · field count — The `command` span keeps its eight fields; the driver's fixed help and meaning texts add no log site or span field. · r1
contract/command-span/graph · The driver's command span · graph — A graph's line is `span="command" verb="graph"` (with `cause` when refused, an argument refused `malformed`) and holds no node, state, id or name. · r1
contract/command-span/line · The driver's command span · line — One closed-span line per command run, INFO at target `escher_driver`, `span="command"`, with the recorded ones of `verb`, `cause`, `settled`, `busy`, `passes`, `added`, `removed`, `changed`; no line at the default `warn`. · r1
contract/command-span/lint · The driver's command span · lint — A lint's line is `span="command" verb="lint"` (with `cause` when refused) and holds no finding, id, bound or count. · r1
contract/command-span/screenshot · The driver's command span · screenshot — A shot's line is `span="command" verb="screenshot"` (with `cause` when refused) and holds no image byte, size or dimension. · r1
contract/command-span/snapshot-part · The driver's command span · snapshot part read — A part read's line is `span="command" verb="snapshot"` (with `cause` when refused, a `part` outside 1 to 1000 refused `malformed`) and holds no part number, count, size or text. · r1
contract/command-span/verb · The driver's command span · verb — The verb table's own word for a known verb, run or refused, one of its fourteen words; never the caller's verb text; unrecorded for a verb the table lacks; the three session-level words and the tool-level help appear only beside the cause unknown-verb. · r1
contract/escher-session-commands/help · escher-session commands · help — `help` (at most one `--session <dir>`, not read) writes the self-description's one line of JSON on stdout, status 0, reaching no session; `help` with an argument is refused `malformed`, status 1. · r1
contract/escher-session-commands/mcp · escher-session commands · mcp — `escher-session mcp` writes JSON-RPC protocol lines only on stdout and nothing at its end: status 0 on success, status 3 with the error's fixed message on stderr on failure; any argument but one `--record <dir>` is usage, status 2. · r1
contract/escher-session-commands/open-recording-directory · escher-session commands · recording directory open to group or others — `start` ends with status 3 and nothing spawned, and `mcp --record` with status 3, its fixed message on stderr and stdout empty. · r1
contract/escher-session-coverage/accepted · escher-session's coverage role · accepted — One uncoloured JSON coverage line on stdout, status 0. · r1
contract/escher-session-coverage/boot-failure · escher-session's coverage role · boot failure — A boot that fails ends with the session error, status 3. · r1
contract/escher-session-coverage/command-shape · escher-session's coverage role · command shape — `escher-session coverage <app> <check>...` takes an app the binary boots and one or more paths, no `--session` or `--record` among them; any other shape is usage, status 2. · r1
contract/escher-session-coverage/malformed · escher-session's coverage role · malformed — A file that cannot be read, is over 1,048,576 bytes, is no check, is a check of another app or carries a format other than 1 is refused `malformed`, status 1, holding nothing of the path or the file. · r2
contract/escher-session-error-object/remedy · escher-session's error object · remedy — The error object carries a fixed `remedy` beside `kind` and `message`. · r1
contract/escher-session-exit-codes/0 · escher-session exit codes · 0 — Accepted: one line of JSON on stdout, stderr empty with `RUST_LOG` unset; also `help`, the host role's quiet end and the `mcp` role's `Ok` end. · r1
contract/escher-session-exit-codes/1 · escher-session exit codes · 1 — Refused: one line of JSON on stdout, stderr empty with `RUST_LOG` unset. · r1
contract/escher-session-exit-codes/2 · escher-session exit codes · 2 — Usage: one fixed line on stderr built from `VERBS` and the six task names, stdout empty. · r1
contract/escher-session-exit-codes/3 · escher-session exit codes · 3 — Session error: the error object on stdout and its fixed message on stderr. · r1
contract/escher-session-stderr/telemetry-not-installed · escher-session's stderr · telemetry not installed — The binary's one direct stderr print is `telemetry not installed: …`. · r1
contract/event-line/inside-a-span · The event line · event inside a span — An event emitted inside a span keeps its line unchanged, gaining no span name and no span field. · r1
contract/graph-answer/shape · The graph answer · shape — `{"nodes":[…],"states":[…]}`. · r1
contract/host-wire/recording-failed · The escher-session host wire · recording-failed — A call after which the host's recording ended is answered by the one fixed reply `recording-failed`. · r1
contract/host-wire/v3 · The escher-session host wire · v3 — Wire version `v3` carries `hello`, `stop` and `call`. · r1
contract/span-line/field-order · The closed span's line · field order — `{time} {LEVEL} {target} service.name=… service.version=… span={the span's name}`, then the span's own fields in the order recorded, then `message`, `time.busy` and `time.idle`. · r1
contract/span-line/level-and-target · The closed span's line · level and target — The line's level and target are the span's own. · r1
contract/span-line/scrub · The closed span's line · scrub — Every pair of the line, `span` included, passes the scrub under the span's target. · r1
contract/span-line/span · The closed span's line · span — `span` is a reserved field name holding the span's name. · r1
contract/span-line/target-outside-allowlist · The closed span's line · target outside the allowlist — A span from a target outside the allowlist writes no byte at any point. · r1
contract/span-line/time-busy · The closed span's line · time.busy — The busy timing is tracing-subscriber's own; no escher code reads a clock for it. · r1
contract/span-line/time-idle · The closed span's line · time.idle — The idle timing is tracing-subscriber's own; no escher code reads a clock for it. · r1
contract/span-line/value-quoting · The closed span's line · value quoting — A string value is Debug-quoted (`span="command"`, `message="close"`); a number or a bool is written bare. · r1
contract/tool-call-span/help · The driver's tool_call span · help — A `help` call's line reads `span="tool_call" tool="help" ending="accepted"` with no `command` line beside it. · r1
contract/tool-call-span/line · The driver's tool_call span · line — One closed-span line per MCP tool call, INFO at target `escher_driver`, `span="tool_call"`, with the recorded ones of `tool` and `ending`, then `message="close"`, `time.busy`, `time.idle`; no line at the default `warn`. · r1
