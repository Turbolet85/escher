# Task — design · 2026-10-24-stand-requirement-sweep

## The chunk
A test-and-gate sweep: every 0.2.0 requirement in reach (v020-01 to v020-09) gets a recorded witness, a task of the 7GUIs stand or a minimal fixture page beside it plus the check that proves it there, and any requirement without one gets a new check.
Every named witness runs inside the stand-repeat leg (three rounds, zero failures), the coverage floors stay held, and the new driver surfaces (the lint verb, snapshot parts, --record recording) are measured id-free on stderr at trace level under both builds.
Folded freight: new fixture checks in both layout modes assert the visual lint's untested readings (a clipped or off-screen node holding text also reads truncated) and text_extent's stated limits (RTL and vertical text read as LTR lines, a transformed box read at its client rect).
It changes no stand task's markup or CSS and no engine painting; the engine text reader and the lint change only if the founder rules a reading a defect.

## Reached files
- tests/blitz-tests/tests/stand_requirement_sweep.rs · merge surface
- tests/blitz-tests/tests/common/mod.rs
- tests/blitz-tests/tests/session_common/mod.rs
- tests/blitz-tests/tests/fixtures/mod.rs
- tests/blitz-tests/tests/text_extent.rs
- tests/blitz-tests/Cargo.toml
- .github/scripts/test_blitz_tests_targets.py
- examples/seven_guis/tests/host_log.rs
- examples/seven_guis/tests/mcp_log.rs
- examples/seven_guis/tests/common/mod.rs
- .github/scripts/ci-leg.sh · conflict surface
- .github/scripts/test_ci_workflows.py
- packages/escher-driver/src/lint.rs
- packages/blitz-dom/src/text_extent.rs

## Needs
- The 7GUIs stand's four lean tasks (counter, flight booker, timer, CRUD) booted headlessly at the pinned viewport with the bundled font, as the screens every re-proven witness reads.
- Minimal fixture pages beside the stand: new pages with text in a clipping box, an off-screen box, RTL text, vertical writing-mode text and a transformed box, each read by the lint and text_extent.
- The bundled font and font loading: blitz-tests needs system-fonts for text to measure, and the fonts-unavailable refusal of screenshot and lint before anything is painted or read.
- Engine text mapping and line layout as text_extent reads it (content box and inline text box), which the truncated and RTL/vertical readings depend on.
- Hit testing and clipping at the padding box of a box that paint clips (overflow or contain: paint), the root excepted, which the clipped, off-screen and covered readings rest on.
- The visual lint's six defects (overlap, clipped, truncated, off-screen, zero-size, contrast) and its findings as ids, closed words and numbers, re-proven on the stand (0 findings at boot per lean task) and on fixtures.
- The contrast defect, indirectly through the stand palette: the lint's contrast reading uses the stand's text and background colours, the accent #3a5ce5 with its hover and active steps, composited over opaque white, under the stand contrast check's exempt rule.
- Engine-painted defaults: the opaque white canvas a screenshot and a contrast read composite over, read by the screenshot witnesses re-proven by the sweep.
- Status and state colours of the stand controls only as they surface in snapshot readings (selected, pressed, invalid, disabled), which the snapshot witnesses re-prove unchanged.
- Motion and time: the VirtualClock the harness documents read, settle answering animation without advancing it, advance moving it by exactly its ms, click, type and press leaving it still, and the load wait moving no clock; the timer and virtual-clock witnesses are re-proven.
- The into-view scroll written at once in every nested scrolling box (smooth applying to the viewport alone), re-proven by the scroll witnesses.
- The double-click window on the document's clock, which password_caret_click's witness reads.
- The cli surface: escher-session and its mcp role write only JSON on stdout, no colour or ESC byte, no terminal test, exit statuses 0, 1, 2, 3; the stderr measurement in host_log and mcp_log keeps stdout as the answer and finds no id, name or typed text in stderr.
- Screenshot output: an 800 by 600 PNG of the stand screen, byte-identical across processes on one host, as a witness the sweep names.
- Cursor and pointer behaviour only as the range input and select checks drive pointer input on fixtures, unchanged.

## Not needed
- The browser app's iconography, toolbar icons, tab strip glyphs, favicons and rdme's octicons: no browser or markdown viewer screen is touched.
- The mobile-native surface (Android and iOS urlbar, back button, capture, stack): nothing here builds or runs on mobile.
- The web-spa surface (the WASM canvas on #f5f5f5): the sweep is headless native only.
- The desktop-native surface's titlebar, custom widget, devtools overlay, window navigation and stack notes: no windowed run is part of this chunk.
- The WPT runner's cli tokens (owo-colors terminal output): the WPT runner is not touched.
- Border radius values and any radius scale: no fixture or stand style depends on corner shape.
- Spacing values and any spacing scale: no stand layout changes, and fixture pages are minimal.
- Depth strategy's elevation values and shadows beyond the hit-at-clip rule: no elevation is read or changed.
- Typography's type scale and the engine's font sizes beyond what a fixture page needs to lay out text: no type style changes.
- Reduced-motion handling: none exists and the chunk adds none.
- The stand's colour tokens re-asserted row by row, Tab order and Enter/Space activation: owned by the later A11y re-read entry, not this sweep.
- A project-wide colour token set or brand identity: none is recorded and the chunk defines none.
