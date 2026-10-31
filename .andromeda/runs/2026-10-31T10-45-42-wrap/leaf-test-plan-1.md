group: test-plan-1
read: .claude/docs/tests-summary.md · 56 · «**Full plan:** `.andromeda/test-plan.md`. Path-scoped rules:»
- leaf: .claude/docs/tests-summary.md:18   class: stale
  leaf-text: «`run font_face_sniffed_source` and `run select_interaction` the two non-stand readers»
  source: .andromeda/test-plan.md:190 «`run {file}` the six non-stand readers (`run font_face_sniffed_source`, which holds a load, `run select_interaction`, `run inert_subtree`, `run accessibility_states`, `run checkable_change_event`»
  replace-with: «`run font_face_sniffed_source`, `run select_interaction`, `run inert_subtree`, `run accessibility_states`, `run checkable_change_event` and `run text_range_no_change_event` the six non-stand readers»
- leaf: .claude/docs/tests-summary.md:18   class: stale
  leaf-text: «`run stand` 205 · 0 · 7 over 50 files at 2026-10-25-cold-agent-bar, unmoved at»
  source: .andromeda/test-plan.md:190 «`run stand` 222 · 0 · 7 over 50 files, as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md»
  replace-with: «`run stand` 222 · 0 · 7 over 50 files at 2026-10-31-engine-debts; 205 · 0 · 7 over 50 files at 2026-10-25-cold-agent-bar, unmoved at»
- leaf: .claude/docs/tests-summary.md:21   class: stale
  leaf-text: «in `accessibility_states` (5), the builder's three state attributes»
  source: .andromeda/test-plan-detail/test-scope-summary.md:132 «`accessibility_states` 6: `aria-selected` reads `true` and `false` and nothing else (9 rows)»
  replace-with: «in `accessibility_states` (6), the builder's three state attributes»
- leaf: .claude/docs/tests-summary.md:21   class: omission
  leaf-text: «a native checkbox or radio reading neither `aria-checked` nor `aria-pressed`; no check reads them»
  source: .andromeda/test-plan-detail/test-scope-summary.md:132 «reading neither author attribute and writing `toggled` from its checked state»
  replace-with: «a native checkbox or radio reading neither author attribute and writing `toggled` from its checked state under a checkable role alone, so `pressed` never reads on it, a click and a Space moving it in the tree (`a_click_and_a_space_move_a_native_control_in_the_tree`); no check reads them»
- leaf: .claude/docs/tests-summary.md:21   class: stale
  leaf-text: «`inert_subtree` (4: Tab and Shift+Tab skipping an inert subtree, a hit passing through it, the tree holding no node for it, removing `inert` restoring all three)»
  source: .andromeda/test-plan-detail/test-scope-summary.md:135 «`inert_subtree` 6 — Tab and Shift+Tab skip an inert subtree, a hit passes through it, the tree holds no node for it, and removing `inert` restores focus, hit and tree, a node coming under `inert` giv»
  replace-with: «`inert_subtree` (6: Tab and Shift+Tab skipping an inert subtree, a hit passing through it, the tree holding no node for it, removing `inert` restoring all three, a node coming under `inert` giving focus up with no `blur` or `focusout` over five ways of being made inert, and the tree's focus reading the window when the focused node was not built)»
- leaf: .claude/docs/tests-summary.md:21   class: stale
  leaf-text: «`stand_contrast` (5, 2 until 2026-10-14-stand-a11y-assertions: every stand control»
  source: .andromeda/test-plan-detail/test-scope-summary.md:150 «`stand_contrast` 7 — every stand control reads its recorded text/background pair»
  replace-with: «`stand_contrast` (7, `the_load_page_meets_its_ratios` and `cells_meets_its_ratios` among them, the `.cell.current` focus outline using the stand accent `#3a5ce5`: every stand control»
- leaf: .claude/docs/tests-summary.md:21   class: stale
  leaf-text: «`stand_keyboard` (15, 11 until 2026-10-14-stand-a11y-assertions, run by the `a11y` leg since then; a stand check `run stand` picks up, each test in both layout modes — Tab and Shift+Tab»
  source: .andromeda/test-plan-detail/test-scope-summary.md:151 «`stand_keyboard` 21 (run by the `a11y` leg), a stand check `run stand` picks up, each test in both layout modes»
  replace-with: «`stand_keyboard` (21, run by the `a11y` leg; a stand check `run stand` picks up, each test in both layout modes — Cells' Tab sequence, its arrows clamped at the grid's ends and bringing a cell into view, Enter opening the cell's editor and committing by keyboard, Tab leaving an open editor committing it, a click on another cell committing an open editor to its own cell, and the load page's sequence with its button taking Enter and Space; Tab and Shift+Tab»
- leaf: .claude/docs/tests-summary.md:21   class: stale
  leaf-text: «the elements that react to a click and state no role counted against a 7-row table — 0 on the lean tasks and Home, Circle Drawer 1, Cells 676, measured and pinned»
  source: .andromeda/test-plan-detail/test-scope-summary.md:155 «the elements that react to input and state no role counted against a stated 7-row table (0 on the four lean tasks and Home, Circle Drawer 1, Cells 677, the 676 data cells and `cells-grid`»
  replace-with: «the elements that react to input and state no role counted against a 7-row table — 0 on the lean tasks and Home, Circle Drawer 1, Cells 677, the 676 data cells and `cells-grid`, whose `onkeydown` and no role join them, measured and pinned»
- leaf: .claude/docs/tests-summary.md:21   class: stale
  leaf-text: «`stand_lint` (7, declaring `mod fixtures;` and `mod session_common;`»
  source: .andromeda/test-plan-detail/test-scope-summary.md:158 «since 2026-10-19-visual-lint, `stand_lint` 8, declaring `mod fixtures;` and `mod session_common;`»
  replace-with: «`stand_lint` (8, declaring `mod fixtures;` and `mod session_common;`»
- leaf: .claude/docs/tests-summary.md:21   class: retired
  leaf-text: «the lean tasks at boot reading `[0, 1, 0, 0]` — the flight booker's one `overlap`, `flight-type-one-way` × `flight-type-return`, the lint's reading of a select's options, a driver defect owned by a `.andromeda/residuals.md` line, the lint reading»
  source: .andromeda/test-plan-detail/test-scope-summary.md:159 «the lean tasks at boot read their measured findings, `[0, 0, 0, 0]`, the lint skipping a closed select's options»
  replace-with: «the lean tasks at boot reading `[0, 0, 0, 0]`, the lint skipping a closed select's options and linting a `multiple` select's rows as usual — its planted overlap two overlapping rows of a `multiple` select, reading exactly `fx-row-a` × `fx-row-b` —, the lint reading»
- leaf: .claude/docs/tests-summary.md:21   class: stale
  leaf-text: «`select_interaction` (7, declaring `mod fixtures;`»
  source: .andromeda/test-plan-detail/test-scope-summary.md:161 «`select_interaction` 18, declaring `mod fixtures;`, its own auto-discovered target with no `stand_` prefix»
  replace-with: «`select_interaction` (18, declaring `mod fixtures;`»
- leaf: .claude/docs/tests-summary.md:21   class: omission
  leaf-text: «each row asserting `has_changes` and one `input` and one `change` per changed choice, none for no change)»
  source: .andromeda/test-plan-detail/test-scope-summary.md:37 «pins a `multiple` select laying out its rows with boxes of their own; its rows chosen, spanned and flipped over a 17-row key table and a 10-row click table»
  replace-with: «each row asserting `has_changes` and one `input` and one `change` per changed choice, none for no change; a `multiple` select laying out its rows with boxes of their own, its rows chosen, spanned and flipped over a 17-row key table and a 10-row click table, typed text choosing its row alone and a Space joining it while the text stands, a disabled select taking no click and no key, `multiple`'s removal settling the set to one option, a set the markup seeds stepped and spanned from its last chosen row, a group's options chosen by a single select's keys and shown alone, a `multiple` select's groups laid out and a disabled group's rows not chosen; and the engine default chosen-row pair `#c8c8c8` on `#000000` reading 12.56:1)»
- leaf: .claude/docs/tests-summary.md:21   class: stale
  leaf-text: «`stand_act_select` (5, declaring `mod fixtures;` and `mod session_common;`, through `Session::run` in both layout modes: at boot»
  source: .andromeda/test-plan-detail/test-scope-summary.md:161 «and `stand_act_select` 10, declaring `mod fixtures;` and `mod session_common;`, through `Session::run` in both layout modes on a page writing `selected` statically»
  replace-with: «`stand_act_select` (10, declaring `mod fixtures;` and `mod session_common;`, through `Session::run` in both layout modes: a `multiple` select's name, role and rows read each with its own bounds, an accepted row click choosing the row, a Shift press spanning rows, `type` choosing its match alone with no stray click, an option under a closed select's group refused `covered`; at boot»
- leaf: .claude/docs/tests-summary.md:21   class: omission
  leaf-text: «a `click` on a translated button accepted and landing, its snapshot `bounds` read where it shows; a shot a PNG»
  source: .andromeda/test-plan-detail/test-scope-summary.md:186 «the driver reaching a control in a scrolling box under a zoomed world where it shows»
  replace-with: «a `click` on a translated button accepted and landing, its snapshot `bounds` read where it shows, a control in a scrolling box under a zoomed world reached where it shows; a shot a PNG»
- leaf: .claude/docs/tests-summary.md:21   class: stale
  leaf-text: «`stand_cells_served` (4, on the stand's own Cells, `stand::boot_cells`, 47,949 bytes in 740 lines, 5 parts of `SNAPSHOT_TEXT_BUDGET`, through `Session::run`: Cells walked in parts through the driver,»
  source: .andromeda/test-plan-detail/test-data-fixtures.md:24 «47,949 bytes in 740 lines as read before `cells-grid` took its `id` and `aria_label`, no later reading claimed, and 5 parts of `SNAPSHOT_TEXT_BUDGET`»
  replace-with: «`stand_cells_served` (5, on the stand's own Cells, `stand::boot_cells`, 47,949 bytes in 740 lines as read before `cells-grid` took its `id` and `aria_label`, no later reading claimed, 5 parts of `SNAPSHOT_TEXT_BUDGET`, through `Session::run`: Cells walked in parts through the driver and by keys to a cell whose committed value reads in the first part — `press` walking and opening the editor, `type` supplying the characters and `press` committing, the driver's twelve key names holding no character —,»
- leaf: .claude/docs/tests-summary.md:21   class: stale
  leaf-text: «`stand_load_page` (2, on the load page, `stand::boot_load`: a click on `load-fetch` waiting out the load `LoadNet` answers after 200 ms and `load-done` showing "Loaded", the page keyed with no lint finding at boot)»
  source: .andromeda/test-plan-detail/test-scope-summary.md:234 «and `stand_load_page` 3, on the load page (`stand::boot_load`)»
  replace-with: «`stand_load_page` (3, on the load page, `stand::boot_load`: a click on `load-fetch` waiting out the load `LoadNet` answers after 200 ms and `load-done` showing "Loaded", the page keyed with no lint finding at boot, and its rows asserted beside its load wait by the file's third test, their Tab sequence and button keys by `stand_keyboard`, their ratios by `stand_contrast`)»
- leaf: .claude/docs/tests-summary.md:21   class: stale
  leaf-text: «(read by 24 checks — 22 `stand_*` files, `stand_snapshot_parts` since 2026-10-20-large-screen-in-parts, `stand_act_select` since 2026-10-20-select-by-id, `stand_load_wait` since 2026-10-20-load-wait, `stand_lint` since 2026-10-19-visual-lint — `font_face_sniffed_source` and `select_interaction`)»
  source: .andromeda/test-plan.md:151 «28 checks declare `mod fixtures;` — 22 `stand_*` files»
  replace-with: «(read by 28 checks — 22 `stand_*` files, `stand_snapshot_parts` since 2026-10-20-large-screen-in-parts, `stand_act_select` since 2026-10-20-select-by-id, `stand_load_wait` since 2026-10-20-load-wait, `stand_lint` since 2026-10-19-visual-lint — `font_face_sniffed_source`, `select_interaction`, `inert_subtree`, `accessibility_states`, `checkable_change_event` and `text_range_no_change_event`)»
- leaf: .claude/docs/tests-summary.md:21   class: omission
  leaf-text: «none of the three a test target) and process telemetry (stdout silent»
  source: .andromeda/test-plan-detail/test-scope-summary.md:168 «`visible_region_transformed` 6, its own target, each test in both layout modes»
  replace-with: «none of the three a test target), since 2026-10-31-engine-debts five targets of their own with no `stand_` prefix, each test in both layout modes, named `--test`s of the `stand-repeat` list and not run by `run stand` — `visible_region_transformed` (6: `BaseDocument::visible_region` reading each clipping box through its transforms, exact under a translate and a scale, the rotated padding box's bounding box under a rotation, `None` on a non-finite edge), `hit_non_invertible_transform` (2: a box under a transform whose determinant is 0 or not finite taking no hit, its invertible twin hit where it shows), `open_state` (4: `:open` read from the attribute at construction, moved by a write and a removal through the mutator, flipped by a summary's click and its Enter), `checkable_change_event` (3: a click and a Space on a checkbox each firing `input` then `change`, a radio's `change` firing on an actual change alone, a label's click reading as its control's) and `text_range_no_change_event` (2: typing in a text input and a range's key and pointerdown each firing `input` and no `change`) — and process telemetry (stdout silent»
- leaf: .claude/docs/tests-summary.md:22   class: omission
  leaf-text: «five on the closed span's line, the identity macro).»
  source: .andromeda/test-plan-detail/unit-test-strategy.md:14 «`packages/blitz-shell/src/event.rs` holds the crate's one unit test in an inline `#[cfg(test)]` module, `a_redraw_request_reaches_the_receiver_and_wakes_the_loop_once`»
  replace-with: «five on the closed span's line, the identity macro); `blitz-shell`: one inline unit test in `src/event.rs`, `a_redraw_request_reaches_the_receiver_and_wakes_the_loop_once`, on a test-local `CountedWakes` event-loop proxy, which states what it cannot read.»
- leaf: .claude/docs/tests-summary.md:24   class: stale
  leaf-text: «the fifteen `stand_act_*` files (55 tests, 50 before `stand_act_select` — 5, the fifteenth»
  source: .andromeda/registries/contracts/test-plan/session-lifecycle.md:11 «`stand_act_select` 10, the fifteenth (61 in all)»
  replace-with: «the fifteen `stand_act_*` files (61 tests, 51 before `stand_act_select` — 10, the fifteenth»
- leaf: .claude/docs/tests-summary.md:24   class: stale
  leaf-text: «`stand_act_transformed` 2 the fourteenth)»
  source: .andromeda/registries/contracts/test-plan/session-lifecycle.md:11 «`stand_act_transformed` 3 (51; 48 before `stand_act_transformed`»
  replace-with: «`stand_act_transformed` 3 the fourteenth)»
- leaf: .claude/docs/tests-summary.md:24   class: stale
  leaf-text: «and a target inside a translated scrolling box reading outside its `visible_region`, the measured `off-screen` limit)»
  source: .andromeda/test-plan-detail/test-scope-summary.md:148 «a target inside a translated scrolling box reading inside its `visible_region`, in both layout modes»
  replace-with: «and a target inside a translated scrolling box reading inside its `visible_region`)»
- leaf: .claude/docs/tests-summary.md:27   class: stale
  leaf-text: «`blitz-vibey-script`: 30 DOM API tests (`tests/dom.rs`)»
  source: .andromeda/test-plan-detail/test-scope-summary.md:101 «tests/dom.rs holds 31 `#[test]` functions covering the JS DOM APIs»
  replace-with: «`blitz-vibey-script`: 31 DOM API tests (`tests/dom.rs`, a radio's `change` among them: a first click hearing `input,change` and a second adding `input` alone)»
- leaf: .claude/docs/tests-summary.md:29   class: stale
  leaf-text: «ten named blitz-tests targets outside the `stand_` prefix as literal `--test` words (`inert_subtree`, `accessibility_states`, `scrolled_box_absolute_position`, `hit_clipped_at_scrolling_box`, `password_caret_click`, `transformed_client_rect`, `accessibility_active_descendant_unread`, `virtual_clock_engine`, `select_interaction`, `text_extent`)»
  source: .andromeda/test-plan-detail/ci-integration.md:9 «fifteen named targets as literal `--test` words, `inert_subtree`, `accessibility_states`, `scrolled_box_absolute_position`»
  replace-with: «fifteen named blitz-tests targets outside the `stand_` prefix as literal `--test` words (`inert_subtree`, `accessibility_states`, `scrolled_box_absolute_position`, `hit_clipped_at_scrolling_box`, `password_caret_click`, `transformed_client_rect`, `accessibility_active_descendant_unread`, `virtual_clock_engine`, `select_interaction`, `text_extent`, `visible_region_transformed`, `hit_non_invertible_transform`, `open_state`, `checkable_change_event`, `text_range_no_change_event`)»
- leaf: .claude/docs/tests-summary.md:29   class: stale
  leaf-text: «(the ten since 2026-10-24-stand-requirement-sweep, `test_stand_repeat_leg_runs_three_rounds` naming the glob and the ten words;»
  source: .andromeda/test-plan-detail/ci-integration.md:9 «`test_stand_repeat_leg_runs_three_rounds` naming the glob and the fifteen words»
  replace-with: «(the fifteen since 2026-10-31-engine-debts, `test_stand_repeat_leg_runs_three_rounds` naming the glob and the fifteen words; ten since 2026-10-24-stand-requirement-sweep;»
- leaf: .claude/docs/tests-summary.md:29   class: omission
  leaf-text: «the first red ending the leg — no retry) and the windows/macos/ios/android matrix;»
  source: .andromeda/test-plan-detail/ci-integration.md:34 «the job runs `jlumbroso/free-disk-space@54081f138730dfa15788a46383842cd2f914a1be` after the toolchain and before `Swatinem/rust-cache`»
  replace-with: «the first red ending the leg — no retry; the job runs `jlumbroso/free-disk-space` after the toolchain and before `Swatinem/rust-cache`, CI#41190771843 having ended red there on linker bus errors with no test run and the step not shown to be the cure) and the windows/macos/ios/android matrix, whose own free-disk step is skipped on every row, its `if:` naming `ubuntu-24.04` and `ubuntu-24.04-arm` and the android row running `ubuntu-latest`;»
- leaf: .claude/docs/tests-summary.md:30   class: stale
  leaf-text: «`cargo test --workspace` 1125 · 0 · 12 over 209 result lines with upstream's eleven commits»
  source: .andromeda/test-plan-detail/ci-integration.md:97 «re-counted with the five new targets: 214 result lines, 1181 passed · 0 failed · 12 ignored»
  replace-with: «`cargo test --workspace` 1181 · 0 · 12 over 214 result lines with the five new targets at 2026-10-31-engine-debts (from 209 · 1125: +5 result lines — `visible_region_transformed` 6, `hit_non_invertible_transform` 2, `open_state` 4, `checkable_change_event` 3, `text_range_no_change_event` 2 — and +56 passed — those 17, 31 more in the edited checks, blitz-dom's lib 6, blitz-shell's lib 1, blitz-vibey-script's `dom` 1; `run stand` 222 passed · 0 failed · 7 ignored over 50 files; timings not re-measured); 1125 · 0 · 12 over 209 result lines with upstream's eleven commits»
- leaf: .claude/docs/tests-summary.md:37   class: omission
  leaf-text: «**Windowed E2E** — NOT YET MEASURED (windowed or browser-driven end-to-end runs lay outside every slice read);»
  source: .andromeda/test-plan-detail/e2e-test-strategy.md:18 «Cells calls its `MountedData` handles synchronously, measured headlessly, their windowed path read from the same lines and not run»
  replace-with: «**Windowed E2E** — NOT YET MEASURED (windowed or browser-driven end-to-end runs lay outside every slice read; Cells' `MountedData` handle calls measured headlessly, their windowed path read from the same lines and not run, a `MountedData` call in a task spawned from a key handler panicking with `RefCell already borrowed` at `events.rs:162`);»
- leaf: .claude/docs/tests-summary.md:42   class: stale
  leaf-text: «and read now at 2026-10-26-codebase-health under the same floors escher-driver 97.99 %, dioxus-native-dom 85.60 %, blitz-test-harness 94.44 %,»
  source: .andromeda/test-plan-detail/quality-gates-coverage-targets.md:16 «read now under the same floors and `coverage_floor.py`: escher-driver 4950/5049 = 98.04 %, dioxus-native-dom 2786/3235 = 86.12 %, blitz-test-harness 956/1008 = 94.84 %»
  replace-with: «and read now on CI#41192433140 at 2026-10-31-engine-debts under the same floors escher-driver 98.04 %, dioxus-native-dom 86.12 %, blitz-test-harness 94.84 %,»
- leaf: .claude/docs/tests-summary.md:43   class: stale
  leaf-text: «the 50 `stand_*.rs` files, the ten named blitz-tests targets of its `--test` list and seven_guis' eight process checks run three times with no retry (with the served Cells»
  source: .andromeda/test-plan-detail/quality-gates-coverage-targets.md:22 «the fifteen named blitz-tests targets of its `--test` list and seven_guis' eight process checks three times with no retry (§9 Legs; with the fifteen CI#41192433140 read 219 · 1008 · 0 · 21»
  replace-with: «the 50 `stand_*.rs` files, the fifteen named blitz-tests targets of its `--test` list and seven_guis' eight process checks run three times with no retry (with the fifteen CI#41192433140 reading 219 · 1008 · 0 · 21, +15 · +144, `stand-repeat: 3 of 3 rounds green`; with the served Cells»
