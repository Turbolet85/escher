# design · cascade read · group 2 of 4

## Carried
- (.andromeda/design-system-detail/typography.md:8) apps/browser is reached whole by the take, so the browser chrome text sizes must hold and their browser.css cites be re-read wherever the take carries apps/browser's commits.
  > | Role | Font | Weight | Size | Tracking | Usage |
  > |------|------|--------|------|----------|-------|
  > | Browser chrome text | sans-serif | not stated | tab title 12px, urlbar 14px (16px on mobile), menu items 14px, status bar 12px | not stated | (apps/browser/assets/browser.css:93-94; apps/browser/assets/browser.css:157-160; apps/browser/assets/browser.css:170; apps/browser/assets/browser.css:225-226; apps/browser/assets/browser.css:297-298) |
- (.andromeda/design-system-detail/typography.md:11) A value inside apps/browser, reached whole: it must hold and its cite be re-read where the take carries apps/browser.
  > | Browser suggestion section header | not stated | 600 | 11px | letter-spacing 0.04em, uppercase | (apps/browser/assets/browser.css:329-336) |
- (.andromeda/design-system-detail/typography.md:12) A value inside apps/browser, reached whole: it must hold and its cite be re-read where the take carries apps/browser.
  > | Browser FPS overlay | monospace | not stated | 12px | not stated | (apps/browser/assets/browser.css:269-270) |
- (.andromeda/design-system-detail/typography.md:13) A value inside apps/browser, reached whole: it must hold and its cite be re-read where the take carries apps/browser.
  > | about:history | `system-ui, sans-serif` | heading 600 | heading 24px | not stated | (apps/browser/assets/about-history.css:7; apps/browser/assets/about-history.css:9-11) |
- (.andromeda/design-system-detail/spacing.md:16) A spacing value inside apps/browser, reached whole: it must hold and its cites be re-read where the take carries apps/browser.
  > - Browser urlbar padding 6px, gap 6px; tab padding 0 8px; menu padding 8px with items 8px 12px gap 8px; suggestion rows 6px 12px (apps/browser/assets/browser.css:39; apps/browser/assets/browser.css:132-133; apps/browser/assets/browser.css:214; apps/browser/assets/browser.css:223; apps/browser/assets/browser.css:231; apps/browser/assets/browser.css:338)
- (.andromeda/design-system-detail/typography.md:15) The default family the stand's form controls are painted with is an input of the rows stand_contrast re-asserts after the take, read through a possibly moved Stylo, so it must hold unchanged.
  > | blitz-dom default form controls | inputs/selects/buttons `system-ui, sans-serif`; textarea monospace | not stated | not stated | not stated | (packages/blitz-dom/assets/default.css:88-96; packages/blitz-dom/assets/default.css:311-357; packages/blitz-dom/assets/default.css:368-376; packages/blitz-dom/assets/default.css:672-677) |
- (.andromeda/design-system-detail/typography.md:18) The stand's base size and title weight decide each stand_contrast row's required ratio (3.0 large, else 4.5), so they must read the same after the take and every row be re-asserted at them.
  > | seven_guis | sans-serif | home title 700 | base 14px; home title 36px | not stated | (examples/seven_guis/src/app.rs:179-205) |
- (.andromeda/design-system-detail/typography.md:19) The flight booker select's size and weight are what its stand_contrast row is measured at, so they must read 15px and 400 after the take.
  > | seven_guis flight booker's `flight-type` select (`.type-select`) | not stated | 400 | 15px | not stated | (examples/seven_guis/src/tasks/flight_booker.rs:160-167) |
- (.andromeda/design-system-detail/typography.md:22) node/text.rs is edited by a09f46792 and 527394724, so the text input editor must still be constructed at 16px beside upstream's edit and the password mask, its cite re-read on the take's tree.
  > | Text input editors | not stated | not stated | `parley::PlainEditor::new(16.0)` | not stated | (packages/blitz-dom/src/node/text.rs:86) |
