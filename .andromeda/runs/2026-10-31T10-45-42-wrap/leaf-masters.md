group: masters
read: CLAUDE.md · 151 · «<!-- USER:session-learnings end -->»
- leaf: CLAUDE.md:22   class: omission
  leaf-text: «passes through an element carrying `inert` and skips a node dropped since the last resolve;»
  source: .andromeda/architecture.md:143 «skips a node dropped since the last resolve, and answers nothing for the whole subtree of a transform whose determinant is 0 or not finite»
  replace-with: «passes through an element carrying `inert`, skips a node dropped since the last resolve and answers nothing for the whole subtree of a transform whose determinant is 0 or not finite;»
- leaf: CLAUDE.md:22   class: omission
  leaf-text: «`visible_region` reads what an element can be seen through»
  source: .andromeda/architecture.md:145 «`visible_region` reads each clipping box through its transforms, exact under a translate and a scale and under a rotation the rotated padding box's bounding box»
  replace-with: «`visible_region` reads what an element can be seen through, each clipping box through its transforms — exact under a translate and a scale, under a rotation the rotated padding box's bounding box, `None` for a mapped box with a non-finite edge»
- leaf: CLAUDE.md:22   class: omission
  leaf-text: «a native checkbox or radio reading neither (ratified by the founder, 2026-10-18);»
  source: .andromeda/a11y-plan.md:35 «a native checkbox or radio under a checkable role writes `toggled` from its own checked state, reading neither author attribute, and under any other role writes nothing»
  replace-with: «a native checkbox or radio reading neither (ratified by the founder, 2026-10-18) and writing `toggled` from its own checked state under a checkable role, nothing under any other;»
- leaf: CLAUDE.md:22   class: omission
  leaf-text: «an inert subtree left out of the tree;»
  source: .andromeda/architecture-detail/standard-contracts.md:31 «`build_accessibility_tree` writes `focus` only where `focus_node_id` is among the built nodes, else `u64::MAX`, the window, so the tree never names a node it did not build»
  replace-with: «an inert subtree left out of the tree; the tree's `focus` written only where the focused node was built, else `u64::MAX`, the window; an `optgroup` `Role::Group`, its non-empty `label` its name, and a `multiple` select `set_multiselectable`;»
- leaf: CLAUDE.md:22   class: retired
  leaf-text: «no popup painted, an option not clicked, a `multiple` select and an `optgroup`'s options not modelled — a direct engine change for every document)»
  source: .andromeda/architecture.md:62 «a `multiple` select and an `optgroup`'s options are modelled in the keys, pointer, form, tree, sheet, snapshot and bridge; no popup is painted, Enter and Space open nothing on a single select»
  replace-with: «an `optgroup`'s options its select's own in tree order (`option_disabled` the one rule, its own attribute or its group's), no popup painted, Enter and Space opening nothing on a single select, an option of a closed single select, under a group too, taking no click, `size` unread — a direct engine change for every document), a `multiple` select's model (the chosen set held as its options' `CHECKED` state, every change through `apply_select_choice`; `press_multiple_select_key`: `ArrowDown` · `ArrowUp` · `Home` · `End` alone choose that row alone, with Shift span from the anchor, with Ctrl or Meta move the current row and change nothing, a Space flips the current row with no typed text standing and joins the text otherwise, a character with no Ctrl, Alt or Meta chooses its match alone; `click_select_row`, behind `handle_click`'s `option` arm: a plain click chooses the row alone, Ctrl or Meta flips it, Shift spans, a disabled row changes nothing; disabled rows never chosen, a removal of `multiple` settling the set to one option, Ctrl+A not modelled — a direct engine change for every document)»
- leaf: CLAUDE.md:22   class: omission
  leaf-text: «`BaseDocument::select_value` its public reader;»
  source: .andromeda/architecture.md:135 «a `multiple` select's first chosen option in tree order, `select_values` its chosen set and `select_of` an option's or group's select, each public and read-only»
  replace-with: «`BaseDocument::select_value` (a `multiple` select's first chosen option in tree order), `select_values` (its chosen set) and `select_of` (an option's or group's select) its public readers, public `clear_select_typeahead` clearing the typed text, its one non-test caller the driver's `focus_multiple_select`;»
- leaf: CLAUDE.md:22   class: omission
  leaf-text: «three direct engine changes for every document, ratified by the founder, 2026-10-13), and the changed set»
  source: .andromeda/architecture.md:141 «the engine dispatches `change` after `input` on a select's choice and on a checkbox's or radio's click, a radio only where it was not checked before, and a page's script hears that one `change`»
  replace-with: «three direct engine changes for every document, ratified by the founder, 2026-10-13), the engine's `change` on a checkbox's or radio's click (after `input`, a radio only where it was not checked before, a page's script hearing that one `change`, a text or range input's `input` bringing none — a direct engine change for every document), and the changed set»
