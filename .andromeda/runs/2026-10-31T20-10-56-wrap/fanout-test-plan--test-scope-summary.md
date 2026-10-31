proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 14
    section: "## 1. Test Scope Summary"
    change: "In the shared module's list, after «`cells_needles(part)` (part `part`'s author keys of the in-process Cells screen and the middle column-A cell it holds)» add «, and since 2026-10-31-select-popup `option_needles` (a select's options read from a snapshot)», the sentence's «called by `host_log` and `mcp_log` alone» standing."
    sidecar: "§1 apps: `tests/common/mod.rs` gains `option_needles`, read by the two log checks' open-list tests."
    rationale: "Report, New text by line, examples/seven_guis/tests/common/mod.rs 425-453 adds `pub fn option_needles`; Outcome: `host_log`, `mcp_log` 0 needles at `trace`."
  - detector: D-tests-coverage
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 15
    section: "## 1. Test Scope Summary"
    change: "Replace «`tests/cli_commands.rs` (13 tests, unix only:» with «`tests/cli_commands.rs` (15 tests, unix only:»."
    sidecar: "§1 apps: `cli_commands`' count read 13; 15 since 2026-10-31-select-popup."
    rationale: "Report, Counts, `cli-count`: `cli_commands` 14 → 15. Its sweep passed this line, whose count reads 13 and matched no 14 pattern, naming only line 16's shared token."
  - detector: D-tests-coverage
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 15
    section: "## 1. Test Scope Summary"
    change: "Replace «`tests/host_log.rs` (4 tests, unix only, the fourth below;» with «`tests/host_log.rs` (5 tests, unix only, the last two below;»."
    sidecar: "§1 apps: `host_log` 4 → 5, the open list's test through the host."
    rationale: "Report, Coverage of new surfaces: process (`cli_commands`, `host_log`, `mcp_log` +1 each); New text: host_log.rs 282-393."
  - detector: D-tests-coverage
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 17
    section: "## 1. Test Scope Summary"
    change: "Replace «`tests/mcp_log.rs` (2 tests — 1 until» with «`tests/mcp_log.rs` (3 tests, the third below — 1 until»."
    sidecar: "§1 apps: `mcp_log` 2 → 3, the open list's test through the role."
    rationale: "Report, Coverage of new surfaces: process +1 each; New text: mcp_log.rs 68-113."
  - detector: D-retired
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 24
    section: "## 1. Test Scope Summary"
    change: "Replace «no popup is painted and» with «a single select's open list is painted past any clipping box, read by `select_popup_painted`, and»."
    sidecar: "§1 blitz-dom covered behaviours: the limit «no popup is painted» retired."
    rationale: "Report, Retired: «no popup is painted»; Counts, `no-popup`: test-scope-summary:24 changed."
    basis: .andromeda/test-plan-detail/test-scope-summary.md:24
  - detector: D-retired
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 24
    section: "## 1. Test Scope Summary"
    change: "Replace «Enter and Space open nothing on a single select (`enter_and_space_open_nothing_on_a_single_select`,» with «Enter, and a Space with no typed text standing, open that list as a click does (`enter_and_space_open_the_list_and_a_released_space_does_nothing_more`,»."
    sidecar: "§1 blitz-dom covered behaviours: Enter and Space open a single select's list; the test's new name."
    rationale: "Report, Retired: «Enter and Space open nothing on a single select»; Symbols, Keys: `press_select_key`; Test renames names this line's old name; Counts, `enter-space`: test-scope-summary:24 changed."
    basis: .andromeda/test-plan-detail/test-scope-summary.md:24
  - detector: D-retired
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 24
    section: "## 1. Test Scope Summary"
    change: "Replace «`size` is unread,» with «`size` on a `multiple` select reads that many rows tall (`select_popup`), on a single select stays unread,»."
    sidecar: "§1 blitz-dom covered behaviours: a `multiple` select's `size` is read; a single select's stays unread."
    rationale: "Report, Retired: «a `multiple` select's `size` is unread»; Symbols, `size`: a single select's `size` stays unread; Counts, `size-unread`: test-scope-summary:24 changed."
    basis: .andromeda/test-plan-detail/test-scope-summary.md:24
  - detector: D-retired
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 24
    section: "## 1. Test Scope Summary"
    change: "Replace «and a group's label is named in the tree and unpainted;» with «and a group's label is named in the tree and painted as its header in the open list;»."
    sidecar: "§1 blitz-dom covered behaviours: a group's label is painted as a header."
    rationale: "Report, Retired: «a group's label is unpainted»; Counts, `group-label`: test-scope-summary:24 under `size-unread`."
    basis: .andromeda/test-plan-detail/test-scope-summary.md:24
  - detector: D-tests-coverage
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 24
    section: "## 1. Test Scope Summary"
    change: "Replace «while a click on a select's label re-dispatched to the select is asserted by no check;» with «while a click on a single select's label focuses it and opens nothing, no longer re-dispatched (`select_popup`'s `a_labels_click_focuses_the_select_and_opens_nothing`), a `multiple` select's still asserted by no check;»."
    sidecar: "§1 blitz-dom covered behaviours: the single-select half of a label's click is asserted."
    rationale: "Report, Spec claims disproved, `label-click`: the single-select half is now asserted, and no longer re-dispatches; Counts, `label-click`: test-scope-summary:24 changed."
    basis: .andromeda/test-plan-detail/test-scope-summary.md:24
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 49
    section: "## 1. Test Scope Summary"
    change: "Add after this line, in its own detail block: «- **blitz-dom (select list) — what no check reaches:** the windowed list is read and not run, no shell or renderer moved; what an assistive technology reads stays unmeasured, `unrunnable-here`, the tree's `expanded` asserted in the harness alone; a highlighted row's contrast is unread by the lint, and the highlight stands in no snapshot and no tree; no local gate builds `blitz-paint/font-embolden`, a header's weight on that build read by naming the feature, `cargo test -p blitz-tests --locked --no-fail-fast --features blitz-paint/font-embolden --test select_popup_painted`, and in CI by the macOS job — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md; owner: this wrap's P5, `.andromeda/residuals.md`»."
    sidecar: "§1: the open list's untested readings stated, with their owner."
    rationale: "Report, Outcome: windowed list read and not run; (a11y) what an assistive technology reads stays unmeasured; Decisions: no local gate builds `blitz-paint/font-embolden`; For P5: the residual lines."
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 58
    section: "## 1. Test Scope Summary"
    change: "Add after this line: «- **blitz-dom (node) — `node/select_list.rs`:** 6 inline tests of the list's state — a list opens where its rows fit and else on the side with more room; it is at least its select's width, at most the viewport's, and stays inside it; every edge of a placed list is finite; `size` reads a whole number above zero, capped; a scroll stays within the rows and brings a row in by the least movement; a point reads the row it lies on where the rows are scrolled (packages/blitz-dom/src/node/select_list.rs:386-539); `BaseDocument`'s public `open_select`, `select_list_bounds`, `select_list_view`, `select_row_layout` and `close_select_list` are read through the checks `select_popup`, `select_popup_painted` and `stand_act_popup` (below) — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md»."
    sidecar: "§1 blitz-dom (node): `node/select_list.rs` and its 6 unit tests."
    rationale: "Report, Crates / modules: new module blitz-dom `node::select_list`; Coverage: tests unit (`select_list` 6); New text: select_list.rs 386-539."
  - detector: D-tests-coverage
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 94
    section: "## 1. Test Scope Summary"
    change: "Replace «the fifteen `stand_act_*` integration files (`stand_act_screenshot` the thirteenth, `stand_act_transformed` the fourteenth, `stand_act_select` the fifteenth)» with «the sixteen `stand_act_*` integration files (`stand_act_screenshot` the thirteenth, `stand_act_transformed` the fourteenth, `stand_act_select` the fifteenth, `stand_act_popup` the sixteenth)»."
    sidecar: "§1 escher-driver: `stand_act_*` files fifteen → sixteen."
    rationale: "Report, Counts, `act-files`: `stand_act_*` files fifteen → sixteen, test-scope-summary:94 changed."
    basis: .andromeda/test-plan-detail/test-scope-summary.md:94
  - detector: D-tests-coverage
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 151
    section: "## 1. Test Scope Summary"
    change: "Replace «`stand_keyboard` 21 (run by the `a11y` leg)» with «`stand_keyboard` 23 (run by the `a11y` leg)»."
    sidecar: "§1 tests/blitz-tests: `stand_keyboard` 21 → 23."
    rationale: "Report, Counts, `kbd-count`: `stand_keyboard` 21 → 23, test-scope-summary:151 changed."
    basis: .andromeda/test-plan-detail/test-scope-summary.md:151
  - detector: D-tests-coverage
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 151
    section: "## 1. Test Scope Summary"
    change: "Replace «`STATE_ROWS`, 18 reached states —» with «`STATE_ROWS`, 20 reached states —», «hover 5, active 5 — 17 pass and 1 exempt» with «hover 5, active 5, and two more — 19 pass and 1 exempt»."
    sidecar: "§1 tests/blitz-tests: `STATE_ROWS` 18 → 20, 19 pass and 1 exempt."
    rationale: "Report, Counts, `state-rows`: `STATE_ROWS` 18 → 20, tally (19, 0, 1, 0), test-scope-summary:151 changed; New text: stand_contrast.rs 304-316. The report names neither row's class, so the six stated classes keep their counts."
    basis: .andromeda/test-plan-detail/test-scope-summary.md:151
  - detector: D-tests-coverage
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 158
    section: "## 1. Test Scope Summary"
    change: "Replace «`stand_lint` 8, declaring `mod fixtures;` and `mod session_common;`» with «`stand_lint` 9, declaring `mod fixtures;` and `mod session_common;`»."
    sidecar: "§1 tests/blitz-tests: `stand_lint` 8 → 9."
    rationale: "Report, Coverage of new surfaces: integ `stand_lint` +1; New text: stand_lint.rs 493-519. No Counts pattern carries this count."
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 192
    section: "## 1. Test Scope Summary"
    change: "At this entry's end, before its «— as measured at» clause, add «; since 2026-10-31-select-popup an open list's rows read no overlap on the stand (`an_open_lists_rows_read_no_overlap_on_the_stand`, tests/blitz-tests/tests/stand_lint.rs:493-519)», and after the clause add «, the open list's at escher-0.3.0/chunks/2026-10-31-select-popup/report.md»."
    sidecar: "§1 `stand_lint` on selects: an open list's rows read no overlap."
    rationale: "Report, Outcome (lint): `an_open_lists_rows_read_no_overlap_on_the_stand`; Coverage: `stand_lint` +1."
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 210
    section: "## 1. Test Scope Summary"
    change: "Add in its own detail block after this entry's: «- **tests/blitz-tests — `stand_keyboard` with a list open:** 21 until 2026-10-31-select-popup, each test in both layout modes: the flight type walk holds with its list open and after each close, over the ten steps of `list_walk`; a click opens the list, focus stays on the select and the walk goes on from it (tests/blitz-tests/tests/stand_keyboard.rs:1055-1134); the closed-arrow test passes untouched, its Enter and Space rows now opening and then choosing, so it reads true and no longer reads what its row order suggests — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md»."
    sidecar: "§1 tests/blitz-tests: `stand_keyboard`'s two list tests and what its closed-arrow test now reads."
    rationale: "Report, Coverage: `stand_keyboard` +2; Outcome (layouts): `the_flight_type_walk_holds_with_its_list_open_and_after_each_close`; Decisions: the closed-arrow test's row order."
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 216
    section: "## 1. Test Scope Summary"
    change: "Add in its own detail block after this entry's: «- **tests/blitz-tests — `select_popup`:** 25, its own target with no `stand_` prefix, each test in both layout modes, run by the workspace `test` leg, the `stand-repeat` leg (a named `--test` of its list), by name and by `run all`, not by `run stand` or the `a11y` leg: a click, Enter and Space open the list on its chosen option and fire nothing; the keys move the highlight over enabled rows and choose nothing; Enter, Space, Tab and a click choose the row, one `input` then one `change`, a choice of the chosen option firing nothing; Tab chooses and closes before focus moves, either way; a disabled row changes nothing and leaves the list open; Escape, a second click, a click outside and the page scrolling close with no event; a label's click focuses the select and opens nothing; a disabled, inert, hidden, singular or empty select opens nothing; a select made inert, disabled, hidden or removed with its list open closes, the ancestor-hidden row skipping its key press, which panics for any control at packages/blitz-dom/src/node/node.rs:1775, with no list opened; an option dropped with the list open is no row; the tree reads the list open on the select, its focus staying there; an author's `aria-expanded` and `open` steer nothing on a select; a box that clips the select clips no row; a pointer at a row's centre lands on it, the page scrolled or not; a press on a row selects no text and a drag across rows chooses where it ends; groups read as headers, a header choosing nothing; a long list shows twenty rows and scrolls by key and wheel; a hostile `size`, 2,000 options and a 5,500-character label keep every box finite; `size` makes a `multiple` select that many rows tall; opening, moving, scrolling and closing each ask a redraw; a focus rule that moves the select's box leaves the list a click opened; a click in the select's box closes and chooses nothing (tests/blitz-tests/tests/select_popup.rs:202-1186) — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md»."
    sidecar: "§1 tests/blitz-tests: `select_popup` 25, the list's behaviours, with its one skipped key press."
    rationale: "Report, Coverage: integ `select_popup` 25; Harness surface: `ci-leg.sh` `stand-repeat` gains the word, the `a11y` leg stays fourteen; Spec claims disproved, `panic-pos`; New text: select_popup.rs."
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 216
    section: "## 1. Test Scope Summary"
    change: "Add in its own detail block after this entry's: «- **tests/blitz-tests — `select_popup_painted`:** 6, its own target with no `stand_` prefix, each test in both layout modes on frames, run by the workspace `test` leg, the `stand-repeat` leg (a named `--test` of its list), by name and by `run all`, not by `run stand` or the `a11y` leg: the highlight and a disabled row read as painted against `SelectListView`'s constants; the plain pair measured at the default sheet and at the stand; a list paints whole past an `opacity` ancestor and a box that clips its select; a closed select's frame byte-equal before opening and after a close with no choice; a group's header darker than a row by weight, green on both builds, plain and `blitz-paint/font-embolden`; a scrolled list paints its thumb, which takes no hit (tests/blitz-tests/tests/select_popup_painted.rs:119-351); `render/select_list.rs` holds no unit test, read on these frames — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md»."
    sidecar: "§1 tests/blitz-tests: `select_popup_painted` 6, the list on frames."
    rationale: "Report, Coverage: integ `select_popup_painted` 6; Crates / modules: new blitz-paint `render::select_list`; Outcome: read on both builds, 6 · 0 each; `ua-pair`: the plain pair measured there."
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 216
    section: "## 1. Test Scope Summary"
    change: "Add in its own detail block after this entry's: «- **tests/blitz-tests — `stand_act_popup`:** 12, the sixteenth `stand_act_*` file, declaring `mod session_common;`, each test in both layout modes through the driver, picked up by `run stand` and the stand-repeat leg by its prefix, not run by the `a11y` leg: a click opens the list and a click on a row chooses it; the keys alone open, move and choose; Escape closes with the value unchanged and the rows back at the select's box; a closed option stays refused `covered`, changing nothing; a row past the scrollport is off-screen until `scroll` brings it in; a button under the list is refused `covered`; `type` leaves no list open and chooses by its text; a shot of a row crops to it and leaves the list open; the lint reads no overlap on a row and the planted pair still reads; the open screen stays under its budget, every actionable element keyed; two clicks on one point open and close, each focusing; a screen read after a step reads the new state with no `advance` (tests/blitz-tests/tests/stand_act_popup.rs:58-423) — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md»."
    sidecar: "§1 tests/blitz-tests: `stand_act_popup` 12, the list through the driver."
    rationale: "Report, Coverage: integ `stand_act_popup` 12; Harness surface: joins `run stand` and `stand-repeat` by its prefix, the `a11y` leg stays fourteen; `mod-decl`: `session_common` 31 → 32; New text: stand_act_popup.rs."
  - detector: D-tests-coverage
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 228
    section: "## 1. Test Scope Summary"
    change: "Replace «and five more of the targets the leg names» with «and seven more of the targets the leg names, `select_popup` and `select_popup_painted` the two since 2026-10-31-select-popup»."
    sidecar: "§1 the 0.2.0 sweep's reach: the leg test asserts two more of the named targets."
    rationale: "Report, Harness / gate surface: `stand-repeat`'s list gains two words; `test_ci_workflows.py:367` asserts them. No Counts pattern reads this wording."
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 235
    section: "## 1. Test Scope Summary"
    change: "Add after this line: «- **The open list through the process:** seven_guis' `cli_commands` `a_hosted_click_opens_a_list_a_shot_shows_it_and_a_click_on_a_row_chooses`, in `cc-popup`, the hosted shot equal to the in-process one (14 tests until 2026-10-31-select-popup; examples/seven_guis/tests/cli_commands.rs:419-464); `host_log` `a_select_list_opened_and_chosen_through_the_host_logs_no_option_at_trace`, in `hl-popup`, `#[cfg(unix)]` as every host-driving test there (examples/seven_guis/tests/host_log.rs:282-393); `mcp_log` `a_select_list_opened_and_chosen_through_the_role_logs_no_option_at_trace`, which records nothing and holds no state directory (examples/seven_guis/tests/mcp_log.rs:68-113) — the two log checks reading 0 needles at `trace` — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md»."
    sidecar: "§1 apps: the three process tests of the open list."
    rationale: "Report, Coverage: process (`cli_commands`, `host_log`, `mcp_log` +1 each); Symbols, Test-only state directories: `cc-popup`, `hl-popup`, no `ml-popup`; Reverted: `mcp_log`'s test records nothing; Outcome (security): 0 needles at `trace`."
  - detector: D-retired
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 237
    section: "## 1. Test Scope Summary"
    change: "Replace «skips, on either side, an entry inside a closed select, `Entry.in_closed_select` read through `closed_select_of`, the element and never the role, the snapshot still giving each such option its select's client rect, and it lints a `multiple` select's rows as usual;» with «skips, on either side, an entry in a select's box, `Entry.in_select_box` — an element of a closed select, read through `closed_select_of`, and since 2026-10-31-select-popup also an open list's row or group — the element and never the role, the snapshot still giving a closed select's option its select's client rect and an open row its own `bounds`, `clip_region` taking an open row's region from `visible_region` alone with no `contain: paint` cut, and it lints a `multiple` select's rows as usual;»."
    sidecar: "§1 the lint's select-option reading: `Entry.in_closed_select` → `in_select_box`, true also for an open list's row or group."
    rationale: "Report, Retired: `Entry.in_closed_select`; Symbols, Driver: `lint.rs`; Bridge: the snapshot's builder reads `select_list_bounds`; Counts, `closed-option` and `lint-exclude`: test-scope-summary:237 changed."
    basis: .andromeda/test-plan-detail/test-scope-summary.md:237
note: "D-tests-framework and D-tests-obs-harness hold: the runner, agent-run.sh and the log format are unmoved. For the orchestrator: §1's body lines 19 and 25 name neither `select_popup` nor `node/select_list.rs` and are outside this file; :151's «the lowest pass 5.11» is left as it stands, the report stating no lowest pass for its two new `STATE_ROWS`. This file took one Write and then two small Edits, past the one call asked: the `select_popup` entry read 2 050 chars and was cut to the 2 000 limit, and this clause was added."
