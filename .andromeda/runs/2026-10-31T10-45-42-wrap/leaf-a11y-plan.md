group: a11y-plan
read: .claude/docs/a11y-summary.md · 45 · «**Full plan:** `.andromeda/a11y-plan.md`. Path-scoped rules:»
read: .claude/rules/a11y.md · 46 · «- 2026-10-06: On the dev host the AT-SPI bus reads `IsEnable»
- leaf: .claude/docs/a11y-summary.md:12   class: stale
    leaf-text: «`inert_subtree` (4)»
    source: .andromeda/a11y-plan.md:55 «`accessibility_merged_states` (3), `inert_subtree` (6) and `accessibility_active_descendant_unread` (1) assert the builder off the built tree in both layout modes»
    replace-with: «`inert_subtree` (6)»
- leaf: .claude/docs/a11y-summary.md:14   class: stale
    leaf-text: «(`the_flight_type_select_moves_its_choice_by_its_arrows`, 15 tests)»
    source: .andromeda/a11y-plan-detail/keyboard-navigation.md:120 «`stand_keyboard` holding 21 tests»
    replace-with: «(`the_flight_type_select_moves_its_choice_by_its_arrows`, 21 tests), and walks the load page's `LOAD_TAB_ORDER` and Cells' `CELLS_TAB_ORDER` forward and back in both layout modes, the load page's button taking Enter and Space, Cells' arrows and Enter opening the moved-to cell's editor and its value committing by keyboard»
- leaf: .claude/docs/a11y-summary.md:14   class: stale
    leaf-text: «`select_interaction` (7 tests:»
    source: .andromeda/a11y-plan-detail/keyboard-navigation.md:119 «`select_interaction` (18 tests)»
    replace-with: «`select_interaction` (18 tests:»
- leaf: .claude/docs/a11y-summary.md:14   class: omission
    leaf-text: «a disabled select taking no key and no click)»
    source: .andromeda/a11y-plan-detail/keyboard-navigation.md:119 «`select_interaction` also drives Enter and Space opening nothing on a single select, a `multiple` select's rows laid out with boxes of their own, a 17-row keys table and a 10-row click table on it»
    replace-with: «a disabled select taking no key and no click, Enter and Space opening nothing on a single select, a `multiple` select's rows laid out with boxes of their own, a 17-row keys table and a 10-row click table on it, its typed text, a disabled one, removing `multiple`, a seeded set, the chosen-row pair's ratio and groups under a single and a `multiple` select)»
- leaf: .claude/docs/a11y-summary.md:14   class: stale
    leaf-text: «`stand_act_select` (5 tests:»
    source: .andromeda/a11y-plan-detail/keyboard-navigation.md:119 «`stand_act_select` (10 tests)»
    replace-with: «`stand_act_select` (10 tests:»
- leaf: .claude/docs/a11y-summary.md:14   class: omission
    leaf-text: «an option chosen by a click then a key)»
    source: .andromeda/a11y-plan-detail/keyboard-navigation.md:119 «`stand_act_select` reads a `multiple` select's rows each with its own bounds, chooses one by a click, spans by `press` with Shift, chooses its match by `type` with no stray click»
    replace-with: «an option chosen by a click then a key, a `multiple` select's rows read each with its own bounds, one chosen by a click, a span by `press` with Shift, its match chosen by `type` with no stray click, and an option under a closed select's group refused `covered`)»
- leaf: .claude/docs/a11y-summary.md:15   class: omission
    leaf-text: «— so SC 1.4.3 is met on every measured non-exempt stand row»
    source: .andromeda/a11y-plan-detail/visual-design-verification.md:14 «`stand_contrast` holds two tests beside its three tables, `the_load_page_meets_its_ratios` and `cells_meets_its_ratios` (five rows), each asserting its rows' required ratios in both layout modes»
    replace-with: «— and two tests beside the tables, `the_load_page_meets_its_ratios` and `cells_meets_its_ratios` (five rows), each asserting its rows' required ratios in both layout modes — so SC 1.4.3 is met on every measured non-exempt stand row, the load page and Cells as well as the lean four»
- leaf: .claude/docs/a11y-summary.md:15   class: omission
    leaf-text: «keeps the same exempt rule (`enabled == Some(false)`), with no second formula or threshold.»
    source: .andromeda/a11y-plan-detail/visual-design-verification.md:18 «The default sheet holds an engine-default chosen-row pair: `select[multiple] > option:checked` and `select[multiple] > optgroup > option:checked` read `#000000` on `#c8c8c8`»
    replace-with: «keeps the same exempt rule (`enabled == Some(false)`), with no second formula or threshold. The default sheet holds an engine-default chosen-row pair for a `multiple` select's rows, `#000000` on `#c8c8c8`, Chromium's inactive selected pair, hardcoded by design as the engine holds no tokens, 12.56:1 measured by `select_interaction` in both layout modes.»
