proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 219
    section: "## Input Validation"
    change: "In the How, replace «`aria-expanded` a boolean,» with «`aria-expanded` a boolean on every element but a single select, which since 2026-10-31-select-popup writes `set_expanded(open)` from the engine's own state and reads no `aria-expanded`,»."
    sidecar: "input-validation: a single select's `expanded` is engine state, no `aria-expanded` read."
    rationale: "Report, Symbols / APIs, Tree: a single select writes `set_expanded(open)` and reads no `aria-expanded`; every other element keeps that read. The `expanded` pattern marks input-validation:219 changed."
    basis: "packages/blitz-dom/src/accessibility.rs:238-242"
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 219
    section: "## Input Validation"
    change: "Add after this row, in the group's shape: Boundary: Markup attributes · What to validate: an author's `open` and `aria-expanded` on a select · How (stack-specific): In every document, remote pages included: an author's `open` on a select and `aria-expanded` on a single select steer nothing: a single select's `expanded` is the closed bool of the engine's own state, and `:open` and `flush_open_state` are unmoved, so `open` on a select does not match `:open`; no attribute text is carried out; pinned by `an_author_aria_expanded_and_open_steer_nothing_on_a_select_and_open_does_not_match` (packages/blitz-dom/src/accessibility.rs:238-242; tests/blitz-tests/tests/select_popup.rs:628-651) — an engine change for every document — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md"
    sidecar: "input-validation: new row, an author's `open` and `aria-expanded` steer nothing on a select."
    rationale: "F10, on the report's Tree bullet. The report drops the `aria-expanded` read on a single select alone («every other element keeps that read»), so the row narrows that half of F10 to a single select rather than saying any select."
    basis: "tests/blitz-tests/tests/select_popup.rs:628-651"
    fact: F10
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 230
    section: "## Input Validation"
    change: "Add after this row, in the group's shape: Boundary: Markup attributes · What to validate: which select opens a list, and what a row takes · How (stack-specific): In every document, remote pages included: a disabled, inert or hidden select and a select with no row to show open no list, and a non-invertible transform is held by the same check set, `select_list_allowed`; a disabled row changes nothing and leaves the list open; a group header chooses nothing; pinned by `a_disabled_inert_hidden_singular_or_empty_select_opens_nothing`, `a_disabled_row_changes_nothing_and_leaves_the_list_open` and `groups_read_as_headers_their_options_under_them_and_a_header_chooses_nothing`, both layout modes (packages/blitz-dom/src/document.rs:761-795; tests/blitz-tests/tests/select_popup.rs:377-388; tests/blitz-tests/tests/select_popup.rs:443-472; tests/blitz-tests/tests/select_popup.rs:805-855) — an engine change for every document — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md"
    sidecar: "input-validation: new row, the selects that open no list and the rows that choose nothing."
    rationale: "F5, on the report's Outcome (security) acceptance and Deviations («a select with no row to show opens nothing»). One proposal, placed beside `inert`; the `hit_inner` transform row keeps its claim."
    basis: "tests/blitz-tests/tests/select_popup.rs:443-472"
    fact: F5
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 242
    section: "## Input Validation"
    change: "Replace «a click on a `select` focusing it (the `select` row), a click on a `multiple` select's row reaching `click_select_row` through an `option` arm (the `select` with `multiple` row),» with «a click on a `select` focusing it and, on a single select, opening its list or closing one standing (the `select` row), a click on a `multiple` select's row reaching `click_select_row` and a click on an open list's row `click_list_row` through an `option` arm (the `select` with `multiple` row and the select list's rows),»."
    sidecar: "input-validation: `handle_click`'s summary of the `select` and `option` arms restated."
    rationale: "Report, Symbols / APIs, Pointer: `handle_click`'s `select` arm opens a single select's list or closes one standing; its `option` arm gains `click_list_row` behind `click_lands_on_row`. The sweep's patterns did not reach this wording."
    basis: "packages/blitz-dom/src/events/pointer.rs:642-655"
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 248
    section: "## Input Validation"
    change: "Replace «on a focused single-choice select that `ElementData::is_disabled` does not read disabled, a pressed» with «on a focused single-choice select that `ElementData::is_disabled` does not read disabled, its list closed, a pressed», and «a `Key::Character` with no Ctrl, Alt or Meta is typeahead and every other key does nothing (Enter and Space open nothing) — a closed key match» with «a `Key::Character` with no Ctrl, Alt or Meta is typeahead, Enter opens its list, a Space with no typed text standing opens it (`press_select_key`) and every other key does nothing — a closed key match»."
    sidecar: "input-validation: Enter and Space open a single select's list; the limit retired."
    rationale: "Report, Retired names «Enter and Space open nothing on a single select»; Keys: `press_select_key` opens on Enter, and on a Space with no typed text standing; `press_open_list_key` holds the open list's keys, so the arrows here read a closed list. The `enter-space` pattern marks input-validation:248 changed."
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 249
    section: "## Input Validation"
    change: "Replace «a click on a select, past `handle_click`'s `is_disabled` walk gate, clears the typeahead and the text selection and focuses it» with «a click on a select, past `handle_click`'s `is_disabled` walk gate, clears the typeahead and the text selection and focuses it, and on a single select opens its list or closes one standing»."
    sidecar: "input-validation: a click on a single select opens or closes its list."
    rationale: "Report, Pointer: the `select` arm opens a single select's list or closes one standing; Outcome (arch): two `click`s open then close, each focusing."
    basis: "tests/blitz-tests/tests/stand_act_popup.rs:379-406"
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 249
    section: "## Input Validation"
    change: "Replace «the driver's `type` on a single select is the same typeahead (a `multiple` select's, its own row below), its help one fixed sentence more» with «the driver's `type` on a single select is the same typeahead, reached through `focus_select` with no click, so it fires no DOM focus events and leaves no list open, a text opening on a space typing nothing there (a `multiple` select's, its own row below), its help fixed text»."
    sidecar: "input-validation: `type` on a single select focuses with no click; its help restated."
    rationale: "Report, Driver: `type` on any select focuses with no click, so it fires no DOM focus events on a single select either, and a text opening on a space types nothing there; `focus_select` also closes any list; `type`'s fixed help restated. The `type-focus` row marks input-validation:249 changed."
    basis: "tests/blitz-tests/tests/stand_act_popup.rs:247-285"
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 249
    section: "## Input Validation"
    change: "Replace «stated limits: no popup is painted, Enter and Space open nothing on a single select, an option of a closed single select has no click arm and stays refused `covered`, under a group too, and `size` is not read;» with «stated limits: an option of a closed single select has no click arm and stays refused `covered`, under a group too, and a single select's `size` is not read;»."
    sidecar: "input-validation: the select row's retired limits end; the closed-option limit stands."
    rationale: "Report, Retired: «no popup is painted», «Enter and Space open nothing on a single select», «a `multiple` select's `size` is unread». Its by-line row for input-validation:249 reads `no-popup`, `enter-space`, `size-unread` changed and `closed-option` true; `size`: a single select's stays unread."
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 250
    section: "## Input Validation"
    change: "Add after this row, in the group's shape: Boundary: Markup attributes · What to validate: the select list's state · How (stack-specific): In every document, remote pages included: the list is one `BaseDocument` field, `select_list`, `None` at construction, read through `live_select_list`, except `reconcile_select_list`, which must see a list that reader no longer answers, and the removal arm, which read the field past it, and `hover_list_row`, which writes `current` directly (packages/blitz-dom/src/document.rs:343-344; packages/blitz-dom/src/document.rs:797-804; packages/blitz-dom/src/document.rs:894; packages/blitz-dom/src/document.rs:1408; packages/blitz-dom/src/events/pointer.rs:604) — an engine change for every document — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md"
    sidecar: "input-validation: new row, the select list's one field and who reads it past its reader."
    rationale: "F4, on the report's `BaseDocument` bullet and its Deviations."
    basis: "packages/blitz-dom/src/document.rs:343-344"
    fact: F4
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 250
    section: "## Input Validation"
    change: "Add after this row, in the group's shape: Boundary: Markup attributes · What to validate: an open select list's keys · How (stack-specific): In every document, remote pages included: an open list's keys pass one closed match, `press_open_list_key`; Tab runs `choose_current_and_close` before focus moves, in either direction; `click_list_row` and `choose_current_and_close` write only through `apply_select_choice`, the one write path; pinned by `the_keys_move_the_highlight_over_enabled_rows_and_choose_nothing`, `enter_space_tab_and_a_click_choose_the_row_one_input_then_one_change` and `tab_chooses_and_closes_before_focus_moves_either_way`, both layout modes (packages/blitz-dom/src/events/keyboard.rs:32-34; packages/blitz-dom/src/events/keyboard.rs:215-250; packages/blitz-dom/src/events/keyboard.rs:252-267; packages/blitz-dom/src/events/pointer.rs:642-655; tests/blitz-tests/tests/select_popup.rs:249-273; tests/blitz-tests/tests/select_popup.rs:294-361) — an engine change for every document — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md"
    sidecar: "input-validation: new row, an open list's closed key match and its one write path."
    rationale: "F6, on the report's Keys bullet and Outcome (arch) the one write path."
    basis: "packages/blitz-dom/src/events/keyboard.rs:215-250"
    fact: F6
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 250
    section: "## Input Validation"
    change: "Add after this row, in the group's shape: Boundary: Markup attributes · What to validate: a pointer in and around an open select list, and what closes it · How (stack-specific): In every document, remote pages included: in the list a press starts no text drag and the release chooses the row under it; a press elsewhere closes it through `close_list_outside`; a page scroll or a box move closes it through `reconcile_select_list` at the next resolve, the list's `fresh` field keeping a list a click just opened through a `:focus` rule's restyle; pinned by `a_press_on_a_row_selects_no_text_and_a_drag_across_rows_chooses_where_it_ends`, `escape_a_second_click_a_click_outside_and_the_page_scrolling_close_with_no_event` and `a_focus_rule_that_moves_the_selects_box_leaves_the_list_a_click_opened`, both layout modes (packages/blitz-dom/src/events/pointer.rs:415-420; packages/blitz-dom/src/events/pointer.rs:613-633; packages/blitz-dom/src/document.rs:891-907; packages/blitz-dom/src/resolve.rs:133-135; tests/blitz-tests/tests/select_popup.rs:401-426; tests/blitz-tests/tests/select_popup.rs:783-803; tests/blitz-tests/tests/select_popup.rs:1105-1122) — an engine change for every document — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md"
    sidecar: "input-validation: new row, a press in the list, a press outside it and what closes it."
    rationale: "F7, on the report's Pointer and resolve bullets. The `fresh` clause is the report's Deviations: without it a box-moving `:focus` rule would read as a move and close the list at once, so the bare «a box move closes it» would overstate."
    basis: "packages/blitz-dom/src/document.rs:891-907"
    fact: F7
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 250
    section: "## Input Validation"
    change: "Add after this row, in the group's shape: Boundary: Markup attributes · What to validate: hostile markup under an open select list · How (stack-specific): In every document, remote pages included: a hostile `size`, 2,000 options and a 5,500-character label keep every box finite and panic nowhere, asserted by `hostile_markup_keeps_every_box_finite_and_panics_nowhere`; the list shows at most `MAX_ROWS` 20 rows, its rows past that scrolled, pinned by `a_long_list_shows_twenty_rows_and_scrolls_by_key_and_by_wheel`, both layout modes (tests/blitz-tests/tests/select_popup.rs:870-911; tests/blitz-tests/tests/select_popup.rs:938-975) — an engine change for every document — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md"
    sidecar: "input-validation: new row, hostile markup bounded by the select list."
    rationale: "F9, on the report's Outcome (security) acceptance and `MAX_ROWS` = 20 in Symbols / APIs."
    basis: "tests/blitz-tests/tests/select_popup.rs:938-975"
    fact: F9
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 250
    section: "## Input Validation"
    change: "Add after this row, in the group's shape: Boundary: Markup attributes · What to validate: the select list's highlighted row · How (stack-specific): The highlighted row crosses no structured carrier: it is in no snapshot and no tree, and the lint does not read its contrast; a stated limit (docs/capabilities.md:208-210) — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md"
    sidecar: "input-validation: new row, the highlighted row crosses no structured carrier."
    rationale: "F11, on the report's Coverage (`tree expanded on a single select`: the highlight unread by the tree) and inputs#I7's limits: a highlighted row's contrast unread by the lint, the highlight absent from the snapshot and the tree."
    basis: "docs/capabilities.md:208-210"
    fact: F11
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 253
    section: "## Input Validation"
    change: "Add after this row, in the group's shape: Boundary: Markup attributes · What to validate: `size` on a `multiple` select · How (stack-specific): In every document, remote pages included: `size_rows` parses it through `parse_unsigned_integer`, capped at 65534, `0` or a non-numeric value giving no hint, and hints a `height` of `size` × 1.2em, carrying no text out; `default.css` gives such a select `overflow-y: auto` and its rows `line-height: 1.2`, and a key brings the current row into view only where `carries_valid_size`; a single select's `size` stays unread; pinned by `size_reads_a_whole_number_above_zero_capped`, `hostile_markup_keeps_every_box_finite_and_panics_nowhere` and `size_makes_a_multiple_select_that_many_rows_tall_and_a_key_brings_the_next_in`, both layout modes (packages/blitz-dom/src/node/select_list.rs:125-130; packages/blitz-dom/src/stylo.rs:1124-1139; packages/blitz-dom/assets/default.css:183-193; packages/blitz-dom/src/events/keyboard.rs:423-430; packages/blitz-dom/src/node/select_list.rs:459-475; tests/blitz-tests/tests/select_popup.rs:995-1032) — an engine change for every document — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md"
    sidecar: "input-validation: new row, `size` on a `multiple` select, a capped whole number."
    rationale: "F8, on the report's `size` bullet and Coverage (`size on a multiple select`). The report's retired «a `multiple` select's `size` is unread» ends at this row's neighbour, the `select` row."
    basis: "packages/blitz-dom/src/stylo.rs:1124-1139"
    fact: F8
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 256
    section: "## Input Validation"
    change: "Replace «its non-empty `label` is its name in the tree, unpainted, inside the ratified name crossing; pinned by `select_interaction`, `stand_act_select` and `form`'s tests» with «its non-empty `label` is its name in the tree, inside the ratified name crossing, and since 2026-10-31-select-popup a header painted in an open list, leaving also as pixels in a `screenshot`; pinned by `select_interaction`, `stand_act_select`, `select_popup`, `select_popup_painted` and `form`'s tests»."
    sidecar: "input-validation: a group's `label` is painted as a header; the limit retired."
    rationale: "Report, Retired: «a group's label is unpainted»; the `group-label` pattern marks input-validation:256 changed; Schema / config: new pixel content in every shot of an open list. Those pixels are the boundary widening P2 halts on (inputs#I4, CARRY `cry-musoudbr`), so this text claims no ratification of them."
    basis: "tests/blitz-tests/tests/select_popup_painted.rs:282-297"
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 256
    section: "## Input Validation"
    change: "Replace «ratified as to the list through a group and not as to the `label` as a name» with «ratified as to the list through a group, the `label` as a name settled by the founder's F7 at 2026-10-31-select-popup (inputs#I4)»."
    sidecar: "input-validation: the clause reserving a group's `label` as a name ends on his F7."
    rationale: "Report, Expected amendments, Playbook: the group's `label` as a name is settled by his F7 (inputs#I4), so input-validation:256's «not as to the `label` as a name» ends."
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 261
    section: "## Input Validation"
    change: "Replace «the driver's `type` on a `multiple` select» with «the driver's `type` on any select»."
    sidecar: "input-validation: the `type` row now names any select."
    rationale: "Report, Driver: `type` on any select focuses with no click; the `type-focus` pattern marks input-validation:261 changed."
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 262
    section: "## Input Validation"
    change: "Replace «`focus_multiple_select` focuses with no click, which would land on the row at the centre, and clears the typeahead;» with «`focus_select` focuses any select with no click, which on a `multiple` select would land on the row at the centre, so it fires no DOM focus events on a single select either, clears the typeahead and closes any list;»."
    sidecar: "input-validation: `focus_multiple_select` is `focus_select`, which also closes any list."
    rationale: "Report, Retired: `focus_multiple_select`; Driver: `focus_multiple_select` → `focus_select`, which also closes any list; `type` on any select fires no DOM focus events on a single select either. `type-focus` and `focus-select` mark input-validation:262 changed."
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 262
    section: "## Input Validation"
    change: "Replace «a text opening on a space types nothing, since no label can match and the Space would flip a row;» with «a text opening on a space types nothing on any select, since no label can match and on a `multiple` select the Space would flip a row;»."
    sidecar: "input-validation: a text opening on a space types nothing on any select."
    rationale: "Report, Driver: a text opening on a space types nothing there, a single select too; the flip of a row is a `multiple` select's alone."
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 262
    section: "## Input Validation"
    change: "Replace «`Target` holds `multiple`,» with «`Target` holds `select`,»."
    sidecar: "input-validation: `Target.multiple` is `Target.select`."
    rationale: "Report, Retired: `Target.multiple`; Driver: `Target.multiple` → `Target.select`."
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 264
    section: "## Input Validation"
    change: "Replace «`dioxus_native_dom::closed_select_of` · `is_multiple_select`» with «`dioxus_native_dom::closed_select_of` · `is_multiple_select` · `is_select`»."
    sidecar: "input-validation: the new public `is_select` joins the select readers."
    rationale: "Report, Bridge: new public `is_select`, re-exported; the `focus-select` pattern marks input-validation:264 changed."
    basis: "packages/dioxus-native-dom/src/lib.rs:37"
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 265
    section: "## Input Validation"
    change: "Replace «`closed_select_of`'s callers are the snapshot's builder and `lint.rs`,» with «`closed_select_of` answers `None` for a select whose list is open, and its callers are the snapshot's builder and `lint.rs`, whose `Entry.in_select_box` reads true also for an open list's row or group, `clip_region` taking an open row's region from `visible_region` alone, an option of a closed select still refused `covered`,»."
    sidecar: "input-validation: `closed_select_of`'s open answer and `Entry.in_select_box`."
    rationale: "Report, Bridge: `closed_select_of` answers `None` for a select whose list is open; Driver: `Entry.in_closed_select` → `in_select_box`, true also for an open list's row or group. `closed-option` marks input-validation:265 changed."
    basis: "packages/escher-driver/src/lint.rs:339-342"
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 265
    section: "## Input Validation"
    change: "Replace «`is_multiple_select`'s `closed_select_of` and `execute.rs` `locate`,» with «`is_multiple_select`'s `closed_select_of` alone within the workspace, it stays re-exported, `is_select`'s `execute.rs` `locate` alone,»."
    sidecar: "input-validation: `locate` reads `is_select`; `is_multiple_select` keeps one caller."
    rationale: "Report, Driver, remaining callers: `is_select` — `execute.rs` `locate` alone; `is_multiple_select` — `closed_select_of` alone within the workspace, still re-exported."
    basis: "packages/dioxus-native-dom/src/lib.rs:37"
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 265
    section: "## Input Validation"
    change: "Replace «`clear_select_typeahead`'s one non-test caller `execute.rs` `focus_multiple_select`» with «`clear_select_typeahead`'s one non-test caller `execute.rs` `focus_select`»."
    sidecar: "input-validation: `clear_select_typeahead`'s caller renamed."
    rationale: "Report, Retired: `focus_multiple_select`; Driver: `clear_select_typeahead` — one non-test caller, `focus_select`."
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 265
    section: "## Input Validation"
    change: "Add after this row, in the group's shape: Boundary: Markup attributes · What to validate: the select list's public surface, `BaseDocument::open_select` · `select_list_bounds` · `select_list_view` · `select_row_layout` · `close_select_list`, the read types `SelectListView` · `SelectListRow` · How (stack-specific): The four readers are read-only and `close_select_list` the one public writer, its callers keyboard, pointer, reconcile and the driver's `focus_select`; the two read types are re-exported from the crate root; `SelectListRow.label` holds page text and is never logged (packages/blitz-dom/src/document.rs:881-889; packages/blitz-dom/src/document.rs:990-1012; packages/blitz-dom/src/document.rs:1022-1064; packages/blitz-dom/src/node/select_list.rs:78-108; packages/blitz-dom/src/lib.rs:108-111) — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md"
    sidecar: "input-validation: new row, the select list's public readers, writer and read types."
    rationale: "F12, on the report's `BaseDocument` bullet and Coverage (`BaseDocument public readers`). `close_select_list` is named a writer there, so the row does not call it a reader."
    basis: "packages/blitz-dom/src/lib.rs:108-111"
    fact: F12
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 284
    section: "## Input Validation"
    change: "Replace «a mapped box with a non-finite edge answers `None`;» with «a mapped box with a non-finite edge answers `None`; for a row or a group of an open select list it answers the list's scrollport instead;», and «`lint.rs` `clip_region`;» with «`lint.rs` `clip_region`, which takes an open row's region from it alone, no `contain: paint` cut;»."
    sidecar: "input-validation: `visible_region` answers the list's scrollport for an open list's row or group."
    rationale: "Report, Seen through the list: `visible_region` answers the list's scrollport for a row or a group of an open list; Driver: `clip_region` takes an open row's region from `visible_region` alone. The sweep's `visible-reg` listing reaches other masters' wording, and this row's «reads each clipping box» no longer covers such a node."
    basis: "packages/blitz-dom/src/scrolling.rs:754-756"
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 287
    section: "## Input Validation"
    change: "Add after this row, in the group's shape: Boundary: Paint values · What to validate: the select list's box and rows · How (stack-specific): `paint_scene` calls `paint_select_list` after the root's render and before the devtools overlays; a non-finite box is left unpainted and only rows in the scrollport are laid out; colours are the engine's own constants, no page value (packages/blitz-paint/src/render.rs:246-247; packages/blitz-paint/src/render/select_list.rs:21-52; packages/blitz-dom/src/node/select_list.rs:110-123) — an engine change for every document — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md"
    sidecar: "input-validation: new row, the select list's paint values."
    rationale: "Report, Paint: a non-finite box unpainted, only rows in the scrollport laid out; Coverage: every edge finite, tokens hardcoded as engine constants on his F6. Report's Expected amendments owe §Input Validation new rows; no `## New facts` item holds this one."
    basis: "packages/blitz-paint/src/render.rs:246-247"
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 302
    section: "## Input Validation"
    change: "Add after this row, in the group's shape: Boundary: Scrolling · What to validate: an open select list's own scroll · How (stack-specific): `scroll_into_view` on a row writes the list's `scroll_y` at once and returns, moving no chain or viewport; a wheel over the list scrolls it, and a wheel elsewhere closes it and then scrolls; its offset stays within its rows (`Geometry::clamped`); pinned by `a_scroll_stays_within_the_rows_and_brings_a_row_in_by_the_least_movement` and `a_long_list_shows_twenty_rows_and_scrolls_by_key_and_by_wheel` (packages/blitz-dom/src/scrolling.rs:669-672; packages/blitz-dom/src/node/select_list.rs:296-304; packages/blitz-dom/src/node/select_list.rs:496-519; tests/blitz-tests/tests/select_popup.rs:870-911) — an engine change for every document — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md"
    sidecar: "input-validation: new row, an open list's own scroll and a wheel's close."
    rationale: "Report, Seen through the list and Pointer's `handle_wheel`; this row's page-scroll claims hold, and no `## New facts` item carries the list's own."
    basis: "packages/blitz-dom/src/scrolling.rs:669-672"
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan.md
    near: 133
    section: "## Input Validation"
    change: "Replace «`rowspan` clamped to 1..=65534, `<ol start>` parsed as `usize`.» with «`rowspan` clamped to 1..=65534, `size` on a `multiple` select capped at 65534 with `0` or a bad value no hint, `<ol start>` parsed as `usize`.»."
    sidecar: "security-plan: the numeric-attributes line names `size` on a `multiple` select."
    rationale: "Report, `size`: `parse_unsigned_integer`, capped at 65534, `0` or a bad value no hint. The broad line over F8's row."
    basis: "packages/blitz-dom/src/stylo.rs:1124-1139"
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan.md
    near: 135
    section: "## Input Validation"
    change: "Replace «Enter activating buttons and links and Space buttons as a click, names and descriptions as page text;» with «Enter activating buttons and links and Space buttons as a click, Enter and Space opening a single select's engine-painted list, names and descriptions as page text, an option's and a group's label also painted in an open list;»."
    sidecar: "security-plan: the Markup attributes line names the select list and its painted labels."
    rationale: "Report, Keys and Schema / config: Enter and Space open a single select's list, new pixel content an option's label and a group's `label`. The line's «ratified carriers» is left as it stands: the pixels are P2's halt (inputs#I4, CARRY `cry-musoudbr`)."
note: Detectors read — D-security-input no drift (the report's new `size`, keys and readers each state their validation), D-security-auth and D-security-deps not reached (no auth surface, no dependency); the new rows and the two pixel edits state no ratification since the painted labels are P2's halt on inputs#I4; the report's `label-click` at input-validation:210 is no entity of this pick, so it is the orchestrator's to place.
