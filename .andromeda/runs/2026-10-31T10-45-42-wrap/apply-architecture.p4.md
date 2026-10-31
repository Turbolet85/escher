# Apply — architecture · part 4 of 5

## Verdicts
- fanout-architecture--cascade-g4.md 1 · applied · .andromeda/architecture-detail/standard-contracts.md:17#1, .andromeda/architecture-detail/standard-contracts.md:17#2
- fanout-architecture--cascade-g4.md 2 · applied · .andromeda/architecture-detail/standard-contracts.md:17+#1
- fanout-architecture--cascade-g4.md 3 · applied · .andromeda/architecture-detail/standard-contracts.md:17+#2
- fanout-architecture--cascade-g4.md 5 · applied · .andromeda/architecture-detail/standard-contracts.md:24#1
- fanout-architecture--cascade-g4.md 6 · applied · .andromeda/architecture-detail/standard-contracts.md:26#1, .andromeda/architecture-detail/standard-contracts.md:26#2
- fanout-architecture--cascade-g4.md 7 · applied · .andromeda/architecture-detail/standard-contracts.md:27#1
- fanout-architecture--cascade-g4.md 8 · applied · .andromeda/architecture-detail/standard-contracts.md:28+#1
- fanout-architecture--cascade-g4.md 9 · applied · .andromeda/architecture-detail/standard-contracts.md:28+#2
- fanout-architecture--cascade-g4.md 12 · applied · .andromeda/architecture-detail/standard-contracts.md:34#1
- fanout-architecture--cascade-g4.md 13 · applied · .andromeda/architecture-detail/standard-contracts.md:36#1
- fanout-architecture--cascade-g4.md 14 · applied · .andromeda/architecture-detail/standard-contracts.md:37+#1
- fanout-architecture--cascade-g4.md 16 · applied · .andromeda/architecture-detail/standard-contracts.md:44#1
- fanout-architecture--cascade-g4.md 17 · applied · .andromeda/architecture-detail/standard-contracts.md:46#1
- fanout-architecture--cascade-g4.md 18 · applied · .andromeda/architecture-detail/standard-contracts.md:53#1, .andromeda/architecture-detail/standard-contracts.md:53#2
- fanout-architecture--cascade-g4.md 19 · applied · .andromeda/architecture-detail/standard-contracts.md:53#3
- fanout-architecture--cascade-g4.md 20 · applied · .andromeda/architecture-detail/standard-contracts.md:53+#1
- fanout-architecture--cascade-g4.md 22 · applied · .andromeda/architecture-detail/standard-contracts.md:55+#1
- fanout-architecture--cascade-g4.md 23 · applied · .andromeda/architecture-detail/standard-contracts.md:76+#1
- fanout-architecture--cascade-g4.md 24 · applied · .andromeda/architecture-detail/standard-contracts.md:78#1
- fanout-architecture--cascade-g5.md 1 · applied · .andromeda/architecture-detail/standard-contracts.md:169#1
- fanout-architecture--cascade-g5.md 2 · applied · .andromeda/architecture-detail/standard-contracts.md:171#1
- fanout-architecture--cascade-g5.md 3 · applied · .andromeda/architecture-detail/standard-contracts.md:171#2
- fanout-architecture--cascade-g5.md 4 · applied · .andromeda/architecture-detail/standard-contracts.md:181#1, .andromeda/architecture-detail/standard-contracts.md:181+#1
- fanout-architecture--cascade-g5.md 5 · applied · .andromeda/architecture-detail/standard-contracts.md:182+#1
- fanout-architecture--cascade-g5.md 6 · applied · .andromeda/architecture-detail/standard-contracts.md:202#1, .andromeda/architecture-detail/standard-contracts.md:202+#1
- fanout-architecture--cascade-g5.md 7 · applied · .andromeda/architecture-detail/standard-contracts.md:209#1
- fanout-architecture--cascade-g6.md 1 · applied · .andromeda/architecture-detail/standard-contracts.md:249#1
- fanout-architecture--cascade-g6.md 2 · applied · .andromeda/architecture-detail/standard-contracts.md:250#1
- fanout-architecture--cascade-g6.md 3 · applied · .andromeda/architecture-detail/standard-contracts.md:251#1
- fanout-architecture--cascade-g6.md 4 · applied · .andromeda/architecture-detail/standard-contracts.md:260#1
- fanout-architecture--cascade-g6.md 5 · applied · .andromeda/architecture-detail/standard-contracts.md:260+#1
- fanout-architecture--cascade-g6.md 6 · applied · .andromeda/architecture-detail/standard-contracts.md:267#1
- fanout-architecture--cascade-g6.md 7 · applied · .andromeda/architecture-detail/standard-contracts.md:267+#1
- fanout-architecture--cascade-g6.md 8 · applied · .andromeda/architecture-detail/standard-contracts.md:267+#2
- fanout-architecture--cascade-g6.md 10 · applied · .andromeda/architecture-detail/standard-contracts.md:298#1, .andromeda/architecture-detail/standard-contracts.md:298#2

