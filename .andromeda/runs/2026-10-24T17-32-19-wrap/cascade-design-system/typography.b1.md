### typography#1.1 · values · typography · ## Typography · lines 8–10
| Role | Font | Weight | Size | Tracking | Usage |
|------|------|--------|------|----------|-------|
| Browser chrome text | sans-serif | not stated | tab title 12px, urlbar 14px (16px on mobile), menu items 14px, status bar 12px | not stated | (apps/browser/assets/browser.css:93-94; apps/browser/assets/browser.css:157-160; apps/browser/assets/browser.css:170; apps/browser/assets/browser.css:225-226; apps/browser/assets/browser.css:297-298) |
### typography#1.2 · values · typography · ## Typography · lines 11–11
| Browser suggestion section header | not stated | 600 | 11px | letter-spacing 0.04em, uppercase | (apps/browser/assets/browser.css:329-336) |
### typography#1.3 · values · typography · ## Typography · lines 12–12
| Browser FPS overlay | monospace | not stated | 12px | not stated | (apps/browser/assets/browser.css:269-270) |
### typography#1.4 · values · typography · ## Typography · lines 13–13
| about:history | `system-ui, sans-serif` | heading 600 | heading 24px | not stated | (apps/browser/assets/about-history.css:7; apps/browser/assets/about-history.css:9-11) |
### typography#1.5 · values · typography · ## Typography · lines 14–14
| rdme body | stack beginning `-apple-system,BlinkMacSystemFont,"Segoe UI"`; monospace stack in `--fontStack-monospace` | weights in `--base-text-weight-*` | 16px / line-height 1.5 | not stated | (apps/readme/assets/github-markdown.css:7-10; apps/readme/assets/github-markdown.css:132-134) |
### typography#1.6 · values · typography · ## Typography · lines 15–15
| blitz-dom default form controls | inputs/selects/buttons `system-ui, sans-serif`; textarea monospace | not stated | not stated | not stated | (packages/blitz-dom/assets/default.css:88-96; packages/blitz-dom/assets/default.css:311-357; packages/blitz-dom/assets/default.css:368-376; packages/blitz-dom/assets/default.css:672-677) |
### typography#1.7 · values · typography · ## Typography · lines 16–16
| blitz-dom default headings | not stated | bold | h1–h6 from 2em down to 0.67em | not stated | (packages/blitz-dom/assets/default.css:88-96; packages/blitz-dom/assets/default.css:311-357; packages/blitz-dom/assets/default.css:368-376; packages/blitz-dom/assets/default.css:672-677) |
### typography#1.8 · values · typography · ## Typography · lines 17–17
| blitz-dom default code/pre | `-moz-fixed` | not stated | not stated | not stated | (packages/blitz-dom/assets/default.css:88-96; packages/blitz-dom/assets/default.css:311-357; packages/blitz-dom/assets/default.css:368-376; packages/blitz-dom/assets/default.css:672-677) |
### typography#1.9 · values · typography · ## Typography · lines 18–18
| seven_guis | sans-serif | home title 700 | base 14px; home title 36px | not stated | (examples/seven_guis/src/app.rs:179-205) |
### typography#1.10 · values · typography · ## Typography · lines 19–19
| todomvc | `'Helvetica Neue', Helvetica, Arial, sans-serif` | body 300; h1 100 | body 14px; h1 100px | not stated | (examples/todomvc/src/todomvc.css:24; examples/todomvc/src/todomvc.css:33; examples/todomvc/src/todomvc.css:73-74) |
### typography#1.11 · values · typography · ## Typography · lines 20–20
| wgpu_texture | `system-ui, sans` | not stated | not stated | not stated | (examples/wgpu_texture/src/styles.css:7) |
### typography#1.12 · values · typography · ## Typography · lines 21–21
| Text input editors | not stated | not stated | `parley::PlainEditor::new(16.0)` | not stated | (packages/blitz-dom/src/node/text.rs:86) |
### typography#1.13 · values · typography · ## Typography · lines 22–22
| Engine generic base size | generics | not stated | monospace 13px; other generics 16px | not stated | (packages/blitz-dom/src/font_metrics.rs:167-176) |
### typography#1.14 · values · typography · ## Typography · lines 23–23
| List bullets | `"Bullet, monospace, sans-serif"` | not stated | not stated | not stated | (packages/blitz-dom/src/layout/list.rs:15; packages/blitz-dom/src/layout/list.rs:160-167; packages/blitz-dom/src/layout/construct.rs:1106-1112) |
### typography#2.1 · values · typography · ## Typography · lines 29–29
- WASM builds register bundled DejaVu Sans for sans-serif, serif, monospace and system-ui (examples/wasm_hello/src/lib.rs:75-100)
### typography#2.2 · values · typography · ## Typography · lines 30–30
- The seven_guis headless stand registers the same bundled DejaVu Sans for every generic, with system fonts off, on native — `build_single_font_ctx(DEJAVU_SANS)` through `HarnessOptions.font_ctx`, decoded by seven_guis' native `woff` feature (examples/seven_guis/src/stand.rs:52-54; examples/seven_guis/src/lib.rs:7; examples/seven_guis/Cargo.toml:31)
### typography#2.3 · values · typography · ## Typography · lines 31–31
- A custom `FontContext` can be set; on WASM a context with bundled fonts must be provided, using `build_single_font_ctx` for one font (packages/dioxus-native/src/config.rs:56-64); `build_single_font_ctx` registers one font as fallback for SansSerif, Serif, Monospace and SystemUi with system fonts disabled (packages/blitz-dom/src/lib.rs:134-165)
### typography#2.4 · values · typography · ## Typography · lines 32–32
- Font features: system-fonts, woff, complex-scripts (dictionary line-breaking), font-embolden and apple-font-embolden (packages/dioxus-native/Cargo.toml:20-25; packages/dioxus-native/Cargo.toml:47-48)
### typography#2.5 · values · typography · ## Typography · lines 33–33
- A bullet font is always registered in the default font context (packages/blitz-dom/src/document.rs:440-442; packages/blitz-dom/src/lib.rs:32)
### typography#2.6 · values · typography · ## Typography · lines 34–34
- Since 2026-10-12-headless-screenshot a driver `screenshot` of an app whose font context resolves no `sans-serif` family (`BaseDocument::can_paint_text` false) is refused `fonts-unavailable` with nothing painted — a shot never paints text skipped; the stand's bundled font passes the check (packages/blitz-dom/src/document.rs:1134-1147; as measured at escher-0.1.0/chunks/2026-10-12-headless-screenshot/report.md)
### typography#2.7 · values · typography · ## Typography · lines 35–35
- `document.fonts` is a stub FontFaceSet where all fonts report as loaded (packages/blitz-vibey-script/src/runtime.rs:1060-1074)
### typography#2.8 · values · typography · ## Typography · lines 36–36
- `blitz-dom` is built with the `system-fonts` feature for the blitz-tests crate; without it text measures 0x0 and font-dependent assertions pass vacuously, and the feature is stated to be enabled by default when testing the whole workspace (tests/blitz-tests/Cargo.toml:17; tests/blitz-tests/tests/br_trailing_line.rs:11-13; tests/blitz-tests/tests/inline_box_baseline.rs:4-6)
### typography#3.1 · values · typography · ## Typography · lines 42–42
- Generic font families map to Parley generics, with `None` as sans-serif (packages/blitz-dom/src/stylo_to_parley.rs:50-60)
### typography#3.2 · values · typography · ## Typography · lines 43–43
- Font weight, width, style and variation settings convert to Parley values (packages/blitz-dom/src/stylo_to_parley.rs:87-112)
### typography#3.3 · values · typography · ## Typography · lines 44–44
- `font-variant-ligatures`, `-caps`, `-position`, `-numeric` and `-east-asian` map to OpenType feature tags (packages/blitz-dom/src/stylo_to_parley.rs:134-262)
