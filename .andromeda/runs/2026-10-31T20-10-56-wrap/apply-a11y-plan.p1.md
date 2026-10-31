# Apply — a11y-plan · part 1 of 4

## Verdicts
- fanout-a11y-plan.md 1 · applied · .andromeda/a11y-plan.md:123#1, .andromeda/a11y-plan.md:123#2, .andromeda/a11y-plan.md:123#3
- fanout-a11y-plan--cascade-g2.md 2 · folded into fanout-a11y-plan.md 1
- fanout-a11y-plan--cascade-g3.md 2 · folded into fanout-a11y-plan.md 1
- fanout-a11y-plan.md 2 · applied · .andromeda/a11y-plan.md:202#1
- fanout-a11y-plan--cascade-g5.md 5 · folded into fanout-a11y-plan.md 2
- fanout-a11y-plan.md 3 · applied · .andromeda/a11y-plan.md:113#1
- fanout-a11y-plan--cascade-g2.md 4 · folded into fanout-a11y-plan.md 3
- fanout-a11y-plan--cascade-g3.md 1 · folded into fanout-a11y-plan.md 3
- fanout-a11y-plan.md 4 · applied · .andromeda/a11y-plan.md:144+#1
- fanout-a11y-plan.md 5 · applied · .andromeda/a11y-plan.md:160+#1, .andromeda/a11y-plan.md:167+#1
- fanout-a11y-plan.md 6 · applied · .andromeda/a11y-plan.md:203+#1
- fanout-a11y-plan.md 7 · applied · .andromeda/a11y-plan.md:29#1
- fanout-a11y-plan--cascade-g1.md 4 · folded into fanout-a11y-plan.md 7
- fanout-a11y-plan.md 8 · applied · .andromeda/a11y-plan.md:55#1
- fanout-a11y-plan.md 9 · applied · .andromeda/a11y-plan.md:37#1
- fanout-a11y-plan.md 10 · applied · .andromeda/a11y-plan.md:64#1
- fanout-a11y-plan--cascade-g1.md 1 · applied · .andromeda/a11y-plan-detail/a11y-scope-summary.md:10+#1
- fanout-a11y-plan--cascade-g1.md 2 · applied · .andromeda/a11y-plan-detail/a11y-scope-summary.md:23#1, .andromeda/a11y-plan-detail/a11y-scope-summary.md:23#2
- fanout-a11y-plan--cascade-g1.md 3 · applied · .andromeda/a11y-plan-detail/a11y-strategy.md:10#1
- fanout-a11y-plan--cascade-g1.md 5 · applied · .andromeda/a11y-plan-detail/a11y-strategy.md:23#1
- fanout-a11y-plan--cascade-g1.md 6 · applied · .andromeda/a11y-plan-detail/aria-patterns-roles.md:19+#1
- fanout-a11y-plan--cascade-g1.md 7 · applied · .andromeda/a11y-plan-detail/aria-patterns-roles.md:21#1
- fanout-a11y-plan--cascade-g1.md 8 · applied · .andromeda/a11y-plan-detail/aria-patterns-roles.md:21+#1

## Edits
- line: .andromeda/a11y-plan.md:123
  old: «that is no disabled form control takes ArrowDown»
  new: «that is no disabled form control, its list closed, takes ArrowDown»
- line: .andromeda/a11y-plan.md:123
  old: «document's clock; every other key does nothing;»
  new: «document's clock; Enter, and a Space with no typed text standing, open its list on the chosen option and fire nothing, a disabled, inert, hidden or empty select opening nothing; every other key does nothing;»
- line: .andromeda/a11y-plan.md:123
  old: «a key that changes nothing none; an `optgroup`'s options are the select's own in tree order, a `multiple` select takes its own keys and row clicks, and no popup is painted.»
  new: «a key that changes nothing none; with the list open the engine paints it over the page and focus stays on the select, its keys moving the highlight over enabled rows and choosing nothing, Enter and Space choosing the highlighted row and closing, Tab in either direction choosing and closing before focus moves, and Escape closing with the value unchanged and no event, as a second click, a press outside and a page scroll do; a close clears the typed text; an `optgroup`'s options are the select's own in tree order, and a `multiple` select takes its own keys and row clicks.»
- line: .andromeda/a11y-plan.md:202
  old: «named in the tree and unpainted»
  new: «named in the tree and painted as a header of an open list, heavier than a row by weight on every build, a header choosing nothing»
- line: .andromeda/a11y-plan.md:113
  old: «a click on a `select` focuses it,»
  new: «a click on a single `select` focuses it and opens its list or closes one standing, each of two clicks on one point focusing; a click on a row of an open list chooses it, a disabled row changing nothing and leaving the list open and a group header choosing nothing; a press outside the list closes it with no event; a label's click on a bound single select focuses it and opens nothing; a click on a `multiple` select focuses it,»
