# Task — design-system · 2026-10-25-capability-statement

## The session
Added docs/capabilities.md, a 304-line statement of what escher 0.2.0 can do, in five sections: what it can do, stated limits, accessibility reach, what an MCP client carries onward, and where each claim is proven.
Added .github/scripts/test_capability_statement.py, a CI-script pin with four cases. It checks that every verb and every refusal cause is named in the statement, that the counts of verbs, causes, session errors, node keys and exit statuses are stated in words, and that README counts the MCP tools.
Replaced one line of README.md: the eleven driver commands as tools became the twelve, and the line now links docs/capabilities.md.
The session added no Rust symbol, verb, field, cause, session error, exit, carrier, log field, span, crate, dependency or config. The workspace test tally is unchanged; the ci-scripts leg moved from 100 tests in seven files to 104 tests in eight.
Re-read and unmoved, as restated in the statement: the CLI and MCP output form (one JSON line or JSON-RPC 2.0 lines on stdout, no colour or ESC byte, exit 0 · 1 · 2 · 3); the accent contrast (white on #3a5ce5 at 5.47, hover 6.83, active 8.27); and the stand's headless reach (800 × 600, scale 1.0, Light, DejaVu Sans, 0 lint findings at boot per lean task, and the last CRUD row clipped inside crud-list after twelve Creates).

## Reached files
- docs/capabilities.md
- .github/scripts/test_capability_statement.py
- README.md

## Touches
- The cli-output contract for escher-session and its mcp role (stdout JSON only, one line per command or newline-delimited JSON-RPC 2.0, no colour or ESC byte, no terminal test, exit 0 accepted · 1 refused · 2 usage · 3 session error, an mcp Err ending exiting 3). The statement re-read it and found it unmoved, so it is a re-read and not a change.
- The seven_guis accent #3a5ce5 (hover #2a4cd3, active #1f3fc1) and its SC 1.4.3 ratios of 5.47 · 6.83 · 8.27 for white text on the accent controls. The statement restates them and none of them moved.
- The fonts-unavailable refusal of a screenshot (and of a lint) for an app with no sans-serif family, nothing painted. The statement restates it and it did not move.
- The bundled stand font (DejaVu Sans), the pinned 800 × 600 viewport at scale 1.0 and the Light scheme. The statement restates them as the stand's reach and they did not move. A windowed end-to-end run is stated as unmeasured.
- A shot's PNG bytes: they repeat on one host and differ between linux x86_64 and macOS aarch64 (CI#39225375577). Now stated as a limit in the statement; the fact itself is unmoved.
- The driver's scroll and the into-view scroll, insofar as the statement's limits name the scroll or covered readings (a box clipped on one axis, a zero-size box hit at its centre, a scrolling box under a transform read untransformed). These are restated and unmoved.
- The hit at a box that paint clips (overflow, or contain: paint), which the statement's covered limits restate. Unmoved.
- Any count of driver verbs or MCP tools a cli-output or surface group holds. That count is twelve since 2026-10-19-visual-lint. README said eleven until this session, so a group that cites README's MCP sentence, or the number eleven for the tools, is reached through the README edit.
- Any citation of README.md:48 or of README's MCP sentence. That line was replaced in this session and now also links docs/capabilities.md.
- Any statement that docs/ holds only upstream's files. docs/ now also holds escher's own capabilities.md.
- Any count of the ci-scripts leg's tests or files that a standing group carries: now 104 tests in eight test files, with test_capability_statement.py's 4 added.
- The lint's six defects (overlap, clipped, truncated, off-screen, zero-size, contrast), which the statement restates. The stand's readings are unmoved: 0 findings at boot per lean task, and crud-person-14 clipped inside crud-list after twelve Creates.
- The VirtualClock and advance-driven animation time, and the load wait that moves no clock, only insofar as the statement's limits name animation or transition events as not queued by the Dioxus bridge. The motion rules themselves are unmoved.
- The WebAssembly surface: the statement says the wasm builds of the examples are browser-target builds through Trunk and that the driver does not run there. This is a re-read of the web-spa surface and nothing moved.

## New facts
- (none)

## Untouched
- The Color Palette's surface scale, text hierarchy, semantic colours, border progression and engine-painted defaults: no colour value, token or CSS file changed.
- Typography loading for blitz-tests (the system-fonts feature) and for WASM (build_single_font_ctx): no build feature or font context changed.
- Spacing values in the apps, examples and engine: no stylesheet or layout value changed.
- The Depth Strategy's observed values and the absence of an elevation scale: no shadow, z-order or stacking value changed.
- Border Radius values and the absence of a radius scale: nothing changed.
- Motion's reduced-motion absence and the harness settle's handling of the animating set: no harness or engine code changed.
- Iconography: the browser toolbar icons, IconButton, favicon sizes and the cursor mapping. No app or engine icon code changed.
- The desktop-native surface (the winit window, the titlebar, custom widget, devtools overlay, window config and stack): no shell or launch code changed.
- The mobile-native surface (url bar, back button, capture, stack, the IS_MOBILE class): nothing in apps/browser changed.
- The web-spa tokens and canvas (the #f5f5f5 page, the canvas sized from host CSS): no index.html or dioxus-native config changed.
- The cli tokens: WPT runner output colours from owo-colors, and the paint_bench and screenshot examples' terminal output. The WPT runner and the examples were not touched.
- The cli Navigation Pattern and Platform-Specific Notes NOT YET MEASURED absences: the statement measured no argument parsing structure or terminal-width handling.
- Brand Identity, Anti-Patterns, Self-Validation Protocol and the Design Decisions Log, all NO RECORDED INTENT: the session recorded no design intent.