## Edits
- line: .andromeda/architecture-detail/standard-contracts.md:17
  old: «HTML's `select.value` — the chosen option's value, `None` with none chosen»
  new: «HTML's `select.value` — the chosen option's value, a `multiple` select's first chosen option in tree order, `None` with none chosen»
- line: .andromeda/architecture-detail/standard-contracts.md:17
  old: «`settle_select_choice`, `set_select_choice` and `set_option_chosen`»
  new: «`settle_select_choice`, `set_select_choice`, which takes the chosen set, and `set_option_chosen`»
- after: .andromeda/architecture-detail/standard-contracts.md:17
  anchor: «- `BaseDocument::select_value(&self, select: NodeId) -> Option<String>`, public and read-only: HTML's `select.value` — the chosen option's value, `None` with none chosen or for a node that is no select; dioxus-native-dom's snapshot `value` and `apply_select_choice` its callers (packages/blitz-dom/src/document.rs:720-734). The choice's writers are crate-private — `settle_select_choice`, `set_select_choice` and `set_option_chosen`, the `CHECKED` write through `snapshot_node_and` for an in-document option — beside the private field `select_typeahead: Option<select::Typeahead>`, `None` at construction (packages/blitz-dom/src/document.rs:338-339; packages/blitz-dom/src/document.rs:757-805).»
  add: «- `BaseDocument::select_values(&self, select: NodeId) -> Vec<String>` and `BaseDocument::select_of(&self, node: NodeId) -> Option<NodeId>`, public and read-only; public `BaseDocument::clear_select_typeahead(&mut self)`, its one non-test caller `execute.rs` `focus_multiple_select`; and `dioxus_native_dom::closed_select_of`, called by the snapshot's builder and `lint.rs`, and `is_multiple_select`, called by `closed_select_of` and `execute.rs` `locate`, public and re-exported because escher-driver has no blitz-dom dependency to name `local_name!` with, `is_masked_input` the precedent (packages/blitz-dom/src/document.rs:736-744; packages/blitz-dom/src/document.rs:746-750; packages/blitz-dom/src/document.rs:752-755; packages/dioxus-native-dom/src/snapshot.rs:242-248; packages/dioxus-native-dom/src/snapshot.rs:242-252; packages/dioxus-native-dom/src/lib.rs:35-38).»
- after: .andromeda/architecture-detail/standard-contracts.md:17
  anchor: «- `BaseDocument::select_value(&self, select: NodeId) -> Option<String>`, public and read-only: HTML's `select.value` — the chosen option's value, `None` with none chosen or for a node that is no select; dioxus-native-dom's snapshot `value` and `apply_select_choice` its callers (packages/blitz-dom/src/document.rs:720-734). The choice's writers are crate-private — `settle_select_choice`, `set_select_choice` and `set_option_chosen`, the `CHECKED` write through `snapshot_node_and` for an in-document option — beside the private field `select_typeahead: Option<select::Typeahead>`, `None` at construction (packages/blitz-dom/src/document.rs:338-339; packages/blitz-dom/src/document.rs:757-805).»
  add: «- A write or removal of `multiple` through the mutator records the select (`record_select_of_multiple_write`), so a removal settles its set to one option, and `set_option_selectedness` leaves a `multiple` select's other options alone (packages/blitz-dom/src/mutator.rs:1262-1271).»