- after: .andromeda/a11y-plan.md:144
  anchor: «- The load page and Cells hold Tab-order, keyboard and contrast rows (`stand_keyboard`, `stand_contrast`, `stand_load_page`, `stand_cells_served`), so SC 2.1.1 and 1.4.3 are asserted on the load page's `load-fetch` Button and on Cells' `cells-grid`.»
  add: «»
  add: «- The select list's keyboard and focus coverage is headless: `select_popup`, `stand_act_popup` and `stand_keyboard`'s list walks assert SC 2.1.1 and 2.4.3 on it in both layout modes, the flight type walk holding with its list open and after each close and a click leaving focus on the select with the walk going on from it; `stand_keyboard` runs in the `a11y` leg (§9), `select_popup` and `stand_act_popup` outside it, in the `stand-repeat` leg; the windowed list is read and not run.»
- after: .andromeda/a11y-plan.md:160
  anchor: «- escher-driver's visual lint (`Session::lint`, the `lint` verb on every carrier) reads `contrast` through `Harness::text_contrast` with `stand_contrast`'s exempt rule and no second formula or threshold, a finding written as ids, closed words and numbers only.»
  add: «»
  add: «- An open select list's colours are engine constants on `SelectListView`, in no sheet — `HIGHLIGHT_TEXT`, `HIGHLIGHT_BACKGROUND`, `DISABLED_TEXT` and `BORDER`, their values in design-system §Color Palette; `select_popup_painted` reads the highlight and a disabled row as painted against those constants and measures the plain pair at the default sheet and at the stand, so SC 1.4.3 is asserted on the list in the harness; a highlighted row's contrast is unread by the lint and a closed select's own pair is unread.»
- after: .andromeda/a11y-plan.md:167
  anchor: «»
  next: «- Observed absent — prefers-reduced-motion and prefers-color-scheme rules in the searched s02 files.»
  add: «- A select list opens and closes with no motion (design-system §Motion).»
  add: «»
- after: .andromeda/a11y-plan.md:203
  anchor: «»
  next: «**Landmark roles:** the engine's landmark mapping is recorded in §4.»
  add: «- A single select reads `ComboBox` and writes `expanded` from its list's open state, reading no `aria-expanded`, and an author's `open` or `aria-expanded` on it steers nothing; the tree's focus stays on the select and its chosen option reads `selected`; the highlighted row is in no snapshot and no tree; the snapshot holds no `expanded` key, an open list's row reading its own `bounds` in the list; what an assistive technology reads of the list is not measured.»
  add: «»
- line: .andromeda/a11y-plan.md:29
  old: «a text control's input and a select's changed choice mark it, node creation, hover, active, scroll and layout do not, so a pointer move»
  new: «a text control's input, a select's changed choice and a single select's list opening or closing mark it — a page scroll or a move that closes a list marks its select through `reconcile_select_list` at the next resolve, a wheel elsewhere through `close_select_list`; node creation, hover, active, scroll and layout do not, save that scroll or move, and a highlight or a wheel move inside a list marks nothing, so a pointer move»
- line: .andromeda/a11y-plan.md:55
  old: «the merged `aria-expanded` (true and false and nothing else)»
  new: «the merged `aria-expanded` (true and false and nothing else, on every element but a single select, which writes its list's open state and reads none, `select_popup` asserting that an author's attribute steers nothing)»
