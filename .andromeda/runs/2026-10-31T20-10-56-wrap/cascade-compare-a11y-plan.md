# Cascade compare — a11y-plan · wrap · 2026-10-31-select-popup
E items 40 · held 20 · lost at classify 1 · lost at strong 1 · beside a placed fact 7 · cascade only 10 · listed - · body 0 · unplaced 0

## Lost — the full read carried these and the cascade did not; the wrap validates each, marked «(sampler)»

### a11y-scope-summary#1.10 · lost at strong · beside a placed fact F1 (near 10, a11y-scope-summary#1.3) · .andromeda/a11y-plan-detail/a11y-scope-summary.md:19
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/a11y-scope-summary.md
    near: 19
    section: "## 1. A11y Scope Summary"
    change: "Add after Served pages: - **Select list** — a single select's list is an engine overlay on every document, remote pages included, held in `BaseDocument`'s one `select_list` field and painted by `paint_scene` through `paint_select_list` (packages/blitz-dom/src/node/select_list.rs:28-46; packages/blitz-paint/src/render/select_list.rs:21-40); a click, Enter or a Space with no typed text standing opens it, its keys are `press_open_list_key`'s closed match, Tab chooses and closes before focus moves either way, and focus stays on the select (packages/blitz-dom/src/events/keyboard.rs:32-34; packages/blitz-dom/src/events/keyboard.rs:215-250; tests/blitz-tests/tests/stand_keyboard.rs:1113-1134); the tree reads `ComboBox` with `expanded` from the engine's state and no `aria-expanded` read, the chosen option `selected`, the highlight unread by the tree and the snapshot (packages/blitz-dom/src/accessibility.rs:238-242; tests/blitz-tests/tests/select_popup.rs:565-614); SC 2.1.1, 2.4.3 and 1.4.3 are asserted in the harness in both layout modes by `select_popup` (25), `select_popup_painted` (6), `stand_act_popup` (12) and two tests of `stand_keyboard`, none of the three new files in the `a11y` leg, which stays fourteen files; what an assistive technology reads stays unmeasured, the windowed list read and not run, a highlighted row's contrast unread by the lint, a closed select's own contrast pair unread and a single select's `size` unread — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md"
    sidecar: "a11y-scope-summary: new scope entity, the select list, with its keys, tree reading, assertions and what stays unmeasured (2026-10-31-select-popup)."
    rationale: "The report's Coverage of new surfaces adds `select list, engine overlay` and `tree expanded on a single select` with SC 2.1.1, 2.4.3 and 1.4.3 asserted and an assistive technology's reading `unrunnable-here`; §1 names the entities needing assertions and holds no row for it."
    basis: escher-0.3.0/chunks/2026-10-31-select-popup/report.md:110

### keyboard-navigation#2.4 · lost at strong · beside a placed fact F4 (near 10, keyboard-navigation#1.3) · .andromeda/a11y-plan-detail/keyboard-navigation.md:27
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 27
    section: "## 5. Keyboard Navigation"
    change: "- On an open list the pointer answers first: `hit_with_scrollbar` reads `select_list_hit` before anything else; a move highlights an enabled row and a later key stands from it; a press on a row selects no text and the release chooses the row under it, `click_list_row` behind `click_lands_on_row` the `option` arm's new path; a disabled row changes nothing and leaves the list open; a group's header chooses nothing; the wheel scrolls the list over it; the thumb is painted and takes no hit (packages/blitz-dom/src/document.rs:2404-2406; packages/blitz-dom/src/events/pointer.rs:595-611; packages/blitz-dom/src/events/pointer.rs:613-617; packages/blitz-dom/src/events/pointer.rs:635-640; packages/blitz-dom/src/events/pointer.rs:642-655; tests/blitz-tests/tests/select_popup.rs:377-388; tests/blitz-tests/tests/select_popup.rs:751-781; tests/blitz-tests/tests/select_popup.rs:783-803; tests/blitz-tests/tests/select_popup.rs:805-855; tests/blitz-tests/tests/select_popup_painted.rs:312-351) — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md"
    sidecar: "keyboard-navigation: new row, the pointer on an open list's rows."
    rationale: "Report Symbols / APIs, The hit and Pointer; Deviations: a press on a row starts no text drag and the release chooses the row under it."
    basis: escher-0.3.0/chunks/2026-10-31-select-popup/report.md:14

