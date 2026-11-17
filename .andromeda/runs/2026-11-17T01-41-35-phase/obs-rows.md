# obs-plan · the rows that bind 2026-11-17-layout-on-the-carriers — picked 106 of 279 (69 by every draw) · rule any · pin 1580

## 2-telemetry-strategy · ## 2. Telemetry Strategy · twin telemetry-strategy
decision/escher-crates-always-trace · Escher crates take tracing unconditionally — The escher telemetry and driver crates take tracing with no feature, so the startup and panic events and the command span are always compiled into the seven_guis package's two native binaries, while their engine features stay off in a package-alone build.
decision/telemetry-via-tracing · Telemetry through the tracing crate — Telemetry is the tracing crate's event and span macros; the engine and upstream crates compile their log sites only with the tracing cargo feature, each call a no-op when the feature is off.

## telemetry-strategy · ## 2. Telemetry Strategy · twin 2-telemetry-strategy
decision/escher-crates-tracing-ungated · escher crates take tracing ungated — escher-telemetry and escher-driver take tracing with no feature gate, so the startup and panic events and the driver's spans are always compiled into the binaries that use them.
decision/tracing-for-telemetry · tracing as the telemetry crate — Telemetry is written with the tracing crate's event macros and span macros.

## 3-observability-harness-contract · ## 3. Observability Harness Contract
decision/answers-echo-nothing-typed · Answers echo nothing the caller typed — No answer, refusal, usage line or session error writes back anything the caller typed, and a password's or a file input's value appears only as the fixed mask. · except: A caused item's message, chain and kept fields hold what their author wrote, a field of the answer's own hidden set reading [redacted] and an error field shown as written, cut at the text bound.
decision/debug-types-never-logged · Answer-bearing types never logged — Types that derive Debug and hold ids, accessible names, control values or page text are never logged, fielded or printed with Debug; a command's outcome is written in one place, as the command's answer.
decision/driver-library-installs-no-subscriber · Driver library installs no subscriber — The driver library, its command line, MCP server and coverage reader included, installs no sink and reads no environment variable, setting only escher-telemetry's scoped capture around an acting step and its chaining panic hook, both printing nothing; only a binary's own main installs the sink.
decision/one-span-per-command · One span per driver command — Each driver command opens exactly one command span, which writes only where a process has installed a sink at info or below.
decision/part-answer-never-logged · A part's answer is never logged — A part's answer ({"text":…,"part":n,"parts":m}) is a command's answer, written over MCP as its JSON line, and is never logged or fielded.
decision/replay-children-uninstrumented · Replay children uninstrumented — A judged check is replayed on further escher-session mcp children spawned directly, without --record, with stderr to /dev/null. · why: uninstrumented by design
decision/session-id-record-unprinted · Session id record kept private — The session's record of the ids its screens have read is bounded, has no reader outside the driver crate and no Debug, and is never printed, logged or fielded.
decision/sink-host-read-at-trace · Sink-installing host read at trace — A sink-installing host and its mcp role are read at trace under both builds, and must show no id, name or typed-sentinel needle in what they write. · guard: host_log, mcp_log
contract/escher-session-stdout/command-answer · escher-session stdout · command answer — Each command's answer, refusal or session-error object is one uncoloured line of JSON on stdout, an answer and never a log line; the host role ends quiet.
contract/kept-check/accepted · kept check file · accepted — Each answered call records whether it was accepted.
contract/kept-check/answer · kept check file · answer — Each answered call records the answer itself, a screenshot's without the PNG.
contract/kept-check/answer-digest · kept check file · answer digest — Each answered call records its answer's FNV-1a 64 digest.
contract/kept-check/arguments · kept check file · arguments — Each answered call's arguments are written into the kept check, a file at rest, never a log line, field or print.
decision/cold-agent-record-owner-only · Cold-agent record directory owner-only — The live session runs mcp --record into target/cold-agent/record/, made owner-only (mkdir -m 700).
decision/headless-stand-no-sink · Headless stand runs with no sink — A headless boot made in process installs no telemetry sink and reads no environment variable, its acting steps running only under escher-telemetry's scoped capture, which prints nothing; a headless boot made by the escher-session binary runs under that binary's sink. · except: The MCP tools and layout stand checks install their own recording subscriber; the command-span check's re-run children and the error-text founder check install escher's sink over an in-memory writer.
contract/command-answer/linted · driver command answer · findings — A lint answer is written as one JSON object with a findings array, kept whole in a recorded check and never logged or fielded.
contract/mcp-role-stdout/json-rpc · escher-session mcp role stdout · JSON-RPC lines — In the mcp role stdout carries newline-delimited JSON-RPC 2.0 protocol lines only, never written by the sink, and the role's end writes nothing on stdout.

