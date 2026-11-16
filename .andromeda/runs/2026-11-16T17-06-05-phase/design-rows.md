# design-system · the rows that bind 2026-11-16-error-text-in-the-answer — picked 33 of 136 (7 by every draw) · rule any · pin 1455

## motion-calibrated-to-expression-level-unrecorded · ## Motion (calibrated to expression level `unrecorded`) · twin motion
decision/load-wait-moves-no-clock · Load wait moves no clock — While an acting driver step waits on a load in flight, for at most LOAD_WAIT, the harness's VirtualClock stands still, so no timer or animation runs during the wait.
contract/driver-verbs-and-virtual-clock/advance · driver verbs and the virtual clock · advance — The driver's advance moves the VirtualClock by exactly its ms and, through the step a session's caller hands it, the app's own time together.
contract/driver-verbs-and-virtual-clock/click · driver verbs and the virtual clock · click — The driver's click leaves the VirtualClock where it was.
contract/driver-verbs-and-virtual-clock/press · driver verbs and the virtual clock · press — The driver's press leaves the VirtualClock where it was.
contract/driver-verbs-and-virtual-clock/type · driver verbs and the virtual clock · type — The driver's type leaves the VirtualClock where it was.

## motion · ## Motion (calibrated to expression level `unrecorded`) · twin motion-calibrated-to-expression-level-unrecorded
decision/driver-advance-moves-one-clock · Driver advance moves the one virtual clock — The driver's advance moves the harness virtual clock by exactly its ms and the app's own time together, while click, type and press leave the clock where it was. · why: Ratified.

## surface · ## Surface: desktop-native · twin surface-desktop-native
contract/escher-session-mcp/other-results · escher-session mcp role · other tool results — Any other tool result's text is the command's JSON line.
contract/escher-session-tasks/cells · escher-session tasks · cells — cells is one of the six tasks escher-session serves, listed in its usage line's apps: and the MCP start tool's enum.
contract/escher-session-tasks/counter · escher-session tasks · counter — counter is one of the six tasks escher-session serves, listed in its usage line's apps: and the MCP start tool's enum.
contract/escher-session-tasks/crud · escher-session tasks · crud — crud is one of the six tasks escher-session serves, listed in its usage line's apps: and the MCP start tool's enum.
contract/escher-session-tasks/flight-booker · escher-session tasks · flight-booker — flight-booker is one of the six tasks escher-session serves, listed in its usage line's apps: and the MCP start tool's enum.
contract/escher-session-tasks/load · escher-session tasks · load — load is one of the six tasks escher-session serves, listed in its usage line's apps: and the MCP start tool's enum.
contract/escher-session-tasks/timer · escher-session tasks · timer — timer is one of the six tasks escher-session serves, listed in its usage line's apps: and the MCP start tool's enum.
decision/escher-session-agent-read-surface · escher-session is read by an agent, not a person — An escher-session command prints one line of JSON on stdout, the answer, the refusal or a session error's object keyed by the driver schema's own words, and nothing else there.
decision/escher-session-no-colour · No colour and no terminal test on escher-session — escher-session uses no colour and no colour dependency, writes no ESC byte on any answer line or MCP reply, and never asks whether a stream is a terminal, so its output is the same piped or not. · guard: the seven_guis checks' shared reader and the shared Speaker assert no ESC byte on every line
decision/exit-statuses-from-schema-table · Exit statuses read from the schema's table — The exit statuses are read from the driver schema's EXIT_STATUSES table, whose meanings help serves.
contract/coverage-check-files/read-bound · coverage check files · read bound — Each check path is read bounded at 1,048,576 + 1 bytes and parsed strictly.
contract/coverage-check-files/refusal · coverage check files · refusal — A file that cannot be read, one past the bound, one that is no check and a check of another app are each refused malformed, status 1, the refusal holding nothing of the path or the file.
contract/escher-session-commands/help · escher-session commands · help — help answers the tool's self-description as one uncoloured line of JSON on stdout with no ESC byte, status 0, reaching no session.
contract/escher-session-exit-status/0 · escher-session exit status · 0 — Accepted.
contract/escher-session-exit-status/1 · escher-session exit status · 1 — Refused.
contract/escher-session-exit-status/3 · escher-session exit status · 3 — Session error, the recording's record-dir-not-private and recording-failed among them, with no exit of their own.
contract/escher-session-mcp/err-ending · escher-session mcp role · Err ending — An Err ending's fixed message goes to stderr with status 3.
contract/escher-session-mcp/help-result · escher-session mcp role · help tool result — help's tool result is the self-description's line.
contract/escher-session-mcp/stdout · escher-session mcp role · stdout — stdout holds newline-delimited JSON-RPC 2.0 protocol lines only, a reply per request that is not a notification, every reply one uncoloured line with no ESC byte; its end writes nothing there.
contract/escher-session-session-error/remedy · escher-session session error object · remedy — A session error's object carries its fixed remedy.
contract/escher-session-stderr/session-error · escher-session stderr · session error — A session error prints its fixed message on stderr.

## surface-cli · ## Surface: cli
decision/driver-output-for-agents · Driver command line output read by agents — The escher-session command line and its mcp role write only JSON to stdout, never colour or an ESC byte, and no code tests for a terminal, so the output is the same piped or not. · why: These surfaces are read by an agent, not a person.
contract/escher-session-exit-codes/0 · escher-session exit codes · 0 — The command was accepted.
contract/escher-session-exit-codes/1 · escher-session exit codes · 1 — The command was refused.
contract/escher-session-exit-codes/3 · escher-session exit codes · 3 — A session error; an mcp role ending in Err also exits 3.
contract/escher-session-stdout/command-line-line · escher-session stdout · command line output — One JSON line per command: the answer, the refusal or a session error's object.
contract/escher-session-stdout/mcp-line · escher-session stdout · mcp role output — Newline-delimited JSON-RPC 2.0 lines.

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
