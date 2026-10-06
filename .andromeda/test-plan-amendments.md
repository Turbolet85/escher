# test-plan — amendments

One entry per amendment to `test-plan.md` (sidecar-contract.md §Entry form). Appended at wrap P2; rewritten only by the epoch-close consolidation.

## 2026-10-05-as-built-baseline — the local baseline reading
**Section:** §9 CI Integration (Pipeline facts)
**Change:** new bullet "Local baseline" — the reference the fork's CI is compared against: dev profile, workspace default features, `--locked`, no `opt-level` rewrite; cold after `cargo clean`, warm on an immediate re-run, on the dev host (32 CPUs, 2026-10-05): `cargo build --workspace` 120.26 s / 5.54 s; `cargo test -p blitz-tests` 486.45 s / 7.41 s, 61 result lines, 255 passed · 0 failed · 3 ignored (`paint_tree_bench`), no font-skip line; `cargo test --workspace` 1632.88 s / 13.08 s, 108 result lines, 407 · 0 · 3, the test-profile compile dominating; fmt and clippy exit 0; workspace rustdoc `-D warnings` exit 101 (3 crates, 9 errors).
**Why:** the as-built baseline chunk exists to fix this starting point; "Fork CI reached" reads its CI leg against it.
**Ref:** .andromeda/runs/2026-10-05T20-35-18-wrap/

