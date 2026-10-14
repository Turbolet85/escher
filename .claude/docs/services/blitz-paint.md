# blitz-paint

_Crate notes. Primary source: `.andromeda/architecture.md` (§Design Philosophy, §Standard Contracts Paint and shell, [Painting])._

## Responsibility
Pushes anyrender drawing commands for an already-resolved `BaseDocument` into an `impl PaintScene`; the scene decides whether to rasterize, emit SVG/PDF or serialize. It assumes styles and layout are resolved and never mutates the document's layout.

## Key integrations

### Consumes from
- blitz-dom (paint tree, `final_layout()`, computed styles), anyrender, kurbo, peniko, color.

### Publishes to
- `paint_scene(PaintScene, &mut BaseDocument, scale, width, height, x_offset, y_offset)`.

## Internal conventions
- One module per concern under `src/render/` (background, border, box_shadow, clip_path, form_controls, mask, …); `kurbo_css` builds rounded border geometry.
- Paint mirrors named engines (Chrome contrast ratio and dash ratios, WebKit/Blink darken/lighten, Firefox per-box decoration). Since 2026-10-12-headless-screenshot a password `input` paints its value masked, in every document and through every renderer that paints with blitz-paint, windowed included: `draw_text_input_text` branches to the private `draw_masked_input_text`, a bullet per character laid out by blitz-dom's `BaseDocument::layout_as_text_input`, so two values of one length paint byte-identical frames, the caret at the bullet of its index in the input's caret colour and the field scrolled on the bullets — a direct engine change for every document, ratified by the founder (2026-10-12); a click still places the caret by the real characters' widths (blitz-dom's pointer path, unchanged), so which bullet it lands on depends on them — measured since 2026-10-14-stand-requirement-sweep by `password_caret_click` (both layout modes, the bundled font at 20 px): a click 30 px into the content box places the caret at index 5 after eight narrow characters and at 2 after eight wide ones, a known 0.1.0 limit whose fix, placing a click's caret by the bullet layout, is owned by a `.andromeda/residuals.md` line (target: next).
- Default features: svg; scrollbars off "while the feature matures".

## Crate-specific gotchas
- `LAYER_LIMIT = 1024`; non-finite transformed boxes are culled; gradients needing >500 tiles are skipped (FIXME).
- Painting an inline root without inline layout data panics.
- Custom widgets are pre-painted into Scenes keyed by document and node id.

## Entry points for modification
- `src/lib.rs` (`paint_scene`), `src/render.rs`, `src/render/*.rs`, `src/color.rs` (contrast helper), `src/text.rs`, `src/layers.rs`

## Testing this crate
- **Unit:** `cargo test -p blitz-paint` (gradient.rs, css_box.rs)
- **Pixel tests:** `tests/blitz-tests/tests/{paint_order,background_size,outset_box_shadow_shape,scrollbars}.rs` render to a CPU buffer (`anyrender_vello_cpu`)
- **Headless screenshots:** `examples/screenshot.rs` renders through `VelloCpuImageRenderer`, and since 2026-10-12-headless-screenshot so does blitz-test-harness's `Harness::screenshot` / `screenshot_rect` (`anyrender_vello_cpu` with no `multithreading`), which composites the frame over opaque white after paint where the example fills before `paint_scene`

## References
- `.andromeda/architecture.md` · `.claude/docs/services/blitz-dom.md`
