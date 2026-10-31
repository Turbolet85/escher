# Apply — architecture · part 1 of 4

## Verdicts
- fanout-architecture.md 1 · applied · .andromeda/architecture.md:62#1
- fanout-architecture--cascade-g2.md 18 · folded into fanout-architecture.md 1
- fanout-architecture.md 2 · applied · .andromeda/architecture.md:135#1
- fanout-architecture--cascade-g4.md 26 · folded into fanout-architecture.md 2
- fanout-architecture.md 3 · applied · .andromeda/architecture.md:135+#1
- fanout-architecture.md 4 · applied · .andromeda/architecture.md:151#1
- fanout-architecture.md 5 · applied · .andromeda/architecture.md:183#1, .andromeda/architecture.md:183#2
- fanout-architecture--cascade-g6.md 7 · folded into fanout-architecture.md 5
- fanout-architecture.md 6 · applied · .andromeda/architecture.md:157#1
- fanout-architecture--cascade-g4.md 29 · folded into fanout-architecture.md 6
- fanout-architecture.md 7 · applied · .andromeda/architecture.md:145#1
- fanout-architecture--cascade-g4.md 28 · folded into fanout-architecture.md 7
- fanout-architecture.md 8 · applied · .andromeda/architecture.md:137#1
- fanout-architecture.md 9 · applied · .andromeda/architecture.md:141#1
- fanout-architecture.md 10 · applied · .andromeda/architecture.md:56#1
- fanout-architecture--cascade-g2.md 19 · folded into fanout-architecture.md 10
- fanout-architecture.md 11 · applied · .andromeda/architecture.md:143#1, .andromeda/architecture.md:143#2
- fanout-architecture--cascade-g4.md 27 · folded into fanout-architecture.md 11
- fanout-architecture.md 12 · applied · .andromeda/architecture.md:60+#1
- fanout-architecture.md 13 · applied · .andromeda/architecture.md:275#1
- fanout-architecture--cascade-g7.md 8 · folded into fanout-architecture.md 13
- fanout-architecture.md 14 · applied · .andromeda/architecture.md:250#1
- fanout-architecture.md 15 · refused · another part's file, main writes it
- fanout-architecture.md 16 · refused · another part's file, main writes it
- fanout-architecture--cascade-g1.md 1 · applied · .andromeda/architecture-detail/design-philosophy.md:51#1
- cascade-compare-architecture.md 2 · applied · .andromeda/architecture-detail/design-philosophy.md:13#1

## Edits
- line: .andromeda/architecture.md:62
  old: «no popup is painted, Enter and Space open nothing on a single select, an option of a closed single select, under a group too, takes no click and stays refused `covered`, `size` is unread and Ctrl+A is not modelled»
  new: «a click, Enter and a Space with no typed text standing open a single select's list, which the engine paints over the page with a group's `label` as a header, and a bound label's click focuses the select and opens nothing; an option of a closed single select, under a group too, takes no click and stays refused `covered`; a `multiple` select's `size` hints its height, a single select's stays unread, and Ctrl+A is not modelled»

- line: .andromeda/architecture.md:135
  old: «its one non-test caller the driver's `focus_multiple_select`»
  new: «its one non-test caller the driver's `focus_select`»

- after: .andromeda/architecture.md:135
  anchor: «- The document's clock: `DocumentConfig.clock`, the config's clock or else a fresh `WallClock`; the crate root re-exports `Clock`, `WallClock` and `VirtualClock`, every clone of a `VirtualClock` reading one time; no `Instant::now()` or `SystemTime::now()` stands in blitz-dom outside `clock.rs`; `BaseDocument::select_value` reads a select's value, a `multiple` select's first chosen option in tree order, `select_values` its chosen set and `select_of` an option's or group's select, each public and read-only; public `clear_select_typeahead` clears the typed text, its one non-test caller the driver's `focus_multiple_select`; the choice's writers are crate-private.»
  add: «»
  add: «- An open select's list on `BaseDocument`: `open_select`, `select_list_bounds`, `select_list_view` and `select_row_layout` are public and read-only; public `close_select_list` closes an open list, its callers keyboard, pointer, reconcile and the driver's `focus_select`; the rest of the list's machinery is crate-private or private.»

