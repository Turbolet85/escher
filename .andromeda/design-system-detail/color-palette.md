# design-system · Color Palette
Read whole. The master's broad view is .andromeda/design-system.md; this file holds its Color Palette detail.

## Color Palette

<!-- detail: values · palette -->

- The `Color` type is `AlphaColor<Srgb>` and Stylo colors convert to it via sRGB (packages/blitz-dom/src/util.rs:12; packages/blitz-dom/src/util.rs:166-178)
- The system color scheme is an enum `Light` (default) / `Dark`, set on the viewport, and it feeds `prefers-color-scheme` (packages/blitz-traits/src/shell.rs:68-74; tests/blitz-tests/tests/device_coalescing.rs:101; packages/blitz-dom/src/stylo_device.rs:80-83)
- Color-scheme changes trigger a full recascade because `light-dark()` and system colors resolve at cascade time (packages/blitz-dom/src/document.rs:2312-2319)
- The parent viewport's color scheme is copied to iframe sub-documents (packages/blitz-dom/src/resolve.rs:148-152)
- The legacy `bgcolor` attribute maps to `background-color` (packages/blitz-dom/src/stylo.rs:1186-1193)
- The frame clear color is an optional `base_color` (a `peniko::Color`), unset by default (packages/dioxus-native/src/config.rs:12; packages/dioxus-native/src/config.rs:40; packages/dioxus-native/src/config.rs:76-80; packages/dioxus-native/src/dioxus_renderer.rs:37-38)
- The canvas background is the root element's background colour, falling back to body's when the root is transparent (packages/blitz-paint/src/render.rs:193-228)
- WPT-rendered pages get a white background fill (wpt/runner/src/test_runners/ref_test.rs:316-323)

<!-- /detail -->

## Color Palette › ### Core Colors

<!-- detail: values · palette -->

| Role | Value | Usage |
|------|-------|-------|
| Browser chrome focus | `#5E9ED6` | Focus color; icon button hover `#CCC`, active `#BBB` (apps/browser/assets/browser.css:172-175; apps/browser/assets/browser.css:182-189) |
| Browser about:history link | `#1a73e8` | History link color (apps/browser/assets/about-history.css:3; apps/browser/assets/about-history.css:69; apps/browser/assets/about-history.css:77; apps/browser/assets/about-history.css:84; apps/browser/assets/about-newtab.css:7; apps/browser/assets/about-stub.css:2) |
| seven_guis stand | — | The stand's accent (its load page's `load-fetch` included), its flight booker's success-message colours and its flight-type select's border are held in layout-templates §Surface: desktop-native → Primary screens (`values · desktop-native-screens`), not as design-system values |
| blitz-dom default link | `rgb(0, 0, 238)` | Default stylesheet link, its rule selecting `a[href]` — an anchor with no `href` takes neither the colour nor the underline; input focus outline `#4D90FE`; button background `#EFEFEF` (packages/blitz-dom/assets/default.css:43-46; packages/blitz-dom/assets/default.css:98-101; packages/blitz-dom/assets/default.css:111) |
| Default text-selection highlight | rgb 180, 213, 255 | Engine default (packages/blitz-paint/src/lib.rs:28-29) |
| counter / transparent buttons | named green, red, blue | Buttons with white text (examples/counter/src/app.rs:83-111; examples/transparent/src/app.rs:184-212) |

<!-- /detail -->

## Color Palette › ### Surface Scale (elevation hierarchy)

<!-- detail: values · palette -->

