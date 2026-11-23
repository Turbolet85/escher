# obs-plan · the rows that bind 2026-11-22-upstream-sync-ahead-of-the-canvas — picked 63 of 295 (24 by every draw) · rule any · pin 3158

## 1-obs-scope-summary · ## 1. Obs Scope Summary
decision/obs-tier · Observability tier — The project's observability tier is 0.

## 2-telemetry-strategy · ## 2. Telemetry Strategy · twin telemetry-strategy
decision/engine-tracing-by-invocation · Engine tracing set by the build's invocation — A build that resolves the whole workspace turns the engine's tracing feature on while a package-alone build of the test crate or seven_guis leaves it off, so under the workspace build a test child that installs escher's sink at info also receives engine events.
decision/telemetry-via-tracing · Telemetry through the tracing crate — Telemetry is the tracing crate's event and span macros; the engine and upstream crates compile their log sites only with the tracing cargo feature, each call a no-op when the feature is off.

## telemetry-strategy · ## 2. Telemetry Strategy · twin 2-telemetry-strategy
decision/engine-tracing-feature-gated · Engine tracing behind a cargo feature — In the engine and upstream crates, tracing is compiled in only with the tracing cargo feature.
decision/tracing-for-telemetry · tracing as the telemetry crate — Telemetry is written with the tracing crate's event macros and span macros.

## 3-observability-harness-contract · ## 3. Observability Harness Contract
decision/checks-silent · Silent checks and shared modules — Checks, their shared modules and the stand-ins they use install no subscriber, read no environment variable and print nothing, their readings going into assertions only, and the code they pin adds no log site, span, field or env read. · except: Telemetry and error-text checks install escher's sink, MCP tools and layout checks a recorder; the paint-tree bench reads an opt-in env var; a re-run child prints ids or digests; the order page logs.
decision/empty-stderr-check-unsets-rust-log · Empty-stderr checks unset RUST_LOG — A check that reads an accepted command's empty stderr removes RUST_LOG from the spawned child's environment.
decision/headless-stand-no-sink · Headless stand runs with no sink — A headless boot made in process installs no telemetry sink and reads no environment variable, its acting steps running only under escher-telemetry's scoped capture, which prints nothing; a headless boot made by the escher-session binary runs under that binary's sink. · except: The MCP tools and layout stand checks install their own recording subscriber; the command-span check's re-run children and the error-text founder check install escher's sink over an in-memory writer.
decision/log-check-failures-name-kind-count · Log check failures name a kind and a count — A failing log check's message names a kind and a count, never a needle or anything the host wrote.
contract/script-diagnostics-drain/error-retention · Embedder drain of script diagnostics · error retention between drains — At most 256 JS errors are retained between drains.
contract/script-diagnostics-drain/take-js-errors · Embedder drain of script diagnostics · take_js_errors — Embedders drain JS errors with take_js_errors.
contract/script-diagnostics-drain/take-messages · Embedder drain of script diagnostics · take_messages — Embedders drain JS messages with take_messages.
decision/debug-types-never-logged · Answer-bearing types never logged — Types that derive Debug and hold ids, accessible names, control values or page text are never logged, fielded or printed with Debug; a command's outcome is written in one place, as the command's answer.
decision/sink-host-read-at-trace · Sink-installing host read at trace — A sink-installing host and its mcp role are read at trace under both builds, and must show no id, name, typed-sentinel or layout-answer key needle in what they write. · guard: host_log, mcp_log
contract/command-answer/shot · driver command answer · shot — A screenshot answer holds width, height and the PNG; over MCP it is an image block followed by the JSON line without the PNG, and is never logged or fielded.
contract/telemetry-sink/log-records · escher's telemetry sink · log records — Records of the log crate are bridged into the tracing sink.

## 4-span-trace-coverage · ## 4. Span / Trace Coverage · twin span-trace-coverage
decision/layout-read-opens-no-span · The layout read opens no span — The driver's layout read opens no span and emits no event of its own: read in process it writes no escher_ line at TRACE, and reached by the layout verb it runs inside that call's command span. · why: No span by design, under the decision that the driver emits no event. · guard: stand_layout's the_read_writes_no_escher_line_at_trace

## metric-coverage · ## 5. Metric Coverage · twin 5-metric-coverage
decision/wpt-scores-upstream-only · WPT scores published from upstream only — WPT scores are computed and published to Pages only on the upstream DioxusLabs/blitz repository, the wpt job being repository-guarded.

