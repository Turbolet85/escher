# obs-plan · the rows that bind 2026-11-19-build-directory-held — picked 57 of 290 (27 by every draw) · rule any · pin 2275

## 2-telemetry-strategy · ## 2. Telemetry Strategy · twin telemetry-strategy
decision/engine-tracing-by-invocation · Engine tracing set by the build's invocation — A build that resolves the whole workspace turns the engine's tracing feature on while a package-alone build of the test crate or seven_guis leaves it off, so under the workspace build a test child that installs escher's sink at info also receives engine events.
decision/escher-crates-always-trace · Escher crates take tracing unconditionally — The escher telemetry and driver crates take tracing with no feature, so the startup and panic events and the command span are always compiled into the seven_guis package's two native binaries, while their engine features stay off in a package-alone build.
decision/telemetry-via-tracing · Telemetry through the tracing crate — Telemetry is the tracing crate's event and span macros; the engine and upstream crates compile their log sites only with the tracing cargo feature, each call a no-op when the feature is off.

## telemetry-strategy · ## 2. Telemetry Strategy · twin 2-telemetry-strategy
decision/engine-tracing-feature-gated · Engine tracing behind a cargo feature — In the engine and upstream crates, tracing is compiled in only with the tracing cargo feature.
decision/escher-crates-tracing-ungated · escher crates take tracing ungated — escher-telemetry and escher-driver take tracing with no feature gate, so the startup and panic events and the driver's spans are always compiled into the binaries that use them.
decision/tracing-for-telemetry · tracing as the telemetry crate — Telemetry is written with the tracing crate's event macros and span macros.

## 3-observability-harness-contract · ## 3. Observability Harness Contract
decision/agent-run-raw-output-never-printed · Agent-run raw output never printed — The raw merged cargo and libtest output goes to target/agent-run/run.log, truncated at each run's start and never printed.
decision/empty-stderr-check-unsets-rust-log · Empty-stderr checks unset RUST_LOG — A check that reads an accepted command's empty stderr removes RUST_LOG from the spawned child's environment.
decision/headless-stand-no-sink · Headless stand runs with no sink — A headless boot made in process installs no telemetry sink and reads no environment variable, its acting steps running only under escher-telemetry's scoped capture, which prints nothing; a headless boot made by the escher-session binary runs under that binary's sink. · except: The MCP tools and layout stand checks install their own recording subscriber; the command-span check's re-run children and the error-text founder check install escher's sink over an in-memory writer.
decision/sink-host-read-at-trace · Sink-installing host read at trace — A sink-installing host and its mcp role are read at trace under both builds, and must show no id, name, typed-sentinel or layout-answer key needle in what they write. · guard: host_log, mcp_log
decision/sink-installed-first-in-main · Telemetry sink installed first in each native main — Each native binary of the seven_guis package installs escher's telemetry sink in its native main before anything boots, reporting a failed install on stderr and continuing.
contract/telemetry-sink/rust-log · escher's telemetry sink · RUST_LOG — The sink's filter is read from RUST_LOG, and is warn when RUST_LOG is unset; it filters the sink layer alone, the capture layer beside it unfiltered.
decision/cold-agent-server-log-counted-only · Cold-agent server log read only to count — The cold-agent pipe reads server.log only to count its span lines, never printing or copying it.
decision/cold-agent-state-never-printed · Cold-agent state never printed or copied — The cold-agent state files (transcript.jsonl, client.log, server.log, check.json and the record directory) are never printed or copied.

