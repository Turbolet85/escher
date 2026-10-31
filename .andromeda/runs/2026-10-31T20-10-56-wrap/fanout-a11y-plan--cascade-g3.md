proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/a11y-plan.md
    near: 113
    section: "## 5. Keyboard Navigation"
    change: "Replace «a click on a `select` focuses it, a click on a `multiple` select's row focuses the select and acts on the row, and an option of a closed single select takes no click» with «a click on a single `select` focuses it and opens its list, or closes one standing, each click focusing; a click on a `multiple` select focuses it, a click on a `multiple` select's row focuses the select and acts on the row; a row of an open list takes a click, and an option of a closed single select takes no click»."
    sidecar: "§5 Focus restoration: a click on a single select now opens or closes its list, each click focusing, and an open list's row takes a click."
    rationale: "Report Changes, Pointer: `handle_click`'s `select` arm opens a single select's list or closes one standing and its `option` arm gains `click_list_row`; `click-help` names a11y-plan:113; Outcome pins two clicks opening then closing, each focusing."
    basis: .andromeda/a11y-plan.md:113
  - detector: D-retired
    severity: warning
    file: .andromeda/a11y-plan.md
    near: 123
    section: "## 5. Keyboard Navigation"
    change: "Replace «every other key does nothing» with «Enter, and a Space with no typed text standing, open its list; every other key does nothing», and «and no popup is painted» with «and an open list, painted by the engine over the page, takes its own key match, its highlight moved by keys and the pointer, a row chosen by Enter, Space, Tab or a click, and Escape, a second click, a click outside and a page scroll closing it with no event»."
    sidecar: "§5 Keyboard handling: the retired limit «no popup is painted» replaced by the open list's keys, choice and close."
    rationale: "Report Retired names «no popup is painted» and «Enter and Space open nothing on a single select»; `no-popup` names a11y-plan:123; Keys gives `press_select_key` opening on Enter and on a Space with no typed text standing."
    basis: .andromeda/a11y-plan.md:123
  - detector: D-cascade
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 48
    section: "## 5. Keyboard Navigation"
    change: "Replace «The driver's `type` on a `multiple` select focuses it with no click, which would land on the row at the centre, and clears the typeahead, and a text opening on a space types nothing» with «The driver's `type` on any select focuses it with no click, which on a `multiple` select would land on the row at the centre, so no DOM focus events fire on a single select either, closes any list and clears the typeahead, and a text opening on a space types nothing there»; the rest of the line stands."
    sidecar: "keyboard-navigation:48: `type` focuses any select with no click, firing no DOM focus events on a single select, and closes any list."
    rationale: "Report Changes, Driver: `focus_multiple_select` became `focus_select`, which also closes any list; `type` on any select focuses with no click; `type-focus` names keyboard-navigation:48."
    basis: .andromeda/a11y-plan-detail/keyboard-navigation.md:48
  - detector: D-retired
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 50
    section: "## 5. Keyboard Navigation"
    change: "Replace «the flight booker's `select#flight-type` takes no activation key» with «a focused single select such as `select#flight-type` instead opens its list on a pressed Enter, or a pressed Space with no typed text standing, and a released Space does nothing more»; the rule for other controls stands."
    sidecar: "keyboard-navigation:50: a single select no longer takes no activation key; Enter and Space on press open its list."
    rationale: "Report Retired names «Enter and Space open nothing on a single select»; Keys gives `press_select_key` opening on Enter and on a Space with no typed text standing; the renamed test reads a released Space doing nothing more."
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 43
    section: "## 5. Keyboard Navigation"
    change: "- On a `multiple` select with a valid `size`, a key brings the current row into view through `scroll_into_view`, gated by `carries_valid_size`, and its other row semantics are unchanged (packages/blitz-dom/src/events/keyboard.rs:423-430; packages/blitz-dom/src/events/keyboard.rs:452-459; tests/blitz-tests/tests/select_popup.rs:995-1032) — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md"
    sidecar: "keyboard-navigation: a `multiple` select with a valid `size` brings the current row into view by key."
    rationale: "Report Changes, Keys: `apply_row_gesture` calls `scroll_into_view` on the current row only where `carries_valid_size`, and `press_multiple_select_key` and `click_select_row` keep their semantics."
    fact: F11
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 51
    section: "## 5. Keyboard Navigation"
    change: "- An open list takes its own closed key match, `press_open_list_key`: its move keys carry the highlight over enabled rows and choose nothing, and the list opens with the highlight on the chosen option, firing no event (packages/blitz-dom/src/events/keyboard.rs:215-250; tests/blitz-tests/tests/select_popup.rs:202-230; tests/blitz-tests/tests/select_popup.rs:249-273) — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md"
    sidecar: "keyboard-navigation: an open list's own key match moves a highlight and chooses nothing."
    rationale: "Report Changes, Keys: `press_open_list_key` (215-250) is the open list's closed key match."
    fact: F2
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 51
    section: "## 5. Keyboard Navigation"
    change: "- Enter, Space, Tab and a click on a row choose the highlighted or clicked row and close through `apply_select_choice`, one `input` then one `change`, and choosing the already chosen option fires nothing (packages/blitz-dom/src/events/keyboard.rs:252-267; packages/blitz-dom/src/events/pointer.rs:642-655; tests/blitz-tests/tests/select_popup.rs:294-327; tests/blitz-tests/tests/select_popup.rs:363-375) — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md"
    sidecar: "keyboard-navigation: Enter, Space, Tab and a row's click choose and close, firing one `input` then one `change`."
    rationale: "Report Outcome, the one write path: `click_list_row` and `choose_current_and_close` call `apply_select_choice`, no other writer in the diff."
    fact: F3
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 51
    section: "## 5. Keyboard Navigation"
    change: "- With a list open, Tab runs `choose_current_and_close` before moving focus, in either direction (packages/blitz-dom/src/events/keyboard.rs:32-34; tests/blitz-tests/tests/select_popup.rs:329-361) — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md"
    sidecar: "keyboard-navigation: Tab chooses and closes an open list before focus moves, either way."
    rationale: "Report Changes, Keys: Tab runs `choose_current_and_close` before moving focus, either direction (32-34)."
    fact: F4
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 51
    section: "## 5. Keyboard Navigation"
    change: "- Escape, a second click, a click outside and the page scrolling each close the list with the value unchanged and no event, a wheel outside the list closes it and then scrolls, and a close clears the typed text (packages/blitz-dom/src/document.rs:881-889; packages/blitz-dom/src/document.rs:891-907; packages/blitz-dom/src/events/pointer.rs:619-633; tests/blitz-tests/tests/select_popup.rs:401-426) — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md"
    sidecar: "keyboard-navigation: the ways an open list closes with no choice and no event."
    rationale: "Report Changes, Pointer: `close_list_outside` and `handle_wheel` closing then scrolling; Deviations: a close clears the typed text; the test at select_popup.rs:401-426 pins the four closers."
    fact: F5
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 51
    section: "## 5. Keyboard Navigation"
    change: "- A disabled row changes nothing and leaves the list open; a disabled, inert or hidden select, or one with no row to show, opens nothing; a select made inert, disabled, hidden or removed while open closes its list (packages/blitz-dom/src/document.rs:761-795; packages/blitz-dom/src/document.rs:891-907; tests/blitz-tests/tests/select_popup.rs:377-388; tests/blitz-tests/tests/select_popup.rs:443-472; tests/blitz-tests/tests/select_popup.rs:478-520) — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md"
    sidecar: "keyboard-navigation: a disabled row, a select that cannot open and one made unavailable while open."
    rationale: "Report Outcome, security acceptance, and Deviations: a select with no row to show opens nothing; the tests at select_popup.rs:377-388, :443-472 and :478-520 pin these."
    fact: F6
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 51
    section: "## 5. Keyboard Navigation"
    change: "- A list shows at most `MAX_ROWS` 20 and scrolls by key and by wheel; a row brought into view writes the list's own `scroll_y`, moving no scroll chain or viewport (packages/blitz-dom/src/scrolling.rs:669-672; packages/blitz-dom/src/document.rs:947-960; tests/blitz-tests/tests/select_popup.rs:870-911) — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md"
    sidecar: "keyboard-navigation: a list shows at most twenty rows and scrolls itself alone."
    rationale: "Report Changes, Seen through the list: `scroll_into_view` on a row writes the list's `scroll_y` at once and returns, no chain or viewport moving (669-672); `MAX_ROWS` = 20."
    fact: F9
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 51
    section: "## 5. Keyboard Navigation"
    change: "- The list's thumb is painted, takes no hit and is not draggable, so keys, the wheel and `scroll` are the ways to move a long list (packages/blitz-paint/src/render/select_list.rs:130-154; packages/blitz-dom/src/document.rs:962-988; tests/blitz-tests/tests/select_popup_painted.rs:312-351) — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md"
    sidecar: "keyboard-navigation: the list's thumb is painted and takes no hit, a stated limit."
    rationale: "Report Changes, The hit: `select_list_hit` answers first and no scrollbar with it; Paint: `paint_list_thumb`; inputs#I7 lists the thumb not draggable as a limit."
    fact: F10
  - detector: D-cascade
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 55
    section: "## 5. Keyboard Navigation"
    change: "Replace «so a click on a select's label focuses the select (§7)» with «while a click on a label bound to a single select focuses the select, opens nothing and no longer re-dispatches to it (§7), asserted by `a_labels_click_focuses_the_select_and_opens_nothing`»."
    sidecar: "keyboard-navigation:55: a single select's label click focuses it, opens nothing and does not re-dispatch."
    rationale: "Report Changes, Pointer: the `label` arm focuses a bound single select and opens nothing; `label-click` names keyboard-navigation:55 and the test at select_popup.rs:428-441."
    basis: .andromeda/a11y-plan-detail/keyboard-navigation.md:55
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 55
    section: "## 5. Keyboard Navigation"
    change: "- A pointer over an enabled row moves the highlight and a later key goes on from it; a press on a row starts no text drag, and the release chooses the row under it (packages/blitz-dom/src/events/pointer.rs:595-611; packages/blitz-dom/src/events/pointer.rs:613-617; tests/blitz-tests/tests/select_popup.rs:751-781; tests/blitz-tests/tests/select_popup.rs:783-803) — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md"
    sidecar: "keyboard-navigation: a pointer moves an open list's highlight, and a press on a row starts no text drag."
    rationale: "Report Changes, Pointer: `handle_pointermove` calls `hover_list_row` and `pressed_in_list` starts no text drag; Deviations: the release chooses the row under it."
    fact: F8
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 115
    section: "## 5. Keyboard Navigation"
    change: "- Focus stays on the select while its list is open, and the Tab walk holds with the list open and after each close, pinned by `the_flight_type_walk_holds_with_its_list_open_and_after_each_close` and `a_click_opens_the_list_focus_stays_on_the_select_and_the_walk_goes_on_from_it` (tests/blitz-tests/tests/stand_keyboard.rs:1082-1111; tests/blitz-tests/tests/stand_keyboard.rs:1113-1134) — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md"
    sidecar: "keyboard-navigation: `stand_keyboard` pins focus on the select and the Tab walk with a list open."
    rationale: "Report New text lists both tests in `stand_keyboard`'s append; `kbd-count` moves it 21 to 23; Outcome (layouts) reads the Tab walk holding."
    fact: F7
  - detector: D-retired
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 119
    section: "## 5. Keyboard Navigation"
    change: "Replace «Enter and Space opening nothing on a single select» with «Enter and Space opening the list on a single select and a released Space doing nothing more»; the rest of the line stands, the 14-row keys table's count included."
    sidecar: "keyboard-navigation:119: `select_interaction` now drives Enter and Space opening a single select's list."
    rationale: "Report Retired names «Enter and Space open nothing on a single select»; Test renames gives `enter_and_space_open_the_list_and_a_released_space_does_nothing_more`; Deviations: `KEYS` moved one row, Enter to `PageDown`, its count still 14; `enter-space` names keyboard-navigation:119."
    basis: .andromeda/a11y-plan-detail/keyboard-navigation.md:119
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 119
    section: "## 5. Keyboard Navigation"
    change: "- New checks read the select list: `select_popup` 25, `select_popup_painted` 6 and `stand_act_popup` 12, in both layout modes, with `select_list` 6 unit tests, and `cli_commands`, `host_log` and `mcp_log` +1 each (packages/blitz-dom/src/node/select_list.rs:386-539; examples/seven_guis/tests/cli_commands.rs:419-465; examples/seven_guis/tests/host_log.rs:282-394; examples/seven_guis/tests/mcp_log.rs:68-114) — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md"
    sidecar: "keyboard-navigation: the select list's new checks and their counts."
    rationale: "Report Coverage of new surfaces, select list: tests unit (`select_list` 6), integ (`select_popup` 25, `select_popup_painted` 6, `stand_act_popup` 12), process (`cli_commands`, `host_log`, `mcp_log` +1 each)."
    fact: F12
  - detector: D-cascade
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 120
    section: "## 5. Keyboard Navigation"
    change: "Replace «`stand_keyboard` holding 21 tests» with «its Enter and Space rows now opening the list and then choosing, so the test reads true and no longer reads what its row order suggests, `stand_keyboard` holding 23 tests»."
    sidecar: "keyboard-navigation:120: `stand_keyboard` 21 to 23 tests, and the closed-arrow test's Enter and Space rows now open and then choose."
    rationale: "Report `kbd-count` names keyboard-navigation:120; Decisions: the closed-arrow test passes untouched, yet its Enter and Space rows now open and then choose."
    basis: .andromeda/a11y-plan-detail/keyboard-navigation.md:120
note: The report names keyboard-navigation:26 and :41 as changed (`closed-option`, `no-popup`, `enter-space`) but neither line is among group 3's picked entities, so no proposal edits them here and they need a pick or the orchestrator's own pass; D-a11y-surface is carried by this list's proposals, and D-a11y-obs-schema does not fire because the report moves no log field or violation schema.
