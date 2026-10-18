# layouts extract

## Relevance
partial: the sync adds and changes no surface, region, component or focus stop. But a Taffy rev move (Taffy main's float placement, `place_floated_box` in `packages/blitz-dom/src/layout/inline.rs`) could move the stand geometry the plan records. Whether the chosen take moves any of it is research's question.

## Constraints
- The headless stand mount must keep its recorded structure after the take: `main#main > #task-shell`, with `#task-header > #back-btn + #task-title` above `#task-body`, at the pinned 800 × 600, scale 1.0, Light viewport with the bundled DejaVu Sans. A held `escher-session` instance (host role and `mcp` role) must read as a fresh boot (per layout-templates desktop-native §Primary screens).
- The stand's author `id`s and attribute-only names, roles and states must still add no element, class, style, wrapper or order. The take changes no stand markup, since adopting new upstream APIs in escher's code is out of scope (per layout-templates desktop-native §Primary screens).
- CRUD's Tab sequence stays the recorded six stops: `back-btn` · `crud-filter` · `crud-list` · `crud-name` · `crud-surname` · `crud-create`. Focus stays on the list, never on a row (per layout-templates desktop-native §Primary screens).
- Home's recorded geometry must be re-read, not assumed. It is `#home`, a scrolling box 600 high that holds six cards, with Cells at 615 to 684 below it, reachable only once scrolled into view. A Taffy move that shifts it is re-measured and recorded (per layout-templates desktop-native §Primary screens).
- The nested-scroller readings the plan records must be re-read, not assumed. `crud-list` reads (24, 112.796875, 369 × 419.203125) at boot and after its content scrolls. `task-body` reads (0, 46, 800 × 554). After twelve Creates, a `click` on the last row is refused `off-screen` until a `scroll` brings it into the list. A moved number is re-measured and recorded, never loosened (per layout-templates desktop-native §IA notes).
- The plan records the document skeleton `<html><head></head><body><main id="main"></main></body></html>`, with the app mounted into `main`, and the harness default of 800 × 600, scale 1, light. Both must hold after the take. Whether upstream's delta touches either is research's question; the scope reads it as two hunks in blitz-dom only (per layout-templates desktop-native §IA notes).

## Patterns to follow
- Record each geometry reading with its source. The plan's readings carry "as measured at {chunk}/evidence/…" or "…/report.md". A re-read after the take cites this chunk's evidence the same way (per layout-templates desktop-native §IA notes).
- State an unmeasured surface explicitly. The plan says "the windowed stand not measured" beside a headless reading; a headless-only re-read after the take says so likewise (per layout-templates desktop-native §Primary screens).
- When markup does change, use attributes alone (`id:`, `aria-*`, `for`, `tabindex`), never a new element, wrapper or class (per layout-templates desktop-native §Primary screens). This applies only if a gate failure forced a stand edit, which the scope does not expect.

## Anti-patterns to avoid
- Loosening a recorded bound, such as `crud-list`'s or `task-body`'s figures or Home's card positions, so a stand check passes after a Taffy move. Re-measure and record instead (per layout-templates desktop-native §IA notes).
- Adding any element, wrapper, class, style or order change to the stand's TaskShell or task markup as part of the sync (per layout-templates desktop-native §Primary screens).

## Contract bindings
- Focus order ↔ a11y SC 2.4.3. The CRUD six-stop Tab sequence (layout-templates desktop-native §Primary screens) is the sequence `stand_keyboard` asserts through `common::tab_order` / `common::ENABLED_TAB_ORDER`. If one moves, both move together.
- Geometry ↔ tests harness. The `crud-list` / `task-body` readings and the `off-screen` refusal after twelve Creates (layout-templates desktop-native §IA notes) are what `tests/blitz-tests/tests/stand_act_scroll.rs` and the session refusal checks rest on. Stand bounds also feed snapshot `bounds`, the snapshot text's byte size and the screenshot's pixels. A Taffy move is re-measured across all of them.
- The cursor change in `fb8d6c15e` (`get_cursor` → `Default` over no box) belongs to the shell and pointer, not to layout. Layout-templates records no cursor reading.

## Acceptance criteria contributions
- (layouts) After the take, in both layout modes, the headless CRUD stand reads `crud-list` at (24, 112.796875, 369 × 419.203125) at boot and `task-body` at (0, 46, 800 × 554). Any moved figure is recorded in the chunk's evidence with its cause (the Taffy rev), and no check is loosened (per layout-templates desktop-native §IA notes).
- (layouts) After the take, the CRUD Tab sequence reads the six recorded stops in order: `back-btn` · `crud-filter` · `crud-list` · `crud-name` · `crud-surname` · `crud-create` (per layout-templates desktop-native §Primary screens).
- (layouts) After the take, the stand mounts `main#main > #task-shell`, with `#task-header > #back-btn + #task-title` above `#task-body`, at 800 × 600, scale 1.0, Light. `examples/seven_guis/src/` carries no markup diff from the sync (per layout-templates desktop-native §Primary screens).
- (layouts) After the take, the headless stand reads Home's Cells card at 615 to 684, below `#home`'s 600-high scrolling box. A moved reading is recorded and the windowed stand stated as not measured (per layout-templates desktop-native §Primary screens).
