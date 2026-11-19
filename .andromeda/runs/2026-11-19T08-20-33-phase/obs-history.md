@history_item n=1 marker=2026-10-05-fork-ci-reached
**2026-10-05-fork-ci-reached** — (per-leg CI failure logs) §9 gained the per-leg log row: `ci-leg.sh` writes each leg's merged output to `target/ci-logs/{leg}.log` (matrix `target/ci-logs/matrix-{platform}.log`), truncated at the leg's start, uploaded only on failure as `ci-log-{job id}`, kept 7 days, from `target/ci-logs/` alone. This is the row `decision/ci-log-artifact-on-failure`. A sweep of `target/` or a change to a leg's invocation must keep `target/ci-logs/` intact for the leg that is running, and must keep the upload path confined there.

@history_item n=2 marker=2026-10-05-ci-gate-legs
**2026-10-05-ci-gate-legs** — §9 gained the `coverage-report` artifact, `target/coverage/lcov.info`, uploaded only on success and kept 7 days. It is the one fork-CI artifact outside `target/ci-logs/` (row `decision/coverage-artifact-on-success`). The coverage leg (`cargo llvm-cov`) is one of the invocations this chunk measures. A sweep or a change to the profile or target directory must leave the lcov path the upload reads where it is.

@history_item n=3 marker=2026-10-06-founder-rulings
**2026-10-06-founder-rulings** — (the coverage-report upload ratified) The founder ratified the `coverage-report` upload, which had been PROVISIONAL. Changing what CI uploads (`decision/fork-ci-two-artifacts`) is a boundary widening that needs the founder's word. A mend must not add an artifact, for example a size or growth report uploaded from CI.

@history_item n=4 marker=2026-10-06-telemetry-bootstrap
**2026-10-06-telemetry-bootstrap** — Set the rule this chunk's telemetry rows rest on. In the engine and upstream crates tracing is compiled only with the `tracing` cargo feature, while escher-telemetry takes tracing ungated (`decision/telemetry-via-tracing`, `decision/engine-tracing-feature-gated`, `decision/escher-crates-always-trace`). Feature unification across invocations is a hypothesized cause of the growth. A mend that aligns feature sets must not turn the engine `tracing` feature on or off by accident.

@history_item n=5 marker=2026-10-06-headless-stand
**2026-10-06-headless-stand** — Fixed that only the windowed binary installs `escher_telemetry::init` and that the headless stand and its checks install no subscriber (the origin of `decision/headless-stand-no-sink`, `decision/sink-installed-first-in-main`). If test files are merged into one binary per leg, no merged target may pick up a sink install from a telemetry check that shares its process.

@history_item n=6 marker=2026-10-06-id-persistence
**2026-10-06-id-persistence** — Recorded `stand_id_persistence`'s re-executed child, which prints ids to stdout for the parent's assertion. Trap for the "one test binary per leg" lever: re-exec children run the current test executable. If files are merged into one binary, each child's re-exec target and test filter must still resolve, or the proof breaks.

@history_item n=7 marker=2026-10-06-stand-test-contract
**2026-10-06-stand-test-contract** — Introduced the agent-run harness log: `scripts/agent-run.sh`'s `boot`/`run.start`/`test`/`run.end` lines appended to `target/agent-run/events.jsonl`, `status`/`cleanup` printed only, and raw output in `run.log`, never printed. One `test` event comes from each libtest line, and nothing inside a `failures:` block is read. These are the `contract/agent-run-events/*`, `decision/agent-run-raw-output-never-printed` and `decision/agent-run-no-captured-output` rows. The `file` field (`contract/agent-run-test-event/file`) is the test binary's stem with its hash stripped. Merging per-file binaries changes that field's value set, and a target-dir change moves `target/agent-run/`.

@history_item n=8 marker=2026-10-06-cold-agent-run-pipe
**2026-10-06-cold-agent-run-pipe** — Introduced the cold-agent pipe state under `target/cold-agent/`: local only, gitignored, uploaded by no CI leg, and removed by its own `cleanup` alone. The per-run session dir is removed on exit. These are the origins of `decision/cold-agent-cleanup-scoped`, `decision/local-state-never-uploaded` and `decision/cold-agent-state-never-printed`. A sweep of `target/` is a second remover of this state, which this precedent reserves to the pipe's `cleanup`.

@history_item n=9 marker=2026-10-07-audit-corrections
**2026-10-07-audit-corrections** — The headless-stand census had to name the shared module `tests/blitz-tests/tests/common/mod.rs`, because checks read helpers from a file that is not itself a `stand_*` check. Precedent: moving or merging test files moves what the census names, and the census bullets (`decision/headless-stand-no-sink`) must be re-read over the new layout.

