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
- Paint mirrors named engines (Chrome contrast ratio and dash ratios, WebKit/Blink darken/lighten, Firefox per-box decoration).
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
- **Headless screenshots:** `examples/screenshot.rs` renders through `VelloCpuImageRenderer`

## References
- `.andromeda/architecture.md` · `.claude/docs/services/blitz-dom.md`
