### typography#1.6 · values · typography · ## Typography · lines 15–15
| blitz-dom default form controls | inputs/selects/buttons `system-ui, sans-serif`; textarea monospace | not stated | not stated | not stated | (packages/blitz-dom/assets/default.css:88-96; packages/blitz-dom/assets/default.css:350-396; packages/blitz-dom/assets/default.css:407-415; packages/blitz-dom/assets/default.css:711-716) |
### typography#1.7 · values · typography · ## Typography · lines 16–16
| blitz-dom default headings | not stated | bold | h1–h6 from 2em down to 0.67em | not stated | (packages/blitz-dom/assets/default.css:88-96; packages/blitz-dom/assets/default.css:350-396; packages/blitz-dom/assets/default.css:407-415; packages/blitz-dom/assets/default.css:711-716) |
### typography#1.8 · values · typography · ## Typography · lines 17–17
| blitz-dom default code/pre | `-moz-fixed` | not stated | not stated | not stated | (packages/blitz-dom/assets/default.css:88-96; packages/blitz-dom/assets/default.css:350-396; packages/blitz-dom/assets/default.css:407-415; packages/blitz-dom/assets/default.css:711-716) |
### typography#1.9 · values · typography · ## Typography · lines 18–18
| blitz-dom default `select[multiple][size]` rows | not stated | not stated | line-height 1.2 | not stated | (packages/blitz-dom/assets/default.css:183-193) |
### typography#1.12 · values · typography · ## Typography · lines 21–21
| Text input editors | not stated | not stated | `parley::PlainEditor::new(16.0)` | not stated | (packages/blitz-dom/src/node/text.rs:86) |
### typography#1.13 · values · typography · ## Typography · lines 22–22
| Engine generic base size | generics | not stated | monospace 13px; other generics 16px | not stated | (packages/blitz-dom/src/font_metrics.rs:167-176) |
### typography#1.14 · values · typography · ## Typography · lines 23–23
| List bullets | `"Bullet, monospace, sans-serif"` | not stated | not stated | not stated | (packages/blitz-dom/src/layout/list.rs:15; packages/blitz-dom/src/layout/list.rs:160-167; packages/blitz-dom/src/layout/list.rs:96-102) |
### typography#2.2 · values · typography · ## Typography · lines 30–30
- The seven_guis headless stand's font registration is held in architecture §Standard Contracts → Headless stand, beside `font_ctx()`, not as a design-system typography value
### typography#2.4 · values · typography · ## Typography · lines 32–32
- Font features: system-fonts, woff, complex-scripts (dictionary line-breaking), font-embolden and apple-font-embolden (packages/dioxus-native/Cargo.toml:20-25; packages/dioxus-native/Cargo.toml:47-48)
### typography#2.6 · values · typography · ## Typography · lines 34–34
- Since 2026-10-12-headless-screenshot a driver `screenshot` of an app whose font context resolves no `sans-serif` family (`BaseDocument::can_paint_text` false) is refused `fonts-unavailable` with nothing painted — a shot never paints text skipped; the stand's bundled font passes the check (packages/blitz-dom/src/document.rs:1500-1513; as measured at escher-0.1.0/chunks/2026-10-12-headless-screenshot/report.md)
### typography#2.7 · values · typography · ## Typography · lines 35–35
- `document.fonts` is a stub FontFaceSet where all fonts report as loaded (packages/blitz-vibey-script/src/runtime/bootstrap/fonts.js:1-15)
### typography#2.8 · values · typography · ## Typography · lines 36–36
- `blitz-dom` is built with the `system-fonts` feature for the blitz-tests crate; without it text measures 0x0 and font-dependent assertions pass vacuously, and the feature is stated to be enabled by default when testing the whole workspace (tests/blitz-tests/Cargo.toml:17; tests/blitz-tests/tests/br_trailing_line.rs:11-13; tests/blitz-tests/tests/inline_box_baseline.rs:4-6)
### typography#3.1 · values · typography · ## Typography · lines 42–42
- Generic font families map to Parley generics, with `None` as sans-serif (packages/blitz-dom/src/stylo_to_parley.rs:50-60)
### typography#3.2 · values · typography · ## Typography · lines 43–43
- Font weight, width, style and variation settings convert to Parley values (packages/blitz-dom/src/stylo_to_parley.rs:87-112)
