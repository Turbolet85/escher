# obs-plan · the rows that bind 2026-11-23-circle-drawer-on-a-canvas — picked 65 of 295 (29 by every draw) · rule any · pin 3410

## 2-telemetry-strategy · ## 2. Telemetry Strategy · twin telemetry-strategy
decision/telemetry-via-tracing · Telemetry through the tracing crate — Telemetry is the tracing crate's event and span macros; the engine and upstream crates compile their log sites only with the tracing cargo feature, each call a no-op when the feature is off.
decision/engine-tracing-by-invocation · Engine tracing set by the build's invocation — A build that resolves the whole workspace turns the engine's tracing feature on while a package-alone build of the test crate or seven_guis leaves it off, so under the workspace build a test child that installs escher's sink at info also receives engine events.
decision/escher-crates-always-trace · Escher crates take tracing unconditionally — The escher telemetry and driver crates take tracing with no feature, so the startup and panic events and the command span are always compiled into the seven_guis package's two native binaries, while their engine features stay off in a package-alone build.

## telemetry-strategy · ## 2. Telemetry Strategy · twin 2-telemetry-strategy
decision/engine-tracing-feature-gated · Engine tracing behind a cargo feature — In the engine and upstream crates, tracing is compiled in only with the tracing cargo feature.
decision/tracing-for-telemetry · tracing as the telemetry crate — Telemetry is written with the tracing crate's event macros and span macros.

## 3-observability-harness-contract · ## 3. Observability Harness Contract
decision/checks-silent · Silent checks and shared modules — Checks, their shared modules and the stand-ins they use install no subscriber, read no environment variable and print nothing, their readings going into assertions only, and the code they pin adds no log site, span, field or env read. · except: Telemetry and error-text checks install escher's sink, MCP tools and layout checks a recorder; the paint-tree bench reads an opt-in env var; a re-run child prints ids or digests; the order page logs.
decision/debug-types-never-logged · Answer-bearing types never logged — Types that derive Debug and hold ids, accessible names, control values or page text are never logged, fielded or printed with Debug; a command's outcome is written in one place, as the command's answer.
decision/driver-library-installs-no-subscriber · Driver library installs no subscriber — The driver library, its command line, MCP server and coverage reader included, installs no sink and reads no environment variable, setting only escher-telemetry's scoped capture around an acting step and its chaining panic hook, both printing nothing; only a binary's own main installs the sink.
decision/headless-stand-no-sink · Headless stand runs with no sink — A headless boot made in process installs no telemetry sink and reads no environment variable, its acting steps running only under escher-telemetry's scoped capture, which prints nothing; a headless boot made by the escher-session binary runs under that binary's sink. · except: The MCP tools and layout stand checks install their own recording subscriber; the command-span check's re-run children and the error-text founder check install escher's sink over an in-memory writer.
decision/one-span-per-command · One span per driver command — Each driver command opens exactly one command span, which writes only where a process has installed a sink at info or below.
contract/command-answer/graphed · driver command answer · nodes and states — A graph answer is one JSON object of nodes and states, a part adding its part number and part count, holding ids, component names, registered state and handler event names, closed flag words and booleans, never a value.
contract/command-answer/linted · driver command answer · findings — A lint answer is written as one JSON object with a findings array, kept whole in a recorded check and never logged or fielded.
contract/command-answer/shot · driver command answer · shot — A screenshot answer holds width, height and the PNG; over MCP it is an image block followed by the JSON line without the PNG, and is never logged or fielded.
contract/kept-check/accepted · kept check file · accepted — Each answered call records whether it was accepted.
contract/kept-check/answer · kept check file · answer — Each answered call records the answer itself, a screenshot's without the PNG.
contract/kept-check/answer-digest · kept check file · answer digest — Each answered call records its answer's FNV-1a 64 digest.

## 4-span-trace-coverage · ## 4. Span / Trace Coverage · twin span-trace-coverage
decision/driver-emits-no-event · Driver emits no event and installs no subscriber — escher-driver emits no log event and installs no sink, setting only escher-telemetry's scoped capture around an acting step, which keeps the step's records and panics in process and prints nothing; its telemetry is its spans alone.
decision/driver-spans-carry-no-content · Driver spans carry no content — escher-driver's spans field only closed words, bools and counts, never the screen text, any argument, a shot byte, a lint finding, a part number, or any Command, Call, ArgValue, Outcome, Refusal or Fault.
contract/escher-driver-command-span/span · escher-driver command span · command — Every call handed to Session::run opens one INFO span named command at target escher_driver.
contract/escher-driver-command-span/added · escher-driver command span · added — A field of the command span, a closed word, bool or count, recorded only where it applies.
contract/escher-driver-command-span/busy · escher-driver command span · busy — A field of the command span, a closed word, bool or count, recorded only where it applies.
contract/escher-driver-command-span/cause · escher-driver command span · cause — A field of the command span, a closed word, bool or count, recorded only where it applies.
contract/escher-driver-command-span/changed · escher-driver command span · changed — A field of the command span, a closed word, bool or count, recorded only where it applies.
contract/escher-driver-command-span/passes · escher-driver command span · passes — A field of the command span, a closed word, bool or count, recorded only where it applies.
contract/escher-driver-command-span/removed · escher-driver command span · removed — A field of the command span, a closed word, bool or count, recorded only where it applies.
contract/escher-driver-command-span/settled · escher-driver command span · settled — A field of the command span, a closed word, bool or count, recorded only where it applies.
contract/escher-driver-command-span/verb · escher-driver command span · verb — A field of the command span, a closed word, bool or count, recorded only where it applies.

