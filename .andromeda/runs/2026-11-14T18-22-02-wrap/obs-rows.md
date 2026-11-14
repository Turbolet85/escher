# obs-plan · the rows 2026-11-14-remote-pages-local-files touches — picked 9 of 276 · rule any · pin 1036

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
decision/actions-read-changes-nothing · Actions read changes nothing in the document — The actions read acts on nothing and changes nothing in the document. · why: stand_actions' the_read_changes_nothing_and_holds_no_typed_text_or_mask pins no change mark, unchanged snapshot, focus and empty diff · r1
decision/capability-statement-updated-by-hand · Capability statement updated by hand — A change to the driver's span set or to the scrub's limits is made in the published capability statement by hand. · why: The statement's pin reads none of its emitter claims. · r1 · stale
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
contract/command-span/line · The driver's command span · line — One closed-span line per command run, INFO at target `escher_driver`, `span="command"`, with the recorded ones of `verb`, `cause`, `settled`, `busy`, `passes`, `added`, `removed`, `changed`; no line at the default `warn`. · r1 · stale
contract/command-span/lint · The driver's command span · lint — A lint's line is `span="command" verb="lint"` (with `cause` when refused) and holds no finding, id, bound or count. · r1
contract/command-span/screenshot · The driver's command span · screenshot — A shot's line is `span="command" verb="screenshot"` (with `cause` when refused) and holds no image byte, size or dimension. · r1
contract/command-span/snapshot-part · The driver's command span · snapshot part read — A part read's line is `span="command" verb="snapshot"` (with `cause` when refused, a `part` outside 1 to 1000 refused `malformed`) and holds no part number, count, size or text. · r1
contract/command-span/verb · The driver's command span · verb — The verb table's own word for a known verb, run or refused, one of its fourteen words; never the caller's verb text; unrecorded for a verb the table lacks; the three session-level words and the tool-level help appear only beside the cause unknown-verb. · r1
contract/escher-session-commands/help · escher-session commands · help — `help` (at most one `--session <dir>`, not read) writes the self-description's one line of JSON on stdout, status 0, reaching no session; `help` with an argument is refused `malformed`, status 1. · r1
contract/escher-session-commands/mcp · escher-session commands · mcp — `escher-session mcp` writes JSON-RPC protocol lines only on stdout and nothing at its end: status 0 on success, status 3 with the error's fixed message on stderr on failure; any argument but one `--record <dir>` is usage, status 2. · r1
contract/escher-session-commands/open-recording-directory · escher-session commands · recording directory open to group or others — `start` ends with status 3 and nothing spawned, and `mcp --record` with status 3, its fixed message on stderr and stdout empty. · r1
contract/escher-session-coverage/accepted · escher-session's coverage role · accepted — One uncoloured JSON coverage line on stdout, status 0. · r1
contract/escher-session-coverage/boot-failure · escher-session's coverage role · boot failure — A boot that fails ends with the session error, status 3. · r1
contract/escher-session-coverage/command-shape · escher-session's coverage role · command shape — `escher-session coverage <app> <check>...` takes an app the binary boots and one or more paths, no `--session` or `--record` among them; any other shape is usage, status 2. · r1 · stale
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

