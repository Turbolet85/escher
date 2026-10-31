### color-palette#1.1 · values · palette · ## Color Palette · lines 8–8
- The `Color` type is `AlphaColor<Srgb>` and Stylo colors convert to it via sRGB (packages/blitz-dom/src/util.rs:12; packages/blitz-dom/src/util.rs:166-178)
### color-palette#1.2 · values · palette · ## Color Palette · lines 9–9
- The system color scheme is an enum `Light` (default) / `Dark`, set on the viewport, and it feeds `prefers-color-scheme` (packages/blitz-traits/src/shell.rs:68-74; tests/blitz-tests/tests/device_coalescing.rs:101; packages/blitz-dom/src/stylo_device.rs:80-83)
### color-palette#1.6 · values · palette · ## Color Palette · lines 13–13
- The frame clear color is an optional `base_color` (a `peniko::Color`), unset by default (packages/dioxus-native/src/config.rs:12; packages/dioxus-native/src/config.rs:40; packages/dioxus-native/src/config.rs:76-80; packages/dioxus-native/src/dioxus_renderer.rs:37-38)
### color-palette#1.7 · values · palette · ## Color Palette · lines 14–14
- The canvas background is the root element's background colour, falling back to body's when the root is transparent (packages/blitz-paint/src/render.rs:193-228)
### color-palette#2.3 · values · palette · ## Color Palette › ### Core Colors · lines 27–27
| seven_guis stand | — | The stand's accent (its load page's `load-fetch` included), its flight booker's success-message colours and its flight-type select's border are held in layout-templates §Surface: desktop-native → Primary screens (`values · desktop-native-screens`), not as design-system values |
### color-palette#2.4 · values · palette · ## Color Palette › ### Core Colors · lines 28–28
| blitz-dom default link | `rgb(0, 0, 238)` | Default stylesheet link, its rule selecting `a[href]` — an anchor with no `href` takes neither the colour nor the underline; input focus outline `#4D90FE`; button background `#EFEFEF` (packages/blitz-dom/assets/default.css:43-46; packages/blitz-dom/assets/default.css:98-101; packages/blitz-dom/assets/default.css:111) |
### color-palette#2.5 · values · palette · ## Color Palette › ### Core Colors · lines 29–29
| Default text-selection highlight | rgb 180, 213, 255 | Engine default (packages/blitz-paint/src/lib.rs:28-29) |
### color-palette#5.1 · values · status-states · ## Color Palette › ### Semantic Colors · lines 68–70
| State | Values (as declared) | Usage |
|-------|----------------------|-------|
| Disabled checkbox/radio | rgba 209, 209, 209, 255 | Disabled accent; otherwise the element's color stands in for accent-color per a TODO (packages/blitz-paint/src/render/form_controls.rs:21-26) |
### color-palette#7.3 · values · palette · ## Color Palette › ### Border Progression · lines 96–96
| Focus — blitz-dom default input outline | `#4D90FE` | (packages/blitz-dom/assets/default.css:43-46; packages/blitz-dom/assets/default.css:98-101; packages/blitz-dom/assets/default.css:111) |
### color-palette#8.3 · values · palette · ## Color Palette › ### Border Progression · lines 104–104
- Unchecked radio rings use the CSS palette GRAY; checkbox ticks/gaps use white (packages/blitz-paint/src/render/form_controls.rs:76-79; packages/blitz-paint/src/render/form_controls.rs:99-101)
### typography#1.6 · values · typography · ## Typography · lines 15–15
| blitz-dom default form controls | inputs/selects/buttons `system-ui, sans-serif`; textarea monospace | not stated | not stated | not stated | (packages/blitz-dom/assets/default.css:88-96; packages/blitz-dom/assets/default.css:311-357; packages/blitz-dom/assets/default.css:368-376; packages/blitz-dom/assets/default.css:672-677) |
### typography#1.7 · values · typography · ## Typography · lines 16–16
| blitz-dom default headings | not stated | bold | h1–h6 from 2em down to 0.67em | not stated | (packages/blitz-dom/assets/default.css:88-96; packages/blitz-dom/assets/default.css:311-357; packages/blitz-dom/assets/default.css:368-376; packages/blitz-dom/assets/default.css:672-677) |
### typography#1.11 · values · typography · ## Typography · lines 20–20
| Text input editors | not stated | not stated | `parley::PlainEditor::new(16.0)` | not stated | (packages/blitz-dom/src/node/text.rs:86) |
### typography#1.12 · values · typography · ## Typography · lines 21–21
| Engine generic base size | generics | not stated | monospace 13px; other generics 16px | not stated | (packages/blitz-dom/src/font_metrics.rs:167-176) |
### typography#2.2 · values · typography · ## Typography · lines 29–29
- The seven_guis headless stand's font registration is held in architecture §Standard Contracts → Headless stand, beside `font_ctx()`, not as a design-system typography value
### typography#3.1 · values · typography · ## Typography · lines 41–41
- Generic font families map to Parley generics, with `None` as sans-serif (packages/blitz-dom/src/stylo_to_parley.rs:50-60)
### typography#3.2 · values · typography · ## Typography · lines 42–42
- Font weight, width, style and variation settings convert to Parley values (packages/blitz-dom/src/stylo_to_parley.rs:87-112)
### typography#3.4 · values · typography · ## Typography · lines 44–44
- Line height maps `normal`, number and length to Parley line heights (packages/blitz-dom/src/stylo_to_parley.rs:407-412); the layout engine resolves `line-height: normal` as 1.2 × font size (packages/blitz-dom/src/layout/mod.rs:232-236)
### typography#3.5 · values · typography · ## Typography · lines 45–45
- Letter and word spacing resolve against the font size (packages/blitz-dom/src/stylo_to_parley.rs:415-424)
### typography#3.6 · values · typography · ## Typography · lines 46–46
- Text locale comes from the computed `_x_lang` value, which the `lang` attribute sets (packages/blitz-dom/src/stylo_to_parley.rs:495; packages/blitz-dom/src/stylo.rs:1201-1203)
### typography#3.7 · values · typography · ## Typography · lines 47–47
- `text-align` and `text-align-last` map to Parley alignment (packages/blitz-dom/src/stylo_to_parley.rs:304-329); word break, line break, overflow wrap, wrap mode and white-space collapse map to Parley settings (packages/blitz-dom/src/stylo_to_parley.rs:331-346; packages/blitz-dom/src/stylo_to_parley.rs:467-484)
### typography#3.9 · values · typography · ## Typography · lines 49–49
- The new stylist device is seeded with the root element's font size and line height so rem/rlh units do not fall back to the 16px default (packages/blitz-dom/src/document.rs:2322-2343); `rem` resolves against the root font-size after viewport and hidpi changes, with an initial root font-size of 16px (tests/blitz-tests/tests/rem_after_viewport_change.rs:1-8; tests/blitz-tests/tests/rem_after_viewport_change.rs:49-77)
### typography#3.10 · values · typography · ## Typography · lines 50–50
- List markers: decimal `"N. "`, lower/upper-alpha, disc `•`, circle `◦`, square `▪`, disclosure-open `▾`, disclosure-closed `▸`, other names `□` (packages/blitz-dom/src/layout/list.rs:116-158)
### typography#5.3 · absence · typography · ## Typography · lines 71–71
- font-family declarations outside test fixtures · searched: `font-family|font_family` over the 32 s10 slice files (matches only packages/blitz-vibey-script/tests/dom.rs)
