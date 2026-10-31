# Cascade compare — architecture · wrap · 2026-10-31-select-popup
E items 75 · held 30 · lost at classify 6 · lost at strong 3 · beside a placed fact 4 · cascade only 10 · listed - · body 0 · unplaced 0

## Lost — the full read carried these and the cascade did not; the wrap validates each, marked «(sampler)»

### cross-cutting-patterns#4.1 · lost at classify · .andromeda/architecture-detail/cross-cutting-patterns.md:27
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/cross-cutting-patterns.md
    near: 27
    section: "## Cross-cutting Patterns"
    change: "Add a row after line 27: «- A single select's open list asks a redraw at its opening, at a move of its highlight, at its scrolling and at its closing (tests/blitz-tests/tests/select_popup.rs:1045-1090)»"
    sidecar: "§Cross-cutting Patterns → Service wiring: new row, a select list's opening, highlight move, scrolling and closing each ask a redraw (2026-10-31-select-popup)."
    rationale: "Report New text, by line lists `opening_moving_scrolling_and_closing_each_ask_a_redraw` in `select_popup`, which its Outcome reads green; line 21 enumerates what asks a redraw and names no list. A row of its own keeps line 21 under the 2 000-char limit; binds with .andromeda/architecture.md:271."
    basis: .andromeda/architecture-detail/cross-cutting-patterns.md:21

### design-philosophy#1.1 · lost at strong · .andromeda/architecture-detail/design-philosophy.md:13
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/design-philosophy.md
    near: 13
    section: "## Design Philosophy"
    change: "after the clause «painting assumes styles and layout are already resolved» and its citation add: «; an open select's list is painted by `paint_select_list` after the root's render inside `paint_scene`, from the public read `select_list_view`, no shell or renderer moved, and its rows' text is the carve-out, laid out at paint through `select_row_layout`, only the rows in the scrollport (packages/blitz-paint/src/render.rs:246-247; packages/blitz-paint/src/render/select_list.rs:85-128; packages/blitz-dom/src/document.rs:1022-1052; packages/blitz-dom/src/document.rs:1054-1064)»"
    sidecar: "Design Philosophy, modular engine: an open select's list paints through `paint_scene` alone, its rows' text laid out at paint, a carve-out to painting's assumption of resolved layout (2026-10-31-select-popup)."
    rationale: "Report Changes › Symbols / APIs › Paint names `paint_list_row` and «only rows in the scrollport laid out», and › `BaseDocument` names the public `select_row_layout`; Outcome (arch) has the list through `paint_scene` alone. The file's line states painting's assumption with no carve-out, and no sweep pattern in the report reaches this wording."
    basis: packages/blitz-dom/src/document.rs:1054-1064

### established-decisions#11.6 · lost at strong · .andromeda/architecture-detail/established-decisions.md:81
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 81
    section: "## Established Decisions"
    change: "New item after line 81: «- The select list stands beside the select model, a direct engine change for every document, remote pages included, on the founder's ten forks, all A (inputs#I4): a closed single select opens a list the engine paints over the page, past an `opacity` ancestor and a box that clips its select. Its state is one `BaseDocument` field, `select_list: Option<SelectList>`, `None` at construction, read through the crate-private `live_select_list`, past it only by `reconcile_select_list` and the removal arm and written directly by `hover_list_row` (packages/blitz-dom/src/document.rs:343-344; packages/blitz-dom/src/document.rs:797-804; packages/blitz-dom/src/document.rs:1408-1410; packages/blitz-dom/src/node/select_list.rs:28-46). It shows at most `MAX_ROWS` = 20 rows, opens where its rows fit and else on the side with more room, and is at least its select's width and at most the viewport's (packages/blitz-dom/src/node/select_list.rs:218-263). A select that is disabled, inert or hidden, or has no row to show, opens nothing, and one made inert, disabled or hidden or removed closes its list. `:open` and `flush_open_state` are unmoved, so an author's `open` on a select steers nothing. Not built, as rejected in the plan: `:open` plus rows as boxes, a second OS window, an overlay hit in `hit_inner`, colours in `default.css`. Its merge surface is sixteen files, its line owned by Epoch 3's «Upstream sync ahead of the UI graph» (as measured at escher-0.3.0/chunks/2026-10-31-select-popup/evidence/merge-surface.md).»"
    sidecar: "New: the select list, a direct engine change for every document — its state, its placement, what opens and closes it, what was rejected, its merge surface."
    rationale: "Report Symbols / APIs (the list's state, `BaseDocument`), Deviations (`live_select_list` and its readers), Coverage (every document, remote pages included), Reverted / negative API facts (the rejected approaches), P5 (merge surface); his forks at inputs#I4."

