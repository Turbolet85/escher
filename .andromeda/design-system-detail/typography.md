# design-system · Typography
Read whole. The master's broad view is .andromeda/design-system.md; this file holds its Typography detail.

## Typography

<!-- detail: values · typography -->

| Role | Font | Weight | Size | Tracking | Usage |
|------|------|--------|------|----------|-------|
| Browser chrome text | sans-serif | not stated | tab title 12px, urlbar 14px (16px on mobile), menu items 14px, status bar 12px | not stated | (apps/browser/assets/browser.css:93-94; apps/browser/assets/browser.css:157-160; apps/browser/assets/browser.css:170; apps/browser/assets/browser.css:225-226; apps/browser/assets/browser.css:297-298) |
| Browser suggestion section header | not stated | 600 | 11px | letter-spacing 0.04em, uppercase | (apps/browser/assets/browser.css:329-336) |
| Browser FPS overlay | monospace | not stated | 12px | not stated | (apps/browser/assets/browser.css:269-270) |
| about:history | `system-ui, sans-serif` | heading 600 | heading 24px | not stated | (apps/browser/assets/about-history.css:7; apps/browser/assets/about-history.css:9-11) |
| rdme body | stack beginning `-apple-system,BlinkMacSystemFont,"Segoe UI"`; monospace stack in `--fontStack-monospace` | weights in `--base-text-weight-*` | 16px / line-height 1.5 | not stated | (apps/readme/assets/github-markdown.css:7-10; apps/readme/assets/github-markdown.css:132-134) |
| blitz-dom default form controls | inputs/selects/buttons `system-ui, sans-serif`; textarea monospace | not stated | not stated | not stated | (packages/blitz-dom/assets/default.css:88-96; packages/blitz-dom/assets/default.css:311-357; packages/blitz-dom/assets/default.css:368-376; packages/blitz-dom/assets/default.css:672-677) |
| blitz-dom default headings | not stated | bold | h1–h6 from 2em down to 0.67em | not stated | (packages/blitz-dom/assets/default.css:88-96; packages/blitz-dom/assets/default.css:311-357; packages/blitz-dom/assets/default.css:368-376; packages/blitz-dom/assets/default.css:672-677) |
| blitz-dom default code/pre | `-moz-fixed` | not stated | not stated | not stated | (packages/blitz-dom/assets/default.css:88-96; packages/blitz-dom/assets/default.css:311-357; packages/blitz-dom/assets/default.css:368-376; packages/blitz-dom/assets/default.css:672-677) |
| seven_guis | sans-serif | home title 700 | base 14px; home title 36px | not stated | (examples/seven_guis/src/app.rs:179-205) |
| todomvc | `'Helvetica Neue', Helvetica, Arial, sans-serif` | body 300; h1 100 | body 14px; h1 100px | not stated | (examples/todomvc/src/todomvc.css:24; examples/todomvc/src/todomvc.css:33; examples/todomvc/src/todomvc.css:73-74) |
| wgpu_texture | `system-ui, sans` | not stated | not stated | not stated | (examples/wgpu_texture/src/styles.css:7) |
| Text input editors | not stated | not stated | `parley::PlainEditor::new(16.0)` | not stated | (packages/blitz-dom/src/node/text.rs:86) |
| Engine generic base size | generics | not stated | monospace 13px; other generics 16px | not stated | (packages/blitz-dom/src/font_metrics.rs:167-176) |
| List bullets | `"Bullet, monospace, sans-serif"` | not stated | not stated | not stated | (packages/blitz-dom/src/layout/list.rs:15; packages/blitz-dom/src/layout/list.rs:160-167; packages/blitz-dom/src/layout/list.rs:96-102) |

<!-- /detail -->

<!-- detail: values · typography -->

