# obs-plan · the rows that bind 2026-11-23-canvas-shot-across-hosts — picked 32 of 295 (11 by every draw) · rule any · pin 3354

## 3-observability-harness-contract · ## 3. Observability Harness Contract
decision/checks-silent · Silent checks and shared modules — Checks, their shared modules and the stand-ins they use install no subscriber, read no environment variable and print nothing, their readings going into assertions only, and the code they pin adds no log site, span, field or env read. · except: Telemetry and error-text checks install escher's sink, MCP tools and layout checks a recorder; the paint-tree bench reads an opt-in env var; a re-run child prints ids or digests; the order page logs.
decision/headless-stand-no-sink · Headless stand runs with no sink — A headless boot made in process installs no telemetry sink and reads no environment variable, its acting steps running only under escher-telemetry's scoped capture, which prints nothing; a headless boot made by the escher-session binary runs under that binary's sink. · except: The MCP tools and layout stand checks install their own recording subscriber; the command-span check's re-run children and the error-text founder check install escher's sink over an in-memory writer.
contract/command-answer/shot · driver command answer · shot — A screenshot answer holds width, height and the PNG; over MCP it is an image block followed by the JSON line without the PNG, and is never logged or fielded.
contract/kept-check/answer · kept check file · answer — Each answered call records the answer itself, a screenshot's without the PNG.
contract/kept-check/answer-digest · kept check file · answer digest — Each answered call records its answer's FNV-1a 64 digest.
decision/debug-types-never-logged · Answer-bearing types never logged — Types that derive Debug and hold ids, accessible names, control values or page text are never logged, fielded or printed with Debug; a command's outcome is written in one place, as the command's answer.
decision/driver-library-installs-no-subscriber · Driver library installs no subscriber — The driver library, its command line, MCP server and coverage reader included, installs no sink and reads no environment variable, setting only escher-telemetry's scoped capture around an acting step and its chaining panic hook, both printing nothing; only a binary's own main installs the sink.
decision/log-check-failures-name-kind-count · Log check failures name a kind and a count — A failing log check's message names a kind and a count, never a needle or anything the host wrote.
decision/one-span-per-command · One span per driver command — Each driver command opens exactly one command span, which writes only where a process has installed a sink at info or below.

## 4-span-trace-coverage · ## 4. Span / Trace Coverage · twin span-trace-coverage
decision/driver-spans-carry-no-content · Driver spans carry no content — escher-driver's spans field only closed words, bools and counts, never the screen text, any argument, a shot byte, a lint finding, a part number, or any Command, Call, ArgValue, Outcome, Refusal or Fault.
decision/driver-emits-no-event · Driver emits no event and installs no subscriber — escher-driver emits no log event and installs no sink, setting only escher-telemetry's scoped capture around an acting step, which keeps the step's records and panics in process and prints nothing; its telemetry is its spans alone.
contract/escher-driver-command-span/span · escher-driver command span · command — Every call handed to Session::run opens one INFO span named command at target escher_driver.

## span-trace-coverage · ## 4. Span / Trace Coverage · twin 4-span-trace-coverage
decision/command-span-no-call-content · No call content in the command span — The command span never fields an argument, an id, the screen text, an accessible name, an image or its size, lint findings, graph content, the session label, or any command, call, outcome or refusal value. · why: An argument fielded by its schema name, or a command or outcome fielded with Debug, would print an id, typed text, an accessible name or a control's value. · guard: stand_act_spans
contract/command-span/settled · The driver's command span · settled — Recorded from an acting verb's outcome only; snapshot, screenshot, lint, graph, actions, layout and a part read record verb alone, and a refusal records verb and cause alone.
contract/command-span/span · The driver's command span · span — One span per call handed to the session's run, target escher_driver, name command, level INFO, entered before validation and held until the run returns, so a refusal is inside it.
contract/refusal-cause/fonts-unavailable · Refusal cause names · fonts-unavailable — A closed refusal class word recorded in the command span's cause field, returned by the executor before anything is painted when the held app has no font it can paint text with.

## 6-log-coverage · ## 6. Log Coverage · twin log-coverage
contract/escher-driver-span-lines/default-level · escher-driver span lines · default warn level — The driver's command and tool_call span lines print only in a process that installs escher's sink at info or below; at the default warn there is no line.

## log-coverage · ## 6. Log Coverage · twin 6-log-coverage
contract/command-span/screenshot · The driver's command span · screenshot — A shot's line is `span="command" verb="screenshot"` (with `cause` when refused) and holds no image byte, size or dimension.
contract/command-span/field-count · The driver's command span · field count — The `command` span keeps its eight fields; the driver's fixed help and meaning texts add no log site or span field.
contract/command-span/line · The driver's command span · line — One closed-span line per command run, INFO at target `escher_driver`, `span="command"`, with the recorded ones of `verb`, `cause`, `settled`, `busy`, `passes`, `added`, `removed`, `changed`; no line at the default `warn`.
contract/tool-call-span/line · The driver's tool_call span · line — One closed-span line per MCP tool call, INFO at target `escher_driver`, `span="tool_call"`, with the recorded ones of `tool` and `ending`, then `message="close"`, `time.busy`, `time.idle`; no line at the default `warn`.

## 8-pii-scrubbing-compliance · ## 8. PII Scrubbing & Compliance · twin pii-scrubbing-compliance
decision/scrub-sink-reach · Reach of the scrubbing sink — The scrubbing sink is installed by the two binaries of the seven_guis package; engine events reach it only when an engine tracing feature is on. · except: The upstream apps' stdout subscribers and the WPT runner's env_logger stay unscrubbed.

## pii-scrubbing-compliance · ## 8. PII Scrubbing & Compliance · twin 8-pii-scrubbing-compliance
decision/shot-image-leaves-only-in-answer · Shot image leaves only in its answer — A shot's image is screen content that leaves only in the command's answer, never on a log line. · guard: host_log, mcp_log
decision/closed-span-lines-judged-alike · Closed span lines judged by the same rule — A closed span's line is judged pair by pair by the target rule, and a span from a target outside the allowlist writes no byte at creation, record, enter, exit or close.
decision/driver-span-fields-print · Driver span fields tool and ending print — The tool_call span's fields tool and ending are not in the content-named set and print. · why: both are closed words; the closed-span line was ratified

## 9-ci-integration · ## 9. CI Integration · twin ci-integration
decision/fork-ci-two-artifacts · Fork CI uploads two artifacts — The fork's CI uploads exactly two artifacts, a leg's merged log on failure and the coverage report on success, each kept 7 days, neither holding user data.
decision/ci-platform-github-workflows · CI on GitHub workflows — CI runs as GitHub workflows under .github/workflows/.
decision/committed-kept-checks-clean · Committed kept checks hold no secrets — The kept checks committed in git hold no typed text, host path or secret.

## ci-integration · ## 9. CI Integration · twin 9-ci-integration
decision/ci-log-artifact-on-failure · Per-leg CI log uploaded on failure — Each CI leg's merged stdout and stderr is written to its own log, truncated at the leg's start, and uploaded only on failure as artifact ci-log-{job id}, kept 7 days; it is unscrubbed build output holding no user data.
decision/committed-checks-hold-no-private-data · Committed checks hold no private data — A kept check committed to git is one of the repository's own scripted stand-Counter sessions and holds no typed text, host path, name, secret or screenshot step. · why: The repository is public.
decision/coverage-artifact-on-success · Coverage report uploaded on success — The coverage leg's lcov file, line counts of the workspace's public source with no user data, is uploaded only on success as artifact coverage-report, kept 7 days.
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
