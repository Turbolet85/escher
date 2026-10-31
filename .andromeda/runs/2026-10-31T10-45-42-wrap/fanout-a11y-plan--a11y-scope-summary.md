proposals:
  - detector: D-a11y-surface
    severity: warning
    file: .andromeda/a11y-plan-detail/a11y-scope-summary.md
    near: 19
    section: "## 1. A11y Scope Summary"
    change: "Replace the clause from `neither is a `LeanTask`` to `the load page's lint 0` with: neither is a `LeanTask`, and since 2026-10-31-engine-debts each carries its own rows, a stated Tab sequence (`CELLS_TAB_ORDER`, `LOAD_TAB_ORDER`), keyboard rows in `stand_keyboard` and contrast rows in `stand_contrast`, the `a11y` leg's fourteen files unmoved: SC 2.1.1 and 1.4.3 are asserted on both, the load page's §5 open gap closed, and SC 2.4.3 on Cells, whose `cells-grid` is one Tab stop with `id`, `tabindex` 0, `aria_label` `Cells` and no role, arrows moving its current cell and Enter opening the editor; its role and cell semantics stay out, owned by a `.andromeda/residuals.md` line, and a person at a window is `unrunnable-here`; beside those rows their reach stays their own checks: keyed, `unkeyed_actionable` empty, the load page's lint 0."
    sidecar: "§1 Served pages: Cells and the load page now carry Tab-order, keyboard and contrast rows, the load page's open gap closed; `cells-grid` recorded as Cells' Tab stop, its role and cell semantics out by a residual line (2026-10-31-engine-debts)."
    rationale: "The line still says neither served page carries a Tab-order, keyboard-activation or contrast row and calls the load page's an open gap, while the report adds `cells-grid` as an interactive element and asserts both pages: Symbols / APIs F13 (`common/mod.rs:93-98` states `LOAD_TAB_ORDER` and `CELLS_TAB_ORDER`), Coverage of new surfaces (the `cells-grid` and Load page rows), Outcome (F12, a11y) and (F13, a11y), Expected amendments (the OPEN gap closed, Cells' row)."
    basis: .andromeda/a11y-plan-detail/a11y-scope-summary.md:19
  - detector: D-a11y-surface
    severity: warning
    file: .andromeda/a11y-plan-detail/a11y-scope-summary.md
    near: 23
    section: "## 1. A11y Scope Summary"
    change: "Replace the parenthesis after `the named limits` with: (the focus fixup firing no `blur`, no popup painted and Enter and Space opening nothing on a single select, `size` unread so a `multiple` select lists every row, `aria-activedescendant` unread), beside the reach it now states: a `multiple` select's rows and an `optgroup`'s options modelled, the native checked state stated in the tree on a checkable role; and close the line's provenance with: its limits as moved at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md."
    sidecar: "§1 Capability statement: the named limits restated, the focus fixup built with no `blur`, `multiple` and `optgroup` modelled, native checked stated in the tree, the select's new limits named (2026-10-31-engine-debts)."
    rationale: "The line names as standing limits what the forks ended: Symbols / APIs F3 (the fixup, no `blur`), F5 (native `toggled` on checkable roles), F6 (`multiple`), F7 (`optgroup`), F8 (no popup, Enter and Space open nothing); the report's Counts / qualifiers moved lists `a11y-scope-summary.md:23` as changed under `focus-fixup`, `multiple-select` and `optgroup`, and New text, by line shows `docs/capabilities.md` 134-139, 194-198 and 208-211 restating them."
    basis: .andromeda/a11y-plan-detail/a11y-scope-summary.md:23
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/a11y-scope-summary.md
    near: 18
    section: "## 1. A11y Scope Summary"
    change: "Append to the end of this entity: ; re-read at 2026-10-31-engine-debts, where `Cargo.lock` alone moves `pulldown-cmark` 0.13.4 to 0.13.5, no crate added or removed: the stand's graph still holds neither `accesskit_xplat` nor `accesskit_winit` (`cargo tree`, `accesskit_(xplat|winit)` 0 rows), so no stand binary builds a platform adapter still — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md"
    sidecar: "§1 Dioxus crates: the stand graph's re-read added for the lock bump, neither platform adapter joining it (2026-10-31-engine-debts)."
    rationale: "The entity records a re-read of the stand graph at each lock or dependency move; the report brings the next one: Dependencies (`pulldown-cmark` 0.13.4 to 0.13.5 in `Cargo.lock` alone, none added), Crates / modules (no crate added or removed), and Outcome's gates (`cargo tree … accesskit_(xplat|winit)`, exit 1, last line 0)."
    basis: .andromeda/a11y-plan-detail/a11y-scope-summary.md:18
note: "Line 19 is not among the two sites the report's `load-rows` sweep marks changed (`a11y-plan.md:142`, `test-scope-summary.md:150`), though its open-gap claim is the one `a11y-plan.md:142` closes; D-a11y-obs-schema reads no drift, the report moving no log site or schema and §3 defining no a11y violation schema."
