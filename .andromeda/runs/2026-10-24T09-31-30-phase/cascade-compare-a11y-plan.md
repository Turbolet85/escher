# Cascade compare — a11y-plan · phase · 2026-10-24-upstream-sync-polish-and-ship
E items 119 · held 45 · lost at classify 2 · lost at strong 9 · cascade only 4 · body 28 · unplaced 0

## Lost — the full read carried these and the cascade did not; the planner takes each, marked «(sampler)»

### a11y-scope-summary#1.10 · lost at classify · .andromeda/a11y-plan-detail/a11y-scope-summary.md:19
- (.andromeda/a11y-plan-detail/a11y-scope-summary.md:19) The browser's `accessibility` feature must stay out of its default set when `9eeebb302` (mimalloc) edits `apps/browser/Cargo.toml`, and the cited manifest lines are re-read where the take moves them.
  > - **Browser app** — the browser's `accessibility` feature (dioxus-native accessibility) is not in its default set (apps/browser/Cargo.toml:14; apps/browser/Cargo.toml:39)

### aria-patterns-roles#1.1 · lost at strong · .andromeda/a11y-plan-detail/aria-patterns-roles.md:8
- (.andromeda/a11y-plan-detail/aria-patterns-roles.md:8) The landmark mapping must read the same on the take's tree, since `06ecec886` reaches `accessibility.rs`; `accessibility_roles` must still pass. Its cited test line ranges move only if the take edits that file.
  > - Landmarks: nav→Navigation, main→Main, aside→Complementary, footer→Footer, article→Article, blockquote→Blockquote, figure→Figure (tests/blitz-tests/tests/accessibility_roles.rs:74-93)
  > - header→Header, section→Section (tests/blitz-tests/tests/accessibility_roles.rs:157-180)

### aria-patterns-roles#2.2 · lost at strong · .andromeda/a11y-plan-detail/aria-patterns-roles.md:16
- (.andromeda/a11y-plan-detail/aria-patterns-roles.md:16) The state reads and the per-role toggle with `role="switch"` → `Switch` make up CARRY 3's engine-debts row and CARRY 5's select-option exception. They must survive the take unchanged, and `accessibility_states` must still pass in both layout modes. Each cited `accessibility.rs` range must be re-read against upstream's `06ecec886` hunks.
  > - **ARIA state attributes** — `aria-selected` → `selected`, `aria-pressed` → `toggled` (a toggle button; `mixed` kept as Mixed), `aria-invalid` → `invalid` (True / Grammar / Spelling; a value outside the list reads True), read by value on any element — a select's option excepted, its `selected` its chosen state (packages/blitz-dom/src/accessibility.rs:171-181) — since 2026-10-10-snapshot-states; `aria-checked` → `toggled` (`true` · `false` · `mixed`) on a `CheckBox` · `Switch` · `RadioButton` · `MenuItemCheckBox` · `MenuItemRadio` role alone, `aria-pressed` read on every other role, a native checkbox or radio reading neither, and `role="switch"` mapping to `Switch` (ratified by the founder, 2026-10-18) — `accessibility_states`' fourth test its pin (packages/blitz-dom/src/accessibility.rs:169-213; tests/blitz-tests/tests/accessibility_states.rs:134-151; tests/blitz-tests/tests/accessibility_states.rs:153-171; tests/blitz-tests/tests/accessibility_states.rs:173-192; tests/blitz-tests/tests/accessibility_states.rs:245-316; packages/blitz-dom/src/accessibility.rs:395-415) — as measured at escher-0.1.0/chunks/2026-10-10-snapshot-states/report.md

