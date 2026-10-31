## 2026-10-31-engine-debts — Session lifecycle: `type` on a `multiple` select and the driver-action totals
**Section:** §3 Test Harness Contract → Session lifecycle (registries/contracts/test-plan/session-lifecycle.md: session-start, session-proof)
**Change:**
- session-start: `type` replaces after its focusing click, a `multiple` select focused with none (was the focusing click on every target).
- session-proof: `stand_act_transformed` was 2; now 3, the running file total was 50; now 51. `stand_act_select` was 5; now 10, the driver-action total was 55; now 61.
**Why:** the driver focuses a `multiple` select with no click, a click landing on the row at the centre, and a text opening on a space types nothing. The `multiple` select is the founder's fork, by dialog in the overseer session, 2026-10-31, relayed verbatim by the operator (inputs#I8, inputs#I9); that its text reaches `type`'s meaning is the operator's reading of it at P2 Validate, 2026-10-31 13:13Z, not a new word of the founder's (inputs#I16). The two targets count 3 and 10 at the chunk.
**Ref:** .andromeda/runs/2026-10-31T10-45-42-wrap/

## 2026-10-31-engine-debts — citations re-pointed to the lines the chunk moved
**Section:** §1, §2, §3, §4, §5, §7, §8, §9, §10 (test-plan.md and the registry's session-lifecycle.md; test-plan-detail: test-scope-summary.md, test-strategy.md, unit-test-strategy.md, integration-test-strategy.md, test-data-fixtures.md, mocking-stubbing-discipline.md, ci-integration.md, quality-gates-coverage-targets.md)
**Change:**
- `file:line` citations follow the lines this chunk's engine, driver, stand, test and CI edits moved. On many lines a citation alone moves; elsewhere it moves beside a change another entry records.
**Why:** the chunk's 47 changed files moved the cited lines. No claim moves with a re-pointed citation.
**Ref:** .andromeda/runs/2026-10-31T10-45-42-wrap/

## 2026-10-31-engine-debts — the a11y leg reads 14 · 112
**Section:** §9 CI Integration (ci-integration.md: Legs, the local `ci-leg.sh` readings)
**Change:**
- The a11y leg's 14 files read tests was 85; now 112, in Legs and in the local reading, which now reads as measured at this chunk's report.
- Per target in the local reading: `stand_contrast` was 5; now 7. `stand_keyboard` was 15; now 21. `accessibility_states` was 5; now 6. `inert_subtree` was 4; now 6. `select_interaction` was 7; now 18. `stand_act_select` was 5; now 10.
**Why:** the chunk's a11y tally reads 14 lines, 112 passed, 0 failed; the leg's fourteen files are unmoved and the tests inside six of them grew.
**Ref:** .andromeda/runs/2026-10-31T10-45-42-wrap/

## 2026-10-31-engine-debts — `stand-repeat` names fifteen targets
**Section:** §9 CI Integration (ci-integration.md: Legs; test-plan.md: Legs), §10 Quality Gates (quality-gates-coverage-targets.md, test-plan.md: Flakiness budget), §1 (test-scope-summary.md: the 0.2.0 sweep test's reach), §4 (unit-test-strategy.md: CI workflows and leg script)
**Change:**
- `stand-repeat`'s named blitz-tests targets were ten; now fifteen, adding `visible_region_transformed`, `hit_non_invertible_transform`, `open_state`, `checkable_change_event` and `text_range_no_change_event`; `test_stand_repeat_leg_runs_three_rounds` names the glob and the fifteen words, pinning `seven_guis`' two words beside the fifteen.
- The flakiness budget adds: with the fifteen, CI#41192433140 read 219 · 1008 · 0 · 21, +15 · +144, `stand-repeat: 3 of 3 rounds green`.
- The sweep test's reach: it asserts `select_interaction` and five more of the targets the leg names.
- unit-test-strategy.md: the three-rounds case names the leg's fifteen named targets. Its line splits in two before the `a11y` row, and three dated clauses leave it: `(the last three added at 2026-10-14-stand-a11y-assertions)`, `since 2026-10-14-quality-gates`, `(five until that chunk)`.
**Why:** the leg's script and its pin name the chunk's five new targets beside the ten. The 50 `stand_*.rs` files are unmoved.
**Ref:** .andromeda/runs/2026-10-31T10-45-42-wrap/

## 2026-10-31-engine-debts — every cargo leg `--locked` but `fmt` too
**Section:** §9 CI Integration (ci-integration.md: Legs; test-plan.md: Legs)
**Change:**
- Was every cargo leg `--locked` but `examples/wasm_hello`; now but `fmt` and `examples/wasm_hello`.
**Why:** read false against the leg script at this wrap's citation sweep: the `fmt` leg runs `cargo fmt --all --check` with no `--locked`, a second exception. The rest of the sentence holds.
**Ref:** .andromeda/runs/2026-10-31T10-45-42-wrap/

## 2026-10-31-engine-debts — `pseudo_element_flow_classification` runs both layout modes
**Section:** §9 CI Integration (ci-integration.md: Legs reaching it), §5 Integration Test Strategy (integration-test-strategy.md: Layout assertions ↔ both layout modes), §1 (test-scope-summary.md: flow classification)
**Change:**
- Was an exception to the rule that pipeline tests run `for incremental in [false, true]`, the file asserting neither mode pair; now it runs them as the pipeline tests do. `incremental_oracle` stays the layout-identity witness.
- ci-integration.md adds: its three tests run `for incremental in [false, true]`, as measured at this chunk's report; it holds no `stand_` prefix and is none of the fifteen named words.
- test-scope-summary.md adds: each test in both layout modes.
**Why:** the chunk's edit makes all three tests run the loop, ending upstream's both-modes exception, a gap the chunk carried.
**Ref:** .andromeda/runs/2026-10-31T10-45-42-wrap/

## 2026-10-31-engine-debts — the free-disk step of `Stand checks repeated`, and the red before it
**Section:** §9 CI Integration (ci-integration.md: four new `values · ci` records)
**Change:**
- Red: CI#41190771843 at `42eed99a6` ended red on `Stand checks repeated` alone, job 124508189681, linker bus errors, `Free space left: 5 MB`, no test ran.
- The step: the job runs `jlumbroso/free-disk-space@54081f138730dfa15788a46383842cd2f914a1be` after the toolchain and before `Swatinem/rust-cache`, with `android: true`, `large-packages: false`, `swap-storage: false`.
- Its cure is unproven: it ran `completed · success` on CI#41192433140, `/dev/root` from 86 G free to 109 G free; that runner had 86 GB free before the step, the failed run has no `df` line, and on that run the job alone among the `ubuntu-24.04` jobs stood on the older image `20261018.96.1`, whose free disk stays unread.
- The matrix job's older free-disk step never fires: its `if:` names `ubuntu-24.04` and `ubuntu-24.04-arm`, the android row runs `ubuntu-latest`, `completed · skipped` on CI#41190771843's `Test (android)`.
**Why:** the step is a scope widening on the operator's word, 2026-10-31, after the first run of the chunk went red; no boundary of the product moves. The readings are the chunk's own of both runs and every job's image.
**Kept:** the step stays, written and not claimed as the remedy.
**Ref:** .andromeda/runs/2026-10-31T10-45-42-wrap/

## 2026-10-31-engine-debts — CI tallies re-read at the final HEAD
**Section:** §9 CI Integration (ci-integration.md: the per-OS tallies, the workspace tally)
**Change:**
- Per-OS adds, on CI#41192433140 at `40ac6ddac`, green 17/17: linux 214 · 1181 · 0 · 12; macos and coverage 195 · 1178 · 0 · 12; windows 195 · 1132 · 0 · 9; the a11y job 14 · 112 · 0 · 0; the stand repeat 219 · 1008 · 0 · 21. Over CI#40487650912: +5 result lines and +56 passed on each of the four test jobs, no new result line and +27 passed on the a11y job, +15 · +144 on the stand repeat.
- The workspace tally adds: 214 result lines, 1181 passed · 0 failed · 12 ignored, +5 lines (`visible_region_transformed` 6, `hit_non_invertible_transform` 2, `open_state` 4, `checkable_change_event` 3, `text_range_no_change_event` 2) and +56 passed (those 17, 31 more in the edited checks, blitz-dom's lib 6, blitz-shell's lib 1, blitz-vibey-script's `dom` 1), timings not re-measured.
**Why:** the chunk's five targets and edited checks moved the tallies; CI#41192433140 is the run the chunk's verdict reads.
**Kept:** the earlier tally rows stand as history, each a past run's reading, the newest beside them.
**Ref:** .andromeda/runs/2026-10-31T10-45-42-wrap/

## 2026-10-31-engine-debts — Cells' keyboard walk, its handle calls and its open editor
**Section:** §6 E2E Test Strategy (e2e-test-strategy.md: new record), §5 Integration Test Strategy (integration-test-strategy.md: the Cells clause; test-plan.md: Session ↔ held instance), §1 (test-scope-summary.md: served pages, new record)
**Change:**
- e2e-test-strategy.md adds: Cells calls its `MountedData` handles synchronously, measured headlessly, the windowed path read from the same lines and not run; a `MountedData` call in a task spawned from a key handler panics with `RefCell already borrowed` at `events.rs:162`.
- integration-test-strategy.md adds: the keyboard walks to a cell and commits a value read in the first part, `press` walking and opening the editor, `type` supplying the characters, `press` committing; and a click inside a cell's own open editor returns early, so the text typed there stays.
- test-plan.md: was Cells walked in parts through the driver; now through the driver, in parts and by keys to a cell whose committed value reads in the first part.
- `stand_cells_served` was 4; now 5, the new record naming `the_keyboard_walks_to_a_cell_and_commits_a_value_read_in_the_first_part`, the driver's twelve key names holding no character.
**Why:** Cells' one Tab stop is the founder's fork, by dialog in the overseer session, 2026-10-31, relayed verbatim by the operator (inputs#I8, inputs#I9). The async form for the handles was tried, panicked and did not ship. The open-editor behaviour moves the plan's criterion that a pointer-opened editor reads as it did: the operator, by dialog at P2 Validate, 2026-10-31 13:13Z, restated the clause and kept the behaviour, the criterion being the plan's and the founder's fork saying nothing of it (inputs#I16); the keyboard path needs it.
**Ref:** .andromeda/runs/2026-10-31T10-45-42-wrap/

## 2026-10-31-engine-debts — `change` on a checkbox and a radio, from the engine
**Section:** §5 Integration Test Strategy (integration-test-strategy.md: new Script ↔ DOM record), §1 (test-scope-summary.md: blitz-vibey-script, two new tests/blitz-tests records)
**Change:**
- New record: dom.rs holds `radio_click_fires_change_only_where_it_was_not_checked`, a first click hearing `input,change`, a second adding `input` alone; a page's script hears one `change`, from the engine, `runtime/events.rs` synthesizing none.
- blitz-vibey-script's tests/dom.rs was 30; now 31 `#[test]` functions.
- New record: `checkable_change_event` 3, its own target, each in both layout modes: a click and a Space on a checkbox firing `input` then `change`, a radio's `change` firing on an actual change alone, a label's click reading as its control's.
- New record: `text_range_no_change_event` 2, its own target, each in both layout modes: typing in a text input, a range's key and its pointerdown each firing `input` and no `change`.
**Why:** the engine's `change` is the founder's fork, by dialog in the overseer session, 2026-10-31, relayed verbatim by the operator (inputs#I8, inputs#I9), its text read so by the operator at P2 Validate, 2026-10-31 13:13Z (inputs#I16). The engine sends `change` after `input`, a radio's only where it was not checked before the click, and the script runtime's synthesis is gone.
**Ref:** .andromeda/runs/2026-10-31T10-45-42-wrap/

## 2026-10-31-engine-debts — the `select` model's limits, now asserted
**Section:** §5 Integration Test Strategy (integration-test-strategy.md: new `multiple` select record), §1 (test-scope-summary.md: blitz-dom covered behaviours, `select_interaction`, `stand_act_select`; test-plan.md: covered behaviours), §2 (test-strategy.md: Claims mapped to checks), §4 (unit-test-strategy.md: new `type`'s help record)
**Change:**
- Was the single-choice `select` model, its limits (`multiple` and `optgroup` not modelled, no popup painted, an option with no author key reading in `unkeyed_actionable`) asserted by no check; now it models a `multiple` select and an `optgroup`'s options, asserted by `select_interaction` and, through the driver, `stand_act_select`. No popup is painted and Enter and Space open nothing on a single select; a closed single select's option is refused `covered`, under a group too; `size` is unread; a group's label is named in the tree and unpainted. The `unkeyed_actionable` reading alone stays asserted by no check.
- test-plan.md: the model reads single and `multiple`, with `optgroup`; the select's limits are asserted by those two checks; the `Parallel` traversal lock stays asserted by no check.
- `select_interaction` was 7; now 18. `stand_act_select` was 5; now 10, new record: name, role and rows each with its own bounds, a row click, a Shift span, `type` choosing its match alone with no stray click, a closed select's group option refused.
- test-strategy.md: the limits stand in two bullets of `docs/capabilities.md`, the keys, rows and groups pinned by those checks.
- schema_tests.rs pins `type`'s help on a `multiple` select inside a standing test.
**Why:** `multiple` and `optgroup` are the founder's forks, by dialog in the overseer session, 2026-10-31, relayed verbatim by the operator (inputs#I8, inputs#I9), their reach the operator's reading of those texts at P2 Validate (inputs#I16). A group's `label` as a name stands on the P5 lean, not on the fork's text, as that reading records. No popup is the founder's fork for a separate chunk.
**Ref:** .andromeda/runs/2026-10-31T10-45-42-wrap/

## 2026-10-31-engine-debts — the event-loop stand-in and the redraw route's check
**Section:** §8 Mocking & Stubbing Discipline (mocking-stubbing-discipline.md: new row), §1 (test-scope-summary.md: the `focus_redraw` clause), §4 (unit-test-strategy.md: new blitz-shell record)
**Change:**
- New row, Event loop (blitz-shell): a test-local `CountedWakes` implements `EventLoopProxyProvider`, read by `a_redraw_request_reaches_the_receiver_and_wakes_the_loop_once` for a loop woken once.
- The windowed shell's route was one that holds no check of its own, the boot smoke its one reading; now read by that blitz-shell unit test, which states what it cannot read, the windowed `seven_guis_native` boot smoke and `focus_redraw` green beside it.
- New record: `packages/blitz-shell/src/event.rs` holds the crate's one unit test in an inline `#[cfg(test)]` module.
**Why:** the chunk closes this gap with blitz-shell's first unit test, 0 to 1.
**Ref:** .andromeda/runs/2026-10-31T10-45-42-wrap/

## 2026-10-31-engine-debts — coverage reads on CI#41192433140
**Section:** §10 Quality Gates & Coverage Targets (quality-gates-coverage-targets.md: the `*_tests.rs` reads)
**Change:**
- escher-driver was 4930/5031 = 97.99 %; now 4950/5049 = 98.04 %. dioxus-native-dom was 2741/3202 = 85.60 %; now 2786/3235 = 86.12 %. blitz-test-harness was 952/1008 = 94.44 %; now 956/1008 = 94.84 %. Now read on the `Coverage report` job of CI#41192433140, as this chunk's report states; `coverage floor: 3 of 3` stands.
**Why:** the chunk's green run re-read all three crates. The floors are unmoved.
**Kept:** the new and reworked functions' complexity readings take no row: they are dev-host readings against the ceiling, with no site in this plan, and the leg's gate prints what binds.
**Ref:** .andromeda/runs/2026-10-31T10-45-42-wrap/

## 2026-10-31-engine-debts — Cells' snapshot figures dated at `cells-grid`
**Section:** §7 Test Data & Fixtures (test-data-fixtures.md: Stand screen over the budget)
**Change:**
- Was 47,949 bytes in 740 lines, 5 parts of `SNAPSHOT_TEXT_BUDGET`; now the bytes and lines read as read before `cells-grid` took its `id` and `aria_label`, no later reading claimed, and the 5 parts as measured at this chunk's counts.
**Why:** the chunk measures `PARTS` 5 unmoved and takes no new byte or line reading, and `cells-grid`'s attributes come after those figures.
**Kept:** test-scope-summary.md's served-pages bullet keeps the same figures under its own older measure.
**Ref:** .andromeda/runs/2026-10-31T10-45-42-wrap/

## 2026-10-31-engine-debts — the load page's rows asserted
**Section:** §7 Test Data & Fixtures (test-data-fixtures.md: Network answer), §1 (test-scope-summary.md: Served pages and the stand's unit tests)
**Change:**
- The network row adds: the load page is also booted by `stand_keyboard`'s `the_load_page_walks_its_stated_sequence_and_its_button_takes_enter_and_space` and `stand_contrast`'s `the_load_page_meets_its_ratios`.
- `stand_load_page` was 2; now 3, its rows asserted beside its load wait by the file's third test, their Tab sequence and button keys by `stand_keyboard`, their ratios by `stand_contrast`.
**Why:** the chunk adds these rows' checks; the load page's rows are measured in all three targets.
**Ref:** .andromeda/runs/2026-10-31T10-45-42-wrap/

## 2026-10-31-engine-debts — a named select's entries read by `form.rs`
**Section:** §1 Test Scope Summary (test-scope-summary.md: blitz-dom document, mutator, net, form; test-plan.md: the same), §4 (unit-test-strategy.md: new blitz-dom form record)
**Change:**
- `#[test]` functions were 36; now 38. form.rs was 1, in its first test module; now 3 in its test module: `a_range_input_submits_its_current_value`, `a_multiple_select_submits_its_set_less_its_disabled_rows`, `a_single_select_submits_its_choice_through_a_group`.
- Was a named `select`'s entries have no test of their own; now read by form.rs's tests, a `multiple` set less its disabled rows and a single select's choice through a group.
- New record: the two tests, which use neither `.unwrap()` nor `.expect(`.
**Why:** the module already stood and gained two tests, covering the founder's `multiple` and `optgroup` forks (inputs#I8, inputs#I9) where they submit. The chunk's added-lines census forbids `.unwrap()` and `.expect(` under `packages/`.
**Ref:** .andromeda/runs/2026-10-31T10-45-42-wrap/

## 2026-10-31-engine-debts — a non-invertible transform takes no hit
**Section:** §1 Test Scope Summary (test-scope-summary.md: new blitz-dom hit testing record)
**Change:**
- New record: `hit_non_invertible_transform` 2, both layout modes: a box under a transform whose determinant is 0 or not finite takes no hit, `Node::hit_inner` answering `None` for its whole subtree, an invertible twin hit where it shows.
**Why:** an engine change on the founder's word, by dialog in the overseer session, 2026-10-31, relayed verbatim by the operator (inputs#I8, inputs#I9); no master stated the rule before.
**Ref:** .andromeda/runs/2026-10-31T10-45-42-wrap/

## 2026-10-31-engine-debts — what `select_interaction` pins, the chosen-row pair, what no driver check reaches
**Section:** §1 Test Scope Summary (test-scope-summary.md: three new blitz-dom select records)
**Change:**
- `multiple` and groups: a `multiple` select's rows with boxes of their own; rows chosen, spanned and flipped over a 17-row key table and a 10-row click table; typed text choosing its row alone, a Space joining it while the text stands; a disabled select taking no click and no key; `multiple`'s removal settling the set to one option; a seeded set stepped and spanned from its last chosen row; a group's options chosen by a single select's keys and shown alone; a `multiple` select's groups laid out, a disabled group's rows not chosen.
- The chosen-row pair: the engine default `#c8c8c8` on `#000000` in `default.css`, Chromium's inactive selected pair, reads 12.56:1, by `a_chosen_row_and_a_plain_one_meet_their_ratio_at_the_user_agents_pair`.
- What no driver check reaches: a non-adjacent `multiple` set, `press` carrying no Ctrl, and Ctrl+A, not modelled; owner the `press` residual line in `.andromeda/residuals.md`.
**Why:** the founder's `multiple` and `optgroup` forks, by dialog in the overseer session, 2026-10-31, relayed verbatim by the operator (inputs#I8, inputs#I9). The `press` limit with a residual line is his word from the P5 card, relayed verbatim by the operator (inputs#I11, inputs#I12).
**Ref:** .andromeda/runs/2026-10-31T10-45-42-wrap/

## 2026-10-31-engine-debts — `node/select.rs` holds eight tests
**Section:** §1 Test Scope Summary (test-scope-summary.md: blitz-dom node), §4 Unit Test Strategy (unit-test-strategy.md: blitz-dom node; new record)
**Change:**
- Was 4 (four) inline table tests; now 8 in both files.
- The selectedness rule was keeping one chosen option; now one on a single select, a `multiple` select's other options left alone.
- Adds: a step moving from the row it is given, a span listing the enabled rows between two either way round, the option list running through a group in tree order.
**Why:** read at this wrap against the module, which now holds eight inline tests, the older count false.
**Ref:** .andromeda/runs/2026-10-31T10-45-42-wrap/

## 2026-10-31-engine-debts — the `:open` gap closes with `open_state`
**Section:** §1 Test Scope Summary (test-scope-summary.md: blitz-dom node `:open`, new record; test-plan.md: covered behaviours)
**Change:**
- Was `:open` has no test of its own, `details_element` and the a11y leg green beside it; now pinned by blitz-tests' `open_state`.
- New record: `open_state` 4, each in both layout modes: `:open` read from the attribute at construction, moved by a write and a removal through the mutator, flipped by a summary's click and its Enter.
**Why:** the chunk adds the target, closing a gap it carried.
**Ref:** .andromeda/runs/2026-10-31T10-45-42-wrap/

## 2026-10-31-engine-debts — Cells' grid as one Tab stop in the checks
**Section:** §1 Test Scope Summary (test-scope-summary.md: the keyed ids, `stand_keyboard`, `stand_contrast`, two new records), §2 Test Strategy (test-strategy.md: new Shared tab tables record)
**Change:**
- Cells' keyed ids add `cells-grid`, taken with `tabindex` 0, `aria_label` `Cells` and an `onkeydown`, with no role; the 676 data cells hold.
- `stand_keyboard` was 15 (11 until 2026-10-14-stand-a11y-assertions; run by the `a11y` leg since then); now 21 (run by the `a11y` leg). New record: Cells' Tab sequence, arrows clamped and bringing a cell into view, Enter opening and committing by keyboard, Tab leaving an open editor committing it, a click on another cell committing an open editor to its own cell, the load page's sequence and its button's keys.
- `stand_contrast` was 5 (2 until 2026-10-14-stand-a11y-assertions); now 7. New record: `the_load_page_meets_its_ratios`, `cells_meets_its_ratios`, the `.cell.current` focus outline using the stand accent `#3a5ce5`.
- New record: `tests/common/mod.rs` states `LOAD_TAB_ORDER` and `CELLS_TAB_ORDER`.
**Why:** the founder's fork for Cells' one Tab stop, by dialog in the overseer session, 2026-10-31, relayed verbatim by the operator (inputs#I8, inputs#I9). No `grid` role is built; role and cell semantics stay out.
**Ref:** .andromeda/runs/2026-10-31T10-45-42-wrap/

## 2026-10-31-engine-debts — a native control's `toggled` in `accessibility_states`
**Section:** §1 Test Scope Summary (test-scope-summary.md: `accessibility_states`, new record)
**Change:**
- `accessibility_states` was 5; now 6.
- A native checkbox or radio was reading neither `aria-checked` nor `aria-pressed`; now reading neither author attribute and writing `toggled` from its checked state.
- New record: `a_click_and_a_space_move_a_native_control_in_the_tree`, both layout modes; a native checkbox or radio writes `toggled` under a checkable role alone, so `pressed` never reads on it.
**Why:** an engine change on the founder's word, by dialog in the overseer session, 2026-10-31, relayed verbatim by the operator (inputs#I8, inputs#I9).
**Ref:** .andromeda/runs/2026-10-31T10-45-42-wrap/

## 2026-10-31-engine-debts — the focus-fixup limit ends in `inert_subtree`
**Section:** §1 Test Scope Summary (test-scope-summary.md: `inert_subtree`, new record)
**Change:**
- `inert_subtree` was 4; now 6.
- Was the focus-fixup limit (a node made inert keeps focus) pinned; now a node coming under `inert` gives focus up with no `blur` or `focusout`.
- New record: `a_node_made_inert_loses_focus_and_takes_no_key` over five ways of being made inert, and a check that the tree's focus reads the window when the focused node was not built.
**Why:** engine changes on the founder's word, by dialog in the overseer session, 2026-10-31, relayed verbatim by the operator (inputs#I8, inputs#I9). The mutator's flush clears such focus, and the tree writes `focus` only where the node was built, else the window. What an assistive technology reads stays unmeasured.
**Ref:** .andromeda/runs/2026-10-31T10-45-42-wrap/

## 2026-10-31-engine-debts — `visible_region` through transforms
**Section:** §1 Test Scope Summary (test-scope-summary.md: `transformed_client_rect`, `stand_act_transformed`, two new records)
**Change:**
- A target inside a translated scrolling box was reading outside its `visible_region`, the measured `off-screen` limit; now reading inside it, the limit's words gone.
- `stand_act_transformed` was 2; now 3. New record: `a_control_in_a_scrolling_box_under_a_zoomed_world_is_reached_where_it_shows`, both layout modes.
- New record: `visible_region_transformed` 6, its own target, both layout modes: each clipping box read through its transforms, exact under a translate and a scale, the rotated padding box's bounding box under a rotation, `None` on a non-finite edge; a panned world, a target in a node body under a transformed world, a page with no transform in the client rects' frame, a translate moving the region, a box mapped to a point, and a rotation.
**Why:** an engine change on the founder's word, by dialog in the overseer session, 2026-10-31, relayed verbatim by the operator (inputs#I8, inputs#I9). `BaseDocument::visible_region` keeps its signature.
**Ref:** .andromeda/runs/2026-10-31T10-45-42-wrap/

## 2026-10-31-engine-debts — Cells' reacting elements read 677
**Section:** §1 Test Scope Summary (test-scope-summary.md: `stand_non_lean_tasks`)
**Change:**
- The count was of the elements that react to a click, Cells 676; now of those that react to input, Cells 677: the 676 data cells and `cells-grid`, whose `onkeydown` and no role join them.
**Why:** measured against the plan's forecast of 676 unmoved: the count is every element with the listener mark reading `GenericContainer`, and the grid joins the data cells. The pin moved with its reason.
**Kept:** the 676 data cells stand as a true count.
**Ref:** .andromeda/runs/2026-10-31T10-45-42-wrap/

## 2026-10-31-engine-debts — the lint skips a closed select's options
**Section:** §1 Test Scope Summary (test-scope-summary.md: `stand_lint`, new record, The lint's select-option reading)
**Change:**
- `stand_lint` was 7; now 8. `at_boot` was `[0, 1, 0, 0]`; now `[0, 0, 0, 0]`, the flight booker's one `overlap`, `flight-type-one-way` × `flight-type-return`, gone.
- Was `overlaps` reading a select's options as overlapping, a driver defect owned by a `.andromeda/residuals.md` line; now `overlaps` skips, on either side, an entry inside a closed select, `Entry.in_closed_select` read through `closed_select_of`, the element and never the role. The snapshot still gives such an option its select's client rect, and a `multiple` select's rows lint as usual.
- New record: the planted overlap case is two overlapping rows of a `multiple` select, reading exactly `fx-row-a` × `fx-row-b`.
**Why:** the founder's fork, as restated, by dialog in the overseer session, 2026-10-31, relayed verbatim by the operator (inputs#I8, inputs#I9).
**Ref:** .andromeda/runs/2026-10-31T10-45-42-wrap/

## 2026-10-31-engine-debts — fixture readers read 28, `run stand` 222
**Section:** §2 Test Strategy (test-strategy.md: the fixtures module), §3 Test Harness Contract (test-plan.md: blitz-tests fixtures, Test-data bootstrap)
**Change:**
- `mod fixtures;` readers were 24; now 28: the 22 `stand_*.rs` beside `font_face_sniffed_source`, `select_interaction`, `inert_subtree`, `accessibility_states`, `checkable_change_event` and `text_range_no_change_event`.
- `run {file}` was the two non-stand readers; now the six.
- `run stand` was 205 · 0 · 7; now 222 · 0 · 7 over 50 files.
**Why:** the chunk's census reads the 28 readers and the split; the chunk's `run stand` reads 222 · 0 · 7, its `stand_*.rs` files unmoved at 50.
**Kept:** the Agent-run Proof record gains no row; its rows stand as history.
**Ref:** .andromeda/runs/2026-10-31T10-45-42-wrap/

## 2026-10-31-engine-debts — blitz-tests holds 153 `.rs` files
**Section:** §2 Test Strategy (test-strategy.md: the manifest's one `[[test]]` table)
**Change:**
- `.rs` files were 148; now 153. Test targets were 147; now 152. The latest are `visible_region_transformed.rs`, `hit_non_invertible_transform.rs`, `open_state.rs`, `checkable_change_event.rs` and `text_range_no_change_event.rs`, each its own auto-discovered target with no manifest line.
**Why:** the chunk adds five new checks.
**Ref:** .andromeda/runs/2026-10-31T10-45-42-wrap/
