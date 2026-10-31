proposals:
  - detector: D-retired
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 41
    section: "## 5. Keyboard Navigation"
    change: "Keep the row's arrows, Home, End, typeahead and change clauses, and edit three clauses. After «that is no disabled form control (`ElementData::is_disabled`)» add «, its list closed,». Replace «every other key does nothing (Enter and Space open nothing, a released Space activates no select)» with «Enter, and a Space with no typed text standing, open the list on press, a released Space doing nothing more, and every other key does nothing». Replace «and no popup is painted» with «and the list the engine paints over the page takes its own keys (the entries below)». End the row's provenance with «; the list's opening as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md»."
    sidecar: "keyboard-navigation:41: Enter and Space open a single select's list; the limits «open nothing» and «no popup is painted» retired."
    rationale: "The report's Retired bullet names «no popup is painted» and «Enter and Space open nothing on a single select»; its Keys entry has `press_select_key` opening on Enter and on a Space with no typed text standing; patterns `no-popup` and `enter-space` both list keyboard-navigation:41 as changed."
    basis: .andromeda/a11y-plan-detail/keyboard-navigation.md:41
  - detector: D-retired
    severity: warning
    file: .andromeda/a11y-plan.md
    near: 123
    section: "## 5. Keyboard Navigation"
    change: "The broad line should read: A focused single-choice `select` that is no disabled form control takes ArrowDown / ArrowUp to the next / previous enabled option, Home / End to the first / last, and a character with no Ctrl, Alt or Meta as typeahead kept for 1 s of the document's clock; Enter, and a Space with no typed text standing, open a list the engine paints over the page, whose own keys move a highlight and whose Enter, Space or Tab choose; every other key does nothing; each change dispatches one `input` then one `change`, a key that changes nothing none; an `optgroup`'s options are the select's own in tree order, and a `multiple` select takes its own keys and row clicks."
    sidecar: "a11y-plan:123: the broad line drops «no popup is painted» and states the list's opening keys."
    rationale: "The report's Retired bullet names «no popup is painted»; pattern `no-popup` lists a11y-plan:123 as changed, and this line is the broad line of keyboard-navigation:41."
    basis: .andromeda/a11y-plan.md:123
    dependent-of: D-retired
  - detector: D-cascade
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 26
    section: "## 5. Keyboard Navigation"
    change: "Split the select clause out, since the row already stands over the 2 000-char limit. In line 26, delete from «, a click on a `select` focuses it» through that clause's closing parenthesis of three citations, so the row goes on «(ratified by the founder, 2026-10-18), and a click on any other element is unchanged». Add directly after it this row, those three citations moved with it unchanged: - A click on a `select` focuses it through `generate_focus_events` / `set_focus_to`, clearing its typeahead and the document's text selection, a disabled select stopped by the walk's gate first so it neither focuses nor acts; on a single select each click also opens its list or closes one standing, two clicks on one point opening then closing, each focusing, and a row of an open list takes a click through `handle_click`'s `option` arm behind `click_lands_on_row`, while an option of a closed single select, under a group too, is not clicked (a driver `click` on one is refused `covered` — on the stand the flight booker's two options among them); a `multiple` select's row takes a click through that arm (the `click_select_row` entry) ({the three citations}) — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md"
    sidecar: "keyboard-navigation:26: the select clause split into its own row, stating that a click opens or closes a single select's list and that an open list's row takes a click."
    rationale: "The report's Pointer entry has `handle_click`'s `select` arm opening a single select's list or closing one standing and its `option` arm gaining `click_list_row` behind `click_lands_on_row`; pattern `closed-option` lists keyboard-navigation:26 as changed and an option of a closed select still `covered`; the report lists this line over 2 000 chars, and the added text cannot fit without the split."
    basis: .andromeda/a11y-plan-detail/keyboard-navigation.md:26
  - detector: D-cascade
    severity: warning
    file: .andromeda/a11y-plan.md
    near: 113
    section: "## 5. Keyboard Navigation"
    change: "Replace «and a click on a `select` focuses it, a click on a `multiple` select's row focuses the select and acts on the row, and an option of a closed single select takes no click» with «a click on a `select` focuses it and on a single select also opens its list or closes one standing, a row of an open list takes a click, a click on a `multiple` select's row focuses the select and acts on the row, and an option of a closed single select takes no click»."
    sidecar: "a11y-plan:113: a click on a single select opens or closes its list, and an open list's row takes a click."
    rationale: "Pattern `click-help` lists a11y-plan:113 as changed through its neighbour clause; the report's Pointer entry states the `select` arm's opening and the `option` arm's `click_list_row`, and acceptance holds `two_clicks_on_one_point_open_and_close_each_focusing`."
    basis: .andromeda/a11y-plan.md:113
    dependent-of: D-cascade
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 10
    section: "## 5. Keyboard Navigation"
    change: "Add after line 10: - With a list open, Tab runs `choose_current_and_close` before moving focus, in either direction (packages/blitz-dom/src/events/keyboard.rs:32-34; tests/blitz-tests/tests/select_popup.rs:329-361) — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md"
    sidecar: "keyboard-navigation: Tab chooses and closes an open list before moving focus."
    rationale: "The report's Keys entry states it at `keyboard.rs` 32-34, and the task's F4 places it in this category."
    basis: packages/blitz-dom/src/events/keyboard.rs:32-34
    fact: F4
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 10
    section: "## 5. Keyboard Navigation"
    change: "Add after line 10: - Focus stays on the select while its list is open, and the Tab walk holds with the list open and after each close, pinned by `the_flight_type_walk_holds_with_its_list_open_and_after_each_close` and `a_click_opens_the_list_focus_stays_on_the_select_and_the_walk_goes_on_from_it` (tests/blitz-tests/tests/stand_keyboard.rs:1082-1111; tests/blitz-tests/tests/stand_keyboard.rs:1113-1134) — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md"
    sidecar: "keyboard-navigation: focus stays on the select and the Tab walk holds with a list open and after each close."
    rationale: "The report's acceptance (layouts) names the first test as met, its New text lists both tests in `stand_keyboard`, and the task's F7 places the fact in this category."
    basis: tests/blitz-tests/tests/stand_keyboard.rs:1082-1111
    fact: F7
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 13
    section: "## 5. Keyboard Navigation"
    change: "Add after line 13: - A disabled row of an open list changes nothing and leaves the list open; a disabled, inert or hidden select, or one with no row to show, opens nothing; a select made inert, disabled, hidden or removed while open closes its list (packages/blitz-dom/src/document.rs:1408-1410; tests/blitz-tests/tests/select_popup.rs:377-388; tests/blitz-tests/tests/select_popup.rs:443-472; tests/blitz-tests/tests/select_popup.rs:478-520) — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md"
    sidecar: "keyboard-navigation: which selects open a list, what a disabled row does, and what closes a list when its select changes."
    rationale: "The report's acceptance (security) states these closes as met, its Deviations state that a select with no row to show opens nothing, its New text lists the three tests, and the task's F6 places the fact in this category."
    basis: tests/blitz-tests/tests/select_popup.rs:478-520
    fact: F6
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 25
    section: "## 5. Keyboard Navigation"
    change: "Add after line 25: - Escape, a second click, a click outside and the page scrolling each close an open list with the value unchanged and no event, a press outside it running `close_list_outside` from `handle_pointerdown`; a wheel outside the list closes it and then scrolls, and a close clears the typed text (packages/blitz-dom/src/events/pointer.rs:415-420; packages/blitz-dom/src/events/pointer.rs:619-633; packages/blitz-dom/src/document.rs:891-907; tests/blitz-tests/tests/select_popup.rs:401-426) — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md"
    sidecar: "keyboard-navigation: what closes an open list, with no event and the value unchanged."
    rationale: "The report's Pointer entry states `close_list_outside` and `handle_wheel`, its Deviations state that a close clears the typed text, and the task's F5 places the fact in this category."
    basis: tests/blitz-tests/tests/select_popup.rs:401-426
    fact: F5
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 26
    section: "## 5. Keyboard Navigation"
    change: "Add after line 26: - A pointer over an enabled row of an open list moves the highlight and a later key goes on from it; a press on a row starts no text drag, and the release chooses the row under it (packages/blitz-dom/src/events/pointer.rs:595-611; packages/blitz-dom/src/events/pointer.rs:613-617; tests/blitz-tests/tests/select_popup.rs:751-781; tests/blitz-tests/tests/select_popup.rs:783-803) — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md"
    sidecar: "keyboard-navigation: the pointer's highlight on a row, and a press and release on one."
    rationale: "The report's Pointer entry states `hover_list_row` and `pressed_in_list`, its Deviations state that the release chooses the row under it, and the task's F8 places the fact in this category."
    basis: tests/blitz-tests/tests/select_popup.rs:751-781
    fact: F8
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 41
    section: "## 5. Keyboard Navigation"
    change: "Add after line 41: - An open list takes its own closed key match, `press_open_list_key`: its move keys carry the highlight over enabled rows and choose nothing, and the list opens with the highlight on the chosen option, firing no event (packages/blitz-dom/src/events/keyboard.rs:215-250; tests/blitz-tests/tests/select_popup.rs:202-230; tests/blitz-tests/tests/select_popup.rs:249-273) — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md"
    sidecar: "keyboard-navigation: an open list's own key match and its highlight."
    rationale: "The report's Keys entry states `press_open_list_key` at 215-250, its New text lists both tests, and the task's F2 places the fact in this category."
    basis: packages/blitz-dom/src/events/keyboard.rs:215-250
    fact: F2
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 41
    section: "## 5. Keyboard Navigation"
    change: "Add after line 41: - With a list open, Enter, Space, Tab and a click on a row choose the highlighted or clicked row and close, through `choose_current_and_close` and `click_list_row` to the one write path `apply_select_choice`: one `input` then one `change`, and choosing the already chosen option fires nothing (packages/blitz-dom/src/events/keyboard.rs:252-267; packages/blitz-dom/src/events/pointer.rs:642-655; tests/blitz-tests/tests/select_popup.rs:294-327; tests/blitz-tests/tests/select_popup.rs:363-375) — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md"
    sidecar: "keyboard-navigation: how an open list's row is chosen, and the events it fires."
    rationale: "The report's acceptance (arch) has `click_list_row` and `choose_current_and_close` calling `apply_select_choice`, its New text lists both tests, and the task's F3 places the fact in this category."
    basis: tests/blitz-tests/tests/select_popup.rs:294-327
    fact: F3
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 41
    section: "## 5. Keyboard Navigation"
    change: "Add after line 41: - A list shows at most `MAX_ROWS` 20 and scrolls by key and by wheel; a row brought into view writes the list's own `scroll_y`, moving no scroll chain or viewport (packages/blitz-dom/src/scrolling.rs:669-672; tests/blitz-tests/tests/select_popup.rs:870-911) — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md"
    sidecar: "keyboard-navigation: a long list's twenty rows and its own scrolling."
    rationale: "The report states `MAX_ROWS` = 20 and, under Seen through the list, `scroll_into_view` writing `scroll_y` and returning at `scrolling.rs` 669-672; the task's F9 places the fact in this category."
    basis: packages/blitz-dom/src/scrolling.rs:669-672
    fact: F9
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 41
    section: "## 5. Keyboard Navigation"
    change: "Add after line 41: - The list's thumb is painted, takes no hit and is not draggable, so keys, the wheel and `scroll` are the ways to move a long list (packages/blitz-dom/src/document.rs:2404-2406; tests/blitz-tests/tests/select_popup_painted.rs:312-351; docs/capabilities.md:211-213) — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md"
    sidecar: "keyboard-navigation: the list's thumb takes no hit, so a long list moves by keys, the wheel and `scroll`."
    rationale: "The report has `hit_with_scrollbar` answering `select_list_hit` first and no scrollbar with it, lists `a_scrolled_list_paints_its_thumb_which_takes_no_hit`, and names the thumb not draggable in the P5 row; the task's F10 places the fact in this category."
    basis: tests/blitz-tests/tests/select_popup_painted.rs:312-351
    fact: F10
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 42
    section: "## 5. Keyboard Navigation"
    change: "Add after line 42: - On a `multiple` select with a valid `size`, a key brings the current row into view through `scroll_into_view`, gated by `carries_valid_size`, and its other row semantics are unchanged (packages/blitz-dom/src/events/keyboard.rs:423-430; packages/blitz-dom/src/events/keyboard.rs:452-459; tests/blitz-tests/tests/select_popup.rs:995-1032) — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md"
    sidecar: "keyboard-navigation: a key brings a sized `multiple` select's current row into view."
    rationale: "The report's Keys entry states `apply_row_gesture` calling `scroll_into_view` only where `carries_valid_size` at 423-430 and 452-459, and `press_multiple_select_key` keeping its semantics; the task's F11 places the fact in this category."
    basis: packages/blitz-dom/src/events/keyboard.rs:423-430
    fact: F11
note: "The report lists keyboard-navigation:48 (`type-focus`), :55 (`label-click`), :119 (`enter-space`) and :120 (`kbd-count`) as changed, but none is in this group's picked listing, so they go unproposed here and need a route; F12 is left to the new-detail pass, no picked row being a check inventory; F6's hidden close stands beside the key-press panic at `node.rs:1775`, which security-plan owns."
