# design extract

## Relevance
partial — the chunk adds no styled UI and no tokens. It rasterises screens the stand already paints, so what applies is the font loading, the canvas background, the scheme and scale defaults, the motion that can change pixels, and the agent-facing output of the CLI surface.

## Constraints
- Fonts on the headless stand: per design-system §Typography (Loading), the stand registers the bundled DejaVu Sans for every generic through `build_single_font_ctx`, with system fonts off, on native, behind seven_guis' `woff` feature. A screenshot that claims determinism has to rasterise with that context only. P3 research must check whether the driver's raster path keeps that context or picks up a host font.
- Vacuous font checks: per design-system §Typography (Loading), without the `system-fonts` feature text measures 0x0 and font-dependent assertions pass vacuously. The "fails, never skips, without fonts" mandate therefore needs a positive font-present reading. An image with no glyphs must not count as a pass.
- Canvas background: per design-system §Color Palette (color model and scheme), the canvas background is the root element's background colour, or body's when the root is transparent. The frame clear colour `base_color` is unset by default, and only WPT-rendered pages get an explicit white fill. P3 research must check whether the CPU raster path fills the stand's page background (`#f5f5f5`, per design-system §Color Palette, Surface Scale) or leaves transparent pixels.
- Colour scheme: per design-system §Color Palette (color model and scheme), the system colour scheme defaults to `Light` and feeds `prefers-color-scheme`. Engine-painted scrollbar thumbs change palette with that scheme (per design-system §Color Palette, Engine-painted defaults). For pixel identity, the scheme has to be pinned along with the viewport.
- Scale: per design-system §Surface: desktop-native (Tokens), the default viewport has hidpi scale 1.0 and zoom 1.0. Logical size is the physical size divided by hidpi × zoom. The PNG's pixel dimensions follow from the stand's pinned viewport and scale, and the verb's result shape has to state that relation.
- Overlay scrollbars: per design-system §Motion (This project's values), they show at full opacity on scroll, stay opaque for `FADE_DELAY` (500 ms), then fade over `FADE_DURATION` (200 ms). A screenshot taken after a `scroll` paints a thumb whose opacity depends on time.
- Animation clock: per design-system §Motion (Animation runtime), the harness drives animation from a controlled clock that settle reads and never advances. The driver's `advance` leaves that clock where it was. P3 research must check whether every repeat of the same stand state reads the same clock value at paint time.

## Patterns to follow
- The agent-facing CLI answer, per design-system §Surface: cli (Component Patterns). `escher-session` prints one uncoloured line of JSON on stdout, keyed by the schema's own words, with no ESC byte. It has no terminal test, so its output is the same piped or not. Exit statuses come from `EXIT_STATUSES`. The `mcp` role's replies are protocol lines only. A `screenshot` answer on both surfaces takes this shape.
- The bundled single-font context, per design-system §Typography (Loading) and §Surface: web-spa (Tokens). `build_single_font_ctx` registers one font as the fallback for SansSerif, Serif, Monospace and SystemUi with system fonts disabled. The stand and the WASM builds use it, and it is the pattern for a font-pinned raster.
- An existing raster path to reuse: the CPU `screenshot` example, listed under design-system §Surface: cli (Component Patterns) as a CLI binary that rasterises a page and writes a file.

## Anti-patterns to avoid
- Printing person-facing progress on the driver's stdout. Per design-system §Surface: cli (Component Patterns), the `screenshot` example prints the URL, per-phase timing lines, `Screenshot is ({w}x{h})` and `Written to {path}`. `escher-session` must print nothing on stdout but its one answer line.
- Colour or terminal detection in the answer. Per design-system §Surface: cli (Component Patterns), the driver uses no colour dependency, writes no ESC byte and never tests whether a stream is a terminal. That excludes the WPT runner's `owo-colors` and hyperlink pattern from the same section.

## Contract bindings
- design ↔ tests: the bundled font (§Typography, Loading) binds to the test plan's font fail-not-skip check and to pixel identity across runs. Without the font, the check has to fail, not pass on an empty raster.
- design ↔ security: the canvas carries painted text, and design-system has no masking rule for a password field's painted characters. The CARRY on painting a password in the clear belongs to the security plan, and the pixels are the carrier.
- design ↔ a11y: the stand's painted colours are a11y surface (SC 1.4.3). The screenshot reproduces them and does not change them. No contrast check is added here.

## Acceptance criteria contributions
- A screenshot of a stand screen is rasterised with the bundled DejaVu Sans context only, with system fonts off. With that font unavailable, the call or check fails with a typed reason and never yields a glyphless image counted as a pass (per design-system §Typography, Loading).
- Two screenshots of the same stand state from separate runs are pixel-identical, including one taken after `scroll`. The overlay scrollbar's fade state and the colour scheme are fixed by the harness clock and the pinned `Light` scheme, not by wall time (per design-system §Motion, This project's values).
- The image's background pixels are the page background the canvas rule resolves (the stand's `#f5f5f5`), not unset or transparent pixels (per design-system §Color Palette, color model and scheme).
- `escher-session screenshot` writes exactly one uncoloured JSON line on stdout with no ESC byte, and exits with a status from `EXIT_STATUSES`. Over MCP, the reply is protocol lines only (per design-system §Surface: cli, Component Patterns).
