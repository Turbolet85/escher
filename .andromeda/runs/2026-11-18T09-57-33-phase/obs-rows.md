# obs-plan · the rows that bind 2026-11-18-served-error-page — picked 88 of 289 (57 by every draw) · rule any · pin 2112

## 2-telemetry-strategy · ## 2. Telemetry Strategy · twin telemetry-strategy
decision/escher-crates-always-trace · Escher crates take tracing unconditionally — The escher telemetry and driver crates take tracing with no feature, so the startup and panic events and the command span are always compiled into the seven_guis package's two native binaries, while their engine features stay off in a package-alone build.
decision/telemetry-via-tracing · Telemetry through the tracing crate — Telemetry is the tracing crate's event and span macros; the engine and upstream crates compile their log sites only with the tracing cargo feature, each call a no-op when the feature is off.

## telemetry-strategy · ## 2. Telemetry Strategy · twin 2-telemetry-strategy
decision/escher-crates-tracing-ungated · escher crates take tracing ungated — escher-telemetry and escher-driver take tracing with no feature gate, so the startup and panic events and the driver's spans are always compiled into the binaries that use them.
decision/tracing-for-telemetry · tracing as the telemetry crate — Telemetry is written with the tracing crate's event macros and span macros.

## 3-observability-harness-contract · ## 3. Observability Harness Contract
decision/answers-echo-nothing-typed · Answers echo nothing the caller typed — No answer, refusal, usage line or session error writes back anything the caller typed, and a password's or a file input's value appears only as the fixed mask. · except: A caused item's message, chain and kept fields hold what their author wrote, a field of the answer's own hidden set reading [redacted] and an error field shown as written, cut at the text bound.
decision/cold-agent-server-log-counted-only · Cold-agent server log read only to count — The cold-agent pipe reads server.log only to count its span lines, never printing or copying it.
decision/cold-agent-state-never-printed · Cold-agent state never printed or copied — The cold-agent state files (transcript.jsonl, client.log, server.log, check.json and the record directory) are never printed or copied.
decision/debug-types-never-logged · Answer-bearing types never logged — Types that derive Debug and hold ids, accessible names, control values or page text are never logged, fielded or printed with Debug; a command's outcome is written in one place, as the command's answer.
decision/driver-library-installs-no-subscriber · Driver library installs no subscriber — The driver library, its command line, MCP server and coverage reader included, installs no sink and reads no environment variable, setting only escher-telemetry's scoped capture around an acting step and its chaining panic hook, both printing nothing; only a binary's own main installs the sink.
decision/headless-stand-no-sink · Headless stand runs with no sink — A headless boot made in process installs no telemetry sink and reads no environment variable, its acting steps running only under escher-telemetry's scoped capture, which prints nothing; a headless boot made by the escher-session binary runs under that binary's sink. · except: The MCP tools and layout stand checks install their own recording subscriber; the command-span check's re-run children and the error-text founder check install escher's sink over an in-memory writer.
decision/part-answer-never-logged · A part's answer is never logged — A part's answer ({"text":…,"part":n,"parts":m}) is a command's answer, written over MCP as its JSON line, and is never logged or fielded.
decision/sink-host-read-at-trace · Sink-installing host read at trace — A sink-installing host and its mcp role are read at trace under both builds, and must show no id, name, typed-sentinel or layout-answer key needle in what they write. · guard: host_log, mcp_log
contract/command-answer/linted · driver command answer · findings — A lint answer is written as one JSON object with a findings array, kept whole in a recorded check and never logged or fielded.
contract/escher-session-stdout/command-answer · escher-session stdout · command answer — Each command's answer, refusal or session-error object is one uncoloured line of JSON on stdout, an answer and never a log line; the host role ends quiet.
contract/kept-check/answer · kept check file · answer — Each answered call records the answer itself, a screenshot's without the PNG.
contract/telemetry-sink/rust-log · escher's telemetry sink · RUST_LOG — The sink's filter is read from RUST_LOG, and is warn when RUST_LOG is unset; it filters the sink layer alone, the capture layer beside it unfiltered.
decision/checks-silent · Silent checks and shared modules — Checks, their shared modules and the stand-ins they use install no subscriber, read no environment variable and print nothing, their readings going into assertions only, and the code they pin adds no log site, span, field or env read. · except: Telemetry and error-text checks install escher's sink, MCP tools and layout checks their own recorder, the paint-tree bench reads an opt-in env var, a re-run child prints ids or digests to its parent.
contract/command-answer/graphed · driver command answer · nodes and states — A graph answer is one JSON object of nodes and states holding ids, component names, registered state and handler event names, closed flag words and booleans, never a value.
contract/mcp-role-exit-status/0 · escher-session mcp role exit status · 0 — The server ended without error.
contract/mcp-role-stdout/json-rpc · escher-session mcp role stdout · JSON-RPC lines — In the mcp role stdout carries newline-delimited JSON-RPC 2.0 protocol lines only, never written by the sink, and the role's end writes nothing on stdout.
contract/telemetry-sink/log-records · escher's telemetry sink · log records — Records of the log crate are bridged into the tracing sink.
contract/telemetry-sink/stream · escher's telemetry sink · output stream — The sink writes uncoloured lines to stderr only, never to stdout.

