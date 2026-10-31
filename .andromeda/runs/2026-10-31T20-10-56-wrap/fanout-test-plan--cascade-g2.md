proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 151
    section: "## 1. Test Scope Summary"
    change: "In `every_colour_state_meets_its_ratio`, `STATE_ROWS` reads 20 reached states since 2026-10-31-select-popup, 18 until then (the six classes stated, return-chosen 2, start-unparseable 2, booked 1, row-selected 3, hover 5 and active 5, being those eighteen), 19 pass and 1 exempt (17 and 1 until then); the lowest pass 5.11 stands as the reading of the eighteen; `stand_contrast` stays 7."
    sidecar: "test-scope-summary: `STATE_ROWS` 18 to 20, tally 19 pass and 1 exempt, at 2026-10-31-select-popup."
    rationale: "The report's `state-rows` row moves `STATE_ROWS` 18 to 20 with tally (19, 0, 1, 0) and lists test-scope-summary:151 as changed; it names neither the two new rows' class nor a new lowest, so neither is stated; the `run stand` re-count (+12, +2, +1) adds no `stand_contrast` test."
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 151
    section: "## 1. Test Scope Summary"
    change: "`stand_keyboard` reads 23 (21 until 2026-10-31-select-popup), still run by the `a11y` leg and picked up by `run stand`."
    sidecar: "test-scope-summary: `stand_keyboard` 21 to 23 at 2026-10-31-select-popup."
    rationale: "The report's `kbd-count` row moves `stand_keyboard` 21 to 23 and lists test-scope-summary:151 as changed."
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 154
    section: "## 1. Test Scope Summary"
    change: "After the Tab walk beyond boot, before `stand_requirement_sweep`, add: and, since 2026-10-31-select-popup, in both layout modes, the flight type's walk holding with its list open and after each close (`the_flight_type_walk_holds_with_its_list_open_and_after_each_close`) and a click opening the list, focus staying on the select and the walk going on from it (`a_click_opens_the_list_focus_stays_on_the_select_and_the_walk_goes_on_from_it`) (tests/blitz-tests/tests/stand_keyboard.rs:1082-1134)."
    sidecar: "test-scope-summary: `stand_keyboard`'s two list tests stated."
    rationale: "The report's New text lists both tests in `stand_keyboard.rs`, and its Outcome and Coverage sections read them under SC 2.1.1 and 2.4.3 as `stand_keyboard` +2."
    basis: tests/blitz-tests/tests/stand_keyboard.rs:1082-1134
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 152
    section: "## 1. Test Scope Summary"
    change: "After the flight type select moving its choice by its arrows, add: since 2026-10-31-select-popup that closed-arrow test passes untouched while its Enter and Space rows open and then choose, so it reads true and no longer reads what its row order suggests."
    sidecar: "test-scope-summary: `stand_keyboard`'s closed-arrow test, what it still reads."
    rationale: "The report's Decisions and corrections state this of `stand_keyboard`'s closed-arrow test; the row that names that test is the place for it."
    fact: F5
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 158
    section: "## 1. Test Scope Summary"
    change: "`stand_lint` reads 9 (8 until 2026-10-31-select-popup, which added `an_open_lists_rows_read_no_overlap_on_the_stand`)."
    sidecar: "test-scope-summary: `stand_lint` 8 to 9 at 2026-10-31-select-popup."
    rationale: "The report's Coverage reads `stand_lint` +1, its New text lists the test, and the `run stand` re-count to 237 holds that +1."
    basis: tests/blitz-tests/tests/stand_lint.rs:493-519
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 192
    section: "## 1. Test Scope Summary"
    change: "Add to the `stand_lint` row on selects: and, since 2026-10-31-select-popup, an open list's rows read no overlap on the stand, in both layout modes (`an_open_lists_rows_read_no_overlap_on_the_stand`, tests/blitz-tests/tests/stand_lint.rs:493-519), `stand_act_popup`'s `the_lint_reads_no_overlap_on_a_row_and_the_planted_pair_still_reads` reading the same through a held session — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md."
    sidecar: "test-scope-summary: the `stand_lint` row on selects gains an open list's rows."
    rationale: "The report turns `Entry.in_closed_select` into `in_select_box`, true also for an open list's row or group, and its Outcome reads a row no `overlap` by both tests; the row's two standing claims hold and it lacked this reading."
    basis: tests/blitz-tests/tests/stand_lint.rs:493-519
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 161
    section: "## 1. Test Scope Summary"
    change: "On `select_interaction`'s 14-row table add: one row moved since 2026-10-31-select-popup, Enter to `PageDown`, Enter now opening the list, the table still 14 rows and the file still 18 tests."
    sidecar: "test-scope-summary: `select_interaction`'s `KEYS` table, Enter to `PageDown`."
    rationale: "The report's Deviations state `KEYS` moved one row, Enter to `PageDown`, its count still 14, and its `select-model` control holds `select_interaction` at 18."
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 161
    section: "## 1. Test Scope Summary"
    change: "In `stand_act_select`, restate a click then a key choosing an option: a click then a key choosing an option, the click since 2026-10-31-select-popup opening the select's list and a key then moving or choosing in it; the file stays 10 tests."
    sidecar: "test-scope-summary: `stand_act_select`'s click-then-key reading, a click now opening the list."
    rationale: "The report's Symbols make a click on a single select open its list and send a key on an open list through `press_open_list_key`; `stand_act_select.rs` is edited at 187-203, its count unmoved by the `select-model` control. The report gives no line-level reading of that edit, so the wording rests on these engine facts."
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 186
    section: "## 1. Test Scope Summary"
    change: "- **tests/blitz-tests — `stand_act_popup`:** 12 tests, the sixteenth `stand_act_*` file, each in both layout modes, driving the select list through a held driver session: click and keys, Escape, a closed option and a button under the list refused `covered`, a row off screen until `scroll`, `type` leaving no list open, a row's shot, the lint, the snapshot budget, two clicks opening then closing, and a read with no `advance`; picked up by `run stand` and the `stand-repeat` leg by its prefix, not among the `a11y` leg's fourteen (tests/blitz-tests/tests/stand_act_popup.rs:58-423) — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md"
    sidecar: "test-scope-summary: new row for `stand_act_popup`."
    rationale: "The report's Coverage and Harness sections state the new target, its 12 tests and its prefix pickup; a driver check of `stand_act_*` belongs beside the `stand_act_transformed` row."
    basis: tests/blitz-tests/tests/stand_act_popup.rs:58-423
    fact: F4
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 204
    section: "## 1. Test Scope Summary"
    change: "- **tests/blitz-tests — `select_popup`:** 25 tests, its own target with no `stand_` prefix, each in both layout modes, reading the select list's engine behaviour: opening, key and pointer moves, choosing, closing, inert, disabled, hidden and removed selects, the tree's `expanded`, clips, groups, the twenty-row cap, hostile markup, `size` on a `multiple` select and redraw requests, and a label's click focusing a single select and opening nothing; run by the workspace `test` leg, `run all`, by name and as a named `--test` of the `stand-repeat` leg, not by `run stand` or the `a11y` leg (tests/blitz-tests/tests/select_popup.rs:202-1186) — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md"
    sidecar: "test-scope-summary: new row for `select_popup`."
    rationale: "The report's Coverage and Outcome state the 25 tests, `ci-leg.sh:59-60` names the target, and the `a11y` leg stays fourteen files; its inert and made-inert selects sit beside the `inert_subtree` row."
    basis: tests/blitz-tests/tests/select_popup.rs:202-1186
    fact: F2
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 204
    section: "## 1. Test Scope Summary"
    change: "- **tests/blitz-tests — `select_popup_painted`:** 6 tests, its own target with no `stand_` prefix, each in both layout modes, reading the select list on frames: highlight and disabled row against their constants, the plain contrast pair, painting past clips and `opacity`, a closed frame byte-equal after a close with no choice, a header darker than a row by weight, and a thumb taking no hit; a named `--test` of the `stand-repeat` leg; blitz-paint's `render/select_list.rs` holds no unit test of its own (tests/blitz-tests/tests/select_popup_painted.rs:119-351) — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md"
    sidecar: "test-scope-summary: new row for `select_popup_painted`."
    rationale: "The report's Coverage and Outcome state the 6 tests, and `ci-leg.sh:59-60` names the target; the row belongs beside `select_popup`, whose engine behaviour it reads on frames."
    basis: tests/blitz-tests/tests/select_popup_painted.rs:119-351
    fact: F3
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 204
    section: "## 1. Test Scope Summary"
    change: "- **tests/blitz-tests — `select_popup`'s skipped key press:** `select_popup` skips the key press in its one row where an ancestor gains `hidden`, because a key press there panics at packages/blitz-dom/src/node/node.rs:1775 in `position_in_containing_block` for any focused control, list or no list; no check reads that panic, and its owner is left to this wrap's P5 — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md"
    sidecar: "test-scope-summary: `select_popup`'s skipped key press and the panic no check reads."
    rationale: "The report's `panic-pos` entry and Outcome state the skip, the panic at `node.rs:1775` and its routing to P5."
    fact: F6
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 204
    section: "## 1. Test Scope Summary"
    change: "- **tests/blitz-tests — the select list's unmeasured limits:** the windowed list read and not run, an assistive technology's reading `unrunnable-here`, a highlighted row's contrast unread by the lint, the highlight absent from snapshot and tree, the thumb not draggable, and a larger authored font clipping — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md"
    sidecar: "test-scope-summary: the select list's unmeasured limits stated."
    rationale: "The report's Coverage, Outcome and the card's row at inputs#I7 state these limits, and no check reads them; they belong beside the list's check rows."
    fact: F7
note: test-scope-summary:24, :94 and :237 carry moves the report names and test-plan:17, :19 and :25 name the select's readers, but no entity of this pick holds those lines, so none is proposed here; no scoped detector fired, as the new paths carry tests at their tier, the runner is unchanged and the harness is unedited.