- (.andromeda/design-system-detail/typography.md:31) The stand's bundled font is the family every re-run stand check measures at; it must stay unchanged by the take, its record held in architecture's Headless stand contract.
  > - The seven_guis headless stand's font registration is held in architecture §Standard Contracts → Headless stand, beside `font_ctx()`, not as a design-system typography value
- (.andromeda/design-system-detail/typography.md:32) The stand's `font_ctx()` builds through `build_single_font_ctx`, whose blitz-dom lib.rs a take carrying git Stylo (06ecec886) moves, so it must still register one font for every generic with system fonts disabled.
  > - A custom `FontContext` can be set; on WASM a context with bundled fonts must be provided, using `build_single_font_ctx` for one font (packages/dioxus-native/src/config.rs:56-64); `build_single_font_ctx` registers one font as fallback for SansSerif, Serif, Monospace and SystemUi with system fonts disabled (packages/blitz-dom/src/lib.rs:134-165)
  (.andromeda/design-system.md:48)
  > On WASM a font context with bundled fonts must be provided, `build_single_font_ctx` for one font.
- (.andromeda/design-system-detail/typography.md:33) dioxus-native/Cargo.toml is edited by 527394724 (the `clipboard` feature leaves its defaults), so the font features must stay in place and their cited lines be re-read.
  > - Font features: system-fonts, woff, complex-scripts (dictionary line-breaking), font-embolden and apple-font-embolden (packages/dioxus-native/Cargo.toml:20-25; packages/dioxus-native/Cargo.toml:47-48)
- (.andromeda/design-system-detail/typography.md:34) document.rs is a merge surface (0980cce5f, and 06ecec886 under a Stylo take), so the bullet font the password mask paints with must stay registered in the default font context.
  > - A bullet font is always registered in the default font context (packages/blitz-dom/src/document.rs:440-442; packages/blitz-dom/src/lib.rs:32)
- (.andromeda/design-system-detail/typography.md:35) The fonts-unavailable refusal lives in document.rs, a merge surface, and the chunk requires it unchanged: a shot or a lint of an app with no `sans-serif` family refused with nothing painted, the stand's bundled font passing.
  > - Since 2026-10-12-headless-screenshot a driver `screenshot` of an app whose font context resolves no `sans-serif` family (`BaseDocument::can_paint_text` false) is refused `fonts-unavailable` with nothing painted — a shot never paints text skipped; the stand's bundled font passes the check (packages/blitz-dom/src/document.rs:1134-1147; as measured at escher-0.1.0/chunks/2026-10-12-headless-screenshot/report.md)
  (.andromeda/design-system.md:44)
  > A driver `screenshot` of an app whose font context resolves no `sans-serif` family is refused `fonts-unavailable` with nothing painted — a shot never paints text skipped.
- (.andromeda/design-system-detail/typography.md:36) The `document.fonts` stub lives in runtime.rs, a predicted conflict surface that e6f4615ec splits into modules, so the stub must still report every font loaded and its cite move to where the take puts it.
  > - `document.fonts` is a stub FontFaceSet where all fonts report as loaded (packages/blitz-vibey-script/src/runtime.rs:1060-1074)
- (.andromeda/design-system-detail/typography.md:43) stylo_to_parley.rs is moved by 06ecec886 under a take that carries git Stylo, so the generic-family mapping must hold there.
  > - Generic font families map to Parley generics, with `None` as sans-serif (packages/blitz-dom/src/stylo_to_parley.rs:50-60)
- (.andromeda/design-system-detail/typography.md:44) stylo_to_parley.rs is moved by 06ecec886 under a git-Stylo take, so the weight, width, style and variation conversions must hold there.
  > - Font weight, width, style and variation settings convert to Parley values (packages/blitz-dom/src/stylo_to_parley.rs:87-112)