| Level | Value | Usage |
|-------|-------|-------|
| Browser tabstrip | `#E0E0E0` | Tab `#D0D0D0`, tab hover `#E8E8E8`, active tab and urlbar `#F9F9F9` (apps/browser/assets/browser.css:25; apps/browser/assets/browser.css:44; apps/browser/assets/browser.css:51; apps/browser/assets/browser.css:77; apps/browser/assets/browser.css:134-135) |
| Browser tooltip / FPS overlay / status bar | `#fff` / `rgba(0, 0, 0, 0.7)` / `rgba(240, 240, 240, 0.95)` | Tooltip background; FPS overlay background behind `rgb(0, 255, 0)`; status bar background (apps/browser/assets/browser.css:61-62; apps/browser/assets/browser.css:267-268; apps/browser/assets/browser.css:292; apps/browser/assets/browser.css:303) |
| Browser about pages | `#f0f0f0` | About-page backgrounds (apps/browser/assets/about-history.css:3; apps/browser/assets/about-history.css:69; apps/browser/assets/about-history.css:77; apps/browser/assets/about-history.css:84; apps/browser/assets/about-newtab.css:7; apps/browser/assets/about-stub.css:2) |
| rdme page | `#0d1117` (dark) / white (light) | Page background per scheme (apps/readme/assets/blitz-markdown-overrides.css:11-27) |
| seven_guis / todomvc page | `#f5f5f5` | Page background (examples/seven_guis/src/tasks/counter.rs:49; examples/seven_guis/src/tasks/counter.rs:59-72; examples/seven_guis/src/app.rs:183; examples/todomvc/src/todomvc.css:26-27; examples/todomvc/src/todomvc.css:76) |
| wasm_hello | `#0f1226`, `#1a1d3a` | Dark palette (examples/wasm_hello/src/lib.rs:46; examples/wasm_hello/src/lib.rs:55; examples/wasm_hello/src/lib.rs:62; examples/wasm_hello/src/lib.rs:68) |
| custom-widget, static-HTML and wgpu_texture examples | `#f4e8d2` | Main background (examples/custom_widget.rs:187; examples/html.rs:16; examples/wgpu_texture/src/styles.css:15) |

<!-- /detail -->

## Color Palette › ### Text Hierarchy

<!-- detail: values · palette -->

| Level | Value | Usage |
|-------|-------|-------|
| Browser tooltip and status bar text | `#333` | (apps/browser/assets/browser.css:61-62; apps/browser/assets/browser.css:267-268; apps/browser/assets/browser.css:292; apps/browser/assets/browser.css:303) |
| Browser about-page muted text | `#888` / `#777` / `#aaa` | (apps/browser/assets/about-history.css:3; apps/browser/assets/about-history.css:69; apps/browser/assets/about-history.css:77; apps/browser/assets/about-history.css:84; apps/browser/assets/about-newtab.css:7; apps/browser/assets/about-stub.css:2) |
| Browser error page paragraph | `#666` | (apps/browser/assets/error.html:14) |
| seven_guis text | `#1a1a1a` | (examples/seven_guis/src/tasks/counter.rs:49; examples/seven_guis/src/tasks/counter.rs:59-72; examples/seven_guis/src/app.rs:183) |
| todomvc body / heading | `#4d4d4d` / `rgba(175, 47, 47, 1.0)` | Body text on `#f5f5f5`; heading (examples/todomvc/src/todomvc.css:26-27; examples/todomvc/src/todomvc.css:76) |
| wasm_hello heading / code | `#ffd166` / `#ff7b9c` | (examples/wasm_hello/src/lib.rs:46; examples/wasm_hello/src/lib.rs:55; examples/wasm_hello/src/lib.rs:62; examples/wasm_hello/src/lib.rs:68) |

<!-- /detail -->

## Color Palette › ### Semantic Colors

<!-- detail: values · status-states -->

| State | Values (as declared) | Usage |
|-------|----------------------|-------|
| seven_guis invalid | `#e53e3e` / `#fff5f5` / `#c53030` | Invalid state (examples/seven_guis/src/tasks/flight_booker.rs:194-198; examples/seven_guis/src/tasks/flight_booker.rs:229-237) |
| Disabled checkbox/radio | rgba 209, 209, 209, 255 | Disabled accent; otherwise the element's color stands in for accent-color per a TODO (packages/blitz-paint/src/render/form_controls.rs:21-26) |

<!-- /detail -->

<!-- detail: values · status-states -->

**Domain status colors** — WPT runner terminal result colors:
| Result | Terminal color |
|--------|----------------|
| PASS | green (wpt/runner/src/main.rs:391-401) |
| FAIL with some passing subtests | yellow (wpt/runner/src/main.rs:391-401) |
| FAIL | red (wpt/runner/src/main.rs:391-401) |
| TIMEOUT | bright red (wpt/runner/src/main.rs:391-401) |
| SKIP | bright black (wpt/runner/src/main.rs:391-401) |
| CRASH | bright magenta (wpt/runner/src/main.rs:391-401) |

