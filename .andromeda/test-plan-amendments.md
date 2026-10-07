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

## 2026-10-06-cold-agent-run-pipe — the cold-agent run pipe contract; CI-scripts 37 → 64
**Section:** §3 Test Harness Contract · §4 Unit Test Strategy (What unit tests cover)
**Change:**
- §3 gains a cold-agent run pipe block beside the agent-run contract: invocation and `.ps1` pass-through; exit grammar `0` · `1` failed verdict · `2` usage before any precondition · `3` precondition; `run counter` (per-run `mktemp -d` session, isolation flags, `timeout 600`, stdin `/dev/null`, no propagated 124); the stdio MCP stub; the verdict fields, the positive-evidence `passed`, the `reasons` tokens, `isolated`, `wrong_calls` recorded but never deciding; events; state `target/cold-agent/` only; proof = `test_cold_agent.py` + one live run (passed, 5 calls, 0 wrong, tokens and cost recorded), never in CI.
- §4 gains the `test_cold_agent.py` bullet (`ColdAgentStubTest` 6 · `ColdAgentPipeTest` 21 under a `claude` shim, no live model).
- §4 CI-scripts count: was 37 (23 + 14) at CI run 37409303977; now 64 (23 + 14 + 27) at CI run 37413576977.
**Why:** the cold-agent run pipe chunk added the cold-agent gate's reachability pipe and its contract tests.
**Ref:** .andromeda/runs/2026-10-06T04-34-08-wrap/

## 2026-10-06-upstream-sync-element-identity — workspace count 430 → 431; wpt/runner attr_test unit test; citation re-point
**Section:** §1 Test Scope Summary (wpt/runner) · §4 Unit Test Strategy (What unit tests cover) · §9 CI Integration (Local baseline) · every section citing a merged upstream file's lines
**Change:**
- §9 Local baseline: `cargo test --workspace` re-counted after the upstream merge — 120 result lines, 431 passed · 0 failed · 4 ignored (was 430 · 0 · 4 at 2026-10-06-headless-stand, kept as history); the +1 is wpt/runner's `subtest_names_include_nonempty_root_titles`.
- §1 wpt/runner unit tests: 8 in fuzzy.rs, 3 in js_wrapper.rs, 2 in harness_test.rs, 1 in attr_test.rs, 1 in mod.rs (was without attr_test.rs).
- §4 wpt/runner: attr_test.rs tests that native checkLayout subtest names include a non-empty root title.
- 19 `file:line` citations into the merged upstream files re-pointed by the merge's measured line map; no other claim text changed by the re-point.
**Why:** the chunk merged upstream `main` at `23354585`, which brought one wpt/runner unit test (upstream `3aa87bc1`).
**Ref:** .andromeda/runs/2026-10-06T08-31-16-wrap/

## 2026-10-06-upstream-sync-element-identity — root-manifest citations re-pointed after the line-61 insert
**Section:** §9 CI Integration (Local baseline)
**Change:** 1 root `Cargo.toml` citation (the `[profile.dev]` debuginfo stanza, now 199-200) re-pointed +1 — it read one line low since the `seven_guis` path entry was inserted at `Cargo.toml:61`; verified against the cited text. No claim text changed.
**Why:** the 2026-10-06-headless-stand wrap did not re-point the root-manifest citations past its insert; the operator chose at this wrap's escalation (2026-10-06) to fix them in this pass rather than carry them.
**Ref:** .andromeda/runs/2026-10-06T08-31-16-wrap/

