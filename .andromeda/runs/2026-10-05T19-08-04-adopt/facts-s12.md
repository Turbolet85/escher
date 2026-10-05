# facts-s12 · HEAD 0f60502ea724ef703b34220b3847bb3483f70b5f · 61 files

## architecture §Design Philosophy
- The crate's lib file states that integration tests for Blitz live in the `tests/` directory of this crate (tests/blitz-tests/lib.rs:1)
- Non-incremental layout is defined as "every node is damaged every frame" and runs through the same damage pipeline as incremental mode, so the two must produce identical layouts after any sequence of mutations (tests/blitz-tests/tests/incremental_oracle.rs:1-7)
- The paint tree (`paint_children` / `stacking_context`) is built once per frame after layout, only for damaged subtrees, and holds topology only; hoisted offsets are derived from current layout and scroll offsets at use time (tests/blitz-tests/tests/paint_tree_incremental.rs:1-4)
- Persistent interaction state (hover/mousedown/active/focus) must only reference DOM nodes, never layout-generated nodes, so it survives box-tree reconstruction "by construction" (tests/blitz-tests/tests/interaction_state_canonicalization.rs:1-10)
- Node-removal teardown of interaction state is stated to match browser semantics, naming WebKit `hoveredElementDidDetach`/`elementInActiveChainDidDetach` and Blink `HoveredElementDetached`/`ActiveChainNodeDetached` (tests/blitz-tests/tests/interaction_state_teardown.rs:1-14)
- Device changes (viewport resizes, zoom, color-scheme, media-type) are coalesced on the document and applied to the stylist as a single device rebuild at the start of the next resolve (tests/blitz-tests/tests/device_coalescing.rs:1-3)
- A viewport resize is stated not to restyle the whole document: only origins whose media query results changed and elements using viewport units are invalidated (tests/blitz-tests/tests/resize_restyle.rs:1-3; tests/blitz-tests/tests/resize_restyle.rs:77-94)

## architecture §Stack and Technologies
- Package `blitz-tests`, description "Integration tests for Blitz", `publish = false` (tests/blitz-tests/Cargo.toml:1-4)
- version, license, homepage, repository, categories, edition and rust-version are all inherited from the workspace; their values are not stated in this manifest (tests/blitz-tests/Cargo.toml:5-11)
- The crate has a `[lib]` at `lib.rs` and only `[dev-dependencies]` (tests/blitz-tests/Cargo.toml:13; tests/blitz-tests/Cargo.toml:37-38)
- Blitz dev-dependencies: `blitz-test-harness`, `blitz-dom` with features `accessibility`, `floats`, `system-fonts`, `blitz-html`, `blitz-traits`, `blitz-paint` with features `scrollbars`, `svg`, `dioxus-native-dom`, `anyrender`, `anyrender_vello_cpu`, all `workspace = true` (tests/blitz-tests/Cargo.toml:14-22)
- DioxusLabs dev-dependencies: `dioxus`, `dioxus-core`, `workspace = true` (tests/blitz-tests/Cargo.toml:24-26)
- Other dev-dependencies: `style`, `accesskit`, `markup5ever`, `keyboard-types`, `taffy`, `test-that`, `usvg`, all `workspace = true` (tests/blitz-tests/Cargo.toml:28-35)
- Pixel output in tests is produced with `anyrender::render_to_buffer` and `anyrender_vello_cpu::VelloCpuImageRenderer` driving `blitz_paint::paint_scene` (tests/blitz-tests/tests/background_size.rs:3-8; tests/blitz-tests/tests/background_size.rs:45-49)
- `anyrender::NullScenePainter` is used as a null render backend for timing (tests/blitz-tests/tests/paint_tree_bench.rs:5; tests/blitz-tests/tests/paint_tree_bench.rs:162-165)
- Dioxus components are written with `rsx!` and `use_signal` and run through `Harness::from_component` (tests/blitz-tests/tests/harness_smoke.rs:89-104)
- Stylo computed values are read directly in tests, e.g. `style::properties::generated::longhands::visibility::computed_value::T` (tests/blitz-tests/tests/render_blocking_stylesheet.rs:14; tests/blitz-tests/tests/render_blocking_stylesheet.rs:73-78)
- Text layout data is a parley-backed `TextLayout` reached through `element_data().inline_layout_data` (tests/blitz-tests/tests/line_break.rs:4-16; tests/blitz-tests/tests/br_trailing_line.rs:4-6)