### established-decisions#11.6 · lost at strong · .andromeda/architecture-detail/established-decisions.md:81
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 81
    section: "## Established Decisions"
    change: "New item after the last: «- An open list's pointer: a click on a single select opens its list or closes one standing, each focusing (`focus_select`); `handle_pointerdown` returns on a press in the list and else runs `close_list_outside`, and a press in the list starts no text drag (`pressed_in_list`); the release chooses the row under it through `click_list_row`, behind `click_lands_on_row`, and `apply_select_choice`, a disabled row changing nothing and leaving the list open, a group's header choosing nothing; a move takes the highlight to an enabled row (`hover_list_row`); a wheel over the list scrolls it, and elsewhere closes it through `close_select_list` and then scrolls (packages/blitz-dom/src/events/pointer.rs:415-420; packages/blitz-dom/src/events/pointer.rs:595-611; packages/blitz-dom/src/events/pointer.rs:613-617; packages/blitz-dom/src/events/pointer.rs:619-633; packages/blitz-dom/src/events/pointer.rs:635-640; packages/blitz-dom/src/events/pointer.rs:642-655; packages/blitz-dom/src/events/pointer.rs:1108-1119; packages/blitz-dom/src/events/pointer.rs:1194-1202).»"
    sidecar: "New: an open select list's pointer rules — open, close, press, choose, hover and wheel."
    rationale: "Report Symbols / APIs, Pointer; Outcome (arch) the one write path; Deviations (the release chooses the row under it); `select_popup`'s `a_press_on_a_row_selects_no_text_and_a_drag_across_rows_chooses_where_it_ends` and `a_disabled_row_changes_nothing_and_leaves_the_list_open`."

### established-decisions#11.6 · lost at strong · .andromeda/architecture-detail/established-decisions.md:81
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 81
    section: "## Established Decisions"
    change: "New item after the last: «- An open list's keys: `press_open_list_key` is its closed key match, moving the highlight over enabled rows and choosing nothing; Enter, Space, Tab and a click on a row choose the row, one `input` then one `change`, through `choose_current_and_close` or `click_list_row` to `apply_select_choice`, and a choice of the chosen option fires nothing; Tab runs `choose_current_and_close` before moving focus, in either direction; Escape closes with the value unchanged; an opening and a close fire nothing, and a close clears the typed text (packages/blitz-dom/src/events/keyboard.rs:32-34; packages/blitz-dom/src/events/keyboard.rs:215-250; packages/blitz-dom/src/events/keyboard.rs:252-267).»"
    sidecar: "New: an open select list's keys — highlight, choice, Tab and Escape."
    rationale: "Report Symbols / APIs, Keys; Outcome (arch) the one write path; Deviations (a close clears the typed text); `select_popup`'s `enter_space_tab_and_a_click_choose_the_row_one_input_then_one_change`, `tab_chooses_and_closes_before_focus_moves_either_way` and `a_click_enter_and_space_open_the_list_on_its_chosen_option_and_fire_nothing`."

### established-decisions#11.6 · lost at strong · .andromeda/architecture-detail/established-decisions.md:81
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 81
    section: "## Established Decisions"
    change: "New item after the last: «- The list answers before the page: `hit_with_scrollbar` answers `select_list_hit` first and no scrollbar with it, so `hit`, the driver's `covered` through it, `set_hover_to` and `handle_pointerdown` all read the list, `hit_inner` not edited; `visible_region` answers the list's scrollport for a row or a group of an open list, and `scroll_into_view` on a row writes the list's `scroll_y` at once and returns, no chain or viewport moving. `resolve` calls `reconcile_select_list` after layout and before `refresh_hover`, so a page's scroll or a move that closes a list marks its select at the next resolve, as a wheel elsewhere does through `close_select_list`, while a highlight or a wheel move inside the list marks nothing. The tree writes a single select's `expanded` from whether its list is open, reading no `aria-expanded` there, every other element keeping that read (packages/blitz-dom/src/document.rs:2404-2406; packages/blitz-dom/src/scrolling.rs:669-672; packages/blitz-dom/src/scrolling.rs:754-756; packages/blitz-dom/src/resolve.rs:133-135; packages/blitz-dom/src/accessibility.rs:238-242).»"
    sidecar: "New: the list in the hit, in `visible_region` and `scroll_into_view`, in the resolve and the changed set, and in the tree's `expanded`."
    rationale: "Report Symbols / APIs (The hit, The resolve, Tree, Seen through the list); Spec claims disproved, `changed-set`: a page scroll or a move that closes a list marks its select, and a highlight or a wheel move inside the list marks nothing."

