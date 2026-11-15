# obs-plan · the rows that bind 2026-11-15-upstream-sync-errors-and-explanation — picked 53 of 276 (30 by every draw) · rule any · pin 1072

## 2-telemetry-strategy · ## 2. Telemetry Strategy · twin telemetry-strategy
decision/engine-tracing-by-invocation · Engine tracing set by the build's invocation — A build that resolves the whole workspace turns the engine's tracing feature on while a package-alone build of the test crate or seven_guis leaves it off, so under the workspace build a test child that installs escher's sink at info also receives engine events.
decision/telemetry-via-tracing · Telemetry through the tracing crate — Telemetry is the tracing crate's event and span macros; the engine and upstream crates compile their log sites only with the tracing cargo feature, each call a no-op when the feature is off.

## telemetry-strategy · ## 2. Telemetry Strategy · twin 2-telemetry-strategy
decision/engine-tracing-feature-gated · Engine tracing behind a cargo feature — In the engine and upstream crates, tracing is compiled in only with the tracing cargo feature.
decision/tracing-for-telemetry · tracing as the telemetry crate — Telemetry is written with the tracing crate's event macros and span macros.

## 3-observability-harness-contract · ## 3. Observability Harness Contract
decision/debug-types-never-logged · Answer-bearing types never logged — Types that derive Debug and hold ids, accessible names, control values or page text are never logged, fielded or printed with Debug; a command's outcome is written in one place, as the command's answer.
decision/sink-host-read-at-trace · Sink-installing host read at trace — A sink-installing host and its mcp role are read at trace under both builds, and must show no id, name or typed-sentinel needle in what they write. · guard: host_log, mcp_log
decision/sink-installed-first-in-main · Telemetry sink installed first in each native main — Each native binary of the seven_guis package installs escher's telemetry sink in its native main before anything boots, reporting a failed install on stderr and continuing.
contract/telemetry-sink/log-records · escher's telemetry sink · log records — Records of the log crate are bridged into the tracing sink.
contract/telemetry-sink/rust-log · escher's telemetry sink · RUST_LOG — The sink's filter is read from RUST_LOG, and is warn when RUST_LOG is unset.
contract/telemetry-sink/stream · escher's telemetry sink · output stream — The sink writes uncoloured lines to stderr only, never to stdout.
decision/headless-stand-no-sink · Headless stand runs with no sink — A headless boot made in process installs no telemetry subscriber and reads no environment variable; a headless boot made by the escher-session binary runs under that binary's sink. · except: The MCP tools check installs its own in-process recording subscriber, and the command-span check's two re-run children install escher's sink over an in-memory capture.

## metric-coverage · ## 5. Metric Coverage · twin 5-metric-coverage
decision/wpt-scores-upstream-only · WPT scores published from upstream only — WPT scores are computed and published to Pages only on the upstream DioxusLabs/blitz repository, the wpt job being repository-guarded.

## 6-log-coverage · ## 6. Log Coverage · twin log-coverage
decision/sink-target-allowlist · Sink target allowlist — escher's sink applies an allowlist of targets in its formatter: an engine target prints only the safe fields, the rest [redacted]; an escher_ target prints every field but the content-named set; any other target is dropped whole, not an empty line, at every level and whatever RUST_LOG names. · why: The accepted cost is that a third-party WARN or ERROR never reaches stderr, at any setting. · guard: telemetry_drop · except: The upstream apps' fmt::init() stdout subscribers and the WPT runner's env_logger stay unscrubbed.
contract/escher-log-line/bridged-log-record · escher's sink log line · bridged log record — A bridged log record is printed under its log.target, that field itself omitted, and is judged against the allowlist by its log.target.
contract/escher-log-line/scrub · escher's sink log line · field scrub — Every field of an event line and every pair of a span line passes the sink's scrub.
contract/escher-sink-env/rust-log · escher's sink configuration · RUST_LOG — Per-module levels come from RUST_LOG through EnvFilter, defaulting to warn when unset or unparsable.
contract/escher-log-line/span-on-close-only · escher's sink log line · span line timing — A span line is written only when a span whose target the allowlist admits closes, never when it is created, recorded to, entered or exited; an event's line is unchanged inside a span.

