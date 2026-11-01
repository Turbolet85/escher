group: architecture-1
read: .claude/docs/conventions.md · 61 · «- Architecture: `.andromeda/architecture.md` · directory lay»
read: .claude/docs/gotchas.md · 68 · «- Decisions and rationale: `.andromeda/architecture.md` §Est»
read: .claude/docs/services/blitz-dom.md · 44 · «- `.andromeda/architecture.md` · `.claude/docs/stack.md` · `»
read: .claude/docs/services/blitz-paint.md · 35 · «- `.andromeda/architecture.md` · `.claude/docs/services/blit»
read: .claude/docs/services/blitz-shell.md · 34 · «- `.andromeda/architecture.md` · `.claude/docs/services/blit»
read: .claude/docs/services/blitz-traits.md · 33 · «- `.andromeda/architecture.md` · `.claude/docs/services/blit»
- leaf: .claude/docs/services/blitz-dom.md:15   class: stale
  leaf-text: «a line owned by Epoch 3's upstream sync,»
  source: .andromeda/architecture.md:60 «the merge surface of those eight is a line owned by Upstream sync ahead of determinism and exploration»
  replace-with: «a line owned by Upstream sync ahead of determinism and exploration,»
- leaf: .claude/docs/services/blitz-dom.md:15   class: stale
  leaf-text: «a textual conflict with the non-invertible answer.»
  source: .andromeda/architecture-detail/established-decisions.md:79 «its `hit_inner` hunk is a textual conflict with the non-invertible answer for the next sync, `0980cce5f` being left out at `e42ee1f25`»
  replace-with: «a textual conflict with the non-invertible answer for the next sync, `0980cce5f` being left out at `e42ee1f25`.»
- leaf: .claude/docs/services/blitz-dom.md:15   class: stale
  leaf-text: «a line owned by Epoch 3's Upstream sync ahead of the UI graph:»
  source: .andromeda/architecture.md:62 «a line owned by Upstream sync ahead of determinism and exploration.»
  replace-with: «a line owned by Upstream sync ahead of determinism and exploration:»
- leaf: .claude/docs/services/blitz-dom.md:15   class: omission
  leaf-text: «The select model, a direct engine change for every document, remote pages included»
  source: .andromeda/architecture-detail/established-decisions.md:80 «`BaseDocument::deallocate_anonymous_block` takes the block's `layout_children` and sets `layout_parent` to `None` on each child still in the slab naming the block»
  replace-with: «A dropped anonymous block clears its children's `layout_parent` beside those eight, a direct engine change for every document (ratified, flagged upstreamable and nothing sent, its merge surface a line owned by Upstream sync ahead of determinism and exploration): `BaseDocument::deallocate_anonymous_block` takes the block's `layout_children` and sets `layout_parent` to `None` on each child still in the slab naming the block, so a key press with focus on a control whose anonymous block was dropped does not panic, for `hidden`, `style="display:none"` and CSS alone, and every reader of a stale `layout_parent` is cured with it; focus stays and the key still acts, `DocumentMutator::flush` reading `is_inert()` alone. The select model, a direct engine change for every document, remote pages included»
- leaf: .claude/docs/services/blitz-dom.md:11   class: omission
  leaf-text: «`HtmlParserProvider` (`Dummy*` defaults).»
  source: .andromeda/architecture-detail/standard-contracts.md:23 «`NetHandler::error(url, error)` stands beside `bytes`: `ResourceHandler<StylesheetHandler>::error` responds `Err(error)`, so `pending_critical_resources` loses the request»
  replace-with: «`HtmlParserProvider` (`Dummy*` defaults). `NetHandler::error(url, error)` stands beside `bytes`: `ResourceHandler<StylesheetHandler>::error` responds `Err(error)`, so `pending_critical_resources` loses the request and a failed stylesheet stops blocking render, and `net.rs`'s other resource kinds keep the trait's do-nothing default; an engine change for every document.»
- leaf: .claude/docs/services/blitz-dom.md:15   class: omission
  leaf-text: «`DocumentMutator`, `EventDriver`/`EventHandler`,»
  source: .andromeda/architecture-detail/standard-contracts.md:39 «`set_attribute`, `clear_attribute`, removal and insertion restyle through `DocumentMutator::restyle_for_attribute_change` and `restyle_for_child_list_change`»
  replace-with: «`DocumentMutator` (`set_attribute`, `clear_attribute`, removal and insertion restyle through `restyle_for_attribute_change` and `restyle_for_child_list_change`, with no blanket `RestyleHint::restyle_subtree()`, and the changed-set insert in `remove_and_drop_all_children` stands first; public `BaseDocument::snapshot_node` takes `attr: &LocalName`, its two callers both in `mutator.rs`; an engine change for every document), `EventDriver`/`EventHandler`,»
