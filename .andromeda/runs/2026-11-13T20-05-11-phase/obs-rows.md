# obs-plan · the rows that bind 2026-11-13-cognitive-load-corrective — picked 121 of 274 (102 by every draw) · rule any · pin 627

## 2-telemetry-strategy · ## 2. Telemetry Strategy · twin telemetry-strategy
decision/escher-crates-always-trace · Escher crates take tracing unconditionally — The escher telemetry and driver crates take tracing with no feature, so the startup and panic events and the command span are always compiled into the seven_guis package's two native binaries, while their engine features stay off in a package-alone build.
decision/telemetry-via-tracing · Telemetry through the tracing crate — Telemetry is the tracing crate's event and span macros; the engine and upstream crates compile their log sites only with the tracing cargo feature, each call a no-op when the feature is off.
decision/engine-tracing-by-invocation · Engine tracing set by the build's invocation — A build that resolves the whole workspace turns the engine's tracing feature on while a package-alone build of the test crate or seven_guis leaves it off, so under the workspace build a test child that installs escher's sink at info also receives engine events.

## telemetry-strategy · ## 2. Telemetry Strategy · twin 2-telemetry-strategy
decision/engine-tracing-feature-gated · Engine tracing behind a cargo feature — In the engine and upstream crates, tracing is compiled in only with the tracing cargo feature.
decision/escher-crates-tracing-ungated · escher crates take tracing ungated — escher-telemetry and escher-driver take tracing with no feature gate, so the startup and panic events and the driver's spans are always compiled into the binaries that use them.

## 3-observability-harness-contract · ## 3. Observability Harness Contract
decision/answers-echo-nothing-typed · Answers echo nothing the caller typed — No answer, refusal, usage line or session error writes back anything the caller typed, and a password's or a file input's value appears only as the fixed mask.
decision/debug-types-never-logged · Answer-bearing types never logged — Types that derive Debug and hold ids, accessible names, control values or page text are never logged, fielded or printed with Debug; a command's outcome is written in one place, as the command's answer.
decision/driver-library-installs-no-subscriber · Driver library installs no subscriber — The driver library, its command line, MCP server and coverage reader included, installs no subscriber and reads no environment variable; only a binary's own main installs the sink.
decision/one-span-per-command · One span per driver command — Each driver command opens exactly one command span, which writes only where a process has installed a sink at info or below.
decision/part-answer-never-logged · A part's answer is never logged — A part's answer ({"text":…,"part":n,"parts":m}) is a command's answer, written over MCP as its JSON line, and is never logged or fielded.
decision/session-id-record-unprinted · Session id record kept private — The session's record of the ids its screens have read is bounded, has no reader outside the driver crate and no Debug, and is never printed, logged or fielded.
contract/command-answer/graphed · driver command answer · nodes and states — A graph answer is one JSON object of nodes and states holding ids, component names, registered state and handler event names, closed flag words and booleans, never a value.
contract/command-answer/linted · driver command answer · findings — A lint answer is written as one JSON object with a findings array, kept whole in a recorded check and never logged or fielded.
contract/command-answer/shot · driver command answer · shot — A screenshot answer holds width, height and the PNG; over MCP it is an image block followed by the JSON line without the PNG, and is never logged or fielded.
contract/coverage-role-stdout/coverage-line · escher-session coverage role stdout · coverage line — The coverage line, holding ids, owner ids, event names and two counts, is written on stdout as an answer and never a log line; a malformed refusal holds nothing of the path.
decision/checks-silent · Silent checks and shared modules — Checks, their shared modules and the stand-ins they use install no subscriber, read no environment variable and print nothing, their readings going into assertions only, and the code they pin adds no log site, span, field or env read. · except: Checks whose subject is telemetry install escher's sink, the MCP tools check its own recorder, the paint-tree bench reads one opt-in env var, and a re-run child prints ids its parent reads.
decision/sink-host-read-at-trace · Sink-installing host read at trace — A sink-installing host and its mcp role are read at trace under both builds, and must show no id, name or typed-sentinel needle in what they write. · guard: host_log, mcp_log
contract/escher-session-stdout/command-answer · escher-session stdout · command answer — Each command's answer, refusal or session-error object is one uncoloured line of JSON on stdout, an answer and never a log line; the host role ends quiet.
contract/kept-check/arguments · kept check file · arguments — Each answered call's arguments are written into the kept check, a file at rest, never a log line, field or print.
contract/script-diagnostics-drain/take-messages · Embedder drain of script diagnostics · take_messages — Embedders drain JS messages with take_messages.