### keyboard-navigation#2.5 · lost at strong · beside a placed fact F7 (near 10, keyboard-navigation#1.3) · .andromeda/a11y-plan-detail/keyboard-navigation.md:28
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 28
    section: "## 5. Keyboard Navigation"
    change: "- An open list closes with no event on Escape, a second click, a press outside it (`close_list_outside`), a page scroll or a move of its select (`reconcile_select_list` at the next resolve) and a wheel outside it (`close_select_list`), and closes where its select is removed or made disabled, inert or hidden, each such close marking the select in the changed set, a highlight or a wheel move inside the list marking nothing; a `:focus` rule moving the select's box leaves a list the click opened (`fresh`); with an ancestor hidden a later key press panics at packages/blitz-dom/src/node/node.rs:1775, for any control and with no list opened, so `select_popup` skips the key press in that one row, the row owed in security-plan §Error Handling (packages/blitz-dom/src/events/pointer.rs:415-420; packages/blitz-dom/src/document.rs:881-889; packages/blitz-dom/src/document.rs:891-907; packages/blitz-dom/src/document.rs:1408-1410; packages/blitz-dom/src/resolve.rs:133-135; tests/blitz-tests/tests/select_popup.rs:401-426; tests/blitz-tests/tests/select_popup.rs:478-520; tests/blitz-tests/tests/select_popup.rs:1105-1122) — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md"
    sidecar: "keyboard-navigation: new row, how an open list closes, with the key-press panic beside it named."
    rationale: "Report Symbols / APIs, Pointer and The resolve; Spec claims disproved `changed-set` and `panic-pos`; Deviations' `fresh`; Outcome (security) met to its letter with a defect beside it. Binds with security-plan §Error Handling, the panic's row."
    basis: escher-0.3.0/chunks/2026-10-31-select-popup/report.md:96

### keyboard-navigation#3.7 · lost at strong · beside a placed fact F6 (near 13, keyboard-navigation#1.6) · .andromeda/a11y-plan-detail/keyboard-navigation.md:45
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 45
    section: "## 5. Keyboard Navigation"
    change: "- A `multiple` select's `size`, a whole number capped at 65534, `0` or a bad value hinting nothing, makes it that many rows tall and scrollable, and a key brings the next row in: `apply_row_gesture` calls `scroll_into_view` on the current row only where `carries_valid_size`; `press_multiple_select_key` and `click_select_row` keep their semantics, and a single select's `size` stays unread (packages/blitz-dom/src/events/keyboard.rs:423-430; packages/blitz-dom/src/events/keyboard.rs:452-459; packages/blitz-dom/src/stylo.rs:1124-1139; packages/blitz-dom/assets/default.css:183-193; tests/blitz-tests/tests/select_popup.rs:995-1032) — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md"
    sidecar: "keyboard-navigation: new row, `size` on a `multiple` select and the row a key brings into view."
    rationale: "Report Retired names «a `multiple` select's `size` is unread»; Symbols / APIs, Keys and `size`; Coverage of new surfaces: a11y the row brought into view by key."
    basis: escher-0.3.0/chunks/2026-10-31-select-popup/report.md:21

