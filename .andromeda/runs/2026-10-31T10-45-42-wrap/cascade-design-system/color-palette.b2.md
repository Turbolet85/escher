### color-palette#4.5 · values · palette · ## Color Palette › ### Text Hierarchy · lines 60–60
| wasm_hello heading / code | `#ffd166` / `#ff7b9c` | (examples/wasm_hello/src/lib.rs:46; examples/wasm_hello/src/lib.rs:55; examples/wasm_hello/src/lib.rs:62; examples/wasm_hello/src/lib.rs:68) |
### color-palette#5.1 · values · status-states · ## Color Palette › ### Semantic Colors · lines 68–70
| State | Values (as declared) | Usage |
|-------|----------------------|-------|
| Disabled checkbox/radio | rgba 209, 209, 209, 255 | Disabled accent; otherwise the element's color stands in for accent-color per a TODO (packages/blitz-paint/src/render/form_controls.rs:21-26) |
### color-palette#6.1 · values · status-states · ## Color Palette › ### Semantic Colors · lines 76–79
**Domain status colors** — WPT runner terminal result colors:
| Result | Terminal color |
|--------|----------------|
| PASS | green (wpt/runner/src/main.rs:391-401) |
### color-palette#6.2 · values · status-states · ## Color Palette › ### Semantic Colors · lines 80–80
| FAIL with some passing subtests | yellow (wpt/runner/src/main.rs:391-401) |
### color-palette#6.3 · values · status-states · ## Color Palette › ### Semantic Colors · lines 81–81
| FAIL | red (wpt/runner/src/main.rs:391-401) |
### color-palette#6.4 · values · status-states · ## Color Palette › ### Semantic Colors · lines 82–82
| TIMEOUT | bright red (wpt/runner/src/main.rs:391-401) |
### color-palette#6.5 · values · status-states · ## Color Palette › ### Semantic Colors · lines 83–83
| SKIP | bright black (wpt/runner/src/main.rs:391-401) |
### color-palette#6.6 · values · status-states · ## Color Palette › ### Semantic Colors · lines 84–84
| CRASH | bright magenta (wpt/runner/src/main.rs:391-401) |
### color-palette#7.1 · values · palette · ## Color Palette › ### Border Progression · lines 92–94
| Intensity | Value | Usage |
|-----------|-------|-------|
| Browser urlbar border | `#EEE` | (apps/browser/assets/browser.css:25; apps/browser/assets/browser.css:44; apps/browser/assets/browser.css:51; apps/browser/assets/browser.css:77; apps/browser/assets/browser.css:134-135) |
### color-palette#7.2 · values · palette · ## Color Palette › ### Border Progression · lines 95–95
| Focus — browser chrome | `#5E9ED6` | (apps/browser/assets/browser.css:172-175; apps/browser/assets/browser.css:182-189) |
### color-palette#7.3 · values · palette · ## Color Palette › ### Border Progression · lines 96–96
| Focus — blitz-dom default input outline | `#4D90FE` | (packages/blitz-dom/assets/default.css:43-46; packages/blitz-dom/assets/default.css:98-101; packages/blitz-dom/assets/default.css:111) |
### color-palette#8.1 · values · palette · ## Color Palette › ### Border Progression · lines 102–102
- Default scrollbar thumbs: dark scheme rest 214,214,214,178 · hover 190,190,190,222 · active 172,172,172,255 · stroke 0,0,0,102; light scheme rest 128,128,128,178 · hover 152,152,152,222 · active 170,170,170,255 · stroke 255,255,255,102 (packages/blitz-paint/src/render.rs:757-776); the palette follows the viewport color scheme and paints as a fill plus a thin contrast stroke (tests/blitz-tests/tests/scrollbars.rs:204-211; tests/blitz-tests/tests/scrollbars.rs:194-202)
### color-palette#8.2 · values · palette · ## Color Palette › ### Border Progression · lines 103–103
- `scrollbar-color` resolves to absolute thumb and track colors against the element's computed `color`, defaulting to `Auto` (packages/blitz-dom/src/node/scrollbar.rs:38-48; packages/blitz-dom/src/node/scrollbar.rs:58-74); author thumbs blend for contrast at 1.8 on hover and 1.3 when active (packages/blitz-paint/src/render.rs:778-780; packages/blitz-paint/src/render.rs:815-822), and a near-white author thumb darkens on hover/drag (tests/blitz-tests/tests/scrollbars.rs:125-160; tests/blitz-tests/tests/scrollbar_drag.rs:224-227)
### color-palette#8.3 · values · palette · ## Color Palette › ### Border Progression · lines 104–104
- Unchecked radio rings use the CSS palette GRAY; checkbox ticks/gaps use white (packages/blitz-paint/src/render/form_controls.rs:76-79; packages/blitz-paint/src/render/form_controls.rs:99-101)
### color-palette#8.4 · values · palette · ## Color Palette › ### Border Progression · lines 105–105
- Debug overlay: content blue 66,144,245,128 · padding green 81,144,66,128 · border red 245,66,66,128 · margin orange 249,204,157,128 (packages/blitz-paint/src/debug_overlay.rs:95; packages/blitz-paint/src/debug_overlay.rs:98; packages/blitz-paint/src/debug_overlay.rs:110; packages/blitz-paint/src/debug_overlay.rs:119)
### color-palette#8.5 · values · palette · ## Color Palette › ### Border Progression · lines 106–106
- Devtools layout strokes are red for block/flow-root, green for flex, blue for grid (packages/blitz-paint/src/render.rs:1355-1362); devtools `show_layout` documents "different border colors" without stating values (packages/blitz-traits/src/devtools.rs:8-10)
### color-palette#8.6 · values · palette · ## Color Palette › ### Border Progression · lines 107–107
- In SVG serialization, `currentColor` in attribute values is replaced with the element's computed `color` (packages/blitz-dom/src/node/serialize.rs:18-19; packages/blitz-dom/src/node/serialize.rs:120-125; packages/blitz-dom/src/node/serialize.rs:186-194)
### color-palette#9.1 · values · palette · ## Color Palette › ### Border Progression · lines 113–113
- rdme uses GitHub markdown color tokens (`--fgColor-*`, `--bgColor-*`, `--borderColor-*`, syntax colors) defined separately for dark and light schemes (apps/readme/assets/github-markdown.css:13-124)
### color-palette#10.1 · absence · palette · ## Color Palette › ### Border Progression · lines 119–119
- Observed absent — color tokens or CSS custom properties in the engine and integration crates · searched: `--[a-z]` over the 15 s05 files; `color|Color` and `--[a-z]+-|var\(` over the 8 s07 slice files; `--[a-z-]+:` over the 32 s10 slice files; `var\(--|--[a-z]+-` over the 21 listed s11 files
### color-palette#11.1 · values · palette · ## Color Palette › ### Border Progression · lines 125–125
- clip-path fixture defines a 25-colour block palette commented "curated harmonious palette" (e.g. #e63946, #2a9d8f, #264653, #8338ec, #3a86ff) (examples/assets/clip-path.html:35-60)
### color-palette#11.2 · values · palette · ## Color Palette › ### Border Progression · lines 126–126
- border-styles fixture uses border colours #d6336c, #1c7ed6, #2f9e44, #f59f00 and dark #1a1a1a on #fffbe6 / #f5f5f5 (examples/assets/border-styles.html:7; examples/assets/border-styles.html:27-29; examples/assets/border-styles.html:77; examples/assets/border-styles.html:82)
### color-palette#11.3 · values · palette · ## Color Palette › ### Border Progression · lines 127–127
- A reduced GitHub fixture sets `--borderColor-default: green` and consumes it via `var()` (examples/assets/github_profile_reduced.html:6-13)
### color-palette#11.4 · values · palette · ## Color Palette › ### Border Progression · lines 128–128
- The BBC fixture uses relative colour syntax `color(from #202224 srgb r g b / …)` (examples/assets/bbc_reduced.html:13-16)
### color-palette#11.5 · values · palette · ## Color Palette › ### Border Progression · lines 129–129
- graphite defines 16 color custom properties on :root, among them --color-navy #16323f, --color-crimson #803847, --color-fog #eee and --color-seaside-rgb 176, 214, 203, repeated in its subsets; page text is var(--color-navy) on #fff and links var(--color-crimson) (examples/assets/graphite.html:890-906; examples/assets/graphite_blog_section.html:890-906; examples/assets/graphite_software_overview.html:888-904; examples/assets/graphite.html:918-922; examples/assets/graphite.html:1405-1407)
### color-palette#11.6 · values · palette · ## Color Palette › ### Border Progression · lines 130–130
- google sets body/input/button text #202124 on #fff, links #1a0dab, visited #681da8, and shows a "Dark theme: Off" menu item (examples/assets/google.html:19-36; examples/assets/google.html:3788); observed absent — CSS custom properties in the google fixture · searched: `var\(--|--[a-z]+:` over examples/assets/google.html
