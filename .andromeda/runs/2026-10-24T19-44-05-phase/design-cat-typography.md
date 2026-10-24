# design · Typography — category read

## Carried
- (.andromeda/design-system-detail/typography.md:18) The re-read `stand_contrast` must take each stand row's size and weight from these values: 14px base text is held to the normal-text ratio, and only the 36px weight-700 home title can qualify for the large-text ratio. A row must not be moved to the large-text ratio unless its computed size and weight actually changed.
  > | seven_guis | sans-serif | home title 700 | base 14px; home title 36px | not stated | (examples/seven_guis/src/app.rs:179-205) |
- (.andromeda/design-system-detail/typography.md:30) The Tab-order, Enter/Space and contrast re-reads must boot the stand through this font setup: bundled DejaVu Sans for every generic, with system fonts off. The stand's text, and the contrast reading over that text, must not depend on the host's fonts.
  > The seven_guis headless stand registers the same bundled DejaVu Sans for every generic, with system fonts off, on native — `build_single_font_ctx(DEJAVU_SANS)` through `HarnessOptions.font_ctx`, decoded by seven_guis' native `woff` feature (examples/seven_guis/src/stand.rs:52-54; examples/seven_guis/src/lib.rs:7; examples/seven_guis/Cargo.toml:31)
- (.andromeda/design-system-detail/typography.md:36) Every blitz-tests target the chunk moves into the `a11y` leg must run with this feature on. Without it, a text-dependent a11y assertion (a contrast row, a label's text, a hit on text) passes without testing anything, and the widened leg must not give a green that tests nothing.
  > `blitz-dom` is built with the `system-fonts` feature for the blitz-tests crate; without it text measures 0x0 and font-dependent assertions pass vacuously, and the feature is stated to be enabled by default when testing the whole workspace (tests/blitz-tests/Cargo.toml:17; tests/blitz-tests/tests/br_trailing_line.rs:11-13; tests/blitz-tests/tests/inline_box_baseline.rs:4-6)
- (.andromeda/design-system.md:48) The body states the same guard broadly, and it applies to the leg's new checks.
  > `blitz-dom` is built with the `system-fonts` feature for the blitz-tests crate: without it text measures 0x0 and font-dependent assertions pass vacuously.

## Bindings
- **seven_guis type sizes ↔ a11y-plan §Contrast (SC 1.4.3)**: `stand_contrast` picks each row's required ratio (`TextContrast::required_ratio`, 3.0 for large text, else 4.5) from the row's computed font size and weight. So the stand's 14px base and its 36px weight-700 title (.andromeda/design-system-detail/typography.md:18) decide which threshold each re-asserted row must meet.
- **seven_guis type sizes ↔ Color Palette (accent)**: white button text on the accent at the 14px base (.andromeda/design-system-detail/typography.md:18) is normal text. It must meet 4.5 at rest, on hover and when active, as the accent line requires (.andromeda/design-system.md:16).
- **bundled stand font ↔ test-plan (the `a11y` leg)**: the stand's single-font context (.andromeda/design-system-detail/typography.md:30) and the blitz-tests `system-fonts` feature (.andromeda/design-system-detail/typography.md:36) are what make the widened leg's text-dependent checks actually test something.
