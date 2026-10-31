# Apply — a11y-plan · part 1 of 5

## Verdicts
- fanout-a11y-plan--cascade-g1.md 5 · applied · .andromeda/a11y-plan.md:35#1
- fanout-a11y-plan.md 10 · folded into fanout-a11y-plan--cascade-g1.md 5
- fanout-a11y-plan--cascade-g2.md 9 · applied · .andromeda/a11y-plan.md:107#1
- fanout-a11y-plan.md 12 · folded into fanout-a11y-plan--cascade-g2.md 9
- fanout-a11y-plan--cascade-g2.md 10 · applied · .andromeda/a11y-plan.md:113#1
- fanout-a11y-plan.md 2 · folded into fanout-a11y-plan--cascade-g2.md 10
- fanout-a11y-plan--cascade-g3.md 2 · applied · .andromeda/a11y-plan.md:123#1
- fanout-a11y-plan.md 1 · folded into fanout-a11y-plan--cascade-g3.md 2
- fanout-a11y-plan--cascade-g3.md 18 · applied · .andromeda/a11y-plan.md:140#1
- fanout-a11y-plan.md 4 · folded into fanout-a11y-plan--cascade-g3.md 19
- fanout-a11y-plan--cascade-g3.md 19 · applied · .andromeda/a11y-plan.md:142#1
- fanout-a11y-plan--cascade-g4.md 2 · applied · .andromeda/a11y-plan.md:156#1
- fanout-a11y-plan.md 7 · folded into fanout-a11y-plan--cascade-g4.md 2
- fanout-a11y-plan.md 3 · applied · .andromeda/a11y-plan.md:133+#1
- fanout-a11y-plan.md 6 · applied · .andromeda/a11y-plan.md:62#1
- fanout-a11y-plan.md 8 · applied · .andromeda/a11y-plan.md:64#1
- fanout-a11y-plan.md 9 · applied · .andromeda/a11y-plan.md:198+#1
- fanout-a11y-plan.md 11 · applied · .andromeda/a11y-plan.md:53#1
- fanout-a11y-plan.md 13 · applied · .andromeda/a11y-plan.md:55#1, .andromeda/a11y-plan.md:55#2
- fanout-a11y-plan--cascade-g1.md 1 · applied · .andromeda/a11y-plan-detail/a11y-scope-summary.md:19#1
- fanout-a11y-plan--cascade-g1.md 2 · applied · .andromeda/a11y-plan-detail/a11y-scope-summary.md:23#1
- cascade-compare-a11y-plan.md 1 · applied · .andromeda/a11y-plan-detail/a11y-scope-summary.md:18#1
- fanout-a11y-plan--cascade-g1.md 3 · applied · .andromeda/a11y-plan-detail/a11y-strategy.md:20#1
- fanout-a11y-plan--cascade-g1.md 4 · applied · .andromeda/a11y-plan-detail/a11y-strategy.md:21+#1

## Edits
- line: .andromeda/a11y-plan.md:35
  old: «nothing else writes `toggled`, so the tree never states a native checkbox or radio checked — that fix owned by a `.andromeda/residuals.md` line (target: next), the windowed AT witness by one that reads `re-carried:0.2.0`»
  new: «a native checkbox or radio under a checkable role writes `toggled` from its own checked state, reading neither author attribute, and under any other role writes nothing, so `pressed` never begins to read on it — the windowed AT witness owned by a `.andromeda/residuals.md` line that reads `re-carried:0.2.0`»
- line: .andromeda/a11y-plan.md:107
  old: «HTML's focus fixup is not built, so a node focused as it becomes inert keeps focus until focus moves — its fix owned by a `.andromeda/residuals.md` line (target: next).»
  new: «HTML's focus fixup is built, so a node holding focus when it comes under `inert`, a subtree moved under an inert box included, gives focus up as the mutator flushes, no DOM `blur` or `focusout` firing.»
