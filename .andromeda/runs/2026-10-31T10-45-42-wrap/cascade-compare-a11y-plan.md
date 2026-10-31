# Cascade compare — a11y-plan · wrap · 2026-10-31-engine-debts
E items 51 · held 28 · lost at classify 1 · lost at strong 2 · beside a placed fact 2 · cascade only 3 · listed - · body 0 · unplaced 0

## Lost — the full read carried these and the cascade did not; the wrap validates each, marked «(sampler)»

### a11y-scope-summary#1.9 · lost at classify · .andromeda/a11y-plan-detail/a11y-scope-summary.md:18
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/a11y-scope-summary.md
    near: 18
    section: "## 1. A11y Scope Summary"
    change: "Append to the end of this entity: ; re-read at 2026-10-31-engine-debts, where `Cargo.lock` alone moves `pulldown-cmark` 0.13.4 to 0.13.5, no crate added or removed: the stand's graph still holds neither `accesskit_xplat` nor `accesskit_winit` (`cargo tree`, `accesskit_(xplat|winit)` 0 rows), so no stand binary builds a platform adapter still — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md"
    sidecar: "§1 Dioxus crates: the stand graph's re-read added for the lock bump, neither platform adapter joining it (2026-10-31-engine-debts)."
    rationale: "The entity records a re-read of the stand graph at each lock or dependency move; the report brings the next one: Dependencies (`pulldown-cmark` 0.13.4 to 0.13.5 in `Cargo.lock` alone, none added), Crates / modules (no crate added or removed), and Outcome's gates (`cargo tree … accesskit_(xplat|winit)`, exit 1, last line 0)."
    basis: .andromeda/a11y-plan-detail/a11y-scope-summary.md:18

### a11y-strategy#2.4 · lost at strong · .andromeda/a11y-plan-detail/a11y-strategy.md:23
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/a11y-strategy.md
    near: 23
    section: "## 2. A11y Strategy"
    change: "New bullet after this line: - Since 2026-10-31-engine-debts the builder maps an `optgroup` to `Role::Group`, its non-empty `label` attribute its name, named in the tree and unpainted, and calls `set_multiselectable` on a `multiple` select, read as `ListBox`; what an assistive technology reads of them is not measured (`unrunnable-here`); both on the founder's «Построить сейчас» (2026-10-31), checked by `select_interaction` and `stand_act_select` — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md"
    sidecar: "2026-10-31-engine-debts: the builder's `optgroup` role and name and a `multiple` select's `set_multiselectable` recorded (F6, F7)."
    rationale: "Report Changes, Symbols / APIs: F6 `accessibility.rs`: `set_multiselectable` on a `multiple` select; F7 `accessibility.rs`: `optgroup` → `Role::Group`, its non-empty `label` attribute its name, unpainted. Coverage of new surfaces rows `multiple` select and `optgroup` name the role, the checks and AT `unrunnable-here`. This file's Role derivation group details what the builder writes and no line of it states either."

### keyboard-navigation#7.3 · lost at classify · beside a placed fact F6 (near 12, keyboard-navigation#1.5) · .andromeda/a11y-plan-detail/keyboard-navigation.md:101
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 101
    section: "## 5. Keyboard Navigation"
    change: "New record after this line, under the keyboard-harness procedure: - `text_range_no_change_event` (2 tests) pins, in both layout modes, typing in a text input and a range's key and pointerdown firing `input` and no `change` (tests/blitz-tests/tests/text_range_no_change_event.rs:59-76; tests/blitz-tests/tests/text_range_no_change_event.rs:78-97) — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md"
    sidecar: "keyboard-harness: new record, text_range_no_change_event."
    rationale: "Report Changes New checks and Counts / qualifiers moved New targets add text_range_no_change_event 2, pinning the input-alone reading the range record states."
    basis: tests/blitz-tests/tests/text_range_no_change_event.rs:78-97

### screen-reader-support#3.1 · lost at strong · .andromeda/a11y-plan-detail/screen-reader-support.md:31
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/screen-reader-support.md
    near: 31
    section: "## 7. Screen Reader Support"
    change: "New bullet after this line: - Since 2026-10-31-engine-debts the tree states a native checkbox's or radio's checked state: on a checkable role `toggled` reads `True` or `False` from `checkbox_input_checked()`, neither author attribute read on a native control; under any other role a native control writes nothing, so `pressed` never begins to read on it; the snapshot's `checked` still reads the native state from the element, and what an assistive technology reads of it stays unmeasured (packages/blitz-dom/src/accessibility.rs:199-224; tests/blitz-tests/tests/accessibility_states.rs:386-427) — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md"
    sidecar: "a11y-plan §7 sr-snapshot: native `toggled` on checkable roles added, `pressed` unmoved on a native control and the snapshot's `checked` unmoved (2026-10-31-engine-debts F5)."
    rationale: "Report Changes, F5, and Expected amendments, a11y-plan §7 with native `toggled`; `native-toggled` keeps the snapshot's own read unchanged, so the tree's new state is added detail and no reader moves."
    basis: packages/blitz-dom/src/accessibility.rs:199-224

### visual-design-verification#8.3 · lost at strong · beside a placed fact F26 (near 78, visual-design-verification#8.2) · .andromeda/a11y-plan-detail/visual-design-verification.md:79
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/visual-design-verification.md
    near: 79
    section: "## 6. Visual Design Verification"
    change: "New item after line 79: - An element under a transform whose determinant is 0 or not finite is no hit target, nor is anything in its subtree: `Node::hit_inner` answers `None` for the whole subtree, an invertible twin hit where it shows, in both layout modes; an engine change the founder ratified (F2, 2026-10-31) (packages/blitz-dom/src/node/node.rs:1472-1477; tests/blitz-tests/tests/hit_non_invertible_transform.rs:33-71 — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md)"
    sidecar: "§6 visual-visibility: F2's non-invertible transform takes no hit, recorded beside the other hit exclusions."
    rationale: "Report, Changes, Symbols / APIs, F2; New targets: `hit_non_invertible_transform` 2; `non-invertible`'s control unfired, so no master states it. This group lists the hit exclusions; relation: design-system §Depth Strategy takes the same fact (Expected amendments)."
    basis: packages/blitz-dom/src/node/node.rs:1472-1477

## Held

- a11y-scope-summary#1.10
- a11y-scope-summary#1.14
- a11y-strategy#2.3
- aria-patterns-roles#2.2
- aria-patterns-roles#2.5
- aria-patterns-roles#2.9
- ci-integration#1.0
- keyboard-navigation#1.1
- keyboard-navigation#2.3
- keyboard-navigation#2.6
- keyboard-navigation#2.7
- keyboard-navigation#3.3
- keyboard-navigation#4.7
- keyboard-navigation#5.7
- keyboard-navigation#7.2
- keyboard-navigation#7.4
- keyboard-navigation#7.5
- screen-reader-support#1.4
- screen-reader-support#4.1
- screen-reader-support#4.2
- screen-reader-support#6.2
- visual-design-verification#1.4
- visual-design-verification#1.5
- visual-design-verification#1.6
- visual-design-verification#1.7
- visual-design-verification#3.4
- visual-design-verification#6.2
- visual-design-verification#8.1

## Cascade only

- keyboard-navigation#1.5
- keyboard-navigation#3.4
- visual-design-verification#8.2
