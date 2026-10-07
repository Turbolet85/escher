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
