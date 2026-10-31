proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/design-system-detail/depth-strategy.md
    near: 26
    section: "## Depth Strategy"
    change: "After the clause on a node paint clips, add: this walk is the page's; a row of an open select list stands outside it, answered before the walk by `select_list_hit` in `hit_with_scrollbar`, so a box clipping the select clips no row (packages/blitz-dom/src/document.rs:2404-2406; tests/blitz-tests/tests/select_popup.rs:677-716; as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)."
    sidecar: "Depth Strategy: hit-testing's clip rule scoped to the page's walk, an open list's rows answered before it."
    rationale: "Report, Symbols / APIs, The hit: `hit_with_scrollbar` answers `select_list_hit` first and `hit_inner` is not edited; Outcome (design): a box that clips the select clips no row."
    basis: packages/blitz-dom/src/document.rs:2404-2406
  - detector: D-cascade
    severity: warning
    file: .andromeda/design-system.md
    near: 85
    section: "## Depth Strategy"
    change: "Keep the broad line and add at its end: a row of an open select list stands outside this, answered before the walk, so a box clipping the select clips no row."
    sidecar: "Depth Strategy broad line: an open list's rows named as outside the clip rule."
    rationale: "The broad line states the rule the detail's line 26 details, and the report's `select_list_hit` answering first makes it false unqualified where a list stands open."
    basis: packages/blitz-dom/src/document.rs:2404-2406
    dependent-of: D-cascade
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/depth-strategy.md
    near: 26
    section: "## Depth Strategy"
    change: "New item after this line: - The open select list answers the hit first in `hit_with_scrollbar`, with no scrollbar answered with it, so `hit`, `set_hover_to`, `handle_pointerdown` and the driver's `covered` all read it, and a button under the list is refused `covered` (packages/blitz-dom/src/document.rs:962-988; packages/blitz-dom/src/document.rs:2404-2406; tests/blitz-tests/tests/stand_act_popup.rs:228-245; as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)"
    sidecar: "Depth Strategy: the open list answers the hit first."
    rationale: "Report, Symbols / APIs, The hit, and the test `a_button_under_the_list_is_refused_covered`."
    basis: packages/blitz-dom/src/document.rs:2404-2406
    fact: F10
  - detector: D-cascade
    severity: warning
    file: .andromeda/design-system-detail/depth-strategy.md
    near: 28
    section: "## Depth Strategy"
    change: "After the padding-box clause, add: for a row or a group of an open select list it answers that list's scrollport, reading no ancestor's clipping box (packages/blitz-dom/src/scrolling.rs:754-756; as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)."
    sidecar: "Depth Strategy: `visible_region` answers the list's scrollport for an open list's row or group."
    rationale: "Report, Counts / qualifiers moved, `visible-reg`, names depth-strategy:28 changed; Symbols / APIs, Seen through the list, states the answer."
    basis: packages/blitz-dom/src/scrolling.rs:754-756
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/depth-strategy.md
    near: 34
    section: "## Depth Strategy"
    change: "New item after this line: - The open select list paints last in `paint_scene`, after the root's render and before the devtools overlays, so it stands above every box of the page (packages/blitz-paint/src/render.rs:246-247; packages/blitz-paint/src/render/select_list.rs:21-40; as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)"
    sidecar: "Depth Strategy: the open list paints last, under the devtools overlays alone."
    rationale: "Report, Symbols / APIs, Paint: `paint_scene` calls `paint_select_list` after the root's render and before the devtools overlays."
    basis: packages/blitz-paint/src/render.rs:246-247
    fact: F8
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/depth-strategy.md
    near: 37
    section: "## Depth Strategy"
    change: "New item after this line: - The open select list paints whole past an `opacity` ancestor and past a box that clips its select; a non-finite box is unpainted and only rows in the scrollport are laid out (packages/blitz-paint/src/render/select_list.rs:21-52; tests/blitz-tests/tests/select_popup_painted.rs:211-235; as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)"
    sidecar: "Depth Strategy: the open list paints past an `opacity` ancestor and a clipping box."
    rationale: "Report, Symbols / APIs, Paint, and Outcome (design): `a_list_paints_whole_past_an_opacity_ancestor_and_a_box_that_clips_its_select`."
    basis: tests/blitz-tests/tests/select_popup_painted.rs:211-235
    fact: F9
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/motion.md
    near: 9
    section: "## Motion (calibrated to expression level `unrecorded`)"
    change: "New item after this line: - A wheel over the open select list scrolls the list; a wheel elsewhere closes it and then scrolls; a key brings the current row into view at once (packages/blitz-dom/src/document.rs:947-960; packages/blitz-dom/src/scrolling.rs:669-672; tests/blitz-tests/tests/select_popup.rs:870-911; as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)"
    sidecar: "Motion: a wheel and a key on the open list."
    rationale: "Report, Symbols / APIs, Pointer: `handle_wheel` scrolls the list over it, else `close_select_list`, then scrolls; Seen through the list: a row's into-view writes `scroll_y` at once."
    basis: packages/blitz-dom/src/scrolling.rs:669-672
    fact: F12
  - detector: D-cascade
    severity: warning
    file: .andromeda/design-system-detail/motion.md
    near: 12
    section: "## Motion (calibrated to expression level `unrecorded`)"
    change: "After the clause on the viewport, add: on a row of an open select list it writes that list's `scroll_y` at once and returns, moving no chain and no viewport (packages/blitz-dom/src/scrolling.rs:669-672; as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)."
    sidecar: "Motion: into-view on an open list's row writes the list's scroll alone."
    rationale: "Report, Symbols / APIs, Seen through the list: `scroll_into_view` on a row writes the list's `scroll_y` at once and returns, no chain or viewport moving."
    basis: packages/blitz-dom/src/scrolling.rs:669-672
  - detector: D-cascade
    severity: warning
    file: .andromeda/design-system.md
    near: 121
    section: "## Motion (calibrated to expression level `unrecorded`)"
    change: "Keep the broad line and add at its end: into view on a row of an open select list writes that list's own scroll at once and moves no other box and no viewport."
    sidecar: "Motion broad line: an open list's row named as the into-view exception."
    rationale: "The broad line states the rule the detail's line 12 details, and the report's row arm returns before any chain or viewport is written."
    basis: packages/blitz-dom/src/scrolling.rs:669-672
    dependent-of: D-cascade
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/motion.md
    near: 28
    section: "## Motion (calibrated to expression level `unrecorded`)"
    change: "New item after this line: - The select list has no motion: opening, closing and moving the highlight show at once, and a screen read after a step holds the new state with no `advance` (tests/blitz-tests/tests/stand_act_popup.rs:408-423; as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)"
    sidecar: "Motion: the select list has no motion and needs no `advance`."
    rationale: "Report, Outcome (design): no motion, no `advance` needed, met by `a_screen_read_after_a_step_reads_the_new_state_with_no_advance`, the reduced-motion gate entry green."
    basis: tests/blitz-tests/tests/stand_act_popup.rs:408-423
    fact: F11
  - detector: D-design-tokens
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 11
    section: "## Surface: desktop-native › ### Tokens (platform-specific)"
    change: "New item after this line: - The open select list's colours are in no sheet: the highlight's text and background, a disabled row's text and the box's border are associated constants on `SelectListView` — engine constants on the founder's F6, no design token; their values at §Color Palette → Core Colors (packages/blitz-dom/src/node/select_list.rs:110-123; as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)"
    sidecar: "Surface desktop-native tokens: the list's colours recorded as engine constants, no token."
    rationale: "Report, Coverage: the list's `tokens` flag reads hardcoded✗, engine constants on his F6; `select-colours` names surface:9's neighbour changed, placed here at the nearest picked entity, which points to the palette and copies no value."
    basis: packages/blitz-dom/src/node/select_list.rs:110-123
  - detector: D-retired
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 13
    section: "## Surface: desktop-native › ### Tokens (platform-specific)"
    change: "The claim, its owner phrase gone, now reads: A single select's list is painted: a click, Enter, or a Space with no typed text standing opens it, a group's `label` is named in the accessibility tree and painted as a header, and `size` is read on a `multiple` select and stays unread on a single select."
    sidecar: "Surface desktop-native tokens: the retired select limits replaced by the painted list, the owner phrase dropped."
    rationale: "Report, Retired names the limits; Counts / qualifiers moved: `no-popup`, `enter-space`, `size-unread` and `group-label` each change surface:13, and the owner rows drop its owner phrase; Symbols / APIs, `size`: a single select's stays unread."
    basis: .andromeda/design-system-detail/surface.md:13
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 13
    section: "## Surface: desktop-native › ### Tokens (platform-specific)"
    change: "New item after this line: - On desktop-native the select's list is an engine overlay reaching the screen through `paint_scene` alone, not a second OS window and not rows as boxes; the windowed list was read and not run (packages/blitz-paint/src/render.rs:246-247; packages/blitz-paint/src/render/select_list.rs:21-40; as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)"
    sidecar: "Surface desktop-native: the select's list is an engine overlay."
    rationale: "Report, Outcome (arch): through `paint_scene` alone, no shell or renderer moved, the windowed list read and not run; Reverted: the rejected approaches stand unbuilt."
    basis: packages/blitz-paint/src/render.rs:246-247
    fact: F13
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 13
    section: "## Surface: desktop-native › ### Tokens (platform-specific)"
    change: "New item after this line: - A click, Enter, or a Space with no typed text standing opens a single select's list on its chosen option and fires no event; keys and the pointer move the highlight over enabled rows and choose nothing (packages/blitz-dom/src/events/keyboard.rs:215-250; packages/blitz-dom/src/events/pointer.rs:595-611; tests/blitz-tests/tests/select_popup.rs:202-230; tests/blitz-tests/tests/select_popup.rs:249-273; tests/blitz-tests/tests/select_popup.rs:751-781; as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)"
    sidecar: "Surface desktop-native: what opens a select's list and what moves its highlight."
    rationale: "Report, Symbols / APIs, Pointer and Keys, and the checks in `select_popup`."
    basis: tests/blitz-tests/tests/select_popup.rs:202-230
    fact: F14
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 13
    section: "## Surface: desktop-native › ### Tokens (platform-specific)"
    change: "New item after this line: - Enter, Space, Tab in either direction or a click on a row chooses the row with one input then one change; a disabled row changes nothing and leaves the list open; a choice of the chosen option fires nothing (packages/blitz-dom/src/events/keyboard.rs:252-267; packages/blitz-dom/src/events/pointer.rs:642-655; tests/blitz-tests/tests/select_popup.rs:294-327; tests/blitz-tests/tests/select_popup.rs:329-361; tests/blitz-tests/tests/select_popup.rs:363-375; tests/blitz-tests/tests/select_popup.rs:377-388; as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)"
    sidecar: "Surface desktop-native: how a select's list chooses."
    rationale: "Report, Symbols / APIs, Keys and Pointer; Outcome (arch): `click_list_row` and `choose_current_and_close` call the one write path."
    basis: tests/blitz-tests/tests/select_popup.rs:294-327
    fact: F15
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 13
    section: "## Surface: desktop-native › ### Tokens (platform-specific)"
    change: "New item after this line: - Escape, a second click, a click outside, a wheel elsewhere and the page scrolling each close the list with no event; removing, disabling, hiding or making its select inert closes it too (packages/blitz-dom/src/document.rs:881-907; packages/blitz-dom/src/events/pointer.rs:619-633; tests/blitz-tests/tests/select_popup.rs:401-426; tests/blitz-tests/tests/select_popup.rs:478-520; as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)"
    sidecar: "Surface desktop-native: what closes a select's list."
    rationale: "Report, Symbols / APIs, Pointer, The resolve and Document; Outcome (security): a removed, disabled, hidden or inert select closes it."
    basis: tests/blitz-tests/tests/select_popup.rs:401-426
    fact: F16
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 13
    section: "## Surface: desktop-native › ### Tokens (platform-specific)"
    change: "New item after this line: - A disabled, inert or hidden select, or one with no row to show, opens nothing; a `multiple` select opens no list; a label's click focuses a single select and opens nothing (tests/blitz-tests/tests/select_popup.rs:428-441; tests/blitz-tests/tests/select_popup.rs:443-472; as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)"
    sidecar: "Surface desktop-native: which selects open no list."
    rationale: "Report, Symbols / APIs, Pointer: the `select` and `label` arms; Deviations: a select with no row to show opens nothing."
    basis: tests/blitz-tests/tests/select_popup.rs:443-472
    fact: F17
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 13
    section: "## Surface: desktop-native › ### Tokens (platform-specific)"
    change: "New item after this line: - In the list a group reads as a header with its options under it, and a header chooses nothing; a group with no label reads the whole list's box (tests/blitz-tests/tests/select_popup.rs:805-855; as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)"
    sidecar: "Surface desktop-native: a group in the select's list."
    rationale: "Report, Symbols / APIs, Pointer: the new `optgroup` arm; Deviations: a group with no label reads the whole list's box; the check `groups_read_as_headers_their_options_under_them_and_a_header_chooses_nothing`."
    basis: tests/blitz-tests/tests/select_popup.rs:805-855
    fact: F18
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 63
    section: "## Surface: desktop-native › ### Platform-Specific Notes"
    change: "New item after this line: - A scrolled select list paints a thumb under `feature = \"scrollbars\"` that takes no hit and is not draggable; keys, the wheel and `scroll` move the rows (packages/blitz-paint/src/render/select_list.rs:130-154; tests/blitz-tests/tests/select_popup_painted.rs:312-351; as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)"
    sidecar: "Surface desktop-native notes: the select list's thumb under the `scrollbars` feature."
    rationale: "Report, Symbols / APIs, Paint: `paint_list_thumb` under `feature = \"scrollbars\"`; The hit: no scrollbar answered with the list; the check `a_scrolled_list_paints_its_thumb_which_takes_no_hit`."
    basis: packages/blitz-paint/src/render/select_list.rs:130-154
    fact: F19
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 13
    section: "## Surface: desktop-native › ### Tokens (platform-specific)"
    change: "New item after this line: - The highlighted row is in no snapshot and no tree; the tree reads `ComboBox`, `expanded` and the chosen option `selected`; an author's `open` steers nothing on a select (packages/blitz-dom/src/accessibility.rs:238-242; tests/blitz-tests/tests/select_popup.rs:565-614; tests/blitz-tests/tests/select_popup.rs:628-651; as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)"
    sidecar: "Surface desktop-native: what the tree and the snapshot read of an open list."
    rationale: "Report, Symbols / APIs, Tree; Coverage, tree expanded on a single select: the highlight unread by the tree, stated."
    basis: packages/blitz-dom/src/accessibility.rs:238-242
    fact: F20
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 13
    section: "## Surface: desktop-native › ### Tokens (platform-specific)"
    change: "New item after this line: - A row of an open list carries its own `bounds` in the snapshot, and a `screenshot` of a row crops to it and leaves the list open (packages/dioxus-native-dom/src/snapshot.rs:152-155; packages/blitz-dom/src/document.rs:995-1012; tests/blitz-tests/tests/stand_act_popup.rs:287-316; as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)"
    sidecar: "Surface desktop-native: an open list's row in the snapshot and a shot."
    rationale: "Report, Symbols / APIs, Bridge; Schema / config: a row's own `bounds` and pixels in a `screenshot`; Outcome (design): `a_shot_of_a_row_crops_to_it_and_leaves_the_list_open`."
    basis: tests/blitz-tests/tests/stand_act_popup.rs:287-316
    fact: F21
note: "F1, F2 and F7 stay with their own categories (the `select[multiple][size]` sheet rules are F7, spacing's); the report's surface:9 neighbour is placed at surface.md:11, line 9 being unpicked; the report measures no cursor over a list row, so iconography gains no claim."
