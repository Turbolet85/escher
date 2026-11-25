# obs-plan · the rows that bind 2026-11-25-virtual-nodes-as-elements — picked 88 of 299 (53 by every draw) · rule any · pin 3791

## 3-observability-harness-contract · ## 3. Observability Harness Contract
decision/answers-echo-nothing-typed · Answers echo nothing the caller typed — No answer, refusal, usage line or session error writes back anything the caller typed, and a password's or a file input's value appears only as the fixed mask. · except: A caused item's message, chain and kept fields hold what their author wrote, a field of the answer's own hidden set reading [redacted] and an error field shown as written, cut at the text bound.
decision/cold-agent-record-owner-only · Cold-agent record directory owner-only — The live session runs mcp --record into target/cold-agent/record/, made owner-only (mkdir -m 700).
decision/debug-types-never-logged · Answer-bearing types never logged — Types that derive Debug and hold ids, accessible names, control values or page text are never logged, fielded or printed with Debug; a command's outcome is written in one place, as the command's answer.
decision/driver-library-installs-no-subscriber · Driver library installs no subscriber — The driver library, its command line, MCP server and coverage reader included, installs no sink and reads no environment variable, setting only escher-telemetry's scoped capture around an acting step and its chaining panic hook, both printing nothing; only a binary's own main installs the sink.
decision/part-answer-never-logged · A part's answer is never logged — A part's answer, a snapshot part ({"text":…,"part":n,"parts":m}), a graph part ({"nodes":[…],"states":[…],"part":n,"parts":m}) or a canvas part, is a command's answer, written over MCP as its JSON line, and is never logged or fielded.
decision/session-id-record-unprinted · Session id record kept private — The session's record of the ids its screens have read is bounded, has no reader outside the driver crate and no Debug, and is never printed, logged or fielded.
decision/sink-host-read-at-trace · Sink-installing host read at trace — A sink-installing host and its mcp role are read at trace under both builds, and must show no id, name, typed-sentinel, layout-answer key or canvas needle in what they write. · guard: host_log, mcp_log
contract/escher-session-stdout/command-answer · escher-session stdout · command answer — Each command's answer, refusal or session-error object is one uncoloured line of JSON on stdout, an answer and never a log line; the host role ends quiet.
contract/kept-check/answer · kept check file · answer — Each answered call records the answer itself, a screenshot's without the PNG.
contract/kept-check/arguments · kept check file · arguments — Each answered call's arguments are written into the kept check, a file at rest, never a log line, field or print.
decision/cold-agent-state-never-printed · Cold-agent state never printed or copied — The cold-agent state files (transcript.jsonl, client.log, server.log, check.json and the record directory) are never printed or copied.
decision/log-check-failures-name-kind-count · Log check failures name a kind and a count — A failing log check's message names a kind and a count, never a needle or anything the host wrote.
decision/one-span-per-command · One span per driver command — Each driver command opens exactly one command span, which writes only where a process has installed a sink at info or below.
contract/command-answer/graphed · driver command answer · nodes and states — A graph answer is one JSON object of nodes and states, a part adding its part number and part count, holding ids, component names, registered state and handler event names, closed flag words and booleans, never a value.

