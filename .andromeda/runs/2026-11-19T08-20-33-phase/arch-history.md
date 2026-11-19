@history_item n=1 marker=2026-10-05-fork-ci-reached
**2026-10-05-fork-ci-reached** (one dev-profile debuginfo level for host and CI) — added `[profile.dev] debug = "line-tables-only"`, inherited by `test`, one level for dev host and CI (rows decision/dev-line-tables-debuginfo, decision/one-debuginfo-level); measured a blitz-tests binary's debuginfo share 82 % → 55 % and the cold baseline 2239.59 s → 161.36 s; chosen over a CI-only `CARGO_PROFILE_DEV_DEBUG` to avoid host/CI divergence. It explicitly deferred consolidating the blitz-tests binaries "until measured after cache + debuginfo" — the direct precedent for this chunk's "less debuginfo" and "one binary per leg" levers.

@history_item n=2 marker=2026-10-05-fork-ci-reached
**2026-10-05-fork-ci-reached** (fork CI on the build branch through one leg script) — established `.github/scripts/ci-leg.sh {leg}` as the one runner CI and the dev host share, every cargo leg `--locked` (but `examples/wasm_hello`), `fast` as the local pre-push gate, merged output tee'd to `target/ci-logs/{leg}.log`, rust-cache on every compiling job saved on `main` and `build/*`, no opt-level rewrite (rows decision/one-ci-leg-runner, decision/cargo-legs-locked, decision/unrewritten-profiles, decision/ci-cache-save-refs, decision/rust-cache-on-compiling-jobs). Any change to a leg's cargo invocation lands in this script and its pins; the founder's concern then was that CI must not rebuild everything each run.

