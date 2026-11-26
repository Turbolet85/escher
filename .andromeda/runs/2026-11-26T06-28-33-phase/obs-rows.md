# obs-plan · the rows that bind 2026-11-26-canvas-keyboard-path — picked 54 of 299 (33 by every draw) · rule any · pin 4036

## 3-observability-harness-contract · ## 3. Observability Harness Contract
contract/command-answer/graphed · driver command answer · nodes and states — A graph answer is one JSON object of nodes and states, a part adding its part number and part count, holding ids, component names, registered state and handler event names, closed flag words and booleans, never a value.
contract/coverage-role-stdout/coverage-line · escher-session coverage role stdout · coverage line — The coverage line, holding ids, owner ids, event names and two counts, is written on stdout as an answer and never a log line; a malformed refusal holds nothing of the path.
contract/escher-session-stdout/command-answer · escher-session stdout · command answer — Each command's answer, refusal or session-error object is one uncoloured line of JSON on stdout, an answer and never a log line; the host role ends quiet.
decision/answers-echo-nothing-typed · Answers echo nothing the caller typed — No answer, refusal, usage line or session error writes back anything the caller typed, and a password's or a file input's value appears only as the fixed mask. · except: A caused item's message, chain and kept fields hold what their author wrote, a field of the answer's own hidden set reading [redacted] and an error field shown as written, cut at the text bound.
decision/debug-types-never-logged · Answer-bearing types never logged — Types that derive Debug and hold ids, accessible names, control values or page text are never logged, fielded or printed with Debug; a command's outcome is written in one place, as the command's answer.
decision/one-span-per-command · One span per driver command — Each driver command opens exactly one command span, which writes only where a process has installed a sink at info or below.
decision/part-answer-never-logged · A part's answer is never logged — A part's answer, a snapshot part ({"text":…,"part":n,"parts":m}), a graph part ({"nodes":[…],"states":[…],"part":n,"parts":m}) or a canvas part, is a command's answer, written over MCP as its JSON line, and is never logged or fielded.

## 4-span-trace-coverage · ## 4. Span / Trace Coverage · twin span-trace-coverage
decision/canvas-read-opens-no-span · The canvas read opens no span — The driver's in-process canvas read opens no span of its own, as the layout read does not; reached by the canvas verb it runs inside that call's command span, and the lint's canvas label reading opens no span and logs nothing, running inside the lint call's command span. · why: No span of its own by design.
decision/driver-spans-carry-no-content · Driver spans carry no content — escher-driver's spans field only closed words, bools and counts, never the screen text, any argument, a shot byte, a lint finding, a part number, or any Command, Call, ArgValue, Outcome, Refusal or Fault.
contract/escher-driver-command-span/added · escher-driver command span · added — A field of the command span, a closed word, bool or count, recorded only where it applies.
contract/escher-driver-command-span/cause · escher-driver command span · cause — A field of the command span, a closed word, bool or count, recorded only where it applies.
contract/escher-driver-command-span/changed · escher-driver command span · changed — A field of the command span, a closed word, bool or count, recorded only where it applies.
contract/escher-driver-command-span/passes · escher-driver command span · passes — A field of the command span, a closed word, bool or count, recorded only where it applies.
contract/escher-driver-command-span/removed · escher-driver command span · removed — A field of the command span, a closed word, bool or count, recorded only where it applies.
contract/escher-driver-command-span/settled · escher-driver command span · settled — A field of the command span, a closed word, bool or count, recorded only where it applies.
contract/escher-driver-command-span/verb · escher-driver command span · verb — A field of the command span, a closed word, bool or count, recorded only where it applies.
contract/escher-driver-command-span/busy · escher-driver command span · busy — A field of the command span, a closed word, bool or count, recorded only where it applies.
contract/escher-driver-command-span/span · escher-driver command span · command — Every call handed to Session::run opens one INFO span named command at target escher_driver.

## span-trace-coverage · ## 4. Span / Trace Coverage · twin 4-span-trace-coverage
decision/command-span-no-call-content · No call content in the command span — The command span never fields an argument, an id, the screen text, an accessible name, an image or its size, lint findings, graph content, the session label, or any command, call, outcome or refusal value. · why: An argument fielded by its schema name, or a command or outcome fielded with Debug, would print an id, typed text, an accessible name or a control's value. · guard: stand_act_spans
contract/command-span/added · The driver's command span · added — The length of the returned diff's added list, recorded from an acting verb's outcome.
contract/command-span/cause · The driver's command span · cause — Recorded at one site for every refusal the run returns: one of eight fixed cause names, class words holding nothing a call supplied.
contract/command-span/changed · The driver's command span · changed — The length of the returned diff's changed list, recorded from an acting verb's outcome.
contract/command-span/passes · The driver's command span · passes — Settle's pass count, recorded from an acting verb's outcome when settle answered quiet; on a step that waited on a load, the settle that went quiet.
contract/command-span/removed · The driver's command span · removed — The length of the returned diff's removed list, recorded from an acting verb's outcome.
contract/command-span/settled · The driver's command span · settled — Recorded from an acting verb's outcome only; snapshot, screenshot, lint, graph, actions, layout, canvas and a part read record verb alone, and a refusal records verb and cause alone.
contract/refusal-cause/covered · Refusal cause names · covered — A closed refusal class word recorded in the command span's cause field, returned by the executor.
contract/refusal-cause/disabled · Refusal cause names · disabled — A closed refusal class word recorded in the command span's cause field, returned by the executor.
contract/refusal-cause/off-screen · Refusal cause names · off-screen — A closed refusal class word recorded in the command span's cause field, returned by the executor; an element screenshot that crops to no pixel is refused off-screen.
contract/command-span/busy · The driver's command span · busy — One of three busy-class words, the schema's word for the busy class, recorded from an acting verb's outcome when the outcome carries one.
contract/command-span/span · The driver's command span · span — One span per call handed to the session's run, target escher_driver, name command, level INFO, entered before validation and held until the run returns, so a refusal is inside it.