## 4-span-trace-coverage · ## 4. Span / Trace Coverage · twin span-trace-coverage
decision/driver-emits-no-event · Driver emits no event and installs no subscriber — escher-driver emits no log event and installs no subscriber; its telemetry is its spans alone.
decision/driver-spans-carry-no-content · Driver spans carry no content — escher-driver's spans field only closed words, bools and counts, never the screen text, any argument, a shot byte, a lint finding, a part number, or any Command, Call, ArgValue, Outcome, Refusal or Fault.
contract/escher-driver-command-span/added · escher-driver command span · added — A field of the command span, a closed word, bool or count, recorded only where it applies.
contract/escher-driver-command-span/busy · escher-driver command span · busy — A field of the command span, a closed word, bool or count, recorded only where it applies.
contract/escher-driver-command-span/cause · escher-driver command span · cause — A field of the command span, a closed word, bool or count, recorded only where it applies.
contract/escher-driver-command-span/changed · escher-driver command span · changed — A field of the command span, a closed word, bool or count, recorded only where it applies.
contract/escher-driver-command-span/passes · escher-driver command span · passes — A field of the command span, a closed word, bool or count, recorded only where it applies.
contract/escher-driver-command-span/removed · escher-driver command span · removed — A field of the command span, a closed word, bool or count, recorded only where it applies.
contract/escher-driver-command-span/settled · escher-driver command span · settled — A field of the command span, a closed word, bool or count, recorded only where it applies.
contract/escher-driver-command-span/span · escher-driver command span · command — Every call handed to Session::run opens one INFO span named command at target escher_driver.
contract/escher-driver-command-span/verb · escher-driver command span · verb — A field of the command span, a closed word, bool or count, recorded only where it applies.
contract/escher-driver-tool-call-span/command-child · escher-driver tool_call span · command child — An instance verb run through Session::run makes its command span the tool_call's child; the session verbs and help, answered by the server, leave a tool_call with no command child.
contract/escher-driver-tool-call-span/ending · escher-driver tool_call span · ending — A closed word, one of accepted, refused or error.
contract/escher-driver-tool-call-span/span · escher-driver tool_call span · tool_call — One tool_call span is opened per MCP tools/call request, before its params are read, and held until its reply is written; it has two fields and no other.
contract/escher-driver-tool-call-span/tool · escher-driver tool_call span · tool — The verb table's word for the called tool, a closed word.

## span-trace-coverage · ## 4. Span / Trace Coverage · twin 4-span-trace-coverage
decision/advance-accepted-every-session · advance accepted on every session — The advance verb is accepted on every session and records the acting fields like every acting verb, with no cause. · why: Ratified.
decision/command-span-no-call-content · No call content in the command span — The command span never fields an argument, an id, the screen text, an accessible name, an image or its size, lint findings, graph content, the session label, or any command, call, outcome or refusal value. · why: An argument fielded by its schema name, or a command or outcome fielded with Debug, would print an id, typed text, an accessible name or a control's value. · guard: stand_act_spans
decision/hosted-call-through-session-run · Hosted call runs through the session's run alone — A hosted part read runs through the session's run alone, so its spans are command spans recording verb alone. · why: stand_act_spans pins a part read's command line carrying its verb alone, and the session host answers each call through Session::run with no span of its own
decision/tool-call-span-two-closed-fields · tool_call span with tool and ending — Each MCP tools/call request writes one tool_call span carrying exactly two fields, tool and ending, each a closed word. · why: Ratified as the span's shape. · guard: stand_mcp_tools
contract/command-span/added · The driver's command span · added — The length of the returned diff's added list, recorded from an acting verb's outcome.
contract/command-span/busy · The driver's command span · busy — One of three busy-class words, the schema's word for the busy class, recorded from an acting verb's outcome when the outcome carries one.
contract/command-span/cause · The driver's command span · cause — Recorded at one site for every refusal the run returns: one of eight fixed cause names, class words holding nothing a call supplied.
contract/command-span/changed · The driver's command span · changed — The length of the returned diff's changed list, recorded from an acting verb's outcome.
contract/command-span/passes · The driver's command span · passes — Settle's pass count, recorded from an acting verb's outcome when settle answered quiet; on a step that waited on a load, the settle that went quiet.
contract/command-span/removed · The driver's command span · removed — The length of the returned diff's removed list, recorded from an acting verb's outcome.
contract/command-span/settled · The driver's command span · settled — Recorded from an acting verb's outcome only; snapshot, screenshot, lint, graph, actions and a part read record verb alone, and a refusal records verb and cause alone.
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
contract/tool-call-span/command-child · The MCP tool_call span · command child — An instance verb run through the session's run makes its command span the tool_call's child; the session verbs start, status, stop and help, answered by the server, leave a tool_call with no command child.
contract/tool-call-span/ending · The MCP tool_call span · ending — One of three closed words saying how the call ended: `accepted` · `refused` · `error` (a session error or a protocol error).
contract/tool-call-span/span · The MCP tool_call span · span — One span, target escher_driver, name tool_call, level INFO, opened once per MCP tools/call request before its params are read and held until its reply is written.
contract/tool-call-span/tool · The MCP tool_call span · tool — The verb table's word for the tool, one of its fourteen; unrecorded for an unknown tool.

