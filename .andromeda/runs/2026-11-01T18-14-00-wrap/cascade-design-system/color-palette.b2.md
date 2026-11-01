### color-palette#3.7 · values · palette · ## Color Palette › ### Surface Scale (elevation hierarchy) · lines 51–51
| custom-widget, static-HTML and wgpu_texture examples | `#f4e8d2` | Main background (examples/custom_widget.rs:187; examples/html.rs:16; examples/wgpu_texture/src/styles.css:15) |
### color-palette#4.1 · values · palette · ## Color Palette › ### Text Hierarchy · lines 59–61
| Level | Value | Usage |
|-------|-------|-------|
| Browser tooltip and status bar text | `#333` | (apps/browser/assets/browser.css:61-62; apps/browser/assets/browser.css:267-268; apps/browser/assets/browser.css:292; apps/browser/assets/browser.css:303) |
### color-palette#4.2 · values · palette · ## Color Palette › ### Text Hierarchy · lines 62–62
| Browser about-page muted text | `#888` / `#777` / `#aaa` | (apps/browser/assets/about-history.css:3; apps/browser/assets/about-history.css:69; apps/browser/assets/about-history.css:77; apps/browser/assets/about-history.css:84; apps/browser/assets/about-newtab.css:7; apps/browser/assets/about-stub.css:2) |
### color-palette#4.3 · values · palette · ## Color Palette › ### Text Hierarchy · lines 63–63
| Browser error page paragraph | `#666` | (apps/browser/assets/error.html:14) |
### color-palette#4.4 · values · palette · ## Color Palette › ### Text Hierarchy · lines 64–64
| todomvc body / heading | `#4d4d4d` / `rgba(175, 47, 47, 1.0)` | Body text on `#f5f5f5`; heading (examples/todomvc/src/todomvc.css:26-27; examples/todomvc/src/todomvc.css:76) |
### color-palette#4.5 · values · palette · ## Color Palette › ### Text Hierarchy · lines 65–65
| wasm_hello heading / code | `#ffd166` / `#ff7b9c` | (examples/wasm_hello/src/lib.rs:46; examples/wasm_hello/src/lib.rs:55; examples/wasm_hello/src/lib.rs:62; examples/wasm_hello/src/lib.rs:68) |
### color-palette#5.1 · values · status-states · ## Color Palette › ### Semantic Colors · lines 73–75
| State | Values (as declared) | Usage |
|-------|----------------------|-------|
| Disabled checkbox/radio | rgba 209, 209, 209, 255 | Disabled accent; otherwise the element's color stands in for accent-color per a TODO (packages/blitz-paint/src/render/form_controls.rs:21-26) |
### color-palette#6.1 · values · status-states · ## Color Palette › ### Semantic Colors · lines 81–84
**Domain status colors** — WPT runner terminal result colors:
| Result | Terminal color |
|--------|----------------|
| PASS | green (wpt/runner/src/main.rs:391-401) |
### color-palette#6.2 · values · status-states · ## Color Palette › ### Semantic Colors · lines 85–85
| FAIL with some passing subtests | yellow (wpt/runner/src/main.rs:391-401) |
### color-palette#6.3 · values · status-states · ## Color Palette › ### Semantic Colors · lines 86–86
| FAIL | red (wpt/runner/src/main.rs:391-401) |
### color-palette#6.4 · values · status-states · ## Color Palette › ### Semantic Colors · lines 87–87
| TIMEOUT | bright red (wpt/runner/src/main.rs:391-401) |
### color-palette#6.5 · values · status-states · ## Color Palette › ### Semantic Colors · lines 88–88
| SKIP | bright black (wpt/runner/src/main.rs:391-401) |
### color-palette#6.6 · values · status-states · ## Color Palette › ### Semantic Colors · lines 89–89
| CRASH | bright magenta (wpt/runner/src/main.rs:391-401) |
### color-palette#7.1 · values · palette · ## Color Palette › ### Border Progression · lines 97–99
| Intensity | Value | Usage |
|-----------|-------|-------|
| Browser urlbar border | `#EEE` | (apps/browser/assets/browser.css:25; apps/browser/assets/browser.css:44; apps/browser/assets/browser.css:51; apps/browser/assets/browser.css:77; apps/browser/assets/browser.css:134-135) |
### color-palette#7.2 · values · palette · ## Color Palette › ### Border Progression · lines 100–100
| Focus — browser chrome | `#5E9ED6` | (apps/browser/assets/browser.css:172-175; apps/browser/assets/browser.css:182-189) |
### color-palette#7.3 · values · palette · ## Color Palette › ### Border Progression · lines 101–101
| Focus — blitz-dom default input outline | `#4D90FE` | (packages/blitz-dom/assets/default.css:43-46; packages/blitz-dom/assets/default.css:98-101; packages/blitz-dom/assets/default.css:111) |
### color-palette#8.1 · values · palette · ## Color Palette › ### Border Progression · lines 107–107
- Default scrollbar thumbs: dark scheme rest 214,214,214,178 · hover 190,190,190,222 · active 172,172,172,255 · stroke 0,0,0,102; light scheme rest 128,128,128,178 · hover 152,152,152,222 · active 170,170,170,255 · stroke 255,255,255,102 (packages/blitz-paint/src/render.rs:799-818); the palette follows the viewport color scheme and paints as a fill plus a thin contrast stroke (tests/blitz-tests/tests/scrollbars.rs:204-211; tests/blitz-tests/tests/scrollbars.rs:194-202)
### color-palette#8.2 · values · palette · ## Color Palette › ### Border Progression · lines 108–108
- `scrollbar-color` resolves to absolute thumb and track colors against the element's computed `color`, defaulting to `Auto` (packages/blitz-dom/src/node/scrollbar.rs:38-48; packages/blitz-dom/src/node/scrollbar.rs:58-74); author thumbs blend for contrast at 1.8 on hover and 1.3 when active (packages/blitz-paint/src/render.rs:820-822; packages/blitz-paint/src/render.rs:857-864), and a near-white author thumb darkens on hover/drag (tests/blitz-tests/tests/scrollbars.rs:125-160; tests/blitz-tests/tests/scrollbar_drag.rs:224-227)
### color-palette#8.3 · values · palette · ## Color Palette › ### Border Progression · lines 109–109
- Unchecked radio rings use the CSS palette GRAY; checkbox ticks/gaps use white (packages/blitz-paint/src/render/form_controls.rs:76-79; packages/blitz-paint/src/render/form_controls.rs:99-101)
### color-palette#8.4 · values · palette · ## Color Palette › ### Border Progression · lines 110–110
- Debug overlay: content blue 66,144,245,128 · padding green 81,144,66,128 · border red 245,66,66,128 · margin orange 249,204,157,128 (packages/blitz-paint/src/debug_overlay.rs:95; packages/blitz-paint/src/debug_overlay.rs:98; packages/blitz-paint/src/debug_overlay.rs:110; packages/blitz-paint/src/debug_overlay.rs:119)
### color-palette#8.5 · values · palette · ## Color Palette › ### Border Progression · lines 111–111
- Devtools layout strokes are red for block/flow-root, green for flex, blue for grid (packages/blitz-paint/src/render.rs:1398-1405); devtools `show_layout` documents "different border colors" without stating values (packages/blitz-traits/src/devtools.rs:8-10)
### color-palette#8.6 · values · palette · ## Color Palette › ### Border Progression · lines 112–112
- In SVG serialization, `currentColor` in attribute values is replaced with the element's computed `color` (packages/blitz-dom/src/node/serialize.rs:18-19; packages/blitz-dom/src/node/serialize.rs:120-125; packages/blitz-dom/src/node/serialize.rs:186-194)
### color-palette#9.1 · values · palette · ## Color Palette › ### Border Progression · lines 118–118
- rdme uses GitHub markdown color tokens (`--fgColor-*`, `--bgColor-*`, `--borderColor-*`, syntax colors) defined separately for dark and light schemes (apps/readme/assets/github-markdown.css:13-124)
### color-palette#10.1 · absence · palette · ## Color Palette › ### Border Progression · lines 124–124
- Observed absent — color tokens or CSS custom properties in the engine and integration crates · searched: `--[a-z]` over the 15 s05 files; `color|Color` and `--[a-z]+-|var\(` over the 8 s07 slice files; `--[a-z-]+:` over the 32 s10 slice files; `var\(--|--[a-z]+-` over the 21 listed s11 files; the open select list's colours are fixed constants on `SelectListView`, hardcoded, and add no token or custom property
### color-palette#11.1 · values · palette · ## Color Palette › ### Border Progression · lines 130–130
- clip-path fixture defines a 25-colour block palette commented "curated harmonious palette" (e.g. #e63946, #2a9d8f, #264653, #8338ec, #3a86ff) (examples/assets/clip-path.html:35-60)
