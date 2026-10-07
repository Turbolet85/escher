## Brand Identity

> NO RECORDED INTENT

---

## Color Palette

**Color model and scheme (engine):**
- The `Color` type is `AlphaColor<Srgb>` and Stylo colors convert to it via sRGB (packages/blitz-dom/src/util.rs:12; packages/blitz-dom/src/util.rs:166-178)
- The system color scheme is an enum `Light` (default) / `Dark`, set on the viewport, and it feeds `prefers-color-scheme` (packages/blitz-traits/src/shell.rs:68-74; tests/blitz-tests/tests/device_coalescing.rs:101; packages/blitz-dom/src/stylo_device.rs:80-83)
- Color-scheme changes trigger a full recascade because `light-dark()` and system colors resolve at cascade time (packages/blitz-dom/src/document.rs:2089-2096)
- The parent viewport's color scheme is copied to iframe sub-documents (packages/blitz-dom/src/resolve.rs:146-150)
- The legacy `bgcolor` attribute maps to `background-color` (packages/blitz-dom/src/stylo.rs:1182-1189)
- The frame clear color is an optional `base_color` (a `peniko::Color`), unset by default (packages/dioxus-native/src/config.rs:12; packages/dioxus-native/src/config.rs:40; packages/dioxus-native/src/config.rs:76-80; packages/dioxus-native/src/dioxus_renderer.rs:37-38)
- The canvas background is the root element's background colour, falling back to body's when the root is transparent (packages/blitz-paint/src/render.rs:193-228)
- WPT-rendered pages get a white background fill (wpt/runner/src/test_runners/ref_test.rs:305-312)

### Core Colors
| Role | Value | Usage |
|------|-------|-------|
| Browser chrome focus | `#5E9ED6` | Focus color; icon button hover `#CCC`, active `#BBB` (apps/browser/assets/browser.css:172-175; apps/browser/assets/browser.css:182-189) |
| Browser about:history link | `#1a73e8` | History link color (apps/browser/assets/about-history.css:3; apps/browser/assets/about-history.css:69; apps/browser/assets/about-history.css:77; apps/browser/assets/about-history.css:84; apps/browser/assets/about-newtab.css:7; apps/browser/assets/about-stub.css:2) |
| seven_guis accent | `#4a6cf7` (hover `#3a5ce5`, active `#2a4cd3`) | Accent on page `#f5f5f5` with text `#1a1a1a` (examples/seven_guis/src/tasks/counter.rs:49; examples/seven_guis/src/tasks/counter.rs:59-72; examples/seven_guis/src/app.rs:182) |
| blitz-dom default link | `rgb(0, 0, 238)` | Default stylesheet link; input focus outline `#4D90FE`; button background `#EFEFEF` (packages/blitz-dom/assets/default.css:42-45; packages/blitz-dom/assets/default.css:92-95; packages/blitz-dom/assets/default.css:105) |
| Default text-selection highlight | rgb 180, 213, 255 | Engine default (packages/blitz-paint/src/lib.rs:28-29) |
| counter / transparent buttons | named green, red, blue | Buttons with white text (examples/counter/src/app.rs:83-111; examples/transparent/src/app.rs:184-212) |

### Surface Scale (elevation hierarchy)
| Level | Value | Usage |
|-------|-------|-------|
| Browser tabstrip | `#E0E0E0` | Tab `#D0D0D0`, tab hover `#E8E8E8`, active tab and urlbar `#F9F9F9` (apps/browser/assets/browser.css:25; apps/browser/assets/browser.css:44; apps/browser/assets/browser.css:51; apps/browser/assets/browser.css:77; apps/browser/assets/browser.css:134-135) |
| Browser tooltip / FPS overlay / status bar | `#fff` / `rgba(0, 0, 0, 0.7)` / `rgba(240, 240, 240, 0.95)` | Tooltip background; FPS overlay background behind `rgb(0, 255, 0)`; status bar background (apps/browser/assets/browser.css:61-62; apps/browser/assets/browser.css:267-268; apps/browser/assets/browser.css:292; apps/browser/assets/browser.css:303) |
| Browser about pages | `#f0f0f0` | About-page backgrounds (apps/browser/assets/about-history.css:3; apps/browser/assets/about-history.css:69; apps/browser/assets/about-history.css:77; apps/browser/assets/about-history.css:84; apps/browser/assets/about-newtab.css:7; apps/browser/assets/about-stub.css:2) |
| rdme page | `#0d1117` (dark) / white (light) | Page background per scheme (apps/readme/assets/blitz-markdown-overrides.css:11-27) |
| seven_guis / todomvc page | `#f5f5f5` | Page background (examples/seven_guis/src/tasks/counter.rs:49; examples/seven_guis/src/tasks/counter.rs:59-72; examples/seven_guis/src/app.rs:182; examples/todomvc/src/todomvc.css:26-27; examples/todomvc/src/todomvc.css:76) |
| wasm_hello | `#0f1226`, `#1a1d3a` | Dark palette (examples/wasm_hello/src/lib.rs:46; examples/wasm_hello/src/lib.rs:55; examples/wasm_hello/src/lib.rs:62; examples/wasm_hello/src/lib.rs:68) |
| custom-widget, static-HTML and wgpu_texture examples | `#f4e8d2` | Main background (examples/custom_widget.rs:187; examples/html.rs:16; examples/wgpu_texture/src/styles.css:15) |

### Text Hierarchy
| Level | Value | Usage |
|-------|-------|-------|
| Browser tooltip and status bar text | `#333` | (apps/browser/assets/browser.css:61-62; apps/browser/assets/browser.css:267-268; apps/browser/assets/browser.css:292; apps/browser/assets/browser.css:303) |
| Browser about-page muted text | `#888` / `#777` / `#aaa` | (apps/browser/assets/about-history.css:3; apps/browser/assets/about-history.css:69; apps/browser/assets/about-history.css:77; apps/browser/assets/about-history.css:84; apps/browser/assets/about-newtab.css:7; apps/browser/assets/about-stub.css:2) |
| Browser error page paragraph | `#666` | (apps/browser/assets/error.html:14) |
| seven_guis text | `#1a1a1a` | (examples/seven_guis/src/tasks/counter.rs:49; examples/seven_guis/src/tasks/counter.rs:59-72; examples/seven_guis/src/app.rs:182) |
| todomvc body / heading | `#4d4d4d` / `rgba(175, 47, 47, 1.0)` | Body text on `#f5f5f5`; heading (examples/todomvc/src/todomvc.css:26-27; examples/todomvc/src/todomvc.css:76) |
| wasm_hello heading / code | `#ffd166` / `#ff7b9c` | (examples/wasm_hello/src/lib.rs:46; examples/wasm_hello/src/lib.rs:55; examples/wasm_hello/src/lib.rs:62; examples/wasm_hello/src/lib.rs:68) |

### Semantic Colors
| State | Values (as declared) | Usage |
|-------|----------------------|-------|
| seven_guis invalid | `#e53e3e` / `#fff5f5` / `#c53030` | Invalid state (examples/seven_guis/src/tasks/flight_booker.rs:193-197; examples/seven_guis/src/tasks/flight_booker.rs:228-236) |
| seven_guis success | `#ebf8ee` / `#68d391` / `#276749` | Success message (examples/seven_guis/src/tasks/flight_booker.rs:193-197; examples/seven_guis/src/tasks/flight_booker.rs:228-236) |
| Disabled checkbox/radio | rgba 209, 209, 209, 255 | Disabled accent; otherwise the element's color stands in for accent-color per a TODO (packages/blitz-paint/src/render/form_controls.rs:21-26) |

