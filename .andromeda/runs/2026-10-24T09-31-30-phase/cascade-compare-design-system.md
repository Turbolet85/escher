# Cascade compare — design-system · phase · 2026-10-24-upstream-sync-polish-and-ship
E items 65 · held 19 · lost at classify 4 · lost at strong 23 · cascade only 0 · body 18 · unplaced 0

## Lost — the full read carried these and the cascade did not; the planner takes each, marked «(sampler)»

### color-palette#5.1 · lost at strong · .andromeda/design-system-detail/color-palette.md:71
- (.andromeda/design-system-detail/color-palette.md:71) The flight booker's invalid and success colours are colour states `stand_contrast` reaches. After the take they must still read as declared.
  > | seven_guis invalid | `#e53e3e` / `#fff5f5` / `#c53030` | Invalid state (examples/seven_guis/src/tasks/flight_booker.rs:197-201; examples/seven_guis/src/tasks/flight_booker.rs:232-240) |
  > | seven_guis success | `#ebf8ee` / `#68d391` / `#276749` | Success message (examples/seven_guis/src/tasks/flight_booker.rs:197-201; examples/seven_guis/src/tasks/flight_booker.rs:232-240) |

### color-palette#1.3 · lost at classify · .andromeda/design-system-detail/color-palette.md:10
- (.andromeda/design-system-detail/color-palette.md:10) Upstream's delta reaches `document.rs` (`06ecec886`, `3954ecb85`). A resolved conflict there must keep the full recascade on a colour-scheme change.
  > - Color-scheme changes trigger a full recascade because `light-dark()` and system colors resolve at cascade time (packages/blitz-dom/src/document.rs:2312-2319)

### color-palette#2.4 · lost at strong · .andromeda/design-system-detail/color-palette.md:28
- (.andromeda/design-system-detail/color-palette.md:28) `assets/default.css` is in CARRY 5's select-by-id merge surface. A take that reaches it must keep these default colours and the `a[href]` selector.
  > | blitz-dom default link | `rgb(0, 0, 238)` | Default stylesheet link, its rule selecting `a[href]` — an anchor with no `href` takes neither the colour nor the underline; input focus outline `#4D90FE`; button background `#EFEFEF` (packages/blitz-dom/assets/default.css:43-46; packages/blitz-dom/assets/default.css:98-101; packages/blitz-dom/assets/default.css:111) |

### color-palette#1.7 · lost at strong · .andromeda/design-system-detail/color-palette.md:14
- (.andromeda/design-system-detail/color-palette.md:14) A screenshot's pixels depend on the canvas background rule. A Taffy or Stylo move can change them, so any moved shot value is re-measured and recorded, and the rule itself stays.
  > - The canvas background is the root element's background colour, falling back to body's when the root is transparent (packages/blitz-paint/src/render.rs:193-228)

### depth-strategy#2.1 · lost at strong · .andromeda/design-system-detail/depth-strategy.md:19
- (.andromeda/design-system-detail/depth-strategy.md:19) Stacking contexts are built after layout and transforms, in `resolve.rs`, and their types are public re-exports. The take's `node/node.rs` and `resolve` paths (`46d7cf993`, `1c5265a8b`) must keep the stacking-context roots, the ones paint and the hit walk read, as stated, and the stated rule must be re-read on the take's tree.
  > - Paint children and stacking contexts are built after layout and transforms; `StackingContext` and `HoistedPaintChild` are public re-exports (packages/blitz-dom/src/resolve.rs:121-125; packages/blitz-dom/src/lib.rs:89)
  > - A node is a stacking-context root for opacity not equal to 1, fixed or sticky position, z-index on relative or absolute (or static flex/grid items), any transform/rotate/scale/translate, atomic paint effects, or `isolation: isolate` (packages/blitz-dom/src/node/node.rs:1323-1366); atomic paint effects are opacity, filter, clip-path and mask-image (packages/blitz-dom/src/node/node.rs:1368-1397)

### depth-strategy#4.1 · lost at strong · .andromeda/design-system-detail/depth-strategy.md:32
- (.andromeda/design-system-detail/depth-strategy.md:32) Paint order for positioned `z-index: auto` descendants must still hold after the take. A Stylo or Taffy move may change style and layout, and `paint_order` must still pass.
  > - Positioned descendants with `z-index: auto` share one paint level per CSS 2.1 Appendix E and paint in tree order (tests/blitz-tests/tests/paint_order.rs:1-2)

