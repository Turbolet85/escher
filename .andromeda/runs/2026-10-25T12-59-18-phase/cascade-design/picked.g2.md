### typography#1.6 · values · typography · ## Typography · lines 15–15
| blitz-dom default form controls | inputs/selects/buttons `system-ui, sans-serif`; textarea monospace | not stated | not stated | not stated | (packages/blitz-dom/assets/default.css:88-96; packages/blitz-dom/assets/default.css:311-357; packages/blitz-dom/assets/default.css:368-376; packages/blitz-dom/assets/default.css:672-677) |
### typography#1.9 · values · typography · ## Typography · lines 18–18
| seven_guis | sans-serif | home title 700 | base 14px; home title 36px | not stated | (examples/seven_guis/src/app.rs:179-205) |
### typography#1.12 · values · typography · ## Typography · lines 21–21
| Text input editors | not stated | not stated | `parley::PlainEditor::new(16.0)` | not stated | (packages/blitz-dom/src/node/text.rs:86) |
### typography#1.13 · values · typography · ## Typography · lines 22–22
| Engine generic base size | generics | not stated | monospace 13px; other generics 16px | not stated | (packages/blitz-dom/src/font_metrics.rs:167-176) |
### typography#2.2 · values · typography · ## Typography · lines 30–30
- The seven_guis headless stand registers the same bundled DejaVu Sans for every generic, with system fonts off, on native — `build_single_font_ctx(DEJAVU_SANS)` through `HarnessOptions.font_ctx`, decoded by seven_guis' native `woff` feature (examples/seven_guis/src/stand.rs:52-54; examples/seven_guis/src/lib.rs:7; examples/seven_guis/Cargo.toml:31)
### typography#2.3 · values · typography · ## Typography · lines 31–31
- A custom `FontContext` can be set; on WASM a context with bundled fonts must be provided, using `build_single_font_ctx` for one font (packages/dioxus-native/src/config.rs:56-64); `build_single_font_ctx` registers one font as fallback for SansSerif, Serif, Monospace and SystemUi with system fonts disabled (packages/blitz-dom/src/lib.rs:134-165)
### typography#2.4 · values · typography · ## Typography · lines 32–32
- Font features: system-fonts, woff, complex-scripts (dictionary line-breaking), font-embolden and apple-font-embolden (packages/dioxus-native/Cargo.toml:20-25; packages/dioxus-native/Cargo.toml:47-48)
### typography#2.6 · values · typography · ## Typography · lines 34–34
- Since 2026-10-12-headless-screenshot a driver `screenshot` of an app whose font context resolves no `sans-serif` family (`BaseDocument::can_paint_text` false) is refused `fonts-unavailable` with nothing painted — a shot never paints text skipped; the stand's bundled font passes the check (packages/blitz-dom/src/document.rs:1134-1147; as measured at escher-0.1.0/chunks/2026-10-12-headless-screenshot/report.md)
### typography#2.8 · values · typography · ## Typography · lines 36–36
- `blitz-dom` is built with the `system-fonts` feature for the blitz-tests crate; without it text measures 0x0 and font-dependent assertions pass vacuously, and the feature is stated to be enabled by default when testing the whole workspace (tests/blitz-tests/Cargo.toml:17; tests/blitz-tests/tests/br_trailing_line.rs:11-13; tests/blitz-tests/tests/inline_box_baseline.rs:4-6)
### typography#3.1 · values · typography · ## Typography · lines 42–42
- Generic font families map to Parley generics, with `None` as sans-serif (packages/blitz-dom/src/stylo_to_parley.rs:50-60)
### typography#3.2 · values · typography · ## Typography · lines 43–43
- Font weight, width, style and variation settings convert to Parley values (packages/blitz-dom/src/stylo_to_parley.rs:87-112)
### typography#3.3 · values · typography · ## Typography · lines 44–44
- `font-variant-ligatures`, `-caps`, `-position`, `-numeric` and `-east-asian` map to OpenType feature tags (packages/blitz-dom/src/stylo_to_parley.rs:134-262)
### typography#3.4 · values · typography · ## Typography · lines 45–45
- Line height maps `normal`, number and length to Parley line heights (packages/blitz-dom/src/stylo_to_parley.rs:407-412); the layout engine resolves `line-height: normal` as 1.2 × font size (packages/blitz-dom/src/layout/mod.rs:232-236)
### typography#3.5 · values · typography · ## Typography · lines 46–46
- Letter and word spacing resolve against the font size (packages/blitz-dom/src/stylo_to_parley.rs:415-424)
### typography#3.6 · values · typography · ## Typography · lines 47–47
- Text locale comes from the computed `_x_lang` value, which the `lang` attribute sets (packages/blitz-dom/src/stylo_to_parley.rs:495; packages/blitz-dom/src/stylo.rs:1201-1203)
### typography#3.7 · values · typography · ## Typography · lines 48–48
- `text-align` and `text-align-last` map to Parley alignment (packages/blitz-dom/src/stylo_to_parley.rs:304-329); word break, line break, overflow wrap, wrap mode and white-space collapse map to Parley settings (packages/blitz-dom/src/stylo_to_parley.rs:331-346; packages/blitz-dom/src/stylo_to_parley.rs:467-484)
### typography#3.8 · values · typography · ## Typography · lines 49–49
- `ch` and `ic` metrics measure `'0'` and `'\u{6C34}'` advances scaled like Parley's shaped glyph advances (packages/blitz-dom/src/font_metrics.rs:104-161)
### typography#3.11 · values · typography · ## Typography · lines 52–52
- Font emboldening is enabled by the font-embolden feature, or apple-font-embolden on macOS and iOS (packages/blitz-paint/Cargo.toml:18-19; packages/blitz-paint/src/lib.rs:22-26); strength is 0.015125 and 0.0121 times the CSS font size, each capped at 0.3, with hinting off when emboldening (packages/blitz-paint/src/text.rs:640-651)
### spacing#2.4 · values · spacing · ## Spacing · lines 19–19
- seven_guis cards pad 24px 32px with 16px gap; home pads 48px 32px 64px (examples/seven_guis/src/tasks/counter.rs:38-42; examples/seven_guis/src/app.rs:191)
### spacing#2.5 · values · spacing · ## Spacing · lines 20–20
- blitz-dom default body margin 8px (packages/blitz-dom/assets/default.css:274-277)
### spacing#3.4 · values · spacing · ## Spacing · lines 29–29
- Single-line text inputs are vertically centered within their content box (packages/blitz-dom/src/node/node.rs:860-878)
