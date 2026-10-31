### color-palette#1.3 · values · palette · ## Color Palette · lines 10–10 · stale
- Color-scheme changes trigger a full recascade because `light-dark()` and system colors resolve at cascade time (packages/blitz-dom/src/document.rs:2681-2688)
### color-palette#1.4 · values · palette · ## Color Palette · lines 11–11 · stale
- The parent viewport's color scheme is copied to iframe sub-documents (packages/blitz-dom/src/resolve.rs:151-155)
### color-palette#1.5 · values · palette · ## Color Palette · lines 12–12 · stale
- The legacy `bgcolor` attribute maps to `background-color` (packages/blitz-dom/src/stylo.rs:1202-1209)
### color-palette#1.7 · values · palette · ## Color Palette · lines 14–14 · stale
- The canvas background is the root element's background colour, falling back to body's when the root is transparent (packages/blitz-paint/src/render.rs:194-229)
### color-palette#2.5 · values · palette · ## Color Palette › ### Core Colors · lines 29–29 · add-to
| blitz-dom default chosen row | `#c8c8c8` background, `#000000` text | A `multiple` select's chosen row: Chromium's inactive selected pair, hardcoded as the engine holds no tokens, 12.56:1 as measured by `select_interaction` (packages/blitz-dom/assets/default.css:161-199; as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md) |
### color-palette#8.1 · values · palette · ## Color Palette › ### Border Progression · lines 103–103 · stale
- Default scrollbar thumbs: dark scheme rest 214,214,214,178 · hover 190,190,190,222 · active 172,172,172,255 · stroke 0,0,0,102; light scheme rest 128,128,128,178 · hover 152,152,152,222 · active 170,170,170,255 · stroke 255,255,255,102 (packages/blitz-paint/src/render.rs:760-779); the palette follows the viewport color scheme and paints as a fill plus a thin contrast stroke (tests/blitz-tests/tests/scrollbars.rs:204-211; tests/blitz-tests/tests/scrollbars.rs:194-202)
### color-palette#8.2 · values · palette · ## Color Palette › ### Border Progression · lines 104–104 · stale
- `scrollbar-color` resolves to absolute thumb and track colors against the element's computed `color`, defaulting to `Auto` (packages/blitz-dom/src/node/scrollbar.rs:38-48; packages/blitz-dom/src/node/scrollbar.rs:58-74); author thumbs blend for contrast at 1.8 on hover and 1.3 when active (packages/blitz-paint/src/render.rs:781-783; packages/blitz-paint/src/render.rs:818-825), and a near-white author thumb darkens on hover/drag (tests/blitz-tests/tests/scrollbars.rs:125-160; tests/blitz-tests/tests/scrollbar_drag.rs:224-227)
### color-palette#8.5 · values · palette · ## Color Palette › ### Border Progression · lines 107–107 · stale
- Devtools layout strokes are red for block/flow-root, green for flex, blue for grid (packages/blitz-paint/src/render.rs:1358-1365); devtools `show_layout` documents "different border colors" without stating values (packages/blitz-traits/src/devtools.rs:8-10)
### color-palette#10.1 · absence · palette · ## Color Palette › ### Border Progression · lines 120–120 · add-to
- Observed absent — color tokens or CSS custom properties in the engine and integration crates · searched: `--[a-z]` over the 15 s05 files; `color|Color` and `--[a-z]+-|var\(` over the 8 s07 slice files; `--[a-z-]+:` over the 32 s10 slice files; `var\(--|--[a-z]+-` over the 21 listed s11 files
### typography#1.6 · values · typography · ## Typography · lines 15–15 · stale
| blitz-dom default form controls | inputs/selects/buttons `system-ui, sans-serif`; textarea monospace | not stated | not stated | not stated | (packages/blitz-dom/assets/default.css:88-96; packages/blitz-dom/assets/default.css:350-396; packages/blitz-dom/assets/default.css:407-415; packages/blitz-dom/assets/default.css:711-716) |
### typography#1.7 · values · typography · ## Typography · lines 16–16 · stale
| blitz-dom default headings | not stated | bold | h1–h6 from 2em down to 0.67em | not stated | (packages/blitz-dom/assets/default.css:88-96; packages/blitz-dom/assets/default.css:350-396; packages/blitz-dom/assets/default.css:407-415; packages/blitz-dom/assets/default.css:711-716) |
### typography#1.8 · values · typography · ## Typography · lines 17–17 · stale
| blitz-dom default code/pre | `-moz-fixed` | not stated | not stated | not stated | (packages/blitz-dom/assets/default.css:88-96; packages/blitz-dom/assets/default.css:350-396; packages/blitz-dom/assets/default.css:407-415; packages/blitz-dom/assets/default.css:711-716) |
### typography#2.4 · values · typography · ## Typography · lines 31–31 · add-to
- Font features: system-fonts, woff, complex-scripts (dictionary line-breaking), font-embolden and apple-font-embolden (packages/dioxus-native/Cargo.toml:20-25; packages/dioxus-native/Cargo.toml:47-48)
### typography#2.5 · values · typography · ## Typography · lines 32–32 · stale
- A bullet font is always registered in the default font context (packages/blitz-dom/src/document.rs:445-447; packages/blitz-dom/src/lib.rs:32)
### typography#2.6 · values · typography · ## Typography · lines 33–33 · stale
- Since 2026-10-12-headless-screenshot a driver `screenshot` of an app whose font context resolves no `sans-serif` family (`BaseDocument::can_paint_text` false) is refused `fonts-unavailable` with nothing painted — a shot never paints text skipped; the stand's bundled font passes the check (packages/blitz-dom/src/document.rs:1500-1513; as measured at escher-0.1.0/chunks/2026-10-12-headless-screenshot/report.md)
### typography#3.4 · values · typography · ## Typography · lines 44–44 · add-to
- Line height maps `normal`, number and length to Parley line heights (packages/blitz-dom/src/stylo_to_parley.rs:407-412); the layout engine resolves `line-height: normal` as 1.2 × font size (packages/blitz-dom/src/layout/mod.rs:232-236)
### typography#3.6 · values · typography · ## Typography · lines 46–46 · stale
- Text locale comes from the computed `_x_lang` value, which the `lang` attribute sets (packages/blitz-dom/src/stylo_to_parley.rs:495; packages/blitz-dom/src/stylo.rs:1217-1219)
### typography#3.9 · values · typography · ## Typography · lines 49–49 · stale
- The new stylist device is seeded with the root element's font size and line height so rem/rlh units do not fall back to the 16px default (packages/blitz-dom/src/document.rs:2691-2712); `rem` resolves against the root font-size after viewport and hidpi changes, with an initial root font-size of 16px (tests/blitz-tests/tests/rem_after_viewport_change.rs:1-8; tests/blitz-tests/tests/rem_after_viewport_change.rs:49-77)
### typography#3.11 · values · typography · ## Typography · lines 51–51 · contradict
- Font emboldening is enabled by the font-embolden feature, or apple-font-embolden on macOS and iOS (packages/blitz-paint/Cargo.toml:18-19; packages/blitz-paint/src/lib.rs:22-26); strength is 0.015125 and 0.0121 times the CSS font size, each capped at 0.3, with hinting off when emboldening (packages/blitz-paint/src/text.rs:678-695)
### spacing#2.5 · values · spacing · ## Spacing · lines 20–20 · stale
- blitz-dom default body margin 8px (packages/blitz-dom/assets/default.css:313-316)
### spacing#2.6 · values · spacing · ## Spacing · lines 21–21 · add-to
- blitz-dom default `multiple` select rows (`select[multiple] > option`, `select[multiple] > optgroup > option`) padding 0 2px (packages/blitz-dom/assets/default.css:173-177)
### spacing#3.2 · values · spacing · ## Spacing · lines 28–28 · stale
- Body `marginwidth`, `marginheight`, `leftmargin` and `topmargin` map to pixel margins; `rightmargin` and `bottommargin` are deliberately ignored (packages/blitz-dom/src/stylo.rs:1163-1200)
### spacing#3.3 · values · spacing · ## Spacing · lines 29–29 · add-to
- Overlay scrollbar thumb geometry: thickness 10.0, thin thickness 6.0, margin 2.0 and minimum length 32.0 CSS px (packages/blitz-dom/src/node/scrollbar.rs:122-126)
### spacing#3.5 · values · spacing · ## Spacing · lines 31–31 · stale
- Outside list markers using a character are padded 8 CSS px from the item's border box (packages/blitz-paint/src/render.rs:1060-1071)
