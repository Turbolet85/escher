### typography#3.3 · values · typography · ## Typography · lines 44–44
- `font-variant-ligatures`, `-caps`, `-position`, `-numeric` and `-east-asian` map to OpenType feature tags (packages/blitz-dom/src/stylo_to_parley.rs:134-262)
### typography#3.4 · values · typography · ## Typography · lines 45–45
- Line height maps `normal`, number and length to Parley line heights (packages/blitz-dom/src/stylo_to_parley.rs:407-412); the layout engine resolves `line-height: normal` as 1.2 × font size (packages/blitz-dom/src/layout/mod.rs:232-236)
### typography#3.5 · values · typography · ## Typography · lines 46–46
- An open select list's row height is the select's font on one line, a row's text laid out through `select_row_layout`; a larger authored font clips, a standing limit (packages/blitz-dom/src/document.rs:1054-1064; as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)
### typography#3.6 · values · typography · ## Typography · lines 47–47
- Letter and word spacing resolve against the font size (packages/blitz-dom/src/stylo_to_parley.rs:415-424)
### typography#3.7 · values · typography · ## Typography · lines 48–48
- Text locale comes from the computed `_x_lang` value, which the `lang` attribute sets (packages/blitz-dom/src/stylo_to_parley.rs:495; packages/blitz-dom/src/stylo.rs:1217-1219)
### typography#3.8 · values · typography · ## Typography · lines 49–49
- `text-align` and `text-align-last` map to Parley alignment (packages/blitz-dom/src/stylo_to_parley.rs:304-329); word break, line break, overflow wrap, wrap mode and white-space collapse map to Parley settings (packages/blitz-dom/src/stylo_to_parley.rs:331-346; packages/blitz-dom/src/stylo_to_parley.rs:467-484)
### typography#3.9 · values · typography · ## Typography · lines 50–50
- `ch` and `ic` metrics measure `'0'` and `'\u{6C34}'` advances scaled like Parley's shaped glyph advances (packages/blitz-dom/src/font_metrics.rs:104-161)
### typography#3.10 · values · typography · ## Typography · lines 51–51
- The new stylist device is seeded with the root element's font size and line height so rem/rlh units do not fall back to the 16px default (packages/blitz-dom/src/document.rs:2691-2712); `rem` resolves against the root font-size after viewport and hidpi changes, with an initial root font-size of 16px (tests/blitz-tests/tests/rem_after_viewport_change.rs:1-8; tests/blitz-tests/tests/rem_after_viewport_change.rs:49-77)
### typography#3.12 · values · typography · ## Typography · lines 53–53
- Font emboldening is enabled by the font-embolden feature, or apple-font-embolden on macOS and iOS (packages/blitz-paint/Cargo.toml:18-19; packages/blitz-paint/src/lib.rs:22-26); strength is 0.015125 and 0.0121 times the CSS font size, each capped at 0.3, with hinting off on a build that emboldens; a synthesized group header on a plain build is emboldened with hinting on, and on an emboldening build it takes twice the stated strength (packages/blitz-paint/src/text.rs:678-695)
### typography#3.13 · values · typography · ## Typography · lines 54–54
- A group's `label` in an open select list paints as a header heavier than a row on every build, through `RunPaint { color, synthesize_bold }` in `stroke_text_as`; a synthesized bold adds one embolden weight to a build's own, and every other caller passes `RunPaint::default()`, adding none (packages/blitz-paint/src/text.rs:586-593; packages/blitz-paint/src/text.rs:683-689; tests/blitz-tests/tests/select_popup_painted.rs:282-297)
### typography#3.14 · values · typography · ## Typography · lines 55–55
- Auto text-decoration thickness is font-size / 10 with a 1px minimum, floored to whole device pixels (packages/blitz-paint/src/text.rs:254-277); overline and line-through positions use the font's OS/2 usWinAscent, cached per font face (packages/blitz-paint/src/text.rs:70-95; packages/blitz-paint/src/text.rs:494-544)
### typography#5.1 · absence · typography · ## Typography · lines 72–72
- font-family declarations · searched: `font-family|font_family` over the 16 listed s08 files (no match)
### typography#5.2 · absence · typography · ## Typography · lines 73–73
- font family declarations · searched: `font_family|font-family|FontFamily` over the 32 listed s09 files
### typography#5.3 · absence · typography · ## Typography · lines 74–74
- font-family declarations outside test fixtures · searched: `font-family|font_family` over the 32 s10 slice files (matches only packages/blitz-vibey-script/tests/dom.rs)