## 6-log-coverage · ## 6. Log Coverage · twin log-coverage
decision/capture-framework-targets-warn-up · Capture counts framework targets at warn and above — The scoped log capture admits the Dioxus framework targets (dioxus*, generational_box, warnings) and the engine targets only at warn and error, escher_ targets never, and every other outside target at every level. · why: The framework traces on every re-render and signal write, so counting it at every level would mark every ordinary step.
decision/sink-target-allowlist · Sink target allowlist — escher's sink applies an allowlist of targets in its formatter: an engine target prints only the safe fields, the rest [redacted]; an escher_ target prints every field but the content-named set; any other target is dropped whole, not an empty line, at every level and whatever RUST_LOG names. · why: The accepted cost is that a third-party WARN or ERROR never reaches stderr, at any setting. · guard: telemetry_drop · except: The upstream apps' fmt::init() stdout subscribers and the WPT runner's env_logger stay unscrubbed.
contract/escher-log-line/bridged-log-record · escher's sink log line · bridged log record — A bridged log record is printed under its log.target, that field itself omitted, and is judged against the allowlist by its log.target.
contract/escher-sink-env/rust-log · escher's sink configuration · RUST_LOG — Per-module levels come from RUST_LOG through EnvFilter, defaulting to warn when unset or unparsable, and apply to the sink layer alone, never to the capture layer beside it.
contract/escher-log-line/event-layout · escher's sink log line · event line layout — An event's line is {RFC 3339 UTC time} {LEVEL} {target} service.name={name} service.version={version} {field}={value}…
contract/escher-log-line/scrub · escher's sink log line · field scrub — Every field of an event line and every pair of a span line passes the sink's scrub.
contract/escher-log-line/span-on-close-only · escher's sink log line · span line timing — A span line is written only when a span whose target the allowlist admits closes, never when it is created, recorded to, entered or exited; an event's line is unchanged inside a span.
contract/escher-log-line/stream · escher's sink log line · output stream — One line per printed event and one per closed span, on stderr, with no ANSI.

## log-coverage · ## 6. Log Coverage · twin 6-log-coverage
decision/closed-span-lines · Closed spans as a second record class — The sink writes one line for each closed span whose target the allowlist admits, and no line when a span is created, recorded to, entered or exited. · why: Ratified as a boundary widening of the log record classes.
contract/span-line/scrub · The closed span's line · scrub — Every pair of the line, `span` included, passes the scrub under the span's target.
contract/span-line/target-outside-allowlist · The closed span's line · target outside the allowlist — A span from a target outside the allowlist writes no byte at any point.

## 7-error-capture-reporting · ## 7. Error Capture & Reporting · twin error-capture-reporting
decision/failed-stylesheet-leaves-pending-set · Failed stylesheet leaves the pending critical set — A failed stylesheet fetch drops its request from the pending critical resources, so loading does not wait on it. · why: render_blocking_stylesheet's failed_stylesheet_unblocks_rendering pins it, and the harness settle and paint_scene both read the pending critical resources
decision/no-file-loads-from-non-file-documents · No file: loads from non-file: documents — A file: load asked by a non-file: document is refused, adding no log site: a script fetch rejects with a TypeError, a module import fails, a script source records a script error, an engine fetch fails with fixed text naming no URL; the browser app's favicon and link navigation drop it silently.
contract/script-error-source-labels/error-event-listener · Script error source labels in reports · error event listener — An error raised in an error event listener is labelled "error event listener" in reports.
contract/script-error-source-labels/event-listener · Script error source labels in reports · event listener — An error raised in an event listener is labelled "event listener" in reports.
contract/script-error-source-labels/event-microtasks · Script error source labels in reports · event microtasks — An error raised in event microtasks is labelled "event microtasks" in reports.
contract/script-error-source-labels/timer-callback · Script error source labels in reports · timer callback — An error raised in a timer callback is labelled "timer callback" in reports.
contract/script-error-source-labels/timer-microtasks · Script error source labels in reports · timer microtasks — An error raised in timer microtasks is labelled "timer microtasks" in reports.
contract/script-error-surface/error-handler-exception · The script error surface · exception thrown by an error handler — An exception thrown by an error handler is recorded but fires no further error event.
contract/window-error-event/colno · The window error event · colno — The window error event carries colno 0.
contract/window-error-event/error · The window error event · error — The window error event carries the thrown error.
contract/window-error-event/filename · The window error event · filename — The window error event carries filename as the empty string.
contract/window-error-event/lineno · The window error event · lineno — The window error event carries lineno 0.
contract/window-error-event/message · The window error event · message — The window error event carries the error's message.
contract/window-onerror/arguments · window.onerror · arguments — window.onerror is called with message, source, lineno, colno and error, in that order.

## error-capture-reporting · ## 7. Error Capture & Reporting · twin 7-error-capture-reporting
contract/telemetry-scrub-set/error · escher-telemetry's scrub set · error — An `error` field is in the sink's scrub set, so the sink prints its value as `[redacted]`; the scoped capture keeps it in a caused item as written, cut at the text bound, so a failed fetch's error text crosses the answer carriers as a field value.
contract/telemetry-scrub-set/url · escher-telemetry's scrub set · url — A `url` field is in the scrub set, so the sink prints its value as `[redacted]` and the scoped capture keeps it as `[redacted]` in a caused item.

