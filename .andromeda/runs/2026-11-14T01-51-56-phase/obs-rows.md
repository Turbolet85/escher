# obs-plan · the rows that bind 2026-11-14-seeded-randomness — picked 74 of 274 (43 by every draw) · rule any · pin 741

## 3-observability-harness-contract · ## 3. Observability Harness Contract
decision/answers-echo-nothing-typed · Answers echo nothing the caller typed — No answer, refusal, usage line or session error writes back anything the caller typed, and a password's or a file input's value appears only as the fixed mask.
decision/driver-library-installs-no-subscriber · Driver library installs no subscriber — The driver library, its command line, MCP server and coverage reader included, installs no subscriber and reads no environment variable; only a binary's own main installs the sink.
decision/headless-stand-no-sink · Headless stand runs with no sink — A headless boot made in process installs no telemetry subscriber and reads no environment variable; a headless boot made by the escher-session binary runs under that binary's sink. · except: The MCP tools check installs its own in-process recording subscriber, and the command-span check's two re-run children install escher's sink over an in-memory capture.
decision/log-check-failures-name-kind-count · Log check failures name a kind and a count — A failing log check's message names a kind and a count, never a needle or anything the host wrote.
decision/replay-children-uninstrumented · Replay children uninstrumented — A judged check is replayed on further escher-session mcp children spawned directly, without --record, with stderr to /dev/null. · why: uninstrumented by design
decision/sink-host-read-at-trace · Sink-installing host read at trace — A sink-installing host and its mcp role are read at trace under both builds, and must show no id, name or typed-sentinel needle in what they write. · guard: host_log, mcp_log
contract/kept-check/arguments · kept check file · arguments — Each answered call's arguments are written into the kept check, a file at rest, never a log line, field or print.
decision/cold-agent-record-owner-only · Cold-agent record directory owner-only — The live session runs mcp --record into target/cold-agent/record/, made owner-only (mkdir -m 700).
contract/escher-session-stdout/command-answer · escher-session stdout · command answer — Each command's answer, refusal or session-error object is one uncoloured line of JSON on stdout, an answer and never a log line; the host role ends quiet.
contract/kept-check/answer-digest · kept check file · answer digest — Each answered call records its answer's FNV-1a 64 digest.

## 4-span-trace-coverage · ## 4. Span / Trace Coverage · twin span-trace-coverage
decision/driver-emits-no-event · Driver emits no event and installs no subscriber — escher-driver emits no log event and installs no subscriber; its telemetry is its spans alone.
decision/driver-spans-carry-no-content · Driver spans carry no content — escher-driver's spans field only closed words, bools and counts, never the screen text, any argument, a shot byte, a lint finding, a part number, or any Command, Call, ArgValue, Outcome, Refusal or Fault.
contract/escher-driver-tool-call-span/ending · escher-driver tool_call span · ending — A closed word, one of accepted, refused or error.
contract/escher-driver-tool-call-span/span · escher-driver tool_call span · tool_call — One tool_call span is opened per MCP tools/call request, before its params are read, and held until its reply is written; it has two fields and no other.
contract/escher-driver-command-span/cause · escher-driver command span · cause — A field of the command span, a closed word, bool or count, recorded only where it applies.
contract/escher-driver-tool-call-span/command-child · escher-driver tool_call span · command child — An instance verb run through Session::run makes its command span the tool_call's child; the session verbs and help, answered by the server, leave a tool_call with no command child.

