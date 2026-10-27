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

## 2026-10-06-upstream-sync-element-identity — file:line citations re-pointed after the upstream merge
**Section:** every section citing a merged upstream file's lines
**Change:** 20 `file:line` citations into the merged upstream files re-pointed by the merge's measured line map; no claim text changed by the re-point.
**Why:** the chunk merged upstream `main` at `23354585`; the merged files' lines moved. The merge added no log, print or tracing site.
**Ref:** .andromeda/runs/2026-10-06T08-31-16-wrap/

## 2026-10-06-upstream-sync-element-identity — root-manifest citations re-pointed after the line-61 insert
**Section:** §1 Obs Scope Summary · §5 Metric Coverage · §7 Error Capture & Reporting (every root `Cargo.toml` citation past line 60)
**Change:** 6 root `Cargo.toml` citations re-pointed +1 per number past 60 — they read one line low since the `seven_guis` path entry was inserted at `Cargo.toml:61`; each verified against the cited text. No claim text changed.
**Why:** the 2026-10-06-headless-stand wrap did not re-point the root-manifest citations past its insert; the operator chose at this wrap's escalation (2026-10-06) to fix them in this pass rather than carry them.
**Ref:** .andromeda/runs/2026-10-06T08-31-16-wrap/

## 2026-10-06-id-persistence — one println! in the stand checks: the re-exec child's captured ids
**Section:** §3 Observability Harness Contract → Logging stack (the headless-stand bullet)
**Change:** the stand bullet said "no `escher_telemetry::init`, no `println!`, no env read in it or its checks". It now says: no `escher_telemetry::init` and no env read; no `println!` save `stand_id_persistence`'s re-executed child, which prints its pid and ids to stdout that the parent captures and reads only into its assertion — never a log. A headless boot still has no escher sink.
**Why:** the fresh-process proof needs the child's ids in the parent. Captured stdout read by an assertion is not a log channel, so the no-sink invariant holds.
**Ref:** .andromeda/runs/2026-10-06T10-55-07-wrap/

## 2026-10-06-accessibility-tree-identity — citations re-pointed
**Section:** citations into blitz-dom `document.rs` and blitz-shell `window.rs`
**Change:** 10 citations re-pointed by the chunk's measured line map (`document.rs` +10 from old line 151; `window.rs` −1 from old 525); no claim text changed — the chunk added no log, span, metric or event field.
**Why:** the new `Document::accessibility_tree` method and the shell's re-threaded callers moved the cited lines.
**Ref:** .andromeda/runs/2026-10-06T12-55-34-wrap/

## 2026-10-06-founder-rulings — OTel export out of escher 0.1.0; coverage-report upload ratified
**Section:** §3 Observability Harness Contract → Bootstrap phases (`otel-sdk-install`) · §9 CI Integration (`coverage-report`)
**Change:**
- `otel-sdk-install`: was "the opt-in export was deferred … (an egress and `OTEL_EXPORTER_OTLP_HEADERS` credential-path decision for the founder) and is carried to "Driver command spans""; now escher 0.1.0 ships no OTel export — no OTel crate, no egress, no credential path — with the export transport and the credential path both undecided, held in `.andromeda/residuals.md`. "Driver command spans" keeps the driver's spans and loses the export.
- `coverage-report` upload: no body text changes; now ratified, no longer PROVISIONAL per the 2026-10-05-ci-gate-legs entry.
**Why:** the founder chose option (c), no OTel export in escher 0.1.0, and ratified the upload, in their own word at the Epoch 2 boundary (the founder, 2026-10-06), superseding the delegate overseer's provisional deferral by rule.
**Ref:** .andromeda/runs/2026-10-06T16-16-18-wrap/

## 2026-10-06-snapshot-state-fidelity — bridge log-site citations re-pointed
**Section:** §6 Log Coverage → Logged events · §8 PII Scrubbing & Compliance → Values logged as-is
**Change:** no claim changes. 6 `mutation_writer.rs` line citations re-pointed: §6 `:119-205` · `:305` · `:388` are now `:152-238` · `:338` · `:421`; §8 `:150` · `:202` · `:388` are now `:183` · `:235` · `:421`.
**Why:** the chunk inserted a 33-line constant above the bridge's log sites and added none — the `trace!` count stays 13, and the snapshot module and the two new test files hold no log, print or env site.
**Ref:** .andromeda/runs/2026-10-06T22-39-16-wrap/

## 2026-10-07-change-tracking-and-diff — blitz-dom log-site citations re-pointed
**Section:** citations into blitz-dom `document.rs` and `mutator.rs`
**Change:** no claim changes. 14 of 16 citations re-pointed by the chunk's measured line map — 8 into `document.rs` (`+7` for old lines 1011–1548, `+11` from 1549) and 6 into `mutator.rs` (`+45` from old line 675); the 2 into blitz-shell `window.rs` keep their numbers.
**Why:** the chunk inserted lines above the cited log sites — the changed-set marks in the mutator, the drain and the mark in the document — and added no log, print, env read or file write: the census over its two new files and every line it added reads 0, and no agent-run event carries a content-named key.
**Ref:** .andromeda/runs/2026-10-07T01-31-03-wrap/

## 2026-10-07-audit-corrections — the headless-stand census covers the checks' shared module
**Section:** §3 Observability Harness Contract → Logging stack (the headless-stand bullet)
**Change:** the headless stand installs no subscriber — no `escher_telemetry::init` and no env read in it, its checks or their shared module `tests/blitz-tests/tests/common/mod.rs`, and no `println!` in any of them save `stand_id_persistence`'s re-executed child (was: "in it or its checks"). The shared module's head is cited; the two existing citations keep their numbers.
**Why:** six stand checks now read their tables and helpers, `boot` among them, from a module that is not itself a `stand_*` check, so the census had to name it; it reads 0 there. The chunk added no log site, print, subscriber, env read or file write anywhere.
**Kept:** the chunk's other new file, dioxus-native-dom's `dioxus_document_tests.rs`, is named in no obs-plan sentence — the plan has none about that crate's unit tests; its census of 0 is in the chunk's report.
**Ref:** .andromeda/runs/2026-10-07T03-59-09-wrap/

## 2026-10-07-driver-session — a second sink installer; the scrub's reach scoped to the measurement
**Section:** §1 Instrumentation scope (examples · citations) · §2 Telemetry mechanism · §3 (Logging stack · Service identity) · §3 → Bootstrap phases (`pii-scrubbing-wire`) · §4 (new census line) · §5 (citation) · §6 Logged events (examples · escher-telemetry group) · §7 Panic hooks · §8 (Values logged as-is · Scrubbing) · §9 Telemetry artifact handling (new row)
**Change:**
- Installers: was `seven_guis_native` alone at eight sites; now the two binaries of the seven_guis package, `seven_guis_native` and the session host `escher-session`, both under `service.name=seven_guis` — the name no longer tells them apart; `escher-session`'s own three `eprintln!` lines recorded; its stdout empty.
- Headless census: "a headless boot has no escher sink" scoped to a boot made in process — it now covers `packages/escher-driver`, the five `stand_session_*` checks and `session_common/mod.rs`; a boot made by the `escher-session` binary runs under its sink; the two spawning checks and `host_binary` described.
- §8: the past-the-scrub list gains "a record from a target outside the engine allowlist prints as written", with the reading on `escher-session` — 0 lines at `warn`; 1 line, no id, no name at `info`; stable ids at `debug`; ids and an accessible name at `trace`; stdout 0 bytes. Typed text and the windowed stand by level stated as NOT measured. The reach bullet names both installers and the limit.
- `pii-scrubbing-wire`: was discharged for escher's own sink; now discharged for engine targets and content-named fields, open for third-party targets, owed by the route entry "Sink target allowlist".
- §4: escher-driver carries no span, event or `tracing` dependency; per-command spans owed by "Driver command spans". §9: the session state directory row (socket file only, local, uploaded by no CI leg).
- Citations: seven root `Cargo.toml` sites re-pointed (+1 / +2).
**Why:** the chunk added a second binary that installs the sink and measured its stderr by level. The founder ruled to record the finding honestly and fix it in the chunk right after (the founder, 2026-10-07, relayed verbatim). Trap: a zero read from a host that installs no sink says nothing about one that does.
**Ref:** .andromeda/runs/2026-10-07T06-51-52-wrap/

## 2026-10-07-sink-target-allowlist — outside-target records dropped; both sink-installing binaries measured by level
**Section:** §8 PII Scrubbing & Compliance → Values logged as-is, Scrubbing · §6 Log Coverage → Log format, Logged events · §3 Observability Harness Contract → Logging stack, Service identity and line format · §3 → Bootstrap phases (derive for route / setup-project) · §9 CI Integration → Session state directory · §1 Obs Scope Summary · §2 Telemetry Strategy
**Change:**
- Scrubbing: was a two-rule scrub whose reach clause let any other target pass "with its message and fields unredacted", the drop "owed by the route entry" (per "2026-10-07-driver-session — a second sink installer; the scrub's reach scoped to the measurement", whose second-installer half stands); now two public prefix sets and three outcomes — `ENGINE_TARGET_PREFIXES` scrubbed to the seven safe fields, `ESCHER_TARGET_PREFIXES` (`escher_`, underscore included; `escher` alone is not admitted) printed with the eleven content-named fields redacted, every other target dropped whole before anything is written, at every level, WARN and ERROR included, whatever `RUST_LOG` names. The dropped families are named — the windowed stand's `naga`, `wgpu`, `winit_wayland`, `sctk` and `calloop` among them — and the cost: a third-party WARN or ERROR no longer prints.
- Values logged as-is: the by-level readings restated before → after — `escher-session` 0 → 0, 1 → 1, 1165 → 1, 1501 → 1 stderr lines (unset, `info`, `debug`, `trace`), ids 15 of 15 → 0, names 6 of 6 → 0; the windowed stand, its first reading by level, 1 → 0, 3 → 1, 12,413 → 1, 47,482 → 1, ids 11 of 11 → 0, names 5 of 5 → 0; stdout 0 bytes in all 16. "NOT measured by level" is retired; typed text stays NOT measured. `log.file` can carry a host path only for a bridged record under an engine target — recorded by construction, not measured.
- Log format: one line per printed event, nothing at all for an outside target; a `RUST_LOG` directive naming an outside target does not re-admit it.
- pii-scrubbing-wire: the third-party-target clause discharged; the upstream apps' and the WPT runner's clause stays open.
- Logging stack: `host_log` joins the host-spawning checks, with the shared module `examples/seven_guis/tests/common/mod.rs`. Session state directory: `hl-trace` added.
- Seven citations into the sink's two source files and one into `host_binary.rs` re-pointed.
**Why:** the chunk delivered the fix the founder ruled should follow the host-log finding (the founder, 2026-10-07, relayed verbatim by the overseer, as the earlier entry records); the drop's place in the formatter and its reach to every level were approved by the operator at the plan review (the operator, 2026-10-07). Rule for later chunks: a target is admitted only together with a scrub rule for it.
**Ref:** .andromeda/runs/2026-10-07T08-23-43-wrap/

