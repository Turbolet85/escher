proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 9
    section: "## Standard Contracts"
    change: "On line 9, after «what does mark it is stated under §Cross-cutting Patterns → Invalidation and state integrity» add « — a select list's open and close, which mark the select, among it»; the rest of the line stands."
    sidecar: "contract-document-core: the changed set's pointer names a select list's open and close."
    rationale: "Report Counts `changed-set`: open and close mark the select, a new kind of site, with standard-contracts:9 among the changed sites."
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 17
    section: "## Standard Contracts"
    change: "New row following line 17: - `BaseDocument` holds one field `select_list: Option<SelectList>`, `None` at construction (packages/blitz-dom/src/document.rs:343-344); public read-only `open_select`, `select_list_bounds`, `select_list_view` and `select_row_layout`; public writer `close_select_list`, its callers keyboard, pointer, reconcile and the driver's `focus_select`; crate-private `live_select_list`, `select_list_for`, `open_select_list`, `reconcile_select_list`, `select_list_options`, `move_select_list_current`, `scroll_select_list_to`, `scroll_select_list_by`, `select_list_hit` and `select_list_region`; private `select_list_allowed`, `measure_select_list` and `text_row_layout` (packages/blitz-dom/src/document.rs:761-1089)."
    sidecar: "contract-document-core: the select list's field and readers registered."
    rationale: "Report Symbols, `BaseDocument`: one new field, four public readers, one public writer and the crate-private and private rest."
    basis: packages/blitz-dom/src/document.rs:761-1089
    fact: F10
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 17
    section: "## Standard Contracts"
    change: "New row following line 17: - `live_select_list`, `pub(crate)` with every caller in `document.rs`, is the reader of the list's state (packages/blitz-dom/src/document.rs:797-804), with two exceptions reading the field past it: `reconcile_select_list` (packages/blitz-dom/src/document.rs:894), which must see a list the reader no longer answers, and the removal arm (packages/blitz-dom/src/document.rs:1408); `hover_list_row` writes `current` directly (packages/blitz-dom/src/events/pointer.rs:604)."
    sidecar: "contract-document-core: the list's one reader and its two stated exceptions."
    rationale: "Report Symbols names `live_select_list` the reader; Deviations from intent names the two reads past it and the direct write."
    basis: packages/blitz-dom/src/document.rs:797-804
    fact: F11
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 17
    section: "## Standard Contracts"
    change: "New row following line 17: - `resolve` calls `reconcile_select_list` after layout and before `refresh_hover` (packages/blitz-dom/src/resolve.rs:133-135), and `clear_interaction_state_for_removed_node` clears the field for its select (packages/blitz-dom/src/document.rs:1408-1410)."
    sidecar: "contract-document-core: the list's reconcile step in `resolve` and its clearing on removal."
    rationale: "Report Symbols, The resolve."
    basis: packages/blitz-dom/src/resolve.rs:133-135
    fact: F14
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 18
    section: "## Standard Contracts"
    change: "On line 18: `clear_select_typeahead`'s one non-test caller reads `execute.rs` `focus_select`; `closed_select_of` keeps its stated callers and answers `None` for a select whose list is open; `is_multiple_select` reads «called by `closed_select_of` alone within the workspace, still re-exported» for «called by `closed_select_of` and `execute.rs` `locate`»; add `is_select`, public and re-exported, called by `execute.rs` `locate` alone, under the same stated reason."
    sidecar: "contract-document-core: `focus_multiple_select` retired for `focus_select`; the select tests' callers restated."
    rationale: "Report Retired names `focus_multiple_select`; report Symbols, Bridge and Driver, state `closed_select_of`'s open answer and the remaining callers of `is_select`, `is_multiple_select` and `clear_select_typeahead`."
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 26
    section: "## Standard Contracts"
    change: "On line 26, an `optgroup`'s label reads «named in the tree and painted as a header in an open list» for «named in the tree and unpainted»."
    sidecar: "contract-mutation-query-and-cssom: a group's label now paints as a header."
    rationale: "Report Retired names «a group's label is unpainted»; report Counts `group-label` lists standard-contracts:26 as changed."
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 27
    section: "## Standard Contracts"
    change: "On line 27, `aria-expanded` reads «`aria-expanded` (a boolean, a single select excepted, which reads none and writes `expanded` from its list)» for «`aria-expanded` (a boolean)»."
    sidecar: "contract-mutation-query-and-cssom: the `aria-expanded` read carves out a single select."
    rationale: "Report Symbols, Tree: a single select reads no `aria-expanded` and every other element keeps that read, so the unqualified read is no longer true of every element."
    basis: packages/blitz-dom/src/accessibility.rs:238-242
    dependent-of: D-new-detail
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 30
    section: "## Standard Contracts"
    change: "On line 30, after «closed values inside the states class» add «, and a single select's `expanded` a closed bool from engine state»; new row following line 30: - A single select writes `set_expanded(open)` and reads no `aria-expanded`, every other element keeping that read, and an author's `open` on a select steers nothing, `:open` and `flush_open_state` being unmoved (packages/blitz-dom/src/accessibility.rs:238-242)."
    sidecar: "contract-mutation-query-and-cssom: a single select's `expanded` comes from engine state."
    rationale: "Report Symbols, Tree; report Counts `expanded` lists standard-contracts:30 as changed; report Coverage gives the closed bool from engine state with no author attribute."
    basis: packages/blitz-dom/src/accessibility.rs:238-242
    fact: F17
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 41
    section: "## Standard Contracts"
    change: "New row following line 41: - Since 2026-10-31-select-popup a select's list takes these default actions: `handle_pointerdown` returns on a press in the list and else calls `close_list_outside` (packages/blitz-dom/src/events/pointer.rs:415-420); `handle_click`'s `select` arm opens a single select's list or closes one standing, its body in the private `focus_select` where it focuses, its `option` arm gains `click_list_row` behind `click_lands_on_row`, and it gains an `optgroup` arm; `handle_pointermove` calls `hover_list_row`, and a press in the list starts no text drag (`pressed_in_list`); `handle_wheel` scrolls the list over it, else calls `close_select_list` and then scrolls; Tab runs `choose_current_and_close` before moving focus, either direction (packages/blitz-dom/src/events/keyboard.rs:32-34); `press_select_key` opens on Enter, and on a Space with no typed text standing, a released Space doing nothing more; `press_open_list_key` is the open list's closed key match (packages/blitz-dom/src/events/keyboard.rs:215-250); a choice from the list goes through `apply_select_choice`, the same `input` then `change`."
    sidecar: "contract-events: the select list's pointer, wheel and key default actions."
    rationale: "Report Symbols, Pointer and Keys, the renamed test `enter_and_space_open_the_list_and_a_released_space_does_nothing_more`, and Outcome's one write path. New row because lines 38 to 40 stand near the line cap."
    basis: packages/blitz-dom/src/events/pointer.rs:415-420
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 41
    section: "## Standard Contracts"
    change: "New row following line 41: - `handle_click`'s `label` arm focuses a bound single select and opens nothing, no longer re-dispatching to it, asserted by `a_labels_click_focuses_the_select_and_opens_nothing` (tests/blitz-tests/tests/select_popup.rs:428-441)."
    sidecar: "contract-events: a label's click on a single select focuses and opens nothing."
    rationale: "Report Symbols, Pointer; report Spec claims disproved, `label-click`."
    basis: tests/blitz-tests/tests/select_popup.rs:428-441
    fact: F22
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 48
    section: "## Standard Contracts"
    change: "On line 48, after «fragment navigation and the test harness's input helper» add «; on a row of an open select list it writes the list's `scroll_y` at once and returns, moving no chain and no viewport (packages/blitz-dom/src/scrolling.rs:669-672)»."
    sidecar: "contract-scrolling-selection-tree: `scroll_into_view`'s exception for a row of an open list."
    rationale: "Report Symbols, Seen through the list: the chain-and-viewport claim no longer holds for a row."
    basis: packages/blitz-dom/src/scrolling.rs:669-672
    fact: F15
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 49
    section: "## Standard Contracts"
    change: "On line 49, after «reached through `BaseDocument::hit`, `hit_with_scrollbar` and `element_from_point`» add «, an open select list answering first in `hit_with_scrollbar`, above that walk»."
    sidecar: "contract-scrolling-selection-tree: the hit walk's statement names the list answering first."
    rationale: "Report Counts `hit-first` lists standard-contracts:49 as changed; report Symbols, The hit."
    basis: packages/blitz-dom/src/document.rs:2404-2406
    dependent-of: D-new-detail
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 51
    section: "## Standard Contracts"
    change: "On line 51, after «and `visible_region` reads it through its transform too» add «; a row or group of an open list reads the list's scrollport (packages/blitz-dom/src/scrolling.rs:754-756)»; the sentence on `Node::absolute_position` stands."
    sidecar: "contract-scrolling-selection-tree: `visible_region` answers the list's scrollport for a row or group."
    rationale: "Report Counts `visible-reg` lists standard-contracts:51 as changed; report Symbols, Seen through the list."
    basis: packages/blitz-dom/src/scrolling.rs:754-756
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 58
    section: "## Standard Contracts"
    change: "New row following line 58: - The snapshot's builder reads `select_list_bounds(node)` before the client rect, so a row of an open select list crosses with its own `bounds` (packages/dioxus-native-dom/src/snapshot.rs:152-155); dioxus-native-dom gains public, re-exported `is_select`."
    sidecar: "contract-scrolling-selection-tree: the snapshot's `bounds` reads the list's bounds first for a row."
    rationale: "Report Symbols, Bridge; report Schema / config: a row's own `bounds` in the snapshot and the diff. Line 58 names the snapshot's `bounds` among the client rect's readers."
    basis: packages/dioxus-native-dom/src/snapshot.rs:152-155
    fact: F18
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 59
    section: "## Standard Contracts"
    change: "On line 59, for «Its callers `execute.rs` `in_view` and `crop` and `lint.rs` `clip_region` are unchanged» read «Its callers `execute.rs` `in_view` and `crop` are unchanged; `lint.rs` `clip_region` takes an open row's region from `visible_region` alone, with no `contain: paint` cut»."
    sidecar: "contract-scrolling-selection-tree: `clip_region` takes an open row's region from `visible_region` alone."
    rationale: "Report Symbols, Driver: `clip_region` takes an open row's region from `visible_region` alone, so «unchanged» is stale for that caller."
    fact: F21
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 61
    section: "## Standard Contracts"
    change: "On line 61, for «its callers `hit` and `hit_with_scrollbar` are unchanged» read «its callers `hit` and `hit_with_scrollbar` keep that walk, and `hit_with_scrollbar` since 2026-10-31-select-popup answers `select_list_hit` first and no scrollbar with it (packages/blitz-dom/src/document.rs:2404-2406), so `hit`, the driver's `covered` through it, `set_hover_to` and `handle_pointerdown` all read the list»."
    sidecar: "contract-scrolling-selection-tree: `hit_with_scrollbar` answers the list first; `hit_inner` stays unedited."
    rationale: "Report Symbols, The hit; report Counts `hit-first` lists standard-contracts:61 as changed."
    basis: packages/blitz-dom/src/document.rs:2404-2406
    fact: F13
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 82
    section: "## Standard Contracts"
    change: "New row following line 82: - `row_disabled`, crate-private in `node::select_list`, is the list's reading of a disabled row, beside `ElementData::is_disabled` and `option_disabled` (packages/blitz-dom/src/node/select_list.rs:381-384)."
    sidecar: "contract-nodes-and-widgets: `row_disabled` registered among the disabled readings."
    rationale: "Report Symbols lists `row_disabled` among the list's crate-private symbols, and no registry row names it."
    basis: packages/blitz-dom/src/node/select_list.rs:381-384
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 84
    section: "## Standard Contracts"
    change: "New row following line 84: - The module `node::select_list` holds, crate-private, the open list's state and geometry: `SelectList` (`select`, `current`, `scroll_y`, `row_height`, `width`, `anchor`, `fresh`), `Row`, `Placement`, `Geometry`, `size_rows`, `rows`, `group_label`, `row_label`, `option_rows`, `place`, `geometry`, `row_disabled`, `MAX_ROWS`, `BORDER_WIDTH`, `ROW_PADDING_X`, `ROW_PADDING_Y` and `GROUP_INDENT` (packages/blitz-dom/src/node/select_list.rs:28-46; packages/blitz-dom/src/node/select_list.rs:125-130; packages/blitz-dom/src/node/select_list.rs:147-192; packages/blitz-dom/src/node/select_list.rs:218-263; packages/blitz-dom/src/node/select_list.rs:265-293)."
    sidecar: "contract-nodes-and-widgets: the new `node::select_list` module registered."
    rationale: "Report Crates / modules adds blitz-dom `node::select_list`; report Symbols lists its crate-private symbols."
    basis: packages/blitz-dom/src/node/select_list.rs:28-46
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 84
    section: "## Standard Contracts"
    change: "New row following line 84: - `SelectListView` and `SelectListRow` are public read types of `node::select_list`, re-exported from blitz-dom's crate root as `ScrollbarRef` is (packages/blitz-dom/src/node/select_list.rs:78-91; packages/blitz-dom/src/node/select_list.rs:93-108; packages/blitz-dom/src/lib.rs:108-111); the list's colours are `SelectListView` associated constants, `HIGHLIGHT_TEXT` `#000000`, `HIGHLIGHT_BACKGROUND` `#c8c8c8`, `DISABLED_TEXT` `#6d6d6d` and `BORDER` `#999999`, engine constants and no tokens."
    sidecar: "contract-nodes-and-widgets: the list's two public read types and its colour constants."
    rationale: "Report Symbols, The list's state; report Coverage records the colours as engine constants on the founder's F6, the engine holding no tokens."
    basis: packages/blitz-dom/src/lib.rs:108-111
    fact: F12
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 84
    section: "## Standard Contracts"
    change: "New row following line 84: - `SelectList.fresh` holds a just-opened list against a `:focus` rule's restyle: a click focuses and opens in one step, before that restyle, so a box-moving rule cannot close the list a click opened (`a_focus_rule_that_moves_the_selects_box_leaves_the_list_a_click_opened`, tests/blitz-tests/tests/select_popup.rs:1105-1122)."
    sidecar: "contract-nodes-and-widgets: `SelectList.fresh` and what it holds against."
    rationale: "Report Deviations from intent: the seventh field `fresh` and the test pinning it."
    basis: tests/blitz-tests/tests/select_popup.rs:1105-1122
    fact: F23
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 84
    section: "## Standard Contracts"
    change: "New row following line 84: - The list shows at most `MAX_ROWS` (20) rows, opens where its rows fit and else on the side with more room, is at least its select's width and at most the viewport's, and a select with no row to show opens nothing (packages/blitz-dom/src/node/select_list.rs:218-263)."
    sidecar: "contract-nodes-and-widgets: the list's row cap and placement."
    rationale: "Report Symbols gives `MAX_ROWS` = 20 and `place`; New text names the placement tests; Deviations from intent gives a select with no row to show."
    basis: packages/blitz-dom/src/node/select_list.rs:218-263
    fact: F24
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 105
    section: "## Standard Contracts"
    change: "On line 105, after the citations of `paint_scene`'s signature add «, which calls `paint_select_list` after the root's render and before the devtools overlays (packages/blitz-paint/src/render.rs:246-247)»."
    sidecar: "contract-paint-and-shell: `paint_scene` paints the open list."
    rationale: "Report Symbols, Paint; report Counts `paint-order` lists standard-contracts:105 as changed."
    basis: packages/blitz-paint/src/render.rs:246-247
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 106
    section: "## Standard Contracts"
    change: "New row following line 106: - `paint_select_list`, in blitz-paint's new module `render::select_list`, paints a non-finite box not at all, lays out only rows in the scrollport, and draws its thumb only under the `scrollbars` feature (packages/blitz-paint/src/render/select_list.rs:21-40; packages/blitz-paint/src/render/select_list.rs:130-154)."
    sidecar: "contract-paint-and-shell: `paint_select_list`'s finite box, row culling and feature-gated thumb."
    rationale: "Report Symbols, Paint; report Crates / modules adds blitz-paint `render::select_list`."
    basis: packages/blitz-paint/src/render/select_list.rs:21-40
    fact: F16
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 106
    section: "## Standard Contracts"
    change: "New row following line 106: - `stroke_text` delegates to the new `pub(crate)` `stroke_text_as`, which takes `RunPaint { color, synthesize_bold }` (packages/blitz-paint/src/text.rs:586-593), every other caller passing `RunPaint::default()`; a synthesized bold, a list group's header, adds one embolden weight to a build's own (packages/blitz-paint/src/text.rs:683-689)."
    sidecar: "contract-paint-and-shell: `stroke_text_as` and `RunPaint` registered."
    rationale: "Report Symbols, Paint; report Deviations from intent: a brush holding a node id alone cannot carry a colour."
    basis: packages/blitz-paint/src/text.rs:586-593
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 120
    section: "## Standard Contracts"
    change: "On line 120, in the `scroll_into_view` helper's sentence, after «by the least movement on each axis» add «, a row of an open select list moving the list's `scroll_y` alone»."
    sidecar: "contract-test-harness: the unedited `scroll_into_view` helper inherits the engine's exception for a row."
    rationale: "The harness is unedited, yet this helper calls `BaseDocument::scroll_into_view`, which report Symbols says writes the list's `scroll_y` at once and returns on a row, no chain or viewport moving."
    basis: packages/blitz-dom/src/scrolling.rs:669-672
    dependent-of: D-new-detail
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture.md
    near: 135
    section: "## Standard Contracts"
    change: "On line 135, «its one non-test caller the driver's `focus_multiple_select`» reads «its one non-test caller the driver's `focus_select`»."
    sidecar: "Standard Contracts broad line: `focus_multiple_select` retired for `focus_select`."
    rationale: "Report Retired names `focus_multiple_select`; report Counts `focus-select` lists architecture:135 as changed."
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture.md
    near: 143
    section: "## Standard Contracts"
    change: "On line 143, after «nested ones instantly with no `scroll` event, then the viewport» add «, a row of an open select list moving the list's own scroll alone»."
    sidecar: "Standard Contracts broad line: `scroll_into_view`'s exception for a row of an open list."
    rationale: "Report Symbols, Seen through the list: on a row no chain or viewport moves, so the broad line's every-box claim needs the exception its detail gains."
    basis: packages/blitz-dom/src/scrolling.rs:669-672
    dependent-of: D-new-detail
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture.md
    near: 145
    section: "## Standard Contracts"
    change: "On line 145, after «a mapped box with a non-finite edge answering `None`» add «, and a row or group of an open select list reading the list's scrollport»."
    sidecar: "Standard Contracts broad line: `visible_region` answers the list's scrollport for a row or group."
    rationale: "Report Counts `visible-reg` lists architecture:145 as changed; report Symbols, Seen through the list."
    basis: packages/blitz-dom/src/scrolling.rs:754-756
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture.md
    near: 157
    section: "## Standard Contracts"
    change: "On line 157, «`paint_scene` paints a document into a `PaintScene`» reads «`paint_scene` paints a document into a `PaintScene`, an open select list after the root and before the devtools overlays»."
    sidecar: "Standard Contracts broad line: `paint_scene` paints the open list."
    rationale: "Report Counts `paint-order` lists architecture:157 as changed; report Symbols, Paint."
    basis: packages/blitz-paint/src/render.rs:246-247
note: "F18 sits beside #6.1 and may overlap the Dioxus bridge group's placement, so keep one; F19, F20 and F25 have no carrier in this group; architecture:183 and :275 are the bridge and cross-cutting groups' broad lines; #14.1's `stand_id_persistence` clause under `dom_string` is left standing on green checks, the report not saying what its one line at :98 reads."
