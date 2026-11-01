# design · Spacing — category read

## Carried
- (.andromeda/design-system-detail/spacing.md:20) Any take that touches blitz-dom's `assets/default.css`, a predicted conflict file, must leave the engine's default body margin at this value unless the taken upstream rule moves it, and a moved value is re-measured and recorded.
  > blitz-dom default body margin 8px (packages/blitz-dom/assets/default.css:313-316)
- (.andromeda/design-system-detail/spacing.md:21) A `default.css` resolution must keep the `multiple` select's row padding, part of the select model that stays unchanged through the take.
  > blitz-dom default `multiple` select rows (`select[multiple] > option`, `select[multiple] > optgroup > option`) padding 0 2px (packages/blitz-dom/assets/default.css:173-177)
- (.andromeda/design-system-detail/spacing.md:22) The `size` hint is named among our surfaces in the predicted `stylo.rs` conflict, so a resolution of `stylo.rs` and `default.css` must keep its height rule, its cap and its no-hint cases.
  > blitz-dom `multiple` select with `size`: `size_rows` hints a `height` of `size × 1.2em`, `size` capped at 65534, `0` or a bad value giving no hint; `select[multiple][size]` takes `overflow-y: auto`, its rows' line height held in §Typography (packages/blitz-dom/src/node/select_list.rs:125-130; packages/blitz-dom/src/stylo.rs:1124-1139; packages/blitz-dom/assets/default.css:183-193)
- (.andromeda/design-system-detail/spacing.md:28) A `stylo.rs` resolution must keep the `hspace` and `vspace` margin mapping on these elements.
  > `hspace` and `vspace` map to horizontal and vertical margins on `embed`, `img`, `object`, `marquee` and image inputs (packages/blitz-dom/src/stylo.rs:1077-1103)
- (.andromeda/design-system-detail/spacing.md:29) A `stylo.rs` resolution must keep the body margin attributes' mapping, the two deliberately ignored attributes staying ignored.
  > Body `marginwidth`, `marginheight`, `leftmargin` and `topmargin` map to pixel margins; `rightmargin` and `bottommargin` are deliberately ignored (packages/blitz-dom/src/stylo.rs:1163-1200)
- (.andromeda/design-system-detail/spacing.md:31) The select list's merge surface is re-read row by row at this take and its list stays unchanged, so its row cap and engine geometry constants must survive and `select_popup` must still prove them.
  > An open select list shows at most `MAX_ROWS` = 20 rows and scrolls past them; its box and rows use the engine constants `BORDER_WIDTH`, `ROW_PADDING_X`, `ROW_PADDING_Y` and `GROUP_INDENT` (tests/blitz-tests/tests/select_popup.rs:870-911; as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)
- (.andromeda/design-system-detail/spacing.md:32) The open list's placement rule must hold after the take, a layout or scrolling number moved by the take being re-measured and never loosened to pass.
  > `place` sets an open select list at least its select's width and at most the viewport's, inside the viewport, opening where its rows fit and else on the side with more room; every edge stays finite (packages/blitz-dom/src/node/select_list.rs:218-263; packages/blitz-dom/src/node/select_list.rs:399-457)
- (.andromeda/design-system-detail/spacing.md:33) A resolution of `node/node.rs`, a predicted conflict file, and any take of the layout-sizing commit `ae774d7f4`, must keep single-line inputs vertically centred.
  > Single-line text inputs are vertically centered within their content box (packages/blitz-dom/src/node/node.rs:860-878)
- (.andromeda/design-system-detail/spacing.md:35) A resolution of blitz-paint's `text.rs`, a predicted conflict file, must keep the double decoration's offset, including under any take of the inline box decorations commit `f76777766`.
  > The double text decoration places its second line thickness + 1 CSS px away (packages/blitz-paint/src/text.rs:323-333)

## Bindings
- **`size` hint rows ↔ Typography**: the hinted height is this category's and the rows' line height is Typography's, so a `default.css` resolution must hold both together, (.andromeda/design-system-detail/spacing.md:22)
- **Open list geometry ↔ Color Palette**: the list's box and row constants and its colours are all engine constants of the list rather than tokens, so the take must not move either into `default.css`, (.andromeda/design-system-detail/spacing.md:31) and (.andromeda/design-system.md:30)
- **Open list placement ↔ Depth Strategy**: the list sits where `place` puts it and its rows are answered before the hit walk, so the `node/node.rs` hit-stop resolution must keep a row reachable wherever it is placed, (.andromeda/design-system-detail/spacing.md:32) and (.andromeda/design-system.md:87)
- **Open list row cap ↔ Motion**: past the cap the list scrolls, and into view on a row writes that list's own scroll at once, so the `scrolling.rs` resolution must keep both, (.andromeda/design-system-detail/spacing.md:31) and (.andromeda/design-system.md:123)
