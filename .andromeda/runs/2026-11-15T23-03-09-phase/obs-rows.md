# obs-plan · the rows that bind 2026-11-15-session-binary-smoke — picked 55 of 276 (39 by every draw) · rule any · pin 1209

## 2-telemetry-strategy · ## 2. Telemetry Strategy · twin telemetry-strategy
decision/engine-tracing-by-invocation · Engine tracing set by the build's invocation — A build that resolves the whole workspace turns the engine's tracing feature on while a package-alone build of the test crate or seven_guis leaves it off, so under the workspace build a test child that installs escher's sink at info also receives engine events.
decision/escher-crates-always-trace · Escher crates take tracing unconditionally — The escher telemetry and driver crates take tracing with no feature, so the startup and panic events and the command span are always compiled into the seven_guis package's two native binaries, while their engine features stay off in a package-alone build.

## telemetry-strategy · ## 2. Telemetry Strategy · twin 2-telemetry-strategy
decision/escher-crates-tracing-ungated · escher crates take tracing ungated — escher-telemetry and escher-driver take tracing with no feature gate, so the startup and panic events and the driver's spans are always compiled into the binaries that use them.

## 3-observability-harness-contract · ## 3. Observability Harness Contract
decision/agent-run-raw-output-never-printed · Agent-run raw output never printed — The raw merged cargo and libtest output goes to target/agent-run/run.log, truncated at each run's start and never printed.
decision/answers-echo-nothing-typed · Answers echo nothing the caller typed — No answer, refusal, usage line or session error writes back anything the caller typed, and a password's or a file input's value appears only as the fixed mask.
decision/checks-silent · Silent checks and shared modules — Checks, their shared modules and the stand-ins they use install no subscriber, read no environment variable and print nothing, their readings going into assertions only, and the code they pin adds no log site, span, field or env read. · except: Checks whose subject is telemetry install escher's sink, the MCP tools check its own recorder, the paint-tree bench reads one opt-in env var, and a re-run child prints ids or digests its parent reads.
decision/empty-stderr-check-unsets-rust-log · Empty-stderr checks unset RUST_LOG — A check that reads an accepted command's empty stderr removes RUST_LOG from the spawned child's environment.
decision/harness-json-via-python-json · Harness JSON lines encoded by python's json module — The agent-run and cold-agent harnesses write JSON lines encoded by python3's stdlib json module embedded in the script, never assembled in bash.
decision/harness-log-not-telemetry-sink · Harness logs are not telemetry sinks — The agent-run and cold-agent harness logs are harness metadata, not telemetry sinks: the harness itself installs no tracing subscriber, OTel setup or third-party logger.
decision/headless-stand-no-sink · Headless stand runs with no sink — A headless boot made in process installs no telemetry subscriber and reads no environment variable; a headless boot made by the escher-session binary runs under that binary's sink. · except: The MCP tools check installs its own in-process recording subscriber, and the command-span check's two re-run children install escher's sink over an in-memory capture.
decision/log-check-failures-name-kind-count · Log check failures name a kind and a count — A failing log check's message names a kind and a count, never a needle or anything the host wrote.
decision/session-id-record-unprinted · Session id record kept private — The session's record of the ids its screens have read is bounded, has no reader outside the driver crate and no Debug, and is never printed, logged or fielded.
decision/sink-installed-first-in-main · Telemetry sink installed first in each native main — Each native binary of the seven_guis package installs escher's telemetry sink in its native main before anything boots, reporting a failed install on stderr and continuing.
contract/escher-session-stdout/command-answer · escher-session stdout · command answer — Each command's answer, refusal or session-error object is one uncoloured line of JSON on stdout, an answer and never a log line; the host role ends quiet.
contract/telemetry-sink/rust-log · escher's telemetry sink · RUST_LOG — The sink's filter is read from RUST_LOG, and is warn when RUST_LOG is unset.
contract/telemetry-sink/stream · escher's telemetry sink · output stream — The sink writes uncoloured lines to stderr only, never to stdout.
decision/part-answer-never-logged · A part's answer is never logged — A part's answer ({"text":…,"part":n,"parts":m}) is a command's answer, written over MCP as its JSON line, and is never logged or fielded.
decision/sink-host-read-at-trace · Sink-installing host read at trace — A sink-installing host and its mcp role are read at trace under both builds, and must show no id, name or typed-sentinel needle in what they write. · guard: host_log, mcp_log
contract/mcp-role-exit-status/0 · escher-session mcp role exit status · 0 — The server ended without error.
contract/mcp-role-exit-status/3 · escher-session mcp role exit status · 3 — The server ended on an error, or a record directory was refused before serving; the fixed message is on stderr and stdout is empty.
contract/mcp-role-stdout/json-rpc · escher-session mcp role stdout · JSON-RPC lines — In the mcp role stdout carries newline-delimited JSON-RPC 2.0 protocol lines only, never written by the sink, and the role's end writes nothing on stdout.

