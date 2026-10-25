# design extract

## Relevance
partial — the chunk's one painted change is the stand: Flight booker's one-way / return choice possibly made a `select`, which retires an accent button, and `cells` served to the driver. The pipe, the live run and the bar carry no design surface beyond the agent-read CLI/MCP output.

## Constraints
- design-system §Color Palette › Core Colors requires the seven_guis accent `#3a5ce5` (hover `#2a4cd3`, active `#1f3fc1`) in every task CSS that declares it, with white button text on accent controls at ≥4.5 at rest, on hover and when active. `flight-one-way` is one of the five measured accent controls (detail: .andromeda/design-system-detail/color-palette.md, Core Colors row). Turning that choice into a `select` removes or changes a measured accent pair. Any accent the new control keeps must use these three values and no other shade.
- design-system §Color Palette › Text Hierarchy and § Surface Scale give the stand's text as `#1a1a1a` on page `#f5f5f5`. A `select` or Cells text added to a stand screen takes those declared values. The plan's project-wide colour token set is NOT YET MEASURED (design-system §Color Palette, Example fixtures), so the declared seven_guis values are the only colours to use, not tokens.
- design-system §Color Palette › Semantic Colors gives Flight booker's invalid state as `#e53e3e` / `#fff5f5` / `#c53030` and its success message as `#ebf8ee` / `#68d391` / `#276749`. A reworked Flight booker keeps these values for its date-field invalid state and its booked message. It must not reuse them for the new select.
- design-system §Surface: desktop-native › Tokens (detail: .andromeda/design-system-detail/surface.md) says a single-choice `select` gets its look from the engine's user-agent defaults: `1px solid #999` border, `2px` padding, `white` background, `display: inline-block`, chosen option shown alone. It holds no design token. Whether the stand styles the select itself or leaves it on these defaults is P3 research's question and possibly a P4 shaping point. Either way, its colours are whatever is declared.
- design-system §Typography requires the stand's bundled DejaVu Sans for every generic, with system fonts off (`build_single_font_ctx` through `HarnessOptions.font_ctx`), and seven_guis' base 14px sans-serif. Form controls, `select` included, default to `system-ui, sans-serif`, which resolves to that one font. A `cells` boot through the stand must take the same font context. Otherwise `screenshot` and `lint` refuse `fonts-unavailable` (design-system §Typography, Loading).
- design-system §Motion requires the driver's load wait to move no clock: the `VirtualClock` stays still for up to `LOAD_WAIT`. Only `advance` moves time, by exactly `ms`, through the session's step. A load in flight added to the stand must not change this. While a step waits, a Timer or animation on the same boot must not run.
- design-system §Surface: cli › Component Patterns requires `escher-session` and its `mcp` role to print JSON lines only on stdout. That means no colour, no ESC byte and no terminal test. The cold-agent pipe's new goals and its replay read this surface, and they must not depend on anything outside it.

## Patterns to follow
- The stand's untokenized geometry (design-system §Border Radius and §Spacing, detail rows "seven_guis (untokenized)"): inputs 4px radius, buttons 6px, cards 8px, card padding 24px 32px with a 16px gap. A styled `select` in Flight booker matches its sibling inputs (4px), not the buttons.
- The 2026-10-14 accent re-colour (design-system §Color Palette › Core Colors): each colour change is measured as a declared-equals-measured row in `stand_contrast`, `TEXT_ROWS` / `STATE_ROWS`, hover and active included. A changed control moves its own rows. Rows are never dropped quietly.
- The stand's headless boot, `examples/seven_guis/src/stand.rs` (design-system §Typography): pinned viewport, bundled font, offline. A newly served task such as `cells` boots through the same path.

## Anti-patterns to avoid
- Do not bring back the old accent `#4a6cf7` / `#3a5ce5`-as-hover set, or any accent shade under 4.5 against white (design-system §Color Palette › Core Colors).
- Do not boot a stand task with system fonts on, or with a font context that does not resolve `sans-serif`. Text would measure 0x0, or shots and lint would be refused (design-system §Typography).
- Do not add colour, an ESC byte or a terminal check to the driver's stdout for the pipe's benefit (design-system §Surface: cli › Component Patterns).

## Contract bindings
- Core Colors ↔ a11y §Contrast (SC 1.4.3): `stand_contrast` asserts every stand control's text/background pair. Replacing `flight-one-way` with a `select` moves its rows, and any Cells text served on the stand comes into the same check's reach.
- Semantic Colors ↔ a11y not-colour-alone (SC 1.4.1): Flight booker's invalid state is stated by `aria-invalid` as well as its red. That pairing has to survive the rework.
- Motion (load wait moves no clock) ↔ tests: `stand_load_wait` / `stand_settle`. A stand-side load in flight is checked against the same no-time-moves reading.
- Surface: cli ↔ tests: the seven_guis checks' shared reader and the MCP `Speaker` assert no ESC byte on every answer line. `test_cold_agent.py` reads the same lines.

## Acceptance criteria contributions
- If `flight-one-way` stops being an accent button, its accent row in `stand_contrast` is re-pinned or explicitly retired. Every text/background pair the new `select` adds is measured, equals its declared colour, and meets 4.5 (per design-system §Color Palette › Core Colors).
- The reworked Flight booker still paints its invalid and success states in the declared `#e53e3e`/`#fff5f5`/`#c53030` and `#ebf8ee`/`#68d391`/`#276749` values (per design-system §Color Palette › Semantic Colors).
- `cells`, booted through the stand for the driver, is not refused `fonts-unavailable` by `screenshot` or `lint` on the bundled font (per design-system §Typography).
- Every `escher-session` and `escher-session mcp` stdout line in the pipe's run and replay stays uncoloured with no ESC byte (per design-system §Surface: cli › Component Patterns).