### established-decisions#11.6 · lost at strong · .andromeda/architecture-detail/established-decisions.md:81
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 81
    section: "## Established Decisions"
    change: "New item after the last: «- The list's limits: its thumb is painted and not draggable; a highlighted row's contrast is unread by the lint; a larger authored font clips; the highlight is in no snapshot and no tree; a single select's `size` is unread; the windowed list is read and not run, and what an assistive technology reads of it stays unmeasured (as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md).»"
    sidecar: "New: the select list's stated limits, in place of the retired «no popup is painted»."
    rationale: "Report inputs#I7's row 1 names the five limits; Outcome (arch) «Windowed list read and not run» and (a11y) «what an assistive technology reads stays unmeasured»."

### established-decisions#21.1 · lost at strong · .andromeda/architecture-detail/established-decisions.md:153
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 153
    section: "## Established Decisions"
    change: "New item after line 153: «- An open select list paints through `paint_scene` alone, no shell or renderer moved: `paint_scene` calls `paint_select_list` after the root's render and before the devtools overlays, a non-finite box unpainted, only the rows in the scrollport laid out and the thumb under the `scrollbars` feature. Its colours are `SelectListView`'s associated constants — `HIGHLIGHT_TEXT` `#000000`, `HIGHLIGHT_BACKGROUND` `#c8c8c8`, `DISABLED_TEXT` `#6d6d6d`, `BORDER` `#999999` — hardcoded on the founder's F6, the engine holding no tokens. A group's header is emboldened through `stroke_text_as`'s `RunPaint { color, synthesize_bold }`, a synthesized bold adding one embolden weight to a build's own and every other caller passing `RunPaint::default()`; `hint` stays `!FONT_EMBOLDEN_ENABLED`, so a synthesized header on a plain build is emboldened with hinting on (packages/blitz-paint/src/render.rs:246-247; packages/blitz-paint/src/render/select_list.rs:21-40; packages/blitz-paint/src/text.rs:586-593; packages/blitz-paint/src/text.rs:683-689; packages/blitz-paint/src/text.rs:695).»"
    sidecar: "New: the select list's paint — its place in `paint_scene`, its hardcoded colours and the synthesized bold of a group's header."
    rationale: "Report Symbols / APIs, Paint and the list's state (the constants); Coverage, tokens hardcoded on his F6; Outcome (arch) «through `paint_scene` alone, no shell or renderer moved»; fact `paint-order`."

### established-decisions#29.1 · lost at strong · beside a placed fact F9 (near 200, established-decisions#28.1) · .andromeda/architecture-detail/established-decisions.md:206
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 206
    section: "## Established Decisions"
    change: "New item after line 206: «- An open select list crosses the ratified carriers with no new key name, node key, field, cause, session error, exit, carrier, listener, port, env var, log field or span: a row's own `bounds` in the snapshot and the diff (`differs` names `bounds`, a closed set inside the ratified crossing) and pixels in a `screenshot`, where an option's label and a group's `label` now paint — a page's labels in pixels, a boundary widening on the founder's recorded word (inputs#I4), put to him on this wrap's answer card; no redaction shape moves, and `describe`'s count row stands unmoved (as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md).»"
    sidecar: "New: the list's crossing — a row's `bounds` and a page's labels as pixels on the ratified carriers, a boundary widening carried to the founder's answer card."
    rationale: "Report Schema / config; Coverage, `the list's pixels on the screenshot carriers` («boundary widening, P2's halt»); Expected amendments, Playbook *Boundary widening* (inputs#I4, CARRY `cry-musoudbr`)."

