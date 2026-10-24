# Task — design · 2026-10-24-a11y-re-read

## The chunk
A re-read of the seven_guis stand's accessibility checks after 0.2.0's engine changes. It changes no engine behaviour and adds no new surface.
It re-asserts the stand's Tab order at boot and in its enabled states, in both layout modes, and Enter and Space activation on each stand button. A row is added for a stand control that 0.2.0 made keyboard-operable, the flight booker's single-choice select being the candidate.
It re-asserts the stand's text/background contrast rows (controls, CRUD list rows, text outside the controls, hover and active states) against SC 1.4.3.
It widens the CI `a11y` leg to the 0.2.0 a11y witnesses outside it, and moves the leg's pin with it. The job keeps its tier and takes no required status check.

## Reached files
- tests/blitz-tests/tests/stand_keyboard.rs
- tests/blitz-tests/tests/common/mod.rs · conflict surface
- tests/blitz-tests/tests/stand_contrast.rs
- .github/scripts/ci-leg.sh · conflict surface
- .github/scripts/test_ci_workflows.py

## Needs
- The seven_guis accent colour and its hover and active shades: the text/background pairs that stand_contrast asserts, white button text on the accent at rest, on hover and when active.
- The stand's surface, text-hierarchy and border colours where they are the background or foreground of a measured text row, the CRUD list rows and the text outside the controls included.
- Semantic and status-state colours the stand reaches: hover, active, selected, invalid and disabled states, wherever a state row reads a colour pair.
- The exemption rule for contrast rows: a disabled control, and a pair that cannot be read (an opacity below 1, a filter or a background image), are exempt from SC 1.4.3. Comes in through stand_contrast's row table.
- The large-text threshold: the 3.0 versus 4.5 required ratio, decided by font size and weight. Comes in through the typography values that set the stand's text sizes and weights.
- Font loading for headless checks: the bundled stand font and blitz-dom's `system-fonts` feature for blitz-tests. Without them text measures zero and contrast or text assertions pass vacuously.
- Focus-visible styling on stand controls: any colour or outline drawn for a focused button, input, select or the CRUD list, where it carries a colour pair a state row reads.
- The flight booker's select as a component, its chosen option's text and its focus state, as a new keyboard row (ArrowDown, ArrowUp, Home, End and typeahead). Indirect: typeahead's 1 s buffer runs on the document's clock.
- The harness's VirtualClock as the time source for keyboard checks. Indirect, through the motion values: settle never advances the clock, and only the driver's `advance` moves it. A typeahead or double-click row depends on that.
- The hit-test stop at a paint-clipping box (`overflow`, `contain: paint`), and `inert` passing a hit through. Indirect: they decide which element a synthesized click on a stand button or row reaches during a re-read.
- The CRUD list as a keyboard region: its Tab stop, and arrow, Home and End selection that fills the fields. Its selected-row colours are read by the contrast rows.
- The stand as a desktop-native Dioxus app booted headlessly at a pinned viewport, offline. That is the surface every re-asserted row is read on.

## Not needed
- Iconography: the browser app's toolbar SVG icons, favicons, tab glyphs and cursor mapping. No stand control or check reads an icon or a cursor.
- Border radius values and the radius absence notes. No check measures corner shape.
- Spacing values and scale. Tab order and contrast read no spacing token, and the chunk changes no layout.
- Depth and elevation values (shadows, z-layering in the apps). Only the hit-test clip rule reaches the chunk.
- Motion's into-view scroll behaviour and the driver's load wait. The chunk adds no `scroll` or load step.
- Mobile-native surface: Android and iOS packaging, the urlbar, back button and capture. The stand is not checked on mobile.
- Web-spa surface: WASM canvas pages and their tokens. The re-read runs natively and headlessly.
- Desktop-native titlebar, custom widget, devtools overlay and window and stack notes. They belong to the browser app and examples, not the stand. The windowed assistive-technology witness is out of scope.
- CLI surface: escher-session's JSON output, exit statuses and the WPT runner's colours. The chunk adds no verb, field or output.
- The screenshot `fonts-unavailable` refusal and the WASM bundled-font note. The chunk takes no screenshot and builds no WASM.