## 4-span-trace-coverage · ## 4. Span / Trace Coverage · twin span-trace-coverage
decision/driver-emits-no-event · Driver emits no event and installs no subscriber — escher-driver emits no log event and installs no sink, setting only escher-telemetry's scoped capture around an acting step, which keeps the step's records and panics in process and prints nothing; its telemetry is its spans alone.
decision/driver-spans-carry-no-content · Driver spans carry no content — escher-driver's spans field only closed words, bools and counts, never the screen text, any argument, a shot byte, a lint finding, a part number, or any Command, Call, ArgValue, Outcome, Refusal or Fault.
contract/escher-driver-tool-call-span/span · escher-driver tool_call span · tool_call — One tool_call span is opened per MCP tools/call request, before its params are read, and held until its reply is written; it has two fields and no other.

## span-trace-coverage · ## 4. Span / Trace Coverage · twin 4-span-trace-coverage
decision/command-span-no-call-content · No call content in the command span — The command span never fields an argument, an id, the screen text, an accessible name, an image or its size, lint findings, graph content, the session label, or any command, call, outcome or refusal value. · why: An argument fielded by its schema name, or a command or outcome fielded with Debug, would print an id, typed text, an accessible name or a control's value. · guard: stand_act_spans
decision/hosted-call-through-session-run · Hosted call runs through the session's run alone — A hosted part read runs through the session's run alone, so its spans are command spans recording verb alone. · why: stand_act_spans pins a part read's command line carrying its verb alone, and the session host answers each call through Session::run with no span of its own
decision/tool-call-span-two-closed-fields · tool_call span with tool and ending — Each MCP tools/call request writes one tool_call span carrying exactly two fields, tool and ending, each a closed word. · why: Ratified as the span's shape. · guard: stand_mcp_tools
contract/cold-agent-verdict/capability-unused · The cold-agent verdict · capability-unused — The reason given when a counted verb, a part read or a list opening went unused.

