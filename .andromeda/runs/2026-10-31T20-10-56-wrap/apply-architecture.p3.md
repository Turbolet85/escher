# Apply — architecture · part 3 of 4

## Verdicts
- fanout-architecture--cascade-g4.md 1 · applied · .andromeda/architecture-detail/standard-contracts.md:9#1
- fanout-architecture--cascade-g4.md 2 · applied · .andromeda/architecture-detail/standard-contracts.md:17+#1
- fanout-architecture--cascade-g4.md 3 · applied · .andromeda/architecture-detail/standard-contracts.md:17+#2
- fanout-architecture--cascade-g4.md 4 · applied · .andromeda/architecture-detail/standard-contracts.md:17+#3
- fanout-architecture--cascade-g4.md 5 · applied · .andromeda/architecture-detail/standard-contracts.md:18#1
- fanout-architecture--cascade-g4.md 6 · applied · .andromeda/architecture-detail/standard-contracts.md:26#1
- fanout-architecture--cascade-g4.md 7 · applied · .andromeda/architecture-detail/standard-contracts.md:27#1
- fanout-architecture--cascade-g4.md 8 · applied · .andromeda/architecture-detail/standard-contracts.md:30#1, .andromeda/architecture-detail/standard-contracts.md:30+#1
- fanout-architecture--cascade-g4.md 9 · applied · .andromeda/architecture-detail/standard-contracts.md:41+#1
- fanout-architecture--cascade-g4.md 10 · applied · .andromeda/architecture-detail/standard-contracts.md:41+#2
- fanout-architecture--cascade-g4.md 11 · applied · .andromeda/architecture-detail/standard-contracts.md:48#1
- fanout-architecture--cascade-g4.md 12 · applied · .andromeda/architecture-detail/standard-contracts.md:49#1
- fanout-architecture--cascade-g4.md 13 · applied · .andromeda/architecture-detail/standard-contracts.md:51#1
- fanout-architecture--cascade-g4.md 15 · applied · .andromeda/architecture-detail/standard-contracts.md:59#1
- fanout-architecture--cascade-g4.md 16 · applied · .andromeda/architecture-detail/standard-contracts.md:61#1
- fanout-architecture--cascade-g4.md 17 · applied · .andromeda/architecture-detail/standard-contracts.md:82+#1
- fanout-architecture--cascade-g4.md 19 · applied · .andromeda/architecture-detail/standard-contracts.md:84+#1
- fanout-architecture--cascade-g4.md 20 · applied · .andromeda/architecture-detail/standard-contracts.md:84+#2
- fanout-architecture--cascade-g4.md 21 · applied · .andromeda/architecture-detail/standard-contracts.md:84+#3
- fanout-architecture--cascade-g4.md 22 · applied · .andromeda/architecture-detail/standard-contracts.md:105#1
- fanout-architecture--cascade-g4.md 23 · applied · .andromeda/architecture-detail/standard-contracts.md:106+#1
- fanout-architecture--cascade-g4.md 24 · applied · .andromeda/architecture-detail/standard-contracts.md:106+#2
- fanout-architecture--cascade-g4.md 25 · applied · .andromeda/architecture-detail/standard-contracts.md:120#1
- fanout-architecture--cascade-g5.md 1 · applied · .andromeda/architecture-detail/standard-contracts.md:178#1, .andromeda/architecture-detail/standard-contracts.md:178#2
- fanout-architecture--cascade-g5.md 2 · applied · .andromeda/architecture-detail/standard-contracts.md:179#1
- fanout-architecture--cascade-g5.md 3 · applied · .andromeda/architecture-detail/standard-contracts.md:189#1
- fanout-architecture--cascade-g5.md 4 · applied · .andromeda/architecture-detail/standard-contracts.md:190#1
- fanout-architecture--cascade-g5.md 5 · applied · .andromeda/architecture-detail/standard-contracts.md:190+#1
- fanout-architecture--cascade-g5.md 6 · applied · .andromeda/architecture-detail/standard-contracts.md:212#1
- fanout-architecture--cascade-g5.md 7 · applied · .andromeda/architecture-detail/standard-contracts.md:212#2
- fanout-architecture--cascade-g5.md 8 · applied · .andromeda/architecture-detail/standard-contracts.md:213#1
- fanout-architecture--cascade-g5.md 9 · applied · .andromeda/architecture-detail/standard-contracts.md:220#1
- fanout-architecture--cascade-g6.md 1 · applied · .andromeda/architecture-detail/standard-contracts.md:259#1
- fanout-architecture--cascade-g6.md 2 · folded into fanout-architecture--cascade-g4.md 5
- fanout-architecture--cascade-g6.md 3 · applied · .andromeda/architecture-detail/standard-contracts.md:279#1
- fanout-architecture--cascade-g6.md 4 · applied · .andromeda/architecture-detail/standard-contracts.md:279+#1
- fanout-architecture--cascade-g6.md 5 · applied · .andromeda/architecture-detail/standard-contracts.md:312#1, .andromeda/architecture-detail/standard-contracts.md:312#2
- fanout-architecture--cascade-g6.md 6 · applied · .andromeda/architecture-detail/standard-contracts.md:319+#1

