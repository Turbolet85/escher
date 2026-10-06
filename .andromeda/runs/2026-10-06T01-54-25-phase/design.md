# design extract

## Relevance
partial — the chunk adds no rendered surface and changes no stand markup, but its pinned font, colour scheme and viewport determine how the stand's existing tokens resolve headlessly, and the timer check depends on the animation clock.

## Constraints
- Bundled-font parity: design-system §Typography (Loading) records the stand's bundled DejaVu Sans registered for sans-serif, serif, monospace and system-ui on WASM, and `build_single_font_ctx` as the one-font fallback for all four generics with system fonts disabled. The headless boot's font context must cover the same generics, so the stand's `sans-serif` resolves to the bundled face (design-system §Typography table, seven_guis row: sans-serif, base 14px, home title 36px/700). Whether the harness can take a font context today is research's question.
- Colour scheme is part of the pin: design-system §Color Palette (engine color model and scheme) records the system scheme as `Light` (default) / `Dark`, set on the viewport and feeding `prefers-color-scheme`; a scheme change forces a full recascade. The stand's pinned viewport must name its scheme explicitly, not inherit an embedder default.
- Pinned viewport geometry: design-system §Surface: desktop-native (Tokens) records the default viewport as Light, hidpi 1.0, zoom 1.0, with logical size equal to physical size ÷ (hidpi × zoom). The single named viewport must fix size, scale and zoom together so logical size, and with it layout, holds across runs.
- Windowed and wasm tokens stay as they are: design-system §Color Palette (Core Colors, Text Hierarchy, Semantic Colors seven_guis rows), §Border Radius (seven_guis row) and §Depth Strategy (observed seven_guis card shadows) record the stand's as-built accent, page, text, invalid/success, radius and shadow values. The headless entry must render the same TaskShell tree, so it must not fork or restyle these values.
- Every document carries the default user-agent sheet: design-system §Surface: desktop-native (Tokens) records that every Dioxus document gets blitz `DEFAULT_CSS`. The headless DioxusDocument boot must keep that so headless computed styles match the windowed app.
- Animation time is harness-driven: design-system §Motion (Animation runtime) records that the test harness drives animation from a controlled clock advanced by tick, while the shell's clock is wall-clock seconds. Any time seam for the timer task should line up with the harness's controlled clock instead of adding a second clock. Whether the timer's `futures_timer::Delay` can be driven from that clock is research's question.

## Patterns to follow
- One-font context: `build_single_font_ctx`, as referenced in design-system §Typography (Loading), registers one face as fallback for every generic with system fonts off. It is the existing pattern for a bundled-font headless context.
- The wasm stand's font registration (design-system §Surface: web-spa, Tokens: bundled DejaVu Sans for all four generics) is the stand's existing precedent for "bundled fonts, not host fonts". The headless boot follows the same font choice.
- The harness's controlled-clock tick (design-system §Motion, Animation runtime) is the existing pattern for deterministic time.

## Anti-patterns to avoid
- Vacuous font-dependent assertions: design-system §Typography (Loading) records that without the `system-fonts` feature text measures 0x0 and font-dependent assertions pass vacuously. A bundled-font boot that silently fails to register its face would reproduce this. Proof checks must not assert layout without first confirming that text measures non-zero.
- Host-font leakage: with `system-fonts` enabled in the harness (design-system §Typography, Loading), a font context that only adds the bundled face, without disabling system fonts, lets host fonts win the generic lookup. That breaks dev-host/CI parity.

## Contract bindings
- design ↔ a11y: TaskShell's back button and the tasks' controls paint the stand's focus and state colours (design-system §Color Palette, seven_guis rows). They are focus and contrast surface (a11y SC 2.4.3, 1.4.3), but the contrast and keyboard harnesses are out of scope (Epoch 6). The binding here is only that this chunk must not change those values.
- design ↔ tests: the pinned viewport, scheme and bundled font are the deterministic-layout inputs for the test-plan harness (design-system §Surface: desktop-native Tokens; §Typography Loading). They should be named once and shared, as the scope requires.
- design ↔ arch: a bundled-font option on `HarnessOptions` would be a public harness-API addition (arch §Standard Contracts). The font asset it reaches for is the stand's existing DejaVu Sans (design-system §Surface: web-spa).

## Acceptance criteria contributions
- (design) Under the headless boot, text in the stand (e.g. the TaskShell title) shapes with the bundled DejaVu Sans face and measures non-zero width and height. The check fails if text measures 0x0 or resolves to a host face (per design-system §Typography, Loading).
- (design) The shared stand viewport names its size, hidpi scale, zoom and colour scheme explicitly. Two boots of the same task yield identical layout boxes for TaskShell's header and title (per design-system §Surface: desktop-native, Tokens; §Color Palette, engine color model and scheme).
- (design) The headless and windowed entries render the same TaskShell tree with the stand's existing style values unchanged. No seven_guis colour, radius or shadow value is edited or duplicated for the headless path (per design-system §Color Palette, seven_guis rows; §Border Radius, seven_guis row).
- (design) The timer check advances elapsed state through the harness's controlled clock, with no real-time sleep (per design-system §Motion, Animation runtime).