## log-coverage · ## 6. Log Coverage · twin 6-log-coverage
decision/reached-record-ids-and-events-only · Reached record holds ids and event names only — The reached record holds stable ids and event names only, no node number, and its debug form prints its count alone. · why: the reached module's tests pin id and event rows and the_records_debug_prints_its_count_alone; handler coverage reads the record through reached_handlers
decision/closed-span-lines · Closed spans as a second record class — The sink writes one line for each closed span whose target the allowlist admits, and no line when a span is created, recorded to, entered or exited. · why: Ratified as a boundary widening of the log record classes.
decision/no-state-page-skips-registry · Page without registered state skips the registry — A page with no registered state reaches no signal registry: its events run their handlers and no dispatch is recorded. · why: signal_registry's a_page_with_no_registered_state_has_no_registry pins no registry context and a click that still acts; the load page's handler-coverage check relies on it
contract/span-line/scrub · The closed span's line · scrub — Every pair of the line, `span` included, passes the scrub under the span's target.
contract/span-line/target-outside-allowlist · The closed span's line · target outside the allowlist — A span from a target outside the allowlist writes no byte at any point.

## 7-error-capture-reporting · ## 7. Error Capture & Reporting · twin error-capture-reporting
decision/failed-stylesheet-leaves-pending-set · Failed stylesheet leaves the pending critical set — A failed stylesheet fetch drops its request from the pending critical resources, so loading does not wait on it. · why: render_blocking_stylesheet's failed_stylesheet_unblocks_rendering pins it, and the harness settle and paint_scene both read the pending critical resources
decision/no-file-loads-from-non-file-documents · No file: loads from non-file: documents — A file: load asked by a non-file: document is refused, adding no log site: a script fetch rejects with a TypeError, a module import fails, a script source records a script error, an engine fetch fails with fixed text naming no URL; the browser app's favicon and link navigation drop it silently.
contract/wpt-report/message · WPT report · message — A crashed WPT test's panic message is carried into the report's message field.
contract/wpt-report/subtest-message · The WPT report · subtest message — A subtest's error strings are joined with newlines into the report's subtest message.

## error-capture-reporting · ## 7. Error Capture & Reporting · twin 7-error-capture-reporting
contract/telemetry-scrub-set/error · escher-telemetry's scrub set · error — An `error` field is in the scrub set, so the sink prints its value as `[redacted]`; a failed fetch's error text reaches no carrier beyond its log line.
contract/telemetry-scrub-set/url · escher-telemetry's scrub set · url — A `url` field is in the scrub set, so the sink prints its value as `[redacted]`.

## 8-pii-scrubbing-compliance · ## 8. PII Scrubbing & Compliance · twin pii-scrubbing-compliance
decision/content-span-fields-never-stored · Content-named span fields never stored — The value of a content-named span field is never stored; its [redacted] marker is written at print time, and every other span field's value is held only in process memory until the span closes.
decision/scrub-at-sink-not-call-sites · Scrubbing at the sink, not at call sites — Scrubbing is applied in escher's own sink, a closed span's line judged pair by pair by the same rule, and no engine call site is edited.
decision/scrub-sink-reach · Reach of the scrubbing sink — The scrubbing sink is installed by the two binaries of the seven_guis package; engine events reach it only when an engine tracing feature is on. · except: The upstream apps' stdout subscribers and the WPT runner's env_logger stay unscrubbed.