- line: .andromeda/a11y-plan.md:113
  old: «and a click on a `select` focuses it;»
  new: «and a click on a `select` focuses it, a click on a `multiple` select's row focuses the select and acts on the row, and an option of a closed single select takes no click;»
- line: .andromeda/a11y-plan.md:123
  old: «a `multiple` select and an `optgroup`'s options are not modelled, and no popup is painted»
  new: «an `optgroup`'s options are the select's own in tree order, a `multiple` select takes its own keys and row clicks, and no popup is painted»
- line: .andromeda/a11y-plan.md:140
  old: «every stand control lying in one of the two orders.»
  new: «every lean task's control lying in one of the two lean orders; it also walks the load page's and Cells' stated orders (`LOAD_TAB_ORDER`, `CELLS_TAB_ORDER`), in both layout modes.»
- line: .andromeda/a11y-plan.md:142
  old: «OPEN — the pages the stand serves beside the lean four hold no Tab-order, keyboard-activation or contrast row, so SC 2.1.1 and 1.4.3 are unasserted on the load page's `load-fetch` Button, a stand control · owner: the served pages' keyboard and contrast coverage, a `.andromeda/residuals.md` line»
  new: «The load page and Cells hold Tab-order, keyboard and contrast rows (`stand_keyboard`, `stand_contrast`, `stand_load_page`, `stand_cells_served`), so SC 2.1.1 and 1.4.3 are asserted on the load page's `load-fetch` Button and on Cells' `cells-grid`.»
- line: .andromeda/a11y-plan.md:156
  old: «`Some(false)`, so SC 1.4.3 is met»
  new: «`Some(false)`, and the load page's and Cells' ratios in tests of their own (`the_load_page_meets_its_ratios`, `cells_meets_its_ratios`), so SC 1.4.3 is met»
- after: .andromeda/a11y-plan.md:133
  anchor: «- seven_guis CRUD: the person rows are `option`s of a `listbox` by `role` and not focusable; the list `crud-list`, named `People`, is the third CRUD Tab stop and, focused, ArrowDown / ArrowUp / Home / End move the selection as a row's click does; the tree's focus is the list, never the option — the builder reads no `aria-activedescendant` (SC 4.1.2 work outside the tier-0 target), kept unread and pinned by `accessibility_active_descendant_unread`.»
  add: «»
  add: «- seven_guis Cells: `cells-grid`, named `Cells` by `aria_label` with no role, is one Tab stop; the arrows move `current`, clamped, and bring the cell into view; Enter opens the editor on `current`, focused, and the editor's Enter commits and returns focus to the grid; Tab leaving an open editor commits it; `.cells-grid:focus .cell.current` draws a 2px outline in the stand accent; role and cell semantics stay out, owned by a `.andromeda/residuals.md` line.»
- line: .andromeda/a11y-plan.md:62
  old: «reading the focused node back per step (tests/blitz-tests/tests/stand_keyboard.rs:337-365)»
  new: «reading the focused node back per step (tests/blitz-tests/tests/stand_keyboard.rs:337-365), and walks the load page's stated sequence, its button taking Enter and Space, and Cells' stated sequence, its arrows and Enter reaching a cell and its edits committing (tests/blitz-tests/tests/stand_keyboard.rs:157-335)»
- line: .andromeda/a11y-plan.md:64
  old: «hover and active included); it runs in the workspace `test` leg»
  new: «hover and active included), and it asserts the load page's and Cells' ratios in both layout modes (§6; tests/blitz-tests/tests/stand_contrast.rs:647-704); it runs in the workspace `test` leg»
- after: .andromeda/a11y-plan.md:198
  anchor: «- On the stand two kinds of control state a role or a state by attribute alone — the CRUD list `listbox` with `option` rows carrying `aria-selected`, and the flight booker's date inputs `aria-invalid`; the stand states no `aria-pressed`, the flight booker's mode a `select` whose options state its choice by `selected`; one gap stands open: the rows are not focusable and the tree's focus is the list, never the option, the missing `aria-activedescendant` kept unread (§5 Per-surface keyboard shortcuts).»
  add: «»
  add: «- A `multiple` select carries `set_multiselectable` in the tree and its rows read their own client rects in the snapshot, its `value` reading `select_value`, its first chosen option in tree order; an `optgroup` reads `Role::Group`, its non-empty `label` attribute its name, named in the tree and unpainted; what an assistive technology reads of them is not measured.»
