# Task — test-plan · 2026-10-31-engine-debts

## The session
Engine changes for every document, each on the founder's word: `visible_region` reads clipping boxes through their transforms, a transform with determinant 0 or not finite takes no hit for its subtree, a focused node coming under `inert` loses focus with no `blur`, the tree's focus falls back to the window when the focused node was not built, a native checkable control writes `toggled`, a `multiple` select and `optgroup` work in keys, pointer, form, tree, sheet, snapshot and bridge, and a checkbox or radio click fires `change` from the engine. No popup is built and `packages/blitz-paint/` shows no diff.
Driver and stand: `type` works on a `multiple` select, the lint skips a closed select's options, and Cells' `cells-grid` becomes one Tab stop with arrows and Enter; no verb, key name, result field, cause, session error or exit was added.
Tests: 5 new blitz-tests targets (`visible_region_transformed` 6, `hit_non_invertible_transform` 2, `open_state` 4, `checkable_change_event` 3, `text_range_no_change_event` 2), 13 edited checks, `tests/common/mod.rs` edited, unit tests added in blitz-dom, blitz-shell and blitz-vibey-script `dom`, and `stand-repeat` names fifteen targets where it named ten.
Measured: the verdict run is CI#41192433140 at `40ac6ddac`, green, 17/17, wall 1271 s; CI#41190771843 at `42eed99a6` went red on `Stand checks repeated` with a runner out of disk, and `ci.yml` gained a free-disk step whose cure stays unproven.
Retired: `runtime/events.rs`' `change` synthesis, the pin name `a_target_inside_a_translated_scrolling_box_reads_outside_its_visible_region`, `inert_subtree::removing_inert_restores_focus_hit_and_tree`'s focus-fixup limit lines, and upstream's both-modes exception in `pseudo_element_flow_classification`.
The change reached 47 files and the list below holds the 40 nearest test-plan, its limit; the seven left out are `packages/blitz-dom/assets/default.css`, `packages/dioxus-native-dom/src/lib.rs`, `packages/escher-driver/src/lib.rs`, `packages/escher-driver/src/refusal.rs`, `packages/escher-driver/src/schema.rs`, `docs/capabilities.md` and `Cargo.lock`.

## Reached files
- packages/blitz-dom/src/scrolling.rs
- packages/blitz-dom/src/document.rs
- packages/blitz-dom/src/node/node.rs
- packages/blitz-dom/src/node/select.rs
- packages/blitz-dom/src/mutator.rs
- packages/blitz-dom/src/accessibility.rs
- packages/blitz-dom/src/events/keyboard.rs
- packages/blitz-dom/src/events/pointer.rs
- packages/blitz-dom/src/form.rs
- packages/blitz-vibey-script/src/runtime/events.rs
- packages/blitz-vibey-script/tests/dom.rs
- packages/blitz-shell/src/event.rs
- packages/dioxus-native-dom/src/snapshot.rs
- packages/dioxus-native-dom/src/dioxus_document.rs
- packages/escher-driver/src/execute.rs
- packages/escher-driver/src/lint.rs
- packages/escher-driver/src/schema_tests.rs
- examples/seven_guis/src/tasks/cells.rs
- .github/scripts/ci-leg.sh
- .github/scripts/test_ci_workflows.py
- .github/workflows/ci.yml
- tests/blitz-tests/tests/visible_region_transformed.rs
- tests/blitz-tests/tests/hit_non_invertible_transform.rs
- tests/blitz-tests/tests/open_state.rs
- tests/blitz-tests/tests/checkable_change_event.rs
- tests/blitz-tests/tests/text_range_no_change_event.rs
- tests/blitz-tests/tests/transformed_client_rect.rs
- tests/blitz-tests/tests/stand_act_transformed.rs
- tests/blitz-tests/tests/inert_subtree.rs
- tests/blitz-tests/tests/accessibility_states.rs
- tests/blitz-tests/tests/select_interaction.rs
- tests/blitz-tests/tests/stand_act_select.rs
- tests/blitz-tests/tests/stand_lint.rs
- tests/blitz-tests/tests/stand_keyboard.rs
- tests/blitz-tests/tests/stand_contrast.rs
- tests/blitz-tests/tests/stand_load_page.rs
- tests/blitz-tests/tests/stand_cells_served.rs
- tests/blitz-tests/tests/pseudo_element_flow_classification.rs
- tests/blitz-tests/tests/stand_non_lean_tasks.rs
- tests/blitz-tests/tests/common/mod.rs

