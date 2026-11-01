### color-palette#1.3 · values · palette · ## Color Palette · lines 10–10 · stale
- Color-scheme changes trigger a full recascade because `light-dark()` and system colors resolve at cascade time (packages/blitz-dom/src/document.rs:2713-2720)
### color-palette#1.5 · values · palette · ## Color Palette · lines 12–12 · stale
- The legacy `bgcolor` attribute maps to `background-color` (packages/blitz-dom/src/stylo.rs:1210-1217)
### color-palette#1.7 · values · palette · ## Color Palette · lines 14–14 · stale
- The canvas background is the root element's background colour, falling back to body's when the root is transparent (packages/blitz-paint/src/render.rs:196-231)
### color-palette#2.4 · values · palette · ## Color Palette › ### Core Colors · lines 28–28 · stale
| blitz-dom default link | `rgb(0, 0, 238)` | Default stylesheet link, its rule selecting `a[href]` — an anchor with no `href` takes neither the colour nor the underline; input focus outline `#4D90FE`; button background `#EFEFEF` (packages/blitz-dom/assets/default.css:43-46; packages/blitz-dom/assets/default.css:98-101; packages/blitz-dom/assets/default.css:111) |
### color-palette#2.5 · values · palette · ## Color Palette › ### Core Colors · lines 29–29 · stale
| blitz-dom default chosen row | `#c8c8c8` background, `#000000` text | A `multiple` select's chosen row: Chromium's inactive selected pair, hardcoded as the engine holds no tokens, 12.56:1 as measured by `select_interaction` (packages/blitz-dom/assets/default.css:161-199; as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md) |
### color-palette#2.10 · values · palette · ## Color Palette › ### Core Colors · lines 34–34 · stale
| Default text-selection highlight | rgb 180, 213, 255 | Engine default (packages/blitz-paint/src/lib.rs:29-30) |
### color-palette#7.3 · values · palette · ## Color Palette › ### Border Progression · lines 101–101 · stale
| Focus — blitz-dom default input outline | `#4D90FE` | (packages/blitz-dom/assets/default.css:43-46; packages/blitz-dom/assets/default.css:98-101; packages/blitz-dom/assets/default.css:111) |
### color-palette#8.1 · values · palette · ## Color Palette › ### Border Progression · lines 107–107 · stale
- Default scrollbar thumbs: dark scheme rest 214,214,214,178 · hover 190,190,190,222 · active 172,172,172,255 · stroke 0,0,0,102; light scheme rest 128,128,128,178 · hover 152,152,152,222 · active 170,170,170,255 · stroke 255,255,255,102 (packages/blitz-paint/src/render.rs:799-818); the palette follows the viewport color scheme and paints as a fill plus a thin contrast stroke (tests/blitz-tests/tests/scrollbars.rs:204-211; tests/blitz-tests/tests/scrollbars.rs:194-202)
### color-palette#8.2 · values · palette · ## Color Palette › ### Border Progression · lines 108–108 · stale
- `scrollbar-color` resolves to absolute thumb and track colors against the element's computed `color`, defaulting to `Auto` (packages/blitz-dom/src/node/scrollbar.rs:38-48; packages/blitz-dom/src/node/scrollbar.rs:58-74); author thumbs blend for contrast at 1.8 on hover and 1.3 when active (packages/blitz-paint/src/render.rs:820-822; packages/blitz-paint/src/render.rs:857-864), and a near-white author thumb darkens on hover/drag (tests/blitz-tests/tests/scrollbars.rs:125-160; tests/blitz-tests/tests/scrollbar_drag.rs:224-227)
### color-palette#8.5 · values · palette · ## Color Palette › ### Border Progression · lines 111–111 · stale
- Devtools layout strokes are red for block/flow-root, green for flex, blue for grid (packages/blitz-paint/src/render.rs:1398-1405); devtools `show_layout` documents "different border colors" without stating values (packages/blitz-traits/src/devtools.rs:8-10)
### color-palette#10.1 · absence · palette · ## Color Palette › ### Border Progression · lines 124–124 · stale
- Observed absent — color tokens or CSS custom properties in the engine and integration crates · searched: `--[a-z]` over the 15 s05 files; `color|Color` and `--[a-z]+-|var\(` over the 8 s07 slice files; `--[a-z-]+:` over the 32 s10 slice files; `var\(--|--[a-z]+-` over the 21 listed s11 files; the open select list's colours are fixed constants on `SelectListView`, hardcoded, and add no token or custom property