- line: .andromeda/architecture-detail/standard-contracts.md:24
  old: «where it has no `aria-label` (packages/blitz-dom/src/accessibility.rs:36-61»
  new: «where it has no `aria-label`, and an `optgroup`'s (`Role::Group`), named in the tree and unpainted (packages/blitz-dom/src/accessibility.rs:36-61»
- line: .andromeda/architecture-detail/standard-contracts.md:26
  old: «and the builder writes no `toggled` from that native checked state either, so the tree states no native checkbox or radio checked while the snapshot's `checked` reads the element — a measured absence (as measured at escher-0.2.0/chunks/2026-10-18-engine-debts/report.md), its fix owned by a `.andromeda/residuals.md` line (target: next)»
  new: «and the builder writes `toggled` from that native checked state under a checkable role (below), so the tree states it as the snapshot's `checked` reads the element»
- line: .andromeda/architecture-detail/standard-contracts.md:26
  old: «and nothing else in the file writes `toggled`»
  new: «and only these reads and that native write set `toggled`»
- line: .andromeda/architecture-detail/standard-contracts.md:27
  old: «on each element node whose markup states them, closed enums»
  new: «on each element node whose markup states them, a native control's `toggled` and a `multiple` select's multiselectable beside them, closed enums»
- after: .andromeda/architecture-detail/standard-contracts.md:28
  anchor: « since the 2026-10-17 upstream sync the names class also carries `aria-labelledby`'s computed text, and the nodes carry a fourth content class, descriptions — `aria-description`'s own page text (free, unbounded) and `aria-describedby`'s targets as `described_by` — to the adapter only (the snapshot reads no description), a widening by upstream merge ratified as upstream's behaviour by the founder (2026-10-17, by question dialog in the overseer session, relayed by the overseer, «Утвердить оба (Рекомендую)», both sources confirmed by his «Подтвердить все 10 (Рекомендую)», 2026-10-18), and `aria-expanded` · `aria-disabled` · `aria-level` closed values inside the states class, unmeasured at an assistive technology like the states; what the poll reads, and before 2026-10-07-change-tracking-and-diff the poll-time refresh did not run (§Document core), so the platform tree was built once; ratified by the founder (2026-10-07) — the shell half of one item with the engine's changed-set contract, "refresh, then drain"; its windowed witness is still owed: no windowed run witnesses the refresh on the dev host — its AT-SPI bus reads inactive and the stand's windowed binary builds blitz-shell without `accessibility` — so it is held by the document-level tests and both feature builds of blitz-shell (packages/blitz-shell/src/accessibility.rs:44-46; packages/blitz-shell/src/window.rs:377-383; packages/blitz-shell/src/window.rs:522-525) — as measured at escher-0.1.0/chunks/2026-10-07-change-tracking-and-diff/report.md (packages/blitz-dom/src/accessibility.rs:11; packages/blitz-dom/src/accessibility.rs:117-123; tests/blitz-tests/tests/accessibility_roles.rs:56-61); `HtmlDocument` API `from_html`, `query_selector`, `mutate().set_inner_html`, `resolve`, `root_element().final_layout()` (examples/inner_html.rs:12-22; examples/screenshot.rs:103)»
  add: «- The tree's focus: `build_accessibility_tree` writes `focus` only where `focus_node_id` is among the built nodes, else `u64::MAX`, the window, so the tree never names a node it did not build; an engine change for every document (packages/blitz-dom/src/accessibility.rs:104-110; packages/blitz-dom/src/accessibility.rs:122).»
- after: .andromeda/architecture-detail/standard-contracts.md:28
  anchor: « since the 2026-10-17 upstream sync the names class also carries `aria-labelledby`'s computed text, and the nodes carry a fourth content class, descriptions — `aria-description`'s own page text (free, unbounded) and `aria-describedby`'s targets as `described_by` — to the adapter only (the snapshot reads no description), a widening by upstream merge ratified as upstream's behaviour by the founder (2026-10-17, by question dialog in the overseer session, relayed by the overseer, «Утвердить оба (Рекомендую)», both sources confirmed by his «Подтвердить все 10 (Рекомендую)», 2026-10-18), and `aria-expanded` · `aria-disabled` · `aria-level` closed values inside the states class, unmeasured at an assistive technology like the states; what the poll reads, and before 2026-10-07-change-tracking-and-diff the poll-time refresh did not run (§Document core), so the platform tree was built once; ratified by the founder (2026-10-07) — the shell half of one item with the engine's changed-set contract, "refresh, then drain"; its windowed witness is still owed: no windowed run witnesses the refresh on the dev host — its AT-SPI bus reads inactive and the stand's windowed binary builds blitz-shell without `accessibility` — so it is held by the document-level tests and both feature builds of blitz-shell (packages/blitz-shell/src/accessibility.rs:44-46; packages/blitz-shell/src/window.rs:377-383; packages/blitz-shell/src/window.rs:522-525) — as measured at escher-0.1.0/chunks/2026-10-07-change-tracking-and-diff/report.md (packages/blitz-dom/src/accessibility.rs:11; packages/blitz-dom/src/accessibility.rs:117-123; tests/blitz-tests/tests/accessibility_roles.rs:56-61); `HtmlDocument` API `from_html`, `query_selector`, `mutate().set_inner_html`, `resolve`, `root_element().final_layout()` (examples/inner_html.rs:12-22; examples/screenshot.rs:103)»
  add: «- A native control's state in the tree: `build_accessibility_node` writes `Toggled::True` or `False` from `checkbox_input_checked()` where `is_native_checkable && is_checkable(role)`; under any other role a native control writes nothing, so `pressed` never reads on it, and neither author attribute is read on a native control; an engine change for every document (packages/blitz-dom/src/accessibility.rs:199-224; tests/blitz-tests/tests/accessibility_states.rs:386-427).»
- line: .andromeda/architecture-detail/standard-contracts.md:34
  old: «focus change dispatches `blur`, `focusout`, `focus`, `focusin` (packages/blitz-dom/src/events/focus.rs:15-39);»
  new: «focus change dispatches `blur`, `focusout`, `focus`, `focusin` (packages/blitz-dom/src/events/focus.rs:15-39), and the inert focus fixup none ([DOM semantics]);»
- line: .andromeda/architecture-detail/standard-contracts.md:36
  old: «are dispatched by the engine only after `input` on a select's choice, a checkbox, radio or text input firing no engine `change` ([Scrolling and input])»
  new: «are dispatched by the engine after `input` on a select's choice and on a checkbox's or radio's click (below), a text or range input firing no engine `change` ([Scrolling and input])»
- after: .andromeda/architecture-detail/standard-contracts.md:37
  anchor: « tests/blitz-tests/tests/touch_events.rs:113-120; tests/blitz-tests/tests/touch_events.rs:197)»
  add: «- `change` on a checkable control: `handle_click`'s checkbox and radio arms dispatch `DomEventData::Change` after `input` with the same value, a checkbox always, a radio only where it was not checked before the click; blitz-vibey-script's runtime synthesizes none, so a page's script hears one `change`, from the engine; an engine change for every document (packages/blitz-dom/src/events/pointer.rs:793-799; packages/blitz-dom/src/events/pointer.rs:811; packages/blitz-dom/src/events/pointer.rs:827-836; tests/blitz-tests/tests/checkable_change_event.rs:68-98; tests/blitz-tests/tests/checkable_change_event.rs:100-119).»