### screen-reader-support#1.6 · lost at classify · beside a placed fact F17 (near 33, screen-reader-support#3.1) · .andromeda/a11y-plan-detail/screen-reader-support.md:13
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/screen-reader-support.md
    near: 13
    section: "## 7. Screen Reader Support"
    change: "New row: - A single select states its list's open state in the tree with `set_expanded(open)`, a closed bool from engine state, and reads no `aria-expanded`, every other element keeping that read; `:open` and `flush_open_state` are unmoved, so an author's `open` on a select steers nothing; with its list open it reads `ComboBox`, `expanded`, its chosen option `selected`, the tree's focus staying on the select; the highlighted row is in no snapshot and no tree, and no `expanded` node key is added, `NODE_FIELDS` staying thirteen; SC 2.1.1, 2.4.3 and 1.4.3 are asserted in the harness, what an assistive technology reads of the list `unrunnable-here` (packages/blitz-dom/src/accessibility.rs:238-242; tests/blitz-tests/tests/select_popup.rs:565-614; tests/blitz-tests/tests/select_popup.rs:628-651) — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md"
    sidecar: "Screen Reader Support: new row for a single select's native `expanded` in the tree, the highlight unread by tree and snapshot, and no `expanded` node key."
    rationale: "Report Changes, Tree (`accessibility.rs:238-242`) and Bridge (`NODE_FIELDS` stays thirteen), Coverage row `tree expanded on a single select`, Decisions (no `expanded` node key, inputs#I6), Outcome (a11y) and Expected amendments `a11y-plan` §7; no row of this file states the select's `expanded` or the highlight's absence."
    basis: .andromeda/a11y-plan-detail/screen-reader-support.md:11

### screen-reader-support#1.6 · lost at classify · beside a placed fact F18 (near 33, screen-reader-support#3.1) · .andromeda/a11y-plan-detail/screen-reader-support.md:13
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/screen-reader-support.md
    near: 13
    section: "## 7. Screen Reader Support"
    change: "New row: - An open list's rows cross the ratified carriers as each row's own `bounds` in the snapshot and the diff, `differs` naming `bounds`; `closed_select_of` answers `None` for a select whose list is open and the snapshot's builder reads `select_list_bounds(node)` before the client rect, `NodeState` gaining no field; a group with no label reads the whole list's box; an Escape close returns the rows to the select's box, and a shot of a row crops to it and leaves the list open (packages/dioxus-native-dom/src/snapshot.rs:152-155; tests/blitz-tests/tests/stand_act_popup.rs:127-155; tests/blitz-tests/tests/stand_act_popup.rs:287-316) — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md"
    sidecar: "Screen Reader Support: new row for how an open list's rows read in the snapshot, the diff and an element shot."
    rationale: "Report Changes, Bridge and Schema / config (a row's own `bounds` in the snapshot and the diff), Deviations (a group with no label reads the whole list's box) and Outcome (layouts, design) name these readings; this file's snapshot rows state the closed case and the `multiple` select's rows alone."
    basis: .andromeda/a11y-plan-detail/screen-reader-support.md:12

### visual-design-verification#3.10 · lost at strong · .andromeda/a11y-plan-detail/visual-design-verification.md:40
  - detector: D-a11y-surface
    severity: warning
    file: .andromeda/a11y-plan-detail/visual-design-verification.md
    near: 40
    section: "## 6. Visual Design Verification"
    change: "New bullet after this line: - While a select's list stands focus stays on the select and the tree's focus with it, the current row shown by the painted highlight pair and read by no snapshot and no tree; the Tab walk goes on from the select, a Tab choosing the current row and closing before focus moves (tests/blitz-tests/tests/select_popup.rs:565-614; tests/blitz-tests/tests/select_popup.rs:329-361; tests/blitz-tests/tests/stand_keyboard.rs:1082-1111 — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)"
    sidecar: "visual-design-verification: focus and the highlight while a select's list stands added to the focus-ring values."
    rationale: "Report Keys, Coverage (`tree expanded`, «the highlight unread by the tree, stated») and Outcome (layouts): the new element's focus reading is §6's focus group and it holds no row for it."
    basis: escher-0.3.0/chunks/2026-10-31-select-popup/report.md:113