### existing-scopes#4.1 · lost at classify · .andromeda/architecture-detail/existing-scopes.md:151
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 151
    section: ## Existing Scopes
    change: New row: blitz-tests' `select_popup`, since 2026-10-31-select-popup, 25 tests in both layout modes, a named `--test` of the `stand-repeat` list and outside the `a11y` leg's fourteen files — a click, Enter and Space opening a single select's list on its chosen option and firing nothing, keys and the pointer moving the highlight over enabled rows, Enter, Space, Tab and a click choosing with one `input` then one `change`, a disabled row changing nothing, Escape, a second click, a click outside and the page scrolling closing with no event, a label's click focusing and opening nothing, a select that cannot open and one made inert, disabled, hidden or removed with its list open, the key press skipped in the ancestor-`hidden` row for the `position_in_containing_block` panic the report routes, the tree's reading, an author's `aria-expanded` and `open` steering nothing, clips, a pointer at a row's centre, groups as headers, a long list's twenty rows, hostile markup and `size` on a `multiple` select (tests/blitz-tests/tests/select_popup.rs:202-1186; .github/scripts/ci-leg.sh:59-60).
    sidecar: New row for the engine check `select_popup`, 25 tests
    rationale: The report's Files, Coverage and Harness entries add this target; the file names each blitz-tests target and holds no row for it, and the Spec claims entry states the one skipped key press.
    basis: escher-0.3.0/chunks/2026-10-31-select-popup/report.md:110

### existing-scopes#4.1 · lost at classify · .andromeda/architecture-detail/existing-scopes.md:151
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 151
    section: ## Existing Scopes
    change: New row: blitz-tests' `select_popup_painted`, since 2026-10-31-select-popup, 6 tests in both layout modes, a named `--test` of the `stand-repeat` list — the highlight and a disabled row read as painted against `SelectListView`'s constants, the plain pair measured at the default sheet and at the stand, a list painted whole past an opacity ancestor and a box that clips its select, a closed select's frame byte-equal before opening and after a close with no choice, a group's header darker than a row by weight and a scrolled list's thumb painted and taking no hit (tests/blitz-tests/tests/select_popup_painted.rs:119-351; .github/scripts/ci-leg.sh:59-60).
    sidecar: New row for the engine check `select_popup_painted`, 6 tests
    rationale: The report's Files, Coverage and Harness entries add this target; the file names each blitz-tests target and holds no row for it.
    basis: escher-0.3.0/chunks/2026-10-31-select-popup/report.md:110

### existing-scopes#4.1 · lost at classify · .andromeda/architecture-detail/existing-scopes.md:151
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 151
    section: ## Existing Scopes
    change: New row: blitz-tests' sixteenth driver-action file `stand_act_popup`, since 2026-10-31-select-popup, 12 tests in both layout modes, reading `tests/session_common/mod.rs` and in `run stand` and `stand-repeat` by its prefix — a click opening the list and a click on a row choosing it, the keys alone opening, moving and choosing, Escape closing with the value unchanged and the rows back at the select's box, a closed option staying refused `covered`, a row past the scrollport off screen until `scroll` brings it in, a button under the list refused `covered`, `type` leaving no list open and choosing by its text, a shot of a row cropping to it and leaving the list open, the lint reading no `overlap` on a row with the planted pair still read, the open screen under its budget and every actionable element keyed, two clicks on one point opening and closing each focusing, and a screen read after a step reading the new state with no `advance` (tests/blitz-tests/tests/stand_act_popup.rs:58-423).
    sidecar: New row for the driver-action check `stand_act_popup`, 12 tests
    rationale: The report's Files, Coverage and Harness entries add this target, the sixteenth by `act-files`; lines 110-112 enumerate the driver-action files through the fifteenth and that long line cannot take it.
    basis: escher-0.3.0/chunks/2026-10-31-select-popup/report.md:70

### existing-scopes#4.1 · lost at classify · .andromeda/architecture-detail/existing-scopes.md:151
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 151
    section: ## Existing Scopes
    change: New row: blitz-tests' `stand_keyboard`, its twenty-second and twenty-third tests since 2026-10-31-select-popup — the flight type walk holding with its list open and after each close, and a click opening the list with focus staying on the select and the walk going on from it, in the `a11y` CI leg with the rest of the file (tests/blitz-tests/tests/stand_keyboard.rs:1082-1111; tests/blitz-tests/tests/stand_keyboard.rs:1113-1134).
    sidecar: New row for `stand_keyboard`'s two list tests
    rationale: The report's `kbd-count` and `a11y-tally` move the file by two tests and the leg by the same two; line 106 enumerates each test's arrival and already runs long, so the detail takes its own row.
    basis: escher-0.3.0/chunks/2026-10-31-select-popup/report.md:33