<!-- /detail -->

## Color Palette › ### Border Progression

<!-- detail: values · palette -->

| Intensity | Value | Usage |
|-----------|-------|-------|
| Browser urlbar border | `#EEE` | (apps/browser/assets/browser.css:25; apps/browser/assets/browser.css:44; apps/browser/assets/browser.css:51; apps/browser/assets/browser.css:77; apps/browser/assets/browser.css:134-135) |
| Focus — browser chrome | `#5E9ED6` | (apps/browser/assets/browser.css:172-175; apps/browser/assets/browser.css:182-189) |
| Focus — blitz-dom default input outline | `#4D90FE` | (packages/blitz-dom/assets/default.css:43-46; packages/blitz-dom/assets/default.css:98-101; packages/blitz-dom/assets/default.css:111) |

<!-- /detail -->

<!-- detail: values · palette -->

- Default scrollbar thumbs: dark scheme rest 214,214,214,178 · hover 190,190,190,222 · active 172,172,172,255 · stroke 0,0,0,102; light scheme rest 128,128,128,178 · hover 152,152,152,222 · active 170,170,170,255 · stroke 255,255,255,102 (packages/blitz-paint/src/render.rs:757-776); the palette follows the viewport color scheme and paints as a fill plus a thin contrast stroke (tests/blitz-tests/tests/scrollbars.rs:204-211; tests/blitz-tests/tests/scrollbars.rs:194-202)
- `scrollbar-color` resolves to absolute thumb and track colors against the element's computed `color`, defaulting to `Auto` (packages/blitz-dom/src/node/scrollbar.rs:38-48; packages/blitz-dom/src/node/scrollbar.rs:58-74); author thumbs blend for contrast at 1.8 on hover and 1.3 when active (packages/blitz-paint/src/render.rs:778-780; packages/blitz-paint/src/render.rs:815-822), and a near-white author thumb darkens on hover/drag (tests/blitz-tests/tests/scrollbars.rs:125-160; tests/blitz-tests/tests/scrollbar_drag.rs:224-227)
- Unchecked radio rings use the CSS palette GRAY; checkbox ticks/gaps use white (packages/blitz-paint/src/render/form_controls.rs:76-79; packages/blitz-paint/src/render/form_controls.rs:99-101)
- Debug overlay: content blue 66,144,245,128 · padding green 81,144,66,128 · border red 245,66,66,128 · margin orange 249,204,157,128 (packages/blitz-paint/src/debug_overlay.rs:95; packages/blitz-paint/src/debug_overlay.rs:98; packages/blitz-paint/src/debug_overlay.rs:110; packages/blitz-paint/src/debug_overlay.rs:119)
- Devtools layout strokes are red for block/flow-root, green for flex, blue for grid (packages/blitz-paint/src/render.rs:1355-1362); devtools `show_layout` documents "different border colors" without stating values (packages/blitz-traits/src/devtools.rs:8-10)
- In SVG serialization, `currentColor` in attribute values is replaced with the element's computed `color` (packages/blitz-dom/src/node/serialize.rs:18-19; packages/blitz-dom/src/node/serialize.rs:120-125; packages/blitz-dom/src/node/serialize.rs:186-194)

<!-- /detail -->

<!-- detail: values · palette -->

- rdme uses GitHub markdown color tokens (`--fgColor-*`, `--bgColor-*`, `--borderColor-*`, syntax colors) defined separately for dark and light schemes (apps/readme/assets/github-markdown.css:13-124)

<!-- /detail -->

<!-- detail: absence · palette -->

- Observed absent — color tokens or CSS custom properties in the engine and integration crates · searched: `--[a-z]` over the 15 s05 files; `color|Color` and `--[a-z]+-|var\(` over the 8 s07 slice files; `--[a-z-]+:` over the 32 s10 slice files; `var\(--|--[a-z]+-` over the 21 listed s11 files