## 6-log-coverage · ## 6. Log Coverage · twin log-coverage
decision/sink-target-allowlist · Sink target allowlist — escher's sink applies an allowlist of targets in its formatter: an engine target prints only the safe fields, the rest [redacted]; an escher_ target prints every field but the content-named set; any other target is dropped whole, not an empty line, at every level and whatever RUST_LOG names. · why: The accepted cost is that a third-party WARN or ERROR never reaches stderr, at any setting. · guard: telemetry_drop · except: The upstream apps' fmt::init() stdout subscribers and the WPT runner's env_logger stay unscrubbed.
contract/escher-driver-span-lines/default-level · escher-driver span lines · default warn level — The driver's command and tool_call span lines print only in a process that installs escher's sink at info or below; at the default warn there is no line.
decision/escher-session-fixed-diagnostics · escher-session diagnostics are fixed strings — Every escher-session diagnostic is a fixed string holding no path, label, id or value, and no ending holds anything the caller typed.
contract/escher-log-line/span-line · escher's sink log line · closed span line — A closed span's line is {time} {LEVEL} {target} service.name=… service.version=… span={the span's name}, then the span's own fields in the order recorded, then message, time.busy and time.idle.
contract/escher-log-line/span-on-close-only · escher's sink log line · span line timing — A span line is written only when a span whose target the allowlist admits closes, never when it is created, recorded to, entered or exited; an event's line is unchanged inside a span.

## log-coverage · ## 6. Log Coverage · twin 6-log-coverage
decision/actions-read-changes-nothing · Actions read changes nothing in the document — The actions read acts on nothing and changes nothing in the document. · why: stand_actions' the_read_changes_nothing_and_holds_no_typed_text_or_mask pins no change mark, unchanged snapshot, focus and empty diff
decision/coverage-line-never-logged · Coverage line never logged — The handler coverage line is a command line answer only, never logged or fielded; each replayed step of a kept check writes its own command span line. · guard: a_coverage_run_logs_no_id_no_name_and_nothing_of_its_check_at_trace
decision/graph-answer-never-logged · Graph answer never logged — The graph answer is written as a command answer alone, never logged or fielded.
decision/no-state-page-skips-registry · Page without registered state skips the registry — A page with no registered state reaches no signal registry: its events run their handlers and no dispatch is recorded. · why: signal_registry's a_page_with_no_registered_state_has_no_registry pins no registry context and a click that still acts; the load page's handler-coverage check relies on it
decision/reached-record-ids-and-events-only · Reached record holds ids and event names only — The reached record holds stable ids and event names only, no node number, and its debug form prints its count alone. · why: the reached module's tests pin id and event rows and the_records_debug_prints_its_count_alone; handler coverage reads the record through reached_handlers
contract/command-span/actions · The driver's command span · actions — An actions line is `span="command" verb="actions"` (with `cause` when refused, an argument refused `malformed`) and holds no node, id, owner, verb word or cause word of its answer.
contract/command-span/field-count · The driver's command span · field count — The `command` span keeps its eight fields; the driver's fixed help and meaning texts add no log site or span field.
contract/command-span/graph · The driver's command span · graph — A graph's line is `span="command" verb="graph"` (with `cause` when refused, an argument refused `malformed`) and holds no node, state, id or name.
contract/command-span/line · The driver's command span · line — One closed-span line per command run, INFO at target `escher_driver`, `span="command"`, with the recorded ones of `verb`, `cause`, `settled`, `busy`, `passes`, `added`, `removed`, `changed`; no line at the default `warn`.
contract/command-span/lint · The driver's command span · lint — A lint's line is `span="command" verb="lint"` (with `cause` when refused) and holds no finding, id, bound or count.
contract/command-span/screenshot · The driver's command span · screenshot — A shot's line is `span="command" verb="screenshot"` (with `cause` when refused) and holds no image byte, size or dimension.
contract/command-span/snapshot-part · The driver's command span · snapshot part read — A part read's line is `span="command" verb="snapshot"` (with `cause` when refused, a `part` outside 1 to 1000 refused `malformed`) and holds no part number, count, size or text.
contract/command-span/verb · The driver's command span · verb — The verb table's own word for a known verb, run or refused, one of its fourteen words; never the caller's verb text; unrecorded for a verb the table lacks; the three session-level words and the tool-level help appear only beside the cause unknown-verb.
contract/escher-session-coverage/accepted · escher-session's coverage role · accepted — One uncoloured JSON coverage line on stdout, status 0.
contract/graph-answer/shape · The graph answer · shape — `{"nodes":[…],"states":[…]}`.
contract/tool-call-span/help · The driver's tool_call span · help — A `help` call's line reads `span="tool_call" tool="help" ending="accepted"` with no `command` line beside it.
contract/tool-call-span/line · The driver's tool_call span · line — One closed-span line per MCP tool call, INFO at target `escher_driver`, `span="tool_call"`, with the recorded ones of `tool` and `ending`, then `message="close"`, `time.busy`, `time.idle`; no line at the default `warn`.
decision/closed-span-lines · Closed spans as a second record class — The sink writes one line for each closed span whose target the allowlist admits, and no line when a span is created, recorded to, entered or exited. · why: Ratified as a boundary widening of the log record classes.
decision/fixed-diagnostics · Fixed diagnostics — Every escher-session diagnostic is a fixed string holding no path, label, id or value, and no ending holds anything the caller typed.
contract/span-line/field-order · The closed span's line · field order — `{time} {LEVEL} {target} service.name=… service.version=… span={the span's name}`, then the span's own fields in the order recorded, then `message`, `time.busy` and `time.idle`.
contract/span-line/level-and-target · The closed span's line · level and target — The line's level and target are the span's own.

## 8-pii-scrubbing-compliance · ## 8. PII Scrubbing & Compliance · twin pii-scrubbing-compliance
decision/scrub-at-sink-not-call-sites · Scrubbing at the sink, not at call sites — Scrubbing is applied in escher's own sink, a closed span's line judged pair by pair by the same rule, and no engine call site is edited.
decision/scrub-sink-reach · Reach of the scrubbing sink — The scrubbing sink is installed by the two binaries of the seven_guis package; engine events reach it only when an engine tracing feature is on. · except: The upstream apps' stdout subscribers and the WPT runner's env_logger stay unscrubbed.
decision/content-span-fields-never-stored · Content-named span fields never stored — The value of a content-named span field is never stored; its [redacted] marker is written at print time, and every other span field's value is held only in process memory until the span closes.

## pii-scrubbing-compliance · ## 8. PII Scrubbing & Compliance · twin 8-pii-scrubbing-compliance
decision/driver-span-fields-print · Driver span fields tool and ending print — The tool_call span's fields tool and ending are not in the content-named set and print. · why: both are closed words; the closed-span line was ratified
decision/engine-target-safe-fields · Engine targets print safe fields only — Under an engine target (blitz, dioxus_native, stylo_taffy, accesskit_xplat, debug_timer, js_console) the sink prints only node_id, status, waiting_nodes, property, log.module_path, log.file and log.line, every other field as {name}=[redacted].
decision/open-list-shot-carries-labels · Open list shot carries labels as pixels — A shot of an open list carries a page's option labels and group labels as pixels, the shot's own fields unmoved. · why: a boundary widening, ratified
decision/optgroup-label-crossing · Optgroup label as group name and painted header — An optgroup's non-empty label is its group name in the accessibility tree and is painted as a group header in an open list, leaving as pixels in a shot and on no log line or field. · why: inside the ratified name crossing; a boundary widening, ratified
decision/select-chosen-set-never-logged · Select chosen set never logged — A multiple select's chosen set leaves only as form entries, the snapshot's value and the options' tree nodes, never as a log line or field.
decision/select-list-label-never-logged · Select list row label never logged — A select list row's label holds page text in a public read type, is never logged, and leaves only as a painted pixel and the snapshot's existing name.
decision/session-logs-carry-no-page-content · Session logs carry no page content — The session host's, the mcp role's and every client's stderr carries no element id, accessible name, typed text, recording path or recorded content at any log level, under the package-alone and the workspace build. · guard: host_log, mcp_log
decision/shot-image-leaves-only-in-answer · Shot image leaves only in its answer — A shot's image is screen content that leaves only in the command's answer, never on a log line. · guard: host_log, mcp_log
contract/actions-answer/cause · the actions answer (actions) · cause — A closed cause word, left out when absent.
contract/actions-answer/content · the actions answer (actions) · content — Carries ids, owner ids, the table's verb words, closed cause words and booleans only, never a value, typed text, mask or NodeId; a boundary widening, ratified.
contract/actions-answer/logging · the actions answer (actions) · logging — The answer is kept whole in a recorded check and is never logged or fielded.
contract/actions-answer/owner · the actions answer (actions) · owner — Left out when the node has no owner.
contract/actions-answer/shape · the actions answer (actions) · shape — Written as {"nodes":[{"id":…,"owner":…,"keyed":…,"verbs":[{"verb":…,"cause":…}]}]}.
contract/coverage-line/content · the coverage line (escher-session coverage) · content — Carries stable ids, owner ids, handler event names and two counts only, never a value or typed text; a boundary widening, ratified.
contract/coverage-line/logging · the coverage line (escher-session coverage) · logging — The line is a command answer, never logged or fielded.
contract/coverage-line/owner · The handler coverage line (escher-session coverage) · owner — `owner` is left out when absent.
contract/coverage-line/shape · The handler coverage line (escher-session coverage) · shape — Written by the crate's hand writer as `{"unexercised":[{"id":…,"owner":…,"event":…}],"checks":n,"held":m}`, a command line answer never logged or fielded.
contract/graph-answer/content · the UI graph answer (graph) · content — Carries stable ids, component names, registered state names, handler event names, the closed flag words undeclared and unconfirmed, and booleans only, never a state's value; a boundary widening, ratified.
contract/multiple-select-form-entries/chosen-option · multiple select form entries · chosen option — On input and change, one (name, FormValue::Text(value)) entry per chosen option.
contract/page-events/checkbox-change · page script events · change on a checkbox — The engine always sends change after input with the same value, carrying the value class input carries.
contract/page-events/radio-change · page script events · change on a radio — The engine sends change after input with the same value only where the radio was not checked before the click.
contract/page-events/single-change · page script events · change count — A page's script hears one change, from the engine, the script runtime synthesising none.
contract/snapshot-node/clickable · the snapshot node · clickable — A closed bool, true where an element's current listeners hold click and it is no disabled form control, carrying no page text; written as a node's "clickable" key.
contract/snapshot-text/clickable · the snapshot text · clickable — The token clickable stands after focused.
contract/snapshot/multiple-select-value · the snapshot · value of a multiple select — The first chosen option in tree order.
decision/closed-span-lines-judged-alike · Closed span lines judged by the same rule — A closed span's line is judged pair by pair by the target rule, and a span from a target outside the allowlist writes no byte at creation, record, enter, exit or close.
decision/content-named-span-values-not-stored · Content-named span values never stored — The value of a content-named span field is never stored, its marker written at print time.
decision/escher-target-content-named-set · Escher targets redact the content-named set — Under an escher_ target the sink prints every field except url, href, src, html, text, value, attrs, path, request, error and panic.payload, which are redacted; escher alone, without the underscore, is not admitted.
decision/state-registry-holds-no-values · State registry holds no values — The state registry holds each registered state's name, declarations, write records and dispatch chain, never a state's value; its Debug prints counts alone and it holds no log site, print or env read.
contract/snapshot-diff/open-list-row-bounds · the snapshot and the diff · open list row bounds — An open list's row crosses by its own bounds, differs naming bounds within its closed set, adding no field, key name, node key or carrier.

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