## 6-log-coverage · ## 6. Log Coverage · twin log-coverage
contract/agent-run-test-event/file · The agent-run harness test event · file — The target binary's stem with its hash stripped, or doc:{crate}.
contract/escher-sink-env/rust-log · escher's sink configuration · RUST_LOG — Per-module levels come from RUST_LOG through EnvFilter, defaulting to warn when unset or unparsable, and apply to the sink layer alone, never to the capture layer beside it.
contract/escher-telemetry-events/panic · escher-telemetry events · panic — ERROR at target escher_telemetry::panic, message panic, fields panic.file, panic.line, panic.column and panic.payload (redacted), once per panic.
contract/escher-telemetry-events/telemetry-installed · escher-telemetry events · telemetry installed — info at target escher_telemetry, message telemetry installed, once per successful init, with no argv, path or URL.
decision/agent-run-no-captured-output · Agent-run events hold no captured output — Nothing between a failures: line and the next test result: line is read, so captured stdout and panic text never reach an agent-run event.
contract/agent-run-events/boot · agent-run harness events · boot — boot {ts, outcome, cargo_exit, head}, printed and appended.
contract/agent-run-events/cleanup · agent-run harness events · cleanup — cleanup {outcome}, printed only.
contract/agent-run-events/events-jsonl · agent-run harness events · events.jsonl — One JSON object per line on stdout, the same lines appended to target/agent-run/events.jsonl, recreated by boot and removed by cleanup; logs prints it verbatim.
contract/agent-run-events/run-end · agent-run harness events · run.end — run.end {ts, selection, passed, failed, ignored, cargo_exit, outcome}, printed and appended.
contract/agent-run-events/run-start · agent-run harness events · run.start — run.start {ts, selection, files}, printed and appended.
contract/agent-run-events/status · agent-run harness events · status — status {booted, boot_ts, head, run}, printed only.
contract/agent-run-events/test · agent-run harness events · test — test {file, test, outcome}, one per libtest result line, printed and appended.
contract/cold-agent-events/cleanup · cold-agent pipe events · cleanup — cleanup {outcome: "done"}, printed only.
contract/cold-agent-events/events-jsonl · cold-agent pipe events · events.jsonl — One JSON object per line on stdout, the same lines appended to target/cold-agent/events.jsonl, recreated by run and removed by cleanup; logs prints it.
contract/cold-agent-events/run-end · cold-agent pipe events · run.end — run.end carrying every verdict field, printed and appended.
contract/cold-agent-events/run-start · cold-agent pipe events · run.start — run.start {ts, task}, printed and appended.
contract/cold-agent-events/status · cold-agent pipe events · status — status {the verdict}, or status {outcome: "none"} with exit 3, printed only.
contract/escher-driver-span-lines/default-level · escher-driver span lines · default warn level — The driver's command and tool_call span lines print only in a process that installs escher's sink at info or below; at the default warn there is no line.

## log-coverage · ## 6. Log Coverage · twin 6-log-coverage
decision/actions-read-changes-nothing · Actions read changes nothing in the document — The actions read acts on nothing and changes nothing in the document. · why: stand_actions' the_read_changes_nothing_and_holds_no_typed_text_or_mask pins no change mark, unchanged snapshot, focus and empty diff
decision/cold-agent-server-log-read-by-span-name · Cold-agent server log read by span name only — The cold-agent pipe reads the server log by span name, the tool_call span's tool and ending and the command span's cause word only, and never prints or copies it.
decision/decoded-screen-text-stays-in-the-match · Decoded screen text stays in the match — The cold-agent pipe's decoded paged screen text goes into the expect match only, never into an event or the verdict.
contract/command-span/line · The driver's command span · line — One closed-span line per command run, INFO at target `escher_driver`, `span="command"`, with the recorded ones of `verb`, `cause`, `settled`, `busy`, `passes`, `added`, `removed`, `changed`; no line at the default `warn`.
contract/escher-session-stderr/telemetry-not-installed · escher-session's stderr · telemetry not installed — The binary's one direct stderr print is `telemetry not installed: …`.
contract/tool-call-span/help · The driver's tool_call span · help — A `help` call's line reads `span="tool_call" tool="help" ending="accepted"` with no `command` line beside it.
contract/tool-call-span/line · The driver's tool_call span · line — One closed-span line per MCP tool call, INFO at target `escher_driver`, `span="tool_call"`, with the recorded ones of `tool` and `ending`, then `message="close"`, `time.busy`, `time.idle`; no line at the default `warn`.

