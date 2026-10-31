proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 176
    section: "## Input Validation"
    change: "Add after this line a new row. Boundary: Markup numeric attributes. What to validate: `size` on a `multiple` select. How (stack-specific): In every document, remote pages included: on a `multiple` select `size` is read through `size_rows` (packages/blitz-dom/src/node/select_list.rs:125-130), `parse_unsigned_integer` capped at 65534, `0` or a bad value no hint, a parse into a capped whole number with no text carried out; a `height` of `size × 1.2em` is hinted (packages/blitz-dom/src/stylo.rs:1124-1139) and the default sheet gives `select[multiple][size]` `overflow-y: auto` and its rows `line-height: 1.2` (packages/blitz-dom/assets/default.css:183-193); `apply_row_gesture` calls `scroll_into_view` on the current row only where `carries_valid_size` (packages/blitz-dom/src/events/keyboard.rs:423-430; packages/blitz-dom/src/events/keyboard.rs:452-459); a single select's `size` stays unread; pinned in `select_popup` by the `size_rows` unit, a hostile `size` and `size=3` showing three rows — an engine change for every document and a new attribute read, carried to P2's answer card on the founder's recorded word, inputs#I4 of escher-0.3.0/chunks/2026-10-31-select-popup — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md"
    sidecar: "New row: `size` on a `multiple` select, a capped whole-number parse, a single select's still unread."
    rationale: "The report's Symbols / APIs `size` and Coverage `size on a multiple select` bring a new attribute read, validated by a capped parse; the plan's expected amendments owe a row for it."
    basis: escher-0.3.0/chunks/2026-10-31-select-popup/report.md:21
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 210
    section: "## Input Validation"
    change: "Replace «a click on a select's label re-dispatches to the select» with «a click on a label bound to a single select focuses it and opens nothing, no longer re-dispatching (`a_labels_click_focuses_the_select_and_opens_nothing`), and a click on a `multiple` select's label re-dispatches to it as before»."
    sidecar: "A single select's label click focuses and opens nothing, asserted by `select_popup`."
    rationale: "The report's `label-click` fact names input-validation:210 changed, and Spec claims disproved records that the single-select half no longer re-dispatches."
    basis: escher-0.3.0/chunks/2026-10-31-select-popup/report.md:57
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 219
    section: "## Input Validation"
    change: "Replace «`aria-expanded` a boolean» with «`aria-expanded` a boolean on every element but a single select, which writes `set_expanded` from its list being open, a closed bool from engine state, and reads no `aria-expanded` (packages/blitz-dom/src/accessibility.rs:238-242; `an_author_aria_expanded_and_open_steer_nothing_on_a_select_and_open_does_not_match`)»."
    sidecar: "A single select's `expanded` comes from engine state, no `aria-expanded` read on it."
    rationale: "The report's Tree bullet and its `expanded` fact name input-validation:219 changed; every other element keeps the read."
    basis: escher-0.3.0/chunks/2026-10-31-select-popup/report.md:16
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 242
    section: "## Input Validation"
    change: "Replace «a click on a `select` focusing it (the `select` row)» with «a click on a `select` focusing it and, on a single select, opening its list or closing one standing (the `select` row)», and after «through an `option` arm (the `select` with `multiple` row)» add «, a click on an open list's row reaching `click_list_row` through the same arm behind `click_lands_on_row`, an `optgroup` arm new»."
    sidecar: "`handle_click`'s arms: a single select opens or closes its list, `click_list_row`, a new `optgroup` arm."
    rationale: "A detector's own sweep past the report's patterns: the report's Pointer bullet changes the `select`, `option` and `optgroup` arms this line lists."
    basis: escher-0.3.0/chunks/2026-10-31-select-popup/report.md:14
    dependent-of: D-new-detail
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 248
    section: "## Input Validation"
    change: "Replace «(Enter and Space open nothing)» with «(Enter, and a Space with no typed text standing, open its list, whose own keys its row below states)»."
    sidecar: "Retired limit: Enter and Space open nothing on a single select."
    rationale: "The report's Retired bullet names «Enter and Space open nothing on a single select»; its Keys bullet has `press_select_key` open on Enter and on a Space with no typed text standing; `enter-space` names input-validation:248 changed."
    basis: escher-0.3.0/chunks/2026-10-31-select-popup/report.md:15
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 249
    section: "## Input Validation"
    change: "Replace «clears the typeahead and the text selection and focuses it» with «clears the typeahead and the text selection and focuses it, and on a single select opens its list or closes one standing (the list's row below)»."
    sidecar: "A click on a single select opens its list or closes one standing."
    rationale: "The report's Pointer bullet: `handle_click`'s `select` arm opens a single select's list or closes one standing, a new private `focus_select` holding what the arm did."
    basis: escher-0.3.0/chunks/2026-10-31-select-popup/report.md:14
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 249
    section: "## Input Validation"
    change: "In the stated limits, strike «no popup is painted, »."
    sidecar: "Retired limit: no popup is painted."
    rationale: "The report's Retired bullet names «no popup is painted»; `no-popup` names input-validation:249 changed."
    basis: escher-0.3.0/chunks/2026-10-31-select-popup/report.md:42
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 249
    section: "## Input Validation"
    change: "In the stated limits, strike «Enter and Space open nothing on a single select, »."
    sidecar: "Retired limit: Enter and Space open nothing on a single select, its second occurrence."
    rationale: "The same retired claim as this file's line 248; `enter-space` names input-validation:249 changed."
    basis: escher-0.3.0/chunks/2026-10-31-select-popup/report.md:43
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 249
    section: "## Input Validation"
    change: "In the stated limits, replace «`size` is not read» with «a single select's `size` is not read»."
    sidecar: "Retired limit: a `multiple` select's `size` is unread; a single select's stays unread."
    rationale: "The report's Retired bullet names «a `multiple` select's `size` is unread», and its `size` bullet keeps a single select's unread; `size-unread` names input-validation:249 changed."
    basis: escher-0.3.0/chunks/2026-10-31-select-popup/report.md:45
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 249
    section: "## Input Validation"
    change: "Replace «the driver's `type` on a single select is the same typeahead (a `multiple` select's, its own row below)» with «the driver's `type` on any select focuses with no click (its row below), on a single select the same typeahead»."
    sidecar: "The driver's `type` on any select focuses with no click."
    rationale: "The report's Driver bullet and `type-focus` name input-validation:249 changed."
    basis: escher-0.3.0/chunks/2026-10-31-select-popup/report.md:48
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 250
    section: "## Input Validation"
    change: "Add after this line a new row. Boundary: Markup attributes. What to validate: a single `select`'s open list, an engine overlay — opening, closing, its state and geometry. How (stack-specific): In every document, remote pages included: `BaseDocument`'s one new field `select_list: Option<SelectList>`, `None` at construction, holds one list at most, read through the crate-private `live_select_list` (packages/blitz-dom/src/document.rs:343-344; packages/blitz-dom/src/document.rs:797-804) but for `reconcile_select_list`, which must see a list the reader no longer answers, and the removal arm; a click, Enter or a Space with no typed text standing opens it and fires no event, and a disabled, inert, hidden, singular or empty select opens nothing (`a_disabled_inert_hidden_singular_or_empty_select_opens_nothing`), nor one under a non-invertible transform; `resolve` calls `reconcile_select_list` after layout and before `refresh_hover` (packages/blitz-dom/src/resolve.rs:133-135), so a select removed, disabled, hidden or made inert closes its list with no panic, and a page scroll or a move of the select's box closes it too; `rows` walks options and groups by closed element-name matches, at most `MAX_ROWS` 20 shown, a longer list scrolled within its rows; `place` and `geometry` keep every edge finite and the list inside the viewport, a non-finite box unpainted; a hostile `size`, 2,000 options and a 5,500-character label hold (`hostile_markup_keeps_every_box_finite_and_panics_nowhere`); `:open` and `flush_open_state` are unmoved, so an author's `open` on a select steers nothing; no log site; a key pressed after an ancestor gained `hidden` panics at `node.rs:1775`, for any control with no list opened, `select_popup` skipping that key press in that one row (§Error Handling) — an engine change for every document, carried to P2's answer card on the founder's recorded word, inputs#I4 of that chunk — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md"
    sidecar: "New row: a single select's open list, its state, opening and closing, geometry, the panic beside it named."
    rationale: "The report's `select list` coverage row brings the overlay on every document, validated by closed matches, one reader, `get_node` and finite edges; its security acceptance reads «met to its letter, with a defect beside it», so the row states the panic and does not read as panic-free."
    basis: escher-0.3.0/chunks/2026-10-31-select-popup/report.md:153
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 250
    section: "## Input Validation"
    change: "Add after this line a new row. Boundary: Markup attributes. What to validate: the open list's keys, pointer and wheel. How (stack-specific): In every document, remote pages included: `press_open_list_key` is the open list's closed key match (packages/blitz-dom/src/events/keyboard.rs:215-250), moving the highlight over enabled rows and choosing nothing (`the_keys_move_the_highlight_over_enabled_rows_and_choose_nothing`); Enter, Space and Tab choose through `choose_current_and_close` (packages/blitz-dom/src/events/keyboard.rs:252-267), Tab before focus moves, either direction (packages/blitz-dom/src/events/keyboard.rs:32-34), and a click on a row through `click_list_row` behind `click_lands_on_row`, each passing `apply_select_choice`, the one write path, one `input` then one `change`, a choice of the chosen option firing nothing and a disabled row changing nothing with the list left open; Escape, a second click and a click outside close with no event and the value unchanged (packages/blitz-dom/src/events/pointer.rs:415-420), a close clearing the typed text; `hit_with_scrollbar` answers `select_list_hit` first and no scrollbar with it (packages/blitz-dom/src/document.rs:2404-2406), so `hit`, `set_hover_to` and `handle_pointerdown` read the list, `hit_inner` not edited; a press in the list starts no text drag, the release choosing the row under it; `handle_pointermove` moves the highlight through `hover_list_row`; `handle_wheel` scrolls the list over it, else closes it and then scrolls; `scroll_into_view` on a row writes the list's `scroll_y` and moves no chain or viewport (packages/blitz-dom/src/scrolling.rs:669-672); the highlight is in no snapshot and no tree, the thumb not draggable; no key name, node key, field, cause, session error, exit, carrier, log field or span added; pinned by `select_popup` 25, `stand_act_popup` 12 and `stand_keyboard` +2 — an engine change for every document, carried to P2's answer card on the founder's recorded word, inputs#I4 of that chunk — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md"
    sidecar: "New row: the open list's closed key match, its pointer and wheel, one write path."
    rationale: "The report's Pointer, Keys, The hit and Seen through the list bullets; the plan's expected amendments owe a row for the list's keys."
    basis: escher-0.3.0/chunks/2026-10-31-select-popup/report.md:103
  - detector: D-new-detail
    severity: escalate
    file: .andromeda/security-plan-detail/input-validation.md
    near: 250
    section: "## Input Validation"
    change: "Add after this line a new row. Boundary: Markup attributes. What to validate: an open list's rows on the ratified carriers — their pixels in a `screenshot`, a row's `bounds`. How (stack-specific): The list crosses the ratified carriers as a row's own `bounds` in the snapshot and the diff — the snapshot's builder reads `select_list_bounds(node)` before the client rect (packages/dioxus-native-dom/src/snapshot.rs:152-155), `differs` naming `bounds`, a closed set inside the ratified crossing — and as pixels in a `screenshot`: every shot of an open list holds new pixel content, an option's label and a group's `label`, page text painted by `paint_select_list`, which `paint_scene` calls after the root's render and before the devtools overlays (packages/blitz-paint/src/render.rs:246-247); a shot's fields unmoved, a hosted shot equal to the in-process one (`cli_commands`), a row's crop leaving the list open (`stand_act_popup`); no log holds a label — `host_log` and `mcp_log` 0 needles at `trace`, `stand_act_spans` eight fields — and no redaction shape moved; no key name, node key, field, cause, session error, exit, carrier, listener, port, env var, log field or span added; the windowed list read and not run, what an assistive technology reads unmeasured — a boundary widening and an engine change for every document, awaiting the founder's answer card at that chunk's wrap P2 on his recorded word inputs#I4 (CARRY `cry-musoudbr`) — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md"
    sidecar: "New row: a page's option and group labels in screenshot pixels, a boundary widening for the founder's answer card."
    rationale: "The report's Coverage names the labels in pixels «boundary widening, P2's halt»; Expected amendments carry it to his answer card, so the row states it awaiting and not ratified."
    basis: escher-0.3.0/chunks/2026-10-31-select-popup/report.md:111
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 256
    section: "## Input Validation"
    change: "Replace «its non-empty `label` is its name in the tree, unpainted, inside the ratified name crossing» with «its non-empty `label` is its name in the tree, inside the ratified name crossing, and the header an open list paints over its options (the list's pixels row)»."
    sidecar: "Retired limit: a group's label is unpainted."
    rationale: "The report's Retired bullet names «a group's label is unpainted»; `group-label` names input-validation:256 changed."
    basis: escher-0.3.0/chunks/2026-10-31-select-popup/report.md:46
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 256
    section: "## Input Validation"
    change: "Replace «ratified as to the list through a group and not as to the `label` as a name» with «ratified as to the list through a group, the `label` as a name settled by the founder's F7 (inputs#I4 of escher-0.3.0/chunks/2026-10-31-select-popup)»."
    sidecar: "A group's `label` as a name, settled by the founder's F7."
    rationale: "The report's Expected amendments: his F7 settles it, so input-validation:256's «not as to the `label` as a name» ends."
    basis: escher-0.3.0/chunks/2026-10-31-select-popup/report.md:101
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 261
    section: "## Input Validation"
    change: "Replace «the driver's `type` on a `multiple` select» with «the driver's `type` on a select»."
    sidecar: "The `type` row now covers any select."
    rationale: "`type-focus` names input-validation:261 changed: `type` on any select focuses with no click."
    basis: escher-0.3.0/chunks/2026-10-31-select-popup/report.md:48
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 262
    section: "## Input Validation"
    change: "Replace «`focus_multiple_select` focuses with no click, which would land on the row at the centre, and clears the typeahead» with «`focus_select` focuses with no click, which on a `multiple` select would land on the row at the centre, so it fires no DOM focus events on a single select either, clears the typeahead and closes any list»."
    sidecar: "Retired name: `focus_multiple_select` is `focus_select`, which also closes any list."
    rationale: "The report's Retired bullet names `focus_multiple_select`; its Driver bullet states the rename and what the new function does."
    basis: escher-0.3.0/chunks/2026-10-31-select-popup/report.md:20
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 262
    section: "## Input Validation"
    change: "Replace «a text opening on a space types nothing, since no label can match and the Space would flip a row» with «a text opening on a space types nothing on either kind, since no label can match and on a `multiple` select the Space would flip a row»."
    sidecar: "A `type` text opening on a space types nothing on a single select too."
    rationale: "The report's Driver bullet: a text opening on a space types nothing there, on a single select; `type-focus` names input-validation:262 changed."
    basis: escher-0.3.0/chunks/2026-10-31-select-popup/report.md:20
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 262
    section: "## Input Validation"
    change: "Replace «`Target` holds `multiple`» with «`Target` holds `select`»."
    sidecar: "Retired name: `Target.multiple` is `Target.select`."
    rationale: "The report's Retired bullet names `Target.multiple`; its Driver bullet states `Target.multiple` → `Target.select`."
    basis: escher-0.3.0/chunks/2026-10-31-select-popup/report.md:20
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 264
    section: "## Input Validation"
    change: "Replace «`dioxus_native_dom::closed_select_of` · `is_multiple_select`» with «`dioxus_native_dom::closed_select_of` · `is_multiple_select` · `is_select`»."
    sidecar: "New public reader `is_select` joins the select readers' row."
    rationale: "The report's Bridge bullet: new public `is_select`, re-exported; `focus-select` names input-validation:264 changed."
    basis: escher-0.3.0/chunks/2026-10-31-select-popup/report.md:19
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 265
    section: "## Input Validation"
    change: "After «`closed_select_of`'s callers are the snapshot's builder and `lint.rs`» add «, and it answers `None` for a select whose list is open, so an open list's rows read their own boxes»."
    sidecar: "`closed_select_of` answers `None` for a select whose list is open."
    rationale: "The report's Bridge bullet; `closed-option` names input-validation:265 changed."
    basis: escher-0.3.0/chunks/2026-10-31-select-popup/report.md:19
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 265
    section: "## Input Validation"
    change: "Replace «`is_multiple_select`'s `closed_select_of` and `execute.rs` `locate`» with «`is_multiple_select`'s `closed_select_of` alone within the workspace, still re-exported, `is_select`'s `execute.rs` `locate` alone»."
    sidecar: "`locate` reads `is_select`; `is_multiple_select` keeps one caller."
    rationale: "The report's Driver bullet states each reader's remaining callers (`git grep -n` at HEAD); `focus-select` names input-validation:265 changed."
    basis: escher-0.3.0/chunks/2026-10-31-select-popup/report.md:20
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 265
    section: "## Input Validation"
    change: "Replace «`execute.rs` `focus_multiple_select`» with «`execute.rs` `focus_select`»."
    sidecar: "Retired name: `focus_multiple_select`, its second occurrence."
    rationale: "The same retired name as this file's line 262; the report's Driver bullet: `clear_select_typeahead`'s one non-test caller is `focus_select`."
    basis: escher-0.3.0/chunks/2026-10-31-select-popup/report.md:20
    dependent-of: D-retired
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 265
    section: "## Input Validation"
    change: "Add after this line a new row. Boundary: Markup attributes. What to validate: the public list readers `BaseDocument::open_select` · `select_list_bounds` · `select_list_view` · `select_row_layout` · `close_select_list`, and the read types `SelectListView` · `SelectListRow`. How (stack-specific): Each goes through `get_node`, a row with no styles reading `None` and no `layout_style()` call; all read-only but `close_select_list`, whose callers are keyboard, pointer, reconcile and the driver's `focus_select`; `SelectListRow.label` is page text in a read type, never logged; `SelectListView`'s colours are associated constants, `HIGHLIGHT_TEXT` · `HIGHLIGHT_BACKGROUND` · `DISABLED_TEXT` · `BORDER` · `BORDER_WIDTH`, no author input; both types re-exported from the crate root; pinned by `select_list`'s unit tests and `select_popup` (packages/blitz-dom/src/document.rs:881-889; packages/blitz-dom/src/document.rs:990-993; packages/blitz-dom/src/document.rs:995-1012; packages/blitz-dom/src/document.rs:1022-1052; packages/blitz-dom/src/document.rs:1054-1064; packages/blitz-dom/src/node/select_list.rs:78-91; packages/blitz-dom/src/node/select_list.rs:93-108; packages/blitz-dom/src/lib.rs:108-111) — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md"
    sidecar: "New row: the public list readers and read types, through `get_node`, a label never logged."
    rationale: "The report's Coverage row `BaseDocument public readers`: validation through `get_node`, a row with no styles `None`, no `layout_style()`, a label in a read type never logged."
    basis: escher-0.3.0/chunks/2026-10-31-select-popup/report.md:114
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 284
    section: "## Input Validation"
    change: "After «with no transform on the chain it reads the padding box in the client rect's frame, and a mapped box with a non-finite edge answers `None`» add «; for a row or a group of an open list it answers the list's scrollport (packages/blitz-dom/src/scrolling.rs:754-756), and `lint.rs` `clip_region` takes an open row's region from it alone, no `contain: paint` cut»."
    sidecar: "`visible_region` answers the list's scrollport for a row or group of an open list."
    rationale: "A detector's own sweep past the report's patterns, none of which hit security-plan here: the Seen through the list and Driver bullets add this answer to the reader this line describes."
    basis: escher-0.3.0/chunks/2026-10-31-select-popup/report.md:17
note: "D-security-input finds every new input validated, and D-security-auth and D-security-deps find nothing (Dependencies none); lines 248 and 249 stand over 2 000 chars and stay so, needing a break at a clause; the panic row is error-handling.md's, not proposed here; the pixels row is unratified until P2's card; the report states no lookup rule for `is_select`, so line 265's `get_node` sentence is unmeasured for it; .andromeda/security-plan.md:133 and :135 lie outside this file and name neither `size` nor the pixels."