- line: .andromeda/architecture.md:151
  old: «with no public type.»
  new: «with no public type; an open list's state lives in `node::select_list`, crate-private but for two public read types re-exported from the crate root, `SelectListView`, the list's colours its associated constants, and `SelectListRow`.»

- line: .andromeda/architecture.md:183
  old: «the public `closed_select_of` and `is_multiple_select` are the select tests the driver reads, naming the element and never the role;»
  new: «the public `closed_select_of`, which answers `None` for a select whose list is open, and `is_select`, read by the driver's `locate` alone, are the select tests the driver reads, naming the element and never the role, and `is_multiple_select` stays re-exported, read within the workspace by `closed_select_of` alone;»

- line: .andromeda/architecture.md:183
  old: «a `multiple` select's rows and groups reading client rects of their own.»
  new: «a `multiple` select's rows and groups reading client rects of their own and an open list's rows and groups reading `select_list_bounds` before the client rect.»

- line: .andromeda/architecture.md:157
  old: «`paint_scene` paints a document into a `PaintScene`;»
  new: «`paint_scene` paints a document into a `PaintScene`, an open select's list last through `paint_select_list`, after the root's render and before the devtools overlays, read from `SelectListView` and past any clip or `opacity` on its select's ancestors, reaching a shell or renderer through `paint_scene` alone;»

- line: .andromeda/architecture.md:145
  old: «a mapped box with a non-finite edge answering `None`;»
  new: «a mapped box with a non-finite edge answering `None`, and a row or a group of an open select list reading the list's scrollport in place of its clipping boxes;»

- line: .andromeda/architecture.md:137
  old: «and writing state attributes each read once and a native control's checked state on a checkable role;»
  new: «and writing state attributes each read once, a native control's checked state on a checkable role and a single select's native `expanded`, true while its list stands open, with no `aria-expanded` read on one, every other element keeping that read;»

- line: .andromeda/architecture.md:141
  old: «which clears focus and dispatches none of them;»
  new: «which clears focus and dispatches none of them, and for the driver's `type` on any select, which focuses with no click and dispatches none of them either;»

- line: .andromeda/architecture.md:56
  old: «and a hit stops at the padding box of every box paint clips.»
  new: «and a hit stops at the padding box of every box paint clips; where a select list stands open a hit answers the list first, with no scrollbar, so a row lies past any box that clips its select, a wheel over the list scrolls the list, and `scroll_into_view` on a row writes the list's `scroll_y` at once and moves no chain or viewport.»

- line: .andromeda/architecture.md:143
  old: «nested ones instantly with no `scroll` event, then the viewport;»
  new: «nested ones instantly with no `scroll` event, then the viewport, but for a row of an open select list, which writes the list's `scroll_y` at once and moves no chain or viewport;»

- line: .andromeda/architecture.md:143
  old: «and answers nothing for the whole subtree of a transform whose determinant is 0 or not finite;»
  new: «and answers nothing for the whole subtree of a transform whose determinant is 0 or not finite, while `hit_with_scrollbar` answers an open select list first and no scrollbar with it, `hit_inner` holding none of it, so `hit`, `set_hover_to` and `handle_pointerdown` all read the list;»

