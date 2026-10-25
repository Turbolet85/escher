# Task — design · 2026-10-25-capability-statement

## The chunk
Writes one capability statement, current with 0.2.0, a document and not code: what escher can do (the driver's twelve verbs on the socket, the command line, MCP and a kept check; stable ids; the snapshot, its text, parts and diff; the headless screenshot; the visual lint's six defects; recording and replay; the load wait; the select and range models; keyboard activation; the one virtual clock).
States its limits in one place: the three measured-off readings (off-screen under a transformed scrolling box, covered on a zero-size box and on a box clipped on one axis), the boolean model's two limits, the select's unmodelled parts, a shot's PNG bytes differing across OSes, HTML's focus fixup not built, RTL and vertical text read as LTR lines, and the unpaged diff and lint findings.
States its a11y reach (SC 2.1.1, 1.4.3, 2.4.3 as proven on the stand's Tab order, Enter and Space activation and contrast rows; no assistive-technology run measured) and what an MCP client carries onward from escher's stdio.
Corrects README's verb count (eleven to twelve); where the statement stands for Viola's planning is an open fork (README, a tracked escher document the README points to, or Viola's incubator). No code, engine, schema, verb, carrier, style or colour changes.

## Reached files
- README.md · edits the MCP role's verb-count line, its 0.2.0 stage list

## Needs
- The seven_guis stand's accent colour and its hover and active steps, as the colour the statement's contrast claim rests on (white button text on the accent passing SC 1.4.3's 4.5 at rest, hover and active)
- The stand's text and background colour pairs and their measured contrast ratios, quoted by the a11y-reach section through the stand contrast check
- Semantic and state colours the stand reaches (hover, active, invalid, selected), as far as the contrast rows the statement cites cover them
- The screenshot's font requirement: an app with no sans-serif family refused fonts-unavailable with nothing painted, stated as a capability limit of the shot and the lint
- The system-fonts build requirement for headless text measurement, as the condition behind the statement's screenshot, lint and contrast claims
- The bundled font the stand boots with, as the reason a headless shot paints text offline
- A password input painted masked in every shot, as part of what a shot carries onward to an MCP client
- Hit-testing stopping at the padding box of a box that clips by overflow or contain paint, as the engine behaviour behind the stated covered limits (a box clipped on one axis, a zero-size box)
- The into-view scroll writing every nested scrolling box at once and the driver's scroll being instant, as a stated behaviour of the scroll verb
- The virtual clock: advance moving it by exactly its ms, click, type and press leaving it, settle never advancing it, as the one-clock capability the statement names
- The load wait moving no clock while it waits, so no timer or animation runs during it, as part of the load-wait capability
- Settle answering an animating document as settled without waiting on or advancing animations, as a limit an agent reading the statement must know
- Reduced-motion handling observed absent, if the statement's a11y reach lists what is not built
- The cli surface's output rules: escher-session and its mcp role write only JSON on stdout, no colour or ESC byte, no terminal test, the exit status carrying the class (0, 1, 2, 3), as the carrier form the statement describes
- Platform reach of the windowed surface (winit window, dioxus-native launch) only as far as the statement says the a11y adapter crossing is unmeasured at a windowed run
- The cursor rules only if the statement describes the windowed shell; otherwise not reached

## Not needed
- The reference browser app's toolbar icons, icon library, size grid, favicons and tab-strip glyphs: the statement covers the driver and the stand, not the upstream browser
- The mobile-native surface (Android APK, iOS build, urlbar, back button, capture): no capability the statement names runs there
- The web-spa surface's WASM canvas tokens and component patterns: the statement's capabilities are headless and native
- The desktop-native titlebar, custom widget and devtools overlay patterns and the browser's packaging targets: no capability or limit the statement states reads them
- Border radius values and absences: no claim in the statement rests on radius
- Spacing values in the apps, engine and fixtures: no claim rests on spacing
- Depth strategy's observed elevation and shadow values in the apps and examples: only the hit-testing rule is reached
- The typography scale, weights and line heights in the apps and examples beyond the large-text threshold used by contrast: the statement quotes no type scale
- The surface scale, text hierarchy and border progression colours outside the stand rows the contrast check measures
- The engine-painted default colours and the user-agent stylesheet: the statement quotes the stand's measured pairs, not engine defaults
- The WPT runner's owo-colors terminal output: a cli surface the statement does not describe