- line: .andromeda/a11y-plan.md:37
  old: «`aria-description`, `aria-expanded`, `aria-disabled`»
  new: «`aria-description`, `aria-expanded` (on every element but a single select, which writes its list's open state instead), `aria-disabled`»
- line: .andromeda/a11y-plan.md:64
  old: «18 `STATE_ROWS`»
  new: «20 `STATE_ROWS`»
- after: .andromeda/a11y-plan-detail/a11y-scope-summary.md:10
  anchor: «- **blitz-dom accessibility tree** — under the `accessibility` feature, `BaseDocument::build_accessibility_tree` builds an AccessKit tree from the DOM (packages/blitz-dom/src/lib.rs:86-87; packages/blitz-dom/src/accessibility.rs:10-124); blitz-dom's default features include `accessibility`, which enables `accesskit`, and `custom-widget` also requires it (packages/blitz-dom/Cargo.toml:14-23; packages/blitz-dom/Cargo.toml:28)»
  add: «- **Select list** — an engine overlay on every document, remote pages included, that a closed single select opens: one `BaseDocument` field `select_list`, answered first by `hit_with_scrollbar` and painted by `paint_scene` after the root's render, with SC 2.1.1, 2.4.3 and 1.4.3 asserted in the harness by `select_popup`, `select_popup_painted`, `stand_act_popup` and `stand_keyboard`'s list walks; what an assistive technology reads of it is not measured and the windowed list is read and not run (packages/blitz-dom/src/node/select_list.rs:28-46; packages/blitz-dom/src/document.rs:343-344; packages/blitz-dom/src/document.rs:2404-2406; packages/blitz-paint/src/render.rs:246-247) — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md»
- line: .andromeda/a11y-plan-detail/a11y-scope-summary.md:23
  old: «no popup painted and Enter and Space opening nothing on a single select, a `multiple` select's `size` unread»
  new: «a single select's `size` unread»
- line: .andromeda/a11y-plan-detail/a11y-scope-summary.md:23
  old: «the named limits at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md»
  new: «the named limits at escher-0.3.0/chunks/2026-10-31-select-popup/report.md»
- line: .andromeda/a11y-plan-detail/a11y-strategy.md:10
  old: «a text control's input and a select's changed choice mark it — `apply_select_choice` marking nothing for a choice of the chosen option (packages/blitz-dom/src/events/keyboard.rs:470-496); node creation, hover, active, scroll and layout do not; `has_changes()`»
  new: «a text control's input, a select's changed choice and a single select's list opening or closing mark it — `apply_select_choice` marking nothing for a choice of the chosen option (packages/blitz-dom/src/events/keyboard.rs:470-496); node creation, hover, active, scroll and layout do not, with no list open; where a list stands, a page scroll or a move that closes it marks its select through `reconcile_select_list` at the next resolve and a wheel elsewhere marks it through `close_select_list`, while a highlight or a wheel move inside the list marks nothing (packages/blitz-dom/src/document.rs:881-907 — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md); `has_changes()`»
- line: .andromeda/a11y-plan-detail/a11y-strategy.md:23
  old: «`aria-expanded` (a boolean);»
  new: «`aria-expanded` (a boolean, on every element but a single select, which reads none and writes `expanded` from its list's open state);»
- after: .andromeda/a11y-plan-detail/aria-patterns-roles.md:19
  anchor: «- **Select** — `<select multiple>` is ListBox, else ComboBox (packages/blitz-dom/src/accessibility.rs:502-519; tests/blitz-tests/tests/accessibility_roles.rs:128-155)»
  add: «- **Select, single** — a single select's pattern is `ComboBox` with `expanded` written from its list's open state, a closed bool of engine state with no `aria-expanded` read, and the chosen option `selected` (packages/blitz-dom/src/accessibility.rs:238-242; tests/blitz-tests/tests/select_popup.rs:565-614) — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md»
- line: .andromeda/a11y-plan-detail/aria-patterns-roles.md:21
  old: «a group's label is named in the tree and unpainted (packages/blitz-dom/src/accessibility.rs) — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md»
  new: «a group's label is named in the tree and painted as a header on an open list (packages/blitz-dom/src/accessibility.rs) — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md»
- after: .andromeda/a11y-plan-detail/aria-patterns-roles.md:21
  anchor: «- **Option group** — `optgroup` → `Role::Group`, its non-empty `label` attribute its name; a group's label is named in the tree and unpainted (packages/blitz-dom/src/accessibility.rs) — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md»
  add: «- **Option group's header row** — in an open list a group's header row chooses nothing, its options stand under it, and a group with no label reads the whole list's box (packages/blitz-dom/src/document.rs:995-1012; tests/blitz-tests/tests/select_popup.rs:805-855) — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md»

## Patterns
[[pattern]]
id = "a11y1-01"
fixed = 'no popup is painted'

[[pattern]]
id = "a11y1-02"
fixed = 'named in the tree and unpainted'

[[pattern]]
id = "a11y1-03"
fixed = 'a click on a `select` focuses it'

[[pattern]]
id = "a11y1-04"
fixed = 'hover, active, scroll and layout do not'

[[pattern]]
id = "a11y1-05"
fixed = 'the merged `aria-expanded` (true and false and nothing else)'

[[pattern]]
id = "a11y1-06"
fixed = '`aria-description`, `aria-expanded`, `aria-disabled`'

[[pattern]]
id = "a11y1-07"
fixed = '18 `STATE_ROWS`'

[[pattern]]
id = "a11y1-08"
fixed = 'no popup painted and Enter and Space opening nothing'

[[pattern]]
id = "a11y1-09"
fixed = "a `multiple` select's `size` unread"

[[pattern]]
id = "a11y1-10"
fixed = '`aria-expanded` (a boolean)'
