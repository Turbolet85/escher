group: test-plan-1
read: .claude/docs/tests-summary.md · 56 · «**Full plan:** `.andromeda/test-plan.md`. Path-scoped rules:»
- leaf: .claude/docs/tests-summary.md:18   class: stale
  leaf-text: «`run stand` runs the 22 `stand_*` readers of the shared module, `run font_face_sniffed_source`, `run select_interaction`, `run inert_subtree`,»
  source: .andromeda/test-plan.md:191 «runs the 23 `stand_*` readers of the shared module, `run {file}` the eight non-stand readers (`run font_face_sniffed_source`, which holds a load, `run select_interaction`, `run select_popup`»
  replace-with: «`run stand` runs the 23 `stand_*` readers of the shared module, `run font_face_sniffed_source`, `run select_interaction`, `run select_popup`, `run select_popup_painted`, `run inert_subtree`,»
- leaf: .claude/docs/tests-summary.md:18   class: stale
  leaf-text: «`run checkable_change_event` and `run text_range_no_change_event` the six non-stand readers»
  source: .andromeda/test-plan.md:191 «`run {file}` the eight non-stand readers»
  replace-with: «`run checkable_change_event` and `run text_range_no_change_event` the eight non-stand readers»
- leaf: .claude/docs/tests-summary.md:18   class: stale
  leaf-text: «`run stand` 222 · 0 · 7 over 50 files at 2026-10-31-engine-debts;»
  source: .andromeda/test-plan.md:191 «`run stand` 237 · 0 · 7 over 51 files, as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md»
  replace-with: «`run stand` 237 · 0 · 7 over 51 files at 2026-10-31-select-popup, where `run all` reads 727 · 0 · 12 (+15 passed in `run stand`: `stand_act_popup` 12, a new file its `stand_` prefix picks up, `stand_keyboard` +2 and `stand_lint` +1; `select_popup` and `select_popup_painted` carrying no `stand_` prefix and picked up by `run all` with no script change); 222 · 0 · 7 over 50 files at 2026-10-31-engine-debts;»
- leaf: .claude/docs/tests-summary.md:21   class: stale
  leaf-text: «(`stand_act_*`, fifteen files, `stand_act_screenshot` the thirteenth since 2026-10-12-headless-screenshot, `stand_act_transformed` the fourteenth, `stand_act_select` the fifteenth since 2026-10-20-select-by-id —»
  source: .andromeda/test-plan-detail/test-scope-summary.md:112 «the sixteen `stand_act_*` integration files (`stand_act_screenshot` the thirteenth, `stand_act_transformed` the fourteenth, `stand_act_select` the fifteenth, `stand_act_popup` the sixteenth)»
  replace-with: «(`stand_act_*`, sixteen files, `stand_act_screenshot` the thirteenth since 2026-10-12-headless-screenshot, `stand_act_transformed` the fourteenth, `stand_act_select` the fifteenth since 2026-10-20-select-by-id, `stand_act_popup` the sixteenth since 2026-10-31-select-popup —»
- leaf: .claude/docs/tests-summary.md:21   class: stale
  leaf-text: «`STATE_ROWS` (18 reached states, 17 pass and 1 exempt, the lowest pass 5.11)»
  source: .andromeda/test-plan-detail/test-scope-summary.md:169 «19 pass and 1 exempt as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md, the lowest pass of the eighteen in those six classes 5.11»
  replace-with: «`STATE_ROWS` (20 reached states, 19 pass and 1 exempt, the lowest pass of the eighteen in its six stated classes 5.11)»
- leaf: .claude/docs/tests-summary.md:21   class: stale
  leaf-text: «`stand_keyboard` (21, run by the `a11y` leg;»
  source: .andromeda/test-plan-detail/test-scope-summary.md:169 «since 2026-10-13-stand-keyboard-harness `stand_keyboard` 23 (run by the `a11y` leg)»
  replace-with: «`stand_keyboard` (23, run by the `a11y` leg;»
- leaf: .claude/docs/tests-summary.md:21   class: omission
  leaf-text: «the flight type select moving its choice by its arrows;»
  source: .andromeda/test-plan-detail/test-scope-summary.md:170 «the flight type select moving its choice by its arrows, that test's Enter and Space rows opening the list and then choosing, so it reads true and not what its row order suggests»
  replace-with: «the flight type select moving its choice by its arrows, that test's Enter and Space rows opening the list and then choosing;»
- leaf: .claude/docs/tests-summary.md:21   class: omission
  leaf-text: «and every `controls(task)` id lying in the union of `tab_order` and `ENABLED_TAB_ORDER`)»
  source: .andromeda/test-plan-detail/test-scope-summary.md:172 «the flight type's walk holding with its list open and after each close (`the_flight_type_walk_holds_with_its_list_open_and_after_each_close`)»
  replace-with: «and every `controls(task)` id lying in the union of `tab_order` and `ENABLED_TAB_ORDER`, the flight type's walk holding with its list open and after each close, and a click opening the list, focus staying on the select and the walk going on from it)»
- leaf: .claude/docs/tests-summary.md:21   class: stale
  leaf-text: «`stand_lint` (8, declaring»
  source: .andromeda/test-plan-detail/test-scope-summary.md:176 «`stand_lint` 9, declaring `mod fixtures;` and `mod session_common;`»
  replace-with: «`stand_lint` (9, declaring»
- leaf: .claude/docs/tests-summary.md:21   class: omission
  leaf-text: «reading exactly `fx-row-a` × `fx-row-b` —,»
  source: .andromeda/test-plan-detail/test-scope-summary.md:216 «an open list's rows read no overlap on the stand, in both layout modes (`an_open_lists_rows_read_no_overlap_on_the_stand`»
  replace-with: «reading exactly `fx-row-a` × `fx-row-b`, an open list's rows reading no overlap on the stand —,»
- leaf: .claude/docs/tests-summary.md:21   class: omission
  leaf-text: «choosing an enabled option over a 14-row table,»
  source: .andromeda/test-plan-detail/test-scope-summary.md:179 «choosing an enabled option over a 14-row table, `KEYS`, holding `PageDown` and no Enter, which opens the list;»
  replace-with: «choosing an enabled option over a 14-row table, `KEYS`, holding `PageDown` and no Enter, which opens the list,»
- leaf: .claude/docs/tests-summary.md:21   class: omission
  leaf-text: «a click then a key choosing an option;»
  source: .andromeda/test-plan-detail/test-scope-summary.md:179 «a click then a key choosing an option, the click opening the select's list and the key moving or choosing in it;»
  replace-with: «a click then a key choosing an option, the click opening the select's list and the key moving or choosing in it;»
- leaf: .claude/docs/tests-summary.md:21   class: stale
  leaf-text: «(read by 28 checks — 22 `stand_*` files, `stand_snapshot_parts` since 2026-10-20-large-screen-in-parts,»
  source: .andromeda/test-plan.md:151 «31 checks declare `mod fixtures;` — 23 `stand_*` files (`stand_snapshot_parts`; `stand_act_select`; `stand_load_wait`, reading `HeldNet`; `stand_lint`; `stand_act_popup`)»
  replace-with: «(read by 31 checks — 23 `stand_*` files, `stand_act_popup` since 2026-10-31-select-popup, `stand_snapshot_parts` since 2026-10-20-large-screen-in-parts,»
- leaf: .claude/docs/tests-summary.md:21   class: stale
  leaf-text: «since 2026-10-19-visual-lint — `font_face_sniffed_source`, `select_interaction`, `inert_subtree`,»
  source: .andromeda/test-plan.md:151 «`font_face_sniffed_source`, `select_interaction`, `select_popup`, `select_popup_painted`, `inert_subtree`,»
  replace-with: «since 2026-10-19-visual-lint — `font_face_sniffed_source`, `select_interaction`, `select_popup`, `select_popup_painted`, `inert_subtree`,»
- leaf: .claude/docs/tests-summary.md:21   class: omission
  leaf-text: «each firing `input` and no `change`) — and process telemetry»
  source: .andromeda/test-plan-detail/test-scope-summary.md:234 «`select_popup`:** 25 tests, its own target with no `stand_` prefix, each in both layout modes, reading the select list's engine behaviour»
  replace-with: «each firing `input` and no `change`), since 2026-10-31-select-popup the single select's painted list — blitz-dom's `node/select_list.rs` 6 inline tests (where a list opens, its bounds, `size` read as a capped whole number, a point reading its row under scroll), `select_popup` (25, its own target with no `stand_` prefix, each test in both layout modes: a click, Enter and Space opening the list on its chosen option and firing nothing; the keys and the pointer moving the highlight over enabled rows and choosing nothing; Enter, Space, Tab and a click choosing a row, one `input` then one `change`, the chosen option firing nothing and a disabled row changing nothing; Escape, a second click, a click outside and the page scrolling closing with no event; a label's click focusing the select and opening nothing; a disabled, inert, hidden, singular or empty select opening nothing and one made inert, disabled, hidden or removed with its list open closing; the tree reading the list open on the select, its focus staying there; a box that clips the select clipping no row; groups read as headers, a header choosing nothing; a long list showing twenty rows and scrolling by key and wheel; hostile markup keeping every box finite; `size` on a `multiple` select making it that many rows tall; each open, move, scroll and close asking a redraw, counted on its own `CountingShell`; one row skipping its key press, a key pressed after an ancestor gained `hidden` panicking at `packages/blitz-dom/src/node/node.rs:1775` for any focused control, no check reading that panic) and `select_popup_painted` (6, its own target with no `stand_` prefix, each test in both layout modes: the highlight and a disabled row against `SelectListView`'s constants, the plain contrast pair, a list painted whole past an `opacity` ancestor and a box that clips its select, a closed frame byte-equal after a close with no choice, a header darker than a row by weight, a scrolled list's thumb painted and taking no hit; blitz-paint's `render/select_list.rs` holding no unit test of its own), each run by the workspace `test` leg, `run all`, by name and as a named `--test` of the `stand-repeat` leg, not by `run stand` or the `a11y` leg, and `stand_act_popup` (12, each test in both layout modes, driving the list through a held session: a click opening it and a click on a row choosing, the keys alone opening, moving and choosing, Escape closing with the value unchanged and the rows back at the select's box, a closed option and a button under the list refused `covered`, a row past the scrollport `off-screen` until `scroll` brings it in, `type` leaving no list open, a shot of a row cropping to it and leaving the list open, the lint, the open screen under its budget with every actionable element keyed, two clicks on one point opening then closing, and a read after a step with no `advance`; picked up by `run stand` and the `stand-repeat` leg by its prefix, not among the `a11y` leg's fourteen); the windowed list read and not run, what an assistive technology reads `unrunnable-here`, and the thumb not draggable, a highlighted row's contrast unread by the lint, a larger authored font clipping and the highlight in no snapshot and no tree its four stated limits — and process telemetry»
- leaf: .claude/docs/tests-summary.md:24   class: stale
  leaf-text: «seven_guis' `cli_commands` (14 — `snapshot --part`»
  source: .andromeda/test-plan-detail/e2e-test-strategy.md:8 «`cli_commands` (15): each of the six served tasks»
  replace-with: «seven_guis' `cli_commands` (15 — a hosted click opening a select's list, a shot of it equal to the in-process one and a click on a row choosing, in the state directory `cc-popup`; `snapshot --part`»
- leaf: .claude/docs/tests-summary.md:24   class: stale
  leaf-text: «`mcp_log` (2, 1 until 2026-10-19-session-recording added»
  source: .andromeda/test-plan-detail/e2e-test-strategy.md:13 «`mcp_log` (3) reads a select list opened and chosen through the role logging no option at `trace`, its test recording nothing»
  replace-with: «`mcp_log` (3, a select list opened and chosen through the role, which records nothing, reading 0 option needles at `trace`; 2 until 2026-10-31-select-popup, 1 until 2026-10-19-session-recording added»
- leaf: .claude/docs/tests-summary.md:24   class: stale
  leaf-text: «the fifteen `stand_act_*` files (61 tests, 51 before `stand_act_select`»
  source: .andromeda/registries/contracts/test-plan/session-lifecycle.md:11 «`stand_act_popup` 12, the sixteenth (73 in all)»
  replace-with: «the sixteen `stand_act_*` files (73 tests, 61 before `stand_act_popup` — 12, the sixteenth since 2026-10-31-select-popup, driving the select list through a held session —, 51 before `stand_act_select`»
- leaf: .claude/docs/tests-summary.md:24   class: stale
  leaf-text: «`host_log` (4, unix only, the fourth reading a paged Cells part through the host's own sink; 1 until»
  source: .andromeda/registries/contracts/test-plan/session-lifecycle.md:11 «unix only, a select list opened and chosen through the host, `a_select_list_opened_and_chosen_through_the_host_logs_no_option_at_trace` — 0 option needles at `trace`)»
  replace-with: «`host_log` (5, unix only, the fourth reading a paged Cells part through the host's own sink, the fifth a select list opened and chosen through the host, 0 option needles at `trace`; 4 until 2026-10-31-select-popup, 1 until»
- leaf: .claude/docs/tests-summary.md:24   class: stale
  leaf-text: «`host_log` holding 4 tests and `mcp_log` 2;»
  source: .andromeda/test-plan-detail/integration-test-strategy.md:35 «`host_log` holds 5 tests and `mcp_log` 3.»
  replace-with: «`host_log` holding 5 tests and `mcp_log` 3;»
- leaf: .claude/docs/tests-summary.md:24   class: omission
  leaf-text: «and the middle column-A cell it holds) — called by `host_log` and `mcp_log` alone.»
  source: .andromeda/test-plan-detail/test-scope-summary.md:14 «`option_needles()`, the needle list of a select's options read from a snapshot, called by the same two»
  replace-with: «and the middle column-A cell it holds) and `option_needles()` (the needle list of a select's options read from a snapshot) — called by `host_log` and `mcp_log` alone.»
- leaf: .claude/docs/tests-summary.md:26   class: omission
  leaf-text: «a password field's pixels equal for two values of one length (`password_painted_masked`).»
  source: .andromeda/test-plan-detail/test-strategy.md:40 «**Painted select list:** `select_popup_painted` reads an open list on a frame's `Raster` against the engine's constants, in both layout modes»
  replace-with: «a password field's pixels equal for two values of one length (`password_painted_masked`), and since 2026-10-31-select-popup an open select list read on a frame's `Raster` against the engine's constants by `select_popup_painted`, in both layout modes; no local gate builds `blitz-paint/font-embolden`, so a paint change under that OS-gated feature has its local witness in `cargo test -p blitz-tests --locked --no-fail-fast --features blitz-paint/font-embolden --test select_popup_painted`, which reads on linux the header and row numbers `Test (macos)` reads through `apple-font-embolden`.»
- leaf: .claude/docs/tests-summary.md:29   class: stale
  leaf-text: «three rounds of the 50 `stand_*.rs` files (48 before `stand_cells_served` and `stand_load_page`), fifteen named blitz-tests targets outside»
  source: .andromeda/test-plan-detail/ci-integration.md:8 «`tests/blitz-tests/tests/stand_*.rs` — 51 files»
  replace-with: «three rounds of the 51 `stand_*.rs` files (50 before `stand_act_popup`, 48 before `stand_cells_served` and `stand_load_page`), seventeen named blitz-tests targets outside»
- leaf: .claude/docs/tests-summary.md:29   class: stale
  leaf-text: «`checkable_change_event`, `text_range_no_change_event`) and seven_guis' eight process checks (the fifteen since 2026-10-31-engine-debts, `test_stand_repeat_leg_runs_three_rounds` naming the glob and the fifteen words; ten since»
  source: .andromeda/test-plan-detail/ci-integration.md:9 «`test_stand_repeat_leg_runs_three_rounds` naming the glob and the seventeen words»
  replace-with: «`checkable_change_event`, `text_range_no_change_event`, `select_popup`, `select_popup_painted`) and seven_guis' eight process checks (the seventeen since 2026-10-31-select-popup, `test_stand_repeat_leg_runs_three_rounds` naming the glob and the seventeen words; fifteen since 2026-10-31-engine-debts; ten since»
- leaf: .claude/docs/tests-summary.md:29   class: omission
  leaf-text: «and the android row running `ubuntu-latest`;»
  source: .andromeda/test-plan-detail/ci-integration.md:34 «CI#41233234207 at `8cd9c1286` ended red on `Test (macos)` alone, sixteen jobs `success`: `apple-font-embolden` made it red on macOS alone, and no local gate builds `blitz-paint/font-embolden`»
  replace-with: «and the android row running `ubuntu-latest`; CI#41233234207 at `8cd9c1286` ended red on `Test (macos)` alone, `apple-font-embolden` making it red on macOS alone and no local gate building `blitz-paint/font-embolden`, so a local gate block reads green over a defect only macOS shows, a paint change under that feature having its local witness in `cargo test -p blitz-tests --locked --no-fail-fast --features blitz-paint/font-embolden --test select_popup_painted`;»
- leaf: .claude/docs/tests-summary.md:29   class: omission
  leaf-text: «no CI run has shown it fire (green in CI run 37985678276, where eight jobs ran the script).»
  source: .andromeda/test-plan-detail/ci-integration.md:40 «`cargo` stops at the first failing target, so a red job's later targets go unread on that run»
  replace-with: «no CI run has shown it fire (green in CI run 37985678276, where eight jobs ran the script). `cargo` stops at the first failing target, so a red job's later targets go unread on that run.»
- leaf: .claude/docs/tests-summary.md:30   class: stale
  leaf-text: «`cargo test --workspace` 1181 · 0 · 12 over 214 result lines with the five new targets at 2026-10-31-engine-debts (from 209 · 1125:»
  source: .andromeda/test-plan-detail/ci-integration.md:110 «re-counted with the select list's three new targets: 217 result lines, 1236 passed · 0 failed · 12 ignored»
  replace-with: «`cargo test --workspace` 1236 · 0 · 12 over 217 result lines with the select list's three new targets at 2026-10-31-select-popup (from 214 · 1181: +3 result lines — `select_popup` 25, `select_popup_painted` 6, `stand_act_popup` 12 — and +55 passed — those 43, blitz-dom's lib `select_list` 6, `stand_keyboard` +2, `stand_lint` +1, `cli_commands`, `host_log` and `mcp_log` +1 each; `run stand` 237 passed · 0 failed · 7 ignored over 51 files, `run all` 727 passed · 0 failed · 12 ignored; timings not re-measured); 1181 · 0 · 12 over 214 result lines with the five new targets at 2026-10-31-engine-debts (from 209 · 1125:»
- leaf: .claude/docs/tests-summary.md:37   class: omission
  leaf-text: «panicking with `RefCell already borrowed` at `events.rs:162`);»
  source: .andromeda/test-plan-detail/e2e-test-strategy.md:25 «**The select list, headless:** the list paints through `paint_scene` alone, no shell or renderer code of its own, and the harness, the session and the binary drive it, measured headlessly»
  replace-with: «panicking with `RefCell already borrowed` at `events.rs:162`; the select list painting through `paint_scene` alone, no shell or renderer code of its own, the harness, the session and the binary driving it, measured headlessly, its windowed path read and not run);»
- leaf: .claude/docs/tests-summary.md:42   class: stale
  leaf-text: «read now on CI#41192433140 at 2026-10-31-engine-debts under the same floors escher-driver 98.04 %, dioxus-native-dom 86.12 %, blitz-test-harness 94.84 %,»
  source: .andromeda/test-plan-detail/quality-gates-coverage-targets.md:16 «escher-driver 4959/5058 = 98.04 %, dioxus-native-dom 2791/3240 = 86.14 %, blitz-test-harness 968/1008 = 96.03 %»
  replace-with: «read now on CI#41235503227 at 2026-10-31-select-popup under the same floors escher-driver 98.04 %, dioxus-native-dom 86.14 %, blitz-test-harness 96.03 %,»
- leaf: .claude/docs/tests-summary.md:43   class: stale
  leaf-text: «the 50 `stand_*.rs` files, the fifteen named blitz-tests targets of its `--test` list»
  source: .andromeda/test-plan-detail/quality-gates-coverage-targets.md:22 «run the 51 `stand_*.rs` files, the seventeen named blitz-tests targets of its `--test` list»
  replace-with: «the 51 `stand_*.rs` files, the seventeen named blitz-tests targets of its `--test` list»
- leaf: .claude/docs/tests-summary.md:43   class: stale
  leaf-text: «(with the fifteen CI#41192433140 reading 219 · 1008 · 0 · 21, +15 · +144, `stand-repeat: 3 of 3 rounds green`;»
  source: .andromeda/test-plan-detail/quality-gates-coverage-targets.md:22 «with the seventeen and `stand_act_popup` CI#41235503227 read 228 · 1155 · 0 · 21»
  replace-with: «(with the seventeen and `stand_act_popup` CI#41235503227 reading 228 · 1155 · 0 · 21;»