### occupied-resources#1.1 · lost at classify · .andromeda/architecture-detail/occupied-resources.md:10
  - detector: D-new-detail
    severity: escalate
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 10
    section: "## Occupied Resources"
    change: Append to this line, after the lint sentence and its measure - Since 2026-10-31-select-popup a shot of an open select list also carries that list's option labels and group `label`s, a page's text as painted pixels, on both carriers, and a row's own `bounds` cross in the snapshot and the diff (`differs` naming `bounds`, a closed set inside the ratified crossing); no key, node key, field, carrier, listener or port is added; the pixels are a boundary widening on the founder's recorded forks, put to him at that chunk's wrap (as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md, inputs#I4).
    sidecar: Network ports and listeners gains the open select list's labels as pixels in a shot and a row's bounds in the snapshot and diff, no carrier or listener added.
    rationale: The report's Schema / config names this new crossing and its Coverage row marks the pixels a boundary widening for P2's halt, while this line lists what a shot carries and holds no page text painted by the engine's own list; severity escalate because the wording of its ratification rests on that halt.
    basis: escher-0.3.0/chunks/2026-10-31-select-popup/report.md:26

### occupied-resources#9.3 · lost at classify · .andromeda/architecture-detail/occupied-resources.md:73
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 73
    section: "## Occupied Resources"
    change: New item after this line - `host_log`'s unix-only `a_select_list_opened_and_chosen_through_the_host_logs_no_option_at_trace` also spawns the built binary's host role beside its `crud` and `cells` runs, its state directory `hl-popup` under `CARGO_TARGET_TMPDIR`, and reads 0 needles at `trace`; `mcp_log`'s select-list check reads the same at `trace`, records nothing and holds no `ml-popup` directory (as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md).
    sidecar: Process-wide state and threads gains a row for the host_log and mcp_log select-list checks, test-only, no thread, socket or port beyond the standing ones named.
    rationale: Line 57 names `host_log`'s host-role runs as `crud` and `cells` alone; the report's New text, by line adds a further one, its Spec claims disproved bullet states its `#[cfg(unix)]`, its Outcome states the 0 needles and its Reverted bullet states `mcp_log`'s negative; a new item since the bullet at lines 53-59 is already long.
    basis: examples/seven_guis/tests/host_log.rs:284

### standard-contracts#2.4 · lost at classify · .andromeda/architecture-detail/standard-contracts.md:19
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 19
    section: "## Standard Contracts"
    change: "add row «- An open select list's public surface on `BaseDocument`, since 2026-10-31-select-popup: read-only `open_select(&self) -> Option<NodeId>`, `select_list_bounds(&self, node: NodeId) -> Option<BoundingRect>`, `select_list_view(&self) -> Option<SelectListView>` and `select_row_layout(&self, row: NodeId, text: &str) -> Option<parley::Layout<TextBrush>>`, and the writer `close_select_list(&mut self)`, its callers keyboard, pointer, reconcile and the driver's `focus_select`; each reads through `get_node`, a row with no styles `None`, with no `layout_style()` (packages/blitz-dom/src/document.rs:881-889; packages/blitz-dom/src/document.rs:990-993; packages/blitz-dom/src/document.rs:995-1012; packages/blitz-dom/src/document.rs:1022-1052; packages/blitz-dom/src/document.rs:1054-1064; as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md).»"
    sidecar: "Document core: five new public `BaseDocument` methods of the select list registered."
    rationale: "Report Symbols / APIs `BaseDocument` and Coverage `BaseDocument public readers` land five public methods; none appears in this file."
    basis: packages/blitz-dom/src/document.rs:990-993