<!-- /detail -->

<!-- detail: values · palette -->

- clip-path fixture defines a 25-colour block palette commented "curated harmonious palette" (e.g. #e63946, #2a9d8f, #264653, #8338ec, #3a86ff) (examples/assets/clip-path.html:35-60)
- border-styles fixture uses border colours #d6336c, #1c7ed6, #2f9e44, #f59f00 and dark #1a1a1a on #fffbe6 / #f5f5f5 (examples/assets/border-styles.html:7; examples/assets/border-styles.html:27-29; examples/assets/border-styles.html:77; examples/assets/border-styles.html:82)
- A reduced GitHub fixture sets `--borderColor-default: green` and consumes it via `var()` (examples/assets/github_profile_reduced.html:6-13)
- The BBC fixture uses relative colour syntax `color(from #202224 srgb r g b / …)` (examples/assets/bbc_reduced.html:13-16)
- graphite defines 16 color custom properties on :root, among them --color-navy #16323f, --color-crimson #803847, --color-fog #eee and --color-seaside-rgb 176, 214, 203, repeated in its subsets; page text is var(--color-navy) on #fff and links var(--color-crimson) (examples/assets/graphite.html:890-906; examples/assets/graphite_blog_section.html:890-906; examples/assets/graphite_software_overview.html:888-904; examples/assets/graphite.html:918-922; examples/assets/graphite.html:1405-1407)
- google sets body/input/button text #202124 on #fff, links #1a0dab, visited #681da8, and shows a "Dark theme: Off" menu item (examples/assets/google.html:19-36; examples/assets/google.html:3788); observed absent — CSS custom properties in the google fixture · searched: `var\(--|--[a-z]+:` over examples/assets/google.html
- gosub is dark: #ffffff text on #121212, cyan links with a 2px solid #6e40c9 bottom border, h1 gradient rgba(96,243,236,1) to rgba(30,84,231,1) (examples/assets/gosub.html:7-9; examples/assets/gosub.html:45-50; examples/assets/gosub.html:74-80)
- hr assigns a distinct hex per case, e.g. #e63946, #2a9d8f, #457b9d, #8338ec (examples/assets/hr.html:43; examples/assets/hr.html:50; examples/assets/hr.html:72; examples/assets/hr.html:78)
- inline-backgrounds uses pastel span backgrounds and one rgba(214, 51, 108, 0.35) background (examples/assets/inline-backgrounds.html:35-53)

<!-- /detail -->

<!-- detail: absence · palette -->

- Observed absent — a prefers-color-scheme rule in the s02 fixtures · searched: `prefers-color-scheme` over the 21 s02 files

<!-- /detail -->

<!-- detail: values · palette -->

- The inline logo SVG fills circles with rgb(1,99,63), rgb(0,118,114), rgb(62,149,147), rgb(252,176,64), rgb(233,86,41), rgb(230,29,50) and a bolt with rgb(244,232,210) (examples/html.rs:31-36; examples/html.rs:39)
- servo.css: page `#121619`, body text `hsl(0, 0%, 96%)`, links `#1191E8`, link hover `#42BF64`, code `hsl(348, 86%, 46%)`; its only custom property is `--columnGap` (examples/assets/servo.css:198-199; examples/assets/servo.css:236-237; examples/assets/servo.css:243-253; examples/assets/servo.css:255-257; examples/assets/servo.css:1577-1588)
- servo.html hero accents `#4fc066`, `#209e9b`, `#f03278`, `#f68243`, `#faae30`, `#712b97` (examples/assets/servo.html:184-191)
- Preact TodoMVC page: background `#f5f5f5`, text `#111`, heading `#b83f45`, muted `#949494`, footer `#777` (examples/preact/index.html:13-14; examples/preact/index.html:17; examples/preact/index.html:27; examples/preact/index.html:34); reference page tags preact `#673ab8`, react `#087ea4`, wpt `#9a6700`, extra `#888` (examples/preact/core_dom_apis.html:44-47)
- The transforms example `.button` uses `#e74310` with white text (examples/transforms.rs:304-305)

<!-- /detail -->