- WASM builds register bundled DejaVu Sans for sans-serif, serif, monospace and system-ui (examples/wasm_hello/src/lib.rs:75-100)
- The seven_guis headless stand registers the same bundled DejaVu Sans for every generic, with system fonts off, on native — `build_single_font_ctx(DEJAVU_SANS)` through `HarnessOptions.font_ctx`, decoded by seven_guis' native `woff` feature (examples/seven_guis/src/stand.rs:52-54; examples/seven_guis/src/lib.rs:7; examples/seven_guis/Cargo.toml:31)
- A custom `FontContext` can be set; on WASM a context with bundled fonts must be provided, using `build_single_font_ctx` for one font (packages/dioxus-native/src/config.rs:56-64); `build_single_font_ctx` registers one font as fallback for SansSerif, Serif, Monospace and SystemUi with system fonts disabled (packages/blitz-dom/src/lib.rs:134-165)
- Font features: system-fonts, woff, complex-scripts (dictionary line-breaking), font-embolden and apple-font-embolden (packages/dioxus-native/Cargo.toml:20-25; packages/dioxus-native/Cargo.toml:47-48)
- A bullet font is always registered in the default font context (packages/blitz-dom/src/document.rs:440-442; packages/blitz-dom/src/lib.rs:32)
- Since 2026-10-12-headless-screenshot a driver `screenshot` of an app whose font context resolves no `sans-serif` family (`BaseDocument::can_paint_text` false) is refused `fonts-unavailable` with nothing painted — a shot never paints text skipped; the stand's bundled font passes the check (packages/blitz-dom/src/document.rs:1134-1147; as measured at escher-0.1.0/chunks/2026-10-12-headless-screenshot/report.md)
- `document.fonts` is a stub FontFaceSet where all fonts report as loaded (packages/blitz-vibey-script/src/runtime.rs:1060-1074)
- `blitz-dom` is built with the `system-fonts` feature for the blitz-tests crate; without it text measures 0x0 and font-dependent assertions pass vacuously, and the feature is stated to be enabled by default when testing the whole workspace (tests/blitz-tests/Cargo.toml:17; tests/blitz-tests/tests/br_trailing_line.rs:11-13; tests/blitz-tests/tests/inline_box_baseline.rs:4-6)

<!-- /detail -->

<!-- detail: values · typography -->

- Generic font families map to Parley generics, with `None` as sans-serif (packages/blitz-dom/src/stylo_to_parley.rs:50-60)
- Font weight, width, style and variation settings convert to Parley values (packages/blitz-dom/src/stylo_to_parley.rs:87-112)
- `font-variant-ligatures`, `-caps`, `-position`, `-numeric` and `-east-asian` map to OpenType feature tags (packages/blitz-dom/src/stylo_to_parley.rs:134-262)
- Line height maps `normal`, number and length to Parley line heights (packages/blitz-dom/src/stylo_to_parley.rs:407-412); the layout engine resolves `line-height: normal` as 1.2 × font size (packages/blitz-dom/src/layout/mod.rs:232-236)
- Letter and word spacing resolve against the font size (packages/blitz-dom/src/stylo_to_parley.rs:415-424)
- Text locale comes from the computed `_x_lang` value, which the `lang` attribute sets (packages/blitz-dom/src/stylo_to_parley.rs:495; packages/blitz-dom/src/stylo.rs:1201-1203)
- `text-align` and `text-align-last` map to Parley alignment (packages/blitz-dom/src/stylo_to_parley.rs:304-329); word break, line break, overflow wrap, wrap mode and white-space collapse map to Parley settings (packages/blitz-dom/src/stylo_to_parley.rs:331-346; packages/blitz-dom/src/stylo_to_parley.rs:467-484)
- `ch` and `ic` metrics measure `'0'` and `'\u{6C34}'` advances scaled like Parley's shaped glyph advances (packages/blitz-dom/src/font_metrics.rs:104-161)
- The new stylist device is seeded with the root element's font size and line height so rem/rlh units do not fall back to the 16px default (packages/blitz-dom/src/document.rs:2322-2343); `rem` resolves against the root font-size after viewport and hidpi changes, with an initial root font-size of 16px (tests/blitz-tests/tests/rem_after_viewport_change.rs:1-8; tests/blitz-tests/tests/rem_after_viewport_change.rs:49-77)
- List markers: decimal `"N. "`, lower/upper-alpha, disc `•`, circle `◦`, square `▪`, disclosure-open `▾`, disclosure-closed `▸`, other names `□` (packages/blitz-dom/src/layout/list.rs:116-158)
- Font emboldening is enabled by the font-embolden feature, or apple-font-embolden on macOS and iOS (packages/blitz-paint/Cargo.toml:18-19; packages/blitz-paint/src/lib.rs:22-26); strength is 0.015125 and 0.0121 times the CSS font size, each capped at 0.3, with hinting off when emboldening (packages/blitz-paint/src/text.rs:640-651)
- Auto text-decoration thickness is font-size / 10 with a 1px minimum, floored to whole device pixels (packages/blitz-paint/src/text.rs:254-277); overline and line-through positions use the font's OS/2 usWinAscent, cached per font face (packages/blitz-paint/src/text.rs:70-95; packages/blitz-paint/src/text.rs:494-544)