## span-trace-coverage · ## 4. Span / Trace Coverage · twin 4-span-trace-coverage
decision/command-span-no-call-content · No call content in the command span — The command span never fields an argument, an id, the screen text, an accessible name, an image or its size, lint findings, graph content, the session label, or any command, call, outcome or refusal value. · why: An argument fielded by its schema name, or a command or outcome fielded with Debug, would print an id, typed text, an accessible name or a control's value. · guard: stand_act_spans
decision/tool-call-span-two-closed-fields · tool_call span with tool and ending — Each MCP tools/call request writes one tool_call span carrying exactly two fields, tool and ending, each a closed word. · why: Ratified as the span's shape. · guard: stand_mcp_tools
contract/refusal-cause/malformed · Refusal cause names · malformed — A closed refusal class word recorded in the command span's cause field, returned by validation.
contract/tool-call-ending/refused · The tool_call span's ending · refused — The tool call was refused, its reply one text block with isError true.
contract/tool-call-span/ending · The MCP tool_call span · ending — One of three closed words saying how the call ended: `accepted` · `refused` · `error` (a session error or a protocol error).
contract/tool-call-span/span · The MCP tool_call span · span — One span, target escher_driver, name tool_call, level INFO, opened once per MCP tools/call request before its params are read and held until its reply is written.
contract/command-span/cause · The driver's command span · cause — Recorded at one site for every refusal the run returns: one of eight fixed cause names, class words holding nothing a call supplied.
contract/tool-call-ending/accepted · The tool_call span's ending · accepted — The tool call was accepted and answered.
contract/tool-call-ending/error · The tool_call span's ending · error — The tool call ended in a session error or a protocol error.
contract/tool-call-span/command-child · The MCP tool_call span · command child — An instance verb run through the session's run makes its command span the tool_call's child; the session verbs start, status, stop and help, answered by the server, leave a tool_call with no command child.

## 6-log-coverage · ## 6. Log Coverage · twin log-coverage
decision/escher-session-fixed-diagnostics · escher-session diagnostics are fixed strings — Every escher-session diagnostic is a fixed string holding no path, label, id or value, and no ending holds anything the caller typed.
decision/sink-target-allowlist · Sink target allowlist — escher's sink applies an allowlist of targets in its formatter: an engine target prints only the safe fields, the rest [redacted]; an escher_ target prints every field but the content-named set; any other target is dropped whole, not an empty line, at every level and whatever RUST_LOG names. · why: The accepted cost is that a third-party WARN or ERROR never reaches stderr, at any setting. · guard: telemetry_drop · except: The upstream apps' fmt::init() stdout subscribers and the WPT runner's env_logger stay unscrubbed.
contract/escher-log-line/scrub · escher's sink log line · field scrub — Every field of an event line and every pair of a span line passes the sink's scrub.
contract/escher-session-mcp/stdout · escher-session mcp role · stdout — The mcp role writes protocol lines only on stdout.