- line: .andromeda/architecture-detail/standard-contracts.md:44
  old: «the region reading as it read before (packages/blitz-dom/src/scrolling.rs:736-788)»
  new: «the region reading as it read before, and through its transforms where its chain carries one (`visible_region` under transforms, below) (packages/blitz-dom/src/scrolling.rs:736-788)»
- line: .andromeda/architecture-detail/standard-contracts.md:46
  old: «(the client rect under transforms, below);»
  new: «(the client rect under transforms, below), and `visible_region` reads it through its transform too;»
- line: .andromeda/architecture-detail/standard-contracts.md:53
  old: «the private `transformed_bounds` maps»
  new: «the `pub(crate)` `transformed_bounds` (`visible_region` reads it too) maps»
- line: .andromeda/architecture-detail/standard-contracts.md:53
  old: «`offset_rect` (CSSOM `offsetTop`, untransformed), `inline_fragment_rects` and `visible_region` are not edited»
  new: «`offset_rect` (CSSOM `offsetTop`, untransformed) and `inline_fragment_rects` are not edited»
- line: .andromeda/architecture-detail/standard-contracts.md:53
  old: «A measured limit: `visible_region` places a scrolling box under a transform where it stands untransformed, so a target inside a translated scrolling box reads outside its visible region (tests/blitz-tests/tests/transformed_client_rect.rs:183-220), stated in `off-screen`'s meaning (§Driver session), its fix owned by a `.andromeda/residuals.md` line (target: next).»
  new: «A target inside a translated scrolling box reads inside its visible region (tests/blitz-tests/tests/transformed_client_rect.rs:183-216), and the limit that stands is the rotation's, stated in `off-screen`'s meaning (§Driver session).»
- after: .andromeda/architecture-detail/standard-contracts.md:53
  anchor: «- The client rect under transforms (ratified by the founder, 2026-10-18): `BaseDocument::get_client_bounding_rect` and `node_client_rects` read an element where its transforms move it — on a containing-block chain that carries a transform (the private `is_transformed_chain`) the private `transformed_bounds` maps the border box's corners through the node's own transform and each containing block's, in device pixels as `hit_inner` and the paint tree map them, the offsets taken from `physical_unrounded_geometry` differences so both of its bodies are served, and reads the axis-aligned bounding box; a non-atomic inline element maps its fragments through the private `transformed_fragment_rects`; with no transform on the chain the reading is the untransformed one in both bodies (packages/blitz-dom/src/document.rs:2540-2542; packages/blitz-dom/src/document.rs:2551-2560; packages/blitz-dom/src/document.rs:2569-2662; packages/blitz-dom/src/document.rs:2733-2735; tests/blitz-tests/tests/transformed_client_rect.rs:79-153). `physical_unrounded_geometry`, `offset_rect` (CSSOM `offsetTop`, untransformed), `inline_fragment_rects` and `visible_region` are not edited; the readers of `get_client_bounding_rect` move with it — the snapshot's `bounds`, `node_client_rects`, the events driver's `adjust_element_coords`, the script `getBoundingClientRect` and dioxus-native-dom's `get_client_rect`. A measured limit: `visible_region` places a scrolling box under a transform where it stands untransformed, so a target inside a translated scrolling box reads outside its visible region (tests/blitz-tests/tests/transformed_client_rect.rs:183-220), stated in `off-screen`'s meaning (§Driver session), its fix owned by a `.andromeda/residuals.md` line (target: next).»
  add: «- `visible_region` under transforms: `BaseDocument::visible_region` keeps its signature and reads each clipping box through its transforms via the private `padding_box_in_view` over `transformed_bounds`: exact under a translate and a scale; under a rotation the rotated padding box's bounding box, so it holds corners the box does not show; `None` for a mapped box with a non-finite edge; as before with no transform on the chain. Its callers `execute.rs` `in_view` and `crop` and `lint.rs` `clip_region` are unchanged; an engine change for every document (packages/blitz-dom/src/scrolling.rs:790-812; tests/blitz-tests/tests/visible_region_transformed.rs:201-216; tests/blitz-tests/tests/visible_region_transformed.rs:221-244; tests/blitz-tests/tests/visible_region_transformed.rs:246-276).»