## 6-log-coverage · ## 6. Log Coverage · twin log-coverage
decision/capture-framework-targets-warn-up · Capture counts framework targets at warn and above — The scoped log capture admits the Dioxus framework targets (dioxus*, generational_box, warnings) and the engine targets only at warn and error, escher_ targets never, and every other outside target at every level. · why: The framework traces on every re-render and signal write, so counting it at every level would mark every ordinary step.
decision/escher-session-fixed-diagnostics · escher-session diagnostics are fixed strings — Every escher-session diagnostic is a fixed string holding no path, label, id or value, and no ending holds anything the caller typed.
decision/verdict-fields-closed · Cold-agent verdict fields closed — Each cold-agent verdict field is a count, a boolean, a closed word, an object of closed words to counts, the task, the client's tool names, version and model, or the transcript file's path, never transcript text, a tool argument, a tool answer, a reply or the check.
contract/cold-agent-events/run-end · cold-agent pipe events · run.end — run.end carrying every verdict field, printed and appended.
contract/cold-agent-events/status · cold-agent pipe events · status — status {the verdict}, or status {outcome: "none"} with exit 3, printed only.
contract/cold-agent-verdict/reasons · cold-agent verdict · reasons — The reasons add capability-unused, recording-not-kept and wrong-calls-over-bar after task-unmet and before the check's.
contract/escher-log-line/scrub · escher's sink log line · field scrub — Every field of an event line and every pair of a span line passes the sink's scrub.
contract/escher-telemetry-events/panic · escher-telemetry events · panic — ERROR at target escher_telemetry::panic, message panic, fields panic.file, panic.line, panic.column and panic.payload (redacted), once per panic.
decision/sink-target-allowlist · Sink target allowlist — escher's sink applies an allowlist of targets in its formatter: an engine target prints only the safe fields, the rest [redacted]; an escher_ target prints every field but the content-named set; any other target is dropped whole, not an empty line, at every level and whatever RUST_LOG names. · why: The accepted cost is that a third-party WARN or ERROR never reaches stderr, at any setting. · guard: telemetry_drop · except: The upstream apps' fmt::init() stdout subscribers and the WPT runner's env_logger stay unscrubbed.
contract/escher-session-mcp/stdout · escher-session mcp role · stdout — The mcp role writes protocol lines only on stdout.
contract/escher-sink-env/rust-log · escher's sink configuration · RUST_LOG — Per-module levels come from RUST_LOG through EnvFilter, defaulting to warn when unset or unparsable, and apply to the sink layer alone, never to the capture layer beside it.

## log-coverage · ## 6. Log Coverage · twin 6-log-coverage
decision/cold-agent-server-log-read-by-span-name · Cold-agent server log read by span name only — The cold-agent pipe reads the server log by span name, the tool_call span's tool and ending and the command span's cause word only, and never prints or copies it.
decision/cold-agent-verdict-holds-no-content · Cold-agent verdict holds no content — Every field of the cold-agent verdict is a count, a boolean, a closed word or an object of closed words to counts, holding no transcript text, tool argument, tool answer, reply or check.
decision/fixed-diagnostics · Fixed diagnostics — Every escher-session diagnostic is a fixed string holding no path, label, id or value, and no ending holds anything the caller typed.
decision/reached-record-ids-and-events-only · Reached record holds ids and event names only — The reached record holds stable ids and event names only, no node number, and its debug form prints its count alone. · why: the reached module's tests pin id and event rows and the_records_debug_prints_its_count_alone; handler coverage reads the record through reached_handlers
contract/cold-agent-verdict/reasons-order · The cold-agent verdict · reasons order — Reasons appear in the order `task-unmet`, `capability-unused`, `recording-not-kept`, `wrong-calls-over-bar`, `no-check`, `check-failed`, `check-cannot-fail`.
contract/command-span/actions · The driver's command span · actions — An actions line is `span="command" verb="actions"` (with `cause` when refused, an argument refused `malformed`) and holds no node, id, owner, verb word or cause word of its answer.
contract/command-span/field-count · The driver's command span · field count — The `command` span keeps its eight fields; the driver's fixed help and meaning texts add no log site or span field.
contract/command-span/line · The driver's command span · line — One closed-span line per command run, INFO at target `escher_driver`, `span="command"`, with the recorded ones of `verb`, `cause`, `settled`, `busy`, `passes`, `added`, `removed`, `changed`; no line at the default `warn`.
contract/escher-session-exit-codes/2 · escher-session exit codes · 2 — Usage: one fixed line on stderr built from `VERBS` and the six task names, stdout empty.
decision/actions-read-changes-nothing · Actions read changes nothing in the document — The actions read acts on nothing and changes nothing in the document. · why: stand_actions' the_read_changes_nothing_and_holds_no_typed_text_or_mask pins no change mark, unchanged snapshot, focus and empty diff
decision/capability-statement-updated-by-hand · Capability statement updated by hand — A change to the driver's span set or to the scrub's limits is made in the published capability statement by hand. · why: The statement's pin reads none of its emitter claims.
decision/decoded-screen-text-stays-in-the-match · Decoded screen text stays in the match — The cold-agent pipe's decoded paged screen text goes into the expect match only, never into an event or the verdict.
decision/graph-answer-never-logged · Graph answer never logged — The graph answer is written as a command answer alone, never logged or fielded.
decision/no-state-page-skips-registry · Page without registered state skips the registry — A page with no registered state reaches no signal registry: its events run their handlers and no dispatch is recorded. · why: signal_registry's a_page_with_no_registered_state_has_no_registry pins no registry context and a click that still acts; the load page's handler-coverage check relies on it
contract/command-span/verb · The driver's command span · verb — The verb table's own word for a known verb, run or refused, one of its fifteen words; never the caller's verb text; unrecorded for a verb the table lacks; the three session-level words and the tool-level help appear only beside the cause unknown-verb.
contract/escher-session-commands/mcp · escher-session commands · mcp — `escher-session mcp` writes JSON-RPC protocol lines only on stdout and nothing at its end: status 0 on success, status 3 with the error's fixed message on stderr on failure; any argument but one `--record <dir>` is usage, status 2.
contract/escher-session-coverage/command-shape · escher-session's coverage role · command shape — `escher-session coverage <app> <check>...` takes an app the binary boots and one or more paths, no `--session` or `--record` among them; any other shape is usage, status 2.
contract/escher-session-exit-codes/0 · escher-session exit codes · 0 — Accepted: one line of JSON on stdout, stderr empty with `RUST_LOG` unset; also `help`, the host role's quiet end and the `mcp` role's `Ok` end.
contract/event-line/inside-a-span · The event line · event inside a span — An event emitted inside a span keeps its line unchanged, gaining no span name and no span field.
contract/host-wire/v3 · The escher-session host wire · v3 — Wire version `v3` carries `hello`, `stop` and `call`.
contract/span-line/scrub · The closed span's line · scrub — Every pair of the line, `span` included, passes the scrub under the span's target.
contract/tool-call-span/line · The driver's tool_call span · line — One closed-span line per MCP tool call, INFO at target `escher_driver`, `span="tool_call"`, with the recorded ones of `tool` and `ending`, then `message="close"`, `time.busy`, `time.idle`; no line at the default `warn`.