## architecture §Established Decisions
- Hit-test results are canonicalized when stored: a hit on an anonymous block resolves to its containing element, a hit inside a pseudo-element subtree to the originating element; hover is re-resolved against fresh layout at the end of each `resolve` (tests/blitz-tests/tests/interaction_state_canonicalization.rs:5-10)
- Interaction state referencing genuinely removed DOM nodes is cleared (tests/blitz-tests/tests/stale_interaction_state.rs:11-14)
- On removal of a node referenced by interaction state, hover/active retarget to the nearest surviving element ancestor as a transient bridge, and focus resets to the body, encoded as `None`, running blur side-effects including disabling IME (tests/blitz-tests/tests/interaction_state_teardown.rs:6-14)
- A container's `layout_children` is kept persistently sorted in order-modified document order, re-sorted without reconstructing boxes when a child's `order` changes (tests/blitz-tests/tests/flex_grid_order.rs:1-4)
- Abspos children of a flex container key as `order` 0 and keep source order among themselves (tests/blitz-tests/tests/flex_grid_order.rs:149-155)
- An out-of-flow box is painted and hit-tested by its containing block, not its DOM parent (tests/blitz-tests/tests/paint_tree_incremental.rs:489-491)
- Hoisted offsets are computed at most once per geometry generation (tests/blitz-tests/tests/paint_tree_incremental.rs:275-316)
- Resolved CSS transforms are stored in device-pixel space: translation components scaled by the viewport scale factor exactly once, linear components not scaled (tests/blitz-tests/tests/transform_viewport_scale.rs:1-5)
- `scrollable_overflow` is stored in device (scaled) pixels while hit-test coordinates are CSS pixels; `Node::hit()` unscales it (tests/blitz-tests/tests/details_element.rs:108-113)
- Style resolution must not run while render-blocking stylesheets are still loading; pending stylesheets are tracked as pending critical resources (tests/blitz-tests/tests/render_blocking_stylesheet.rs:3-8; tests/blitz-tests/tests/render_blocking_stylesheet.rs:53-55)
- Inline `<svg>` is rendered by serializing its subtree and parsing it with usvg, from a snapshot with `currentColor` resolved, and is rebuilt when its styles change (tests/blitz-tests/tests/inline_svg_serialize.rs:1-2; tests/blitz-tests/tests/inline_svg_restyle.rs:1-2)
- `outer_html` does not resolve `currentColor` (tests/blitz-tests/tests/inline_svg_serialize.rs:68-76)
- SVG `width`/`height` attributes on `<svg>` are treated as presentation attributes mapped to CSS `width`/`height`, at the lowest cascade level (tests/blitz-tests/tests/svg_attr_sizing.rs:3-5; tests/blitz-tests/tests/svg_attr_sizing.rs:92-106)
- `lang` and `xml:lang` set the element's language through Stylo's internal inherited `-x-lang` property; `xml:lang` takes precedence regardless of attribute order (tests/blitz-tests/tests/lang_attribute.rs:1-2; tests/blitz-tests/tests/lang_attribute.rs:47-78)
- The `dir` attribute sets CSS direction as the HTML spec's bidi rendering section maps it; the engine stylesheet's inherited Gecko bidi rules were dropped by the servo selector parser (tests/blitz-tests/tests/dir_attribute.rs:1-6)
- On `remove_node`/`replace_node_with`, `node_id_mapping` entries for the subtree are intentionally not cleared so dioxus-core can reuse detached DOM nodes (tests/blitz-tests/tests/stale_node_mapping.rs:4-7)
- `disabled` is read as a parsed boolean rather than treating the bare attribute as disabling, so `""` does not count (tests/blitz-tests/tests/focusability_updates.rs:59-62)
- A `rotate` axis leaving the plane is still dropped (3d support missing); a z-axis rotate is applied as planar (tests/blitz-tests/tests/rotate_z_axis.rs:1-8; tests/blitz-tests/tests/rotate_z_axis.rs:59-63)
- A replaced element carrying a widget with no attributes uses a 300x150 default object size; content attributes override the widget-reported size (tests/blitz-tests/tests/custom_widget_layout.rs:69-78; tests/blitz-tests/tests/custom_widget_layout.rs:102-111)
- A `<link>` starts loading its stylesheet once it has both `rel=stylesheet` and an `href`, whichever is set last (tests/blitz-tests/tests/link_rel_attribute.rs:1-7)

## architecture §Conventions
- Each test file covers one behavior and opens with a `//!` module doc stating the behavior and, for regressions, the bug it guards (tests/blitz-tests/tests/anonymous_block_leak.rs:1-7; tests/blitz-tests/tests/comment_layout.rs:1-7)
- Regression tests cite upstream issue or PR URLs (tests/blitz-tests/tests/animations.rs:4; tests/blitz-tests/tests/render_blocking_stylesheet.rs:1; tests/blitz-tests/tests/stale_interaction_state.rs:5; tests/blitz-tests/tests/stale_node_mapping.rs:2; tests/blitz-tests/tests/custom_widget_layout.rs:3)
- Documents are built with `HtmlDocument::from_html` and a `DocumentConfig` giving `viewport: Some(Viewport::new(w, h, scale, ColorScheme::Light))` and `html_parser_provider: Some(Arc::new(HtmlProvider) as _)` with the rest defaulted, followed by `resolve(0.0)` (tests/blitz-tests/tests/accessibility_hidden.rs:139-145; tests/blitz-tests/tests/comment_layout.rs:14-25)
- Scenarios run in both layout modes with `for incremental in [false, true]` and an `incremental=` assertion message (tests/blitz-tests/tests/anonymous_block_cache_invalidation.rs:92-101; tests/blitz-tests/tests/flex_grid_order.rs:85-99)
- Test names are descriptive snake_case sentences (tests/blitz-tests/tests/dir_attribute.rs:32; tests/blitz-tests/tests/dir_attribute.rs:39)
- Assertion helpers are marked `#[track_caller]` (tests/blitz-tests/tests/accessibility_roles.rs:40-41)
- Some tests use `test_that` matchers (`verify_that!`, `matches_pattern!`, `contains`, `not`) and return `TestResult<()>` (tests/blitz-tests/tests/accessibility_hidden.rs:6; tests/blitz-tests/tests/accessibility_hidden.rs:9-25)
- A pointer-event builder function is repeated per file, constructing `BlitzPointerEvent` with identical page/screen/client coords (tests/blitz-tests/tests/details_element.rs:43-62; tests/blitz-tests/tests/fragment_navigation.rs:45-64; tests/blitz-tests/tests/scrollbar_drag.rs:13-32)
- Tests assert that a fixture actually produces the condition under test before asserting the fix ("otherwise the test proves nothing") (tests/blitz-tests/tests/anonymous_block_leak.rs:50-55; tests/blitz-tests/tests/interaction_state_canonicalization.rs:76-91)

