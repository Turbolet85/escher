proposals:
  - detector: D-retired
    severity: warning
    file: .andromeda/a11y-plan-detail/a11y-scope-summary.md
    near: 23
    section: "## 1. A11y Scope Summary"
    change: "In the Capability statement's named limits, replace `no popup painted` with `an option of a closed select not clicked, the highlighted row in no snapshot and no tree, the list's thumb painted and not draggable`."
    sidecar: "a11y-scope-summary: the capability statement's named limits drop «no popup painted» and name the open list's own limits (2026-10-31-select-popup)."
    rationale: "The report's Retired bullet retires «no popup is painted»; its `no-popup` row counts a11y-scope-summary:23 as changed, and its New text lists docs/capabilities.md's new limits (the closed option, the unread highlight, the thumb)."
    basis: escher-0.3.0/chunks/2026-10-31-select-popup/report.md:28
  - detector: D-retired
    severity: warning
    file: .andromeda/a11y-plan-detail/a11y-scope-summary.md
    near: 23
    section: "## 1. A11y Scope Summary"
    change: "In the Capability statement's named limits, delete ` and Enter and Space opening nothing on a single select`; no limit replaces it, since Enter and Space now open the list."
    sidecar: "a11y-scope-summary: the capability statement's named limits drop «Enter and Space opening nothing on a single select» (2026-10-31-select-popup)."
    rationale: "The report's Retired bullet retires «Enter and Space open nothing on a single select»; its `enter-space` row counts a11y-scope-summary:23 as changed, and `press_select_key` opens on Enter and on a Space with no typed text standing (Symbols / APIs, Keys)."
    basis: escher-0.3.0/chunks/2026-10-31-select-popup/report.md:28
  - detector: D-retired
    severity: warning
    file: .andromeda/a11y-plan-detail/a11y-scope-summary.md
    near: 23
    section: "## 1. A11y Scope Summary"
    change: "In the Capability statement's named limits, replace `a `multiple` select's `size` unread` with `a `multiple` select with no `size` listing every row`."
    sidecar: "a11y-scope-summary: the capability statement's `size` limit restated, `size` now read on a `multiple` select (2026-10-31-select-popup)."
    rationale: "The report's Retired bullet retires «a `multiple` select's `size` is unread»; its `size-unread` row counts a11y-scope-summary:23 as changed, and its New text gives docs/capabilities.md's standing limit in these words."
    basis: escher-0.3.0/chunks/2026-10-31-select-popup/report.md:28
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/a11y-scope-summary.md
    near: 19
    section: "## 1. A11y Scope Summary"
    change: "Add after Served pages: - **Select list** — a single select's list is an engine overlay on every document, remote pages included, held in `BaseDocument`'s one `select_list` field and painted by `paint_scene` through `paint_select_list` (packages/blitz-dom/src/node/select_list.rs:28-46; packages/blitz-paint/src/render/select_list.rs:21-40); a click, Enter or a Space with no typed text standing opens it, its keys are `press_open_list_key`'s closed match, Tab chooses and closes before focus moves either way, and focus stays on the select (packages/blitz-dom/src/events/keyboard.rs:32-34; packages/blitz-dom/src/events/keyboard.rs:215-250; tests/blitz-tests/tests/stand_keyboard.rs:1113-1134); the tree reads `ComboBox` with `expanded` from the engine's state and no `aria-expanded` read, the chosen option `selected`, the highlight unread by the tree and the snapshot (packages/blitz-dom/src/accessibility.rs:238-242; tests/blitz-tests/tests/select_popup.rs:565-614); SC 2.1.1, 2.4.3 and 1.4.3 are asserted in the harness in both layout modes by `select_popup` (25), `select_popup_painted` (6), `stand_act_popup` (12) and two tests of `stand_keyboard`, none of the three new files in the `a11y` leg, which stays fourteen files; what an assistive technology reads stays unmeasured, the windowed list read and not run, a highlighted row's contrast unread by the lint, a closed select's own contrast pair unread and a single select's `size` unread — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md"
    sidecar: "a11y-scope-summary: new scope entity, the select list, with its keys, tree reading, assertions and what stays unmeasured (2026-10-31-select-popup)."
    rationale: "The report's Coverage of new surfaces adds `select list, engine overlay` and `tree expanded on a single select` with SC 2.1.1, 2.4.3 and 1.4.3 asserted and an assistive technology's reading `unrunnable-here`; §1 names the entities needing assertions and holds no row for it."
    basis: escher-0.3.0/chunks/2026-10-31-select-popup/report.md:110
note: "D-a11y-surface and D-a11y-obs-schema find no violation in this file (the report moves no log field or violation schema); line 23's provenance tail still credits engine-debts for the named limits, a citation left unproposed."
