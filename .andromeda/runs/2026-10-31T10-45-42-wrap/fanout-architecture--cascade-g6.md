proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 249
    section: "## Standard Contracts"
    change: "In the snapshot's `value` clause, `value` a `select`'s chosen option's value (`BaseDocument::select_value`, below) now reads: `value` a `select`'s chosen option's value, a `multiple` select's first chosen option in tree order (`BaseDocument::select_value`, below); the line's `checked` clause stands."
    sidecar: "The snapshot's `value` of a `multiple` select is named: its first chosen option in tree order, through `select_value`."
    rationale: "Report, Symbols / APIs F6: `select_value` now reads a `multiple` select's first chosen option in tree order, and `snapshot.rs`'s `value` reads `select_value`; Counts, `multiple-select` lists `standard-contracts.md:249` changed, while `native-toggled` holds the same line's own `checked` read as no change."
    basis: packages/dioxus-native-dom/src/snapshot.rs:256-257
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 250
    section: "## Standard Contracts"
    change: "After `aria-invalid` in the clause naming what the engine's builder writes, add: on a native checkable control the builder reads neither `aria-pressed` nor `aria-checked`, writing `toggled` from `checkbox_input_checked()` under a checkable role and nothing under any other, so `pressed` never reads on one."
    sidecar: "The snapshot's account of the builder's state writes gains the native control's case, neither author attribute read."
    rationale: "Report, Symbols / APIs F5: `is_native_checkable && is_checkable(role)` writes `Toggled::True` or `False` from `checkbox_input_checked()`, under any other role a native control writes nothing, so `pressed` never begins to read on it, and neither author attribute is read on a native control; this clause states the author attributes alone, and it reads no `native-toggled` hit, so the sweep did not place it."
    basis: packages/blitz-dom/src/accessibility.rs:199-224
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 251
    section: "## Standard Contracts"
    change: "`so a choice names the old and the new option with `selected` in `differs`` now reads: so a single select's choice names the old and the new option with `selected` in `differs`, and a `multiple` select's names each row whose chosen state moved."
    sidecar: "The `differs` reading of a choice is narrowed to a single select, a `multiple` select's naming each row that moved."
    rationale: "Report, Symbols / APIs F6: on a `multiple` select Ctrl or Meta flips one row and Shift spans, so a choice need not name an old and a new option; the sentence stood when a select held one chosen option, and `multiple-select` did not hit it because it names no `multiple`."
    basis: packages/blitz-dom/src/events/pointer.rs:1013-1054
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 260
    section: "## Standard Contracts"
    change: "After `cell-input` in the list of author ids, add: and, since 2026-10-31-engine-debts, `cells-grid` on the grid, focusable and carrying a listener, so it too reads its key; the nine ids of 2026-10-14-stand-requirement-sweep and the 676 data cells stand as true counts."
    sidecar: "Cells' author keys gain `cells-grid`; the 676 data cells stand."
    rationale: "Report, Counts, `cells-grid`: 4 hits, 4 gaining `cells-grid`, `standard-contracts.md:260` among them; `cells-676` holds the 676 data cells at `standard-contracts.md:259` and `:260` as a true count, the 677 being `stand_non_lean_tasks`' reacting elements; Symbols / APIs F13 gives the grid `id`, `tabindex` and `onkeydown`."
    basis: examples/seven_guis/src/tasks/cells.rs:362-393
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 260
    section: "## Standard Contracts"
    change: "New record after this line: on the stand Cells' `cells-grid` takes `id`, `tabindex` 0, `aria_label` Cells and an `onkeydown`, with no role, so it is one Tab stop: arrows move `current`, clamped, into view through a synchronous `MountedData::scroll_to(ScrollBehavior::Instant)`, one `onmounted` per data cell holding the handles; Enter opens the editor on `current`, focused, and its Enter commits, stops propagation and returns focus to the grid; a click inside an open editor returns early and keeps typed text; `.cells-grid:focus .cell.current` draws the accent outline, `2px solid #3a5ce5` (examples/seven_guis/src/tasks/cells.rs:425-433; examples/seven_guis/src/tasks/cells.rs:551-554)."
    sidecar: "New record: `cells-grid`'s Tab stop, keys, editor return and current-cell outline."
    rationale: "Report, Symbols / APIs F13 and Deviations 6a, the task's fact F20; this entity holds Cells' author ids, and `cells-grid` lands at `standard-contracts.md:260` by the `cells-grid` pattern."
    basis: examples/seven_guis/src/tasks/cells.rs:551-554
    fact: F20
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 267
    section: "## Standard Contracts"
    change: "The clause `an option whose parent is a `select` reads its select's client rect as its `bounds` (the private `select_parent`), so a select's options are its children in the snapshot, named by their labels` now reads: an option of a closed select, under an `optgroup` too, reads its select's client rect as its `bounds` (the public `closed_select_of`, with `select_parent` retired for `BaseDocument::select_of`), so a select's options are its children in the snapshot, a group's its group's, named by their labels."
    sidecar: "`select_parent` retired for `select_of`; the option's client-rect rule narrows to a closed select and reaches through a group."
    rationale: "Report, Retired: `snapshot.rs`'s private `select_parent`, for `select_of`; Symbols / APIs F6 and F7 and the Sole-caller facts: `closed_select_of` is public, a `multiple` select's rows read their own rects, and `optgroup` is `Role::Group`; Counts, `multiple-select` and `optgroup` each list `standard-contracts.md:267` changed, and `lint-overlap` holds a closed select's options keeping its client rect."
    basis: packages/dioxus-native-dom/src/snapshot.rs:242-248
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 267
    section: "## Standard Contracts"
    change: "New record after this line: in the snapshot a `multiple` select's rows and groups read their own client rects as their `bounds`, and its `value` reads `select_value` (packages/dioxus-native-dom/src/snapshot.rs:242-252; packages/dioxus-native-dom/src/snapshot.rs:256-257)."
    sidecar: "New record: a `multiple` select's rows and groups in the snapshot."
    rationale: "Report, Symbols / APIs F6, `snapshot.rs`, the task's fact F11; this entity is the select in the bridge and the snapshot."
    basis: packages/dioxus-native-dom/src/snapshot.rs:242-252
    fact: F11
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 267
    section: "## Standard Contracts"
    change: "New record after this line: `multiple_select_values` in `dioxus_document.rs` fills `values` on `Input` and `Change` with one `(name, FormValue::Text(value))` per chosen option of a `multiple` select, so `evt.values()` reads the set, while `value` stays `select_value` (packages/dioxus-native-dom/src/dioxus_document.rs:31-45)."
    sidecar: "New record: a `multiple` select's set reaches the app through `values`."
    rationale: "Report, Symbols / APIs F6, `dioxus_document.rs`, the task's fact F10; this entity holds what a select's `Input | Change` carry to the app."
    basis: packages/dioxus-native-dom/src/dioxus_document.rs:31-45
    fact: F10
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 267
    section: "## Standard Contracts"
    change: "New record after this line: public read-only `BaseDocument::select_values` and `select_of`, public `clear_select_typeahead`, whose one non-test caller is `execute.rs` `focus_multiple_select`, and re-exported `dioxus_native_dom::closed_select_of` and `is_multiple_select`, which exist because escher-driver has no blitz-dom dependency to name `local_name!` with, `is_masked_input` their precedent; `closed_select_of` has two callers, the snapshot's builder and `lint.rs`, and `is_multiple_select` has `closed_select_of` and `execute.rs` `locate` (packages/blitz-dom/src/document.rs:736-744; packages/blitz-dom/src/document.rs:746-750; packages/blitz-dom/src/document.rs:752-755; packages/dioxus-native-dom/src/snapshot.rs:242-252; packages/dioxus-native-dom/src/lib.rs:35-38)."
    sidecar: "New record: the public select readers and their callers, and why the bridge re-exports two of them."
    rationale: "Report, Symbols / APIs F6, F7 and the Sole-caller facts, and Deviations 4, the task's fact F13; the two re-exported readers stand in `snapshot.rs` beside this entity's option rule."
    basis: packages/dioxus-native-dom/src/lib.rs:35-38
    fact: F13
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 298
    section: "## Standard Contracts"
    change: "`ten named blitz-tests targets outside that prefix` now reads `fifteen named blitz-tests targets outside that prefix` (ten until 2026-10-31-engine-debts); the rest of the clause stands."
    sidecar: "`stand-repeat`'s named blitz-tests targets move from ten to fifteen."
    rationale: "Report, Counts: `stand-repeat`'s named targets ten to fifteen, `ten-named` listing `standard-contracts.md:298` changed; Harness / gate surface: `ci-leg.sh` names five more and `test_stand_repeat_leg_runs_three_rounds` names them too."
    basis: .github/scripts/ci-leg.sh:58-59
note: "contract-javascript names no `change` synthesis, so a page's script hearing one `change` lands where `change-event` places it, `established-decisions.md:153` and `standard-contracts.md:36`; F13's `BaseDocument` half also reads against contract-document-core, outside this group's picks; the free-disk step (F23, F24) is occupied-resources', not contract-ci-contracts'."
