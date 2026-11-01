# Apply — architecture · part 3 of 4

## Verdicts
- fanout-architecture--cascade-g5.md 1 · applied · .andromeda/architecture-detail/standard-contracts.md:22+#1
- fanout-architecture--cascade-g5.md 2 · applied · .andromeda/architecture-detail/standard-contracts.md:36+#1
- fanout-architecture--cascade-g5.md 3 · applied · .andromeda/architecture-detail/standard-contracts.md:36+#2
- fanout-architecture--cascade-g5.md 4 · applied · .andromeda/architecture-detail/standard-contracts.md:68+#1
- fanout-architecture--cascade-g5.md 5 · applied · .andromeda/architecture-detail/standard-contracts.md:77+#1
- fanout-architecture--cascade-g5.md 6 · applied · .andromeda/architecture-detail/standard-contracts.md:82#1
- fanout-architecture--cascade-g5.md 7 · applied · .andromeda/architecture-detail/standard-contracts.md:95+#1
- fanout-architecture--cascade-g6.md 1 · applied · .andromeda/architecture-detail/standard-contracts.md:103#1
- fanout-architecture--cascade-g6.md 2 · applied · .andromeda/architecture-detail/standard-contracts.md:109#1
- fanout-architecture--cascade-g6.md 3 · applied · .andromeda/architecture-detail/standard-contracts.md:118+#1
- fanout-architecture--cascade-g6.md 4 · applied · .andromeda/architecture-detail/standard-contracts.md:118+#2
- fanout-architecture--cascade-g6.md 7 · applied · .andromeda/architecture-detail/standard-contracts.md:131#1
- fanout-architecture--cascade-g6.md 8 · applied · .andromeda/architecture-detail/standard-contracts.md:135#1
- fanout-architecture--cascade-g7.md 1 · applied · .andromeda/architecture-detail/standard-contracts.md:225+#1
- fanout-architecture--cascade-g7.md 2 · applied · .andromeda/architecture-detail/standard-contracts.md:227#1
- fanout-architecture--cascade-g8.md 1 · applied · .andromeda/architecture-detail/standard-contracts.md:240#1, .andromeda/architecture-detail/standard-contracts.md:241#1
- fanout-architecture--cascade-g8.md 3 · applied · .andromeda/architecture-detail/standard-contracts.md:253+#1
- fanout-architecture--cascade-g8.md 4 · applied · .andromeda/architecture-detail/standard-contracts.md:266#1
- fanout-architecture--cascade-g8.md 5 · applied · .andromeda/architecture-detail/standard-contracts.md:295+#1
- fanout-architecture--cascade-g8.md 6 · applied · .andromeda/architecture-detail/standard-contracts.md:324#1
- fanout-architecture--cascade-g8.md 7 · applied · .andromeda/architecture-detail/standard-contracts.md:327#1
- fanout-architecture--cascade-g8.md 8 · applied · .andromeda/architecture-detail/standard-contracts.md:333+#1
- fanout-architecture--cascade-g8.md 9 · applied · .andromeda/architecture-detail/standard-contracts.md:333+#2
- fanout-architecture--cascade-g8.md 10 · applied · .andromeda/architecture-detail/standard-contracts.md:334#1

## Edits
- after: .andromeda/architecture-detail/standard-contracts.md:22
  anchor: «- A write or removal of `multiple` through the mutator records the select (`record_select_of_multiple_write`), so a removal settles its set to one option, and `set_option_selectedness` leaves a `multiple` select's other options alone (packages/blitz-dom/src/mutator.rs:1334-1343).»
  add: «- `NetHandler::error(url, error)` stands beside `bytes`: `ResourceHandler<StylesheetHandler>::error` responds `Err(error)`, so `pending_critical_resources` loses the request and a failed stylesheet stops blocking render, and `net.rs`'s other resource kinds keep the trait's do-nothing default; an engine change for every document (packages/blitz-dom/src/net.rs:144-146).»
- after: .andromeda/architecture-detail/standard-contracts.md:36
  anchor: «- A native control's state in the tree: `build_accessibility_node` writes `Toggled::True` or `False` from `checkbox_input_checked()` where `is_native_checkable && is_checkable(role)`; under any other role a native control writes nothing, so `pressed` never reads on it, and neither author attribute is read on a native control; an engine change for every document (packages/blitz-dom/src/accessibility.rs:199-224; tests/blitz-tests/tests/accessibility_states.rs:386-427).»
  add: «- The resolved `width` and `height` of `resolved_style_value` read `node.unrounded_layout()`, so a reader gets sub-pixel values where `final_layout` rounds; an engine change for every document (packages/blitz-dom/src/resolved_style.rs:468).»
