# design · Spacing — category read

## Carried
- (.andromeda/design-system-detail/spacing.md:19) The stand's card and home spacing feed the exact bounds the stand checks assert. A take must leave these values as written. Upstream's one stand edit, `cells.rs`, is a key-match change. Any bound that does move is re-measured and recorded, never loosened to pass.
  > - seven_guis cards pad 24px 32px with 16px gap; home pads 48px 32px 64px (examples/seven_guis/src/tasks/counter.rs:38-42; examples/seven_guis/src/app.rs:191)
- (.andromeda/design-system-detail/spacing.md:20) The user-agent body margin sits under every stand screen's bounds, and `assets/default.css` is a CARRY 5 merge-surface file. The value must stand on the take's tree.
  > - blitz-dom default body margin 8px (packages/blitz-dom/assets/default.css:274-277)
- (.andromeda/design-system-detail/spacing.md:26) Upstream's Stylo move edits `stylo.rs`. The presentational-hint margin mappings must survive the take unchanged. Where upstream's hunks land above them, their cited line ranges are re-pointed at the wrap.
  > - `hspace` and `vspace` map to horizontal and vertical margins on `embed`, `img`, `object`, `marquee` and image inputs (packages/blitz-dom/src/stylo.rs:1075-1101)
  > - Body `marginwidth`, `marginheight`, `leftmargin` and `topmargin` map to pixel margins; `rightmargin` and `bottommargin` are deliberately ignored (packages/blitz-dom/src/stylo.rs:1145-1182)
- (.andromeda/design-system-detail/spacing.md:28) Upstream's delta edits `node/scrollbar.rs` (a style-getter rename). The overlay scrollbar geometry must stay at these values, and its cite is re-pointed where the take shifts its lines.
  > - Overlay scrollbar thumb geometry: thickness 10.0, thin thickness 6.0, margin 2.0 and minimum length 32.0 CSS px (packages/blitz-dom/src/node/scrollbar.rs:122-126)
- (.andromeda/design-system-detail/spacing.md:29) Upstream's `46d7cf993` and `f102663b8` edit `node/node.rs`. The single-line input's vertical centring must hold on the take's tree, and its cite is re-pointed where the take shifts its lines.
  > - Single-line text inputs are vertically centered within their content box (packages/blitz-dom/src/node/node.rs:855-873)
- (.andromeda/design-system-detail/spacing.md:30) Upstream's delta edits blitz-paint `render.rs` and `text.rs` (`TextDecorationStyle::MozNone` → `None` among them). The marker offset and the double-decoration gap must stay as recorded, and their cites are re-pointed where the take shifts their lines.
  > - Outside list markers using a character are padded 8 CSS px from the item's border box (packages/blitz-paint/src/render.rs:1057-1068)
  > - The double text decoration places its second line thickness + 1 CSS px away (packages/blitz-paint/src/text.rs:323-333)

## Bindings
- **Stand spacing ↔ test-plan (stand checks in both layout modes)**: the stand's padding, gaps and body margin set the bounds that `stand_snapshot`, the lint and `stand_contrast` read. A take that moves Taffy or Stylo may move a number. That number is re-measured and recorded, never loosened. (.andromeda/design-system-detail/spacing.md:19) (.andromeda/design-system-detail/spacing.md:20)
- **Engine spacing cites ↔ the wrap's citation re-point**: the take's hunks in `stylo.rs`, `node/node.rs`, `node/scrollbar.rs`, `render.rs` and `text.rs` sit above the cited ranges. The values must stand while their `(file:line)` cites move. (.andromeda/design-system-detail/spacing.md:26) (.andromeda/design-system-detail/spacing.md:28) (.andromeda/design-system-detail/spacing.md:29) (.andromeda/design-system-detail/spacing.md:30)
- **Spacing ↔ Depth Strategy (hit testing in `node/node.rs`)**: `f102663b8` edits the hit test in the same file as the input-centring value. The padding-box hit stop is the Depth body's rule, re-read beside it. (.andromeda/design-system.md:87)
- **Spacing scale ↔ body**: the chunk adds no spacing scale or base unit, so the body's measured gap stands as written. (.andromeda/design-system.md:72)
