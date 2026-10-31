proposals:
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 78
    section: "## Established Decisions"
    change: "Replace «every other key does nothing — Enter and Space open nothing, and a released Space activates no select» with «a pressed Enter, or a Space with no typed text standing, opens its list (below), every other key does nothing, and a released Space does nothing more»."
    sidecar: "Select model: Enter and a Space with no typed text standing now open a single select's list; the limit «Enter and Space open nothing» ended."
    rationale: "The report's Retired bullet names «Enter and Space open nothing on a single select»; Symbols / APIs: `press_select_key` opens on Enter and on a Space with no typed text standing; fact `enter-space` lists established-decisions:78 changed; the renamed test reads `enter_and_space_open_the_list_and_a_released_space_does_nothing_more`."
    basis: .andromeda/architecture-detail/established-decisions.md:78
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 78
    section: "## Established Decisions"
    change: "Delete «no popup is painted (owner: the route entry «Select popup»), » from the sentence opening «Its limits:»; the painted list and its own limits stand in the new items below this model."
    sidecar: "Select model: the limit «no popup is painted» and its route owner dropped, the select list having landed."
    rationale: "The report's Retired bullet names «no popup is painted»; fact `no-popup` lists established-decisions:78 changed, and the Sweep reach row `owner-title` lists established-decisions:78 as dropping the owner phrase."
    basis: .andromeda/architecture-detail/established-decisions.md:78
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 78
    section: "## Established Decisions"
    change: "Replace «`size` is unread,» with «a single select's `size` is unread, a `multiple` select's read ([DOM semantics]),»."
    sidecar: "Select model: `size` is now read on a `multiple` select; a single select's stays unread."
    rationale: "The report's Retired bullet names «a `multiple` select's `size` is unread»; Symbols / APIs **`size`**: «A single select's `size` stays unread»; fact `size-unread` lists established-decisions:78 changed."
    basis: .andromeda/architecture-detail/established-decisions.md:78
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 121
    section: "## Established Decisions"
    change: "Replace «named by its non-empty `label`, which stays unpainted» with «named by its non-empty `label`, which an open list paints as the group's header ([Scrolling and input], [Painting])»."
    sidecar: "`optgroup`: a group's `label` is now painted as an open list's header."
    rationale: "The report's Retired bullet names «a group's label is unpainted»; Schema / config: «New pixel content in every shot of an open list: an option's label and a group's `label`, painted»; fact `group-label` lists established-decisions:121 changed."
    basis: .andromeda/architecture-detail/established-decisions.md:121
  - detector: D-arch-decisions
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 78
    section: "## Established Decisions"
    change: "Replace «a focused single-choice `select` (no `multiple`) that is no disabled form control takes» with «a focused single-choice `select` (no `multiple`) with no list open that is no disabled form control takes»; an open list's keys move a highlight and choose nothing (below)."
    sidecar: "Select model: the arrow, Home, End and typeahead choice now reads for a single select with no list open."
    rationale: "Symbols / APIs: `press_open_list_key` is the open list's closed key match, so the line's chosen-option keys no longer hold while a list stands; `select_popup`'s `the_keys_move_the_highlight_over_enabled_rows_and_choose_nothing`."
    basis: .andromeda/architecture-detail/established-decisions.md:78
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 78
    section: "## Established Decisions"
    change: "Replace «and focuses it through `generate_focus_events` / `set_focus_to`;» with «, focuses it through `generate_focus_events` / `set_focus_to` and opens a single select's list or closes one standing (below);»."
    sidecar: "Select model: a click on a single select now opens its list or closes one standing."
    rationale: "Symbols / APIs, Pointer: `handle_click`'s `select` arm opens a single select's list or closes one standing, and a new private `focus_select` holds what the arm did; `two_clicks_on_one_point_open_and_close_each_focusing`."
    basis: .andromeda/architecture-detail/established-decisions.md:78
  - detector: D-arch-decisions
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 63
    section: "## Established Decisions"
    change: "After «and `scroll_into_view` writes every scrolling box on its target's containing-block chain, innermost first, and then the viewport» add «(a row of an open select list excepted, below)»."
    sidecar: "`scroll_into_view`: a row of an open select list is the exception to the chain walk."
    rationale: "Symbols / APIs, Seen through the list: `scroll_into_view` on a row writes the list's `scroll_y` at once and returns, no chain or viewport moving (`scrolling.rs` 669-672)."
    basis: .andromeda/architecture-detail/established-decisions.md:63
  - detector: D-arch-decisions
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 68
    section: "## Established Decisions"
    change: "Before «and a click on any other element still clears focus» add «a click on a `label` bound to a single select focuses that select and opens nothing through the walk's `label` arm, a row or a group header of an open list takes the walk's `option` or `optgroup` arm (below),»."
    sidecar: "`handle_click`: the `label` arm on a bound single select and the new `optgroup` arm named beside «any other element»."
    rationale: "Symbols / APIs, Pointer: the `option` arm gains `click_list_row`, a new `optgroup` arm stands, and the `label` arm focuses a bound single select and opens nothing; fact `label-click`; `a_labels_click_focuses_the_select_and_opens_nothing`."
    basis: .andromeda/architecture-detail/established-decisions.md:68
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 88
    section: "## Established Decisions"
    change: "After «(`events/pointer.rs`, `hit_with_scrollbar` while `scrollbar_opacity > 0`)» add «, and a row of an open select list is never so taken, `hit_with_scrollbar` answering `select_list_hit` first and no scrollbar with it»."
    sidecar: "Overlay scrollbar hazard: a row of an open select list is never a thumb drag; the hazard itself stands."
    rationale: "Fact `hit-first` lists established-decisions:88's neighbour clause, a row never a thumb drag; Symbols / APIs, The hit (`document.rs` 2404-2406)."
    basis: .andromeda/architecture-detail/established-decisions.md:88
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 81
    section: "## Established Decisions"
    change: "New item after line 81: «- The select list stands beside the select model, a direct engine change for every document, remote pages included, on the founder's ten forks, all A (inputs#I4): a closed single select opens a list the engine paints over the page, past an `opacity` ancestor and a box that clips its select. Its state is one `BaseDocument` field, `select_list: Option<SelectList>`, `None` at construction, read through the crate-private `live_select_list`, past it only by `reconcile_select_list` and the removal arm and written directly by `hover_list_row` (packages/blitz-dom/src/document.rs:343-344; packages/blitz-dom/src/document.rs:797-804; packages/blitz-dom/src/document.rs:1408-1410; packages/blitz-dom/src/node/select_list.rs:28-46). It shows at most `MAX_ROWS` = 20 rows, opens where its rows fit and else on the side with more room, and is at least its select's width and at most the viewport's (packages/blitz-dom/src/node/select_list.rs:218-263). A select that is disabled, inert or hidden, or has no row to show, opens nothing, and one made inert, disabled or hidden or removed closes its list. `:open` and `flush_open_state` are unmoved, so an author's `open` on a select steers nothing. Not built, as rejected in the plan: `:open` plus rows as boxes, a second OS window, an overlay hit in `hit_inner`, colours in `default.css`. Its merge surface is sixteen files, its line owned by Epoch 3's «Upstream sync ahead of the UI graph» (as measured at escher-0.3.0/chunks/2026-10-31-select-popup/evidence/merge-surface.md).»"
    sidecar: "New: the select list, a direct engine change for every document — its state, its placement, what opens and closes it, what was rejected, its merge surface."
    rationale: "Report Symbols / APIs (the list's state, `BaseDocument`), Deviations (`live_select_list` and its readers), Coverage (every document, remote pages included), Reverted / negative API facts (the rejected approaches), P5 (merge surface); his forks at inputs#I4."
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 81
    section: "## Established Decisions"
    change: "New item after the last: «- An open list's pointer: a click on a single select opens its list or closes one standing, each focusing (`focus_select`); `handle_pointerdown` returns on a press in the list and else runs `close_list_outside`, and a press in the list starts no text drag (`pressed_in_list`); the release chooses the row under it through `click_list_row`, behind `click_lands_on_row`, and `apply_select_choice`, a disabled row changing nothing and leaving the list open, a group's header choosing nothing; a move takes the highlight to an enabled row (`hover_list_row`); a wheel over the list scrolls it, and elsewhere closes it through `close_select_list` and then scrolls (packages/blitz-dom/src/events/pointer.rs:415-420; packages/blitz-dom/src/events/pointer.rs:595-611; packages/blitz-dom/src/events/pointer.rs:613-617; packages/blitz-dom/src/events/pointer.rs:619-633; packages/blitz-dom/src/events/pointer.rs:635-640; packages/blitz-dom/src/events/pointer.rs:642-655; packages/blitz-dom/src/events/pointer.rs:1108-1119; packages/blitz-dom/src/events/pointer.rs:1194-1202).»"
    sidecar: "New: an open select list's pointer rules — open, close, press, choose, hover and wheel."
    rationale: "Report Symbols / APIs, Pointer; Outcome (arch) the one write path; Deviations (the release chooses the row under it); `select_popup`'s `a_press_on_a_row_selects_no_text_and_a_drag_across_rows_chooses_where_it_ends` and `a_disabled_row_changes_nothing_and_leaves_the_list_open`."
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 81
    section: "## Established Decisions"
    change: "New item after the last: «- An open list's keys: `press_open_list_key` is its closed key match, moving the highlight over enabled rows and choosing nothing; Enter, Space, Tab and a click on a row choose the row, one `input` then one `change`, through `choose_current_and_close` or `click_list_row` to `apply_select_choice`, and a choice of the chosen option fires nothing; Tab runs `choose_current_and_close` before moving focus, in either direction; Escape closes with the value unchanged; an opening and a close fire nothing, and a close clears the typed text (packages/blitz-dom/src/events/keyboard.rs:32-34; packages/blitz-dom/src/events/keyboard.rs:215-250; packages/blitz-dom/src/events/keyboard.rs:252-267).»"
    sidecar: "New: an open select list's keys — highlight, choice, Tab and Escape."
    rationale: "Report Symbols / APIs, Keys; Outcome (arch) the one write path; Deviations (a close clears the typed text); `select_popup`'s `enter_space_tab_and_a_click_choose_the_row_one_input_then_one_change`, `tab_chooses_and_closes_before_focus_moves_either_way` and `a_click_enter_and_space_open_the_list_on_its_chosen_option_and_fire_nothing`."
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 81
    section: "## Established Decisions"
    change: "New item after the last: «- The list answers before the page: `hit_with_scrollbar` answers `select_list_hit` first and no scrollbar with it, so `hit`, the driver's `covered` through it, `set_hover_to` and `handle_pointerdown` all read the list, `hit_inner` not edited; `visible_region` answers the list's scrollport for a row or a group of an open list, and `scroll_into_view` on a row writes the list's `scroll_y` at once and returns, no chain or viewport moving. `resolve` calls `reconcile_select_list` after layout and before `refresh_hover`, so a page's scroll or a move that closes a list marks its select at the next resolve, as a wheel elsewhere does through `close_select_list`, while a highlight or a wheel move inside the list marks nothing. The tree writes a single select's `expanded` from whether its list is open, reading no `aria-expanded` there, every other element keeping that read (packages/blitz-dom/src/document.rs:2404-2406; packages/blitz-dom/src/scrolling.rs:669-672; packages/blitz-dom/src/scrolling.rs:754-756; packages/blitz-dom/src/resolve.rs:133-135; packages/blitz-dom/src/accessibility.rs:238-242).»"
    sidecar: "New: the list in the hit, in `visible_region` and `scroll_into_view`, in the resolve and the changed set, and in the tree's `expanded`."
    rationale: "Report Symbols / APIs (The hit, The resolve, Tree, Seen through the list); Spec claims disproved, `changed-set`: a page scroll or a move that closes a list marks its select, and a highlight or a wheel move inside the list marks nothing."
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 81
    section: "## Established Decisions"
    change: "New item after the last: «- The list's limits: its thumb is painted and not draggable; a highlighted row's contrast is unread by the lint; a larger authored font clips; the highlight is in no snapshot and no tree; a single select's `size` is unread; the windowed list is read and not run, and what an assistive technology reads of it stays unmeasured (as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md).»"
    sidecar: "New: the select list's stated limits, in place of the retired «no popup is painted»."
    rationale: "Report inputs#I7's row 1 names the five limits; Outcome (arch) «Windowed list read and not run» and (a11y) «what an assistive technology reads stays unmeasured»."
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 122
    section: "## Established Decisions"
    change: "New item after line 122: «- `size` on a `multiple` select hints a `height` of `size` × 1.2em through `size_rows` (`parse_unsigned_integer`, capped at 65534, `0` or a bad value no hint), the default sheet's `select[multiple][size] { overflow-y: auto; }` scrolling it with `line-height: 1.2` on its rows, and `apply_row_gesture` calls `scroll_into_view` on the current row only where `carries_valid_size`; a single select's `size` stays unread (packages/blitz-dom/src/stylo.rs:1124-1139; packages/blitz-dom/src/node/select_list.rs:125-130; packages/blitz-dom/assets/default.css:183-193; packages/blitz-dom/src/events/keyboard.rs:423-430; packages/blitz-dom/src/events/keyboard.rs:452-459).»"
    sidecar: "New: `size` read on a `multiple` select, capped, through a height hint and the default sheet."
    rationale: "Report Symbols / APIs, **`size`** and Keys (`apply_row_gesture`); Coverage, `size on a multiple select`; Schema / config: «New attribute read: `size` on a `multiple` select, a parse into a capped whole number»."
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 153
    section: "## Established Decisions"
    change: "New item after line 153: «- An open select list paints through `paint_scene` alone, no shell or renderer moved: `paint_scene` calls `paint_select_list` after the root's render and before the devtools overlays, a non-finite box unpainted, only the rows in the scrollport laid out and the thumb under the `scrollbars` feature. Its colours are `SelectListView`'s associated constants — `HIGHLIGHT_TEXT` `#000000`, `HIGHLIGHT_BACKGROUND` `#c8c8c8`, `DISABLED_TEXT` `#6d6d6d`, `BORDER` `#999999` — hardcoded on the founder's F6, the engine holding no tokens. A group's header is emboldened through `stroke_text_as`'s `RunPaint { color, synthesize_bold }`, a synthesized bold adding one embolden weight to a build's own and every other caller passing `RunPaint::default()`; `hint` stays `!FONT_EMBOLDEN_ENABLED`, so a synthesized header on a plain build is emboldened with hinting on (packages/blitz-paint/src/render.rs:246-247; packages/blitz-paint/src/render/select_list.rs:21-40; packages/blitz-paint/src/text.rs:586-593; packages/blitz-paint/src/text.rs:683-689; packages/blitz-paint/src/text.rs:695).»"
    sidecar: "New: the select list's paint — its place in `paint_scene`, its hardcoded colours and the synthesized bold of a group's header."
    rationale: "Report Symbols / APIs, Paint and the list's state (the constants); Coverage, tokens hardcoded on his F6; Outcome (arch) «through `paint_scene` alone, no shell or renderer moved»; fact `paint-order`."
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 206
    section: "## Established Decisions"
    change: "New item after line 206: «- An open select list crosses the ratified carriers with no new key name, node key, field, cause, session error, exit, carrier, listener, port, env var, log field or span: a row's own `bounds` in the snapshot and the diff (`differs` names `bounds`, a closed set inside the ratified crossing) and pixels in a `screenshot`, where an option's label and a group's `label` now paint — a page's labels in pixels, a boundary widening on the founder's recorded word (inputs#I4), put to him on this wrap's answer card; no redaction shape moves, and `describe`'s count row stands unmoved (as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md).»"
    sidecar: "New: the list's crossing — a row's `bounds` and a page's labels as pixels on the ratified carriers, a boundary widening carried to the founder's answer card."
    rationale: "Report Schema / config; Coverage, `the list's pixels on the screenshot carriers` («boundary widening, P2's halt»); Expected amendments, Playbook *Boundary widening* (inputs#I4, CARRY `cry-musoudbr`)."
note: the widening item at near 206 stands unratified until P2's halt, so fill in his answer's wording when it comes; lines 63, 68 and 78 were not measured against the 2 000-char limit (the guard refused the measuring pipe) and the body's own broad lines at .andromeda/architecture.md:56, :60 and :62 state what these proposals retire but lie outside this file.
