proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/a11y-plan-detail/visual-design-verification.md
    near: 12
    section: "## 6. Visual Design Verification"
    change: "In the `STATE_ROWS` clause write `20 colour states reached by input` for `18 colour states reached by input`, append `, and two more, added by 2026-10-31-select-popup` after `active 5`, and write `pass 19 and exempt 1` for `pass 17 and exempt 1`; the rest of the line stands."
    sidecar: "visual-contrast: `STATE_ROWS` 18 to 20, pass 19 and exempt 1, tally (19, 0, 1, 0)."
    rationale: "The report's Counts, `state-rows`, moves `STATE_ROWS` 18 to 20 with tally (19, 0, 1, 0) and names visual-design-verification:12 as changed."
    basis: tests/blitz-tests/tests/stand_contrast.rs:304-316
  - detector: D-cascade
    severity: warning
    file: .andromeda/a11y-plan-detail/visual-design-verification.md
    near: 15
    section: "## 6. Visual Design Verification"
    change: "Write `the a11y leg reads 114 tests over 14 targets, stand_contrast's 7 among them (§9; as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)` for the closing clause reading 112 and the engine-debts report; the rest of the line stands."
    sidecar: "visual-contrast: the `a11y` leg's count in the lint row, 112 to 114 tests over 14 targets."
    rationale: "The report's Counts, `a11y-tally`, reads the a11y leg 14 · 114 · 0 and its Harness section keeps the leg at fourteen files; the +2 are `stand_keyboard`'s under `kbd-count`, so `stand_contrast` stays at 7."
  - detector: D-retired
    severity: warning
    file: .andromeda/a11y-plan-detail/visual-design-verification.md
    near: 16
    section: "## 6. Visual Design Verification"
    change: "Replace the text before `the stand's flight booker at boot` with: `- The driver's visual lint leaves a select's options out of its overlap reading in every document, the snapshot giving a closed select's ListBoxOption its select's client rect and an open list's row its painted row's bounds: overlaps skips such an entry on either side, Entry.in_select_box true for an option of a closed select, read through closed_select_of and never the role, and also for an open list's row or group, so CRUD's role=\"option\" rows are not read as a select's and a multiple select's rows are linted as usual; clip_region takes an open row's region from visible_region alone, no contain: paint cut;` and keep the rest of the line as it stands."
    sidecar: "visual-contrast: the lint's select exclusion restated, `Entry.in_closed_select` to `in_select_box`, with an open row's `clip_region`."
    rationale: "The report's Retired bullet names `Entry.in_closed_select`; Symbols, Driver, states `in_select_box` and `clip_region`; Bridge states `closed_select_of` answers `None` for an open list and the builder reads `select_list_bounds` first; `lint-exclude` and `closed-option` name visual-design-verification:16 as changed."
    basis: packages/escher-driver/src/lint.rs:339-342
  - detector: D-cascade
    severity: warning
    file: .andromeda/a11y-plan-detail/visual-design-verification.md
    near: 17
    section: "## 6. Visual Design Verification"
    change: "Write `a closed single select's own user-agent pair stays unread, the open list's plain pair is measured by select_popup_painted, while select_interaction measures` for `that user-agent pair is not measured on a single select, while select_interaction measures`; the rest of the line stands."
    sidecar: "visual-contrast: the single select's unmeasured pair split into a closed select's unread pair and an open list's measured one."
    rationale: "The report's Counts, `ua-pair`, removes capabilities' line that a closed select's contrast pair is unmeasured, measures the open list's plain pair by `select_popup_painted`, keeps the closed select's own pair unread and leaves a detector's own sweep as the control, this wording being outside its pattern."
    basis: tests/blitz-tests/tests/select_popup_painted.rs:161-187
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/visual-design-verification.md
    near: 16
    section: "## 6. Visual Design Verification"
    change: "Add after this line: `- The lint reads no overlap on an open list's row on the stand, the planted pair still reads, at_boot stays [0, 0, 0, 0], and a highlighted row's contrast is unread by the lint, a stated limit (tests/blitz-tests/tests/stand_lint.rs:493-519; tests/blitz-tests/tests/stand_act_popup.rs:318-350 — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)`"
    sidecar: "visual-contrast: new row, the lint over an open list, its highlighted row's contrast a stated limit."
    rationale: "The report's Outcome, (lint), is met by `an_open_lists_rows_read_no_overlap_on_the_stand` and `the_lint_reads_no_overlap_on_a_row_and_the_planted_pair_still_reads`; inputs#I7 lists the highlighted row's contrast as a stated limit."
    fact: F22
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/visual-design-verification.md
    near: 17
    section: "## 6. Visual Design Verification"
    change: "Add after this line: `- select_popup_painted reads the highlight and a disabled row as painted against SelectListView's constants and measures the open list's plain pair at the default sheet and at the stand, in both layout modes (tests/blitz-tests/tests/select_popup_painted.rs:119-159; tests/blitz-tests/tests/select_popup_painted.rs:161-187 — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)`"
    sidecar: "visual-contrast: new row, `select_popup_painted`'s reading of the highlight, a disabled row and the open list's plain pair."
    rationale: "The report's `ua-pair`, Coverage and New text name these two tests in `select_popup_painted`, green in both layout modes per acceptance `v030-17`."
    fact: F21
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/visual-design-verification.md
    near: 18
    section: "## 6. Visual Design Verification"
    change: "Add after this line: `- The open list's colours are engine constants on SelectListView, not tokens, the engine holding none: HIGHLIGHT_TEXT #000000, HIGHLIGHT_BACKGROUND #c8c8c8, DISABLED_TEXT #6d6d6d, BORDER #999999 (packages/blitz-dom/src/node/select_list.rs:110-123 — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)`"
    sidecar: "visual-contrast: new row, the list's four colour constants."
    rationale: "The report's Symbols, the list's state, names the constants and their values; Coverage records them as engine constants on the founder's F6, the engine holding no tokens."
    fact: F20
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/visual-design-verification.md
    near: 18
    section: "## 6. Visual Design Verification"
    change: "Add after this line: `- An open list's row height is the select's font on one line, so a larger authored font clips, a stated limit (docs/capabilities.md:211-213 — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)`"
    sidecar: "visual-contrast: new row, one row height, a larger authored font clipping as a stated limit."
    rationale: "The report's Deviations state the row height is the select's font on one line; inputs#I7 lists a larger authored font clipping as a stated limit."
    fact: F26
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/visual-design-verification.md
    near: 52
    section: "## 6. Visual Design Verification"
    change: "Add after this line: `- Observed absent — motion on an open select list, which needs no advance: a screen read after a step reads the new state, pinned by a_screen_read_after_a_step_reads_the_new_state_with_no_advance (tests/blitz-tests/tests/stand_act_popup.rs:408-423 — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)`"
    sidecar: "visual-motion: new row, the open list has no motion and needs no `advance`."
    rationale: "The report's Outcome, (design), records no motion and no `advance` needed as met by this test, its `prefers-reduced-motion` gate entry green."
    fact: F24
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/visual-design-verification.md
    near: 60
    section: "## 6. Visual Design Verification"
    change: "Add after this line: `- A group's header in an open list is heavier than a row by a synthesized bold, on plain and emboldening builds, pinned by a_groups_header_is_darker_than_a_row_by_weight (tests/blitz-tests/tests/select_popup_painted.rs:282-297 — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)`"
    sidecar: "visual-state-color: new row, a group's header heavier than a row by weight."
    rationale: "The report's Symbols, Paint, states the synthesized bold, and Deviations record the operator's direction and fix `715798ce2` for a header heavier than a row on every build, with the test read green on both builds."
    fact: F23
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/visual-design-verification.md
    near: 85
    section: "## 6. Visual Design Verification"
    change: "Add after this line: `- The open list paints whole past an opacity ancestor and a box that clips its select, a point outside it reads what shows, and an element under it refuses covered (tests/blitz-tests/tests/select_popup_painted.rs:211-235; tests/blitz-tests/tests/select_popup.rs:677-716; tests/blitz-tests/tests/stand_act_popup.rs:228-245 — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)`"
    sidecar: "visual-visibility: new row, the list painted past clips and `opacity`, with what a point outside and an element under it read."
    rationale: "The report's Outcome, (design), records clips and `opacity` as met by these tests; New text lists `a_button_under_the_list_is_refused_covered` for the element under the list; `hit-first` leaves the thumb hazard row itself unchanged."
    fact: F25
note: The report names no kind for the two new `STATE_ROWS` rows and no new lowest pass, so the proposal for line 12 keeps 5.11 and says only «two more»; line 15's 112 lies outside `a11y-tally`'s three listed hits; body lines 64 (`STATE_ROWS` in §3) and 202 (a group's label in §7) are not this group's; F27 belongs to ci-integration.
