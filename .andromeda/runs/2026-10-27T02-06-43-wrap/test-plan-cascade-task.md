# Task — test-plan · 2026-10-26-codebase-health

## The session
Codebase health (v030-01): three functions over a cognitive-complexity ceiling of 15 were split into private helpers, and the engine function over the ceiling was left alone. No public symbol, verb, field, carrier, socket, port, env var, crate, module, dependency, CI leg or script changed.
In escher-driver's lint.rs, `findings` now walks a new private `readings` (the entry's defect and reason pairs in push order: zero-size alone for a box with no area, else placement, then truncated, then contrast) and a new private `placement` (the off-screen or clipped match). `Session::lint` and every reader are untouched, and the findings and their order are unchanged.
In mcp_tests.rs, the test `the_listing_is_the_twelve_verbs_of_the_table` now checks each row through three new private `#[track_caller]` helpers: `assert_described`, `assert_schema` and `assert_property`. The file still holds 16 `#[test]`, 15 on unix.
In stand_requirement_sweep.rs, `every_0_2_0_requirement_names_its_witness_inside_stand_repeat` now checks each row through the new private `#[track_caller]` helpers `assert_0_2_0_row` and `assert_0_2_0_witness`. The new private predicate `inside_stand_repeat` holds the path-prefix constants `BLITZ_TESTS` and `SEVEN_GUIS`. Every message is unchanged.
Measured on the dev host with rust-code-analysis-cli 0.0.25, test functions counted: functions over 15 went from 88 to 85. `findings` went from 16 to 7 and each of the two tests from 18 to 3. Each new helper reads 1–4, and `clear_interaction_state_for_removed_node` still reads 17.0. escher-driver's coverage moved to 4930/5031 = 97.99 %. CI#40434241601 is green 17/17, and its job tallies equal the base run's.
A claim was disproved by measurement. The sweep test reads `ci-leg.sh` whole, so dropping `select_interaction` from `stand-repeat` alone leaves it green, because the `a11y` leg still names that target. Stand-repeat membership is held jointly by the sweep test and `test_ci_workflows.py`'s `test_stand_repeat_leg_runs_three_rounds`. This property predates the chunk, and the chunk made no change for it.

## Reached files
- packages/escher-driver/src/lint.rs
- packages/escher-driver/src/mcp_tests.rs
- tests/blitz-tests/tests/stand_requirement_sweep.rs
- escher-0.3.0/verification-matrix.json
- escher-0.3.0/working-route.md
- .andromeda/master-route.md
- .andromeda/friction-log.ndjson
- escher-0.3.0/chunks/2026-10-26-codebase-health/scope.md
- escher-0.3.0/chunks/2026-10-26-codebase-health/research.md
- escher-0.3.0/chunks/2026-10-26-codebase-health/plan.md
- escher-0.3.0/chunks/2026-10-26-codebase-health/evidence/complexity.md
- escher-0.3.0/chunks/2026-10-26-codebase-health/evidence/counts.md
- escher-0.3.0/chunks/2026-10-26-codebase-health/evidence/row-controls.md
- escher-0.3.0/chunks/2026-10-26-codebase-health/evidence/operator-pass.md
- escher-0.3.0/chunks/2026-10-26-codebase-health/evidence/ci.md

