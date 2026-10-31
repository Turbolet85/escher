### color-palette#1.1 · values · palette · ## Color Palette · lines 8–8
- The `Color` type is `AlphaColor<Srgb>` and Stylo colors convert to it via sRGB (packages/blitz-dom/src/util.rs:12; packages/blitz-dom/src/util.rs:166-178)
### color-palette#1.2 · values · palette · ## Color Palette · lines 9–9
- The system color scheme is an enum `Light` (default) / `Dark`, set on the viewport, and it feeds `prefers-color-scheme` (packages/blitz-traits/src/shell.rs:68-74; tests/blitz-tests/tests/device_coalescing.rs:101; packages/blitz-dom/src/stylo_device.rs:80-83)
### color-palette#1.3 · values · palette · ## Color Palette · lines 10–10
- Color-scheme changes trigger a full recascade because `light-dark()` and system colors resolve at cascade time (packages/blitz-dom/src/document.rs:2342-2349)
### color-palette#1.4 · values · palette · ## Color Palette · lines 11–11
- The parent viewport's color scheme is copied to iframe sub-documents (packages/blitz-dom/src/resolve.rs:148-152)
### color-palette#2.3 · values · palette · ## Color Palette › ### Core Colors · lines 27–27
| seven_guis stand | — | The stand's accent (its load page's `load-fetch` and its Cells grid's current-cell focus outline included; examples/seven_guis/src/tasks/cells.rs:551-554), its flight booker's success-message colours and its flight-type select's border are held in layout-templates §Surface: desktop-native → Primary screens (`values · desktop-native-screens`), not as design-system values |
### color-palette#2.4 · values · palette · ## Color Palette › ### Core Colors · lines 28–28
| blitz-dom default link | `rgb(0, 0, 238)` | Default stylesheet link, its rule selecting `a[href]` — an anchor with no `href` takes neither the colour nor the underline; input focus outline `#4D90FE`; button background `#EFEFEF` (packages/blitz-dom/assets/default.css:43-46; packages/blitz-dom/assets/default.css:98-101; packages/blitz-dom/assets/default.css:111) |
### color-palette#2.5 · values · palette · ## Color Palette › ### Core Colors · lines 29–29
| blitz-dom default chosen row | `#c8c8c8` background, `#000000` text | A `multiple` select's chosen row: Chromium's inactive selected pair, hardcoded as the engine holds no tokens, 12.56:1 as measured by `select_interaction` (packages/blitz-dom/assets/default.css:161-188; as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md) |
### color-palette#2.6 · values · palette · ## Color Palette › ### Core Colors · lines 30–30
| Default text-selection highlight | rgb 180, 213, 255 | Engine default (packages/blitz-paint/src/lib.rs:28-29) |
### color-palette#5.1 · values · status-states · ## Color Palette › ### Semantic Colors · lines 69–71
| State | Values (as declared) | Usage |
|-------|----------------------|-------|
| Disabled checkbox/radio | rgba 209, 209, 209, 255 | Disabled accent; otherwise the element's color stands in for accent-color per a TODO (packages/blitz-paint/src/render/form_controls.rs:21-26) |
### color-palette#7.3 · values · palette · ## Color Palette › ### Border Progression · lines 97–97
| Focus — blitz-dom default input outline | `#4D90FE` | (packages/blitz-dom/assets/default.css:43-46; packages/blitz-dom/assets/default.css:98-101; packages/blitz-dom/assets/default.css:111) |
### color-palette#8.1 · values · palette · ## Color Palette › ### Border Progression · lines 103–103
- Default scrollbar thumbs: dark scheme rest 214,214,214,178 · hover 190,190,190,222 · active 172,172,172,255 · stroke 0,0,0,102; light scheme rest 128,128,128,178 · hover 152,152,152,222 · active 170,170,170,255 · stroke 255,255,255,102 (packages/blitz-paint/src/render.rs:757-776); the palette follows the viewport color scheme and paints as a fill plus a thin contrast stroke (tests/blitz-tests/tests/scrollbars.rs:204-211; tests/blitz-tests/tests/scrollbars.rs:194-202)
### color-palette#8.2 · values · palette · ## Color Palette › ### Border Progression · lines 104–104
- `scrollbar-color` resolves to absolute thumb and track colors against the element's computed `color`, defaulting to `Auto` (packages/blitz-dom/src/node/scrollbar.rs:38-48; packages/blitz-dom/src/node/scrollbar.rs:58-74); author thumbs blend for contrast at 1.8 on hover and 1.3 when active (packages/blitz-paint/src/render.rs:778-780; packages/blitz-paint/src/render.rs:815-822), and a near-white author thumb darkens on hover/drag (tests/blitz-tests/tests/scrollbars.rs:125-160; tests/blitz-tests/tests/scrollbar_drag.rs:224-227)
### color-palette#10.1 · absence · palette · ## Color Palette › ### Border Progression · lines 120–120
- Observed absent — color tokens or CSS custom properties in the engine and integration crates · searched: `--[a-z]` over the 15 s05 files; `color|Color` and `--[a-z]+-|var\(` over the 8 s07 slice files; `--[a-z-]+:` over the 32 s10 slice files; `var\(--|--[a-z]+-` over the 21 listed s11 files
### color-palette#12.1 · absence · palette · ## Color Palette › ### Border Progression · lines 140–140
- Observed absent — a prefers-color-scheme rule in the s02 fixtures · searched: `prefers-color-scheme` over the 21 s02 files