## 7-error-capture-reporting · ## 7. Error Capture & Reporting · twin error-capture-reporting
decision/exploration-catches-panics · Exploration catches a panicking call — A panic in a call an exploration drives is caught and reported as the closed ending Panicked holding no payload, the process's panic hook still running, and the exploration goes on. · r1
decision/failed-stylesheet-leaves-pending-set · Failed stylesheet leaves the pending critical set — A failed stylesheet fetch drops its request from the pending critical resources, so loading does not wait on it. · why: render_blocking_stylesheet's failed_stylesheet_unblocks_rendering pins it, and the harness settle and paint_scene both read the pending critical resources · r1 · stale
decision/no-file-loads-from-non-file-documents · No file: loads from non-file: documents — A file: load asked by a non-file: document is refused as a typed forbidden error: a script fetch rejects with a TypeError, a module import fails, a script element's source is recorded as a script error, and an engine sub-resource fetch fails with fixed text naming no URL or path, adding no log site. · r2 · contradict
decision/wasm-console-panic-hook · console_error_panic_hook for WASM builds — WASM builds install console_error_panic_hook, a workspace dependency, as their panic hook. · r1
contract/escher-telemetry-panic-hook/previous-hook · escher-telemetry panic hook · previous hook — The hook takes the previous hook, logs its ERROR event, then runs the previous hook, so std's default still prints the raw message to stderr and the exit code is unchanged. · r1
contract/script-error-source-labels/error-event-listener · Script error source labels in reports · error event listener — An error raised in an error event listener is labelled "error event listener" in reports. · r1
contract/script-error-source-labels/event-listener · Script error source labels in reports · event listener — An error raised in an event listener is labelled "event listener" in reports. · r1
contract/script-error-source-labels/event-microtasks · Script error source labels in reports · event microtasks — An error raised in event microtasks is labelled "event microtasks" in reports. · r1
contract/script-error-source-labels/timer-callback · Script error source labels in reports · timer callback — An error raised in a timer callback is labelled "timer callback" in reports. · r1
contract/script-error-source-labels/timer-microtasks · Script error source labels in reports · timer microtasks — An error raised in timer microtasks is labelled "timer microtasks" in reports. · r1
contract/script-error-surface/error-handler-exception · The script error surface · exception thrown by an error handler — An exception thrown by an error handler is recorded but fires no further error event. · r1
contract/window-error-event/colno · The window error event · colno — The window error event carries colno 0. · r1
contract/window-error-event/error · The window error event · error — The window error event carries the thrown error. · r1
contract/window-error-event/filename · The window error event · filename — The window error event carries filename as the empty string. · r1
contract/window-error-event/lineno · The window error event · lineno — The window error event carries lineno 0. · r1
contract/window-error-event/message · The window error event · message — The window error event carries the error's message. · r1
contract/window-onerror/arguments · window.onerror · arguments — window.onerror is called with message, source, lineno, colno and error, in that order. · r1
contract/wpt-report/message · WPT report · message — A crashed WPT test's panic message is carried into the report's message field. · r1
contract/wpt-report/subtest-message · The WPT report · subtest message — A subtest's error strings are joined with newlines into the report's subtest message. · r1
absence/error-reporting-service — An error-reporting client or error-capture service (such as Sentry) · r1

## error-capture-reporting · ## 7. Error Capture & Reporting · twin 7-error-capture-reporting
contract/telemetry-scrub-set/error · escher-telemetry's scrub set · error — An `error` field is in the scrub set, so the sink prints its value as `[redacted]`; a failed fetch's error text reaches no carrier beyond its log line. · r1
contract/telemetry-scrub-set/url · escher-telemetry's scrub set · url — A `url` field is in the scrub set, so the sink prints its value as `[redacted]`. · r1

## 8-pii-scrubbing-compliance · ## 8. PII Scrubbing & Compliance · twin pii-scrubbing-compliance
decision/agent-run-log-no-user-content · Agent-run harness log carries no user content — The agent-run harness log sits outside the sink and carries no user content: none of its event keys is in the content-named set and no captured test output enters it. · r1
decision/agent-run-raw-log-unscrubbed · Agent-run raw log left unscrubbed — The agent-run raw cargo/libtest output log stays unscrubbed, is never printed by logs, and is gitignored under target/. · why: Unscrubbed by design, like the CI logs. · r1
decision/cold-agent-kept-checks-at-rest · Cold-agent kept checks held at rest only — The live cold-agent session's kept checks live in an owner-only record directory, holding calls and answers at rest, never printed, copied or committed. · r1
decision/cold-agent-log-no-user-content · Cold-agent pipe log carries no user content — The cold-agent pipe log sits outside the sink and carries no user content: no event or verdict key is content-named and no transcript text, tool argument, answer, reply or check enters an event or the verdict. · r1
decision/cold-agent-transcript-raw · Cold-agent transcript kept raw — The cold-agent transcript, raw model output and tool I/O, carries the stand's screens to the model provider and holds host paths, unscrubbed, never printed, and gitignored under target/. · why: Carried to the model provider by design. · r1
decision/content-span-fields-never-stored · Content-named span fields never stored — The value of a content-named span field is never stored; its [redacted] marker is written at print time, and every other span field's value is held only in process memory until the span closes. · r1
decision/scrub-at-sink-not-call-sites · Scrubbing at the sink, not at call sites — Scrubbing is applied in escher's own sink, a closed span's line judged pair by pair by the same rule, and no engine call site is edited. · r1
decision/scrub-sink-reach · Reach of the scrubbing sink — The scrubbing sink is installed by the two binaries of the seven_guis package; engine events reach it only when an engine tracing feature is on. · except: The upstream apps' stdout subscribers and the WPT runner's env_logger stay unscrubbed. · r1

