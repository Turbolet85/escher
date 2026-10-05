# design-system — gathered facts

## §Brand Identity

(no fact block)

## §Color Palette

### facts-s01.md:151

- An example fixture defines a 25-colour block palette commented "curated harmonious palette" (e.g. #e63946, #2a9d8f, #264653, #8338ec, #3a86ff) (examples/assets/clip-path.html:35-60)
- An example fixture uses border colours #d6336c, #1c7ed6, #2f9e44, #f59f00 and dark #1a1a1a on background #fffbe6 / #f5f5f5 (examples/assets/border-styles.html:7; examples/assets/border-styles.html:27-29; examples/assets/border-styles.html:77; examples/assets/border-styles.html:82)
- A reduced GitHub fixture sets custom property `--borderColor-default: green` and consumes it via `var()` (examples/assets/github_profile_reduced.html:6-13)
- The BBC fixture uses relative colour syntax `color(from #202224 srgb r g b / …)` (examples/assets/bbc_reduced.html:13-16)
- out of slice — the project's own UI colour tokens

### facts-s02.md:100

- the graphite fixture defines 16 color custom properties on :root, among them --color-navy #16323f, --color-crimson #803847, --color-fog #eee and --color-seaside-rgb 176, 214, 203 (examples/assets/graphite.html:890-906)
- the graphite subsets repeat the same :root color properties (examples/assets/graphite_blog_section.html:890-906; examples/assets/graphite_software_overview.html:888-904)
- the graphite fixture's page text is var(--color-navy) on #fff and links are var(--color-crimson) (examples/assets/graphite.html:918-922; examples/assets/graphite.html:1405-1407)
- the google fixture sets body/input/button text #202124 on #fff, links #1a0dab and visited links #681da8 (examples/assets/google.html:19-36)
- observed absent — CSS custom properties in the google fixture · searched: `var\(--|--[a-z]+:` over examples/assets/google.html
- the gosub fixture is dark: #ffffff text on #121212, cyan links with a 2px solid #6e40c9 bottom border, and an h1 gradient from rgba(96,243,236,1) to rgba(30,84,231,1) (examples/assets/gosub.html:7-9; examples/assets/gosub.html:45-50; examples/assets/gosub.html:74-80)
- the hr fixture assigns a distinct hex color per case, e.g. #e63946, #2a9d8f, #457b9d, #8338ec (examples/assets/hr.html:43; examples/assets/hr.html:50; examples/assets/hr.html:72; examples/assets/hr.html:78)
- the inline-backgrounds fixture uses pastel span backgrounds and one rgba(214, 51, 108, 0.35) translucent background (examples/assets/inline-backgrounds.html:35-53)
- observed absent — a prefers-color-scheme rule · searched: `prefers-color-scheme` over the 21 s02 files
- the google fixture shows a "Dark theme: Off" settings menu item (examples/assets/google.html:3788)

### facts-s03.md:136

- The custom-widget and static-HTML examples use background `#f4e8d2` (examples/custom_widget.rs:187; examples/html.rs:16)
- The inline logo SVG fills circles with rgb(1,99,63), rgb(0,118,114), rgb(62,149,147), rgb(252,176,64), rgb(233,86,41), rgb(230,29,50) and a bolt with rgb(244,232,210) (examples/html.rs:31-36; examples/html.rs:39)
- servo.css fixture: page background `#121619`, body text `hsl(0, 0%, 96%)`, links `#1191E8`, link hover `#42BF64`, code text `hsl(348, 86%, 46%)` (examples/assets/servo.css:198-199; examples/assets/servo.css:236-237; examples/assets/servo.css:243-253; examples/assets/servo.css:255-257)
- servo.html hero accent colors `#4fc066`, `#209e9b`, `#f03278`, `#f68243`, `#faae30`, `#712b97` (examples/assets/servo.html:184-191)
- TodoMVC page: background `#f5f5f5`, text `#111`, heading `#b83f45`, muted `#949494`, footer `#777` (examples/preact/index.html:13-14; examples/preact/index.html:17; examples/preact/index.html:27; examples/preact/index.html:34)
- Reference page tag colors: preact `#673ab8`, react `#087ea4`, wpt `#9a6700`, extra `#888` (examples/preact/core_dom_apis.html:44-47)
- The transforms example `.button` uses `#e74310` with white text (examples/transforms.rs:304-305)
- The only CSS custom property in the slice is `--columnGap` in servo.css (examples/assets/servo.css:1577-1588)

### facts-s04.md:210

- Browser chrome: tabstrip `#E0E0E0`, tab `#D0D0D0`, tab hover `#E8E8E8`, active tab and urlbar `#F9F9F9`, urlbar border `#EEE` (apps/browser/assets/browser.css:25; apps/browser/assets/browser.css:44; apps/browser/assets/browser.css:51; apps/browser/assets/browser.css:77; apps/browser/assets/browser.css:134-135)
- Browser focus color `#5E9ED6`; icon button hover `#CCC`, active `#BBB` (apps/browser/assets/browser.css:172-175; apps/browser/assets/browser.css:182-189)
- Tooltip `#333` on `#fff`; FPS overlay `rgb(0, 255, 0)` on `rgba(0, 0, 0, 0.7)`; status bar `rgba(240, 240, 240, 0.95)` with `#333` text (apps/browser/assets/browser.css:61-62; apps/browser/assets/browser.css:267-268; apps/browser/assets/browser.css:292; apps/browser/assets/browser.css:303)
- About pages use `#f0f0f0` backgrounds; history link color `#1a73e8`; muted text `#888`/`#777`/`#aaa` (apps/browser/assets/about-history.css:3; apps/browser/assets/about-history.css:69; apps/browser/assets/about-history.css:77; apps/browser/assets/about-history.css:84; apps/browser/assets/about-newtab.css:7; apps/browser/assets/about-stub.css:2)
- Error page paragraph color `#666` (apps/browser/assets/error.html:14)
- rdme uses GitHub markdown color tokens (`--fgColor-*`, `--bgColor-*`, `--borderColor-*`, syntax colors) defined separately for dark and light schemes (apps/readme/assets/github-markdown.css:13-124)
- rdme override sets dark-mode page background `#0d1117` and light-mode white (apps/readme/assets/blitz-markdown-overrides.css:11-27)
- seven_guis accent `#4a6cf7`, hover `#3a5ce5`, active `#2a4cd3`, page `#f5f5f5`, text `#1a1a1a` (examples/seven_guis/src/tasks/counter.rs:48; examples/seven_guis/src/tasks/counter.rs:58-71; examples/seven_guis/src/app.rs:173)
- seven_guis invalid state `#e53e3e`/`#fff5f5`/`#c53030` and success message `#ebf8ee`/`#68d391`/`#276749` (examples/seven_guis/src/tasks/flight_booker.rs:186-190; examples/seven_guis/src/tasks/flight_booker.rs:221-229)
- todomvc heading `rgba(175, 47, 47, 1.0)`, body text `#4d4d4d` on `#f5f5f5` (examples/todomvc/src/todomvc.css:26-27; examples/todomvc/src/todomvc.css:76)
- counter and transparent buttons use named green, red, blue with white text (examples/counter/src/app.rs:83-111; examples/transparent/src/app.rs:184-212)
- wasm_hello dark palette `#0f1226`, `#1a1d3a`, heading `#ffd166`, code `#ff7b9c` (examples/wasm_hello/src/lib.rs:46; examples/wasm_hello/src/lib.rs:55; examples/wasm_hello/src/lib.rs:62; examples/wasm_hello/src/lib.rs:68)
- wgpu_texture main background `#f4e8d2` (examples/wgpu_texture/src/styles.css:15)
- blitz-dom default stylesheet: link `rgb(0, 0, 238)`, input focus outline `#4D90FE`, button background `#EFEFEF` (packages/blitz-dom/assets/default.css:42-45; packages/blitz-dom/assets/default.css:92-95; packages/blitz-dom/assets/default.css:105)

### facts-s05.md:194

- Color-scheme changes trigger a full recascade because `light-dark()` and system colors resolve at cascade time (packages/blitz-dom/src/document.rs:2069-2076)
- The parent viewport's color scheme is copied to iframe sub-documents (packages/blitz-dom/src/resolve.rs:146-150)
- observed absent — CSS custom properties defining colors · searched: `--[a-z]` over the 15 s05 files
- out of slice — contents of the embedded default user-agent stylesheet `assets/default.css`

### facts-s06.md:185

- The `Color` type is `AlphaColor<Srgb>` and Stylo colors convert to it via sRGB (packages/blitz-dom/src/util.rs:12; packages/blitz-dom/src/util.rs:166-178)
- The viewport's `ColorScheme` Light or Dark feeds `prefers-color-scheme` (packages/blitz-dom/src/stylo_device.rs:80-83)
- The legacy `bgcolor` attribute maps to `background-color` (packages/blitz-dom/src/stylo.rs:1182-1189)

### facts-s07.md:126

- observed absent — color tokens or CSS custom properties · searched: `color|Color` and `--[a-z]+-|var\(` over the 8 slice files

### facts-s08.md:165

- `scrollbar-color` is resolved to absolute thumb and track colors against the element's computed `color`, defaulting to `Auto` (packages/blitz-dom/src/node/scrollbar.rs:38-48; packages/blitz-dom/src/node/scrollbar.rs:58-74)
- In SVG serialization, `currentColor` in attribute values is replaced with the element's computed `color` (packages/blitz-dom/src/node/serialize.rs:18-19; packages/blitz-dom/src/node/serialize.rs:120-125; packages/blitz-dom/src/node/serialize.rs:186-194)
- out of slice — color tokens or a product palette

### facts-s09.md:136

- default text-selection highlight colour is rgb 180, 213, 255 (packages/blitz-paint/src/lib.rs:28-29)
- default scrollbar thumb colours: dark scheme rest 214,214,214,178 · hover 190,190,190,222 · active 172,172,172,255 · stroke 0,0,0,102; light scheme rest 128,128,128,178 · hover 152,152,152,222 · active 170,170,170,255 · stroke 255,255,255,102 (packages/blitz-paint/src/render.rs:747-766)
- author scrollbar-color thumbs are blended for contrast at 1.8 on hover and 1.3 when active (packages/blitz-paint/src/render.rs:768-770; packages/blitz-paint/src/render.rs:805-812)
- disabled checkbox/radio accent is rgba 209,209,209,255, otherwise the element's color stands in for accent-color per a TODO (packages/blitz-paint/src/render/form_controls.rs:21-26)
- unchecked radio ring uses the CSS palette GRAY and checkbox ticks/gaps use white (packages/blitz-paint/src/render/form_controls.rs:76-79; packages/blitz-paint/src/render/form_controls.rs:99-101)
- debug overlay colours: content blue 66,144,245,128 · padding green 81,144,66,128 · border red 245,66,66,128 · margin orange 249,204,157,128 (packages/blitz-paint/src/debug_overlay.rs:95; packages/blitz-paint/src/debug_overlay.rs:98; packages/blitz-paint/src/debug_overlay.rs:110; packages/blitz-paint/src/debug_overlay.rs:119)
- devtools layout strokes are red for block/flow-root, green for flex, blue for grid (packages/blitz-paint/src/render.rs:1208-1215)
- the canvas background is the root element's background colour, falling back to body's when the root is transparent (packages/blitz-paint/src/render.rs:193-228)

### facts-s10.md:198

- the system color scheme is an enum `Light` (default) / `Dark` (packages/blitz-traits/src/shell.rs:68-74)
- devtools `show_layout` outlines elements "with different border colors"; no colour values are stated (packages/blitz-traits/src/devtools.rs:8-10)
- observed absent — CSS custom properties · searched: `--[a-z-]+:` over the 32 slice files

### facts-s11.md:173

- The frame clear color is an optional `base_color` (a `peniko::Color`), unset by default (packages/dioxus-native/src/config.rs:12; packages/dioxus-native/src/config.rs:40; packages/dioxus-native/src/config.rs:76-80; packages/dioxus-native/src/dioxus_renderer.rs:37-38)
- observed absent — color tokens or CSS custom properties · searched: `var\(--|--[a-z]+-` over the 21 listed s11 files

### facts-s12.md:157

- The engine's default overlay scrollbar palette follows the viewport color scheme (Light vs Dark give distinct thumbs) (tests/blitz-tests/tests/scrollbars.rs:204-211)
- Default scrollbar thumbs paint as a fill plus a thin contrast stroke (tests/blitz-tests/tests/scrollbars.rs:194-202)
- Author `scrollbar-color` styles thumb and track; hover/drag feedback on a near-white author thumb blends toward the pole with contrast headroom (darken) (tests/blitz-tests/tests/scrollbars.rs:125-160; tests/blitz-tests/tests/scrollbar_drag.rs:224-227)
- `ColorScheme` has `Light` and `Dark` values set on the viewport (tests/blitz-tests/tests/device_coalescing.rs:101)

### facts-s13.md:169

- Terminal result colors: PASS green, FAIL with some passing subtests yellow, FAIL red, TIMEOUT bright red, SKIP bright black, CRASH bright magenta (wpt/runner/src/main.rs:382-392)
- Test kind, flag markers and summary section headings print in bright black (wpt/runner/src/main.rs:395; wpt/runner/src/main.rs:405-432; wpt/runner/src/main.rs:794; wpt/runner/src/main.rs:798; wpt/runner/src/main.rs:812; wpt/runner/src/main.rs:817)
- Rendered pages get a white background fill (wpt/runner/src/test_runners/ref_test.rs:305-312)

## §Typography

### facts-s01.md:158

- Example fixtures use `font-family: sans-serif` for body and `monospace` for code labels (examples/assets/cursor.html:6; examples/assets/cursor.html:46; examples/assets/border-styles.html:6; examples/assets/border-styles.html:38)
- The Google fixture sets 14px `arial,sans-serif` with colour #202124 (examples/assets/bottom_only.html:8)
- out of slice — the project's own type scale

### facts-s02.md:112

- the graphite fixture sets html/body to Inter Variable, 18px, weight 500, line-height 1.5, dropping to 16px at width<=780px (examples/assets/graphite.html:918-930; examples/assets/graphite.html:940-943)
- the graphite fixture sets h1 to Bona Nova, Palatino, serif at 2.66667rem weight 700, h2 1.75rem, h3 1.25rem, h4-h6 1rem in Inter Variable weight 800 (examples/assets/graphite.html:1322-1330; examples/assets/graphite.html:1341-1366)
- the graphite fixture defines --font-size-link as calc(1rem*4/3) (examples/assets/graphite.html:915)
- the graphite fixture declares Inter Variable @font-face rules (weight 100 900, woff2-variations, font-display swap) per unicode range, normal and italic, and Bona Nova weight 700 rules (examples/assets/graphite.html:1635-1758; examples/assets/graphite.html:1761-1822)
- the graphite nav font size steps down through --nav-font-size from 28px to 8px across width breakpoints (examples/assets/graphite.html:986-995; examples/assets/graphite.html:1149-1156)
- the google fixture sets body/input/button to 14px arial,sans-serif and the bar to 13px/27px Roboto (examples/assets/google.html:22-26; examples/assets/google.html:45-48)
- the google fixture uses Google Sans,Roboto,Helvetica,Arial,sans-serif stacks (examples/assets/google.html:563; examples/assets/google.html:1050)
- the gosub fixture uses "Arial", sans-serif with h1 at 6em, then overridden to 72px (examples/assets/gosub.html:10; examples/assets/gosub.html:20-21; examples/assets/gosub.html:74-75)
- the servo-new-reduced-1 fixture sets body to 'Space Grotesk', sans-serif (examples/assets/servo-new-reduced-1.html:13)
- the pseudo fixture's .qqq class renders Font Awesome 6 Brands glyphs at 32px (examples/assets/pseudo.html:21-25; examples/assets/pseudo.html:80)

### facts-s03.md:146

- servo.css fixture sets headings h1-h6 to "Fira Sans", sans-serif and code/pre to "Fira Mono", monospace (examples/assets/servo.css:1-23; examples/assets/servo.css:229-234)
- servo.css body font stack begins "Fira Sans", BlinkMacSystemFont, -apple-system and ends sans-serif; body weight 400, line-height 1.5 (examples/assets/servo.css:220-227; examples/assets/servo.css:236-241)
- servo.html loads Fira Sans and Fira Mono from Google Fonts (examples/assets/servo.html:27)
- The custom-widget example uses `font-family: system-ui, sans` (examples/custom_widget.rs:179)
- TodoMVC page uses `font: 16px/1.4 system-ui, -apple-system, "Segoe UI", Roboto, sans-serif` and a 64px weight-200 heading (examples/preact/index.html:12; examples/preact/index.html:17)
- Reference page uses `15px/1.55` system stack and a ui-monospace code stack (examples/preact/core_dom_apis.html:10; examples/preact/core_dom_apis.html:23)
- text-decoration fixture exercises text-decoration-line/thickness/style, text-underline-offset/position and text-decoration-inset (examples/assets/text-decoration.html:48-102)

### facts-s04.md:226

- Browser chrome text is sans-serif: tab title 12px, urlbar 14px (16px on mobile), menu items 14px, status bar 12px (apps/browser/assets/browser.css:93-94; apps/browser/assets/browser.css:157-160; apps/browser/assets/browser.css:170; apps/browser/assets/browser.css:225-226; apps/browser/assets/browser.css:297-298)
- Suggestion section headers are 11px, weight 600, uppercase, letter-spacing 0.04em (apps/browser/assets/browser.css:329-336)
- FPS overlay is monospace 12px (apps/browser/assets/browser.css:269-270)
- about:history uses `system-ui, sans-serif` with a 24px/600 heading (apps/browser/assets/about-history.css:7; apps/browser/assets/about-history.css:9-11)
- rdme body font stack begins `-apple-system,BlinkMacSystemFont,"Segoe UI"` at 16px/1.5; monospace stack in `--fontStack-monospace`; weights in `--base-text-weight-*` (apps/readme/assets/github-markdown.css:7-10; apps/readme/assets/github-markdown.css:132-134)
- blitz-dom default stylesheet: inputs/selects/buttons `system-ui, sans-serif`, textarea monospace, h1–h6 from 2em down to 0.67em bold, code/pre `-moz-fixed` (packages/blitz-dom/assets/default.css:82-90; packages/blitz-dom/assets/default.css:301-347; packages/blitz-dom/assets/default.css:358-366; packages/blitz-dom/assets/default.css:662-667)
- seven_guis base is sans-serif 14px with a 36px/700 home title (examples/seven_guis/src/app.rs:169-195)
- todomvc body font `14px 'Helvetica Neue', Helvetica, Arial, sans-serif` weight 300; h1 100px weight 100 (examples/todomvc/src/todomvc.css:24; examples/todomvc/src/todomvc.css:33; examples/todomvc/src/todomvc.css:73-74)
- wgpu_texture uses `system-ui, sans` (examples/wgpu_texture/src/styles.css:7)
- WASM builds register bundled DejaVu Sans for sans-serif, serif, monospace and system-ui (examples/wasm_hello/src/lib.rs:75-100)

### facts-s05.md:200

- Generic base font size is 13px for monospace and 16px for other generics (packages/blitz-dom/src/font_metrics.rs:167-176)
- `ch` and `ic` metrics measure `'0'` and `'\u{6C34}'` advances scaled like Parley's shaped glyph advances (packages/blitz-dom/src/font_metrics.rs:104-161)
- A bullet font is always registered in the default font context (packages/blitz-dom/src/document.rs:389-391; packages/blitz-dom/src/lib.rs:32)
- `build_single_font_ctx` registers one font as fallback for SansSerif, Serif, Monospace and SystemUi with system fonts disabled (packages/blitz-dom/src/lib.rs:126-157)
- The new stylist device is seeded with the root element's font size and line height so rem/rlh units do not fall back to the 16px default (packages/blitz-dom/src/document.rs:2079-2100)

### facts-s06.md:190

- Generic font families map to Parley generics, with `None` as sans-serif (packages/blitz-dom/src/stylo_to_parley.rs:50-60)
- Font weight, width, style and variation settings convert to Parley values (packages/blitz-dom/src/stylo_to_parley.rs:87-112)
- `font-variant-ligatures`, `-caps`, `-position`, `-numeric` and `-east-asian` map to OpenType feature tags (packages/blitz-dom/src/stylo_to_parley.rs:134-262)
- Line height maps `normal`, number and length to Parley line heights (packages/blitz-dom/src/stylo_to_parley.rs:407-412)
- Letter and word spacing resolve against the font size (packages/blitz-dom/src/stylo_to_parley.rs:415-424)
- Text locale comes from the computed `_x_lang` value, which the `lang` attribute sets (packages/blitz-dom/src/stylo_to_parley.rs:495; packages/blitz-dom/src/stylo.rs:1197-1199)
- `text-align` and `text-align-last` map to Parley alignment (packages/blitz-dom/src/stylo_to_parley.rs:304-329)
- Word break, line break, overflow wrap, wrap mode and white-space collapse map to Parley settings (packages/blitz-dom/src/stylo_to_parley.rs:331-346; packages/blitz-dom/src/stylo_to_parley.rs:467-484)

### facts-s07.md:129

- The layout engine resolves `line-height: normal` as 1.2 × font size (packages/blitz-dom/src/layout/mod.rs:116-120)
- List bullets use the font family `"Bullet, monospace, sans-serif"` (packages/blitz-dom/src/layout/list.rs:15; packages/blitz-dom/src/layout/list.rs:160-167; packages/blitz-dom/src/layout/construct.rs:1086-1092)
- List markers: decimal `"N. "`, lower/upper-alpha, disc `•`, circle `◦`, square `▪`, disclosure-open `▾`, disclosure-closed `▸`, other names `□` (packages/blitz-dom/src/layout/list.rs:116-158)

### facts-s08.md:170

- Text input editors are created with `parley::PlainEditor::new(16.0)` (packages/blitz-dom/src/node/text.rs:79)
- observed absent — font-family declarations · searched: `font-family|font_family` over the 16 listed files (no match)

### facts-s09.md:146

- font emboldening is enabled by the font-embolden feature, or apple-font-embolden on macOS and iOS (packages/blitz-paint/Cargo.toml:18-19; packages/blitz-paint/src/lib.rs:22-26)
- embolden strength is 0.015125 and 0.0121 times the CSS font size, each capped at 0.3, and hinting is turned off when emboldening (packages/blitz-paint/src/text.rs:631-642)
- auto text-decoration thickness is font-size / 10 with a 1px minimum, floored to whole device pixels (packages/blitz-paint/src/text.rs:249-272)
- overline and line-through positions use the font's OS/2 usWinAscent, cached per font face (packages/blitz-paint/src/text.rs:70-95; packages/blitz-paint/src/text.rs:485-535)
- observed absent — font family declarations · searched: `font_family|font-family|FontFamily` over the 32 listed s09 files

### facts-s10.md:203

- `document.fonts` is a stub FontFaceSet where all fonts report as loaded (packages/blitz-vibey-script/src/runtime.rs:1057-1071)
- observed absent — font-family declarations outside test fixtures · searched: `font-family|font_family` over the 32 slice files (matches only packages/blitz-vibey-script/tests/dom.rs)

### facts-s11.md:177

- A custom `FontContext` can be set; on WASM a context with bundled fonts must be provided, using `build_single_font_ctx` for one font (packages/dioxus-native/src/config.rs:56-64)
- Font features: system-fonts, woff, complex-scripts (dictionary line-breaking), font-embolden and apple-font-embolden (packages/dioxus-native/Cargo.toml:20-25; packages/dioxus-native/Cargo.toml:45-46)

### facts-s12.md:163

- `blitz-dom` is built with the `system-fonts` feature for these tests (tests/blitz-tests/Cargo.toml:16)
- Without the `system-fonts` feature text measures 0x0 and font-dependent assertions pass vacuously; the feature is stated to be enabled by default when testing the whole workspace (tests/blitz-tests/tests/br_trailing_line.rs:11-13; tests/blitz-tests/tests/inline_box_baseline.rs:4-6)
- `rem` resolves against the root element's font-size, including after viewport and hidpi changes; the stylist's initial root font-size is 16px (tests/blitz-tests/tests/rem_after_viewport_change.rs:1-8; tests/blitz-tests/tests/rem_after_viewport_change.rs:49-77)
- observed absent — a project type scale or font tokens · searched: `font-family` over the 61 slice files (one fixture hit, `sans-serif`)

### facts-s13.md:174

- out of slice — no typography is defined in these files

## §Spacing

### facts-s01.md:163

- Example fixtures use grid/flex gaps of 24px and 8px and 24px body padding (examples/assets/border-styles.html:9; examples/assets/border-styles.html:17; examples/assets/cursor.html:9; examples/assets/cursor.html:23)
- out of slice — the project's own spacing scale

### facts-s02.md:124

- the graphite fixture's :root sets --max-width 1200px, --max-extended-width 1600px, --max-width-reading-material 800px, --variable-px Min(1px, .15vw), --page-edge-padding 40px, --border-thickness 2px and --feature-box-padding 80 (examples/assets/graphite.html:907-914)
- the graphite fixture lowers --page-edge-padding to 28px and --feature-box-padding to 40 at width<=780px, and --page-edge-padding to 20px for print or width<=500px (examples/assets/graphite.html:932-938; examples/assets/graphite.html:946-951)
- the graphite fixture spaces sibling main sections by calc(120*var(--variable-px)) (examples/assets/graphite.html:1203-1205)
- the hr fixture gives hr a 24px vertical margin and the body 16px 40px 40px padding (examples/assets/hr.html:8; examples/assets/hr.html:25-29)

### facts-s03.md:155

- servo.css `.columns.is-variable` sets `--columnGap: 0.75rem`, applied as column padding and negative margin (examples/assets/servo.css:1577-1585)
- servo.css `.container.is-fluid` uses 32px side padding (examples/assets/servo.css:342-345)
- servo.css `.button` padding is `calc(0.5em - 1px)` vertical and `calc(0.75em - 1px)` horizontal (examples/assets/servo.css:38-41)
- TodoMVC page: `.app` max-width 520px with 40px auto margin; list items padding 14px 16px with 12px gap (examples/preact/index.html:16; examples/preact/index.html:24)
- Reference page: body max-width 60rem, padding 0 1.25rem (examples/preact/core_dom_apis.html:11-13)

### facts-s04.md:238

- Browser urlbar padding 6px, gap 6px; tab padding 0 8px; menu padding 8px with items 8px 12px gap 8px; suggestion rows 6px 12px (apps/browser/assets/browser.css:39; apps/browser/assets/browser.css:132-133; apps/browser/assets/browser.css:214; apps/browser/assets/browser.css:223; apps/browser/assets/browser.css:231; apps/browser/assets/browser.css:338)
- about:history padding 32px 48px; list items 12px 16px with gap 12px and margin-bottom 8px (apps/browser/assets/about-history.css:4; apps/browser/assets/about-history.css:49-54)
- rdme spacing tokens `--base-size-4/8/16/24/40` equal 0.25–2.5rem (apps/readme/assets/github-markdown.css:2-6)
- rdme overrides markdown body to max-width 892px with padding 16px 32px (apps/readme/assets/blitz-markdown-overrides.css:1-5)
- seven_guis cards pad 24px 32px with 16px gap; home pads 48px 32px 64px (examples/seven_guis/src/tasks/counter.rs:37-41; examples/seven_guis/src/app.rs:181)
- blitz-dom default body margin 8px (packages/blitz-dom/assets/default.css:264-267)

### facts-s05.md:207

- out of slice — spacing scale or tokens

### facts-s06.md:200

- `hspace` and `vspace` map to horizontal and vertical margins on `embed`, `img`, `object`, `marquee` and image inputs (packages/blitz-dom/src/stylo.rs:1073-1099)
- Body `marginwidth`, `marginheight`, `leftmargin` and `topmargin` map to pixel margins; `rightmargin` and `bottommargin` are deliberately ignored (packages/blitz-dom/src/stylo.rs:1143-1180)

### facts-s07.md:134

- out of slice — the slice states no spacing scale

### facts-s08.md:174

- Overlay scrollbar thumb geometry: thickness 10.0, thin thickness 6.0, margin 2.0 and minimum length 32.0 CSS px (packages/blitz-dom/src/node/scrollbar.rs:122-126)
- Single-line text inputs are vertically centered within their content box (packages/blitz-dom/src/node/node.rs:807-825)

### facts-s09.md:153

- outside list markers using a character are padded 8 CSS px from the item's border box (packages/blitz-paint/src/render.rs:959-970)
- the double text decoration places its second line thickness + 1 CSS px away (packages/blitz-paint/src/text.rs:318-328)

### facts-s10.md:207

- out of slice — no spacing scale or spacing tokens appear in these files

### facts-s11.md:181

- out of slice — no spacing scale or tokens; these files only convert CSS margin, padding and gap values to taffy

### facts-s12.md:169

- out of slice — no spacing scale or tokens in this test crate

### facts-s13.md:177

- out of slice — no spacing scale is defined in these files

## §Depth Strategy

### facts-s01.md:167

- A fixture exercises `filter: drop-shadow(...)` and `backdrop-filter: blur(10px)` (examples/assets/filters.html:37; examples/assets/filters.html:80)
- A captured GitHub fixture uses `z-index: 2147483647` on a fixed progress bar (examples/assets/github_profile_reduced2.html:3-10)
- out of slice — the project's own elevation scale

### facts-s02.md:130

- the google fixture's search box takes box-shadow 0 1px 6px rgba(32,33,36,.28) in its active state (examples/assets/google.html:3151)
- the google fixture uses layered shadows such as 0 1px 3px 1px rgba(66,64,67,.15),0 1px 2px 0 rgba(60,64,67,.3) (examples/assets/google.html:364-365)
- the google fixture's bar sits at z-index 986 and the graphite header at z-index 1000 (examples/assets/google.html:47; examples/assets/graphite.html:961)
- the hr fixture exercises an inset shadow and an outset glow (examples/assets/hr.html:137; examples/assets/hr.html:146)

### facts-s03.md:162

- servo.css `.box` uses a two-layer shadow `0 0.5em 1em -0.125em ... , 0 0px 0 1px ...` (examples/assets/servo.css:3041-3044)
- TodoMVC `.card` uses `box-shadow: 0 2px 4px rgba(0,0,0,.15)` (examples/preact/index.html:18)
- The custom-widget example stacks layers with z-index 100 (header), 10 (overlay) and -10 (underlay) (examples/custom_widget.rs:195-198; examples/custom_widget.rs:201-206; examples/custom_widget.rs:213-217)
- The transforms example's `.overlay` uses z-index 99 (examples/transforms.rs:277-282)
- Box-shadow fixtures cover outset, spread, layered and inset shadows (examples/box_shadow.rs:26-38; examples/assets/shadow.html:31-114)

### facts-s04.md:246

- Browser z-index: tooltip, menu and suggestions 100; FPS overlay 50; status bar 10 (apps/browser/assets/browser.css:69; apps/browser/assets/browser.css:218; apps/browser/assets/browser.css:274; apps/browser/assets/browser.css:304; apps/browser/assets/browser.css:324)
- Browser menu and suggestions share `box-shadow: 0 2px 8px rgba(0, 0, 0, 0.15)` (apps/browser/assets/browser.css:217; apps/browser/assets/browser.css:323)
- seven_guis cards use `0 2px 8px rgba(0, 0, 0, 0.08)`; circle dialog uses 0.10 (examples/seven_guis/src/tasks/counter.rs:42; examples/seven_guis/src/tasks/timer.rs:76; examples/seven_guis/src/tasks/flight_booker.rs:129; examples/seven_guis/src/tasks/circle_drawer.rs:205)
- todomvc uses layered shadows on the app and footer (examples/todomvc/src/todomvc.css:48; examples/todomvc/src/todomvc.css:279)
- wgpu_texture layers overlay z-index 10, underlay -10, header 100 (examples/wgpu_texture/src/styles.css:26; examples/wgpu_texture/src/styles.css:34; examples/wgpu_texture/src/styles.css:45)
- blitz-dom default dialog backdrop `rgba(0, 0, 0, 0.1)` (packages/blitz-dom/assets/default.css:989-992)

### facts-s05.md:210

- The engine builds paint children and stacking contexts after layout and transforms (packages/blitz-dom/src/resolve.rs:119-123); `StackingContext` and `HoistedPaintChild` are public re-exports (packages/blitz-dom/src/lib.rs:86)
- out of slice — elevation or shadow tokens

### facts-s06.md:204

- observed absent — z-index or shadow handling · searched: `border-radius|border_radius|box-shadow|box_shadow|z-index|z_index` over the 17 listed s06 files

### facts-s07.md:137

- out of slice — the slice implements CSS z-index paint order (packages/blitz-dom/src/layout/paint_tree.rs:420-462) but states no project depth scale

### facts-s08.md:178

- A node is a stacking-context root for opacity not equal to 1, fixed or sticky position, z-index on relative or absolute (or static flex/grid items), any transform/rotate/scale/translate, atomic paint effects, or `isolation: isolate` (packages/blitz-dom/src/node/node.rs:1203-1246)
- Atomic paint effects are opacity, filter, clip-path and mask-image (packages/blitz-dom/src/node/node.rs:1248-1277)
- Hit-testing walks positive-z hoisted children, then paint children in reverse, then negative-z hoisted children (packages/blitz-dom/src/node/node.rs:1385-1447)
- observed absent — shadow or elevation tokens · searched: `shadow|elevation` over the 16 listed files (no match)

### facts-s09.md:157

- outset box shadows are clipped when opacity is below 1 or the background is not opaque, and blurred shadows use the averaged border radius per a TODO (packages/blitz-paint/src/render/box_shadow.rs:15-24; packages/blitz-paint/src/render/box_shadow.rs:77-82)
- inset box shadows are drawn by filling the padding box then cutting a blurred hole with Compose::DestOut (packages/blitz-paint/src/render/box_shadow.rs:89-156)
- children paint in stacking order: negative z-index hoisted children, regular paint children, then positive z-index hoisted children (packages/blitz-paint/src/render.rs:1007-1070)
- opacity, filter and backdrop-filter are applied via a layer clipped to the border box expanded by the filter area (packages/blitz-paint/src/render.rs:496-528)

### facts-s10.md:210

- observed absent — shadow or elevation definitions · searched: `box-shadow|shadow|elevation|z-index` over the 32 slice files

### facts-s11.md:184

- observed absent — shadows, elevation or z-index tokens · searched: `radius|shadow|elevation|z-index` over the 21 listed s11 files (only a touch-point `radius` matched)

### facts-s12.md:172

- An outset box shadow takes the element's shape corner for corner, so a shadow with no blur and no spread hides behind the element ("what an elevation of 0 relies on"); expected pixels come from Chromium (tests/blitz-tests/tests/outset_box_shadow_shape.rs:1-6)
- Positioned descendants with `z-index: auto` share one paint level per CSS 2.1 Appendix E and paint in tree order (tests/blitz-tests/tests/paint_order.rs:1-2)

### facts-s13.md:180

- out of slice — no depth strategy is defined in these files

## §Border Radius

### facts-s01.md:172

- Fixtures exercise uniform, percentage, per-corner and elliptical radii (examples/assets/border.html:16-20; examples/assets/border-styles.html:65-73; examples/assets/cursor.html:34)
- out of slice — the project's own radius scale

### facts-s02.md:136

- the google fixture's search box has border-radius 24px and its buttons 4px (examples/assets/google.html:3151; examples/assets/google.html:3491)
- the google fixture's style sheet also uses 2px, 8px and 50% radii (examples/assets/google.html:100; examples/assets/google.html:923; examples/assets/google.html:459)
- the graphite fixture's newsletter inputs have border-radius 0 and its carousel dots 50% (examples/assets/graphite.html:341; examples/assets/graphite.html:577)
- the hr fixture's pill cases use border-radius 999px (examples/assets/hr.html:128)
- the inline-flex-transform fixture's .button uses border-radius 1.5rem 0 (examples/assets/inline-flex-transform.html:22)
- the input fixture's third div uses border-radius 100px (examples/assets/input.html:9)

### facts-s03.md:169

- servo.css: `.button` 4px, `.box` 6px, loader 9999px (examples/assets/servo.css:31; examples/assets/servo.css:3043; examples/assets/servo.css:85)
- shadow fixture `.card` 12px (examples/assets/shadow.html:24)
- TodoMVC filter buttons 3px (examples/preact/index.html:37)
- Reference page `code` 4px and `.tag` 999px (examples/preact/core_dom_apis.html:27; examples/preact/core_dom_apis.html:39)
- The transforms example `.button` uses `border-radius: 1.5rem 0` (examples/transforms.rs:291)

### facts-s04.md:254

- Browser: tabs 4px top corners, tooltip 4px, close button 3px, urlbar input 4px, icon buttons 4px, menu 4px, status bar 3px top-right (apps/browser/assets/browser.css:45-46; apps/browser/assets/browser.css:67; apps/browser/assets/browser.css:104; apps/browser/assets/browser.css:166; apps/browser/assets/browser.css:180; apps/browser/assets/browser.css:216; apps/browser/assets/browser.css:295)
- About pages: clear button 6px, history rows 8px, newtab search input 8px (apps/browser/assets/about-history.css:27; apps/browser/assets/about-history.css:48; apps/browser/assets/about-newtab.css:29)
- seven_guis: cards 8px, buttons 6px, inputs 4px, task cards 6px, tags 3px (examples/seven_guis/src/tasks/counter.rs:40; examples/seven_guis/src/tasks/counter.rs:60; examples/seven_guis/src/tasks/temp_converter.rs:69; examples/seven_guis/src/app.rs:218; examples/seven_guis/src/app.rs:259)
- transparent card 16px (examples/transparent/src/app.rs:121)
- blitz-dom default button radius 1px (packages/blitz-dom/assets/default.css:102)

### facts-s05.md:214

- observed absent — border-radius values or tokens · searched: `border-radius|border_radius` over the 15 s05 files

### facts-s06.md:207

- observed absent — border-radius handling · searched: `border-radius|border_radius|box-shadow|box_shadow|z-index|z_index` over the 17 listed s06 files
- The `border` attribute on `img`, `object` and image inputs maps to four solid pixel borders (packages/blitz-dom/src/stylo.rs:1120-1141)

### facts-s07.md:140

- observed absent — border radius values · searched: `radius` over the 8 slice files

### facts-s08.md:184

- observed absent — border radius values · searched: `border.radius|radius` over the 16 listed files (no match)

### facts-s09.md:163

- per-corner elliptical radii are resolved from computed border-*-radius values and scaled to device pixels (packages/blitz-paint/src/render.rs:1248-1265)
- checkbox frames use a corner radius of 2 times the control scale (packages/blitz-paint/src/render/form_controls.rs:28-33)
- scrollbar thumbs are fully rounded with radius half their thickness (packages/blitz-paint/src/render.rs:813-823)
- inset() clip-path ignores border-radius per a TODO (packages/blitz-paint/src/render/clip_path.rs:167-168)

### facts-s10.md:213

- observed absent — radius definitions · searched: `border-radius|radius` over the 32 slice files

### facts-s11.md:187

- observed absent — border-radius values or tokens · searched: `radius|shadow|elevation|z-index` over the 21 listed s11 files (only a touch-point `radius` matched)

### facts-s12.md:176

- out of slice — only per-fixture `border-radius` values, no radius tokens

### facts-s13.md:183

- out of slice — no border radius is defined in these files

## §Motion

### facts-s01.md:176

- Fixtures exercise `@keyframes` animations (big-small 2s alternate, gradient-animation 20s, spin 4s linear) (examples/assets/animated_layout.html:21; examples/assets/animated_layout.html:28-43; examples/assets/animation.html:32-38)
- Fixtures exercise transitions on transform and filter with `:hover`/`:active` nesting (examples/assets/animation.html:17-21; examples/assets/animation.html:46-51; examples/assets/filters.html:16-17)
- The BBC fixture contains `prefers-reduced-motion` media queries (examples/assets/bbc_reduced.html:10; examples/assets/bbc_reduced.html:11; examples/assets/bbc_reduced.html:17)

### facts-s02.md:144

- the google fixture declares keyframes gb__a, g-bubble-show, g-bubble-hide, g-snackbar-show/hide and qli spinner animations (examples/assets/google.html:49-56; examples/assets/google.html:1589; examples/assets/google.html:1597; examples/assets/google.html:1700; examples/assets/google.html:1709; examples/assets/google.html:2757)
- the google fixture uses transitions such as box-shadow 250ms and transform/opacity/visibility .3s ease-in-out (examples/assets/google.html:597; examples/assets/google.html:1457)
- the graphite fixture transitions carousel images with transform .5s and rotates the open details marker 90deg (examples/assets/graphite.html:498-500; examples/assets/graphite.html:1243-1245)
- the gosub fixture transitions link border-bottom over 0.3s ease-in-out (examples/assets/gosub.html:49)
- the inline-flex-transform fixture transitions scale, translate, rotate and transform over 0.15s and scales to 1.2 on hover (examples/assets/inline-flex-transform.html:27-28; examples/assets/inline-flex-transform.html:37-38)
- observed absent — a prefers-reduced-motion rule · searched: `prefers-reduced-motion` over the 21 s02 files

### facts-s03.md:176

- servo.css defines `@keyframes spinAround` used by `.button.is-loading::after` at 500ms infinite linear (examples/assets/servo.css:82-83; examples/assets/servo.css:328-335)
- servo.css uses 86ms ease-out transitions on background-color, opacity, transform (examples/assets/servo.css:6096-6098)
- svg_native transitions `fill 0.15s` on hover (examples/svg_native.rs:14-15)
- The transforms example `.button` transitions `filter, scale` over 0.15s and on hover applies `brightness(90%)` and `scale: 1.2` (examples/transforms.rs:302-303; examples/transforms.rs:312-315)
- observed absent — reduced-motion handling · searched: `prefers-reduced-motion` over the 32 slice files

### facts-s04.md:261

- todomvc transitions label color over 0.4s and destroy button color over 0.2s ease-out (examples/todomvc/src/todomvc.css:214; examples/todomvc/src/todomvc.css:234)
- observed absent — transitions or animations in browser chrome and seven_guis styles · searched: `transition|animation|@keyframes` over apps/browser/assets/*.css and examples/seven_guis/src/**/*.rs
- observed absent — reduced-motion handling · searched: `prefers-reduced-motion` over the 86 slice files

### facts-s05.md:217

- Overlay scrollbars show at full opacity on scroll and fade out after a delay, documented as Chromium's overlay timings (packages/blitz-dom/src/document.rs:1752-1769)
- Finished scrollbar fades (`FADE_DELAY + FADE_DURATION`) are dropped each resolve (packages/blitz-dom/src/resolve.rs:66-72)
- Active CSS animations/transitions, canvases, animating sub-documents, custom widgets, scroll animations and scrollbar fades keep the document animating (packages/blitz-dom/src/document.rs:2019-2036)
- observed absent — reduced-motion handling · searched: `prefers-reduced-motion|reduced_motion` over the 15 s05 files

### facts-s06.md:211

- Smooth scrolls run 300 ms on a cubic ease-in-out curve (packages/blitz-dom/src/scrolling.rs:113-123; packages/blitz-dom/src/scrolling.rs:433-434)
- `scroll-behavior: smooth` in computed style makes `Auto` scrolls animate (packages/blitz-dom/src/scrolling.rs:532-549)
- Touch flings decelerate per frame until velocity drops below 0.1 (packages/blitz-dom/src/scrolling.rs:724-747)
- CSS animations and transitions move from pending to running to finished by the current time during style resolution (packages/blitz-dom/src/stylo.rs:105-123)
- observed absent — reduced-motion preference handling · searched: `prefers-reduced-motion|reduced_motion|prefers_reduced` over the 17 listed s06 files

### facts-s07.md:143

- observed absent — transitions or animation · searched: `transition|animation` over the 8 slice files

### facts-s08.md:187

- Overlay scrollbars stay opaque for `FADE_DELAY` = 500 ms after their last activity, then fade linearly over `FADE_DURATION` = 200 ms (packages/blitz-dom/src/node/scrollbar.rs:12-26)
- A custom widget returning `true` from `requires_redraw` causes continuous redraw scheduling, e.g. for animation (packages/blitz-dom/src/node/custom_widget.rs:100-106)

### facts-s09.md:169

- the shell's animation clock is seconds since the first animation-time query, and frames keep redrawing while the document is animating (packages/blitz-shell/src/window.rs:280-288; packages/blitz-shell/src/window.rs:437-439)
- overlay scrollbar thumbs appear on scroll and fade out after a delay via scrollbar_opacity (packages/blitz-paint/src/render.rs:715-720; packages/blitz-paint/src/render.rs:742-745)
- the test harness drives animation from a controlled clock advanced by tick (packages/blitz-test-harness/src/harness.rs:108-123)
- observed absent — reduced-motion handling · searched: `reduced.motion|prefers` over the 32 listed s09 files

### facts-s10.md:216

- `requestAnimationFrame` is a timer approximated as 16ms away, passing 16 as the timestamp (packages/blitz-vibey-script/src/runtime.rs:2084-2109)
- scroll behaviour values `auto`, `instant`, `smooth` are parsed and passed to blitz-dom scroll calls (packages/blitz-vibey-script/src/dom/element.rs:1059-1074)

### facts-s11.md:190

- Programmatic scrolling supports `Smooth` and `Instant` behaviour (packages/dioxus-native-dom/src/events.rs:235-238; packages/dioxus-native-dom/src/events.rs:274-277)
- Animation and transition event data are `unimplemented!()` (packages/dioxus-native-dom/src/events.rs:70-72; packages/dioxus-native-dom/src/events.rs:122-124)

### facts-s12.md:179

- Smooth scrolling follows `scroll-behavior` on the root or element style; `auto` behavior jumps (tests/blitz-tests/tests/fragment_navigation.rs:264-293; tests/blitz-tests/tests/fragment_navigation.rs:374-383)
- A wheel event over a scroller cancels an in-progress smooth scroll (tests/blitz-tests/tests/fragment_navigation.rs:356-372)
- Overlay scrollbars appear on scroll activity and fade out after a delay; hovering where a hidden thumb would be does not summon it (tests/blitz-tests/tests/scrollbars.rs:1-4; tests/blitz-tests/tests/scrollbar_drag.rs:136-166)
- The scroll thumb changes appearance on hover and while dragged (tests/blitz-tests/tests/scrollbar_drag.rs:168-222)

### facts-s13.md:186

- out of slice — no motion is defined in these files

## §Iconography

### facts-s01.md:181

- A fixture enumerates 36 CSS cursor values as "the built-in cursor styles supported by blitz" (examples/assets/cursor.html:54-91)
- The captured GitHub fixture uses inline SVG octicons (examples/assets/github_profile_reduced2.html:65-67)
- out of slice — the project's own icon set

### facts-s02.md:152

- Font Awesome icon classes are used in the gosub, pseudo and servo-new fixtures (examples/assets/gosub.html:107-109; examples/assets/pseudo.html:71-73; examples/assets/servo-new.html:206-218)
- the pseudo fixture defines .fa-github:before and .gh:before with content "\f09b" (examples/assets/pseudo.html:55-65)
- the google fixture draws its icons as inline SVG paths in 24-unit viewBoxes (examples/assets/google.html:3086-3095; examples/assets/google.html:3159-3161; examples/assets/google.html:3219-3221)
- the graphite fixture positions icons from one sprite atlas through an --atlas-index custom property (examples/assets/graphite.html:661-666; examples/assets/graphite.html:748-753; examples/assets/graphite.html:1948)
- the graphite fixture embeds SVG icons as data URIs in CSS (examples/assets/graphite.html:1229; examples/assets/graphite.html:1627)

### facts-s03.md:183

- servo.html fixture uses Font Awesome classes (`fab fa-github`, `fab fa-mastodon`, `fab fa-twitter`, `fas fa-link`) loaded from use.fontawesome.com (examples/assets/servo.html:26; examples/assets/servo.html:154; examples/assets/servo.html:161; examples/assets/servo.html:168; examples/assets/servo.html:253)
- svg.html fixture holds inline SVG icons using `fill="currentColor"` (examples/assets/svg.html:4-12)
- svg_native example fills an icon circle with `currentColor` set from the svg's `color` (examples/svg_native.rs:27-29)
- The TodoMVC destroy button renders `×` as its glyph (examples/preact/index.html:99-103)

### facts-s04.md:266

- Browser toolbar icons are SVG assets rotate-cw, house, arrow-left, arrow-right, ellipsis-vertical, external-link, code, and camera (feature-gated) (apps/browser/src/icons.rs:3-11)
- `IconButton` renders an `img.urlbar-icon` (20px high) inside a clickable div (apps/browser/src/icons.rs:13-40; apps/browser/assets/browser.css:200-202)
- Menu item icons are 16x16 (apps/browser/assets/browser.css:243-246)
- Favicons render as 16x16 images (apps/browser/src/tab.rs:269-281)
- Tab close and new-tab controls are text glyphs "×" and "+" (apps/browser/src/tab_strip.rs:92-96; apps/browser/src/tab_strip.rs:102-106)
- Bundle icons are `blitz-logo.png` and `blitz-logo.ico` (apps/browser/Dioxus.toml:7)
- rdme styles GitHub octicons (apps/readme/assets/github-markdown.css:139-157)

### facts-s05.md:223

- The cursor is taken from the CSS `cursor` keyword, else Text for text inputs, Pointer inside links, Text over selectable text, Default otherwise (packages/blitz-dom/src/document.rs:2147-2201)
- `favicon_url` returns the `href` of the first `<link>` whose `rel` contains `icon` (packages/blitz-dom/src/document.rs:582-597)

### facts-s06.md:218

- CSS `cursor` keywords map one-to-one to `CursorIcon` values, and `cursor: none` maps to no cursor (packages/blitz-dom/src/stylo_to_cursor_icon.rs:4-49)

### facts-s07.md:146

- out of slice — no icon set is referenced

### facts-s08.md:191

- observed absent — icon assets or icon handling · searched: `icon` over the 16 listed files (no match)

### facts-s09.md:175

- cursor icons are winit CursorIcon values; None hides the cursor and resets it to Default (packages/blitz-shell/src/lib.rs:101-112)

### facts-s10.md:220

- cursor icons use the `cursor-icon` crate; `ShellProvider::set_cursor` takes an optional `CursorIcon` (packages/blitz-traits/Cargo.toml:21; packages/blitz-traits/src/shell.rs:3; packages/blitz-traits/src/shell.rs:13-15)

### facts-s11.md:194

- observed absent — icons or icon sets · searched: `icon` over the 21 listed s11 files

### facts-s12.md:185

- out of slice — only an inline SVG test icon fixture, no icon set

### facts-s13.md:189

- out of slice — no iconography is defined in these files

## §Surface: desktop-native

### facts-s01.md:186

- The flake's default package is the browser app whose binary is `blitz`, wrapped with winit/wgpu runtime libraries on Linux (flake.nix:40-53; flake.nix:117-121)
- Desktop bundles are produced for Windows (NSIS .exe), macOS (.dmg) and Linux (.AppImage) on x86_64 and aarch64 (.github/workflows/publish-browser.yml:46-81)
- out of slice — the desktop UI's own visual design

### facts-s03.md:189

- Dioxus examples open a native window through `dioxus_native::launch` (examples/box_shadow.rs:4; examples/custom_widget.rs:18)
- HTML documents open in a native window via `WindowConfig::new(Box::new(doc), VelloWindowRenderer::new())` (examples/inner_html.rs:28-30; examples/preact_script.rs:37)
- Custom GPU content composites with HTML layers at `opacity: 0.8` in the canvas container (examples/custom_widget.rs:190-193)

### facts-s04.md:275

- The browser window on macOS uses a transparent, unified, hidden-title titlebar with full-size content view (apps/browser/src/main.rs:82-89)
- On macOS the tabstrip gets `merged-titlebar` with 90px left padding and 44px height (apps/browser/src/tab_strip.rs:9-12; apps/browser/assets/browser.css:28-31)
- The browser window title follows the active tab's display title (apps/browser/src/main.rs:166; apps/browser/src/main.rs:173)
- rdme titles its window "README for" plus the last path segment, and toggles light/dark theme with Ctrl/Cmd+T (apps/readme/src/main.rs:95-98; apps/readme/src/readme_application.rs:124-131; apps/readme/src/readme_application.rs:175)
- The transparent example opens a 360x300 decoration-less transparent window and closes via the shell provider (examples/transparent/src/app.rs:9-31; examples/transparent/src/app.rs:49-52)
- counter, seven_guis and todomvc launch through `dioxus_native::launch` (examples/counter/src/main.rs:15-17; examples/seven_guis/src/main.rs:4-7; examples/todomvc/src/main.rs:15-19)

### facts-s09.md:178

- windows are winit windows; the shell sets the window title from the document's title node when it was set before the window existed (packages/blitz-shell/src/window.rs:200-206)
- the shell provider exposes minimize, maximize, decorations toggle and window drag for custom titlebars (packages/blitz-shell/src/lib.rs:139-153; packages/blitz-shell/src/event.rs:28-32)
- the initial theme is the window's theme, defaulting to Light (packages/blitz-shell/src/window.rs:181-182)

### facts-s10.md:223

- blitz launches HTML into a native window with `WindowConfig` and the Vello window renderer (packages/blitz/src/lib.rs:106-131)
- the default viewport is window size (0, 0), hidpi scale 1.0, zoom 1.0, Light scheme (packages/blitz-traits/src/shell.rs:84-93)
- the viewport's logical size is the physical window size divided by hidpi × zoom (packages/blitz-traits/src/shell.rs:110-127)
- devtools can draw browser-style overlays of content, padding, border and margin for the hovered or a chosen node (packages/blitz-traits/src/devtools.rs:11-17)
- JS `innerWidth`/`innerHeight` are window size ÷ scale, `outerWidth`/`outerHeight` alias them, `devicePixelRatio` is the scale (packages/blitz-vibey-script/src/runtime.rs:1405-1410; packages/blitz-vibey-script/src/runtime.rs:2113-2135)

### facts-s11.md:197

- The surface is a winit window built from `WindowAttributes`, titled from dioxus-cli-config or "Dioxus App" (packages/dioxus-native/src/config.rs:17-20; packages/dioxus-native/src/lib.rs:236-237)
- Window compositing alpha mode is configurable, for example for transparent windows; unsupported modes are ignored by the renderer (packages/dioxus-native/src/config.rs:66-74)
- Every document gets the blitz `DEFAULT_CSS` user-agent stylesheet (packages/dioxus-native-dom/src/dioxus_document.rs:95-96)
- On wasm32 the same surface is a canvas appended to the page body, sized from host CSS (packages/dioxus-native/src/config.rs:22-34)
- Optional `scrollbars` feature forwards to blitz-paint (packages/dioxus-native/Cargo.toml:47)

## §Surface: mobile-native

### facts-s01.md:191

- An Android aarch64 APK is bundled with `--android --package-types apk --no-default-features --features android-defaults` (.github/workflows/publish-browser.yml:82-88)
- CI builds (does not test) for `aarch64-apple-ios` and `aarch64-linux-android` (.github/workflows/ci.yml:159-174)
- out of slice — the mobile UI's own visual design

### facts-s04.md:283

- `IS_MOBILE` is true for Android and iOS and adds the `mobile` class to the frame (apps/browser/src/main.rs:48; apps/browser/src/main.rs:172)
- On mobile the urlbar input grows to 16px font with 8px 6px padding (apps/browser/assets/browser.css:140-161)
- The Android hardware back button navigates back in the active tab (apps/browser/src/main.rs:140-145)
- On mobile, screenshots save to a default file name without a dialog (apps/browser/src/capture.rs:109-110)

## §Surface: none observed

### facts-s02.md:159

- observed absent — a UI surface of this repository's own product; every listed file is a standalone example HTML document · searched: `<html` over the 21 s02 files

### facts-s05.md:227

- observed absent — an app surface (window, CLI entry, web bindings) · searched: `winit|wgpu|fn main|clap|wasm_bindgen` over the 15 s05 files

### facts-s06.md:221

- observed absent — an application entry point or window creation · searched: `fn main|winit|EventLoop` over the 17 listed s06 files

### facts-s07.md:149

- observed absent — an application surface (entry point, CLI, window, webview) · searched: `fn main|clap|winit|wasm_bindgen|webview` over the 8 slice files

### facts-s08.md:194

- observed absent — any UI surface entry point (binary, CLI, window) · searched: `fn main|\[\[bin\]\]|clap` over the 16 listed files (no match)

### facts-s12.md:188

- observed absent — an application surface (entry point, window or web bootstrap) · searched: `fn main|winit|wasm_bindgen|launch\(` over the 61 slice files (one comment-only hit naming the winit resize path)

## §Surface: cli

### facts-s03.md:194

- `paint_bench` prints `Loaded {url} at {w}x{h}@{scale}x; running {iters} paint iterations ({backend} backend)` then one stats line per phase (examples/paint_bench.rs:113-115; examples/paint_bench.rs:34)
- `screenshot` prints the URL, per-phase `... in {n}ms` lines, `Screenshot is ({w}x{h})` and `Written to {path}` (examples/screenshot.rs:31; examples/screenshot.rs:203; examples/screenshot.rs:146-148)

### facts-s04.md:293

- bump prints "Bumped anyrender versions" or "Bumped blitz versions" on success and errors to stderr (apps/bump/src/main.rs:25-31; apps/bump/src/main.rs:100; apps/bump/src/main.rs:109)

### facts-s13.md:192

- The runner is a command-line binary with `fn main` (wpt/runner/src/main.rs:457)
- Test names are wrapped in OSC 8 hyperlinks to `https://wpt.live/{name}` only when `supports_hyperlinks::on(Stdout)` (stdout is a terminal, honouring `FORCE_HYPERLINK`) (wpt/runner/src/main.rs:54-63; wpt/runner/src/main.rs:360-370)
- Output colors come from `owo-colors` (wpt/runner/src/main.rs:24; wpt/runner/Cargo.toml:38)

## §Surface: web-spa

### facts-s04.md:289

- seven_guis and todomvc WASM pages fill the window with a canvas on `#f5f5f5` (examples/seven_guis/index.html:7-10; examples/todomvc/index.html:7-10)
- wasm_hello's canvas is 80% of the page with a 4px radius, focusable via tabindex 0 and outline removed (examples/wasm_hello/index.html:17-24; examples/wasm_hello/src/lib.rs:113-118)

## §Anti-Patterns

(no fact block)

## §Self-Validation Protocol

(no fact block)

## §Design Decisions Log

(no fact block)