## pii-scrubbing-compliance · ## 8. PII Scrubbing & Compliance · twin 8-pii-scrubbing-compliance
decision/closed-span-lines-judged-alike · Closed span lines judged by the same rule — A closed span's line is judged pair by pair by the target rule, and a span from a target outside the allowlist writes no byte at creation, record, enter, exit or close.
decision/content-named-span-values-not-stored · Content-named span values never stored — The value of a content-named span field is never stored, its marker written at print time.
decision/engine-target-safe-fields · Engine targets print safe fields only — Under an engine target (blitz, dioxus_native, stylo_taffy, accesskit_xplat, debug_timer, js_console) the sink prints only node_id, status, waiting_nodes, property, log.module_path, log.file and log.line, every other field as {name}=[redacted].
decision/select-chosen-set-never-logged · Select chosen set never logged — A multiple select's chosen set leaves only as form entries, the snapshot's value and the options' tree nodes, never as a log line or field.
decision/select-list-label-never-logged · Select list row label never logged — A select list row's label holds page text in a public read type, is never logged, and leaves only as a painted pixel and the snapshot's existing name.
contract/multiple-select-form-entries/chosen-option · multiple select form entries · chosen option — On input and change, one (name, FormValue::Text(value)) entry per chosen option.
decision/escher-target-content-named-set · Escher targets redact the content-named set — Under an escher_ target the sink prints every field except url, href, src, html, text, value, attrs, path, request, error and panic.payload, which are redacted; escher alone, without the underscore, is not admitted.
decision/mutation-buffer-memory-only · Mutation buffer contents stay in memory — The buffered render's mutation buffer holds page content in process memory only from a render to its replay, its Debug printing its length alone, and nothing logs, fields or prints its contents.
decision/open-list-shot-carries-labels · Open list shot carries labels as pixels — A shot of an open list carries a page's option labels and group labels as pixels, the shot's own fields unmoved. · why: a boundary widening, ratified
decision/optgroup-label-crossing · Optgroup label as group name and painted header — An optgroup's non-empty label is its group name in the accessibility tree and is painted as a group header in an open list, leaving as pixels in a shot and on no log line or field. · why: inside the ratified name crossing; a boundary widening, ratified
decision/select-size-attribute-read · Select size attribute read — The size attribute on a multiple select is read as a whole number capped at 65534, carrying no text out and moving no redaction shape, for every document. · why: an engine change for every document and a boundary widening, ratified
contract/graph-answer/content · the UI graph answer (graph) · content — Carries stable ids, component names, registered state names, handler event names, the closed flag words undeclared and unconfirmed, and booleans only, never a state's value; a boundary widening, ratified.
contract/page-events/checkbox-change · page script events · change on a checkbox — The engine always sends change after input with the same value, carrying the value class input carries.
contract/page-events/radio-change · page script events · change on a radio — The engine sends change after input with the same value only where the radio was not checked before the click.
contract/page-events/single-change · page script events · change count — A page's script hears one change, from the engine, the script runtime synthesising none.
contract/snapshot-diff/open-list-row-bounds · the snapshot and the diff · open list row bounds — An open list's row crosses by its own bounds, differs naming bounds within its closed set, adding no field, key name, node key or carrier.
contract/snapshot-node/clickable · the snapshot node · clickable — A closed bool, true where an element's current listeners hold click and it is no disabled form control, carrying no page text; written as a node's "clickable" key.
contract/snapshot-text/clickable · the snapshot text · clickable — The token clickable stands after focused.
contract/snapshot/multiple-select-value · the snapshot · value of a multiple select — The first chosen option in tree order.

## 9-ci-integration · ## 9. CI Integration · twin ci-integration
decision/wpt-publishing-upstream-only · WPT results published upstream only — The WPT report and wptscores.json are published to GitHub Pages and the WPT results repository only on upstream DioxusLabs/blitz; the publishing jobs are repository-guarded and never run on the fork.
decision/ci-platform-github-workflows · CI on GitHub workflows — CI runs as GitHub workflows under .github/workflows/.

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