## 4-span-trace-coverage · ## 4. Span / Trace Coverage · twin span-trace-coverage
decision/driver-emits-no-event · Driver emits no event and installs no subscriber — escher-driver emits no log event and installs no sink, setting only escher-telemetry's scoped capture around an acting step, which keeps the step's records and panics in process and prints nothing; its telemetry is its spans alone.
decision/driver-spans-carry-no-content · Driver spans carry no content — escher-driver's spans field only closed words, bools and counts, never the screen text, any argument, a shot byte, a lint finding, a part number, or any Command, Call, ArgValue, Outcome, Refusal or Fault.
decision/layout-read-opens-no-span · The layout read opens no span — The driver's in-process layout read opens no span and emits no event, runs outside the command span, and writes no escher_ line at TRACE. · why: No span by design, under the decision that the driver emits no event. · guard: stand_layout's the_read_writes_no_escher_line_at_trace
contract/escher-driver-command-span/cause · escher-driver command span · cause — A field of the command span, a closed word, bool or count, recorded only where it applies.
contract/escher-driver-command-span/span · escher-driver command span · command — Every call handed to Session::run opens one INFO span named command at target escher_driver.
contract/escher-driver-command-span/verb · escher-driver command span · verb — A field of the command span, a closed word, bool or count, recorded only where it applies.
contract/escher-driver-tool-call-span/command-child · escher-driver tool_call span · command child — An instance verb run through Session::run makes its command span the tool_call's child; the session verbs and help, answered by the server, leave a tool_call with no command child.
contract/escher-driver-tool-call-span/ending · escher-driver tool_call span · ending — A closed word, one of accepted, refused or error.
contract/escher-driver-tool-call-span/span · escher-driver tool_call span · tool_call — One tool_call span is opened per MCP tools/call request, before its params are read, and held until its reply is written; it has two fields and no other.
contract/escher-driver-tool-call-span/tool · escher-driver tool_call span · tool — The verb table's word for the called tool, a closed word.
contract/escher-driver-command-span/added · escher-driver command span · added — A field of the command span, a closed word, bool or count, recorded only where it applies.
contract/escher-driver-command-span/busy · escher-driver command span · busy — A field of the command span, a closed word, bool or count, recorded only where it applies.
contract/escher-driver-command-span/changed · escher-driver command span · changed — A field of the command span, a closed word, bool or count, recorded only where it applies.
contract/escher-driver-command-span/passes · escher-driver command span · passes — A field of the command span, a closed word, bool or count, recorded only where it applies.
contract/escher-driver-command-span/removed · escher-driver command span · removed — A field of the command span, a closed word, bool or count, recorded only where it applies.
contract/escher-driver-command-span/settled · escher-driver command span · settled — A field of the command span, a closed word, bool or count, recorded only where it applies.

