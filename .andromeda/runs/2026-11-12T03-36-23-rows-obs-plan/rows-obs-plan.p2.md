@row_add n=1 row=decision/part-answer-never-logged area=3-observability-harness-contract rec=1
name: A part's answer is never logged
rule: A part's answer ({"text":…,"part":n,"parts":m}) is a command's answer, written over MCP as its JSON line, and is never logged or fielded.
@row_none n=1 rec=2 type=5
@row_none n=2 rec=3 type=5
@row_none n=3 rec=4 type=S
@row_none n=4 rec=5 type=4
@row_none n=5 rec=6 type=4
@row_none n=6 rec=7 type=4
@row_none n=7 rec=8 type=7
@row_none n=8 rec=9 type=7
@row_none n=9 rec=10 type=7
@row_none n=10 rec=11 type=S
@row_add n=2 row=contract/script-diagnostics-drain/take-js-errors area=3-observability-harness-contract rec=12,11
boundary: Embedder drain of script diagnostics
item: take_js_errors
promise: Embedders drain JS errors with take_js_errors.
@row_add n=3 row=contract/script-diagnostics-drain/take-messages area=3-observability-harness-contract rec=12,11
boundary: Embedder drain of script diagnostics
item: take_messages
promise: Embedders drain JS messages with take_messages.
@row_add n=4 row=contract/script-diagnostics-drain/error-retention area=3-observability-harness-contract rec=13,11
boundary: Embedder drain of script diagnostics
item: error retention between drains
promise: At most 256 JS errors are retained between drains.
@row_add n=5 row=contract/escher-log-line/service-name area=3-observability-harness-contract rec=14
boundary: escher's sink log line
item: service.name
promise: Every line carries service.name, the OTel resource key, read from the binary's own CARGO_PKG_NAME; the name is the package's, so two binaries of one package are not told apart.
@row_add n=6 row=contract/escher-log-line/service-version area=3-observability-harness-contract rec=14
boundary: escher's sink log line
item: service.version
promise: Every line carries service.version, the OTel resource key, read from the binary's own CARGO_PKG_VERSION.
@row_add n=7 row=contract/escher-log-line/text-format area=3-observability-harness-contract rec=14
boundary: escher's sink log line
item: line format
promise: Each record is written in a one-line text format, not JSON.
@row_add n=8 row=decision/harness-json-via-python-json area=3-observability-harness-contract rec=15,16
name: Harness JSON lines encoded by python's json module
rule: The agent-run and cold-agent harnesses write JSON lines encoded by python3's stdlib json module embedded in the script, never assembled in bash.
@row_add n=9 row=decision/harness-log-not-telemetry-sink area=3-observability-harness-contract rec=15,16
name: Harness logs are not telemetry sinks
rule: The agent-run and cold-agent harness logs are harness metadata, not telemetry sinks: the harness itself installs no tracing subscriber, OTel setup or third-party logger.
@row_add n=10 row=decision/agent-run-raw-output-never-printed area=3-observability-harness-contract rec=15,33
name: Agent-run raw output never printed
rule: The raw merged cargo and libtest output goes to target/agent-run/run.log, truncated at each run's start and never printed.
@row_add n=11 row=decision/cold-agent-sink-on-live-child-only area=3-observability-harness-contract rec=16,37
name: Cold-agent live server child logs at info
rule: The cold-agent pipe sets RUST_LOG=info on the live escher-session mcp server child only, so that child installs escher's own sink and its stderr goes to target/cold-agent/server.log.
@row_add n=12 row=decision/cold-agent-server-log-counted-only area=3-observability-harness-contract rec=16,37,38
name: Cold-agent server log read only to count
rule: The cold-agent pipe reads server.log only to count tool_call span lines, their refused and error endings and the accepted lines of each counted verb (lint, screenshot, advance), never printing or copying it.
@row_add n=13 row=decision/replay-children-uninstrumented area=3-observability-harness-contract rec=16,37
name: Replay children uninstrumented
rule: A judged check is replayed on further escher-session mcp children spawned directly, without --record, with stderr to /dev/null.
why: uninstrumented by design
@row_add n=14 row=decision/cold-agent-state-never-printed area=3-observability-harness-contract rec=16,38
name: Cold-agent state never printed or copied
rule: The cold-agent state files (transcript.jsonl, client.log, server.log, check.json and the record directory) are never printed or copied.
@row_add n=15 row=decision/cold-agent-record-owner-only area=3-observability-harness-contract rec=16,37
name: Cold-agent record directory owner-only
rule: The live session runs mcp --record into target/cold-agent/record/, made owner-only (mkdir -m 700).
@row_none n=11 rec=17 type=S
@row_add n=16 row=contract/escher-driver-command-span/span area=4-span-trace-coverage rec=18
boundary: escher-driver command span
item: command
promise: Every call handed to Session::run opens one INFO span named command at target escher_driver.
@row_add n=17 row=contract/escher-driver-command-span/verb area=4-span-trace-coverage rec=18
boundary: escher-driver command span
item: verb
promise: A field of the command span, a closed word, bool or count, recorded only where it applies.
@row_add n=18 row=contract/escher-driver-command-span/settled area=4-span-trace-coverage rec=18
boundary: escher-driver command span
item: settled
promise: A field of the command span, a closed word, bool or count, recorded only where it applies.
@row_add n=19 row=contract/escher-driver-command-span/busy area=4-span-trace-coverage rec=18
boundary: escher-driver command span
item: busy
promise: A field of the command span, a closed word, bool or count, recorded only where it applies.
@row_add n=20 row=contract/escher-driver-command-span/passes area=4-span-trace-coverage rec=18
boundary: escher-driver command span
item: passes
promise: A field of the command span, a closed word, bool or count, recorded only where it applies.
@row_add n=21 row=contract/escher-driver-command-span/added area=4-span-trace-coverage rec=18
boundary: escher-driver command span
item: added
promise: A field of the command span, a closed word, bool or count, recorded only where it applies.
@row_add n=22 row=contract/escher-driver-command-span/removed area=4-span-trace-coverage rec=18
boundary: escher-driver command span
item: removed
promise: A field of the command span, a closed word, bool or count, recorded only where it applies.
@row_add n=23 row=contract/escher-driver-command-span/changed area=4-span-trace-coverage rec=18
boundary: escher-driver command span
item: changed
promise: A field of the command span, a closed word, bool or count, recorded only where it applies.
@row_add n=24 row=contract/escher-driver-command-span/cause area=4-span-trace-coverage rec=18
boundary: escher-driver command span
item: cause
promise: A field of the command span, a closed word, bool or count, recorded only where it applies.
@row_add n=25 row=decision/driver-spans-carry-no-content area=4-span-trace-coverage rec=18,43
name: Driver spans carry no content
rule: escher-driver's spans field only closed words, bools and counts, never the screen text, any argument, a shot byte, a lint finding, a part number, or any Command, Call, ArgValue, Outcome, Refusal or Fault.
@row_add n=26 row=decision/driver-emits-no-event area=4-span-trace-coverage rec=18
name: Driver emits no event and installs no subscriber
rule: escher-driver emits no log event and installs no subscriber; its telemetry is its spans alone.
@row_add n=27 row=contract/escher-driver-tool-call-span/span area=4-span-trace-coverage rec=19
boundary: escher-driver tool_call span
item: tool_call
promise: One tool_call span is opened per MCP tools/call request, before its params are read, and held until its reply is written; it has two fields and no other.
@row_add n=28 row=contract/escher-driver-tool-call-span/tool area=4-span-trace-coverage rec=19
boundary: escher-driver tool_call span
item: tool
promise: The verb table's word for the called tool, a closed word.
@row_add n=29 row=contract/escher-driver-tool-call-span/ending area=4-span-trace-coverage rec=19
boundary: escher-driver tool_call span
item: ending
promise: A closed word, one of accepted, refused or error.
@row_add n=30 row=contract/escher-driver-tool-call-span/command-child area=4-span-trace-coverage rec=19
boundary: escher-driver tool_call span
item: command child
promise: An instance verb run through Session::run makes its command span the tool_call's child; the session verbs and help, answered by the server, leave a tool_call with no command child.
@row_none n=12 rec=20 type=7
@row_none n=13 rec=21 type=S
@row_none n=14 rec=22 type=S
@row_add n=31 row=overview/5-metric-coverage area=5-metric-coverage rec=23
text: In-process timings and counters are printed, shown in the browser's in-app FPS overlay or held in process: the frame and phase timings behind the log-times features, blitz-dom's resolve phases, debug_timer, blitz-paint's layer counters, the bench, screenshot and multi_doc examples and the WPT runner's figures; the WPT scores are published to Pages on upstream DioxusLabs/blitz only.
@row_none n=15 rec=24 type=S
@row_add n=32 row=absence/metrics-export area=5-metric-coverage rec=25
capability: Metrics exporters and metric emission (counters, gauges, histograms)
@row_none n=16 rec=26 type=S
@row_none n=17 rec=27 type=S
@row_add n=33 row=contract/escher-log-line/stream area=6-log-coverage rec=28
boundary: escher's sink log line
item: output stream
promise: One line per printed event and one per closed span, on stderr, with no ANSI.
@row_add n=34 row=contract/escher-log-line/event-layout area=6-log-coverage rec=28
boundary: escher's sink log line
item: event line layout
promise: An event's line is {RFC 3339 UTC time} {LEVEL} {target} service.name={name} service.version={version} {field}={value}…
@row_add n=35 row=contract/escher-log-line/escaping area=6-log-coverage rec=28
boundary: escher's sink log line
item: value escaping
promise: Newlines and carriage returns in values are escaped as \n and \r, and string values are Debug-quoted.
@row_add n=36 row=contract/escher-log-line/bridged-log-record area=6-log-coverage rec=28
boundary: escher's sink log line
item: bridged log record
promise: A bridged log record is printed under its log.target, that field itself omitted, and is judged against the allowlist by its log.target.
@row_add n=37 row=contract/escher-log-line/scrub area=6-log-coverage rec=28,29
boundary: escher's sink log line
item: field scrub
promise: Every field of an event line and every pair of a span line passes the sink's scrub.
@row_add n=38 row=decision/sink-target-allowlist area=6-log-coverage rec=28,30
name: Sink target allowlist enforced by the formatter
rule: A record from a target outside the sink's allowlist prints nothing at all, not an empty line, even when a RUST_LOG directive lets it past the filter: the formatter is the last thing a record meets.
guard: telemetry_drop
@row_add n=39 row=contract/escher-log-line/span-line area=6-log-coverage rec=29
boundary: escher's sink log line
item: closed span line
promise: A closed span's line is {time} {LEVEL} {target} service.name=… service.version=… span={the span's name}, then the span's own fields in the order recorded, then message, time.busy and time.idle.
@row_add n=40 row=contract/escher-log-line/span-on-close-only area=6-log-coverage rec=29
boundary: escher's sink log line
item: span line timing
promise: A span line is written only when a span whose target the allowlist admits closes, never when it is created, recorded to, entered or exited; an event's line is unchanged inside a span.
@row_add n=41 row=contract/escher-sink-env/rust-log area=6-log-coverage rec=30
boundary: escher's sink configuration
item: RUST_LOG
promise: Per-module levels come from RUST_LOG through EnvFilter, defaulting to warn when unset or unparsable.
@row_none n=18 rec=31 type=S
@row_add n=42 row=contract/agent-run-events/events-jsonl area=6-log-coverage rec=32,15
boundary: agent-run harness events
item: events.jsonl
promise: One JSON object per line on stdout, the same lines appended to target/agent-run/events.jsonl, recreated by boot and removed by cleanup; logs prints it verbatim.
@row_add n=43 row=contract/agent-run-events/boot area=6-log-coverage rec=32,15
boundary: agent-run harness events
item: boot
promise: boot {ts, outcome, cargo_exit, head}, printed and appended.
@row_add n=44 row=contract/agent-run-events/run-start area=6-log-coverage rec=32,15
boundary: agent-run harness events
item: run.start
promise: run.start {ts, selection, files}, printed and appended.
@row_add n=45 row=contract/agent-run-events/test area=6-log-coverage rec=32,15
boundary: agent-run harness events
item: test
promise: test {file, test, outcome}, one per libtest result line, printed and appended.
@row_add n=46 row=contract/agent-run-events/run-end area=6-log-coverage rec=32,15
boundary: agent-run harness events
item: run.end
promise: run.end {ts, selection, passed, failed, ignored, cargo_exit, outcome}, printed and appended.
@row_add n=47 row=contract/agent-run-events/status area=6-log-coverage rec=32,15
boundary: agent-run harness events
item: status
promise: status {booted, boot_ts, head, run}, printed only.
@row_add n=48 row=contract/agent-run-events/cleanup area=6-log-coverage rec=32,15
boundary: agent-run harness events
item: cleanup
promise: cleanup {outcome}, printed only.
@row_add n=49 row=contract/agent-run-events/test-file area=6-log-coverage rec=33
boundary: agent-run harness events
item: test.file
promise: The target binary's stem with its hash stripped, or doc:{crate}.
@row_add n=50 row=decision/agent-run-no-captured-output area=6-log-coverage rec=33
name: Agent-run events hold no captured output
rule: Nothing between a failures: line and the next test result: line is read, so captured stdout and panic text never reach an agent-run event.
@row_none n=19 rec=34 type=S
@row_add n=51 row=contract/cold-agent-events/events-jsonl area=6-log-coverage rec=35,16
boundary: cold-agent pipe events
item: events.jsonl
promise: One JSON object per line on stdout, the same lines appended to target/cold-agent/events.jsonl, recreated by run and removed by cleanup; logs prints it.
@row_add n=52 row=contract/cold-agent-events/run-start area=6-log-coverage rec=35,16
boundary: cold-agent pipe events
item: run.start
promise: run.start {ts, task}, printed and appended.
@row_add n=53 row=contract/cold-agent-events/run-end area=6-log-coverage rec=35,16
boundary: cold-agent pipe events
item: run.end
promise: run.end carrying every verdict field, printed and appended.
@row_add n=54 row=contract/cold-agent-events/status area=6-log-coverage rec=35,16
boundary: cold-agent pipe events
item: status
promise: status {the verdict}, or status {outcome: "none"} with exit 3, printed only.
@row_add n=55 row=contract/cold-agent-events/cleanup area=6-log-coverage rec=35,16
boundary: cold-agent pipe events
item: cleanup
promise: cleanup {outcome: "done"}, printed only.
@row_add n=56 row=decision/verdict-fields-closed area=6-log-coverage rec=36
name: Cold-agent verdict fields closed
rule: Each cold-agent verdict field is a count, a boolean or a closed word, never transcript text, a tool argument, a tool answer, a reply or the check.
@row_add n=57 row=contract/cold-agent-verdict/counts-agree area=6-log-coverage rec=36
boundary: cold-agent verdict
item: counts_agree
promise: Ties the server's span lines to the transcript.
@row_add n=58 row=contract/cold-agent-verdict/wrong-calls area=6-log-coverage rec=36
boundary: cold-agent verdict
item: wrong_calls
promise: Counts refused, error and foreign tool uses and permission denials, and decides against wrong_call_bar_pct.
@row_add n=59 row=contract/cold-agent-verdict/lint-calls area=6-log-coverage rec=36
boundary: cold-agent verdict
item: lint_calls
promise: Counts the accepted tool_call span lines of the lint verb.
@row_add n=60 row=contract/cold-agent-verdict/screenshot-calls area=6-log-coverage rec=36
boundary: cold-agent verdict
item: screenshot_calls
promise: Counts the accepted tool_call span lines of the screenshot verb.
@row_add n=61 row=contract/cold-agent-verdict/advance-calls area=6-log-coverage rec=36
boundary: cold-agent verdict
item: advance_calls
promise: Counts the accepted tool_call span lines of the advance verb.
@row_add n=62 row=contract/cold-agent-verdict/part-reads area=6-log-coverage rec=36
boundary: cold-agent verdict
item: part_reads
promise: Counts the paged snapshot reads.
@row_add n=63 row=contract/cold-agent-verdict/recorded-sessions area=6-log-coverage rec=36
boundary: cold-agent verdict
item: recorded_sessions
promise: Reports the live child's kept checks.
@row_add n=64 row=contract/cold-agent-verdict/recording-kept area=6-log-coverage rec=36
boundary: cold-agent verdict
item: recording_kept
promise: Reports whether the live child's checks were kept.
@row_add n=65 row=contract/cold-agent-verdict/wrong-call-bar-pct area=6-log-coverage rec=36
boundary: cold-agent verdict
item: wrong_call_bar_pct
promise: The bar, 5; wrong-calls-over-bar holds when wrong_calls × 100 > 5 × tool_calls.
@row_add n=66 row=contract/cold-agent-verdict/reasons area=6-log-coverage rec=36
boundary: cold-agent verdict
item: reasons
promise: The reasons add capability-unused, recording-not-kept and wrong-calls-over-bar after task-unmet and before the check's.
@row_none n=20 rec=39 type=S
@row_none n=21 rec=40 type=5
@row_add n=67 row=decision/escher-session-fixed-diagnostics area=6-log-coverage rec=41
name: escher-session diagnostics are fixed strings
rule: Every escher-session diagnostic is a fixed string holding no path, label, id or value, and no ending holds anything the caller typed.
@row_add n=68 row=contract/escher-session-mcp/stdout area=6-log-coverage rec=41
boundary: escher-session mcp role
item: stdout
promise: The mcp role writes protocol lines only on stdout.
@row_add n=69 row=contract/escher-telemetry-events/telemetry-installed area=6-log-coverage rec=42
boundary: escher-telemetry events
item: telemetry installed
promise: info at target escher_telemetry, message telemetry installed, once per successful init, with no argv, path or URL.
@row_add n=70 row=contract/escher-telemetry-events/panic area=6-log-coverage rec=42,52
boundary: escher-telemetry events
item: panic
promise: ERROR at target escher_telemetry::panic, message panic, fields panic.file, panic.line, panic.column and panic.payload (redacted), once per panic.
@row_add n=71 row=contract/escher-driver-span-lines/default-level area=6-log-coverage rec=43
boundary: escher-driver span lines
item: default warn level
promise: The driver's command and tool_call span lines print only in a process that installs escher's sink at info or below; at the default warn there is no line.
@row_none n=22 rec=44 type=5
@row_none n=23 rec=45 type=S
@row_none n=24 rec=46 type=7
@row_none n=25 rec=47 type=S
@row_none n=26 rec=48 type=S
@row_add n=72 row=decision/wasm-console-panic-hook area=7-error-capture-reporting rec=50,49
name: console_error_panic_hook for WASM builds
rule: WASM builds install console_error_panic_hook, a workspace dependency, as their panic hook.
@row_none n=27 rec=51 type=4
@row_add n=73 row=contract/escher-telemetry-panic-hook/previous-hook area=7-error-capture-reporting rec=52
boundary: escher-telemetry panic hook
item: previous hook
promise: The hook takes the previous hook, logs its ERROR event, then runs the previous hook, so std's default still prints the raw message to stderr and the exit code is unchanged.
@row_add n=74 row=contract/wpt-report/message area=7-error-capture-reporting rec=53
boundary: WPT report
item: message
promise: A crashed WPT test's panic message is carried into the report's message field.
@row_none n=28 rec=54 type=4
@row_none n=29 rec=55 type=S
