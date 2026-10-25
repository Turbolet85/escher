# design · Typography — category read

## Carried
- (.andromeda/design-system-detail/typography.md:18) Every screen the extended stand adds or changes (the flight booker's select, `cells` served to the driver, a task holding a load in flight) must render in the stand's own type: sans-serif, base 14px, with the home title alone at 700 / 36px.
  > | seven_guis | sans-serif | home title 700 | base 14px; home title 36px | not stated | (examples/seven_guis/src/app.rs:179-205) |
- (.andromeda/design-system-detail/typography.md:30) Every task `start` boots after this chunk, the newly served ones included, must go through the same headless boot that registers bundled DejaVu Sans for every generic with system fonts off, so its shots, lint and contrast readings do not depend on the host's fonts.
  > The seven_guis headless stand registers the same bundled DejaVu Sans for every generic, with system fonts off, on native — `build_single_font_ctx(DEJAVU_SANS)` through `HarnessOptions.font_ctx`, decoded by seven_guis' native `woff` feature
- (.andromeda/design-system-detail/typography.md:15) The flight booker's new `select` takes its font from the engine's default form-control rule unless the stand's CSS sets one, so on the stand it must resolve to the bundled font registered for `system-ui` and `sans-serif`.
  > | blitz-dom default form controls | inputs/selects/buttons `system-ui, sans-serif`; textarea monospace |
- (.andromeda/design-system-detail/typography.md:34) A `screenshot` or `lint` the cold agent calls on any newly served stand screen must pass the font check: the boot's font context must resolve a `sans-serif` family, or the call is refused `fonts-unavailable`.
  > Since 2026-10-12-headless-screenshot a driver `screenshot` of an app whose font context resolves no `sans-serif` family (`BaseDocument::can_paint_text` false) is refused `fonts-unavailable` with nothing painted — a shot never paints text skipped; the stand's bundled font passes the check
- (.andromeda/design-system.md:46) This is the body's loading rule for the shots the extended stand and the cold-agent run take.
  > A driver `screenshot` of an app whose font context resolves no `sans-serif` family is refused `fonts-unavailable` with nothing painted — a shot never paints text skipped.
- (.andromeda/design-system-detail/typography.md:36) The stand checks this chunk re-pins (the select's contrast rows, lint on `cells`, the snapshot bounds, parts and diffs) are font-dependent, so they must run in blitz-tests with `system-fonts` on, or they pass vacuously.
  > `blitz-dom` is built with the `system-fonts` feature for the blitz-tests crate; without it text measures 0x0 and font-dependent assertions pass vacuously
- (.andromeda/design-system.md:48) This is the body's statement of the same guard for the re-pinned stand checks.
  > `blitz-dom` is built with the `system-fonts` feature for the blitz-tests crate: without it text measures 0x0 and font-dependent assertions pass vacuously.

## Bindings
- **seven_guis base 14px ↔ a11y-plan contrast (SC 1.4.3)**: the stand's text is 14px and not large, so `TextContrast::required_ratio` must apply 4.5 to the select's new `stand_contrast` rows, and to any other text a new stand screen adds (.andromeda/design-system-detail/typography.md:18)
- **seven_guis type ↔ Color Palette accent**: the select and any new stand control must keep the body's accent (`#3a5ce5`, white text passing 4.5) wherever it carries text in the stand's 14px type (.andromeda/design-system-detail/typography.md:18)
- **bundled DejaVu Sans boot ↔ test-plan stand checks and the cold-agent replay**: a shot or lint answer is deterministic only when every booted task goes through the stand's one font context. This covers `stand-repeat` and the replay of the agent-written check on fresh `escher-session mcp` children (.andromeda/design-system-detail/typography.md:30)
- **`fonts-unavailable` ↔ escher-driver schema**: the chunk's boundary allows no new cause, so a new stand boot that fails the font check would show up as the existing eighth cause and never as a new one (.andromeda/design-system.md:46)
