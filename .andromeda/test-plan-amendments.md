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

## 2026-10-07-refusal-detection — nine driver-action files, 27 tests; the record's unit test; the baselines re-counted
**Section:** §1 (escher-driver · tests/blitz-tests) · §2 (Process-lifecycle checks · Directory pattern) · §3 (Input helpers · blitz-tests stand checks · session checks · Agent-run Proof) · §3 → Session lifecycle (session-start · session-state · session-proof) · §4 escher-driver · §5 Session ↔ held instance · §9 Local baseline
**Change:**
- escher-driver: 27 unit tests (was 26) — `session.rs` 3, the third pinning the record of ids on a count of ids; six verbs (was five); the command tables read 28 admitted and 35 malformed rows (was 26, 34).
- Nine `stand_act_*` files with 27 tests (was six, 17): `stand_act_refused` 3, and new `stand_act_disabled` 2 · `stand_act_obstructed` 3 · `stand_act_scroll` 4; `stand_act_ids` reads 70 nodes accepted and 7 refused, and of the 15 controls 8 changing, 4 accepted with an empty diff, 3 refused (was accepted for all 77; 8 and 7). New engine check `scroll_into_view_nested` 7. Nine further mutation controls.
- `session_common`: `scroll`, `refused`, `refused_unchanged`, `Acted.in_view`; 14 readers (was 11); `mod common;` still 14. Input helper `scroll_into_view`, with no check of its own.
- Session lifecycle key: the lookup answers `stale` or `not-found`, the order `disabled` · `off-screen` · `covered`, `Outcome::Scrolled`; CI run 37673662374 on `983d8973` is the witness of the new files on macOS and windows.
- §5: any of the eight causes refused (was four named); `scroll` among the acting verbs. Re-counts appended: `run stand` 107 passed over 29 files; the workspace 647 passed over 153 result lines.
**Why:** the chunk's checks. Each figure is from the run's own listing; the reader counts are from the wrap's own grep.
**Kept:** the dated re-counts of earlier chunks stand as history. `stand_act_scroll` reads the list's box from its `bounds` before the scroll, since a scrolled box's own `bounds` read shifted.
**Ref:** .andromeda/runs/2026-10-07T20-27-47-wrap/