## Touches
- blitz-tests `.rs` files 148 → 153 and test targets 147 → 152: `test-strategy.md:54`; other 148 hits are citation digits.
- `stand-repeat`'s named targets ten → fifteen: `test-plan.md:377`, `:413`, `ci-integration.md:9`, `:16`, `quality-gates-coverage-targets.md:22`.
- `test_stand_repeat_leg_runs_three_rounds` names the five more targets, so an inventory of test_ci_workflows.py's pins over the leg moves; `BlitzTestsTargetsTest`'s manifest and `Ran 108 tests` hold.
- Workspace tally `209 · 1125 · 0 · 12` → `214 · 1181 · 0 · 12` at `ci-integration.md:71`, with a new row beside `:50`.
- Per-OS rows gain CI#41192433140 beside `ci-integration.md:49` and `:50`: macos and coverage `195 · 1178 · 0 · 12`, windows `195 · 1132 · 0 · 9`, a11y `14 · 112 · 0 · 0`, stand repeat `219 · 1008 · 0 · 21`.
- `a11y` leg 14 · 85 → 14 · 112, fourteen files unmoved: `ci-integration.md:8`, `:73`.
- `agent-run.sh run stand` `205 · 0 · 7` → `222 · 0 · 7` over 50 files, and `run all` reads `681 · 0 · 12` where the last stated total was 633: `test-plan.md:190`, and any proof chain continuing past `test-plan.md:176`.
- Coverage reads at `quality-gates-coverage-targets.md:16`: escher-driver 98.04 %, dioxus-native-dom 86.12 %, blitz-test-harness 94.84 %, on CI#41192433140; the floors do not move.
- Unit counts: blitz-dom 106 → 112, blitz-shell 0 → 1, blitz-vibey-script `dom` 30 → 31 (`test-scope-summary.md:71`); escher-driver 100 holds at `test-scope-summary.md:59` and dioxus-native-dom 82 holds.
- Target counts in test-scope-summary: `inert_subtree` 4 → 6 (`:105`), `accessibility_states` 5 → 6 (`:102`), `select_interaction` 7 → 18 and `stand_act_select` 5 → 10 (`:131`), `stand_act_transformed` 2 → 3 (`:118`), `stand_lint` 7 → 8 (`:128`), `stand_keyboard` 15 → 21 (`:121`), `stand_contrast` 5 → 7 (`:120`), `stand_load_page` 2 → 3 and `stand_cells_served` 4 → 5 (`:150`).
- The same counts at `ci-integration.md:73` for `inert_subtree`, `accessibility_states`, `select_interaction`, `stand_act_select`, `stand_keyboard` and `stand_contrast`; that line also names pulldown-cmark, which moved 0.13.4 → 0.13.5 in `Cargo.lock` for RUSTSEC-2026-0414.
- The contract `.andromeda/registries/contracts/test-plan/session-lifecycle.md:11`: `stand_act_transformed` 3 and `stand_act_select` 10; its 100 unit tests hold.
- `stand_lint`'s `at_boot` `[0, 1, 0, 0]` → `[0, 0, 0, 0]`: `test-scope-summary.md:129`, `:153`; `:109` holds, a closed select's options keeping its client rect.
- Fixture readers of `mod fixtures` 24 → 28: `test-plan.md:151`, `test-strategy.md:53`; the 22 and two split there and at `test-plan.md:190` moves with it, the report giving the total alone; `session_common` 31 holds at `test-plan.md:149`, `test-strategy.md:52`.
- The select's limits are now asserted: `test-plan.md:19` says they are asserted by no check and names a single-choice model, and `test-plan.md:17` says a named select's entries have no test of their own, where `form.rs` gained `a_multiple_select_submits_its_set_less_its_disabled_rows` and `a_single_select_submits_its_choice_through_a_group`; `no-assert-gap` also fires at `integration-test-strategy.md:8`.
- The `:open` gap closes through `open_state`: `test-plan.md:19`, `test-scope-summary.md:34`.
- The redraw-route gap closes through blitz-shell's unit test: `test-scope-summary.md:104`.
- The both-modes exception ends, `pseudo_element_flow_classification`'s three tests now running `for incremental in [false, true]`: `ci-integration.md:16`, `integration-test-strategy.md:8`, `test-scope-summary.md:32`.
- `visible_region`'s limit moves at `test-scope-summary.md:118`: exact under translate and scale, the rotated padding box's bounding box under a rotation, `None` on a non-finite edge; `transformed_client_rect`'s pin is renamed `…reads_inside_its_visible_region` and inverted; `ci-integration.md:37` only names the reader and holds.
- The focus fixup's limit moves at `test-scope-summary.md:105`: focus clears under `inert` with no `blur` or `focusout`, and the old limit lines of `removing_inert_restores_focus_hit_and_tree` are gone, its name and first assertions standing.
- `multiple` and `optgroup` limits move at `test-scope-summary.md:24`; a closed select's option stays refused `covered`, and `:131`'s statement of that holds with its counts moved.
- Indirect, through `runtime/events.rs` losing its `change` synthesis: script-event coverage of blitz-vibey-script's `dom` target now hears one `change`, from the engine; 7 of the `change-event` hits are the harness's input synthesis and hold.
- The load page's rows, `test-scope-summary.md:150`: its Tab sequence, button keys and ratios are now asserted, and its own load wait is unchanged.
- `cells-grid` joins the statement at `test-scope-summary.md:101`, where 676 stays the true count of data cells; `stand_non_lean_tasks`' reacting-element count moves 676 → 677 because `cells-grid` has an `onkeydown` and no role.
- Indirect, through the keyboard rows: §5's Session and held-instance group (`test-plan.md:260` and its detail) now holds Cells walked by keys, a commit read in the first part, and one pointer path moved, a click inside a cell's own open editor returning early.
- `tests/common/mod.rs` gains 6 lines at `:93-98`, so its citations past `:93` in §3 (`test-plan.md:146-147`) shift.
- Line citations past the added text shift across the detail and the body in `document.rs`, `mutator.rs`, `node/node.rs`, `form.rs`, `dioxus_document.rs`, `snapshot.rs`, `node/select.rs`, `stand_keyboard.rs` and `select_interaction.rs`; `cites.py` re-points them and no claim moves with them.
- `ci.yml` citations past `:311` shift by 6 (the matrix and `needs` lines of §1 and §9) and `ci-leg.sh` citations past `:58` shift by 2 (`:81-86`, `:89-90` in §9).
- §4's blitz-dom node and form bullets (`test-plan.md:223`, `:227`) grow: `node/select.rs`' stated tables now hold steps, spans and groups, and `form.rs`' cited test range moves.