- after: .andromeda/architecture-detail/standard-contracts.md:55
  anchor: «- The hit walk skips a node dropped since the last resolve: `Node::hit_inner` reads each child id through `self.tree().get(id)` and passes over one that no longer resolves, in its positive-z hoisted, paint-children and negative-z hoisted loops and its text-cluster lookup, and the paint tree's walk does the same; `hit_inner` keeps its signature, and its callers `hit` and `hit_with_scrollbar` are unchanged — an engine change for every document (packages/blitz-dom/src/node/node.rs:1414-1417; packages/blitz-dom/src/node/node.rs:1578-1580; packages/blitz-dom/src/node/node.rs:1593-1595; packages/blitz-dom/src/node/node.rs:1627-1629; packages/blitz-dom/src/node/node.rs:1652-1656; packages/blitz-dom/src/layout/paint_tree.rs:164-172; tests/blitz-tests/tests/hit_test_dropped_nodes.rs:32-82).»
  add: «- A non-invertible transform takes no hit: `Node::hit_inner` answers `None` for the whole subtree under a transform whose determinant is 0 or not finite; an engine change for every document (packages/blitz-dom/src/node/node.rs:1472-1477; tests/blitz-tests/tests/hit_non_invertible_transform.rs:33-56; tests/blitz-tests/tests/hit_non_invertible_transform.rs:58-71).»
- after: .andromeda/architecture-detail/standard-contracts.md:76
  anchor: «- The crate-private module `node::select` holds the select's rules — `OptionReading`, `SelectStep` (`Next` · `Previous` · `First` · `Last`), `Typeahead` and `TYPEAHEAD_TIMEOUT` (1 s), and the readers `is_single_select`, `is_select_option`, `options`, `is_chosen`, `chosen_option`, `readings`, `option_label`, `option_value`, `settled`, `stepped` and `matched` — no public type (packages/blitz-dom/src/node/mod.rs:10; packages/blitz-dom/src/node/select.rs:18-304).»
  add: «- `node::select` also holds `SelectCursor`, `RowGesture`, `Gestured`, `first_chosen_option`, `last_chosen`, `spanned`, `gestured`, `owner_select` and `option_disabled`, none of them public, and `stepped` takes the row it steps from; `apply_select_choice` takes a set and stays the one write path, and `document.rs` holds a `pub(crate)` field `select_cursor` (packages/blitz-dom/src/node/select.rs:56-65; packages/blitz-dom/src/node/select.rs:67-78; packages/blitz-dom/src/node/select.rs:80-89; packages/blitz-dom/src/node/select.rs:125-129; packages/blitz-dom/src/node/select.rs:175-190; packages/blitz-dom/src/node/select.rs:200-205; packages/blitz-dom/src/node/select.rs:269-272; packages/blitz-dom/src/node/select.rs:306-315; packages/blitz-dom/src/node/select.rs:317-358).»
- line: .andromeda/architecture-detail/standard-contracts.md:78
  old: «and by `remove_attribute`,»
  new: «and by `clear_attribute`,»
- line: .andromeda/architecture-detail/standard-contracts.md:169
  old: «a scrolling box under a transform is read where it stands untransformed, so an element inside it can read `off-screen` though it shows»
  new: «under a rotation a box's visible part reads as the bounding box of its rotated padding box, so an element in a corner it does not show can read in view»
- line: .andromeda/architecture-detail/standard-contracts.md:171
  old: «and a scrolling box under a transform read where it stands untransformed, so an element inside it can read `off-screen` though it shows»
  new: «and, under a rotation, a box's visible part read as the bounding box of its rotated padding box, so an element in a corner it does not show can read in view»
- line: .andromeda/architecture-detail/standard-contracts.md:171
  old: «whose label starts with it (packages/escher-driver/src/schema.rs:237-239)»
  new: «whose label starts with it and, on a `multiple` select, that the text chooses its match alone (packages/escher-driver/src/schema.rs:237-239)»
- line: .andromeda/architecture-detail/standard-contracts.md:181
  old: «and `type` clicks it the same way, which focuses a text input,»
  new: «and `type`, on every target but a `multiple` select, which it focuses with no click (below), clicks it the same way, which focuses a text input,»