## 8-pii-scrubbing-compliance · ## 8. PII Scrubbing & Compliance · twin pii-scrubbing-compliance
decision/agent-run-log-no-user-content · Agent-run harness log carries no user content — The agent-run harness log sits outside the sink and carries no user content: none of its event keys is in the content-named set and no captured test output enters it.
decision/cold-agent-log-no-user-content · Cold-agent pipe log carries no user content — The cold-agent pipe log sits outside the sink and carries no user content: no event or verdict key is content-named and no transcript text, tool argument, answer, reply or check enters an event or the verdict.
decision/scrub-sink-reach · Reach of the scrubbing sink — The scrubbing sink is installed by the two binaries of the seven_guis package; engine events reach it only when an engine tracing feature is on. · except: The upstream apps' stdout subscribers and the WPT runner's env_logger stay unscrubbed.
decision/agent-run-raw-log-unscrubbed · Agent-run raw log left unscrubbed — The agent-run raw cargo/libtest output log stays unscrubbed, is never printed by logs, and is gitignored under target/. · why: Unscrubbed by design, like the CI logs.
decision/cold-agent-kept-checks-at-rest · Cold-agent kept checks held at rest only — The live cold-agent session's kept checks live in an owner-only record directory, holding calls and answers at rest, never printed, copied or committed.
decision/cold-agent-transcript-raw · Cold-agent transcript kept raw — The cold-agent transcript, raw model output and tool I/O, carries the stand's screens to the model provider and holds host paths, unscrubbed, never printed, and gitignored under target/. · why: Carried to the model provider by design.

## pii-scrubbing-compliance · ## 8. PII Scrubbing & Compliance · twin 8-pii-scrubbing-compliance
decision/cold-agent-events-carry-no-user-content · Cold-agent events carry no user content — No cold-agent pipe log event or verdict key is in the content-named set, and no transcript text, tool argument, tool answer, reply or check enters an event or the verdict.
decision/session-logs-carry-no-page-content · Session logs carry no page content — The session host's, the mcp role's and every client's stderr carries no element id, accessible name, typed text, recording path or recorded content at any log level, under the package-alone and the workspace build. · guard: host_log, mcp_log
decision/cold-agent-artifacts-never-printed · Cold-agent server log and check never printed — The cold-agent server log is counted by span name and never printed or copied, and the check file holding the agent's check is never printed.
decision/cold-agent-transcript-unscrubbed · Cold-agent transcript unscrubbed — The cold-agent transcript holds raw model output and tool I/O, carried to the model provider, never printed or logged and gitignored under target. · why: unscrubbed by design, the provider crossing ratified
decision/driver-span-fields-print · Driver span fields tool and ending print — The tool_call span's fields tool and ending are not in the content-named set and print. · why: both are closed words; the closed-span line was ratified

## 9-ci-integration · ## 9. CI Integration · twin ci-integration
decision/fork-ci-two-artifacts · Fork CI uploads two artifacts — The fork's CI uploads exactly two artifacts, a leg's merged log on failure and the coverage report on success, each kept 7 days, neither holding user data.
decision/local-state-never-uploaded · Local state stays local — The agent-run and cold-agent state, a session's state directory and a recording directory stay local and are uploaded by no CI leg.

## ci-integration · ## 9. CI Integration · twin 9-ci-integration
decision/agent-run-cleanup-scoped · Agent-run cleanup touches only its own state — The agent-run cleanup removes the harness's status, events and raw run log and touches nothing else under target/.
decision/ci-log-artifact-on-failure · Per-leg CI log uploaded on failure — Each CI leg's merged stdout and stderr is written to its own log, truncated at the leg's start, and uploaded only on failure as artifact ci-log-{job id}, kept 7 days; it is unscrubbed build output holding no user data.
decision/cold-agent-cleanup-scoped · Cold-agent cleanup touches only its own state — The cold-agent cleanup removes the pipe's state and touches nothing else under target/; the per-run temporary session directory is removed when the run exits.
decision/cold-agent-kept-checks-stay-local · Cold-agent kept checks stay local — The cold-agent pipe's live run keeps its checks in an owner-only (mode 700) recording directory; the kept checks are never printed, copied, uploaded by a CI leg or committed, and are removed with the rest of the state at cleanup; the replay children run without recording.
decision/coverage-artifact-on-success · Coverage report uploaded on success — The coverage leg's lcov file, line counts of the workspace's public source with no user data, is uploaded only on success as artifact coverage-report, kept 7 days.

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
