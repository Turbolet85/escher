proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/visual-design-verification.md
    near: 12
    section: "## 6. Visual Design Verification"
    change: "In the `STATE_ROWS` sentence read `STATE_ROWS`, 20 colour states reached by input — return-chosen 2 (`flight-type` and `flight-return-date`, each 17.40), start-unparseable 2, booked 1, row-selected 3, hover 5, active 5 and 2 added since 2026-10-31-select-popup — pass 19 and exempt 1 (`flight-book` disabled, `#ffffff` on `#a0aec0`, 2.26); the rest of the line stands."
    sidecar: "visual-design-verification: `STATE_ROWS` 18 → 20, tally pass 19 · fail 0 · exempt 1 · no-text 0."
    rationale: "Report Counts / qualifiers `state-rows`: `STATE_ROWS` 18 → 20, tally (19, 0, 1, 0), naming visual-design-verification:12 among the changed sites."
    basis: escher-0.3.0/chunks/2026-10-31-select-popup/report.md:35
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/visual-design-verification.md
    near: 15
    section: "## 6. Visual Design Verification"
    change: "At the line's end read: the `a11y` leg reads 114 tests over 14 targets, `stand_contrast`'s 7 among them (§9; as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md); the rest of the line stands."
    sidecar: "visual-design-verification: the `a11y` leg 112 → 114 tests over 14 targets."
    rationale: "Report Counts / qualifiers `a11y-tally` and Gates: the a11y leg 14 · 112 · 0 → 14 · 114 · 0. The `a11y-tally` row lists three sites and not this one, whose wording «112 tests over 14 targets» its pattern did not reach; `stand_contrast` gains table rows and no named test in New text, so its 7 stands."
    basis: escher-0.3.0/chunks/2026-10-31-select-popup/report.md:33
  - detector: D-retired
    severity: warning
    file: .andromeda/a11y-plan-detail/visual-design-verification.md
    near: 16
    section: "## 6. Visual Design Verification"
    change: "Before the line's citations read: The driver's visual lint leaves a select's options out of its overlap reading in every document: a closed select's options, the snapshot giving each such `ListBoxOption` its select's client rect, and since 2026-10-31-select-popup an open list's rows and groups, the snapshot's builder reading `select_list_bounds` before the client rect; `overlaps` skips such an entry on either side, `Entry.in_select_box` reading the element through `closed_select_of`, which answers `None` for a select whose list is open, true also for an open list's row or group, and never the role, so CRUD's `role=\"option\"` rows are not read as a select's and a `multiple` select's rows are linted as usual; `clip_region` takes an open row's region from `visible_region` alone, no `contain: paint` cut; the stand's flight booker at boot reads no `overlap` in both layout modes, `stand_lint`'s `the_stand_at_boot_reads_its_measured_findings` reading `[0, 0, 0, 0]`, and `an_open_lists_rows_read_no_overlap_on_the_stand` reads none on an open list's rows, in both layout modes."
    sidecar: "visual-design-verification: `Entry.in_closed_select` retired for `in_select_box`, the overlap exclusion restated to take in an open list's rows and groups."
    rationale: "Report **Retired** names `Entry.in_closed_select`; Symbols / APIs Driver gives `in_select_box` and `clip_region`, Bridge gives `closed_select_of`'s open answer and the builder's read; Counts `closed-option` and `lint-exclude` both name visual-design-verification:16; Outcome (lint) gives the stand row."
    basis: escher-0.3.0/chunks/2026-10-31-select-popup/report.md:28
  - detector: D-a11y-surface
    severity: warning
    file: .andromeda/a11y-plan-detail/visual-design-verification.md
    near: 17
    section: "## 6. Visual Design Verification"
    change: "Replace the clause «that user-agent pair is not measured on a single select, while `select_interaction` measures a `multiple` select's plain and chosen rows at the user agent's pair» by: that user-agent pair stays unmeasured on a closed single select's own box, while `select_interaction` measures a `multiple` select's plain and chosen rows at the user agent's pair and, since 2026-10-31-select-popup, `select_popup_painted`'s `the_plain_pair_is_measured_at_the_default_sheet_and_at_the_stand` measures an open list's plain pair; the rest of the line stands."
    sidecar: "visual-design-verification: the single select's unmeasured pair narrowed to its closed box; the open list's plain pair measured by `select_popup_painted`."
    rationale: "Report `ua-pair`: capabilities' «A closed select's contrast pair is unmeasured» removed, the open list's plain pair measured by `select_popup_painted`, the closed select's own pair still unread, zero master sites in that wording and a detector's own sweep the control; this line is that sweep's hit."
    basis: escher-0.3.0/chunks/2026-10-31-select-popup/report.md:66
  - detector: D-a11y-surface
    severity: warning
    file: .andromeda/a11y-plan-detail/visual-design-verification.md
    near: 18
    section: "## 6. Visual Design Verification"
    change: "New bullet after this line: - An open single `select` lists its options in a box the engine paints over the page (`paint_select_list`, after the root's render and before the devtools overlays), its colours `SelectListView`'s associated constants and no sheet rule, hardcoded on the founder's F6 as the engine holds no tokens: `HIGHLIGHT_TEXT` `#000000` on `HIGHLIGHT_BACKGROUND` `#c8c8c8`, `DISABLED_TEXT` `#6d6d6d`, `BORDER` `#999999`; `select_popup_painted` reads the highlight and a disabled row as painted against those constants and the plain pair at the default sheet and at the stand, in both layout modes, so SC 1.4.3 is asserted on the list in the harness; a highlighted row's contrast stays unread by the lint, and what an assistive technology reads stays unmeasured (packages/blitz-dom/src/node/select_list.rs:78-91; packages/blitz-dom/src/node/select_list.rs:110-123; tests/blitz-tests/tests/select_popup_painted.rs:119-159; tests/blitz-tests/tests/select_popup_painted.rs:161-187 — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)"
    sidecar: "visual-design-verification: the open select list's colour constants and their SC 1.4.3 reading added."
    rationale: "Report Symbols / APIs (constants, Paint) and Coverage: the list is a new interactive element with SC 1.4.3 asserted and tokens hardcoded on F6; `inputs#I7` row 1 names the highlighted row's contrast unread by the lint. This file's contrast values hold no statement of the list, so §6's coverage of it is uncovered."
    basis: escher-0.3.0/chunks/2026-10-31-select-popup/report.md:10
  - detector: D-a11y-surface
    severity: warning
    file: .andromeda/a11y-plan-detail/visual-design-verification.md
    near: 40
    section: "## 6. Visual Design Verification"
    change: "New bullet after this line: - While a select's list stands focus stays on the select and the tree's focus with it, the current row shown by the painted highlight pair and read by no snapshot and no tree; the Tab walk goes on from the select, a Tab choosing the current row and closing before focus moves (tests/blitz-tests/tests/select_popup.rs:565-614; tests/blitz-tests/tests/select_popup.rs:329-361; tests/blitz-tests/tests/stand_keyboard.rs:1082-1111 — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)"
    sidecar: "visual-design-verification: focus and the highlight while a select's list stands added to the focus-ring values."
    rationale: "Report Keys, Coverage (`tree expanded`, «the highlight unread by the tree, stated») and Outcome (layouts): the new element's focus reading is §6's focus group and it holds no row for it."
    basis: escher-0.3.0/chunks/2026-10-31-select-popup/report.md:113
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/visual-design-verification.md
    near: 46
    section: "## 6. Visual Design Verification"
    change: "New bullet after this line: - A select's list opens, moves and closes with no motion and needs no `advance`, a screen read after a step reading the new state (tests/blitz-tests/tests/stand_act_popup.rs:408-423 — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)"
    sidecar: "visual-design-verification: the select list's no-motion reading added."
    rationale: "Report Outcome (design): no motion, no `advance` needed, met by `a_screen_read_after_a_step_reads_the_new_state_with_no_advance`, its `prefers-reduced-motion` gate entry green."
    basis: escher-0.3.0/chunks/2026-10-31-select-popup/report.md:157
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/visual-design-verification.md
    near: 59
    section: "## 6. Visual Design Verification"
    change: "New bullet after this line: - An open select's list is no `:open` state: `:open` and `flush_open_state` are unmoved, an author's `open` on a select steers nothing and `:open` does not match it, so no author style keys on a list standing (tests/blitz-tests/tests/select_popup.rs:628-651 — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)"
    sidecar: "visual-design-verification: `:open` stated as not reading a select's list."
    rationale: "Report Tree: `:open` and `flush_open_state` are unmoved, so an author's `open` on a select steers nothing; the neighbouring line states `:open` for `details` and `dialog` alone and a reader of it now meets a third thing that opens."
    basis: escher-0.3.0/chunks/2026-10-31-select-popup/report.md:16
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/visual-design-verification.md
    near: 61
    section: "## 6. Visual Design Verification"
    change: "New bullet after this line: - A group's `label` paints as a header told from an option row by weight and not by colour, a synthesized bold (`RunPaint.synthesize_bold`) heavier than a row on an emboldening build and a plain one alike, asserted by `a_groups_header_is_darker_than_a_row_by_weight` in both layout modes; a header chooses nothing (packages/blitz-paint/src/text.rs:586-593; packages/blitz-paint/src/text.rs:683-689; tests/blitz-tests/tests/select_popup_painted.rs:282-297; tests/blitz-tests/tests/select_popup.rs:805-855 — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)"
    sidecar: "visual-design-verification: a group header's not-colour-alone cue, a synthesized bold, added."
    rationale: "Report Paint, Retired «a group's label is unpainted», `group-label`, and the operator's direction «a header heavier than a row on every build» at `715798ce2`; both builds read after the fix (Gates)."
    basis: escher-0.3.0/chunks/2026-10-31-select-popup/report.md:18
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/visual-design-verification.md
    near: 85
    section: "## 6. Visual Design Verification"
    change: "New bullet after this line: - An open select's list answers a hit first: `hit_with_scrollbar` answers `select_list_hit` and no scrollbar with it, `hit_inner` not edited, so `hit` (the driver's `covered` through it), `set_hover_to` and `handle_pointerdown` read the list and a button under it is refused `covered`; the list paints whole past a box that clips its select and past an `opacity` ancestor, a point outside it reading what shows; its thumb is painted and takes no hit; a disabled, inert or hidden select or one with no row to show opens nothing, and a select made inert, disabled or hidden or removed closes its list with no panic (packages/blitz-dom/src/document.rs:962-988; packages/blitz-dom/src/document.rs:2404-2406; tests/blitz-tests/tests/select_popup.rs:443-472; tests/blitz-tests/tests/select_popup.rs:478-520; tests/blitz-tests/tests/select_popup.rs:677-716; tests/blitz-tests/tests/select_popup_painted.rs:211-235; tests/blitz-tests/tests/select_popup_painted.rs:312-351; tests/blitz-tests/tests/stand_act_popup.rs:228-245 — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)"
    sidecar: "visual-design-verification: the open list's hit, clip, opacity and close rules added to the visibility and hit-testing values."
    rationale: "Report The hit, Paint and Outcome (security, design); `hit-first` keeps visual-design-verification:85's own thumb hazard as no-change, so this follows it and leaves it unedited."
    basis: escher-0.3.0/chunks/2026-10-31-select-popup/report.md:12
note: "The report names no group for `STATE_ROWS`' two new rows and does not say whether «the lowest pass 5.11» still holds, so :12's proposal leaves both unstated; `a11y-tally`'s sweep missed :15's wording; D-a11y-obs-schema no drift, the report moving no log field or span and §3 defining no violation schema; a keypress panic after an ancestor gains `hidden` bears on the last proposal's close clause and is routed to P5 by the report."