- leaf: .claude/docs/a11y-summary.md:19   class: omission
    leaf-text: «links only with `href`.»
    source: .andromeda/a11y-plan.md:202 «an `optgroup` reads `Role::Group`, its non-empty `label` attribute its name, named in the tree and unpainted; what an assistive technology reads of them is not measured.»
    replace-with: «links only with `href`; a `multiple` select's `ListBox` writes `set_multiselectable`, and an `optgroup` reads `Role::Group`, its non-empty `label` attribute its name, named in the tree and unpainted — what an assistive technology reads of them is not measured.»
- leaf: .claude/docs/a11y-summary.md:20   class: stale
    leaf-text: «an `option` whose parent is a `select` excepted from `display: none`, which every option but a single-choice select's chosen one computes:»
    source: .andromeda/a11y-plan-detail/aria-patterns-roles.md:25 «an `option` of a `select`, its child or under its `optgroup` (`is_select_option`), excepted from `display: none`, which every option of a single-choice select but its chosen one computes»
    replace-with: «an `option` of a `select`, its child or under its `optgroup` (`is_select_option`), excepted from `display: none`, which every option of a single-choice select but its chosen one computes and a `multiple` select's rows do not:»
- leaf: .claude/docs/a11y-summary.md:21   class: stale
    leaf-text: «each attribute is read once and nothing else writes `toggled`, so the builder writes none from a native checkbox's or radio's own checked state — a measured absence, its fix owned by a `.andromeda/residuals.md` line (target: next).»
    source: .andromeda/a11y-plan-detail/a11y-strategy.md:22 «A native checkbox or radio on a checkable role writes `Toggled::True` or `False` from `checkbox_input_checked()`; under any other role it writes nothing, so `pressed` never begins to read on it»
    replace-with: «each attribute is read once, and the one other write of `toggled` is a native checkbox's or radio's own checked state on a checkable role — `Toggled::True` or `False` from `checkbox_input_checked()`; under any other role it writes nothing, so `pressed` never begins to read on it, `accessibility_states`' `a_click_and_a_space_move_a_native_control_in_the_tree` its pin, what an assistive technology reads of it not measured.»
- leaf: .claude/docs/a11y-summary.md:21   class: stale
    leaf-text: «Check: `accessibility_states` (5, both layout modes;»
    source: .andromeda/a11y-plan.md:53 «6 tests, the fourth reading `aria-checked`'s three values on a checkable role alone»
    replace-with: «Check: `accessibility_states` (6, both layout modes;»
- leaf: .claude/docs/a11y-summary.md:21   class: omission
    leaf-text: «the fifth `aria-pressed` on a checkable role reading nothing)»
    source: .andromeda/a11y-plan.md:53 «the sixth moving a native control by a click and a Space and reading it in the tree»
    replace-with: «the fifth `aria-pressed` on a checkable role reading nothing, the sixth moving a native control by a click and a Space and reading it in the tree)»
- leaf: .claude/docs/a11y-summary.md:22   class: omission
    leaf-text: «the list is named `People` by `aria-label`.»
    source: .andromeda/a11y-plan.md:135 «seven_guis Cells: `cells-grid`, named `Cells` by `aria_label` with no role, is one Tab stop;»
    replace-with: «the list is named `People` by `aria-label`. Cells: `cells-grid`, named `Cells` by `aria_label` with no role, is one Tab stop; the arrows move `current`, clamped, and bring the cell into view; Enter opens the editor on `current`, focused, and the editor's Enter commits and returns focus to the grid; Tab leaving an open editor commits it; `.cells-grid:focus .cell.current` draws a 2px outline in the stand accent; role and cell semantics stay out, owned by a `.andromeda/residuals.md` line.»
