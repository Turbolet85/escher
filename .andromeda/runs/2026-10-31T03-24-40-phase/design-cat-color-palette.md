# design · Color Palette — category read

## Carried
- (.andromeda/design-system-detail/color-palette.md:27) The served pages' contrast row on the load page's `load-fetch` Button, and any stand colour a red row moves, must take its values from layout-templates and must not add them to this palette as design-system values.
  > | seven_guis stand | — | The stand's accent (its load page's `load-fetch` included), its flight booker's success-message colours and its flight-type select's border are held in layout-templates §Surface: desktop-native → Primary screens (`values · desktop-native-screens`), not as design-system values |
- (.andromeda/design-system-detail/color-palette.md:28) A served-page control left to the engine's default stylesheet must be read against these default values in its contrast row, and an `inert` focus fixup or a key-handling fix must leave them unmoved in every document.
  > input focus outline `#4D90FE`; button background `#EFEFEF`
- (.andromeda/design-system-detail/color-palette.md:9) Each contrast row must state the colour scheme it reads under, the default being `Light` unless the row sets `Dark` on the viewport.
  > The system color scheme is an enum `Light` (default) / `Dark`, set on the viewport, and it feeds `prefers-color-scheme`
- (.andromeda/design-system-detail/color-palette.md:14) A contrast row on served-page text with no background of its own must pair the text against the canvas background this rule resolves.
  > The canvas background is the root element's background colour, falling back to body's when the root is transparent
- (.andromeda/design-system-detail/color-palette.md:70) A native-checkedness fix must leave the disabled checkbox and radio paint at this value, since the debt is the tree's stated `toggled` and not the control's colour.
  > | Disabled checkbox/radio | rgba 209, 209, 209, 255 | Disabled accent; otherwise the element's color stands in for accent-color per a TODO
- (.andromeda/design-system-detail/color-palette.md:104) A native-checkedness fix must leave the unchecked radio ring and checkbox tick colours as they are.
  > Unchecked radio rings use the CSS palette GRAY; checkbox ticks/gaps use white
- (.andromeda/design-system-detail/color-palette.md:119) A painted select popup, if the founder's fork takes it, must not cite an engine colour token, because the engine crates hold none; its colours are a new engine default the fork must state.
  > Observed absent — color tokens or CSS custom properties in the engine and integration crates
- (.andromeda/design-system.md:34) Design acceptance for this chunk must not require "uses only design tokens", because no project-wide colour token set is recorded for it to check against.
  > NOT YET MEASURED — a project-wide UI colour token set: the examples slice (s01) and the DOM-node slice (s08) recorded the project's own colour tokens as out of slice, and the default user-agent stylesheet's full contents were out of slice for s05.

## Bindings
- **The stand's `load-fetch` colours ↔ layout-templates §Surface: desktop-native → Primary screens**: the palette names the stand's accent and points away; layout-templates holds the literal the contrast row reads and any fork over a red row moves (.andromeda/design-system-detail/color-palette.md:27)
- **Engine default button background and focus outline ↔ a11y-plan §5 (SC 1.4.3, SC 2.4.3)**: the new served-page contrast rows read these literals where a control is default-styled, and the plan's OPEN gap closes against them (.andromeda/design-system-detail/color-palette.md:28)
- **Colour scheme ↔ a11y-plan contrast rows**: a contrast figure holds for the scheme the row names, so the row's assertion names `Light` or `Dark` (.andromeda/design-system-detail/color-palette.md:9)
- **Checkbox and radio paint ↔ a11y-plan and architecture's platform-adapter crossing**: checkedness is a state the tree carries and the paint colours are separate; a fix moves the first and holds the second (.andromeda/design-system-detail/color-palette.md:104)
- **No engine colour tokens ↔ Surface: desktop-native tokens and layout-templates**: a painted popup's colours have no palette row to point to, so a fork that takes the popup also brings a new literal for one of these files to hold (.andromeda/design-system-detail/color-palette.md:119)
- **No project-wide token set ↔ the plan's design acceptance criteria**: the token-only criterion is dropped for this chunk and literal values are cited instead (.andromeda/design-system.md:34)
