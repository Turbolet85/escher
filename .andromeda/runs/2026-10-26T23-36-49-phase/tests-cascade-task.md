# Task — tests · 2026-10-26-codebase-health

## The chunk
A pure refactor: three escher-written functions the Epoch 5 code audit read over the cognitive-complexity ceiling of 15 are brought to at most 15 as a table walk or named helpers, and every helper introduced also reads at or under 15.
The three are the visual lint's per-node walk `findings` (16), the MCP tool-listing unit test (18) and the stand requirement-sweep test (18); the two tests stay in the complexity count, keep their names and assert the same rows.
No behaviour, verb, field, carrier, log site or span changes, and no test's set of assertions changes; the engine function `clear_interaction_state_for_removed_node` and every other over-ceiling function stay as they are.
It re-measures the audit's complexity reading (rust-code-analysis 0.0.25, cognitive per function, threshold over 15) before and after, expecting at most 85 functions over the ceiling with no new entrant, and re-runs the visual-lint checks and the fast and doc legs green.

## Reached files
- packages/escher-driver/src/lint.rs · edits `findings`, the visual lint's per-node walk (its inline unit tests run unchanged)
- packages/escher-driver/src/mcp_tests.rs · edits the test `the_listing_is_the_twelve_verbs_of_the_table`
- tests/blitz-tests/tests/stand_requirement_sweep.rs · edits the test `every_0_2_0_requirement_names_its_witness_inside_stand_repeat`
- tests/blitz-tests/tests/stand_lint.rs · re-reads stand_lint, unchanged, in both layout modes
- .github/scripts/test_blitz_tests_targets.py · re-reads the blitz-tests target pin (`BlitzTestsTargetsTest`)
- .github/scripts/ci-leg.sh · re-reads the `fast` and `doc` legs
- .andromeda/runs/2026-10-25T21-43-01-code-audit/over_ceiling.py · re-reads the audit's over-ceiling complexity reading

## Needs
- escher-driver's unit tests: lint and MCP are among what they pin. The lint's inline unit tests pass unchanged, and the MCP listing test still asserts twelve tools in `VERBS` order, `lint` after `screenshot`, `help` last, each described by its help and its result fields.
- The visual lint's output as the integration checks read it: the same `Finding`s in document order, then `Defect::ALL` order, the same six defects, `fonts-unavailable` refused before anything is read, and nothing changed on the held instance. `Session::lint` is read by integration checks, not unit tests, because escher-driver cannot build a held instance.
- The lint's readers it keeps calling unchanged: `BaseDocument::text_extent` and `Harness::text_contrast` under the stand contrast exempt rule, through the helpers the factoring adds.
- The session-held stand path the lint check runs on: `Session` booted through `seven_guis::stand` (`session_common::hold`, `lint`, `linted`), in both layout modes, with the stand's boot-time readings (counter, timer, CRUD and load page 0 findings, flight booker one `overlap`) unmoved.
- The requirement-sweep stand check as a blitz-tests target. It declares the shared `common` and `session_common` modules, and its witness rows (each 0.2.0 requirement naming a witness inside stand-repeat) are asserted unchanged.
- The stand-repeat leg runs the stand checks for three rounds with no retry under a zero-failure budget. Both refactored tests, and the lint check, stay green across all three rounds.
- The two builds the workspace runs (`cargo test --workspace` and `-p blitz-tests`, which differ in engine features): both touched tests pass in each.
- Evidence that the same rows are asserted: dropping a row from the table a refactored test walks (a `VERBS` row, a requirement-witness row) still turns it red. P3 names the evidence form.
- The fast gate (fmt, clippy `-D warnings`, workspace tests, ci-scripts) and the doc leg (rustdoc `-D warnings`) pass on the refactored code.
- escher-driver's per-crate line-coverage floor (97 %) in the coverage leg. The new helpers in `findings` stay covered by the lint's existing checks, so the crate does not drop below its floor.
- The agent-run tally: the chunk adds or removes no test, so `run stand` stays at 205 passed · 0 failed · 7 ignored over 50 files.
- The driver's log line rule: `Session::run` and `Session::lint` leave only the `command` span of fixed words and counts. A lint helper adds no print, log site, span or field, and never carries a finding or its count.
- The test-name and assertion-message rule: a refused call's id, name, value or call-supplied argument never appears in a test name or assertion message. A table walk's failure message carries a row index or a fixed schema word only.
- `mcp_tests.rs` is a `#[cfg(test)]` `#[path]` child of `mcp.rs` with its test paths unchanged, so the listing test keeps its path and name.

## Standing conventions
- S1 · A stand check reads the tables and helpers it shares from `tests/common/mod.rs` and `tests/session_common/mod.rs`, declared after its `use` block. Those tables are stated expectation, never a second mapping, and a new reader moves that module's recorded reader count · §3 Test Harness Contract (blitz-tests stand checks · session checks)
- S2 · A unit-test module moved out of line sits in a file of its own, declared `#[cfg(test)]` with `#[path]` at its parent, its test paths unchanged · §4 Unit Test Strategy, Conventions (unit-test-strategy.md)
- S3 · Assertions over a call carry a row index only, so a failing log prints no argument value · §4 Unit Test Strategy, escher-driver (unit-test-strategy.md)
- S4 · blitz-tests keeps one test-target file per behaviour, pinned by `BlitzTestsTargetsTest`, with the shared modules no target · §2 Test Strategy, Directory pattern (test-strategy.md)
- S5 · Session-held stand checks run in both layout modes, with no sleep and no wall-clock read in a check · §5 Integration Test Strategy (integration-test-strategy.md)
- S6 · A green per-package run is not the workspace leg's green, so a check is read under both builds · §9 CI Integration, Engine features by runner (ci-integration.md)

## Not needed
- blitz-dom's engine tests: the `document.rs` and `mutator.rs` pipeline and unit tests, the incremental oracle, and hover and font-face loading. The engine function stays untouched, and no engine code moves.
- The range-input, select, clock, scrollbar and text-input unit tests in blitz-dom's `node/` and `clock.rs`. No engine behaviour changes.
- The WPT runner, its conformance runs, scoring, quarantine list and the upstream-only WPT workflows.
- The apps/browser tests (about pages, history, favicon, URL suggestions, persistence) and the human-read example fixtures under `examples/assets`.
- blitz-vibey-script's DOM, Selection and Preact TodoMVC tests.
- The cold-agent run pipe, its live run, its verdict fields and its contract tests. No agent run is fired.
- The session socket host lifecycle, idle expiry and state directory checks. No carrier or wire changes.
- The command line `escher-session` process checks and the shell flows. No argv, exit status or answer form changes.
- The session recording, kept checks, replay of the committed Counter checks, and `record_checks.sh`. No recorded answer changes.
- Pixel tests, the headless screenshot and its byte-identity across runs.
- Hand-written network and time stand-ins (`HeldNet`, `LoadNet`, `LateNet`, `TimerTicks`). The chunk adds or changes no stand-in.
- The CI matrix (windows, macos, ios, android builds), MSRV build, package-install bounding and caches. No workflow or job changes.
- The mutation tier: a dev-host cargo-mutants run, which the scope does not ask for.
- The benchmark tests (`paint_tree_bench`) and the `PAINT_TREE_BENCH_HTML` seed.