## 2026-10-05-fork-ci-reached — CI on the build branch, legs through ci-leg.sh, upstream-only WPT
**Section:** §1 Test Scope Summary (Workspace (CI) · CI Python scripts · Native platforms · Web Platform Tests) · §4 Unit Test Strategy (What unit tests cover) · §6 E2E Test Strategy (Drivers per surface — WPT) · §9 CI Integration (Platform · Pipeline facts)
**Change:** was CI on PRs and pushes to main/v0.*, inline `cargo test --workspace`, a matrix testing windows/macos/linux, WPT in this repository's CI; now:
- CI runs on PRs and pushes to main, v0.* and build/**; each linux leg runs `bash .github/scripts/ci-leg.sh {leg}` (every cargo leg `--locked` but `examples/wasm_hello`), the same command the host runs; the workspace tests are the fast `test` leg, `cargo test --workspace --locked`; the CI-script tests the `ci-scripts` leg (PyYAML ensured);
- fast/slow: fmt, clippy, test-features-default, ci-scripts carry no `needs`, every other job needs all four; rust-cache on every compiling job saved on main and build/*; each leg's log uploaded on failure, 7 days; local pre-push gate `ci-leg.sh fast`;
- the matrix tests windows and macos and builds ios and android, `--locked`; linux is the `test` leg's;
- WPT and its PR-results posting run only on upstream `DioxusLabs/blitz` (repository guard); the fork runs WPT on the host;
- §4 adds `test_ci_workflows.py` (CiWorkflowTest · UpstreamGuardTest · LegScriptTest with a cargo shim) — the CI-scripts leg runs 16 tests;
- the first build-branch run took 1255 s cold and its same-sha re-run 475 s warm (the fast `test` leg 315 → 169 s), as measured on GitHub-hosted runners.
**Why:** fork CI reached on escher's build branch with host-reproducible legs, caches and a fast/slow split (the chunk's acceptance; CI run green cold and warm).
**Ref:** .andromeda/runs/2026-10-05T21-59-55-wrap/

## 2026-10-05-fork-ci-reached — local baseline re-measured under line-tables-only debuginfo
**Section:** §9 CI Integration (Local baseline)
**Change:** was the full-debuginfo dev profile's figures, per "2026-10-05-as-built-baseline — the local baseline reading" — build 120.26 / 5.54 s, blitz-tests 486.45 / 7.41 s, workspace tests 1632.88 / 13.08 s cold/warm; now the dev profile carries `debug = "line-tables-only"` and the baseline reads build 63.90 / 2.07 s, blitz-tests 45.36 / 6.51 s (61 lines, 255 · 0 · 3, font-skip line absent), workspace tests 52.10 / 11.44 s (108 lines, 407 · 0 · 3), cold total 161.36 s against 2239.59 s, `target/debug` 32G; fmt and clippy legs exit 0. The workspace rustdoc reading (exit 101, 3 crates, 9 errors) keeps its earlier evidence, not re-measured.
**Why:** the §9 baseline's "dev profile" condition moved; the counts held, so the profile change cost no test.
**Ref:** .andromeda/runs/2026-10-05T21-59-55-wrap/

## 2026-10-05-fork-ci-reached — file:line citations re-pointed after the CI files moved
**Section:** every section citing `ci.yml` or `wpt.yml` lines
**Change:** 13 citations re-pointed — `wpt.yml` from line 26 on +1 (the repository guard), `ci.yml` by a range map over the rewritten file (e.g. the test job 72-89, the CI-scripts job 175-188, the matrix platforms 220-252); no claim text changed by the re-point.
**Why:** this chunk moved the cited lines.
**Ref:** .andromeda/runs/2026-10-05T21-59-55-wrap/

## 2026-10-05-ci-gate-legs — audit, a11y and coverage legs; workspace rustdoc green; first coverage reading
**Section:** §3 → `coverage-tooling-install` · §4 Unit Test Strategy (CI workflows and leg script) · §9 CI Integration (Legs · Cache · Failure logs · Local baseline · Coverage — was Observed absent) · §10 Quality Gates & Coverage Targets · every section citing `ci.yml`, `ci-leg.sh` or `test_ci_workflows.py` lines
**Change:**
- was nine linux leg jobs; now twelve — `audit` (`cargo deny check advisories`), `a11y` (`--test accessibility_hidden --test accessibility_roles --test focusability_updates`), `coverage`; the `doc` leg runs `cargo doc --workspace --no-deps --locked`;
- was "Coverage tooling" observed absent and `coverage-tooling-install` recorded absent; now the `coverage` leg runs `cargo llvm-cov --workspace --locked --lcov --output-path target/coverage/lcov.info`, then `cargo llvm-cov report --workspace --locked`, no threshold; cargo-llvm-cov 0.9.1 + `llvm-tools-preview` in CI, the `llvm-tools` component on the dev host; the job uploads `coverage-report` on success, 7 days; the key reads discharged;
- §10 records the first line-coverage reading, no threshold: 53.23 % dev host (34 900 lines, 151 files), 53.25 % CI; the NOT YET MEASURED marker keeps the gate, threshold, flakiness and performance budgets;
- Local baseline: was workspace rustdoc exit 101 (3 crates, 9 errors), per "2026-10-05-fork-ci-reached — local baseline re-measured under line-tables-only debuginfo"; now `ci-leg.sh doc` exit 0, no collision, audit `advisories ok`, a11y 6 · 6 · 3, coverage exit 0 (its instrumented run 404 · 0 · 3 — no doctests);
- Cache: was rust-cache on every compiling job; now every compiling job but `coverage`, the test job under `shared-key: workspace-test`, `a11y` restoring it with `save-if: false` (an exact hit in CI); run 37386253475 took 769 s wall, windows the critical path at 506 s, the uncached coverage job 256 s off it;
- §4: was 16 tests; now 23 — the CI-workflow invariants add SHA pins, toolchain inputs, workflow `permissions`, the a11y job name, the `coverage-report` upload and the cache-less / restore-only split; the leg-script shim checks the doc, audit, a11y and coverage legs;
- 15 citations re-pointed by a measured line map; no claim text changed by the re-point.
**Why:** the CI gate legs chunk; the `coverage-report` upload is a boundary widening recorded PROVISIONAL — delegate overseer, 2026-10-05, under the founder's standing delegation (relayed verbatim by overseer). Trap: `cargo llvm-cov report` without `--workspace` exits 0 with an empty table in this workspace.
**Ref:** .andromeda/runs/2026-10-05T23-50-17-wrap/

## 2026-10-06-telemetry-bootstrap — stand log format measured; telemetry tests; workspace count re-counted
**Section:** §1 Coverage scope (escher-telemetry · tests/blitz-tests) · §3 Test Harness Contract (Stand log format · NOT YET MEASURED marker) · §9 CI Integration → Local baseline · every section citing `Cargo.toml` or `tests/blitz-tests/Cargo.toml` lines
**Change:**
- §1: escher-telemetry holds 5 inline unit tests (three scrub branches, a bridged `log` record, the identity macro); tests/blitz-tests coverage adds process telemetry through `telemetry_stdout_silent`, `telemetry_scrub`, `telemetry_panic_hook`, `telemetry_init_idempotent`.
- §3: was "no … log format … was gathered"; now the stand's stderr line `{RFC 3339 UTC time} {LEVEL} {target} service.name=… service.version=… {field}={value}…` with the allowlist scrub (bound to obs-plan §3), exercised by those four files and the boot smoke `RUST_LOG=info timeout 10 target/debug/seven_guis_native` (exit 124, `service.name=seven_guis`); the marker keeps boot / status / cleanup / logs commands, status shape, PID file and test-data bootstrap.
- §9 Local baseline: `cargo test --workspace` re-counted 416 passed · 0 failed · 4 ignored, 114 result lines (was 407 · 0 · 3, 108 lines — its timings kept as the 2026-10-05 run's); the blitz-tests 255 · 0 · 3 and coverage-leg 404 · 0 · 3 readings marked as before this chunk's +9 tests / +1 ignored.
- 2 `file:line` citations re-pointed by the chunk's measured line shifts.
**Why:** the telemetry bootstrap chunk added the crate's unit tests, four one-process-per-behaviour integration files and the stand's log line.
**Ref:** .andromeda/runs/2026-10-06T01-33-33-wrap/

## 2026-10-06-headless-stand — stand checks, HarnessOptions fields, bundled font, timer seam, workspace re-count
**Section:** §1 Coverage scope (tests/blitz-tests) · §2 Font-dependent tests · §3 blitz-test-harness Construction · Core · Pump semantics · §7 Seed strategies (Font payload) · Builders and options · §8 Hand-written fakes (Events, Time) · Real dependencies kept · §9 Fonts · Local baseline
**Change:**
- §1: tests/blitz-tests covers the seven_guis headless stand (`stand_boot` · `stand_counter` · `stand_flight_booker` · `stand_timer` · `stand_crud`) and the Dioxus falsy-`disabled` clearing (`dioxus_falsy_disabled`).
- §2 / §8 / §9 Fonts: was "font-dependent tests skip with `eprintln!`" and "rely on `system-fonts`"; the skip and the dependency now cover the pre-existing tests only — stand checks assert unconditionally under the bundled DejaVu Sans with system fonts off, in the package-alone build too.
- §3 Construction: the stand boot `seven_guis::stand::{boot, boot_timer}` with `stand::options(incremental)` is the stand checks' drive surface.
- §7: HarnessOptions was six fields; now eight (`font_ctx`, `incremental`); `stand::options` builds the pinned literal; the Font payload row adds `seven_guis::DEJAVU_SANS`.
- §8 Time: `TimerTicks` is the stand's time stand-in (one 0.1 s step per delivered tick, no `Delay`).
- §9 Local baseline: `cargo test --workspace` re-counted 430 passed · 0 failed · 4 ignored, 120 result lines (was 416 · 0 · 4, 114); the blitz-tests note adds 6 files / +14 tests, not re-measured per crate.
- 6 `harness.rs` citations re-pointed by the two new HarnessOptions fields' line shift.
**Why:** the headless stand chunk wrote 13 stand checks and 1 regression test and gave the harness a font and layout-mode option.
**Ref:** .andromeda/runs/2026-10-06T02-44-36-wrap/

## 2026-10-06-stand-test-contract — the agent-run 5-command contract measured
**Section:** §3 Test Harness Contract · §4 Unit Test Strategy
**Change:**
- §3: a new "Agent-run contract" block — `bash scripts/agent-run.sh {boot | run {selection} | status | cleanup | logs}` from the repository root, `scripts/agent-run.ps1` a pass-through Windows entry with no logic (not run on the dev host); exit grammar `0` success · `1` failed (a build failure, a failing run, an empty run) · `2` usage, checked before · `3` precondition unmet; selections `stand` · `all` · `{name}` (`^[a-z0-9_]+$` plus an existing file); `run.start.files` is `[]` under `all`; `run stand` with no `stand_*.rs` is an empty run (no cargo call, `cargo_exit` null, exit 1); JSON-line events `boot` · `run.start` · `test` · `run.end` (printed and appended to `events.jsonl`) and `status` · `cleanup` (printed only), never captured test output; state in `target/agent-run/{status.json, events.jsonl, run.log}` only; no daemon, PID file, status endpoint, port or env var.
- §3 marker: was "no product boot, status, cleanup or logs command, status endpoint shape, PID file or test-data bootstrap mechanism … was gathered"; now the test-data bootstrap mechanism alone.
- §4: `test_agent_run.py` (`AgentRunTest`, 14 cases under a `cargo` shim) joins "What unit tests cover"; the CI-scripts leg was 23 tests, now 37.
**Why:** the stand test contract chunk wrote the scripts and their contract tests; the three points the plan left open (`files` under `all`, the stand-less empty run, usage before not-booted) stand as built on the operator's ruling in the implementing session (2026-10-06).
**Kept:** the event schema is harness metadata bound to obs-plan §3, not escher's telemetry line format.
**Ref:** .andromeda/runs/2026-10-06T03-39-41-wrap/