- leaf: CLAUDE.md:33   class: stale
  leaf-text: «a select's its chosen option's value (`BaseDocument::select_value`, no mask applying), its options read as its children with its client rect as their `bounds`, named by their labels»
  source: .andromeda/architecture-detail/standard-contracts.md:279 «an option of a closed select, under an `optgroup` too, reads its select's client rect as its `bounds` (the public `closed_select_of` and `BaseDocument::select_of`)»
  replace-with: «a select's its chosen option's value, a `multiple` select's first chosen option in tree order (`BaseDocument::select_value`, no mask applying), a closed select's options, under an `optgroup` too, read as its children with its client rect as their `bounds`, a `multiple` select's rows and groups with client rects of their own, named by their labels»
- leaf: CLAUDE.md:33   class: omission
  leaf-text: «and lists the actionable elements that read no author key (`DioxusDocument::unkeyed_actionable`)»
  source: .andromeda/architecture.md:183 «the public `closed_select_of` and `is_multiple_select` are the select tests the driver reads, naming the element and never the role»
  replace-with: «and lists the actionable elements that read no author key (`DioxusDocument::unkeyed_actionable`); its public `closed_select_of` and `is_multiple_select` are the select tests the driver reads, naming the element and never the role, and `evt.values()` reads a `multiple` select's set (`multiple_select_values`, one `(name, FormValue::Text(value))` per chosen option on `Input` and `Change`), `value` staying `select_value`»
- leaf: CLAUDE.md:37   class: omission
  leaf-text: «(an empty text clears it; on a select the text chooses the first option whose label starts with it)»
  source: .andromeda/architecture-detail/standard-contracts.md:190 «`type` does not click such a target, a click landing on the row at the centre: the crate-private `focus_multiple_select` focuses it with no click and clears the typeahead»
  replace-with: «(an empty text clears it; on a select the text chooses the first option whose label starts with it, on a `multiple` select its match alone, focused with no click, a text opening on a space typing nothing; `press` carries no Ctrl, so a set of rows that are not adjacent cannot be built through the driver, its help saying so)»
- leaf: CLAUDE.md:37   class: stale
  leaf-text: «`off-screen`'s meaning: a scrolling box under a transform is read where it stands untransformed, so an element inside it can read `off-screen` though it shows;»
  source: .andromeda/architecture-detail/standard-contracts.md:177 «under a rotation a box's visible part reads as the bounding box of its rotated padding box, so an element in a corner it does not show can read in view»
  replace-with: «`off-screen`'s meaning: under a rotation a box's visible part reads as the bounding box of its rotated padding box, so an element in a corner it does not show can read in view;»
- leaf: CLAUDE.md:38   class: omission
  leaf-text: «moving the selection and filling the fields as a row's click does;»
  source: .andromeda/architecture-detail/standard-contracts.md:272 «`cells-grid` takes `id`, `tabindex` 0, `aria_label` Cells and an `onkeydown`, with no role, so it is one Tab stop: arrows move `current`, clamped, into view»
  replace-with: «moving the selection and filling the fields as a row's click does; its Cells grid `cells-grid` is a keyboard region named `Cells` by `aria_label` with no role, one Tab stop — the arrows move the current cell, clamped, into view, Enter opens its editor, focused, and the editor's Enter commits and returns focus to the grid;»
- leaf: CLAUDE.md:45   class: retired
  leaf-text: «the pages the stand serves beside the lean four hold no Tab-order, keyboard-activation or contrast row, so SC 2.1.1 and 1.4.3 are unasserted on the load page's `load-fetch` Button, a gap owned by a `.andromeda/residuals.md` line;»
  source: .andromeda/a11y-plan.md:144 «The load page and Cells hold Tab-order, keyboard and contrast rows (`stand_keyboard`, `stand_contrast`, `stand_load_page`, `stand_cells_served`), so SC 2.1.1 and 1.4.3 are asserted on the load page's»
  replace-with: «the load page and Cells hold Tab-order (`LOAD_TAB_ORDER`, `CELLS_TAB_ORDER`), keyboard and contrast rows, so SC 2.1.1 and 1.4.3 are asserted on the load page's `load-fetch` Button and on Cells' `cells-grid`;»
- leaf: CLAUDE.md:45   class: retired
  leaf-text: «while HTML's focus fixup is not built (a node made inert keeps focus until focus moves), owned by a `.andromeda/residuals.md` line (target: next);»
  source: .andromeda/architecture.md:74 «The focus fixup: a node that holds focus when it comes under `inert`, a subtree moved under an inert box included, gives it up at the mutator's flush through `BaseDocument::clear_focus`»
  replace-with: «and HTML's focus fixup is built: a node that holds focus when it comes under `inert`, a subtree moved under an inert box included, gives it up at the mutator's flush, with no DOM `blur` or `focusout`, reading neither `disabled` nor `hidden`;»
