group: a11y-plan
read: .claude/docs/a11y-summary.md · 45 · «**Full plan:** `.andromeda/a11y-plan.md`. Path-scoped rules:»
read: .claude/rules/a11y.md · 46 · «- 2026-10-06: On the dev host the AT-SPI bus reads `IsEnable»
- leaf: .claude/docs/a11y-summary.md:14   class: stale
  leaf-text: «(`the_flight_type_select_moves_its_choice_by_its_arrows`, 21 tests)»
  source: .andromeda/a11y-plan-detail/keyboard-navigation.md:132 «`stand_keyboard` holding 23 tests»
  replace-with: «(`the_flight_type_select_moves_its_choice_by_its_arrows`, 23 tests)»
- leaf: .claude/docs/a11y-summary.md:14   class: omission
  leaf-text: «both in the `a11y` leg, both three rounds in the `stand-repeat` leg, `stand_act_select` by its `stand_` prefix and `select_interaction` by the leg's named `--test` list.»
  source: .andromeda/a11y-plan.md:146 «The select list's keyboard and focus coverage is headless: `select_popup`, `stand_act_popup` and `stand_keyboard`'s list walks assert SC 2.1.1 and 2.4.3 on it in both layout modes»
  replace-with: «both in the `a11y` leg, both three rounds in the `stand-repeat` leg, `stand_act_select` by its `stand_` prefix and `select_interaction` by the leg's named `--test` list; the select list's keyboard and focus coverage is headless — `select_popup` (25 tests), `stand_act_popup` (12) and `stand_keyboard`'s list walks (`the_flight_type_walk_holds_with_its_list_open_and_after_each_close`, `a_click_opens_the_list_focus_stays_on_the_select_and_the_walk_goes_on_from_it`) assert SC 2.1.1 and 2.4.3 on it in both layout modes, `stand_keyboard` in the `a11y` leg and the other two outside it, in the workspace `test` leg and three `stand-repeat` rounds, and the windowed list is read and not run.»
- leaf: .claude/docs/a11y-summary.md:15   class: stale
  leaf-text: «18 `STATE_ROWS`»
  source: .andromeda/a11y-plan.md:64 «20 `STATE_ROWS` (the colour states it reaches, hover and active included)»
  replace-with: «20 `STATE_ROWS`»
- leaf: .claude/docs/a11y-summary.md:15   class: stale
  leaf-text: «pass 17, exempt 1»
  source: .andromeda/a11y-plan-detail/visual-design-verification.md:12 «list-open 2 (`flight-type-one-way` and `flight-type-return`, each 17.40) — pass 19 and exempt 1»
  replace-with: «pass 19, exempt 1»
- leaf: .claude/docs/a11y-summary.md:15   class: omission
  leaf-text: «12.56:1 measured by `select_interaction` in both layout modes.»
  source: .andromeda/a11y-plan.md:164 «An open select list's colours are engine constants on `SelectListView`, in no sheet — `HIGHLIGHT_TEXT`, `HIGHLIGHT_BACKGROUND`, `DISABLED_TEXT` and `BORDER`»
  replace-with: «12.56:1 measured by `select_interaction` in both layout modes. An open select list's colours are engine constants on `SelectListView`, in no sheet — `HIGHLIGHT_TEXT` `#000000`, `HIGHLIGHT_BACKGROUND` `#c8c8c8`, `DISABLED_TEXT` `#6d6d6d`, `BORDER` `#999999` — and `select_popup_painted` (6 tests) reads the highlight and a disabled row as painted against those constants and measures the plain pair at the default sheet and at the stand, in both layout modes, so SC 1.4.3 is asserted on the list in the harness; a highlighted row's contrast is unread by the lint and a closed select's own pair is unread.»
- leaf: .claude/docs/a11y-summary.md:19   class: retired
  leaf-text: «named in the tree and unpainted — what an assistive technology reads of them is not measured.»
  source: .andromeda/a11y-plan.md:208 «named in the tree and painted as a header of an open list, heavier than a row by weight on every build, a header choosing nothing»
  replace-with: «named in the tree and painted as a header of an open list, heavier than a row by weight on every build, a header choosing nothing — what an assistive technology reads of them is not measured.»
