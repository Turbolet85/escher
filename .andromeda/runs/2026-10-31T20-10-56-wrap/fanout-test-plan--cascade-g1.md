proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 14
    section: "## 1. Test Scope Summary"
    change: "In the listing of `tests/common/mod.rs`, after the clause on `cells_needles(part)` and the words `called by host_log and mcp_log alone`, add: since 2026-10-31-select-popup `option_needles()`, the needle list of a select's options read from a snapshot."
    sidecar: "test-scope-summary: seven_guis' shared module's listing gains `option_needles()`."
    rationale: "The report's New text lists `pub fn option_needles() -> Vec<String>` added to `examples/seven_guis/tests/common/mod.rs`; this entity lists that module's readers by the chunk that added them, so the listing is behind. The fact's own record, F12, is test-data-fixtures' to place."
    basis: examples/seven_guis/tests/common/mod.rs:425-453
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 15
    section: "## 1. Test Scope Summary"
    change: "`tests/host_log.rs` (4 tests, unix only, the fourth below; 1 until ... becomes `tests/host_log.rs` (5 tests, unix only, the fourth below, and since 2026-10-31-select-popup `a_select_list_opened_and_chosen_through_the_host_logs_no_option_at_trace`; 4 until then, 1 until ..., the rest of the clause as it stands."
    sidecar: "test-scope-summary: `host_log` 4 to 5 tests."
    rationale: "The report's coverage states process `cli_commands`, `host_log`, `mcp_log` +1 each, and the task's touch says any text counting `host_log` moves by +1. The report states the increment, not a total, so 5 rests on this entity's standing 4."
    basis: examples/seven_guis/tests/host_log.rs:282-393
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 15
    section: "## 1. Test Scope Summary"
    change: "`tests/cli_commands.rs` (13 tests, unix only: becomes `tests/cli_commands.rs` (15 tests, unix only:, the rest of the clause as it stands."
    sidecar: "test-scope-summary: `cli_commands` 13 to 15 tests, its reading already behind at the base."
    rationale: "The report's `cli-count` reads `cli_commands` 14 to 15. This line states 13, one behind at the base, so the sweep's pattern for 14 missed it and its no-change row for test-scope-summary:16 does not reach it."
    basis: examples/seven_guis/tests/cli_commands.rs:419-464
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 17
    section: "## 1. Test Scope Summary"
    change: "`tests/mcp_log.rs` (2 tests — 1 until ... becomes `tests/mcp_log.rs` (3 tests — 2 until 2026-10-31-select-popup added `a_select_list_opened_and_chosen_through_the_role_logs_no_option_at_trace`, which records nothing; 1 until ..., the rest of the clause as it stands."
    sidecar: "test-scope-summary: `mcp_log` 2 to 3 tests."
    rationale: "The report's coverage states `mcp_log` +1, and its Reverted section that the popup test records nothing. The report states the increment, not a total, so 3 rests on this entity's standing 2."
    basis: examples/seven_guis/tests/mcp_log.rs:68-113
  - detector: D-retired
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 24
    section: "## 1. Test Scope Summary"
    change: "«no popup is painted and Enter and Space open nothing on a single select (`enter_and_space_open_nothing_on_a_single_select`» becomes «a closed single select opens a list the engine paints over the page, on a click, Enter or Space, read by blitz-tests' `select_popup` (`enter_and_space_open_the_list_and_a_released_space_does_nothing_more`, `enter_and_space_open_nothing_on_a_single_select` until 2026-10-31-select-popup»; the rest of the parenthesis as it stands."
    sidecar: "test-scope-summary: the limits «no popup is painted» and «Enter and Space open nothing» retired; the `select_interaction` test's rename carried."
    rationale: "The report's Retired bullet names both limits, its `no-popup` and `enter-space` rows change test-scope-summary:24, and its Test renames entry names this line's old name."
    basis: tests/blitz-tests/tests/select_popup.rs:202-230
  - detector: D-retired
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 24
    section: "## 1. Test Scope Summary"
    change: "«`size` is unread, and a group's label is named in the tree and unpainted» becomes «a `multiple` select's `size` is read, a height of `size × 1.2em` through `size_rows`, capped at 65534, `0` or a bad value hinting nothing, by `select_popup`'s `size_makes_a_multiple_select_that_many_rows_tall_and_a_key_brings_the_next_in` and `node::select_list`'s `size_reads_a_whole_number_above_zero_capped`, while a single select's `size` stays unread; a group's label is named in the tree and painted as a header over its options, by `select_popup`'s `groups_read_as_headers_their_options_under_them_and_a_header_chooses_nothing`»."
    sidecar: "test-scope-summary: the limits «a `multiple` select's `size` is unread» and «a group's label is unpainted» retired; a single select's `size` stays unread."
    rationale: "The report's Retired bullet names both limits; its `size-unread` row changes test-scope-summary:24 and states a single select's `size` still unread, and its `group-label` row places this line's label clause under `size-unread`."
    basis: tests/blitz-tests/tests/select_popup.rs:995-1032
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 24
    section: "## 1. Test Scope Summary"
    change: "«while a click on a select's label re-dispatched to the select is asserted by no check» becomes «a click on a single select's label focuses the select and opens nothing, with no re-dispatch, asserted by `select_popup`'s `a_labels_click_focuses_the_select_and_opens_nothing`, while a click on a `multiple` select's label re-dispatched to it is asserted by no check»."
    sidecar: "test-scope-summary: a label's click on a single select now asserted, focusing and opening nothing."
    rationale: "The report's Spec claims disproved, `label-click`, disproves this clause at test-scope-summary:24: the single-select half is now asserted and no longer re-dispatches. The report moves only that half, so the `multiple` half stands."
    basis: tests/blitz-tests/tests/select_popup.rs:428-441
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan.md
    near: 19
    section: "## 1. Test Scope Summary"
    change: "«the `select` model, single and `multiple`, with `optgroup`,» becomes «the `select` model, single and `multiple`, with `optgroup` and a single select's painted list,», and «the select's by `select_interaction` and `stand_act_select`» becomes «the select's by `select_interaction` and `stand_act_select`, and its list's by `select_popup`, `select_popup_painted` and `stand_act_popup`»."
    sidecar: "test-plan §1: the broad line of the blitz-dom covered behaviours names the select's list and its three checks."
    rationale: "This broad line summarises the group the select limits at test-scope-summary:24 detail. The report's coverage reads the list by `select_popup`, `select_popup_painted` and `stand_act_popup`, which the line does not name."
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan.md
    near: 25
    section: "## 1. Test Scope Summary"
    change: "«and the select's rules in `node/select.rs`» becomes «the select's rules in `node/select.rs` and, in `node/select_list.rs`, where a list opens, its bounds, `size` read as a capped whole number and a point reading its row under scroll»."
    sidecar: "test-plan §1: the blitz-dom (node) broad line names `node/select_list.rs`'s in-file tests."
    rationale: "The report's coverage states unit tests `select_list` 6 and its New text lists them in `packages/blitz-dom/src/node/select_list.rs`; this broad line enumerates the node module's in-file tests. The record itself is F1, unit-test-strategy's to place."
    basis: packages/blitz-dom/src/node/select_list.rs:386-539
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 37
    section: "## 1. Test Scope Summary"
    change: "- **blitz-dom (select) — the list:** blitz-tests' `select_popup` 25, both layout modes, an engine check with no `stand_` prefix, so not selected by `run stand` though run by the `stand-repeat` leg (a named `--test` of its list), reads the list's engine behaviour: opening, key and pointer moves, choosing, closing, inert, disabled, hidden and removed selects, the tree's `expanded`, clips, groups, the twenty-row cap, hostile markup, `size` on a `multiple` select and redraw requests (tests/blitz-tests/tests/select_popup.rs:202-1186) — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md"
    sidecar: "test-scope-summary: new record of `select_popup`, the list's engine behaviour."
    rationale: "The report's coverage states `select_popup` 25, Acceptance `v030-17` reads it green in both layout modes, and `ci-leg.sh`'s `stand-repeat` list names it; it stands beside this group's other `blitz-dom (select)` records."
    basis: tests/blitz-tests/tests/select_popup.rs:202-230
    fact: F2
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 49
    section: "## 1. Test Scope Summary"
    change: "- **blitz-dom (select) — the key press `select_popup` skips:** `select_popup` skips the key press in its one row where an ancestor gains `hidden`, because a key press there panics at `position_in_containing_block` for any focused control, list or no list; no check reads that panic (tests/blitz-tests/tests/select_popup.rs:478-520; packages/blitz-dom/src/node/node.rs:1775) — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md; owner: left to this wrap's P5"
    sidecar: "test-scope-summary: new record of the skipped key press and the panic no check reads, its owner left to P5."
    rationale: "The report's Spec claims disproved, `panic-pos`, and its Outcome state the skip, the panic for any control and that its owner falls to P5; it stands beside this entity's record of what no check reaches."
    basis: packages/blitz-dom/src/node/node.rs:1775
    fact: F6
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 49
    section: "## 1. Test Scope Summary"
    change: "- **blitz-dom (select) — the list's unmeasured limits:** the windowed list is read and not run; what an assistive technology reads is `unrunnable-here`; a highlighted row's contrast is unread by the lint; the highlight is in no snapshot and no tree; the thumb is not draggable; a larger authored font clips — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md; owner of the last four: their `.andromeda/residuals.md` lines, written at this wrap's P5"
    sidecar: "test-scope-summary: new record of the list's unmeasured limits, four owned by residual lines."
    rationale: "The report's Outcome and Coverage state the windowed list read and not run and an assistive technology's reading `unrunnable-here`; inputs#I7 and its P5 bullet route the thumb, the highlighted row's contrast, the larger font and the highlight's absence to `.andromeda/residuals.md`."
    fact: F7
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 76
    section: "## 1. Test Scope Summary"
    change: "- **blitz-paint — the select list:** blitz-tests' `select_popup_painted` 6, both layout modes, reads the list on frames: the highlight and a disabled row against their constants, the plain contrast pair, painting past clips and `opacity`, a closed frame byte-equal after a close with no choice, a header darker than a row by weight, and a thumb taking no hit; `render/select_list.rs` holds no unit test of its own (tests/blitz-tests/tests/select_popup_painted.rs:119-351; packages/blitz-paint/src/render/select_list.rs:20-155) — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md"
    sidecar: "test-scope-summary: new record of `select_popup_painted`, the list on frames, `render/select_list.rs` holding no unit test."
    rationale: "The report's coverage states `select_popup_painted` 6 and Acceptance `v030-17` reads it in both layout modes; its New text lists no test in `render/select_list.rs`. It stands beside this entity's record of blitz-paint's frame reads."
    basis: tests/blitz-tests/tests/select_popup_painted.rs:119-159
    fact: F3
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 94
    section: "## 1. Test Scope Summary"
    change: "«the fifteen `stand_act_*` integration files (`stand_act_screenshot` the thirteenth, `stand_act_transformed` the fourteenth, `stand_act_select` the fifteenth)» becomes «the sixteen `stand_act_*` integration files (`stand_act_screenshot` the thirteenth, `stand_act_transformed` the fourteenth, `stand_act_select` the fifteenth, `stand_act_popup` the sixteenth)»."
    sidecar: "test-scope-summary: `stand_act_*` files fifteen to sixteen."
    rationale: "The report's `act-files` row reads `stand_act_*` files fifteen to sixteen and names test-scope-summary:94 among the changed sites."
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 95
    section: "## 1. Test Scope Summary"
    change: "- **escher-driver — the select list through the driver:** blitz-tests' `stand_act_popup` 12, both layout modes, drives the list through a held driver session: a click and the keys, Escape, a closed option and a button under the list refused `covered`, a row off screen until `scroll`, `type` leaving no list open, a row's shot, the lint, the snapshot budget, two clicks opening then closing, and a read with no `advance` (tests/blitz-tests/tests/stand_act_popup.rs:58-423) — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md"
    sidecar: "test-scope-summary: new record of `stand_act_popup`, the list through a held driver session."
    rationale: "The report's coverage states `stand_act_popup` 12 and Acceptance `v030-17` reads it in both layout modes; this entity names the `stand_act_*` files as the readers of the executor's verbs and refusals."
    basis: tests/blitz-tests/tests/stand_act_popup.rs:58-101
    fact: F4
note: The report changes test-scope-summary:151 (`kbd-count`, `state-rows`) and :237 (`closed-option`, `lint-exclude`), and no group 1 entity holds either line, so check that those two sites were picked; F5 is left to the new-detail pass, as its home is `stand_keyboard`'s record at :151; no scoped detector fired.