## pii-scrubbing-compliance · ## 8. PII Scrubbing & Compliance · twin 8-pii-scrubbing-compliance
decision/closed-span-lines-judged-alike · Closed span lines judged by the same rule — A closed span's line is judged pair by pair by the target rule, and a span from a target outside the allowlist writes no byte at creation, record, enter, exit or close. · r1
decision/cold-agent-artifacts-never-printed · Cold-agent server log and check never printed — The cold-agent server log is counted by span name and never printed or copied, and the check file holding the agent's check is never printed. · r1
decision/cold-agent-events-carry-no-user-content · Cold-agent events carry no user content — No cold-agent pipe log event or verdict key is in the content-named set, and no transcript text, tool argument, tool answer, reply or check enters an event or the verdict. · r1
decision/cold-agent-transcript-unscrubbed · Cold-agent transcript unscrubbed — The cold-agent transcript holds raw model output and tool I/O, carried to the model provider, never printed or logged and gitignored under target. · why: unscrubbed by design, the provider crossing ratified · r1
decision/committed-transcript-copies-masked · Committed transcript copies masked — Each committed copy of a cold-agent transcript masks the session dir and the home prefix. · r1
decision/content-named-span-values-not-stored · Content-named span values never stored — The value of a content-named span field is never stored, its marker written at print time. · r1
decision/driver-span-fields-print · Driver span fields tool and ending print — The tool_call span's fields tool and ending are not in the content-named set and print. · why: both are closed words; the closed-span line was ratified · r1
decision/engine-target-safe-fields · Engine targets print safe fields only — Under an engine target (blitz, dioxus_native, stylo_taffy, accesskit_xplat, debug_timer, js_console) the sink prints only node_id, status, waiting_nodes, property, log.module_path, log.file and log.line, every other field as {name}=[redacted]. · r1
decision/escher-target-content-named-set · Escher targets redact the content-named set — Under an escher_ target the sink prints every field except url, href, src, html, text, value, attrs, path, request, error and panic.payload, which are redacted; escher alone, without the underscore, is not admitted. · r1
decision/mutation-buffer-memory-only · Mutation buffer contents stay in memory — The buffered render's mutation buffer holds page content in process memory only from a render to its replay, its Debug printing its length alone, and nothing logs, fields or prints its contents. · r1
decision/open-list-shot-carries-labels · Open list shot carries labels as pixels — A shot of an open list carries a page's option labels and group labels as pixels, the shot's own fields unmoved. · why: a boundary widening, ratified · r1
decision/optgroup-label-crossing · Optgroup label as group name and painted header — An optgroup's non-empty label is its group name in the accessibility tree and is painted as a group header in an open list, leaving as pixels in a shot and on no log line or field. · why: inside the ratified name crossing; a boundary widening, ratified · r1
decision/runtime-probe-leaks-no-read-value · Runtime probe leaks no read value — The dioxus runtime probe lets no read value leave it: its failure messages name a layout mode, a task or page and a link or index only, and its reading quotes no captured stderr, panic payload, signal value or node id. · r1
decision/select-chosen-set-never-logged · Select chosen set never logged — A multiple select's chosen set leaves only as form entries, the snapshot's value and the options' tree nodes, never as a log line or field. · r1
decision/select-list-label-never-logged · Select list row label never logged — A select list row's label holds page text in a public read type, is never logged, and leaves only as a painted pixel and the snapshot's existing name. · r1
decision/select-size-attribute-read · Select size attribute read — The size attribute on a multiple select is read as a whole number capped at 65534, carrying no text out and moving no redaction shape, for every document. · why: an engine change for every document and a boundary widening, ratified · r1
decision/session-logs-carry-no-page-content · Session logs carry no page content — The session host's, the mcp role's and every client's stderr carries no element id, accessible name, typed text, recording path or recorded content at any log level, under the package-alone and the workspace build. · guard: host_log, mcp_log · r1
decision/shot-image-leaves-only-in-answer · Shot image leaves only in its answer — A shot's image is screen content that leaves only in the command's answer, never on a log line. · guard: host_log, mcp_log · r1
decision/state-registry-holds-no-values · State registry holds no values — The state registry holds each registered state's name, declarations, write records and dispatch chain, never a state's value; its Debug prints counts alone and it holds no log site, print or env read. · r1
contract/actions-answer/cause · the actions answer (actions) · cause — A closed cause word, left out when absent. · r1
contract/actions-answer/content · the actions answer (actions) · content — Carries ids, owner ids, the table's verb words, closed cause words and booleans only, never a value, typed text, mask or NodeId; a boundary widening, ratified. · r1
contract/actions-answer/logging · the actions answer (actions) · logging — The answer is kept whole in a recorded check and is never logged or fielded. · r1
contract/actions-answer/owner · the actions answer (actions) · owner — Left out when the node has no owner. · r1
contract/actions-answer/shape · the actions answer (actions) · shape — Written as {"nodes":[{"id":…,"owner":…,"keyed":…,"verbs":[{"verb":…,"cause":…}]}]}. · r1
contract/coverage-line/content · the coverage line (escher-session coverage) · content — Carries stable ids, owner ids, handler event names and two counts only, never a value or typed text; a boundary widening, ratified. · r1 · stale
contract/coverage-line/logging · the coverage line (escher-session coverage) · logging — The line is a command answer, never logged or fielded. · r1
contract/coverage-line/owner · The handler coverage line (escher-session coverage) · owner — `owner` is left out when absent. · r1
contract/coverage-line/shape · The handler coverage line (escher-session coverage) · shape — Written by the crate's hand writer as `{"unexercised":[{"id":…,"owner":…,"event":…}],"checks":n,"held":m}`, a command line answer never logged or fielded. · r1 · stale
contract/graph-answer/content · the UI graph answer (graph) · content — Carries stable ids, component names, registered state names, handler event names, the closed flag words undeclared and unconfirmed, and booleans only, never a state's value; a boundary widening, ratified. · r1 · stale
contract/multiple-select-form-entries/chosen-option · multiple select form entries · chosen option — On input and change, one (name, FormValue::Text(value)) entry per chosen option. · r1
contract/page-events/checkbox-change · page script events · change on a checkbox — The engine always sends change after input with the same value, carrying the value class input carries. · r1
contract/page-events/radio-change · page script events · change on a radio — The engine sends change after input with the same value only where the radio was not checked before the click. · r1
contract/page-events/single-change · page script events · change count — A page's script hears one change, from the engine, the script runtime synthesising none. · r1
contract/snapshot-diff/open-list-row-bounds · the snapshot and the diff · open list row bounds — An open list's row crosses by its own bounds, differs naming bounds within its closed set, adding no field, key name, node key or carrier. · r1
contract/snapshot-node/clickable · the snapshot node · clickable — A closed bool, true where an element's current listeners hold click and it is no disabled form control, carrying no page text; written as a node's "clickable" key. · r1
contract/snapshot-text/clickable · the snapshot text · clickable — The token clickable stands after focused. · r1
contract/snapshot/multiple-select-value · the snapshot · value of a multiple select — The first chosen option in tree order. · r1

