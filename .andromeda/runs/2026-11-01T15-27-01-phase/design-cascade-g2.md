# design · cascade read · group 2 of 6

## Carried
- (.andromeda/design-system-detail/typography.md:15) `default.css` is a predicted conflict surface, so its resolution must keep the form controls' default families as recorded, or any moved value must be re-measured and recorded.
  > | blitz-dom default form controls | inputs/selects/buttons `system-ui, sans-serif`; textarea monospace | not stated | not stated | not stated | (packages/blitz-dom/assets/default.css:88-96; packages/blitz-dom/assets/default.css:350-396; packages/blitz-dom/assets/default.css:407-415; packages/blitz-dom/assets/default.css:711-716) |
- (.andromeda/design-system-detail/typography.md:16) `default.css` is a predicted conflict surface, so its resolution must keep the headings' default weight and sizes as recorded, or any moved value must be re-measured and recorded.
  > | blitz-dom default headings | not stated | bold | h1–h6 from 2em down to 0.67em | not stated | (packages/blitz-dom/assets/default.css:88-96; packages/blitz-dom/assets/default.css:350-396; packages/blitz-dom/assets/default.css:407-415; packages/blitz-dom/assets/default.css:711-716) |
- (.andromeda/design-system-detail/typography.md:17) `default.css` is a predicted conflict surface and `7a3dd8918` moves Stylo preferences ahead of the UA sheet, so `code` and `pre` must still resolve to the recorded `-moz-fixed` default, or any moved value must be re-measured and recorded.
  > | blitz-dom default code/pre | `-moz-fixed` | not stated | not stated | not stated | (packages/blitz-dom/assets/default.css:88-96; packages/blitz-dom/assets/default.css:350-396; packages/blitz-dom/assets/default.css:407-415; packages/blitz-dom/assets/default.css:711-716) |
- (.andromeda/design-system-detail/typography.md:18) This rule lives in the conflicted `default.css` and belongs to the select model the resolution must keep unchanged, so the rows of a `select[multiple][size]` must keep this line height.
  > | blitz-dom default `select[multiple][size]` rows | not stated | not stated | line-height 1.2 | not stated | (packages/blitz-dom/assets/default.css:183-193) |
- (.andromeda/design-system-detail/typography.md:22) Every text number a take re-measures reads this base size as an input, so a moved bound, byte size or pixel must be explained against it and recorded, never loosened to pass.
  > | Engine generic base size | generics | not stated | monospace 13px; other generics 16px | not stated | (packages/blitz-dom/src/font_metrics.rs:167-176) |
- (.andromeda/design-system-detail/typography.md:30) The stand checks re-run in both layout modes are measured at this stand's font, so its registration must be read from the architecture contract named here and must not be restated as a typography value by any record of the take.
  > - The seven_guis headless stand's font registration is held in architecture §Standard Contracts → Headless stand, beside `font_ctx()`, not as a design-system typography value
- (.andromeda/design-system-detail/typography.md:32) The font-embolden run the chunk adds to a local gate must exercise the feature recorded here, and a take that moves manifests must keep this feature set or record what moved.
  > - Font features: system-fonts, woff, complex-scripts (dictionary line-breaking), font-embolden and apple-font-embolden (packages/dioxus-native/Cargo.toml:20-25; packages/dioxus-native/Cargo.toml:47-48)
- (.andromeda/design-system-detail/typography.md:35) `runtime/bootstrap.rs` is a predicted add/add conflict over the module this stub sits under, so a resolution that keeps both sides' intent, `91ef65e21`'s script API included, must leave `document.fonts` reporting all fonts as loaded.
  > - `document.fonts` is a stub FontFaceSet where all fonts report as loaded (packages/blitz-vibey-script/src/runtime/bootstrap/fonts.js:1-15)
- (.andromeda/design-system-detail/typography.md:42) The `None` mapping gives the family that text falls to in the numbers the chunk re-measures, so a take that moves Stylo must keep this mapping or explain and record each number it moves.
  > - Generic font families map to Parley generics, with `None` as sans-serif (packages/blitz-dom/src/stylo_to_parley.rs:50-60)
- (.andromeda/design-system-detail/typography.md:43) These conversions are inputs of the text numbers the chunk re-measures, so a take that moves Stylo must keep them converting to Parley values and must explain and record each number that moves.
  > - Font weight, width, style and variation settings convert to Parley values (packages/blitz-dom/src/stylo_to_parley.rs:87-112)

## Bindings
- **Typography's `select[multiple][size]` rows ↔ Color Palette's engine-painted select list**: the `default.css` resolution must keep the rows' line height in the sheet while the open list's colours stay engine constants outside it, so the select model holds on both sides, (.andromeda/design-system-detail/typography.md:18) and (.andromeda/design-system.md:30)
- **Typography's stand font ↔ the architecture plan's Headless stand contract**: a re-measured stand number reads its font from the architecture contract while its recorded text defaults stay in this plan, so neither plan restates the other's value, (.andromeda/design-system-detail/typography.md:30)