**Domain status colors** — WPT runner terminal result colors:
| Result | Terminal color |
|--------|----------------|
| PASS | green (wpt/runner/src/main.rs:382-392) |
| FAIL with some passing subtests | yellow (wpt/runner/src/main.rs:382-392) |
| FAIL | red (wpt/runner/src/main.rs:382-392) |
| TIMEOUT | bright red (wpt/runner/src/main.rs:382-392) |
| SKIP | bright black (wpt/runner/src/main.rs:382-392) |
| CRASH | bright magenta (wpt/runner/src/main.rs:382-392) |

### Border Progression
| Intensity | Value | Usage |
|-----------|-------|-------|
| Browser urlbar border | `#EEE` | (apps/browser/assets/browser.css:25; apps/browser/assets/browser.css:44; apps/browser/assets/browser.css:51; apps/browser/assets/browser.css:77; apps/browser/assets/browser.css:134-135) |
| Focus — browser chrome | `#5E9ED6` | (apps/browser/assets/browser.css:172-175; apps/browser/assets/browser.css:182-189) |
| Focus — blitz-dom default input outline | `#4D90FE` | (packages/blitz-dom/assets/default.css:42-45; packages/blitz-dom/assets/default.css:92-95; packages/blitz-dom/assets/default.css:105) |

**Engine-painted defaults:**
- Default scrollbar thumbs: dark scheme rest 214,214,214,178 · hover 190,190,190,222 · active 172,172,172,255 · stroke 0,0,0,102; light scheme rest 128,128,128,178 · hover 152,152,152,222 · active 170,170,170,255 · stroke 255,255,255,102 (packages/blitz-paint/src/render.rs:747-766); the palette follows the viewport color scheme and paints as a fill plus a thin contrast stroke (tests/blitz-tests/tests/scrollbars.rs:204-211; tests/blitz-tests/tests/scrollbars.rs:194-202)
- `scrollbar-color` resolves to absolute thumb and track colors against the element's computed `color`, defaulting to `Auto` (packages/blitz-dom/src/node/scrollbar.rs:38-48; packages/blitz-dom/src/node/scrollbar.rs:58-74); author thumbs blend for contrast at 1.8 on hover and 1.3 when active (packages/blitz-paint/src/render.rs:768-770; packages/blitz-paint/src/render.rs:805-812), and a near-white author thumb darkens on hover/drag (tests/blitz-tests/tests/scrollbars.rs:125-160; tests/blitz-tests/tests/scrollbar_drag.rs:224-227)
- Unchecked radio rings use the CSS palette GRAY; checkbox ticks/gaps use white (packages/blitz-paint/src/render/form_controls.rs:76-79; packages/blitz-paint/src/render/form_controls.rs:99-101)
- Debug overlay: content blue 66,144,245,128 · padding green 81,144,66,128 · border red 245,66,66,128 · margin orange 249,204,157,128 (packages/blitz-paint/src/debug_overlay.rs:95; packages/blitz-paint/src/debug_overlay.rs:98; packages/blitz-paint/src/debug_overlay.rs:110; packages/blitz-paint/src/debug_overlay.rs:119)
- Devtools layout strokes are red for block/flow-root, green for flex, blue for grid (packages/blitz-paint/src/render.rs:1213-1220); devtools `show_layout` documents "different border colors" without stating values (packages/blitz-traits/src/devtools.rs:8-10)
- In SVG serialization, `currentColor` in attribute values is replaced with the element's computed `color` (packages/blitz-dom/src/node/serialize.rs:18-19; packages/blitz-dom/src/node/serialize.rs:120-125; packages/blitz-dom/src/node/serialize.rs:186-194)

**Token sets:**
- rdme uses GitHub markdown color tokens (`--fgColor-*`, `--bgColor-*`, `--borderColor-*`, syntax colors) defined separately for dark and light schemes (apps/readme/assets/github-markdown.css:13-124)
- Observed absent — color tokens or CSS custom properties in the engine and integration crates · searched: `--[a-z]` over the 15 s05 files; `color|Color` and `--[a-z]+-|var\(` over the 8 s07 slice files; `--[a-z-]+:` over the 32 s10 slice files; `var\(--|--[a-z]+-` over the 21 listed s11 files