## 6-log-coverage · ## 6. Log Coverage · twin log-coverage
decision/agent-run-no-captured-output · Agent-run events hold no captured output — Nothing between a failures: line and the next test result: line is read, so captured stdout and panic text never reach an agent-run event.
contract/agent-run-events/boot · agent-run harness events · boot — boot {ts, outcome, cargo_exit, head}, printed and appended.
contract/agent-run-events/cleanup · agent-run harness events · cleanup — cleanup {outcome}, printed only.
contract/agent-run-events/events-jsonl · agent-run harness events · events.jsonl — One JSON object per line on stdout, the same lines appended to target/agent-run/events.jsonl, recreated by boot and removed by cleanup; logs prints it verbatim.
contract/agent-run-events/run-end · agent-run harness events · run.end — run.end {ts, selection, passed, failed, ignored, cargo_exit, outcome}, printed and appended.
contract/agent-run-events/run-start · agent-run harness events · run.start — run.start {ts, selection, files}, printed and appended.
contract/agent-run-events/status · agent-run harness events · status — status {booted, boot_ts, head, run}, printed only.
contract/agent-run-events/test · agent-run harness events · test — test {file, test, outcome}, one per libtest result line, printed and appended.
contract/agent-run-test-event/file · The agent-run harness test event · file — The target binary's stem with its hash stripped, or doc:{crate}.
contract/escher-sink-env/rust-log · escher's sink configuration · RUST_LOG — Per-module levels come from RUST_LOG through EnvFilter, defaulting to warn when unset or unparsable.
contract/escher-telemetry-events/telemetry-installed · escher-telemetry events · telemetry installed — info at target escher_telemetry, message telemetry installed, once per successful init, with no argv, path or URL.
decision/escher-session-fixed-diagnostics · escher-session diagnostics are fixed strings — Every escher-session diagnostic is a fixed string holding no path, label, id or value, and no ending holds anything the caller typed.
contract/escher-driver-span-lines/default-level · escher-driver span lines · default warn level — The driver's command and tool_call span lines print only in a process that installs escher's sink at info or below; at the default warn there is no line.

## log-coverage · ## 6. Log Coverage · twin 6-log-coverage
contract/escher-session-commands/open-recording-directory · escher-session commands · recording directory open to group or others — `start` ends with status 3 and nothing spawned, and `mcp --record` with status 3, its fixed message on stderr and stdout empty.
contract/escher-session-exit-codes/0 · escher-session exit codes · 0 — Accepted: one line of JSON on stdout, stderr empty with `RUST_LOG` unset; also `help`, the host role's quiet end and the `mcp` role's `Ok` end.
contract/escher-session-exit-codes/1 · escher-session exit codes · 1 — Refused: one line of JSON on stdout, stderr empty with `RUST_LOG` unset.
contract/escher-session-exit-codes/2 · escher-session exit codes · 2 — Usage: one fixed line on stderr built from `VERBS` and the six task names, stdout empty.
contract/escher-session-exit-codes/3 · escher-session exit codes · 3 — Session error: the error object on stdout and its fixed message on stderr.
contract/escher-session-stderr/telemetry-not-installed · escher-session's stderr · telemetry not installed — The binary's one direct stderr print is `telemetry not installed: …`.
contract/host-wire/v3 · The escher-session host wire · v3 — Wire version `v3` carries `hello`, `stop` and `call`.
decision/fixed-diagnostics · Fixed diagnostics — Every escher-session diagnostic is a fixed string holding no path, label, id or value, and no ending holds anything the caller typed.
contract/command-span/line · The driver's command span · line — One closed-span line per command run, INFO at target `escher_driver`, `span="command"`, with the recorded ones of `verb`, `cause`, `settled`, `busy`, `passes`, `added`, `removed`, `changed`; no line at the default `warn`.
contract/host-wire/recording-failed · The escher-session host wire · recording-failed — A call after which the host's recording ended is answered by the one fixed reply `recording-failed`.

## 8-pii-scrubbing-compliance · ## 8. PII Scrubbing & Compliance · twin pii-scrubbing-compliance
decision/agent-run-log-no-user-content · Agent-run harness log carries no user content — The agent-run harness log sits outside the sink and carries no user content: none of its event keys is in the content-named set and no captured test output enters it.
decision/agent-run-raw-log-unscrubbed · Agent-run raw log left unscrubbed — The agent-run raw cargo/libtest output log stays unscrubbed, is never printed by logs, and is gitignored under target/. · why: Unscrubbed by design, like the CI logs.
decision/scrub-sink-reach · Reach of the scrubbing sink — The scrubbing sink is installed by the two binaries of the seven_guis package; engine events reach it only when an engine tracing feature is on. · except: The upstream apps' stdout subscribers and the WPT runner's env_logger stay unscrubbed.

## pii-scrubbing-compliance · ## 8. PII Scrubbing & Compliance · twin 8-pii-scrubbing-compliance
decision/session-logs-carry-no-page-content · Session logs carry no page content — The session host's, the mcp role's and every client's stderr carries no element id, accessible name, typed text, recording path or recorded content at any log level, under the package-alone and the workspace build. · guard: host_log, mcp_log
decision/shot-image-leaves-only-in-answer · Shot image leaves only in its answer — A shot's image is screen content that leaves only in the command's answer, never on a log line. · guard: host_log, mcp_log

## 9-ci-integration · ## 9. CI Integration · twin ci-integration
decision/local-state-never-uploaded · Local state stays local — The agent-run and cold-agent state, a session's state directory and a recording directory stay local and are uploaded by no CI leg.
decision/committed-kept-checks-clean · Committed kept checks hold no secrets — The kept checks committed in git hold no typed text, host path or secret.

## ci-integration · ## 9. CI Integration · twin 9-ci-integration
decision/agent-run-cleanup-scoped · Agent-run cleanup touches only its own state — The agent-run cleanup removes the harness's status, events and raw run log and touches nothing else under target/.
decision/session-state-dir-private · Session state directory private and empty of content — A session's state directory is created 0700 by the host and holds the socket file alone while a session is up and nothing after stop or idle expiry; no log, event or screen content is written into it.
decision/committed-checks-hold-no-private-data · Committed checks hold no private data — A kept check committed to git is one of the repository's own scripted stand-Counter sessions and holds no typed text, host path, name, secret or screenshot step. · why: The repository is public.

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