## log-coverage · ## 6. Log Coverage · twin 6-log-coverage
decision/fixed-diagnostics · Fixed diagnostics — Every escher-session diagnostic is a fixed string holding no path, label, id or value, and no ending holds anything the caller typed.
contract/command-span/field-count · The driver's command span · field count — The `command` span keeps its eight fields; the driver's fixed help and meaning texts add no log site or span field.
contract/command-span/line · The driver's command span · line — One closed-span line per command run, INFO at target `escher_driver`, `span="command"`, with the recorded ones of `verb`, `cause`, `settled`, `busy`, `passes`, `added`, `removed`, `changed`; no line at the default `warn`.
contract/escher-session-commands/help · escher-session commands · help — `help` (at most one `--session <dir>`, not read) writes the self-description's one line of JSON on stdout, status 0, reaching no session; `help` with an argument is refused `malformed`, status 1.
contract/escher-session-commands/open-recording-directory · escher-session commands · recording directory open to group or others — `start` ends with status 3 and nothing spawned, and `mcp --record` with status 3, its fixed message on stderr and stdout empty.
contract/escher-session-error-object/remedy · escher-session's error object · remedy — The error object carries a fixed `remedy` beside `kind` and `message`.
contract/escher-session-exit-codes/0 · escher-session exit codes · 0 — Accepted: one line of JSON on stdout, stderr empty with `RUST_LOG` unset; also `help`, the host role's quiet end and the `mcp` role's `Ok` end.
contract/escher-session-exit-codes/1 · escher-session exit codes · 1 — Refused: one line of JSON on stdout, stderr empty with `RUST_LOG` unset.
contract/host-wire/v3 · The escher-session host wire · v3 — Wire version `v3` carries `hello`, `stop` and `call`.
contract/span-line/scrub · The closed span's line · scrub — Every pair of the line, `span` included, passes the scrub under the span's target.
contract/tool-call-span/help · The driver's tool_call span · help — A `help` call's line reads `span="tool_call" tool="help" ending="accepted"` with no `command` line beside it.
contract/tool-call-span/line · The driver's tool_call span · line — One closed-span line per MCP tool call, INFO at target `escher_driver`, `span="tool_call"`, with the recorded ones of `tool` and `ending`, then `message="close"`, `time.busy`, `time.idle`; no line at the default `warn`.
decision/capability-statement-updated-by-hand · Capability statement updated by hand — A change to the driver's span set or to the scrub's limits is made in the published capability statement by hand. · why: The statement's pin reads none of its emitter claims.
decision/coverage-line-never-logged · Coverage line never logged — The handler coverage line is a command line answer only, never logged or fielded; each replayed step of a kept check writes its own command span line. · guard: a_coverage_run_logs_no_id_no_name_and_nothing_of_its_check_at_trace
contract/command-span/snapshot-part · The driver's command span · snapshot part read — A part read's line is `span="command" verb="snapshot"` (with `cause` when refused, a `part` outside 1 to 1000 refused `malformed`) and holds no part number, count, size or text.
contract/command-span/verb · The driver's command span · verb — The verb table's own word for a known verb, run or refused, one of its fourteen words; never the caller's verb text; unrecorded for a verb the table lacks; the three session-level words and the tool-level help appear only beside the cause unknown-verb.
contract/escher-session-commands/mcp · escher-session commands · mcp — `escher-session mcp` writes JSON-RPC protocol lines only on stdout and nothing at its end: status 0 on success, status 3 with the error's fixed message on stderr on failure; any argument but one `--record <dir>` is usage, status 2.
contract/escher-session-coverage/malformed · escher-session's coverage role · malformed — A file that cannot be read, is over 1,048,576 bytes, is no check or is a check of another app is refused `malformed`, status 1, holding nothing of the path or the file.
contract/escher-session-exit-codes/2 · escher-session exit codes · 2 — Usage: one fixed line on stderr built from `VERBS` and the six task names, stdout empty.
contract/escher-session-exit-codes/3 · escher-session exit codes · 3 — Session error: the error object on stdout and its fixed message on stderr.
contract/host-wire/recording-failed · The escher-session host wire · recording-failed — A call after which the host's recording ended is answered by the one fixed reply `recording-failed`.

## error-capture-reporting · ## 7. Error Capture & Reporting · twin 7-error-capture-reporting
contract/telemetry-scrub-set/error · escher-telemetry's scrub set · error — An `error` field is in the scrub set, so the sink prints its value as `[redacted]`; a failed fetch's error text reaches no carrier beyond its log line.

## 8-pii-scrubbing-compliance · ## 8. PII Scrubbing & Compliance · twin pii-scrubbing-compliance
decision/content-span-fields-never-stored · Content-named span fields never stored — The value of a content-named span field is never stored; its [redacted] marker is written at print time, and every other span field's value is held only in process memory until the span closes.
decision/cold-agent-kept-checks-at-rest · Cold-agent kept checks held at rest only — The live cold-agent session's kept checks live in an owner-only record directory, holding calls and answers at rest, never printed, copied or committed.
decision/scrub-at-sink-not-call-sites · Scrubbing at the sink, not at call sites — Scrubbing is applied in escher's own sink, a closed span's line judged pair by pair by the same rule, and no engine call site is edited.