@history_item n=10 marker=2026-10-07-driver-session
**2026-10-07-driver-session** — Named the second sink installer, `escher-session`, beside `seven_guis_native`, both in the seven_guis package (`decision/sink-installed-first-in-main`, `decision/scrub-sink-reach`). Recorded the trap: a zero read from a host that installs no sink says nothing about one that does. This applies to re-measuring host logs after a build change.

@history_item n=11 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** — (`tracing` ungated in escher-driver) escher-driver became the second escher crate taking tracing ungated. The plan also recorded that a workspace-wide build turns `blitz-dom/tracing` on, so a test child with the sink at `info` also receives engine events, while a package-alone build leaves it off. This is the origin of `decision/engine-tracing-by-invocation`. It is the measured feature-unification difference between invocations that this chunk's cause hypothesis names.

@history_item n=12 marker=2026-10-09-audit-corrections-agent-surfaces
**2026-10-09-audit-corrections-agent-surfaces** — §9: package-install output stays in the job's step log, because `apt-install.sh` is run by no `ci-leg.sh` leg and no `target/ci-logs/` file holds it. The entry also kept that a new artifact out of fork CI waits for the founder. Changes to `ci-leg.sh` or `ci.yml` must keep this split.

@history_item n=13 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** — Two sub-entries. The first measured the "By invocation" wiring: the workspace-built session host writes one `blitz_dom::document` INFO engine line from `info` up, and the host built alone writes none. The second measured the host by level under both builds; `host_log` and `mcp_log` read under the package-alone and workspace builds. Both builds are load-bearing for `decision/engine-tracing-by-invocation` and `decision/sink-host-read-at-trace`. A mend that unifies feature sets must keep both builds reachable or restate these rows.

@history_item n=14 marker=2026-10-14-stand-requirement-sweep
**2026-10-14-stand-requirement-sweep** — (§2 Feature wiring re-read) The workspace-built host's engine line printed twice (6 lines / 978 B), while built alone it was unchanged. This was a measured build-dependent difference in log output, recorded under §2 Feature wiring. It is precedent that the two builds give different telemetry and that each re-read is dated.

@history_item n=15 marker=2026-10-14-quality-gates
**2026-10-14-quality-gates** — §2 and §8: a client's stderr, the host and the workspace-built windowed `seven_guis_native` were read by build and level in 28 runs, with 0 needles. This extends the by-build readings that a change to invocations or features could invalidate.

@history_item n=16 marker=2026-10-15-cold-agent-test
**2026-10-15-cold-agent-test** — The cold-agent pipe's live child runs `escher-session mcp` with `RUST_LOG=info`, and its span lines land in `target/cold-agent/server.log`, which the pipe only counts. The state files are `verdict.json`, `events.jsonl`, `transcript.jsonl`, `client.log`, `server.log` and `check.json`. These are the rows `decision/cold-agent-server-log-counted-only` and `decision/cold-agent-state-never-printed`. The pipe is one of the invocations this chunk measures, and its state shares `target/` with build artifacts.

@history_item n=17 marker=2026-10-17-upstream-sync-engine-debts
**2026-10-17-upstream-sync-engine-debts** — The coverage report's lcov excludes `keyboard_compat.rs` by filename, bound to test-plan's coverage leg. A change to the coverage leg's invocation must keep this exclusion (`decision/coverage-artifact-on-success`).

@history_item n=18 marker=2026-10-19-session-recording
**2026-10-19-session-recording** — §9 gained the Recording directory row: mode `0700`, local only, uploaded by no CI leg. This sits beside the session state directory under `decision/local-state-never-uploaded`. A `target/` sweep must not reach state that rows place outside `target/`. Local state under `target/` is removed only by its owner's cleanup.

@history_item n=19 marker=2026-10-19-test-splits
**2026-10-19-test-splits** — Precedent for a test-only move. Tests were split into `cli_tests.rs`, `recording_tests.rs` and `recording_off_unix_tests.rs`, and a shared fragment moved to seven_guis' `tests/common/mod.rs`. The silence census was re-read over the moved files, and the log checks kept the same zeros. A "one binary per leg" mend needs the same census re-read.

@history_item n=20 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — Recorded `stand_screenshot_repeat`, a second re-executed child whose stdout the parent reads into assertions only. It belongs with `stand_id_persistence` and `stand_act_spans`' re-run children (the exception in `decision/headless-stand-no-sink`). If test files are merged into one binary, each re-exec must still target the right binary and test.

@history_item n=21 marker=2026-10-24-a11y-re-read
**2026-10-24-a11y-re-read** — Recorded silent the eight checks the `a11y` leg runs beside its six earlier files, and the two shared modules. The leg's membership is a list of test files, so merging per-file binaries changes how this leg selects its targets.