## 2026-10-07-settle-detection — the settle wait exists and is silent
**Section:** §3 Observability Harness Contract → Logging stack (the no-subscriber census) · §4 Span / Trace Coverage (the escher-driver bullet)
**Change:**
- §3: the census of what installs no subscriber, reads no env var and prints nothing now names the session step `Session::act`, the harness's settle loop (`Harness::settle`; no `tracing` dependency in the crate) and the check `stand_settle`; was "the five `stand_session_*` checks and … their module"; now the shared session module is read by four of those five and by `stand_settle`.
- §4: the driver bullet names `Session::act` and the settle wait it runs, `Harness::settle`, as observed absent of spans, events and a `tracing` dependency; the settle wait exists and carries no span — one span per driver command, covering it, stays owed by the route entry "Driver command spans".
**Why:** the chunk built the wait the owed span is for and kept both crates silent by its constraint. Rule for later chunks: add no span, event or log line to the settle loop or the session step before "Driver command spans".
**Ref:** .andromeda/runs/2026-10-07T11-06-31-wrap/

## 2026-10-07-command-and-refusal-schema — the command schema is silent; the cause names are the owed span field's domain
**Section:** §3 Observability Harness Contract → Logging stack (the no-subscriber census) · §4 Span / Trace Coverage (the escher-driver bullet)
**Change:**
- §3: the session library's silence clause was "its settled step `Session::act` included"; now it also includes the command and refusal schema — the private modules `command`, `refusal` and `schema` install no subscriber, read no env var and no clock, and print or log nothing — and records that `Command`, `Call` and `ArgValue` derive `Debug`, which prints the id and the text they hold, while nothing in the crate prints, logs or fields them.
- §4: the observed-absent bullet names the schema (`validate`, `VERBS`, `Refusal` with `Cause` and `Fault`) beside the session library. The span per driver command stays owed by "Driver command spans"; the value domain its refusal-cause field reads now exists — the eight names `Cause::name` returns. An argument fielded by its schema name, or a `Command` fielded with `Debug`, would print an id and typed text: of the argument names only `text` is in the sink's scrub set.
**Why:** the chunk added the driver's schema and kept the crate silent by its constraint. Rule for later chunks: the span's cause field takes a `Cause` name; never field a `Command`, `Call` or `ArgValue` with `Debug`, nor an argument by its schema name.
**Ref:** .andromeda/runs/2026-10-07T12-34-00-wrap/

## 2026-10-07-act-by-id — the executor is silent; the command span is still owed; typed text still not measured
**Section:** §3 Observability Harness Contract → Logging stack (the headless-stand bullet) · §4 Span / Trace Coverage (the escher-driver bullet) · §6 Log Coverage → Logged events (examples) · §8 PII Scrubbing & Compliance → Values logged as-is · §9 CI Integration → Telemetry artifact handling
**Change:**
- §3: the session library's silence now names its executor — the private module `execute`, `Session::run` and `Session::with_time` install no subscriber, read no env var and no clock, and print or log nothing; `Outcome` joins `Command`, `Call` and `ArgValue` as a `Debug`-deriving type nothing prints, logs or fields. The six `stand_act_*` checks install no sink and print nothing. `mod common;` is read by 14 stand checks (was eleven); `session_common` by eleven of the twelve checks that hold a session (was four of five and `stand_settle`).
- §4: was "held in process, reached by no socket and taken by no `Session` method"; now the schema is run in process by `Session::run`, and the settle wait is run by `act` and by each acting verb of `run`. The command executor exists and carries no span; the one span per driver command is still owed by "Driver command spans". Of the eight cause names four are returned by code today — `unknown-verb` and `malformed` by `validate`, `not-found` and `time-unavailable` by the executor. An `Outcome` fielded with `Debug` would print an id, an accessible name or a control's value.
- §8: was "no command can type into the held instance yet"; now a typing command exists and types into an instance held in process only, where no sink is installed; none reaches a sink-installing host, so typed text in a host's log is still NOT measured.
- Three citations re-pointed: `stand.rs`, `session_host.rs`, `session_common/mod.rs`.
**Why:** the chunk built the executor under the crate's standing constraint — no `tracing` dependency, no subscriber, no env read, no print. One detector proposal rested on its own grep and was rejected as proposed; its fact — the new checks print nothing — was re-measured by the wrap. Rule kept for the span's owner: field a cause by `Cause::name`, never a `Command`, a `Call`, an `ArgValue` or an `Outcome` with `Debug`.
**Kept:** the two `packages/escher-driver/Cargo.toml` citations — the range now holds the `accessibility` feature line, and the claim beside each (two dependencies, no `tracing`) still holds.
**Ref:** .andromeda/runs/2026-10-07T14-22-35-wrap/

## 2026-10-07-refusal-detection — eight cause names returned; the record among the silent values
**Section:** §3 Observability Harness Contract → Logging stack · §4 Span / Trace Coverage · §9 (one citation)
**Change:**
- §3: the driver crate stays silent with the refusal detection and the sixth verb `scroll`; the session's record of ids — id text, at most 4096 ids, no reader outside the crate, no `Debug` — is printed, logged and fielded by nothing. The silent check set names nine `stand_act_*` checks (was six) and `scroll_into_view_nested`; fourteen of the fifteen session-holding checks read `session_common` (was eleven of twelve).
- §4: the refusal-cause field's domain is returned by code eight of eight (was four of eight), six by the executor.
- Two citations re-pointed (`command.rs:161` → `:167`; `session_common/mod.rs:194-202` → `:232-240`).
**Why:** the chunk built the detection under the crate's silence constraint; the per-command span is still owed by the route entry "Driver command spans". The plan's wording "six of the eight" was the executor's share; all eight are returned.
**Ref:** .andromeda/runs/2026-10-07T20-27-47-wrap/

## 2026-10-07-driver-command-spans — the driver's command span as built; `tracing` ungated in escher-driver
**Section:** §1 (escher-telemetry, one citation) · §2 Telemetry mechanism, Feature wiring · §3 → Logging stack, Service identity (one citation), Agent-run harness log · §4 Span / Trace Coverage · §6 → Logged events
**Change:**
- §4: the `escher-driver` bullet was "observed absent … one span per driver command … is owed by the route entry"; now the span as built — target `escher_driver`, name `command`, INFO, `tracing::info_span!`, entered before `validate`; eight fields each recorded only where it applies (`verb` · `cause` · `settled` · `busy` · `passes` · `added` · `removed` · `changed`); never fielded: `in_view`, `advanced_ms`, the screen text or its length, the label, the record of ids, any argument, any `Command`, `Call`, `ArgValue`, `Outcome`, `Refusal` or `Fault`. Still observed absent: a span, event or `tracing` dependency in `Harness::settle`, and a span of their own on `Session::act` and the lifecycle. `busy` is covered by its unit test only.
- §2: escher-driver is the second escher crate to take `tracing` ungated; its span is compiled into both seven_guis binaries and reached by no code path there. "The engine `tracing` features stay off" (two sites, with §8) now reads of a package-alone build: a workspace-wide build turns `blitz-dom/tracing` on, so a test child with the sink at `info` also receives engine events. Not measured: the two binaries' stderr by level under that build.
- §3: the driver has a `tracing` dependency (was none) and its executor opens the span (was "print or log nothing"). The census of stand checks that install no subscriber has one exception: `stand_act_spans`'s two re-run children, the sink over an in-memory capture. Counts: 15 `common` readers (was 14), ten `stand_act_*` (was nine), fifteen of sixteen session-holding checks read `session_common` (was fourteen of fifteen).
- §6 Logged events: a new group for the driver's one closed-span line per call.
- Citations re-pointed.
**Why:** the chunk delivered the route entry "Driver command spans".
**Kept:** "nothing in the crate prints, logs or fields any of the four" and the same of the record of ids — both still true.
**Ref:** .andromeda/runs/2026-10-07T22-32-46-wrap/

## 2026-10-07-driver-command-spans — the closed span's line, PROVISIONAL; a span's fields under the scrub; typed text read in process
**Section:** §3 → Logging stack (the layer) · §6 → Log format · §8 → Values logged as-is, Scrubbing
**Change:**
- §6 Log format owns the mark: one line per printed event and, PROVISIONAL, one per closed span whose target the allowlist admits; no line when a span is created, recorded to, entered or exited. Shape: `{time} {LEVEL} {target} service.name=… service.version=… span={the span's name}`, the span's own fields in the order recorded, then `message`, `time.busy`, `time.idle`; level and target the span's; the timings `tracing-subscriber`'s own; `span` a reserved field name; a string Debug-quoted, a number or bool bare. An event's line is unchanged, inside a span included.
- §8 Scrubbing: a closed span's every pair is judged by the same rule as an event's field — escher target: every pair unless content-named; engine target: only the safe fields, `span` and the three close fields among the redacted; any other target: no byte at any point. A content-named span field's value is never stored; every other is held in process memory until the span closes. The five scrub sets and the rule are unchanged.
- §3: the layer is stated once in `sink_layer` — `with_span_events(FmtSpan::CLOSE)`, field formatter `SpanFields`.
- §8 Values logged as-is: was "Typed text is NOT measured … where no sink is installed"; now measured in process only — 0 of 36 id needles, 0 of 8 name needles, 0 occurrences of the three supplied texts in a sink capture at `info`, both layout modes — and still NOT measured in a host's log. The session host's by-level reading re-measured unchanged (0 · 1 · 1 · 1).
**Why:** a new record class in the sink is a boundary widening answered by the operator at the plan forks, not by the founder. The operator kept the mark at this wrap's invocation; it waits for the founder's batch at the Epoch 4 boundary, and no later wrap discharges it on its own judgement.
**Ref:** .andromeda/runs/2026-10-07T22-32-46-wrap/