- leaf: .claude/docs/a11y-summary.md:19   class: omission
  leaf-text: «a `multiple` select's `ListBox` writes `set_multiselectable`, and»
  source: .andromeda/a11y-plan.md:210 «A single select reads `ComboBox` and writes `expanded` from its list's open state, reading no `aria-expanded`, and an author's `open` or `aria-expanded` on it steers nothing»
  replace-with: «a single select's `ComboBox` writes `expanded` from its list's open state, reading no `aria-expanded`, an author's `open` or `aria-expanded` on it steering nothing and `:open` not matching it; the tree's focus stays on the select and its chosen option reads `selected`; the highlighted row is in no snapshot and no tree and the snapshot holds no `expanded` key, and what an assistive technology reads of the list is not measured; a `multiple` select's `ListBox` writes `set_multiselectable`, and»
- leaf: .claude/docs/a11y-summary.md:21   class: stale
  leaf-text: «`aria-expanded` (a boolean),»
  source: .andromeda/a11y-plan-detail/a11y-strategy.md:23 «`aria-expanded` (a boolean, on every element but a single select, which reads none and writes `expanded` from its list's open state)»
  replace-with: «`aria-expanded` (a boolean, on every element but a single select, which reads none and writes `expanded` from its list's open state),»
- leaf: .claude/docs/a11y-summary.md:23   class: omission
  leaf-text: «a click on a `select` focuses it, clearing its typeahead and the document's text selection (a disabled select stopped by the walk's gate first, so it neither focuses nor acts;»
  source: .andromeda/a11y-plan.md:113 «a click on a single `select` focuses it and opens its list or closes one standing, each of two clicks on one point focusing; a click on a row of an open list chooses it»
  replace-with: «a click on a `select` focuses it, clearing its typeahead and the document's text selection, and on a single select also opens its list or closes one standing, two clicks on one point opening then closing, each focusing (a disabled select stopped by the walk's gate first, so it neither focuses nor acts; a click on a row of an open list choosing it, a disabled row changing nothing and leaving the list open and a group header choosing nothing; a press outside the list closing it with no event, and a driver `click` on an element under the list refused `covered`; a label's click on a bound single select focusing it and opening nothing;»
- leaf: .claude/docs/a11y-summary.md:23   class: stale
  leaf-text: «Select: a focused single-choice `select` (no `multiple`) that is no disabled form control (`ElementData::is_disabled`) takes»
  source: .andromeda/a11y-plan-detail/keyboard-navigation.md:47 «that is no disabled form control (`ElementData::is_disabled`), its list closed, takes a pressed ArrowDown / ArrowUp»
  replace-with: «Select: a focused single-choice `select` (no `multiple`) that is no disabled form control (`ElementData::is_disabled`), its list closed, takes»
- leaf: .claude/docs/a11y-summary.md:23   class: retired
  leaf-text: «every other key does nothing (Enter and Space open nothing, a released Space activates no select);»
  source: .andromeda/a11y-plan-detail/keyboard-navigation.md:47 «Enter, and a Space with no typed text standing, open the list on press, a released Space doing nothing more, and every other key does nothing»
  replace-with: «Enter, and a Space with no typed text standing, open its list on press, on the chosen option and firing nothing, a released Space doing nothing more, a disabled, inert, hidden or empty select opening nothing, and every other key does nothing;»
- leaf: .claude/docs/a11y-summary.md:23   class: retired
  leaf-text: «a disabled group's rows reading as disabled, and no popup is painted.»
  source: .andromeda/a11y-plan.md:123 «with the list open the engine paints it over the page and focus stays on the select, its keys moving the highlight over enabled rows and choosing nothing»
  replace-with: «a disabled group's rows reading as disabled. With the list open — an engine overlay on every document, remote pages included — the engine paints it over the page and focus stays on the select: its keys move the highlight over enabled rows and choose nothing, a pointer over an enabled row moving it too; Enter and Space choose the highlighted row and close, Tab in either direction chooses and closes before focus moves, one `input` then one `change` firing and none for the option already chosen; Escape closes with the value unchanged and no event, as a second click, a press outside and a page scroll do, a wheel outside closing it and then scrolling; a close clears the typed text, and a select made inert, disabled, hidden or removed while open closes its list. A list shows at most `MAX_ROWS` 20 and scrolls by key and by wheel, its thumb painted, taking no hit and not draggable, its rows one height, and it opens and closes with no motion.»
- leaf: .claude/docs/a11y-summary.md:23   class: omission
  leaf-text: «and with Ctrl or Meta they move the current row and change nothing;»
  source: .andromeda/a11y-plan-detail/keyboard-navigation.md:53 «On a `multiple` select with a valid `size`, a key brings the current row into view through `scroll_into_view`, gated by `carries_valid_size`»
  replace-with: «and with Ctrl or Meta they move the current row and change nothing; with a valid `size` a key brings the current row into view through `scroll_into_view`;»
- leaf: .claude/docs/a11y-summary.md:23   class: omission
  leaf-text: «the driver's `type` focuses it with no click and clears the typeahead»
  source: .andromeda/a11y-plan-detail/keyboard-navigation.md:59 «so it fires no DOM focus events on a single select either, closes any list and clears the typeahead»
  replace-with: «the driver's `type` on any select focuses it with no click, so it fires no DOM focus events on a single select either, closes any list and clears the typeahead»
- leaf: .claude/docs/a11y-summary.md:24   class: stale
  leaf-text: «which the driver's visual lint leaves out of its overlap reading in every document (`Entry.in_closed_select`, never the role)»
  source: .andromeda/a11y-plan-detail/visual-design-verification.md:16 «`Entry.in_select_box` true for an option of a closed select, read through `closed_select_of` and never the role, and for an open list's row or group, read through `select_list_bounds`»
  replace-with: «which the driver's visual lint leaves out of its overlap reading in every document with an open list's rows and groups, an open list's row reading its painted row's `bounds` (`Entry.in_select_box`, read through `closed_select_of` for a closed select's option and through `select_list_bounds` for an open list's row or group, never the role)»
- leaf: .claude/docs/a11y-summary.md:24   class: tree
  leaf-text: «`fixtures::password_named` (`tests/fixtures/mod.rs`)»
  source: .andromeda/a11y-plan-detail/screen-reader-support.md:36 «`fixtures::password_named`, no stand task or app gaining one with it (tests/blitz-tests/tests/fixtures/mod.rs:37-44»
  replace-with: «`fixtures::password_named` (`tests/blitz-tests/tests/fixtures/mod.rs`)»
- leaf: .claude/docs/a11y-summary.md:25   class: retired
  leaf-text: «the flight booker's `select#flight-type` takes no activation key»
  source: .andromeda/a11y-plan-detail/keyboard-navigation.md:61 «a focused single select such as the flight booker's `select#flight-type` instead opens its list on a pressed Enter, or a pressed Space with no typed text standing»
  replace-with: «a focused single select such as the flight booker's `select#flight-type` instead opens its list on a pressed Enter, or a pressed Space with no typed text standing, and a released Space does nothing more»
- leaf: .claude/docs/a11y-summary.md:26   class: omission
  leaf-text: «a focus move, a checked change, a rewritten name and other mutations mark it, a pointer move alone does not»
  source: .andromeda/a11y-plan.md:29 «a focus or checked change, a text control's input, a select's changed choice and a single select's list opening or closing mark it»
  replace-with: «a focus move, a checked change, a rewritten name, a text control's input, a select's changed choice, a single select's list opening or closing and other mutations mark it, a page scroll or a move that closes a list marking its select through `reconcile_select_list` at the next resolve and a wheel elsewhere through `close_select_list`, while a highlight or a wheel move inside a list marks nothing and a pointer move alone does not»
- leaf: .claude/rules/a11y.md:19   class: retired
  leaf-text: «the focus fixup firing no `blur`, no popup painted and Enter and Space opening nothing on a single select, a `multiple` select's `size` unread, `aria-activedescendant` unread»
  source: .andromeda/a11y-plan-detail/a11y-scope-summary.md:24 «the named limits (the focus fixup firing no `blur`, a single select's `size` unread, `aria-activedescendant` unread)»
  replace-with: «the focus fixup firing no `blur`, a single select's `size` unread, `aria-activedescendant` unread»
- leaf: .claude/rules/a11y.md:25   class: stale
  leaf-text: «`aria-expanded` (a boolean),»
  source: .andromeda/a11y-plan-detail/a11y-strategy.md:23 «`aria-expanded` (a boolean, on every element but a single select, which reads none and writes `expanded` from its list's open state)»
  replace-with: «`aria-expanded` (a boolean, on every element but a single select, which reads none and writes `expanded` from its list's open state),»
- leaf: .claude/rules/a11y.md:26   class: stale
  leaf-text: «`Entry.in_closed_select` reading the element through `closed_select_of` and never the role,»
  source: .andromeda/a11y-plan-detail/visual-design-verification.md:16 «`Entry.in_select_box` true for an option of a closed select, read through `closed_select_of` and never the role, and for an open list's row or group, read through `select_list_bounds`»
  replace-with: «`Entry.in_select_box` true for an option of a closed select, read through `closed_select_of` and never the role, and for an open list's row or group, read through `select_list_bounds`,»
- leaf: .claude/rules/a11y.md:26   class: omission
  leaf-text: «the stand's flight booker at boot reading no `overlap`, `stand_lint` reading `[0, 0, 0, 0]`.»
  source: .andromeda/a11y-plan-detail/screen-reader-support.md:11 «a select whose list is open is no closed select, `closed_select_of` answering `None` for it, and the snapshot's builder reads `select_list_bounds(node)` before the client rect»
  replace-with: «the stand's flight booker at boot reading no `overlap`, `stand_lint` reading `[0, 0, 0, 0]`. A select whose list is open is no closed select, `closed_select_of` answering `None` for it, and the snapshot's builder reads `select_list_bounds(node)` before the client rect, so an open row's `bounds` is its painted row's in the snapshot and the diff, `differs` naming `bounds`.»
- leaf: .claude/rules/a11y.md:26   class: omission
  leaf-text: «A `multiple` select carries `set_multiselectable` in the tree,»
  source: .andromeda/a11y-plan-detail/screen-reader-support.md:12 «A single select's tree node takes `expanded` from a closed bool of engine state, reading no `aria-expanded`, its focus staying on the select while the list is open»
  replace-with: «A single select's tree node takes `expanded` from a closed bool of engine state, reading no `aria-expanded`, its focus staying on the select while the list is open and its chosen option reading `selected`; an author's `open` or `aria-expanded` on it steers nothing and `:open` does not match it; the highlighted row is in no snapshot and no tree, and `NodeState` holds no `expanded` field (`NODE_FIELDS` thirteen), so a list's open state reaches the tree and not the snapshot; what an assistive technology reads of an open list is not measured and the windowed list is read and not run. A `multiple` select carries `set_multiselectable` in the tree,»
- leaf: .claude/rules/a11y.md:26   class: retired
  leaf-text: «named in the tree and unpainted;»
  source: .andromeda/a11y-plan.md:208 «named in the tree and painted as a header of an open list, heavier than a row by weight on every build, a header choosing nothing»
  replace-with: «named in the tree and painted as a header of an open list, heavier than a row by weight on every build, a header choosing nothing;»
- leaf: .claude/rules/a11y.md:33   class: omission
  leaf-text: «a click on a `select` focuses it through `generate_focus_events` / `set_focus_to`, clearing its typeahead and the document's text selection (a disabled select stopped by the walk's gate first, so it neither focuses nor acts;»
  source: .andromeda/a11y-plan-detail/keyboard-navigation.md:31 «on a single select each click also opens its list or closes one standing, two clicks on one point opening then closing, each focusing, and a row of an open list takes a click through `handle_click`'s»
  replace-with: «a click on a `select` focuses it through `generate_focus_events` / `set_focus_to`, clearing its typeahead and the document's text selection, and on a single select also opens its list or closes one standing, two clicks on one point opening then closing, each focusing (a disabled select stopped by the walk's gate first, so it neither focuses nor acts; a row of an open list taking a click through `handle_click`'s `option` arm behind `click_lands_on_row`, choosing it, a disabled row changing nothing and leaving the list open and a group header choosing nothing; a press outside the list running `close_list_outside` from `handle_pointerdown`, closing it with no event, and a driver `click` on an element under the list refused `covered`; a label's click on a bound single select focusing it, opening nothing and re-dispatching nothing to it;»
- leaf: .claude/rules/a11y.md:33   class: stale
  leaf-text: «A focused single-choice `select` (no `multiple`) that is no disabled form control (`ElementData::is_disabled`) takes a pressed»
  source: .andromeda/a11y-plan-detail/keyboard-navigation.md:47 «that is no disabled form control (`ElementData::is_disabled`), its list closed, takes a pressed ArrowDown / ArrowUp»
  replace-with: «A focused single-choice `select` (no `multiple`) that is no disabled form control (`ElementData::is_disabled`), its list closed, takes a pressed»
- leaf: .claude/rules/a11y.md:33   class: retired
  leaf-text: «every other key does nothing (Enter and Space open nothing, a released Space activates no select);»
  source: .andromeda/a11y-plan-detail/keyboard-navigation.md:47 «Enter, and a Space with no typed text standing, open the list on press, a released Space doing nothing more, and every other key does nothing»
  replace-with: «Enter, and a Space with no typed text standing, open its list on press, on the chosen option and firing nothing, a released Space doing nothing more, a disabled, inert, hidden or empty select opening nothing, and every other key does nothing;»
- leaf: .claude/rules/a11y.md:33   class: retired
  leaf-text: «a disabled group's rows reading as disabled through `option_disabled`, and no popup is painted.»
  source: .andromeda/a11y-plan.md:123 «with the list open the engine paints it over the page and focus stays on the select, its keys moving the highlight over enabled rows and choosing nothing»
  replace-with: «a disabled group's rows reading as disabled through `option_disabled`. With the list open — one `BaseDocument` field `select_list`, an engine overlay on every document, remote pages included, answered first by `hit_with_scrollbar` and painted by `paint_scene` after the root's render — focus stays on the select and its keys, `press_open_list_key`, move the highlight over enabled rows and choose nothing, a pointer over an enabled row moving it too; Enter, Space, Tab and a click on a row choose the highlighted or clicked row and close, through `choose_current_and_close` and `click_list_row` to the one write path `apply_select_choice`, one `input` then one `change` firing and none for the option already chosen, Tab in either direction choosing before focus moves; Escape closes with the value unchanged and no event, as a second click, a press outside and a page scroll do, a wheel outside closing it and then scrolling; a close clears the typed text, and a select made inert, disabled, hidden or removed while open closes its list. A list's opening or closing marks the changed set, a page scroll or a move that closes it marking its select through `reconcile_select_list` at the next resolve and a wheel elsewhere through `close_select_list`, while a highlight or a wheel move inside it marks nothing. A list shows at most `MAX_ROWS` 20 and scrolls by key and by wheel, a row brought into view writing the list's own `scroll_y` and moving no scroll chain or viewport, its thumb painted, taking no hit and not draggable, its rows one height, and it opens and closes with no motion.»
- leaf: .claude/rules/a11y.md:33   class: omission
  leaf-text: «and with Ctrl or Meta they move the current row and change nothing;»
  source: .andromeda/a11y-plan-detail/keyboard-navigation.md:53 «On a `multiple` select with a valid `size`, a key brings the current row into view through `scroll_into_view`, gated by `carries_valid_size`»
  replace-with: «and with Ctrl or Meta they move the current row and change nothing; with a valid `size` a key brings the current row into view through `scroll_into_view`, gated by `carries_valid_size`;»
- leaf: .claude/rules/a11y.md:33   class: omission
  leaf-text: «the driver's `type` on one focuses it with no click and clears the typeahead»
  source: .andromeda/a11y-plan-detail/keyboard-navigation.md:59 «so it fires no DOM focus events on a single select either, closes any list and clears the typeahead»
  replace-with: «the driver's `type` on any select focuses it with no click, so it fires no DOM focus events on a single select either, closes any list and clears the typeahead»
- leaf: .claude/rules/a11y.md:35   class: stale
  leaf-text: «(`the_flight_type_select_moves_its_choice_by_its_arrows`, 21 tests)»
  source: .andromeda/a11y-plan-detail/keyboard-navigation.md:132 «`stand_keyboard` holding 23 tests»
  replace-with: «(`the_flight_type_select_moves_its_choice_by_its_arrows`, 23 tests)»
- leaf: .claude/rules/a11y.md:35   class: omission
  leaf-text: «both in the `a11y` leg, both three rounds in the `stand-repeat` leg, `stand_act_select` by its `stand_` prefix and `select_interaction` by the leg's named `--test` list.»
  source: .andromeda/a11y-plan.md:146 «The select list's keyboard and focus coverage is headless: `select_popup`, `stand_act_popup` and `stand_keyboard`'s list walks assert SC 2.1.1 and 2.4.3 on it in both layout modes»
  replace-with: «both in the `a11y` leg, both three rounds in the `stand-repeat` leg, `stand_act_select` by its `stand_` prefix and `select_interaction` by the leg's named `--test` list; the select list's keyboard and focus coverage is headless — `select_popup` (25 tests), `stand_act_popup` (12) and `stand_keyboard`'s list walks (`the_flight_type_walk_holds_with_its_list_open_and_after_each_close`, `a_click_opens_the_list_focus_stays_on_the_select_and_the_walk_goes_on_from_it`) assert SC 2.1.1 and 2.4.3 on it in both layout modes, and the windowed list is read and not run.»
- leaf: .claude/rules/a11y.md:38   class: stale
  leaf-text: «18 `STATE_ROWS`»
  source: .andromeda/a11y-plan.md:64 «20 `STATE_ROWS` (the colour states it reaches, hover and active included)»
  replace-with: «20 `STATE_ROWS`»
- leaf: .claude/rules/a11y.md:38   class: omission
  leaf-text: «12.56:1 measured by `select_interaction` in both layout modes.»
  source: .andromeda/a11y-plan.md:164 «An open select list's colours are engine constants on `SelectListView`, in no sheet — `HIGHLIGHT_TEXT`, `HIGHLIGHT_BACKGROUND`, `DISABLED_TEXT` and `BORDER`»
  replace-with: «12.56:1 measured by `select_interaction` in both layout modes. An open select list's colours are engine constants on `SelectListView`, in no sheet — `HIGHLIGHT_TEXT` `#000000`, `HIGHLIGHT_BACKGROUND` `#c8c8c8`, `DISABLED_TEXT` `#6d6d6d`, `BORDER` `#999999` — and `select_popup_painted` (6 tests) reads the highlight and a disabled row as painted against those constants and measures the plain pair at the default sheet and at the stand, in both layout modes, so SC 1.4.3 is asserted on the list in the harness; a highlighted row's contrast is unread by the lint and a closed select's own pair is unread.»
- leaf: .claude/rules/a11y.md:42   class: stale
  leaf-text: «runs fourteen files (112 tests)»
  source: .andromeda/a11y-plan-detail/ci-integration.md:11 «14 result lines · 114 passed · 0 failed · 0 ignored»
  replace-with: «runs fourteen files (114 tests)»
- leaf: .claude/rules/a11y.md:42   class: omission
  leaf-text: «as `stand_keyboard`, `stand_contrast` and `stand_accessibility_ids` do.»
  source: .andromeda/a11y-plan-detail/ci-integration.md:13 «`select_popup`, `select_popup_painted` and `stand_act_popup` run in the workspace `test` leg and three `stand-repeat` rounds»
  replace-with: «as `stand_keyboard`, `stand_contrast` and `stand_accessibility_ids` do. `select_popup`, `select_popup_painted` and `stand_act_popup` run in the workspace `test` leg and three `stand-repeat` rounds, the first two by its named `--test` list and `stand_act_popup` by its `stand_*` glob; none is among this leg's fourteen files, though `stand_keyboard`, one of them, carries the list's walk here.»