## 4-span-trace-coverage · ## 4. Span / Trace Coverage · twin span-trace-coverage
decision/canvas-read-opens-no-span · The canvas read opens no span — The driver's in-process canvas read opens no span of its own, as the layout read does not; reached by the canvas verb it runs inside that call's command span, and the lint's canvas label reading opens no span and logs nothing, running inside the lint call's command span. · why: No span of its own by design.
decision/driver-emits-no-event · Driver emits no event and installs no subscriber — escher-driver emits no log event and installs no sink, setting only escher-telemetry's scoped capture around an acting step, which keeps the step's records and panics in process and prints nothing; its telemetry is its spans alone.
decision/driver-spans-carry-no-content · Driver spans carry no content — escher-driver's spans field only closed words, bools and counts, never the screen text, any argument, a shot byte, a lint finding, a part number, or any Command, Call, ArgValue, Outcome, Refusal or Fault.
contract/escher-driver-command-span/added · escher-driver command span · added — A field of the command span, a closed word, bool or count, recorded only where it applies.
contract/escher-driver-command-span/cause · escher-driver command span · cause — A field of the command span, a closed word, bool or count, recorded only where it applies.
contract/escher-driver-command-span/changed · escher-driver command span · changed — A field of the command span, a closed word, bool or count, recorded only where it applies.
contract/escher-driver-command-span/passes · escher-driver command span · passes — A field of the command span, a closed word, bool or count, recorded only where it applies.
contract/escher-driver-command-span/removed · escher-driver command span · removed — A field of the command span, a closed word, bool or count, recorded only where it applies.
contract/escher-driver-command-span/settled · escher-driver command span · settled — A field of the command span, a closed word, bool or count, recorded only where it applies.
contract/escher-driver-command-span/verb · escher-driver command span · verb — A field of the command span, a closed word, bool or count, recorded only where it applies.
contract/escher-driver-command-span/span · escher-driver command span · command — Every call handed to Session::run opens one INFO span named command at target escher_driver.
contract/escher-driver-tool-call-span/ending · escher-driver tool_call span · ending — A closed word, one of accepted, refused or error.
contract/escher-driver-tool-call-span/span · escher-driver tool_call span · tool_call — One tool_call span is opened per MCP tools/call request, before its params are read, and held until its reply is written; it has two fields and no other.

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
contract/refusal-cause/not-found · Refusal cause names · not-found — A closed refusal class word recorded in the command span's cause field, returned by the executor.
contract/refusal-cause/off-screen · Refusal cause names · off-screen — A closed refusal class word recorded in the command span's cause field, returned by the executor; an element screenshot that crops to no pixel is refused off-screen.
contract/refusal-cause/stale · Refusal cause names · stale — A closed refusal class word recorded in the command span's cause field, returned by the executor.
decision/exploration-adds-no-span · Auto-exploration adds no span of its own — Auto-exploration opens no span and emits no event of its own; each call it drives, re-reach steps included, runs through the session's run and leaves its own command span.
decision/tool-call-span-two-closed-fields · tool_call span with tool and ending — Each MCP tools/call request writes one tool_call span carrying exactly two fields, tool and ending, each a closed word. · why: Ratified as the span's shape. · guard: stand_mcp_tools
contract/command-span/span · The driver's command span · span — One span per call handed to the session's run, target escher_driver, name command, level INFO, entered before validation and held until the run returns, so a refusal is inside it.
contract/tool-call-ending/accepted · The tool_call span's ending · accepted — The tool call was accepted and answered.
contract/tool-call-ending/refused · The tool_call span's ending · refused — The tool call was refused, its reply one text block with isError true.
contract/tool-call-span/ending · The MCP tool_call span · ending — One of three closed words saying how the call ended: `accepted` · `refused` · `error` (a session error or a protocol error).

## 6-log-coverage · ## 6. Log Coverage · twin log-coverage
decision/declared-node-read-logs-nothing · The declared-node read logs nothing — The in-process read of a canvas's declared virtual nodes, its validation and its answer have no log site, print, span or field; a refused declaration holds a closed reason and its index alone.
decision/verdict-fields-closed · Cold-agent verdict fields closed — Each cold-agent verdict field is a count, a boolean, a closed word, an object of closed words to counts, the task, the client's tool names, version and model, or the transcript file's path, never transcript text, a tool argument, a tool answer, a reply or the check.
decision/escher-session-fixed-diagnostics · escher-session diagnostics are fixed strings — Every escher-session diagnostic is a fixed string holding no path, label, id or value, and no ending holds anything the caller typed.
decision/sink-target-allowlist · Sink target allowlist — escher's sink applies an allowlist of targets in its formatter: an engine target prints only the safe fields, the rest [redacted]; an escher_ target prints every field but the content-named set; any other target is dropped whole, not an empty line, at every level and whatever RUST_LOG names. · why: The accepted cost is that a third-party WARN or ERROR never reaches stderr, at any setting. · guard: telemetry_drop · except: The upstream apps' fmt::init() stdout subscribers and the WPT runner's env_logger stay unscrubbed.
contract/cold-agent-verdict/refused-by-cause · The cold-agent verdict · refused_by_cause — An object of the eight closed refusal cause words to counts, read from the command span lines' cause field in the server log; a word outside the eight is not counted.