## architecture §Standard Contracts
- `blitz_test_harness::Harness` constructors used: `from_html`, `from_html_with(html, HarnessOptions)`, `from_component`, `from_vdom(vdom, HarnessOptions)` (tests/blitz-tests/tests/harness_smoke.rs:10; tests/blitz-tests/tests/dir_attribute.rs:21-28; tests/blitz-tests/tests/harness_smoke.rs:104; tests/blitz-tests/tests/stale_node_mapping.rs:56)
- Harness inspection methods used: `layout_rect`, `layout_rect_of`, `center_of`, `text_content`, `attr`, `query`, `query_all`, `dom_string`, `node`, `focused`, `base`, `base_mut`, `hit`, `hit_node` (tests/blitz-tests/tests/harness_smoke.rs:16-28; tests/blitz-tests/tests/harness_smoke.rs:41-46; tests/blitz-tests/tests/oof_dynamic_cb.rs:226-233)
- Harness input methods used: `click`, `click_at`, `type_text`, `wheel_at`, `pump`, `dispatch_recorded` (tests/blitz-tests/tests/harness_smoke.rs:42; tests/blitz-tests/tests/pointer_events.rs:119; tests/blitz-tests/tests/harness_smoke.rs:62-63; tests/blitz-tests/tests/harness_smoke.rs:82; tests/blitz-tests/tests/dir_attribute.rs:78; tests/blitz-tests/tests/touch_events.rs:40-44)
- `dom_string()` renders an element as `<div #box .a .b> @ (20,10) 100x50` and text as a quoted string (tests/blitz-tests/tests/harness_smoke.rs:28-30)
- `HarnessOptions` has `width` and `height` fields and a `Default` (tests/blitz-tests/tests/dir_attribute.rs:23-27)
- `Harness` is generic over the document, e.g. `Harness<DioxusDocument>`, exposing `doc` (tests/blitz-tests/tests/stale_node_mapping.rs:41-43; tests/blitz-tests/tests/stale_node_mapping.rs:60-67)
- `DocumentConfig` fields used: `viewport`, `html_parser_provider`, `font_ctx`, `incremental`, `base_url`, `net_provider`, `shell_provider` (tests/blitz-tests/tests/accessibility_hidden.rs:140-143; tests/blitz-tests/tests/br_trailing_line.rs:26; tests/blitz-tests/tests/incremental_oracle.rs:45; tests/blitz-tests/tests/link_rel_attribute.rs:37-38; tests/blitz-tests/tests/interaction_state_teardown.rs:43)
- `NetProvider::fetch(&self, doc_id: usize, request: Request, handler: Box<dyn NetHandler>)` and `NetHandler::bytes(url, Bytes)` (tests/blitz-tests/tests/link_rel_attribute.rs:20-24; tests/blitz-tests/tests/render_blocking_stylesheet.rs:63-65)
- `ShellProvider::set_ime_enabled(&self, is_enabled: bool)` (tests/blitz-tests/tests/interaction_state_teardown.rs:31-35)
- `Widget` trait implementable with defaults, with `intrinsic_sizes()` returning `IntrinsicSizes` of optional `width`, `height`, `ratio` (tests/blitz-tests/tests/custom_widget_layout.rs:12-24)
- `doc.mutate()` mutator methods used: `create_element`, `create_text_node`, `set_attribute`, `clear_attribute`, `append_children`, `insert_nodes_before`, `remove_node`, `remove_and_drop_node`, `set_inner_html`, `set_node_text`, `set_custom_widget`, `set_style_property` (tests/blitz-tests/tests/detached_attribute.rs:27-36; tests/blitz-tests/tests/incremental_oracle.rs:484-494; tests/blitz-tests/tests/incremental_oracle.rs:517-528; tests/blitz-tests/tests/incremental_oracle.rs:554; tests/blitz-tests/tests/incremental_oracle.rs:588; tests/blitz-tests/tests/custom_widget_layout.rs:40; tests/blitz-tests/tests/line_break.rs:46-49; tests/blitz-tests/tests/animations.rs:23)
- Selector API: `query_selector`, `query_selector_all`, `query_selector_in`, `query_selector_all_in`, `matches_selector`, `closest`, `try_parse_selector_list`, and node-level `query_selector_all_raw`/`matches_selector_raw`/`closest_raw`, returning `Result` (tests/blitz-tests/tests/scoped_query_selector.rs:46-66; tests/blitz-tests/tests/scoped_query_selector.rs:158-174)
- Scroll API: `get_fragment_target`, `scroll_to_fragment`, `scroll_to_fragment_smooth`, `scroll_into_view(node, ScrollBehavior, ScrollLogicalPosition, ScrollLogicalPosition)`, `scroll_to`, `scroll_by`, `set_viewport_scroll`, `viewport_scroll`, `scroll_node_by` (tests/blitz-tests/tests/fragment_navigation.rs:97; tests/blitz-tests/tests/fragment_navigation.rs:121; tests/blitz-tests/tests/fragment_navigation.rs:156; tests/blitz-tests/tests/fragment_navigation.rs:203-208; tests/blitz-tests/tests/fragment_navigation.rs:308; tests/blitz-tests/tests/fragment_navigation.rs:349; tests/blitz-tests/tests/fragment_navigation.rs:230; tests/blitz-tests/tests/paint_tree_incremental.rs:262)
- `UiEvent` variants used: `PointerDown`, `PointerUp`, `PointerMove`, `PointerCancel`, `Wheel` with `BlitzWheelDelta::Pixels` (tests/blitz-tests/tests/details_element.rs:66-67; tests/blitz-tests/tests/fragment_navigation.rs:76-83; tests/blitz-tests/tests/touch_events.rs:110)
- `BlitzPointerId` variants used: `Mouse`, `Finger(0)`, `Pen` (tests/blitz-tests/tests/details_element.rs:45; tests/blitz-tests/tests/touch_action.rs:31; tests/blitz-tests/tests/touch_events.rs:74)
- Dispatched event names include `pointerdown`, `pointermove`, `pointerup`, `pointercancel`, `touchstart`, `touchmove`, `touchend`, `touchcancel`, `mousedown` (tests/blitz-tests/tests/touch_events.rs:46-62; tests/blitz-tests/tests/touch_events.rs:113-120; tests/blitz-tests/tests/touch_events.rs:197)
- `build_accessibility_tree()` returns an update whose `nodes` are `(NodeId, accesskit::Node)` pairs (tests/blitz-tests/tests/accessibility_roles.rs:56-61)
- `paint_scene(scene, doc, scale, width, height, x, y)` call shape (tests/blitz-tests/tests/background_size.rs:46)
- `EventDriver::new(doc, NoopEventHandler)` drives UI events through an event handler (tests/blitz-tests/tests/scrollbar_drag.rs:35-50)
- `DioxusDocument` exposes `vdom`, `inner` and `vdom_state.try_element_to_node_id(ElementId)` (tests/blitz-tests/tests/stale_node_mapping.rs:66; tests/blitz-tests/tests/stale_node_mapping.rs:76-80)

