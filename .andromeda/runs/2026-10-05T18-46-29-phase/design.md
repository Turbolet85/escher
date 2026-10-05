# design extract

## Relevance
partial — the chunk renders no surface and adds no token, but whether its blitz-tests reading is meaningful depends on the host's font setup, and several pixel and timing tests check the engine's as-built painted defaults.

## Constraints
- Font-dependent tests need system fonts: design-system §Typography (Loading) says blitz-tests builds blitz-dom with the `system-fonts` feature. Without it, text measures zero and font-dependent assertions pass without testing anything. A green run counts only if the feature was active and the host's fonts were found. Whether this host's run actually loaded fonts is a question for research.
- The host's font set belongs in the reproducibility record: design-system §Typography (Engine generic base size, Engine text mapping) maps the CSS generic families to whatever fonts the host provides, so the installed fonts and their fontconfig resolution are host facts to record next to the wall-clock. On Arch the package names differ from the `libfontconfig1-dev` that CLAUDE.md names.
- Painted-colour tests measure as-built values. Per design-system §Color Palette (Engine-painted defaults), the scrollbar palette follows the viewport colour scheme, and per §Surface: desktop-native (Tokens) that scheme defaults to Light. A red in a colour or pixel test is recorded as the engine's reading, not treated as a palette defect to fix here.
- Pixel expectations come from a reference engine: design-system §Depth Strategy (Engine depth behavior) says the outset-shadow shape expectations come from Chromium. A red in a shadow or paint-order test is recorded with its file name for P4 to judge whether the cause is host-local.
- Motion timing should not depend on host speed: design-system §Motion (Animation runtime) says the test harness drives animation from a controlled clock advanced by tick. Smooth-scroll and scrollbar-fade tests should therefore not depend on wall-clock speed. If any of them goes red, research should check whether it really uses the harness clock before blaming host load.

## Patterns to follow
- Use the font-registration paths in design-system §Typography (Loading) only to explain a font-related red. The chunk does not add a custom `FontContext` or bundled fonts.
- Read each colour or pixel red against the values in design-system §Color Palette (Engine-painted defaults, Border Progression) and §Border Radius (engine rows), then record it unchanged as the baseline.
- Treat the colour scheme as part of the test environment. Per design-system §Color Palette, scheme changes trigger a recascade and `prefers-color-scheme` comes from the viewport, so record any scheme-dependent red together with the scheme it ran under.

## Anti-patterns to avoid
- Do not edit the user-agent stylesheet, the engine-painted colours or the font defaults to turn a red green. The values in design-system §Color Palette and §Typography are what this baseline measures, and the scope says no product change.
- Do not report a green blitz-tests run without checking that font-dependent tests really ran with fonts (design-system §Typography, Loading).

## Contract bindings
- design ↔ tests: the `system-fonts` feature on blitz-dom in the blitz-tests crate (design-system §Typography, Loading) is what makes font-dependent integration assertions meaningful. The tests extractor owns the run itself.
- design ↔ a11y: painted colours and focus outlines are a11y surface (CLAUDE.md Critical Warnings, SC 1.4.3). The baseline records them as-is, and contrast checks of those colours belong to a11y, not this chunk.

## Acceptance criteria contributions
- (design) The baseline record names the host's font provisioning (the fontconfig package and the installed fonts that resolve sans-serif and monospace) and confirms blitz-tests ran with blitz-dom `system-fonts` enabled (per design-system §Typography)
- (design) The chunk's diff touches no user-agent stylesheet, engine paint colour or font default. Any colour or pixel red is recorded as the as-built reading (per design-system §Color Palette)
- (design) Any red in a timing or animation test (smooth scroll, scrollbar fade) is recorded with a note on whether it runs on the harness's controlled clock (per design-system §Motion)