### standard-contracts#2.4 · lost at classify · beside a placed fact F10 (near 17, standard-contracts#2.2) · .andromeda/architecture-detail/standard-contracts.md:19
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 19
    section: "## Standard Contracts"
    change: "add row «- An open select list's state in the document: one field `select_list: Option<SelectList>`, `None` at construction, read through the `pub(crate)` `live_select_list`, past it only by `reconcile_select_list`, which must see a list the reader no longer answers, and by the removal arm, with `hover_list_row` writing `current` directly; crate-private `select_list_for`, `open_select_list`, `select_list_options`, `move_select_list_current`, `scroll_select_list_to`, `scroll_select_list_by`, `select_list_hit` and `select_list_region`, private `select_list_allowed`, `measure_select_list` and `text_row_layout`; `resolve` calls `reconcile_select_list` after layout and before `refresh_hover`, and `clear_interaction_state_for_removed_node` clears the field for its select; a select with no row to show opens nothing; an engine change for every document, remote pages included (packages/blitz-dom/src/document.rs:343-344; packages/blitz-dom/src/document.rs:797-804; packages/blitz-dom/src/document.rs:1408-1410; packages/blitz-dom/src/resolve.rs:133-135; as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md).»"
    sidecar: "Document core: the list's one field, its crate-private methods and its reconcile in `resolve` stated."
    rationale: "Report Symbols / APIs `BaseDocument` and The resolve, Deviations on `live_select_list`, Coverage `select list, engine overlay`; Expected amendments name §Standard Contracts → Document core."

### standard-contracts#3.3 · lost at classify · beside a placed fact F11 (near 17, standard-contracts#2.2) · .andromeda/architecture-detail/standard-contracts.md:32
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 32
    section: "## Standard Contracts"
    change: "add row «- A single select in the tree: `build_accessibility_node` writes `set_expanded(open)` from the engine's own state and reads no `aria-expanded`, a closed bool and no author attribute, every other element keeping that read; the role stays `ComboBox`, the chosen option reads `selected` and the tree's focus stays on the select; the highlighted row is unread by the tree, and `:open` and `flush_open_state` are unmoved, so an author's `open` on a select steers nothing; an engine change for every document (packages/blitz-dom/src/accessibility.rs:238-242; tests/blitz-tests/tests/select_popup.rs:565-614; tests/blitz-tests/tests/select_popup.rs:628-651).»"
    sidecar: "Mutation, query and CSSOM: the tree's reading of an open single select stated."
    rationale: "Report Symbols / APIs Tree and Coverage `tree expanded on a single select`."
    basis: packages/blitz-dom/src/accessibility.rs:238-242

### standard-contracts#4.2 · lost at classify · beside a placed fact F14 (near 17, standard-contracts#2.2) · .andromeda/architecture-detail/standard-contracts.md:42
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 42
    section: "## Standard Contracts"
    change: "add row «- An open select list under the pointer: `handle_pointerdown` returns on a press in the list and else runs `close_list_outside`; `handle_click`'s `select` arm opens a single select's list or closes one standing, the new private `focus_select` holding what the arm did before, its `option` arm gains `click_list_row` behind `click_lands_on_row`, a new `optgroup` arm stands, and the `label` arm focuses a bound single select and opens nothing; `handle_pointermove` calls `hover_list_row`, a press in the list starts no text drag (`pressed_in_list`) and the release chooses the row under it; `handle_wheel` scrolls the list over it, else runs `close_select_list` and then scrolls; `click_list_row` chooses through `apply_select_choice`, the one write path; an engine change for every document (packages/blitz-dom/src/events/pointer.rs:415-420; packages/blitz-dom/src/events/pointer.rs:595-656; packages/blitz-dom/src/events/pointer.rs:1108-1120; as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md).»"
    sidecar: "Events: the pointer's default actions on an open select list stated."
    rationale: "Report Symbols / APIs Pointer, Outcome (arch) the one write path; a surface this contract group states for every other control and no line here holds."
    basis: packages/blitz-dom/src/events/pointer.rs:595-656

### standard-contracts#4.2 · lost at classify · beside a placed fact F22 (near 41, standard-contracts#4.1) · .andromeda/architecture-detail/standard-contracts.md:42
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 42
    section: "## Standard Contracts"
    change: "add row «- An open select list under the keys: `press_select_key` opens on Enter, and on a Space with no typed text standing; `press_open_list_key` is the open list's closed key match; Enter, Space, Tab and a click on a row choose it through `choose_current_and_close` or `click_list_row`, one `input` then one `change`, a choice of the chosen option firing nothing and a disabled row leaving the list open; Tab runs `choose_current_and_close` before moving focus, either direction; a close clears the typed text; `apply_row_gesture` calls `scroll_into_view` on the current row only where `carries_valid_size`; `press_multiple_select_key` and `click_select_row` keep their semantics; an engine change for every document (packages/blitz-dom/src/events/keyboard.rs:32-34; packages/blitz-dom/src/events/keyboard.rs:215-250; packages/blitz-dom/src/events/keyboard.rs:252-267; packages/blitz-dom/src/events/keyboard.rs:423-430; packages/blitz-dom/src/events/keyboard.rs:452-459; tests/blitz-tests/tests/select_popup.rs:294-327; as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md).»"
    sidecar: "Events: the keys' default actions on an open select list stated."
    rationale: "Report Symbols / APIs Keys and Deviations (a close clears the typed text); the chosen-row test named in New text, by line."
    basis: packages/blitz-dom/src/events/keyboard.rs:215-250