<!-- /detail -->

<!-- detail: values · typography -->

- `font-family: sans-serif` for body and `monospace` for code labels (examples/assets/cursor.html:6; examples/assets/cursor.html:46; examples/assets/border-styles.html:6; examples/assets/border-styles.html:38); the Google fixtures set 14px `arial,sans-serif` (examples/assets/bottom_only.html:8; examples/assets/google.html:22-26), the bar 13px/27px Roboto (examples/assets/google.html:45-48) and Google Sans,Roboto,Helvetica,Arial,sans-serif stacks (examples/assets/google.html:563; examples/assets/google.html:1050)
- graphite: html/body Inter Variable 18px weight 500 line-height 1.5, 16px at width<=780px; h1 Bona Nova, Palatino, serif 2.66667rem weight 700, h2 1.75rem, h3 1.25rem, h4-h6 1rem Inter Variable weight 800; `--font-size-link` calc(1rem*4/3); Inter Variable and Bona Nova @font-face rules; `--nav-font-size` from 28px to 8px across breakpoints (examples/assets/graphite.html:918-930; examples/assets/graphite.html:940-943; examples/assets/graphite.html:1322-1330; examples/assets/graphite.html:1341-1366; examples/assets/graphite.html:915; examples/assets/graphite.html:1635-1758; examples/assets/graphite.html:1761-1822; examples/assets/graphite.html:986-995; examples/assets/graphite.html:1149-1156)
- gosub `"Arial", sans-serif` with h1 at 6em overridden to 72px (examples/assets/gosub.html:10; examples/assets/gosub.html:20-21; examples/assets/gosub.html:74-75); servo-new-reduced-1 `'Space Grotesk', sans-serif` (examples/assets/servo-new-reduced-1.html:13); pseudo `.qqq` Font Awesome 6 Brands glyphs at 32px (examples/assets/pseudo.html:21-25; examples/assets/pseudo.html:80)
- servo.css headings "Fira Sans", code/pre "Fira Mono", body stack "Fira Sans" … sans-serif weight 400 line-height 1.5, loaded from Google Fonts by servo.html (examples/assets/servo.css:1-23; examples/assets/servo.css:229-234; examples/assets/servo.css:220-227; examples/assets/servo.css:236-241; examples/assets/servo.html:27)
- custom-widget `system-ui, sans` (examples/custom_widget.rs:179); Preact TodoMVC `16px/1.4 system-ui, -apple-system, "Segoe UI", Roboto, sans-serif` with a 64px weight-200 heading (examples/preact/index.html:12; examples/preact/index.html:17); reference page `15px/1.55` system stack and ui-monospace code (examples/preact/core_dom_apis.html:10; examples/preact/core_dom_apis.html:23)
- text-decoration fixture exercises text-decoration-line/thickness/style, text-underline-offset/position and text-decoration-inset (examples/assets/text-decoration.html:48-102)

<!-- /detail -->

<!-- detail: absence · typography -->

- font-family declarations · searched: `font-family|font_family` over the 16 listed s08 files (no match)
- font family declarations · searched: `font_family|font-family|FontFamily` over the 32 listed s09 files
- font-family declarations outside test fixtures · searched: `font-family|font_family` over the 32 s10 slice files (matches only packages/blitz-vibey-script/tests/dom.rs)
- a project type scale or font tokens · searched: `font-family` over the 61 s12 slice files (one fixture hit, `sans-serif`)

<!-- /detail -->
