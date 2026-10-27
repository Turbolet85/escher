# Task — tests · 2026-10-27-upstream-sync-engine-debts

## The chunk
This fork's eighth upstream take from DioxusLabs/blitz at the measured sha `ebacd68bd`. The shape is a fork decided at P4: cherry-picks onto the registry pins, those picks plus winit beta.4, a whole merge, or nothing. 27 upstream commits are candidates, the ten left out at the last sync among them.
Conflicts are resolved keeping both sides' intent: the changed set, the clock invariant, the select model, the hit stops and the scrolled-box bounds all stay unchanged. Upstream's new and edited test files run under any take that carries them.
Re-runs every escher proof on the take's tree: the fast and doc legs, the stand checks in both layout modes, the incremental oracle, the a11y leg and the dependency audit, with fork CI green.
Re-reads the per-OS test tallies of the take's green CI run, explaining each moved count. Re-reads the sink allowlist against upstream's new log sites, and each merge-surface owner row of residual line 36.

## Reached files
- Cargo.toml · merge surface
- Cargo.lock · conflict surface
- packages/blitz-dom/src/layout/construct.rs · conflict surface
- packages/blitz-dom/src/layout/inline/build.rs · conflict surface
- packages/blitz-dom/src/layout/writing_mode.rs · conflict surface
- packages/blitz-dom/src/mutator.rs · conflict surface
- packages/blitz-dom/src/node/node.rs · conflict surface
- packages/blitz-dom/src/scrolling.rs · conflict surface
- packages/blitz-vibey-script/src/runtime.rs · conflict surface
- wpt/WPT_COMMIT · conflict surface
- packages/blitz-dom/src/node/element.rs · merge surface
- packages/blitz-dom/src/stylo.rs · merge surface
- packages/blitz-dom/src/document.rs · merge surface
- packages/blitz-dom/src/events/pointer.rs · merge surface
- packages/blitz-dom/src/events/driver.rs · merge surface
- packages/blitz-dom/src/node/text.rs · merge surface
- packages/blitz-shell/src/lib.rs · merge surface
- packages/blitz-shell/src/window.rs · merge surface
- packages/blitz-shell/src/convert_events.rs · merge surface
- packages/blitz-traits/src/events.rs · merge surface
- packages/blitz-traits/src/shell.rs · merge surface
- packages/dioxus-native-dom/src/dioxus_document.rs · merge surface
- packages/dioxus-native-dom/src/keyboard_compat.rs · merge surface
- packages/blitz-vibey-script/src/fetch.rs · merge surface
- packages/blitz-vibey-script/tests/dom.rs · merge surface
- tests/blitz-tests/tests/all.rs · merge surface
- tests/blitz-tests/tests/pseudo_element_flow_classification.rs · merge surface
- tests/blitz-tests/tests/clipboard.rs · merge surface
- tests/blitz-tests/tests/inline_box_scrollable_overflow.rs · merge surface
- tests/blitz-tests/tests/display_contents.rs · merge surface
- tests/blitz-tests/tests/pre_overflow_scroll.rs · merge surface
- .github/scripts/ci-leg.sh · edits the coverage leg's two keyboard_compat ignore-filename-regex lines, under a take of Dioxus 0.8
- .github/scripts/test_blitz_tests_targets.py · re-reads BlitzTestsTargetsTest, the blitz-tests target-list pin
- packages/escher-telemetry/src/format.rs · re-reads the sink's target allowlist
- deny.toml · re-reads the audit's ignored advisory IDs and licence table
- tests/blitz-tests/tests/incremental_oracle.rs · re-reads the incremental/non-incremental oracle
- tests/blitz-tests/tests/stand_keyboard.rs · re-reads the Tab order and Enter/Space activation tables
- tests/blitz-tests/tests/stand_contrast.rs · re-reads the stand's contrast rows
- tests/blitz-tests/tests/stand_element_ids.rs · re-reads the stable-id derivation