- leaf: .claude/docs/services/blitz-dom.md:15   class: omission
  leaf-text: «query-selector API, CSSOM, scroll API (»
  source: .andromeda/architecture-detail/standard-contracts.md:38 «The resolved `width` and `height` of `resolved_style_value` read `node.unrounded_layout()`, so a reader gets sub-pixel values where `final_layout` rounds»
  replace-with: «query-selector API, CSSOM (the resolved `width` and `height` of `resolved_style_value` read `node.unrounded_layout()`, so a reader gets sub-pixel values where `final_layout` rounds; an engine change for every document), scroll API (»
- leaf: .claude/docs/services/blitz-dom.md:15   class: omission
  leaf-text: «`Widget` trait, the public»
  source: .andromeda/architecture-detail/standard-contracts.md:92 «`ElementData::set_custom_widget` replays each existing attribute to `Widget::attribute_changed` with no old value, behind `custom-widget`»
  replace-with: «`Widget` trait (`ElementData::set_custom_widget` replays each existing attribute to `Widget::attribute_changed` with no old value, behind `custom-widget`), the public»
- leaf: .claude/docs/services/blitz-dom.md:18   class: omission
  leaf-text: «→ hover refresh → sub-documents.»
  source: .andromeda/architecture-detail/standard-contracts.md:72 «`TElement::borrow_data` filters data with no primary style, and the style-resolve context is scoped; an engine change for every document»
  replace-with: «→ hover refresh → sub-documents. `TElement::borrow_data` filters data with no primary style, and the style-resolve context is scoped; an engine change for every document.»
- leaf: .claude/docs/services/blitz-dom.md:20   class: omission
  leaf-text: «`writing-mode` (vertical writing modes) is opt-in:»
  source: .andromeda/architecture-detail/occupied-resources.md:80 «The `Once` `INIT` in blitz-dom's `pub(crate) fn init_stylo_prefs`, process-wide: the first call, from `UserAgentStylesheet::parse` or `BaseDocument::new`, sets Stylo's prefs»
  replace-with: «The `Once` `INIT` in the `pub(crate) fn init_stylo_prefs`, process-wide: the first call, from `UserAgentStylesheet::parse` or `BaseDocument::new`, sets Stylo's prefs under their Stylo 0.22 names before the user-agent sheet is parsed, and a later call does nothing. `writing-mode` (vertical writing modes) is opt-in:»
- leaf: .claude/docs/services/blitz-dom.md:21   class: omission
  leaf-text: «`BaseDocument` implements none of them.»
  source: .andromeda/architecture-detail/standard-contracts.md:106 «`render_element` skips such a box's own shadow, background and border through `paints_own_box`, with no frame test; an engine change for every document»
  replace-with: «`BaseDocument` implements none of them. `collect_layout_children_with_wrap` sets `NodeFlags::IS_INLINE_WITH_ONLY_BLOCK_CHILDREN`, read by the public `Node::is_inline_with_only_block_children`; `render_element` skips such a box's own shadow, background and border through `paints_own_box`, with no frame test; an engine change for every document.»
- leaf: .claude/docs/services/blitz-dom.md:21   class: omission
  leaf-text: «Every box-generating `::before` / `::after` votes in flow classification»
  source: .andromeda/architecture-detail/standard-contracts.md:85 «`Node`'s stacking-context arm reads a `mix-blend-mode` other than `normal`; an engine change for every document»
  replace-with: «`Node`'s stacking-context arm reads a `mix-blend-mode` other than `normal`; an engine change for every document. Every box-generating `::before` / `::after` votes in flow classification»
- leaf: .claude/docs/services/blitz-dom.md:21   class: omission
  leaf-text: «two upstream engine changes for every document, not counted among escher's direct ones.»
  source: .andromeda/architecture-detail/established-decisions.md:93 «Twelve more behaviour changes for every document are upstream's and are not counted among escher's direct engine changes above, each ratified for its take, by its commit»
  replace-with: «two upstream engine changes for every document, not counted among escher's direct ones. Twelve more behaviour changes for every document are upstream's and are not counted among escher's direct ones, each ratified for its take, by its commit: `fe33b778b` (#1276, restyle narrowing) · `8b1a0a3ce` (#1312, table damage) · `dee854c76` (#1317, `<picture>`) · `f6edb9701` (#1318, failed stylesheets) · `6fca6308d` (#1316, blend and isolation) · `7a3dd8918` (#1310, prefs before the UA sheet) · `91ef65e21` (#1314, `URLSearchParams`) · `a26c6ddeb` (#1322, `CustomWidget` attributes) · `f76777766` (#1319, inline decorations) · `6ef4b532d` (#1305, mounted events) · `ae774d7f4` (#1339, computed sizes) · `e42ee1f25` (#1342, culling bounds, a port).»
- leaf: .claude/docs/gotchas.md:21   class: omission
  leaf-text: «`resolve` returns before styling while a `<head>` stylesheet is pending — layout reads look empty.»
  source: .andromeda/architecture-detail/established-decisions.md:45 «a failed, empty or non-CSS stylesheet stops blocking render»
  replace-with: «`resolve` returns before styling while a `<head>` stylesheet is pending — layout reads look empty; a failed, empty or non-CSS stylesheet stops blocking render, and a bad `href`, `doc.url.resolve_relative` refusing it, starts no load.»
- leaf: .claude/docs/gotchas.md:22   class: omission
  leaf-text: «a `NetProvider` with `is_noop() == true` registers no pending-critical resources.»
  source: .andromeda/architecture-detail/design-philosophy.md:26 «a wrapping handler must forward it or the failure stops at the wrapper»
  replace-with: «a `NetProvider` with `is_noop() == true` registers no pending-critical resources; a wrapping handler must forward `NetHandler::error` or the failure stops at the wrapper, as blitz-test-harness's `CountedHandler` forwards it.»
- leaf: .claude/docs/gotchas.md:41   class: omission
  leaf-text: «subgrid/masonry are absent.»
  source: .andromeda/architecture-detail/established-decisions.md:58 «`background-clip: text` is not supported, `BackgroundClip::Text` being `gecko`-only on Stylo 0.22, and upstream `ac6f9f582`, which paints it, stays left out»
  replace-with: «subgrid/masonry are absent; `background-clip: text` is not supported, `BackgroundClip::Text` being `gecko`-only on Stylo 0.22, and upstream `ac6f9f582`, which paints it, stays left out.»
- leaf: .claude/docs/services/blitz-paint.md:14   class: omission
  leaf-text: «past any clip or `opacity` on its select's ancestors,»
  source: .andromeda/architecture-detail/standard-contracts.md:130 «so an open list, through the `paint_select_list` call above, paints last, past the blend layers»
  replace-with: «past any clip or `opacity` on its select's ancestors and past the blend layers,»
- leaf: .claude/docs/services/blitz-paint.md:17   class: omission
  leaf-text: «`kurbo_css` builds rounded border geometry;»
  source: .andromeda/architecture-detail/standard-contracts.md:130 «`mix-blend-mode` and `isolation` paint in every document: blitz-paint's private module `blend` holds `pub(crate) convert_blend_mode`»
  replace-with: «`kurbo_css` builds rounded border geometry; `mix-blend-mode` and `isolation` paint in every document: the private module `blend` holds `pub(crate) convert_blend_mode`, `LayerManager::maybe_with_layer` and `maybe_push_layer` take a `BlendMode`, and `render_element` pushes a viewport-wide blend layer and pops it last;»
- leaf: .claude/docs/services/blitz-paint.md:22   class: omission
  leaf-text: «non-finite transformed boxes are culled;»
  source: .andromeda/architecture-detail/established-decisions.md:173 «`render_element`'s border box and culling stand on the scaled size, `location` and `box_position` kept for the transform»
  replace-with: «non-finite transformed boxes are culled; `render_element`'s border box and culling stand on the scaled size, `location` and `box_position` kept for the transform (an upstream engine change for every document, upstream #1342);»
- leaf: .claude/docs/services/blitz-paint.md:23   class: omission
  leaf-text: «Painting an inline root without inline layout data panics.»
  source: .andromeda/architecture-detail/standard-contracts.md:106 «`render_element` skips such a box's own shadow, background and border through `paints_own_box`, with no frame test»
  replace-with: «Painting an inline root without inline layout data panics. `render_element` skips the own shadow, background and border of a box whose `Node::is_inline_with_only_block_children` reads true, through `paints_own_box`, with no frame test.»
- leaf: .claude/docs/services/blitz-paint.md:27   class: omission
  leaf-text: «`src/layers.rs`»
  source: .andromeda/architecture-detail/existing-scopes.md:58 «Modules blend (private, its `pub(crate) convert_blend_mode` converting Stylo's blend mode: packages/blitz-paint/src/blend.rs:5-26)»
  replace-with: «`src/layers.rs`, `src/blend.rs` (`convert_blend_mode`)»
- leaf: .claude/docs/services/blitz-paint.md:30   class: omission
  leaf-text: «`cargo test -p blitz-paint` (gradient.rs, css_box.rs)»
  source: .andromeda/architecture-detail/standard-contracts.md:131 «The blend has no frame witness: no fork or upstream test paints a blended element to a frame or names `convert_blend_mode`, and its unit tests are the 3 of `layers.rs`»
  replace-with: «`cargo test -p blitz-paint` (gradient.rs, css_box.rs, layers.rs); the blend has no frame witness: no fork or upstream test paints a blended element to a frame or names `convert_blend_mode`, and its unit tests are the 3 of `layers.rs` (owner: a `CARRY:` on Upstream sync ahead of determinism and exploration)»
- leaf: .claude/docs/services/blitz-traits.md:14   class: omission
  leaf-text: «`NetHandler::bytes`;»
  source: .andromeda/architecture-detail/standard-contracts.md:114 «and beside it `NetHandler::error(self: Box<Self>, url: String, error: String)`, a default that does nothing»
  replace-with: «`NetHandler::bytes` and, beside it, `NetHandler::error(self: Box<Self>, url: String, error: String)`, a default that does nothing, which a wrapping handler must forward or the failure stops at the wrapper;»