## architecture §Occupied Resources
- The environment variable `PAINT_TREE_BENCH_HTML` is read as a path to an HTML file (tests/blitz-tests/tests/paint_tree_bench.rs:258-269)
- Test documents use base URLs `http://example.com/` and `https://example.com/` (tests/blitz-tests/tests/link_rel_attribute.rs:37; tests/blitz-tests/tests/paint_tree_bench.rs:22)
- observed absent — network ports or listen addresses · searched: `localhost|127\.0\.0\.1|0\.0\.0\.0|bind\(` over the 61 slice files

## architecture §Infrastructure Patterns
- Network loads are dispatched through a pluggable `NetProvider` set on `DocumentConfig.net_provider`; the provider receives a document id and a `Request` with a `url` (tests/blitz-tests/tests/link_rel_attribute.rs:20-24; tests/blitz-tests/tests/link_rel_attribute.rs:34-41)
- out of slice — deployment, hosting, containers and build pipeline

## architecture §Cross-cutting Patterns
- Damage and dirty-flag invariant: `mark_ancestors_dirty` early-outs on the rule that a set `dirty_descendants` bit implies all ancestors are set; `clear_damage_and_dirty_flags` only descends into damaged subtrees (tests/blitz-tests/tests/stale_dirty_descendants.rs:5-20; tests/blitz-tests/tests/style_property_invalidation.rs:4-12)
- Nodes live in a slab/SlotMap; a stale `NodeId` panics with "invalid SlotMap key used" (tests/blitz-tests/tests/stale_interaction_state.rs:5-9; tests/blitz-tests/tests/stale_node_mapping.rs:12-16)
- Anonymous blocks exist only in `layout_children`, never DOM `children`, and the damage pass reaches them via `layout_parent` (tests/blitz-tests/tests/anonymous_block_cache_invalidation.rs:1-4)
- Every DOM mutation path inserts `CONSTRUCT_BOX` damage, which reconstructs anonymous blocks (tests/blitz-tests/tests/anonymous_block_cache_invalidation.rs:6-9)
- Animations, transitions and smooth scrolls advance through `resolve(time)` and are observed via `is_animating()` (tests/blitz-tests/tests/pseudo_element_update.rs:46-50; tests/blitz-tests/tests/fragment_navigation.rs:31-43)
- A color-scheme change recascades even viewport-independent elements because `light-dark()` and system colors resolve at cascade time (tests/blitz-tests/tests/device_coalescing.rs:94-110)
- Reading the stylist device flushes pending viewport changes (tests/blitz-tests/tests/device_coalescing.rs:80-92)

## architecture §Project Intent
- The crate describes itself as "Integration tests for Blitz" (tests/blitz-tests/Cargo.toml:3)
- Some tests are Rust ports of script-driven WPT tests that Blitz's WPT runner cannot run because they require script (tests/blitz-tests/tests/oof_dynamic_cb.rs:7-12)
- Regressions are traced to real sites rendered by the engine: Hacker News, gosub.io, old.reddit.com, www.wikipedia.org, bbc.co.uk, crowdsupply.com (tests/blitz-tests/tests/br_trailing_line.rs:6-9; tests/blitz-tests/tests/details_element.rs:112-113; tests/blitz-tests/tests/pseudo_element_update.rs:5-8; tests/blitz-tests/tests/rem_after_viewport_change.rs:4-8; tests/blitz-tests/tests/stale_dirty_descendants.rs:4; tests/blitz-tests/tests/svg_background_size.rs:6-7)
- Fixtures reproduce shapes from a "kopuz" application (route shell, showcase, titlebar) (tests/blitz-tests/tests/display_contents.rs:57; tests/blitz-tests/tests/display_contents.rs:91; tests/blitz-tests/tests/display_contents.rs:121; tests/blitz-tests/tests/pointer_events.rs:31)
- A comment states Tailwind v4 emits `inset: calc(var(--spacing) * 0)` and the test exercises it (tests/blitz-tests/tests/comment_layout.rs:91-113)

## architecture §Existing Scopes
- Accessibility tree construction and role mapping (tests/blitz-tests/tests/accessibility_hidden.rs:14; tests/blitz-tests/tests/accessibility_roles.rs:1-7)
- Block, inline, flex, grid, abspos/fixed and `display: contents` layout (tests/blitz-tests/tests/comment_layout.rs:1-7; tests/blitz-tests/tests/display_contents.rs:1-2; tests/blitz-tests/tests/flex_grid_order.rs:1-4; tests/blitz-tests/tests/oof_dynamic_cb.rs:1-5)
- Inline text layout: whitespace modes, `line-break`, baselines, `<br>`, fragment client rects (tests/blitz-tests/tests/whitespace_modes.rs:15-25; tests/blitz-tests/tests/line_break.rs:1-2; tests/blitz-tests/tests/inline_box_baseline.rs:1-2; tests/blitz-tests/tests/br_trailing_line.rs:1-2; tests/blitz-tests/tests/inline_fragment_rects.rs:1-4)
- Painting: backgrounds, SVG backgrounds, box shadows, paint order, overlay scrollbars (tests/blitz-tests/tests/background_size.rs:1; tests/blitz-tests/tests/svg_background_size.rs:1; tests/blitz-tests/tests/outset_box_shadow_shape.rs:1-4; tests/blitz-tests/tests/paint_order.rs:1-2; tests/blitz-tests/tests/scrollbars.rs:1-4)
- Paint tree / stacking contexts and hit testing (tests/blitz-tests/tests/paint_tree_incremental.rs:1-4; tests/blitz-tests/tests/pointer_events.rs:1-4)
- Input: pointer, touch, pen, wheel, `touch-action`, scrollbar drag, text selection, `<details>` toggling (tests/blitz-tests/tests/touch_events.rs:1-4; tests/blitz-tests/tests/touch_action.rs:1-3; tests/blitz-tests/tests/scrollbar_drag.rs:1; tests/blitz-tests/tests/text_selection_anonymous_block.rs:1-8; tests/blitz-tests/tests/details_element.rs:1-3)
- Scrolling and fragment navigation (tests/blitz-tests/tests/fragment_navigation.rs:1-2)
- Style invalidation, transforms, `rem`/viewport units, transitions (tests/blitz-tests/tests/style_property_invalidation.rs:1-2; tests/blitz-tests/tests/transform_2d_subset.rs:1-2; tests/blitz-tests/tests/rem_after_viewport_change.rs:1-2; tests/blitz-tests/tests/pseudo_element_update.rs:1-3)
- DOM mutation, memory leaks of anonymous blocks and inner HTML fragments (tests/blitz-tests/tests/anonymous_block_leak.rs:1-2; tests/blitz-tests/tests/inner_html_leak.rs:1)
- Dioxus integration via `DioxusDocument` (tests/blitz-tests/tests/stale_node_mapping.rs:18-23; tests/blitz-tests/tests/harness_smoke.rs:102-113)
- Stylesheet loading via `<link>` (tests/blitz-tests/tests/link_rel_attribute.rs:1-2; tests/blitz-tests/tests/render_blocking_stylesheet.rs:1-8)