- leaf: .claude/docs/a11y-summary.md:23   class: omission
    leaf-text: «focus resets to body on removal;»
    source: .andromeda/a11y-plan-detail/screen-reader-support.md:56 «`build_accessibility_tree` writing `focus` only where `focus_node_id` is among the built nodes, else `u64::MAX`, the window, so the tree never names a node it did not build»
    replace-with: «focus resets to body on removal; the tree's focus is the focused node only where it is among the built nodes, else `u64::MAX`, the window, so the tree never names a node it did not build;»
- leaf: .claude/docs/a11y-summary.md:23   class: stale
    leaf-text: «an option is not clicked — a driver `click` on one is refused `covered`)»
    source: .andromeda/a11y-plan-detail/keyboard-navigation.md:27 «A click on a `multiple` select's row runs `click_select_row` behind `handle_click`'s `option` arm, in every document: a disabled select stops it; otherwise it focuses the select»
    replace-with: «an option of a closed single select, under a group too, is not clicked — a driver `click` on one is refused `covered`; a click on a `multiple` select's row runs `click_select_row`, a disabled select stopping it, otherwise focusing the select and clearing the typeahead and the text selection, a plain click choosing the row alone, Ctrl or Meta flipping it, Shift spanning and a disabled row changing nothing)»
- leaf: .claude/docs/a11y-summary.md:23   class: stale
    leaf-text: «— no test asserts `:open`.»
    source: .andromeda/a11y-plan-detail/visual-design-verification.md:59 «`open_state`'s 4 tests, in both layout modes, read the `open` attribute by presence at construction, move it through a mutator write and removal and a summary's click and Enter»
    replace-with: «— `open_state`'s 4 tests, in both layout modes, read the `open` attribute by presence at construction, move it through a mutator write and removal and a summary's click and Enter, and read `:open` matching.»
- leaf: .claude/docs/a11y-summary.md:23   class: omission
    leaf-text: «the Click (`Node::synthetic_click_event_data`) runs the app's handlers (a Dioxus `onclick`), then `handle_click`.»
    source: .andromeda/a11y-plan-detail/keyboard-navigation.md:32 «After `input`, `handle_click`'s checkbox and radio arms dispatch a `DomEventData::Change` with the same value, in every document, a checkbox always and a radio only where it was not checked»
    replace-with: «the Click (`Node::synthetic_click_event_data`) runs the app's handlers (a Dioxus `onclick`), then `handle_click`, whose checkbox and radio arms dispatch, after `input`, a `change` with the same value, in every document, a checkbox always and a radio only where it was not checked before the click (`checkable_change_event`), while typing in a text input and a range's key and pointerdown fire `input` and no `change` (`text_range_no_change_event`).»
- leaf: .claude/docs/a11y-summary.md:23   class: stale
    leaf-text: «a `multiple` select takes no key, an `optgroup`'s options are not modelled and no popup is painted.»
    source: .andromeda/a11y-plan-detail/keyboard-navigation.md:42 «A focused `multiple` select that is no disabled form control takes ArrowDown, ArrowUp, Home and End: alone they choose that row alone, with Shift they span from the anchor»
    replace-with: «an `optgroup`'s options are the select's own in tree order, a disabled group's rows reading as disabled, and no popup is painted. A focused `multiple` select that is no disabled form control takes ArrowDown, ArrowUp, Home and End: alone they choose that row alone, with Shift they span from the anchor (with no cursor yet from the last chosen row), and with Ctrl or Meta they move the current row and change nothing; a Space with no typed text standing flips the current row and with text standing joins it, a character with no Ctrl, Alt or Meta chooses its match alone, its disabled rows are never chosen and Ctrl+A is not modelled; the driver's `type` focuses it with no click and clears the typeahead, a text opening on a space typing nothing, and the driver's `press` holds no Ctrl, so a non-adjacent `multiple` set cannot be built through the driver, a limit its help states, its fix owned by a `.andromeda/residuals.md` line (target: next).»
