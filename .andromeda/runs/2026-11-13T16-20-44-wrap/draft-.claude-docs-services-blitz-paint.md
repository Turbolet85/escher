# blitz-paint

_Crate notes. Primary source: the architecture master `doc:architecture` (§Design Philosophy, §Standard Contracts Paint and shell, [Painting])._

## Responsibility
Pushes anyrender drawing commands for an already-resolved `BaseDocument` into an `impl PaintScene`; the scene decides whether to rasterize, emit SVG/PDF or serialize. It assumes styles and layout are resolved and never mutates the document's layout; an open select's rows' text is the carve-out, laid out at paint through blitz-dom's public `select_row_layout`, only the rows in the scrollport.

## Key integrations

### Consumes from
- blitz-dom (paint tree, `final_layout()`, computed styles), anyrender, kurbo, peniko, color.

### Publishes to
- `paint_scene(PaintScene, &mut BaseDocument, scale, width, height, x_offset, y_offset)`, which calls `paint_select_list` after the root's render and before the devtools overlays, an open select's list read from `SelectListView` and past any clip or `opacity` on its select's ancestors and past the blend layers, reaching a shell or renderer through `paint_scene` alone.

## Internal conventions
- One module per concern under `src/render/` (background, border, box_shadow, clip_path, form_controls, mask, …); `kurbo_css` builds rounded border geometry; `mix-blend-mode` and `isolation` paint in every document: the private module `blend` holds `pub(crate) convert_blend_mode`, `LayerManager::maybe_with_layer` and `maybe_push_layer` take a `BlendMode`, and `render_element` pushes a viewport-wide blend layer and pops it last; `paint_select_list`, in the module `render::select_list`, paints a non-finite box not at all, lays out only rows in the scrollport, and draws its thumb only under the `scrollbars` feature.
- Paint mirrors named engines (Chrome contrast ratio and dash ratios, WebKit/Blink darken/lighten, Firefox per-box decoration). Since 2026-10-12-headless-screenshot a password `input` paints its value masked, in every document and through every renderer that paints with blitz-paint, windowed included: `draw_text_input_text` branches to the private `draw_masked_input_text`, a bullet per character laid out by blitz-dom's `BaseDocument::layout_as_text_input`, so two values of one length paint byte-identical frames, the caret at the bullet of its index in the input's caret colour and the field scrolled on the bullets — a direct engine change for every document, ratified by the founder (2026-10-12); a click inside a field holding a value places the caret by the bullets — blitz-dom's pointer path (`click_password_input`) lays them out through `layout_as_text_input`, scrolled as paint scrolls them, and moves the real editor to the character under the point, so no width of the real characters reaches the caret: one click at one offset into two values of one length lands on one index, and a double click selects the whole value (ratified by the founder, 2026-10-18; `password_caret_click`).
- Default features: svg; scrollbars off "while the feature matures".

## Crate-specific gotchas
- `LAYER_LIMIT = 1024`; non-finite transformed boxes are culled; `render_element`'s border box and culling stand on the scaled size, `location` and `box_position` kept for the transform (an upstream engine change for every document, upstream #1342); gradients needing >500 tiles are skipped (FIXME).
- Painting an inline root without inline layout data panics. `render_element` skips the own shadow, background and border of a box whose `Node::is_inline_with_only_block_children` reads true, through `paints_own_box`, read on a frame by `inline_box_decorations_frame` in both layout modes.
- Custom widgets are pre-painted into Scenes keyed by document and node id.

## Entry points for modification
- `src/lib.rs` (`paint_scene`), `src/render.rs`, `src/render/*.rs`, `src/color.rs` (contrast helper), `src/text.rs` (`stroke_text` delegates to the `pub(crate)` `stroke_text_as`, which takes `RunPaint { color, synthesize_bold }`, every other caller passing `RunPaint::default()`; a synthesized bold, a list group's header, adds one embolden weight to a build's own), `src/layers.rs`, `src/blend.rs` (`convert_blend_mode`)

## Testing this crate
- **Unit:** `cargo test -p blitz-paint` (gradient.rs, css_box.rs, layers.rs); the blend's frame witness is `blend_mode_frame` (a multiply blend, a normal blend and an isolated group read on a frame in both layout modes), beside the 3 unit tests of `layers.rs`
- **Pixel tests:** `tests/blitz-tests/tests/{paint_order,background_size,outset_box_shadow_shape,scrollbars}.rs` render to a CPU buffer (`anyrender_vello_cpu`); `tests/blitz-tests/tests/select_popup_painted.rs`, 6 tests in both layout modes, holds an open list's painted frames — the highlight and a disabled row against their constants, the plain pair, opacity and clips, a closed select's frame byte-equal, a group's header darker than a row by weight and a thumb that takes no hit
- **Headless screenshots:** `examples/screenshot.rs` renders through `VelloCpuImageRenderer`, and since 2026-10-12-headless-screenshot so does blitz-test-harness's `Harness::screenshot` / `screenshot_rect` (`anyrender_vello_cpu` with no `multithreading`), which composites the frame over opaque white after paint where the example fills before `paint_scene`

## References
- `doc:architecture` (the records store) · `.claude/docs/services/blitz-dom.md`
