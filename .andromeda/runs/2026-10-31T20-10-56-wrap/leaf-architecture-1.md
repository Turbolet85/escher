group: architecture-1
read: .claude/docs/conventions.md · 61 · «- Architecture: `.andromeda/architecture.md` · directory lay»
read: .claude/docs/gotchas.md · 68 · «- Decisions and rationale: `.andromeda/architecture.md` §Est»
read: .claude/docs/services/blitz-dom.md · 44 · «- `.andromeda/architecture.md` · `.claude/docs/stack.md` · `»
read: .claude/docs/services/blitz-paint.md · 35 · «- `.andromeda/architecture.md` · `.claude/docs/services/blit»
read: .claude/docs/services/blitz-shell.md · 34 · «- `.andromeda/architecture.md` · `.claude/docs/services/blit»
read: .claude/docs/services/blitz-traits.md · 33 · «- `.andromeda/architecture.md` · `.claude/docs/services/blit»
- leaf: .claude/docs/services/blitz-dom.md:15   class: omission
  leaf-text: «`scroll_into_view` scrolls every scrolling box that holds its target, innermost first, then the viewport — a nested box at once, the requested behaviour for the viewport alone;»
  source: .andromeda/architecture-detail/standard-contracts.md:54 «on a row of an open select list it writes the list's `scroll_y` at once and returns, moving no chain and no viewport»
  replace-with: «`scroll_into_view` scrolls every scrolling box that holds its target, innermost first, then the viewport — a nested box at once, the requested behaviour for the viewport alone — and on a row of an open select list writes the list's `scroll_y` at once and returns, moving no chain and no viewport;»
- leaf: .claude/docs/services/blitz-dom.md:15   class: omission
  leaf-text: «`visible_region` reads the part of the viewport an element can be seen through — never a hit»
  source: .andromeda/architecture-detail/standard-contracts.md:55 «`visible_region(&self, node_id) -> Option<BoundingRect>` is the read-only reader beside it, for a row or a group of an open select list the list's scrollport (below)»
  replace-with: «`visible_region` reads the part of the viewport an element can be seen through — never a hit — and for a row or a group of an open select list the list's scrollport»