**Example fixtures:**
- clip-path fixture defines a 25-colour block palette commented "curated harmonious palette" (e.g. #e63946, #2a9d8f, #264653, #8338ec, #3a86ff) (examples/assets/clip-path.html:35-60)
- border-styles fixture uses border colours #d6336c, #1c7ed6, #2f9e44, #f59f00 and dark #1a1a1a on #fffbe6 / #f5f5f5 (examples/assets/border-styles.html:7; examples/assets/border-styles.html:27-29; examples/assets/border-styles.html:77; examples/assets/border-styles.html:82)
- A reduced GitHub fixture sets `--borderColor-default: green` and consumes it via `var()` (examples/assets/github_profile_reduced.html:6-13)
- The BBC fixture uses relative colour syntax `color(from #202224 srgb r g b / …)` (examples/assets/bbc_reduced.html:13-16)
- graphite defines 16 color custom properties on :root, among them --color-navy #16323f, --color-crimson #803847, --color-fog #eee and --color-seaside-rgb 176, 214, 203, repeated in its subsets; page text is var(--color-navy) on #fff and links var(--color-crimson) (examples/assets/graphite.html:890-906; examples/assets/graphite_blog_section.html:890-906; examples/assets/graphite_software_overview.html:888-904; examples/assets/graphite.html:918-922; examples/assets/graphite.html:1405-1407)
- google sets body/input/button text #202124 on #fff, links #1a0dab, visited #681da8, and shows a "Dark theme: Off" menu item (examples/assets/google.html:19-36; examples/assets/google.html:3788); observed absent — CSS custom properties in the google fixture · searched: `var\(--|--[a-z]+:` over examples/assets/google.html
- gosub is dark: #ffffff text on #121212, cyan links with a 2px solid #6e40c9 bottom border, h1 gradient rgba(96,243,236,1) to rgba(30,84,231,1) (examples/assets/gosub.html:7-9; examples/assets/gosub.html:45-50; examples/assets/gosub.html:74-80)
- hr assigns a distinct hex per case, e.g. #e63946, #2a9d8f, #457b9d, #8338ec (examples/assets/hr.html:43; examples/assets/hr.html:50; examples/assets/hr.html:72; examples/assets/hr.html:78)
- inline-backgrounds uses pastel span backgrounds and one rgba(214, 51, 108, 0.35) background (examples/assets/inline-backgrounds.html:35-53)
- Observed absent — a prefers-color-scheme rule in the s02 fixtures · searched: `prefers-color-scheme` over the 21 s02 files
- The inline logo SVG fills circles with rgb(1,99,63), rgb(0,118,114), rgb(62,149,147), rgb(252,176,64), rgb(233,86,41), rgb(230,29,50) and a bolt with rgb(244,232,210) (examples/html.rs:31-36; examples/html.rs:39)
- servo.css: page `#121619`, body text `hsl(0, 0%, 96%)`, links `#1191E8`, link hover `#42BF64`, code `hsl(348, 86%, 46%)`; its only custom property is `--columnGap` (examples/assets/servo.css:198-199; examples/assets/servo.css:236-237; examples/assets/servo.css:243-253; examples/assets/servo.css:255-257; examples/assets/servo.css:1577-1588)
- servo.html hero accents `#4fc066`, `#209e9b`, `#f03278`, `#f68243`, `#faae30`, `#712b97` (examples/assets/servo.html:184-191)
- Preact TodoMVC page: background `#f5f5f5`, text `#111`, heading `#b83f45`, muted `#949494`, footer `#777` (examples/preact/index.html:13-14; examples/preact/index.html:17; examples/preact/index.html:27; examples/preact/index.html:34); reference page tags preact `#673ab8`, react `#087ea4`, wpt `#9a6700`, extra `#888` (examples/preact/core_dom_apis.html:44-47)
- The transforms example `.button` uses `#e74310` with white text (examples/transforms.rs:304-305)

> NOT YET MEASURED — a project-wide UI colour token set: the examples slice (s01) and the DOM-node slice (s08) recorded the project's own colour tokens as out of slice, and the default user-agent stylesheet's full contents were out of slice for s05.

---

## Typography

| Role | Font | Weight | Size | Tracking | Usage |
|------|------|--------|------|----------|-------|
| Browser chrome text | sans-serif | not stated | tab title 12px, urlbar 14px (16px on mobile), menu items 14px, status bar 12px | not stated | (apps/browser/assets/browser.css:93-94; apps/browser/assets/browser.css:157-160; apps/browser/assets/browser.css:170; apps/browser/assets/browser.css:225-226; apps/browser/assets/browser.css:297-298) |
| Browser suggestion section header | not stated | 600 | 11px | letter-spacing 0.04em, uppercase | (apps/browser/assets/browser.css:329-336) |
| Browser FPS overlay | monospace | not stated | 12px | not stated | (apps/browser/assets/browser.css:269-270) |
| about:history | `system-ui, sans-serif` | heading 600 | heading 24px | not stated | (apps/browser/assets/about-history.css:7; apps/browser/assets/about-history.css:9-11) |
| rdme body | stack beginning `-apple-system,BlinkMacSystemFont,"Segoe UI"`; monospace stack in `--fontStack-monospace` | weights in `--base-text-weight-*` | 16px / line-height 1.5 | not stated | (apps/readme/assets/github-markdown.css:7-10; apps/readme/assets/github-markdown.css:132-134) |
| blitz-dom default form controls | inputs/selects/buttons `system-ui, sans-serif`; textarea monospace | not stated | not stated | not stated | (packages/blitz-dom/assets/default.css:82-90; packages/blitz-dom/assets/default.css:301-347; packages/blitz-dom/assets/default.css:358-366; packages/blitz-dom/assets/default.css:662-667) |
| blitz-dom default headings | not stated | bold | h1–h6 from 2em down to 0.67em | not stated | (packages/blitz-dom/assets/default.css:82-90; packages/blitz-dom/assets/default.css:301-347; packages/blitz-dom/assets/default.css:358-366; packages/blitz-dom/assets/default.css:662-667) |
| blitz-dom default code/pre | `-moz-fixed` | not stated | not stated | not stated | (packages/blitz-dom/assets/default.css:82-90; packages/blitz-dom/assets/default.css:301-347; packages/blitz-dom/assets/default.css:358-366; packages/blitz-dom/assets/default.css:662-667) |
| seven_guis | sans-serif | home title 700 | base 14px; home title 36px | not stated | (examples/seven_guis/src/app.rs:178-204) |
| todomvc | `'Helvetica Neue', Helvetica, Arial, sans-serif` | body 300; h1 100 | body 14px; h1 100px | not stated | (examples/todomvc/src/todomvc.css:24; examples/todomvc/src/todomvc.css:33; examples/todomvc/src/todomvc.css:73-74) |
| wgpu_texture | `system-ui, sans` | not stated | not stated | not stated | (examples/wgpu_texture/src/styles.css:7) |
| Text input editors | not stated | not stated | `parley::PlainEditor::new(16.0)` | not stated | (packages/blitz-dom/src/node/text.rs:82) |
| Engine generic base size | generics | not stated | monospace 13px; other generics 16px | not stated | (packages/blitz-dom/src/font_metrics.rs:167-176) |
| List bullets | `"Bullet, monospace, sans-serif"` | not stated | not stated | not stated | (packages/blitz-dom/src/layout/list.rs:15; packages/blitz-dom/src/layout/list.rs:160-167; packages/blitz-dom/src/layout/construct.rs:1086-1092) |

**Loading:**
- WASM builds register bundled DejaVu Sans for sans-serif, serif, monospace and system-ui (examples/wasm_hello/src/lib.rs:75-100)
- The seven_guis headless stand registers the same bundled DejaVu Sans for every generic, with system fonts off, on native — `build_single_font_ctx(DEJAVU_SANS)` through `HarnessOptions.font_ctx`, decoded by seven_guis' native `woff` feature (examples/seven_guis/src/stand.rs:52-54; examples/seven_guis/src/lib.rs:7; examples/seven_guis/Cargo.toml:31)
- A custom `FontContext` can be set; on WASM a context with bundled fonts must be provided, using `build_single_font_ctx` for one font (packages/dioxus-native/src/config.rs:56-64); `build_single_font_ctx` registers one font as fallback for SansSerif, Serif, Monospace and SystemUi with system fonts disabled (packages/blitz-dom/src/lib.rs:126-157)
- Font features: system-fonts, woff, complex-scripts (dictionary line-breaking), font-embolden and apple-font-embolden (packages/dioxus-native/Cargo.toml:20-25; packages/dioxus-native/Cargo.toml:45-46)
- A bullet font is always registered in the default font context (packages/blitz-dom/src/document.rs:400-402; packages/blitz-dom/src/lib.rs:32)
- `document.fonts` is a stub FontFaceSet where all fonts report as loaded (packages/blitz-vibey-script/src/runtime.rs:1060-1074)
- `blitz-dom` is built with the `system-fonts` feature for the blitz-tests crate; without it text measures 0x0 and font-dependent assertions pass vacuously, and the feature is stated to be enabled by default when testing the whole workspace (tests/blitz-tests/Cargo.toml:17; tests/blitz-tests/tests/br_trailing_line.rs:11-13; tests/blitz-tests/tests/inline_box_baseline.rs:4-6)

**Engine text mapping:**
- Generic font families map to Parley generics, with `None` as sans-serif (packages/blitz-dom/src/stylo_to_parley.rs:50-60)
- Font weight, width, style and variation settings convert to Parley values (packages/blitz-dom/src/stylo_to_parley.rs:87-112)
- `font-variant-ligatures`, `-caps`, `-position`, `-numeric` and `-east-asian` map to OpenType feature tags (packages/blitz-dom/src/stylo_to_parley.rs:134-262)
- Line height maps `normal`, number and length to Parley line heights (packages/blitz-dom/src/stylo_to_parley.rs:407-412); the layout engine resolves `line-height: normal` as 1.2 × font size (packages/blitz-dom/src/layout/mod.rs:116-120)
- Letter and word spacing resolve against the font size (packages/blitz-dom/src/stylo_to_parley.rs:415-424)
- Text locale comes from the computed `_x_lang` value, which the `lang` attribute sets (packages/blitz-dom/src/stylo_to_parley.rs:495; packages/blitz-dom/src/stylo.rs:1197-1199)
- `text-align` and `text-align-last` map to Parley alignment (packages/blitz-dom/src/stylo_to_parley.rs:304-329); word break, line break, overflow wrap, wrap mode and white-space collapse map to Parley settings (packages/blitz-dom/src/stylo_to_parley.rs:331-346; packages/blitz-dom/src/stylo_to_parley.rs:467-484)
- `ch` and `ic` metrics measure `'0'` and `'\u{6C34}'` advances scaled like Parley's shaped glyph advances (packages/blitz-dom/src/font_metrics.rs:104-161)
- The new stylist device is seeded with the root element's font size and line height so rem/rlh units do not fall back to the 16px default (packages/blitz-dom/src/document.rs:2099-2120); `rem` resolves against the root font-size after viewport and hidpi changes, with an initial root font-size of 16px (tests/blitz-tests/tests/rem_after_viewport_change.rs:1-8; tests/blitz-tests/tests/rem_after_viewport_change.rs:49-77)
- List markers: decimal `"N. "`, lower/upper-alpha, disc `•`, circle `◦`, square `▪`, disclosure-open `▾`, disclosure-closed `▸`, other names `□` (packages/blitz-dom/src/layout/list.rs:116-158)
- Font emboldening is enabled by the font-embolden feature, or apple-font-embolden on macOS and iOS (packages/blitz-paint/Cargo.toml:18-19; packages/blitz-paint/src/lib.rs:22-26); strength is 0.015125 and 0.0121 times the CSS font size, each capped at 0.3, with hinting off when emboldening (packages/blitz-paint/src/text.rs:631-642)
- Auto text-decoration thickness is font-size / 10 with a 1px minimum, floored to whole device pixels (packages/blitz-paint/src/text.rs:249-272); overline and line-through positions use the font's OS/2 usWinAscent, cached per font face (packages/blitz-paint/src/text.rs:70-95; packages/blitz-paint/src/text.rs:485-535)

**Example fixtures:**
- `font-family: sans-serif` for body and `monospace` for code labels (examples/assets/cursor.html:6; examples/assets/cursor.html:46; examples/assets/border-styles.html:6; examples/assets/border-styles.html:38); the Google fixtures set 14px `arial,sans-serif` (examples/assets/bottom_only.html:8; examples/assets/google.html:22-26), the bar 13px/27px Roboto (examples/assets/google.html:45-48) and Google Sans,Roboto,Helvetica,Arial,sans-serif stacks (examples/assets/google.html:563; examples/assets/google.html:1050)
- graphite: html/body Inter Variable 18px weight 500 line-height 1.5, 16px at width<=780px; h1 Bona Nova, Palatino, serif 2.66667rem weight 700, h2 1.75rem, h3 1.25rem, h4-h6 1rem Inter Variable weight 800; `--font-size-link` calc(1rem*4/3); Inter Variable and Bona Nova @font-face rules; `--nav-font-size` from 28px to 8px across breakpoints (examples/assets/graphite.html:918-930; examples/assets/graphite.html:940-943; examples/assets/graphite.html:1322-1330; examples/assets/graphite.html:1341-1366; examples/assets/graphite.html:915; examples/assets/graphite.html:1635-1758; examples/assets/graphite.html:1761-1822; examples/assets/graphite.html:986-995; examples/assets/graphite.html:1149-1156)
- gosub `"Arial", sans-serif` with h1 at 6em overridden to 72px (examples/assets/gosub.html:10; examples/assets/gosub.html:20-21; examples/assets/gosub.html:74-75); servo-new-reduced-1 `'Space Grotesk', sans-serif` (examples/assets/servo-new-reduced-1.html:13); pseudo `.qqq` Font Awesome 6 Brands glyphs at 32px (examples/assets/pseudo.html:21-25; examples/assets/pseudo.html:80)
- servo.css headings "Fira Sans", code/pre "Fira Mono", body stack "Fira Sans" … sans-serif weight 400 line-height 1.5, loaded from Google Fonts by servo.html (examples/assets/servo.css:1-23; examples/assets/servo.css:229-234; examples/assets/servo.css:220-227; examples/assets/servo.css:236-241; examples/assets/servo.html:27)
- custom-widget `system-ui, sans` (examples/custom_widget.rs:179); Preact TodoMVC `16px/1.4 system-ui, -apple-system, "Segoe UI", Roboto, sans-serif` with a 64px weight-200 heading (examples/preact/index.html:12; examples/preact/index.html:17); reference page `15px/1.55` system stack and ui-monospace code (examples/preact/core_dom_apis.html:10; examples/preact/core_dom_apis.html:23)
- text-decoration fixture exercises text-decoration-line/thickness/style, text-underline-offset/position and text-decoration-inset (examples/assets/text-decoration.html:48-102)

**Observed absent:**
- font-family declarations · searched: `font-family|font_family` over the 16 listed s08 files (no match)
- font family declarations · searched: `font_family|font-family|FontFamily` over the 32 listed s09 files
- font-family declarations outside test fixtures · searched: `font-family|font_family` over the 32 s10 slice files (matches only packages/blitz-vibey-script/tests/dom.rs)
- a project type scale or font tokens · searched: `font-family` over the 61 s12 slice files (one fixture hit, `sans-serif`)

> NOT YET MEASURED — a project-wide type scale: the examples slice (s01) recorded the project's own type scale as out of slice, and the WPT runner slice (s13) recorded typography as out of slice.

---

## Spacing

| Token | Value | Usage |
|-------|-------|-------|
| `--base-size-4/8/16/24/40` | 0.25–2.5rem | rdme spacing tokens (apps/readme/assets/github-markdown.css:2-6) |

**Untokenized values in the apps and examples:**
- Browser urlbar padding 6px, gap 6px; tab padding 0 8px; menu padding 8px with items 8px 12px gap 8px; suggestion rows 6px 12px (apps/browser/assets/browser.css:39; apps/browser/assets/browser.css:132-133; apps/browser/assets/browser.css:214; apps/browser/assets/browser.css:223; apps/browser/assets/browser.css:231; apps/browser/assets/browser.css:338)
- about:history padding 32px 48px; list items 12px 16px with gap 12px and margin-bottom 8px (apps/browser/assets/about-history.css:4; apps/browser/assets/about-history.css:49-54)
- rdme markdown body max-width 892px with padding 16px 32px (apps/readme/assets/blitz-markdown-overrides.css:1-5)
- seven_guis cards pad 24px 32px with 16px gap; home pads 48px 32px 64px (examples/seven_guis/src/tasks/counter.rs:38-42; examples/seven_guis/src/app.rs:190)
- blitz-dom default body margin 8px (packages/blitz-dom/assets/default.css:264-267)

**Engine spacing:**
- `hspace` and `vspace` map to horizontal and vertical margins on `embed`, `img`, `object`, `marquee` and image inputs (packages/blitz-dom/src/stylo.rs:1073-1099)
- Body `marginwidth`, `marginheight`, `leftmargin` and `topmargin` map to pixel margins; `rightmargin` and `bottommargin` are deliberately ignored (packages/blitz-dom/src/stylo.rs:1143-1180)
- Overlay scrollbar thumb geometry: thickness 10.0, thin thickness 6.0, margin 2.0 and minimum length 32.0 CSS px (packages/blitz-dom/src/node/scrollbar.rs:122-126)
- Single-line text inputs are vertically centered within their content box (packages/blitz-dom/src/node/node.rs:807-825)
- Outside list markers using a character are padded 8 CSS px from the item's border box (packages/blitz-paint/src/render.rs:963-974)
- The double text decoration places its second line thickness + 1 CSS px away (packages/blitz-paint/src/text.rs:318-328)

**Example fixtures:**
- Grid/flex gaps of 24px and 8px and 24px body padding (examples/assets/border-styles.html:9; examples/assets/border-styles.html:17; examples/assets/cursor.html:9; examples/assets/cursor.html:23)
- graphite :root sets --max-width 1200px, --max-extended-width 1600px, --max-width-reading-material 800px, --variable-px Min(1px, .15vw), --page-edge-padding 40px, --border-thickness 2px and --feature-box-padding 80, lowered at width<=780px and for print or width<=500px; sibling main sections spaced by calc(120*var(--variable-px)) (examples/assets/graphite.html:907-914; examples/assets/graphite.html:932-938; examples/assets/graphite.html:946-951; examples/assets/graphite.html:1203-1205)
- hr gives hr a 24px vertical margin and the body 16px 40px 40px padding (examples/assets/hr.html:8; examples/assets/hr.html:25-29)
- servo.css `--columnGap: 0.75rem` as column padding and negative margin; `.container.is-fluid` 32px side padding; `.button` padding `calc(0.5em - 1px)` / `calc(0.75em - 1px)` (examples/assets/servo.css:1577-1585; examples/assets/servo.css:342-345; examples/assets/servo.css:38-41)
- Preact TodoMVC `.app` max-width 520px with 40px auto margin, list items 14px 16px with 12px gap; reference page body max-width 60rem, padding 0 1.25rem (examples/preact/index.html:16; examples/preact/index.html:24; examples/preact/core_dom_apis.html:11-13)

> NOT YET MEASURED — a project-wide spacing scale or base unit: the examples, engine, test and WPT slices (s01, s05, s07, s10, s11, s12, s13) recorded a spacing scale as out of slice.

---

## Depth Strategy

**Chosen approach:**
> NO RECORDED INTENT

**Observed values in the apps and examples:**
- Browser z-index: tooltip, menu and suggestions 100; FPS overlay 50; status bar 10 (apps/browser/assets/browser.css:69; apps/browser/assets/browser.css:218; apps/browser/assets/browser.css:274; apps/browser/assets/browser.css:304; apps/browser/assets/browser.css:324)
- Browser menu and suggestions share `box-shadow: 0 2px 8px rgba(0, 0, 0, 0.15)` (apps/browser/assets/browser.css:217; apps/browser/assets/browser.css:323)
- seven_guis cards use `0 2px 8px rgba(0, 0, 0, 0.08)`; the circle dialog uses 0.10 (examples/seven_guis/src/tasks/counter.rs:43; examples/seven_guis/src/tasks/timer.rs:76; examples/seven_guis/src/tasks/flight_booker.rs:136; examples/seven_guis/src/tasks/circle_drawer.rs:205)
- todomvc uses layered shadows on the app and footer (examples/todomvc/src/todomvc.css:48; examples/todomvc/src/todomvc.css:279)
- wgpu_texture layers overlay z-index 10, underlay -10, header 100 (examples/wgpu_texture/src/styles.css:26; examples/wgpu_texture/src/styles.css:34; examples/wgpu_texture/src/styles.css:45); the custom-widget example stacks header 100, overlay 10, underlay -10 (examples/custom_widget.rs:195-198; examples/custom_widget.rs:201-206; examples/custom_widget.rs:213-217); the transforms example's `.overlay` uses z-index 99 (examples/transforms.rs:277-282)
- blitz-dom default dialog backdrop `rgba(0, 0, 0, 0.1)` (packages/blitz-dom/assets/default.css:989-992)

**Engine depth behavior:**
- Paint children and stacking contexts are built after layout and transforms; `StackingContext` and `HoistedPaintChild` are public re-exports (packages/blitz-dom/src/resolve.rs:119-123; packages/blitz-dom/src/lib.rs:86)
- A node is a stacking-context root for opacity not equal to 1, fixed or sticky position, z-index on relative or absolute (or static flex/grid items), any transform/rotate/scale/translate, atomic paint effects, or `isolation: isolate` (packages/blitz-dom/src/node/node.rs:1203-1246); atomic paint effects are opacity, filter, clip-path and mask-image (packages/blitz-dom/src/node/node.rs:1248-1277)
- Children paint in stacking order: negative z-index hoisted children, regular paint children, then positive z-index hoisted children (packages/blitz-paint/src/render.rs:1012-1075); hit-testing walks positive-z hoisted children, then paint children in reverse, then negative-z hoisted children (packages/blitz-dom/src/node/node.rs:1385-1447)
- Positioned descendants with `z-index: auto` share one paint level per CSS 2.1 Appendix E and paint in tree order (tests/blitz-tests/tests/paint_order.rs:1-2)
- Outset box shadows are clipped when opacity is below 1 or the background is not opaque, and blurred shadows use the averaged border radius per a TODO (packages/blitz-paint/src/render/box_shadow.rs:15-24; packages/blitz-paint/src/render/box_shadow.rs:77-82); an outset shadow takes the element's shape corner for corner, so one with no blur and no spread hides behind the element, with expected pixels from Chromium (tests/blitz-tests/tests/outset_box_shadow_shape.rs:1-6)
- Inset box shadows fill the padding box then cut a blurred hole with Compose::DestOut (packages/blitz-paint/src/render/box_shadow.rs:89-156)
- Opacity, filter and backdrop-filter are applied via a layer clipped to the border box expanded by the filter area (packages/blitz-paint/src/render.rs:496-528)

**Example fixtures:**
- `filter: drop-shadow(...)` and `backdrop-filter: blur(10px)` (examples/assets/filters.html:37; examples/assets/filters.html:80); a GitHub fixture's fixed progress bar at `z-index: 2147483647` (examples/assets/github_profile_reduced2.html:3-10)
- google search box active shadow 0 1px 6px rgba(32,33,36,.28) and layered shadows such as 0 1px 3px 1px rgba(66,64,67,.15),0 1px 2px 0 rgba(60,64,67,.3); google bar z-index 986 and graphite header z-index 1000 (examples/assets/google.html:3151; examples/assets/google.html:364-365; examples/assets/google.html:47; examples/assets/graphite.html:961)
- hr fixture inset shadow and outset glow (examples/assets/hr.html:137; examples/assets/hr.html:146); servo.css `.box` two-layer shadow (examples/assets/servo.css:3041-3044); Preact TodoMVC `.card` `0 2px 4px rgba(0,0,0,.15)` (examples/preact/index.html:18); box-shadow fixtures cover outset, spread, layered and inset shadows (examples/box_shadow.rs:26-38; examples/assets/shadow.html:31-114)

**Observed absent:**
- z-index or shadow handling · searched: `border-radius|border_radius|box-shadow|box_shadow|z-index|z_index` over the 17 listed s06 files
- shadow or elevation tokens · searched: `shadow|elevation` over the 16 listed s08 files (no match)
- shadow or elevation definitions · searched: `box-shadow|shadow|elevation|z-index` over the 32 s10 slice files
- shadows, elevation or z-index tokens · searched: `radius|shadow|elevation|z-index` over the 21 listed s11 files (only a touch-point `radius` matched)

> NOT YET MEASURED — a project-wide elevation scale: the examples, engine-resolve, layout and WPT slices (s01, s05, s07, s13) recorded it as out of slice.

---

## Border Radius

| Token | Value | Usage |
|-------|-------|-------|
| Browser chrome (untokenized) | 4px tabs (top corners), tooltip, urlbar input, icon buttons, menu; 3px close button and status bar (top-right) | (apps/browser/assets/browser.css:45-46; apps/browser/assets/browser.css:67; apps/browser/assets/browser.css:104; apps/browser/assets/browser.css:166; apps/browser/assets/browser.css:180; apps/browser/assets/browser.css:216; apps/browser/assets/browser.css:295) |
| Browser about pages (untokenized) | clear button 6px, history rows 8px, newtab search input 8px | (apps/browser/assets/about-history.css:27; apps/browser/assets/about-history.css:48; apps/browser/assets/about-newtab.css:29) |
| seven_guis (untokenized) | cards 8px, buttons 6px, inputs 4px, task cards 6px, tags 3px | (examples/seven_guis/src/tasks/counter.rs:41; examples/seven_guis/src/tasks/counter.rs:61; examples/seven_guis/src/tasks/temp_converter.rs:69; examples/seven_guis/src/app.rs:227; examples/seven_guis/src/app.rs:268) |
| transparent example card | 16px | (examples/transparent/src/app.rs:121) |
| blitz-dom default button | 1px | (packages/blitz-dom/assets/default.css:102) |
| Checkbox frame (engine) | 2 × control scale | (packages/blitz-paint/src/render/form_controls.rs:28-33) |
| Scrollbar thumb (engine) | half the thumb thickness (fully rounded) | (packages/blitz-paint/src/render.rs:813-823) |

**Personality:**
> NO RECORDED INTENT

**Engine radius handling:**
- Per-corner elliptical radii are resolved from computed border-*-radius values and scaled to device pixels (packages/blitz-paint/src/render.rs:1253-1270)
- `inset()` clip-path ignores border-radius per a TODO (packages/blitz-paint/src/render/clip_path.rs:167-168)
- The `border` attribute on `img`, `object` and image inputs maps to four solid pixel borders (packages/blitz-dom/src/stylo.rs:1120-1141)

**Example fixtures:**
- Uniform, percentage, per-corner and elliptical radii (examples/assets/border.html:16-20; examples/assets/border-styles.html:65-73; examples/assets/cursor.html:34)
- google search box 24px, buttons 4px, also 2px, 8px and 50% (examples/assets/google.html:3151; examples/assets/google.html:3491; examples/assets/google.html:100; examples/assets/google.html:923; examples/assets/google.html:459); graphite inputs 0 and carousel dots 50% (examples/assets/graphite.html:341; examples/assets/graphite.html:577); hr pills 999px (examples/assets/hr.html:128); input fixture 100px (examples/assets/input.html:9)
- `.button` `border-radius: 1.5rem 0` in the inline-flex-transform fixture and the transforms example (examples/assets/inline-flex-transform.html:22; examples/transforms.rs:291)
- servo.css `.button` 4px, `.box` 6px, loader 9999px (examples/assets/servo.css:31; examples/assets/servo.css:3043; examples/assets/servo.css:85); shadow fixture `.card` 12px (examples/assets/shadow.html:24); Preact TodoMVC filter buttons 3px (examples/preact/index.html:37); reference page `code` 4px and `.tag` 999px (examples/preact/core_dom_apis.html:27; examples/preact/core_dom_apis.html:39)

**Observed absent:**
- border-radius values or tokens · searched: `border-radius|border_radius` over the 15 s05 files
- border-radius handling · searched: `border-radius|border_radius|box-shadow|box_shadow|z-index|z_index` over the 17 listed s06 files
- border radius values · searched: `radius` over the 8 s07 slice files
- border radius values · searched: `border.radius|radius` over the 16 listed s08 files (no match)
- radius definitions · searched: `border-radius|radius` over the 32 s10 slice files
- border-radius values or tokens · searched: `radius|shadow|elevation|z-index` over the 21 listed s11 files (only a touch-point `radius` matched)

> NOT YET MEASURED — a project-wide radius scale: the examples, test and WPT slices (s01, s12, s13) recorded radius tokens as out of slice.

---

## Motion (calibrated to expression level `unrecorded`)

**Expression level:**
> NO RECORDED INTENT

**This project's values:**
- Micro-interactions (apps): todomvc transitions label color over 0.4s and destroy button color over 0.2s ease-out (examples/todomvc/src/todomvc.css:214; examples/todomvc/src/todomvc.css:234); observed absent — transitions or animations in browser chrome and seven_guis styles · searched: `transition|animation|@keyframes` over apps/browser/assets/*.css and examples/seven_guis/src/**/*.rs
- Smooth scrolls run 300 ms on a cubic ease-in-out curve (packages/blitz-dom/src/scrolling.rs:113-123; packages/blitz-dom/src/scrolling.rs:433-434); `scroll-behavior: smooth` makes `Auto` scrolls animate and `auto` behavior jumps (packages/blitz-dom/src/scrolling.rs:532-549; tests/blitz-tests/tests/fragment_navigation.rs:264-293; tests/blitz-tests/tests/fragment_navigation.rs:374-383); a wheel event over a scroller cancels an in-progress smooth scroll (tests/blitz-tests/tests/fragment_navigation.rs:356-372)
- Scroll behaviour values `auto`, `instant`, `smooth` are parsed by the script runtime and passed to blitz-dom (packages/blitz-vibey-script/src/dom/element.rs:1087-1102); programmatic scrolling in dioxus-native-dom supports `Smooth` and `Instant` (packages/dioxus-native-dom/src/events.rs:235-238; packages/dioxus-native-dom/src/events.rs:274-277)
- Touch flings decelerate per frame until velocity drops below 0.1 (packages/blitz-dom/src/scrolling.rs:724-747)
- Overlay scrollbars show at full opacity on scroll, stay opaque for `FADE_DELAY` = 500 ms after last activity, then fade linearly over `FADE_DURATION` = 200 ms, documented as Chromium's overlay timings (packages/blitz-dom/src/node/scrollbar.rs:12-26; packages/blitz-dom/src/document.rs:1772-1789; packages/blitz-paint/src/render.rs:715-720; packages/blitz-paint/src/render.rs:742-745); finished fades are dropped each resolve (packages/blitz-dom/src/resolve.rs:66-72); hovering where a hidden thumb would be does not summon it, and the thumb changes appearance on hover and while dragged (tests/blitz-tests/tests/scrollbars.rs:1-4; tests/blitz-tests/tests/scrollbar_drag.rs:136-166; tests/blitz-tests/tests/scrollbar_drag.rs:168-222)

**Animation runtime:**
- CSS animations and transitions move from pending to running to finished by the current time during style resolution (packages/blitz-dom/src/stylo.rs:105-123)
- Active CSS animations/transitions, canvases, animating sub-documents, custom widgets, scroll animations and scrollbar fades keep the document animating (packages/blitz-dom/src/document.rs:2039-2056); a custom widget returning `true` from `requires_redraw` causes continuous redraw scheduling (packages/blitz-dom/src/node/custom_widget.rs:100-106); a harness settle answers for every member of that set the same way — none holds it open and none is waited on or advanced: it returns `Settled` with `animating` reading the document's animating flag (packages/blitz-test-harness/src/settle.rs:151-186; one member, a CSS animation, exercised at tests/blitz-tests/tests/stand_settle.rs:279-307 — as measured at escher-0.1.0/chunks/2026-10-07-settle-detection/report.md)
- The shell's animation clock is seconds since the first animation-time query, and frames keep redrawing while the document is animating (packages/blitz-shell/src/window.rs:280-288; packages/blitz-shell/src/window.rs:437-439); the test harness drives animation from a controlled clock advanced by tick (packages/blitz-test-harness/src/harness.rs:140-155), and settle reads animation time only through that clock, never advancing it; the driver's `advance` moves an app's own time — through the step a session's caller hands it, on the stand whole Timer ticks — and leaves that clock where it was, as do `click`, `type` and `press`: a driver click beside a running CSS animation returns settled without waiting on it (tests/blitz-tests/tests/stand_act_timer.rs:1-11 — as measured at escher-0.1.0/chunks/2026-10-07-act-by-id/report.md)
- `requestAnimationFrame` is a timer approximated as 16ms away, passing 16 as the timestamp (packages/blitz-vibey-script/src/runtime.rs:2087-2112)
- Animation and transition event data are `unimplemented!()` in dioxus-native-dom (packages/dioxus-native-dom/src/events.rs:70-72; packages/dioxus-native-dom/src/events.rs:122-124)

**Reduced motion:**
- The BBC fixture contains `prefers-reduced-motion` media queries (examples/assets/bbc_reduced.html:10; examples/assets/bbc_reduced.html:11; examples/assets/bbc_reduced.html:17)
- Observed absent — reduced-motion handling · searched: `prefers-reduced-motion` over the 21 s02 files, the 32 s03 slice files and the 86 s04 slice files; `prefers-reduced-motion|reduced_motion` over the 15 s05 files; `prefers-reduced-motion|reduced_motion|prefers_reduced` over the 17 listed s06 files; `reduced.motion|prefers` over the 32 listed s09 files
- Observed absent — transitions or animation · searched: `transition|animation` over the 8 s07 slice files

**Example fixtures:**
- `@keyframes` animations (big-small 2s alternate, gradient-animation 20s, spin 4s linear) and transitions on transform and filter with `:hover`/`:active` nesting (examples/assets/animated_layout.html:21; examples/assets/animated_layout.html:28-43; examples/assets/animation.html:32-38; examples/assets/animation.html:17-21; examples/assets/animation.html:46-51; examples/assets/filters.html:16-17)
- google keyframes gb__a, g-bubble-show, g-bubble-hide, g-snackbar-show/hide and qli spinner, with transitions such as box-shadow 250ms and transform/opacity/visibility .3s ease-in-out (examples/assets/google.html:49-56; examples/assets/google.html:1589; examples/assets/google.html:1597; examples/assets/google.html:1700; examples/assets/google.html:1709; examples/assets/google.html:2757; examples/assets/google.html:597; examples/assets/google.html:1457)
- graphite carousel transform .5s and details marker rotating 90deg (examples/assets/graphite.html:498-500; examples/assets/graphite.html:1243-1245); gosub link border-bottom 0.3s ease-in-out (examples/assets/gosub.html:49); inline-flex-transform scale/translate/rotate/transform over 0.15s, scale 1.2 on hover (examples/assets/inline-flex-transform.html:27-28; examples/assets/inline-flex-transform.html:37-38)
- servo.css `@keyframes spinAround` on `.button.is-loading::after` at 500ms infinite linear and 86ms ease-out transitions on background-color, opacity, transform (examples/assets/servo.css:82-83; examples/assets/servo.css:328-335; examples/assets/servo.css:6096-6098)
- svg_native transitions `fill 0.15s` on hover (examples/svg_native.rs:14-15); the transforms example `.button` transitions `filter, scale` over 0.15s and on hover applies `brightness(90%)` and `scale: 1.2` (examples/transforms.rs:302-303; examples/transforms.rs:312-315)

---

## Iconography

**Style:** Browser toolbar icons are SVG assets rotate-cw, house, arrow-left, arrow-right, ellipsis-vertical, external-link, code, and camera (feature-gated) (apps/browser/src/icons.rs:3-11); tab close and new-tab controls are text glyphs "×" and "+" (apps/browser/src/tab_strip.rs:92-96; apps/browser/src/tab_strip.rs:102-106)
**Library:** `IconButton` renders an `img.urlbar-icon` inside a clickable div (apps/browser/src/icons.rs:13-40; apps/browser/assets/browser.css:200-202); bundle icons are `blitz-logo.png` and `blitz-logo.ico` (apps/browser/Dioxus.toml:7); rdme styles GitHub octicons (apps/readme/assets/github-markdown.css:139-157)
**Size grid:** toolbar icon images 20px high (apps/browser/src/icons.rs:13-40; apps/browser/assets/browser.css:200-202); menu item icons 16x16 (apps/browser/assets/browser.css:243-246); favicons 16x16 (apps/browser/src/tab.rs:269-281)

**Cursors and favicons (engine):**
- The cursor is taken from the CSS `cursor` keyword, else Text for text inputs, Pointer inside links, Text over selectable text, Default otherwise (packages/blitz-dom/src/document.rs:2167-2221)
- CSS `cursor` keywords map one-to-one to `CursorIcon` values, and `cursor: none` maps to no cursor (packages/blitz-dom/src/stylo_to_cursor_icon.rs:4-49); cursor icons use the `cursor-icon` crate and `ShellProvider::set_cursor` takes an optional `CursorIcon` (packages/blitz-traits/Cargo.toml:21; packages/blitz-traits/src/shell.rs:3; packages/blitz-traits/src/shell.rs:13-15); in the shell they are winit CursorIcon values, and None hides the cursor and resets it to Default (packages/blitz-shell/src/lib.rs:101-112)
- `favicon_url` returns the `href` of the first `<link>` whose `rel` contains `icon` (packages/blitz-dom/src/document.rs:593-608)

**Example fixtures:**
- A fixture enumerates 36 CSS cursor values as "the built-in cursor styles supported by blitz" (examples/assets/cursor.html:54-91)
- Inline SVG octicons in the captured GitHub fixture (examples/assets/github_profile_reduced2.html:65-67); google inline SVG paths in 24-unit viewBoxes (examples/assets/google.html:3086-3095; examples/assets/google.html:3159-3161; examples/assets/google.html:3219-3221)
- Font Awesome classes in the gosub, pseudo, servo-new and servo fixtures, servo.html loading from use.fontawesome.com; pseudo defines .fa-github:before and .gh:before with content "\f09b" (examples/assets/gosub.html:107-109; examples/assets/pseudo.html:71-73; examples/assets/servo-new.html:206-218; examples/assets/servo.html:26; examples/assets/servo.html:154; examples/assets/servo.html:161; examples/assets/servo.html:168; examples/assets/servo.html:253; examples/assets/pseudo.html:55-65)
- graphite positions icons from one sprite atlas through `--atlas-index` and embeds SVG icons as CSS data URIs (examples/assets/graphite.html:661-666; examples/assets/graphite.html:748-753; examples/assets/graphite.html:1948; examples/assets/graphite.html:1229; examples/assets/graphite.html:1627)
- Inline SVG icons with `fill="currentColor"` (examples/assets/svg.html:4-12); svg_native fills an icon circle with `currentColor` set from the svg's `color` (examples/svg_native.rs:27-29); the Preact TodoMVC destroy button renders `×` (examples/preact/index.html:99-103)

**Observed absent:**
- icon assets or icon handling · searched: `icon` over the 16 listed s08 files (no match)
- icons or icon sets · searched: `icon` over the 21 listed s11 files

---

## Surface: desktop-native

**Platform:** Windows (NSIS .exe), macOS (.dmg) and Linux (.AppImage) on x86_64 and aarch64 (.github/workflows/publish-browser.yml:47-82); the flake's default package is the browser app whose binary is `blitz`, wrapped with winit/wgpu runtime libraries on Linux (flake.nix:40-53; flake.nix:117-121)
**Toolkit / Framework:** winit windows (packages/blitz-shell/src/window.rs:200-206); HTML launches into a native window with `WindowConfig` and the Vello window renderer (packages/blitz/src/lib.rs:106-131; examples/inner_html.rs:28-30; examples/preact_script.rs:37); Dioxus apps open through `dioxus_native::launch` (examples/box_shadow.rs:4; examples/custom_widget.rs:18; examples/counter/src/main.rs:15-17; examples/seven_guis/src/main.rs:4-10; examples/todomvc/src/main.rs:15-19), built from `WindowAttributes` and titled from dioxus-cli-config or "Dioxus App" (packages/dioxus-native/src/config.rs:17-20; packages/dioxus-native/src/lib.rs:236-237)

### Tokens (platform-specific)
- Every document gets the blitz `DEFAULT_CSS` user-agent stylesheet (packages/dioxus-native-dom/src/dioxus_document.rs:97-98)
- The initial theme is the window's theme, defaulting to Light (packages/blitz-shell/src/window.rs:181-182)
- The default viewport is window size (0, 0), hidpi scale 1.0, zoom 1.0, Light scheme; its logical size is the physical window size divided by hidpi × zoom (packages/blitz-traits/src/shell.rs:84-93; packages/blitz-traits/src/shell.rs:110-127)
- JS `innerWidth`/`innerHeight` are window size ÷ scale, `outerWidth`/`outerHeight` alias them, `devicePixelRatio` is the scale (packages/blitz-vibey-script/src/runtime.rs:1408-1413; packages/blitz-vibey-script/src/runtime.rs:2116-2138)

### Component Patterns
- On macOS the browser tabstrip gets `merged-titlebar` with 90px left padding and 44px height (apps/browser/src/tab_strip.rs:9-12; apps/browser/assets/browser.css:28-31)
- The shell provider exposes minimize, maximize, decorations toggle and window drag for custom titlebars (packages/blitz-shell/src/lib.rs:139-153; packages/blitz-shell/src/event.rs:28-32)
- Custom GPU content composites with HTML layers at `opacity: 0.8` in the canvas container (examples/custom_widget.rs:190-193)
- Devtools can draw browser-style overlays of content, padding, border and margin for the hovered or a chosen node (packages/blitz-traits/src/devtools.rs:11-17)

### Navigation Pattern
- The browser window title follows the active tab's display title (apps/browser/src/main.rs:166; apps/browser/src/main.rs:173); the shell sets the window title from the document's title node when it was set before the window existed (packages/blitz-shell/src/window.rs:200-206)
- rdme titles its window "README for" plus the last path segment, and toggles light/dark theme with Ctrl/Cmd+T (apps/readme/src/main.rs:95-98; apps/readme/src/readme_application.rs:124-131; apps/readme/src/readme_application.rs:175)

### Platform-Specific Notes
- The browser window on macOS uses a transparent, unified, hidden-title titlebar with full-size content view (apps/browser/src/main.rs:82-89)
- The transparent example opens a 360x300 decoration-less transparent window and closes via the shell provider (examples/transparent/src/app.rs:9-31; examples/transparent/src/app.rs:49-52)
- Window compositing alpha mode is configurable, for example for transparent windows; unsupported modes are ignored by the renderer (packages/dioxus-native/src/config.rs:66-74)
- On wasm32 the same surface is a canvas appended to the page body, sized from host CSS (packages/dioxus-native/src/config.rs:22-34)
- The optional `scrollbars` feature forwards to blitz-paint (packages/dioxus-native/Cargo.toml:47)

---

## Surface: mobile-native

**Platform:** Android aarch64 APK bundled with `--android --package-types apk --no-default-features --features android-defaults` (.github/workflows/publish-browser.yml:83-89); CI builds (does not test) for `aarch64-apple-ios` and `aarch64-linux-android` (.github/workflows/ci.yml:328-343)
**Toolkit / Framework:** the browser app; `IS_MOBILE` is true for Android and iOS and adds the `mobile` class to the frame (apps/browser/src/main.rs:48; apps/browser/src/main.rs:172)

### Tokens (platform-specific)
- On mobile the urlbar input grows to 16px font with 8px 6px padding (apps/browser/assets/browser.css:140-161)

### Component Patterns
- The `mobile` frame class drives the mobile urlbar sizing (apps/browser/src/main.rs:48; apps/browser/src/main.rs:172; apps/browser/assets/browser.css:140-161)

### Navigation Pattern
- The Android hardware back button navigates back in the active tab (apps/browser/src/main.rs:140-145)

### Platform-Specific Notes
- On mobile, screenshots save to a default file name without a dialog (apps/browser/src/capture.rs:109-110)
- iOS and Android targets are built but not tested in CI (.github/workflows/ci.yml:328-343)

---

## Surface: cli

**Platform:** command-line binaries — the WPT runner has `fn main` (wpt/runner/src/main.rs:457); `paint_bench` and `screenshot` examples and the `bump` app print to the terminal (examples/paint_bench.rs:113-115; examples/screenshot.rs:31; apps/bump/src/main.rs:25-31)
**Toolkit / Framework:** output colors come from `owo-colors` (wpt/runner/src/main.rs:24; wpt/runner/Cargo.toml:38)

### Tokens (platform-specific)
- Result colors: PASS green, FAIL with some passing subtests yellow, FAIL red, TIMEOUT bright red, SKIP bright black, CRASH bright magenta (wpt/runner/src/main.rs:382-392)
- Test kind, flag markers and summary section headings print in bright black (wpt/runner/src/main.rs:395; wpt/runner/src/main.rs:405-432; wpt/runner/src/main.rs:794; wpt/runner/src/main.rs:798; wpt/runner/src/main.rs:812; wpt/runner/src/main.rs:817)
- Test names are wrapped in OSC 8 hyperlinks to `https://wpt.live/{name}` only when `supports_hyperlinks::on(Stdout)` (stdout is a terminal, honouring `FORCE_HYPERLINK`) (wpt/runner/src/main.rs:54-63; wpt/runner/src/main.rs:360-370)

### Component Patterns
- `paint_bench` prints `Loaded {url} at {w}x{h}@{scale}x; running {iters} paint iterations ({backend} backend)` then one stats line per phase (examples/paint_bench.rs:113-115; examples/paint_bench.rs:34)
- `screenshot` prints the URL, per-phase `... in {n}ms` lines, `Screenshot is ({w}x{h})` and `Written to {path}` (examples/screenshot.rs:31; examples/screenshot.rs:203; examples/screenshot.rs:146-148)
- bump prints "Bumped anyrender versions" or "Bumped blitz versions" on success and errors to stderr (apps/bump/src/main.rs:25-31; apps/bump/src/main.rs:100; apps/bump/src/main.rs:109)

### Navigation Pattern
> NOT YET MEASURED — no slice recorded argument parsing or command structure for the CLI binaries.

### Platform-Specific Notes
> NOT YET MEASURED — no slice recorded terminal-width handling or platform differences for the CLI binaries.

---

## Surface: web-spa

**Platform:** WASM pages rendering into a canvas — seven_guis and todomvc fill the window with a canvas on `#f5f5f5` (examples/seven_guis/index.html:7-10; examples/todomvc/index.html:7-10)
**Toolkit / Framework:** on wasm32 dioxus-native's surface is a canvas appended to the page body, sized from host CSS (packages/dioxus-native/src/config.rs:22-34)

### Tokens (platform-specific)
- WASM builds register bundled DejaVu Sans for sans-serif, serif, monospace and system-ui (examples/wasm_hello/src/lib.rs:75-100); on WASM a font context with bundled fonts must be provided (packages/dioxus-native/src/config.rs:56-64)

### Component Patterns
- wasm_hello's canvas is 80% of the page with a 4px radius, focusable via tabindex 0 with its outline removed (examples/wasm_hello/index.html:17-24; examples/wasm_hello/src/lib.rs:113-118)

### Navigation Pattern
> NOT YET MEASURED — no slice recorded routing or navigation for the WASM pages.

### Platform-Specific Notes
> NOT YET MEASURED — no slice recorded browser-specific limitations for the WASM pages.

---

## Anti-Patterns (NEVER do these)

> NO RECORDED INTENT

---

## Self-Validation Protocol

> NO RECORDED INTENT

---

## Design Decisions Log

> NO RECORDED INTENT