## security-plan §Threat Model Summary
- out of slice — no threat model is stated in this test crate

## security-plan §Authentication & Authorization
- observed absent — authentication or authorization code · searched: `token|password|secret|api[_-]?key|authorization|cookie` (case-insensitive) over the 61 slice files
- The only `password` matches are an `<input type="password">` fixture mapped to `Role::PasswordInput` in an accessibility test (tests/blitz-tests/tests/accessibility_roles.rs:140; tests/blitz-tests/tests/accessibility_roles.rs:153)

## security-plan §Input Validation
- Inline SVG serialization must be well-formed XML: text with `&amp;`/`&lt;` and `xlink:href` must still parse (tests/blitz-tests/tests/inline_svg_serialize.rs:1-2; tests/blitz-tests/tests/inline_svg_serialize.rs:40-58)
- Selector parsing returns a `Result` (`try_parse_selector_list`, `query_selector(...).unwrap()`) (tests/blitz-tests/tests/scoped_query_selector.rs:32-35; tests/blitz-tests/tests/scoped_query_selector.rs:165)
- Scoped queries with a text or comment node as scope return no matches (tests/blitz-tests/tests/scoped_query_selector.rs:126-149)
- The `dir` attribute value is matched case-insensitively (tests/blitz-tests/tests/dir_attribute.rs:45-50)
- `disabled` is parsed as a boolean value (tests/blitz-tests/tests/focusability_updates.rs:59-76)

## security-plan §Data Protection
- out of slice — no data storage or protection code in this test crate

## security-plan §API Security
- out of slice — no network-facing API in this test crate

## security-plan §Dependency Security
- Every dev-dependency is declared `workspace = true`; no version is stated in this manifest (tests/blitz-tests/Cargo.toml:15-35)
- The crate is not published (`publish = false`) (tests/blitz-tests/Cargo.toml:4)
- out of slice — lockfile, audit or deny configuration

## security-plan §Secret Management
- observed absent — secret, token, API key or credential handling · searched: `token|password|secret|api[_-]?key|authorization|cookie` (case-insensitive) over the 61 slice files
- The only environment read is `PAINT_TREE_BENCH_HTML`, an HTML file path for an ignored benchmark (tests/blitz-tests/tests/paint_tree_bench.rs:262-269)

## security-plan §Error Handling
- Tests guard against panics from stale ids ("invalid SlotMap key used", "invalid key") and `unreachable!()` (tests/blitz-tests/tests/stale_interaction_state.rs:117-118; tests/blitz-tests/tests/animations.rs:4-7; tests/blitz-tests/tests/custom_widget_layout.rs:3-5; tests/blitz-tests/tests/stale_node_mapping.rs:118-136)
- Query APIs return `Result<Option<NodeId>>`; missing elements are `None` (tests/blitz-tests/tests/comment_layout.rs:37; tests/blitz-tests/tests/harness_smoke.rs:25)
- Unknown fragment targets return `None`/`false` and leave scroll at 0 (tests/blitz-tests/tests/fragment_navigation.rs:107-111; tests/blitz-tests/tests/fragment_navigation.rs:140-145)
- A stale animation entry for a removed animated node is to be skipped safely on the next resolve (tests/blitz-tests/tests/animations.rs:26-27)

## security-plan §Logging & Monitoring
- observed absent — a logging or monitoring crate · searched: `tracing|log::|env_logger|sentry|opentelemetry|metrics` over the 61 slice files
- Tests print diagnostics with `println!`/`eprintln!` (tests/blitz-tests/tests/br_trailing_line.rs:46-50; tests/blitz-tests/tests/text_selection_anonymous_block.rs:107)

## design-system §Color Palette
- The engine's default overlay scrollbar palette follows the viewport color scheme (Light vs Dark give distinct thumbs) (tests/blitz-tests/tests/scrollbars.rs:204-211)
- Default scrollbar thumbs paint as a fill plus a thin contrast stroke (tests/blitz-tests/tests/scrollbars.rs:194-202)
- Author `scrollbar-color` styles thumb and track; hover/drag feedback on a near-white author thumb blends toward the pole with contrast headroom (darken) (tests/blitz-tests/tests/scrollbars.rs:125-160; tests/blitz-tests/tests/scrollbar_drag.rs:224-227)
- `ColorScheme` has `Light` and `Dark` values set on the viewport (tests/blitz-tests/tests/device_coalescing.rs:101)