## Edits
- line: .andromeda/architecture-detail/standard-contracts.md:9
  old: «what does mark it is stated under §Cross-cutting Patterns → Invalidation and state integrity»
  new: «what does mark it is stated under §Cross-cutting Patterns → Invalidation and state integrity — a select list's open and close, which mark the select, among it»
- after: .andromeda/architecture-detail/standard-contracts.md:17
  anchor: «- `BaseDocument::select_value(&self, select: NodeId) -> Option<String>`, public and read-only: HTML's `select.value` — the chosen option's value, a `multiple` select's first chosen option in tree order, `None` with none chosen or for a node that is no select; dioxus-native-dom's snapshot `value` and `apply_select_choice` its callers (packages/blitz-dom/src/document.rs:724-738). The choice's writers are crate-private — `settle_select_choice`, `set_select_choice`, which takes the chosen set, and `set_option_chosen`, the `CHECKED` write through `snapshot_node_and` for an in-document option — beside the private field `select_typeahead: Option<select::Typeahead>`, `None` at construction (packages/blitz-dom/src/document.rs:339-340; packages/blitz-dom/src/document.rs:1090-1138).»
  add: «- `BaseDocument` holds one field `select_list: Option<SelectList>`, `None` at construction (packages/blitz-dom/src/document.rs:343-344); public read-only `open_select`, `select_list_bounds`, `select_list_view` and `select_row_layout`; public writer `close_select_list`, its callers keyboard, pointer, reconcile and the driver's `focus_select`; crate-private `live_select_list`, `select_list_for`, `open_select_list`, `reconcile_select_list`, `select_list_options`, `move_select_list_current`, `scroll_select_list_to`, `scroll_select_list_by`, `select_list_hit` and `select_list_region`; private `select_list_allowed`, `measure_select_list` and `text_row_layout` (packages/blitz-dom/src/document.rs:761-1089).»
- after: .andromeda/architecture-detail/standard-contracts.md:17
  anchor: «- `BaseDocument::select_value(&self, select: NodeId) -> Option<String>`, public and read-only: HTML's `select.value` — the chosen option's value, a `multiple` select's first chosen option in tree order, `None` with none chosen or for a node that is no select; dioxus-native-dom's snapshot `value` and `apply_select_choice` its callers (packages/blitz-dom/src/document.rs:724-738). The choice's writers are crate-private — `settle_select_choice`, `set_select_choice`, which takes the chosen set, and `set_option_chosen`, the `CHECKED` write through `snapshot_node_and` for an in-document option — beside the private field `select_typeahead: Option<select::Typeahead>`, `None` at construction (packages/blitz-dom/src/document.rs:339-340; packages/blitz-dom/src/document.rs:1090-1138).»
  add: «- `live_select_list`, `pub(crate)` with every caller in `document.rs`, is the reader of the list's state (packages/blitz-dom/src/document.rs:797-804), with two reads past it: `reconcile_select_list` (packages/blitz-dom/src/document.rs:894), which must see a list the reader no longer answers, and the removal arm (packages/blitz-dom/src/document.rs:1408); `hover_list_row` writes `current` directly (packages/blitz-dom/src/events/pointer.rs:604).»