## log-coverage · ## 6. Log Coverage · twin 6-log-coverage
decision/cold-agent-verdict-holds-no-content · Cold-agent verdict holds no content — Every field of the cold-agent verdict is a count, a boolean, a closed word or an object of closed words to counts, holding no transcript text, tool argument, tool answer, reply or check.
decision/decoded-screen-text-stays-in-the-match · Decoded screen text stays in the match — The cold-agent pipe's decoded paged screen text goes into the expect match only, never into an event or the verdict.
decision/fixed-diagnostics · Fixed diagnostics — Every escher-session diagnostic is a fixed string holding no path, label, id or value, and no ending holds anything the caller typed.
contract/command-span/field-count · The driver's command span · field count — The `command` span keeps its eight fields; the driver's fixed help and meaning texts add no log site or span field.
contract/command-span/line · The driver's command span · line — One closed-span line per command run, INFO at target `escher_driver`, `span="command"`, with the recorded ones of `verb`, `cause`, `settled`, `busy`, `passes`, `added`, `removed`, `changed`; no line at the default `warn`.
contract/command-span/snapshot-part · The driver's command span · snapshot part read — A part read's line is `span="command" verb="snapshot"` (with `cause` when refused, a `part` outside 1 to 1000 refused `malformed`) and holds no part number, count, size or text.
contract/command-span/verb · The driver's command span · verb — The verb table's own word for a known verb, run or refused, one of its sixteen words; never the caller's verb text; unrecorded for a verb the table lacks; the three session-level words and the tool-level help appear only beside the cause unknown-verb.
contract/tool-call-span/line · The driver's tool_call span · line — One closed-span line per MCP tool call, INFO at target `escher_driver`, `span="tool_call"`, with the recorded ones of `tool` and `ending`, then `message="close"`, `time.busy`, `time.idle`; no line at the default `warn`.
decision/actions-read-changes-nothing · Actions read changes nothing in the document — The actions read acts on nothing and changes nothing in the document. · why: stand_actions' the_read_changes_nothing_and_holds_no_typed_text_or_mask pins no change mark, unchanged snapshot, focus and empty diff
decision/capability-statement-updated-by-hand · Capability statement updated by hand — A change to the driver's span set or to the scrub's limits is made in the published capability statement by hand. · why: The statement's pin reads none of its emitter claims.
decision/coverage-line-never-logged · Coverage line never logged — The handler coverage line is a command line answer only, never logged or fielded; each replayed step of a kept check writes its own command span line. · guard: a_coverage_run_logs_no_id_no_name_and_nothing_of_its_check_at_trace
contract/command-span/canvas · The driver's command span · canvas — A canvas's line, a part read's included, is `span="command" verb="canvas"` (with `cause` when refused) and holds no id, full flag, part number or anything of its answer.
contract/command-span/layout · The driver's command span · layout — A layout's line is `span="command" verb="layout"` (with `cause` when refused) and holds nothing of its answer; the layout read the call reaches opens no span of its own.
contract/command-span/screenshot · The driver's command span · screenshot — A shot's line is `span="command" verb="screenshot"` (with `cause` when refused) and holds no image byte, size or dimension.
contract/escher-session-commands/open-recording-directory · escher-session commands · recording directory open to group or others — `start` ends with status 3 and nothing spawned, and `mcp --record` with status 3, its fixed message on stderr and stdout empty.
contract/escher-session-exit-codes/0 · escher-session exit codes · 0 — Accepted: one line of JSON on stdout, stderr empty with `RUST_LOG` unset; also `help`, the host role's quiet end and the `mcp` role's `Ok` end.
contract/escher-session-exit-codes/1 · escher-session exit codes · 1 — Refused: one line of JSON on stdout, stderr empty with `RUST_LOG` unset.
contract/graph-answer/shape · The graph answer · shape — `{"nodes":[…],"states":[…]}` for a whole graph; a part read is `{"nodes":[…],"states":[…],"part":n,"parts":m}`, its states held in part 1 alone.

