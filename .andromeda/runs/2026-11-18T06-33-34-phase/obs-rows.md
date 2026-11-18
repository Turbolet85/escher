# obs-plan · the rows that bind 2026-11-18-cold-agent-bar — picked 123 of 282 (87 by every draw) · rule any · pin 2041

## 2-telemetry-strategy · ## 2. Telemetry Strategy · twin telemetry-strategy
decision/escher-crates-always-trace · Escher crates take tracing unconditionally — The escher telemetry and driver crates take tracing with no feature, so the startup and panic events and the command span are always compiled into the seven_guis package's two native binaries, while their engine features stay off in a package-alone build.

## telemetry-strategy · ## 2. Telemetry Strategy · twin 2-telemetry-strategy
decision/escher-crates-tracing-ungated · escher crates take tracing ungated — escher-telemetry and escher-driver take tracing with no feature gate, so the startup and panic events and the driver's spans are always compiled into the binaries that use them.

## 3-observability-harness-contract · ## 3. Observability Harness Contract
decision/agent-run-raw-output-never-printed · Agent-run raw output never printed — The raw merged cargo and libtest output goes to target/agent-run/run.log, truncated at each run's start and never printed.
decision/cold-agent-server-log-counted-only · Cold-agent server log read only to count — The cold-agent pipe reads server.log only to count tool_call span lines, their refused and error endings and the accepted lines of each counted verb (lint, screenshot, advance), never printing or copying it.
decision/cold-agent-sink-on-live-child-only · Cold-agent live server child logs at info — The cold-agent pipe sets RUST_LOG=info on the live escher-session mcp server child only, so that child installs escher's own sink and its stderr goes to target/cold-agent/server.log.
decision/cold-agent-state-never-printed · Cold-agent state never printed or copied — The cold-agent state files (transcript.jsonl, client.log, server.log, check.json and the record directory) are never printed or copied.
decision/one-span-per-command · One span per driver command — Each driver command opens exactly one command span, which writes only where a process has installed a sink at info or below.
decision/sink-host-read-at-trace · Sink-installing host read at trace — A sink-installing host and its mcp role are read at trace under both builds, and must show no id, name, typed-sentinel or layout-answer key needle in what they write. · guard: host_log, mcp_log
contract/command-answer/graphed · driver command answer · nodes and states — A graph answer is one JSON object of nodes and states holding ids, component names, registered state and handler event names, closed flag words and booleans, never a value.
contract/escher-log-line/text-format · escher's sink log line · line format — Each record is written in a one-line text format, not JSON.
contract/telemetry-sink/rust-log · escher's telemetry sink · RUST_LOG — The sink's filter is read from RUST_LOG, and is warn when RUST_LOG is unset; it filters the sink layer alone, the capture layer beside it unfiltered.
contract/telemetry-sink/stream · escher's telemetry sink · output stream — The sink writes uncoloured lines to stderr only, never to stdout.
decision/answers-echo-nothing-typed · Answers echo nothing the caller typed — No answer, refusal, usage line or session error writes back anything the caller typed, and a password's or a file input's value appears only as the fixed mask. · except: A caused item's message, chain and kept fields hold what their author wrote, a field of the answer's own hidden set reading [redacted] and an error field shown as written, cut at the text bound.
decision/cold-agent-record-owner-only · Cold-agent record directory owner-only — The live session runs mcp --record into target/cold-agent/record/, made owner-only (mkdir -m 700).
decision/headless-stand-no-sink · Headless stand runs with no sink — A headless boot made in process installs no telemetry sink and reads no environment variable, its acting steps running only under escher-telemetry's scoped capture, which prints nothing; a headless boot made by the escher-session binary runs under that binary's sink. · except: The MCP tools and layout stand checks install their own recording subscriber; the command-span check's re-run children and the error-text founder check install escher's sink over an in-memory writer.
decision/part-answer-never-logged · A part's answer is never logged — A part's answer ({"text":…,"part":n,"parts":m}) is a command's answer, written over MCP as its JSON line, and is never logged or fielded.
contract/kept-check/answer · kept check file · answer — Each answered call records the answer itself, a screenshot's without the PNG.
contract/kept-check/arguments · kept check file · arguments — Each answered call's arguments are written into the kept check, a file at rest, never a log line, field or print.
contract/mcp-role-stdout/json-rpc · escher-session mcp role stdout · JSON-RPC lines — In the mcp role stdout carries newline-delimited JSON-RPC 2.0 protocol lines only, never written by the sink, and the role's end writes nothing on stdout.

