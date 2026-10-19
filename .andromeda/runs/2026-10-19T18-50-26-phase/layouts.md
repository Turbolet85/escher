# layouts extract

## Relevance
partial: the sync creates and moves no surface, region or component. Upstream's `38ca4e777` (`:focus-within`) moves engine focus code, and a Taffy take, if P4 chooses one, can move layout numbers. The stand's measured layout, focus order and geometry are what the take must leave standing.

## Constraints
- The headless stand mounts one lean task in TaskShell as `main#main > #task-shell`, with `#task-header > #back-btn + #task-title` above `#task-body`, at the pinned 800 × 600, scale 1.0, Light viewport with the bundled DejaVu Sans. The take must leave this mount and viewport as they are (per layout-templates §Surface: desktop-native → Primary screens).
- The author `id` on each lean-task control is that control's stable element id. Ids add no class, style, wrapper or order, and no task CSS selects by them. The take adds no stand markup, id, class, style, wrapper or order. Because out-of-scope keeps the snapshot from reading `:focus-within`, no `:focus-within` rule enters the stand's CSS either (per layout-templates §Surface: desktop-native → Primary screens).
- CRUD's Tab sequence has six stops: `back-btn` · `crud-filter` · `crud-list` · `crud-name` · `crud-surname` · `crud-create`. The tree's focus is the list, never a row. `38ca4e777` moves focus code (`set_focus_to`, `clear_focus`, the Tab arm of `handle_key_or_input_event`), so this sequence must hold after the take. Whether the new rule that a node outside the document is not focused moves it is research's question (per layout-templates §Surface: desktop-native → Primary screens).
- Two nested-scroller geometries were measured at boot. `crud-list` reads (24, 112.796875, 369 × 419.203125) and is the box that clips its rows. `task-body` reads (0, 46, 800 × 554). Neither may move unless the take moves Taffy. If Taffy moves, a moved figure is re-measured and recorded, never loosened (per layout-templates §Surface: desktop-native → IA notes).
- Home's `#home` is a scrolling box 600 high that holds six of the seven cards. The Cells card lies at 615 to 684, below the box. That is the measured reading this geometry must keep, or re-measure if Taffy moves (per layout-templates §Surface: desktop-native → Primary screens).
- Every document starts from the fixed skeleton `<html><head></head><body><main id="main"></main></body></html>`, and the app mounts into `main`. Whether upstream's delta touches `dioxus_document.rs` is research's question (per layout-templates §Surface: desktop-native → IA notes).
- An `escher-session` held instance, in the host role or the `mcp` role, reads as a fresh boot. The TaskShell mount and the pinned viewport hold after commands, in both layout modes. The take must not move this reading (per layout-templates §Surface: cli → Primary screens and §Surface: desktop-native → Primary screens).

## Patterns to follow
- Record any change to stand markup as attributes alone, stating what it adds no element, class, style, wrapper, id or order to. A conflict resolution or gate fix that must touch the stand follows the same form (per layout-templates §Surface: desktop-native → Primary screens).
- Give each geometry reading its provenance, "as measured at {chunk}/evidence/…" or "{chunk}/report.md". A re-measured figure after a Taffy move cites this chunk's evidence path (per layout-templates §Surface: desktop-native → IA notes).
- Read layout and focus in both layout modes, as the held-instance and nested-scroller readings were read (per layout-templates §Surface: desktop-native → Primary screens).

## Anti-patterns to avoid
- Do not add `:focus-within`, or any selector keyed on the new focus state, to stand CSS. Do not add CSS that selects by a stable id (per layout-templates §Surface: desktop-native → Primary screens).
- Do not loosen a stand bound, such as `crud-list`'s or `task-body`'s figures, to pass after a Taffy move. Re-measure it and record it (per layout-templates §Surface: desktop-native → IA notes).

## Contract bindings
- The focus order binds to the a11y focus-order SC 2.4.3 and `stand_keyboard`'s tables (`common::tab_order`, `common::ENABLED_TAB_ORDER`). The CRUD six-stop sequence in layout-templates §Surface: desktop-native → Primary screens is the same order those tables assert.
- The nested-scroller geometry binds to the tests harness's stand checks `stand_act_scroll` and the scrolled-box checks (`scrolled_box_client_rect`, `hit_clipped_at_scrolling_box`). The figures in layout-templates §Surface: desktop-native → IA notes are the bounds those checks read.
- The held instance reading as a fresh boot binds to the tests harness's `stand_session_fresh` (layout-templates §Surface: cli → Primary screens).

## Acceptance criteria contributions
- (layouts) After the take, CRUD's Tab sequence reads `back-btn` · `crud-filter` · `crud-list` · `crud-name` · `crud-surname` · `crud-create` in both layout modes, with focus on the list and never on a row (per layout-templates §Surface: desktop-native → Primary screens).
- (layouts) At boot after the take, `crud-list` reads (24, 112.796875, 369 × 419.203125) and `task-body` reads (0, 46, 800 × 554). If the take moves Taffy and a figure moves, the new figure is recorded in the chunk's evidence with its cause (per layout-templates §Surface: desktop-native → IA notes).
- (layouts) The take's diff holds no change to stand markup, id, class, style, wrapper or order under `examples/seven_guis/src/`, and adds no `:focus-within` rule to stand CSS (per layout-templates §Surface: desktop-native → Primary screens).
- (layouts) An `escher-session` held instance still reads as a fresh boot, with the TaskShell mount and the pinned 800 × 600 viewport holding after commands in both layout modes (per layout-templates §Surface: cli → Primary screens).