- after: .andromeda/architecture-detail/standard-contracts.md:36
  anchor: «- A native control's state in the tree: `build_accessibility_node` writes `Toggled::True` or `False` from `checkbox_input_checked()` where `is_native_checkable && is_checkable(role)`; under any other role a native control writes nothing, so `pressed` never reads on it, and neither author attribute is read on a native control; an engine change for every document (packages/blitz-dom/src/accessibility.rs:199-224; tests/blitz-tests/tests/accessibility_states.rs:386-427).»
  add: «- Restyle on mutation: `set_attribute`, `clear_attribute`, removal and insertion restyle through `DocumentMutator::restyle_for_attribute_change` and `restyle_for_child_list_change`, with no blanket `RestyleHint::restyle_subtree()`, and the changed-set insert in `remove_and_drop_all_children` stands first; public `BaseDocument::snapshot_node` takes `attr: &LocalName`, its two callers both in `mutator.rs`; an engine change for every document (packages/blitz-dom/src/mutator.rs:689-735; packages/blitz-dom/src/mutator.rs:737-807; packages/blitz-dom/src/mutator.rs:288; packages/blitz-dom/src/mutator.rs:429).»
- after: .andromeda/architecture-detail/standard-contracts.md:68
  anchor: «- A non-invertible transform takes no hit: `Node::hit_inner` answers `None` for the whole subtree under a transform whose determinant is 0 or not finite; an engine change for every document (packages/blitz-dom/src/node/node.rs:1485-1490; tests/blitz-tests/tests/hit_non_invertible_transform.rs:33-56; tests/blitz-tests/tests/hit_non_invertible_transform.rs:58-71).»
  add: «- Style data and the resolve context: `TElement::borrow_data` filters data with no primary style, and the style-resolve context is scoped; an engine change for every document (packages/blitz-dom/src/stylo.rs:771-776; packages/blitz-dom/src/stylo.rs:231-238).»
- after: .andromeda/architecture-detail/standard-contracts.md:77
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · contract-layout -->»
  add: «»
  add: «- `Node`'s stacking-context arm reads a `mix-blend-mode` other than `normal`; an engine change for every document (packages/blitz-dom/src/node/node.rs:1382-1384).»
  add: «»
  add: «<!-- /detail -->»
- line: .andromeda/architecture-detail/standard-contracts.md:82
  old: «; `ProxyRenderContext` unregisters a widget's resources»
  new: «; `ElementData::set_custom_widget` replays each existing attribute to `Widget::attribute_changed` with no old value, behind `custom-widget` (packages/blitz-dom/src/node/element.rs:902-906); `ProxyRenderContext` unregisters a widget's resources»
- after: .andromeda/architecture-detail/standard-contracts.md:95
  anchor: «- `Node::out_of_flow: AtomicBool`, public: written by the style traversal from the node's new primary style through the crate-private `Node::record_out_of_flow`, beside the crate-private `style_is_out_of_flow` — `position` `absolute` or `fixed`, and not `display: none`; the crate-private `LayoutPassState::is_out_of_flow` alone reads the recorded flag, while `Node::is_out_of_flow` keeps its signature and its style reading, now through `style_is_out_of_flow`, as do its other callers (packages/blitz-dom/src/node/node.rs:128-131; packages/blitz-dom/src/node/node.rs:1282-1290; packages/blitz-dom/src/node/node.rs:2088-2095; packages/blitz-dom/src/layout/mod.rs:856-858).»
  add: «- `NodeFlags::IS_INLINE_WITH_ONLY_BLOCK_CHILDREN` and the public reader `Node::is_inline_with_only_block_children`, set in `collect_layout_children_with_wrap`; `render_element` skips such a box's own shadow, background and border through `paints_own_box`, with no frame test; an engine change for every document (packages/blitz-dom/src/node/node.rs:531-537; packages/blitz-dom/src/layout/construct.rs:587-599).»