## 7-error-capture-reporting · ## 7. Error Capture & Reporting · twin error-capture-reporting
contract/escher-telemetry-panic-hook/previous-hook · escher-telemetry panic hook · previous hook — The hook takes the previous hook, logs its ERROR event, then runs the previous hook, so std's default still prints the raw message to stderr and the exit code is unchanged.
decision/exploration-catches-panics · Exploration catches a panicking call — A panic in a call an exploration drives is caught and reported as the closed ending Panicked holding no payload, the process's panic hook still running, and the exploration goes on.

## error-capture-reporting · ## 7. Error Capture & Reporting · twin 7-error-capture-reporting
contract/telemetry-scrub-set/error · escher-telemetry's scrub set · error — An `error` field is in the sink's scrub set, so the sink prints its value as `[redacted]`; the scoped capture keeps it in a caused item as written, cut at the text bound, so a failed fetch's error text crosses the answer carriers as a field value.
contract/telemetry-scrub-set/url · escher-telemetry's scrub set · url — A `url` field is in the scrub set, so the sink prints its value as `[redacted]` and the scoped capture keeps it as `[redacted]` in a caused item.

## 8-pii-scrubbing-compliance · ## 8. PII Scrubbing & Compliance · twin pii-scrubbing-compliance
decision/cold-agent-kept-checks-at-rest · Cold-agent kept checks held at rest only — The live cold-agent session's kept checks live in an owner-only record directory, holding calls and answers at rest, never printed, copied or committed.
decision/cold-agent-log-no-user-content · Cold-agent pipe log carries no user content — The cold-agent pipe log sits outside the sink and carries no user content: no event or verdict key is content-named and no transcript text, tool argument, answer, reply or check enters an event or the verdict.
decision/cold-agent-transcript-raw · Cold-agent transcript kept raw — The cold-agent transcript, raw model output and tool I/O, carries the stand's screens to the model provider and holds host paths, unscrubbed, never printed, and gitignored under target/. · why: Carried to the model provider by design.
decision/engine-sites-fixed-messages · Engine log sites write fixed messages — Engine and framework warn and error sites write fixed messages and carry their values in named fields (href, url, src, text, request), so the field-name scrub reaches them.
decision/scrub-by-field-name · Scrubbing by field name in the sink and the capture — Scrubbing is by field name: in escher's own sink by the content-named set, a closed span's line judged pair by pair by the same rule, and in the scoped capture for every source by the answer's own hidden set; a scrubbed field reads [redacted], the message and other fields kept as written.
decision/scrub-sink-reach · Reach of the scrubbing sink — The scrubbing sink is installed by the two binaries of the seven_guis package; engine events reach it only when an engine tracing feature is on. · except: The upstream apps' stdout subscribers and the WPT runner's env_logger stay unscrubbed.

