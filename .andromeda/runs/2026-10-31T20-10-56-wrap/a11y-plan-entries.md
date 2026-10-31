## 2026-10-31-select-popup — the select list as a scope entity
**Section:** §1 A11y Scope Summary (a11y-scope-summary.md: the new Select list row)
**Change:**
- New row: the select list is an engine overlay on every document, remote pages included, that a closed single select opens: one `BaseDocument` field `select_list`, answered first by `hit_with_scrollbar` and painted by `paint_scene` after the root's render.
- SC 2.1.1, 2.4.3 and 1.4.3 are asserted in the harness by `select_popup`, `select_popup_painted`, `stand_act_popup` and `stand_keyboard`'s list walks; what an assistive technology reads of it is not measured and the windowed list is read and not run.
**Why:** the report's new surface, the list on every document, with an assistive technology's reading `unrunnable-here`. That a closed select opens one is the founder's F1, by dialog in the overseer session, 2026-10-31, relayed verbatim by the operator (inputs#I4).
**Ref:** .andromeda/runs/2026-10-31T20-10-56-wrap/

## 2026-10-31-select-popup — the capability statement's named limits
**Section:** §1 A11y Scope Summary (a11y-scope-summary.md: the Capability statement row)
**Change:**
- The named limits were the focus fixup firing no `blur`, no popup painted and Enter and Space opening nothing on a single select, a `multiple` select's `size` unread, `aria-activedescendant` unread; now the focus fixup firing no `blur`, a single select's `size` unread, `aria-activedescendant` unread.
- The named limits' measure was the 2026-10-31-engine-debts report; now this chunk's report.
**Why:** the report retires «no popup is painted», «Enter and Space open nothing on a single select» and «a `multiple` select's `size` is unread». `size` is read on a `multiple` select on the founder's F8, by dialog in the overseer session, 2026-10-31, relayed verbatim by the operator (inputs#I4).
**Kept:** a single select's `size` stays unread and a named limit; the statement's pin still reads none of these claims.
**Ref:** .andromeda/runs/2026-10-31T20-10-56-wrap/

## 2026-10-31-select-popup — citations re-pinned where the chunk moved lines
**Section:** §1 (a11y-scope-summary.md: Served pages); §2 (a11y-strategy.md); §3 (the master's Keyboard test harness); §4 (aria-patterns-roles.md; the master's Native element mapping); §5 (keyboard-navigation.md); §6 (visual-design-verification.md); §7 (screen-reader-support.md); §9 (ci-integration.md: the job row); cognitive-accessibility.md (the invalid dates row)
**Change:**
- Citation digits only, on rows no other entry here names: the rows' claims stand as they read. cognitive-accessibility.md takes nothing else.
- On a row another entry names, its moved citations are likewise re-pinned with it.
**Why:** the chunk's edits to the engine, driver and checks moved the cited lines, and the report leaves citation digits to this pass's sweep.
**Ref:** .andromeda/runs/2026-10-31T20-10-56-wrap/

## 2026-10-31-select-popup — the changed set where a list stands
**Section:** §2 A11y Strategy (a11y-strategy.md: the `changed_nodes` row); the master's Accessibility tree lifecycle
**Change:**
- The sites marking the set were a focus or checked change, a text control's input and a select's changed choice; a single select's list opening or closing now marks it too.
- Was: hover, active, scroll and layout do not mark it, without exception. Now that holds with no list open; where a list stands, a page scroll or a move that closes it marks its select through `reconcile_select_list` at the next resolve, and a wheel elsewhere marks it through `close_select_list`.
- A highlight or a wheel move inside the list marks nothing.
**Why:** the report disproves the unqualified exclusion by measurement: with a list standing those moves mark its select, and with none open it holds.
**Ref:** .andromeda/runs/2026-10-31T20-10-56-wrap/

## 2026-10-31-select-popup — a single select's `expanded` in the tree
**Section:** §2 A11y Strategy (a11y-strategy.md: upstream's six reads; the master's Role derivation); §3 A11y Assertion Harness Contract (the merged states row); §4 ARIA Patterns & Roles (aria-patterns-roles.md: the new Select, single row); §7 Screen Reader Support (screen-reader-support.md: new row; the master's Accessibility tree output)
**Change:**
- `aria-expanded` was read on every element; now on every element but a single select, which reads none and writes `expanded` from its list's open state.
- New §4 row: a single select's pattern is `ComboBox` with `expanded` so written, a closed bool of engine state, and the chosen option `selected`.
- New §7 row and master bullet: an author's `open` or `aria-expanded` steers nothing and the tree's focus stays on the select; the row's pins are `the_tree_reads_the_list_open_on_the_select_its_focus_staying_there` and `an_author_aria_expanded_and_open_steer_nothing_on_a_select_and_open_does_not_match`.
- §3: `select_popup` asserts that an author's attribute steers nothing.
**Why:** the report's tree change. The answer card reads a single select's native `expanded` as a closed bool of engine state, inside the states class and outside the widening.
**Kept:** every other element keeps the `aria-expanded` read.
**Ref:** .andromeda/runs/2026-10-31T20-10-56-wrap/

## 2026-10-31-select-popup — a group's label painted as a header
**Section:** §4 ARIA Patterns & Roles (aria-patterns-roles.md: Option group, the new Option group's header row); §6 Visual Design Verification (visual-design-verification.md: new row under state colour); §7 Screen Reader Support (the master's Accessibility tree output)
**Change:**
- A group's label was named in the tree and unpainted; now named in the tree and painted as a header on an open list.
- New §4 row: a header row chooses nothing, its options stand under it, and a group with no label reads the whole list's box.
- New §6 row: a header is heavier than a row by a synthesized bold, on plain and emboldening builds, pinned by `a_groups_header_is_darker_than_a_row_by_weight`.
- The master states the same: heavier by weight on every build, a header choosing nothing.
**Why:** the report retires «a group's label is unpainted» on the founder's F7, by dialog in the overseer session, 2026-10-31, relayed verbatim by the operator (inputs#I4); the card records that its question told him the answer settles a group's name in the tree too. Heavier on every build is the operator's direction past implement, after an emboldening build read a header no heavier than a row. An unlabelled group's box was chosen at implement.
**Ref:** .andromeda/runs/2026-10-31T20-10-56-wrap/

## 2026-10-31-select-popup — the `a11y` leg's counts and the `stand-repeat` list
**Section:** §9 CI Integration (ci-integration.md: the three a11y rows); §6 Visual Design Verification (visual-design-verification.md: the visual lint row)
**Change:**
- `stand-repeat`'s named `--test` words were fifteen; now seventeen.
- The `a11y` leg was 14 files and 112 tests, `stand_keyboard` 21; now 14 files and 114 tests, `stand_keyboard` 23. Over the tier-0 witnesses alone it was 9 files and 86 tests; now 9 files and 88.
- Its log: the six files were 52; now 54. Result lines 14, passed was 112; now 114, 0 failed, 0 ignored. The run was CI#41192433140; now CI#41235503227.
- The lint row's count of the leg was 112 tests over 14 targets; now 114.
**Why:** the report's counts move `stand_keyboard` by its two list walks and the leg with it. It states no 88: that is this line's own sum, 114 less the five tree-reading checks' 26.
**Kept:** the leg stays fourteen files, with its slow tier and no required status check.
**Ref:** .andromeda/runs/2026-10-31T20-10-56-wrap/

## 2026-10-31-select-popup — the list's checks and the legs they run in
**Section:** §9 CI Integration (ci-integration.md: new row); §5 Keyboard Navigation (keyboard-navigation.md: the harness rows; the master's Test harness pattern)
**Change:**
- New §9 row: `select_popup`, `select_popup_painted` and `stand_act_popup` run in the workspace `test` leg and three `stand-repeat` rounds, the first two by its named `--test` list and `stand_act_popup` by its `stand_*` glob; none is among the a11y leg's fourteen, `stand_keyboard` carrying the list's walk there (`the_flight_type_walk_holds_with_its_list_open_and_after_each_close`, `a_click_opens_the_list_focus_stays_on_the_select_and_the_walk_goes_on_from_it`).
- New §5 row: `select_popup` 25, `select_popup_painted` 6, `stand_act_popup` 12, in both layout modes, with `select_list`'s 6 unit tests and one test each in `cli_commands`, `host_log` and `mcp_log`.
- New master bullet: the list's keyboard and focus coverage is headless, SC 2.1.1 and 2.4.3 asserted in both layout modes; the windowed list is read and not run.
- `select_interaction` was said to drive Enter and Space opening nothing; now opening the list, a released Space doing nothing more.
- `stand_keyboard`'s closed-arrow row now says its Enter and Space rows open the list and then choose, so the test reads true and not what its row order suggests; the count was 21 tests, now 23.
**Why:** the report's new-surface counts and leg change, and its correction on the closed-arrow test.
**Ref:** .andromeda/runs/2026-10-31T20-10-56-wrap/

## 2026-10-31-select-popup — what opens a single select's list
**Section:** §5 Keyboard Navigation (keyboard-navigation.md: the single-choice select row, the pressed-Enter row, a new row; the master's Keyboard handling)
**Change:**
- The single-choice row now applies with the list closed. Was: Enter and Space open nothing, a released Space activates no select, no popup is painted. Now: Enter, and a Space with no typed text standing, open the list on press, a released Space doing nothing more, and the list the engine paints over the page takes its own keys.
- The pressed-Enter row was: `select#flight-type` takes no activation key; now a focused single select such as it opens its list on a pressed Enter, or a pressed Space with no typed text standing.
- The master adds: the list opens on the chosen option and fires nothing, a disabled, inert, hidden or empty select opening nothing; «no popup is painted» ends.
- New row: a disabled row changes nothing and leaves the list open; a disabled, inert or hidden select, or one with no row to show, opens nothing; a select made inert, disabled, hidden or removed while open closes its list.
**Why:** the report retires «no popup is painted» and «Enter and Space open nothing on a single select». The opening is the founder's F1, by dialog in the overseer session, 2026-10-31, relayed verbatim by the operator (inputs#I4). A select with no row to show was chosen at implement.
**Ref:** .andromeda/runs/2026-10-31T20-10-56-wrap/

## 2026-10-31-select-popup — an open list's keys, choice and close
**Section:** §5 Keyboard Navigation (keyboard-navigation.md: new rows; the master's Keyboard handling)
**Change:**
- New: `press_open_list_key` is an open list's own closed key match; its move keys carry the highlight over enabled rows and choose nothing, and the list opens with the highlight on the chosen option, firing no event.
- New: Enter, Space, Tab and a click on a row choose and close, through `choose_current_and_close` and `click_list_row` to the one write path `apply_select_choice`: one `input` then one `change`, the already chosen option firing nothing.
- New: Tab, in either direction, runs `choose_current_and_close` before moving focus.
- New: Escape, a second click, a click outside and the page scrolling close with the value unchanged and no event, a press outside running `close_list_outside` from `handle_pointerdown`; a wheel outside closes and then scrolls; a close clears the typed text.
- The master states the keys, the choice and the closes in one sentence of the single-choice bullet.
**Why:** the highlight then the choice is the founder's F2 and a click outside his F4, by dialog in the overseer session, 2026-10-31, relayed verbatim by the operator (inputs#I4). The page's scroll closing is a lean the answer card says he was shown and did not object to (inputs#I5, inputs#I6), the report naming it the operator's lean. A close clearing the typed text was chosen at implement.
**Ref:** .andromeda/runs/2026-10-31T20-10-56-wrap/

## 2026-10-31-select-popup — a click, the pointer and focus on an open list
**Section:** §5 Keyboard Navigation (keyboard-navigation.md: the pointer click row, the label row, new rows; the master's Focus restoration)
**Change:**
- A click on a `select` was a clause of the pointer click row; now its own row: on a single select each click also opens its list or closes one standing, two clicks on one point opening then closing, each focusing, and a row of an open list takes a click through `handle_click`'s `option` arm behind `click_lands_on_row`.
- New: focus stays on the select while its list is open and the Tab walk holds, open and after each close.
- New: a pointer over an enabled row moves the highlight and a later key goes on from it; a press on a row starts no text drag, and the release chooses the row under it.
- The label row was: a click on a select's label focuses the select; now a label bound to a single select focuses it, opens nothing and re-dispatches nothing, asserted by `a_labels_click_focuses_the_select_and_opens_nothing`.
- The master adds a row's click, a disabled row and a header choosing nothing, a press outside closing with no event, and a label's click.
**Why:** the click's opening is the founder's F1 and focus staying his F3, by dialog in the overseer session, 2026-10-31, relayed verbatim by the operator (inputs#I4). A label opening nothing is a lean the answer card says he was shown and did not object to (inputs#I5, inputs#I6), the report naming it the operator's lean. The press and release were chosen at implement.
**Kept:** an option of a closed single select is still refused `covered`, on his F9 (inputs#I4).
**Ref:** .andromeda/runs/2026-10-31T20-10-56-wrap/

## 2026-10-31-select-popup — a long list and a sized `multiple` select
**Section:** §5 Keyboard Navigation (keyboard-navigation.md: new rows); §6 Visual Design Verification (visual-design-verification.md: new row under contrast)
**Change:**
- New: a list shows at most `MAX_ROWS` 20 and scrolls by key and by wheel; a row brought into view writes the list's own `scroll_y`, moving no scroll chain or viewport.
- New: the list's thumb is painted, takes no hit and is not draggable, so keys, the wheel and `scroll` move a long list.
- New: an open list's rows take one height, the select's font on one line, so a row with a larger authored font clips, a stated limit.
- New: on a `multiple` select with a valid `size`, a key brings the current row into view through `scroll_into_view`, gated by `carries_valid_size`, its other row semantics unchanged.
**Why:** the report's scrolling and key changes, with the thumb and the row height stated as limits in `docs/capabilities.md`. `size` on a `multiple` select is the founder's F8, by dialog in the overseer session, 2026-10-31, relayed verbatim by the operator (inputs#I4).
**Ref:** .andromeda/runs/2026-10-31T20-10-56-wrap/

## 2026-10-31-select-popup — the driver's `type` on any select
**Section:** §5 Keyboard Navigation (keyboard-navigation.md: the driver's `type` row)
**Change:**
- Was: `type` on a `multiple` select focuses it with no click. Now: on any select, so it fires no DOM focus events on a single select either and closes any list, a text opening on a space typing nothing there.
**Why:** the report's driver change: `Target.multiple` became `Target.select` and `focus_multiple_select` became `focus_select`, which also closes any list.
**Ref:** .andromeda/runs/2026-10-31T20-10-56-wrap/

## 2026-10-31-select-popup — what the snapshot and an assistive technology read of an open list
**Section:** §7 Screen Reader Support (screen-reader-support.md: the select row, new rows; the master's Accessibility tree output)
**Change:**
- The select row adds: a select whose list is open is no closed select, `closed_select_of` answering `None`, and the snapshot's builder reads `select_list_bounds(node)` before the client rect, so an open row's `bounds` is its painted row's in the snapshot and the diff, `differs` naming `bounds`.
- New: no `expanded` node key exists, `NodeState` holding no such field and `NODE_FIELDS` reading thirteen, so the open state reaches the tree and not the snapshot.
- New: the highlighted row is in no snapshot and no tree, a stated limit; an agent pressing keys sees it only in a screenshot.
- New: what an assistive technology reads of an open list stays unmeasured, recorded as `unrunnable-here`, the windowed list read and not run; SC 2.1.1, 2.4.3 and 1.4.3 and the tree are asserted in the harness alone.
- The master states the highlight, the key's absence, a row's own `bounds` and the unmeasured reading.
**Why:** the answer card reads a row's own `bounds` as a closed set inside the ratified crossing, by the field rule of 2026-10-10, and no `expanded` node key as the operator's call (inputs#I6). The report states the highlight's absence as a limit and the assistive technology's reading as `unrunnable-here`.
**Ref:** .andromeda/runs/2026-10-31T20-10-56-wrap/

## 2026-10-31-select-popup — how an open list's labels leave
**Section:** §7 Screen Reader Support (screen-reader-support.md: new row)
**Change:**
- New: an option's label and a group's `label` leave an open list only as painted pixels in a `screenshot` and the snapshot's `name`, with no log site, ratified; a shot of a row crops to it and leaves the list open (`a_shot_of_a_row_crops_to_it_and_leaves_the_list_open`).
**Why:** a page's option and group labels leaving as pixels in every shot of an open list is the widening's first half. Ratified by the founder: his F7 and F1, by dialog in the overseer session, 2026-10-31, relayed verbatim by the operator (inputs#I4), and his own word on this wrap's answer card (Q1), by dialog at P2 Validate, 2026-10-31, relayed verbatim by the operator.
**Ref:** .andromeda/runs/2026-10-31T20-10-56-wrap/

## 2026-10-31-select-popup — `STATE_ROWS`' list-open rows
**Section:** §6 Visual Design Verification (visual-design-verification.md: the contrast tables row); §3 A11y Assertion Harness Contract (the master's Contrast verification harness)
**Change:**
- `STATE_ROWS` was 18; now 20, adding list-open 2 (`flight-type-one-way` and `flight-type-return`, each 17.40).
- Pass was 17; now 19. Exempt stays 1.
**Why:** the report's count of `STATE_ROWS` moves by the two rows an open list shows.
**Ref:** .andromeda/runs/2026-10-31T20-10-56-wrap/

## 2026-10-31-select-popup — the visual lint over an open list
**Section:** §6 Visual Design Verification (visual-design-verification.md: the lint's select row, a new row)
**Change:**
- The lint's exclusion was a closed select's options; now those and an open list's rows and groups, the snapshot giving an open row its painted row's bounds.
- `Entry.in_closed_select` was the field; now `Entry.in_select_box`, true for an option of a closed select, read through `closed_select_of`, and for an open list's row or group, read through `select_list_bounds`.
- New clause: `clip_region` takes an open row's region from `visible_region` alone, no `contain: paint` cut.
- New row: the lint reads no `overlap` on an open list's row on the stand, `at_boot` reading `[0, 0, 0, 0]` and a planted pair reading as with no list open; its `contrast` reads a row's plain pair even when highlighted, so a highlighted row's contrast is unread by the lint, a stated limit.
**Why:** the report retires `Entry.in_closed_select`. A row reading no `overlap` is the founder's F10, by dialog in the overseer session, 2026-10-31, relayed verbatim by the operator (inputs#I4).
**Ref:** .andromeda/runs/2026-10-31T20-10-56-wrap/

## 2026-10-31-select-popup — the open list's colours and its measured pair
**Section:** §6 Visual Design Verification (visual-design-verification.md: the user-agent select row, new rows; the master's Color contrast)
**Change:**
- New: the colours are engine constants on `SelectListView`, not tokens: `HIGHLIGHT_TEXT` `#000000`, `HIGHLIGHT_BACKGROUND` `#c8c8c8`, `DISABLED_TEXT` `#6d6d6d`, `BORDER` `#999999`.
- New: `select_popup_painted` reads the highlight and a disabled row as painted against those constants and measures the plain pair at the default sheet and at the stand, in both layout modes.
- The user-agent row was: that pair is not measured on a single select; now a closed single select's own pair stays unread and the open list's plain pair is measured.
- The master states the constants by name, their values in design-system §Color Palette, and SC 1.4.3 asserted on the list in the harness.
**Why:** the answer card records the colours as engine constants on the founder's F6, by dialog in the overseer session, 2026-10-31, relayed verbatim by the operator (inputs#I4), without a widening. The report records `docs/capabilities.md`'s unmeasured-pair line removed and the open list's plain pair measured.
**Kept:** a closed select's own pair stays unread, and a highlighted row's contrast is unread by the lint.
**Ref:** .andromeda/runs/2026-10-31T20-10-56-wrap/

## 2026-10-31-select-popup — an open list has no motion
**Section:** §6 Visual Design Verification (visual-design-verification.md: new absence row under motion; the master's Motion)
**Change:**
- New row: motion on an open select's list is observed absent and needs no `advance`, a screen read after a step reading the new state, pinned by `a_screen_read_after_a_step_reads_the_new_state_with_no_advance`.
- New master bullet: a select list opens and closes with no motion (design-system §Motion).
**Why:** no animation is the founder's F5, by dialog in the overseer session, 2026-10-31, relayed verbatim by the operator (inputs#I4); the report's acceptance reads it met with no `advance` needed.
**Ref:** .andromeda/runs/2026-10-31T20-10-56-wrap/

## 2026-10-31-select-popup — an open list against `:open`, clips and `opacity`
**Section:** §6 Visual Design Verification (visual-design-verification.md: new rows under state colour and visibility)
**Change:**
- New: an open select's list is no `:open` state: neither `:open` nor `flush_open_state` reads the list, an author's `open` on a select steers nothing and `:open` does not match it, so no author style keys on a list standing.
- New: the list paints whole past an `opacity` ancestor and a box that clips its select, a point the list does not cover reads what paint shows, and a driver `click` on an element under the list is refused `covered`.
**Why:** the report leaves `:open` and `flush_open_state` unmoved, and its acceptance reads the clips, `opacity` and the point outside met.
**Ref:** .andromeda/runs/2026-10-31T20-10-56-wrap/