- line: .andromeda/architecture-detail/standard-contracts.md:103
  old: «`NetHandler::bytes(self: Box<Self>, resolved_url: String, bytes: Bytes)` (packages/blitz-traits/src/net.rs:34-39)»
  new: «`NetHandler::bytes(self: Box<Self>, resolved_url: String, bytes: Bytes)` (packages/blitz-traits/src/net.rs:34-39), and beside it `NetHandler::error(self: Box<Self>, url: String, error: String)`, a default that does nothing (packages/blitz-traits/src/net.rs:36-38)»
- line: .andromeda/architecture-detail/standard-contracts.md:109
  old: «waking the waker for `doc_id` then calling `handler.bytes` (packages/blitz-net/src/lib.rs:279-318)»
  new: «calling `handler.bytes` on a success or `handler.error(url, e.to_string())` on a failure, then waking the waker for `doc_id` (packages/blitz-net/src/lib.rs:279-318; packages/blitz-net/src/lib.rs:313; packages/blitz-net/src/lib.rs:316)»
- after: .andromeda/architecture-detail/standard-contracts.md:118
  anchor: «- `stroke_text` delegates to the `pub(crate)` `stroke_text_as`, which takes `RunPaint { color, synthesize_bold }` (packages/blitz-paint/src/text.rs:586-593), every other caller passing `RunPaint::default()`; a synthesized bold, a list group's header, adds one embolden weight to a build's own (packages/blitz-paint/src/text.rs:683-689).»
  add: «- `mix-blend-mode` and `isolation` paint in every document: blitz-paint's private module `blend` holds `pub(crate) convert_blend_mode`, `LayerManager::maybe_with_layer` and `maybe_push_layer` take a `BlendMode`, and `render_element` pushes a viewport-wide blend layer and pops it last, so an open list, through the `paint_select_list` call above, paints last, past the blend layers (packages/blitz-paint/src/lib.rs:6; packages/blitz-paint/src/blend.rs:5-26; packages/blitz-paint/src/render.rs:485-506).»
- after: .andromeda/architecture-detail/standard-contracts.md:118
  anchor: «- `stroke_text` delegates to the `pub(crate)` `stroke_text_as`, which takes `RunPaint { color, synthesize_bold }` (packages/blitz-paint/src/text.rs:586-593), every other caller passing `RunPaint::default()`; a synthesized bold, a list group's header, adds one embolden weight to a build's own (packages/blitz-paint/src/text.rs:683-689).»
  add: «- The blend has no frame witness: no fork or upstream test paints a blended element to a frame or names `convert_blend_mode`, and its unit tests are the 3 of `layers.rs` (packages/blitz-paint/src/layers.rs:104-181); owner: a `CARRY:` on «Upstream sync ahead of determinism and exploration».»