- line: .andromeda/a11y-plan.md:53
  old: «— 5 tests, the fourth reading `aria-checked`'s three values on a checkable role alone and four unchecked natives reading neither attribute, the fifth `aria-pressed` on a checkable role reading nothing and a 5,500-character `aria-checked` nowhere in the tree (tests/blitz-tests/tests/accessibility_states.rs:141-444) — as measured at escher-0.1.0/chunks/2026-10-10-snapshot-states/report.md»
  new: «— 6 tests, the fourth reading `aria-checked`'s three values on a checkable role alone and four unchecked natives reading neither attribute, the fifth `aria-pressed` on a checkable role reading nothing and a 5,500-character `aria-checked` nowhere in the tree, the sixth moving a native control by a click and a Space and reading it in the tree (tests/blitz-tests/tests/accessibility_states.rs:141-444; tests/blitz-tests/tests/accessibility_states.rs:386-427) — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md»
- line: .andromeda/a11y-plan.md:55
  old: «`inert_subtree` (4) and»
  new: «`inert_subtree` (6) and»
- line: .andromeda/a11y-plan.md:55
  old: «restored when `inert` is removed; and a focused listbox»
  new: «restored when `inert` is removed; a node made inert losing focus and taking no key, and the tree's focus falling back to the window where the focused node was not built; and a focused listbox»
- line: .andromeda/a11y-plan-detail/a11y-scope-summary.md:19
  old: «neither is a `LeanTask`, so neither carries a Tab-order, keyboard-activation or contrast row — the load page's an open gap (§5) — and their a11y reach is their own checks: keyed, `unkeyed_actionable` empty, the load page's lint 0 (examples/seven_guis/src/stand.rs:131-152; tests/blitz-tests/tests/stand_cells_served.rs:94-136; tests/blitz-tests/tests/stand_load_page.rs:113-127)»
  new: «neither is a `LeanTask`; each holds a stated Tab sequence (`LOAD_TAB_ORDER`, `CELLS_TAB_ORDER`) that `stand_keyboard` walks, with keyboard rows of its own (the load page's `load-fetch` taking Enter and Space, Cells' arrows and Enter on `cells-grid`) and contrast rows in `stand_contrast`, and their own checks read them keyed, `unkeyed_actionable` empty, the load page's lint 0 (examples/seven_guis/src/stand.rs:131-152; tests/blitz-tests/tests/common/mod.rs:93-98; tests/blitz-tests/tests/stand_keyboard.rs:157-335; tests/blitz-tests/tests/stand_contrast.rs:647-704; tests/blitz-tests/tests/stand_cells_served.rs:94-136; tests/blitz-tests/tests/stand_load_page.rs:113-127)»
