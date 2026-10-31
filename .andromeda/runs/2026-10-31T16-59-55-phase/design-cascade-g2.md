# design · cascade read · group 2 of 4

## Carried
- (.andromeda/design-system-detail/typography.md:15) The open list's rows and group labels in `default.css` must state or inherit their family against this form-control default, so a row and the closed select's label read one family unless a fork says otherwise.
  > | blitz-dom default form controls | inputs/selects/buttons `system-ui, sans-serif`; textarea monospace | not stated | not stated | not stated | (packages/blitz-dom/assets/default.css:88-96; packages/blitz-dom/assets/default.css:339-385; packages/blitz-dom/assets/default.css:396-404; packages/blitz-dom/assets/default.css:700-705) |
- (.andromeda/design-system-detail/typography.md:21) A row's size is an input of every width, row height and contrast reading of the list, so the plan must name the size a row is painted and measured at, this generic base where the defaults set none.
  > | Engine generic base size | generics | not stated | monospace 13px; other generics 16px | not stated | (packages/blitz-dom/src/font_metrics.rs:167-176) |
- (.andromeda/design-system-detail/typography.md:28) The list must paint its text on the WASM canvas surface, where its family resolves only through these bundled generics, so its default family must stay inside them.
  > - WASM builds register bundled DejaVu Sans for sans-serif, serif, monospace and system-ui (examples/wasm_hello/src/lib.rs:75-100)
- (.andromeda/design-system-detail/typography.md:29) A new row over the list in `stand_contrast`, `stand_keyboard`, `stand_act_select` or `stand_lint` must be measured at the stand's own font registration, read where this line sends the reader.
  > - The seven_guis headless stand's font registration is held in architecture §Standard Contracts → Headless stand, beside `font_ctx()`, not as a design-system typography value
- (.andromeda/design-system-detail/typography.md:30) The overlay lives in the engine for every document and renderer, so its rows must shape and paint through the document's own font context and build none of their own, a one-font context included.
  > - A custom `FontContext` can be set; on WASM a context with bundled fonts must be provided, using `build_single_font_ctx` for one font (packages/dioxus-native/src/config.rs:56-64); `build_single_font_ctx` registers one font as fallback for SansSerif, Serif, Monospace and SystemUi with system fonts disabled (packages/blitz-dom/src/lib.rs:134-165)
  (.andromeda/design-system.md:48)
  > On WASM a font context with bundled fonts must be provided, `build_single_font_ctx` for one font.
- (.andromeda/design-system-detail/typography.md:41) A row's and a group label's family must reach Parley through this mapping, so an unstated family on a row reads as sans-serif and not as a new overlay-only fallback.
  > - Generic font families map to Parley generics, with `None` as sans-serif (packages/blitz-dom/src/stylo_to_parley.rs:50-60)
- (.andromeda/design-system-detail/typography.md:42) The weight of a row and of a group label, and any width or style a default gives them, must reach the list's text through this conversion and no second one.
  > - Font weight, width, style and variation settings convert to Parley values (packages/blitz-dom/src/stylo_to_parley.rs:87-112)
- (.andromeda/design-system-detail/typography.md:44) The list's row height, unset as yet, and the box `size` reads must agree with this `line-height` resolution, so a stated row height and a measured one do not part.
  > - Line height maps `normal`, number and length to Parley line heights (packages/blitz-dom/src/stylo_to_parley.rs:407-412); the layout engine resolves `line-height: normal` as 1.2 × font size (packages/blitz-dom/src/layout/mod.rs:232-236)
- (.andromeda/design-system-detail/typography.md:51) The new overlay is painted in `packages/blitz-paint/src`, so its rows and any weighted group label must go through this text path and keep its emboldening, strength cap and hinting rule.
  > - Font emboldening is enabled by the font-embolden feature, or apple-font-embolden on macOS and iOS (packages/blitz-paint/Cargo.toml:18-19; packages/blitz-paint/src/lib.rs:22-26); strength is 0.015125 and 0.0121 times the CSS font size, each capped at 0.3, with hinting off when emboldening (packages/blitz-paint/src/text.rs:640-651)
- (.andromeda/design-system-detail/typography.md:69) The list's family must be stated in the stylesheet, so `node/select.rs` and the other DOM-node files keep holding no family declaration of their own.
  > - font-family declarations · searched: `font-family|font_family` over the 16 listed s08 files (no match)
- (.andromeda/design-system-detail/typography.md:72) The list's text values must be engine defaults in their own right, since no project scale or font token exists for them to read and the chunk introduces none.
  > - a project type scale or font tokens · searched: `font-family` over the 61 s12 slice files (one fixture hit, `sans-serif`)
  (.andromeda/design-system.md:56)
  > > NOT YET MEASURED — a project-wide type scale: the examples slice (s01) recorded the project's own type scale as out of slice, and the WPT runner slice (s13) recorded typography as out of slice.
- (.andromeda/design-system-detail/spacing.md:21) The list's row defaults must not match a `multiple` select's rows, whose padding and paint stay as built apart from `size`, and the list's own padding must be set beside this value and stated against it.
  > - blitz-dom default `multiple` select rows (`select[multiple] > option`, `select[multiple] > optgroup > option`) padding 0 2px (packages/blitz-dom/assets/default.css:173-177)
  (.andromeda/design-system.md:70)
  > > NOT YET MEASURED — a project-wide spacing scale or base unit: the examples, engine, test and WPT slices (s01, s05, s07, s10, s11, s12, s13) recorded a spacing scale as out of slice.
- (.andromeda/design-system-detail/spacing.md:29) A long list that scrolls must paint and answer its thumb at this geometry in the overlay, and the plan must settle how a thumb over a row's right edge and a list too short for the 32 CSS px minimum behave.
  > - Overlay scrollbar thumb geometry: thickness 10.0, thin thickness 6.0, margin 2.0 and minimum length 32.0 CSS px (packages/blitz-dom/src/node/scrollbar.rs:122-126)

## Bindings
- **Row text ↔ row spacing**: the row height the list sets must hold the `line-height` its text resolves to, plus the row padding set beside the `multiple` rows' own, (.andromeda/design-system-detail/typography.md:44) with (.andromeda/design-system-detail/spacing.md:21).
- **Row family and size ↔ colour and contrast**: the contrast of the list's text and row states under SC 1.4.3 is measured at the family and size the row is painted with, on the colours that are this chunk's new engine default, (.andromeda/design-system-detail/typography.md:15) and (.andromeda/design-system-detail/typography.md:21) with (.andromeda/design-system.md:28).
- **Bundled fonts ↔ web-spa canvas**: the list on the wasm32 canvas surface reads only the fonts its context bundles, (.andromeda/design-system-detail/typography.md:28) and (.andromeda/design-system-detail/typography.md:30) with (.andromeda/design-system.md:219).
- **List scrollbar ↔ motion**: the thumb moves over a nested scrolling box that an into-view scroll to the keyed row writes at once, whatever behaviour was asked, and the driver's `scroll` moves instantly, (.andromeda/design-system-detail/spacing.md:29) with (.andromeda/design-system.md:121).
- **List scrollbar ↔ depth**: a list that clips its own rows stops hits at its padding box under the rule that binds paint clips, so the thumb and the rows must answer inside that box, (.andromeda/design-system-detail/spacing.md:29) with (.andromeda/design-system.md:85).
