proposals:
  - detector: D-retired
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 41
    section: "## 5. Keyboard Navigation"
    change: "Replace «A focused single-choice `select` (no `multiple`) that is no disabled form control» with «A focused single-choice `select` (no `multiple`) that is no disabled form control, its list closed,», and replace «every other key does nothing (Enter and Space open nothing, a released Space activates no select)» with «Enter, and a Space with no typed text standing, open its list (the entry below), a released Space doing nothing more; every other key does nothing»."
    sidecar: "keyboard-navigation: a single select's Enter and Space open its list; the closed-select keys scoped to a closed list."
    rationale: "Report Retired names «Enter and Space open nothing on a single select»; Symbols / APIs, Keys: `press_select_key` opens on Enter, and on a Space with no typed text standing; Counts `enter-space` lists keyboard-navigation:41."
    basis: escher-0.3.0/chunks/2026-10-31-select-popup/report.md:43
  - detector: D-retired
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 41
    section: "## 5. Keyboard Navigation"
    change: "Replace «, and no popup is painted» with «, and the list a click, Enter or Space opens is painted by the engine over the page (`paint_select_list`)»."
    sidecar: "keyboard-navigation: the limit «no popup is painted» ended; the engine paints an open list."
    rationale: "Report Retired names «no popup is painted»; Symbols / APIs, Paint: `paint_scene` calls `paint_select_list`; Counts `no-popup` lists keyboard-navigation:41."
    basis: escher-0.3.0/chunks/2026-10-31-select-popup/report.md:42
  - detector: D-retired
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 50
    section: "## 5. Keyboard Navigation"
    change: "Replace «the flight booker's `select#flight-type` takes no activation key» with «the flight booker's `select#flight-type` takes neither key as a click, its Enter and Space opening its list (the select's entry above)»."
    sidecar: "keyboard-navigation: the stand's flight-type select now opens its list on Enter and Space."
    rationale: "A further wording of the retired «Enter and Space open nothing on a single select», beyond the `enter-space` pattern's listed sites; report Symbols / APIs, Keys, and `select_interaction`'s test rename; `stand_keyboard`'s `list_walk` walks this select's list."
    basis: escher-0.3.0/chunks/2026-10-31-select-popup/report.md:15
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 119
    section: "## 5. Keyboard Navigation"
    change: "Replace «`select_interaction` also drives Enter and Space opening nothing on a single select» with «`select_interaction` also drives Enter and Space opening the list on a single select and a released Space doing nothing more»."
    sidecar: "keyboard-navigation: `select_interaction`'s Enter and Space test restated after its rename."
    rationale: "Report Test renames: `enter_and_space_open_nothing_on_a_single_select` became `enter_and_space_open_the_list_and_a_released_space_does_nothing_more`; Counts `enter-space` lists keyboard-navigation:119."
    basis: escher-0.3.0/chunks/2026-10-31-select-popup/report.md:23
    dependent-of: D-retired
  - detector: D-a11y-surface
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 26
    section: "## 5. Keyboard Navigation"
    change: "Replace «clearing its typeahead and the document's text selection, a disabled select» with «clearing its typeahead and the document's text selection, a single select's click also opening its list or closing one standing (the entry below), a disabled select»."
    sidecar: "keyboard-navigation: a click on a single select opens its list or closes one standing."
    rationale: "Report Symbols / APIs, Pointer: `handle_click`'s `select` arm opens a single select's list or closes one standing; Counts `closed-option` lists keyboard-navigation:26, the closed-option clause itself true on his F9."
    basis: escher-0.3.0/chunks/2026-10-31-select-popup/report.md:14
  - detector: D-a11y-surface
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 48
    section: "## 5. Keyboard Navigation"
    change: "Replace «The driver's `type` on a `multiple` select focuses it with no click, which would land on the row at the centre, and clears the typeahead, and a text opening on a space types nothing;» with «The driver's `type` on any select focuses it with no click, which on a `multiple` select would land on the row at the centre, so firing no DOM focus events on a single select either, closes any list standing and clears the typeahead, and a text opening on a space types nothing;»."
    sidecar: "keyboard-navigation: the driver's `type` focuses any select with no click and closes any list."
    rationale: "Report Symbols / APIs, Driver: `Target.multiple` became `Target.select`; `focus_select` also closes any list; `type` on any select focuses with no click; Counts `type-focus` lists keyboard-navigation:48."
    basis: escher-0.3.0/chunks/2026-10-31-select-popup/report.md:20
  - detector: D-a11y-surface
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 55
    section: "## 5. Keyboard Navigation"
    change: "Replace «, so a click on a select's label focuses the select (§7)» with «, while a click on a single select's label focuses that select through the `label` arm and opens nothing, no click re-dispatched, asserted by `select_popup`'s `a_labels_click_focuses_the_select_and_opens_nothing` (§7)»."
    sidecar: "keyboard-navigation: a single select's label focuses it and opens nothing, now asserted."
    rationale: "Report Symbols / APIs, Pointer: the `label` arm focuses a bound single select and opens nothing; Spec claims disproved `label-click`: no longer re-dispatches; Counts `label-click` lists keyboard-navigation:55."
    basis: escher-0.3.0/chunks/2026-10-31-select-popup/report.md:57
  - detector: D-a11y-surface
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 120
    section: "## 5. Keyboard Navigation"
    change: "Replace «`stand_keyboard` holding 21 tests» with «`stand_keyboard` holding 23 tests»."
    sidecar: "keyboard-navigation: `stand_keyboard` count 21 to 23."
    rationale: "Report Counts `kbd-count`: `stand_keyboard` 21 to 23, listing keyboard-navigation:120."
    basis: escher-0.3.0/chunks/2026-10-31-select-popup/report.md:34
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 41
    section: "## 5. Keyboard Navigation"
    change: "- A single select's open list takes the keys through `press_open_list_key`, a closed key match: they move the highlight over enabled rows and choose nothing, a list of more than twenty rows scrolling by key; Enter, Space and Tab choose the highlighted row through `choose_current_and_close` and close, one `input` then one `change`, a choice of the chosen option firing nothing; Tab runs it before focus moves, in either direction; Escape closes with the value unchanged; a close clears the typed text; a select with no row to show opens nothing; the select keeps the tree's focus, the tree writes `expanded` on it and the highlight is in no snapshot and no tree (§7) (packages/blitz-dom/src/events/keyboard.rs:32-34; packages/blitz-dom/src/events/keyboard.rs:215-250; packages/blitz-dom/src/events/keyboard.rs:252-267; tests/blitz-tests/tests/select_popup.rs:202-230; tests/blitz-tests/tests/select_popup.rs:249-273; tests/blitz-tests/tests/select_popup.rs:294-327; tests/blitz-tests/tests/select_popup.rs:329-361; tests/blitz-tests/tests/select_popup.rs:363-375; tests/blitz-tests/tests/select_popup.rs:565-614; tests/blitz-tests/tests/select_popup.rs:870-911) — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md"
    sidecar: "keyboard-navigation: new row, the open list's keys."
    rationale: "The list is a new interactive element and §5 holds no row for its keys (D-a11y-surface); report Symbols / APIs, Keys and Tree, Coverage of new surfaces, and Deviations' details chosen at implement."
    basis: escher-0.3.0/chunks/2026-10-31-select-popup/report.md:15
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 26
    section: "## 5. Keyboard Navigation"
    change: "- A click on a single select opens its list or closes one standing, in every document, each through `focus_select`, so two driver `click`s on one point open then close, each focusing; focus stays on the select while its list stands, and on the stand the Tab walk goes on from it (packages/blitz-dom/src/events/pointer.rs:1108-1119; tests/blitz-tests/tests/stand_act_popup.rs:379-406; tests/blitz-tests/tests/stand_keyboard.rs:1113-1134) — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md"
    sidecar: "keyboard-navigation: new row, a click opening or closing a single select's list, focus kept."
    rationale: "Report Symbols / APIs, Pointer; Outcome (arch) two `click`s open then close, each focusing; (layouts) the Tab walk holds; SC 2.4.3 asserted per Coverage of new surfaces."
    basis: escher-0.3.0/chunks/2026-10-31-select-popup/report.md:152
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 27
    section: "## 5. Keyboard Navigation"
    change: "- On an open list the pointer answers first: `hit_with_scrollbar` reads `select_list_hit` before anything else; a move highlights an enabled row and a later key stands from it; a press on a row selects no text and the release chooses the row under it, `click_list_row` behind `click_lands_on_row` the `option` arm's new path; a disabled row changes nothing and leaves the list open; a group's header chooses nothing; the wheel scrolls the list over it; the thumb is painted and takes no hit (packages/blitz-dom/src/document.rs:2404-2406; packages/blitz-dom/src/events/pointer.rs:595-611; packages/blitz-dom/src/events/pointer.rs:613-617; packages/blitz-dom/src/events/pointer.rs:635-640; packages/blitz-dom/src/events/pointer.rs:642-655; tests/blitz-tests/tests/select_popup.rs:377-388; tests/blitz-tests/tests/select_popup.rs:751-781; tests/blitz-tests/tests/select_popup.rs:783-803; tests/blitz-tests/tests/select_popup.rs:805-855; tests/blitz-tests/tests/select_popup_painted.rs:312-351) — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md"
    sidecar: "keyboard-navigation: new row, the pointer on an open list's rows."
    rationale: "Report Symbols / APIs, The hit and Pointer; Deviations: a press on a row starts no text drag and the release chooses the row under it."
    basis: escher-0.3.0/chunks/2026-10-31-select-popup/report.md:14
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 28
    section: "## 5. Keyboard Navigation"
    change: "- An open list closes with no event on Escape, a second click, a press outside it (`close_list_outside`), a page scroll or a move of its select (`reconcile_select_list` at the next resolve) and a wheel outside it (`close_select_list`), and closes where its select is removed or made disabled, inert or hidden, each such close marking the select in the changed set, a highlight or a wheel move inside the list marking nothing; a `:focus` rule moving the select's box leaves a list the click opened (`fresh`); with an ancestor hidden a later key press panics at packages/blitz-dom/src/node/node.rs:1775, for any control and with no list opened, so `select_popup` skips the key press in that one row, the row owed in security-plan §Error Handling (packages/blitz-dom/src/events/pointer.rs:415-420; packages/blitz-dom/src/document.rs:881-889; packages/blitz-dom/src/document.rs:891-907; packages/blitz-dom/src/document.rs:1408-1410; packages/blitz-dom/src/resolve.rs:133-135; tests/blitz-tests/tests/select_popup.rs:401-426; tests/blitz-tests/tests/select_popup.rs:478-520; tests/blitz-tests/tests/select_popup.rs:1105-1122) — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md"
    sidecar: "keyboard-navigation: new row, how an open list closes, with the key-press panic beside it named."
    rationale: "Report Symbols / APIs, Pointer and The resolve; Spec claims disproved `changed-set` and `panic-pos`; Deviations' `fresh`; Outcome (security) met to its letter with a defect beside it. Binds with security-plan §Error Handling, the panic's row."
    basis: escher-0.3.0/chunks/2026-10-31-select-popup/report.md:96
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 45
    section: "## 5. Keyboard Navigation"
    change: "- A `multiple` select's `size`, a whole number capped at 65534, `0` or a bad value hinting nothing, makes it that many rows tall and scrollable, and a key brings the next row in: `apply_row_gesture` calls `scroll_into_view` on the current row only where `carries_valid_size`; `press_multiple_select_key` and `click_select_row` keep their semantics, and a single select's `size` stays unread (packages/blitz-dom/src/events/keyboard.rs:423-430; packages/blitz-dom/src/events/keyboard.rs:452-459; packages/blitz-dom/src/stylo.rs:1124-1139; packages/blitz-dom/assets/default.css:183-193; tests/blitz-tests/tests/select_popup.rs:995-1032) — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md"
    sidecar: "keyboard-navigation: new row, `size` on a `multiple` select and the row a key brings into view."
    rationale: "Report Retired names «a `multiple` select's `size` is unread»; Symbols / APIs, Keys and `size`; Coverage of new surfaces: a11y the row brought into view by key."
    basis: escher-0.3.0/chunks/2026-10-31-select-popup/report.md:21
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 119
    section: "## 5. Keyboard Navigation"
    change: "- A select list check runs in both layout modes: `select_popup` (25 tests) drives its openers, keys, choosers, closers, tree, pointer, groups, long list, hostile markup and `size` through the harness; `select_popup_painted` (6) reads its pixels; and through the driver `stand_act_popup` (12) opens by click and moves and chooses by keys alone, closes with the value unchanged and types leaving no list open, so SC 2.1.1, 2.4.3 and 1.4.3 are asserted in the harness, what an assistive technology reads unmeasured and the windowed list read and not run; `select_popup` and `select_popup_painted` are named `--test`s of the `stand-repeat` leg and `stand_act_popup` joins by its prefix, none of the three in the `a11y` leg, which stays fourteen files (tests/blitz-tests/tests/stand_act_popup.rs:58-101; tests/blitz-tests/tests/stand_act_popup.rs:103-125; tests/blitz-tests/tests/stand_act_popup.rs:127-155; tests/blitz-tests/tests/stand_act_popup.rs:247-285; .github/scripts/ci-leg.sh:59-60) — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md"
    sidecar: "keyboard-navigation: new row, the select list's checks and their legs."
    rationale: "Report Coverage of new surfaces (tests, a11y); Harness / gate surface; Outcome (a11y) SC 2.1.1, 2.4.3, 1.4.3 and the tree met in the harness."
    basis: escher-0.3.0/chunks/2026-10-31-select-popup/report.md:110
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 120
    section: "## 5. Keyboard Navigation"
    change: "- `stand_keyboard::the_flight_type_walk_holds_with_its_list_open_and_after_each_close` steps the ten rows of `list_walk` and `a_click_opens_the_list_focus_stays_on_the_select_and_the_walk_goes_on_from_it` takes the click's path, both in both layout modes; the closed-arrow test passes untouched, yet its Enter and Space rows now open and then choose, so it reads true and no longer reads what its row order suggests (tests/blitz-tests/tests/stand_keyboard.rs:1060-1080; tests/blitz-tests/tests/stand_keyboard.rs:1082-1111; tests/blitz-tests/tests/stand_keyboard.rs:1113-1134) — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md"
    sidecar: "keyboard-navigation: new row, `stand_keyboard`'s two list walks and what its closed-arrow test now reads."
    rationale: "Report New text, by line for `stand_keyboard.rs`; Outcome (layouts) the Tab walk holds; Decisions & corrections on the closed-arrow test."
    basis: escher-0.3.0/chunks/2026-10-31-select-popup/report.md:142
note: D-a11y-obs-schema not violated (report Schema / config, no log field or span); keyboard-navigation:26 and :119 already stand over the 2 000-char line limit, so their edits stay one clause and new detail takes rows of its own; body lines a11y-plan:113 and :123 carry the same claims and are outside this file's proposals.