## 2026-10-06-stable-element-ids — stand element-id checks counted; element_id unit tests
**Section:** §1 tests/blitz-tests coverage · §3 Crate-local test helpers (dioxus-native-dom) · §3 Agent-run contract Proof · §9 Local baseline · the citations below
**Change:**
- §1: the headless-stand coverage gains one stable element id per element in both layout modes (`stand_element_ids`).
- §3 helpers: the `element_id` unit tests build a `DioxusDocument` from a plain app, re-render through a `GlobalSignal` in `vdom.in_runtime`, and stale a detached node by dropping it through `DocumentMutator::remove_node_if_unparented` — a re-render alone only detaches.
- §3 Proof: re-counted `run stand` 17 `ok` stand events (was 13), the new file picked up by its `stand_` prefix.
- §9: re-counted 121 result lines, 441 passed · 0 failed · 4 ignored (was 120 · 431 · 0 · 4): +6 unit, +4 stand checks.
- 7 `file:line` citations into `dioxus_document.rs`, `lib.rs` and the four lean-task files re-pointed by the chunk's measured line maps.
**Why:** the chunk added the six unit tests and four stand checks proving v010-01; counts measured at its fast-leg run on the pre-CI commit.
**Ref:** .andromeda/runs/2026-10-06T09-35-44-wrap/

## 2026-10-06-id-persistence — id persistence coverage; run stand and workspace re-counts
**Section:** §1 Test Scope Summary → tests/blitz-tests · §3 Test Harness Contract → Proof · §9 CI Integration → Local baseline
**Change:**
- §1: stand coverage adds the same id across a re-render, a remount (fresh `NodeId`s bar the four document-skeleton elements) and a fresh process. The test binary re-executes its own `#[ignore]` child `ids_in_a_child_process`. `stand_id_persistence` is the seventh stand file.
- §3 Proof: re-count appended — `run stand` 20 `ok` stand events (+3 `stand_id_persistence`, picked up by its `stand_` prefix).
- §9 Local baseline: re-count appended — 122 result lines, 444 passed · 0 failed · 5 ignored (+3 stand checks, +1 ignored child).
**Why:** v010-02's witness landed as a new stand file. The counts are measured at the chunk's `ci-leg.sh fast` and `run stand` runs.
**Ref:** .andromeda/runs/2026-10-06T10-55-07-wrap/

## 2026-10-06-accessibility-tree-identity — two new test files; stand and workspace re-counts
**Section:** §1 → tests/blitz-tests · §3 → Agent-run contract → Proof · §9 → Local baseline · citations re-pointed
**Change:**
- §1: the blitz-tests coverage list gains `stand_accessibility_ids.rs` (the stable id carried as AccessKit `author_id` on every element node and no other, the 15 controls' roles and names, the ids after a re-render, the Tab order unchanged) and `accessibility_names.rs` (accessible names from `aria-label` and `<label>` association; a plain document's tree carries no `author_id`).
- §3 Proof: re-counted `run stand` 25 `ok` stand events (+5 `stand_accessibility_ids`, picked up by its `stand_` prefix with no script change); the 20 at id-persistence stays as history.
- §9 Local baseline: re-counted `cargo test --workspace` 454 passed · 0 failed · 5 ignored (+5 `accessibility_names`, +5 `stand_accessibility_ids`, two new result lines); result lines and timings not re-measured.
- 22 citations into the changed files re-pointed by the measured line map.
**Why:** the chunk added the two files, and its `ci-leg.sh fast` runs (at /implement and on its pre-CI commit) and `run stand` measured the counts. The new files ride the workspace test leg, not the a11y leg, which keeps its three files.
**Ref:** .andromeda/runs/2026-10-06T12-55-34-wrap/