### visual-design-verification#4.1 · lost at classify · beside a placed fact F24 (near 52, visual-design-verification#5.1) · .andromeda/a11y-plan-detail/visual-design-verification.md:46
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/visual-design-verification.md
    near: 46
    section: "## 6. Visual Design Verification"
    change: "New bullet after this line: - A select's list opens, moves and closes with no motion and needs no `advance`, a screen read after a step reading the new state (tests/blitz-tests/tests/stand_act_popup.rs:408-423 — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)"
    sidecar: "visual-design-verification: the select list's no-motion reading added."
    rationale: "Report Outcome (design): no motion, no `advance` needed, met by `a_screen_read_after_a_step_reads_the_new_state_with_no_advance`, its `prefers-reduced-motion` gate entry green."
    basis: escher-0.3.0/chunks/2026-10-31-select-popup/report.md:157

### visual-design-verification#6.2 · lost at strong · beside a placed fact F23 (near 60, visual-design-verification#6.3) · .andromeda/a11y-plan-detail/visual-design-verification.md:59
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/visual-design-verification.md
    near: 59
    section: "## 6. Visual Design Verification"
    change: "New bullet after this line: - An open select's list is no `:open` state: `:open` and `flush_open_state` are unmoved, an author's `open` on a select steers nothing and `:open` does not match it, so no author style keys on a list standing (tests/blitz-tests/tests/select_popup.rs:628-651 — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)"
    sidecar: "visual-design-verification: `:open` stated as not reading a select's list."
    rationale: "Report Tree: `:open` and `flush_open_state` are unmoved, so an author's `open` on a select steers nothing; the neighbouring line states `:open` for `details` and `dialog` alone and a reader of it now meets a third thing that opens."
    basis: escher-0.3.0/chunks/2026-10-31-select-popup/report.md:16

### visual-design-verification#6.4 · lost at classify · .andromeda/a11y-plan-detail/visual-design-verification.md:61
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/visual-design-verification.md
    near: 61
    section: "## 6. Visual Design Verification"
    change: "New bullet after this line: - A group's `label` paints as a header told from an option row by weight and not by colour, a synthesized bold (`RunPaint.synthesize_bold`) heavier than a row on an emboldening build and a plain one alike, asserted by `a_groups_header_is_darker_than_a_row_by_weight` in both layout modes; a header chooses nothing (packages/blitz-paint/src/text.rs:586-593; packages/blitz-paint/src/text.rs:683-689; tests/blitz-tests/tests/select_popup_painted.rs:282-297; tests/blitz-tests/tests/select_popup.rs:805-855 — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)"
    sidecar: "visual-design-verification: a group header's not-colour-alone cue, a synthesized bold, added."
    rationale: "Report Paint, Retired «a group's label is unpainted», `group-label`, and the operator's direction «a header heavier than a row on every build» at `715798ce2`; both builds read after the fix (Gates)."
    basis: escher-0.3.0/chunks/2026-10-31-select-popup/report.md:18

## Held

- a11y-scope-summary#1.14
- a11y-strategy#1.2
- a11y-strategy#2.5
- aria-patterns-roles#2.5
- aria-patterns-roles#2.7
- ci-integration#1.0
- keyboard-navigation#2.3
- keyboard-navigation#3.10
- keyboard-navigation#3.12
- keyboard-navigation#3.16
- keyboard-navigation#3.3
- keyboard-navigation#7.6
- keyboard-navigation#7.7
- screen-reader-support#1.4
- visual-design-verification#1.4
- visual-design-verification#1.6
- visual-design-verification#1.7
- visual-design-verification#1.8
- visual-design-verification#1.9
- visual-design-verification#8.6

## Cascade only

- a11y-scope-summary#1.3
- keyboard-navigation#1.3
- keyboard-navigation#1.6
- keyboard-navigation#2.2
- keyboard-navigation#3.4
- keyboard-navigation#3.5
- keyboard-navigation#7.2
- screen-reader-support#3.1
- visual-design-verification#5.1
- visual-design-verification#6.3