- after: .andromeda/architecture-detail/standard-contracts.md:17
  anchor: «- `BaseDocument::select_value(&self, select: NodeId) -> Option<String>`, public and read-only: HTML's `select.value` — the chosen option's value, a `multiple` select's first chosen option in tree order, `None` with none chosen or for a node that is no select; dioxus-native-dom's snapshot `value` and `apply_select_choice` its callers (packages/blitz-dom/src/document.rs:724-738). The choice's writers are crate-private — `settle_select_choice`, `set_select_choice`, which takes the chosen set, and `set_option_chosen`, the `CHECKED` write through `snapshot_node_and` for an in-document option — beside the private field `select_typeahead: Option<select::Typeahead>`, `None` at construction (packages/blitz-dom/src/document.rs:339-340; packages/blitz-dom/src/document.rs:1090-1138).»
  add: «- `resolve` calls `reconcile_select_list` after layout and before `refresh_hover` (packages/blitz-dom/src/resolve.rs:133-135), and `clear_interaction_state_for_removed_node` clears the field for its select (packages/blitz-dom/src/document.rs:1408-1410).»
- line: .andromeda/architecture-detail/standard-contracts.md:18
  old: «its one non-test caller `execute.rs` `focus_multiple_select`; and `dioxus_native_dom::closed_select_of`, called by the snapshot's builder and `lint.rs`, and `is_multiple_select`, called by `closed_select_of` and `execute.rs` `locate`, public and re-exported because»
  new: «its one non-test caller `execute.rs` `focus_select`; and `dioxus_native_dom::closed_select_of`, called by the snapshot's builder and `lint.rs`, answering `None` for a select whose list is open (§Dioxus DOM bridge); `is_multiple_select`, called by `closed_select_of` alone within the workspace, still re-exported; and `is_select`, the driver's select test, called by `execute.rs` `locate` alone, public and re-exported because»
- line: .andromeda/architecture-detail/standard-contracts.md:26
  old: «named in the tree and unpainted»
  new: «named in the tree and painted as a header in an open list»
- line: .andromeda/architecture-detail/standard-contracts.md:27
  old: «`aria-expanded` (a boolean)»
  new: «`aria-expanded` (a boolean, a single select excepted, which reads none and writes `expanded` from its list)»
- line: .andromeda/architecture-detail/standard-contracts.md:30
  old: «closed values inside the states class»
  new: «closed values inside the states class, and a single select's `expanded` a closed bool from engine state»
- after: .andromeda/architecture-detail/standard-contracts.md:30
  anchor: « since the 2026-10-17 upstream sync the names class also carries `aria-labelledby`'s computed text, and the nodes carry a fourth content class, descriptions — `aria-description`'s own page text (free, unbounded) and `aria-describedby`'s targets as `described_by` — to the adapter only (the snapshot reads no description), a widening by upstream merge ratified as upstream's behaviour by the founder (2026-10-17, by question dialog in the overseer session, relayed by the overseer, «Утвердить оба (Рекомендую)», both sources confirmed by his «Подтвердить все 10 (Рекомендую)», 2026-10-18), and `aria-expanded` · `aria-disabled` · `aria-level` closed values inside the states class, unmeasured at an assistive technology like the states; what the poll reads, and before 2026-10-07-change-tracking-and-diff the poll-time refresh did not run (§Document core), so the platform tree was built once; ratified by the founder (2026-10-07) — the shell half of one item with the engine's changed-set contract, "refresh, then drain"; its windowed witness is still owed: no windowed run witnesses the refresh on the dev host — its AT-SPI bus reads inactive and the stand's windowed binary builds blitz-shell without `accessibility` — so it is held by the document-level tests and both feature builds of blitz-shell (packages/blitz-shell/src/accessibility.rs:44-46; packages/blitz-shell/src/window.rs:377-383; packages/blitz-shell/src/window.rs:522-525) — as measured at escher-0.1.0/chunks/2026-10-07-change-tracking-and-diff/report.md (packages/blitz-dom/src/accessibility.rs:11; packages/blitz-dom/src/accessibility.rs:117-123; tests/blitz-tests/tests/accessibility_roles.rs:56-61); `HtmlDocument` API `from_html`, `query_selector`, `mutate().set_inner_html`, `resolve`, `root_element().final_layout()` (examples/inner_html.rs:12-22; examples/screenshot.rs:103)»
  add: «- A single select writes `set_expanded(open)` and reads no `aria-expanded`, every other element keeping that read, and an author's `open` on a select steers nothing, `flush_open_state` and `:open` holding to a `details` or a `dialog` (packages/blitz-dom/src/accessibility.rs:238-242).»