## 2026-10-06-founder-rulings — coverage-report upload ratified
**Section:** §4 Unit Test Strategy · §9 CI Integration (the `coverage` job's `coverage-report` upload)
**Change:** no body text changes — the body states the upload (`target/coverage/` on success, kept 7 days) as current truth with no provisional clause; its status is now ratified, no longer PROVISIONAL per the 2026-10-05-ci-gate-legs entry.
**Why:** the founder's own word at the Epoch 2 boundary (the founder, 2026-10-06), superseding the delegate overseer's provisional answer by rule.
**Ref:** .andromeda/runs/2026-10-06T16-16-18-wrap/

## 2026-10-06-snapshot-model — snapshot checks counted; dioxus-native-dom unit tests re-counted
**Section:** §1 Test Scope Summary → Coverage scope (dioxus-native-dom · dioxus-native and stylo_taffy · tests/blitz-tests) · §3 Test Harness Contract → Crate-local test helpers · → Agent-run contract → Proof · §9 CI Integration → Local baseline
**Change:**
- §1 dioxus-native-dom: was "three unit tests"; now 18 in four files (1 document, 2 touch, 6 `element_id`, 9 `snapshot`, the last compiled only under the default `accessibility` feature).
- §1 dioxus-native and stylo_taffy: the search note was "matches only in dioxus_document.rs and events.rs"; now names `element_id.rs` and `snapshot.rs` as added since. "No tests" for both crates stands.
- §1 tests/blitz-tests: the stand coverage sentence gains each lean task's snapshot in both layout modes, citing `stand_snapshot.rs:1`.
- §3 Crate-local test helpers: the `snapshot` unit tests call `resolve(0.0)` through `inner_mut()` after `initial_build`.
- §3 Proof: `run stand` 25 → 33 `ok` stand events (+8 `stand_snapshot`).
- §9 Local baseline: `cargo test --workspace` 454 · 0 · 5 → 471 · 0 · 5 over 125 result lines (+9 unit, +8 stand).
**Why:** the chunk added 17 tests, and the unit-test count had not been updated since the six `element_id` tests landed. Standing practice kept: each count is appended as a dated link, the earlier links left as history.
**Ref:** .andromeda/runs/2026-10-06T19-14-10-wrap/

## 2026-10-06-id-stability-across-code-edits — counts and coverage after the anchored tier and the actionable-key check
**Section:** §1 Coverage scope → dioxus-native-dom · → dioxus-native and stylo_taffy · → tests/blitz-tests · §2 Test levels observed · → Test function naming · §3 Crate-local test helpers · → Agent-run contract → Proof · §5 Boundaries covered · §7 Builders and options · §9 Local baseline
**Change:**
- §1 unit census: was 18 unit tests in four files, six on the stable element id; now 29 in five — thirteen on the id (seven of them on the anchored tier), four on `unkeyed_actionable` in the new `actionable.rs`, nine on the snapshot, one document test, two touch tests. The `#[test]` census names `actionable.rs`.
- §1 stand coverage gains `stand_id_edits` (5 checks: the ids held across code edits) and `stand_actionable_keys` (3 checks: every actionable element keyed; the three other tasks pinned at 2 · 3 · 676); stand files 9 → 11.
- §3 Proof chain, one link: `run stand` 33 → 41 `ok`.
- §9 baseline chain, one link: 471 · 0 · 5 over 125 result lines → 490 · 0 · 5 over 127.
- Ten line citations re-pointed (`dioxus_document.rs`, `element_id.rs`).
**Why:** the chunk's own measured counts; earlier links of both chains stay as written.
**Ref:** .andromeda/runs/2026-10-06T20-55-58-wrap/

## 2026-10-06-snapshot-state-fidelity — state checks counted; the unit-level route for a `disabled` pin recorded
**Section:** §1 Coverage scope → dioxus-native-dom · → tests/blitz-tests · §3 Crate-local test helpers → dioxus-native-dom · §3 Agent-run contract → Proof · §9 Local baseline
**Change:**
- dioxus-native-dom unit tests: was 29 in five files, nine on the snapshot builder; now 33, thirteen on the snapshot builder (a password mask, a present-`disabled` reader pin, a radio, a range input).
- tests/blitz-tests coverage names two new files: `stand_snapshot_state.rs` (per-control state through real input; checked and the password mask on an in-file fixture) and `dioxus_falsy_boolean_attrs.rs` (the 27-name falsy clear on both attribute paths).
- Crate-local helpers: the snapshot unit tests' `build` helper carries the no-op HTML parser, so `dangerous_inner_html` renders nothing at unit level; the `disabled` pin there writes its attributes through `DocumentMutator`, and parsed markup is pinned in blitz-tests.
- Agent-run Proof: a dated link appended — `run stand` 41 → 48 `ok` stand events.
- Local baseline: a dated link appended — 127 result lines, 490 · 0 · 5 → 129 result lines, 504 · 0 · 5.
- 2 line citations re-pointed (`snapshot.rs`).
**Why:** the chunk's own measured counts; earlier links of both chains stay as written. Trap for later chunks: a headless key press is not the same on every platform — the editor's Backspace arm is compiled out on macOS, so a deleting key in a stand check read green on Linux and red on the macOS CI leg; prove a state change with clicks and typed characters.
**Ref:** .andromeda/runs/2026-10-06T22-39-16-wrap/

## 2026-10-06-compact-snapshot-serialization — the text form's tests and the re-counted totals
**Section:** §1 Coverage scope (dioxus-native-dom · dioxus-native and stylo_taffy · tests/blitz-tests) · §3 Crate-local test helpers · §3 Agent-run contract → Proof · §9 Pipeline facts → Local baseline
**Change:**
- dioxus-native-dom unit tests: was 33 in five files, thirteen on the snapshot builder; now 41 in six — fourteen on the snapshot builder (a file input reading the mask and never its path, an upper-case `FILE` included) and a new group of seven on `Snapshot::to_text` (a line's fields, the state tokens and their order, pre-order indent, `Debug`-form quoting with an apostrophe written as it is, shortest-decimal bounds, the empty snapshot, a real document), compiled only under `accessibility`. The list of the crate's files holding tests gains `snapshot_text.rs`.
- tests/blitz-tests coverage names a new stand file, `stand_snapshot_text.rs`: each lean task's screen as the snapshot's text in both layout modes — one line per node, every id on exactly one line, each screen under its in-file ceiling and under `SNAPSHOT_TEXT_BUDGET` (10,000 bytes; the largest reads 2034), the same text across two calls, the two layout modes and a second boot, `focused` on no line at boot and on `back-btn`'s after a Tab press, and a typed password and a file input's path masked on two in-file fixtures.
- Agent-run Proof: a dated link appended — `run stand` 48 → 54 `ok` stand events.
- Local baseline: a dated link appended — 129 result lines, 504 · 0 · 5 → 130 result lines, 518 · 0 · 5.
- 3 line citations re-pointed (`snapshot.rs`) by the chunk's measured line map.
**Why:** the chunk's own measured counts; earlier links of both chains stay as written. Trap for later chunks: a search for `snapshot.rs:` also matches `stand_snapshot.rs:`, a different file with its own line ranges — anchor a citation search on the path.
**Ref:** .andromeda/runs/2026-10-06T23-58-14-wrap/

## 2026-10-07-change-tracking-and-diff — changed-set and diff coverage; counts re-measured
**Section:** §1 Coverage scope → blitz-dom · blitz-dom covered behaviours · dioxus-native-dom · dioxus-native and stylo_taffy · tests/blitz-tests; §3 Agent-run contract → Proof; §9 Local baseline; `file:line` citations in §1 · §2 · §4 · §5 · §7 · §8
**Change:**
- blitz-dom: 32 `#[test]` functions, 16 in document.rs (was 25, 9); covered behaviours gain the changed set — the flag false on a fresh document and true from a tracked write until the drain, a write outside the document and a hover marking nothing, a focus move and a checked change marking, an idle `resolve` marking nothing in either layout mode, a drained id of a dropped node reading `None`.
- dioxus-native-dom: 49 unit tests in seven files (was 41 in six) — eight on `Snapshot::diff` over hand-built snapshots; the crate's test-file list gains `snapshot_diff.rs`.
- tests/blitz-tests: the stand narrative gains the diff check `stand_diff.rs`, the fourteenth stand file — real steps on the four lean tasks and three in-file fixtures, both layout modes, every step held against the file's own reading of the two snapshots.
- Proof: `run stand` 63 `ok` stand events (was 54). Local baseline: 131 result lines, 542 passed · 0 failed · 5 ignored (was 130, 518 · 0 · 5).
- 20 of 20 citations into blitz-dom `document.rs` and `mutator.rs` re-pointed by the measured line map; the document.rs test-module range is now `:2760-3319` for the pre-existing modules and `:3321-3552` for the new one.
**Why:** the chunk added 7 + 8 unit tests and 9 stand checks and moved every cited test range in the two blitz-dom files. The one new path without a test is the shell's `accessibility`-gated refresh, which this host cannot run; its windowed witness is carried on the working route's "Stand a11y assertions".
**Ref:** .andromeda/runs/2026-10-07T01-31-03-wrap/

## 2026-10-07-audit-corrections — bridge tests, the shared stand module; counts re-measured
**Section:** §1 Coverage scope → dioxus-native-dom · dioxus-native and stylo_taffy · tests/blitz-tests; §2 Directory pattern; §3 Crate-local test helpers → dioxus-native-dom · blitz-tests (stand checks) (new bullet); §4 Test file location; §9 Local baseline; `file:line` citations in §1 · §3
**Change:**
- dioxus-native-dom: 55 unit tests in eight files (was 49 in seven) — six on the bridge, in the `cfg(test)` child module `dioxus_document::bridge_tests`, under no feature gate: a refused Dioxus key (empty, or holding `/`) reading a positional segment, `create_head_element`, `mounted` delivery after the build and after a poll, `Document::id`, a handled event's cancel and stop. The list of the crate's test-holding files gains `dioxus_document_tests.rs`.
- Layout conventions (§2, §4): unit tests sit inline, bar one `cfg(test)` module held in a file of its own through `#[path]` (`bridge_tests`); `tests/blitz-tests` holds one test-target file per behavior and one shared module that is no target, `tests/common/mod.rs` (was: one file per behavior, no exception).
- Crate-local helpers: the bridge tests are a child of `dioxus_document` because `DioxusEventHandler`'s two fields are private to it and it has no constructor; they keep the `EventState` they pass, the one way to read `propagation_is_stopped()`. New bullet for the stand checks' shared module: three tables (`controls`, 18 rows naming 15 controls; `INPUT_NAMES`, six; `rendered`, 26 rows), six helpers, six reading checks; `stand_element_ids` keeps its own `boot`.
- Local baseline: 131 result lines, 548 passed · 0 failed · 5 ignored (was 542 · 0 · 5 over 131); no result line added. `run stand` stays at 63.
- 4 of 20 citations into the edited files re-pointed by the measured line map; the `element_id.rs` test module is now `:284-604`.
**Why:** the chunk added six unit tests that kill the seven mutants the Epoch 2 code audit left surviving, and moved the stand checks' restated tables and helpers into one module; it added no stand check and no test target. Trap for later chunks: a sed or grep address that ends at a function's name also selects every item whose name opens with it, and a control built with the same address cannot catch that.
**Ref:** .andromeda/runs/2026-10-07T03-59-09-wrap/

## 2026-10-07-driver-session — the driver session's coverage, a Session lifecycle key, the agent-run premise scoped, re-counts
**Section:** §1 Coverage scope (apps · new escher-driver bullet · tests/blitz-tests) · §2 (Test levels: new Process-lifecycle checks · Directory pattern) · §3 (Crate-local test helpers · Stand log format · Agent-run contract → Invocation · State · Proof) · §3 → Session lifecycle (new key) · §4 (new escher-driver bullet) · §5 (three new boundaries · Observed absent · citation) · §6 Observed absent · §9 Local baseline
**Change:**
- New key `§3 → Session lifecycle` (labels `session-start` · `session-host` · `session-wire` · `session-state` · `session-binary` · `session-checks` · `session-proof`): the surface the session checks drive and what a process check owes.
- Agent-run Invocation: was "The stand is an in-process library boot, so nothing is started, polled or listened on"; now the script itself starts and listens on nothing, and two checks of `run stand` start their own session host and socket. State names their two directories under `target/tmp/`.
- Coverage: escher-driver's 13 unit tests (error 3 · session 2 · wire 8); `host_binary` (2), the stand crate's first integration test — the slice-s04 "no tests" readings kept as the reading at that search; five `stand_session_*` checks (3 · 2 · 2 · 1 · 1, two `#[ignore]` host children); two shared modules, `mod common;` readers six → eleven.
- §5 states what `stand_session_quiet` proves (a host with no log sink writes nothing of the screen) and what it does not (a sink-installing host at `debug` / `trace`), owed by "Sink target allowlist".
- Re-counts appended: `run stand` 63 → 72 ok (ignored 1 → 3); workspace 131 · 548 · 0 · 5 → 140 · 572 · 0 · 7.
- Citations: root `Cargo.toml:23` → `:24`, `:199-200` → `:201-202`.
**Why:** the chunk added the session library, its host binary and their checks. The quiet check's limit is recorded on the founder's ruling to record the host-log finding honestly (the founder, 2026-10-07, relayed verbatim). Traps: an `#[ignore]` host child blocks a bare `--ignored` run; a child's inherited libtest lines are counted by `agent-run.sh` as tests; a sink-installing host at `trace` overruns an undrained pipe.
**Ref:** .andromeda/runs/2026-10-07T06-51-52-wrap/

## 2026-10-07-sink-target-allowlist — the sink-installing host is covered: `host_log`, `telemetry_drop`, re-counts
**Section:** §1 Test Scope Summary → Coverage scope (apps · escher-telemetry · tests/blitz-tests) · §2 Test Strategy → Process-lifecycle checks, Directory pattern · §3 Test Harness Contract → Stand log format · §3 → Session lifecycle · §5 Integration Test Strategy → Session host ↔ lifecycle socket, Session host ↔ what it writes · §9 CI Integration → Local baseline
**Change:**
- Session host ↔ what it writes: was "It does NOT cover a host that installs escher's sink", the check "owed by the route entry"; now seven_guis' `host_log` covers it — the real `escher-session` binary on `crud` at `RUST_LOG=trace`, needles read from the stand booted in the test's own process (at least 15 ids asserted, 16 read; row and label texts), both streams drained by a thread each, `attach` under a 60 s bound and never `start`, exit 0, empty stdout, no state directory, every stderr line stamped, no id and no name on stderr, a failure printing counts only; seen red on the unfixed sink at 16 of 16 ids and 6 of 6 names in 1501 lines. `telemetry_drop` proves the drop for a third-party target under a naming `RUST_LOG` directive.
- Stand log format: the scrub clause restated as the three-outcome target allowlist; both binaries read no id and no name at the default level, `info`, `debug` and `trace` (0 lines, then the one 124-byte install line); "one line per printed event".
- escher-telemetry: unit tests 5 → 10 (9 in `format.rs`, five of them on the drop); its `telemetry_*` integration files four → five.
- seven_guis: integration-test targets one → two, with a shared module `tests/common/mod.rs` that is no target (the apps row and Directory pattern).
- Process-lifecycle checks: the 1.1 MB-at-`trace` figure is dated to before the drop (now one 124-byte line); a capturing check still drains while the host runs.
- Local baseline: 140 · 572 · 0 · 7 → 142 result lines · 579 passed · 0 failed · 8 ignored.
- Session lifecycle → session-proof: `host_log` 1 added, the fork's CI run on the chunk's pre-CI commit its witness on the other platforms.
- Three citations into `host_binary.rs` and two into the sink's source re-pointed.
**Why:** the chunk wrote the check the previous entry recorded as owed, and the drop it proves. Traps: a check that must fail red on a leaking sink cannot use `start` on a piped host — the unfixed host filled the pipe before it answered; a needle list read from the stand can outgrow a hand-kept one (16 against the instrument's 15).
**Kept:** §3 Agent-run Proof gains no link — `run stand` reads 72 · 0 · 3, unmoved.
**Ref:** .andromeda/runs/2026-10-07T08-23-43-wrap/

## 2026-10-07-settle-detection — settle beside pump: the harness's first unit tests, `stand_settle`, re-counts
**Section:** §1 Coverage scope · §2 Harness-driven tests, Directory pattern · §3 blitz-test-harness (Construction, Core, Pump semantics), Crate-local test helpers, Agent-run Proof · §3 → Session lifecycle · §4 What unit tests cover · §5 Harness ↔ event pipeline, Session ↔ held instance · §7 Builders and options · §8 Hand-written fakes and stubs · §9 Local baseline
**Change:**
- §3 Core and Pump semantics state `settle`: pump's pass repeated with poll's answer kept; quiet = poll false, hover node unchanged, no load finished; a load in flight is `NotSettled { busy: Loads }` at once; `Render`, `Loads` or `Layout` after `SETTLE_PASS_LIMIT` (64) passes; no clock read, no time advanced, no load waited on, the changed set neither read nor drained; pump and every input helper unchanged.
- §3 → Session lifecycle: `session-start` gains `act(step) -> Result<Settled, SessionError>` beside `harness()` / `harness_mut()` — not rolled back, in process, nothing of it on the socket; `session-proof` gains `stand_settle` 8 and the fork's CI run on the chunk's pre-CI commit.
- §1 and §4: blitz-test-harness holds its first unit tests, 11 in `settle.rs` (4 counter · 6 loop · 1 messages); escher-driver stays at 13, its message pin now covering `NotSettled`'s three classes; tests/blitz-tests gains `stand_settle` (8, both layout modes).
- §5: a step through `Session::act` returns settled, or `NotSettled(Busy::Render)` at the bound with the instance still readable.
- §7: a fetching `net_provider` is wrapped in the load counter settle reads; a no-op or absent one and a wrapped document carry none.
- §8 Time: was "applied on the next pump"; now applied on the next pass — a pump, an input helper's own pump, or a settle — so a delivery made after an input helper waits for a later pass. §8 Network names `stand_settle`'s file-private `ManualNetProvider`; a new row names the unit tests' scripted `Document` and held-handler `NetProvider`.
- `mod session_common;` readers: four `stand_session_*.rs` checks → five checks, `stand_settle` the fifth (§2, §3 helpers).
- Counts: `run stand` 72 → passed 80 · failed 0 · ignored 3 over 20 files; workspace 142 · 579 · 0 · 8 → 143 result lines · 598 passed · 0 failed · 8 ignored.
- Citations into the harness's and the driver's edited files re-pointed.
**Why:** the chunk added the settled step and its checks. Traps: a delivered tick needs a pass of the check's own only when it follows the input helper — the plan predicted the Timer step green before the fix and it measured red in that order; the changed set is non-empty on a freshly booted harness, so a check reading the flag drains first.
**Kept:** `has_changes` false on a fresh blitz-dom document (§1) — a bare document, not a booted harness.
**Ref:** .andromeda/runs/2026-10-07T11-06-31-wrap/

## 2026-10-07-command-and-refusal-schema — escher-driver's unit tests 13 → 25: the schema's pins, the workspace re-count
**Section:** §1 Test Scope Summary → escher-driver · §4 Unit Test Strategy → What unit tests cover → escher-driver · §9 CI Integration → Local baseline · §3 → Session lifecycle (the `session-proof` label)
**Change:**
- §1: was "13 inline unit tests in three files"; now 25 in six — `refusal.rs` 3, `schema.rs` 4 and `command.rs` 5 beside `error.rs` 3, `session.rs` 2 and `wire.rs` 8, each new test named by what it pins.
- §4: the row adds the three pins: the closed cause set and a refusal's printed form; the verb table, its lookup, shapes, kinds, bounds and key names; `validate` over admitted calls, unknown verbs, one malformed witness per rule per kind, the first-failure order, and table and `Command` in agreement. Every table is stated in its test with its row count asserted; an assertion over a call carries a row index only; four mutation controls each turned its named test red. No test drives the schema over the socket or through a `Session`.
- §9: the re-count chain gains a link — 143 result lines, 610 passed · 0 failed · 8 ignored (+12 `escher-driver` unit tests in the crate's existing result line); was 598 at the previous link, which stays as history.
- §3 → Session lifecycle, `session-proof`: was "escher-driver unit tests 13"; now the lifecycle's 13 of the crate's 25, the other 12 named as no part of that contract.
**Why:** the chunk added twelve unit tests and no other check. Trap for later chunks: the crate's count is stated in two word orders ("13 unit tests", "unit tests 13") — a count site is found by both.
**Kept:** `run stand` is unmoved (no `stand_*` check added) and takes no link.
**Ref:** .andromeda/runs/2026-10-07T12-34-00-wrap/

## 2026-10-07-act-by-id — the driver's calls are covered: six `stand_act_*` files, the stand's time step, one recorded gap
**Section:** §1 Coverage scope (the apps, escher-driver and tests/blitz-tests rows) · §2 Directory pattern · Process-lifecycle checks · §3 Harness (Construction, Core, Input helpers) · Crate-local test helpers · Agent-run contract → Proof · §3 → Session lifecycle (`session-start`, `session-state`, `session-binary`, `session-proof`) · §4 escher-driver · §5 Session ↔ held instance · §8 Time (seven_guis timer) · §9 Local baseline
**Change:**
- Coverage: `stand_act_ids` 2 · `stand_act_diff` 6 · `stand_act_timer` 4 · `stand_act_refused` 2 · `stand_act_keys` 2 · `stand_act_range` 1 — 17 tests, `Session::run` in each, both layout modes; what each proves is stated in §1 and §5. escher-driver holds 26 unit tests in seven files (was 25 in six; `execute.rs` 1). seven_guis' library holds its first unit test, the five-row tick table.
- Recorded gap: no standing check boots `escher-session` on `timer`, the one task whose session carries a time step (`host_binary` boots `counter`, `host_log` `crud`); that branch is booted by hand only. `apple_keybinding` has one standing check, on the macOS CI leg only.
- §4: was "the schema is held in process, so no test drives it … through a `Session`"; now `Session::run` executes it and the six files drive it through a `Session`; five mutation controls are recorded.
- Session lifecycle: `session-start` states `run` and `with_time`; `session-binary` the timer's step and the gap; `session-proof` the six files, the crate's 26 and CI run 37633611745 on `f8eb42c8`. No label added or renamed.
- §8 Time: the driver moves the Timer by `advance` through `stand::timer_step` — whole ticks of 100 ms, the remainder dropped per call, `advanced_ms` the time delivered to the app and never more than asked, the harness clock untouched.
- Counts: `run stand` 97 · 0 · 3 over 26 files (was 80 · 0 · 3 over 20); workspace 149 result lines, 629 · 0 · 8 (was 143, 610); `mod common;` read by 14 stand checks (was eleven), `mod session_common;` by eleven (was five).
- Harness: `Key` and `Modifiers` re-exported; `apple_keybinding` listed. Ten citations into six edited files re-pointed.
**Why:** the chunk built the executor and its checks. Rules it sets: a driver check's failure message names a layout mode, a task and an index, never an id or what a screen reads; no deleting key in any check but `stand_act_keys`' one; `stand_act_range` pins a measured reading, so a range interaction model turns it red by design. The meaning of `advanced_ms` is the overseer's technical answer (2026-10-07).
**Kept:** "no command can type into the host's instance yet" in §3 and §5 — true as worded; the agent-run contract.
**Ref:** .andromeda/runs/2026-10-07T14-22-35-wrap/