## span-trace-coverage · ## 4. Span / Trace Coverage · twin 4-span-trace-coverage
decision/command-span-no-call-content · No call content in the command span — The command span never fields an argument, an id, the screen text, an accessible name, an image or its size, lint findings, graph content, the session label, or any command, call, outcome or refusal value. · why: An argument fielded by its schema name, or a command or outcome fielded with Debug, would print an id, typed text, an accessible name or a control's value. · guard: stand_act_spans
contract/command-span/settled · The driver's command span · settled — Recorded from an acting verb's outcome only; snapshot, screenshot, lint, graph, actions, layout and a part read record verb alone, and a refusal records verb and cause alone.
contract/command-span/span · The driver's command span · span — One span per call handed to the session's run, target escher_driver, name command, level INFO, entered before validation and held until the run returns, so a refusal is inside it.
decision/advance-accepted-every-session · advance accepted on every session — The advance verb is accepted on every session and records the acting fields like every acting verb, with no cause. · why: Ratified.
contract/command-span/added · The driver's command span · added — The length of the returned diff's added list, recorded from an acting verb's outcome.
contract/command-span/busy · The driver's command span · busy — One of three busy-class words, the schema's word for the busy class, recorded from an acting verb's outcome when the outcome carries one.
contract/command-span/cause · The driver's command span · cause — Recorded at one site for every refusal the run returns: one of eight fixed cause names, class words holding nothing a call supplied.
contract/command-span/changed · The driver's command span · changed — The length of the returned diff's changed list, recorded from an acting verb's outcome.
contract/command-span/passes · The driver's command span · passes — Settle's pass count, recorded from an acting verb's outcome when settle answered quiet; on a step that waited on a load, the settle that went quiet.
contract/command-span/removed · The driver's command span · removed — The length of the returned diff's removed list, recorded from an acting verb's outcome.
contract/refusal-cause/fonts-unavailable · Refusal cause names · fonts-unavailable — A closed refusal class word recorded in the command span's cause field, returned by the executor before anything is painted when the held app has no font it can paint text with.
contract/refusal-cause/off-screen · Refusal cause names · off-screen — A closed refusal class word recorded in the command span's cause field, returned by the executor; an element screenshot that crops to no pixel is refused off-screen.

## 6-log-coverage · ## 6. Log Coverage · twin log-coverage
decision/sink-target-allowlist · Sink target allowlist — escher's sink applies an allowlist of targets in its formatter: an engine target prints only the safe fields, the rest [redacted]; an escher_ target prints every field but the content-named set; any other target is dropped whole, not an empty line, at every level and whatever RUST_LOG names. · why: The accepted cost is that a third-party WARN or ERROR never reaches stderr, at any setting. · guard: telemetry_drop · except: The upstream apps' fmt::init() stdout subscribers and the WPT runner's env_logger stay unscrubbed.
contract/escher-driver-span-lines/default-level · escher-driver span lines · default warn level — The driver's command and tool_call span lines print only in a process that installs escher's sink at info or below; at the default warn there is no line.