- after: .andromeda/architecture-detail/standard-contracts.md:41
  anchor: « tests/blitz-tests/tests/touch_events.rs:113-120; tests/blitz-tests/tests/touch_events.rs:197)»
  add: «- A select's list takes these default actions: `handle_pointerdown` returns on a press in the list and else calls `close_list_outside` (packages/blitz-dom/src/events/pointer.rs:415-420); `handle_click`'s `select` arm opens a single select's list or closes one standing, its focusing in the private `focus_select` (packages/blitz-dom/src/events/pointer.rs:1108-1119), its `option` arm calls `click_list_row` behind `click_lands_on_row`, and an `optgroup` arm stands beside them; `handle_pointermove` calls `hover_list_row`, and a press in the list starts no text drag (`pressed_in_list`); `handle_wheel` scrolls the list over it, else calls `close_select_list` and then scrolls; Tab runs `choose_current_and_close` before moving focus, either direction (packages/blitz-dom/src/events/keyboard.rs:32-34); `press_select_key` opens on Enter, and on a Space with no typed text standing, a released Space doing nothing more; `press_open_list_key` is the open list's closed key match (packages/blitz-dom/src/events/keyboard.rs:215-250); a choice from the list goes through `apply_select_choice`, the same `input` then `change`.»
- after: .andromeda/architecture-detail/standard-contracts.md:41
  anchor: « tests/blitz-tests/tests/touch_events.rs:113-120; tests/blitz-tests/tests/touch_events.rs:197)»
  add: «- `handle_click`'s `label` arm focuses a bound single select and opens nothing, with no re-dispatch to it, asserted by `a_labels_click_focuses_the_select_and_opens_nothing` (tests/blitz-tests/tests/select_popup.rs:428-441).»
- line: .andromeda/architecture-detail/standard-contracts.md:48
  old: «fragment navigation and the test harness's input helper»
  new: «fragment navigation and the test harness's input helper; on a row of an open select list it writes the list's `scroll_y` at once and returns, moving no chain and no viewport (packages/blitz-dom/src/scrolling.rs:669-672)»
- line: .andromeda/architecture-detail/standard-contracts.md:49
  old: «reached through `BaseDocument::hit`, `hit_with_scrollbar` and `element_from_point`»
  new: «reached through `BaseDocument::hit`, `hit_with_scrollbar` and `element_from_point`, an open select list answering first in `hit_with_scrollbar`, above that walk (below)»
- line: .andromeda/architecture-detail/standard-contracts.md:51
  old: «and `visible_region` reads it through its transform too»
  new: «and `visible_region` reads it through its transform too, and answers the list's scrollport for a row or a group of an open list (packages/blitz-dom/src/scrolling.rs:754-756)»
- line: .andromeda/architecture-detail/standard-contracts.md:59
  old: «Its callers `execute.rs` `in_view` and `crop` and `lint.rs` `clip_region` are unchanged»
  new: «Its callers `execute.rs` `in_view` and `crop` are unchanged, and `lint.rs` `clip_region` reads it with the exception §Driver session states for an open row»
- line: .andromeda/architecture-detail/standard-contracts.md:61
  old: «its callers `hit` and `hit_with_scrollbar` are unchanged»
  new: «its callers `hit` and `hit_with_scrollbar` keep that walk, and `hit_with_scrollbar` answers `select_list_hit` first and no scrollbar with it (packages/blitz-dom/src/document.rs:2404-2406), so `hit`, the driver's `covered` through it, `set_hover_to` and `handle_pointerdown` all read the list»
- after: .andromeda/architecture-detail/standard-contracts.md:82
  anchor: «- `ElementData::is_disabled(&self) -> bool`, public: `can_be_disabled()` — `button` · `input` · `select` · `textarea` — and a `disabled` attribute present; the one reading focusability, the pointer gates and the activation keys share (packages/blitz-dom/src/node/element.rs:516-520).»
  add: «- `row_disabled`, crate-private in `node::select_list`, is the list's reading of a disabled row, beside `ElementData::is_disabled` and `option_disabled` (packages/blitz-dom/src/node/select_list.rs:381-384).»
