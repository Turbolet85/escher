@carried n=1 label=test-plan/test-scope-summary fact=fct-tz35iqmq
why: the served-page stand checks must stay green with their counts unmoved in both layout modes, since the scope names the stand_* checks as the witness that the buffered render leaves behaviour unchanged.
quote: `tests/blitz-tests` holds two checks on the pages the stand serves beside the lean four, each declaring `mod session_common;` and no `mod fixtures;`, each test in both layout modes, picked up by `run stand` and the stand-repeat leg by their prefix — `stand_cells_served` 5, on the stand's own Cells (`stand::boot_cells`, a screen of 47,089 bytes in 740 lines, 5 parts of `SNAPSHOT_TEXT_BUDGET`, as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md) through `Session::run`
@carried n=1 run=2 label=test-plan fact=fct-glytb3w3
quote: `run stand`'s `run.end` reads passed 237 · failed 0 · ignored 7 over 51 files and `run all`'s reads passed 743 · failed 0 · ignored 12, `dioxus_runtime_probe`, with no `stand_` prefix, picked up by `run all` with no script change — as measured at escher-0.3.0/chunks/2026-11-04-dioxus-runtime-probe/report.md
@carried n=2 label=test-plan/test-scope-summary fact=fct-poisqw4h
why: the snapshot the stand reads must stay identical under the buffered replay, and the host's paged read must still log no id and no name, as the scope keeps the snapshot unchanged and the buffer out of every log.
quote: `stand_snapshot` reads the flight booker's `flight-type` as `ComboBox` "Flight type", value `one-way`, holding its two `ListBoxOption`s, one selected, each at the select's client rect (`the_flight_type_select_holds_its_two_options`, tests/blitz-tests/tests/stand_snapshot.rs:122-170); seven_guis' `host_log` reads a paged part of a served Cells screen through the `escher-session` host's own sink — `serve cells` at `RUST_LOG=trace` in `hl-cells`, a sentinel committed into a part-2 cell first, then `snapshot --part 2`, 0 id and 0 name needles under both builds
@carried n=2 run=2 label=test-plan fact=fct-jydhmppz
quote: **dioxus-native-dom:** unit tests of the snapshot builder, its text form, its parts and its diff, the stable element id, the actionable-key check, the bridge, the mutation writer and the document's events
@carried n=2 run=3 label=test-plan fact=fct-i47d76zr
quote: the bridge tests sit in a `cfg(test)` child module of `dioxus_document` (`bridge_tests`, file `dioxus_document_tests.rs`) because `DioxusEventHandler`'s two fields are private to that module and it has no constructor — they build one over `doc.vdom` and `doc.vdom_state` and call `handle_event` with an `EventState` they keep
@carried n=2 run=4 label=test-plan fact=fct-i47d76zr
quote: assert that a re-render-removed node no longer resolves, reads no id and stands in no pair of `element_ids()`
@carried n=3 label=test-plan/test-scope-summary fact=fct-vwiy3q7s
why: the stand's lint reading at boot must stay at its measured value over the buffered render, a stand_* reading the chunk re-asserts.
quote: `stand_lint` pins the stand at `[0, 0, 0, 0]` at boot and, with `stand_act_popup`, reads an open list's rows no overlap (§1 above) — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md
@carried n=4 label=test-plan/test-scope-summary fact=fct-rovrmg6e
why: the two log checks must still read 0 needles at trace, the witness that the buffer's ids, tag and attribute names and text never reach a log line.
quote: `host_log` `a_select_list_opened_and_chosen_through_the_host_logs_no_option_at_trace`, in `hl-popup`, `#[cfg(unix)]` as every host-driving test there (examples/seven_guis/tests/host_log.rs:282-393); `mcp_log` `a_select_list_opened_and_chosen_through_the_role_logs_no_option_at_trace`, which records nothing and holds no state directory (examples/seven_guis/tests/mcp_log.rs:68-113) — the two log checks reading 0 needles at `trace`
@carried n=4 run=2 label=test-plan fact=fct-xxpbx2ay
quote: on both binaries stderr holds no stable id and no accessible name at the default level, `info`, `debug` and `trace`
@carried n=5 label=test-plan/test-scope-summary fact=fct-odg47pg2
why: the keyboard walk drives press through the harness into the poll the chunk buffers, in both layout modes, and must stay green beside the inverted pins that take the same key path.
quote: `the_keyboard_walks_to_a_cell_and_commits_a_value_read_in_the_first_part`, in both layout modes: `press` walks and opens the editor, `type` supplies the characters and `press` commits, the driver's twelve key names holding no character (tests/blitz-tests/tests/stand_cells_served.rs:179-214)
@carried n=5 run=2 label=test-plan fact=fct-e36kow75
quote: pump polls the document with no waker context and resolves at the harness time; dispatch and dispatch_recorded do not pump
@carried n=5 run=3 label=test-plan fact=fct-e36kow75
quote: `settle` repeats that pass and keeps poll's answer — a pass is quiet when poll answered `false`, the hover node did not change and no load finished in it
@carried n=5 run=4 label=test-plan fact=fct-dncmjjxa
quote: **Input helpers:** click, click_at, mouse_down_at, mouse_up_at, move_mouse_to, drag, tap, tap_at, touch_down, touch_move, touch_up, wheel_at, press, press_with, type_text, apple_keybinding, scroll_into_view, ime — each pumps after dispatch (packages/blitz-test-harness/src/input.rs:96-252)
@carried n=5 run=5 label=test-plan fact=fct-542cugs7
quote: **Inspection helpers:** query, node, query_all, layout_rect, layout_rect_of, center_of, text_content, attr, hit, hit_node, focused, hovered, dom_string (packages/blitz-test-harness/src/inspect.rs:26-131)
@binding n=1
**test-scope-summary (stand witnesses) ↔ the agent-run proof (body §3)**: the stand_* checks the chunk re-asserts must keep their counts (`stand_cells_served` 5, `stand_load_page` 3) and the `run stand` / `run all` readings 237 · 0 · 7 and 743 · 0 · 12 must move only by the inverted pins' outcome, never their count, unless the measurement says otherwise (test-plan/test-scope-summary fct-tz35iqmq) (test-plan fct-glytb3w3)
@binding n=2
**test-scope-summary (log needle checks) ↔ the stand log format (body §3)**: the buffered render must add no log line or field, so `host_log` and `mcp_log` must still read 0 id, name and option needles at `trace` under both builds (test-plan/test-scope-summary fct-rovrmg6e) (test-plan/test-scope-summary fct-poisqw4h) (test-plan fct-xxpbx2ay)
@binding n=3
**test-scope-summary (the keyboard walk) ↔ the harness pump and settle contract (body §3)**: the inverted pins and the stand's keyboard walk reach the buffered poll through `press` and `pump` with no waker context, so the buffered poll must return the same answer `settle` keeps, and focus must be read through `focused` (test-plan/test-scope-summary fct-odg47pg2) (test-plan fct-e36kow75) (test-plan fct-542cugs7)
@binding n=4
**test-scope-summary (snapshot witness) ↔ dioxus-native-dom unit tests (body §1, §3)**: the snapshot, element ids and removal mapping must read identically after the buffered replay, and a buffer living on `vdom_state` must leave the bridge tests building over `doc.vdom` and `doc.vdom_state` intact (test-plan/test-scope-summary fct-poisqw4h) (test-plan fct-jydhmppz) (test-plan fct-i47d76zr)