## design-system §Typography
- `blitz-dom` is built with the `system-fonts` feature for these tests (tests/blitz-tests/Cargo.toml:16)
- Without the `system-fonts` feature text measures 0x0 and font-dependent assertions pass vacuously; the feature is stated to be enabled by default when testing the whole workspace (tests/blitz-tests/tests/br_trailing_line.rs:11-13; tests/blitz-tests/tests/inline_box_baseline.rs:4-6)
- `rem` resolves against the root element's font-size, including after viewport and hidpi changes; the stylist's initial root font-size is 16px (tests/blitz-tests/tests/rem_after_viewport_change.rs:1-8; tests/blitz-tests/tests/rem_after_viewport_change.rs:49-77)
- observed absent — a project type scale or font tokens · searched: `font-family` over the 61 slice files (one fixture hit, `sans-serif`)

## design-system §Spacing
- out of slice — no spacing scale or tokens in this test crate

## design-system §Depth Strategy
- An outset box shadow takes the element's shape corner for corner, so a shadow with no blur and no spread hides behind the element ("what an elevation of 0 relies on"); expected pixels come from Chromium (tests/blitz-tests/tests/outset_box_shadow_shape.rs:1-6)
- Positioned descendants with `z-index: auto` share one paint level per CSS 2.1 Appendix E and paint in tree order (tests/blitz-tests/tests/paint_order.rs:1-2)

## design-system §Border Radius
- out of slice — only per-fixture `border-radius` values, no radius tokens

## design-system §Motion
- Smooth scrolling follows `scroll-behavior` on the root or element style; `auto` behavior jumps (tests/blitz-tests/tests/fragment_navigation.rs:264-293; tests/blitz-tests/tests/fragment_navigation.rs:374-383)
- A wheel event over a scroller cancels an in-progress smooth scroll (tests/blitz-tests/tests/fragment_navigation.rs:356-372)
- Overlay scrollbars appear on scroll activity and fade out after a delay; hovering where a hidden thumb would be does not summon it (tests/blitz-tests/tests/scrollbars.rs:1-4; tests/blitz-tests/tests/scrollbar_drag.rs:136-166)
- The scroll thumb changes appearance on hover and while dragged (tests/blitz-tests/tests/scrollbar_drag.rs:168-222)

## design-system §Iconography
- out of slice — only an inline SVG test icon fixture, no icon set

## design-system §Surface: none observed
- observed absent — an application surface (entry point, window or web bootstrap) · searched: `fn main|winit|wasm_bindgen|launch\(` over the 61 slice files (one comment-only hit naming the winit resize path)

## layout-templates §Surface: none observed
- observed absent — page or screen layout templates · searched: `fn main|winit|wasm_bindgen|launch\(` over the 61 slice files (one comment-only hit naming the winit resize path)

## test-plan §Test Scope Summary
- The crate holds Blitz's integration tests, one file per behavior under `tests/` (tests/blitz-tests/lib.rs:1; tests/blitz-tests/Cargo.toml:3)
- Coverage spans accessibility, layout, text, paint, hit testing, input events, scrolling, style invalidation, DOM mutation leaks, network-driven stylesheet loading and Dioxus integration (tests/blitz-tests/tests/accessibility_roles.rs:1; tests/blitz-tests/tests/incremental_oracle.rs:1; tests/blitz-tests/tests/whitespace_modes.rs:15; tests/blitz-tests/tests/paint_order.rs:1; tests/blitz-tests/tests/pointer_events.rs:1; tests/blitz-tests/tests/touch_events.rs:1; tests/blitz-tests/tests/fragment_navigation.rs:1; tests/blitz-tests/tests/style_property_invalidation.rs:1; tests/blitz-tests/tests/inner_html_leak.rs:1; tests/blitz-tests/tests/link_rel_attribute.rs:1; tests/blitz-tests/tests/stale_node_mapping.rs:1)

## test-plan §Test Strategy
- Two styles: direct `HtmlDocument` + `DocumentConfig` construction, and the `blitz_test_harness::Harness` wrapper over `HtmlDocument` and `DioxusDocument` (tests/blitz-tests/tests/comment_layout.rs:14-25; tests/blitz-tests/tests/harness_smoke.rs:1-3)
- Differential oracle: the same fixture and mutation sequence runs in an incremental and a non-incremental document, comparing every node's `final_layout()`, `layout_children` and paint tree after each step and asserting no damage remains (tests/blitz-tests/tests/incremental_oracle.rs:1-19; tests/blitz-tests/tests/incremental_oracle.rs:314-347)
- Pixel tests render to a CPU buffer and sample pixels; box-shadow expectations come from Chromium renders with a 1px antialiasing tolerance (tests/blitz-tests/tests/paint_order.rs:12-29; tests/blitz-tests/tests/outset_box_shadow_shape.rs:6; tests/blitz-tests/tests/outset_box_shadow_shape.rs:82-88)
- Regression tests document the bug and the real-world page that exposed it (tests/blitz-tests/tests/stale_dirty_descendants.rs:1-20; tests/blitz-tests/tests/rem_after_viewport_change.rs:1-8)
- WPT script tests are ported to Rust when the WPT runner cannot run them (tests/blitz-tests/tests/oof_dynamic_cb.rs:7-12)
- Deterministic pseudo-random template swaps (an LCG) run 1000 steps to shake out slab/ElementId reuse bugs (tests/blitz-tests/tests/stale_node_mapping.rs:176-217)
- Pure-restyle paths are driven by `:hover` rather than attribute mutation because mutations insert full damage and mask under-damaging bugs (tests/blitz-tests/tests/oof_dynamic_cb.rs:143-145; tests/blitz-tests/tests/anonymous_block_cache_invalidation.rs:6-9)
- Font-dependent tests skip at runtime with `eprintln!` when no usable font is available (tests/blitz-tests/tests/text_selection_anonymous_block.rs:63-78; tests/blitz-tests/tests/text_selection_anonymous_block.rs:104-109)