## 2026-10-09-founder-rulings — the closed span's line ratified
**Section:** §6 → Log format · §8 → Scrubbing
**Change:**
- §6 Log format: the closed span's line was "PROVISIONAL: a boundary widening on the operator's answer … not yet the founder's word", per the entry `2026-10-07-driver-command-spans — the closed span's line, PROVISIONAL; a span's fields under the scrub; typed text read in process`; now "a second record class since 2026-10-07-driver-command-spans, a boundary widening ratified by the founder (2026-10-09) as built". §6 no longer owns a mark; it owns the statement the other plans point at.
- §8 Scrubbing: the command span's clause read "PROVISIONAL, as §6 → Log format marks the closed span's line"; now "its closed-span line ratified by the founder (2026-10-09), as §6 → Log format states".
**Why:** the founder ruled on it at the Epoch 4 boundary (the founder, 2026-10-09, relayed verbatim by the overseer as his chosen option label; the English wording of the ruling is the overseer's), which supersedes the operator's provisional answer by rule. With it he ruled that the three sizes of a diff may reach a log while its content never does — the command span's `added` · `removed` · `changed` fields stand on that.
**Ref:** .andromeda/runs/2026-10-09T19-17-00-wrap/

## 2026-10-09-audit-corrections-agent-surfaces — the package install's output stays in the step log
**Section:** §9 CI Integration → Telemetry artifact handling
**Change:** a new sentence under the artifact table: the package install's output stays in the job's step log — ci.yml's install steps call `.github/scripts/apt-install.sh`, which no `ci-leg.sh` leg runs, so no `target/ci-logs/` file holds its output and no artifact uploads it; the script's own diagnostic is one stderr line per failed attempt (the attempt number, the phase, the exit status), nothing of a user's. No row of the table changed: failure logs still come from `target/ci-logs/` alone and the coverage report is still the one fork-CI artifact outside it.
**Why:** the chunk put a script with its own output into nine CI jobs, and §9 is where a reader asks whether that output is kept. The detector found every claim standing; the sentence was raised from the plan's reviewed list, which asked for the fact to be recorded.
**Kept:** no upload step or upload path was added to keep the install's output — a new artifact out of fork CI is a boundary widening that waits for the founder.
**Ref:** .andromeda/runs/2026-10-09T20-50-14-wrap/

## 2026-10-10-driver-cli — the session host writes the command span; the command line writes answers, not logs
**Section:** §2 Telemetry Strategy → Telemetry mechanism · Feature wiring (By invocation) · §3 Observability Harness Contract → Logging stack (the `escher-session` bullet · the headless-stand bullet) · §4 Span / Trace Coverage → `packages/escher-driver` · §6 Log Coverage → Logged events (examples · escher-driver)
**Change:**
- §2: was "its command span … is reached by no code path in either"; now reached in `escher-session` by every call its host role runs, and still by none in `seven_guis_native`. By invocation: was "Not measured: the two seven_guis binaries' stderr by level as a workspace build makes them"; now measured for the session host — one engine line, `blitz_dom::document` at INFO, from `info` up, none for the host built alone; the windowed binary's stays not measured.
- §3, `escher-session`: was "nothing on stdout on any path"; now `main` hands argv to `escher_driver::command_line`, which installs no subscriber, reads no environment variable and writes each command's answer, refusal or error object as one line of JSON on stdout — an answer, never a log line; the sink still writes to stderr only.
- §3, the headless stand: the driver crate's "no print" becomes no print but its command line's; an `Outcome` is written in one place, by `Outcome::to_json`; the record holds no id longer than `MAX_ID_BYTES`; eleven `stand_act_*` and sixteen of seventeen session-holding checks read the shared module; `stand_session_lifecycle` re-runs two children; three more seven_guis targets spawn the sink-installing binary, five sharing the module.
- §4: the span is also written by a hosted call — 3 lines for 3 calls at `trace`; a session-level verb records its table word beside `unknown-verb`; a call refused on the command line or in the client opens no span. Observed absent gains the wire's `call`, the client's `call`, `command_line`, the `start` spawn and the idle expiry, which writes no record.
- §6: `escher-session`'s own lines are the command line's four endings and one `eprintln!`, was three `eprintln!`s and exits 0 · 1 · 2; the span's emitters gain the session host; six verb words becomes nine.
**Why:** what a command prints is its answer under the ratified crossing (the founder, 2026-10-10, relayed by the overseer), which is why none of it is a log field. Standing rule: no span field and no log field was added, and the next surface adds none that holds what a call supplied or a screen reads.
**Kept:** the sink-install citation stays at its three lines — the sweep proved them unmoved.
**Ref:** .andromeda/runs/2026-10-10T10-03-00-wrap/

## 2026-10-10-driver-cli — typed text measured in a host's log; the session host by level under both builds
**Section:** §8 PII Scrubbing & Compliance → Values logged as-is (the "Past escher's scrub" bullet) · §9 CI Integration → Telemetry artifact handling (the Session state directory row)
**Change:**
- §8: was "Typed text is measured in process only" and "Typed text in a HOST's log is still NOT measured: no command reaches the instance of a sink-installing host"; now measured — a hosted `type` reaches the `escher-session` host's instance, and its stderr at `RUST_LOG=trace` holds 0 occurrences of the typed sentinel, 0 id needles and 0 name needles under both builds. By level, three commands on CRUD: the host built alone writes 0 lines unset and 4 at `info`, `debug` and `trace` (three command spans and the install line, 742 B); the workspace-built host 5, one `blitz_dom::document` INFO line more (860 B); 0 of 15 ids, 0 of 6 names and 0 sentinel occurrences at every level; stdout 0 B in all eight runs. Not measured there: a client's own stderr by level, the windowed binary, any other flow.
- §8: "the six verbs" of the in-process capture reads the six instance-level verbs.
- §9: the checks' session state directories are `ss-life`, `ss-idle`, `ss-quiet`, `hb-*`, `hl-trace`, `cc-*`, `cf-*`, `ht-timer`, and one is empty after `stop` or after the host's idle expiry.
**Why:** the proof the sink fix has owed since 2026-10-07 — a typed sentinel read absent from a sink-installing host's log — became possible when a command first typed into that host's instance. The earlier in-process reading and the host's no-call reading stand as history beside it. The hypothesis that the workspace-built host prints engine lines at `trace` reads false as stated: one engine line, at INFO.
**Ref:** .andromeda/runs/2026-10-10T10-03-00-wrap/

## 2026-10-10-scrolling-box-bounds-and-hit — three engine checks join the no-subscriber census
**Section:** §3 Observability Harness Contract → Logging stack (the headless-stand bullet)
**Change:** the census of checks that install no subscriber gains the engine's three scrolled-box checks — `scrolled_box_client_rect`, `hit_clipped_at_scrolling_box` and `scrolled_box_absolute_position` — none of which reads an env var or declares a shared module. No count in the bullet moves: 15 readers of `common/mod.rs`, five `stand_session_*`, eleven `stand_act_*`, sixteen of the seventeen session-holding checks.
**Why:** three new check files landed and the bullet names its members. The chunk added no log site: the engine's added lines hold no `tracing` call, print or panic, and the command span keeps its eight fields.
**Ref:** .andromeda/runs/2026-10-10T15-29-49-wrap/

## 2026-10-10-snapshot-states — the no-subscriber census: twelve driver-action checks, one more engine check, 17 readers
**Section:** §3 Observability Harness Contract → Logging stack (the headless-stand bullet)
**Change:**
- Was "the eleven driver-action checks `stand_act_*`"; now twelve, `stand_act_states` the twelfth.
- The census gains the engine's state check `accessibility_states`: no env read, no print, no shared module.
- Was "the module sixteen of the seventeen checks that hold a session read"; now seventeen of the eighteen.
- Was "15 of which read the shared module `tests/blitz-tests/tests/common/mod.rs`"; now 17 — the count was wrong before this chunk, which added no reader of it.
- The two restatements of what an answer and an `Outcome`'s `Debug` hold gain the three state readings, as closed words and booleans; neither is a log line and nothing logs or fields one.
**Why:** two check files were added, neither installing a subscriber; the 15 was disproved by a count at the chunk's base and now. No log site was added, the command span keeps its eight fields, and the host-log figures did not move.
**Ref:** .andromeda/runs/2026-10-10T17-47-57-wrap/

## 2026-10-10-mcp-surface — the `tool_call` span, ratified; the `mcp` role on the sink
**Section:** §2 Telemetry Strategy (the mechanism bullet) · §3 Observability Harness Contract → Logging stack (the `escher-session` and headless-stand bullets) · §4 Span / Trace Coverage (escher-driver) · §6 Log Coverage (examples · escher-driver) · §8 PII Scrubbing → Scrubbing
**Change:**
- §4: a second span, `tool_call` — `info_span!(target: "escher_driver", "tool_call", tool, ending)`, one per MCP `tools/call`, entered before its params are read and held until its reply is written; `tool` the table's word, `ending` `accepted` · `refused` · `error`; `command` its child by nesting, the executor unchanged; session verbs leave a `tool_call` with no child. Was "the span is `run`'s" alone; `serve_mcp` outside a tool call writes no record.
- §2: was "one span macro", now two; the `mcp` role reaches both. §8: was "`escher_driver` now has an emitter", now two, neither of `tool_call`'s fields content-named.
- §6: the `tool_call` closed-span line and its emitters; the role's 9 / 10 lines over one CRUD flow at `info` (5 `tool_call`, 3 `command`); the role's endings and usage.
- §3: the `mcp` role (protocol lines on stdout, the sink stderr-only, still two install sites); `serve_mcp`'s replies a further stdout write, never a log line; `stand_mcp_tools`' in-process recorder a second exception to "installs no subscriber"; was "three more seven_guis targets … the five share", now two more, the seven share, and `mcp_log` reads the role at `trace` — `host_log` no longer the one such check.
**Why:** a boundary widening (a new log field) ratified by the founder (2026-10-10, by question dialog in the overseer session, relayed by the overseer; his confirm at this wrap's Validate, relayed by the overseer, as the operator's answer) — recorded ratified. Both fields are closed words; no content field was added.
**Ref:** .andromeda/runs/2026-10-11T08-35-17-wrap/

## 2026-10-12-self-description — `help` in the two spans' closed sets; the census; readers eighteen
**Section:** §3 Observability Harness Contract → Logging stack (the headless-stand and escher-driver bullets) · §4 Span / Trace Coverage (escher-driver, both spans) · §6 Log Coverage (examples: `escher-session`'s own lines · escher-driver's two line classes)
**Change:**
- §4: `help` joins `tool_call`'s `tool` and `command`'s `verb` closed sets; no field is added (eight and two). A `help` call answered by the server leaves a `tool_call`, ending `accepted`, with no `command` child; an argument to it is refused `malformed`. A `help` handed to `Session::run` is refused `unknown-verb` with its table word recorded. The command line's `help` ending and `describe()` write no record.
- §6: the `command` line was "the verb table's nine words", now ten. A `help` call's `tool_call` line has no `command` beside it. `escher-session`'s endings gain `help` (0 · 1 · 2), its session-error object gains a fixed `remedy`, and its usage line lists `help`.
- §3: was "seventeen of the eighteen checks that hold a session", now eighteen readers of `session_common`, `stand_mcp_tools` among them. `stand_mcp_masked` installs no subscriber, reads no env var and declares no shared module. The module `describe` installs no subscriber, reads no env var and prints nothing; its clock is not stated by the report.
**Why:** no log field was added and no new text reaches a log — the self-description and remedies are an answer on stdout, never a log line; the readers' count was disproved by measurement.
**Ref:** .andromeda/runs/2026-10-12T08-55-10-wrap/

## 2026-10-12-headless-screenshot — `screenshot` in the spans' closed sets, nine causes, no image in a log
**Section:** §3 Observability Harness Contract → Logging stack (the headless-stand bullet) · §4 Span / Trace Coverage (escher-driver, both spans) · §6 Log Coverage (escher-driver's two line classes) · §8 PII Scrubbing → Values logged as-is
**Change:**
- §4 `command`: was one of eight cause names, six returned by the executor; now nine, seven — `fonts-unavailable`, before anything is painted. An accepted `screenshot`, whole or by `id`, records `verb` alone; a refused one `verb` and `cause` (an empty crop `off-screen`); never fielded gains the image, its byte size and its `width` / `height`. Still observed absent gains `Harness::screenshot` / `screenshot_rect` and the `image` encoder. `tool_call`: was one of ten words, now eleven, `screenshot` among them; a shot's call ends `accepted` (image then text) or `refused` and parents its `command`; no field added.
- §6: the `command` line was the table's ten words, six instance-level, eight cause names; now eleven, seven, nine; a shot's line is `verb="screenshot"` (with `cause` when refused) and holds no image byte — 0 of a shot's first 64 base64 characters on the host's and the `mcp` role's stderr at `trace`, both builds. The `tool_call` line's `tool`: one of eleven.
- §3: `Session::run` gains its seventh verb; the census gains the module `image` (no subscriber, env var, print, panic or log), `Outcome::Shot` written by `to_json` and the MCP image block only, the harness's screenshot, `stand_act_screenshot` (thirteen `stand_act_*`), `stand_screenshot_repeat` — a second re-executed child whose stdout the parent reads into assertions only — and `password_painted_masked`; was eighteen `session_common` readers, now twenty.
- §8: `stand_act_spans`' child drove the six verbs of that day; seven now, and a refused shot's cause; a shot's image leaves only in its answer — 0 image bytes on stderr, both builds.
**Why:** the shot adds no log field and writes no image to a log; its verb and cause are closed words of the ratified sets.
**Ref:** .andromeda/runs/2026-10-12T20-34-17-wrap/

## 2026-10-13-upstream-sync-polish-and-ship — the merge's five targets join the no-subscriber census; the founder's word respelled
**Section:** §3 Observability Harness Contract → Logging stack (subscriber installation) · §4 Span / Trace Coverage (the `tool_call` span)
**Change:**
- §3: the census gains the five upstream integration targets of the `434e0661` merge — `anonymous_block_restyle`, `button_alignment`, `checked_attribute`, `pointer_leave`, `ua_stylesheet_order`. Each installs no subscriber, reads no env var and prints nothing; none installs escher's sink; `host_log` and `mcp_log` pass under both builds after the merge.
- §4: the `tool_call` span's ruling now reads "relayed verbatim by the overseer as his chosen option label; the English wording is the overseer's" beside «Span с tool и ending», the form of the entry it copies.
**Why:** the merge added no log site, span, env read or print (the fixed-sha census reads 0), so the census grows by five rows and nothing else moves. The reconcile was confirmed by the founder (2026-10-13, by question dialog in the overseer session, relayed verbatim by the overseer as his chosen option label «Подтвердить (Рекомендую)»; the English wording is the overseer's); the respelling was folded at the cascade sweep under playbook rule A, which he added at the same card.
**Kept:** §6's sample closed-span line still reads `service.version=0.3.0-beta.2` — a capture dated to 2026-10-07-driver-command-spans, true of that run.
**Ref:** .andromeda/runs/2026-10-13T15-50-37-wrap/

## 2026-10-13-stand-contrast-harness — the contrast reader and `stand_contrast` silent
**Section:** §3 Observability Harness Contract → Logging stack (subscriber installation) · §4 Span / Trace Coverage
**Change:**
- §3: the headless stand's `stand_*` checks reading the shared module are 18 (was 17), `stand_contrast` the eighteenth; the census names `stand_contrast` — no subscriber, no env read, no print, no listener, its readings into assertions only; the silent harness listing adds `Harness::text_contrast` (with `contrast_ratio` and `TextContrast`), which prints, panics and logs nothing and reads and changes nothing in the document.
- §4: "Still observed absent" adds blitz-test-harness's contrast reader — `Harness::text_contrast` and the root re-exports, the private sixth module `contrast` — which opens no span, emits no event, reads no env var and mutates nothing; the crate still takes no `tracing`.
**Why:** a test-harness reader and a stand check, no hot path; no instrumentation is owed, and the record of what is uninstrumented names them. The "17 of which" wording carried the readers' count without the `mod common;` token.
**Ref:** .andromeda/runs/2026-10-13T18-00-36-wrap/

## 2026-10-13-range-input-interaction — `range_input_interaction` joins the silent census
**Section:** §3 Observability Harness Contract → Logging stack (the census of checks that install no subscriber)
**Change:** the census names the engine check `range_input_interaction`, which drives `blitz_test_harness::Harness` on its own parsed pages in both layout modes. It installs no subscriber, reads no env var and prints nothing; its readings go into assertions only. The bullet's measured-at list cites this chunk's report. The rewritten `stand_act_range` stays one of the named `stand_act_*` checks.
**Why:** a new check was added. The chunk adds no span, event, field or log site; the range value leaves only through the ratified carriers.
**Ref:** .andromeda/runs/2026-10-13T21-11-52-wrap/

## 2026-10-13-stand-keyboard-harness — `stand_keyboard` joins the silent census
**Section:** §3 Observability Harness Contract → Logging stack (subscriber installation)
**Change:**
- The stand checks reading the shared module are 19 (was 18), `stand_keyboard` the nineteenth.
- `stand_keyboard` joins the silent census: it drives the four lean tasks and its own fixtures in process in both layout modes, installs no subscriber, reads no env var, prints nothing and binds no listener, its readings going into assertions only. The silence probe reads 0 over it and over the shared module whose `tab_order` and `focused` it reads.
**Why:** this chunk's new check and its count. The shared module's probe was run at this wrap, the report's probe having read the check file alone.
**Ref:** .andromeda/runs/2026-10-13T23-23-45-wrap/

## 2026-10-14-stand-a11y-assertions — the silence census over the extended checks
**Section:** §3 Observability Harness Contract → Logging stack (the headless-stand census)
**Change:** `stand_contrast` and `stand_keyboard` with their new tests, and `stand_accessibility_ids` (its three tests reading the tree through `&dyn blitz_dom::Document`), each still install no subscriber, read no env var, print nothing and bind no listener; the silence probe reads 0 over the three files and the shared module. The shared module's reader count stays 19.
**Why:** the chunk added tests to three checks and none installs a sink or prints; `stand_accessibility_ids` was not named in the census before.
**Ref:** .andromeda/runs/2026-10-14T19-50-50-wrap/

## 2026-10-14-stand-requirement-sweep — the census over three new files; the workspace host's engine line printed twice
**Section:** §3 Logging stack (the headless-stand census bullet) · §8 Values logged as-is · §6 Logged events (the `tool_call` bullet) · §2 Feature wiring
**Change:**
- §3: `common/mod.rs` readers 19 → 21 and `session_common/mod.rs` 20 → 21; `stand_requirement_sweep`, `stand_non_lean_tasks` and `password_caret_click` install no subscriber, read no env var and print nothing, the silence census reading 0 over them and the four extended files.
- §8, §6, §2: re-read on the same flows — the workspace-built host writes 6 lines / 978 B, the `blitz_dom::document` `focussed node` INFO line printing twice (was 5 / 860 B, once); the `mcp` role 11 lines workspace-built (was 10); built alone unchanged (4 / 742 B; 9); needles 0 at `info` and `trace` under both builds. The dated readings stand as dated, the re-reads appended.
**Why:** measured at this chunk. No log site, field or span added — the second line is the engine's existing event firing once more; why it fires twice is recorded, not established. The C16 CARRY asked for the re-read.
**Ref:** .andromeda/runs/2026-10-14T22-00-51-wrap/

## 2026-10-14-quality-gates — `busy` on a real line, the stderr readings, the `repair` phase
**Section:** §2 Telemetry Strategy → Feature wiring · §4 Span / Trace Coverage (`command`) · §8 PII Scrubbing & Compliance · §9 CI Integration (package install)
**Change:**
- §4: `busy` is driven. `stand_act_spans` runs `press tab` on a fixture that re-keys an element every render, in both layout modes. The line carries `settled=false` and `busy="render"`, no `passes` and no `cause` — six of the span's eight fields. Was "not driven … covered by its unit test only".
- §8 and §2: a client's own stderr, the host beyond three CRUD commands, and the workspace-built windowed `seven_guis_native` on the host's display were read by build and level, 28 runs, 0 needles. Was "not measured there".
- §9: the install diagnostic's phase is `update`, `install` or `repair` — the bounded `dpkg --configure -a` before each retried attempt. The line's shape is unchanged (was "`update` or `install`").
**Why:** measured at this chunk; no scrub, redaction, log field or span changed.
**Ref:** .andromeda/runs/2026-10-15T06-51-44-wrap/

## 2026-10-15-cold-agent-test — the cold-agent pipe log on the escher server
**Section:** §3 Cold-agent pipe log · §6 Log format (the cold-agent pipe) · §8 Scrubbing · §9 Telemetry artifact handling (Cold-agent pipe state)
**Change:**
- §3: `run stand` (was `run counter`); the per-run `mcp.json` spawns `escher-session mcp` with `RUST_LOG=info` on that child only, so the child installs escher's own sink at `info` and its `tool_call` / `command` span lines land in `target/cold-agent/server.log`, which the pipe only counts by span name; two replay children write stderr to `/dev/null`. The pipe itself installs no subscriber and reads no env var. The stub and its call log are retired; state files `{verdict.json, events.jsonl, transcript.jsonl, client.log, server.log, check.json}` (was `calls.jsonl`, `stub-state.json` in place of the last two).
- §6: the verdict fields add `server_tool_calls refused_calls error_calls spacer_calls tasks_met check_steps check_expects check_passed check_can_fail` and retire `final_count`; every field a count, a boolean or a closed word; `counts_agree` ties the span lines to the transcript; reasons add `no-check`, `check-failed`, `check-cannot-fail`. The stub's call-log bullet becomes the server log (`span_counts`: ` span="tool_call" ` and its ` ending="refused" ` / ` ending="error" `). `server.log` and `check.json` join the never-printed files.
- §8: no transcript text, tool argument, tool answer, reply or check enters an event or the verdict (four planted markers); `server.log` is escher's redacted sink output, never printed; `check.json` holds stand fixture ids and texts, never printed; `transcript.jsonl` now carries the stand's screen (ids, names, values), raw by design, never printed; each committed copy masks the session dir and the home prefix.
- §9: the state row names the six files; the session dir holds `mcp.json` alone.
**Why:** the pipe's swap to the driver's own MCP surface; the transcript's new content is the boundary widening the founder ratified — his «Один запуск, до $3 (Рекомендую)» (2026-10-15, relayed verbatim by the overseer as his chosen option label), confirmed by his «Подтвердить (Рекомендую)» at this wrap's answer card. The pipe is a new reader of the ratified `tool_call` span line, not a new emitter: no span field added.
**Ref:** .andromeda/runs/2026-10-15T09-44-57-wrap/

## 2026-10-17-upstream-sync-engine-debts — the coverage report's lcov excludes `keyboard_compat.rs`
**Section:** §9 (the Coverage report row)
**Change:** the row's lcov file — line counts of the workspace's public source, no user data — excludes `keyboard_compat.rs` by filename, the exclusion test-plan §10 records.
**Why:** kept bound to test-plan's coverage leg, which takes the exclusion on the operator's answer to P4 fork 3.
**Ref:** .andromeda/runs/2026-10-17T15-18-19-wrap/
## 2026-10-18-engine-debts — the silence census gains the engine-debts checks
**Section:** §3 Observability Harness Contract (Logging stack, the headless stand bullet, with the group `logs`) · the subject line
**Change:**
- A new group: the five new checks (`accessibility_merged_states`, `inert_subtree`, `accessibility_active_descendant_unread`, `transformed_client_rect`, `stand_act_transformed`) and the five rewritten (`stand_keyboard`, `accessibility_states`, `scrolled_box_absolute_position`, `hit_clipped_at_scrolling_box`, `password_caret_click`) install no subscriber, read no env var and print nothing, the silence probe 0 over them and `common/mod.rs`; the seven engine changes and the reworded driver texts add no log site, span, field, log field or env read.
- Driver-action checks: was thirteen; now fourteen, `stand_act_transformed`. `session_common` readers: was twenty-one; now twenty-two.
- The master's first subject line, above the Decisions Log heading.
**Why:** the chunk's checks, measured silent; no instrumentation owed — the changes are engine reads and fixed texts with no hot-path operation.
**Ref:** .andromeda/runs/2026-10-18T04-30-47-wrap/
## 2026-10-18-one-virtual-clock — §3, §4, §6: eight causes, an accepted `advance`, the clock checks silent
**Section:** §3 (the silent census; the module the session checks read) · §4 (the command span; the `tool_call` span) · §6 (escher-driver's command-span line)
**Change:**
- §4 command span: `cause` was one of nine names, the executor returning seven; now one of eight, `time-unavailable` retired, the executor returning six; an accepted `advance` — accepted on every session — records `verb` and the acting fields like every acting verb, read by `stand_act_spans`' CRUD `advance` row. The span keeps its eight fields; no time, clock or `advanced_ms` is fielded.
- §4 `tool_call`: an `advance` tool call ends `accepted` and parents one `command` child whose `verb` is `advance` and which records no `cause`, read by `stand_mcp_tools`. The span keeps its two fields.
- §6: the command-span line's closed values read eight cause names (was nine).
- §3: the module twenty-three checks read (was twenty-two), `stand_virtual_clock` among them; a new census bullet: `virtual_clock_engine` and `stand_virtual_clock` install no subscriber, read no env var and print nothing, and the document's clock and `Harness::advance_time` add no log site, span, field, log field or env read.
The delegate-answered facts PROVISIONAL.
**Why:** the founder's forks F2 A and F2b A, answered by the operator as the founder's delegate (delegate the operator, 2026-10-18), PROVISIONAL, for the founder's epoch-boundary batch (the operator, 2026-10-18, at the wrap resume); the silence probe was read at this wrap.
**Ref:** .andromeda/runs/2026-10-18T08-45-11-wrap/
## 2026-10-18-engine-debts — the PROVISIONAL batch ratified by the founder (Epoch 2 close): one-virtual-clock, 4 marks
**Section:** §4 Span / Trace Coverage (the `command` span's causes, an accepted `advance` and its `tool_call`) · §6 Log Coverage (the span line's eight cause names)
**Change:** each PROVISIONAL mark now reads `ratified by the founder (2026-10-18)` (inside parentheses `ratified by the founder, 2026-10-18`), the clause unchanged: the eight cause names, `fonts-unavailable` among them; an `advance` accepted on every session, recording `verb` and the acting fields; an `advance` tool call ending `accepted` with one `command` child; the line's eight cause names. Partly retires the PROVISIONAL clause of the earlier 2026-10-18-one-virtual-clock entry here; its other claims stand. No span, field or log field moves.
**Why:** the Provisional discharge at the epoch boundary, on the founder's own word: «Утвердить всё (Рекомендую)» on one-virtual-clock F1 B · F2 A · F2b A · F3 A (the founder, 2026-10-18, by question dialog in the overseer session, relayed by the overseer with his chosen labels verbatim), discharged at this wrap on the operator's direction (the operator, 2026-10-18, at the gated wrap's resume).
**Ref:** .andromeda/runs/2026-10-18T20-46-02-wrap/
## 2026-10-19-session-recording — recording opens no span and logs nothing; the recording-directory row
**Section:** §3 Observability Harness Contract → Logging stack (and its `absence · logs` detail group) · §4 Span / Trace Coverage · §6 Log Coverage · §9 CI Integration → Telemetry artifact handling
**Change:**
- §4: the recording — `Session::recording_into`, `Session::recording_failed`, the recorder, `Check::from_bytes` and the wire's `recording-failed` reply — joins the observed-absent list; the `command` span keeps its eight fields with a recording held or not, and an MCP call after which the recording ended is answered `isError` as `RecordingFailed`, its `tool_call` span ending `error` with `tool` and `ending` alone.
- §3: was «`serde_json` reading requests and writing nothing»; now requests and kept checks; was «for the five `stand_session_*` checks» and «twenty-three checks read — four of the five»; now six, twenty-four, five of the six; was «the seven share the module»; now eight, `recording` spawning the binary on `start … --record` and `mcp --record`; the `mcp` role taken alone or with one `--record <dir>`, checked before it serves.
- §3 detail group: a bullet for the session-recording checks' silence (the probe's last line 0), the recorded `host_log` / `mcp_log` runs (0 recording needles, span counts unchanged) and the fifteenth module `recording` (no tracing site, env read or print; its kept check a file, never a log).
- §6: was wire `v2`; now `v3` with the fixed `recording-failed` reply; the `escher-session` endings gain `--record`'s usage cases, the usage line and the refused directory's status-3 endings.
- §9: a new row «Recording directory» beside the session state directory's — `0700`, one `0600` check per recorded session, call content at rest and no screen reading, local only.
**Why:** this chunk's own facts (the playbook's «Accurate this-chunk addition»); the §9 row records the at-rest widening ratified by the founder — F1 A and F2 B as his (the founder, 2026-10-19, by question dialog in the overseer session, relayed by the operator) and the card confirmed «Подтвердить все 25 (Рекомендую)» (the founder, 2026-10-19, by question dialog in the overseer session, relayed by the overseer).
**Ref:** .andromeda/runs/2026-10-19T05-20-24-wrap/
## 2026-10-19-recorded-check-replay — the replay observed silent, each kept answer at rest, the committed checks
**Section:** §3 Observability Harness Contract (Logging stack: the headless-stand readers, the session-recording silence bullet) · §4 Span / Trace Coverage (the `command` span's observed-absent list) · §9 CI Integration (Telemetry artifact handling: Recording directory, Committed checks)
**Change:**
- §3: `session_common`'s readers twenty-four → twenty-five (+`stand_replay`); the silence bullet — was «through its `replay`, `kept` and `mode`» — now `kept`, `mode` and `shot_prefix`, replaying through `Check::replay`; `stand_replay` silent (no subscriber, env read or print; failure messages the mode, the file and a step index alone); the recorder also writes the answer itself as `answer` (a shot's without `png`); the sixteenth module, `replay`, holds no tracing site, env read or print and calls `serde_json` only to read.
- §4: `Check::replay` joins the observed-absent list — each replayed step writes its own `command` span through `Session::run`, the replay adding none; `Replay` / `Change` hold bools, a step index and closed key words only.
- §9 Recording directory: was «answer digest, and no screen reading»; now the answer kept whole as `answer`, masked as on every carrier, a shot's without `png` — the screens' readings at rest, still no log or event line.
- §9 gains «Committed checks»: `tests/blitz-tests/checks/counter-{cli,mcp}.json` in git (`100644`, public), written by `record_checks.sh` (`step N` only), read by `stand_replay` and seven_guis' freshness check, no log or event line, uploaded by no CI leg.
**Why:** boundary widenings ratified by the founder — F3 B (the founder's word, relayed verbatim by the operator, 2026-10-19 05:02Z) and F1 A «Коммитить две проверки (Рекомендую)» (the founder, 2026-10-19 08:04Z, by question dialog in the overseer session, relayed by the overseer); the answer card confirmed «Подтвердить все 22 (Рекомендую)» (the founder, by question dialog in the overseer session, relayed by the overseer through the operator). No raw input reaches a log: the silence probe and `host_log` / `mcp_log` read 0. Not PROVISIONAL.
**Ref:** .andromeda/runs/2026-10-19T09-49-14-wrap/

## 2026-10-19-upstream-sync-visual-lint-and-real-app-readiness — the upstream focus checks are silent
**Section:** §3 Observability Harness Contract (Logging stack — the `absence · logs` group)
**Change:** a bullet is added: `focus_within` and `focus_redraw`, upstream's own targets, install no subscriber, read no env var and print nothing (0 · 0 · 0 each). `focus_redraw`'s `RecordingShell` is a counting `ShellProvider`, not a sink. The engine changes they pin (`:focus-within` matching, the not-in-document focus refusal, the focus-move redraw; PROVISIONAL) add no log site, span, field or env read. The `Focussed node` INFO line stands after the new gate, and `host_log` and `mcp_log` pass under both builds.
**Why:** the silence census records every new check of the tree (the playbook's «Accurate this-chunk addition»). No raw input reaches a log. The engine take is PROVISIONAL on the operator's delegate answer to F1 (2026-10-19).
**Ref:** .andromeda/runs/2026-10-19T19-54-24-wrap/

## 2026-10-19-test-splits — the log checks' shared fragment and its silence census
**Section:** §3 Observability Harness Contract → Logging stack (`absence · logs` group)
**Change:** a census item added: escher-driver's `cli_tests.rs`, `recording_tests.rs`, `recording_off_unix_tests.rs` and seven_guis' shared `tests/common/mod.rs` are silent — 0 print, tracing and subscriber sites over the four; the shared module now states once, for `host_log` and `mcp_log` alone, `Needles`, `stand_needles`, `stamped_lines`, `found`, `occurrences` and the unix-only `recorded_needles`, its assertion messages a kind and a count, never a needle or a captured line; escher-driver still holds its 2 `info_span!` sites, the env-read set unchanged.
**Why:** this chunk's own test-only move (the playbook's «Accurate this-chunk addition», the plan's reviewed expected amendment); the log checks the security and obs readings rest on still assert the same zeros.
**Ref:** .andromeda/runs/2026-10-19T21-24-54-wrap/

## 2026-10-19-shared-fixtures — the fixtures module in the silence census
**Section:** §3 Observability Harness Contract → Logging stack (`absence · logs` group)
**Change:** the stand checks' third shared module, `tests/blitz-tests/tests/fixtures/mod.rs` (the fixture boot and hold, three pages and the held network stand-in `HeldNet`), is silent: no print, `tracing` call, subscriber, env read, file, socket, listener or `blitz_net`, the census reading 0. Its 19 readers, now including `stand_mcp_masked`, `stand_boot` and `font_face_sniffed_source`, hold the 4 print or sink sites of `stand_act_spans`' re-run children, none added, and the blitz-tests env-read set is unchanged.
**Why:** this chunk's own test-only module: the playbook's «Accurate this-chunk addition» and the plan's reviewed expected amendment. The «their shared modules» clause of the stand line already covers all three modules, so it is left as written.
**Ref:** .andromeda/runs/2026-10-19T23-02-04-wrap/

## 2026-10-19-visual-lint — the visual lint recorded silent and spanless
**Section:** §3 Observability Harness Contract (Logging stack, the absence · logs group) · §4 Span / Trace Coverage
**Change:**
- §3: a new absence bullet — the visual lint is silent: escher-driver's `lint`, blitz-dom's `text_extent` and the checks `stand_lint` and `text_extent` hold no print, log site, subscriber, sink, env read, file or socket, the silence census reading 0 over the four files; `Finding` and `Reason` derive `Debug` and hold ids, closed words and numbers only, and nothing logs, fields or prints one. The `session_common` reader count was twenty-five, now twenty-six (`stand_lint`); the `fixtures` module's readers were 19, now 20.
- §4: the `command` span's observed-absent list gains the in-process `Session::lint` (no verb, never handed to `Session::run`), `BaseDocument::text_extent` and `Harness::text_contrast` as the lint's contrast reader — no span, event or field (PROVISIONAL: no new field); escher-driver keeps its 2 `info_span!` sites.
**Why:** this chunk's own silent modules and checks (the playbook's «Accurate this-chunk addition»). The no-new-field rule rests on the operator's P4 answer as delegate (the operator, 2026-10-20; delegate the operator), recorded on his P2 answer as delegate (the operator, 2026-10-20, by question dialog; delegate the operator) — PROVISIONAL for the founder's own word, listed for his Epoch 4 boundary batch.
**Ref:** .andromeda/runs/2026-10-20T00-54-29-wrap/

## 2026-10-20-load-wait — the load wait reads a clock and opens no span
**Section:** §3 Observability Harness Contract (Logging stack · detail: absence · logs) · §4 Span / Trace Coverage (escher-driver command span)
**Change:**
- §3: the driver crate's clock readers were its `host` and its MCP server, now also its `session`, for the load wait and nothing else, `LOAD_WAIT` bounding it; `session_common` readers were twenty-six, now twenty-seven (`stand_load_wait`); the `fixtures` module's readers were 20, now 21; a new silence bullet — `stand_load_wait` and its in-file `LateNet` stand-in print nothing, install no subscriber, read no env var and use no `blitz_net`, and the load wait adds no log site, span, field or env read, escher-driver keeping 2 `info_span!` sites and 0 events.
- §4: the `command` span's `passes` on a step that waited on a load is the settle that went quiet; the observed-absent list gains the driver's load wait (`settle_waiting`) — no span, event or field of its own, inside the `command` span, its eight fields unchanged.
**Why:** this chunk's own wait and check (the playbook's «Accurate this-chunk addition», the plan's reviewed expected amendments); the twenty-six site was one the report's digit-keyed sweep missed.
**Ref:** .andromeda/runs/2026-10-20T03-00-02-wrap/

## 2026-10-20-select-by-id — the select checks in the silence census, and the PROVISIONAL discharge
**Section:** §3 Observability Harness Contract · §4 Span / Trace Coverage
**Change:**
- §3 Logging stack: a new group bullet — `select_interaction` and `stand_act_select` are silent (no subscriber, env read or print; the census probe 0), the select model adds no log site, span, field, log field or env read and reads no wall clock, escher-driver keeping its 2 `info_span!` sites. The stand census names the fifteenth driver-action check `stand_act_select` and the engine check `select_interaction`; the `session_common` list reads twenty-eight checks (was twenty-seven) and fifteen `stand_act_*`; the fixtures module's readers 21 → 23.
- §3: the take's «(PROVISIONAL)» dropped from the focus checks' bullet. §4: the lint's «PROVISIONAL: no new field» dropped — no new field, as built.
**Why:** this chunk's silent checks (the plan's expected amendment); «twenty-seven» was a word form the report's sweep missed. The marks drop on the founder's own word, relayed verbatim by the overseer (2026-10-20, chunk inputs#I3): the take «Утвердить (Рекомендую)», the visual lint F2 «Без поля (Рекомендую)» — on the operator's «Drop all now» at this wrap's halt.
**Ref:** .andromeda/runs/2026-10-20T05-49-12-wrap/

## 2026-10-19-visual-lint — the gated part: the `lint` verb inside the command span, no field
**Section:** §3 Observability Harness Contract · §4 Span / Trace Coverage · §6 Log Coverage → Logged events · §8 PII Scrubbing & Compliance
**Change:**
- §3: `Session::run` holds an eighth instance verb, `lint`. `Outcome::Linted { findings }` is written by `to_json` alone as `{"findings":[…]}`, never logged or fielded.
- §4:
  - `Session::lint` is reached by the `lint` verb inside its call's `command` span and opens none of its own (was «no verb, never handed to `Session::run`»).
  - An accepted `lint` records `verb` alone; a lint's findings and their count are never fielded.
  - `tool_call`'s `tool` is one of twelve words, `lint` added. A `lint` tool call parents one `command` child, measured in process.
- §6:
  - The `command` line's words number twelve, eight instance-level (was eleven and seven). The «ten until … nine until» history is dropped.
  - A lint's line is `span="command" verb="lint"` and holds no finding; `host_log` and `mcp_log` each drive one `lint` with 0 needles.
  - `tool_call`'s words number twelve.
- §8: the verb table holds eight instance verbs with `lint`, and `stand_act_spans` reads its lines (34 calls per mode).
**Why:** F2 is the founder's own word, «Без поля (Рекомендую)» (no new span field), with F1 «Новая команда lint (Рекомендую)». Both were given by question dialog in the overseer session and relayed verbatim by the overseer (2026-10-20, chunk inputs#I7). His card confirm is inputs#I11.
**Ref:** .andromeda/runs/2026-10-20T07-57-08-wrap/

## 2026-10-20-large-screen-in-parts — a part read's span and its silence
**Section:** §3 Observability Harness Contract → Logging stack · §4 Span / Trace Coverage · §6 Log Coverage · §8 PII Scrubbing & Compliance
**Change:**
- §4: an accepted `lint` and a part read (`Outcome::Part`) record `verb` alone; a part's number and the part count are never fielded; dioxus-native-dom's `Snapshot::text_parts` opens no span, emits no event and records no field.
- §6: a part read's line is `span="command" verb="snapshot"` (with `cause` when refused) and holds no part number, count, size or text; `host_log` and `mcp_log` each drive one part read at `trace` with 0 needles.
- §8: `stand_act_spans` 35 calls per mode (was 34), a part read among them.
- §3: the parts check `stand_snapshot_parts` is silent, and `text_parts` and `Outcome::Part` are never logged (a new bullet in the `absence · logs` group). `session_common` readers twenty-nine (was twenty-eight), `fixtures` readers 24 (was 23).
**Why:** this chunk's span reading and silence census, CI green 17/17 on 191e66fdb (CI#39410865604). No field is added, so no ruling is owed.
**Ref:** .andromeda/runs/2026-10-20T11-11-53-wrap/

## 2026-10-24-u55-door — obs-plan: split into its body and 7 category files
**Section:** (the whole master — the door)
**Change:** split into its body and 7 category files; the map .andromeda/obs-plan.map.tsv
**Why:** the operator's word: the founder's standing word of 2026-10-24 for the door on escher's masters, given after two review rounds, the last four open rows taken in the reviewer's own wording; relayed by overseer1.
**Ref:** $TMPDIR/claude-1000/-home-turbolet-dev-projects-additional-andromeda-worker/9a5434fe-fa07-4859-9671-f503cd8de21b/scratchpad/u55-door-2026-10-24/obs-plan/

## 2026-10-24-upstream-sync-polish-and-ship — the ten-commit take read silent, and `multi_doc`'s printed timings
**Section:** §1 Obs Scope Summary (body: examples) · §3 Observability Harness Contract (body: the silence census) · §5 Metric Coverage (body; metric-coverage.md) · §6 Log Coverage (log-coverage.md: examples) · §8 PII Scrubbing & Compliance (pii-scrubbing-compliance.md)
**Change:**
- §1 and §5: the root example `multi_doc` prints its timings — the run's and each resolve's, read with `Instant::now()` — in one `println!` on stdout; a metric row and the examples' scope name it; §6 cites it among the examples' `println!` output.
- §3 census: the upstream take read at `f9cbb2756` is silent — `hit_test_dropped_nodes` installs no subscriber, reads no env var and prints nothing; the taken commits add no tracing site over `packages/` (one commented `dbg!`) and no wall-clock read in blitz-dom outside `clock.rs`; `PARALLEL_TRAVERSAL_LOCK`, `Node::out_of_flow`, `ElementData::classes`, the dropped-node hit skip and the replaced element's `aspect-ratio` height add no log site, span, field, log field or env read; `packages/escher-telemetry/` unchanged; `host_log` and `mcp_log` pass under both builds.
- §8: the sink allowlist re-check holds — no scrub set, redaction shape or violation schema moves; `multi_doc`'s line is example output outside escher's sink, holding no page or user content.
**Why:** each upstream sync is read for log sites and the sink re-checked; the take is the founder's own word «Десять коммитов (Рекомендую)», relayed verbatim by the overseer (2026-10-24, chunk inputs#I4). No record class, field or carrier is added.
**Ref:** .andromeda/runs/2026-10-24T12-21-29-wrap/

## 2026-10-24-stand-requirement-sweep — the sweep's checks silent, the paged part and the recording clients read
**Section:** §3 Observability Harness Contract (body: Logging stack — `stand_requirement_sweep`; the fixtures module bullet) · §6 Log Coverage (log-coverage.md: absence · logs) · §8 PII Scrubbing & Compliance (pii-scrubbing-compliance.md) · §9 CI Integration (ci-integration.md: Session state directory)
**Change:**
- `stand_requirement_sweep`'s clause names its `WITNESSES` and `WITNESSES_020` tables and its leg check, read through `include_str!`, still silent; the fixtures module holds four pages (was three), `many_items` added.
- log-coverage: one `observed absent` row — the chunk's test additions hold no log site, span, field, env read or listener, the silence census reading 0.
- pii: `stand_act_spans` 36 calls per mode (was 35), two part reads, part 2 of `many_items` with its 400 ids and its buttons' text among the needles, none on a line; the client's-stderr reading named a one-off, joined by `host_log`'s standing check of a recording session's clients at `trace`, 0 of each needle class, both builds; a paged part read through the host's own sink unmeasured, owned by a `.andromeda/residuals.md` line (target: next).
- ci-integration: the state directories gain `hl-client` and `hl-client-rec`.
**Why:** measured at this chunk (its gate entry 8 and the new tests' needle readings); `RUST_LOG` on `host_log`'s clients is the registered read. The limit's owner is the operator's own word «yes — apply.» on the phase's P5 card 2 (chunk inputs#I2).
**Ref:** .andromeda/runs/2026-10-24T17-32-19-wrap/

## 2026-10-24-a11y-re-read — the a11y leg's eight joining checks recorded silent
**Section:** §6 Log Coverage (log-coverage.md: the logs absence group)
**Change:** a new absence row: the eight checks the `a11y` leg runs beside its six earlier files (`accessibility_states`, `accessibility_label`, `accessibility_description`, `accessibility_merged_states`, `accessibility_active_descendant_unread`, `inert_subtree`, `select_interaction`, `stand_act_select`) and the two shared modules they read (`tests/common/mod.rs`, `tests/fixtures/mod.rs`) hold no print, env read, subscriber or listener — the silence census over the ten files reading 0, `packages/` unchanged.
**Why:** measured by the chunk's silence-census gate, read red with a planted `println!` and green on the tree.
**Ref:** .andromeda/runs/2026-10-24T21-49-26-wrap/

## 2026-10-25-capability-statement — the statement restates the driver's spans; a kept check at rest names a lint's findings
**Section:** §6 Log Coverage (log-coverage.md: a new `emitter-escher-driver` group) · §9 CI Integration (ci-integration.md: the recording row)
**Change:**
- Log Coverage: the published statement `docs/capabilities.md` restates the escher-driver emitter's reach — the `command` and `tool_call` spans with closed-word, bool and count fields, stderr only, no driver line at the default `warn`, and the scrub's limits; its pin reads none of these claims, so a change to the span set or the scrub's limits is made there by hand.
- CI Integration: a kept check holds the screens' ids, names, values, snapshot text, diffs and a lint's findings at rest (was no lint finding).
**Why:** the statement is this chunk's new surface and restates what this plan owns; the at-rest list follows the visual lint's crossing onto every ratified carrier, one of the eight sites the cascade sweep found, reconciled on the founder's own word «Да, и эти 8 (Рекомендую)», extending his P2-halt answer «Применить как сверку (Рекомендую)» to all ten carried-set sites, by dialog in the overseer session, relayed by the operator.
**Kept:** the capability-statement pin is not recorded silent: its proposal named no silence census over the file, so an `observed absent` row would have stood on no search.
**Ref:** .andromeda/runs/2026-10-25T11-21-44-wrap/

## 2026-10-25-cold-agent-bar — the cold-agent pipe's verdict fields, its live child's recording and the six apps' carriage
**Section:** §3 (body: the pipe's spawn line and state area) · §6 Log format (body: the cold-agent pipe's verdict and its `server.log`; log-coverage.md: the verdict row, a new `values · log-fields` group) · §8 (body: the transcript's carriage; pii-scrubbing-compliance.md: the live run's carriage) · §9 CI (ci-integration.md: the state-area row, a new Cold-agent recording row) · span-trace-coverage.md (a new `tool_call` read-by-the-pipe group)
**Change:**
- Spawn and state: the live server child is `exec env RUST_LOG=info <abs target/debug/escher-session> mcp --record <abs target/cold-agent/record>`; the two replay children run without `--record`; the state set is `target/cold-agent/{verdict.json, events.jsonl, transcript.jsonl, client.log, server.log, check.json, record/}` (was without `record/`), `record/` the live session's kept checks, owner-only (`mkdir -m 700`), never printed or copied, removed by `cleanup`.
- `server.log` is read only to count ` span="tool_call" ` lines, the refused and error endings and the accepted lines of each counted verb (`lint` · `screenshot` · `advance`) (`span_counts`), never printed or copied.
- Verdict fields add `wrong_call_bar_pct` (5), `lint_calls` · `screenshot_calls` · `advance_calls` (accepted `tool_call` lines per counted verb), `part_reads` (paged snapshot reads), `recorded_sessions` · `recording_kept` (`recording`, the live child's kept checks, counts only); `wrong_calls` now decides against the bar; reasons add `capability-unused` (a counted verb or a part read unused) · `recording-not-kept` · `wrong-calls-over-bar` (`wrong_calls × 100 > 5 × tool_calls`), after `task-unmet` and before `no-check` · `check-failed` · `check-cannot-fail`.
- §8: the transcript carries the stand's screens — on the live run of six apps also Cells' contents, the flight booker's select options and value, a lint's findings and a shot; the live session's kept checks hold its calls and answers at rest under the owner-only `record/`, never printed, copied or committed.
- CI: a Cold-agent recording row — local only, uploaded by no leg, no kept check of the live run committed (the two Counter checks the only committed ones); the `tool_call` span keeps `tool` and `ending`, none added.
**Why:** the pipe's own measured verdict shape, a closed set of counts and words read from the server's span lines (the playbook's field-inside-a-ratified-carrier rule); the six apps' carriage on the founder's own confirmation at this wrap's P2 halt, on his earlier one-run, $3 answer and his word to run it now, each relayed by the operator; the live child's recording inside the ratified session recording.
**Ref:** .andromeda/runs/2026-10-25T17-25-04-wrap/

## 2026-10-25-cold-agent-bar — the host's six slugs, the served stand's silence and the host-sink part read measured
**Section:** §3 (body: the host's `main` and the headless stand clause; the `host_log` clause; a new served-stand silence line in the absence block) · §6 (log-coverage.md: the usage line's task names, the hosted part read) · §8 PII (pii-scrubbing-compliance.md: the part read through the host's own sink) · §9 CI (ci-integration.md: `hl-cells` among the state directories) · span-trace-coverage.md (a new hosted part read group)
**Change:**
- `main` hands argv, the stand's six slugs (`counter` · `flight-booker` · `timer` · `crud` · `cells` · `load`, its private `Served` table) and the stand's boot to `command_line`, the usage line's `apps:` and the MCP `start` tool's `enum` following as the program's own fixed text (was four slugs); the stand's boots are `boot` / `boot_timer` / `boot_cells` / `boot_load` / `options`, `boot_load` the one whose options' `net_provider` is the in-process `LoadNet`, every other caller keeping `net_provider: None`.
- The served stand is silent: `boot_cells`, `boot_load` and `LoadNet` — each `fetch` answering `LOADED_CSS` from a thread of its own after `LOAD_ANSWER_DELAY` (200 ms), no `is_noop` override, a third hand-written network stand-in beside `HeldNet` and `LateNet` — hold no `blitz_net`, print, `tracing` call, env read, socket or listener, the silence census reading 0 over `stand.rs`, `flight_booker.rs`, `stand_cells_served`, `stand_load_page` and the blitz-tests shared `common` and `fixtures` modules; no driver schema, span field or log line moves.
- The paged part read through the `escher-session` host's own sink is measured (was unmeasured, the host booting the four lean tasks and no paged screen, owned by a `.andromeda/residuals.md` line): `host_log`'s fourth test (unix only) runs `serve cells` at `RUST_LOG=trace` in `hl-cells`, commits a sentinel into a part-2 cell and calls `snapshot --part 2` — 0 id and 0 name needles under both builds, its failures naming a kind and a count only (`host_log` 4, was 3); its spans are the `command` spans, a part read recording `verb` alone.
**Why:** measured at the chunk; the residual line's discharge is the operator's own word on the phase's P5 card (chunk inputs#I4). No log site, field, env read or listener added.
**Ref:** .andromeda/runs/2026-10-25T17-25-04-wrap/

## 2026-10-26-codebase-health — the complexity factoring's private items recorded silent
**Section:** §6 Log Coverage (log-coverage.md: the logs absence group, a new row after the a11y leg's joining checks)
**Change:** a new `observed absent` row: the private items that factor the lint's findings walk and the two table-asserting tests — escher-driver `lint.rs`'s `readings` and `placement`; `mcp_tests.rs`'s `#[track_caller]` `assert_described`, `assert_schema` and `assert_property`; `stand_requirement_sweep`'s `#[track_caller]` `assert_0_2_0_row` and `assert_0_2_0_witness` and its target predicate `inside_stand_repeat` — hold no print, log site, span, field, env read or subscriber, the test helpers' assertion messages naming a row and an index only. Searched: the silence census over `lint.rs`, `mcp_tests.rs` and `stand_requirement_sweep.rs` reading 0 (exit 1), `mcp_tests.rs`'s first census; `mcp_tests.rs`'s env reads the 2 compile-time `env!` alone (`CARGO_MANIFEST_DIR`, `CARGO_PKG_VERSION`); escher-driver still holding 2 `info_span!` sites, no event and no `#[instrument]`.
**Why:** measured at the chunk, which brought the lint's findings walk and the two table-asserting tests under the cognitive-complexity ceiling of 15 with private items only — no public symbol, verb, field, carrier, env var or log site added — and read the silence census 0 over the three files it changed.
**Kept:** `mcp_tests.rs`'s first census reading is recorded in this log-coverage row, not folded into the body's §3 lint census line, which still names its own four files.
**Ref:** .andromeda/runs/2026-10-27T02-06-43-wrap/

## 2026-10-27-upstream-sync-engine-debts — citations re-pointed into the code the take moved
**Section:** §1 Obs Scope Summary (body: the blitz-dom layout and node rows, blitz-shell · blitz-paint, blitz-vibey-script · blitz) · §2 Telemetry Strategy (body: Telemetry mechanism) · §3 Observability Harness Contract (body: Embedder drain of script diagnostics) · §6 Log Coverage (log-coverage.md: values · logs) · §7 Error Capture & Reporting (body: Error classes captured; Script (JS) error surface) · §8 PII Scrubbing & Compliance (body: Values logged as-is) · §9 CI Integration (body: Telemetry artifact handling)
**Change:**
- The script error surface's citations — the window `error` event, `window.onerror`, a handler's exception recorded with no further event, the labelled error sources — and §6's uncaught-JS-error log cite blitz-vibey-script's `runtime/events.rs` and `runtime/timers.rs` (was `runtime.rs`, which no longer holds that code); claim text unchanged.
- The citations into blitz-dom `mutator.rs`, `layout/construct.rs` and `node/element.rs`, blitz-shell `window.rs` and blitz-vibey-script `document.rs` and `runtime.rs` (the JS console bridge to `log`) re-pointed to the lines the cited code now stands at; claim text unchanged.
- The publish build's `--verbose --trace` citation, in §6 and §9, points to the `dx bundle` line, which now stands inside a retry loop in `publish-browser.yml`; the upstream-only and repository-guard claims unchanged.
**Why:** the take split blitz-vibey-script's runtime into `runtime/` modules and shifted lines in the other files it touched; a citation whose code left its file read false and was re-pointed by hand at Validate, the rest by the take's line map. No log site, field or reading moved.
**Ref:** .andromeda/runs/2026-10-27T10-45-32-wrap/

## 2026-10-27-upstream-sync-engine-debts — the take read at `ebacd68bd` silent, and the sink re-check restated
**Section:** §3 Observability Harness Contract (body: Logging stack — the silence census) · §6 Log Coverage (log-coverage.md: absence · logs) · §8 PII Scrubbing & Compliance (pii-scrubbing-compliance.md: values · pii, the sink allowlist re-check)
**Change:**
- §3 census: the take reading is the upstream take read at `ebacd68bd` (was the one read at `f9cbb2756`, per «2026-10-24-upstream-sync-polish-and-ship — the ten-commit take read silent, and `multi_doc`'s printed timings», its `hit_test_dropped_nodes`, commented `dbg!` and five named changes leaving the census): upstream's target `pseudo_element_flow_classification` reads 0 env · 0 print · 0 subscriber; the taken commits add no log site, print, listener or env read in source — the fixed-sha census over their `packages/*.rs` lines reading its two named hits, the uncaught-JS-error `tracing::error!` standing whole in `report_js_error`, still `#[cfg(feature = "tracing")]` at the engine target `blitz_vibey_script`, and a test-only `TMPDIR` read — and no wall-clock read in blitz-dom outside `clock.rs`; `ElementData::flush_open_state` (`:open`), `document_may_load`, the IME enable at a focused input's editor construction, the pseudo vote in flow classification and the shell's redraw route add no log site, span, field, log field or env read; `packages/escher-telemetry/` and `packages/escher-driver/` unchanged (was escher-telemetry alone named); `host_log` and `mcp_log` pass under both builds.
- §8: the re-check restated at `ebacd68bd` — no scrub set, redaction shape or violation schema moves; dioxus-native-dom takes one test alone, `input_focused_on_mount_enables_ime` (was named unchanged); the sink prints the JS-error site's safe fields and the message `[redacted]`; the `multi_doc` clause leaves this line.
- §6: one `observed absent` row — blitz-vibey-script `tests/dom.rs`'s `style_property_accessors`, `remote_document_cannot_load_file_urls` and `file_document_can_load_sibling_file_urls` hold no print or log site; the census's one hit there is `temp_dir_with_files`'s `TMPDIR` read through `std::env::temp_dir()`, test code only and not secret, a second test-only `TMPDIR` site beside escher-driver's.
**Why:** each upstream sync is read for log sites and the sink re-checked; the take is the founder's own word on the eleven commits, relayed verbatim by the overseer, with the operator's dialog answer. At Validate the census line was replaced, not given a sibling: a body holds current truth, the earlier reading's history in this sidecar; two drafted §6 rows repeating the census were dropped. No record class, field or carrier is added.
**Kept:** the earlier entry's `multi_doc` claims (§1, §5, §6) are untouched, so it is not superseded.
**Ref:** .andromeda/runs/2026-10-27T10-45-32-wrap/

## 2026-10-27-upstream-sync-engine-debts — the blitz-vibey-script slice's absences after the module split
**Section:** §5 Metric Coverage (metric-coverage.md: absence · metrics) · §8 PII Scrubbing & Compliance (pii-scrubbing-compliance.md: absence · pii) · §4 Span / Trace Coverage (span-trace-coverage.md: absence · spans)
**Change:**
- The 32-file blitz-vibey-script / blitz slice's three absences — metrics, redaction or scrubbing, spans — each note that the slice's blitz-vibey-script code stands split into the `runtime/` and `dom/element/` module trees (13 Rust modules, 9 JS bootstrap files, upstream's side moved whole), 32 the reading's own count; each absence stands.
- metric-coverage: the take moves no dependency, feature or manifest and adds no log site, span or field; pii-scrubbing-compliance: the take adds no scrub or redaction shape; span-trace-coverage: the take adds no span, so the absence stands for the moved code.
**Why:** the take split blitz-vibey-script's runtime and element bindings into new files, a move; the report records no dependency moved and no log site, span, field, scrub or redaction shape added, so each slice reading holds for the moved code under its own count.
**Ref:** .andromeda/runs/2026-10-27T10-45-32-wrap/

## 2026-10-27-upstream-sync-engine-debts — the test crate slice's absences hold `pseudo_element_flow_classification`
**Section:** §4 Span / Trace Coverage (span-trace-coverage.md: absence · spans) · §2 Telemetry Strategy (telemetry-strategy.md: absence · strategy)
**Change:**
- The 61-file test crate slice's two absences — spans or traces, telemetry or logging crates — note that the slice also holds upstream's target `pseudo_element_flow_classification`, which installs no subscriber, reads no env var and prints nothing, 61 the reading's own count; each absence stands.
- span-trace-coverage: it opens no span; telemetry-strategy: 3 tests, its `mod` line in the unbuilt `all.rs`, 0 env · 0 print · 0 subscriber and no log site.
**Why:** the take adds the target as a file of its own, read silent at the chunk's sink re-check, so the 61-file reading no longer named the whole slice.
**Ref:** .andromeda/runs/2026-10-27T10-45-32-wrap/

## 2026-10-27-upstream-sync-engine-debts — the `file:` refusal in the script error surface
**Section:** §7 Error Capture & Reporting (body: Script (JS) error surface — blitz-vibey-script)
**Change:**
- One bullet added: a `file:` load asked by a non-`file:` document is refused by `document_may_load` as a typed `FetchError::Forbidden`, with no log site of its own — `fetch()` rejects with a TypeError, a module `import` fails, and a `<script src>` is skipped and recorded as a script error, which the recorded-error path drains and logs (§6).
**Why:** the refusal is upstream's, taken on the founder's own word on the eleven commits, relayed verbatim by the overseer, which ratifies for this take an engine behaviour change for every document, the `file:` refusal among them; the fact found no home among the drafts, and Validate placed it in §7's script error surface as the error class it is. No log site, field or carrier is added.
**Ref:** .andromeda/runs/2026-10-27T10-45-32-wrap/