@history_item n=3 marker=2026-10-05-fork-ci-reached
**2026-10-05-fork-ci-reached** (the fork's cache budget) — measured the Actions cache at 12 entries ≈ 9.73 GB of the 10 GB per-repository budget, one rust cache per compiling job; a lockfile or toolchain change mints new keys and evicts. A mend that changes profiles, flags or package selections changes what each job's cache holds and so this budget.

@history_item n=4 marker=2026-10-05-ci-gate-legs
**2026-10-05-ci-gate-legs** (CI gate legs) — added the `audit`, `a11y` and `coverage` legs; `coverage` runs `cargo llvm-cov` to `target/coverage/lcov.info` (registered Filesystem path, uploaded as `coverage-report`); rust-cache on every compiling job but `coverage`, `a11y` restore-only. Bears on the scope's open premise of where `cargo llvm-cov` builds and whether it counts toward `target/` growth (rows decision/rust-cache-on-compiling-jobs, contract/ci-leg-legs/coverage).

@history_item n=5 marker=2026-10-05-ci-gate-legs
**2026-10-05-ci-gate-legs** (shared test cache, the cache budget re-measured over) — the test job saves under `shared-key: workspace-test`, which `a11y` (and later `stand-repeat`) restore with `save-if: false`; a re-keyed lockfile left the cache at 10.7 GB, over budget, evicting entries (row decision/test-cache-restore-only). Trap: a new cache entry or a key change starves the legs that restore last.

@history_item n=6 marker=2026-10-06-stand-test-contract
**2026-10-06-stand-test-contract** — registered the agent-run harness `scripts/agent-run.sh` driving `cargo test -p blitz-tests --locked`, its exit grammar 0/1/2/3, JSON-line events, and its state area `target/agent-run/` recreated by `boot`, removed by `cleanup`, local only (rows contract/agent-run-*, decision/agent-state-areas-local). One of the cargo invocations the chunk must measure; a sweep must leave its state area's contract intact.

@history_item n=7 marker=2026-10-06-cold-agent-run-pipe
**2026-10-06-cold-agent-run-pipe** — registered the cold-agent pipe and its state area `target/cold-agent/`, uploaded by no CI job (rows decision/run-state-areas-local-only, decision/agent-state-areas-local). A sweep of `target/` must remove nothing else under `target/` than what each area's cleanup owns.

@history_item n=8 marker=2026-10-15-cold-agent-test
**2026-10-15-cold-agent-test** (its contract and stack row) — the cold-agent pipe exits 3 when `target/debug/escher-session` is absent, building nothing; its MCP server is the workspace's own `escher-session mcp` (rows decision/cold-agent-mcp-server-is-own, decision/cold-agent-client-host-only). The pipe's build is the operator's, so its growth share comes from whatever cargo build produces that binary, and a sweep must not delete it before a run.

@history_item n=9 marker=2026-10-06-accessibility-tree-identity
**2026-10-06-accessibility-tree-identity** — recorded the trap that a feature-gated item can pass under `cargo test --workspace` and be compiled out under `-p blitz-tests` (check `cargo tree -e features -i`). Direct evidence for the scope's premise that package selection changes feature unification and so the units' hashes.

@history_item n=10 marker=2026-10-06-id-persistence
**2026-10-06-id-persistence** — registered the first test re-exec spawns: `telemetry_stdout_silent` and `stand_id_persistence` re-execute their own test binary via `current_exe()` with `--ignored --exact {child}`. A one-binary-per-leg mend changes what `current_exe()` is and what `--exact` names.

@history_item n=11 marker=2026-10-07-sink-target-allowlist
**2026-10-07-sink-target-allowlist** — a third re-executing blitz-tests binary, `telemetry_drop`, whose child runs under its own `RUST_LOG` and installs the sink; the telemetry checks need a process each. Constrains merging test files into one binary.

@history_item n=12 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** (the driver's command span) — a fourth re-executing binary, `stand_act_spans`, runs two ignored children with `RUST_LOG=info` set on one and removed from the other, each installing the sink over an in-memory capture. Another per-process test a merged binary must keep.

@history_item n=13 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** (the scopes, the tests and the stack) — five blitz-tests binaries re-execute their test binary (`stand_screenshot_repeat` runs it twice, comparing shots byte for byte across child runs). The re-exec count a binary consolidation must preserve.

@history_item n=14 marker=2026-10-10-upstream-sync-agent-surfaces
**2026-10-10-upstream-sync-agent-surfaces** (blitz-tests keeps one test target per file) — the decision behind rows decision/blitz-tests-target-per-file, decision/blitz-tests-one-target-per-file, contract/blitz-tests-single-test-target: upstream moved to one test binary (`autotests = false`, one `all` target); escher kept every file its own target because under upstream's shape `--test {name}` names no target, `mod common;` / `mod session_common;` do not resolve from a module, the re-executing checks' bare `--exact` name would match nothing and still exit 0, and the telemetry checks need a process each. `tests/all.rs` is held at `test = false`, pinned by `test_blitz_tests_targets.py` in the `ci-scripts` leg. The main trap and precedent for the "one test binary per leg" lever.

@history_item n=15 marker=2026-10-10-upstream-sync-agent-surfaces
**2026-10-10-upstream-sync-agent-surfaces** (the merge base, Parley's pin form) — added `test_blitz_tests_targets.py`, which uses stdlib `tomllib`, so the `ci-scripts` leg needs Python 3.11+. One of the two pins the scope says a leg-invocation change goes through.

@history_item n=16 marker=2026-10-07-audit-corrections
**2026-10-07-audit-corrections** — introduced `tests/blitz-tests/tests/common/mod.rs`: a module in a subdirectory of the integration-test directory is no test target, so each test-target file still covers one behavior; tables typed by dev-dependencies are shared this way because the test crate's `[lib]` cannot see them (rows decision/shared-test-modules, decision/stand-check-shared-helpers, decision/integration-test-one-behavior). A layout change of `tests/` must keep this.

@history_item n=17 marker=2026-10-19-shared-fixtures
**2026-10-19-shared-fixtures** — added the third shared non-target module `tests/blitz-tests/tests/fixtures/mod.rs` (`mod fixtures;`), a page with one reader staying in its check's file (rows decision/fixtures-test-module, decision/shared-fixtures-stated-once, decision/single-reader-page-stays-local). Another reader graph a test-layout mend moves.

@history_item n=18 marker=2026-10-07-driver-session
**2026-10-07-driver-session** (manifest citations re-pointed after the two root-manifest inserts) — trap: any insert into the root `Cargo.toml` shifts every citation below it in all masters. A `[profile.*]` edit to the root manifest is such an insert.

@history_item n=19 marker=2026-10-07-driver-session
**2026-10-07-driver-session** (the driver session registered) — registered the checks' state directories under `target/tmp/` and `host_binary` spawning the real `escher-session` binary. Test-only scratch lives inside `target/`, so a sweep must not race running checks or count their directories as stale.

@history_item n=20 marker=2026-10-19-session-recording
**2026-10-19-session-recording** (wire v3, fifteen session errors, the recording module and the eighth seven_guis target) — stand-repeat's seven_guis list gained `recording`; the recording checks' and escher-driver's unit-test scratch lives under `target/tmp` through compile-time `CARGO_MANIFEST_DIR` (test-only). More of `target/` that is not build artifact.

@history_item n=21 marker=2026-11-13-local-files-for-local-documents
**2026-11-13-local-files-for-local-documents** (Occupied Resources) — added name/other/cargo-target-tmpdir: a blitz-tests target reads `CARGO_TARGET_TMPDIR` for its test-only fixture directory. A sweep or a target-directory change must respect it.

@history_item n=22 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** (scope rows after the command line) — seven_guis' unix-gated `cli_flow` runs two POSIX `sh` scripts under plain `sh` against the built binary, with no dependency, manifest line or test framework added (row decision/cli-flow-under-plain-sh); seven_guis targets went to five. Part of the `examples/seven_guis/tests/` layout the scope may touch.

@history_item n=23 marker=2026-10-09-audit-corrections-agent-surfaces
**2026-10-09-audit-corrections-agent-surfaces** — fork CI's package install became one bounded, retried script `.github/scripts/apt-install.sh`, no `ci.yml` line calling `apt-get` (rows decision/bounded-retried-package-install, decision/one-apt-install-script, decision/bounded-package-install). Any CI job the mend adds or changes installs through it.

@history_item n=24 marker=2026-10-14-quality-gates
**2026-10-14-quality-gates** (the quality gates) — added the `stand-repeat` leg (three rounds of every `stand_*` target plus seven_guis process checks, first red ends, no retry), the workspace-wide `cargo deny` audit, the second clippy line `cargo clippy -p blitz-dom --no-default-features --features …` run only when the workspace line is green, and `coverage_floor.py` with per-crate floors (rows decision/stand-repeat-zero-failure-budget, decision/blitz-dom-feature-subset-clippy, decision/per-crate-coverage-floors, contract/coverage-floor-*). The blitz-dom subset clippy line and stand-repeat's `-p blitz-tests` runs are named in the scope's premise as candidate second-hash sources.

@history_item n=25 marker=2026-10-14-stand-a11y-assertions
**2026-10-14-stand-a11y-assertions** — the `a11y` leg widened in place (no new job, required check or `ci.yml` change), its target list pinned by `LegScriptTest` (row decision/a11y-leg-widens-without-new-job). Precedent that leg membership changes go through the leg script and its pin.

@history_item n=26 marker=2026-10-24-a11y-re-read
**2026-10-24-a11y-re-read** — the `a11y` leg runs its named targets on one `cargo test --workspace --locked` line, the pin asserting each target word plus `--workspace` and `--locked` (row contract/ci-leg-legs/a11y). A `--workspace` selection differing from `-p blitz-tests` legs is one of the invocation differences to measure.

@history_item n=27 marker=2026-11-18-a11y-re-read
**2026-11-18-a11y-re-read** — contract/ci-leg-legs/a11y dropped its count ("fourteen") since the leg runs eighteen targets; the count is the leg script's inventory. A merge of targets changes that inventory and what the capability pin reads.

@history_item n=28 marker=2026-10-24-stand-requirement-sweep
**2026-10-24-stand-requirement-sweep** — `stand-repeat` names targets beyond the `stand_*` glob as literal `--test` words, pinned by `test_stand_repeat_leg_runs_three_rounds` and read from the script's text by `stand_requirement_sweep`'s leg check (row contract/ci-leg-legs/stand-repeat). Renaming or merging targets breaks these `--test` words and their pins.

@history_item n=29 marker=2026-10-26-codebase-health
**2026-10-26-codebase-health** (`stand-repeat`'s membership held jointly) — the sweep test reads `ci-leg.sh` whole by `contains`, so stand-repeat membership is held jointly with `test_stand_repeat_leg_runs_three_rounds`; a target dropped from `stand-repeat` alone still reads inside while `a11y` names it. Trap for any leg rewrite.

@history_item n=30 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (stand-repeat's named targets) — stand-repeat's named blitz-tests targets went ten → fifteen while `stand_requirement_sweep`'s `STAND_REPEAT_NAMED` still holds the first ten. Shows the named lists that move with any target change.

@history_item n=31 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the select list's checks and the counts they moved) — stand-repeat's named targets fifteen → seventeen (`select_popup`, `select_popup_painted`), asserted by `test_stand_repeat_leg_runs_three_rounds`; shared-module reader counts re-measured. The same named-list coupling.

@history_item n=32 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the legs that do not run locked) — corrected the reading to: every cargo leg `--locked` but `fmt` and `examples/wasm_hello` (rows decision/cargo-legs-locked, decision/locked-cargo-legs). This is the invariant the scope says must stay.

@history_item n=33 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the free-disk steps and what they show) — the `Stand checks repeated` job went red out of disk (`Free space left: 5 MB`, CI#41190771843) and gained a SHA-pinned `jlumbroso/free-disk-space` step; the record says it is not shown to be the cure. The CI-side face of the same build-directory growth this chunk measures.

@history_item n=34 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (`matrix_test`'s free-disk step) — the skipped matrix step was removed and `test_only_stand_repeat_frees_disk` pinned that only stand-repeat frees disk. Precedent for disk handling living in CI pins.

@history_item n=35 marker=2026-11-17-upstream-sync-ahead-of-polish-and-ship
**2026-11-17-upstream-sync-ahead-of-polish-and-ship** (Standard Contracts / Occupied Resources: the free-disk rows retired) — `test-features-default` gained the same free-disk step, its guard renamed, and the rows decision/only-stand-repeat-frees-disk and decision/free-disk-space-only-in-stand-repeat were retired, the fact's home now test-plan's CI integration. Two CI jobs now hit disk limits — context for the growth measurement.

@history_item n=36 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (no local gate builds `font-embolden`) — recorded that no local gate built `blitz-paint/font-embolden`; `--features blitz-paint/font-embolden` on `cargo test -p blitz-tests --test select_popup_painted` named as the local witness. Origin of the `embolden` step.

@history_item n=37 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (the `embolden` step of `fast`) — `fast` became fmt → clippy → test → ci-scripts → embolden; `embolden` runs `cargo test -p blitz-tests --locked --no-fail-fast --features blitz-paint/font-embolden --test select_popup_painted`, no CI job, pinned by `test_fast_leg_ends_on_the_font_embolden_witness` (rows contract/ci-leg-legs/embolden, contract/ci-leg-legs/fast, decision/font-embolden-local-witness, decision/local-pre-push-gate). A feature set differing from the `test` leg on every pre-push — a prime suspect for a second hash of shared units.

@history_item n=38 marker=2026-10-16-fast-failing-mutants
**2026-10-16-fast-failing-mutants** — the host-only mutation tier: cargo-mutants 27.1.0 with `--test-tool nextest`, `--baseline=skip`, cargo-nextest under the `mutants` profile of root `.config/nextest.toml`, default profile untouched, never in CI (rows decision/mutation-tier-host-only, decision/nextest-mutants-profile). One of the invocations to measure; the scope keeps it dev-host-only.

@history_item n=39 marker=2026-11-15-upstream-sync-errors-and-explanation
**2026-11-15-upstream-sync-errors-and-explanation** (Stack and Technologies) — added decision/mutation-tier-excludes-demo-shells: the tier does not run `rdme` and `blitz`. Bounds which packages the mutation tier builds.

@history_item n=40 marker=2026-10-17-upstream-sync-engine-debts
**2026-10-17-upstream-sync-engine-debts** (the keyboard-types 0.8 surfaces) — both `cargo llvm-cov` lines take `--ignore-filename-regex 'keyboard_compat\.rs$'`, pinned by `test_coverage_leg_reaches_the_floor_on_each_crate` (rows decision/coverage-excludes-keyboard-compat, decision/keyboard-compat-coverage-exclusion). A change to the coverage leg's invocation must keep it.

@history_item n=41 marker=2026-10-25-capability-statement
**2026-10-25-capability-statement** — added `docs/capabilities.md` and its pin `CapabilityStatementTest` in `ci-scripts` (eight test files discovered then). Its counts are read from code and scripts, so a test-layout or leg change can turn it red.

@history_item n=42 marker=2026-11-18-capability-statement
**2026-11-18-capability-statement** — decision/capability-statement now holds the statement's a11y leg file count and stand-repeat named-target count to the CI leg runner. Merging, renaming or moving targets in those legs must update the statement or the pin goes red.

@history_item n=43 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (`::before` / `::after` in flow classification, and its one-mode test) — `pseudo_element_flow_classification` is an auto-discovered target whose `mod` line is also in the unbuilt `all.rs`; the both-layout-modes convention gained an exception for it. Shows how upstream-taken targets enter the per-file layout and `all.rs`.

@history_item n=44 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (upstream's both-modes exception ended) — `pseudo_element_flow_classification` now runs both layout modes, the exception gone (row decision/both-layout-modes at that time). The scope requires a moved or merged test to keep its two layout modes.

@history_item n=45 marker=2026-11-13-upstream-sync-determinism-and-exploration
**2026-11-13-upstream-sync-determinism-and-exploration** (Conventions) — decision/both-layout-modes gained its exception: upstream-taken blitz-tests targets are kept as taken even where they run one mode; escher's own run both. A test merge must not convert or drop either kind.

@history_item n=46 marker=2026-11-13-upstream-sync-determinism-and-exploration
**2026-11-13-upstream-sync-determinism-and-exploration** (Infrastructure Patterns) — decision/pinned-actions now covers every workflow's `uses:`, each pinned to a 40-hex SHA with its ref as a trailing comment. Any action a mend adds to `ci.yml` (e.g. a cache or sweep step) must follow it.

@history_item n=47 marker=2026-11-16-errors-with-the-action
**2026-11-16-errors-with-the-action** (Conventions; Standard Contracts) — added decision/capture-before-own-recorder (a stand check installing its own recorder opens an empty capture first so no call site's interest is cached from a thread with no subscriber) and decision/capture-dispatcher-pair (the capture's process-wide dispatcher pair). Both are process-wide; merging stand checks into one binary puts several such checks in one process, where this ordering and global state now interact.

@history_item n=48 marker=2026-10-24-upstream-sync-polish-and-ship
**2026-10-24-upstream-sync-polish-and-ship** (`Parallel` style traversals serialised) — registered process-wide `PARALLEL_TRAVERSAL_LOCK`, and that no headless document sets `style_threading`. Process-wide state that one-binary-per-leg would share across checks that today run in separate processes.

@history_item n=49 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (the Stylo prefs behind a `Once`) — registered the process-wide `Once` `INIT` that sets Stylo prefs on first call. Same concern: shared once-per-process state if test files merge into one binary.

@history_item n=50 marker=2026-11-17-upstream-sync-ahead-of-polish-and-ship
**2026-11-17-upstream-sync-ahead-of-polish-and-ship** (Stack and Technologies; Conventions) — renamed the renderer feature `vello-hybrid` → `vello-gpu`; `seven_guis`' path entry keeps its default `vello-gpu` renderer feature because `dioxus-native` raises `compile_error!` with no renderer (rows decision/window-renderer-chosen-at-compile-time, decision/in-repo-default-features-off, decision/in-repo-crates-no-default-features). Feature defaults per crate decide unification, so they shape which invocations rebuild shared units under a second hash.
