# Task — design-system · 2026-10-25-cold-agent-bar

## The session
The flight booker's two aria-pressed mode buttons (flight-one-way, flight-return) and their CSS (.flight-type-row, .type-btn, .type-btn-active) are retired; one select flight-type (aria-label Flight type, class type-select) with two options one-way and return takes their place.
The select's CSS: 15px, #1a1a1a on #ffffff, 1px solid #c0c0c0 border, radius 6, padding 8/10; on focus it drops the outline for a #3a5ce5 border, as the date inputs do; measured contrast 17.40 at 15px 400.
The headless stand gains two served pages beside the lean four: Cells (boot_cells) and a load page (boot_load) whose Load button is white on the accent #3a5ce5 (hover #2a4cd3, active #1f3fc1), with a Loaded paragraph hidden until an in-process stand-in answers its sheet after 200 ms.
escher-session now serves six tasks (counter, flight-booker, timer, crud, cells, load); the cold-agent pipe drives all six, and the founder's one live run passed (66 calls, 0 wrong).
Measured: the stand states no aria-pressed anywhere now; the flight booker's lint at boot reads one overlap (its two options at the select's client rect); right after a scroll the shown overlay scrollbar takes a click in its band as a thumb drag until an advance passes its fade.
Stand counts moved: controls 15 to 14; stand_contrast control rows 18 to 17 and STATE_ROWS 19 to 18; the keyboard activation table 20 to 16 rows.

## Reached files
- tests/blitz-tests/tests/stand_cells_served.rs
- tests/blitz-tests/tests/stand_load_page.rs
- examples/seven_guis/src/tasks/flight_booker.rs
- examples/seven_guis/src/stand.rs
- examples/seven_guis/src/session_host.rs
- examples/seven_guis/tests/cli_commands.rs
- examples/seven_guis/tests/host_log.rs
- examples/seven_guis/tests/common/mod.rs
- tests/blitz-tests/tests/common/mod.rs
- tests/blitz-tests/tests/session_common/mod.rs
- tests/blitz-tests/tests/stand_keyboard.rs
- tests/blitz-tests/tests/stand_contrast.rs
- tests/blitz-tests/tests/stand_act_states.rs
- tests/blitz-tests/tests/stand_snapshot_state.rs
- tests/blitz-tests/tests/stand_act_disabled.rs
- tests/blitz-tests/tests/stand_act_ids.rs
- tests/blitz-tests/tests/stand_act_diff.rs
- tests/blitz-tests/tests/stand_element_ids.rs
- tests/blitz-tests/tests/stand_id_persistence.rs
- tests/blitz-tests/tests/stand_snapshot.rs
- tests/blitz-tests/tests/stand_session_recording.rs
- tests/blitz-tests/tests/stand_requirement_sweep.rs
- tests/blitz-tests/tests/stand_lint.rs
- tests/blitz-tests/tests/stand_accessibility_ids.rs
- scripts/cold-agent.sh
- .github/scripts/test_cold_agent.py
- escher-0.2.0/verification-matrix.json
- escher-0.2.0/chunks/2026-10-25-cold-agent-bar/scope-record.md
- escher-0.2.0/chunks/2026-10-25-cold-agent-bar/evidence/operator-pass.md
- escher-0.2.0/chunks/2026-10-25-cold-agent-bar/evidence/live-verdict.json
- escher-0.2.0/chunks/2026-10-25-cold-agent-bar/evidence/live-run.md
- escher-0.2.0/chunks/2026-10-25-cold-agent-bar/evidence/live-check.json
- escher-0.2.0/chunks/2026-10-25-cold-agent-bar/evidence/live-transcript.jsonl