- leaf: CLAUDE.md:48   class: retired
  leaf-text: «pipeline tests run `for incremental in [false, true]`, except upstream's `pseudo_element_flow_classification`, which does not assert both layout modes, `incremental_oracle` green beside it.»
  source: .andromeda/architecture-detail/established-decisions.md:102 «its test file asserting both layout modes, every test running `for incremental in [false, true]`»
  replace-with: «pipeline tests run `for incremental in [false, true]`.»
- leaf: CLAUDE.md:75   class: omission
  leaf-text: «its focus and submit in the private `click_button` · check `tests/blitz-tests/tests/stand_keyboard.rs`»
  source: .andromeda/architecture-detail/existing-scopes.md:151 «`checkable_change_event`, 3 tests — a click and a Space on a checkbox each firing `input` then `change`, a radio's `change` firing on an actual change alone»
  replace-with: «its focus and submit in the private `click_button` · check `tests/blitz-tests/tests/stand_keyboard.rs` · the engine's `change` on a checkbox and a radio, and none on a text or range input: the checkbox and radio arms of `handle_click` in `packages/blitz-dom/src/events/pointer.rs`, checks `tests/blitz-tests/tests/{checkable_change_event,text_range_no_change_event}.rs`»
- leaf: CLAUDE.md:81   class: omission
  leaf-text: «(`scroll_into_view`, `visible_region`; check `tests/blitz-tests/tests/scroll_into_view_nested.rs`)»
  source: .andromeda/architecture-detail/existing-scopes.md:151 «`visible_region_transformed`, 6 tests — a world under an untransformed viewport showing a node and panning one out»
  replace-with: «(`scroll_into_view`, `visible_region`; checks `tests/blitz-tests/tests/{scroll_into_view_nested,visible_region_transformed}.rs`)»
- leaf: CLAUDE.md:82   class: omission
  leaf-text: «checks `tests/blitz-tests/tests/{scrolled_box_client_rect,hit_clipped_at_scrolling_box,scrolled_box_absolute_position,transformed_client_rect}.rs`»
  source: .andromeda/architecture-detail/existing-scopes.md:151 «`hit_non_invertible_transform`, 2 tests — a box under a non-invertible transform taking no hit and its invertible twin hit where it shows»
  replace-with: «checks `tests/blitz-tests/tests/{scrolled_box_client_rect,hit_clipped_at_scrolling_box,hit_non_invertible_transform,scrolled_box_absolute_position,transformed_client_rect}.rs`»
- leaf: CLAUDE.md:98   class: stale
  leaf-text: «the stand checks, ten named blitz-tests targets outside the `stand_` prefix»
  source: .andromeda/test-plan.md:377 «the `stand-repeat` leg runs the stand checks, fifteen named blitz-tests targets outside the `stand_` prefix and seven_guis' process checks for three rounds»
  replace-with: «the stand checks, fifteen named blitz-tests targets outside the `stand_` prefix»
- leaf: CLAUDE.md:110   class: omission
  leaf-text: «(on a select it chooses the first option whose label starts with the text)»
  source: .andromeda/architecture-detail/standard-contracts.md:179 «on a select the text chooses the first option whose label starts with it and, on a `multiple` select, that the text chooses its match alone»
  replace-with: «(on a select it chooses the first option whose label starts with the text, on a `multiple` select, focused with no click, its match alone)»
- leaf: CLAUDE.md:110   class: stale
  leaf-text: «and a scrolling box under a transform is read where it stands untransformed, so an element inside it can read `off-screen` though it shows —»
  source: .andromeda/architecture-detail/standard-contracts.md:177 «under a rotation a box's visible part reads as the bounding box of its rotated padding box, so an element in a corner it does not show can read in view»
  replace-with: «and under a rotation a box's visible part reads as the bounding box of its rotated padding box, so an element in a corner it does not show can read in view —»
- leaf: CLAUDE.md:110   class: retired
  leaf-text: «the counter, the timer, CRUD and the load page reading 0 findings at boot and the flight booker one `overlap` (`flight-type-one-way` × `flight-type-return`, the snapshot giving each option its select's client rect — the lint's reading of a select's options, owned by a `.andromeda/residuals.md` line)»
  source: .andromeda/architecture-detail/standard-contracts.md:220 «the snapshot gives a closed select's options its client rect and the lint's `overlaps` skips them, so on the stand all four lean tasks read 0 `overlap` at boot»
  replace-with: «the counter, the flight booker, the timer, CRUD and the load page reading 0 findings at boot (the snapshot giving a closed select's options its client rect and the lint's `overlaps` skipping them, a `multiple` select's rows reading their own client rects and linted as usual)»