- line: .andromeda/a11y-plan-detail/a11y-scope-summary.md:23
  old: «the named limits (the focus fixup not built, a `multiple` select and an `optgroup`'s options not modelled, `aria-activedescendant` unread, the native checked state never stated in the tree); its pin `.github/scripts/test_capability_statement.py` reads none of these claims, so a change to the reach is made in that document by hand (as measured at escher-0.2.0/chunks/2026-10-25-capability-statement/report.md)»
  new: «the named limits (the focus fixup firing no `blur`, no popup painted and Enter and Space opening nothing on a single select, a `multiple` select's `size` unread, `aria-activedescendant` unread); its pin `.github/scripts/test_capability_statement.py` reads none of these claims, so a change to the reach is made in that document by hand (as measured at escher-0.2.0/chunks/2026-10-25-capability-statement/report.md, the named limits at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md)»
- line: .andromeda/a11y-plan-detail/a11y-scope-summary.md:18
  old: «so no stand binary builds a platform adapter still — as measured at escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/report.md»
  new: «so no stand binary builds a platform adapter still — as measured at escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/report.md; re-read on `Cargo.lock`'s own move of `pulldown-cmark` to 0.13.5, no crate added or removed: the stand's graph still holds neither `accesskit_xplat` nor `accesskit_winit` (`cargo tree`, `accesskit_(xplat|winit)` 0 rows), so no stand binary builds a platform adapter still — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md»
- line: .andromeda/a11y-plan-detail/a11y-strategy.md:20
  old: «each attribute is read once and nothing else writes `toggled`; the builder writes no `toggled` from a native checkbox's or radio's own checked state, so the tree the shell hands the adapter never states it checked while the snapshot's `checked` reads the element — a measured absence, its fix owned by a `.andromeda/residuals.md` line (target: next);»
  new: «each attribute is read once, and the one other write of `toggled` is a native checkbox's or radio's own checked state on a checkable role, the next bullet;»
- after: .andromeda/a11y-plan-detail/a11y-strategy.md:21
  anchor: « an element whose role does not support an attribute reads it as one whose role does (packages/blitz-dom/src/accessibility.rs:182-236; tests/blitz-tests/tests/accessibility_states.rs:141-158; tests/blitz-tests/tests/accessibility_states.rs:160-178; tests/blitz-tests/tests/accessibility_states.rs:180-199; tests/blitz-tests/tests/accessibility_states.rs:252-444; packages/blitz-dom/src/accessibility.rs:418-438) — ratified by the founder (2026-10-10, by question dialog in the overseer session, relayed verbatim by the overseer as his chosen option label «Ратифицировать», the English wording is the overseer's: the builder writes the three states on the node); what an assistive technology reads of them is not measured — no stand binary builds a platform adapter and the dev host's AT-SPI bus reads inactive — and the windowed witness, at an active assistive technology, is owned by a `.andromeda/residuals.md` line that reads `re-carried:0.2.0`, which this version does not reach, on the founder's F1 «Пока нет, перенести (Рекомендую)» (2026-10-14), the headless half — the tree the shell hands the adapter — proven on the stand since 2026-10-14-stand-a11y-assertions (§7) — as measured at escher-0.1.0/chunks/2026-10-10-snapshot-states/report.md»
  add: «- A native checkbox or radio on a checkable role writes `Toggled::True` or `False` from `checkbox_input_checked()`; under any other role it writes nothing, so `pressed` never begins to read on it, and neither author attribute is read on a native control, `accessibility_states`' `a_click_and_a_space_move_a_native_control_in_the_tree` its pin; what an assistive technology reads of it is not measured (packages/blitz-dom/src/accessibility.rs:199-224; tests/blitz-tests/tests/accessibility_states.rs:386-427) — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md»

## Patterns
[[pattern]]
id = "a11y1-01"
fixed = 'nothing else writes `toggled`'

[[pattern]]
id = "a11y1-02"
fixed = 'never states a native checkbox or radio checked'

[[pattern]]
id = "a11y1-03"
fixed = 'focus fixup is not built'

[[pattern]]
id = "a11y1-04"
fixed = 'keeps focus until focus moves'

[[pattern]]
id = "a11y1-05"
regex = 'options (are )?not modelled'

[[pattern]]
id = "a11y1-06"
fixed = 'every stand control lying in'

[[pattern]]
id = "a11y1-07"
fixed = 'hold no Tab-order, keyboard-activation or contrast row'

[[pattern]]
id = "a11y1-08"
fixed = '5 tests, the fourth reading'

[[pattern]]
id = "a11y1-09"
fixed = '`inert_subtree` (4)'

[[pattern]]
id = "a11y1-10"
fixed = 'neither carries a Tab-order'

[[pattern]]
id = "a11y1-11"
fixed = 'the focus fixup not built'

[[pattern]]
id = "a11y1-12"
fixed = 'native checked state never stated'

[[pattern]]
id = "a11y1-13"
fixed = 'writes no `toggled` from a native'