- leaf: .claude/docs/services/blitz-dom.md:15   class: omission
  leaf-text: «the hit walk (it tests a node's own area where the box stands»
  source: .andromeda/architecture.md:147 «`hit_with_scrollbar` answers an open select list first and no scrollbar with it, `hit_inner` holding none of it, so `hit`, `set_hover_to` and `handle_pointerdown` all read the list»
  replace-with: «the hit walk (`hit_with_scrollbar` answers an open select list first and no scrollbar with it, `hit_inner` holding none of it, so `hit`, `set_hover_to` and `handle_pointerdown` all read the list; past that the walk tests a node's own area where the box stands»
- leaf: .claude/docs/services/blitz-dom.md:15   class: stale
  leaf-text: «an `optgroup`'s on its `Role::Group`, named in the tree and unpainted»
  source: .andromeda/architecture-detail/standard-contracts.md:29 «and an `optgroup`'s (`Role::Group`), named in the tree and painted as a header in an open list»
  replace-with: «an `optgroup`'s on its `Role::Group`, named in the tree and painted as a header in an open list»
- leaf: .claude/docs/services/blitz-dom.md:15   class: stale
  leaf-text: «`aria-expanded` (a boolean)»
  source: .andromeda/architecture-detail/standard-contracts.md:30 «`aria-expanded` (a boolean, a single select excepted, which reads none and writes `expanded` from its list)»
  replace-with: «`aria-expanded` (a boolean, a single select excepted, which reads none and writes `expanded` from its list, an author's `open` on a select steering nothing)»
- leaf: .claude/docs/services/blitz-dom.md:15   class: stale
  leaf-text: «a focused single-choice select that is no disabled form control takes a pressed»
  source: .andromeda/architecture-detail/established-decisions.md:78 «a focused single-choice `select` (no `multiple`), its list closed, that is no disabled form control takes a pressed `ArrowDown` / `ArrowUp`»
  replace-with: «a focused single-choice select, its list closed, that is no disabled form control takes a pressed»
- leaf: .claude/docs/services/blitz-dom.md:15   class: retired
  leaf-text: «every other key doing nothing, so Enter and Space open nothing and a released Space activates no select;»
  source: .andromeda/architecture-detail/established-decisions.md:78 «Enter, and a Space with no typed text standing, opens its list (below), every other key does nothing, and a released Space does nothing more»
  replace-with: «Enter, and a Space with no typed text standing, opening its list, every other key doing nothing and a released Space doing nothing more;»
- leaf: .claude/docs/services/blitz-dom.md:15   class: stale
  leaf-text: «a click on a select clears the typeahead and the text selection and focuses it, a disabled select neither focusing nor acting;»
  source: .andromeda/architecture-detail/established-decisions.md:78 «focuses it through `generate_focus_events` / `set_focus_to`, and on a single select opens its list or closes one standing»
  replace-with: «a click on a select clears the typeahead and the text selection and focuses it, and on a single select opens its list or closes one standing, a disabled select neither focusing nor acting;»
- leaf: .claude/docs/services/blitz-dom.md:15   class: stale
  leaf-text: «and the public `clear_select_typeahead(&mut self)`, its one non-test caller escher-driver's `focus_multiple_select`;»
  source: .andromeda/architecture-detail/standard-contracts.md:21 «public `BaseDocument::clear_select_typeahead(&mut self)`, its one non-test caller `execute.rs` `focus_select`»
  replace-with: «and the public `clear_select_typeahead(&mut self)`, its one non-test caller escher-driver's `focus_select`;»
- leaf: .claude/docs/services/blitz-dom.md:15   class: omission
  leaf-text: «form submission sends every chosen option that is not disabled, by its value;»
  source: .andromeda/architecture-detail/standard-contracts.md:18 «`open_select`, `select_list_bounds`, `select_list_view` and `select_row_layout`; public writer `close_select_list`, its callers keyboard, pointer, reconcile and the driver's `focus_select`»
  replace-with: «an open list's API on `BaseDocument` — public read-only `open_select`, `select_list_bounds`, `select_list_view` and `select_row_layout`, public `close_select_list` (its callers keyboard, pointer, reconcile and the driver's `focus_select`), the rest crate-private or private, the list's state in the crate-private `node::select_list` but for two public read types re-exported from the crate root, `SelectListView`, the list's colours its associated constants, and `SelectListRow`; form submission sends every chosen option that is not disabled, by its value;»
- leaf: .claude/docs/services/blitz-dom.md:15   class: retired
  leaf-text: «Its limits: no popup is painted (owner: the route entry Select popup), Enter and Space open nothing on a single select, an option of a closed single select»
  source: .andromeda/architecture.md:64 «a click, Enter and a Space with no typed text standing open a single select's list, which the engine paints over the page with a group's `label` as a header»
  replace-with: «Its limits: an option of a closed single select»
- leaf: .claude/docs/services/blitz-dom.md:15   class: stale
  leaf-text: «`size` is unread and Ctrl+A is not modelled»
  source: .andromeda/architecture.md:64 «a `multiple` select's `size` hints its height, a single select's stays unread, and Ctrl+A is not modelled»
  replace-with: «a single select's `size` is unread — a `multiple` select's hints a `height` of `size` × 1.2em through `size_rows`, capped at 65534, `0` or a bad value no hint, with `select[multiple][size] { overflow-y: auto; }` and `line-height: 1.2` on its rows in the default sheet, and a key brings the current row into view only where the select carries a valid `size` — and Ctrl+A is not modelled»
- leaf: .claude/docs/services/blitz-dom.md:15   class: omission
  leaf-text: «a `multiple` select and an `optgroup`'s options are modelled in the keys, pointer, form, tree, sheet, snapshot and bridge.»
  source: .andromeda/architecture-detail/established-decisions.md:80 «The select list stands beside the select model, a direct engine change for every document, remote pages included, ratified: a single select opens a list the engine paints over the page by a click,»
  replace-with: «a `multiple` select and an `optgroup`'s options are modelled in the keys, pointer, form, tree, sheet, snapshot and bridge. The select list stands beside the select model, a direct engine change for every document, remote pages included, ratified, its merge surface the sixteen files of `escher-0.3.0/chunks/2026-10-31-select-popup/evidence/merge-surface.md`, a line owned by Epoch 3's Upstream sync ahead of the UI graph: a single select opens a list the engine paints over the page by a click, by Enter, or by a Space with no typed text standing, a bound label's click focusing the select and opening nothing; the list shows at most `MAX_ROWS` (20) rows, opens where its rows fit and else on the side with more room, is at least its select's width and at most the viewport's, and a select with no row to show opens nothing; `SelectList.fresh` holds a just-opened list against a `:focus` rule's restyle; it closes with no event on Escape, a second click on the select, a press outside, a wheel outside the list and a page scroll or move of the select's box, and removal, `disabled`, `hidden` or `inert` on the select closes it too; Tab chooses the current row and closes before focus moves, in either direction; Enter, Space and a click on a row choose through `apply_select_choice`, a disabled row changing nothing and leaving the list open; the move keys shift the highlight over enabled rows and choose nothing; a wheel over the list scrolls the list alone and a wheel elsewhere closes it and then scrolls. Its limits: the thumb is painted and not draggable, a highlighted row's contrast is unread by the lint, a larger authored font clips in the one row height, and the highlight is absent from the snapshot and the tree.»
- leaf: .claude/docs/services/blitz-dom.md:18   class: stale
  leaf-text: «clear damage → hover refresh → sub-documents.»
  source: .andromeda/architecture.md:254 «paint tree, clear damage, `reconcile_select_list`, hover refresh, sub-documents»
  replace-with: «clear damage → `reconcile_select_list` → hover refresh → sub-documents.»
- leaf: .claude/docs/services/blitz-dom.md:19   class: stale
  leaf-text: «a key or typeahead or, on a `multiple` select, a click on a row changed its chosen option or set»
  source: .andromeda/architecture-detail/cross-cutting-patterns.md:40 «marks a select where a key or typeahead or, on a `multiple` select or a single select's open list, a click on a row»
  replace-with: «a key or typeahead or, on a `multiple` select or a single select's open list, a click on a row changed its chosen option or set»
- leaf: .claude/docs/services/blitz-dom.md:19   class: stale
  leaf-text: «— and NOT by node creation, hover, active, scroll, resize, resource loads, animation or layout, so an idle `resolve` marks nothing in either layout mode.»
  source: .andromeda/architecture-detail/cross-cutting-patterns.md:41 «the fifth kind of site is a select's list: its opening or closing marks the select, a page scroll or a move of the select's box that closes the list marks it through `reconcile_select_list`»
  replace-with: «— and by a select's list, its opening or closing marking the select, a page scroll or a move of the select's box that closes the list marking it through `reconcile_select_list` at the next resolve, a wheel elsewhere through `close_select_list`, and a highlight or a wheel move inside the list marking nothing — and, with no select list standing, NOT by node creation, hover, active, scroll, resize, resource loads, animation or layout, so an idle `resolve` marks nothing in either layout mode.»
- leaf: .claude/docs/services/blitz-dom.md:27   class: omission
  leaf-text: «a main-button pointerdown in the shown overlay scrollbar's band is taken as a thumb drag and dispatches nothing (`events/pointer.rs`, `hit_with_scrollbar` while `scrollbar_opacity > 0`; a measured hazard, no engine change)»
  source: .andromeda/architecture-detail/established-decisions.md:94 «A row of an open select list is never taken as a thumb drag, `hit_with_scrollbar` answering `select_list_hit` first and no scrollbar with it»
  replace-with: «a main-button pointerdown in the shown overlay scrollbar's band is taken as a thumb drag and dispatches nothing (`events/pointer.rs`, `hit_with_scrollbar` while `scrollbar_opacity > 0`; a measured hazard, no engine change, and a row of an open select list is never taken as a thumb drag, `hit_with_scrollbar` answering `select_list_hit` first and no scrollbar with it)»
- leaf: .claude/docs/services/blitz-dom.md:27   class: omission
  leaf-text: «on the virtual clock the bar stays shown until an `advance` passes its fade, so»
  source: .andromeda/architecture-detail/established-decisions.md:94 «on the virtual clock the bar stays shown until an `advance` passes its fade, and a pointer left on the thumb keeps it shown»
  replace-with: «on the virtual clock the bar stays shown until an `advance` passes its fade, and a pointer left on the thumb keeps it shown, so»
- leaf: .claude/docs/services/blitz-paint.md:6   class: omission
  leaf-text: «It assumes styles and layout are resolved and never mutates the document's layout.»
  source: .andromeda/architecture-detail/design-philosophy.md:13 «its rows' text is the carve-out, laid out at paint through the public `select_row_layout`, only the rows in the scrollport»
  replace-with: «It assumes styles and layout are resolved and never mutates the document's layout; an open select's rows' text is the carve-out, laid out at paint through blitz-dom's public `select_row_layout`, only the rows in the scrollport.»
- leaf: .claude/docs/services/blitz-paint.md:14   class: omission
  leaf-text: «`paint_scene(PaintScene, &mut BaseDocument, scale, width, height, x_offset, y_offset)`.»
  source: .andromeda/architecture-detail/standard-contracts.md:115 «which calls `paint_select_list` after the root's render and before the devtools overlays»
  replace-with: «`paint_scene(PaintScene, &mut BaseDocument, scale, width, height, x_offset, y_offset)`, which calls `paint_select_list` after the root's render and before the devtools overlays, an open select's list read from `SelectListView` and past any clip or `opacity` on its select's ancestors, reaching a shell or renderer through `paint_scene` alone.»
- leaf: .claude/docs/services/blitz-paint.md:17   class: omission
  leaf-text: «`kurbo_css` builds rounded border geometry.»
  source: .andromeda/architecture-detail/standard-contracts.md:117 «`paint_select_list`, in blitz-paint's module `render::select_list`, paints a non-finite box not at all, lays out only rows in the scrollport, and draws its thumb only under the `scrollbars` feature»
  replace-with: «`kurbo_css` builds rounded border geometry; `paint_select_list`, in the module `render::select_list`, paints a non-finite box not at all, lays out only rows in the scrollport, and draws its thumb only under the `scrollbars` feature.»
- leaf: .claude/docs/services/blitz-paint.md:27   class: omission
  leaf-text: «`src/text.rs`»
  source: .andromeda/architecture-detail/standard-contracts.md:118 «`stroke_text` delegates to the `pub(crate)` `stroke_text_as`, which takes `RunPaint { color, synthesize_bold }`»
  replace-with: «`src/text.rs` (`stroke_text` delegates to the `pub(crate)` `stroke_text_as`, which takes `RunPaint { color, synthesize_bold }`, every other caller passing `RunPaint::default()`; a synthesized bold, a list group's header, adds one embolden weight to a build's own)»
- leaf: .claude/docs/services/blitz-paint.md:31   class: omission
  leaf-text: «`tests/blitz-tests/tests/{paint_order,background_size,outset_box_shadow_shape,scrollbars}.rs` render to a CPU buffer (`anyrender_vello_cpu`)»
  source: .andromeda/architecture-detail/existing-scopes.md:114 «`select_popup_painted`, 6 tests — the highlight and a disabled row against their constants, the plain pair, opacity and clips, a closed select's frame byte-equal»
  replace-with: «`tests/blitz-tests/tests/{paint_order,background_size,outset_box_shadow_shape,scrollbars}.rs` render to a CPU buffer (`anyrender_vello_cpu`); `tests/blitz-tests/tests/select_popup_painted.rs`, 6 tests in both layout modes, holds an open list's painted frames — the highlight and a disabled row against their constants, the plain pair, opacity and clips, a closed select's frame byte-equal, a group's header darker than a row by weight and a thumb that takes no hit»
- leaf: .claude/docs/services/blitz-shell.md:18   class: stale
  leaf-text: «the stand binary builds without `accessibility`»
  source: .andromeda/architecture-detail/standard-contracts.md:33 «the stand's windowed binary builds blitz-shell without `accessibility`»
  replace-with: «the stand's windowed binary builds blitz-shell without `accessibility`»
- leaf: .claude/docs/gotchas.md:31   class: stale
  leaf-text: «and bounds-only movement (a re-wrap, a scroll) changes a reading and marks nothing.»
  source: .andromeda/architecture-detail/cross-cutting-patterns.md:41 «the fifth kind of site is a select's list: its opening or closing marks the select, a page scroll or a move of the select's box that closes the list marks it through `reconcile_select_list`»
  replace-with: «and bounds-only movement (a re-wrap, a scroll with no select list standing) changes a reading and marks nothing, while a select list's opening or closing, a page scroll or a move of the select's box that closes it included, marks its select, and a highlight or a wheel move inside the list marks nothing.»
- leaf: .claude/docs/conventions.md:7   class: omission
  leaf-text: «- Modules and files open with a `//!` doc; examples»
  source: .andromeda/architecture-detail/conventions.md:21 «public items carry `///`»
  replace-with: «- Modules and files open with a `//!` doc and public items carry `///`; examples»
- leaf: .claude/docs/conventions.md:19   class: omission
  leaf-text: «and wpt set `publish = false`;»
  source: .andromeda/architecture.md:318 «app and example crates, blitz-tests, blitz-test-harness, escher-telemetry, escher-driver, blitz-vibey-script and wpt set `publish = false`, rdme `publish = true`»
  replace-with: «and wpt set `publish = false`, rdme `publish = true`;»
- leaf: .claude/docs/conventions.md:48   class: omission
  leaf-text: «other allows are local, ideally with a reason.»
  source: .andromeda/architecture.md:104 «rustdoc warnings are errors workflow-wide, over every library crate; `collapsible_if` allowed crate-wide in four crates and `module_inception` in blitz-dom's node module, the rest locally»
  replace-with: «`clippy::module_inception` in blitz-dom's node module; other allows are local, ideally with a reason; rustdoc warnings are errors workflow-wide, over every library crate.»
