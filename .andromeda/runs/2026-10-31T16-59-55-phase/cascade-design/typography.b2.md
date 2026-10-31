### typography#3.5 · values · typography · ## Typography · lines 45–45
- Letter and word spacing resolve against the font size (packages/blitz-dom/src/stylo_to_parley.rs:415-424)
### typography#3.6 · values · typography · ## Typography · lines 46–46
- Text locale comes from the computed `_x_lang` value, which the `lang` attribute sets (packages/blitz-dom/src/stylo_to_parley.rs:495; packages/blitz-dom/src/stylo.rs:1201-1203)
### typography#3.7 · values · typography · ## Typography · lines 47–47
- `text-align` and `text-align-last` map to Parley alignment (packages/blitz-dom/src/stylo_to_parley.rs:304-329); word break, line break, overflow wrap, wrap mode and white-space collapse map to Parley settings (packages/blitz-dom/src/stylo_to_parley.rs:331-346; packages/blitz-dom/src/stylo_to_parley.rs:467-484)
### typography#3.8 · values · typography · ## Typography · lines 48–48
- `ch` and `ic` metrics measure `'0'` and `'\u{6C34}'` advances scaled like Parley's shaped glyph advances (packages/blitz-dom/src/font_metrics.rs:104-161)
### typography#3.9 · values · typography · ## Typography · lines 49–49
- The new stylist device is seeded with the root element's font size and line height so rem/rlh units do not fall back to the 16px default (packages/blitz-dom/src/document.rs:2352-2373); `rem` resolves against the root font-size after viewport and hidpi changes, with an initial root font-size of 16px (tests/blitz-tests/tests/rem_after_viewport_change.rs:1-8; tests/blitz-tests/tests/rem_after_viewport_change.rs:49-77)
### typography#3.10 · values · typography · ## Typography · lines 50–50
- List markers: decimal `"N. "`, lower/upper-alpha, disc `•`, circle `◦`, square `▪`, disclosure-open `▾`, disclosure-closed `▸`, other names `□` (packages/blitz-dom/src/layout/list.rs:116-158)
### typography#3.11 · values · typography · ## Typography · lines 51–51
- Font emboldening is enabled by the font-embolden feature, or apple-font-embolden on macOS and iOS (packages/blitz-paint/Cargo.toml:18-19; packages/blitz-paint/src/lib.rs:22-26); strength is 0.015125 and 0.0121 times the CSS font size, each capped at 0.3, with hinting off when emboldening (packages/blitz-paint/src/text.rs:640-651)
### typography#3.12 · values · typography · ## Typography · lines 52–52
- Auto text-decoration thickness is font-size / 10 with a 1px minimum, floored to whole device pixels (packages/blitz-paint/src/text.rs:254-277); overline and line-through positions use the font's OS/2 usWinAscent, cached per font face (packages/blitz-paint/src/text.rs:70-95; packages/blitz-paint/src/text.rs:494-544)
### typography#4.1 · values · typography · ## Typography · lines 58–58
- `font-family: sans-serif` for body and `monospace` for code labels (examples/assets/cursor.html:6; examples/assets/cursor.html:46; examples/assets/border-styles.html:6; examples/assets/border-styles.html:38); the Google fixtures set 14px `arial,sans-serif` (examples/assets/bottom_only.html:8; examples/assets/google.html:22-26), the bar 13px/27px Roboto (examples/assets/google.html:45-48) and Google Sans,Roboto,Helvetica,Arial,sans-serif stacks (examples/assets/google.html:563; examples/assets/google.html:1050)
### typography#4.2 · values · typography · ## Typography · lines 59–59
- graphite: html/body Inter Variable 18px weight 500 line-height 1.5, 16px at width<=780px; h1 Bona Nova, Palatino, serif 2.66667rem weight 700, h2 1.75rem, h3 1.25rem, h4-h6 1rem Inter Variable weight 800; `--font-size-link` calc(1rem*4/3); Inter Variable and Bona Nova @font-face rules; `--nav-font-size` from 28px to 8px across breakpoints (examples/assets/graphite.html:918-930; examples/assets/graphite.html:940-943; examples/assets/graphite.html:1322-1330; examples/assets/graphite.html:1341-1366; examples/assets/graphite.html:915; examples/assets/graphite.html:1635-1758; examples/assets/graphite.html:1761-1822; examples/assets/graphite.html:986-995; examples/assets/graphite.html:1149-1156)
### typography#4.3 · values · typography · ## Typography · lines 60–60
- gosub `"Arial", sans-serif` with h1 at 6em overridden to 72px (examples/assets/gosub.html:10; examples/assets/gosub.html:20-21; examples/assets/gosub.html:74-75); servo-new-reduced-1 `'Space Grotesk', sans-serif` (examples/assets/servo-new-reduced-1.html:13); pseudo `.qqq` Font Awesome 6 Brands glyphs at 32px (examples/assets/pseudo.html:21-25; examples/assets/pseudo.html:80)
### typography#4.4 · values · typography · ## Typography · lines 61–61
- servo.css headings "Fira Sans", code/pre "Fira Mono", body stack "Fira Sans" … sans-serif weight 400 line-height 1.5, loaded from Google Fonts by servo.html (examples/assets/servo.css:1-23; examples/assets/servo.css:229-234; examples/assets/servo.css:220-227; examples/assets/servo.css:236-241; examples/assets/servo.html:27)
### typography#4.5 · values · typography · ## Typography · lines 62–62
- custom-widget `system-ui, sans` (examples/custom_widget.rs:179); Preact TodoMVC `16px/1.4 system-ui, -apple-system, "Segoe UI", Roboto, sans-serif` with a 64px weight-200 heading (examples/preact/index.html:12; examples/preact/index.html:17); reference page `15px/1.55` system stack and ui-monospace code (examples/preact/core_dom_apis.html:10; examples/preact/core_dom_apis.html:23)
### typography#4.6 · values · typography · ## Typography · lines 63–63
- text-decoration fixture exercises text-decoration-line/thickness/style, text-underline-offset/position and text-decoration-inset (examples/assets/text-decoration.html:48-102)
### typography#5.1 · absence · typography · ## Typography · lines 69–69
- font-family declarations · searched: `font-family|font_family` over the 16 listed s08 files (no match)
### typography#5.2 · absence · typography · ## Typography · lines 70–70
- font family declarations · searched: `font_family|font-family|FontFamily` over the 32 listed s09 files
### typography#5.3 · absence · typography · ## Typography · lines 71–71
- font-family declarations outside test fixtures · searched: `font-family|font_family` over the 32 s10 slice files (matches only packages/blitz-vibey-script/tests/dom.rs)
### typography#5.4 · absence · typography · ## Typography · lines 72–72
- a project type scale or font tokens · searched: `font-family` over the 61 s12 slice files (one fixture hit, `sans-serif`)
