proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 9
    section: "## Standard Contracts"
    change: "after «what does mark it is stated under §Cross-cutting Patterns → Invalidation and state integrity» add « — an open list's opening or closing marks its select, a highlight or a wheel inside it nothing»"
    sidecar: "Document core: the changed-set contract names an open list's open and close as marking its select."
    rationale: "Report Counts `changed-set` names this line changed; Spec claims disproved: a close marks the select through `reconcile_select_list` or `close_select_list`, a highlight or a wheel inside the list marks nothing."
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 18
    section: "## Standard Contracts"
    change: "replace «its one non-test caller `execute.rs` `focus_multiple_select`» with «its one non-test caller `execute.rs` `focus_select`»; replace «`dioxus_native_dom::closed_select_of`, called by the snapshot's builder and `lint.rs`, and `is_multiple_select`, called by `closed_select_of` and `execute.rs` `locate`, public and re-exported» with «`dioxus_native_dom::closed_select_of`, called by the snapshot's builder and `lint.rs` and answering `None` for a select whose list is open, `is_multiple_select`, called by `closed_select_of` alone within the workspace and still re-exported, and `is_select`, called by `execute.rs` `locate` alone, public and re-exported»"
    sidecar: "Document core: `focus_multiple_select` retired for `focus_select`; `closed_select_of`'s open answer and the select tests' callers restated."
    rationale: "Report Retired names `focus_multiple_select`; Symbols / APIs Driver and Bridge state the renames and the remaining callers; Counts `focus-select` and `closed-option` name this line."
    dependent-of: D-retired
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 19
    section: "## Standard Contracts"
    change: "add row «- An open select list's public surface on `BaseDocument`, since 2026-10-31-select-popup: read-only `open_select(&self) -> Option<NodeId>`, `select_list_bounds(&self, node: NodeId) -> Option<BoundingRect>`, `select_list_view(&self) -> Option<SelectListView>` and `select_row_layout(&self, row: NodeId, text: &str) -> Option<parley::Layout<TextBrush>>`, and the writer `close_select_list(&mut self)`, its callers keyboard, pointer, reconcile and the driver's `focus_select`; each reads through `get_node`, a row with no styles `None`, with no `layout_style()` (packages/blitz-dom/src/document.rs:881-889; packages/blitz-dom/src/document.rs:990-993; packages/blitz-dom/src/document.rs:995-1012; packages/blitz-dom/src/document.rs:1022-1052; packages/blitz-dom/src/document.rs:1054-1064; as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md).»"
    sidecar: "Document core: five new public `BaseDocument` methods of the select list registered."
    rationale: "Report Symbols / APIs `BaseDocument` and Coverage `BaseDocument public readers` land five public methods; none appears in this file."
    basis: packages/blitz-dom/src/document.rs:990-993
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 19
    section: "## Standard Contracts"
    change: "add row «- An open select list's state in the document: one field `select_list: Option<SelectList>`, `None` at construction, read through the `pub(crate)` `live_select_list`, past it only by `reconcile_select_list`, which must see a list the reader no longer answers, and by the removal arm, with `hover_list_row` writing `current` directly; crate-private `select_list_for`, `open_select_list`, `select_list_options`, `move_select_list_current`, `scroll_select_list_to`, `scroll_select_list_by`, `select_list_hit` and `select_list_region`, private `select_list_allowed`, `measure_select_list` and `text_row_layout`; `resolve` calls `reconcile_select_list` after layout and before `refresh_hover`, and `clear_interaction_state_for_removed_node` clears the field for its select; a select with no row to show opens nothing; an engine change for every document, remote pages included (packages/blitz-dom/src/document.rs:343-344; packages/blitz-dom/src/document.rs:797-804; packages/blitz-dom/src/document.rs:1408-1410; packages/blitz-dom/src/resolve.rs:133-135; as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md).»"
    sidecar: "Document core: the list's one field, its crate-private methods and its reconcile in `resolve` stated."
    rationale: "Report Symbols / APIs `BaseDocument` and The resolve, Deviations on `live_select_list`, Coverage `select list, engine overlay`; Expected amendments name §Standard Contracts → Document core."
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 26
    section: "## Standard Contracts"
    change: "replace «and an `optgroup`'s (`Role::Group`), named in the tree and unpainted» with «and an `optgroup`'s (`Role::Group`), named in the tree and painted as a header in an open list»"
    sidecar: "Mutation, query and CSSOM: a group's label no longer reads unpainted."
    rationale: "Report Retired names «a group's label is unpainted»; Counts `group-label` names this line."
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 30
    section: "## Standard Contracts"
    change: "after «closed values inside the states class, unmeasured at an assistive technology like the states» add «, a single select writing `set_expanded(open)` and reading no `aria-expanded`, every other element keeping that read»"
    sidecar: "Mutation, query and CSSOM: a single select's `expanded` is native, `aria-expanded` unread on it."
    rationale: "Report Symbols / APIs Tree; Counts `expanded` names this line changed and the attribute read still true on every other element."
    dependent-of: D-new-detail
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 32
    section: "## Standard Contracts"
    change: "add row «- A single select in the tree: `build_accessibility_node` writes `set_expanded(open)` from the engine's own state and reads no `aria-expanded`, a closed bool and no author attribute, every other element keeping that read; the role stays `ComboBox`, the chosen option reads `selected` and the tree's focus stays on the select; the highlighted row is unread by the tree, and `:open` and `flush_open_state` are unmoved, so an author's `open` on a select steers nothing; an engine change for every document (packages/blitz-dom/src/accessibility.rs:238-242; tests/blitz-tests/tests/select_popup.rs:565-614; tests/blitz-tests/tests/select_popup.rs:628-651).»"
    sidecar: "Mutation, query and CSSOM: the tree's reading of an open single select stated."
    rationale: "Report Symbols / APIs Tree and Coverage `tree expanded on a single select`."
    basis: packages/blitz-dom/src/accessibility.rs:238-242
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 42
    section: "## Standard Contracts"
    change: "add row «- An open select list under the pointer: `handle_pointerdown` returns on a press in the list and else runs `close_list_outside`; `handle_click`'s `select` arm opens a single select's list or closes one standing, the new private `focus_select` holding what the arm did before, its `option` arm gains `click_list_row` behind `click_lands_on_row`, a new `optgroup` arm stands, and the `label` arm focuses a bound single select and opens nothing; `handle_pointermove` calls `hover_list_row`, a press in the list starts no text drag (`pressed_in_list`) and the release chooses the row under it; `handle_wheel` scrolls the list over it, else runs `close_select_list` and then scrolls; `click_list_row` chooses through `apply_select_choice`, the one write path; an engine change for every document (packages/blitz-dom/src/events/pointer.rs:415-420; packages/blitz-dom/src/events/pointer.rs:595-656; packages/blitz-dom/src/events/pointer.rs:1108-1120; as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md).»"
    sidecar: "Events: the pointer's default actions on an open select list stated."
    rationale: "Report Symbols / APIs Pointer, Outcome (arch) the one write path; a surface this contract group states for every other control and no line here holds."
    basis: packages/blitz-dom/src/events/pointer.rs:595-656
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 42
    section: "## Standard Contracts"
    change: "add row «- An open select list under the keys: `press_select_key` opens on Enter, and on a Space with no typed text standing; `press_open_list_key` is the open list's closed key match; Enter, Space, Tab and a click on a row choose it through `choose_current_and_close` or `click_list_row`, one `input` then one `change`, a choice of the chosen option firing nothing and a disabled row leaving the list open; Tab runs `choose_current_and_close` before moving focus, either direction; a close clears the typed text; `apply_row_gesture` calls `scroll_into_view` on the current row only where `carries_valid_size`; `press_multiple_select_key` and `click_select_row` keep their semantics; an engine change for every document (packages/blitz-dom/src/events/keyboard.rs:32-34; packages/blitz-dom/src/events/keyboard.rs:215-250; packages/blitz-dom/src/events/keyboard.rs:252-267; packages/blitz-dom/src/events/keyboard.rs:423-430; packages/blitz-dom/src/events/keyboard.rs:452-459; tests/blitz-tests/tests/select_popup.rs:294-327; as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md).»"
    sidecar: "Events: the keys' default actions on an open select list stated."
    rationale: "Report Symbols / APIs Keys and Deviations (a close clears the typed text); the chosen-row test named in New text, by line."
    basis: packages/blitz-dom/src/events/keyboard.rs:215-250
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 48
    section: "## Standard Contracts"
    change: "after «its signature unchanged and for every caller: script `scrollIntoView`, Dioxus `MountedData::scroll_to`, fragment navigation and the test harness's input helper» add «, a row of an open list excepted, which writes the list's `scroll_y` at once and returns, no chain or viewport moving (packages/blitz-dom/src/scrolling.rs:669-672)»"
    sidecar: "Scrolling: `scroll_into_view` on an open list's row writes the list's `scroll_y` alone."
    rationale: "Report Symbols / APIs Seen through the list: the row case moves no chain or viewport, so «every scrolling box on its target's chain» no longer holds for it."
    basis: packages/blitz-dom/src/scrolling.rs:669-672
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 49
    section: "## Standard Contracts"
    change: "after «reached through `BaseDocument::hit`, `hit_with_scrollbar` and `element_from_point`» add «, `hit_with_scrollbar` answering `select_list_hit` first with no scrollbar (packages/blitz-dom/src/document.rs:2404-2406)»"
    sidecar: "Scrolling: the hit's reach restated, an open list answering first in `hit_with_scrollbar`."
    rationale: "Report Symbols / APIs The hit; Counts `hit-first` names this line."
    basis: packages/blitz-dom/src/document.rs:2404-2406
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 49
    section: "## Standard Contracts"
    change: "after «(`visible_region` under transforms, below) (packages/blitz-dom/src/scrolling.rs:740-796)» add «, a row or a group of an open list reading the list's scrollport instead (packages/blitz-dom/src/scrolling.rs:754-756)»"
    sidecar: "Scrolling: `visible_region` answers the list's scrollport for a row or a group of an open list."
    rationale: "Report Symbols / APIs Seen through the list: the reader's definition here, a viewport narrowed along the chain, does not hold for a row of an open list."
    basis: packages/blitz-dom/src/scrolling.rs:754-756
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 51
    section: "## Standard Contracts"
    change: "after «and `visible_region` reads it through its transform too» add «, a row of an open list reading the list's scrollport alone»"
    sidecar: "Scrolling: the transformed-box reading states an open row's scrollport carve-out."
    rationale: "Report Counts `visible-reg` names this line changed; `panic-pos` on this line stays no-change, as the report states."
    dependent-of: D-new-detail
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 59
    section: "## Standard Contracts"
    change: "replace «Its callers `execute.rs` `in_view` and `crop` and `lint.rs` `clip_region` are unchanged» with «Its callers `execute.rs` `in_view` and `crop` are unchanged, and `lint.rs` `clip_region` takes an open row's region from it alone, no `contain: paint` cut»"
    sidecar: "`visible_region` under transforms: `clip_region` no longer reads as an unchanged caller."
    rationale: "Report Symbols / APIs Driver: `clip_region` takes an open row's region from `visible_region` alone; the line's «unchanged» is retired for that caller, found by this detector's own sweep."
    dependent-of: D-new-detail
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 61
    section: "## Standard Contracts"
    change: "replace «and its callers `hit` and `hit_with_scrollbar` are unchanged» with «and its callers are edited only in that `hit_with_scrollbar` answers `select_list_hit` first, so `hit` reads an open list through it»"
    sidecar: "Hit walk: `hit_inner` not edited, its caller `hit_with_scrollbar` answering the list first."
    rationale: "Report Symbols / APIs The hit; Counts `hit-first` names this line."
    dependent-of: D-new-detail
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 62
    section: "## Standard Contracts"
    change: "add row «- An open list under a point: `select_list_hit` answers a point in the list before any walk and no scrollbar with it, so `hit` (the driver's `covered` through it), `set_hover_to` and `handle_pointerdown` all read the list, a row is never a thumb drag and `hit_inner` is not edited; a box that clips the select clips no row, and the list's painted thumb takes no hit; an engine change for every document (packages/blitz-dom/src/document.rs:962-988; packages/blitz-dom/src/document.rs:2404-2406; tests/blitz-tests/tests/select_popup.rs:677-716; tests/blitz-tests/tests/select_popup_painted.rs:312-351).»"
    sidecar: "Scrolling, selection, tree: `select_list_hit` and what reads it stated."
    rationale: "Report Symbols / APIs The hit; Counts `hit-first` (a row never a thumb drag); the two named tests in New text, by line."
    basis: packages/blitz-dom/src/document.rs:962-988
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 84
    section: "## Standard Contracts"
    change: "add row «- The crate-private module `node::select_list` holds an open list's state and geometry: `SelectList` (`select`, `current`, `scroll_y`, `row_height`, `width`, `anchor`, `fresh`, the last because a click focuses and opens in one step, before a `:focus` rule's restyle, so a box-moving rule would read as moved and close the list at once), `Row`, `Placement`, `Geometry`, `size_rows`, `rows`, `group_label`, `row_label`, `option_rows`, `place`, `geometry`, `row_disabled`, `MAX_ROWS` = 20, `BORDER_WIDTH`, `ROW_PADDING_X`, `ROW_PADDING_Y` and `GROUP_INDENT`; the row height is the select's font on one line, and a group with no label reads the whole list's box (packages/blitz-dom/src/node/mod.rs:11; packages/blitz-dom/src/node/select_list.rs:28-46; packages/blitz-dom/src/node/select_list.rs:125-130; packages/blitz-dom/src/node/select_list.rs:147-192; packages/blitz-dom/src/node/select_list.rs:218-263; packages/blitz-dom/src/node/select_list.rs:265-293).»"
    sidecar: "Nodes and widgets: the crate-private `node::select_list` module registered."
    rationale: "Report Crates / modules names the new module; Symbols / APIs The list's state; Deviations on `fresh` and the details chosen at implement."
    basis: packages/blitz-dom/src/node/select_list.rs:28-46
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 84
    section: "## Standard Contracts"
    change: "add row «- `SelectListView` and `SelectListRow`, the list's two public read types, re-exported from the crate root as `ScrollbarRef` is; `SelectListView`'s associated constants `HIGHLIGHT_TEXT` `#000000`, `HIGHLIGHT_BACKGROUND` `#c8c8c8`, `DISABLED_TEXT` `#6d6d6d`, `BORDER` `#999999` and `BORDER_WIDTH` are engine constants, the engine holding no tokens; `SelectListRow.label` is page text in a read type, never logged (packages/blitz-dom/src/node/select_list.rs:78-91; packages/blitz-dom/src/node/select_list.rs:93-108; packages/blitz-dom/src/lib.rs:108-111).»"
    sidecar: "Nodes and widgets: public read types `SelectListView` and `SelectListRow` registered."
    rationale: "Report Symbols / APIs The list's state and Deviations (two public read types, the colours associated constants); Coverage on `SelectListRow.label`."
    basis: packages/blitz-dom/src/lib.rs:108-111
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 84
    section: "## Standard Contracts"
    change: "add row «- `size` on a `multiple` select: `stylo.rs` hints a `height` of `size × 1.2em` through `size_rows` (`parse_unsigned_integer`, capped at 65534, `0` or a bad value no hint), and the default sheet gives `select[multiple][size]` `overflow-y: auto` and its rows `line-height: 1.2`; a single select's `size` stays unread; a new attribute read, a parse into a capped whole number with no text carried out (packages/blitz-dom/src/stylo.rs:1124-1139; packages/blitz-dom/assets/default.css:183-193; packages/blitz-dom/src/node/select_list.rs:125-130).»"
    sidecar: "Nodes and widgets: the `size` reading on a `multiple` select stated."
    rationale: "Report Symbols / APIs `size`; Schema / config (new attribute read); Retired names «a `multiple` select's `size` is unread», stated nowhere in this file."
    basis: packages/blitz-dom/src/stylo.rs:1124-1139
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 105
    section: "## Standard Contracts"
    change: "after «tests/blitz-tests/tests/background_size.rs:46)» add «, which calls `paint_select_list` after the root's render and before the devtools overlays (packages/blitz-paint/src/render.rs:246-247)»"
    sidecar: "Paint and shell: `paint_scene` paints an open list last, before the devtools overlays."
    rationale: "Report Symbols / APIs Paint; Counts `paint-order` names this line."
    basis: packages/blitz-paint/src/render.rs:246-247
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 112
    section: "## Standard Contracts"
    change: "add row «- An open list's paint: the new `render/select_list.rs` holds `paint_select_list`, `device_rect`, `paint_list_box`, `paint_list_row` and, under the `scrollbars` feature, `paint_list_thumb`, a non-finite box unpainted and only rows in the scrollport laid out, past an `opacity` ancestor and a box that clips its select, with no shell or renderer moved; `text.rs`'s `stroke_text` delegates to the `pub(crate)` `stroke_text_as` with `RunPaint { color, synthesize_bold }`, every other caller passing `RunPaint::default()`; a synthesized bold adds one embolden weight to a build's own, and `hint` stays `!FONT_EMBOLDEN_ENABLED`, so a synthesized header on a plain build is emboldened with hinting on (packages/blitz-paint/src/render/select_list.rs:21-40; packages/blitz-paint/src/text.rs:586-593; packages/blitz-paint/src/text.rs:683-689; packages/blitz-paint/src/text.rs:695; tests/blitz-tests/tests/select_popup_painted.rs:211-235).»"
    sidecar: "Paint and shell: the module `render::select_list` and `stroke_text_as` registered."
    rationale: "Report Crates / modules names `render::select_list`; Symbols / APIs Paint; Outcome (arch) through `paint_scene` alone."
    basis: packages/blitz-paint/src/text.rs:586-593
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 178
    section: "## Standard Contracts"
    change: "before « — the remedy, the eight causes and their names unchanged» add «, and since 2026-10-31-select-popup the clause «an option of a closed select lies under it: click the select first, then the option»»"
    sidecar: "Driver session: `covered`'s meaning gains the closed-option clause, name, remedy and order unchanged."
    rationale: "Report Symbols / APIs Driver (`refusal.rs`) and Counts `covered-text` name this line."
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 189
    section: "## Standard Contracts"
    change: "replace «on every target but a `multiple` select, which it focuses with no click (below)» with «on every target but a select, single or `multiple`, which it focuses with no click (below)»"
    sidecar: "Driver session: `type` focuses any select with no click."
    rationale: "Report Symbols / APIs Driver: `type` on any select focuses with no click; Counts `type-focus` names this line."
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 190
    section: "## Standard Contracts"
    change: "replace «`type` on a `multiple` select: `Target` carries `multiple`, and `type` does not click such a target, a click landing on the row at the centre: the crate-private `focus_multiple_select` focuses it with no click and clears the typeahead, without which a second `type` joins the first's text; a text opening on a space types nothing, since no label can match and the Space would flip a row» with «`type` on a select, single or `multiple`: `Target` carries `select`, and `type` does not click such a target, a click opening a single select's list and landing on a `multiple` select's row at the centre: the crate-private `focus_select` focuses it with no click, closes any list and clears the typeahead, without which a second `type` joins the first's text; it fires no DOM focus events on a single select either; a text opening on a space types nothing, since no label can match and the Space would flip a row or open the list»"
    sidecar: "Driver session: `Target.multiple` and `focus_multiple_select` retired for `Target.select` and `focus_select`, over any select."
    rationale: "Report Retired names `Target.multiple` and `focus_multiple_select`; Symbols / APIs Driver; Counts `focus-select` and `type-focus` name this line."
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 192
    section: "## Standard Contracts"
    change: "add row « Three fixed helps restated at 2026-10-31-select-popup, each the program's own `'static` text: `click`'s says a click on a select opens its list, `press`'s names a closed select's keys, and `type`'s is restated with them; `VERBS` stays 12, the causes 8, and `describe`'s count row `[12, 13, 12, 3, 8, 4, 15]` is unmoved (packages/escher-driver/src/schema.rs:222-226; packages/escher-driver/src/schema.rs:240-243; packages/escher-driver/src/schema.rs:264-269).»"
    sidecar: "Driver session: `click`'s, `type`'s and `press`'s fixed help restated, the count row unmoved."
    rationale: "Report Symbols / APIs Driver (`schema.rs`) and Schema / config; Outcome (arch) verbs 12, causes 8; the two help heads in New text, by line."
    basis: packages/escher-driver/src/schema.rs:222-226
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 192
    section: "## Standard Contracts"
    change: "add row « A select's list through the driver: a `click` on a single select opens its list and a second on one point closes it, each focusing; `press` opens, moves and chooses by its keys; a row's own `bounds` cross in the snapshot and the diff, `differs` naming `bounds`, and a shot holds an option's label and a group's `label` as pixels, a boundary widening on the founder's forks (inputs#I4 of 2026-10-31-select-popup); a row past the scrollport reads `off-screen` until a `scroll` brings it in, a control under the list is refused `covered`, an element shot of a row crops to it and leaves the list open, and `type` leaves no list open; an option of a closed select stays refused `covered`; no key name, node key, field, cause, session error, exit, carrier, listener, port, env var, log field or span is added (tests/blitz-tests/tests/stand_act_popup.rs:58-101; tests/blitz-tests/tests/stand_act_popup.rs:157-166; tests/blitz-tests/tests/stand_act_popup.rs:198-226; tests/blitz-tests/tests/stand_act_popup.rs:228-245; tests/blitz-tests/tests/stand_act_popup.rs:247-285; tests/blitz-tests/tests/stand_act_popup.rs:287-316; tests/blitz-tests/tests/stand_act_popup.rs:379-406; as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md).»"
    sidecar: "Driver session: what an open list reads through the verbs and the carriers stated."
    rationale: "Report Schema / config (the list on the ratified carriers, the new pixel content), Coverage `the list's pixels on the screenshot carriers`, Expected amendments (Boundary widening); tests named in New text, by line."
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 212
    section: "## Standard Contracts"
    change: "replace «whose visible parts intersect, a closed select's options excepted (below)» with «whose visible parts intersect, a closed select's options and an open list's rows and groups excepted (below)»; after «the lint's own walk, `visible_region` unchanged» add «, an open list's row taking its region from `visible_region` alone, no `contain: paint` cut»"
    sidecar: "Visual lint: the `overlap` exception and the clip region state an open list's rows."
    rationale: "Report Symbols / APIs Driver (`lint.rs`); Counts `closed-option` names this line. The retired `Entry.in_closed_select`'s meaning stands here in prose."
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 213
    section: "## Standard Contracts"
    change: "replace «leaves out a closed select's options: `Entry.in_closed_select` reads through `closed_select_of`, by the element and never the role, so» with «leaves out a closed select's options and an open list's rows and groups: `Entry.in_select_box` reads through `closed_select_of`, by the element and never the role, and is true also for an open list's row or group, so»"
    sidecar: "Visual lint: `Entry.in_closed_select` retired for `Entry.in_select_box`."
    rationale: "Report Retired names `Entry.in_closed_select`; Symbols / APIs Driver (`lint.rs`); Counts `closed-option` and `lint-exclude` name this line."
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 220
    section: "## Standard Contracts"
    change: "after «`stand_lint` pinning `at_boot` as `[0, 0, 0, 0]` in both layout modes» add «; an open list's rows read no `overlap` either, the planted pair still reading (tests/blitz-tests/tests/stand_lint.rs:493-519; tests/blitz-tests/tests/stand_act_popup.rs:318-350)»"
    sidecar: "Visual lint: the `overlap` pin states an open list's rows."
    rationale: "Report Counts `closed-option` and `lint-exclude` name this line; Outcome (lint) names both checks."
    dependent-of: D-retired
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 278
    section: "## Standard Contracts"
    change: "add row «- `is_select`, public and re-exported beside `is_multiple_select`, is the select test the driver reads, by the element and never the role, its one caller `execute.rs` `locate` (packages/dioxus-native-dom/src/lib.rs:37).»"
    sidecar: "Dioxus DOM bridge: public `is_select` registered."
    rationale: "Report Symbols / APIs Bridge and Driver: new public `is_select`, re-exported, its remaining caller `locate` alone; the file holds no such name."
    basis: packages/dioxus-native-dom/src/lib.rs:37
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 279
    section: "## Standard Contracts"
    change: "at the row's end add « `closed_select_of` answers `None` for a select whose list is open, where the snapshot's builder reads `select_list_bounds(node)` before the client rect, so a row or a group of an open list reads its own `bounds`; `NodeState` gains no field and `NODE_FIELDS` stays thirteen (packages/dioxus-native-dom/src/snapshot.rs:152-155).»"
    sidecar: "Dioxus DOM bridge: an open list's rows read their own `bounds` in the snapshot."
    rationale: "Report Symbols / APIs Bridge; Counts `closed-option` names this line."
    basis: packages/dioxus-native-dom/src/snapshot.rs:152-155
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 312
    section: "## Standard Contracts"
    change: "replace «fifteen named blitz-tests targets outside that prefix» with «seventeen named blitz-tests targets outside that prefix, `select_popup` and `select_popup_painted` the last two,»"
    sidecar: "CI contracts: `stand-repeat`'s named targets fifteen to seventeen."
    rationale: "Report Harness / gate surface; Counts `repeat-names` names this line."
    basis: .github/scripts/ci-leg.sh:59-60
note: "no D-arch-decisions proposal, no dependency or locked decision moved in this file; the widening clause in the near-192 driver row rests on inputs#I4 and is P2's halt to confirm; this file was written twice, the second Write a whole-file correction of the first, against the task's one-call rule."