## 2026-10-07-driver-command-spans — `stand_act_spans`, the span line in the stand log format, every re-count; engine features by runner
**Section:** §1 (escher-telemetry, escher-driver, tests/blitz-tests) · §2 Directory pattern · §3 → Crate-local test helpers, Stand log format, Agent-run contract → Proof, Session lifecycle (key file: `session-start`, `session-proof`) · §4 (escher-driver) · §5 Session ↔ held instance, Session host ↔ what it writes · §9 Pipeline facts (new bullet), Local baseline
**Change:**
- Counts: escher-telemetry 15 unit tests, 14 in `format.rs` (was 10, 9; +5 on the closed span); escher-driver 29, `execute.rs` 3 (was 27, 1; +2 on the span's fields and busy words); `stand_act_*` 10 files (was 9), `stand_act_spans` 2 tests and 2 ignored children; `mod common;` 15 readers and `mod session_common;` 15 (was 14 each), four driver-action checks declare both (was three); `run stand` 109 passed · 0 failed · 5 ignored over 30 files (was 107 · 0 · 3 over 29); workspace 154 result lines, 656 passed · 0 failed · 10 ignored (was 153, 647, 8). Earlier re-counts stay as history.
- Stand log format: two record classes; the closed span's line — the event prefix, `span={name}`, the span's fields, `message="close"`, `time.busy`, `time.idle` — is PROVISIONAL by pointer to obs-plan §6, which owns the mark. Neither seven_guis binary writes one today.
- Typed text: measured in process only (`stand_act_spans`: 0 occurrences, 0 of 36 ids, 0 of 8 names per layout mode); still not measured on either binary's stderr.
- §9, new bullet "Engine features by runner": `cargo test --workspace` builds blitz-dom with its `tracing` call sites compiled in (blitz's default `tracing` → blitz-shell and blitz-html → blitz-dom); `cargo test -p blitz-tests` and a `-p seven_guis` build do not. The seven_guis binaries' stderr by level under a workspace build is not measured.
- Key file: the proof row names ten files (29) and the crate's other 16 unit tests; CI witness 37694873705 on `44ad3887` added. Citations re-pointed.
**Why:** the same check file read green per package and red in the workspace leg. A check that reads a sink's capture counts lines by target, never in total; a green per-package run is not the workspace leg's green.
**Ref:** .andromeda/runs/2026-10-07T22-32-46-wrap/

## 2026-10-09-founder-rulings — the stand log format's second record class reads ratified
**Section:** §3 → Stand log format
**Change:** the parenthesis on the closed span's line read "the second class is PROVISIONAL, as obs-plan §6 → Log format marks it: a boundary widening on the operator's answer, for the founder's batch at the Epoch 4 boundary"; now "the second class is a boundary widening ratified by the founder, 2026-10-09, as obs-plan §6 → Log format states". The rest of the parenthesis — neither binary runs a driver command, so neither writes a span line today — is unchanged.
**Why:** the clause restates obs-plan §6, which the founder's ruling changed (the founder, 2026-10-09, relayed verbatim by the overseer as his chosen option label; the English wording of the ruling is the overseer's).
**Ref:** .andromeda/runs/2026-10-09T19-17-00-wrap/

## 2026-10-09-audit-corrections-agent-surfaces — the install script's tests, the install invariant, the bounded install in §9
**Section:** §1 Test Scope Summary → CI Python scripts · §4 Unit Test Strategy → CI workflows and leg script · §9 CI Integration → Pipeline facts
**Change:**
- §4 CI workflows and leg script: `CiWorkflowTest`'s pins gain `test_o_package_installs_are_bounded` — no step's `run` names `apt-get`, each step calling `apt-install.sh` is that script and package names with no option, the calling jobs are exactly the nine of `INSTALL_JOBS`, and the one action step that installs packages carries an integer `timeout-minutes` above 0. `InstallScriptTest` is new: it runs the script in a session of its own against a stand-in `sudo` and `apt-get` with the real `timeout` — five tests (a healthy install; a failed attempt tried again from `update`; exit 1 after three attempts, never 124 or 137; a stalled call ended at its bound and tried again; nine bad calls each exiting 2 before anything runs). Four mutation controls each turned a named test red and were restored by sha256, and the invariant read red on the unedited workflow (11 failures). The leg count was "64 tests — the 23 of the files above, 14 and 27, CI run 37413576977"; now 70 — the 29 of the files above (25 + 4), 14 and 27 — on the dev host and at CI run 37985678276. The bullet keeps its title.
- §1 CI Python scripts: the row names the 70 tests, the install script's five contract tests and the install invariant, and the PyYAML step as a guard in front of the script.
- §9 Pipeline facts: a new bullet, "Package installs" — nine steps call the script (`libfontconfig1-dev` in eight jobs, `python3-yaml` in `ci-scripts` behind its guard), 3 attempts, a 120 s bound per call with a 10 s kill grace, a 10 s pause, exits 0 · 1 · 2; the matrix action step's `timeout-minutes: 10`; the hang of CI run 37673662374 it answers; the witness CI run 37985678276 (eight jobs ran the script, the guarded call not reached, the android step 5 s); not shown by any run: the bound or the step timeout firing.
**Why:** a package-install step hung two jobs of one run and nothing bounded it. The stand-in tests are the bound's only witness, so §9 says what the green run does not show. The plan's gate note expected nine jobs to run the script; eight did — a step that reads 0 s in the jobs listing did not say which, the job log did.
**Ref:** .andromeda/runs/2026-10-09T20-50-14-wrap/

## 2026-10-09-audit-corrections-agent-surfaces — a textarea's value read on the fixture; the stand and workspace re-counts
**Section:** §1 Test Scope Summary → tests/blitz-tests · §3 Test Harness Contract → Agent-run contract → Proof · §9 CI Integration → Local baseline
**Change:**
- §1 tests/blitz-tests: the in-file fixture's readings were "a checkbox and a radio reading checked after a click and a typed password reading the fixed mask"; they gain a textarea named by `aria-label` reading empty before typing and its typed text back as its value, exactly it reading focused after the click — `a_textarea_reads_back_typed_text`, the eighth test of `stand_snapshot_state`, read red with the `textarea` arm of the snapshot's `value` deleted and green on the restored tree. It is test-plan's first statement of a textarea reading.
- §3 Agent-run contract → Proof: re-counted — `run stand`'s `run.end` reads passed 110 · failed 0 · ignored 5 over the same 30 files (was 109; +1 `stand_snapshot_state`, no script change).
- §9 Local baseline: re-counted — `cargo test --workspace --locked` 154 result lines, 657 passed · 0 failed · 10 ignored (was 656; +1 in the file's existing result line), timings not re-measured.
**Why:** the Epoch 4 code audit left one mutant alive — no test read a textarea's value from a snapshot. The check sits in `stand_snapshot_state` because the audit runs the snapshot unit against a fixed test set that holds that file; a new stand file would sit outside the set the next audit runs.
**Kept:** the earlier re-count clauses, "+7 `stand_snapshot_state`" included, stand as their chunks' readings. The stand's tables in `tests/common/mod.rs` do not gain the textarea — they hold stand controls, and no stand task holds one.
**Ref:** .andromeda/runs/2026-10-09T20-50-14-wrap/

## 2026-10-10-upstream-sync-agent-surfaces — the CI-scripts leg runs 78 tests over six files; `tests/all.rs` is declared and never built, pinned by `BlitzTestsTargetsTest`
**Section:** §1 Test Scope Summary → CI Python scripts, tests/blitz-tests (the opening clause) · §2 Test Strategy → Directory pattern · §4 Unit Test Strategy → CI Python script, CI workflows and leg script (the count clause), blitz-tests targets (new bullet)
**Change:**
- The leg's count: was 70 tests over four files (test_ci_workflows.py 25, test_wpt_diff_to_pr.py 4, test_agent_run.py 14, test_cold_agent.py 27); now 78 over six — 25, 7, 14, 27, upstream's new test_wpt_area_changes.py 4 and our test_blitz_tests_targets.py 1 — on the dev host and at CI#38013740580. §1 states "78 tests in six test files" and names `BlitzTestsTargetsTest`.
- §4 CI Python script: test_wpt_diff_to_pr.py is stated at 7 tests with what the three new ones cover, and test_wpt_area_changes.py's `CompareTest` (4) is named with its four cases.
- §4, new bullet "blitz-tests targets": `BlitzTestsTargetsTest`, one test, asserts `[package]` does not set `autotests` to false and that the manifest holds exactly one `[[test]]` table, `all` at `tests/all.rs` with `test = false`; controlled red on three scratch manifests; it reads the manifest with `tomllib`, so the leg needs Python 3.11 or later — the CI runner's version was not read.
- §2 Directory pattern and §1 tests/blitz-tests: the list of what under `tests/` is no built target was the two shared modules; it gains `tests/all.rs`, upstream's single-binary target, held at `test = false`. 102 `.rs` files, 101 test targets; read at each run as 0 `Running tests/all.rs` lines and no `all` executable among 107.
**Why:** the fork keeps one target per file by the operator's decision of 2026-10-10, by question dialog at the plan, and carries upstream's table switched off so the difference from upstream stays two lines; the check is what turns a later sync red if it takes upstream's two lines. The leg's count moved by upstream's tests and one of ours.
**Kept:** test_ci_workflows.py, test_agent_run.py and test_cold_agent.py are unedited and their counts unmoved.
**Ref:** .andromeda/runs/2026-10-10T01-56-50-wrap/

## 2026-10-10-upstream-sync-agent-surfaces — the test inventory gains upstream's tests: Selection, text-transform, autofocus, WPT variants; two arrivals with no test
**Section:** §1 Test Scope Summary → tests/blitz-tests (the coverage list), blitz-vibey-script, blitz-dom (layout), wpt/runner, blitz-net · §4 Unit Test Strategy → blitz-dom (layout), wpt/runner · §5 Integration Test Strategy → Script ↔ DOM
**Change:**
- tests/blitz-tests coverage gains four upstream checks: `autofocus_attribute` 5, `text_transform` 3, `anonymous_block_percentage_height` 1 and one test of `inline_fragment_rects` over three writing modes; the package's blitz-dom dev-dependency gains the `autofocus` and `text-transform-icu` features.
- blitz-vibey-script: tests/dom.rs was 26 `#[test]` functions, now 27; a third file, tests/selection.rs, holds 15 tests of the script Selection API; §5 names it as the crossing's coverage. Observed absent: a test of the `innerText` / `outerText` getters in the crate (whether a WPT case covers them was not read).
- blitz-dom (layout): was "one inline unit-test module, in list.rs"; now two — list.rs and upstream's `layout/text_transform.rs`, 22 tests; `layout/writing_mode.rs` holds none. §4 states what the 22 check.
- wpt/runner: the unit-test list (8, 3, 2, 1, 1) gains 10 in test_variants.rs, 2 in ref_test.rs and 1 in main.rs; §4 states what they check.
- blitz-net: the "no test" row adds that its `file:` read through `Url::to_file_path` has no test named for it either.
**Why:** every one of these arrived by the merge; none is a cargo test of ours added, removed or edited. The plan is tier 0 with no critical-path list, so the two untested arrivals are recorded as absences, not as failed gates.
**Kept:** upstream's default link rule on `a[href]` has no test named and no test-plan row states it; it is design-system's.
**Ref:** .andromeda/runs/2026-10-10T01-56-50-wrap/

## 2026-10-10-upstream-sync-agent-surfaces — readings on the merged tree: 158 result lines and 719 passed, `run all` 396, a second engine feature split by runner, a 1100 s cold CI run
**Section:** §3 Test Harness Contract → Agent-run contract → Proof · §9 CI Integration → Local baseline, Engine features by runner, Cache
**Change:**
- Local baseline: a re-count clause — `cargo test --workspace --locked` was 154 result lines, 657 passed · 0 failed · 10 ignored; on the merge it reads 158 lines, 719 · 0 · 10: +62 passed and +4 lines, every one upstream's, the workspace's integration-test executables 103 → 107, eight differing rows each attributed to a named upstream file and PR.
- Agent-run Proof: a re-read clause — `run stand` unmoved at passed 110 · failed 0 · ignored 5 over the same 30 files with the script byte-identical; `run all` reads 396 · 0 · 10, the whole package's first stated total.
- Engine features by runner: the workspace build resolves blitz-dom with 16 features and the per-package build with 10, `writing-mode` on in the first only; it selects which of two bodies of the bounds reader `BaseDocument::physical_unrounded_geometry` is compiled, so the same stand check runs over a different reader under each runner; `scroll_into_view_nested` 7, `stand_act_scroll` 4 and `stand_act_obstructed` 3 pass under both.
- Cache: the merge's run, CI#38013740580, 16 of 16 green at its first attempt, took 1100 s wall, a cold run, against 641 s on the chunk start.
**Why:** the merge moved every one of these numbers, and a moved number is restated where the plan states it. The feature split is the standing rule of that bullet made sharper: a green per-package run is not the workspace leg's green, and now the two also differ in the code that reads every snapshot `bounds`.
**Kept:** the earlier re-count clauses stay as their chunks' readings. The per-OS tallies of that CI run — 139 result lines on three jobs, 712 passed and 8 ignored on Windows — are not attributed and are stated in no section: a route entry owns them.
**Ref:** .andromeda/runs/2026-10-10T01-56-50-wrap/

## 2026-10-10-driver-cli — coverage scope after the command line: five seven_guis targets, 50 driver unit tests, eleven driver-action files
**Section:** §1 Test Scope Summary → Coverage scope (apps · escher-driver · tests/blitz-tests)
**Change:**
- apps: was two seven_guis integration-test targets and a shared module read "by both"; now five — `cli_commands` (8 tests), `cli_flow` (2) and `host_timer` (1) joined, all unix only — with `tests/flows/`, a script directory that is no target, and the shared module holding the stream drain, the client runner (`Ran`, `run`, `client`), the answer readers (`keys`, `number`, `cause`) and the `Started` guard. `host_binary` (2) names the host role `serve <task> --session <dir>`, a host that cannot serve ending as a session error (exit 3) and nine argv rows ending as usage errors. `host_log` (1) reads three hosted calls: three command-span lines, 0 occurrences of the typed sentinel beside 0 ids and 0 names, per package and under the workspace build.
- apps: was "no standing check boots `escher-session` on `timer` … booted by hand only"; now `host_timer` boots it and moves it by a hosted `advance` with no sleep (250 ms asked reads 200, 99 reads 0), and `cli_commands` starts all four tasks.
- escher-driver: was 29 inline unit tests in seven files; now 50 in ten — error 3, session 4, wire 12, refusal 3, schema 4, command 7, execute 3, json 7, cli 5, client 2; host.rs holds none. The wire's rows are the `v2` grammar's; the schema's are nine verbs at their levels; `SessionError` is thirteen variants. The driver-action files are eleven.
- tests/blitz-tests: `stand_session_lifecycle` 2 tests and two ignored host children (was 1 and one); `stand_act_obstructed` 5 (was 3), `stand_act_scroll` 6 (was 4), a new `stand_act_filled` 4; the Backspace check is no longer the only deleting path; `stand_act_spans` drives the six instance-level verbs.
**Why:** each count and file list is the chunk's. The per-file unit-test counts were measured at this wrap — the report states the total, 50, and the new modules, not the split.
**Kept:** the per-row figures of command.rs's tests (rows per test) left the body: the chunk changed them and neither the report nor this wrap measured the new ones.
**Ref:** .andromeda/runs/2026-10-10T10-03-00-wrap/

## 2026-10-10-driver-cli — test strategy and harness contract: a command-line test level, three host children, the session-lifecycle keys restated
**Section:** §2 Test Strategy → Test levels observed (Process-lifecycle checks · a new bullet) · Test directory + naming conventions → Directory pattern · §3 Test Harness Contract → blitz-test-harness → Input helpers · Crate-local test helpers (stand checks · session checks) · Stand log format · Agent-run contract → State · Proof · §3 → Session lifecycle (session-start · session-host · session-wire · session-state · session-binary · session-proof)
**Change:**
- Process-lifecycle checks: was two ignored host children serving until stopped; now three — `session_host_with_a_short_expiry` joined — each serving until stopped or idle for its expiry, so a bare `--ignored` run waits out an expiry; a check that waits for an expiry watches the state directory, because an `attach` or a `status` is an answered request and restarts the count; a host the `start` command spawns is neither killed nor reaped by it. The stream-drain citation moved to the shared module.
- A new level: command-line checks and shell flows — the binary run once per command as a client, its streams and exit status read; two `sh` scripts run by the Rust target `cli_flow`, assets of a cargo target, no framework added.
- Directory pattern: `mod common;` read by 17 checks (was 15), `mod session_common;` by 16 (was 15), six of eleven `stand_act_*` declaring both; 103 `.rs` files and 102 targets (was 102 and 101); seven_guis holds five targets and `tests/flows/`.
- Input helpers: an empty-text `type` is a second check on the macOS backward-delete path — read from the code, not from the macOS job's log.
- Stand log format: the session host writes one command-span line per hosted call; typed text is measured on that host; a command's stdout is its answer and nothing else.
- Agent-run: `ss-idle` joins the state directories; `run stand` re-counted at 119 passed · 0 failed · 6 ignored over 31 files.
- Session lifecycle key file: session-wire is `v2` with `call` and the three bounds and states what crosses; session-host states `serve`'s idle expiry, `Hello`'s fourth field, `call` and `AnswerTooLarge`; session-binary is the nine-verb command line with four endings; session-start adds the session-level refusal, `type` replacing and the record's length rule; session-proof carries the new counts and the fork's CI run 38042555355.
**Why:** the harness contract is what an agent drives. session-wire's crossing is ratified by the founder (2026-10-10, relayed by the overseer).
**Ref:** .andromeda/runs/2026-10-10T10-03-00-wrap/

## 2026-10-10-driver-cli — unit, integration, e2e and CI sections: the schema driven over the socket, the command line as a driven surface, the workspace baseline at 760
**Section:** §4 Unit Test Strategy → What unit tests cover → escher-driver · §5 Integration Test Strategy → Boundaries covered (Session ↔ held instance · Session host ↔ session socket · Session host ↔ what it writes) · §6 E2E Test Strategy → Drivers per surface · Observed absent · §9 CI Integration → Pipeline facts (Engine features by runner · Local baseline)
**Change:**
- §4: wire.rs pins the `v2` grammar with `call`, its three answers, the strict escapes and the bounds; schema.rs nine verbs in two levels, six argument kinds and seven field kinds; command.rs that a verb is checked at its own level only; session.rs that the record holds no id longer than a call can name; json.rs, cli.rs and client.rs are new. Was "no test drives it over the socket — nothing but `hello` and `stop` crosses it"; now the schema is driven over the socket by `stand_session_lifecycle` and through the binary by four seven_guis targets, and in process by eleven `stand_act_*` files. Eight mutation controls each read red, then green on the restored tree.
- §5: the bullet "Session host ↔ lifecycle socket" is renamed "Session host ↔ session socket" and gains a hosted call, the grammar and bound refusals, `idle_expiry_s`, the idle expiry and the binary's `start` · `status` · `stop`; Session ↔ held instance gains `type` replacing, the mask in a written form and the two pinned engine readings; what a host writes: was "typed text is not measured on either binary's log", now measured on the session host — three calls, three command-span lines, 0 sentinel occurrences, both builds.
- §6: a new driven surface, the command line — `cli_commands` (8), `cli_flow` (2) with the 8-step and 7-step flows, `host_timer` (1); Observed absent no longer says the binary answers `hello` and `stop` only.
- §9: the session host's stderr by level under the workspace build is measured — one engine line at INFO, none more at `debug` or `trace`; the windowed binary's is still not; `stand_act_scroll` 6 and `stand_act_obstructed` 5 pass under both runners; the workspace leg re-counted at 162 result lines, 760 passed · 0 failed · 11 ignored.
**Why:** the chunk's tests and its measured readings. The fork's CI run on the macOS leg is the only witness of the select-all modifier and of the two shell flows there; no local run reads them.
**Kept:** `run all` was not re-read, and the baseline's timings were not re-measured.
**Ref:** .andromeda/runs/2026-10-10T10-03-00-wrap/

## 2026-10-10-scrolling-box-bounds-and-hit — the two pinned engine readings read fixed, their checks restated
**Section:** §1 Coverage scope → tests/blitz-tests (`stand_act_obstructed`, `stand_act_scroll`) · §3 → blitz-test-harness, Input helpers and Inspection helpers · §5 Boundaries covered → Session ↔ held instance
**Change:**
- §5: was "two engine readings are pinned as they read, not fixed — a scrolled box's `bounds` read shifted … and a control that comes before a scrolling box … reads `covered`"; now both are fixed in the engine and their checks restated — a scrolled box reads the same `bounds` before and after its content is scrolled, the scroll's diff does not name it, a `click` naming it lands on the row at its centre; a button lying where a scrolled-out row extends is clicked, its handler runs and no row's, and no element reads focused after. One reading stays: a hit still reaches content clipped by `contain: paint`.
- §5, the `scroll` clause: was "the list's `bounds` taken before the scroll, since a scrolled box's own `bounds` read shifted by its scroll offset"; now the list's `bounds` are taken once, before the scroll, and a scroll of its content does not move them.
- §1: `stand_act_obstructed` stays 5 and `stand_act_scroll` 6 — the two tests each gained at 2026-10-10-driver-cli are restated to the fixed readings, three renamed, none added or removed.
- §3 Input helpers: was "exercised through the driver's `scroll` only"; now `stand_actionable_keys` calls `scroll_into_view` too, and a raw `click` on an element outside the padding box of a box that clips it no longer lands on it. It still has no check of its own.
- §3 Inspection helpers: gains the measured reading of `layout_rect`, `layout_rect_of` and `center_of` for a scrolled box — its position less its own scroll offset, (40, 50) scrolled by (70, 120) reading (−30, −70) — pinned, not fixed.
**Why:** the bounds reader and the hit walk were fixed in the engine on the founder's ruling of 2026-10-10. A check written with a raw harness click on an element outside its clipping box leaned on the old hit and holds none of the words a sweep for it searches: it is found by running the stand gates.
**Kept:** the harness's three position readers as they read — their fix is owned on the route.
**Ref:** .andromeda/runs/2026-10-10T15-29-49-wrap/

## 2026-10-10-scrolling-box-bounds-and-hit — three engine checks, the re-measured counts, and the first conformance reading
**Section:** §1 Coverage scope → blitz-dom (layout) · §1 Coverage scope → tests/blitz-tests · §2 Test levels observed → WPT conformance · §2 Directory pattern · §9 Pipeline facts → Engine features by runner · §9 Pipeline facts → Local baseline
**Change:**
- §1 tests/blitz-tests: gains three engine checks on parsed pages, each in both layout modes and green under the per-package and the workspace build — `scrolled_box_client_rect` 6, `hit_clipped_at_scrolling_box` 9, `scrolled_box_absolute_position` 1 — with what each case reads, and the gaps: no check reads a box both transformed and scrolled or a scrolled box under a vertical writing mode, and a hit at an image's, a sub-document's or a text input's own box is not measured.
- §1 blitz-dom (layout): the `writing_mode.rs` body of the bounds reader is read by `scrolled_box_client_rect` under the workspace build, on horizontal pages only.
- §2 Directory pattern: 106 `.rs` files, 105 test targets and the unbuilt `all.rs` (was 103 and 102).
- §9 Local baseline: 165 result lines, 776 passed · 0 failed · 11 ignored (was 162 and 760); +16 in the three new result lines.
- §9 Engine features by runner: each of the reader's two bodies now holds a check of ours, its two mutation controls each red under its own build only.
- §2 WPT conformance: the first conformance reading taken on the fork's dev host — `css/cssom-view` at the pinned WPT commit, 966 → 967 subtests passed of 2292 run, one moved fail to pass, none pass to fail; report-only.
**Why:** the two engine fixes each needed a regression check under both builds, because the workspace build and the per-package build compile different bodies of the reader, and a fix in one body alone reads green under the other build. The vertical-writing-mode gap was measured at the wrap, not by the chunk. Not re-measured: "no `all` executable among 107" in §2, a 2026-10-10-upstream-sync-agent-surfaces reading.
**Ref:** .andromeda/runs/2026-10-10T15-29-49-wrap/

## 2026-10-10-snapshot-states — the driver-action census at twelve; counts re-read
**Section:** §1 → Coverage scope (escher-driver · dioxus-native-dom · tests/blitz-tests) · §2 → Directory pattern · §3 → Crate-local test helpers · §3 → Agent-run contract → Proof · §3 → Session lifecycle (`session-proof`) · §4 → What unit tests cover (escher-driver) · §5 → Session ↔ held instance · §6 → Command line · §9 → Local baseline
**Change:**
- Was "the eleven `stand_act_*` files" at seven sites (§1 escher-driver, §2, §3 twice, §4, §5 and the key file); now twelve, `stand_act_states` the twelfth with 3 tests. The key file's total was 37 over eleven files, now 40 over twelve, and its CI witnesses gain run 38072258834 on `2fc32829` (16 of 16 checks green).
- `mod session_common;` was read by 16 checks, now 17; `mod common;` stays at 17, six driver-action checks declaring both.
- §2: was 106 `.rs` files and 105 test targets; now 108 and 107 — `accessibility_states.rs` and `stand_act_states.rs`, auto-discovered with no manifest line. "No `all` executable among 107" is left as it reads, not re-measured.
- §1 dioxus-native-dom: was 55 unit tests — fourteen on the snapshot builder, seven on the text form, eight on the diff; now 60 — seventeen, eight and nine. The two limits of the boolean model are pinned there and at the engine only; no stand screen shows either.
- §1: the largest boot text, Crud, was 2034 bytes, now 2049. §6: the hosted `snapshot` line was 832 · 1,375 · 1,368 · 2,187 bytes, now 832 · 1,393 · 1,368 · 2,202.
- §3 Proof: `run stand` was 119 passed · 6 ignored over 31 files, now 125 · 6 over 32; `run all` not re-read. §9: the workspace leg was 165 result lines and 776 passed, now 167 and 791, 0 failed and 11 ignored both.
- "The file's eighth test" of `stand_snapshot_state` reads "the eighth test the file gained, of eleven".
**Why:** each figure is a measured reading at the chunk's gates or its operator pass.
**Ref:** .andromeda/runs/2026-10-10T17-47-57-wrap/

## 2026-10-10-snapshot-states — what the new and extended checks assert; the adapter reading untested
**Section:** §1 → Coverage scope → tests/blitz-tests · §5 → Session ↔ held instance
**Change:**
- §1 gains `accessibility_states` (4): on a parsed page, in both layout modes, `aria-selected` reads `true` and `false` and nothing else (9 rows), `aria-pressed` its three values and nothing else (10), `aria-invalid` its values and an unrecognized one as true (11), each on a role that supports the attribute and one that does not, row counts asserted; `aria-checked` sets none of the three. It runs in the workspace `test` leg and per package, not in the `a11y` leg.
- §1 states two absences: blitz-dom holds no unit test of the builder's reads, and no check reads the three states as the platform adapter hands them to an assistive technology — no stand binary builds an adapter and the dev host's AT-SPI bus reads inactive. The gap is owned by a route entry.
- §1, `stand_snapshot_state` (8 → 11 tests): exactly the active flight mode reads pressed and the reading swaps with the mode; exactly the selected CRUD row reads selected across a move, a Delete and a filter edit; a date that is no date reads invalid on both date inputs; no other node reads any of the three, and every node's three readings equal its accessibility node's.
- §1 and §5, `stand_act_states` (3): through `Session::run` a flight-mode click returns both mode buttons with their pressed reading, a row click the row selected and the row it leaves not selected, a typed start date the field invalid or not — each returned diff equal to the file's own and equal across the two layout modes.
**Why:** the chunk's acceptance, v010-17, is proved by these files under the per-package and the workspace build.
**Ref:** .andromeda/runs/2026-10-10T17-47-57-wrap/

## 2026-10-10-mcp-surface — the MCP checks, the `tool_call` log line, the counts
**Section:** §1 Test Scope Summary (escher-driver · examples) · §2 Test Strategy → Process-lifecycle checks · Integration suites · §3 Test Harness Contract → Stand log format · §3 → Session lifecycle (`session-start`, `session-binary`, `session-checks`, `session-proof`) · §3 → agent-run counts · §5 → Session host ↔ what it writes · §6 → Drivers per surface (MCP) · §9 → the workspace count
**Change:**
- escher-driver: 50 → 59 unit tests in eleven files (`mcp.rs` 9); `cli.rs`'s reader table 61 → 64 rows.
- seven_guis: five → seven integration targets (`mcp_server` 2, `mcp_log` 1), read through `mod common;` by all seven; the shared module gains `string_at`, `Speaker`, `Spoke`, `REPLY_BOUND`. blitz-tests: `stand_mcp_tools` 2.
- Process-lifecycle: was "such a check is `cfg(unix)`-gated", now a check holding a session over the socket is; the MCP stdio-child checks through `Speaker` are not but `mcp_server`'s comparison, and run on windows (CI#38124028789 attempt 2). Same in `session-checks`.
- Log format (§2, §3): the `mcp` role writes one `tool_call` line per `tools/call`, `tool` and `ending` only, the parent of `command`; 9 / 10 lines over one CRUD flow at `info`. §5: `mcp_log` covers the role at `trace` under both builds.
- §6: a new MCP driver row, and the one live trial's figures (`client_exit 0 · isolated true · tool_calls 7 · end_state reached · protocol_version 2025-11-25 · discover sent`).
- Counts: `run stand` 127 · 0 · 6 over 33 files; workspace 805 · 0 · 11 over 170 result lines; `session-proof` gains the three MCP checks and the crate's 59; `session-binary` the `mcp` role; `session-start` the MCP server as `run`'s third caller.
**Why:** three checks and nine unit tests landed with the MCP server; the `tool_call` line is the founder's ratified span (2026-10-10, relayed by the overseer), bound to obs-plan §3. Standing trap: `tracing`'s callsite interest is process-global — a `with_default` check needs its own recorder per test.
**Ref:** .andromeda/runs/2026-10-11T08-35-17-wrap/

## 2026-10-12-self-description — the self-description's checks and counts
**Section:** §1 (escher-driver · apps/seven_guis · dioxus-native-dom · tests/blitz-tests) · §2 Directory pattern · §3 (Crate-local test helpers; agent-run Proof; Stand log format) · §3 → Session lifecycle (`session-binary`, `session-proof`) · §4 escher-driver · §6 (Command line; MCP) · §9 Local baseline
**Change:**
- escher-driver: was 59 unit tests in eleven files, now 66 in twelve — error.rs 5, schema.rs 6, mcp.rs 10, the new describe.rs 2; json.rs still 7, now pinning the remedy and `differs`. §4: ten verbs in three levels, nine field kinds, thirteen node keys with help, the exit statuses, the remedies and the bound, `help` via `validate_session`, and the chunk's eleven mutation controls.
- dioxus-native-dom: 60 → 61 unit tests; the diff's tests pin `differs`.
- seven_guis: `cli_commands` was 8, now 10 (`help`; the 94/95-byte bound; each cause's `remedy`). `mcp_server` was 2, now 4 (the newest revision; `help` before any `start`; `help` the same bytes on both surfaces). Was "the one CLI comparison", now two, both unix-gated.
- blitz-tests: `stand_mcp_tools` 2 → 4 (remedies; `help` parenting no `command`); `stand_mcp_masked` 1; `stand_diff` gains the Book's `differs` = `[bounds]`. Was 108 `.rs` files / 107 targets — stale by `stand_mcp_tools` since mcp-surface — now 110 / 109, measured at this wrap.
- `session_common` readers: was 17, now 18 (`stand_mcp_tools`); `stand_mcp_masked` declares none.
- Proof: `run stand` 127 → 131 over 34 files. §9: 805 → 821 over 171 result lines.
- Stand log format: a `help` call writes one `tool_call` line, `tool="help"`, with no `command` beneath.
- Session lifecycle: `session-binary` ten verbs, the `help` ending, the error object's `remedy`, statuses from `EXIT_STATUSES`; `session-proof` the new counts and CI#38225228282 on `22fc4553` (16/16) as witness.
**Why:** the chunk's new paths carry unit and integration checks at the tiers §2 requires; the readers' count was disproved by measurement, and the target count was re-measured because it was stale.
**Kept:** the live trial's reading (asked `2025-11-25`, answered `2025-06-18`) stands as that trial measured it.
**Ref:** .andromeda/runs/2026-10-12T08-55-10-wrap/

## 2026-10-12-headless-screenshot — the counts moved and the new checks of the screenshot and the mask
**Section:** §1 Test Scope Summary (apps · blitz-dom · blitz-dom covered behaviours · blitz-paint · blitz-test-harness · escher-driver · tests/blitz-tests) · §2 Harness-driven tests · Pixel tests · Font-dependent tests · Directory pattern · §4 What unit tests cover (blitz-test-harness · escher-driver) · §9 Engine features by runner · Local baseline
**Change:**
- §1: escher-driver was 66 unit tests in twelve files, now 72 in thirteen (image.rs 5, mcp.rs 10 → 11); refusal.rs's causes nine, schema.rs's verbs eleven (seven instance-level), the listing eleven; the thirteen `stand_act_*` files. blitz-dom 32 → 34 (`font_reading_tests`; `layout_as_text_input` read only through the mask's frames). blitz-paint: the mask has no unit test, read by `password_painted_masked` and `stand_act_screenshot`; the click-placed caret is read by no check. blitz-test-harness 11 → 17 (screenshot.rs 6). seven_guis: `cli_commands` 10 → 11, `mcp_server` 4 → 5; `host_log` / `mcp_log` read 0 image bytes. blitz-tests: `stand_act_screenshot` 6, `stand_screenshot_repeat` 1 (+1 child), `password_painted_masked` 4; `stand_mcp_tools` 4 → 5 (eight causes served, a shot parenting its `command`); `stand_act_spans` also reads `fonts-unavailable`.
- §2: the harness paints with no display; frames compared whole (PNG equal to the raster, byte-identical across processes, both builds); a no-font shot refused, never skipped. Directory pattern: was 110 / 109 targets, now 113 / 112; `session_common` readers 18 → 20; `mod common;` readers 17, unchanged.
- §4: screenshot.rs's six tests; schema.rs eleven verbs and ten field kinds, refusal.rs nine causes, image.rs, mcp.rs's image bound and shot form; `cli_commands` 11; three mutation controls.
- §9: was 171 lines / 821 · 0 · 11; now 174 / 849 · 0 · 12. The harness takes blitz-paint without default features; shots equal under both builds; the 0.7 s fade limit owned by a CARRY on "Stand requirement sweep".
**Why:** the chunk's tests as its report counts them; the per-file splits sum to its stated totals.
**Ref:** .andromeda/runs/2026-10-12T20-34-17-wrap/

## 2026-10-12-headless-screenshot — the harness and session contracts gain the shot; §3 → session-lifecycle
**Section:** §3 Harness Core · Crate-local test helpers (stand checks · session checks) · Stand log format · Agent-run contract → Proof · §3 → Session lifecycle (session-start · session-wire · session-binary · session-proof) · §5 Session ↔ held instance · Session host ↔ what it writes · §6 Command line · MCP
**Change:**
- Harness Core: `screenshot` / `screenshot_rect`, `PixelRect`, `Raster`, `ScreenshotError::{FontsUnavailable, EmptyRect}` — at the harness's own time, `VelloCpuImageRenderer`, white composite, clip and `EmptyRect`, no font refused with nothing painted, no print, panic or log.
- Helpers: six of the thirteen driver-action checks read `mod common;` (17 readers); `session_common` gains `screenshot()`, `screenshot_of(id)` and `shot(session, call)`, `act` reading `None` for a shot; was 18 readers, now 20.
- Stand log format: `screenshot` joins `command.verb` and `tool_call.tool`, no field added; a shot's line holds no image byte (0 at `trace`, host and `mcp` role, both builds) — in step with obs-plan §4.
- Proof: `run stand` was 131 / 0 / 6 over 34 files; now 139 / 0 / 7 over 36.
- session-start: `run` takes `screenshot` and returns `Outcome::Shot { width, height, png }` — no diff, nothing changed; id refused as `click`'s, a crop of no pixel `off-screen`, no font `fonts-unavailable`. session-wire: a shot's PNG crosses in band as padded base64 in its answer line, the bound unchanged. session-binary: was ten verbs, six instance-level; now eleven, seven. session-proof: `cli_commands` 11, `mcp_server` 5, `stand_mcp_tools` 5, `stand_screenshot_repeat` 1, thirteen driver-action files 46, the crate 72; CI 38297557417.
- §5: a shot through `Session::run` changes nothing; nine causes; `host_log` / `mcp_log` read no image byte. §6: `cli_commands` 11 (seven instance verbs, the shot's `png` equal in process); `mcp_server` 5 (eleven tools, image then text), `stand_mcp_tools` 5.
**Why:** the in-band image on the wire is the founder's fork 1, ratified by his own word (2026-10-12, by question dialog in the overseer session, relayed verbatim by the overseer; the wrap's answer card confirmed the same way, 2026-10-13); the rest is the chunk's counted tests.
**Ref:** .andromeda/runs/2026-10-12T20-34-17-wrap/

## 2026-10-13-upstream-sync-polish-and-ship — the merge's tests: five upstream targets, the bridge's six unit tests, the CI-scripts and workspace tallies
**Section:** §1 Test Scope Summary (CI Python scripts · dioxus-native-dom · dioxus-native and stylo_taffy · tests/blitz-tests) · §2 Test Strategy → Directory pattern · §3 Test Harness Contract → Crate-local test helpers · §4 Unit Test Strategy (CI Python script · CI workflows and leg script) · §9 CI Integration → Local baseline
**Change:**
- dioxus-native-dom: 61 unit tests in eight files → 67 in nine — upstream's four drop tests in `dioxus_document.rs`'s module and two in a new `mutation_writer::tests`; mutation_writer.rs joins the files holding tests; stylo_taffy still holds none.
- tests/blitz-tests: five upstream checks, each its own target, with the dev-dependency `cursor-icon`: `anonymous_block_restyle` 10, `button_alignment` 16, `checked_attribute` 2, `pointer_leave` 7, `ua_stylesheet_order` 3.
- Directory pattern: 113 files and 112 targets → 118 and 117, `all.rs` gaining five `mod` lines and staying unbuilt; no `all` executable among 128.
- Dioxus integration: the falsy check still names its 27; the bridge clears by upstream's 28-name `is_bool_attr` (`inert` added). Whether a check reads a falsy `inert` is not measured.
- §3 helpers: was "stale a detached node's id by dropping it through `DocumentMutator::remove_node_if_unparented` — a re-render alone only detaches it"; now a re-render drops the node (#1185), and `a_removed_element_reads_no_id` asserts it no longer resolves, reads no id and is in no pair of `element_ids()`.
- CI scripts: 78 → 79 tests, test_wpt_area_changes.py 4 → 5 (nested areas sort by path segment, #1183); §1, §4 and the leg tally agree.
- §9 Local baseline: 174 · 849 · 0 · 12 → 179 result lines · 893 passed · 0 failed · 12 ignored; executables 123 → 128, `all` none of them.
**Why:** every figure re-read on the merged tree at this chunk's runs, the old figures kept as dated history. The reconcile was confirmed by the founder (2026-10-13, by question dialog in the overseer session, relayed verbatim by the overseer as his chosen option label «Подтвердить (Рекомендую)»; the English wording is the overseer's).
**Ref:** .andromeda/runs/2026-10-13T15-50-37-wrap/

## 2026-10-13-upstream-sync-polish-and-ship — the session-lifecycle key file: the founder's word respelled
**Section:** §3 → session-lifecycle (`.andromeda/registries/contracts/test-plan/session-lifecycle.md`)
**Change:** the socket's crossing of a call's id and typed text in and its answer's content out was "ratified by the founder (2026-10-10, relayed by the overseer)"; it now reads "relayed verbatim by the overseer as his chosen option label «Сокет несёт команды»; the English wording is the overseer's", the form of the entry it copies.
**Why:** the key file follows the entry it copies; no claim moved. Folded at this wrap's cascade sweep under playbook rule A, which the founder added at its card (2026-10-13, by question dialog in the overseer session, relayed verbatim by the overseer as his chosen option label «Добавить правило (Рекомендую)»; the English wording is the overseer's).
**Ref:** .andromeda/runs/2026-10-13T15-50-37-wrap/

## 2026-10-13-stand-contrast-harness — the contrast reader, `stand_contrast` and the counts it moved
**Section:** §1 Test Scope Summary (blitz-test-harness · tests/blitz-tests) · §2 Test Strategy (Harness-driven tests · Directory pattern) · §3 Test Harness Contract (Harness Core · Crate-local stand-check helpers · Agent-run Proof) · §4 Unit Test Strategy (blitz-test-harness) · §9 CI Integration (Local baseline)
**Change:**
- §1 and §4 blitz-test-harness: unit tests 17 → 22, five in `contrast.rs` — the WCAG 2.x ratio table (5 rows: 21.0 both ways, 1.0, 4.48, 4.39 within 0.005), the threshold table (5 rows: 3.0 for large text — 24 px, or 14 pt at weight 700 — else 4.5), each row count asserted, and three fixtures (a button's own pair, a translucent ancestor composited, no text reads `None`).
- §1 tests/blitz-tests: `stand_contrast` 2 — 18 `controls(task)` rows at boot read through `Harness::text_contrast` (pass 9 · fail 5 · exempt-inactive 3 · no-text 1, exempt exactly where `enabled == Some(false)`, identical in both modes) and the painted background within 1 per channel on the 17 measured rows; run by the workspace `test` leg and per package, not the `a11y` leg (three files); it gates nothing — SC 1.4.3's assertion is "Stand a11y assertions"'s.
- §2 Harness-driven tests and §3 Harness Core: `text_contrast` in the crate's sixth module `contrast`, `TextContrast` and `contrast_ratio` re-exported; read-only, computed styles only, `color` its one new dependency, no span, no print or log.
- §2 Directory pattern: `mod common;` readers 17 → 18; 118 → 119 `.rs` files and 117 → 118 targets (the new file auto-discovered, no manifest line). §3 stand-check helpers: `stand_contrast` named, 17 → 18 in all.
- §3 Agent-run Proof: `run stand` 139 · 0 · 7 over 36 files → 141 · 0 · 7 over 37, no script change; `run all` not re-read.
- §9 Local baseline: 179 · 893 · 0 · 12 → 180 result lines · 900 passed · 0 failed · 12 ignored (+7: `stand_contrast` 2 in one new line, the harness +5 in its lib line).
**Why:** every figure read at this chunk's gate runs, the old ones kept as dated history; the file and target count read on the tree at this wrap.
**Ref:** .andromeda/runs/2026-10-13T18-00-36-wrap/

## 2026-10-13-range-input-interaction — the range checks and the counts as measured
**Section:** §1 Test Scope Summary (blitz-dom: document, mutator, net, form · node · dioxus-native-dom · tests/blitz-tests) · §2 Directory pattern · §4 What unit tests cover (blitz-dom net, mutator · node) · §9 Local baseline
**Change:**
- §1 blitz-dom:
  - 35 `#[test]` functions, was 34, now with form.rs's `a_range_input_submits_its_current_value`.
  - `node/range.rs` adds 7 inline value-rule tests over stated tables (20 · 9 · 12 · 9 · 8 rows).
- §1 dioxus-native-dom: 68 unit tests, was 67; snapshot-builder tests eighteen, was seventeen. A range input reads its current value (the renamed `a_range_input_reads_its_current_value`), and an uncontrolled stepped range input reads its moved value. Was: "a range input reading its `value` attribute".
- §1 blitz-tests:
  - `stand_act_range` 1 now asserts that the driver moves the slider by click and key (sequences A and B, the diffs as the check's own and equal across modes). Was: a pinned no-op over eleven calls.
  - `stand_act_ids`: 9 controls change at boot, was 8, `timer-duration` the ninth; the other 3, was 4. The Timer's Reset is still read after `advance`.
  - The new engine check `range_input_interaction` 6 runs in both modes, by the `test` leg and per package, not `run stand`.
- §2: 120 `.rs` files and 119 targets, was 119 and 118.
- §4: the range.rs and form.rs unit tests are named.
- §9: re-counted at 181 result lines, 915 · 0 · 12, was 180 · 900 · 0 · 12 (+15).
- §3 is unchanged: `session-lifecycle`'s `stand_act_range` 1 stands.
**Why:** the chunk added the target, the unit tables and the form test, and rewrote `stand_act_range` to the new model. The counts are as measured at its `ci-leg.sh fast` run.
**Ref:** .andromeda/runs/2026-10-13T21-11-52-wrap/

## 2026-10-13-stand-keyboard-harness — `stand_keyboard`, the restated focus pins and the counts as measured
**Section:** §1 Test Scope Summary → Coverage scope (tests/blitz-tests · dioxus-native-dom) · §2 Directory pattern · §3 Crate-local test helpers (blitz-tests) · Input helpers · Agent-run contract → Proof · §5 Session ↔ held instance · §9 Pipeline facts · registry Session lifecycle
**Change:**
- §1 blitz-tests:
  - + `stand_keyboard` 11: the Tab walk with wrap, three focus readings agreeing; Enter and Space as a click on 20 stand rows; the fixture controls; a click focusing a button; `disabled` of any value skipped, the `<div disabled tabindex=0>` limit pinned; the CRUD list's keys; the Apple delete binding on every OS; no `inert`.
  - `stand_act_keys` 2 → 3: `press enter` and `press space` on a focused button count as its click.
  - `stand_act_obstructed`: after the accepted click the clicked button reads `focused` and is the tree's focus. Was: no element reads `focused`.
  - `stand_act_ids`: every accepted control changes the screen at boot, 15 control rows over `controls(task)`. Was 9.
  - `stand_diff`: the Book names seven bounds-only nodes and `flight-book` `[focused, bounds]`. Was eight `[bounds]`.
  - `dioxus_falsy_boolean_attrs`: 28 names, `inert` the 28th. Was 27, the falsy `inert` not measured.
  - `stand_accessibility_ids`: CRUD's Tab order restated, six stops. Was "unchanged".
  - `focusability_updates` +1: a bare `disabled` set after creation removes focusability.
- §1 dioxus-native-dom: the snapshot `disabled` pin reads not focusable as well. Was: "stays focusable".
- §2: `common` readers 18 → 19; blitz-tests 120 files · 119 targets → 121 · 120.
- §3 helpers: `tab_order` and `focused` moved into `common`, 19 readers; `tab_order` off the per-file list.
- §3 Input helpers: `apple_keybinding` exercised on every OS, green on linux, windows and macos. Was: no standing check calls it on Linux or Windows.
- §3 Proof: `run stand` 141 → 153 passed over 38 files.
- §5: `stand_act_obstructed`'s focus reading restated.
- §9: workspace 181 · 915 · 0 · 12 → 182 · 928 · 0 · 12.
- Registry Session lifecycle: `stand_act_keys` 3, the thirteen driver-action files 47 (was 46).
**Why:** this chunk's new check, the pins its engine changes restated (a click focuses a button; `disabled` by presence, both ratified by the founder) and the counts as measured.
**Ref:** .andromeda/runs/2026-10-13T23-23-45-wrap/

## 2026-10-14-stand-a11y-assertions — the three stand checks extended, the a11y leg at six files, counts restated
**Section:** §1 Test Scope Summary (tests/blitz-tests) · §3 Crate-local test helpers → blitz-tests (stand checks), Agent-run contract → Proof · §4 CI workflows and leg script · §9 Pipeline facts → Legs, Local baseline
**Change:**
- §1: `stand_contrast` 5 (was 2), record 14 · 0 · 3 · 1 (was 9 · 5 · 3 · 1), three tests asserting SC 1.4.3 over the control rows, `TEXT_ROWS` 14 and `STATE_ROWS` 19; was "not by the `a11y` leg … records and gates nothing, owned by the route entry". `stand_keyboard` 14 (was 11) with the enabled-state walks, Enter/Space on the later-enabled CRUD buttons, every control reached by Tab. `stand_accessibility_ids` 8 (was 5): interactive-role naming over nine roles (`crud-list` → `People`), the `&dyn Document` tree carrying stable ids only, the refresh's two halves. The adapter gap's owner is a `.andromeda/residuals.md` line (target: next), was "a route entry"; `accessibility_states` is not in the six-file leg.
- §3: the shared module gains `ENABLED_TAB_ORDER` and `REGION_NAMES`; `run stand` 162 · 0 · 7 over 38 files (was 153).
- §4: `LegScriptTest`'s a11y row pins six files.
- §9: the a11y leg runs six files (was three), its local reading 6 · 6 · 4 · 8 · 5 · 14 beside the 6 · 6 · 3 history; workspace 182 · 937 · 0 · 12 (was 928).
**Why:** the founder's F4 «Расширить задание a11y (Рекомендую)», by question dialog in the overseer session, relayed by the overseer as his chosen option label, widened the existing leg; the re-home is his F1 «Пока нет, перенести (Рекомендую)», placed by the operator's P5 word. No runner, framework or harness shape changed.
**Ref:** .andromeda/runs/2026-10-14T19-50-50-wrap/

## 2026-10-14-stand-requirement-sweep — the driver sweep and `WITNESSES`; the non-lean pin at 0; the measured gaps
**Section:** §1 (tests/blitz-tests · blitz-dom (layout) · blitz-paint · apps) · §2 Directory pattern, Process-lifecycle checks · §3 stand- and session-check helpers, Agent-run Proof, Stand log format, Inspection helpers, `§3 → Session lifecycle` · §5 Session ↔ held instance · §6 Command line · §9 Local baseline, Engine features by runner
**Change:**
- §1: `stand_requirement_sweep` 2 (every requirement through `Session::run` alone on each lean task; `WITNESSES`, the requirement-to-check record, 16 rows) and `stand_non_lean_tasks` 3; the non-lean pin 0 · 0 · 0, was 2 · 3 · 676; `stand_act_obstructed` 5 → 6, `range_input_interaction` 6 → 8, `scrolled_box_client_rect` 6 → 7, `hit_clipped_at_scrolling_box` 9 → 13 with their cases, the transformed box's client rect at its layout position owned by a `.andromeda/residuals.md` line; the "no check reads … transformed and scrolled / vertical writing mode, own box not measured" qualifier retired; `password_caret_click` 1 (index 5 vs 2), its fix a residuals line; `cli_commands`' off-Linux 90/91 test, still 11 per OS.
- §2: readers 19 → 21 and 20 → 21; 124 files · 123 targets, was 121 · 120. The workspace host's engine line one → two.
- §3: helper readers 21 and 21; `run stand` re-counted 168 · 0 · 7 over 40 files; the `mcp` role 10 → 11 lines workspace-built and the host 6 / 978 B re-read; `Node::absolute_position`'s fix a residuals line, was "a route entry"; key file: `stand_act_obstructed` 6, the thirteen's sum 47 → 48, the sweep in the proof.
- §5: `covered`'s two measured limits, was one. §6: the 90/91 bound with its macOS witness, CI#38642493451. §9: workspace re-counted 185 · 951 · 0 · 12; the host re-read; the overlay fade's owner a residuals line, was a CARRY on "Stand requirement sweep".
**Why:** measured at this chunk. The owners rest on the founder's C10 and C14 «Отложить на следующую версию (Рекомендую)» and C11 «Замерить сейчас, исправить потом (Рекомендую)», by question dialog in the overseer session, relayed verbatim by the overseer as his chosen option labels, placed by the operator's word on the P5 card; the client-rect owner on the overseer's directive relayed by the operator at this wrap.
**Ref:** .andromeda/runs/2026-10-14T22-00-51-wrap/

## 2026-10-14-quality-gates — the floors and the flakiness budget, the new checks, the counts
**Section:** §1 (CI Python scripts · seven_guis · blitz-dom · escher-driver · tests/blitz-tests) · §4 escher-driver, CI workflows and leg script · §5 Session host · §6 Command line · §8 Real dependencies kept · §9 Legs, Cache, Package installs, Engine features by runner, Per-OS test tallies (new), Local baseline, Coverage · §10 · `§3 → Session lifecycle`
**Change:**
- §10: coverage floors escher-driver 97 % · dioxus-native-dom 83 % · blitz-test-harness 90 % (read 98.34 · 84.25 · 91.69 %), and a zero-failure flakiness budget on the stand checks (20 local rounds 0 failures). 67.06 % beside the first 53.23 % reading. NOT YET MEASURED narrowed to a performance budget (was a quality gate, threshold, flakiness or performance budget).
- §9: thirteen linux jobs; the `stand-repeat` leg; the audit, clippy and coverage legs widened; `stand-repeat` restore-only; ten install steps with the repair (was "dpkg … not handled"). The per-OS tallies of CI#38649787488 attributed. `seven_guis_native`'s stderr by level measured (was "Not measured"). Workspace 186 · 955 · 0 · 12 (was 185 · 951).
- §1 / §4: CI scripts 94 in seven files (was 79 in six), `InstallScriptTest` eight, `CoverageFloorTest` 7. escher-driver 73 (was 72), client.rs's per-read bound. `cli_commands` 12 (was 11) with the closed-stdout check. `font_face_sniffed_source` 2. `stand_act_spans` reads a not-settled call (was "`busy` … unit test only").
- §8: no `document.rs` font test skips — the skip was the one-argument `qual_name!` fixture, not missing system fonts (was "skip with an `eprintln!` … without `system-fonts`").
- Key file: `session-wire` and `session-host` name the per-read bound's pin; `cli_commands` 12.
**Why:** the floors and budget on the founder's «3 крейта, на 1 пункт ниже (Рекомендую)» and «Повторное задание, ноль сбоев (Рекомендую)», relayed verbatim by the overseer as his chosen option labels; the rest measured at this chunk.
**Ref:** .andromeda/runs/2026-10-15T06-51-44-wrap/

## 2026-10-15-cold-agent-test — the cold-agent run pipe on the escher server (§3)
**Section:** §3 Cold-agent run pipe (Invocation · Exit grammar · `run stand` · the server and the check's judge, in place of the stub bullet · Verdict · Events · State · Proof)
**Change:**
- One task, `stand` (was `counter`; `run counter` is a usage error); `RUST_LOG=info` set on the server child only. Exit 3 also for an absent or non-executable `target/debug/escher-session`; the pipe builds nothing.
- `run stand`: the session dir holds `mcp.json` alone, server `escher` (`escher-session mcp`); the client line adds `--model claude-opus-5-5`, `--allowedTools mcp__escher`, `--max-budget-usd 3` (was `mcp__stub`, `1`); `STAND_PROMPT` names no id or slug; the reply's last fenced json block (≤ 65,536 B, a JSON array) becomes `check.json`.
- The stub bullet is retired. In its place: `server.log` counted by span name; the check judged — `valid_step` (the eleven verbs, closed keys), ≤ 400 steps, a malformed check never replayed; `judge_check` replays it in full, then ablated, on two fresh children, 30 s per reply, each step validated again by the server; a failing replay child reads `check_passed` / `check_can_fail` false.
- Verdict: nine fields added, `final_count` retired; `passed` needs all four lean apps in `tasks_met` (an accepted `stop` clears the started app) and `check_passed` and `check_can_fail`; reasons add `no-check` (alone), `check-failed` (alone, nothing replayed), `check-cannot-fail` (after a replay); `isolated` on the eleven `mcp__escher__{verb}` tools and server `escher`; `counts_agree` = span lines vs tool uses AND refused + error vs `is_error`; `wrong_calls` = refused + error + foreign + denials.
- Events: no check text reaches one (a fourth planted marker). State: `server.log` and `check.json` added, `calls.jsonl` and `stub-state.json` retired.
- Proof: the 2026-10-15 live run, fired once — `passed`, 43 tool calls, 0 wrong (`spacer_calls` 0), 0.3197 USD, 48.8 s, `claude` 2.1.296; the 2026-10-06 stub run kept as history.
**Why:** the replayed check is the founder's «JSON-проверка с повтором (Рекомендую)», the one run his «Один запуск, до $3 (Рекомендую)», the one task his «Все четыре в одной сессии» (2026-10-15, relayed verbatim by the overseer as his chosen option labels), the crossing confirmed by his «Подтвердить (Рекомендую)» at this wrap's answer card.
**Ref:** .andromeda/runs/2026-10-15T09-44-57-wrap/

## 2026-10-15-cold-agent-test — the cold-agent contract tests and the CI-scripts tally (§1, §4)
**Section:** §4 What unit tests cover (Cold-agent pipe · CI workflows and leg script, the CI-scripts tally) · §1 Coverage scope (CI Python scripts)
**Change:**
- Cold-agent pipe: test_cold_agent.py 33 tests (was 27); `ColdAgentStubTest` (6) retired with the stub; `ColdAgentPipeTest` 33 (was 21) under a `claude` shim and a fake `target/debug/escher-session` that writes `tool_call` span lines and records its spawns — no live model, credential or built binary; the case list restated (the check's cases, the missing binary, the spacer call, four planted markers, the replay's 3 spawns per run, 1 for a malformed check); the 33 read red against the parent's pipe.
- The CI-scripts leg runs 100 tests over seven files (was 94), test_cold_agent.py's 33; green at CI#38713461945. The 94 reading is kept as history.
**Why:** the stub's deletion and the pipe's swap; the counts as measured by the chunk's gates.
**Ref:** .andromeda/runs/2026-10-15T09-44-57-wrap/

## 2026-10-16-file-splits — the moved unit tests counted by their new files; escher-driver's 73 (§1, §2, §3, §4)
**Section:** §1 Coverage scope (blitz-dom · escher-driver) · §2 Directory pattern · §4 Test file location · §4 What unit tests cover (escher-driver) · §3 → Session lifecycle
**Change:**
- §1 blitz-dom: was «18 in document.rs»; now 18 `document::` tests — 9 in document.rs, 7 in document_changed_set_tests.rs (`document::changed_set_tests`) and 2 in document_font_reading_tests.rs (`document::font_reading_tests`); the 35 total unchanged.
- §1 escher-driver: was «73 inline unit tests», «schema.rs 6», «mcp.rs 11»; now 73 unit tests in thirteen files, 56 inline and 17 in two `#[path]` test files — schema_tests.rs 6 (`schema::tests`), mcp_tests.rs 11 (`mcp::tests`), each in its old file until this chunk.
- §2 Directory pattern and §4 Test file location: was one `cfg(test)` module out of line (`bridge_tests`); now five — `bridge_tests`, `document::changed_set_tests`, `document::font_reading_tests`, `mcp::tests`, `schema::tests` — each declared `#[cfg(test)]` with `#[path]` at its parent, moved out whole with test paths unchanged.
- §4 escher-driver: schema_tests.rs pins the verb table; mcp_tests.rs that `help` is answered by the server.
- §3 → Session lifecycle: was «escher-driver's 72 unit tests (… client.rs 2 … mcp.rs 11 … schema.rs 6 …)»; now 73 (client.rs 3, mcp_tests.rs 11, schema_tests.rs 6), «72 until 2026-10-14-quality-gates» in its history, its basis the 73 and client.rs's third measured at 2026-10-14-quality-gates and again at this chunk.
**Why:** the per-file counts follow the moved tests to their sibling files, and the key file's 72 — standing since 2026-10-14-quality-gates while §1 read 73 — is amended to the measured 73, both on the operator's directive at this wrap's resume (the operator, 2026-10-16).
**Ref:** .andromeda/runs/2026-10-16T17-27-46-wrap/

## 2026-10-16-fast-failing-mutants — three direct tests counted; the mutation tier's recipe (§1, §3, §10)
**Section:** §1 Coverage scope (apps · escher-driver · dioxus-native-dom) · §3 → Session lifecycle · §10 Quality Gates & Coverage Targets
**Change:**
- §1 apps: was «the library's first unit test» alone; now the seven_guis library's second, `tasks::timer::tests::a_delivered_tick_is_taken_once_and_an_empty_wait_parks` (the private `TimerTicks::next()` polled under `Waker::noop()` over a three-row table), `cargo test -p seven_guis --locked --lib` 2 passed.
- §1 escher-driver: was «73 unit tests … 17 in two `#[path]` test files», «mcp_tests.rs 11»; now 74, 18, mcp_tests.rs 12 — `framing::read_request` taking each line with its terminator over a slice and a one-byte `BufReader`, at most count + 1 reads each.
- §1 dioxus-native-dom: was «68 unit tests», «six on the bridge»; now 69 and seven — a text node `WriteMutations::create_text_node` creates mapped to its element id and left on the writer's stack, `mutation_writer.rs` byte-identical to upstream's at `434e0661`.
- §3 → Session lifecycle: was «seven_guis' library unit test 1», «escher-driver's 73 unit tests (… mcp_tests.rs 11 …)»; now 2 and 74 (mcp_tests.rs 12).
- §10: a new Mutation tier bullet — cargo-mutants `--test-tool nextest` under the root `.config/nextest.toml`'s `mutants` profile (`fail-fast = { max-fail = 1, terminate = "immediate" }`, `test-threads = 4`, a `priority = 100` override naming the direct tests, the default profile untouched, selected by `NEXTEST_PROFILE=mutants`), `--baseline=skip`, each unit's mutant test command first read green on the unmutated tree; under plain `cargo test` the six still read `timeout`; dev host only, never in CI, cargo-mutants 27.1.0 and cargo-nextest 0.9.146 host tools no lockfile resolves; 137 mutants over the three files, 0 missed, 0 timeout.
**Why:** the counts are this chunk's measured gates; the recipe is the remedy's owner — cargo-mutants 27.1.0's baseline tests the mutated package alone yet passes every `--cargo-test-arg`, so the baseline is skipped and read directly, and the next audit must run under the profile or the six read `timeout` again.
**Ref:** .andromeda/runs/2026-10-17T07-59-25-wrap/

## 2026-10-17-upstream-sync-engine-debts — the two merged ARIA checks; 127 files and 126 targets; bump leaves the inventory (§1, §2)
**Section:** §1 Test Scope Summary (tests/blitz-tests · apps) · §2 Test Strategy (Directory pattern)
**Change:**
- tests/blitz-tests: gains upstream's `accessibility_label` 10 and `accessibility_description` 3, asserted through test-that's `verify_that!`, each its own target, run by the workspace `test` leg and per package, not by the `a11y` leg's six; and records observed absent a test naming the merged `aria-expanded`, `aria-disabled` or `aria-level` reads.
- apps: `bump` leaves the no-tests inventory.
- Directory pattern: was «124 `.rs` files, 123 test targets»; now 127 and 126, `all.rs` gaining the two `mod` lines and staying unbuilt — the tree before the merge measured 125 and 124, so the earlier figures were one short; the run-time read gains 137 executables with no `all` among them.
**Why:** upstream's #859 · #1013 files, merged by the sync, and the measured count correcting a figure one short; the 127 / 126 on the operator's resume directive. The report credited the three closed-value reads to upstream tests, and the wrap's search found none.
**Ref:** .andromeda/runs/2026-10-17T15-18-19-wrap/

## 2026-10-17-upstream-sync-engine-debts — the tallies after the merge: workspace, per-OS, agent-run (§3, §9)
**Section:** §3 Test Harness Contract (Agent-run contract → Proof) · §9 CI Integration (Per-OS test tallies · the workspace tally history)
**Change:**
- Workspace tally: was 186 · 958 · 0 · 12; now 187 result lines, 971 passed · 0 failed · 12 ignored — +13 passed in the two merged files' new result lines, the bump crate's empty `unittests src/main.rs` line gone.
- Per-OS tallies: was the 2026-10-14 reading (linux 185 · 951 · 0 · 12); now re-read on the final HEAD's green 17/17 run — linux 187 · 971 · 0 · 12, macos and coverage 168 · 968 · 0 · 12, windows 168 · 944 · 0 · 9, the a11y job 6 · 43 · 0 · 0, the stand repeat 141 · 576 · 0 · 21, ios and android build only, each equal to the merge's first green run.
- Agent-run: `run stand` unmoved at 168 · 0 · 7 over 40 files; `run all` 546 · 0 · 12, +13 over 533, with no script change.
**Why:** the merge's two test files and bump's departure, every moved count explained by name.
**Ref:** .andromeda/runs/2026-10-17T15-18-19-wrap/

## 2026-10-17-upstream-sync-engine-debts — the coverage leg excludes `keyboard_compat.rs` by filename (§9, §10)
**Section:** §9 CI Integration (Coverage) · §10 Quality Gates & Coverage Targets (Coverage floors)
**Change:**
- Coverage: was «`cargo llvm-cov --workspace --locked --lcov …`, then `cargo llvm-cov report --workspace --locked`»; now both lines take `--ignore-filename-regex 'keyboard_compat\.rs$'`.
- Coverage floors: the exclusion of dioxus-native-dom's crate-private `keyboard_compat.rs` — upstream's temporary keyboard-types 0.7/0.8 bridge, 564 lines whose 530 arms no test of ours reaches — until upstream retires it; the floors and `coverage_floor.py` unchanged; the merged tree reads dioxus-native-dom 84.25 % (equal to the baseline), escher-driver 98.19 %, blitz-test-harness 91.69 %, `3 of 3` with no `keyboard_compat` record in the lcov; pinned by `test_coverage_leg_reaches_the_floor_on_each_crate`.
**Why:** the operator's answer to P4 fork 3 — a named filename exclusion over lowering the floor or testing the bridge's arms.
**Ref:** .andromeda/runs/2026-10-17T15-18-19-wrap/

## 2026-10-17-upstream-sync-engine-debts — the harness re-exports `NamedKey`; test-that named as the assertion crate (§3)
**Section:** §3 Test Harness Contract (blitz-test-harness → Core · Runners and invocation → Rust tests · Third-party test frameworks)
**Change:**
- Core: was «re-exports `keyboard_types::Key` and `keyboard_types::Modifiers`, the two types `press_with` takes»; now keyboard-types 0.8's `Key`, `Modifiers` and `NamedKey` — the two types `press_with` takes and the named-key enum a 0.8 `Key::Named` holds.
- Third-party test frameworks: beside the slice search, blitz-tests carries test-that 0.6.0 and test-that-macro 0.6.0 as dev-dependencies (0.5.2 until this sync, reviewed by hand at the bump), whose `verify_that!` and `TestResult<()>` the merged checks assert through.
- Rust tests: the merged checks return `TestResult<()>` and assert with `verify_that!` under the same `#[test]` harness.
**Why:** the keyboard-types 0.8 move and the test-that bump, both taken by the sync.
**Ref:** .andromeda/runs/2026-10-17T15-18-19-wrap/
## 2026-10-18-engine-debts — §1 coverage: the five new checks, the fixes' checks restated, the CARRY observed present
**Section:** §1 Test Scope Summary (blitz-paint · escher-driver · dioxus-native-dom · tests/blitz-tests)
**Change:**
- blitz-paint: `password_caret_click` was 1, pinning index 5 and 2 «by the real characters' widths»; now 2 — one index for two values of one length, a double click selecting the whole value.
- escher-driver: the executor covered by fourteen `stand_act_*` files (was thirteen).
- dioxus-native-dom: was 69 unit tests; now 72, three on the toggle's sources and `aria-disabled`; the `disabled` snapshot pin's gloss reads presence on a form control (was «in every document»).
- tests/blitz-tests: `accessibility_states` 4 → 5, `aria-checked` read on the checkable roles alone and neither attribute on a native checkbox or radio; the «observed absent» merged-ARIA clause gives way to `accessibility_merged_states` 3; added `accessibility_active_descendant_unread` 1, `inert_subtree` 4, `transformed_client_rect` 4 and `stand_act_transformed` 2; `hit_clipped_at_scrolling_box` reads a `contain: paint` box stopping the hit, `scrolled_box_absolute_position` a scrolled box where it stands, the transformed box's client rect where its transform moves it — «what stays measured and not fixed» retired; `stand_keyboard`'s `disabled` and `inert` readings restated; the falsy-`inert` clause: a truthy one honoured.
**Why:** the chunk's checks; the merged reads were the CARRY this chunk discharged.
**Kept:** two «observed absent» claims a detector proposed — the password click's shift-click and triple-click branches, and a disabled `summary` or link activating — were returned at Validate, no search behind them; a later chunk's report may carry them.
**Ref:** .andromeda/runs/2026-10-18T04-30-47-wrap/
## 2026-10-18-engine-debts — §2–§10 counts and the harness readings
**Section:** §2 Test Strategy (Directory pattern) · §3 Test Harness Contract (Inspection helpers · Crate-local test helpers · → `session-lifecycle`, its `session-proof` label) · §4 Unit Test Strategy · §5 Integration Test Strategy (Session ↔ held instance) · §9 CI Integration (Legs) · §10 Quality Gates (Flakiness budget)
**Change:**
- blitz-tests: was 127 `.rs` files and 126 targets; now 132 and 131, the five new each its own target with no manifest line.
- `session_common` readers: was 21; now 22. `stand_act_*`: was thirteen; now fourteen, in §2, §3, §4, §5 and the key file, whose total reads 50 (was 48), `stand_act_transformed` 2.
- `stand_*.rs` files run by `stand-repeat`: was 40; now 41.
- Inspection helpers: `layout_rect`, `layout_rect_of` and `center_of` read a scrolled box where it stands (was «less its own scroll offset … (−30, −70) … C14»).
- Session ↔ held instance: `covered`'s two measured readings are the no-area and the one-axis ones (was `contain: paint` and the one-axis); `stand_act_transformed` added.
**Why:** the chunk's new files and F4's fix (PROVISIONAL, delegate operator); no harness verb, status shape or log format moved. The §9 per-OS tallies were not read by the report and stand as dated.
**Ref:** .andromeda/runs/2026-10-18T04-30-47-wrap/
## 2026-10-18-upstream-sync-determinism-and-traces — §1, §8, §9: upstream's cursor test counted, the tallies re-read on CI#39104364530
**Section:** §1 (blitz-dom) · §8 (the font-metric `hover_state_tests` line) · §9 (Per-OS test tallies · Local baseline)
**Change:**
- §1 blitz-dom: was 35 `#[test]` functions, 18 `document::`, 9 in document.rs; now 36, 19, 10 — upstream's `hovering_below_content_reports_default_cursor`.
- §8: the two `hover_state_tests` gain a third, upstream's, on the same two-argument `qual_name!` fixture, no precondition and no skip; `3 passed`.
- §9 Per-OS: re-read on CI#39104364530 (`275950dc0`, green 17/17) — linux 192 · 991 · 0 · 12; macos and coverage 173 · 988 · 0 · 12; windows 173 · 964 · 0 · 9; a11y 6 · 43 · 0 · 0; stand repeat 144 · 582 · 0 · 21; was CI#39056637407's 187 · 971 / 168 · 968 / 168 · 944 / 6 · 43 / 141 · 576. The skipped reading, CI#39100630123 on `fb842671` (2026-10-18-engine-debts' final HEAD), is named in the body: engine-debts' +5 lines · +19 passed (stand repeat +3 · +6), this chunk's +1 passed on the four jobs that run blitz-dom's lib.
- §9 Local baseline: re-counted 192 result lines, 991 · 0 · 12, from 187 · 971 — engine-debts' +5 · +19 (its 192 · 990 never written) and this chunk's +1 passed, no new result line.
**Why:** the operator's direction (the operator, 2026-10-18): the tallies move from the masters' recorded reading straight to CI#39104364530, the skipped engine-debts reading named. Rule for later syncs: a wrap that does not re-read the tallies leaves the next to attribute the gap.
**Ref:** .andromeda/runs/2026-10-18T06-47-20-wrap/
## 2026-10-18-one-virtual-clock — §1, §3, §4, §5, §6, §8: `advance` on every session, eight causes, the harness's clock, PROVISIONAL
**Section:** §1 (escher-driver; `stand_act_spans`; `stand_mcp_tools`; `stand_act_timer`; the seven_guis library tests and `host_timer`) · §3 (Construction; Core) · §3 → session-lifecycle (session-start) · §4 (escher-driver) · §5 (Session ↔ held instance) · §6 (Command line) · §8 (Time)
**Change:**
- Causes: was nine (`time-unavailable` among them), the MCP checks serving eight; now eight, `the_eight_causes_are_named_in_order`, `fonts-unavailable` the eighth, seven served over MCP, `cli_commands` no longer provoking `time-unavailable`.
- `advance`: was refused `time-unavailable` without a step, `advanced_ms` the lesser of the step's return and `ms` (50 → 0, 250 → 200), the remainder dropped per call and the harness clock unmoved; now accepted on every session, moving the session's clock by exactly `ms` inside one settled step, then handing `ms` to the step, `advanced_ms` exactly `ms`; `timer_step` returns nothing and carries its remainder (250 then 250 → 2 then 3 ticks); `host_timer` 250 reads 250.
- `stand_act_timer`: the no-step sessions (Counter, FlightBooker, Crud) accept `advance` and settle; the animated fixture's step is handed exactly `ms`, the clock moving by exactly `ms`; `stand_act_spans`' CRUD `advance` row reads an accepted line; `stand_mcp_tools` reads the accepted `advance`'s `tool_call` ending `accepted` with one `command` child.
- seven_guis' library tests: was `the_timer_step_maps_milliseconds_onto_whole_ticks`, 2 passed; now `the_timer_step_carries_milliseconds_onto_whole_ticks` and `the_timer_step_takes_the_largest_advance`, 3 passed.
- §3 Core gains `advance_time`; the harness's time is a `VirtualClock` its documents read — `time`, `pump`, `tick` (a negative or non-finite `dt` moving nothing) and `advance_time`, never moved by `screenshot`, `screenshot_rect`, `text_contrast` or `settle`.
- §5: `stand_virtual_clock`'s 800 ms sleep is the one recorded sleep in a session check.
The delegate-answered facts PROVISIONAL.
**Why:** the founder's forks F1 B, F2 A and F2b A, answered by the operator as the founder's delegate (delegate the operator, 2026-10-18), PROVISIONAL, for the founder's epoch-boundary batch (the operator, 2026-10-18, at the wrap resume).
**Ref:** .andromeda/runs/2026-10-18T08-45-11-wrap/
## 2026-10-18-one-virtual-clock — §1, §2, §3, §4, §7, §9, §10: the clock checks counted, the tallies re-read on CI#39111579925
**Section:** §1 (blitz-dom (clock); tests/blitz-tests) · §2 (Directory pattern) · §3 (Crate-local test helpers) · §3 → session-lifecycle (session-proof) · §4 (blitz-dom (clock)) · §7 (Builders and options) · §9 (Engine features by runner; Per-OS test tallies; Local baseline; Legs) · §10 (Flakiness budget)
**Change:**
- New: blitz-dom's `clock.rs` holds 3 unit tests (a virtual clock's sum on every clone, its saturation, a wall clock never reading less); blitz-tests' `virtual_clock_engine` 4 and `stand_virtual_clock` 2, in both layout modes; session-proof gains `stand_virtual_clock`; §7: `into_config` puts the harness's clock in `DocumentConfig.clock`, a wrapped document keeping its own.
- Counts: blitz-tests was 132 `.rs` files and 131 targets, now 134 and 133; `session_common` readers was 22, now 23; the stand-repeat files was 41, now 42; seven_guis' library tests was 2, now 3.
- §9 Engine features: was «the `screenshot` verb's help states the limit … fades by wall time — no check shoots inside that window, … owned by a `.andromeda/residuals.md` line»; now the fade reads the document's clock, a shot after a scroll repeating until `advance` moves it, read by `stand_virtual_clock` across an 800 ms sleep (PROVISIONAL).
- §9 Per-OS: re-read on CI#39111579925 (`87f12079`, green 17/17) — linux 194 · 1001 · 0 · 12; macos and coverage 175 · 998 · 0 · 12; windows 175 · 974 · 0 · 9; a11y 6 · 43 · 0 · 0; stand repeat 147 · 588 · 0 · 21; was CI#39104364530's 192 · 991 / 173 · 988 / 173 · 964 / 6 · 43 / 144 · 582.
- §9 Local baseline: was 192 · 991 · 0 · 12; now 194 · 1001 · 0 · 12.
**Why:** the chunk's new module and checks; the per-OS tallies read at reconcile on the operator's direction (the operator, 2026-10-18, at the wrap resume), beside the local `fast` and `stand-repeat` readings; the counts measured at this wrap and at implement.
**Ref:** .andromeda/runs/2026-10-18T08-45-11-wrap/
## 2026-10-18-engine-debts — the PROVISIONAL batch ratified by the founder (Epoch 2 close): two batches, 13 marks and the session-lifecycle key
**Section:** §1 Test Scope Summary · §3 Test Harness Contract → session-lifecycle (the key file) · §4 Unit Test Strategy · §5 Integration Test Strategy · §8 Mocking & Stubbing Discipline · §9 CI Integration
**Change:** every PROVISIONAL mark in the body and in `registries/contracts/test-plan/session-lifecycle.md` now reads `ratified by the founder (2026-10-18)` (inside parentheses `ratified by the founder, 2026-10-18`), the clause each one qualifies unchanged:
- engine-debts, 2 marks: `password_caret_click`'s caret by the bullets and a double click's whole value; `scrolled_box_absolute_position`'s reading;
- one-virtual-clock, 11 body marks and 2 key-file marks: `advanced_ms` exactly the `ms` asked, the eight causes in order (`the_eight_causes_are_named_in_order`), the clock moved by exactly `ms` on every session, seven causes served through MCP, `stand_virtual_clock`'s repeat and double click, the stand's time step returning nothing, the harness's time never moved by `screenshot` · `screenshot_rect` · `text_contrast` · `settle`, the shot after a scroll repeating until `advance` moves the fade.
Partly retires the PROVISIONAL clause of the earlier 2026-10-18-engine-debts and 2026-10-18-one-virtual-clock entries here; each one's other claims stand. The counts, tallies and measured readings are unchanged.
**Why:** the Provisional discharge at the epoch boundary, on the founder's own word: «Утвердить всё (Рекомендую)» on the engine-debts batch and on one-virtual-clock F1 B · F2 A · F2b A · F3 A (the founder, 2026-10-18, by question dialog in the overseer session, relayed by the overseer with his chosen labels verbatim), discharged at this wrap on the operator's direction (the operator, 2026-10-18, at the gated wrap's resume).
**Ref:** .andromeda/runs/2026-10-18T20-46-02-wrap/
## 2026-10-19-session-recording — the session lifecycle contract gains the recording; wire v3
**Section:** §3 → Session lifecycle (`.andromeda/registries/contracts/test-plan/session-lifecycle.md`: session-start, session-host, session-wire, session-binary, session-checks, session-proof)
**Change:**
- session-start: `Session::recording_into` (unix only) creates the directory `0700` or refuses `RecordDirNotPrivate`, then `check-{n}.json` `0600`; `run` records each answered instance call (`tool` · `arguments` in schema order · `masked` · `accepted` · `digest`), a masked or unresolved `type` as `MASKED_STAND_IN`; past 400 records or 1,048,576 bytes, or on a failed write, the recording ends and `recording_failed()` reads true once; `Check::from_bytes` reads strictly; `run`'s signature and span unchanged.
- session-host: the edges gain `RecordDirNotPrivate` and `RecordingFailed` — fifteen session errors.
- session-wire: was version `v2`; now `v3` on every request, with the fixed `ok v3 recording-failed` reply.
- session-binary: `start <task> [--record <dir>]`, the host role `[--record <dir>]`, the `mcp` role alone or `mcp --record <dir>` (was alone); a stray `--record` usage (status 2) after `validate`, a refused directory status 3, the usage line naming it.
- session-checks: a check that records is `cfg(unix)`-gated item by item beside an off-unix `Unsupported` check the windows job alone runs.
- session-proof: `stand_session_recording` 3 and seven_guis `recording` 3 (+1 off-unix); `host_log` and `mcp_log` 2 each (were 1); escher-driver's unit tests 87 (were 74: recording.rs 9, session.rs 4 → 6, mcp_tests.rs 12 → 14, three off-unix besides).
**Why:** this chunk's own facts (the playbook's «Accurate this-chunk addition»); the `--record` argument and the kept check are the boundary widening ratified by the founder — F1 A and F2 B as his (the founder, 2026-10-19, by question dialog in the overseer session, relayed by the operator), the card confirmed «Подтвердить все 25 (Рекомендую)» (the founder, 2026-10-19, by question dialog in the overseer session, relayed by the overseer); the reply word sits inside the ratified carrier.
**Ref:** .andromeda/runs/2026-10-19T05-20-24-wrap/
## 2026-10-19-session-recording — the recording's tests, the counts they move and the stand-repeat list
**Section:** §1 Test Scope Summary (apps, blitz-dom (clock), escher-driver, dioxus-native-dom, tests/blitz-tests) · §2 → Directory pattern · §3 → Crate-local test helpers (session checks), Stand log format · §4 → escher-driver · §5 → Session host ↔ what it writes · §6 → Command line, MCP · §9 → Legs · §10 → Flakiness budget
**Change:**
- §1 apps: seven_guis holds eight integration-test targets (was seven) with `tests/recording.rs` (3, unix only, +1 off-unix), `mod common;` read by all eight; `host_log` and `mcp_log` 2 tests each (were 1); the host role `[--record <dir>]`.
- §1 escher-driver: was 74 unit tests in thirteen files; now 87 in fourteen, 67 inline and 20 in two `#[path]` files, three off-unix besides — error.rs over fifteen variants (was thirteen), session.rs 6 (was 4), wire.rs's `v3` grammar (was `v2`) with `recording-failed`, mcp_tests.rs 14 (was 12), recording.rs 9, describe.rs's count row `[11, 13, 12, 3, 8, 4, 15]`.
- §1 dioxus-native-dom: was 72 unit tests; now 73 (`is_masked_input`'s seven rows). §1 blitz-dom (clock): pan and the fling among the reads, and `iframe`'s carried-clock test. §1 tests/blitz-tests: `stand_session_recording` 3 among the session checks; `virtual_clock_engine` 5 (was 4).
- §2: was 134 `.rs` files and 133 targets; now 135 and 134; `session_common` read by 24 checks (was 23). §3 helpers: `replay`, `kept` and the unix-only `mode` (24 readers). §3 Stand log format: the host role `[--record <dir>]`, spans the same with a recording held.
- §4 escher-driver: wire.rs's grammar at `v3` with `ok v3 recording-failed` (was `v2`); error.rs over the fifteen errors; the recorder's, the reader's and the recording-failed unit tests.
- §5: the recorded `host_log` / `mcp_log` runs read 0 recording needles, span counts unchanged. §6: `start --record` and `cli_commands`' usage line; `mcp_log` 2 and `recording`'s `mcp --record` runs.
- §9: stand-repeat runs 43 `stand_*` files (was 42) and seven_guis' `recording` too, both words pinned; §10: 43 files and eight process checks (were 42 and seven).
**Why:** this chunk's own shipped tests and the counts they move (the playbook's «Accurate this-chunk addition»).
**Ref:** .andromeda/runs/2026-10-19T05-20-24-wrap/
## 2026-10-19-recorded-check-replay — the kept answer, the replay and the committed checks as tests
**Section:** §3 → session-start · §3 → session-proof · §1 Test Scope Summary (tests/blitz-tests) · §2 Test Strategy (Recorded-check replay, Directory pattern) · §7 Test Data & Fixtures (tests/blitz-tests/checks)
**Change:**
- session-start: each record `tool` · `arguments` · `masked` · `accepted` · `digest` · `answer` (the kept line as an object, masked, a shot's without `png`), `Check::from_bytes` requiring `answer` as an object; `Check::replay` stated — nothing on another app's session, each step through `Session::run`, stopping at the first differing acceptance or kept answer, `Replay` / `Change` holding no reading or `serde_json` type.
- session-proof and §1: `stand_session_recording` 3 — was «every step's acceptance and digest equal, keeping no screen reading (at rest)»; now each answer kept but a shot's image, replayed through `Check::replay`, its scripts ending `snapshot` · `screenshot`, so the shot path's kept form is covered on linux and macOS (the file unix only); new `stand_replay` 3, not unix-gated (both committed checks hold in both layout modes, red at step 1 / step 3 on a test-local `Counter` variant, another app's session running nothing, no `png` or shot prefix).
- §2: a new test level, recorded-check replay — committed checks under `tests/blitz-tests/checks/`, a data directory that is no target, replayed by `stand_replay` and kept fresh by seven_guis `recording`; §7 lists the two checks (2,929 B and 3,720 B) as fixtures.
**Why:** boundary widenings ratified by the founder — F3 B (the founder's word, relayed verbatim by the operator, 2026-10-19 05:02Z) and F1 A «Коммитить две проверки (Рекомендую)» (the founder, 2026-10-19 08:04Z, by question dialog in the overseer session, relayed by the overseer); the answer card confirmed «Подтвердить все 22 (Рекомендую)» (the founder, by question dialog in the overseer session, relayed by the overseer through the operator). The shot path's coverage is the operator's resume directive, the fix directive's «same-host CLI-vs-MCP test» having been wrong; recording is unix only, so «each OS» reads linux and macOS. Not PROVISIONAL.
**Ref:** .andromeda/runs/2026-10-19T09-49-14-wrap/
## 2026-10-19-recorded-check-replay — test counts, the third flow script and the shot's same-host limit
**Section:** §1 Test Scope Summary (escher-driver; apps: seven_guis) · §2 Test Strategy (Command-line checks and shell flows, Pixel tests, Directory pattern) · §3 Test Harness Contract (blitz-tests session checks) · §3 → session-proof · §4 Unit Test Strategy · §6 E2E Test Strategy (Command line) · §9 CI Integration (Legs → stand-repeat, Engine features by runner) · §10 Quality Gates & Coverage Targets (flakiness budget)
**Change:**
- escher-driver unit tests 87 → 92 in fifteen files, 72 inline, `replay.rs` 5 listed; `recording.rs` rows reworked — refused 35 → 40, read 6 → 8, each record keeping its answer.
- seven_guis `recording` 3 → 4 (+1 off-unix): `the_committed_checks_are_what_the_binary_records` runs `record_checks.sh` through the binary and reads both checks byte-equal to the committed copies; `tests/flows/` and §2's shell flows hold three `sh` scripts (was two).
- `session_common`: `replay(session, check)` removed (its callers on `Check::replay`), `shot_prefix` added, readers 24 → 25; blitz-tests 135 `.rs` / 134 targets → 136 / 135.
- stand-repeat and the flakiness budget: 43 → 44 `stand_*.rs` files (+`stand_replay`), the leg's test pin naming it.
- `stand_screenshot_repeat`'s byte-identity (session-proof, §1, §2 Pixel tests, §9 Engine features) restated as on one host: a shot's PNG bytes differ between linux x86_64 and macOS aarch64 (CI#39225375577), so a check compared byte for byte across OSes — a committed check's digest included — holds no `screenshot` step.
**Why:** this chunk's own counts (the playbook's «Accurate this-chunk addition»); the same-host limit is the operator's resume directive, measured at CI#39225375577 (`Test (macos)`).
**Ref:** .andromeda/runs/2026-10-19T09-49-14-wrap/

## 2026-10-19-upstream-sync-visual-lint-and-real-app-readiness — upstream's two focus targets, the directory, the tallies
**Section:** §1 Test Scope Summary (the blitz-tests bullet) · §2 Test Strategy (Directory pattern) · §3 Test Harness Contract (Proof — the agent-run re-counts) · §9 CI Integration (Per-OS test tallies; the local workspace baseline)
**Change:**
- §1: upstream's `focus_within` 7 and `focus_redraw` 4 are each their own auto-discovered target, run by the workspace `test` leg and `run all`, not by the `a11y` leg's six or `run stand`. The behaviour they pin is PROVISIONAL.
- §2: blitz-tests was 136 `.rs` files / 135 targets, now 138 / 137, with `all.rs` gaining their two `mod` lines and staying unbuilt.
- §3 Proof: `run stand` 178 · 0 · 7 over 44 files, unmoved by this chunk; `run all` 586 · 0 · 12, +11 over 575. The series was last written at 546 (`run stand` 168 over 40), and the chunks between were not re-counted there.
- §9 per-OS: re-read on CI#39312031677 (`79603d1ef`, green 17/17): linux 199 · 1045 · 0 · 12; macos and coverage 180 · 1042; windows 180 · 1000 · 0 · 9; a11y 6 · 43; stand repeat 156 · 624 · 0 · 21. Against CI#39302945352 that is +2 · +11 on the four test jobs (exactly the two targets). The two 2026-10-19 chunks' moves since CI#39111579925 are attributed per target: session-recording +2 · +26 (windows +2 · +7, stand repeat +6 · +24); recorded-check-replay +1 · +7 (windows +1 · +8, stand repeat +3 · +12).
- §9 local baseline: was 194 · 1001, now 199 · 1045 (197 · 1034 before the take).
**Why:** this chunk's own counts (the playbook's «Accurate this-chunk addition»). The report's claim that no master site states the `run all` figure was disproved at Validate: the §3 Proof series stands, and its sweep pattern could not meet it. The two earlier chunks' tallies had not been written, so they are attributed here from the job logs.
**Ref:** .andromeda/runs/2026-10-19T19-54-24-wrap/

## 2026-10-19-test-splits — escher-driver's cli and recording tests out of line; the log checks' shared fragment; a split moves the coverage reading
**Section:** §1 Test Scope Summary (escher-driver row; apps row — seven_guis `mcp_log` and the shared `tests/common/mod.rs`) · §2 Test Strategy → Directory pattern · §4 Unit Test Strategy → Test file location and the escher-driver pins · §10 Quality Gates & Coverage Targets → Coverage floors · §3 → Session lifecycle (key file `session-lifecycle.md`)
**Change:**
- §1 escher-driver: 92 unit tests in fifteen files, unchanged; the split was 72 inline / 20 in two `#[path]` files, now 58 inline / 34 in four (`mcp_tests.rs` 14 · `schema_tests.rs` 6 · `cli_tests.rs` 5 · `recording_tests.rs` 9); the off-unix recording test sits in a fifth `#[path]` file, `recording_off_unix_tests.rs`. Per-file `cli.rs 5` → `cli_tests.rs 5`, `recording.rs 9` → `recording_tests.rs 9`; the cli tests' citation was `cli.rs:476-1032`, now `cli_tests.rs:1-559`.
- §1 apps: `mcp_log`'s recorded test reads the shared `common::recorded_needles`, no longer its own; the shared module also holds `Needles`, `stand_needles`, `stamped_lines`, `found`, `occurrences` and `recorded_needles`, called by `host_log` and `mcp_log` alone.
- §2 / §4: out-of-line `#[path]` unit-test modules were five, now eight — `cli::tests`, `recording::tests` (`#[cfg(all(test, unix))]`) and `recording::off_unix_tests` (`#[cfg(all(test, not(unix)))]`) added, test paths unchanged. §4's pins name `cli_tests.rs` and `recording_tests.rs`; the off-unix refusal names `recording::off_unix_tests`; the citation was `recording.rs:469-981`, now `recording_tests.rs:1-517` and `recording_off_unix_tests.rs:1-15`.
- §10: a `*_tests.rs` `#[path]` file is no record of the per-crate reading (none in the coverage job's per-file report), so a test module moved out of line moves its crate's reading: escher-driver was 98.33 % (5180/5268), now 98.07 % (4266/4350); floors unchanged, 3 of 3 at or above them.
- Session lifecycle key file: the per-file list names `cli_tests.rs 5` and `recording_tests.rs 9`; `recorded_needles` is `common`'s; the off-unix check names `recording::off_unix_tests` in `recording_off_unix_tests.rs`.
**Why:** this chunk's own moves (the playbook's «Accurate this-chunk addition», the plan's reviewed expected amendments). The §10 sentence disposes the chunk's disproved forecast that moved lines stay counted; no master stated it, and the floor held. A later test split should forecast a reading change, not none.
**Ref:** .andromeda/runs/2026-10-19T21-24-54-wrap/

## 2026-10-19-shared-fixtures — a third shared module, one held network stand-in, the test-data bootstrap measured
**Section:** §1 Test Scope Summary (blitz-dom covered behaviours; tests/blitz-tests) · §2 Test Strategy → Directory pattern · §3 Test Harness Contract → Crate-local test helpers and the test-data bootstrap · §7 Test Data & Fixtures → Fixture sources · §8 Mocking & Stubbing Discipline → Hand-written fakes and stubs (Network rows)
**Change:**
- §1 / §2: blitz-tests' shared modules that are no target were two, now three. `tests/fixtures/mod.rs` joins `common` and `session_common`, read through `mod fixtures;` by 19 checks (18 `stand_*` files and `font_face_sniffed_source`); `common` still has 21 readers and `session_common` 25. `tests/all.rs` was «a third file», now «a fourth file» that is no built target.
- §1: `font_face_sniffed_source` was «held by a hand `HeldNetProvider`», now held by `fixtures::HeldNet`. Three pages that were in-file now come from the shared module: `stand_snapshot_text`'s password page and `stand_diff`'s are `fixtures::password_labelled`, and `stand_mcp_tools`' `covered` page is `fixtures::covered`.
- §3: a `blitz-tests (fixtures)` entry: `boot(app, incremental)` (`VirtualDom::new(app)` under `stand::options`), `hold(label, app, incremental)` (`Session::start`, a refused label panicking), the pages `password_labelled` · `password_named` · `covered`, and `HeldNet`. The MCP boot closures keep `Session::start(app, …)` and build through `fixtures::boot`; a page with one reader, or whose ids differ, stays in its file. The module holds no print, `tracing` call, env read, file, socket, listener or `blitz_net`.
- §3: the test-data bootstrap was NOT YET MEASURED, now measured. Nothing is seeded: the fixtures are code compiled into each target by `agent-run.sh boot` (`cargo test --no-run`). `run stand` runs the 18 `stand_*` readers, `run {file}` and `run all` the others, and there is no `run fixtures` selection.
- §7: the shared pages `password_labelled`, `password_named` and `covered`, stated as `rsx!` components.
- §8: the `ManualNetProvider` row was «two file-private ones: `render_blocking_stylesheet`'s, and `stand_settle`'s», now upstream's alone. A `HeldNet` row replaces escher's three file-private copies (`stand_settle`'s `ManualNetProvider`, `font_face_sniffed_source`'s `HeldNetProvider`, `stand_boot`'s `RecordingNetProvider`). It holds each request with its handler, `answer(body)` answers the last through `NetHandler::bytes`, and no `is_noop` override lets `stand_settle` read `NotSettled { busy: Loads }`. Upstream's `RecordingNetProvider` row is unchanged.
**Why:** this chunk's own test-only moves (the playbook's «Accurate this-chunk addition», the plan's reviewed expected amendments); no `run fixtures` on the operator's P4 answer «Existing selections (Recommended)». A page two checks share goes in `fixtures`.
**Ref:** .andromeda/runs/2026-10-19T23-02-04-wrap/

## 2026-10-19-visual-lint — `stand_lint` and `text_extent`, and the counts they move
**Section:** §1 Test Scope Summary (blitz-dom covered behaviours · escher-driver · tests/blitz-tests) · §2 Directory pattern · §3 Crate-local test helpers, test-data bootstrap and agent-run Proof · §3 → session-lifecycle · §4 Unit Test Strategy (escher-driver) · §9 Legs, Per-OS test tallies and Local baseline · §10 Coverage floors and Flakiness budget
**Change:**
- §1: `stand_lint` 5 (both modes, a `fixtures` and `session_common` reader) on the in-process `Session::lint` (PROVISIONAL): the planted page's seven findings, each lean task 0 at boot, no change, the last CRUD row `clipped` against `off-screen`, `fonts-unavailable`; a clipped or off-screen text node also reading `truncated` is asserted by no check. blitz-dom: `text_extent` 3 (both modes) on `BaseDocument::text_extent`, its limits asserted by no check. escher-driver: was 92 unit tests in fifteen files, 58 inline; now 94 in sixteen, 60 inline, `lint.rs` 2; `Session::lint` covered by `stand_lint`.
- §2: blitz-tests was 138 `.rs` files and 137 targets, now 140 and 139; `fixtures` readers were 19 (18 `stand_*`), now 20 (19); `session_common` readers were 25, now 26.
- §3: `fixtures` readers 19 → 20; `session_common` 25 → 26 (`stand_lint` reads `hold`, `act`, `click`, `type_into`, `refused`, `focused`, `slug`); `run stand` runs 19 `stand_*` readers (was 18) and reads 183 · 0 · 7 over 45 files (was 178 over 44); `run text_extent` 3 · 0 · 0. session-lifecycle: escher-driver's unit tests 92 → 94, `lint.rs 2` listed.
- §4: `lint.rs` — `Defect::ALL` names the six defects in order; findings follow document order, then the defect order.
- §9: stand-repeat files 44 → 45; CI#39348671810 tallies — linux 201 · 1055 · 0 · 12, macos and coverage 182 · 1052, windows 182 · 1010 · 0 · 9, a11y 6 · 43 unmoved, stand repeat 159 · 639 · 0 · 21; the local workspace 199 · 1045 → 201 · 1055 · 0 · 12.
- §10: escher-driver coverage was 4266/4350 = 98.07 %, now 4634/4732 = 97.93 %, the 97 % floor holding, 3 of 3 crates at or above their floors; the flakiness budget's files 44 → 45.
**Why:** this chunk's own checks and the counts they move (the playbook's «Accurate this-chunk addition», the plan's reviewed expected amendments); the lint's in-process scope is PROVISIONAL for the founder's own word (delegate the operator, 2026-10-20).
**Ref:** .andromeda/runs/2026-10-20T00-54-29-wrap/

## 2026-10-20-load-wait — `stand_load_wait`, the load wait's units, and the counts they move
**Section:** §1 (escher-driver · tests/blitz-tests) · §2 Directory pattern · §3 (session checks and fixtures helpers · test-data bootstrap · Agent-run Proof) · §3 → Session lifecycle (session-start · session-proof) · §4 (escher-driver) · §5 Session ↔ held instance · §8 Hand-written fakes · §9 (Legs · Per-OS tallies · Local baseline) · §10 (Coverage floors · Flakiness budget)
**Change:**
- §1 / §4 / session-proof: escher-driver was 94 unit tests (60 inline · 34 `#[path]`), now 99 (64 · 35); session.rs 6 → 10 (the load wait's four over a scripted settle), schema_tests.rs 6 → 7 (the `settled` help states `LOAD_WAIT` in whole seconds, the acting verbs carry it, the wait ends before an answer read). §1 enters `stand_load_wait` 2: both layout modes, a load answered at 200 ms ends the `click` settled with `fx-box` at 70, a load never answered ends it at `LOAD_WAIT` still `Loads`, no time moved.
- §2 / §3 / Existing counts: blitz-tests `.rs` files 140 → 141, targets 139 → 140; `session_common` readers 26 → 27; `fixtures` readers 20 → 21, `stand_*` 19 → 20 (§3 bootstrap's «the 19 `stand_*` readers» too).
- §3 → Session lifecycle session-start: `act` and each acting verb wait on a load in flight up to `LOAD_WAIT`, settling every 10 ms and moving no time; one still loading at the bound is `NotSettled(Loads)`; `snapshot` and `screenshot` wait on nothing.
- §5: the load wait and its proof stated; «no sleep and no wall-clock read» gains a second exception, `stand_load_wait`'s `Instant::now()` around its call and `LateNet`'s 200 ms sleep. §8: `LateNet` beside `HeldNet`.
- Readings: `run stand` 183 · 0 · 7 over 45 → 185 · 0 · 7 over 46; workspace 201 · 1055 → 202 · 1062 · 0 · 12; CI#39360124515 linux 202 · 1062, macos and coverage 183 · 1059, windows 183 · 1017, stand repeat 162 · 645 · 0 · 21; escher-driver coverage 4634/4732 → 4695/4794 = 97.93 %, floor holding; stand-repeat and flakiness files 45 → 46.
**Why:** this chunk's own checks and the counts they move (the playbook's «Accurate this-chunk addition», the plan's reviewed expected amendments); a check whose subject is wall time may read it, the plan's stated second exception.
**Ref:** .andromeda/runs/2026-10-20T03-00-02-wrap/

## 2026-10-20-select-by-id — the select checks, the counts, and the PROVISIONAL discharge
**Section:** §1 Test Scope Summary · §2 Test Strategy · §3 Test Harness Contract (and its key `session-lifecycle`) · §4 Unit Test Strategy · §5 Integration Test Strategy · §9 CI Integration · §10 Quality Gates & Coverage Targets
**Change:**
- §1: `select_interaction` 7 and `stand_act_select` 5 entered under tests/blitz-tests; `accessibility_label` 10 → 13; the select model's proofs and its stated limits (asserted by no check), a select's label click (asserted by no check), a select's form entries (no test of their own), `node/select.rs`'s 4 unit tests; dioxus-native-dom 73 → 75 unit tests, the snapshot builder's eighteen → twenty; escher-driver's `stand_act_*` fourteen → fifteen.
- §2: 141 → 143 `.rs` files, 140 → 142 targets; `session_common` readers 27 → 28, `fixtures` 21 → 23; fifteen `stand_act_*`, `stand_act_select` declaring no `mod common;`; the chunk markers in those counts' history parentheses dropped (the files named stand).
- §3: fixtures helpers 23 readers; `run stand` runs 21 `stand_*` readers and `run {file}` two non-stand readers, `run stand` 190 · 0 · 7; session-check helpers fifteen driver-action checks, 28 readers; «six of the fifteen»; Proof re-counted, 190 · 0 · 7 over 47 files. Key `session-lifecycle` (session-proof): fifteen driver-action files, `stand_act_select` 5, 55 in all.
- §4: the four `node/select.rs` tests described; fifteen `stand_act_*`. §5: `type` on a select and its proof `stand_act_select`.
- §9: stand-repeat over 47 files; CI#39371468367's per-OS tallies (linux 204 · 1083 · 0 · 12, macos and coverage 185 · 1080, windows 185 · 1038, stand repeat 165 · 660 · 0 · 21); local baseline 204 · 1083 · 0 · 12. §10: 47 files.
- The take's checks' «the behaviour they pin PROVISIONAL» and `stand_lint`'s «PROVISIONAL: in process only» dropped.
**Why:** this chunk's checks and counts (the report's sweep; the playbook's accurate this-chunk addition). The marks drop on the founder's own word, relayed verbatim by the overseer (2026-10-20, chunk inputs#I3) — the take «Утвердить (Рекомендую)», the visual lint F1 «Новая команда lint (Рекомендую)» — on the operator's «Drop all now» at this wrap's halt. The history markers dropped by classification rule 7: a count moved beside them re-anchors them as new provenance.
**Ref:** .andromeda/runs/2026-10-20T05-49-12-wrap/

## 2026-10-19-visual-lint — the gated part: the `lint` verb's tests and the counts it moved
**Section:** §1 Test Scope Summary · §3 Test Harness Contract · §3 → Session lifecycle (`session-start` · `session-binary` · `session-proof`) · §4 Unit Test Strategy · §5 Integration Test Strategy · §6 E2E Test Strategy · §9 CI Integration · §10 Quality Gates & Coverage Targets
**Change:**
- The verb table pinned at twelve verbs, eight instance-level, `lint` the eighth (was eleven and seven).
- The MCP listing is twelve, and describe's count row is `[12, 13, 12, 3, 8, 4, 15]`. json.rs's tests pin a finding's written form.
- Test counts:
  - `cli_commands` 13 (was 12): a hosted `lint` answers `{"findings":[]}`.
  - `mcp_server` 6 (was 5) and `stand_mcp_tools` 6 (was 5): a `lint` tool call.
  - `stand_lint` 6 (was 5): the `lint` verb through `Session::run` answers what the reader reads, 7 on the page and 0 on each task. Its «in process only» is retired.
  - `stand_act_spans` 34 calls per mode (was 32), reading a lint's line and a refused no-font one.
  - `host_log` and `mcp_log` each drive one `lint`, 0 needles.
- `session_common` gains `lint()` and `linted()`, and `act` covers a lint. The reader count stays 28.
- The cold-agent pipe: its server, `valid_step`, `isolated` and `FAKE` take twelve tools (was eleven). The 2026-10-15 live run stays dated.
- Session lifecycle: twelve verbs, eight instance-level, `lint`'s answer line. A recording keeps the eight instance verbs' calls. `lint` waits on nothing.
- Readings:
  - `run stand` 192 · 0 · 7 (was 190 · 0 · 7).
  - The workspace tally 204 · 1087 · 0 · 12 (was 1083).
  - CI#39387656399: linux 204 · 1087, macos and coverage 185 · 1084, windows 185 · 1041, stand repeat 165 · 672.
- Count histories («N until chunk») are dropped where a count moved.
**Why:** this chunk's measured counts, CI green 17/17 on ea3153f34 (CI#39387656399). The carriage is the founder's own word, F1 «Новая команда lint (Рекомендую)», relayed verbatim by the overseer (2026-10-20, chunk inputs#I7). His card confirm is inputs#I11.
**Ref:** .andromeda/runs/2026-10-20T07-57-08-wrap/

## 2026-10-20-large-screen-in-parts — the parts check, `--part` and the counts it moved
**Section:** §1 Test Scope Summary · §2 Test Strategy · §3 Test Harness Contract (crate-local helpers · agent-run Proof · test-data bootstrap) · §3 → Session lifecycle (`session-start` · `session-binary` · `session-proof`) · §4 Unit Test Strategy · §5 Integration Test Strategy · §6 E2E Test Strategy · §9 CI Integration · §10 Quality Gates & Coverage Targets
**Change:**
- New check `stand_snapshot_parts` 3 (§1, `session-proof`): a fixture screen over `SNAPSHOT_TEXT_BUDGET` walked in parts through `Session::run` and `serve_mcp` in both layout modes, and a screen inside the budget read whole.
- Unit counts: dioxus-native-dom 81 (was 75), six on `Snapshot::text_parts`; escher-driver 100 (was 99), 36 in four `#[path]` files (was 35), `mcp_tests.rs` 15 (was 14), the `snapshot` tool's optional `part`. §4: seven argument kinds (was six), `ArgKind::Part` 1 to `MAX_PART` = 1000.
- `session-start`: `snapshot` returns `Outcome::Screen`, or `Outcome::Part` for a screen over the budget or a `part` asked (was `Outcome::Screen` alone).
- `cli_commands` 14 (was 13): `snapshot --part` by number, and 0, 1001 and `x` refused, exit 1. `session-binary`: `--part` with one value, the paged answer line, the usage line opening `snapshot [--part <part>]`.
- `stand_act_spans` 35 calls per mode (was 34). `host_log` and `mcp_log` each drive one part read, 0 needles.
- `session_common` 29 readers (was 28), gaining `snapshot_part` and `part`; `fixtures` 24 readers (was 23), 22 `stand_*` (was 21). Stand-repeat and the flakiness budget: 48 files (was 47).
- Readings, each the newest only: `run stand` 195 · 0 · 7 over 48 files (was 192 · 0 · 7 over 47); the workspace tally 205 · 1098 · 0 · 12 (was 204 · 1087); CI#39410865604: linux 205 · 1098, macos and coverage 186 · 1095, windows 186 · 1051, stand repeat 168 · 684 (was CI#39387656399's 204 · 1087, 185 · 1084, 185 · 1041, 165 · 672).
**Why:** this chunk's measured counts, CI green 17/17 on 191e66fdb (CI#39410865604). The paged carriage is the founder's own word, F1 «Аргумент part, части по умолчанию (Рекомендую)», relayed verbatim by the overseer (2026-10-20, chunk inputs#I2); his card confirm is inputs#I5.
**Ref:** .andromeda/runs/2026-10-20T11-11-53-wrap/

## 2026-10-24-u55-door — test-plan: split into its body and 9 category files
**Section:** (the whole master — the door)
**Change:** split into its body and 9 category files; the map .andromeda/test-plan.map.tsv
**Why:** the operator's word: the founder's, 2026-10-24, given after the door's third pass, once the scoped reviewer accepted with 0 disputes; relayed by overseer1.
**Ref:** $TMPDIR/claude-1000/-home-turbolet-dev-projects-additional-andromeda-worker/d27ed186-56d7-4e67-a724-455c06ee4635/scratchpad/u55-door-2026-10-23-r2/test-plan/

## 2026-10-24-upstream-sync-polish-and-ship — `hit_test_dropped_nodes`, the counts it moves, and the gaps the take leaves
**Section:** §1 Test Scope Summary (body: blitz-dom covered behaviours; test-scope-summary.md: apps, blitz-dom covered behaviours, blitz-dom (node), blitz-tests) · §2 Test Strategy (test-strategy.md: Directory pattern) · §3 Agent-run contract (body: Proof) · §9 CI Integration (ci-integration.md: Per-OS test tallies, Local baseline)
**Change:**
- Covered behaviours gain hit testing past a node dropped since the last resolve, by blitz-tests' `hit_test_dropped_nodes` 1 — upstream's check, its own auto-discovered target, run by the workspace `test` leg and `run all`, not by `run stand` or the `a11y` leg, read red with its engine hunks reversed and green with them.
- Stated gaps: the process-wide `PARALLEL_TRAVERSAL_LOCK` is asserted by no check (no headless document sets `Parallel`; the windowed smoke its one local reading); `Node::out_of_flow`, `record_out_of_flow`, `style_is_out_of_flow`, `ElementData::classes` and `parse_classes` have no unit test of their own, read through the pipeline and stand checks; the root example `multi_doc` holds no test.
- Directory pattern: 145 `.rs` files and 144 test targets (was 143 and 142, one short of the 144 · 143 the tree held since `stand_snapshot_parts.rs`).
- Agent-run Proof: `run stand` unmoved at 195 · 0 · 7 over 48 files; `run all` 617 · 0 · 12.
- Per-OS tallies on CI#40092092468: linux 206 · 1099 · 0 · 12, macos and coverage 187 · 1096 · 0 · 12, windows 187 · 1052 · 0 · 9, the a11y job and the stand repeat unmoved; local workspace 206 result lines, 1099 · 0 · 12.
**Why:** upstream's check arrives with the dropped-node hit skip, an engine change the founder's own word «Десять коммитов (Рекомендую)» ratified (relayed verbatim by the overseer, 2026-10-24, chunk inputs#I4). Each count is measured and moves by that one check alone; the 143 · 142 the strategy stated was a stale count.
**Ref:** .andromeda/runs/2026-10-24T12-21-29-wrap/

## 2026-10-24-upstream-sync-polish-and-ship — the `keyboard_compat.rs` exclusion re-read
**Section:** §10 Quality Gates & Coverage Targets (body; quality-gates-coverage-targets.md: Coverage floors)
**Change:** was «… excluded until upstream retires it»; now «until escher takes upstream's retirement of it» — upstream retires `keyboard_compat.rs` in `286874cc1` (Dioxus 0.8.0-alpha.3), a commit escher has not taken, so the file stays in the tree, both `--ignore-filename-regex 'keyboard_compat\.rs$'` lines stand, and the three floors and `coverage_floor.py` are unchanged; the exclusion lifts with that move, owned by the `.andromeda/residuals.md` line of the left-out upstream delta (target: next).
**Why:** measured at this sync (`286874cc1` holds no `keyboard_compat.rs`; the file and both regex lines stand in the tree). The left-out delta's residuals line is the operator's own word «yes — apply» on the phase's P5 card (inputs#I5).
**Kept:** the exclusion itself and the floors — the bridge is still in the tree.
**Ref:** .andromeda/runs/2026-10-24T12-21-29-wrap/

## 2026-10-24-stand-requirement-sweep — `stand-repeat` runs ten named targets beside the `stand_` files
**Section:** §9 CI Integration (body: Legs; ci-integration.md: Legs, Engine features by runner, Per-OS test tallies, Local baseline) · §10 Quality Gates & Coverage Targets (body: Flakiness budget; quality-gates-coverage-targets.md: Coverage floors, Flakiness budget) · §1 (test-scope-summary.md: the runner clauses of `text_extent`, `password_caret_click`, `accessibility_states`, `inert_subtree`, `accessibility_active_descendant_unread`, `select_interaction`) · §3 Agent-run contract (body: Proof; Test-data bootstrap)
**Change:**
- Legs and Flakiness budget: was «the stand checks and seven_guis' process checks»; now the `stand_*.rs` files, then on the same `cargo test -p blitz-tests --locked` line ten named targets as literal `--test` words — `inert_subtree`, `accessibility_states`, `scrolled_box_absolute_position`, `hit_clipped_at_scrolling_box`, `password_caret_click`, `transformed_client_rect`, `accessibility_active_descendant_unread`, `virtual_clock_engine`, `select_interaction`, `text_extent`, each file keeping its name — then seven_guis' eight; `test_stand_repeat_leg_runs_three_rounds` pins the glob and the ten words.
- The six runner clauses in §1 gain the `stand-repeat` leg; `accessibility_merged_states` does not.
- `run stand` keeps its prefix rule: 197 · 0 · 7 over 48 files (was 195), the ten reached through `run all` and `run {name}` alone.
- Readings, each the newest: CI#40114977936 linux 206 · 1105, macos and coverage 187 · 1102, windows 187 · 1057, a11y 6 · 43, stand repeat 198 · 837 · 0 · 21 (was CI#40092092468's 206 · 1099, 187 · 1096, 187 · 1052, 168 · 684); the workspace 206 · 1105 · 0 · 12 (was 1099); coverage 97.99 · 85.57 · 94.15 %, the floors unchanged.
**Why:** the selection is the founder's own word «Список в скрипте (Рекомендую)» (2026-10-24, relayed verbatim by the overseer, chunk inputs#I1): the 0.2.0 witnesses outside the prefix enter the leg by a fixed list, «Rename them stand_*» passed over. The counts are measured, CI green 17/17 on 18b50a2a2.
**Kept:** `run stand`'s prefix selection and the 48-file `stand_*.rs` count.
**Ref:** .andromeda/runs/2026-10-24T17-32-19-wrap/

## 2026-10-24-stand-requirement-sweep — the 0.2.0 record, the asserted limits and the recording clients' stderr
**Section:** §1 Test Scope Summary (body: blitz-dom covered behaviours; test-scope-summary.md: apps, blitz-dom covered behaviours, tests/blitz-tests) · §3 (body: Crate-local test helpers → blitz-tests (fixtures); → Session lifecycle) · §5 Integration Test Strategy (body and integration-test-strategy.md: Session host ↔ what it writes) · §7 Test Data & Fixtures (body: tests/blitz-tests/tests/fixtures; test-data-fixtures.md: Data seeds)
**Change:**
- `stand_requirement_sweep` 3 (was 2): `WITNESSES_020`, nine rows and 46 witnesses, `Subject::Unit` a crate's unit test outside `stand-repeat` by construction, and its leg check reading `ci-leg.sh` for the glob and all 18 names; `WITNESSES` unchanged.
- `stand_lint` 7 (was 6): a clipped or off-screen node holding text also reading `truncated` is asserted (was «asserted by no check»), on `reading_page`'s `fx-cut-text` · `fx-padded-text` · `fx-off-text`.
- `text_extent` 6 (was 3): its three stated limits asserted (was «asserted by no check»); the select's limits and `PARALLEL_TRAVERSAL_LOCK` still asserted by none.
- `stand_act_spans` 36 calls per mode (was 35), a part 2 of the shared `many_items` page among them, its ids and names joining the needles.
- `host_log` 3 (was 2): a recording session's clients read 0 needles of each class over their own stderr at `trace`, a standing check beside the 2026-10-14 one-off reading; `mcp_log` 2.
- The shared pages gain `many_items` with `ITEMS` (400), moved unchanged from `stand_snapshot_parts`, read by it and `stand_act_spans`.
- New stated limit: a paged `part` read through the `escher-session` host's own sink is unmeasured (the host boots only the four lean tasks, none pages), owned by a `.andromeda/residuals.md` line (target: next).
**Why:** each count and reading measured at this chunk (red before green, both layout modes, both builds). The limit's owner is the operator's own word «yes — apply.» on the phase's P5 card 2 (chunk inputs#I2).
**Ref:** .andromeda/runs/2026-10-24T17-32-19-wrap/

## 2026-10-24-a11y-re-read — the a11y leg's fourteen files in §1, §4 and §9
**Section:** §1 Test Scope Summary (test-scope-summary.md: tests/blitz-tests) · §4 Unit Test Strategy (unit-test-strategy.md: CI workflows and leg script) · §9 CI Integration (ci-integration.md: Legs, Per-OS test tallies, Local baseline)
**Change:**
- Legs: `a11y` names its fourteen `--test` words on one line (was six) — its membership the tier-0 witnesses and the five tree-reading checks, 14 files and 84 tests, over the tier-0 witnesses alone, 9 and 59; `focus_redraw`, `focus_within` and `accessibility_names` not added; each file an SC 2.1.1 or 2.4.3 witness, a tree reading the founder took or a recorded gap (`accessibility_active_descendant_unread`); each joining file keeps its legs, `stand_act_select` in both `a11y` and `stand-repeat`.
- Per-OS test tallies: CI#40134371519 on `4cbf0a4c9` read — the a11y job 14 · 84 · 0 · 0 (was 6 · 43 · 0 · 0), every other job equal to CI#40125511019's; a per-job tally strips ANSI codes with `(\x1b|\^\[)\[[0-9;]*m`, the log spelling ESC as the literal `^[`.
- Local baseline: `ci-leg.sh a11y` fourteen files, 14 · 84 · 0 · 0, per file (was «6 · 6 · 3 passed», the six files' 43).
- Unit Test Strategy: the a11y pin row asserts each of the fourteen target words, beside `--workspace` and `--locked`, by one `assertIn`, in no order, `:298-302` (was «pinning all six test files in order», `:298-299`).
- Test Scope Summary: `accessibility_states`, `accessibility_label`, `accessibility_description`, `accessibility_merged_states`, `accessibility_active_descendant_unread`, `inert_subtree`, `select_interaction` and `stand_act_select` run by the `a11y` leg (was «not by the `a11y` leg's six», or no leg named); `focus_within`, `focus_redraw` and `hit_test_dropped_nodes` still outside, «not among its fourteen» (was «the `a11y` leg's six»); the adapter gap gains the `cargo tree --locked -p seven_guis -e normal` probe reading 0, so no a11y-leg check reaches the adapter crossing.
**Why:** the leg's membership is the founder's own word on the chunk's P4 fork, «Плюс проверки дерева (Рекомендую)», relayed verbatim by the overseer (2026-10-24, chunk inputs#I1); the counts measured at the chunk; «in order» disproved by reading the pin, one `assertIn` per word.
**Ref:** .andromeda/runs/2026-10-24T21-49-26-wrap/

## 2026-10-25-capability-statement — the capability-statement pin, the ci-scripts tally and the AT owner
**Section:** §1 Test Scope Summary (body: CI Python scripts, tests/blitz-tests; test-scope-summary.md: CI Python scripts, tests/blitz-tests) · §2 Test Strategy (test-strategy.md, a new group) · §3 → Session lifecycle (the key file: the socket's answer) · §4 Unit Test Strategy (body: a Capability statement bullet; unit-test-strategy.md: the CI-scripts tally, a new Capability statement group) · §7 Test Data & Fixtures (test-data-fixtures.md: a seeds row)
**Change:**
- §1: the `ci-scripts` leg runs 104 tests in eight test files (was 100 in seven), among them the four of `CapabilityStatementTest`, through which `docs/capabilities.md`, a document with no runtime surface, is in test scope; the body names the pin among what the leg tests.
- §1: the adapter-crossing gap, owed at an active assistive technology this version does not reach, is owned by the residuals line that reads `re-carried:0.2.0` (was «(target: next)»), in the body and the detail.
- §2: a new group, claims mapped to checks — the statement's `## Where each claim is proven` names each claim's check, only its names and counts pinned, so a renamed check changes it by hand.
- §3 Session lifecycle: the socket's answer names a lint's findings beside the snapshot text and the diff.
- §4: a body bullet and a detail group for `test_capability_statement.py` — four tests: every `VERBS` verb and every `Cause::name` a code span, five counts in words from the schema's tables, README's «the {word} driver commands as tools»; by path, no write, no env read, whitespace folded, the phrase as each `assertIn`'s message, controlled red-before-green; the CI-scripts tally 104 over eight, its per-file list adding test_capability_statement.py's 4, the 2026-10-15 reading kept as the series' second.
- §7: a seeds row — repository text read by path and an inline `NUMBER_WORDS` table, no fixture file.
**Why:** the pin and its leg tally are this chunk's own new test surface, measured green (`Ran 104 tests`, CI#40184674449); the AT owner is the report's measured disposition of the residuals line; the session-lifecycle list follows the visual lint's crossing onto every ratified carrier, one of the eight sites the cascade sweep found, reconciled on the founder's own word «Да, и эти 8 (Рекомендую)», extending his P2-halt answer «Применить как сверку (Рекомендую)» to all ten carried-set sites, by dialog in the overseer session, relayed by the operator.
**Ref:** .andromeda/runs/2026-10-25T11-21-44-wrap/

## 2026-10-25-cold-agent-bar — the cold-agent pipe over six apps by the 5 % bar, its recorded live child and the second live run
**Section:** §3 Cold-agent run pipe (body: the one task, `run stand`, the prompt, the judge, the server log, the replays, the state set, the verdict fields and reasons, Proof)
**Change:**
- The one task `stand` names six apps — counter, flight booker (its return booking chosen on its flight type control, plus one screenshot), timer, CRUD, cells (`A0` 5, `B1` `=A0*2` reading 10, `Z25` `=B1+1` reading 11, plus one visual lint) and the load page (its resource loaded and its message shown) — still naming no element id and no app slug (was the four lean apps).
- `run stand` recreates `target/cold-agent/` with its owner-only `record/` (`mkdir -m 700`); the live server is `exec env RUST_LOG=info "$1" mcp --record "$3"`; the replay children are spawned without `--record`, the expect match reading a paged answer's decoded text (`paged`, `screen_text`); `server.log` is read to count ` span="tool_call" ` lines, the refused and error endings and the accepted lines of each counted verb (`lint` · `screenshot` · `advance`); the state set is `{verdict.json, events.jsonl, transcript.jsonl, client.log, server.log, check.json, record/}`, `record/` never printed, copied or committed.
- `passed` adds all six apps met from accepted answers (an accepted `start` resets an app, a paged screen read as each part's latest answer joined), each of `lint_calls` · `screenshot_calls` · `advance_calls` · `part_reads` above 0, `recording_kept`, and `wrong_calls × 100 ≤ 5 × tool_calls`; the verdict adds `wrong_call_bar_pct, lint_calls, screenshot_calls, advance_calls, part_reads, recorded_sessions, recording_kept`; the reasons add `capability-unused` · `recording-not-kept` · `wrong-calls-over-bar`; `wrong_calls` decides against `wrong_call_bar_pct` (5) (was recorded, never deciding).
- Proof: the contract tests (§4, 37) and the live run, fired once — `passed`, reasons none, `isolated` (twelve `mcp__escher__*` tools), `counts_agree` (66 · 66), `tasks_met` all six, `lint_calls` 1 · `screenshot_calls` 1 · `advance_calls` 1 · `part_reads` 2, `recorded_sessions` 6 with `recording_kept`, `check_passed` and `check_can_fail` (59 steps, 16 expects), 66 tool calls, 0 wrong of 66 against the 5 % bar, 0.8244 USD, `claude` 2.1.308 (was the 2026-10-15 run — four apps, eleven tools, 43 calls, 0.3197 USD, `claude` 2.1.296 — and the retired stub's 0.0418 USD run).
**Why:** the bar is the founder's own answer at the phase (chunk inputs#I3); the run fired on his word at the operator pass, relayed by the operator (inputs#I6); measured at the chunk.
**Ref:** .andromeda/runs/2026-10-25T17-25-04-wrap/

## 2026-10-25-cold-agent-bar — the counts the served pages and the select moved, the pipe's contract tests
**Section:** §3 agent harness (body: `run stand`'s re-count, the §1 reading) · §9 CI (ci-integration.md: the `test` leg's `stand_*` targets, the workspace tally, the a11y leg) · §10 (quality-gates-coverage-targets.md: coverage, `stand-repeat`) · §4 (unit-test-strategy.md: the `ci-scripts` leg, the cold-agent contract tests; test-scope-summary.md: the CI Python scripts) · test-strategy.md (the target count, `session_common`'s readers) · e2e-test-strategy.md (a second real client of the `mcp` role)
**Change:**
- `run stand`'s `run.end` reads passed 205 · failed 0 · ignored 7 over 50 files — +8 (`stand_cells_served` 4, `stand_load_page` 2, two new files picked up by their `stand_` prefix with no script change, `stand_keyboard` +1, `stand_snapshot` +1); `run all` not re-counted (was 197 over 48).
- The `test` leg: one `--test` per `stand_*.rs`, 50 (was 48), `stand_snapshot_parts`, `stand_cells_served` and `stand_load_page` the latest; the workspace tally 208 result lines, 1117 passed · 0 failed · 12 ignored (was 206 · 1105) — +2 lines and +12 passed (`stand_cells_served` 4, `stand_load_page` 2, three stand unit tests, `host_log` 1, `stand_keyboard` 1, `stand_snapshot` 1); the a11y leg 14 result lines, 85 passed, `stand_keyboard` 15 (was 84, 14).
- Coverage: escher-driver 4918/5019 = 97.99 %, dioxus-native-dom 2741/3202 = 85.60 % (was 85.57), blitz-test-harness 952/1008 = 94.44 % (was 94.15), every crate at or above its floor; `stand-repeat` runs the 50 `stand_*` files, `3 of 3 rounds green`, CI#40214713710 green 17/17.
- `ci-scripts`: 108 tests in eight files, `test_cold_agent.py` 37 (was 104, 33), green at CI#40214713710.
- `tests/` holds 147 `.rs` files, 146 test targets and the unbuilt `all.rs` (was 145, 144); `tests/session_common/mod.rs` is read by 31 checks (was 29), `stand_cells_served` reading its call builders, `Acted` and `act`, `stand_load_page` reading `act`, `click`, `lint`, `linted` and `snapshot`.
- §4: `test_cold_agent.py` 37 (was 33), its fake answering Cells as a paged JSON line of two parts and the load page and keeping one check per accepted start under `--record`: the six-app pass, five wrong of a hundred passing and six failing, a paged answer joined, each unused capability (four sub-cases), a `start` with no kept check (two); 34 of 37 fail against the base pipe, a red-before-green control. The `mcp` role's second real client is the live run (`claude` 2.1.308, `mcp --record`).
**Why:** the newest reading of each run, measured at the chunk; each earlier figure stays under its own pointer in the body only where the body keeps a dated series.
**Ref:** .andromeda/runs/2026-10-25T17-25-04-wrap/

## 2026-10-25-cold-agent-bar — the stand's tables and checks re-pinned to the flight booker's select
**Section:** §3 stand checks (body: the shared tables, `change`) · test-scope-summary.md (`stand_accessibility_ids`, `stand_snapshot_state`, `stand_act_ids`, `stand_act_states`, `stand_contrast`, `stand_keyboard`, `stand_requirement_sweep`, `stand_lint`) · integration-test-strategy.md (`stand_act_states`' diff) · test-strategy.md (`stand_id_persistence`)
**Change:**
- Shared tables: 17 `controls(task)` rows naming 14 controls, `flight-type` a `Role::ComboBox`; `ENABLED_TAB_ORDER`'s flight booker after the return flight is chosen on `flight-type` by ArrowDown, five stops, `back-btn` → `flight-type` → `flight-start` → `flight-return-date` → `flight-book` (was 18 · 15, six stops after `flight-return`); `session_common::change` moves the flight booker by `type flight-type return`.
- Exactly the chosen flight type's option reads `selected`, swapping with the choice (was the active mode reading pressed); a flight-type choice returns the select `flight-type` with its `value` and both options' `selected` reading, the stand stating no `aria-pressed`, `pressed` pinned on a fixture by `accessibility_states`.
- `stand_act_ids`: of the 77 nodes, `click` accepted on 68 and refused on 9 (was 70 · 7), the two options `flight-type-one-way` · `flight-type-return` refused `covered`, each at its select's client rect; 14 controls, 17 rows changing the screen at boot.
- `stand_contrast`: 17 rows, `flight-type` `#1a1a1a` on `#ffffff` at 17.40, 15px 400, pass 13; `STATE_ROWS` 18, return-chosen 2 (`flight-type`, `flight-return-date`, each 17.40), 17 pass (was 18 · 14 · 19 · 3 · 18).
- `stand_keyboard` 15 (was 14), activation 16 rows (was 20), the fifteenth moving the select's choice by its arrows.
- `stand_requirement_sweep`: `WITNESSES_020` 49 witnesses (was 46), a `Subject::Served` witness naming a check on a page served beside the lean four (`stand_load_page` for v020-07, `stand_cells_served` for v020-09), the flight witness `type flight-type return`.
- `stand_lint`: the lean tasks at boot read their measured findings, `[0, 1, 0, 0]` — the flight booker's one `overlap`, `flight-type-one-way` × `flight-type-return`, the lint's reading of a select's options, a driver defect owned by a `.andromeda/residuals.md` line (`the_stand_at_boot_reads_its_measured_findings`; was 0 on each).
- A select's choice moves no class or text `dom_string` shows, so `stand_id_persistence` chooses the return flight and books it in one re-render.
**Why:** a stand change, no engine crate moving, measured at the chunk; the lint re-pin on the founder's own word at /implement, relayed by the operator, its owner line written at this wrap's P5.
**Ref:** .andromeda/runs/2026-10-25T17-25-04-wrap/

## 2026-10-25-cold-agent-bar — the served pages' boots and checks, `LoadNet` and the host-sink part read
**Section:** §3 (body: the stand's boots; the stand-in's provider line; test-scope-summary.md: a new `procedure · scope-entities` group, seven_guis' checks; test-data-fixtures.md: two new rows; mocking-stubbing-discipline.md: the `HeldNet` row, a new `LoadNet` row) · §5 (body: the served pages driven, the wall-time line, the host-sink part read; integration-test-strategy.md: time, the scrollbar, the lifecycle, `host_log`, the recording bound) · registry: session-lifecycle (`host_log` 4)
**Change:**
- The stand boots Cells through `stand::boot_cells` (`Task::Cells` in TaskShell, no `LeanTask`) and the load page through `stand::boot_load` (TaskShell "Load", its options' `net_provider` replaced by `LoadNet`, every other field kept); `net_provider: None` kept for every caller but `boot_load`; an offline provider is every stand boot's but `boot_load`'s.
- `LoadNet`, a third hand-written network stand-in beside `HeldNet` and `LateNet`: each `fetch` answers the fixed `LOADED_CSS` for `http://stand.test/load.css` from a thread of its own after `LOAD_ANSWER_DELAY` (200 ms), no `is_noop` override, so the harness counts its load; no `blitz_net`, print, env read, socket or listener. Fixtures: the stand's Cells screen over the budget (47,949 bytes, 740 lines, 5 parts) and the load page's network answer.
- Served-page checks, each declaring `mod session_common;` and no `mod fixtures;`, both layout modes, picked up by prefix: `stand_cells_served` 4 (Cells walked in parts, a cell past the viewport in the last part, a click right after a `scroll` landing on the shown overlay scrollbar so the walk takes `advance 5000` first, the lint at boot) and `stand_load_page` 2 (the click waiting out the load to `load-done` "Loaded", the page keyed with no finding).
- §5: `stand_load_page` reads no clock and sleeps nowhere — the 200 ms sleep is the stand's own `LoadNet`'s, waited out through `LOAD_WAIT`; `stand_cells_served` moves time by `advance` alone. The paged part read through the host's own sink is a standing check: `host_log` 4 (was 3) serves `cells` at `RUST_LOG=trace`, commits a sentinel into a part-2 cell and reads `snapshot --part 2`, 0 id and 0 name needles under both builds, its needles from `common::cells_needles(part)` (was unmeasured, owned by a `.andromeda/residuals.md` line).
**Why:** measured at the chunk; the residual's discharge is the operator's own word on the phase's P5 card (chunk inputs#I4); §5's wall-time line the overseer's technical call at this wrap's P2 halt, verified in the tree.
**Ref:** .andromeda/runs/2026-10-25T17-25-04-wrap/

## 2026-10-25-cold-agent-bar — six served slugs through the binary, the recording bound and the snapshot line sizes
**Section:** e2e-test-strategy.md (`cli_commands`, the host role, the snapshot sizes) · integration-test-strategy.md (the lifecycle, the recording bound) · registry: session-lifecycle (the six slugs, the time step)
**Change:**
- `cli_commands` (14) starts each of the six served tasks; `host_binary` and the lifecycle run on the six (`counter` · `flight-booker` · `timer` · `crud` · `cells` · `load`); `main` hands the six slugs, the other five sessions no time step; a full recorded Cells session keeps 312,431 bytes and the load page's 1,660, by a probe, no standing check; the hosted snapshot line 832 · 1,403 · 1,368 · 2,208 bytes on the four lean tasks (re-measured at this wrap).
**Why:** measured at the chunk; the snapshot line sizes re-measured on the dev host at this wrap's cascade sweep, the flight booker's select having moved them (was 1,393 and 2,202 on the flight booker and CRUD, at 2026-10-10-snapshot-states).
**Ref:** .andromeda/runs/2026-10-25T17-25-04-wrap/