- (.andromeda/design-system-detail/typography.md:46) layout/mod.rs is moved by 0980cce5f (and 06ecec886), and `line-height: normal` at 1.2 × font size is an input of every stand bound re-measured after the take, so it must hold or a moved bound be recorded, never loosened.
  > - Line height maps `normal`, number and length to Parley line heights (packages/blitz-dom/src/stylo_to_parley.rs:407-412); the layout engine resolves `line-height: normal` as 1.2 × font size (packages/blitz-dom/src/layout/mod.rs:232-236)
- (.andromeda/design-system-detail/typography.md:48) stylo.rs is edited by 53e9d81c6 (and 06ecec886 under a Stylo take), so the `lang`-to-locale read must hold and its cite be re-read.
  > - Text locale comes from the computed `_x_lang` value, which the `lang` attribute sets (packages/blitz-dom/src/stylo_to_parley.rs:495; packages/blitz-dom/src/stylo.rs:1201-1203)
- (.andromeda/design-system-detail/typography.md:49) stylo_to_parley.rs is moved by 06ecec886 under a git-Stylo take, so the alignment and wrapping maps must hold there.
  > - `text-align` and `text-align-last` map to Parley alignment (packages/blitz-dom/src/stylo_to_parley.rs:304-329); word break, line break, overflow wrap, wrap mode and white-space collapse map to Parley settings (packages/blitz-dom/src/stylo_to_parley.rs:331-346; packages/blitz-dom/src/stylo_to_parley.rs:467-484)
- (.andromeda/design-system-detail/typography.md:51) document.rs is a merge surface, so the stylist device must still be seeded with the root's font size and line height, `rem_after_viewport_change` green after the take.
  > - The new stylist device is seeded with the root element's font size and line height so rem/rlh units do not fall back to the 16px default (packages/blitz-dom/src/document.rs:2322-2343); `rem` resolves against the root font-size after viewport and hidpi changes, with an initial root font-size of 16px (tests/blitz-tests/tests/rem_after_viewport_change.rs:1-8; tests/blitz-tests/tests/rem_after_viewport_change.rs:49-77)
- (.andromeda/design-system-detail/typography.md:52) layout/list.rs is moved by 06ecec886 under a git-Stylo take, and the disclosure markers are what a `<details>` summary paints beside the `:open` change (53e9d81c6), so the marker set must hold.
  > - List markers: decimal `"N. "`, lower/upper-alpha, disc `•`, circle `◦`, square `▪`, disclosure-open `▾`, disclosure-closed `▸`, other names `□` (packages/blitz-dom/src/layout/list.rs:116-158)
- (.andromeda/design-system-detail/spacing.md:30) node/node.rs is a predicted conflict surface (0980cce5f, ebacd68bd), so single-line text inputs must stay vertically centred through its resolution, its cite re-read.
  > - Single-line text inputs are vertically centered within their content box (packages/blitz-dom/src/node/node.rs:860-878)

## Bindings
- **typography (the stand's sizes and weights) ↔ color-palette (the stand's text/background pairs)**: stand_contrast's required ratio per row is set by the size and weight it reads (.andromeda/design-system-detail/typography.md:18) (.andromeda/design-system-detail/typography.md:19), over the colour pairs another group carries; a row re-asserted after the take reads both.
- **typography (the stand's bundled font) ↔ architecture (Standard Contracts → Headless stand)**: the family every stand check measures at is recorded in architecture, not here (.andromeda/design-system-detail/typography.md:31), built through `build_single_font_ctx` (.andromeda/design-system-detail/typography.md:32); a take that moves either keeps both records in step.
- **spacing (single-line input centring) ↔ depth (the hit stop at a clipping box)**: both rules live in node/node.rs, one conflict resolution (.andromeda/design-system-detail/spacing.md:30) (.andromeda/design-system.md:85); the resolution keeps both.
- **typography (the `document.fonts` stub) ↔ security plan (the script engine as an untrusted-input surface)**: e6f4615ec's split of runtime.rs moves the stub (.andromeda/design-system-detail/typography.md:36) in the same take that narrows the script fetcher; the stub's behaviour and the fetcher rule are re-read on one tree.