## Touches
- §10 escher-driver coverage reading (quality-gates-coverage-targets, its last reading 4918/5019 = 97.99 %): now 4930/5031 = 97.99 %, with 12 more counted lines and 12 more covered. The 97 % floor holds. dioxus-native-dom (2741/3202 = 85.60 %) and blitz-test-harness (952/1008 = 94.44 %) are unchanged, and the leg still reads 3 of 3 crates at or above their floors.
- §1 / §4 citation of lint.rs's inline test module: `packages/escher-driver/src/lint.rs:526-604` is now `:542-620`. The file grew from 604 to 620 lines. This reaches the lint test-module citations in test-scope-summary and unit-test-strategy.
- §1 / §4 citations into mcp_tests.rs: `:754-785` → `:771-802`, `:1-786` → `:1-803` and `:615-717` → `:632-734`. Each moved +17, and the file grew from 786 to 803 lines. `:167-196` is unmoved. This reaches test-scope-summary and unit-test-strategy.
- §1 citation of the 0.2.0 sweep test: `tests/blitz-tests/tests/stand_requirement_sweep.rs:1015-1090` now runs from the test through its three helpers to `:1109`. The file grew from 1090 to 1109 lines. The file's other spans are unmoved: `:92-218`, `:245-540`, `:978-1013`, `:231-233`, `:542-837` and `:839-863`.
- §1 description of the 0.2.0 sweep test (that `ci-leg.sh` holds the glob and `--test {name}` for all 18): re-read, and it states the test's read exactly. Its rows, witnesses and table-order and leg checks are unchanged. Its per-row and per-witness assertions now live in private helpers.
- §1 escher-driver scope entry (the executor's verbs and `Session::lint` read by integration checks, not unit tests): the lint's findings walk is now three private functions. `Session::lint`'s findings, order and `fonts-unavailable` refusal are unchanged, and `stand_lint` is still green, 7 tests, in both layout modes and both builds.
- §4 escher-driver unit-test coverage (MCP among the pinned subjects): the MCP listing test keeps its name, its outer assertions and its row walk. Its description, schema and property checks moved into helpers, and every assertion is kept.
- §4 assertion helpers: escher-driver's MCP listing test now asserts per row through `#[track_caller]` helpers whose messages carry row and index only, beside the standing `assert_solves`.
- Test inventory counts re-read and unmoved: escher-driver 100 unit tests; `mcp_tests.rs` 16 `#[test]` (15 on unix); `stand_requirement_sweep` 3; `stand_lint` 7; blitz-tests 147 `.rs` files and 146 targets; escher-driver's seventeen private modules.
- Agent-run readings re-read and unmoved: `run stand` 205 · 0 · 7 over 50 files, and `boot` ready.
- Leg tallies re-read and unmoved: a11y 14 · 85 · 0 · 0; stand-repeat 3 of 3 rounds, 204 · 864 · 0 · 21; workspace 208 · 1117 · 0 · 12; CI scripts 108.
- §9 per-OS test tallies on CI#40434241601 equal the base run CI#40389488295: Test (macos) 189 · 1114 · 0 · 12; Coverage 189 · 1114 · 0 · 12; Test (windows) 189 · 1068 · 0 · 9. The sweep test's per-round test count in ci-integration is unmoved.
- §9 / §10 stand-repeat leg and flakiness budget: the sweep test's claim to hold the leg's named targets is narrower than it reads. It cannot see a target dropped from `stand-repeat` alone while the `a11y` leg still names it. `test_ci_workflows.py`'s `test_stand_repeat_leg_runs_three_rounds` asserts `select_interaction` among that leg's logged words. This reaches any test-plan line that credits stand-repeat membership to the sweep test alone.
- §1 citations of `stand_lint.rs` (test-scope-summary): unmoved, since `stand_lint.rs` was not edited. These citations match a `lint.rs:` token, but their file is not the edited lint.rs.

## New facts
- F1 · quality-gates-coverage-targets · A cognitive-complexity ceiling of 15 is read on the dev host with rust-code-analysis-cli 0.0.25, counting test functions. It has no CI leg, and no lockfile resolves the tool. At this chunk 85 functions read over 15, among them blitz-dom's `clear_interaction_state_for_removed_node` (17.0) and `get_text_selection_ranges` (32.0), both left alone.
- F2 · quality-gates-coverage-targets · rust-code-analysis's `cognitive.sum` adds nested spaces into their parent, so a closure or nested `fn` keeps its branches counted in the enclosing function. Only splitting a function into module-level sibling items lowers its reading.
- F3 · unit-test-strategy · escher-driver's `the_listing_is_the_twelve_verbs_of_the_table` asserts each listed tool through three private `#[track_caller]` helpers: `assert_described` (the description and its result fields), `assert_schema` (the input schema's properties and their order) and `assert_property` (one argument's kind, enum and maximum). Their messages name the row and index only.
- F4 · test-scope-summary · The 0.2.0 sweep test asserts each requirement row through private `#[track_caller]` helpers `assert_0_2_0_row` and `assert_0_2_0_witness`. Its target predicate `inside_stand_repeat` holds the path prefixes `BLITZ_TESTS` and `SEVEN_GUIS`.
- F5 · test-scope-summary · The sweep test reads `ci-leg.sh` as a whole text with `contains`. A named target dropped from the `stand-repeat` leg alone stays green while the `a11y` leg still names it, and only removing both sites reads red. Stand-repeat membership is therefore held jointly with `test_ci_workflows.py`'s `test_stand_repeat_leg_runs_three_rounds`. This gap predates the chunk, and its owner is to be placed at P5.

## Untouched
- §3 agent-run contract and `scripts/agent-run.sh`: the script, its exit grammar, events and state are unchanged, and the `run stand` reading is unmoved.
- §3 cold-agent run pipe: `scripts/cold-agent.sh`, its verdict fields, the 5 % bar and the live-run proof are unchanged.
- §3 blitz-test-harness `Harness` API, including settle, screenshot, contrast, input and inspection helpers: no harness file was edited.
- §3 shared test modules `tests/common/mod.rs`, `tests/session_common/mod.rs` and `tests/fixtures/mod.rs`: unchanged, and their reader counts are unmoved.
- §3 stand log format and telemetry: no span, field or event was added, and escher-driver still has 2 `info_span!` and no `#[instrument]`.
- §9 ci.yml jobs, caches, failure-log uploads, package installs and `ci-leg.sh` legs: byte-unchanged, including the stand-repeat leg's ten named targets and `--test select_interaction` at both of its sites.
- §10 coverage floors (escher-driver 97 %, dioxus-native-dom 83 %, blitz-test-harness 90 %) and the `keyboard_compat.rs` exclusion: unchanged. Only the escher-driver reading moved.
- §10 mutation tier and the `.config/nextest.toml` `mutants` profile: no mutation run in this chunk.
- §7 / §8 the committed checks under `tests/blitz-tests/checks/`, the stand's `LoadNet`, `HeldNet` and the fixture pages: byte-unchanged.
- §1 / §4 blitz-dom scope and unit tests: nothing under `packages/blitz-dom/` changed.
- §1 seven_guis stand, its six served tasks and the `escher-session` binary checks: unchanged.
- §1 / §5 / §6 WPT runner, apps/browser, blitz-vibey-script and examples/assets: not reached.
- §3 third-party test frameworks and dependencies: no `Cargo.toml`, `Cargo.lock` or `deny.toml` change.
