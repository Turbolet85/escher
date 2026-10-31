### typography#1.6 · values · typography · ## Typography · lines 15–15
| blitz-dom default form controls | inputs/selects/buttons `system-ui, sans-serif`; textarea monospace | not stated | not stated | not stated | (packages/blitz-dom/assets/default.css:88-96; packages/blitz-dom/assets/default.css:339-385; packages/blitz-dom/assets/default.css:396-404; packages/blitz-dom/assets/default.css:700-705) |
### typography#1.12 · values · typography · ## Typography · lines 21–21
| Engine generic base size | generics | not stated | monospace 13px; other generics 16px | not stated | (packages/blitz-dom/src/font_metrics.rs:167-176) |
### typography#2.1 · values · typography · ## Typography · lines 28–28
- WASM builds register bundled DejaVu Sans for sans-serif, serif, monospace and system-ui (examples/wasm_hello/src/lib.rs:75-100)
### typography#2.2 · values · typography · ## Typography · lines 29–29
- The seven_guis headless stand's font registration is held in architecture §Standard Contracts → Headless stand, beside `font_ctx()`, not as a design-system typography value
### typography#2.3 · values · typography · ## Typography · lines 30–30
- A custom `FontContext` can be set; on WASM a context with bundled fonts must be provided, using `build_single_font_ctx` for one font (packages/dioxus-native/src/config.rs:56-64); `build_single_font_ctx` registers one font as fallback for SansSerif, Serif, Monospace and SystemUi with system fonts disabled (packages/blitz-dom/src/lib.rs:134-165)
### typography#2.4 · values · typography · ## Typography · lines 31–31
- Font features: system-fonts, woff, complex-scripts (dictionary line-breaking), font-embolden and apple-font-embolden (packages/dioxus-native/Cargo.toml:20-25; packages/dioxus-native/Cargo.toml:47-48)
### typography#2.7 · values · typography · ## Typography · lines 34–34
- `document.fonts` is a stub FontFaceSet where all fonts report as loaded (packages/blitz-vibey-script/src/runtime/bootstrap/fonts.js:1-15)
### typography#3.1 · values · typography · ## Typography · lines 41–41
- Generic font families map to Parley generics, with `None` as sans-serif (packages/blitz-dom/src/stylo_to_parley.rs:50-60)
### typography#3.2 · values · typography · ## Typography · lines 42–42
- Font weight, width, style and variation settings convert to Parley values (packages/blitz-dom/src/stylo_to_parley.rs:87-112)
### typography#3.4 · values · typography · ## Typography · lines 44–44
- Line height maps `normal`, number and length to Parley line heights (packages/blitz-dom/src/stylo_to_parley.rs:407-412); the layout engine resolves `line-height: normal` as 1.2 × font size (packages/blitz-dom/src/layout/mod.rs:232-236)
### typography#3.9 · values · typography · ## Typography · lines 49–49
- The new stylist device is seeded with the root element's font size and line height so rem/rlh units do not fall back to the 16px default (packages/blitz-dom/src/document.rs:2352-2373); `rem` resolves against the root font-size after viewport and hidpi changes, with an initial root font-size of 16px (tests/blitz-tests/tests/rem_after_viewport_change.rs:1-8; tests/blitz-tests/tests/rem_after_viewport_change.rs:49-77)
### typography#3.11 · values · typography · ## Typography · lines 51–51
- Font emboldening is enabled by the font-embolden feature, or apple-font-embolden on macOS and iOS (packages/blitz-paint/Cargo.toml:18-19; packages/blitz-paint/src/lib.rs:22-26); strength is 0.015125 and 0.0121 times the CSS font size, each capped at 0.3, with hinting off when emboldening (packages/blitz-paint/src/text.rs:640-651)
### typography#5.1 · absence · typography · ## Typography · lines 69–69
- font-family declarations · searched: `font-family|font_family` over the 16 listed s08 files (no match)
### typography#5.2 · absence · typography · ## Typography · lines 70–70
- font family declarations · searched: `font_family|font-family|FontFamily` over the 32 listed s09 files
### typography#5.3 · absence · typography · ## Typography · lines 71–71
- font-family declarations outside test fixtures · searched: `font-family|font_family` over the 32 s10 slice files (matches only packages/blitz-vibey-script/tests/dom.rs)
### typography#5.4 · absence · typography · ## Typography · lines 72–72
- a project type scale or font tokens · searched: `font-family` over the 61 s12 slice files (one fixture hit, `sans-serif`)
### spacing#2.4 · values · spacing · ## Spacing · lines 19–19
- The seven_guis stand's card and Home padding is held in layout-templates §Surface: desktop-native → Primary screens (`values · desktop-native-screens`), not as a design-system value
### spacing#2.6 · values · spacing · ## Spacing · lines 21–21
- blitz-dom default `multiple` select rows (`select[multiple] > option`, `select[multiple] > optgroup > option`) padding 0 2px (packages/blitz-dom/assets/default.css:173-177)
### spacing#3.3 · values · spacing · ## Spacing · lines 29–29
- Overlay scrollbar thumb geometry: thickness 10.0, thin thickness 6.0, margin 2.0 and minimum length 32.0 CSS px (packages/blitz-dom/src/node/scrollbar.rs:122-126)