- after: .andromeda/architecture-detail/standard-contracts.md:84
  anchor: «- `node::select` also holds `SelectCursor`, `RowGesture`, `Gestured`, `first_chosen_option`, `last_chosen`, `spanned`, `gestured`, `owner_select` and `option_disabled`, none of them public, and `stepped` takes the row it steps from; `apply_select_choice` takes a set and stays the one write path, and `document.rs` holds a `pub(crate)` field `select_cursor` (packages/blitz-dom/src/node/select.rs:56-65; packages/blitz-dom/src/node/select.rs:67-78; packages/blitz-dom/src/node/select.rs:80-89; packages/blitz-dom/src/node/select.rs:125-129; packages/blitz-dom/src/node/select.rs:175-190; packages/blitz-dom/src/node/select.rs:200-205; packages/blitz-dom/src/node/select.rs:269-272; packages/blitz-dom/src/node/select.rs:306-315; packages/blitz-dom/src/node/select.rs:317-358).»
  add: «- `SelectListView` and `SelectListRow` are public read types of `node::select_list`, re-exported from blitz-dom's crate root as `ScrollbarRef` is (packages/blitz-dom/src/node/select_list.rs:78-91; packages/blitz-dom/src/node/select_list.rs:93-108; packages/blitz-dom/src/lib.rs:108-111); the list's colours are `SelectListView` associated constants, `HIGHLIGHT_TEXT` `#000000`, `HIGHLIGHT_BACKGROUND` `#c8c8c8`, `DISABLED_TEXT` `#6d6d6d` and `BORDER` `#999999`, engine constants and no tokens.»
- after: .andromeda/architecture-detail/standard-contracts.md:84
  anchor: «- `node::select` also holds `SelectCursor`, `RowGesture`, `Gestured`, `first_chosen_option`, `last_chosen`, `spanned`, `gestured`, `owner_select` and `option_disabled`, none of them public, and `stepped` takes the row it steps from; `apply_select_choice` takes a set and stays the one write path, and `document.rs` holds a `pub(crate)` field `select_cursor` (packages/blitz-dom/src/node/select.rs:56-65; packages/blitz-dom/src/node/select.rs:67-78; packages/blitz-dom/src/node/select.rs:80-89; packages/blitz-dom/src/node/select.rs:125-129; packages/blitz-dom/src/node/select.rs:175-190; packages/blitz-dom/src/node/select.rs:200-205; packages/blitz-dom/src/node/select.rs:269-272; packages/blitz-dom/src/node/select.rs:306-315; packages/blitz-dom/src/node/select.rs:317-358).»
  add: «- `SelectList.fresh` holds a just-opened list against a `:focus` rule's restyle: a click focuses and opens in one step, before that restyle, so a box-moving rule cannot close the list a click opened (`a_focus_rule_that_moves_the_selects_box_leaves_the_list_a_click_opened`, tests/blitz-tests/tests/select_popup.rs:1105-1122).»
- after: .andromeda/architecture-detail/standard-contracts.md:84
  anchor: «- `node::select` also holds `SelectCursor`, `RowGesture`, `Gestured`, `first_chosen_option`, `last_chosen`, `spanned`, `gestured`, `owner_select` and `option_disabled`, none of them public, and `stepped` takes the row it steps from; `apply_select_choice` takes a set and stays the one write path, and `document.rs` holds a `pub(crate)` field `select_cursor` (packages/blitz-dom/src/node/select.rs:56-65; packages/blitz-dom/src/node/select.rs:67-78; packages/blitz-dom/src/node/select.rs:80-89; packages/blitz-dom/src/node/select.rs:125-129; packages/blitz-dom/src/node/select.rs:175-190; packages/blitz-dom/src/node/select.rs:200-205; packages/blitz-dom/src/node/select.rs:269-272; packages/blitz-dom/src/node/select.rs:306-315; packages/blitz-dom/src/node/select.rs:317-358).»
  add: «- The list shows at most `MAX_ROWS` (20) rows, opens where its rows fit and else on the side with more room, is at least its select's width and at most the viewport's, and a select with no row to show opens nothing (packages/blitz-dom/src/node/select_list.rs:218-263).»
