proposals:
  - detector: D-a11y-surface
    severity: warning
    file: .andromeda/a11y-plan-detail/screen-reader-support.md
    near: 11
    section: "## 7. Screen Reader Support"
    change: "In the clause «and a closed select's options under it, each with its select's client rect as its `bounds`» append «, an open list's rows each their own», the clause then reading «and a closed select's options under it, each with its select's client rect as its `bounds`, an open list's rows each their own»; the rest of the line stands."
    sidecar: "Screen Reader Support: a select's snapshot reading gains the open list, whose rows read their own `bounds` where a closed select's options read the select's client rect."
    rationale: "The chunk adds an interactive list and this line states the select's option `bounds` for the closed case alone; report Changes, Bridge has `closed_select_of` answering `None` for a select whose list is open and the builder reading `select_list_bounds(node)` first, and Counts `closed-option` lists screen-reader-support:11 as changed. The clause is kept short because the line already stands near the 2 000-char limit; the detail goes in the new row at 13."
    basis: .andromeda/a11y-plan-detail/screen-reader-support.md:11
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/screen-reader-support.md
    near: 13
    section: "## 7. Screen Reader Support"
    change: "New row: - A single select states its list's open state in the tree with `set_expanded(open)`, a closed bool from engine state, and reads no `aria-expanded`, every other element keeping that read; `:open` and `flush_open_state` are unmoved, so an author's `open` on a select steers nothing; with its list open it reads `ComboBox`, `expanded`, its chosen option `selected`, the tree's focus staying on the select; the highlighted row is in no snapshot and no tree, and no `expanded` node key is added, `NODE_FIELDS` staying thirteen; SC 2.1.1, 2.4.3 and 1.4.3 are asserted in the harness, what an assistive technology reads of the list `unrunnable-here` (packages/blitz-dom/src/accessibility.rs:238-242; tests/blitz-tests/tests/select_popup.rs:565-614; tests/blitz-tests/tests/select_popup.rs:628-651) — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md"
    sidecar: "Screen Reader Support: new row for a single select's native `expanded` in the tree, the highlight unread by tree and snapshot, and no `expanded` node key."
    rationale: "Report Changes, Tree (`accessibility.rs:238-242`) and Bridge (`NODE_FIELDS` stays thirteen), Coverage row `tree expanded on a single select`, Decisions (no `expanded` node key, inputs#I6), Outcome (a11y) and Expected amendments `a11y-plan` §7; no row of this file states the select's `expanded` or the highlight's absence."
    basis: .andromeda/a11y-plan-detail/screen-reader-support.md:11
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/screen-reader-support.md
    near: 13
    section: "## 7. Screen Reader Support"
    change: "New row: - An open list's rows cross the ratified carriers as each row's own `bounds` in the snapshot and the diff, `differs` naming `bounds`; `closed_select_of` answers `None` for a select whose list is open and the snapshot's builder reads `select_list_bounds(node)` before the client rect, `NodeState` gaining no field; a group with no label reads the whole list's box; an Escape close returns the rows to the select's box, and a shot of a row crops to it and leaves the list open (packages/dioxus-native-dom/src/snapshot.rs:152-155; tests/blitz-tests/tests/stand_act_popup.rs:127-155; tests/blitz-tests/tests/stand_act_popup.rs:287-316) — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md"
    sidecar: "Screen Reader Support: new row for how an open list's rows read in the snapshot, the diff and an element shot."
    rationale: "Report Changes, Bridge and Schema / config (a row's own `bounds` in the snapshot and the diff), Deviations (a group with no label reads the whole list's box) and Outcome (layouts, design) name these readings; this file's snapshot rows state the closed case and the `multiple` select's rows alone."
    basis: .andromeda/a11y-plan-detail/screen-reader-support.md:12
note: "D-a11y-obs-schema not violated, report Schema / config moving no log field or span; a11y-plan body line 202 (§7) still reads «named in the tree and unpainted», retired by the report's `group-label`, outside this file's proposals."