- leaf: .claude/docs/a11y-summary.md:24   class: stale
    leaf-text: «HTML's focus fixup is not built, so a node that holds focus when it becomes inert keeps it until focus moves, its fix owned by a `.andromeda/residuals.md` line (target: next)»
    source: .andromeda/a11y-plan.md:107 «HTML's focus fixup is built, so a node holding focus when it comes under `inert`, a subtree moved under an inert box included, gives focus up as the mutator flushes, no DOM `blur` or `focusout` firing»
    replace-with: «HTML's focus fixup is built, in every document, so a node holding focus when it comes under `inert`, a subtree moved under an inert box included, gives focus up as the mutator flushes, no DOM `blur` or `focusout` firing»
- leaf: .claude/docs/a11y-summary.md:24   class: stale
    leaf-text: «the chosen option's value, nothing with none chosen — its options its children, kept in the tree as `ListBoxOption` nodes, each `selected` from its chosen state and with its select's client rect as its `bounds`, so the driver's visual lint reads a select's options as overlapping — the stand's flight booker at boot one `overlap` (`flight-type-one-way` × `flight-type-return`), an open defect owned by a `.andromeda/residuals.md` line;»
    source: .andromeda/a11y-plan-detail/visual-design-verification.md:16 «The driver's visual lint leaves a closed select's options out of its overlap reading in every document, the snapshot giving each such `ListBoxOption` its select's client rect»
    replace-with: «the chosen option's value, for a `multiple` select its first chosen option's in tree order, nothing with none chosen — its options, its children and those under an `optgroup`, kept in the tree as `ListBoxOption` nodes, each `selected` from its chosen state, a closed select's options under it with its select's client rect as their `bounds`, which the driver's visual lint leaves out of its overlap reading in every document (`Entry.in_closed_select`, never the role) — the stand's flight booker at boot no `overlap`, `stand_lint` reading `[0, 0, 0, 0]` — while a `multiple` select's rows and groups read their own client rects and its rows are linted as usual;»
- leaf: .claude/docs/a11y-summary.md:25   class: stale
    leaf-text: «the pages served beside the lean four (Cells, the load page) hold no Tab-order, keyboard-activation or contrast row, so SC 2.1.1 and 1.4.3 are unasserted on the load page's `load-fetch` Button, an open gap owned by a `.andromeda/residuals.md` line;»
    source: .andromeda/a11y-plan.md:144 «The load page and Cells hold Tab-order, keyboard and contrast rows (`stand_keyboard`, `stand_contrast`, `stand_load_page`, `stand_cells_served`), so SC 2.1.1 and 1.4.3 are asserted»
    replace-with: «the load page and Cells hold Tab-order, keyboard and contrast rows (`stand_keyboard`, `stand_contrast`, `stand_load_page`, `stand_cells_served`; `LOAD_TAB_ORDER` and `CELLS_TAB_ORDER` in the shared module beside `tab_order` and `ENABLED_TAB_ORDER`), so SC 2.1.1 and 1.4.3 are asserted on the load page's `load-fetch` Button and on Cells' `cells-grid`;»
- leaf: .claude/docs/a11y-summary.md:25   class: stale
    leaf-text: «the elements that react to a click and state no interactive role are counted and pinned as measured (`stand_non_lean_tasks`: 0 on the four lean tasks, Home and Temperature Converter, Circle Drawer 1, Cells 676)»
    source: .andromeda/a11y-plan-detail/screen-reader-support.md:41 «and the elements that react to input and state no interactive role are counted on the stand, in both layout modes, and pinned as measured»
    replace-with: «the elements that react to input and state no interactive role are counted and pinned as measured (`stand_non_lean_tasks`: 0 on the four lean tasks, Home and Temperature Converter, Circle Drawer 1, Cells 677, the 676 data cells and `cells-grid`)»
