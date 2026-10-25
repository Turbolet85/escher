# design · Color Palette — category read

## Carried
- (.andromeda/design-system.md:16) The statement's a11y reach, where it names the stand's contrast, must state the accent and its two states as the body holds them and claim the SC 1.4.3 pass for white button text at rest, on hover and when active, no wider.
  > seven_guis accent — `#3a5ce5` (hover `#2a4cd3`, active `#1f3fc1`) in every task CSS that declares it, its non-text uses moving with it, so white button text on the accent controls passes SC 1.4.3's 4.5 at rest, on hover and when active.
- (.andromeda/design-system-detail/color-palette.md:27) Any contrast figure the statement quotes must match the measured ratios and the five measured controls exactly (5.47 at rest, 6.83 hover, 8.27 active, against 4.5), and name `stand_contrast` as their witness rather than a new measurement.
  > measured as the background of white `#ffffff` button text on the five accent controls (`counter-increment`, `flight-one-way`, `flight-book`, `timer-reset`, `crud-create`, 14–16 px, weight 500–600, at boot): 5.47 against SC 1.4.3's 4.5, a pass — the old `#4a6cf7` read 4.39, a fail — and white on the hover shade 6.83 and on the active shade 8.27, each a reached colour state of `stand_contrast`; every measured stand colour equals its declared value
- (.andromeda/design-system.md:36) The statement must not claim a project-wide colour token set for escher: none is measured, so a consumer is told the palette is per-app declared values, not tokens.
  > NOT YET MEASURED — a project-wide UI colour token set: the examples slice (s01) and the DOM-node slice (s08) recorded the project's own colour tokens as out of slice, and the default user-agent stylesheet's full contents were out of slice for s05.

## Bindings
- **seven_guis accent contrast ↔ a11y-plan (contrast, SC 1.4.3) and the scope's «Its a11y reach»**: the statement's contrast claim is closed at P3 against a11y-plan's `stand_contrast` rows; the palette side fixes the values and ratios it must quote (.andromeda/design-system-detail/color-palette.md:27), (.andromeda/design-system.md:16)
- **Absent token set ↔ the statement's stated limits**: a consumer-facing capability claim about colour stays inside what is measured (.andromeda/design-system.md:36)