## log-coverage · ## 6. Log Coverage · twin 6-log-coverage
contract/command-span/field-count · The driver's command span · field count — The `command` span keeps its eight fields; the driver's fixed help and meaning texts add no log site or span field.
contract/command-span/line · The driver's command span · line — One closed-span line per command run, INFO at target `escher_driver`, `span="command"`, with the recorded ones of `verb`, `cause`, `settled`, `busy`, `passes`, `added`, `removed`, `changed`; no line at the default `warn`.
contract/command-span/screenshot · The driver's command span · screenshot — A shot's line is `span="command" verb="screenshot"` (with `cause` when refused) and holds no image byte, size or dimension.
contract/graph-answer/shape · The graph answer · shape — `{"nodes":[…],"states":[…]}` for a whole graph; a part read is `{"nodes":[…],"states":[…],"part":n,"parts":m}`, its states held in part 1 alone.
decision/actions-read-changes-nothing · Actions read changes nothing in the document — The actions read acts on nothing and changes nothing in the document. · why: stand_actions' the_read_changes_nothing_and_holds_no_typed_text_or_mask pins no change mark, unchanged snapshot, focus and empty diff
decision/capability-statement-updated-by-hand · Capability statement updated by hand — A change to the driver's span set or to the scrub's limits is made in the published capability statement by hand. · why: The statement's pin reads none of its emitter claims.
decision/no-state-page-skips-registry · Page without registered state skips the registry — A page with no registered state reaches no signal registry: its events run their handlers and no dispatch is recorded. · why: signal_registry's a_page_with_no_registered_state_has_no_registry pins no registry context and a click that still acts; the load page's handler-coverage check relies on it
contract/command-span/actions · The driver's command span · actions — An actions line is `span="command" verb="actions"` (with `cause` when refused, an argument refused `malformed`) and holds no node, id, owner, verb word or cause word of its answer.
contract/command-span/graph · The driver's command span · graph — A graph's line, a part read's included, is `span="command" verb="graph"` (with `cause` when refused, an argument or a `part` outside 1 to 1000 refused `malformed`) and holds no node, state, id, name, part number or count.
contract/command-span/lint · The driver's command span · lint — A lint's line is `span="command" verb="lint"` (with `cause` when refused) and holds no finding, id, bound or count.
contract/command-span/verb · The driver's command span · verb — The verb table's own word for a known verb, run or refused, one of its fifteen words; never the caller's verb text; unrecorded for a verb the table lacks; the three session-level words and the tool-level help appear only beside the cause unknown-verb.

## 8-pii-scrubbing-compliance · ## 8. PII Scrubbing & Compliance · twin pii-scrubbing-compliance
decision/engine-sites-fixed-messages · Engine log sites write fixed messages — Engine and framework warn and error sites write fixed messages and carry their values in named fields (href, url, src, text, request), so the field-name scrub reaches them.
decision/scrub-sink-reach · Reach of the scrubbing sink — The scrubbing sink is installed by the two binaries of the seven_guis package; engine events reach it only when an engine tracing feature is on. · except: The upstream apps' stdout subscribers and the WPT runner's env_logger stay unscrubbed.

## pii-scrubbing-compliance · ## 8. PII Scrubbing & Compliance · twin 8-pii-scrubbing-compliance
decision/engine-target-safe-fields · Engine targets print safe fields only — Under an engine target (blitz, dioxus_native, stylo_taffy, accesskit_xplat, debug_timer, js_console) the sink prints only node_id, status, waiting_nodes, property, log.module_path, log.file and log.line, every other field as {name}=[redacted].
decision/shot-image-leaves-only-in-answer · Shot image leaves only in its answer — A shot's image is screen content that leaves only in the command's answer, never on a log line. · guard: host_log, mcp_log
contract/graph-answer/content · the UI graph answer (graph) · content — Carries stable ids, component names, registered state names, handler event names, the closed flag words undeclared and unconfirmed, booleans, and for a part its part number and part count only, never a state's value; a boundary widening, ratified.
contract/snapshot-node/clickable · the snapshot node · clickable — A closed bool, true where an element's current listeners hold click and it is no disabled form control, carrying no page text; written as a node's "clickable" key.
contract/snapshot-text/clickable · the snapshot text · clickable — The token clickable stands after focused.
decision/driver-span-fields-print · Driver span fields tool and ending print — The tool_call span's fields tool and ending are not in the content-named set and print. · why: both are closed words; the closed-span line was ratified
decision/escher-target-content-named-set · Escher targets redact the content-named set — Under an escher_ target the sink prints every field except url, href, src, html, text, value, attrs, path, request, error, password and panic.payload, which are redacted; escher alone, without the underscore, is not admitted.
decision/session-logs-carry-no-page-content · Session logs carry no page content — The session host's, the mcp role's and every client's stderr carries no element id, accessible name, typed text, recording path or recorded content at any log level, under the package-alone and the workspace build. · guard: host_log, mcp_log
contract/actions-answer/content · the actions answer (actions) · content — Carries ids, owner ids, the table's verb words, closed cause words and booleans only, never a value, typed text, mask or NodeId; a boundary widening, ratified.
contract/actions-answer/shape · the actions answer (actions) · shape — Written as {"nodes":[{"id":…,"owner":…,"keyed":…,"verbs":[{"verb":…,"cause":…}]}]}.

## ci-integration · ## 9. CI Integration · twin 9-ci-integration
contract/recording-file/answer · The recording file · answer — Each call's answer is kept whole as the line the driver wrote, masked as on every carrier, with its accepted flag and digest; a shot's answer is held as width and height without png.

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