- leaf: .claude/rules/a11y.md:19   class: stale
    leaf-text: «the focus fixup not built, a `multiple` select and an `optgroup`'s options not modelled, `aria-activedescendant` unread, the native checked state never stated in the tree»
    source: .andromeda/a11y-plan-detail/a11y-scope-summary.md:23 «the named limits (the focus fixup firing no `blur`, no popup painted and Enter and Space opening nothing on a single select, a `multiple` select's `size` unread, `aria-activedescendant` unread)»
    replace-with: «the focus fixup firing no `blur`, no popup painted and Enter and Space opening nothing on a single select, a `multiple` select's `size` unread, `aria-activedescendant` unread»
- leaf: .claude/rules/a11y.md:22   class: stale
    leaf-text: «the focused DOM node is tree focus,»
    source: .andromeda/a11y-plan-detail/screen-reader-support.md:56 «`build_accessibility_tree` writing `focus` only where `focus_node_id` is among the built nodes, else `u64::MAX`, the window, so the tree never names a node it did not build»
    replace-with: «the focused DOM node is tree focus only where it is among the built nodes, else `u64::MAX`, the window, so the tree never names a node it did not build,»
- leaf: .claude/rules/a11y.md:24   class: stale
    leaf-text: «an `option` whose parent is a `select` excepted from `display: none`, which every option but a single-choice select's chosen one computes:»
    source: .andromeda/a11y-plan-detail/aria-patterns-roles.md:25 «an `option` of a `select`, its child or under its `optgroup` (`is_select_option`), excepted from `display: none`, which every option of a single-choice select but its chosen one computes»
    replace-with: «an `option` of a `select`, its child or under its `optgroup` (`is_select_option`), excepted from `display: none`, which every option of a single-choice select but its chosen one computes and a `multiple` select's rows do not:»
- leaf: .claude/rules/a11y.md:25   class: stale
    leaf-text: «each attribute is read once and nothing else writes `toggled`, so the builder writes none from a native checkbox's or radio's own checked state — a measured absence, its fix owned by a `.andromeda/residuals.md` line (target: next).»
    source: .andromeda/a11y-plan-detail/a11y-strategy.md:22 «A native checkbox or radio on a checkable role writes `Toggled::True` or `False` from `checkbox_input_checked()`; under any other role it writes nothing, so `pressed` never begins to read on it»
    replace-with: «each attribute is read once, and the one other write of `toggled` is a native checkbox's or radio's own checked state on a checkable role — `Toggled::True` or `False` from `checkbox_input_checked()`; under any other role it writes nothing, so `pressed` never begins to read on it, `accessibility_states`' `a_click_and_a_space_move_a_native_control_in_the_tree` its pin, what an assistive technology reads of it not measured.»
- leaf: .claude/rules/a11y.md:26   class: stale
    leaf-text: «keeps its `option` children in the tree as `ListBoxOption` nodes,»
    source: .andromeda/a11y-plan-detail/screen-reader-support.md:11 «keeps its `option` children, and those under an `optgroup`, in the tree as `ListBoxOption` nodes»
    replace-with: «keeps its `option` children, and those under an `optgroup`, in the tree as `ListBoxOption` nodes,»
- leaf: .claude/rules/a11y.md:26   class: stale
    leaf-text: «(the chosen option's value, nothing with none chosen) and its options as its children, each with its select's client rect as its `bounds` — so the driver's visual lint reads a select's options as overlapping, the stand's flight booker at boot one `overlap` (`flight-type-one-way` × `flight-type-return`), an open defect owned by a `.andromeda/residuals.md` line.»
    source: .andromeda/a11y-plan-detail/visual-design-verification.md:16 «The driver's visual lint leaves a closed select's options out of its overlap reading in every document, the snapshot giving each such `ListBoxOption` its select's client rect»
    replace-with: «(the chosen option's value, for a `multiple` select its first chosen option's in tree order, nothing with none chosen) and a closed select's options under it, each with its select's client rect as its `bounds` — which the driver's visual lint leaves out of its overlap reading in every document (`overlaps` skipping such an entry on either side, `Entry.in_closed_select` reading the element through `closed_select_of` and never the role, so CRUD's `role="option"` rows are not read as a select's), the stand's flight booker at boot reading no `overlap`, `stand_lint` reading `[0, 0, 0, 0]`. A `multiple` select carries `set_multiselectable` in the tree, its rows and groups reading their own client rects as their `bounds` and its rows linted as usual; an `optgroup` reads `Role::Group`, its non-empty `label` attribute its name, named in the tree and unpainted; what an assistive technology reads of them is not measured.»
- leaf: .claude/rules/a11y.md:32   class: stale
    leaf-text: «HTML's focus fixup is not built, so a node that holds focus when it becomes inert keeps it until focus moves, its fix owned by a `.andromeda/residuals.md` line (target: next)»
    source: .andromeda/a11y-plan-detail/keyboard-navigation.md:9 «`DocumentMutator::flush` calls `BaseDocument::clear_focus` where `focus_node_id` resolves to a node reading `is_inert()`, in every document, with no `inert` arm joining the attribute writes»
    replace-with: «HTML's focus fixup is built, in every document: `DocumentMutator::flush` calls `BaseDocument::clear_focus` where `focus_node_id` resolves to a node reading `is_inert()`, with no `inert` arm joining the attribute writes, so a subtree moved under an inert box is covered too; no DOM `blur` or `focusout` fires, and `disabled` and `hidden` take no part in it»
- leaf: .claude/rules/a11y.md:33   class: omission
    leaf-text: «the Click (`Node::synthetic_click_event_data`) is queued by the event driver, so the app's handlers run, then `handle_click`;»
    source: .andromeda/a11y-plan-detail/keyboard-navigation.md:32 «After `input`, `handle_click`'s checkbox and radio arms dispatch a `DomEventData::Change` with the same value, in every document, a checkbox always and a radio only where it was not checked»
    replace-with: «the Click (`Node::synthetic_click_event_data`) is queued by the event driver, so the app's handlers run, then `handle_click`, whose checkbox and radio arms dispatch, after `input`, a `change` with the same value, in every document, a checkbox always and a radio only where it was not checked before the click, the script runtime synthesizing none (`checkable_change_event`), while typing in a text input and a range's key and pointerdown fire `input` and no `change` (`text_range_no_change_event`);»
- leaf: .claude/rules/a11y.md:33   class: omission
    leaf-text: «removing the focused node resets focus to body and runs blur side effects.»
    source: .andromeda/a11y-plan-detail/keyboard-navigation.md:73 «A `MountedData` call in a task spawned from a key handler that writes a signal panics with `RefCell already borrowed` (`events.rs:162`), the task first polled inside `render_immediate`»
    replace-with: «removing the focused node resets focus to body and runs blur side effects. A `MountedData` call in a task spawned from a key handler that writes a signal panics with `RefCell already borrowed` (`events.rs:162`), the task first polled inside `render_immediate`, measured headlessly and the windowed path read and not run, so Cells calls its handles synchronously, each acting at the call.»
- leaf: .claude/rules/a11y.md:33   class: stale
    leaf-text: «an option is not clicked — a driver `click` on one is refused `covered`)»
    source: .andromeda/a11y-plan-detail/keyboard-navigation.md:27 «A click on a `multiple` select's row runs `click_select_row` behind `handle_click`'s `option` arm, in every document: a disabled select stops it; otherwise it focuses the select»
    replace-with: «an option of a closed single select, under a group too, is not clicked — a driver `click` on one is refused `covered`; a click on a `multiple` select's row runs `click_select_row` behind `handle_click`'s `option` arm, a disabled select stopping it, otherwise focusing the select and clearing the typeahead and the text selection, a plain click choosing the row alone, Ctrl or Meta flipping it, Shift spanning and a disabled row changing nothing)»
