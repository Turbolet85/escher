### typography#1.6 · values · typography · ## Typography · lines 15–15 · stale
| blitz-dom default form controls | inputs/selects/buttons `system-ui, sans-serif`; textarea monospace | not stated | not stated | not stated | (packages/blitz-dom/assets/default.css:88-96; packages/blitz-dom/assets/default.css:346-392; packages/blitz-dom/assets/default.css:403-411; packages/blitz-dom/assets/default.css:707-712) |
### typography#1.7 · values · typography · ## Typography · lines 16–16 · stale
| blitz-dom default headings | not stated | bold | h1–h6 from 2em down to 0.67em | not stated | (packages/blitz-dom/assets/default.css:88-96; packages/blitz-dom/assets/default.css:346-392; packages/blitz-dom/assets/default.css:403-411; packages/blitz-dom/assets/default.css:707-712) |
### typography#1.8 · values · typography · ## Typography · lines 17–17 · stale
| blitz-dom default code/pre | `-moz-fixed` | not stated | not stated | not stated | (packages/blitz-dom/assets/default.css:88-96; packages/blitz-dom/assets/default.css:346-392; packages/blitz-dom/assets/default.css:403-411; packages/blitz-dom/assets/default.css:707-712) |
### typography#1.9 · values · typography · ## Typography · lines 18–18 · stale
| blitz-dom default `select[multiple][size]` rows | not stated | not stated | line-height 1.2 | not stated | (packages/blitz-dom/assets/default.css:183-193) |
### typography#2.3 · values · typography · ## Typography · lines 31–31 · stale
- A custom `FontContext` can be set; on WASM a context with bundled fonts must be provided, using `build_single_font_ctx` for one font (packages/dioxus-native/src/config.rs:56-64); `build_single_font_ctx` registers one font as fallback for SansSerif, Serif, Monospace and SystemUi with system fonts disabled (packages/blitz-dom/src/lib.rs:135-166)
### typography#2.5 · values · typography · ## Typography · lines 33–33 · stale
- A bullet font is always registered in the default font context (packages/blitz-dom/src/document.rs:469-471; packages/blitz-dom/src/lib.rs:32)
### typography#2.6 · values · typography · ## Typography · lines 34–34 · stale
- Since 2026-10-12-headless-screenshot a driver `screenshot` of an app whose font context resolves no `sans-serif` family (`BaseDocument::can_paint_text` false) is refused `fonts-unavailable` with nothing painted — a shot never paints text skipped; the stand's bundled font passes the check (packages/blitz-dom/src/document.rs:1522-1535; as measured at escher-0.1.0/chunks/2026-10-12-headless-screenshot/report.md)
### typography#3.5 · values · typography · ## Typography · lines 46–46 · stale
- An open select list's row height is the select's font on one line, a row's text laid out through `select_row_layout`; a larger authored font clips, a standing limit (packages/blitz-dom/src/document.rs:1065-1075; as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)
### typography#3.7 · values · typography · ## Typography · lines 48–48 · stale
- Text locale comes from the computed `_x_lang` value, which the `lang` attribute sets (packages/blitz-dom/src/stylo_to_parley.rs:495; packages/blitz-dom/src/stylo.rs:1225-1227)
### typography#3.10 · values · typography · ## Typography · lines 51–51 · stale
- The new stylist device is seeded with the root element's font size and line height so rem/rlh units do not fall back to the 16px default (packages/blitz-dom/src/document.rs:2723-2744); `rem` resolves against the root font-size after viewport and hidpi changes, with an initial root font-size of 16px (tests/blitz-tests/tests/rem_after_viewport_change.rs:1-8; tests/blitz-tests/tests/rem_after_viewport_change.rs:49-77)
### typography#3.12 · values · typography · ## Typography · lines 53–53 · stale
- Font emboldening is enabled by the font-embolden feature, or apple-font-embolden on macOS and iOS (packages/blitz-paint/Cargo.toml:18-19; packages/blitz-paint/src/lib.rs:23-27); strength is 0.015125 and 0.0121 times the CSS font size, each capped at 0.3, with hinting off on a build that emboldens; a synthesized group header on a plain build is emboldened with hinting on, and on an emboldening build it takes twice the stated strength (packages/blitz-paint/src/text.rs:678-695)
### typography#5.1 · absence · typography · ## Typography · lines 72–72 · stale
- font-family declarations · searched: `font-family|font_family` over the 16 listed s08 files (no match)
### typography#5.2 · absence · typography · ## Typography · lines 73–73 · stale
- font family declarations · searched: `font_family|font-family|FontFamily` over the 32 listed s09 files
### typography#5.3 · absence · typography · ## Typography · lines 74–74 · stale
- font-family declarations outside test fixtures · searched: `font-family|font_family` over the 32 s10 slice files (matches only packages/blitz-vibey-script/tests/dom.rs)
### typography#5.4 · absence · typography · ## Typography · lines 75–75 · stale
- a project type scale or font tokens · searched: `font-family` over the 61 s12 slice files (one fixture hit, `sans-serif`)
### spacing#2.5 · values · spacing · ## Spacing · lines 20–20 · stale
- blitz-dom default body margin 8px (packages/blitz-dom/assets/default.css:309-312)
### spacing#2.6 · values · spacing · ## Spacing · lines 21–21 · stale
- blitz-dom default `multiple` select rows (`select[multiple] > option`, `select[multiple] > optgroup > option`) padding 0 2px (packages/blitz-dom/assets/default.css:173-177)
### spacing#2.7 · values · spacing · ## Spacing · lines 22–22 · stale
- blitz-dom `multiple` select with `size`: `size_rows` hints a `height` of `size × 1.2em`, `size` capped at 65534, `0` or a bad value giving no hint; `select[multiple][size]` takes `overflow-y: auto`, its rows' line height held in §Typography (packages/blitz-dom/src/node/select_list.rs:125-130; packages/blitz-dom/src/stylo.rs:1132-1147; packages/blitz-dom/assets/default.css:183-193)
### spacing#3.1 · values · spacing · ## Spacing · lines 28–28 · stale
- `hspace` and `vspace` map to horizontal and vertical margins on `embed`, `img`, `object`, `marquee` and image inputs (packages/blitz-dom/src/stylo.rs:1085-1111)
### spacing#3.2 · values · spacing · ## Spacing · lines 29–29 · stale
- Body `marginwidth`, `marginheight`, `leftmargin` and `topmargin` map to pixel margins; `rightmargin` and `bottommargin` are deliberately ignored (packages/blitz-dom/src/stylo.rs:1171-1208)
### spacing#3.4 · values · spacing · ## Spacing · lines 31–31 · stale
- An open select list shows at most `MAX_ROWS` = 20 rows and scrolls past them; its box and rows use the engine constants `BORDER_WIDTH`, `ROW_PADDING_X`, `ROW_PADDING_Y` and `GROUP_INDENT` (tests/blitz-tests/tests/select_popup.rs:866-907; as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)
### spacing#3.6 · values · spacing · ## Spacing · lines 33–33 · stale
- Single-line text inputs are vertically centered within their content box (packages/blitz-dom/src/node/node.rs:871-889)
### spacing#3.7 · values · spacing · ## Spacing · lines 34–34 · stale
- Outside list markers using a character are padded 8 CSS px from the item's border box (packages/blitz-paint/src/render.rs:1099-1110)