## pii-scrubbing-compliance · ## 8. PII Scrubbing & Compliance · twin 8-pii-scrubbing-compliance
decision/content-named-span-values-not-stored · Content-named span values never stored — The value of a content-named span field is never stored, its marker written at print time.
decision/escher-target-content-named-set · Escher targets redact the content-named set — Under an escher_ target the sink prints every field except url, href, src, html, text, value, attrs, path, request, error and panic.payload, which are redacted; escher alone, without the underscore, is not admitted.
decision/session-logs-carry-no-page-content · Session logs carry no page content — The session host's, the mcp role's and every client's stderr carries no element id, accessible name, typed text, recording path or recorded content at any log level, under the package-alone and the workspace build. · guard: host_log, mcp_log
contract/snapshot-node/clickable · the snapshot node · clickable — A closed bool, true where an element's current listeners hold click and it is no disabled form control, carrying no page text; written as a node's "clickable" key.
contract/snapshot-text/clickable · the snapshot text · clickable — The token clickable stands after focused.
decision/closed-span-lines-judged-alike · Closed span lines judged by the same rule — A closed span's line is judged pair by pair by the target rule, and a span from a target outside the allowlist writes no byte at creation, record, enter, exit or close.
decision/engine-target-safe-fields · Engine targets print safe fields only — Under an engine target (blitz, dioxus_native, stylo_taffy, accesskit_xplat, debug_timer, js_console) the sink prints only node_id, status, waiting_nodes, property, log.module_path, log.file and log.line, every other field as {name}=[redacted].
contract/snapshot-diff/open-list-row-bounds · the snapshot and the diff · open list row bounds — An open list's row crosses by its own bounds, differs naming bounds within its closed set, adding no field, key name, node key or carrier.
contract/snapshot/multiple-select-value · the snapshot · value of a multiple select — The first chosen option in tree order.

## 9-ci-integration · ## 9. CI Integration · twin ci-integration
decision/committed-kept-checks-clean · Committed kept checks hold no secrets — The kept checks committed in git hold no typed text, host path or secret.
decision/local-state-never-uploaded · Local state stays local — The agent-run and cold-agent state, a session's state directory and a recording directory stay local and are uploaded by no CI leg.

## ci-integration · ## 9. CI Integration · twin 9-ci-integration
decision/committed-checks-hold-no-private-data · Committed checks hold no private data — A kept check committed to git is one of the repository's own scripted stand-Counter sessions and holds no typed text, host path, name, secret or screenshot step. · why: The repository is public.
decision/recording-bounds · Recording bounds — A recording ends on a failed write or a passed bound of 400 records or 1,048,576 bytes, and only then does the recorder remove it; escher otherwise never removes a recording.
decision/recording-dir-private · Recording directory private — A recording directory is created 0700 with its parent existing and is refused when open to its group or others; it holds call content at rest and never a log or event line; recording is unix only.
contract/record-refusals/record-dir-not-private · Session recording refusals · record-dir-not-private — A recording directory open to its group or others is refused with record-dir-not-private.
contract/recording-file/check-file · The recording file · check-{n}.json — Each recorded session gets one check-{n}.json, mode 0600, created new and never replaced, rewritten whole after every call through a temporary file and a rename.
decision/cold-agent-kept-checks-stay-local · Cold-agent kept checks stay local — The cold-agent pipe's live run keeps its checks in an owner-only (mode 700) recording directory; the kept checks are never printed, copied, uploaded by a CI leg or committed, and are removed with the rest of the state at cleanup; the replay children run without recording.
contract/recording-file/answer · The recording file · answer — Each call's answer is kept whole as the line the driver wrote, masked as on every carrier, with its accepted flag and digest; a shot's answer is held as width and height without png.
contract/recording-file/typed-text · The recording file · typed text — Typed text is held unmasked, except a type to a masked or unresolved control, held as masked.

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