## test-plan §Test Harness Contract
- `Harness::from_html(html)` and `Harness::from_html_with(html, HarnessOptions)` build an HTML-backed harness; `Harness::from_component(fn)` and `Harness::from_vdom(vdom, options)` build Dioxus-backed ones (tests/blitz-tests/tests/harness_smoke.rs:10; tests/blitz-tests/tests/pointer_events.rs:8-17; tests/blitz-tests/tests/harness_smoke.rs:104; tests/blitz-tests/tests/stale_node_mapping.rs:49-56)
- `pump()` applies pending changes after a mutation through `base_mut().mutate()` (tests/blitz-tests/tests/dir_attribute.rs:72-79; tests/blitz-tests/tests/oof_dynamic_cb.rs:37-43)
- `dispatch_recorded([UiEvent, ...])` returns the list of dispatched event names (tests/blitz-tests/tests/touch_events.rs:40-44)
- The harness crate exports a `pointer_event(id, x, y, button, buttons, mods)` builder (tests/blitz-tests/tests/touch_events.rs:6; tests/blitz-tests/tests/touch_events.rs:12-21)
- `layout_rect(selector)` returns a rect with `x`, `y`, `width`, `height`; `center_of` returns an `(x, y)` tuple (tests/blitz-tests/tests/harness_smoke.rs:16-21)

## test-plan §Unit Test Strategy
- observed absent — in-file `#[cfg(test)]` unit-test modules · searched: `#\[cfg` over the 61 slice files
- out of slice — unit tests inside the engine crates

## test-plan §Integration Test Strategy
- All tests are Cargo integration tests in `tests/blitz-tests/tests/` with `#[test]` functions, depending only on dev-dependencies (tests/blitz-tests/lib.rs:1; tests/blitz-tests/Cargo.toml:13)
- Layout assertions read `final_layout()` location/size, `scrollable_overflow_rect`, `scroll_width()`/`scroll_height()` (tests/blitz-tests/tests/display_contents.rs:76-86; tests/blitz-tests/tests/inline_box_scrollable_overflow.rs:32-40)
- Style assertions read computed values via `primary_styles()` (tests/blitz-tests/tests/style_property_invalidation.rs:45-56)
- Restyle-avoidance is asserted by comparing computed-style pointers before and after (tests/blitz-tests/tests/resize_restyle.rs:39-44; tests/blitz-tests/tests/resize_restyle.rs:77-94)
- Leak checks compare `doc.tree().len()` and anonymous-block counts across repeated operations (tests/blitz-tests/tests/anonymous_block_leak.rs:23-76; tests/blitz-tests/tests/inner_html_leak.rs:11-43)

## test-plan §E2E Test Strategy
- `harness_smoke.rs` is described as end-to-end smoke tests for the harness covering document construction, inspection and input synthesis for `HtmlDocument` and `DioxusDocument` (tests/blitz-tests/tests/harness_smoke.rs:1-3)
- A Dioxus counter component is clicked through the harness and its rendered text asserted (tests/blitz-tests/tests/harness_smoke.rs:89-113)
- out of slice — windowed or browser-driven end-to-end runs

## test-plan §Test Data & Fixtures
- Fixtures are inline HTML string constants or `format!`-built pages (tests/blitz-tests/tests/incremental_oracle.rs:371-420; tests/blitz-tests/tests/dir_attribute.rs:12-29)
- Image load results are injected directly into `background_images` layers as `ImageData::Raster` or `ImageData::Svg` with `Status::Ok` (tests/blitz-tests/tests/background_size.rs:31-43; tests/blitz-tests/tests/svg_background_size.rs:37-46)
- Benchmark pages are generated in code: ~2000-node realistic, 5000-item stress, 50 nested stacking contexts, ~40k-node large page (tests/blitz-tests/tests/paint_tree_bench.rs:59-98; tests/blitz-tests/tests/paint_tree_bench.rs:208-221)
- An external page can be supplied through `PAINT_TREE_BENCH_HTML`, with stylesheets inlined (tests/blitz-tests/tests/paint_tree_bench.rs:258-269)

## test-plan §Mocking & Stubbing Discipline
- `RecordingNetProvider` records requested URLs instead of fetching (tests/blitz-tests/tests/link_rel_attribute.rs:14-24)
- `ManualNetProvider` holds requests and handlers so the test delivers responses when it chooses (tests/blitz-tests/tests/render_blocking_stylesheet.rs:16-30; tests/blitz-tests/tests/render_blocking_stylesheet.rs:62-66)
- `RecordingShell` implements `ShellProvider` and records `set_ime_enabled` calls (tests/blitz-tests/tests/interaction_state_teardown.rs:27-35)
- `NoopEventHandler` is passed to `EventDriver` (tests/blitz-tests/tests/scrollbar_drag.rs:3; tests/blitz-tests/tests/scrollbar_drag.rs:35)
- `Probe`/`SizedProbe` stub widgets implement `Widget` (tests/blitz-tests/tests/custom_widget_layout.rs:12-24)
- `NullScenePainter` stands in for a renderer in timings (tests/blitz-tests/tests/paint_tree_bench.rs:162)

## test-plan §CI Integration
- Benchmark tests are `#[ignore]` and run with `cargo test -p blitz-tests --release --test paint_tree_bench -- --ignored --nocapture` (tests/blitz-tests/tests/paint_tree_bench.rs:3; tests/blitz-tests/tests/paint_tree_bench.rs:225-227; tests/blitz-tests/tests/paint_tree_bench.rs:262-263; tests/blitz-tests/tests/paint_tree_bench.rs:341-342)
- Font-dependent assertions rely on the `system-fonts` feature, stated to be on by default when testing the whole workspace (tests/blitz-tests/tests/br_trailing_line.rs:11-13)
- out of slice — CI workflow files

## obs-plan §Obs Scope Summary
- out of slice — no observability scope is stated in this test crate

## obs-plan §Telemetry Strategy
- observed absent — telemetry or logging crates · searched: `tracing|log::|env_logger|sentry|opentelemetry|metrics` over the 61 slice files

## obs-plan §Observability Harness Contract
- out of slice — no observability harness in this test crate

## obs-plan §Span / Trace Coverage
- observed absent — spans or traces · searched: `tracing|log::|env_logger|sentry|opentelemetry|metrics` over the 61 slice files