## span-trace-coverage · ## 4. Span / Trace Coverage · twin 4-span-trace-coverage
decision/command-span-no-call-content · No call content in the command span — The command span never fields an argument, an id, the screen text, an accessible name, an image or its size, lint findings, graph content, the session label, or any command, call, outcome or refusal value. · why: An argument fielded by its schema name, or a command or outcome fielded with Debug, would print an id, typed text, an accessible name or a control's value. · guard: stand_act_spans
decision/hosted-call-through-session-run · Hosted call runs through the session's run alone — A hosted part read runs through the session's run alone, so its spans are command spans recording verb alone. · why: stand_act_spans pins a part read's command line carrying its verb alone, and the session host answers each call through Session::run with no span of its own
decision/tool-call-span-two-closed-fields · tool_call span with tool and ending — Each MCP tools/call request writes one tool_call span carrying exactly two fields, tool and ending, each a closed word. · why: Ratified as the span's shape. · guard: stand_mcp_tools
contract/command-span/cause · The driver's command span · cause — Recorded at one site for every refusal the run returns: one of eight fixed cause names, class words holding nothing a call supplied.
contract/command-span/settled · The driver's command span · settled — Recorded from an acting verb's outcome only; snapshot, screenshot, lint, graph, actions and a part read record verb alone, and a refusal records verb and cause alone.
contract/command-span/span · The driver's command span · span — One span per call handed to the session's run, target escher_driver, name command, level INFO, entered before validation and held until the run returns, so a refusal is inside it.
contract/refusal-cause/malformed · Refusal cause names · malformed — A closed refusal class word recorded in the command span's cause field, returned by validation.
contract/refusal-cause/not-found · Refusal cause names · not-found — A closed refusal class word recorded in the command span's cause field, returned by the executor.
contract/refusal-cause/stale · Refusal cause names · stale — A closed refusal class word recorded in the command span's cause field, returned by the executor.
contract/tool-call-ending/accepted · The tool_call span's ending · accepted — The tool call was accepted and answered.
contract/tool-call-ending/error · The tool_call span's ending · error — The tool call ended in a session error or a protocol error.
contract/tool-call-ending/refused · The tool_call span's ending · refused — The tool call was refused, its reply one text block with isError true.
contract/tool-call-span/command-child · The MCP tool_call span · command child — An instance verb run through the session's run makes its command span the tool_call's child; the session verbs start, status, stop and help, answered by the server, leave a tool_call with no command child.
contract/tool-call-span/ending · The MCP tool_call span · ending — One of three closed words saying how the call ended: `accepted` · `refused` · `error` (a session error or a protocol error).
contract/tool-call-span/span · The MCP tool_call span · span — One span, target escher_driver, name tool_call, level INFO, opened once per MCP tools/call request before its params are read and held until its reply is written.
contract/tool-call-span/tool · The MCP tool_call span · tool — The verb table's word for the tool, one of its fourteen; unrecorded for an unknown tool.
contract/command-span/added · The driver's command span · added — The length of the returned diff's added list, recorded from an acting verb's outcome.
contract/command-span/busy · The driver's command span · busy — One of three busy-class words, the schema's word for the busy class, recorded from an acting verb's outcome when the outcome carries one.
contract/command-span/changed · The driver's command span · changed — The length of the returned diff's changed list, recorded from an acting verb's outcome.
contract/command-span/passes · The driver's command span · passes — Settle's pass count, recorded from an acting verb's outcome when settle answered quiet; on a step that waited on a load, the settle that went quiet.
contract/command-span/removed · The driver's command span · removed — The length of the returned diff's removed list, recorded from an acting verb's outcome.
contract/refusal-cause/unknown-verb · Refusal cause names · unknown-verb — A closed refusal class word recorded in the command span's cause field, returned by validation.

## 6-log-coverage · ## 6. Log Coverage · twin log-coverage
decision/escher-session-fixed-diagnostics · escher-session diagnostics are fixed strings — Every escher-session diagnostic is a fixed string holding no path, label, id or value, and no ending holds anything the caller typed.
contract/escher-session-mcp/stdout · escher-session mcp role · stdout — The mcp role writes protocol lines only on stdout.
contract/escher-driver-span-lines/default-level · escher-driver span lines · default warn level — The driver's command and tool_call span lines print only in a process that installs escher's sink at info or below; at the default warn there is no line.