- leaf: .claude/rules/a11y.md:33   class: stale
    leaf-text: «a `multiple` select and an `optgroup`'s options are not modelled (a `multiple` select takes no key) and no popup is painted.»
    source: .andromeda/a11y-plan-detail/keyboard-navigation.md:42 «A focused `multiple` select that is no disabled form control takes ArrowDown, ArrowUp, Home and End: alone they choose that row alone, with Shift they span from the anchor»
    replace-with: «an `optgroup`'s options are the select's own in tree order, reached by keys and text, a disabled group's rows reading as disabled through `option_disabled`, and no popup is painted. A focused `multiple` select that is no disabled form control takes ArrowDown, ArrowUp, Home and End: alone they choose that row alone, with Shift they span from the anchor (with no cursor yet from the last chosen row), and with Ctrl or Meta they move the current row and change nothing; a Space with no typed text standing flips the current row and with text standing joins it, a character with no Ctrl, Alt or Meta chooses its match alone, its disabled rows are never chosen and Ctrl+A is not modelled; a `multiple` write or removal records the select (`record_select_of_multiple_write`), so removal settles it to one, and `set_option_selectedness` leaves its other options alone; `evt.values()` reads its set through `multiple_select_values`, one form entry per chosen option on `Input` and `Change`, while `value` stays `select_value`; the driver's `type` on one focuses it with no click and clears the typeahead, a text opening on a space typing nothing, and the driver's `press` holds no Ctrl, so a non-adjacent `multiple` set cannot be built through the driver, a limit its help states, its fix owned by a `.andromeda/residuals.md` line (target: next).»