## 4-span-trace-coverage · ## 4. Span / Trace Coverage · twin span-trace-coverage
decision/layout-read-opens-no-span · The layout read opens no span — The driver's layout read opens no span and emits no event of its own: read in process it writes no escher_ line at TRACE, and reached by the layout verb it runs inside that call's command span. · why: No span by design, under the decision that the driver emits no event. · guard: stand_layout's the_read_writes_no_escher_line_at_trace
contract/escher-driver-command-span/cause · escher-driver command span · cause — A field of the command span, a closed word, bool or count, recorded only where it applies.
contract/escher-driver-command-span/span · escher-driver command span · command — Every call handed to Session::run opens one INFO span named command at target escher_driver.
contract/escher-driver-tool-call-span/ending · escher-driver tool_call span · ending — A closed word, one of accepted, refused or error.
contract/escher-driver-tool-call-span/span · escher-driver tool_call span · tool_call — One tool_call span is opened per MCP tools/call request, before its params are read, and held until its reply is written; it has two fields and no other.
contract/escher-driver-tool-call-span/tool · escher-driver tool_call span · tool — The verb table's word for the called tool, a closed word.
decision/driver-spans-carry-no-content · Driver spans carry no content — escher-driver's spans field only closed words, bools and counts, never the screen text, any argument, a shot byte, a lint finding, a part number, or any Command, Call, ArgValue, Outcome, Refusal or Fault.

## span-trace-coverage · ## 4. Span / Trace Coverage · twin 4-span-trace-coverage
decision/tool-call-span-two-closed-fields · tool_call span with tool and ending — Each MCP tools/call request writes one tool_call span carrying exactly two fields, tool and ending, each a closed word. · why: Ratified as the span's shape. · guard: stand_mcp_tools
contract/cold-agent-verdict/capability-unused · The cold-agent verdict · capability-unused — The reason given when lint_calls, screenshot_calls or advance_calls reads 0.
contract/command-span/cause · The driver's command span · cause — Recorded at one site for every refusal the run returns: one of eight fixed cause names, class words holding nothing a call supplied.
contract/command-span/settled · The driver's command span · settled — Recorded from an acting verb's outcome only; snapshot, screenshot, lint, graph, actions, layout and a part read record verb alone, and a refusal records verb and cause alone.
contract/command-span/span · The driver's command span · span — One span per call handed to the session's run, target escher_driver, name command, level INFO, entered before validation and held until the run returns, so a refusal is inside it.
contract/refusal-cause/covered · Refusal cause names · covered — A closed refusal class word recorded in the command span's cause field, returned by the executor.
contract/refusal-cause/disabled · Refusal cause names · disabled — A closed refusal class word recorded in the command span's cause field, returned by the executor.
contract/refusal-cause/fonts-unavailable · Refusal cause names · fonts-unavailable — A closed refusal class word recorded in the command span's cause field, returned by the executor before anything is painted when the held app has no font it can paint text with.
contract/refusal-cause/malformed · Refusal cause names · malformed — A closed refusal class word recorded in the command span's cause field, returned by validation.
contract/refusal-cause/not-found · Refusal cause names · not-found — A closed refusal class word recorded in the command span's cause field, returned by the executor.
contract/refusal-cause/off-screen · Refusal cause names · off-screen — A closed refusal class word recorded in the command span's cause field, returned by the executor; an element screenshot that crops to no pixel is refused off-screen.
contract/refusal-cause/stale · Refusal cause names · stale — A closed refusal class word recorded in the command span's cause field, returned by the executor.
contract/refusal-cause/unknown-verb · Refusal cause names · unknown-verb — A closed refusal class word recorded in the command span's cause field, returned by validation.
contract/tool-call-ending/accepted · The tool_call span's ending · accepted — The tool call was accepted and answered.
contract/tool-call-ending/error · The tool_call span's ending · error — The tool call ended in a session error or a protocol error.
contract/tool-call-ending/refused · The tool_call span's ending · refused — The tool call was refused, its reply one text block with isError true.
contract/tool-call-span/ending · The MCP tool_call span · ending — One of three closed words saying how the call ended: `accepted` · `refused` · `error` (a session error or a protocol error).
contract/tool-call-span/span · The MCP tool_call span · span — One span, target escher_driver, name tool_call, level INFO, opened once per MCP tools/call request before its params are read and held until its reply is written.
contract/tool-call-span/tool · The MCP tool_call span · tool — The verb table's word for the tool, one of its fifteen; unrecorded for an unknown tool.
decision/advance-accepted-every-session · advance accepted on every session — The advance verb is accepted on every session and records the acting fields like every acting verb, with no cause. · why: Ratified.
decision/command-span-no-call-content · No call content in the command span — The command span never fields an argument, an id, the screen text, an accessible name, an image or its size, lint findings, graph content, the session label, or any command, call, outcome or refusal value. · why: An argument fielded by its schema name, or a command or outcome fielded with Debug, would print an id, typed text, an accessible name or a control's value. · guard: stand_act_spans

