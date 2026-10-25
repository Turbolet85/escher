# Task — design · 2026-10-25-cold-agent-bar

## The chunk
Extends the seven_guis stand so the driver serves every 0.2.0 check: Cells booted by `escher-session start` (the one stand screen over the snapshot text budget, so a paged `snapshot` is reached), flight booker's one-way / return choice turned from two buttons into a real `select`, and a load in flight carried on the stand (how is a P4 fork).
Gives the cold-agent pipe new goals over 0.2.0's capabilities (recording and replay, lint, `advance` on the virtual clock, the load wait, the select, the screen in parts) and runs it live once on the operator's login, judged by a wrong-call bar the founder sets at P4 from 0.1.0's 0 wrong in 43.
Re-pins every stand table the select and the served Cells move: the Tab order and Enter/Space tables, the contrast rows, element ids and actionable keys, the snapshot, diff and act checks, and the flight flow.
Re-reads whether the stand's colours, contrast and painted text still hold once a select and Cells are served headlessly and shot or linted by the driver.

## Reached files
- examples/seven_guis/src/session_host.rs · edits the task list `start` boots
- examples/seven_guis/src/stand.rs · edits `stand::options` (the offline headless boot)
- examples/seven_guis/src/tasks/flight_booker.rs · edits the one-way / return choice
- examples/seven_guis/src/tasks/cells.rs
- examples/seven_guis/tests/flows/flight.sh
- scripts/cold-agent.sh · edits `TASKS`, the `stand` prompt's goals
- .github/scripts/test_cold_agent.py
- tests/blitz-tests/tests/common/mod.rs · edits `tab_order`, `ENABLED_TAB_ORDER`
- tests/blitz-tests/tests/stand_keyboard.rs · edits the Tab order and Enter/Space tables
- tests/blitz-tests/tests/stand_contrast.rs · edits `TEXT_ROWS`, `STATE_ROWS`
- tests/blitz-tests/tests/stand_element_ids.rs
- tests/blitz-tests/tests/stand_actionable_keys.rs
- tests/blitz-tests/tests/stand_snapshot.rs
- tests/blitz-tests/tests/stand_snapshot_state.rs
- tests/blitz-tests/tests/stand_snapshot_text.rs
- tests/blitz-tests/tests/stand_snapshot_parts.rs
- tests/blitz-tests/tests/stand_diff.rs
- tests/blitz-tests/tests/stand_act_select.rs
- tests/blitz-tests/tests/stand_load_wait.rs
- tests/blitz-tests/tests/fixtures/mod.rs

## Needs
- The seven_guis accent and its hover and active shades, and every stand colour a new flight-booker `select` or a served Cells grid paints with, held to SC 1.4.3 by the stand contrast rows
- The stand's text, surface, border and status colours (invalid and selected states included) as the flight booker's select and its invalid-date state show them
- Semantic and status colours for invalid, selected and pressed states, since flight booker states them by `aria-*` and a select now carries the choice
- Engine-painted defaults for a form control the stand did not style before: how a `select` paints with no popup, its user-agent colours, border and padding
- The stand's typography: its bundled font, the font context a headless boot resolves, and the `fonts-unavailable` refusal a `screenshot` or `lint` of a served task returns before painting
- The engine's text mapping as Cells' cells and the select's chosen label lay out, since the lint reads `truncated` and `clipped` from text extents
- The stand's untokenized spacing and the engine's form-control spacing that set the select's size and the Cells grid's cell boxes, read by `bounds` and the lint's `overlap` and `zero-size`
- The stand's border and radius values a restyled flight-booker choice and the Cells grid carry
- Hit-testing at the padding box of a box that clips (by `overflow` or `contain: paint`), through Cells' scrolling grid, which sets the driver's `covered` and `off-screen` refusals there
- The stand's depth values and the engine's depth behaviour as a select and a scrolling grid stack over the task frame
- The driver's into-view `scroll`, instant in every box and the viewport, as an agent scrolls Cells' grid
- The harness's `VirtualClock`: settle never advancing it, `advance` moving it by exactly its `ms` with the Timer's step, and `click`, `type` and `press` leaving it, as the new goals exercise `advance`
- The load wait moving no clock while an acting step waits on a load in flight, which a stand load in flight now reaches
- Settle's answer to the animating set, as a stand task now holds a load or an animation the agent waits on
- The cursor rules (CSS `cursor`, else Text for a text input) a select and Cells' inputs fall under
- The cli surface's output rule for `escher-session` and its `mcp` role: JSON only on stdout, no colour or ESC byte, no terminal test, the exit status carrying the class — the cold-agent pipe's client reads it
- The headless stand's pinned viewport and the native window surface it stands in for, as Cells and the select must fit the pinned 800 × 600 screen the screenshot and lint read

## Not needed
- The browser app's toolbar, tab strip, URL bar, menu and favicon icons, its SVG icon set and GitHub octicons in rdme: the chunk touches no app under `apps/`
- The mobile-native surface (Android APK, iOS build, mobile URL bar, back button, capture): the stand and cold-agent run are headless on the dev host
- The desktop-native titlebar, custom widget and devtools overlay patterns: the stand runs headless through the driver, not in a winit window with chrome
- The web-spa surface and its canvas on `#f5f5f5`: no WASM page is built or changed
- The WPT runner's terminal colours through `owo-colors` and the `paint_bench` and `screenshot` example binaries' output: neither is the driver's command line
- The desktop installers and the flake package (NSIS, dmg, AppImage): no packaging or release is in scope
- Reduced-motion handling: observed absent and not built by this chunk