- leaf: .claude/rules/a11y.md:34   class: stale
    leaf-text: «— no test asserts `:open`.»
    source: .andromeda/a11y-plan-detail/visual-design-verification.md:59 «`open_state`'s 4 tests, in both layout modes, read the `open` attribute by presence at construction, move it through a mutator write and removal and a summary's click and Enter»
    replace-with: «— `open_state`'s 4 tests, in both layout modes, read the `open` attribute by presence at construction, move it through a mutator write and removal and a summary's click and Enter, and read `:open` matching.»
- leaf: .claude/rules/a11y.md:35   class: stale
    leaf-text: «(`the_flight_type_select_moves_its_choice_by_its_arrows`, 15 tests)»
    source: .andromeda/a11y-plan-detail/keyboard-navigation.md:120 «`stand_keyboard` holding 21 tests»
    replace-with: «(`the_flight_type_select_moves_its_choice_by_its_arrows`, 21 tests)»
- leaf: .claude/rules/a11y.md:35   class: stale
    leaf-text: «while the pages the stand serves beside the lean four (Cells, the load page) hold no Tab-order, keyboard-activation or contrast row — SC 2.1.1 and 1.4.3 unasserted on the load page's `load-fetch` Button, an open gap owned by a `.andromeda/residuals.md` line;»
    source: .andromeda/a11y-plan.md:144 «The load page and Cells hold Tab-order, keyboard and contrast rows (`stand_keyboard`, `stand_contrast`, `stand_load_page`, `stand_cells_served`), so SC 2.1.1 and 1.4.3 are asserted»
    replace-with: «while the load page and Cells hold Tab-order, keyboard and contrast rows (`stand_keyboard`, `stand_contrast`, `stand_load_page`, `stand_cells_served`) — `stand_keyboard` walks the shared tables `LOAD_TAB_ORDER` and `CELLS_TAB_ORDER`, beside `tab_order` and `ENABLED_TAB_ORDER`, forward and back in both layout modes, the load page's button taking Enter and Space, Cells' arrows and Enter opening the moved-to cell's editor and its value committing by keyboard, Tab leaving an open editor and a click on another cell each committing it, and the arrows clamping at the grid's ends and bringing a cell into view — so SC 2.1.1 and 1.4.3 are asserted on the load page's `load-fetch` Button and on Cells' `cells-grid`;»
- leaf: .claude/rules/a11y.md:35   class: stale
    leaf-text: «`select_interaction` (7 tests:»
    source: .andromeda/a11y-plan-detail/keyboard-navigation.md:119 «`select_interaction` (18 tests)»
    replace-with: «`select_interaction` (18 tests:»
- leaf: .claude/rules/a11y.md:35   class: omission
    leaf-text: «an app writing `selected` re-seeding the choice)»
    source: .andromeda/a11y-plan-detail/keyboard-navigation.md:119 «`select_interaction` also drives Enter and Space opening nothing on a single select, a `multiple` select's rows laid out with boxes of their own, a 17-row keys table and a 10-row click table on it»
    replace-with: «an app writing `selected` re-seeding the choice, Enter and Space opening nothing on a single select, a `multiple` select's rows laid out with boxes of their own, a 17-row keys table and a 10-row click table on it, its typed text, a disabled one, removing `multiple`, a seeded set, the chosen-row pair's ratio and groups under a single and a `multiple` select)»
- leaf: .claude/rules/a11y.md:35   class: stale
    leaf-text: «`stand_act_select` (5 tests:»
    source: .andromeda/a11y-plan-detail/keyboard-navigation.md:119 «`stand_act_select` (10 tests)»
    replace-with: «`stand_act_select` (10 tests:»
- leaf: .claude/rules/a11y.md:35   class: omission
    leaf-text: «an option chosen by a click then a key)»
    source: .andromeda/a11y-plan-detail/keyboard-navigation.md:119 «`stand_act_select` reads a `multiple` select's rows each with its own bounds, chooses one by a click, spans by `press` with Shift, chooses its match by `type` with no stray click»
    replace-with: «an option chosen by a click then a key, a `multiple` select's rows read each with its own bounds, one chosen by a click, a span by `press` with Shift, its match chosen by `type` with no stray click, and an option under a closed select's group refused `covered`)»