## 8-pii-scrubbing-compliance · ## 8. PII Scrubbing & Compliance · twin pii-scrubbing-compliance
decision/cold-agent-kept-checks-at-rest · Cold-agent kept checks held at rest only — The live cold-agent session's kept checks live in an owner-only record directory, holding calls and answers at rest, never printed, copied or committed.
decision/cold-agent-log-no-user-content · Cold-agent pipe log carries no user content — The cold-agent pipe log sits outside the sink and carries no user content: no event or verdict key is content-named and no transcript text, tool argument, answer, reply or check enters an event or the verdict.
decision/cold-agent-transcript-raw · Cold-agent transcript kept raw — The cold-agent transcript, raw model output and tool I/O, carries the stand's screens to the model provider and holds host paths, unscrubbed, never printed, and gitignored under target/. · why: Carried to the model provider by design.
decision/scrub-by-field-name · Scrubbing by field name in the sink and the capture — Scrubbing is by field name: in escher's own sink by the content-named set, a closed span's line judged pair by pair by the same rule, and in the scoped capture for every source by the answer's own hidden set; a scrubbed field reads [redacted], the message and other fields kept as written.

## pii-scrubbing-compliance · ## 8. PII Scrubbing & Compliance · twin 8-pii-scrubbing-compliance
decision/cold-agent-artifacts-never-printed · Cold-agent server log and check never printed — The cold-agent server log is counted by span name and never printed or copied, and the check file holding the agent's check is never printed.
decision/cold-agent-events-carry-no-user-content · Cold-agent events carry no user content — No cold-agent pipe log event or verdict key is in the content-named set, and no transcript text, tool argument, tool answer, reply or check enters an event or the verdict.
decision/session-logs-carry-no-page-content · Session logs carry no page content — The session host's, the mcp role's and every client's stderr carries no element id, accessible name, typed text, recording path or recorded content at any log level, under the package-alone and the workspace build. · guard: host_log, mcp_log
contract/snapshot-node/clickable · the snapshot node · clickable — A closed bool, true where an element's current listeners hold click and it is no disabled form control, carrying no page text; written as a node's "clickable" key.
contract/snapshot-text/clickable · the snapshot text · clickable — The token clickable stands after focused.
decision/cold-agent-transcript-unscrubbed · Cold-agent transcript unscrubbed — The cold-agent transcript holds raw model output and tool I/O, carried to the model provider, never printed or logged and gitignored under target. · why: unscrubbed by design, the provider crossing ratified
decision/driver-span-fields-print · Driver span fields tool and ending print — The tool_call span's fields tool and ending are not in the content-named set and print. · why: both are closed words; the closed-span line was ratified
contract/graph-answer/content · the UI graph answer (graph) · content — Carries stable ids, component names, registered state names, handler event names, the closed flag words undeclared and unconfirmed, booleans, and for a part its part number and part count only, never a state's value; a boundary widening, ratified.
contract/snapshot-diff/open-list-row-bounds · the snapshot and the diff · open list row bounds — An open list's row crosses by its own bounds, differs naming bounds within its closed set, adding no field, key name, node key or carrier.

## 9-ci-integration · ## 9. CI Integration · twin ci-integration
decision/local-state-never-uploaded · Local state stays local — The agent-run and cold-agent state, a session's state directory and a recording directory stay local and are uploaded by no CI leg.

## ci-integration · ## 9. CI Integration · twin 9-ci-integration
decision/recording-dir-private · Recording directory private — A recording directory is created 0700 with its parent existing and is refused when open to its group or others; it holds call content at rest and never a log or event line; recording is unix only.
contract/record-refusals/record-dir-not-private · Session recording refusals · record-dir-not-private — A recording directory open to its group or others is refused with record-dir-not-private.
contract/recording-file/answer · The recording file · answer — Each call's answer is kept whole as the line the driver wrote, masked as on every carrier, with its accepted flag and digest; a shot's answer is held as width and height without png.
contract/recording-file/check-file · The recording file · check-{n}.json — Each recorded session gets one check-{n}.json, mode 0600, created new and never replaced, rewritten whole after every call through a temporary file and a rename.
decision/cold-agent-kept-checks-stay-local · Cold-agent kept checks stay local — The cold-agent pipe's live run keeps its checks in an owner-only (mode 700) recording directory; the kept checks are never printed, copied, uploaded by a CI leg or committed, and are removed with the rest of the state at cleanup; the replay children run without recording.
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