- after: .andromeda/architecture.md:60
  anchor: «- Beside those, seven more direct engine changes for every document, the document's clock — every engine time read goes through `DocumentConfig.clock`, wall time when it is unset — and the select model of keys, typeahead and click, and eight more — `visible_region` through transforms, a non-invertible transform's no hit, the focus fixup under `inert`, the tree's focus falling back to the window, native `toggled`, the `multiple` select, `optgroup` and `change` on a checkbox and a radio — each flagged upstreamable with no upstream pull request open; the merge surface of those eight is a line owned by Epoch 3's sync, `hit_inner` a textual conflict with upstream `0980cce5f`.»
  add: «»
  add: «- The select list is one more direct engine change for every document, remote pages included, ratified by the founder (2026-10-31) in his forks F1 to F10; its merge surface is the sixteen files of `escher-0.3.0/chunks/2026-10-31-select-popup/evidence/merge-surface.md`, a line owned by Epoch 3's Upstream sync ahead of the UI graph.»

- line: .andromeda/architecture.md:275
  old: «the changed set is written at four kinds of site and by no node creation, hover, scroll, resize, load, animation or layout;»
  new: «the changed set is written at five kinds of site, the fifth a select list's open and close marking its select, and, with no list open, by no node creation, hover, scroll, resize, load, animation or layout; where a list stands, a page scroll or a move that closes it marks its select through `reconcile_select_list` at the next resolve and a wheel elsewhere through `close_select_list`, while a highlight or a wheel move inside the list marks nothing;»

- line: .andromeda/architecture.md:250
  old: «clear damage, hover refresh, sub-documents»
  new: «clear damage, `reconcile_select_list`, hover refresh, sub-documents»

- line: .andromeda/architecture-detail/design-philosophy.md:51
  old: «differing only in the `ScrollRequest` (packages/blitz-dom/src/scrolling.rs:125-129)»
  new: «differing only in the `ScrollRequest` (packages/blitz-dom/src/scrolling.rs:125-129), an open select list's rows standing apart from it: a wheel over the list and `scroll_into_view` on a row move the list's own `scroll_y`, the latter writing it at once and returning with no chain or viewport moving (packages/blitz-dom/src/document.rs:930-960; packages/blitz-dom/src/scrolling.rs:669-672)»

- line: .andromeda/architecture-detail/design-philosophy.md:13
  old: «painting assumes styles and layout are already resolved (packages/blitz-paint/src/lib.rs:37-38; packages/blitz-paint/src/render.rs:171-175)»
  new: «painting assumes styles and layout are already resolved (packages/blitz-paint/src/lib.rs:37-38; packages/blitz-paint/src/render.rs:171-175); an open select's list is painted by `paint_select_list` after the root's render inside `paint_scene`, from the public read `select_list_view`, reaching a shell or renderer through `paint_scene` alone, and its rows' text is the carve-out, laid out at paint through the public `select_row_layout`, only the rows in the scrollport (packages/blitz-paint/src/render.rs:246-247; packages/blitz-paint/src/render/select_list.rs:85-128; packages/blitz-dom/src/document.rs:1022-1052; packages/blitz-dom/src/document.rs:1054-1064)»

## Patterns
[[pattern]]
id = "arch1-01"
fixed = 'no popup is painted'

[[pattern]]
id = "arch1-02"
fixed = 'Enter and Space open nothing on a single select'

[[pattern]]
id = "arch1-03"
fixed = '`size` is unread'

[[pattern]]
id = "arch1-04"
regex = 'non-test caller the driver.s `focus_multiple_select`'

[[pattern]]
id = "arch1-05"
fixed = '`closed_select_of` and `is_multiple_select` are the select tests'

[[pattern]]
id = "arch1-06"
fixed = 'written at four kinds of site'

[[pattern]]
id = "arch1-07"
fixed = 'by no node creation, hover, scroll, resize, load, animation or layout'

[[pattern]]
id = "arch1-08"
fixed = 'a hit stops at the padding box'

[[pattern]]
id = "arch1-09"
fixed = 'nested ones instantly with no `scroll` event, then the viewport'

[[pattern]]
id = "arch1-10"
fixed = 'differing only in the `ScrollRequest`'

[[pattern]]
id = "arch1-11"
fixed = 'painting assumes styles and layout are already resolved'