## Needs
- The blitz-tests target rule: each new upstream test file becomes its own target by auto-discovery, and the unbuilt `all.rs` takes its `mod` line. The manifest pin `BlitzTestsTargetsTest` must still hold one `[[test]]` table, `all` with `test = false`, and no `autotests = false`.
- Upstream's edited tests (`inline_box_scrollable_overflow`, `display_contents`, `pre_overflow_scroll`) and the extended blitz-vibey-script `tests/dom.rs`. Each runs as part of the workspace test leg, and P3 reads whether each is a standing target here.
- The agent-run tallies: `run stand` and `run all` are re-counted on the take's tree, and the body's §3 Proof list records the new count. Upstream's new non-`stand_` files are picked up by `run all` with no script change, as earlier syncs recorded them.
- The per-OS test tallies (linux over macOS by the doc-test lines, macOS over windows by the unix-gated binaries; ios and android build only). They are re-read from the take's green CI run's test-bearing job logs by the recorded method, with each moved count explained. A new `clipboard` target may be OS-gated.
- Engine features by runner: the workspace build turns on `tracing` and `writing-mode` and `-p blitz-tests` does not. A green per-package run is not the workspace leg's green, and both bodies of the bounds reader must still have a check (the second body lives in `writing_mode.rs`, a conflict surface).
- A moved layout, style or event number from a Taffy, Stylo or winit move is re-measured and recorded, never loosened. This covers the stand checks' exact bounds, a snapshot text's byte size, a screenshot's pixels and the harness raster equality.
- The committed Counter checks under `tests/blitz-tests/checks/` keep whole answers. If the take moves a kept answer, `stand_replay` and seven_guis `recording` go red, and the checks are re-recorded through `record_checks.sh`, never edited by hand.
- The hit path and scrolled-box checks decide the node.rs, scrolling.rs and writing_mode.rs resolutions: `hit_clipped_at_scrolling_box`, `scrolled_box_client_rect`, `scrolled_box_absolute_position`, `transformed_client_rect`, `scroll_into_view_nested`, `hit_test_dropped_nodes` and `inert_subtree`.
- The mutator's changed set, the select and range arms, and the clock surface are decided by blitz-dom's inline tests and the checks `stand_diff`, `select_interaction`, `stand_act_select`, `range_input_interaction`, `virtual_clock_engine` and `stand_virtual_clock`. The in-file tests of the conflicted blitz-dom files are kept.
- dioxus-native-dom's unit tests (element id, snapshot, bridge_tests in `dioxus_document_tests.rs`, which builds over the bridge's private fields) are rebuilt against an edited `dioxus_document.rs`. So are `stand_id_persistence`, `stand_id_edits` and the snapshot checks, which decide the stable-id port under a take of Dioxus 0.8.
- The coverage leg's per-crate floors (dioxus-native-dom 83 % among them) are read on the take's tree. Its `keyboard_compat.rs` exclusion drops only with a take of `286874cc1`.
- The a11y leg and its checks (`stand_keyboard`, `stand_contrast`, `accessibility_*`, `inert_subtree`) stay green. `7fc9ee3af` (IME on a focused input), `53e9d81c6` (`:open`) and `ebacd68bd` (hit testing) are the commits that reach this surface.
- The dependency audit's `advisories ok, licenses ok` over the whole workspace's graph, with the ignored IDs as recorded. `arboard` leaves under `527394724`, winit's lock entries move under `a09f46792`, and any crate that enters is reviewed by hand.
- The sink capture checks stay at 0 needles on the take's tree: `host_log`, `mcp_log`, `stand_act_spans` and the `telemetry_*` files. Upstream's two new `tracing::error!` sites (blitz-shell's clipboard error, blitz-vibey-script's uncaught JS error) are read against the engine-target rule. A capture check counts lines by target.
- The script engine's test surface: `cee6b8731` narrows `file:` loads in the fetcher and `efdc5472a` adds CSSStyleDeclaration accessors. Both extend `tests/dom.rs`, whose timer tests sleep in real time.
- The stand-repeat leg's zero-failure budget, three rounds with no retry, must hold on the take's pushed commit, together with every other fork CI job, matrix included.

## Standing conventions
- S1 · A blitz-tests test is one file and one target per behaviour, auto-discovered, its `mod` line in the unbuilt `all.rs`, and three shared modules are no target · test-strategy.md
- S2 · Pipeline and stand tests run in both layout modes, `for incremental in [false, true]`, and the differential oracle compares every node after each step · test-strategy.md
- S3 · Older font-dependent tests skip at runtime with no usable font, while the headless-stand checks take no skip path under the bundled font · test-strategy.md
- S4 · A shot's PNG bytes differ by OS, so a check compared byte for byte across OSes holds no screenshot step · test-strategy.md
- S5 · Unit tests sit in `#[cfg(test)]` modules in the source file or in `#[path]` child files, their test paths unchanged · unit-test-strategy.md
- S6 · A check that reads a sink's capture counts lines by target, never in total · ci-integration.md
- S7 · Every cargo leg runs through `ci-leg.sh` with `--locked`, and each leg's log goes to `target/ci-logs/{leg}.log` · ci-integration.md
- S8 · The stand checks, the ten named targets and seven_guis' process checks hold a zero-failure budget with no retry · quality-gates-coverage-targets.md
- S9 · A new test data seed is stated in code, with no seeded store · test-data-fixtures.md

## Not needed
- WPT conformance and scoring, WPT reftest rendering, and the WPT and post-results workflows. They run on upstream only, and upstream's WPT numbers are out of scope.
- The cold-agent run pipe's live run and its verdict fields. It runs on the operator host only, and the chunk changes no driver verb, tool or prompt.
- The driver's socket wire grammar, the MCP framing bounds and the command line's exit grammar. The chunk changes no driver code and adds no verb, field, cause or exit.
- The capability statement pin over `docs/capabilities.md` and README.md, because no verb, cause, session error, node key or exit status moves.
- The process-lifecycle and waiting-on-host rules for session checks, because the chunk adds no check that holds a session process.
- The harness construction, input-helper and inspection API contract. The chunk adopts no new upstream API in escher's own code.
- The shared fixture pages and stand-ins (`HeldNet`, `LoadNet`, `LateNet`, `TimerTicks`), because the chunk adds no fixture page and no stand-in.
- The mutation tier. It runs on the dev host only and is owed by no sync.
- The manual inspection tools and example fixtures (`examples/assets`, `paint_bench`, the `screenshot` example) and the `#[ignore]` paint benchmark.