- leaf: .claude/rules/a11y.md:35   class: omission
    leaf-text: «`stand_act_select` by its `stand_` prefix and `select_interaction` by the leg's named `--test` list.»
    source: .andromeda/a11y-plan.md:135 «seven_guis Cells: `cells-grid`, named `Cells` by `aria_label` with no role, is one Tab stop;»
    replace-with: «`stand_act_select` by its `stand_` prefix and `select_interaction` by the leg's named `--test` list. Cells: `cells-grid`, named `Cells` by `aria_label` with no role, is one Tab stop; the arrows move `current`, clamped, and bring the cell into view; Enter opens the editor on `current`, focused, and the editor's Enter commits and returns focus to the grid; Tab leaving an open editor commits it; `.cells-grid:focus .cell.current` draws a 2px outline in the stand accent; role and cell semantics stay out, owned by a `.andromeda/residuals.md` line.»
- leaf: .claude/rules/a11y.md:38   class: omission
    leaf-text: «— so SC 1.4.3 is met on every measured non-exempt stand row:»
    source: .andromeda/a11y-plan-detail/visual-design-verification.md:14 «`stand_contrast` holds two tests beside its three tables, `the_load_page_meets_its_ratios` and `cells_meets_its_ratios` (five rows), each asserting its rows' required ratios in both layout modes»
    replace-with: «— and two tests beside the tables, `the_load_page_meets_its_ratios` and `cells_meets_its_ratios` (five rows), each asserting its rows' required ratios in both layout modes, the load page's colours read through the shared `read`, no hex copied — so SC 1.4.3 is met on every measured non-exempt stand row, the load page and Cells as well as the lean four:»
- leaf: .claude/rules/a11y.md:38   class: omission
    leaf-text: «so a change to `text_contrast` or that exempt rule moves both.»
    source: .andromeda/a11y-plan-detail/visual-design-verification.md:18 «The default sheet holds an engine-default chosen-row pair: `select[multiple] > option:checked` and `select[multiple] > optgroup > option:checked` read `#000000` on `#c8c8c8`»
    replace-with: «so a change to `text_contrast` or that exempt rule moves both. The default sheet holds an engine-default chosen-row pair: `select[multiple] > option:checked` and `select[multiple] > optgroup > option:checked` read `#000000` on `#c8c8c8`, Chromium's inactive selected pair, hardcoded by design as the engine holds no tokens, 12.56:1 measured by `select_interaction` in both layout modes.»
- leaf: .claude/rules/a11y.md:42   class: stale
    leaf-text: «runs fourteen files (85 tests)»
    source: .andromeda/a11y-plan-detail/ci-integration.md:10 «14 files and 112 tests (`stand_keyboard` 21)»
    replace-with: «runs fourteen files (112 tests)»
- leaf: .claude/rules/a11y.md:42   class: stale
    leaf-text: «`accessibility_states` (5 tests:»
    source: .andromeda/a11y-plan.md:53 «6 tests, the fourth reading `aria-checked`'s three values on a checkable role alone»
    replace-with: «`accessibility_states` (6 tests:»
- leaf: .claude/rules/a11y.md:42   class: omission
    leaf-text: «the fifth `aria-pressed` on a checkable role reading nothing)»
    source: .andromeda/a11y-plan.md:53 «the sixth moving a native control by a click and a Space and reading it in the tree»
    replace-with: «the fifth `aria-pressed` on a checkable role reading nothing, the sixth moving a native control by a click and a Space and reading it in the tree)»
- leaf: .claude/rules/a11y.md:42   class: stale
    leaf-text: «`inert_subtree` (4)»
    source: .andromeda/a11y-plan.md:55 «`accessibility_merged_states` (3), `inert_subtree` (6) and `accessibility_active_descendant_unread` (1) assert the builder off the built tree in both layout modes»
    replace-with: «`inert_subtree` (6)»