## 9-ci-integration · ## 9. CI Integration · twin ci-integration
> NOT YET MEASURED — snapshot artifact upload and CI resource attributes: the reading recorded none
decision/ci-platform-github-workflows · CI on GitHub workflows — CI runs as GitHub workflows under .github/workflows/. · r1
decision/committed-kept-checks-clean · Committed kept checks hold no secrets — The kept checks committed in git hold no typed text, host path or secret. · r1
decision/fork-ci-two-artifacts · Fork CI uploads two artifacts — The fork's CI uploads exactly two artifacts, a leg's merged log on failure and the coverage report on success, each kept 7 days, neither holding user data. · r1
decision/local-state-never-uploaded · Local state stays local — The agent-run and cold-agent state, a session's state directory and a recording directory stay local and are uploaded by no CI leg. · r1
decision/wpt-publishing-upstream-only · WPT results published upstream only — The WPT report and wptscores.json are published to GitHub Pages and the WPT results repository only on upstream DioxusLabs/blitz; the publishing jobs are repository-guarded and never run on the fork. · r1
contract/apt-install-diagnostic/attempt · The package-install diagnostic line · attempt — One stderr line per failed attempt names the attempt number, and nothing of a user's. · r1
contract/apt-install-diagnostic/exit-status · The package-install diagnostic line · exit status — The line names the failed attempt's exit status. · r1
contract/apt-install-diagnostic/phase · The package-install diagnostic line · phase — The line names the phase as one of update, install or repair. · r1