- line: .andromeda/architecture-detail/standard-contracts.md:131
  old: «whose `filter` list is not empty or whose `background-image` holds a layer other than `none` — read through `BaseDocument::resolved_style_value`, the harness taking no `style` crate (packages/blitz-test-harness/src/contrast.rs:56-91;»
  new: «whose `filter` list is not empty, whose `background-image` holds a layer other than `none` or whose `mix-blend-mode` is other than `normal` — read through `BaseDocument::resolved_style_value`, the harness taking no `style` crate (packages/blitz-test-harness/src/contrast.rs:56-91; packages/blitz-test-harness/src/contrast.rs:154-156;»
- line: .andromeda/architecture-detail/standard-contracts.md:135
  old: «counts a request in flight until its handler is consumed or dropped, and stores counts only»
  new: «counts a request in flight until its handler is consumed or dropped — its handler `CountedHandler` forwards `NetHandler::error`, so a failed load reaches the document on a harness or driver session and ends the wait, returning where it would otherwise run to `LOAD_WAIT` (packages/blitz-test-harness/src/settle.rs:121-125) — and stores counts only»
- after: .andromeda/architecture-detail/standard-contracts.md:225
  anchor: « Each node's clip region is `visible_region` cut by the padding box of every DOM ancestor below the root whose computed `contain` includes `paint` — the lint's own walk, `visible_region` unchanged — with the computed `contain`, `overflow-x` / `overflow-y` and `display` read through `BaseDocument::resolved_style_value` as closed keyword sets. `clip_region` takes an open row's region from `visible_region` alone, with no `contain: paint` cut (packages/escher-driver/src/lint.rs:405-408). The six readings: `overlap`, two nodes neither an ancestor of the other whose visible parts intersect, a closed select's options and an open list's rows and groups excepted (below), reported once at the highest level, on the earlier node — a part scrolled out of view overlaps nothing; `clipped` and `off-screen`, read against that clip region; `truncated`, a node that is no text control whose `text_extent` text box does not lie inside its clip region cut by its own padding box where it clips; `zero-size`, a node with no area that holds a name, a value, an `enabled` reading, focusability or text descendants; `contrast`, a node with direct non-whitespace text or a text control, read through `Harness::text_contrast` and reported below `required_ratio()`, exempt at `enabled == Some(false)`, `None` not measured. The `lint` verb carries it — `Command::Lint`, `Outcome::Linted`, the `{"findings":[…]}` written form, an MCP tool and a recorded step, its answer kept whole — its `command` span recording `verb` and, refused, `cause` alone, no field added, and `VERBS` holds twelve; a node both clipped or off-screen and holding text also reads `truncated`, pinned by `stand_lint` in both layout modes, the `lint` verb answering the same (test-plan §1; as measured at escher-0.2.0/chunks/2026-10-19-visual-lint/report.md, its check at escher-0.2.0/chunks/2026-10-24-stand-requirement-sweep/report.md).»
  add: « The `contrast` reading reads a node past an element on the walk with a `mix-blend-mode` other than `normal` as not measured: `Harness::text_contrast` reads `None` there through `paints_more_than_colour`, the same reading with no second formula, and no stand row reads it (packages/blitz-test-harness/src/contrast.rs:154-156).»
- line: .andromeda/architecture-detail/standard-contracts.md:227
  old: «until the load is answered or `LOAD_WAIT` has passed»
  new: «until the load is answered, a failed load answering it, or `LOAD_WAIT` has passed»
- line: .andromeda/architecture-detail/standard-contracts.md:240
  old: «a `concat!` of nine `include_str!` scripts under `runtime/bootstrap/`, defines history, fetch, DOMException, CSSOM classes and interface objects (packages/blitz-vibey-script/src/runtime/bootstrap.rs:14-37); `ScriptDocument` provides a Boa-based engine for HTML with JavaScript enabled (examples/preact_script.rs:1-3; examples/preact_script.rs:12; examples/preact_script.rs:28-35);»
  new: «a `concat!` of ten `include_str!` scripts under `runtime/bootstrap/`, `url_search_params` first, defines history, fetch, DOMException, `URLSearchParams`, CSSOM classes and interface objects (packages/blitz-vibey-script/src/runtime/bootstrap.rs:14-37);»
- line: .andromeda/architecture-detail/standard-contracts.md:241
  old: «a script reaches the document's text selection through»
  new: «`ScriptDocument` provides a Boa-based engine for HTML with JavaScript enabled (examples/preact_script.rs:1-3; examples/preact_script.rs:12; examples/preact_script.rs:28-35); a script reaches the document's text selection through»
- after: .andromeda/architecture-detail/standard-contracts.md:253
  anchor: «- CSS properties are accessors on `CSSStyleDeclaration`: the bootstrap's `installStyleProperties` defines, once per style prototype, an accessor under each name `__blitz_css_property_names` returns and under its camel-case form — `css_property_names`, Stylo's non-custom property names that blitz-dom supports (`css_property_is_supported`) — on the registered script-to-DOM crossing, no carrier added (packages/blitz-vibey-script/src/runtime/css.rs:8-18; packages/blitz-vibey-script/src/runtime/bootstrap/style.js:26-48; packages/blitz-vibey-script/tests/dom.rs:495-527).»
  add: «- `URLSearchParams` is a global class with `URL.prototype.searchParams`, `href` and `search` live both ways, a realization on the registered script crossing; its native `parse_search_params`, registered as `__blitz_parse_search_params`, is deleted from `globalThis` before any page script; two WPT files stand in `timeout-quarantine.txt` with it (packages/blitz-vibey-script/src/runtime/bootstrap/url_search_params.js:3-4; packages/blitz-vibey-script/src/runtime/bootstrap/url_search_params.js:29-105; packages/blitz-vibey-script/src/runtime/bootstrap/url_search_params.js:123-146; packages/blitz-vibey-script/src/runtime/bootstrap.rs:41-46; packages/blitz-vibey-script/src/runtime/bootstrap.rs:167-184; wpt/runner/timeout-quarantine.txt:105; wpt/runner/timeout-quarantine.txt:107).»
- line: .andromeda/architecture-detail/standard-contracts.md:266
  old: «`vdom_state.try_element_to_node_id(ElementId)` (tests/blitz-tests/tests/stale_node_mapping.rs:66»
  new: «`vdom_state.try_element_to_node_id(ElementId)`, which the guard of `flush_queued_mounted_events` (below) also reads (tests/blitz-tests/tests/stale_node_mapping.rs:66»
- after: .andromeda/architecture-detail/standard-contracts.md:295
  anchor: «- `multiple_select_values` in `dioxus_document.rs` fills `values` on `Input` and `Change` with one `(name, FormValue::Text(value))` per chosen option of a `multiple` select, so `evt.values()` reads the set, while `value` stays `select_value` (packages/dioxus-native-dom/src/dioxus_document.rs:31-45).»
  add: «- `flush_queued_mounted_events` skips an element removed since its event was queued (packages/dioxus-native-dom/src/dioxus_document.rs:235-239); upstream's test `element_removed_before_its_mounted_event_is_flushed` is left out, reading 1 mounted event on Dioxus 0.7 where it asserts 0.»
- line: .andromeda/architecture-detail/standard-contracts.md:324
  old: «`.github/scripts/ci-leg.sh {leg}` — legs `fmt` · `clippy` (the workspace line»
  new: «`.github/scripts/ci-leg.sh {leg}` — legs `embolden` (the last step of `fast`, below) · `fmt` · `clippy` (the workspace line»
- line: .andromeda/architecture-detail/standard-contracts.md:327
  old: «`fast` (fmt → clippy → test → ci-scripts, stopping at the first red»
  new: «`fast` (fmt → clippy → test → ci-scripts → embolden, stopping at the first red»
- after: .andromeda/architecture-detail/standard-contracts.md:333
  anchor: « root crate feature `log-times` = `log-frame-times` + `log-phase-times` forwarding to dioxus-native (Cargo.toml:265-268)»
  add: «- `.github/scripts/ci-leg.sh` holds the leg `embolden`, `cargo test -p blitz-tests --locked --no-fail-fast --features blitz-paint/font-embolden --test select_popup_painted`, with no CI job, macOS exercising the feature there, pinned by `test_fast_leg_ends_on_the_font_embolden_witness` (.github/scripts/ci-leg.sh:8-9; .github/scripts/ci-leg.sh:28-29; .github/scripts/test_ci_workflows.py:361-375).»
- after: .andromeda/architecture-detail/standard-contracts.md:333
  anchor: « root crate feature `log-times` = `log-frame-times` + `log-phase-times` forwarding to dioxus-native (Cargo.toml:265-268)»
  add: «- `test_only_stand_repeat_frees_disk` pins that only the `stand-repeat` job frees disk (.github/scripts/test_ci_workflows.py:110-115).»
- line: .andromeda/architecture-detail/standard-contracts.md:334
  old: «No local gate builds `blitz-paint/font-embolden`, which CI's macOS leg alone exercises; `--features blitz-paint/font-embolden` on `cargo test -p blitz-tests` is the local witness for a paint change under that feature, `--test select_popup_painted` so reproducing macOS's two readings on linux (as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md); owner: a CARRY on «Upstream sync ahead of the UI graph».»
  new: «`blitz-paint/font-embolden` is built locally by the `embolden` step of `fast`, `--features blitz-paint/font-embolden` on `cargo test -p blitz-tests` the local witness for a paint change under that feature, `--test select_popup_painted` so reproducing macOS's two readings on linux (as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md).»

## Patterns
[[pattern]]
id = "arch3-01"
fixed = 'waking the waker for `doc_id` then calling'

[[pattern]]
id = "arch3-02"
fixed = 'whose `filter` list is not empty or whose `background-image`'

[[pattern]]
id = "arch3-03"
fixed = 'until the load is answered or'

[[pattern]]
id = "arch3-04"
fixed = 'of nine `include_str!` scripts'

[[pattern]]
id = "arch3-05"
fixed = 'clippy → test → ci-scripts, stopping'

[[pattern]]
id = "arch3-06"
fixed = 'No local gate builds'