- line: .andromeda/architecture-detail/standard-contracts.md:105
  old: «tests/blitz-tests/tests/background_size.rs:46); blitz-shell re-exports»
  new: «tests/blitz-tests/tests/background_size.rs:46), which calls `paint_select_list` after the root's render and before the devtools overlays (packages/blitz-paint/src/render.rs:246-247); blitz-shell re-exports»
- after: .andromeda/architecture-detail/standard-contracts.md:106
  anchor: « since 2026-10-12-headless-screenshot a password `input` (its `type` reading `password`) paints its value masked, in every document and through every renderer that paints with blitz-paint, windowed included: `draw_text_input_text` branches to the private `draw_masked_input_text`, which paints a bullet per character laid out by `BaseDocument::layout_as_text_input`, so two values of one length paint byte-identical frames, focused or not; the caret is painted at the bullet of its index in the input's caret colour (private `caret_color`), and the field scrolls only to keep the caret in its content box, measured on the bullets rather than the editor's stored `scroll_offset`; a click inside a field holding a value places the caret by the bullets — blitz-dom's pointer path lays them out through `layout_as_text_input`, scrolled as paint scrolls them, and moves the real editor to the character under the point (§Established Decisions → [Scrolling and input]), so no width of the real characters reaches the caret: one click at one offset into two values of one length lands on one index, and a double click selects the whole value (ratified by the founder, 2026-10-18) (packages/blitz-dom/src/events/pointer.rs:657-723; tests/blitz-tests/tests/password_caret_click.rs:83-121; packages/blitz-paint/src/render.rs:915-919; packages/blitz-paint/src/render.rs:969-976; packages/blitz-paint/src/render.rs:978-1052; as measured at escher-0.1.0/chunks/2026-10-12-headless-screenshot/report.md)»
  add: «- `paint_select_list`, in blitz-paint's module `render::select_list`, paints a non-finite box not at all, lays out only rows in the scrollport, and draws its thumb only under the `scrollbars` feature (packages/blitz-paint/src/render/select_list.rs:21-40; packages/blitz-paint/src/render/select_list.rs:130-154).»
- after: .andromeda/architecture-detail/standard-contracts.md:106
  anchor: « since 2026-10-12-headless-screenshot a password `input` (its `type` reading `password`) paints its value masked, in every document and through every renderer that paints with blitz-paint, windowed included: `draw_text_input_text` branches to the private `draw_masked_input_text`, which paints a bullet per character laid out by `BaseDocument::layout_as_text_input`, so two values of one length paint byte-identical frames, focused or not; the caret is painted at the bullet of its index in the input's caret colour (private `caret_color`), and the field scrolls only to keep the caret in its content box, measured on the bullets rather than the editor's stored `scroll_offset`; a click inside a field holding a value places the caret by the bullets — blitz-dom's pointer path lays them out through `layout_as_text_input`, scrolled as paint scrolls them, and moves the real editor to the character under the point (§Established Decisions → [Scrolling and input]), so no width of the real characters reaches the caret: one click at one offset into two values of one length lands on one index, and a double click selects the whole value (ratified by the founder, 2026-10-18) (packages/blitz-dom/src/events/pointer.rs:657-723; tests/blitz-tests/tests/password_caret_click.rs:83-121; packages/blitz-paint/src/render.rs:915-919; packages/blitz-paint/src/render.rs:969-976; packages/blitz-paint/src/render.rs:978-1052; as measured at escher-0.1.0/chunks/2026-10-12-headless-screenshot/report.md)»
  add: «- `stroke_text` delegates to the `pub(crate)` `stroke_text_as`, which takes `RunPaint { color, synthesize_bold }` (packages/blitz-paint/src/text.rs:586-593), every other caller passing `RunPaint::default()`; a synthesized bold, a list group's header, adds one embolden weight to a build's own (packages/blitz-paint/src/text.rs:683-689).»
- line: .andromeda/architecture-detail/standard-contracts.md:120
  old: «by the least movement on each axis»
  new: «by the least movement on each axis, a row of an open select list moving the list's `scroll_y` alone»