@history_item n=22 marker=2026-10-24-stand-requirement-sweep
**2026-10-24-stand-requirement-sweep** — (the sweep's checks silent) `stand_requirement_sweep` reads its `WITNESSES` tables and its leg check through `include_str!`. The check reads CI-leg membership from files. A change to legs or to the test layout is pinned by this check as well as by the CI scripts' tests.

@history_item n=23 marker=2026-10-26-codebase-health
**2026-10-26-codebase-health** — Named `stand_requirement_sweep`'s target predicate `inside_stand_repeat`. The check judges which targets the `stand-repeat` leg holds, so a merge of test binaries or a change to `stand-repeat`'s invocation moves what this predicate reads.

@history_item n=24 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** — (the test crate slice's absences) The test crate slice's absences note upstream's `pseudo_element_flow_classification`, with its `mod` line in the unbuilt `all.rs`. `tests/blitz-tests` already holds an aggregated `all.rs` that is not built. Any "one binary per leg" mend meets this file and its stale membership.

@history_item n=25 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** — (citations re-pointed) §9's `.github/scripts/ci-leg.sh` and `.github/workflows/ci.yml` citations were re-pointed after `ci.yml`'s free-disk step moved. Edits to the CI files move the plan's line citations, and the wrap re-points them. The free-disk step is a disk-pressure precedent on CI.

@history_item n=26 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — (citations re-pointed) Precedent for re-pointing `ci-leg.sh` citations when lines are added. The same entry's silent-checks census counted `mod session_common;` declarers (31 to 32). Moving test files or merging binaries moves every such count stated in the plan.

@history_item n=27 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** — (citations re-pointed) Kept: telemetry-strategy.md still names `ci-leg.sh fast` among the workspace builds, though `fast` now ends on a `cargo test -p blitz-tests` step. This is a known stale point in the by-invocation row (`decision/engine-tracing-by-invocation`): which legs build the whole workspace and which build one package. This chunk measures exactly that.

@history_item n=28 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** — (`session_common` readers) The `session_common` readers moved from 34 to 35. This is the latest of many counted file-layout facts (readers of `session_common`, `common` and `fixtures`) in the plan's §3 census. A mend that merges or moves test files must recount them.

@history_item n=29 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — The master's text became rows. Every picked row (decision/contract/absence/overview) dates from this conversion, each verified against the code at that time. Row edits since then follow the `~`/`+`/`−` handle form.

@history_item n=30 marker=2026-11-16-layout-explained
**2026-11-16-layout-explained** — (`decision/headless-stand-no-sink` · except) The exception was widened to the MCP tools and layout stand checks' own recording subscribers. These checks install subscribers in process, so merging them into one binary with silent checks needs care, because a global subscriber is per process.

@history_item n=31 marker=2026-11-16-errors-with-the-action
**2026-11-16-errors-with-the-action** — Changed `~ decision/headless-stand-no-sink` (acting steps run only under escher-telemetry's scoped capture). Also changed `~ contract/telemetry-sink/rust-log` and `~ contract/escher-sink-env/rust-log` (the RUST_LOG filter applies to the sink layer alone, the capture layer unfiltered). These are picked rows. Any env or feature change in the build path must keep them true.

@history_item n=32 marker=2026-11-16-error-text-in-the-answer
**2026-11-16-error-text-in-the-answer** — (3. Observability Harness Contract) Changed `~ decision/headless-stand-no-sink` · except: the command-span check's re-run children and the error-text founder check install escher's sink over an in-memory writer. This is another in-process sink install among the test targets. It bears on a one-binary-per-leg merge, because a process-global sink must not leak into silent checks.

@history_item n=33 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** — (3. Observability Harness Contract) Changed `~ decision/sink-host-read-at-trace`: the host and the mcp role are read at trace under both builds with the layout-answer key needles added. This is the picked row requiring both the package-alone and the workspace build. A mend that unifies features or package selection must keep both readings possible.

@history_item n=34 marker=2026-11-18-cold-agent-bar
**2026-11-18-cold-agent-bar** — Changed `~ decision/cold-agent-server-log-read-by-span-name` (the pipe also reads the tool_call span's tool and ending and the command span's cause word) and `~ decision/cold-agent-server-log-counted-only` (`server.log` is read only to count its span lines). These are picked rows describing the cold-agent pipe, one of the invocations measured. Its `target/cold-agent/` state must survive any sweep until its own cleanup.

@history_item n=35 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** — (the cold-agent pipe's verdict fields and live child's recording) The live server child is spawned as `<abs target/debug/escher-session> mcp --record <abs target/cold-agent/record>`, with `record/` owner-only (`mkdir -m 700`) and removed by `cleanup` (`decision/cold-agent-kept-checks-stay-local`, `decision/cold-agent-kept-checks-at-rest`). The pipe hard-binds `target/debug/` as its binary path. A profile, target-dir or `CARGO_TARGET_DIR` change in the mend must keep that path valid.