- after: .andromeda/architecture-detail/standard-contracts.md:181
  anchor: « Then `click` clicks the centre of the snapshot's `bounds` for the node plus the viewport's scroll — a page point, where until 2026-10-07-refusal-detection it was the centre of the layout border box — through `Harness::click_at`, and `type` clicks it the same way, which focuses a text input, and types through `Harness::type_text` — replacing what the control holds since 2026-10-10-driver-cli (the founder's answer, 2026-10-10, relayed by the overseer): when the snapshot reads a `value` for the target — an empty text input reads one, the empty string — the driver first presses the engine's select-all, `a` with Control, Super on macOS, and with an empty text then deletes the selection through its own Backspace press, so an empty text clears the control; a target that reads no `value` is typed into as before, and until that chunk the text was appended after the old value (as measured at escher-0.1.0/chunks/2026-10-10-driver-cli/evidence/type-filled.md); for a box that is itself scrolled those `bounds` are where the box stands since 2026-10-10-scrolling-box-bounds-and-hit (the Dioxus DOM bridge contract below), so a `click` or a `type` naming such a box lands at its centre — measured in both layout modes: on the stand `crud-list` reads the same four figures after its content is scrolled, the scroll's diff does not name it, and a `click` naming it lands on the row at the box's centre at 12 Creates and at 14, while the last row is still refused `off-screen` until a `scroll` brings it in; the restated `stand_act_scroll` holds the reading (as measured at escher-0.1.0/chunks/2026-10-10-scrolling-box-bounds-and-hit/report.md; the shifted reading it replaces at escher-0.1.0/chunks/2026-10-10-driver-cli/evidence/engine-findings.md).»
  add: « `type` on a `multiple` select: `Target` carries `multiple`, and `type` does not click such a target, a click landing on the row at the centre: the crate-private `focus_multiple_select` focuses it with no click and clears the typeahead, without which a second `type` joins the first's text; a text opening on a space types nothing, since no label can match and the Space would flip a row; the reading holds no verb, argument, key name, result field, cause, session error or exit of its own (packages/escher-driver/src/execute.rs:235-239; packages/escher-driver/src/execute.rs:557-563).»
- after: .andromeda/architecture-detail/standard-contracts.md:182
  anchor: « `press` presses its key on what holds focus through `Harness::press_with`, with the shift modifier when `shift` is true; on macOS only, a `backspace` press also dispatches `deleteBackward:` through the harness's `apple_keybinding`, the editor's own Backspace arm being compiled out there — one of the module's two macOS gates, the other choosing the select-all's modifier. `advance` moves the harness's clock by exactly `ms` (`Harness::advance_time`) inside one settled step, then hands `ms` to the session's time step where one was handed, and `advanced_ms` is exactly `ms` — the clock's move, not what a counter inside the app gained, and a step's diff can be non-empty on any animated page (ratified by the founder, 2026-10-18): on the stand's Timer an `advance` of 60000 reports 60000 while the elapsed display stops at its 15 s duration. `scroll` applies none of the three checks that stand in front of a click — it is accepted on a disabled or a covered element: it calls the harness's `scroll_into_view` input helper on the resolved node, an instant scroll of every scrolling box that holds the element and then of the viewport, as one settled step, and answers `in_view` by making the `off-screen` reading again on the snapshot taken after the step — false when no scroll could bring the element into view, and when the step removed it; it is never refused for failing to reach its target, its result says so.»
  add: « `press` carries no Ctrl, so a set of rows of a `multiple` select that are not adjacent cannot be built through the driver, and its help states it: "a set of rows that are not adjacent needs Ctrl, which this verb does not carry" (packages/escher-driver/src/schema.rs:261-263).»
- line: .andromeda/architecture-detail/standard-contracts.md:202
  old: «whose visible parts intersect, reported once»
  new: «whose visible parts intersect, a closed select's options excepted (below), reported once»
- after: .andromeda/architecture-detail/standard-contracts.md:202
  anchor: « Each node's clip region is `visible_region` cut by the padding box of every DOM ancestor below the root whose computed `contain` includes `paint` — the lint's own walk, `visible_region` unchanged — with the computed `contain`, `overflow-x` / `overflow-y` and `display` read through `BaseDocument::resolved_style_value` as closed keyword sets. The six readings: `overlap`, two nodes neither an ancestor of the other whose visible parts intersect, reported once at the highest level, on the earlier node — a part scrolled out of view overlaps nothing; `clipped` and `off-screen`, read against that clip region; `truncated`, a node that is no text control whose `text_extent` text box does not lie inside its clip region cut by its own padding box where it clips; `zero-size`, a node with no area that holds a name, a value, an `enabled` reading, focusability or text descendants; `contrast`, a node with direct non-whitespace text or a text control, read through `Harness::text_contrast` and reported below `required_ratio()`, exempt at `enabled == Some(false)`, `None` not measured. The `lint` verb carries it — `Command::Lint`, `Outcome::Linted`, the `{"findings":[…]}` written form, an MCP tool and a recorded step, its answer kept whole — its `command` span recording `verb` and, refused, `cause` alone, no field added, and `VERBS` holds twelve; a node both clipped or off-screen and holding text also reads `truncated`, pinned by `stand_lint` in both layout modes, the `lint` verb answering the same (test-plan §1; as measured at escher-0.2.0/chunks/2026-10-19-visual-lint/report.md, its check at escher-0.2.0/chunks/2026-10-24-stand-requirement-sweep/report.md).»
  add: « The `overlap` reading leaves out a closed select's options: `Entry.in_closed_select` reads through `closed_select_of`, by the element and never the role, so CRUD's `role="option"` rows are not read as a select's, and `overlaps` skips such an entry on either side; a `multiple` select's rows are linted as usual (packages/escher-driver/src/lint.rs:246-247; packages/escher-driver/src/lint.rs:337-338; packages/escher-driver/src/lint.rs:347; packages/escher-driver/src/lint.rs:364; packages/escher-driver/src/lint.rs:368).»
- line: .andromeda/architecture-detail/standard-contracts.md:209
  old: «the snapshot gives each option its select's client rect, so the lint reads two options of one select as overlapping — on the stand the flight booker reads one `overlap` at boot (`flight-type-one-way` × `flight-type-return`) and the other three lean tasks 0, which `stand_lint` pins as measured in both layout modes; the defect is open, its fix — a driver change for every document in the lint's overlap reading (`packages/escher-driver/src/lint.rs`, `overlaps`) — owned by a `.andromeda/residuals.md` line (tests/blitz-tests/tests/stand_lint.rs:277-363; as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md).»
  new: «the snapshot gives a closed select's options its client rect and the lint's `overlaps` skips them, so on the stand all four lean tasks read 0 `overlap` at boot, `stand_lint` pinning `at_boot` as `[0, 0, 0, 0]` in both layout modes; a `multiple` select's rows read their own client rects, and the pair of overlapping rows `stand_lint` plants reads exactly `fx-row-a` × `fx-row-b` (tests/blitz-tests/tests/stand_lint.rs:282-332; as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md).»
- line: .andromeda/architecture-detail/standard-contracts.md:249
  old: «`value` a `select`'s chosen option's value (`BaseDocument::select_value`, below)»
  new: «`value` a `select`'s chosen option's value, a `multiple` select's first chosen option in tree order (`BaseDocument::select_value`, below)»
- line: .andromeda/architecture-detail/standard-contracts.md:250
  old: «and `aria-invalid` (§Standard Contracts → Mutation, query and CSSOM)»
  new: «and `aria-invalid` (§Standard Contracts → Mutation, query and CSSOM), and on a native checkable control it reads neither `aria-pressed` nor `aria-checked`, writing `toggled` from `checkbox_input_checked()` under a checkable role and nothing under any other, so `pressed` never reads on one»
- line: .andromeda/architecture-detail/standard-contracts.md:251
  old: «so a choice names the old and the new option with `selected` in `differs`,»
  new: «so a single select's choice names the old and the new option with `selected` in `differs` and a `multiple` select's names each row whose chosen state moved,»
- line: .andromeda/architecture-detail/standard-contracts.md:260
  old: «through the file's own `col_label(c)` and `cell-input` (examples»
  new: «through the file's own `col_label(c)` and `cell-input`, and, beside those nine, `cells-grid` on the grid, focusable and carrying a listener, so it reads its key (examples»
- after: .andromeda/architecture-detail/standard-contracts.md:260
  anchor: « `circle-undo`, `circle-redo`, `circle-canvas`, `circle-diameter` and `circle-close` (the last two only while the dialog is open); `cell-{A..Z}{0..25}` on the 676 data cells through the file's own `col_label(c)` and `cell-input` (examples/seven_guis/src/tasks/temp_converter.rs:13; examples/seven_guis/src/tasks/temp_converter.rs:26; examples/seven_guis/src/tasks/cells.rs:413; examples/seven_guis/src/tasks/cells.rs:442; tests/blitz-tests/tests/stand_non_lean_tasks.rs:136-149; packages/dioxus-native-dom/src/actionable.rs:15-44; packages/dioxus-native-dom/src/actionable.rs:55-89; packages/dioxus-native-dom/src/actionable.rs:91-122; packages/dioxus-native-dom/src/lib.rs:15-16; packages/dioxus-native-dom/src/lib.rs:29-30; tests/blitz-tests/tests/stand_actionable_keys.rs:1-4) — as measured at escher-0.1.0/chunks/2026-10-06-id-stability-across-code-edits/report.md and, for the three non-lean tasks, at escher-0.1.0/chunks/2026-10-14-stand-requirement-sweep/report.md; it returns nothing on the load page `stand::boot_load` mounts too, its two elements keyed `load-fetch` · `load-done`, so it reads empty on all six served tasks (tests/blitz-tests/tests/stand_load_page.rs:113-127)»
  add: «- Cells' keyboard on the stand: `cells-grid` takes `id`, `tabindex` 0, `aria_label` Cells and an `onkeydown`, with no role, so it is one Tab stop: arrows move `current`, clamped, into view through a synchronous `MountedData::scroll_to(ScrollBehavior::Instant)`, one `onmounted` per data cell holding the handles; Enter opens the editor on `current`, focused, and its Enter commits, stops propagation and returns focus to the grid; a click inside an open editor returns early and keeps its typed text; `.cells-grid:focus .cell.current` draws the accent outline, `2px solid #3a5ce5` (examples/seven_guis/src/tasks/cells.rs:362-393; examples/seven_guis/src/tasks/cells.rs:425-433; examples/seven_guis/src/tasks/cells.rs:551-554).»
- line: .andromeda/architecture-detail/standard-contracts.md:267
  old: «and an option whose parent is a `select` reads its select's client rect as its `bounds` (the private `select_parent`), so a select's options are its children in the snapshot, named by their labels»
  new: «and an option of a closed select, under an `optgroup` too, reads its select's client rect as its `bounds` (the public `closed_select_of` and `BaseDocument::select_of`), so a select's options are its children in the snapshot, a group's its group's, named by their labels»
- after: .andromeda/architecture-detail/standard-contracts.md:267
  anchor: «- A select in the bridge and the snapshot: `Input | Change` reach the app as `NativeFormData { value }`, so an `onchange` handler fires (packages/dioxus-native-dom/src/dioxus_document.rs:393-398); the snapshot's `value` reader takes the document and the node and reads a `select` through `BaseDocument::select_value`, no mask applying (packages/dioxus-native-dom/src/snapshot.rs:256-261); and an option whose parent is a `select` reads its select's client rect as its `bounds` (the private `select_parent`), so a select's options are its children in the snapshot, named by their labels (packages/dioxus-native-dom/src/snapshot.rs:144-153; packages/dioxus-native-dom/src/snapshot.rs:241-250).»
  add: «- A `multiple` select in the snapshot: its rows and groups read their own client rects as their `bounds`, and its `value` reads as above (packages/dioxus-native-dom/src/snapshot.rs:242-252; packages/dioxus-native-dom/src/snapshot.rs:256-257).»
- after: .andromeda/architecture-detail/standard-contracts.md:267
  anchor: «- A select in the bridge and the snapshot: `Input | Change` reach the app as `NativeFormData { value }`, so an `onchange` handler fires (packages/dioxus-native-dom/src/dioxus_document.rs:393-398); the snapshot's `value` reader takes the document and the node and reads a `select` through `BaseDocument::select_value`, no mask applying (packages/dioxus-native-dom/src/snapshot.rs:256-261); and an option whose parent is a `select` reads its select's client rect as its `bounds` (the private `select_parent`), so a select's options are its children in the snapshot, named by their labels (packages/dioxus-native-dom/src/snapshot.rs:144-153; packages/dioxus-native-dom/src/snapshot.rs:241-250).»
  add: «- `multiple_select_values` in `dioxus_document.rs` fills `values` on `Input` and `Change` with one `(name, FormValue::Text(value))` per chosen option of a `multiple` select, so `evt.values()` reads the set, while `value` stays `select_value` (packages/dioxus-native-dom/src/dioxus_document.rs:31-45).»
- line: .andromeda/architecture-detail/standard-contracts.md:298
  old: «ten named blitz-tests targets outside that prefix»
  new: «fifteen named blitz-tests targets outside that prefix»
- line: .andromeda/architecture-detail/standard-contracts.md:298
  old: «and `stand_requirement_sweep`'s leg check (test-plan §1), which reads the script whole»
  new: «and `stand_requirement_sweep`'s leg check (test-plan §1), whose `STAND_REPEAT_NAMED` still holds the first ten (tests/blitz-tests/tests/stand_requirement_sweep.rs:840), which reads the script whole»

## Patterns
[[pattern]]
id = "arch4-01"
fixed = 'writes no `toggled` from that native checked state'

[[pattern]]
id = "arch4-02"
fixed = 'nothing else in the file writes `toggled`'

[[pattern]]
id = "arch4-03"
fixed = 'a checkbox, radio or text input firing no engine `change`'

[[pattern]]
id = "arch4-04"
fixed = 'the private `transformed_bounds`'

[[pattern]]
id = "arch4-05"
fixed = '`inline_fragment_rects` and `visible_region` are not edited'

[[pattern]]
id = "arch4-06"
fixed = 'places a scrolling box under a transform where it stands untransformed'

[[pattern]]
id = "arch4-07"
fixed = 'class` arm and by `remove_attribute`'

[[pattern]]
id = "arch4-08"
fixed = 'a scrolling box under a transform is read where it stands untransformed'

[[pattern]]
id = "arch4-09"
fixed = 'a scrolling box under a transform read where it stands untransformed'

[[pattern]]
id = "arch4-10"
fixed = '`type` clicks it the same way'

[[pattern]]
id = "arch4-11"
fixed = 'so the lint reads two options of one select as overlapping'

[[pattern]]
id = "arch4-12"
fixed = 'so a choice names the old and the new option'

[[pattern]]
id = "arch4-13"
fixed = 'an option whose parent is a `select` reads'

[[pattern]]
id = "arch4-14"
fixed = 'ten named blitz-tests targets outside that prefix'