## New facts
- F1 · test-scope-summary · `visible_region_transformed` (6, both layout modes) pins a world panned under an untransformed viewport, a target in a node body under a transformed world, a page with no transform in the client rects' frame, a translate, a box mapped to a point leaving nothing visible, and a rotation reading the bounding box.
- F2 · test-scope-summary · `hit_non_invertible_transform` (2) pins that a box under a transform with determinant 0 or not finite takes no hit and that an invertible twin is hit where it shows; no standing text states this rule yet.
- F3 · test-scope-summary · `open_state` (4) pins `:open` read from the attribute at construction, moved by a mutator write and removal, and flipped by a summary's click and Enter.
- F4 · test-scope-summary · `checkable_change_event` (3) pins a click and a Space on a checkbox firing `input` then `change`, a radio's `change` firing on an actual change alone, and a label's click reading as its control's.
- F5 · test-scope-summary · `text_range_no_change_event` (2) pins typing in a text input and a range's key and pointerdown firing `input` and no `change`.
- F6 · test-scope-summary · `stand_act_transformed` gains `a_control_in_a_scrolling_box_under_a_zoomed_world_is_reached_where_it_shows`, the driver reaching a control under a zoomed world.
- F7 · test-scope-summary · `select_interaction` now pins a `multiple` select's rows having their own boxes, a 17-row key table, typed text, a 10-row click table, a disabled select, removal of `multiple` settling to one option, a seeded set stepped from its last chosen row, and groups on both select kinds.
- F8 · test-scope-summary · The chosen-row pair `#c8c8c8` on `#000000`, an engine default in `default.css`, is measured at 12.56:1 by `select_interaction`.
- F9 · integration-test-strategy · `stand_act_select` now drives a `multiple` select through the driver: name, role and rows with their own bounds, an accepted row click, a Shift press spanning rows, `type` choosing its match alone with no stray click, and an option under a closed select's group refused `covered`.
- F10 · test-scope-summary · The driver cannot build a non-adjacent `multiple` set, `press` holding no Ctrl, and Ctrl+A is not modelled, so no driver check reaches either.
- F11 · test-scope-summary · `stand_lint`'s planted overlap case is two overlapping rows of a `multiple` select, reading exactly `fx-row-a` × `fx-row-b`, and a closed select's options are excluded by element, never by role.
- F12 · test-scope-summary · `accessibility_states` gains `a_click_and_a_space_move_a_native_control_in_the_tree`; a native control writes `toggled` under a checkable role alone, so `pressed` never reads on it.
- F13 · test-scope-summary · `inert_subtree` gains `a_node_made_inert_loses_focus_and_takes_no_key` over five ways of being made inert, and a check that the tree's focus reads the window when the focused node was not built.
- F14 · test-scope-summary · `stand_keyboard` now pins Cells' Tab sequence, arrows clamped at the grid's ends and bringing a cell into view, Enter opening the cell's editor and committing by keyboard, Tab leaving an open editor committing it, a click on another cell committing to its own cell, and the load page's sequence with its button taking Enter and Space.
- F15 · test-scope-summary · `stand_contrast` adds `the_load_page_meets_its_ratios` and `cells_meets_its_ratios`, the `.cell.current` focus outline using the stand accent `#3a5ce5`.
- F16 · test-scope-summary · `stand_cells_served` gains `the_keyboard_walks_to_a_cell_and_commits_a_value_read_in_the_first_part`, where `press` walks and opens the editor, `type` supplies the characters and `press` commits, the twelve key names holding no character.
- F17 · test-strategy · `tests/common/mod.rs:93-98` states `LOAD_TAB_ORDER` and `CELLS_TAB_ORDER` beside the shared tables.
- F18 · unit-test-strategy · `packages/blitz-shell/src/event.rs:128-172` holds the crate's first unit test, `a_redraw_request_reaches_the_receiver_and_wakes_the_loop_once`, which states what it cannot read.
- F19 · unit-test-strategy · `form.rs`' test module gains two tests over the select's submitted entries, a `multiple` set less its disabled rows and a single choice through a group, using neither `.unwrap()` nor `.expect(`.
- F20 · unit-test-strategy · `node/select.rs`' tests gain a step moving from the row it is given, a span listing enabled rows either way round, and the option list running through a group in tree order.
- F21 · integration-test-strategy · blitz-vibey-script's `dom` target gains `radio_click_fires_change_only_where_it_was_not_checked`.
- F22 · e2e-test-strategy · Cells' synchronous `MountedData` handle calls are measured headlessly and their windowed path is read from the same lines and not run; the async form tried first panicked with `RefCell already borrowed` at `events.rs:162`.
- F23 · ci-integration · `Stand checks repeated` runs `jlumbroso/free-disk-space@54081f138730dfa15788a46383842cd2f914a1be` after the toolchain and before `Swatinem/rust-cache`, with `android: true`, `large-packages: false`, `swap-storage: false` (`ci.yml:311-316`).
- F24 · ci-integration · That step ran `completed · success`, taking `/dev/root` from 86 G to 109 G free, and is not shown to be the cure: the green runner had 86 GB free before it, the failed run has no `df` line, and the red job alone among `ubuntu-24.04` jobs stood on the older image `20261018.96.1`.
- F25 · ci-integration · The older free-disk step, now `ci.yml:392-394`, is skipped on every matrix row: its `if:` names `ubuntu-24.04` and `ubuntu-24.04-arm` and the android row runs `ubuntu-latest` (`completed · skipped` on CI#41190771843's `Test (android)`).
- F26 · ci-integration · CI#41190771843 at `42eed99a6` ended red on `Stand checks repeated` alone (job 124508189681, linker bus errors, `Free space left: 5 MB`, no test ran).

## Untouched
- WPT: `wpt.yml`, `wpt-post-results.yml`, `wpt/runner`, its CLI, scoring, quarantine and fixtures; the session reached none of them and WPT stays upstream-guarded.
- `apps/browser` tests and helpers, and the no-test statements for `apps/` other than seven_guis.
- `packages/blitz-paint/`: its inline tests, the password mask checks and the paint comments; it shows no diff.
- The harness's API and semantics in `packages/blitz-test-harness/`: construction, pump, settle, screenshot, contrast, input and inspection helpers; only its coverage read moves, and that is a Touches item.
- Stand log format and sink coverage: `escher-telemetry`, the `telemetry_*` targets, `host_log`, `mcp_log` and the 2 `info_span!` sites; no log site was added and the needles read 0.
- The agent-run contract's verbs, exit grammar, events and state, and `scripts/agent-run.sh`; the script is unchanged, `stand_*.rs` stays at 50, and only the stated totals move.
- The cold-agent pipe, `scripts/cold-agent.sh`, `test_cold_agent.py` and its live proof; not re-run.
- Python test inventories of `test_wpt_diff_to_pr.py`, `test_wpt_area_changes.py`, `test_agent_run.py`, `test_cold_agent.py` and `test_capability_statement.py`; each is unchanged and `Ran 108 tests` holds.
- Mocking: `HeldNet`, `LateNet`, `LoadNet`, `TimerTicks` and the real pinned font; `tests/fixtures/mod.rs`' own pages and `examples/seven_guis/src/stand.rs` are not reached, only the reader count moves.
- Data sources: `examples/assets`, `examples/preact`, the WPT checkout and the committed checks under `tests/blitz-tests/checks`.
- Coverage floors 97, 83 and 90, `coverage_floor.py` and the `keyboard_compat.rs` exclusion; the floors are unmoved and only the reads change.
- Mutation tier and the complexity gate: `.config/nextest.toml` is not reached, and this chunk's complexity readings have no site in the plan.
- History rows recording a measure at a past run stay as they read: `test-plan.md:176`, `ci-integration.md:47`, `:48`, `:49`, `:50`, `:70`; a new row may sit beside them.