- line: .andromeda/architecture-detail/standard-contracts.md:178
  old: «another element is hit at the point the action would land; a box clipped on one axis stops»
  new: «another element is hit at the point the action would land; an option of a closed select lies under it: click the select first, then the option; a box clipped on one axis stops»
- line: .andromeda/architecture-detail/standard-contracts.md:178
  old: «— the remedy, the eight causes and their names unchanged —»
  new: «— the clause on an option of a closed select added, `covered`'s name, remedy and order, the eight causes and their names unchanged —»
- line: .andromeda/architecture-detail/standard-contracts.md:179
  old: «that the text chooses its match alone (packages/escher-driver/src/schema.rs:239-242);»
  new: «that the text chooses its match alone (packages/escher-driver/src/schema.rs:239-242); `click`'s help says a click on a select opens its list, and `click`'s, `type`'s and `press`'s help each read as a fixed literal of the verb table (packages/escher-driver/src/schema.rs:222-226; packages/escher-driver/src/schema.rs:240-243; packages/escher-driver/src/schema.rs:264-269);»
- line: .andromeda/architecture-detail/standard-contracts.md:189
  old: «on every target but a `multiple` select, which it focuses with no click (below)»
  new: «on every target but a select, single or `multiple`, which it focuses with no click (below)»
- line: .andromeda/architecture-detail/standard-contracts.md:190
  old: «`type` on a `multiple` select: `Target` carries `multiple`, and `type` does not click such a target, a click landing on the row at the centre: the crate-private `focus_multiple_select` focuses it»
  new: «`type` on a select, single or `multiple`: `Target` carries `select`, and `type` does not click such a target, a click landing on a `multiple` select's row at the centre and opening a single select's list: the crate-private `focus_select` focuses it»
- after: .andromeda/architecture-detail/standard-contracts.md:190
  anchor: « `type` on a `multiple` select: `Target` carries `multiple`, and `type` does not click such a target, a click landing on the row at the centre: the crate-private `focus_multiple_select` focuses it with no click and clears the typeahead, without which a second `type` joins the first's text; a text opening on a space types nothing, since no label can match and the Space would flip a row; the reading holds no verb, argument, key name, result field, cause, session error or exit of its own (packages/escher-driver/src/execute.rs:235-239; packages/escher-driver/src/execute.rs:557-563).»
  add: « `focus_select` also closes any list, and `type` on a single select fires no DOM focus events, a text opening on a space typing nothing there (packages/escher-driver/src/execute.rs:558-560).»
- line: .andromeda/architecture-detail/standard-contracts.md:212
  old: «a closed select's options excepted (below)»
  new: «a closed select's options and an open list's rows and groups excepted (below)»
- line: .andromeda/architecture-detail/standard-contracts.md:212
  old: «as closed keyword sets. The six readings:»
  new: «as closed keyword sets. `clip_region` takes an open row's region from `visible_region` alone, with no `contain: paint` cut (packages/escher-driver/src/lint.rs:405-408). The six readings:»
- line: .andromeda/architecture-detail/standard-contracts.md:213
  old: «The `overlap` reading leaves out a closed select's options: `Entry.in_closed_select` reads through `closed_select_of`, by the element and never the role, so CRUD's `role="option"` rows are not read as a select's, and `overlaps` skips such an entry on either side;»
  new: «The `overlap` reading leaves out a closed select's options and an open list's rows and groups: `Entry.in_select_box` reads a closed select's option through `closed_select_of`, by the element and never the role, so CRUD's `role="option"` rows are not read as a select's, and is true also for an open list's row or group, and `overlaps` skips such an entry on either side;»
- line: .andromeda/architecture-detail/standard-contracts.md:220
  old: «a `multiple` select's rows read their own client rects»
  new: «an open list's rows read their own bounds in the list and `overlaps` skips them too, so they read no `overlap` on the stand, `an_open_lists_rows_read_no_overlap_on_the_stand` and `the_lint_reads_no_overlap_on_a_row_and_the_planted_pair_still_reads` pinning it (tests/blitz-tests/tests/stand_lint.rs:493-519; tests/blitz-tests/tests/stand_act_popup.rs:318-350); a `multiple` select's rows read their own client rects»