### depth-strategy#4.2 · lost at strong · .andromeda/design-system-detail/depth-strategy.md:33
- (.andromeda/design-system-detail/depth-strategy.md:33) The outset shadow's shape check asserts expected pixels from Chromium. If a take moves Stylo or Taffy and a pixel reading moves, it must be re-measured and recorded, never loosened to pass.
  > - Outset box shadows are clipped when opacity is below 1 or the background is not opaque, and blurred shadows use the averaged border radius per a TODO (packages/blitz-paint/src/render/box_shadow.rs:15-24; packages/blitz-paint/src/render/box_shadow.rs:77-82); an outset shadow takes the element's shape corner for corner, so one with no blur and no spread hides behind the element, with expected pixels from Chromium (tests/blitz-tests/tests/outset_box_shadow_shape.rs:1-6)

### iconography#1.1 · lost at strong · .andromeda/design-system-detail/iconography.md:8
- (.andromeda/design-system-detail/iconography.md:8) A take carrying Stylo v0.23-pre (`06ecec886`) rewrites the cursor read in `packages/blitz-dom/src/document.rs` (upstream's diff moves `style.clone_cursor().keyword` to `style.get_cursor().keyword`); after the take the cursor resolution must still follow this ratified order, the arrow kept over no box, whatever shape the pin fork takes.
  > - The cursor is taken from the CSS `cursor` keyword, else Text for text inputs, Pointer inside links, Text over selectable text, Default otherwise — a pointer over no box included, so the shell keeps a visible arrow there — upstream's #1220, taken on F1 answered B by the founder's delegate, ratified by the founder (2026-10-18) (packages/blitz-dom/src/document.rs:2390-2450; packages/blitz-dom/src/document.rs:3279-3287)

### iconography#1.2 · lost at strong · .andromeda/design-system-detail/iconography.md:9
- (.andromeda/design-system-detail/iconography.md:9) A Stylo move changes the `CursorKind` keyword set the engine reads, so after the take every CSS `cursor` keyword must still map one-to-one to a `CursorIcon`, `cursor: none` to no cursor, through `ShellProvider::set_cursor`'s optional `CursorIcon`; a keyword the new Stylo adds or renames is mapped, never dropped.
  > - CSS `cursor` keywords map one-to-one to `CursorIcon` values, and `cursor: none` maps to no cursor (packages/blitz-dom/src/stylo_to_cursor_icon.rs:4-49); cursor icons use the `cursor-icon` crate and `ShellProvider::set_cursor` takes an optional `CursorIcon` (packages/blitz-traits/Cargo.toml:21; packages/blitz-traits/src/shell.rs:3; packages/blitz-traits/src/shell.rs:13-15); in the shell they are winit CursorIcon values, and None hides the cursor and resets it to Default (packages/blitz-shell/src/lib.rs:101-112)

### motion#1.5 · lost at strong · .andromeda/design-system-detail/motion.md:12
- (.andromeda/design-system-detail/motion.md:12) The fling's per-frame deceleration lives in the conflicted `scrolling.rs`, and its frames are read off the document's clock (CARRY 4). The resolution must keep its stop below velocity 0.1.
  > - Touch flings decelerate per frame until velocity drops below 0.1 (packages/blitz-dom/src/scrolling.rs:867-885)

### motion#1.6 · lost at strong · .andromeda/design-system-detail/motion.md:13
- (.andromeda/design-system-detail/motion.md:13) Upstream's delta reaches `document.rs`, and `resolve.rs` is a CARRY 4 file. After the take, the overlay scrollbar must still hold `FADE_DELAY` = 500 ms then fade over `FADE_DURATION` = 200 ms, with finished fades dropped at each resolve. `scrollbars` and `scrollbar_drag` must still pass.
  > - Overlay scrollbars show at full opacity on scroll, stay opaque for `FADE_DELAY` = 500 ms after last activity, then fade linearly over `FADE_DURATION` = 200 ms, documented as Chromium's overlay timings (packages/blitz-dom/src/node/scrollbar.rs:12-26; packages/blitz-dom/src/document.rs:1975-1992; packages/blitz-paint/src/render.rs:725-730; packages/blitz-paint/src/render.rs:752-755); finished fades are dropped each resolve (packages/blitz-dom/src/resolve.rs:67-74); hovering where a hidden thumb would be does not summon it, and the thumb changes appearance on hover and while dragged (tests/blitz-tests/tests/scrollbars.rs:1-4; tests/blitz-tests/tests/scrollbar_drag.rs:136-166; tests/blitz-tests/tests/scrollbar_drag.rs:168-222)

### spacing#2.4 · lost at strong · .andromeda/design-system-detail/spacing.md:19
- (.andromeda/design-system-detail/spacing.md:19) The stand's card and home spacing feed the exact bounds the stand checks assert. A take must leave these values as written. Upstream's one stand edit, `cells.rs`, is a key-match change. Any bound that does move is re-measured and recorded, never loosened to pass.
  > - seven_guis cards pad 24px 32px with 16px gap; home pads 48px 32px 64px (examples/seven_guis/src/tasks/counter.rs:38-42; examples/seven_guis/src/app.rs:191)

### spacing#2.5 · lost at strong · .andromeda/design-system-detail/spacing.md:20
- (.andromeda/design-system-detail/spacing.md:20) The user-agent body margin sits under every stand screen's bounds, and `assets/default.css` is a CARRY 5 merge-surface file. The value must stand on the take's tree.
  > - blitz-dom default body margin 8px (packages/blitz-dom/assets/default.css:274-277)

### spacing#3.1 · lost at strong · .andromeda/design-system-detail/spacing.md:26
- (.andromeda/design-system-detail/spacing.md:26) Upstream's Stylo move edits `stylo.rs`. The presentational-hint margin mappings must survive the take unchanged. Where upstream's hunks land above them, their cited line ranges are re-pointed at the wrap.
  > - `hspace` and `vspace` map to horizontal and vertical margins on `embed`, `img`, `object`, `marquee` and image inputs (packages/blitz-dom/src/stylo.rs:1075-1101)
  > - Body `marginwidth`, `marginheight`, `leftmargin` and `topmargin` map to pixel margins; `rightmargin` and `bottommargin` are deliberately ignored (packages/blitz-dom/src/stylo.rs:1145-1182)

### spacing#3.3 · lost at strong · .andromeda/design-system-detail/spacing.md:28
- (.andromeda/design-system-detail/spacing.md:28) Upstream's delta edits `node/scrollbar.rs` (a style-getter rename). The overlay scrollbar geometry must stay at these values, and its cite is re-pointed where the take shifts its lines.
  > - Overlay scrollbar thumb geometry: thickness 10.0, thin thickness 6.0, margin 2.0 and minimum length 32.0 CSS px (packages/blitz-dom/src/node/scrollbar.rs:122-126)

### spacing#3.4 · lost at strong · .andromeda/design-system-detail/spacing.md:29
- (.andromeda/design-system-detail/spacing.md:29) Upstream's `46d7cf993` and `f102663b8` edit `node/node.rs`. The single-line input's vertical centring must hold on the take's tree, and its cite is re-pointed where the take shifts its lines.
  > - Single-line text inputs are vertically centered within their content box (packages/blitz-dom/src/node/node.rs:855-873)

### spacing#3.5 · lost at classify · .andromeda/design-system-detail/spacing.md:30
- (.andromeda/design-system-detail/spacing.md:30) Upstream's delta edits blitz-paint `render.rs` and `text.rs` (`TextDecorationStyle::MozNone` → `None` among them). The marker offset and the double-decoration gap must stay as recorded, and their cites are re-pointed where the take shifts their lines.
  > - Outside list markers using a character are padded 8 CSS px from the item's border box (packages/blitz-paint/src/render.rs:1057-1068)
  > - The double text decoration places its second line thickness + 1 CSS px away (packages/blitz-paint/src/text.rs:323-333)

### surface#6.3 · lost at strong · .andromeda/design-system-detail/surface.md:52
- (.andromeda/design-system-detail/surface.md:52) The Dioxus 0.8 take (`286874cc1`) syncs `packages/dioxus-native` from the Dioxus repo. Where the take carries it, this row's `config.rs` reading is re-read on the take's tree.
  > - Window compositing alpha mode is configurable, for example for transparent windows; unsupported modes are ignored by the renderer (packages/dioxus-native/src/config.rs:66-74)

### surface#7.1 · lost at classify · .andromeda/design-system-detail/surface.md:58
- (.andromeda/design-system-detail/surface.md:58) Same ground as above: the wasm32 canvas surface and the `scrollbars` feature forwarding are re-read against the synced `packages/dioxus-native` (`config.rs`, `Cargo.toml`) where the take carries `286874cc1`.
  > - On wasm32 the same surface is a canvas appended to the page body, sized from host CSS (packages/dioxus-native/src/config.rs:22-34)
  > - The optional `scrollbars` feature forwards to blitz-paint (packages/dioxus-native/Cargo.toml:49)

### surface#16.1 · lost at classify · .andromeda/design-system-detail/surface.md:131
- (.andromeda/design-system-detail/surface.md:131) The bundled-font rule on WASM cites `packages/dioxus-native/src/config.rs`, which the Dioxus sync may move. Where the take carries `286874cc1`, re-read it there.
  > - WASM builds register bundled DejaVu Sans for sans-serif, serif, monospace and system-ui (examples/wasm_hello/src/lib.rs:75-100); on WASM a font context with bundled fonts must be provided (packages/dioxus-native/src/config.rs:56-64)

### typography#2.3 · lost at strong · .andromeda/design-system-detail/typography.md:31
- (.andromeda/design-system-detail/typography.md:31) The stand's font context is built through `build_single_font_ctx` in blitz-dom's `lib.rs`. A take that moves `lib.rs` or `dioxus-native`'s `config.rs` (`286874cc1`, `fb297a38c`) keeps that function registering one font as the fallback for all four generics.
  > - A custom `FontContext` can be set; on WASM a context with bundled fonts must be provided, using `build_single_font_ctx` for one font (packages/dioxus-native/src/config.rs:56-64); `build_single_font_ctx` registers one font as fallback for SansSerif, Serif, Monospace and SystemUi with system fonts disabled (packages/blitz-dom/src/lib.rs:134-165)

### typography#2.6 · lost at strong · .andromeda/design-system-detail/typography.md:34
- (.andromeda/design-system-detail/typography.md:34) The screenshot and lint checks re-run on the merged engine still refuse `fonts-unavailable` before anything is painted or read. The stand's bundled font still passes `can_paint_text`, which lives in `document.rs`, a file upstream's delta reaches.
  > - Since 2026-10-12-headless-screenshot a driver `screenshot` of an app whose font context resolves no `sans-serif` family (`BaseDocument::can_paint_text` false) is refused `fonts-unavailable` with nothing painted — a shot never paints text skipped; the stand's bundled font passes the check (packages/blitz-dom/src/document.rs:1134-1147; as measured at escher-0.1.0/chunks/2026-10-12-headless-screenshot/report.md)

### typography#2.5 · lost at strong · .andromeda/design-system-detail/typography.md:33
- (.andromeda/design-system-detail/typography.md:33) The bullet font registered in the default font context lives in `document.rs`, which upstream moves. The take keeps that registration.
  > - A bullet font is always registered in the default font context (packages/blitz-dom/src/document.rs:440-442; packages/blitz-dom/src/lib.rs:32)

### typography#3.1 · lost at strong · .andromeda/design-system-detail/typography.md:42
- (.andromeda/design-system-detail/typography.md:42) The Stylo 0.23 move (`06ecec886`, 43 files) reaches the Stylo-to-Parley bridge. Unless the take records the move as measured, generic families still map to Parley generics with `None` as sans-serif, the family `can_paint_text` resolves.
  > - Generic font families map to Parley generics, with `None` as sans-serif (packages/blitz-dom/src/stylo_to_parley.rs:50-60)

### typography#3.4 · lost at strong · .andromeda/design-system-detail/typography.md:45
- (.andromeda/design-system-detail/typography.md:45) `#1233` splits the inline layout module. A take carrying it keeps `line-height: normal` resolving as 1.2 × font size, because the stand's bounds and snapshot sizes depend on it, and any moved number is re-measured, never loosened.
  > - Line height maps `normal`, number and length to Parley line heights (packages/blitz-dom/src/stylo_to_parley.rs:407-412); the layout engine resolves `line-height: normal` as 1.2 × font size (packages/blitz-dom/src/layout/mod.rs:232-236)

### typography#3.9 · lost at strong · .andromeda/design-system-detail/typography.md:50
- (.andromeda/design-system-detail/typography.md:50) The stylist device seeding sits in `document.rs`, which the Stylo move reaches. It keeps seeding the root font size and line height, so `rem_after_viewport_change` still passes on the take.
  > - The new stylist device is seeded with the root element's font size and line height so rem/rlh units do not fall back to the 16px default (packages/blitz-dom/src/document.rs:2322-2343); `rem` resolves against the root font-size after viewport and hidpi changes, with an initial root font-size of 16px (tests/blitz-tests/tests/rem_after_viewport_change.rs:1-8; tests/blitz-tests/tests/rem_after_viewport_change.rs:49-77)

### typography#1.9 · lost at strong · .andromeda/design-system-detail/typography.md:18
- (.andromeda/design-system-detail/typography.md:18) The stand's own type stays as recorded: base 14px, home title 700 at 36px. `286874cc1` edits a stand task (`cells.rs`), and `stand_contrast` reads font size and weight for its large-text threshold. A take moves no stand font value.
  > | seven_guis | sans-serif | home title 700 | base 14px; home title 36px | not stated | (examples/seven_guis/src/app.rs:179-205) |

## Held

- color-palette#1.1
- color-palette#2.3
- color-palette#3.5
- color-palette#4.4
- depth-strategy#3.1
- motion#1.2
- motion#1.3
- motion#1.4
- motion#2.1
- motion#2.2
- motion#3.1
- motion#4.3
- surface#1.1
- surface#1.2
- surface#15.1
- typography#1.14
- typography#2.2
- typography#2.8
- typography#3.7

## Cascade only
