# obs-plan — amendments

One entry per amendment to `obs-plan.md` (sidecar-contract.md §Entry form). Appended at wrap P2; rewritten only by the epoch-close consolidation.

## 2026-10-05-fork-ci-reached — per-leg CI failure logs; WPT and publish telemetry upstream-only
**Section:** §5 Metric Coverage (WPT scores (CI)) · §6 Log Coverage (CI publish builds) · §9 CI Integration (Telemetry artifact handling · publish-build logging · NOT YET MEASURED)
**Change:** §9 gains a row: each ci.yml leg's merged stdout+stderr, written by `ci-leg.sh` to `target/ci-logs/{leg}.log` (matrix `target/ci-logs/matrix-{platform}.log`), truncated at the leg's start, uploaded only on failure as `ci-log-{job id}` (`if-no-files-found: ignore`), kept 7 days, from `target/ci-logs/` alone, unscrubbed build output carrying no user data. The NOT YET MEASURED line was "log-file and snapshot artifact upload, CI resource attributes and artifact retention"; now "snapshot artifact upload and CI resource attributes". The WPT report archive and dispatch, `wptscores.json`, the §5 WPT-scores metric and the publish-build `CARGO_LOG` trace logging (§6, §9) are now marked upstream `DioxusLabs/blitz` only — their jobs are repository-guarded.
**Why:** a failing leg must be diagnosable without a re-run (the chunk's failure-artifact acceptance); the upload path is confined to `target/ci-logs/` so no artifact reaches the keystore or `Dioxus.toml`.
**Ref:** .andromeda/runs/2026-10-05T21-59-55-wrap/

## 2026-10-05-fork-ci-reached — file:line citations re-pointed after the CI files moved
**Section:** every section citing `Cargo.toml`, `wpt.yml` or `publish-browser.yml` lines
**Change:** 8 citations re-pointed — `Cargo.toml` from line 195 on +3, `wpt.yml` from line 26 on +1, `publish-browser.yml` from line 37 on +1; no claim text changed by the re-point.
**Why:** this chunk moved the cited lines.
**Ref:** .andromeda/runs/2026-10-05T21-59-55-wrap/

## 2026-10-05-ci-gate-legs — a coverage-report artifact beside the per-leg failure logs
**Section:** §9 CI Integration (Telemetry artifact handling) · every section citing `ci.yml` or `ci-leg.sh` lines
**Change:**
- §9 gains a row: the `coverage` leg's lcov file `target/coverage/lcov.info` — line counts of the workspace's public source, no user data — uploaded only on success as artifact `coverage-report` from `target/coverage/`, kept 7 days, the one fork-CI artifact outside `target/ci-logs/`. The per-leg failure-log row stands, confined to `target/ci-logs/`; the confinement rationale of "2026-10-05-fork-ci-reached — per-leg CI failure logs; WPT and publish telemetry upstream-only" now holds for the failure logs only;
- 3 citations re-pointed by a measured line map; no claim text changed by the re-point.
**Why:** test-plan §3 `coverage-tooling-install` — a coverage report kept with the run. A boundary widening (a new upload out of fork CI), recorded PROVISIONAL: delegate overseer, 2026-10-05, under the founder's standing delegation of technical decisions (relayed verbatim by overseer) — basis: public OSS line counts, no secret, the repository's own Actions store, 7-day retention; the founder's own later word supersedes it.
**Ref:** .andromeda/runs/2026-10-05T23-50-17-wrap/

## 2026-10-06-telemetry-bootstrap — escher-telemetry: stderr subscriber, identity, line format, chaining panic hook, allowlist scrub
**Section:** §1 Instrumentation scope · §2 Telemetry mechanism · §3 Logging stack · §3 Service identity and line format · §3 → Bootstrap phases (key file) · §6 Log format · §6 Logged events · §6 Absent · §7 Panic hooks · §8 Values logged as-is · §8 Scrubbing · every section citing `Cargo.toml` or `examples/seven_guis` lines
**Change:**
- §3 Logging stack: was `fmt::init()` as the native install; now `seven_guis_native` calls `escher_telemetry::init` (Registry + `EnvFilter` from `RUST_LOG`, default `warn`, + one non-ANSI fmt layer, stderr only, `LogTracer` bridge; an `Err` is `eprintln!`ed), and `fmt::init()` is the upstream apps' stdout install.
- §3: service identity measured — `service.name` / `service.version` from the binary's `CARGO_PKG_*`; the marker keeps product mode, a JSON schema, log file, snapshot, trace context, heartbeat.
- §6: the one-line text format and `RUST_LOG` levels; events `telemetry installed` (info, `escher_telemetry`) and `panic` (ERROR, `escher_telemetry::panic`); the examples slice no longer `println!`-only.
- §2 / §1: tracing compiled only with the `tracing` feature is the engine and upstream crates' rule — escher-telemetry is ungated; §1 lists tracing-log and the escher-telemetry entity.
- §7: the chaining native hook (logs, then the previous hook; std still prints the raw message).
- §8: was "Scrubbing (absent)"; now the allowlist scrub — engine prefixes `blitz`, `dioxus_native`, `stylo_taffy`, `accesskit_xplat`, `debug_timer`, `js_console` print only `node_id`, `status`, `waiting_nodes`, `property`, `log.module_path`, `log.file`, `log.line`; `url`, `href`, `src`, `html`, `text`, `value`, `attrs`, `path`, `request`, `error`, `panic.payload` redacted everywhere — reaching escher's sink only; the slice search records kept; residue as-is: the raw panic message, `log.file` host paths.
- Bootstrap phases: `pii-scrubbing-wire` discharged for escher's sink, open for the upstream sinks; `otel-sdk-install` open, the opt-in export carried to "Driver command spans".
- 7 `file:line` citations re-pointed.
**Why:** the telemetry bootstrap chunk shipped the bootstrap; OTel export was deferred at P4 by the overseer delegate under the founder's standing delegation of technical forks (egress + credential path), provisional on the founder's word.
**Ref:** .andromeda/runs/2026-10-06T01-33-33-wrap/

## 2026-10-06-headless-stand — telemetry installer named; the headless stand installs none
**Section:** §3 Observability Harness Contract → Logging stack
**Change:** was "escher's stand installs `escher_telemetry::init`"; now `seven_guis_native` (the windowed stand binary) installs it, and the headless stand `seven_guis::stand` with its in-process checks installs no subscriber — no `escher_telemetry::init`, no `println!`, no env read — so a headless boot has no escher sink.
**Why:** after the headless stand chunk "the stand" names two surfaces; only the windowed binary installs telemetry, and init stays once per process.
**Ref:** .andromeda/runs/2026-10-06T02-44-36-wrap/

## 2026-10-06-stand-test-contract — the agent-run harness log
**Section:** §3 Observability Harness Contract · §6 Log Coverage · §8 PII Scrubbing & Compliance · §9 CI Integration
**Change:**
- §3: a new "Agent-run harness log" paragraph — `scripts/agent-run.sh` writes JSON lines encoded by python3's `json` (`boot`, `run.start`, `test`, `run.end` printed and appended to `target/agent-run/events.jsonl`; `status`, `cleanup` printed only); harness metadata, not a telemetry sink. The marker was "product mode, a JSON log schema, log file location, …"; it now reads "a JSON schema or log-file location for escher's own sink".
- §6: a "Log format (the agent-run harness)" block — the event fields, one `test` per libtest line, nothing read inside a `failures:` block, raw output in `run.log`; the marker is scoped "for escher's own sink".
- §8: the scrub-reach list adds the harness log — outside escher's scrub, no content-named field, no captured output; `run.log` unscrubbed like `target/ci-logs/`.
- §9: the artifact table adds "Agent-run harness state" — `target/agent-run/`, local only, uploaded by no CI leg.
**Why:** the stand test contract chunk added a JSON-line harness log beside escher's text sink.
**Kept:** escher's own sink stays one text line per event; its JSON schema and file sink stay unmeasured.
**Ref:** .andromeda/runs/2026-10-06T03-39-41-wrap/

## 2026-10-06-cold-agent-run-pipe — the cold-agent pipe log, scrub position and artifact
**Section:** §3 Observability Harness Contract · §6 Log Coverage (Log format) · §8 Scrubbing · §9 Telemetry artifact handling
**Change:**
- §3: the cold-agent pipe log — python3-`json` events (`run.start`, `run.end` appended to `target/cold-agent/events.jsonl`; `status`, `cleanup` printed only), `verdict.json`, the stub's call log; no `tracing`, OTel or third-party logger, no env read.
- §6: its log format — the four events, the verdict fields, the call-log line `{seq, tool, outcome, cause}` (an unknown tool logged `tool: null`), the raw transcript and client stderr never printed.
- §8: the pipe sits outside escher's sink with no content-named key and no transcript content in events or verdict; `transcript.jsonl` is raw by design (model output, tool I/O, host paths, the client's socket path, a rate-limit line), never printed, gitignored; the one committed copy host-path-masked.
- §9: a `Cold-agent pipe state` row — local only, gitignored, uploaded by no CI leg, removed by its `cleanup` alone; the per-run session dir removed on exit.
**Why:** the cold-agent run pipe chunk added a harness log beside agent-run's. The §8 record was a D-obs-pii escalation, ratified at this wrap by the overseer under the founder's standing delegation; no rule added — `gate.py hygiene` P1 already refuses an unmasked host path in committed evidence.
**Ref:** .andromeda/runs/2026-10-06T04-34-08-wrap/