## obs-plan §Metric Coverage
- A timing harness measures medians of `hit()`, render on a null backend, an incremental hover-only frame and a full non-incremental frame, printed as a markdown table (tests/blitz-tests/tests/paint_tree_bench.rs:1-3; tests/blitz-tests/tests/paint_tree_bench.rs:100-134; tests/blitz-tests/tests/paint_tree_bench.rs:341-370)
- The external-page timing also reports node count and hoisted stacking-context entries (tests/blitz-tests/tests/paint_tree_bench.rs:271-278)

## obs-plan §Log Coverage
- A `blitz-dom/log-phase-times` feature prints per-phase resolve timings (tests/blitz-tests/tests/paint_tree_bench.rs:223-224; tests/blitz-tests/tests/paint_tree_bench.rs:261)
- Test diagnostics go to stdout/stderr via `println!`/`eprintln!` (tests/blitz-tests/tests/br_trailing_line.rs:46-50; tests/blitz-tests/tests/paint_tree_bench.rs:266)

## obs-plan §Error Capture & Reporting
- out of slice — no error capture or reporting in this test crate

## obs-plan §PII Scrubbing & Compliance
- out of slice — no PII handling in this test crate

## obs-plan §CI Integration
- out of slice — CI workflow files

## a11y-plan §A11y Scope Summary
- `blitz-dom` is built with the `accessibility` feature and `accesskit` is a dev-dependency (tests/blitz-tests/Cargo.toml:16; tests/blitz-tests/Cargo.toml:30)
- Accessibility tests cover hidden-element exclusion and HTML-to-AccessKit role mapping (tests/blitz-tests/tests/accessibility_hidden.rs:8-137; tests/blitz-tests/tests/accessibility_roles.rs:1-7)

## a11y-plan §A11y Strategy
- Role mapping follows the HTML-AAM spec, linked in the test doc; previously links, lists, tables, labels and landmarks arrived as `Role::Unknown` (tests/blitz-tests/tests/accessibility_roles.rs:1-7)
- A semantic page asserts that only `<html>` and `<body>` map to `Role::Unknown` (tests/blitz-tests/tests/accessibility_roles.rs:182-198)

## a11y-plan §A11y Assertion Harness Contract
- Tests call `document.build_accessibility_tree()` after `resolve` and match `tree_update.nodes` with `test_that` matchers on `role()` and `is_hidden()` (tests/blitz-tests/tests/accessibility_hidden.rs:9-26)
- An `assert_role(html, element_id, expected)` helper maps the element's `NodeId` to an AccessKit `NodeId(node_id.as_u64())` and compares roles (tests/blitz-tests/tests/accessibility_roles.rs:39-72)
- An `unknown_tags(html)` helper lists the `html_tag` of nodes with `Role::Unknown` (tests/blitz-tests/tests/accessibility_roles.rs:16-37)

## a11y-plan §ARIA Patterns & Roles
- Landmarks: nav→Navigation, main→Main, aside→Complementary, footer→Footer, article→Article, blockquote→Blockquote, figure→Figure (tests/blitz-tests/tests/accessibility_roles.rs:74-93)
- Lists and tables: ul/ol→List, li→ListItem, table→Table, thead→RowGroup, tr→Row, th→ColumnHeader, th scope=row→RowHeader, td→Cell (tests/blitz-tests/tests/accessibility_roles.rs:95-115)
- An anchor is `Link` only with an `href`, otherwise `GenericContainer` (tests/blitz-tests/tests/accessibility_roles.rs:117-126)
- Form controls: label→Label, select→ComboBox, select multiple→ListBox, textarea→MultilineTextInput, progress→ProgressIndicator, meter→Meter, radio→RadioButton, range→Slider, email→EmailInput, password→PasswordInput, submit→Button (tests/blitz-tests/tests/accessibility_roles.rs:128-155)
- button→Button, div→GenericContainer, header→Header, h2→Heading, p→Paragraph, section→Section, text→TextInput, number→NumberInput, checkbox→CheckBox (tests/blitz-tests/tests/accessibility_roles.rs:157-180)
- `aria-hidden="true"` keeps the node but sets `is_hidden`; `hidden`, `display: none` and `visibility: hidden` exclude it; children of a hidden element are excluded (tests/blitz-tests/tests/accessibility_hidden.rs:28-137)

## a11y-plan §Keyboard Navigation
- An element's focusability is cached and follows `tabindex` set or cleared after creation (roving tabindex) (tests/blitz-tests/tests/focusability_updates.rs:1-6; tests/blitz-tests/tests/focusability_updates.rs:34-57)
- Setting `disabled` removes a button's focusability (tests/blitz-tests/tests/focusability_updates.rs:63-76)
- Clicking a checkbox focuses it (tests/blitz-tests/tests/harness_smoke.rs:33-44)
- Removing a focused text input runs blur side-effects and disables IME (tests/blitz-tests/tests/interaction_state_teardown.rs:189-217)
- observed absent — keyboard-event or Tab-order navigation tests · searched: `UiEvent::Key|KeyDown|Key::|Code::Tab|focus_next` over the 61 slice files

## a11y-plan §Visual Design Verification
- Default scrollbar thumbs carry a contrast stroke (tests/blitz-tests/tests/scrollbars.rs:194-202)
- A near-white author thumb must still visibly change on hover and drag (tests/blitz-tests/tests/scrollbar_drag.rs:224-292)
- `scrollbar-width: none` paints no scrollbar (tests/blitz-tests/tests/scrollbars.rs:162-175)

## a11y-plan §Screen Reader Support
- The role-mapping test states its goal as giving assistive technology something to navigate by (tests/blitz-tests/tests/accessibility_roles.rs:3-5; tests/blitz-tests/tests/accessibility_roles.rs:195-196)
- `lang`/`xml:lang` set the element language (`-x-lang`), inherited, with `lang=""` resetting to unknown (tests/blitz-tests/tests/lang_attribute.rs:1-2; tests/blitz-tests/tests/lang_attribute.rs:40-45)

## a11y-plan §Cognitive Accessibility
- out of slice — no cognitive-accessibility handling in this test crate

## a11y-plan §CI Integration
- out of slice — CI workflow files
