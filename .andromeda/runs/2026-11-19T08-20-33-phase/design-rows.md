# design-system · the rows that bind 2026-11-19-build-directory-held — picked 17 of 139 (3 by every draw) · rule any · pin 2275

## typography · ## Typography
decision/test-engine-with-system-fonts · Test engine built with system fonts — The blitz-dom engine is built with the system-fonts feature for the blitz-tests crate. · why: Without system fonts text measures 0x0 and font-dependent assertions pass vacuously.
decision/tests-built-with-system-fonts · Tests built with system fonts — The blitz-tests crate builds `blitz-dom` with the `system-fonts` feature, which is also enabled by default when testing the whole workspace. · why: Without system fonts text measures 0x0 and font-dependent assertions pass vacuously.
decision/no-generic-family-lays-out-sans · No generic family lays out as sans-serif — An element with no generic font family is laid out as sans-serif. · why: can_paint_text, behind the driver's screenshot refusal, reads only the sans-serif generic because unstyled text lays out with it
decision/screenshot-never-skips-text · Screenshot never paints with text skipped — A driver screenshot of an app whose font context resolves no sans-serif family is refused with nothing painted, so a shot never paints text skipped.
decision/unavailable-families-use-default-font · Unavailable font families fall back to the default font — When none of an element's listed font families is available, its text lays out in the engine's default font, and font-relative units measure against that font. · why: Taken from upstream, ratified as upstream's behaviour. · guard: font_family_fallback
contract/driver-screenshot-refusals/fonts-unavailable · driver screenshot refusals · fonts-unavailable — A screenshot is refused fonts-unavailable, with nothing painted, when the app's font context resolves no sans-serif family.
contract/driver-screenshot/fonts-unavailable · driver screenshot · fonts-unavailable — A `screenshot` of an app whose font context resolves no `sans-serif` family is refused `fonts-unavailable` with nothing painted; a shot never paints with text skipped.

## surface · ## Surface: desktop-native · twin surface-desktop-native
decision/mobile-targets-built-not-tested · Mobile targets built, not tested — CI builds the iOS and Android targets but runs no tests on them.
decision/default-ua-sheet-via-default-config · Default user-agent sheet through the default config — Every document gets the blitz default user-agent stylesheet once, through the default document config. · why: pinned by default_config_uses_the_default_sheet; the test harness and Dioxus documents take the sheet only through DocumentConfig default
decision/escher-session-no-colour · No colour and no terminal test on escher-session — escher-session uses no colour and no colour dependency, writes no ESC byte on any answer line or MCP reply, and never asks whether a stream is a terminal, so its output is the same piped or not. · guard: the seven_guis checks' shared reader and the shared Speaker assert no ESC byte on every line
contract/escher-session-tasks/cells · escher-session tasks · cells — cells is one of the seven tasks escher-session serves, listed in its usage line's apps: and the MCP start tool's enum.
contract/escher-session-tasks/counter · escher-session tasks · counter — counter is one of the seven tasks escher-session serves, listed in its usage line's apps: and the MCP start tool's enum.
contract/escher-session-tasks/crud · escher-session tasks · crud — crud is one of the seven tasks escher-session serves, listed in its usage line's apps: and the MCP start tool's enum.
contract/escher-session-tasks/flight-booker · escher-session tasks · flight-booker — flight-booker is one of the seven tasks escher-session serves, listed in its usage line's apps: and the MCP start tool's enum.
contract/escher-session-tasks/load · escher-session tasks · load — load is one of the seven tasks escher-session serves, listed in its usage line's apps: and the MCP start tool's enum.
contract/escher-session-tasks/order · escher-session tasks · order — order is one of the seven tasks escher-session serves, listed in its usage line's apps: and the MCP start tool's enum.
contract/escher-session-tasks/timer · escher-session tasks · timer — timer is one of the seven tasks escher-session serves, listed in its usage line's apps: and the MCP start tool's enum.

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