## Touches
- Color Palette's flight-mode site (color-palette.md:27): the flight-one-way accent row (white on #3a5ce5, 5.47, 14px 600) leaves the accent rows; the select's pair (#1a1a1a on #ffffff, 17.40, 15px 400) is measured in its place.
- Core Colors' accent rule (white text on the accent controls passing SC 1.4.3 at rest, hover and active): the flight booker's active mode button is no longer one of those controls; the load page's Load button joins them from CSS that lives outside the task files; the select uses the accent as its focus border, a non-text use.
- Any stand contrast census held in the palette: control rows 18 to 17 (tally 14 · 0 · 3 · 1 to 13 · 0 · 3 · 1), STATE_ROWS 19 to 18 (tally 18 · 0 · 1 · 0 to 17 · 0 · 1 · 0); the return-chosen rows now read flight-type 17.40 and flight-return-date 17.40, where they read flight-return 5.47, flight-one-way 9.33 and flight-return-date 17.40.
- Text Hierarchy or Surface Scale pairs citing the flight booker's inactive mode button (#444444 on #fafafa, 9.33): that pair is retired with the buttons.
- Border Progression: the .type-btn borders are retired; the select adds a 1px solid #c0c0c0 border and a #3a5ce5 focus border replacing the outline, matching the date inputs.
- Semantic Colors / status states: the pressed or active mode-button state (.type-btn-active, aria-pressed) is retired; the stand now states no aria-pressed anywhere, and the flight type's state is a select choice (selected).
- Typography's example-fixture values for the flight booker: the 14px 600 mode-button text is retired; the select is 15px 400.
- Border Radius and Spacing example-fixture values for the flight booker: the .type-btn radius and padding are retired; the select reads radius 6 and padding 8/10.
- Any count of the stand's controls: 15 to 14 (controls(task) rows 18 to 17).
- Any count of keyboard activation rows held as design evidence: 20 to 16 rows (BUTTONS 10 to 8).
- Any claim that the lean tasks read 0 visual-lint findings at boot: the flight booker now reads one overlap (flight-type-one-way × flight-type-return), the other three lean tasks 0.
- Surface desktop-native component patterns that describe the flight booker's controls: its mode control is a select read as ComboBox Flight type, holding two ListBoxOptions, with Tab order back-btn, flight-type, flight-start, flight-book (+ flight-return-date when enabled).
- Surface cli: escher-session serves six tasks, not the four lean ones; its usage line's apps list and the MCP start tool's enum follow.
- Line citations into examples/seven_guis/src/session_host.rs (lines 1-5, 12-27, 43, 48-50 added), examples/seven_guis/src/stand.rs (lines 1-9, 12, 17, 131-245, 283-330 added) and examples/seven_guis/src/tasks/flight_booker.rs (lines 66-80, 83-86, 160-171 added): any path:N past those lines has shifted.
- Motion's load-wait rule: the stand now holds a page that exercises it, the load page's stand-in answering after LOAD_ANSWER_DELAY (200 ms) from its own thread while the acting click waits it out and the Loaded message shows.
- Any claim that the stand is wholly offline: every boot keeps net_provider None except the load page, which holds an in-process stand-in (LoadNet) with no blitz_net, socket or listener.
- Motion or animation-runtime facts about the overlay scrollbar fade: the session measured its interaction with a click right after a scroll (see New facts).
- The accent's design-token use on new surfaces: the load page uses #3a5ce5 / #2a4cd3 / #1f3fc1, #1a1a1a and #f5f5f5.

## New facts
- F1 · color-palette · The flight booker's flight-type select paints #1a1a1a on #ffffff with a 1px solid #c0c0c0 border; focused, it drops the outline for a #3a5ce5 border, as the date inputs do; its text/background ratio is 17.40.
- F2 · color-palette · The load page's Load button (id load-fetch) is white on the accent #3a5ce5, hover #2a4cd3, active #1f3fc1; the page also uses #1a1a1a and #f5f5f5, and it has no contrast row because it is no lean task.
- F3 · color-palette · The stand states no aria-pressed anywhere: the flight type's state is the select's chosen option (selected), so the stand has no pressed-state colour pair left.
- F4 · typography · The flight-type select's text is 15px at weight 400.
- F5 · border-radius · The flight-type select's corner radius is 6.
- F6 · spacing · The flight-type select's padding is 8 vertical, 10 horizontal.
- F7 · motion · Right after a scroll the shown overlay scrollbar takes a main-button pointerdown in its band as a thumb drag and dispatches nothing; on the virtual clock it stays shown until an advance passes its fade, and a pointer left on the thumb keeps it shown.
- F8 · motion · The load page's in-process stand-in answers its one sheet after LOAD_ANSWER_DELAY (200 ms) from a thread of its own; the Loaded paragraph stays display none until that sheet lands.
- F9 · surface · escher-session serves six tasks (counter, flight-booker, timer, crud, cells, load), Cells and the load page mounted in the same TaskShell as the lean four; its usage line and the MCP start enum list all six.
- F10 · surface · The load page holds two keyed elements, the Load button and the Loaded paragraph; no CSS selects by id, and its classes are load-root, load-body, load-btn (with hover and active) and load-done.

## Untouched
- Brand Identity, Anti-Patterns, Self-Validation Protocol and the Design Decisions Log: still NO RECORDED INTENT; the session recorded no design intent.
- Iconography: the browser's toolbar icons, cursors and favicons; no browser app or cursor rule changed.
- Surface mobile-native and Surface web-spa: no Android, iOS or WASM page or canvas changed.
- Depth Strategy's hit-testing rule at a paint-clipping box: no engine crate changed.
- Typography loading: the fonts-unavailable refusal, the system-fonts feature and the WASM font context; no engine or driver change.
- Engine-painted defaults and engine text mapping: no engine crate changed.
- Motion's into-view scroll and settle rules: the driver's scroll stays instant and settle still neither waits on nor advances an animation.
- The accent hex values themselves (#3a5ce5, #2a4cd3, #1f3fc1): unchanged; only which controls carry them moved.
- The counter, timer and CRUD colours and the CRUD list rows: no change to those tasks.
- The escher-session stdout contract (JSON only, no colour or ESC byte, the four exit statuses): unchanged; only the served task list grew.
- The NOT YET MEASURED notes for project-wide colour, type, spacing, elevation and radius scales: still unmeasured.