## ci-integration · ## 9. CI Integration · twin 9-ci-integration
decision/agent-run-cleanup-scoped · Agent-run cleanup touches only its own state — The agent-run cleanup removes the harness's status, events and raw run log and touches nothing else under target/. · r1
decision/ci-log-artifact-on-failure · Per-leg CI log uploaded on failure — Each CI leg's merged stdout and stderr is written to its own log, truncated at the leg's start, and uploaded only on failure as artifact ci-log-{job id}, kept 7 days; it is unscrubbed build output holding no user data. · r1
decision/cold-agent-cleanup-scoped · Cold-agent cleanup touches only its own state — The cold-agent cleanup removes the pipe's state and touches nothing else under target/; the per-run temporary session directory is removed when the run exits. · r1
decision/cold-agent-kept-checks-stay-local · Cold-agent kept checks stay local — The cold-agent pipe's live run keeps its checks in an owner-only (mode 700) recording directory; the kept checks are never printed, copied, uploaded by a CI leg or committed, and are removed with the rest of the state at cleanup; the replay children run without recording. · r1
decision/committed-checks-hold-no-private-data · Committed checks hold no private data — A kept check committed to git is one of the repository's own scripted stand-Counter sessions and holds no typed text, host path, name, secret or screenshot step. · why: The repository is public. · r1
decision/coverage-artifact-on-success · Coverage report uploaded on success — The coverage leg's lcov file, line counts of the workspace's public source with no user data, is uploaded only on success as artifact coverage-report, kept 7 days. · r1
decision/recording-bounds · Recording bounds — A recording ends on a failed write or a passed bound of 400 records or 1,048,576 bytes, and only then does the recorder remove it; escher otherwise never removes a recording. · r1
decision/recording-dir-private · Recording directory private — A recording directory is created 0700 with its parent existing and is refused when open to its group or others; it holds call content at rest and never a log or event line; recording is unix only. · r1 · stale
decision/session-state-dir-private · Session state directory private and empty of content — A session's state directory is created 0700 by the host and holds the socket file alone while a session is up and nothing after stop or idle expiry; no log, event or screen content is written into it. · r1
contract/record-refusals/record-dir-not-private · Session recording refusals · record-dir-not-private — A recording directory open to its group or others is refused with record-dir-not-private. · r1
contract/recording-file/answer · The recording file · answer — Each call's answer is kept whole as the line the driver wrote, masked as on every carrier, with its accepted flag and digest; a shot's answer is held as width and height without png. · r1
contract/recording-file/check-file · The recording file · check-{n}.json — Each recorded session gets one check-{n}.json, mode 0600, created new and never replaced, rewritten whole after every call through a temporary file and a rename. · r1
contract/recording-file/typed-text · The recording file · typed text — Typed text is held unmasked, except a type to a masked or unresolved control, held as masked. · r1
contract/session-lifecycle/idle-expiry · The session lifecycle · idle expiry — The host's idle expiry ends a session exactly as stop does. · r1