### standard-contracts#6.5 · lost at classify · beside a placed fact F18 (near 58, standard-contracts#6.1) · .andromeda/architecture-detail/standard-contracts.md:62
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 62
    section: "## Standard Contracts"
    change: "add row «- An open list under a point: `select_list_hit` answers a point in the list before any walk and no scrollbar with it, so `hit` (the driver's `covered` through it), `set_hover_to` and `handle_pointerdown` all read the list, a row is never a thumb drag and `hit_inner` is not edited; a box that clips the select clips no row, and the list's painted thumb takes no hit; an engine change for every document (packages/blitz-dom/src/document.rs:962-988; packages/blitz-dom/src/document.rs:2404-2406; tests/blitz-tests/tests/select_popup.rs:677-716; tests/blitz-tests/tests/select_popup_painted.rs:312-351).»"
    sidecar: "Scrolling, selection, tree: `select_list_hit` and what reads it stated."
    rationale: "Report Symbols / APIs The hit; Counts `hit-first` (a row never a thumb drag); the two named tests in New text, by line."
    basis: packages/blitz-dom/src/document.rs:962-988

### standard-contracts#13.1 · lost at classify · .andromeda/architecture-detail/standard-contracts.md:112
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 112
    section: "## Standard Contracts"
    change: "add row «- An open list's paint: the new `render/select_list.rs` holds `paint_select_list`, `device_rect`, `paint_list_box`, `paint_list_row` and, under the `scrollbars` feature, `paint_list_thumb`, a non-finite box unpainted and only rows in the scrollport laid out, past an `opacity` ancestor and a box that clips its select, with no shell or renderer moved; `text.rs`'s `stroke_text` delegates to the `pub(crate)` `stroke_text_as` with `RunPaint { color, synthesize_bold }`, every other caller passing `RunPaint::default()`; a synthesized bold adds one embolden weight to a build's own, and `hint` stays `!FONT_EMBOLDEN_ENABLED`, so a synthesized header on a plain build is emboldened with hinting on (packages/blitz-paint/src/render/select_list.rs:21-40; packages/blitz-paint/src/text.rs:586-593; packages/blitz-paint/src/text.rs:683-689; packages/blitz-paint/src/text.rs:695; tests/blitz-tests/tests/select_popup_painted.rs:211-235).»"
    sidecar: "Paint and shell: the module `render::select_list` and `stroke_text_as` registered."
    rationale: "Report Crates / modules names `render::select_list`; Symbols / APIs Paint; Outcome (arch) through `paint_scene` alone."
    basis: packages/blitz-paint/src/text.rs:586-593

## Held

- conventions#8.1
- cross-cutting-patterns#6.1
- design-philosophy#4.1
- established-decisions#10.1
- established-decisions#11.4
- established-decisions#12.1
- established-decisions#16.5
- established-decisions#16.6
- existing-scopes#1.14
- existing-scopes#1.17
- existing-scopes#1.23
- existing-scopes#1.31
- existing-scopes#1.7
- existing-scopes#3.2
- occupied-resources#4.1
- occupied-resources#9.1
- standard-contracts#1.1
- standard-contracts#12.1
- standard-contracts#2.3
- standard-contracts#20.1
- standard-contracts#21.3
- standard-contracts#22.1
- standard-contracts#28.1
- standard-contracts#28.2
- standard-contracts#3.1
- standard-contracts#32.1
- standard-contracts#5.1
- standard-contracts#6.2
- standard-contracts#6.4
- standard-contracts#9.3

## Cascade only

- cross-cutting-patterns#3.1
- established-decisions#11.5
- established-decisions#28.1
- occupied-resources#7.1
- standard-contracts#14.1
- standard-contracts#2.2
- standard-contracts#27.1
- standard-contracts#4.1
- standard-contracts#6.1
- standard-contracts#9.1