## log-coverage · ## 6. Log Coverage · twin 6-log-coverage
decision/actions-read-changes-nothing · Actions read changes nothing in the document — The actions read acts on nothing and changes nothing in the document. · why: stand_actions' the_read_changes_nothing_and_holds_no_typed_text_or_mask pins no change mark, unchanged snapshot, focus and empty diff
decision/coverage-line-never-logged · Coverage line never logged — The handler coverage line is a command line answer only, never logged or fielded; each replayed step of a kept check writes its own command span line. · guard: a_coverage_run_logs_no_id_no_name_and_nothing_of_its_check_at_trace
decision/fixed-diagnostics · Fixed diagnostics — Every escher-session diagnostic is a fixed string holding no path, label, id or value, and no ending holds anything the caller typed.
contract/command-span/actions · The driver's command span · actions — An actions line is `span="command" verb="actions"` (with `cause` when refused, an argument refused `malformed`) and holds no node, id, owner, verb word or cause word of its answer.
contract/command-span/field-count · The driver's command span · field count — The `command` span keeps its eight fields; the driver's fixed help and meaning texts add no log site or span field.
contract/command-span/line · The driver's command span · line — One closed-span line per command run, INFO at target `escher_driver`, `span="command"`, with the recorded ones of `verb`, `cause`, `settled`, `busy`, `passes`, `added`, `removed`, `changed`; no line at the default `warn`.
contract/command-span/verb · The driver's command span · verb — The verb table's own word for a known verb, run or refused, one of its fourteen words; never the caller's verb text; unrecorded for a verb the table lacks; the three session-level words and the tool-level help appear only beside the cause unknown-verb.
contract/escher-session-commands/help · escher-session commands · help — `help` (at most one `--session <dir>`, not read) writes the self-description's one line of JSON on stdout, status 0, reaching no session; `help` with an argument is refused `malformed`, status 1.
contract/escher-session-commands/mcp · escher-session commands · mcp — `escher-session mcp` writes JSON-RPC protocol lines only on stdout and nothing at its end: status 0 on success, status 3 with the error's fixed message on stderr on failure; any argument but one `--record <dir>` is usage, status 2.
contract/escher-session-commands/open-recording-directory · escher-session commands · recording directory open to group or others — `start` ends with status 3 and nothing spawned, and `mcp --record` with status 3, its fixed message on stderr and stdout empty.
contract/escher-session-exit-codes/0 · escher-session exit codes · 0 — Accepted: one line of JSON on stdout, stderr empty with `RUST_LOG` unset; also `help`, the host role's quiet end and the `mcp` role's `Ok` end.
contract/escher-session-exit-codes/1 · escher-session exit codes · 1 — Refused: one line of JSON on stdout, stderr empty with `RUST_LOG` unset.
contract/escher-session-exit-codes/2 · escher-session exit codes · 2 — Usage: one fixed line on stderr built from `VERBS` and the six task names, stdout empty.
contract/host-wire/v3 · The escher-session host wire · v3 — Wire version `v3` carries `hello`, `stop` and `call`.
contract/tool-call-span/line · The driver's tool_call span · line — One closed-span line per MCP tool call, INFO at target `escher_driver`, `span="tool_call"`, with the recorded ones of `tool` and `ending`, then `message="close"`, `time.busy`, `time.idle`; no line at the default `warn`.
decision/capability-statement-updated-by-hand · Capability statement updated by hand — A change to the driver's span set or to the scrub's limits is made in the published capability statement by hand. · why: The statement's pin reads none of its emitter claims.
decision/graph-answer-never-logged · Graph answer never logged — The graph answer is written as a command answer alone, never logged or fielded.
contract/escher-session-coverage/accepted · escher-session's coverage role · accepted — One uncoloured JSON coverage line on stdout, status 0.
contract/escher-session-coverage/malformed · escher-session's coverage role · malformed — A file that cannot be read, is over 1,048,576 bytes, is no check, is a check of another app or carries a format other than 1 is refused `malformed`, status 1, holding nothing of the path or the file.
contract/escher-session-exit-codes/3 · escher-session exit codes · 3 — Session error: the error object on stdout and its fixed message on stderr.
contract/tool-call-span/help · The driver's tool_call span · help — A `help` call's line reads `span="tool_call" tool="help" ending="accepted"` with no `command` line beside it.

## 8-pii-scrubbing-compliance · ## 8. PII Scrubbing & Compliance · twin pii-scrubbing-compliance
decision/content-span-fields-never-stored · Content-named span fields never stored — The value of a content-named span field is never stored; its [redacted] marker is written at print time, and every other span field's value is held only in process memory until the span closes.
decision/scrub-by-field-name · Scrubbing by field name in the sink and the capture — Scrubbing is by field name: in escher's own sink by the content-named set, a closed span's line judged pair by pair by the same rule, and in the scoped capture for every source by the answer's own hidden set; a scrubbed field reads [redacted], the message and other fields kept as written.

