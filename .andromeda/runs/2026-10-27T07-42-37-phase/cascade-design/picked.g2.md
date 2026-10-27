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
### typography#1.6 · values · typography · ## Typography · lines 15–15
| blitz-dom default form controls | inputs/selects/buttons `system-ui, sans-serif`; textarea monospace | not stated | not stated | not stated | (packages/blitz-dom/assets/default.css:88-96; packages/blitz-dom/assets/default.css:311-357; packages/blitz-dom/assets/default.css:368-376; packages/blitz-dom/assets/default.css:672-677) |
### typography#1.9 · values · typography · ## Typography · lines 18–18
| seven_guis | sans-serif | home title 700 | base 14px; home title 36px | not stated | (examples/seven_guis/src/app.rs:179-205) |
### typography#1.10 · values · typography · ## Typography · lines 19–19
| seven_guis flight booker's `flight-type` select (`.type-select`) | not stated | 400 | 15px | not stated | (examples/seven_guis/src/tasks/flight_booker.rs:160-167) |
### typography#1.13 · values · typography · ## Typography · lines 22–22
| Text input editors | not stated | not stated | `parley::PlainEditor::new(16.0)` | not stated | (packages/blitz-dom/src/node/text.rs:86) |
### typography#1.14 · values · typography · ## Typography · lines 23–23
| Engine generic base size | generics | not stated | monospace 13px; other generics 16px | not stated | (packages/blitz-dom/src/font_metrics.rs:167-176) |
### typography#2.2 · values · typography · ## Typography · lines 31–31
- The seven_guis headless stand's font registration is held in architecture §Standard Contracts → Headless stand, beside `font_ctx()`, not as a design-system typography value
### typography#2.3 · values · typography · ## Typography · lines 32–32
- A custom `FontContext` can be set; on WASM a context with bundled fonts must be provided, using `build_single_font_ctx` for one font (packages/dioxus-native/src/config.rs:56-64); `build_single_font_ctx` registers one font as fallback for SansSerif, Serif, Monospace and SystemUi with system fonts disabled (packages/blitz-dom/src/lib.rs:134-165)
### typography#2.4 · values · typography · ## Typography · lines 33–33
- Font features: system-fonts, woff, complex-scripts (dictionary line-breaking), font-embolden and apple-font-embolden (packages/dioxus-native/Cargo.toml:20-25; packages/dioxus-native/Cargo.toml:47-48)
### typography#2.5 · values · typography · ## Typography · lines 34–34
- A bullet font is always registered in the default font context (packages/blitz-dom/src/document.rs:440-442; packages/blitz-dom/src/lib.rs:32)
### typography#2.6 · values · typography · ## Typography · lines 35–35
- Since 2026-10-12-headless-screenshot a driver `screenshot` of an app whose font context resolves no `sans-serif` family (`BaseDocument::can_paint_text` false) is refused `fonts-unavailable` with nothing painted — a shot never paints text skipped; the stand's bundled font passes the check (packages/blitz-dom/src/document.rs:1134-1147; as measured at escher-0.1.0/chunks/2026-10-12-headless-screenshot/report.md)
### typography#2.7 · values · typography · ## Typography · lines 36–36
- `document.fonts` is a stub FontFaceSet where all fonts report as loaded (packages/blitz-vibey-script/src/runtime.rs:1060-1074)
### typography#2.8 · values · typography · ## Typography · lines 37–37
- `blitz-dom` is built with the `system-fonts` feature for the blitz-tests crate; without it text measures 0x0 and font-dependent assertions pass vacuously, and the feature is stated to be enabled by default when testing the whole workspace (tests/blitz-tests/Cargo.toml:17; tests/blitz-tests/tests/br_trailing_line.rs:11-13; tests/blitz-tests/tests/inline_box_baseline.rs:4-6)
### typography#3.1 · values · typography · ## Typography · lines 43–43
- Generic font families map to Parley generics, with `None` as sans-serif (packages/blitz-dom/src/stylo_to_parley.rs:50-60)
### typography#3.2 · values · typography · ## Typography · lines 44–44
- Font weight, width, style and variation settings convert to Parley values (packages/blitz-dom/src/stylo_to_parley.rs:87-112)
### typography#3.4 · values · typography · ## Typography · lines 46–46
- Line height maps `normal`, number and length to Parley line heights (packages/blitz-dom/src/stylo_to_parley.rs:407-412); the layout engine resolves `line-height: normal` as 1.2 × font size (packages/blitz-dom/src/layout/mod.rs:232-236)
### typography#3.6 · values · typography · ## Typography · lines 48–48
- Text locale comes from the computed `_x_lang` value, which the `lang` attribute sets (packages/blitz-dom/src/stylo_to_parley.rs:495; packages/blitz-dom/src/stylo.rs:1201-1203)
### typography#3.7 · values · typography · ## Typography · lines 49–49
- `text-align` and `text-align-last` map to Parley alignment (packages/blitz-dom/src/stylo_to_parley.rs:304-329); word break, line break, overflow wrap, wrap mode and white-space collapse map to Parley settings (packages/blitz-dom/src/stylo_to_parley.rs:331-346; packages/blitz-dom/src/stylo_to_parley.rs:467-484)
### typography#3.9 · values · typography · ## Typography · lines 51–51
- The new stylist device is seeded with the root element's font size and line height so rem/rlh units do not fall back to the 16px default (packages/blitz-dom/src/document.rs:2322-2343); `rem` resolves against the root font-size after viewport and hidpi changes, with an initial root font-size of 16px (tests/blitz-tests/tests/rem_after_viewport_change.rs:1-8; tests/blitz-tests/tests/rem_after_viewport_change.rs:49-77)
### typography#3.10 · values · typography · ## Typography · lines 52–52
- List markers: decimal `"N. "`, lower/upper-alpha, disc `•`, circle `◦`, square `▪`, disclosure-open `▾`, disclosure-closed `▸`, other names `□` (packages/blitz-dom/src/layout/list.rs:116-158)
### spacing#2.1 · values · spacing · ## Spacing · lines 16–16
- Browser urlbar padding 6px, gap 6px; tab padding 0 8px; menu padding 8px with items 8px 12px gap 8px; suggestion rows 6px 12px (apps/browser/assets/browser.css:39; apps/browser/assets/browser.css:132-133; apps/browser/assets/browser.css:214; apps/browser/assets/browser.css:223; apps/browser/assets/browser.css:231; apps/browser/assets/browser.css:338)
### spacing#3.4 · values · spacing · ## Spacing · lines 30–30
- Single-line text inputs are vertically centered within their content box (packages/blitz-dom/src/node/node.rs:860-878)