## 6-log-coverage · ## 6. Log Coverage · twin log-coverage
decision/declared-node-read-logs-nothing · The declared-node read logs nothing — The in-process read of a canvas's declared virtual nodes, its validation and its answer, and the virtual nodes' place in the accessibility tree, the snapshot and a click on one, have no log site, print, span or field; a refused declaration holds a closed reason and its index alone.

## log-coverage · ## 6. Log Coverage · twin 6-log-coverage
decision/coverage-line-never-logged · Coverage line never logged — The handler coverage line is a command line answer only, never logged or fielded; each replayed step of a kept check writes its own command span line. · guard: a_coverage_run_logs_no_id_no_name_and_nothing_of_its_check_at_trace
decision/reached-record-ids-and-events-only · Reached record holds ids and event names only — The reached record holds stable ids and event names only, no node number, and its debug form prints its count alone. · why: the reached module's tests pin id and event rows and the_records_debug_prints_its_count_alone; handler coverage reads the record through reached_handlers
contract/command-span/canvas · The driver's command span · canvas — A canvas's line, a part read's included, is `span="command" verb="canvas"` (with `cause` when refused) and holds no id, full flag, part number or anything of its answer.
contract/command-span/field-count · The driver's command span · field count — The `command` span keeps its eight fields; the driver's fixed help and meaning texts add no log site or span field.
contract/command-span/line · The driver's command span · line — One closed-span line per command run, INFO at target `escher_driver`, `span="command"`, with the recorded ones of `verb`, `cause`, `settled`, `busy`, `passes`, `added`, `removed`, `changed`; no line at the default `warn`.
decision/graph-answer-never-logged · Graph answer never logged — The graph answer is written as a command answer alone, never logged or fielded.
decision/no-state-page-skips-registry · Page without registered state skips the registry — A page with no registered state reaches no signal registry: its events run their handlers and no dispatch is recorded. · why: signal_registry's a_page_with_no_registered_state_has_no_registry pins no registry context and a click that still acts; the load page's handler-coverage check relies on it
contract/command-span/layout · The driver's command span · layout — A layout's line is `span="command" verb="layout"` (with `cause` when refused) and holds nothing of its answer; the layout read the call reaches opens no span of its own.
contract/command-span/verb · The driver's command span · verb — The verb table's own word for a known verb, run or refused, one of its sixteen words; never the caller's verb text; unrecorded for a verb the table lacks; the three session-level words and the tool-level help appear only beside the cause unknown-verb.
contract/graph-answer/shape · The graph answer · shape — `{"nodes":[…],"states":[…]}` for a whole graph; a part read is `{"nodes":[…],"states":[…],"part":n,"parts":m}`, its states held in part 1 alone.

## pii-scrubbing-compliance · ## 8. PII Scrubbing & Compliance · twin 8-pii-scrubbing-compliance
decision/driver-span-fields-print · Driver span fields tool and ending print — The tool_call span's fields tool and ending are not in the content-named set and print. · why: both are closed words; the closed-span line was ratified
contract/coverage-line/content · the coverage line (escher-session coverage) · content — Carries stable ids, owner ids, handler event names and two counts only, never a value or typed text; a canvas virtual node's click is listed under its {owner}///{key} id, owned by its canvas, and a canvas click inside a node is counted for the canvas alone; a boundary widening, ratified.
contract/coverage-line/owner · The handler coverage line (escher-session coverage) · owner — `owner` is left out when absent.
contract/coverage-line/shape · The handler coverage line (escher-session coverage) · shape — Written by the crate's hand writer as `{"unexercised":[{"id":…,"owner":…,"event":…}],"checks":n,"held":m}`, a command line answer never logged or fielded.
contract/graph-answer/content · the UI graph answer (graph) · content — Carries stable ids, component names, registered state names, handler event names, the closed flag words undeclared and unconfirmed, booleans, and for a part its part number and part count only, never a state's value; a boundary widening, ratified.
decision/session-logs-carry-no-page-content · Session logs carry no page content — The session host's, the mcp role's and every client's stderr carries no element id, accessible name, typed text, recording path or recorded content at any log level, under the package-alone and the workspace build. · guard: host_log, mcp_log
contract/actions-answer/cause · the actions answer (actions) · cause — A closed cause word, left out when absent.
contract/actions-answer/content · the actions answer (actions) · content — Carries ids, owner ids, the table's verb words, closed cause words and booleans only, never a value, typed text, mask or NodeId; a boundary widening, ratified.
contract/actions-answer/logging · the actions answer (actions) · logging — The answer is kept whole in a recorded check and is never logged or fielded.
contract/coverage-line/logging · the coverage line (escher-session coverage) · logging — The line is a command answer, never logged or fielded.
contract/snapshot-node/clickable · the snapshot node · clickable — A closed bool, true where an element's current listeners hold click and it is no disabled form control, carrying no page text; written as a node's "clickable" key.
contract/snapshot-text/clickable · the snapshot text · clickable — The token clickable stands after focused.

## ci-integration · ## 9. CI Integration · twin 9-ci-integration
decision/ci-log-artifact-on-failure · Per-leg CI log uploaded on failure — Each CI leg's merged stdout and stderr is written to its own log, truncated at the leg's start, and uploaded only on failure as artifact ci-log-{job id}, kept 7 days; it is unscrubbed build output holding no user data.

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