## 6-log-coverage · ## 6. Log Coverage · twin log-coverage
decision/verdict-fields-closed · Cold-agent verdict fields closed — Each cold-agent verdict field is a count, a boolean, a closed word, the task, the client's tool names, version and model, or the transcript file's path, never transcript text, a tool argument, a tool answer, a reply or the check.
contract/cold-agent-events/cleanup · cold-agent pipe events · cleanup — cleanup {outcome: "done"}, printed only.
contract/cold-agent-events/run-end · cold-agent pipe events · run.end — run.end carrying every verdict field, printed and appended.
contract/cold-agent-events/run-start · cold-agent pipe events · run.start — run.start {ts, task}, printed and appended.
contract/cold-agent-events/status · cold-agent pipe events · status — status {the verdict}, or status {outcome: "none"} with exit 3, printed only.
contract/cold-agent-verdict/advance-calls · The cold-agent verdict · advance_calls — The count of accepted tool_call span lines of the advance verb in the live server child's server.log.
contract/cold-agent-verdict/lint-calls · The cold-agent verdict · lint_calls — The count of accepted tool_call span lines of the lint verb in the live server child's server.log.
contract/cold-agent-verdict/reasons · cold-agent verdict · reasons — The reasons add capability-unused, recording-not-kept and wrong-calls-over-bar after task-unmet and before the check's.
contract/cold-agent-verdict/screenshot-calls · The cold-agent verdict · screenshot_calls — The count of accepted tool_call span lines of the screenshot verb in the live server child's server.log.
contract/cold-agent-verdict/wrong-call-bar-pct · The cold-agent verdict · wrong_call_bar_pct — The bar on wrong calls, 5; the reason wrong-calls-over-bar is added when wrong_calls × 100 > 5 × tool_calls.
contract/cold-agent-verdict/wrong-calls · The cold-agent verdict · wrong_calls — The sum of refused, error and foreign tool uses and permission denials, the refused and error counts read from the live server child's server.log, deciding the verdict against wrong_call_bar_pct.
contract/escher-driver-span-lines/default-level · escher-driver span lines · default warn level — The driver's command and tool_call span lines print only in a process that installs escher's sink at info or below; at the default warn there is no line.
contract/cold-agent-events/events-jsonl · cold-agent pipe events · events.jsonl — One JSON object per line on stdout, the same lines appended to target/cold-agent/events.jsonl, recreated by run and removed by cleanup; logs prints it.
contract/cold-agent-verdict/counts-agree · The cold-agent verdict · counts_agree — Ties the server's span lines to the transcript: the tool_call lines equal the escher tool uses, and its refused and error lines equal the escher results marked is_error.
contract/escher-log-line/scrub · escher's sink log line · field scrub — Every field of an event line and every pair of a span line passes the sink's scrub.
contract/escher-log-line/span-line · escher's sink log line · closed span line — A closed span's line is {time} {LEVEL} {target} service.name=… service.version=… span={the span's name}, then the span's own fields in the order recorded, then message, time.busy and time.idle.
contract/escher-log-line/span-on-close-only · escher's sink log line · span line timing — A span line is written only when a span whose target the allowlist admits closes, never when it is created, recorded to, entered or exited; an event's line is unchanged inside a span.
contract/escher-sink-env/rust-log · escher's sink configuration · RUST_LOG — Per-module levels come from RUST_LOG through EnvFilter, defaulting to warn when unset or unparsable, and apply to the sink layer alone, never to the capture layer beside it.

## log-coverage · ## 6. Log Coverage · twin 6-log-coverage
decision/actions-read-changes-nothing · Actions read changes nothing in the document — The actions read acts on nothing and changes nothing in the document. · why: stand_actions' the_read_changes_nothing_and_holds_no_typed_text_or_mask pins no change mark, unchanged snapshot, focus and empty diff
decision/cold-agent-server-log-read-by-span-name · Cold-agent server log read by span name only — The cold-agent pipe reads the server log by span name only and never prints or copies it.
decision/cold-agent-verdict-holds-no-content · Cold-agent verdict holds no content — Every field of the cold-agent verdict is a count, a boolean or a closed word, holding no transcript text, tool argument, tool answer, reply or check.
decision/decoded-screen-text-stays-in-the-match · Decoded screen text stays in the match — The cold-agent pipe's decoded paged screen text goes into the expect match only, never into an event or the verdict.
decision/graph-actions-layout-not-counted-verbs · Graph, actions and layout not counted for a cold agent — The `graph`, `actions` and `layout` verbs are not counted verbs and are not required of a cold agent, so none of them adds a verdict field or a `capability-unused` reason.
decision/graph-answer-never-logged · Graph answer never logged — The graph answer is written as a command answer alone, never logged or fielded.
contract/cold-agent-counted-verbs/advance · The cold-agent pipe's counted verbs · advance — A counted verb: its accepted `tool_call` span lines are counted from the server log.
contract/cold-agent-counted-verbs/lint · The cold-agent pipe's counted verbs · lint — A counted verb: its accepted `tool_call` span lines are counted from the server log.
contract/cold-agent-counted-verbs/screenshot · The cold-agent pipe's counted verbs · screenshot — A counted verb: its accepted `tool_call` span lines are counted from the server log.
contract/cold-agent-verdict-reasons/capability-unused · The cold-agent verdict's reasons · capability-unused — Given when a counted verb or a part read went unused.
contract/cold-agent-verdict-reasons/wrong-calls-over-bar · The cold-agent verdict's reasons · wrong-calls-over-bar — Given when `wrong_calls × 100 > 5 × tool_calls`.
contract/cold-agent-verdict/reasons-order · The cold-agent verdict · reasons order — Reasons appear in the order `task-unmet`, `capability-unused`, `recording-not-kept`, `wrong-calls-over-bar`, `no-check`, `check-failed`, `check-cannot-fail`.
contract/command-span/actions · The driver's command span · actions — An actions line is `span="command" verb="actions"` (with `cause` when refused, an argument refused `malformed`) and holds no node, id, owner, verb word or cause word of its answer.
contract/command-span/graph · The driver's command span · graph — A graph's line is `span="command" verb="graph"` (with `cause` when refused, an argument refused `malformed`) and holds no node, state, id or name.
contract/command-span/layout · The driver's command span · layout — A layout's line is `span="command" verb="layout"` (with `cause` when refused) and holds nothing of its answer; the layout read the call reaches opens no span of its own.
contract/command-span/line · The driver's command span · line — One closed-span line per command run, INFO at target `escher_driver`, `span="command"`, with the recorded ones of `verb`, `cause`, `settled`, `busy`, `passes`, `added`, `removed`, `changed`; no line at the default `warn`.
contract/graph-answer/shape · The graph answer · shape — `{"nodes":[…],"states":[…]}`.
contract/span-line/span · The closed span's line · span — `span` is a reserved field name holding the span's name.
contract/span-line/value-quoting · The closed span's line · value quoting — A string value is Debug-quoted (`span="command"`, `message="close"`); a number or a bool is written bare.
contract/tool-call-span/line · The driver's tool_call span · line — One closed-span line per MCP tool call, INFO at target `escher_driver`, `span="tool_call"`, with the recorded ones of `tool` and `ending`, then `message="close"`, `time.busy`, `time.idle`; no line at the default `warn`.
decision/closed-span-lines · Closed spans as a second record class — The sink writes one line for each closed span whose target the allowlist admits, and no line when a span is created, recorded to, entered or exited. · why: Ratified as a boundary widening of the log record classes.
contract/command-span/lint · The driver's command span · lint — A lint's line is `span="command" verb="lint"` (with `cause` when refused) and holds no finding, id, bound or count.
contract/command-span/screenshot · The driver's command span · screenshot — A shot's line is `span="command" verb="screenshot"` (with `cause` when refused) and holds no image byte, size or dimension.
contract/command-span/snapshot-part · The driver's command span · snapshot part read — A part read's line is `span="command" verb="snapshot"` (with `cause` when refused, a `part` outside 1 to 1000 refused `malformed`) and holds no part number, count, size or text.
contract/command-span/verb · The driver's command span · verb — The verb table's own word for a known verb, run or refused, one of its fifteen words; never the caller's verb text; unrecorded for a verb the table lacks; the three session-level words and the tool-level help appear only beside the cause unknown-verb.
contract/span-line/field-order · The closed span's line · field order — `{time} {LEVEL} {target} service.name=… service.version=… span={the span's name}`, then the span's own fields in the order recorded, then `message`, `time.busy` and `time.idle`.
contract/span-line/scrub · The closed span's line · scrub — Every pair of the line, `span` included, passes the scrub under the span's target.

## error-capture-reporting · ## 7. Error Capture & Reporting · twin 7-error-capture-reporting
contract/telemetry-scrub-set/error · escher-telemetry's scrub set · error — An `error` field is in the sink's scrub set, so the sink prints its value as `[redacted]`; the scoped capture keeps it in a caused item as written, cut at the text bound, so a failed fetch's error text crosses the answer carriers as a field value.
contract/telemetry-scrub-set/url · escher-telemetry's scrub set · url — A `url` field is in the scrub set, so the sink prints its value as `[redacted]` and the scoped capture keeps it as `[redacted]` in a caused item.

## 8-pii-scrubbing-compliance · ## 8. PII Scrubbing & Compliance · twin pii-scrubbing-compliance
decision/cold-agent-kept-checks-at-rest · Cold-agent kept checks held at rest only — The live cold-agent session's kept checks live in an owner-only record directory, holding calls and answers at rest, never printed, copied or committed.
decision/cold-agent-log-no-user-content · Cold-agent pipe log carries no user content — The cold-agent pipe log sits outside the sink and carries no user content: no event or verdict key is content-named and no transcript text, tool argument, answer, reply or check enters an event or the verdict.
decision/cold-agent-transcript-raw · Cold-agent transcript kept raw — The cold-agent transcript, raw model output and tool I/O, carries the stand's screens to the model provider and holds host paths, unscrubbed, never printed, and gitignored under target/. · why: Carried to the model provider by design.
decision/scrub-by-field-name · Scrubbing by field name in the sink and the capture — Scrubbing is by field name: in escher's own sink by the content-named set, a closed span's line judged pair by pair by the same rule, and in the scoped capture for every source by the answer's own hidden set; a scrubbed field reads [redacted], the message and other fields kept as written.
decision/agent-run-raw-log-unscrubbed · Agent-run raw log left unscrubbed — The agent-run raw cargo/libtest output log stays unscrubbed, is never printed by logs, and is gitignored under target/. · why: Unscrubbed by design, like the CI logs.

## pii-scrubbing-compliance · ## 8. PII Scrubbing & Compliance · twin 8-pii-scrubbing-compliance
decision/closed-span-lines-judged-alike · Closed span lines judged by the same rule — A closed span's line is judged pair by pair by the target rule, and a span from a target outside the allowlist writes no byte at creation, record, enter, exit or close.
decision/cold-agent-artifacts-never-printed · Cold-agent server log and check never printed — The cold-agent server log is counted by span name and never printed or copied, and the check file holding the agent's check is never printed.
decision/cold-agent-events-carry-no-user-content · Cold-agent events carry no user content — No cold-agent pipe log event or verdict key is in the content-named set, and no transcript text, tool argument, tool answer, reply or check enters an event or the verdict.
decision/cold-agent-transcript-unscrubbed · Cold-agent transcript unscrubbed — The cold-agent transcript holds raw model output and tool I/O, carried to the model provider, never printed or logged and gitignored under target. · why: unscrubbed by design, the provider crossing ratified
decision/driver-span-fields-print · Driver span fields tool and ending print — The tool_call span's fields tool and ending are not in the content-named set and print. · why: both are closed words; the closed-span line was ratified
decision/select-list-label-never-logged · Select list row label never logged — A select list row's label holds page text in a public read type, is never logged, and leaves only as a painted pixel and the snapshot's existing name.
contract/actions-answer/cause · the actions answer (actions) · cause — A closed cause word, left out when absent.
contract/actions-answer/content · the actions answer (actions) · content — Carries ids, owner ids, the table's verb words, closed cause words and booleans only, never a value, typed text, mask or NodeId; a boundary widening, ratified.
contract/actions-answer/logging · the actions answer (actions) · logging — The answer is kept whole in a recorded check and is never logged or fielded.
contract/actions-answer/owner · the actions answer (actions) · owner — Left out when the node has no owner.
contract/actions-answer/shape · the actions answer (actions) · shape — Written as {"nodes":[{"id":…,"owner":…,"keyed":…,"verbs":[{"verb":…,"cause":…}]}]}.
contract/graph-answer/content · the UI graph answer (graph) · content — Carries stable ids, component names, registered state names, handler event names, the closed flag words undeclared and unconfirmed, and booleans only, never a state's value; a boundary widening, ratified.
contract/layout-answer/content · the layout answer (layout) · content — Carries ids, closed words and numbers only, never a value or typed text; the sink at trace carries none of its keys (laid_out, placed, shifts).
contract/layout-answer/logging · the layout answer (layout) · logging — The answer leaves on the session socket, the command line and MCP, crossing to the model provider in the transcript on a live cold-agent run, and is kept whole in a recorded check; it is never logged or fielded.
decision/committed-transcript-copies-masked · Committed transcript copies masked — Each committed copy of a cold-agent transcript masks the session dir and the home prefix.
decision/escher-target-content-named-set · Escher targets redact the content-named set — Under an escher_ target the sink prints every field except url, href, src, html, text, value, attrs, path, request, error, password and panic.payload, which are redacted; escher alone, without the underscore, is not admitted.
decision/open-list-shot-carries-labels · Open list shot carries labels as pixels — A shot of an open list carries a page's option labels and group labels as pixels, the shot's own fields unmoved. · why: a boundary widening, ratified
decision/optgroup-label-crossing · Optgroup label as group name and painted header — An optgroup's non-empty label is its group name in the accessibility tree and is painted as a group header in an open list, leaving as pixels in a shot and on no log line or field. · why: inside the ratified name crossing; a boundary widening, ratified
decision/select-chosen-set-never-logged · Select chosen set never logged — A multiple select's chosen set leaves only as form entries, the snapshot's value and the options' tree nodes, never as a log line or field.
decision/session-logs-carry-no-page-content · Session logs carry no page content — The session host's, the mcp role's and every client's stderr carries no element id, accessible name, typed text, recording path or recorded content at any log level, under the package-alone and the workspace build. · guard: host_log, mcp_log
contract/snapshot-diff/open-list-row-bounds · the snapshot and the diff · open list row bounds — An open list's row crosses by its own bounds, differs naming bounds within its closed set, adding no field, key name, node key or carrier.

## ci-integration · ## 9. CI Integration · twin 9-ci-integration
decision/cold-agent-cleanup-scoped · Cold-agent cleanup touches only its own state — The cold-agent cleanup removes the pipe's state and touches nothing else under target/; the per-run temporary session directory is removed when the run exits.
decision/cold-agent-kept-checks-stay-local · Cold-agent kept checks stay local — The cold-agent pipe's live run keeps its checks in an owner-only (mode 700) recording directory; the kept checks are never printed, copied, uploaded by a CI leg or committed, and are removed with the rest of the state at cleanup; the replay children run without recording.
decision/recording-dir-private · Recording directory private — A recording directory is created 0700 with its parent existing and is refused when open to its group or others; it holds call content at rest and never a log or event line; recording is unix only.

# obs-plan · its absences, names and overviews, whole

## 1-obs-scope-summary · ## 1. Obs Scope Summary
> NOT YET MEASURED — the telemetry-surface table, must-trace critical paths and telemetry triggers: the reading recorded none of them for any slice

## 2-telemetry-strategy · ## 2. Telemetry Strategy · twin telemetry-strategy
> NOT YET MEASURED — span, metric, log-field and resource-attribute naming conventions: the reading recorded none
absence/telemetry-backend — A telemetry backend or OTel SDK (Sentry, OpenTelemetry, Prometheus or a metrics exporter)

## telemetry-strategy · ## 2. Telemetry Strategy · twin 2-telemetry-strategy
overview/telemetry-strategy — Telemetry is the tracing crate's event macros and, in escher-driver, two hand-built spans, command and tool_call. In the engine and upstream crates tracing is compiled in only with the tracing cargo feature. escher-telemetry takes tracing with no feature and always compiles its startup and panic events into the two native binaries, seven_guis_native and escher-session. escher-driver takes tracing ungated, so its spans are written in escher-session through its host, mcp and coverage roles, and by no code path in the windowed seven_guis_native.

## 3-observability-harness-contract · ## 3. Observability Harness Contract
> NOT YET MEASURED — product mode, snapshot integration, trace context propagation and heartbeat ticks, and a JSON schema or log-file location for escher's own sink: the reading recorded none of them

## 5-metric-coverage · ## 5. Metric Coverage · twin metric-coverage
absence/metrics-export — Metrics exporters and metric emission (counters, gauges, histograms)
overview/5-metric-coverage — In-process timings and counters are printed, shown in the browser's in-app FPS overlay or held in process: the frame and phase timings behind the log-times features, blitz-dom's resolve phases, debug_timer, blitz-paint's layer counters, the bench, screenshot and multi_doc examples and the WPT runner's figures; the WPT scores are published to Pages on upstream DioxusLabs/blitz only.

## metric-coverage · ## 5. Metric Coverage · twin 5-metric-coverage
absence/metric-emission — Metric emission: counters, gauges or histograms
absence/metrics-exporters — Metrics exporters or telemetry backends (OpenTelemetry, Prometheus, Sentry)

## 6-log-coverage · ## 6. Log Coverage · twin log-coverage
> NOT YET MEASURED — a log JSON schema, a log-file sink and a rotation policy for escher's own sink: the reading recorded none of them

## 7-error-capture-reporting · ## 7. Error Capture & Reporting · twin error-capture-reporting
absence/error-reporting-service — An error-reporting client or error-capture service (such as Sentry)

## 9-ci-integration · ## 9. CI Integration · twin ci-integration
> NOT YET MEASURED — snapshot artifact upload and CI resource attributes: the reading recorded none

## 10-slo-invariants-telemetry-budgets · ## 10. SLO Invariants & Telemetry Budgets
> NO RECORDED INTENT

## 11-obs-anti-patterns-never-do-these · ## 11. Obs Anti-Patterns (NEVER do these)
> NO RECORDED INTENT

## 12-obs-decisions-log · ## 12. Obs Decisions Log
> NO RECORDED INTENT
