# design extract

## Relevance
partial — the chunk is a proof sweep, mostly test, schema and engine-fork work. Design applies where it touches stand markup or CSS (C2 keys, C8 Cells' selected cell, C12 Circle Drawer's dialog rule), fixtures that paint or measure text (C11, any screenshot), the overlay-scrollbar fade's timing (C10), the hit order (C4/C5), and the agent-read CLI/MCP output (C9, C16).

## Constraints
- design-system §Color Palette (Core Colors row "seven_guis accent"; Text Hierarchy row "seven_guis text"; Surface Scale row "seven_guis / todomvc page") requires every stand colour to stay at its recorded value: the accent `#3a5ce5` (hover `#2a4cd3`, active `#1f3fc1`), the page `#f5f5f5` and the text `#1a1a1a`. The same row records that every measured stand colour equals its declared value. Keying the three non-lean tasks (C2) and stating Cells' selected cell (C8) change markup only, unless P4 also decides to change a colour.
- design-system §Color Palette (Semantic Colors) records the stand's only state colours, both in flight_booker: invalid (`#e53e3e` / `#fff5f5` / `#c53030`) and success (`#ebf8ee` / `#68d391` / `#276749`). The section records no selected-cell colour for Cells. Whether Cells already paints its selected cell differently, and with which colour, is research's question (C8).
- design-system §Typography (Loading) requires the headless stand to register the bundled DejaVu Sans for every generic family, with system fonts off. The same section says the blitz-tests crate builds blitz-dom with `system-fonts`, and that without it text measures 0x0 and font-dependent assertions pass vacuously. A fixture whose reading depends on glyph widths (C11's click-placed caret in a password field) has to boot with a real font context.
- design-system §Typography (Loading) also says a driver `screenshot` of an app whose font context resolves no `sans-serif` family is refused `fonts-unavailable` with nothing painted. A sweep fixture that is shot needs a font context that resolves `sans-serif`.
- design-system §Motion (This project's values) records the overlay-scrollbar timings: full opacity on scroll, `FADE_DELAY` 500 ms, then a linear `FADE_DURATION` of 200 ms, so a 700 ms window (C10). The same section says `scroll_into_view` writes nested boxes at once and the driver's `scroll` is instant in every box. Whether the fade reads the wall clock or the harness clock is research's question (C10).
- design-system §Motion (Animation runtime) requires settle to read animation time only through the harness's controlled clock and never advance it. A driver `click`, `type` or `press` beside a running CSS animation returns settled without waiting on it. Any sweep check that takes a reading during a transition or a fade has to step time explicitly.
- design-system §Depth Strategy (Engine depth behavior) records the hit order: positive-z hoisted children, then paint children in reverse, then negative-z hoisted children. It also records the hit stop at the padding box of an `overflow`-clipping node, the root element excepted. That reading is the baseline that C4's `contain: paint` question and C5's single-axis `overflow: clip` question compare against.

## Patterns to follow
- The stand's headless boot with its bundled font (`build_single_font_ctx(DEJAVU_SANS)` through `HarnessOptions.font_ctx`, in `examples/seven_guis/src/stand.rs`) as the font context for fixtures beside the stand (per design-system §Typography).
- `tests/blitz-tests/tests/stand_contrast.rs` as the colour record: every measured stand colour equals its declared value, and hover and active are each a reached colour state. Any stand colour this chunk touches is re-read against it (per design-system §Color Palette).
- The overlay-scrollbar checks `tests/blitz-tests/tests/scrollbars.rs` and `scrollbar_drag.rs` as the existing shape for fade, hover and drag readings, if C10 is measured (per design-system §Motion).
- The seven_guis checks' shared reader and the shared `Speaker` (`examples/seven_guis/tests/common/mod.rs`) assert no ESC byte on every answer line and reply. New CLI or MCP checks (C9, C16) go through them (per design-system §Surface: cli).

## Anti-patterns to avoid
- Writing ESC bytes, colour or a terminal test into `escher-session` output or into the `mcp` role's replies. Both are agent-read surfaces whose output has to be the same piped or not (per design-system §Surface: cli).
- Relying on a glyph-width or text-size assertion in a fixture with no real font context: it reads 0x0 and passes vacuously (per design-system §Typography).
- Changing a stand colour, even as a side effect of a Cells or Circle Drawer style edit, without re-measuring its record row (per design-system §Color Palette).

## Contract bindings
- Token contrast ↔ a11y-plan §Contrast (SC 1.4.3): any stand colour this chunk changes or adds (C8's selected-cell style, C12's dialog rule now applying) moves `stand_contrast`'s `TEXT_ROWS` / `STATE_ROWS`.
- State colour + label ↔ a11y-plan Use of Color (SC 1.4.1): if C8 is taken, Cells' selected cell must not be shown by colour alone. Its `aria-selected` reading is the non-colour cue in the snapshot.
- The overlay-scrollbar fade's timing ↔ test-plan's harness clock (C10): a shot taken inside the 700 ms window binds design-system §Motion to the harness's settle and screenshot contract.
- The CLI/MCP surface ↔ the seven_guis tests' shared reader and `Speaker` (the no-ESC reading on every line).

## Acceptance criteria contributions
- (design) Every stand task or fixture style the chunk touches declares only the stand values already on record: the accent with its hover and active, the page, the text, and the invalid and success colours. If it adds another, `stand_contrast` re-measures that colour as a row in the same chunk (per design-system §Color Palette).
- (design) Every new fixture check whose reading depends on painted or measured text (C11's caret, any `screenshot`) boots with a font context resolving `sans-serif`, the stand's bundled DejaVu Sans, and asserts a non-zero text extent (per design-system §Typography).
- (design) Every new `escher-session` answer line or `mcp` reply that the sweep's checks read holds no ESC byte (per design-system §Surface: cli).
- (design) If C10 is measured, its check shoots both inside and after the `FADE_DELAY` + `FADE_DURATION` window, 500 + 200 ms, and states which clock drove it (per design-system §Motion).