## pii-scrubbing-compliance · ## 8. PII Scrubbing & Compliance · twin 8-pii-scrubbing-compliance
decision/cold-agent-artifacts-never-printed · Cold-agent server log and check never printed — The cold-agent server log is counted by span name and never printed or copied, and the check file holding the agent's check is never printed.
decision/cold-agent-events-carry-no-user-content · Cold-agent events carry no user content — No cold-agent pipe log event or verdict key is in the content-named set, and no transcript text, tool argument, tool answer, reply or check enters an event or the verdict.
decision/cold-agent-transcript-unscrubbed · Cold-agent transcript unscrubbed — The cold-agent transcript holds raw model output and tool I/O, carried to the model provider, never printed or logged and gitignored under target. · why: unscrubbed by design, the provider crossing ratified
decision/escher-target-content-named-set · Escher targets redact the content-named set — Under an escher_ target the sink prints every field except url, href, src, html, text, value, attrs, path, request, error, password and panic.payload, which are redacted; escher alone, without the underscore, is not admitted.
decision/session-logs-carry-no-page-content · Session logs carry no page content — The session host's, the mcp role's and every client's stderr carries no element id, accessible name, typed text, recording path or recorded content at any log level, under the package-alone and the workspace build. · guard: host_log, mcp_log
contract/actions-answer/cause · the actions answer (actions) · cause — A closed cause word, left out when absent.
contract/actions-answer/content · the actions answer (actions) · content — Carries ids, owner ids, the table's verb words, closed cause words and booleans only, never a value, typed text, mask or NodeId; a boundary widening, ratified.
contract/actions-answer/logging · the actions answer (actions) · logging — The answer is kept whole in a recorded check and is never logged or fielded.
contract/actions-answer/owner · the actions answer (actions) · owner — Left out when the node has no owner.
contract/actions-answer/shape · the actions answer (actions) · shape — Written as {"nodes":[{"id":…,"owner":…,"keyed":…,"verbs":[{"verb":…,"cause":…}]}]}.
decision/driver-span-fields-print · Driver span fields tool and ending print — The tool_call span's fields tool and ending are not in the content-named set and print. · why: both are closed words; the closed-span line was ratified
contract/snapshot-node/clickable · the snapshot node · clickable — A closed bool, true where an element's current listeners hold click and it is no disabled form control, carrying no page text; written as a node's "clickable" key.
contract/snapshot-text/clickable · the snapshot text · clickable — The token clickable stands after focused.

## ci-integration · ## 9. CI Integration · twin 9-ci-integration
decision/cold-agent-kept-checks-stay-local · Cold-agent kept checks stay local — The cold-agent pipe's live run keeps its checks in an owner-only (mode 700) recording directory; the kept checks are never printed, copied, uploaded by a CI leg or committed, and are removed with the rest of the state at cleanup; the replay children run without recording.
contract/recording-file/answer · The recording file · answer — Each call's answer is kept whole as the line the driver wrote, masked as on every carrier, with its accepted flag and digest; a shot's answer is held as width and height without png.
contract/recording-file/check-file · The recording file · check-{n}.json — Each recorded session gets one check-{n}.json, mode 0600, created new and never replaced, rewritten whole after every call through a temporary file and a rename.

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