### aria-patterns-roles#2.3 · lost at strong · .andromeda/a11y-plan-detail/aria-patterns-roles.md:17
- (.andromeda/a11y-plan-detail/aria-patterns-roles.md:17) The native element-to-role mappings must stay as listed on the merged engine. That covers the link-only-with-`href` rule, the `select` ComboBox/ListBox split (CARRY 5's ground), the `th` scope rule and the input-type mapping. `accessibility_roles` is their pin.
  > - **Button** — button→Button, submit→Button (tests/blitz-tests/tests/accessibility_roles.rs:157-180; tests/blitz-tests/tests/accessibility_roles.rs:128-155)
  > - **Link** — `<a>` is a Link only with `href`, else GenericContainer (packages/blitz-dom/src/accessibility.rs:475-492; tests/blitz-tests/tests/accessibility_roles.rs:117-126)
  > - **Select** — `<select multiple>` is ListBox, else ComboBox (packages/blitz-dom/src/accessibility.rs:475-492; tests/blitz-tests/tests/accessibility_roles.rs:128-155)
  > - **Lists and tables** — ul/ol→List, li→ListItem, table→Table, thead→RowGroup, tr→Row, th→ColumnHeader, th scope=row→RowHeader, td→Cell (tests/blitz-tests/tests/accessibility_roles.rs:95-115); `<th>` is RowHeader for `scope=row|rowgroup`, else ColumnHeader (packages/blitz-dom/src/accessibility.rs:475-492)
  > - **Form input** — `<input>` roles are mapped by `type`, defaulting to TextInput (packages/blitz-dom/src/accessibility.rs:509-527); label→Label, textarea→MultilineTextInput, progress→ProgressIndicator, meter→Meter, radio→RadioButton, range→Slider, email→EmailInput, password→PasswordInput (tests/blitz-tests/tests/accessibility_roles.rs:128-155); text→TextInput, number→NumberInput, checkbox→CheckBox (tests/blitz-tests/tests/accessibility_roles.rs:157-180)
  > - **Generic and text** — div→GenericContainer, h2→Heading, p→Paragraph (tests/blitz-tests/tests/accessibility_roles.rs:157-180)

### keyboard-navigation#1.6 · lost at strong · .andromeda/a11y-plan-detail/keyboard-navigation.md:13
- (.andromeda/a11y-plan-detail/keyboard-navigation.md:13) The file input's inner-button `tabindex="-1"` lives in the conflicted `mutator.rs`, and the resolution must keep it.
  > - The file input's generated inner button gets `tabindex="-1"` (packages/blitz-dom/src/mutator.rs:1430-1444)

### keyboard-navigation#1.7 · lost at strong · .andromeda/a11y-plan-detail/keyboard-navigation.md:14
- (.andromeda/a11y-plan-detail/keyboard-navigation.md:14) Autofocus handling lives in the conflicted `mutator.rs` too. The resolution must keep its "any present value but false" reading, and `autofocus_attribute` must pass.
  > - Under the `autofocus` feature, the latest mounted focussable node whose `autofocus` attribute is present with any value but "false" — bare, empty or "true" — is focused on flush, and `autofocus="false"` is ignored (packages/blitz-dom/src/mutator.rs:1106-1118; packages/blitz-dom/src/mutator.rs:1021-1026; tests/blitz-tests/tests/autofocus_attribute.rs:27-50)

### keyboard-navigation#1.9 · lost at classify · .andromeda/a11y-plan-detail/keyboard-navigation.md:16
- (.andromeda/a11y-plan-detail/keyboard-navigation.md:16) Focus pseudo-class matching is in `stylo.rs`. A take that moves Stylo (`06ecec886`, v0.23-pre) must keep `:focus` / `:focus-within` matching and the `:focus-visible` reading, or record the move.
  > - `:focus` matches the focus element state and `:focus-within` the focused element and its ancestors by the `FOCUS_WITHIN` element state, while `:focus-visible` never matches (packages/blitz-dom/src/stylo.rs:476-480)

### keyboard-navigation#2.1 · lost at strong · .andromeda/a11y-plan-detail/keyboard-navigation.md:23
- (.andromeda/a11y-plan-detail/keyboard-navigation.md:23) Removing the focused node must still reset focus and run blur side effects. Upstream's dropped-node hunks (`f102663b8`, and `stale_node_mapping.rs` with `286874cc1`) touch node teardown, so this reading and `interaction_state_teardown` must hold.
  > - Removing the focused node resets focus to the body (encoded as `None`) and runs blur side effects (packages/blitz-dom/src/document.rs:1004-1006; packages/blitz-dom/src/document.rs:1022-1041); removing a focused text input runs blur side-effects and disables IME (tests/blitz-tests/tests/interaction_state_teardown.rs:189-217)

### keyboard-navigation#3.2 · lost at strong · .andromeda/a11y-plan-detail/keyboard-navigation.md:36
- (.andromeda/a11y-plan-detail/keyboard-navigation.md:36) The range input's key arms in `events/keyboard.rs` must hold on the take: one `input` per change, none for no change.
  > - A focused `input type="range"` with no `disabled` attribute (read by presence) steps up on ArrowRight and ArrowUp, down on ArrowLeft and ArrowDown, goes to its minimum on Home and its maximum on End, and every other key does nothing — since 2026-10-13-range-input-interaction, in every document; its value is a current value held apart from the `value` attribute (`RangeInputData`), sanitized to `[min, max]` on the step grid, and each change marks the changed set and fires one `input` event, a move that changes nothing firing none

### keyboard-navigation#5.8 · lost at strong · .andromeda/a11y-plan-detail/keyboard-navigation.md:75
- (.andromeda/a11y-plan-detail/keyboard-navigation.md:75) The CRUD list's keyboard model is the third CRUD Tab stop and runs through a Dioxus `onkeydown`. Under a Dioxus 0.8 take it must still move the selection as a row's click does.
  > - seven_guis CRUD: since 2026-10-10-snapshot-states the person rows are `option`s of a `listbox` by `role` attribute and are not focusable (examples/seven_guis/src/tasks/crud.rs:58; examples/seven_guis/src/tasks/crud.rs:96-97 — as measured at escher-0.1.0/chunks/2026-10-10-snapshot-states/report.md); since 2026-10-13-stand-keyboard-harness the list `crud-list` takes `tabindex="0"` and an `onkeydown` — the third CRUD Tab stop — and, focused, ArrowDown selects the next visible row (the first when none is selected), ArrowUp the previous, Home the first and End the last, each setting the selection and the two fields as a row's click does, a key at the end of the list or ArrowUp with no row selected changing nothing

### visual-design-verification#8.2 · lost at strong · .andromeda/a11y-plan-detail/visual-design-verification.md:76
- (.andromeda/a11y-plan-detail/visual-design-verification.md:76) The visibility and `pointer-events: none` hit rules sit in the same `node/node.rs` hit path upstream's delta edits: both must hold on the take's tree, and their line cites move with upstream's hunks.
  > - Elements with `visibility: hidden` or `collapse` are never hit-test targets (packages/blitz-dom/src/node/node.rs:1435-1443)
  > - `pointer-events: none` makes an element transparent to hits while its descendants are still tested (packages/blitz-dom/src/node/node.rs:1445-1449; packages/blitz-dom/src/node/node.rs:1647-1655)

## Held

- a11y-scope-summary#1.1
- a11y-scope-summary#1.11
- a11y-scope-summary#1.3
- a11y-scope-summary#1.6
- a11y-scope-summary#1.9
- a11y-strategy#1.1
- a11y-strategy#1.2
- a11y-strategy#2.1
- a11y-strategy#2.3
- a11y-strategy#2.4
- a11y-strategy#4.1
- aria-patterns-roles#2.1
- aria-patterns-roles#2.9
- ci-integration#1.0
- cognitive-accessibility#1.2
- keyboard-navigation#1.1
- keyboard-navigation#1.2
- keyboard-navigation#1.3
- keyboard-navigation#1.5
- keyboard-navigation#2.3
- keyboard-navigation#2.4
- keyboard-navigation#2.7
- keyboard-navigation#3.3
- keyboard-navigation#3.4
- keyboard-navigation#3.8
- keyboard-navigation#4.4
- keyboard-navigation#4.5
- keyboard-navigation#4.9
- keyboard-navigation#5.7
- keyboard-navigation#7.1
- keyboard-navigation#7.2
- keyboard-navigation#7.3
- keyboard-navigation#7.4
- screen-reader-support#1.2
- screen-reader-support#1.3
- screen-reader-support#1.4
- screen-reader-support#2.1
- screen-reader-support#3.1
- screen-reader-support#4.1
- screen-reader-support#5.1
- screen-reader-support#6.2
- screen-reader-support#8.2
- visual-design-verification#1.4
- visual-design-verification#1.5
- visual-design-verification#8.1

## Cascade only

- a11y-strategy#2.2
- aria-patterns-roles#2.5
- screen-reader-support#1.1
- visual-design-verification#1.3