## 8-pii-scrubbing-compliance · ## 8. PII Scrubbing & Compliance · twin pii-scrubbing-compliance
decision/engine-sites-fixed-messages · Engine log sites write fixed messages — Engine and framework warn and error sites write fixed messages and carry their values in named fields (href, url, src, text, request), so the field-name scrub reaches them.
decision/scrub-sink-reach · Reach of the scrubbing sink — The scrubbing sink is installed by the two binaries of the seven_guis package; engine events reach it only when an engine tracing feature is on. · except: The upstream apps' stdout subscribers and the WPT runner's env_logger stay unscrubbed.
decision/scrub-by-field-name · Scrubbing by field name in the sink and the capture — Scrubbing is by field name: in escher's own sink by the content-named set, a closed span's line judged pair by pair by the same rule, and in the scoped capture for every source by the answer's own hidden set; a scrubbed field reads [redacted], the message and other fields kept as written.

## pii-scrubbing-compliance · ## 8. PII Scrubbing & Compliance · twin 8-pii-scrubbing-compliance
decision/engine-target-safe-fields · Engine targets print safe fields only — Under an engine target (blitz, dioxus_native, stylo_taffy, accesskit_xplat, debug_timer, js_console) the sink prints only node_id, status, waiting_nodes, property, log.module_path, log.file and log.line, every other field as {name}=[redacted].
decision/mutation-buffer-memory-only · Mutation buffer contents stay in memory — The buffered render's mutation buffer holds page content in process memory only from a render to its replay, its Debug printing its length alone, and nothing logs, fields or prints its contents.
decision/select-chosen-set-never-logged · Select chosen set never logged — A select's chosen state never leaves as a log line, span or field: a multiple select's chosen set leaves only as form entries, the snapshot's value and the options' tree nodes, and a single list box's chosen row and row labels only in the snapshot, the diff and the actions answer.
decision/select-list-label-never-logged · Select list row label never logged — A select list row's label holds page text in a public read type, is never logged, and leaves only as a painted pixel and the snapshot's existing name.
decision/closed-span-lines-judged-alike · Closed span lines judged by the same rule — A closed span's line is judged pair by pair by the target rule, and a span from a target outside the allowlist writes no byte at creation, record, enter, exit or close.
decision/escher-target-content-named-set · Escher targets redact the content-named set — Under an escher_ target the sink prints every field except url, href, src, html, text, value, attrs, path, request, error, password and panic.payload, which are redacted; escher alone, without the underscore, is not admitted.
decision/open-list-shot-carries-labels · Open list shot carries labels as pixels — A shot of an open list carries a page's option labels and group labels as pixels, the shot's own fields unmoved. · why: a boundary widening, ratified
decision/optgroup-label-crossing · Optgroup label as group name and painted header — An optgroup's non-empty label is its group name in the accessibility tree and is painted as a group header in an open list, leaving as pixels in a shot and on no log line or field. · why: inside the ratified name crossing; a boundary widening, ratified
decision/session-logs-carry-no-page-content · Session logs carry no page content — The session host's, the mcp role's and every client's stderr carries no element id, accessible name, typed text, recording path or recorded content at any log level, under the package-alone and the workspace build. · guard: host_log, mcp_log

## 9-ci-integration · ## 9. CI Integration · twin ci-integration
decision/fork-ci-two-artifacts · Fork CI uploads two artifacts — The fork's CI uploads exactly two artifacts, a leg's merged log on failure and the coverage report on success, each kept 7 days, neither holding user data.
decision/wpt-publishing-upstream-only · WPT results published upstream only — The WPT report and wptscores.json are published to GitHub Pages and the WPT results repository only on upstream DioxusLabs/blitz; the publishing jobs are repository-guarded and never run on the fork.
decision/local-state-never-uploaded · Local state stays local — The agent-run and cold-agent state, a session's state directory and a recording directory stay local and are uploaded by no CI leg.

## ci-integration · ## 9. CI Integration · twin 9-ci-integration
decision/ci-log-artifact-on-failure · Per-leg CI log uploaded on failure — Each CI leg's merged stdout and stderr is written to its own log, truncated at the leg's start, and uploaded only on failure as artifact ci-log-{job id}, kept 7 days; it is unscrubbed build output holding no user data.
decision/coverage-artifact-on-success · Coverage report uploaded on success — The coverage leg's lcov file, line counts of the workspace's public source with no user data, is uploaded only on success as artifact coverage-report, kept 7 days.

## bootstrap-phases-derive-for-route-setup-project · key of ## 3. Observability Harness Contract
decision/no-otel-export · No OTel export — The product ships no OTel export: no OTel crate, no egress and no OTEL_EXPORTER_OTLP_HEADERS credential path. · why: Ratified; the export transport and the credential path are left undecided.

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
