# design · Typography — category read

## Carried
- (.andromeda/design-system-detail/typography.md:15) The opened list's option and group-label text is a select's own text and must resolve to the engine's default form-control stack, `system-ui, sans-serif`, in `default.css`, not to a second font of its own; this row states no weight or size, so any the list sets is the chunk's to state.
  > | blitz-dom default form controls | inputs/selects/buttons `system-ui, sans-serif`; textarea monospace | not stated | not stated | not stated | (packages/blitz-dom/assets/default.css:88-96; packages/blitz-dom/assets/default.css:339-385; packages/blitz-dom/assets/default.css:396-404; packages/blitz-dom/assets/default.css:700-705) |
- (.andromeda/design-system-detail/typography.md:21) The list's text, where the chunk sets no size of its own, must take the engine's 16px base for a non-monospace generic, and option boxes and their tests must read that size.
  > | Engine generic base size | generics | not stated | monospace 13px; other generics 16px | not stated | (packages/blitz-dom/src/font_metrics.rs:167-176) |
- (.andromeda/design-system-detail/typography.md:44) An option row whose line height the chunk leaves at `normal` must measure 1.2 × its font size, since the real boxes an open list's options read in the snapshot and the lint derive from it.
  > the layout engine resolves `line-height: normal` as 1.2 × font size (packages/blitz-dom/src/layout/mod.rs:232-236)
- (.andromeda/design-system-detail/typography.md:72) The chunk must not cite or invent a type token or scale for the list: none is recorded for the project, so its type is an engine default, as the scope already holds for its colours.
  > a project type scale or font tokens · searched: `font-family` over the 61 s12 slice files (one fixture hit, `sans-serif`)
- (.andromeda/design-system-detail/typography.md:33) A driver `screenshot` of an open list must honour this guard: with no `sans-serif` family resolved it stays refused `fonts-unavailable`, with no shot that paints the list's text skipped, and the overlay's text adds no path around the check.
  > Since 2026-10-12-headless-screenshot a driver `screenshot` of an app whose font context resolves no `sans-serif` family (`BaseDocument::can_paint_text` false) is refused `fonts-unavailable` with nothing painted — a shot never paints text skipped; the stand's bundled font passes the check (packages/blitz-dom/src/document.rs:1164-1177; as measured at escher-0.1.0/chunks/2026-10-12-headless-screenshot/report.md)
- (.andromeda/design-system-detail/typography.md:35) Any `blitz-tests` assertion on the list's text or on boxes sized by it must run with `system-fonts` on, or it passes vacuously on 0x0 text.
  > `blitz-dom` is built with the `system-fonts` feature for the blitz-tests crate; without it text measures 0x0 and font-dependent assertions pass vacuously, and the feature is stated to be enabled by default when testing the whole workspace (tests/blitz-tests/Cargo.toml:17; tests/blitz-tests/tests/br_trailing_line.rs:11-13; tests/blitz-tests/tests/inline_box_baseline.rs:4-6)
- (.andromeda/design-system.md:48) The list paints "in every document and renderer", so on WASM its text must come from the document's provided font context, and the overlay must take no font from anywhere else.
  > On WASM a font context with bundled fonts must be provided, `build_single_font_ctx` for one font.

## Bindings
- **The list's text size ↔ a11y-plan SC 1.4.3**: the size the list's text reads, 16px by the engine's base unless the chunk sets another, is the input to the contrast threshold its new engine-default colours must clear; the size here, the pair and ratio in the a11y plan and Color Palette (.andromeda/design-system-detail/typography.md:21)
- **The option row's line height ↔ Spacing**: a row's box is its line height here plus whatever padding the chunk states under Spacing, both read by the snapshot's boxes and the lint's `overlaps` restatement (.andromeda/design-system-detail/typography.md:44)
- **The `fonts-unavailable` guard ↔ architecture §Standard Contracts → Headless stand**: the stand's shots of `select#flight-type` opened pass the guard on the font registration architecture holds, and this category holds the refusal (.andromeda/design-system-detail/typography.md:29)
- **`system-fonts` ↔ test-plan**: the chunk's font-dependent list assertions in `blitz-tests` name this feature as their precondition (.andromeda/design-system-detail/typography.md:35)
- **The WASM font context ↔ Surface: web-spa**: the same list on the canvas surface reads the bundled font context this rule requires, the overlay living in the engine and not in a shell (.andromeda/design-system.md:48)