- line: .andromeda/architecture-detail/standard-contracts.md:259
  old: «`bounds` `BaseDocument::get_client_bounding_rect` (viewport-relative, unclamped, for every element»
  new: «`bounds` `BaseDocument::select_list_bounds(node)` first, a row's or a group's place in an open list (packages/dioxus-native-dom/src/snapshot.rs:152-155), else `BaseDocument::get_client_bounding_rect` (viewport-relative, unclamped, for every element»
- line: .andromeda/architecture-detail/standard-contracts.md:279
  old: «packages/dioxus-native-dom/src/snapshot.rs:244-258).»
  new: «packages/dioxus-native-dom/src/snapshot.rs:244-258). `closed_select_of` answers `None` for a select whose list is open, so a row reads its own `bounds` from `BaseDocument::select_list_bounds` while the list stands and its select's client rect again once the list closes (packages/dioxus-native-dom/src/snapshot.rs:152-155).»
- after: .andromeda/architecture-detail/standard-contracts.md:279
  anchor: «- A select in the bridge and the snapshot: `Input | Change` reach the app as `NativeFormData { value }`, so an `onchange` handler fires (packages/dioxus-native-dom/src/dioxus_document.rs:393-398); the snapshot's `value` reader takes the document and the node and reads a `select` through `BaseDocument::select_value`, no mask applying (packages/dioxus-native-dom/src/snapshot.rs:264-269); and an option of a closed select, under an `optgroup` too, reads its select's client rect as its `bounds` (the public `closed_select_of` and `BaseDocument::select_of`), so a select's options are its children in the snapshot, a group's its group's, named by their labels (packages/dioxus-native-dom/src/snapshot.rs:144-156; packages/dioxus-native-dom/src/snapshot.rs:244-258).»
  add: «- The snapshot carries no `expanded` node key and `NodeState` holds no field for a list, `NODE_FIELDS` keeping the thirteen keys §Driver session names.»
- line: .andromeda/architecture-detail/standard-contracts.md:312
  old: «fifteen named blitz-tests targets outside that prefix»
  new: «seventeen named blitz-tests targets outside that prefix»
- line: .andromeda/architecture-detail/standard-contracts.md:312
  old: «which asserts named targets, `select_interaction` among them, in the leg's own logged words»
  new: «which asserts named targets, `select_interaction`, `select_popup` and `select_popup_painted` among them, in the leg's own logged words»
- after: .andromeda/architecture-detail/standard-contracts.md:319
  anchor: « root crate feature `log-times` = `log-frame-times` + `log-phase-times` forwarding to dioxus-native (Cargo.toml:265-268)»
  add: «- No local gate builds `blitz-paint/font-embolden`, which CI's macOS leg alone exercises; `--features blitz-paint/font-embolden` on `cargo test -p blitz-tests` is the local witness for a paint change under that feature, `--test select_popup_painted` so reproducing macOS's two readings on linux (as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md); owner: a CARRY on «Upstream sync ahead of the UI graph».»

## Patterns
[[pattern]]
id = "arch3-01"
fixed = 'its one non-test caller `execute.rs` `focus_multiple_select`'
[[pattern]]
id = "arch3-02"
fixed = 'called by `closed_select_of` and `execute.rs` `locate`'
[[pattern]]
id = "arch3-03"
fixed = 'named in the tree and unpainted'
[[pattern]]
id = "arch3-04"
fixed = '`aria-expanded` (a boolean)'
[[pattern]]
id = "arch3-05"
fixed = '`lint.rs` `clip_region` are unchanged'
[[pattern]]
id = "arch3-06"
fixed = '`hit_with_scrollbar` are unchanged'
[[pattern]]
id = "arch3-07"
fixed = 'on every target but a `multiple` select'
[[pattern]]
id = "arch3-08"
fixed = '`Target` carries `multiple`'
[[pattern]]
id = "arch3-09"
fixed = 'crate-private `focus_multiple_select` focuses it'
[[pattern]]
id = "arch3-10"
fixed = 'options excepted (below)'
[[pattern]]
id = "arch3-11"
fixed = '`Entry.in_closed_select` reads through `closed_select_of`'
[[pattern]]
id = "arch3-12"
fixed = 'fifteen named blitz-tests targets'
[[pattern]]
id = "arch3-13"
fixed = '`get_client_bounding_rect` (viewport-relative, unclamped, for every element'