## pii-scrubbing-compliance · ## 8. PII Scrubbing & Compliance · twin 8-pii-scrubbing-compliance
decision/closed-span-lines-judged-alike · Closed span lines judged by the same rule — A closed span's line is judged pair by pair by the target rule, and a span from a target outside the allowlist writes no byte at creation, record, enter, exit or close.
decision/driver-span-fields-print · Driver span fields tool and ending print — The tool_call span's fields tool and ending are not in the content-named set and print. · why: both are closed words; the closed-span line was ratified
decision/session-logs-carry-no-page-content · Session logs carry no page content — The session host's, the mcp role's and every client's stderr carries no element id, accessible name, typed text, recording path or recorded content at any log level, under the package-alone and the workspace build. · guard: host_log, mcp_log
contract/actions-answer/cause · the actions answer (actions) · cause — A closed cause word, left out when absent.
contract/actions-answer/content · the actions answer (actions) · content — Carries ids, owner ids, the table's verb words, closed cause words and booleans only, never a value, typed text, mask or NodeId; a boundary widening, ratified.
contract/actions-answer/logging · the actions answer (actions) · logging — The answer is kept whole in a recorded check and is never logged or fielded.
contract/actions-answer/shape · the actions answer (actions) · shape — Written as {"nodes":[{"id":…,"owner":…,"keyed":…,"verbs":[{"verb":…,"cause":…}]}]}.
decision/content-named-span-values-not-stored · Content-named span values never stored — The value of a content-named span field is never stored, its marker written at print time.
decision/escher-target-content-named-set · Escher targets redact the content-named set — Under an escher_ target the sink prints every field except url, href, src, html, text, value, attrs, path, request, error, password and panic.payload, which are redacted; escher alone, without the underscore, is not admitted.
contract/actions-answer/owner · the actions answer (actions) · owner — Left out when the node has no owner.
contract/snapshot-diff/open-list-row-bounds · the snapshot and the diff · open list row bounds — An open list's row crosses by its own bounds, differs naming bounds within its closed set, adding no field, key name, node key or carrier.
contract/snapshot-node/clickable · the snapshot node · clickable — A closed bool, true where an element's current listeners hold click and it is no disabled form control, carrying no page text; written as a node's "clickable" key.
contract/snapshot-text/clickable · the snapshot text · clickable — The token clickable stands after focused.
contract/snapshot/multiple-select-value · the snapshot · value of a multiple select — The first chosen option in tree order.

## 9-ci-integration · ## 9. CI Integration · twin ci-integration
decision/committed-kept-checks-clean · Committed kept checks hold no secrets — The kept checks committed in git hold no typed text, host path or secret.

## ci-integration · ## 9. CI Integration · twin 9-ci-integration
decision/committed-checks-hold-no-private-data · Committed checks hold no private data — A kept check committed to git is one of the repository's own scripted stand-Counter sessions and holds no typed text, host path, name, secret or screenshot step. · why: The repository is public.
decision/recording-bounds · Recording bounds — A recording ends on a failed write or a passed bound of 400 records or 1,048,576 bytes, and only then does the recorder remove it; escher otherwise never removes a recording.
decision/recording-dir-private · Recording directory private — A recording directory is created 0700 with its parent existing and is refused when open to its group or others; it holds call content at rest and never a log or event line; recording is unix only.
contract/recording-file/answer · The recording file · answer — Each call's answer is kept whole as the line the driver wrote, masked as on every carrier, with its accepted flag and digest; a shot's answer is held as width and height without png.
contract/recording-file/check-file · The recording file · check-{n}.json — Each recorded session gets one check-{n}.json, mode 0600, created new and never replaced, rewritten whole after every call through a temporary file and a rename.
contract/record-refusals/record-dir-not-private · Session recording refusals · record-dir-not-private — A recording directory open to its group or others is refused with record-dir-not-private.

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
