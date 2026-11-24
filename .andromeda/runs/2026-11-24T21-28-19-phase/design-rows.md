# design-system · the rows that bind 2026-11-24-canvas-lint — picked 17 of 140 (9 by every draw) · rule any · pin 3685

## color-palette · ## Color Palette
decision/circle-drawer-canvas-colours-in-code · Circle Drawer canvas colours as widget constants — Circle Drawer's canvas colours, the stand accent stroke, the dark label and the accent fill at alpha 38/255, are constants in the sample's widget code, as the removed CSS rules held them. · why: The canvas paints from widget code, not the cascade; an exception to colours living in stylesheets.

## typography · ## Typography
decision/test-engine-with-system-fonts · Test engine built with system fonts — The blitz-dom engine is built with the system-fonts feature for the blitz-tests crate. · why: Without system fonts text measures 0x0 and font-dependent assertions pass vacuously.
decision/tests-built-with-system-fonts · Tests built with system fonts — The blitz-tests crate builds `blitz-dom` with the `system-fonts` feature, which is also enabled by default when testing the whole workspace. · why: Without system fonts text measures 0x0 and font-dependent assertions pass vacuously.
decision/unavailable-families-use-default-font · Unavailable font families fall back to the default font — When none of an element's listed font families is available, its text lays out in the engine's default font, and font-relative units measure against that font. · why: Taken from upstream, ratified as upstream's behaviour. · guard: font_family_fallback
decision/no-generic-family-lays-out-sans · No generic family lays out as sans-serif — An element with no generic font family is laid out as sans-serif. · why: can_paint_text, behind the driver's screenshot refusal, reads only the sans-serif generic because unstyled text lays out with it

## motion-calibrated-to-expression-level-unrecorded · ## Motion (calibrated to expression level `unrecorded`) · twin motion
contract/driver-verbs-and-virtual-clock/canvas · driver verbs and the virtual clock · canvas — The driver's canvas read changes nothing, neither snapshot, focus, diff nor VirtualClock, and applies no pending Timer tick.

## surface · ## Surface: desktop-native · twin surface-desktop-native
decision/canvas-line-form-conventions · Canvas line form conventions — The canvas line writes colours as rgba hex and a Custom paint as #00000000, blend= as mix,compose, cap= as one word where both caps agree, dash= as the lengths, @, then the offset, and a glyph run with no glyph as at=none. · why: Settled from neighbouring code and stated in the text field's help.
decision/escher-session-agent-read-surface · escher-session is read by an agent, not a person — An escher-session command prints one line of JSON on stdout, the answer, the refusal or a session error's object keyed by the driver schema's own words, and nothing else there.
contract/escher-session-commands/help · escher-session commands · help — help answers the tool's self-description as one uncoloured line of JSON on stdout with no ESC byte, status 0, reaching no session.
decision/escher-session-no-colour · No colour and no terminal test on escher-session — escher-session uses no colour and no colour dependency, writes no ESC byte on any answer line or MCP reply, and never asks whether a stream is a terminal, so its output is the same piped or not. · guard: the seven_guis checks' shared reader and the shared Speaker assert no ESC byte on every line
decision/exit-statuses-from-schema-table · Exit statuses read from the schema's table — The exit statuses are read from the driver schema's EXIT_STATUSES table, whose meanings help serves.
decision/logical-viewport-from-scale · Logical viewport is physical size over total scale — The logical viewport size is the physical window size divided by hidpi scale times zoom. · why: the stylo device is built from logical_size; pinned by resize_and_zoom_coalesce in the device_coalescing tests
contract/escher-session-exit-status/0 · escher-session exit status · 0 — Accepted.
contract/escher-session-mcp/help-result · escher-session mcp role · help tool result — help's tool result is the self-description's line.

## surface-cli · ## Surface: cli
contract/escher-session-stdout/command-line-line · escher-session stdout · command line output — One JSON line per command: the answer, the refusal or a session error's object.
decision/driver-output-for-agents · Driver command line output read by agents — The escher-session command line and its mcp role write only JSON to stdout, never colour or an ESC byte, and no code tests for a terminal, so the output is the same piped or not. · why: These surfaces are read by an agent, not a person.
contract/escher-session-exit-codes/0 · escher-session exit codes · 0 — The command was accepted.

# design-system · its absences, names and overviews, whole

## brand-identity · ## Brand Identity
> NO RECORDED INTENT

## color-palette · ## Color Palette
Border Progression › > NOT YET MEASURED — a project-wide UI colour token set: the examples slice (s01) and the DOM-node slice (s08) recorded the project's own colour tokens as out of slice, and the default user-agent stylesheet's full contents were out of slice for s05.
absence/engine-color-tokens — Color tokens or CSS custom properties in the engine and integration crates

## typography · ## Typography
> NOT YET MEASURED — a project-wide type scale: the examples slice (s01) recorded the project's own type scale as out of slice, and the WPT runner slice (s13) recorded typography as out of slice.
absence/project-type-scale — A project type scale or font tokens

## spacing · ## Spacing
> NOT YET MEASURED — a project-wide spacing scale or base unit: the examples, engine, test and WPT slices (s01, s05, s07, s10, s11, s12, s13) recorded a spacing scale as out of slice.

## depth-strategy · ## Depth Strategy
> NO RECORDED INTENT ¶ > NOT YET MEASURED — a project-wide elevation scale: the examples, engine-resolve, layout and WPT slices (s01, s05, s07, s13) recorded it as out of slice.

## border-radius · ## Border Radius
> NO RECORDED INTENT ¶ > NOT YET MEASURED — a project-wide radius scale: the examples, test and WPT slices (s01, s12, s13) recorded radius tokens as out of slice.

## motion-calibrated-to-expression-level-unrecorded · ## Motion (calibrated to expression level `unrecorded`) · twin motion
> NO RECORDED INTENT
absence/reduced-motion-handling — Reduced-motion handling

## surface-cli · ## Surface: cli
Navigation Pattern › > NOT YET MEASURED — no slice recorded argument parsing or command structure for the CLI binaries. ¶ Platform-Specific Notes › > NOT YET MEASURED — no slice recorded terminal-width handling or platform differences for the CLI binaries.

## surface-web-spa · ## Surface: web-spa
Navigation Pattern › > NOT YET MEASURED — no slice recorded routing or navigation for the WASM pages. ¶ Platform-Specific Notes › > NOT YET MEASURED — no slice recorded browser-specific limitations for the WASM pages.
overview/surface-web-spa — The web-spa surface is WASM pages rendering into a canvas: on wasm32 the dioxus-native surface is a canvas appended to the page body and sized from host CSS, so a page such as todomvc fills the window with that canvas.

## anti-patterns-never-do-these · ## Anti-Patterns (NEVER do these)
> NO RECORDED INTENT

## self-validation-protocol · ## Self-Validation Protocol
> NO RECORDED INTENT

## design-decisions-log · ## Design Decisions Log
> NO RECORDED INTENT
