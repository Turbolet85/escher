@carried n=1 label=architecture/conventions fact=fct-bhxvo7z2
why: The new record of which (element, event) handlers a run reached must hold on every app, registry or not, so it must not live behind the registry, while the coverage read over the UI graph sits under the accessibility gate the graph module carries.
quote: dioxus-native-dom's `graph` module (`DioxusDocument::graph`, `UiGraph`) is gated by `accessibility`, beside the snapshot model it reads, and its `signal_registry` module (`use_graph_state`, `DeclaredWrite`, `SignalRegistry`) is not, so an app registers state whatever its features
@carried n=2 label=architecture/conventions fact=fct-bhxvo7z2
why: Any coverage verb added to execute.rs and mcp.rs must run under the hand-built ungated `command` and `tool_call` spans, with no `#[instrument]` and no tracing feature gate.
quote: escher-driver, since 2026-10-07-driver-command-spans, takes `tracing` with no feature and builds its spans by hand with `tracing::info_span!` — the `command` span and, since 2026-10-10-mcp-surface, the `tool_call` span, the parent of `command` on an MCP tool call (packages/escher-driver/src/mcp.rs:118-121) — no `#[instrument]`, no `cfg(feature = "tracing")` gate
@carried n=2 run=2 label=architecture fact=fct-6yr3qnpo
quote: escher's own two crates take `tracing` ungated — escher-telemetry emits its startup and panic events ungated and escher-driver builds its spans by hand, with no `#[instrument]`
@carried n=3 label=architecture/conventions fact=fct-knw5nmxh
why: The exercised record is written on the bridge's event path in dioxus_document.rs, so its write must keep the document's RefCell unborrowed while the vdom renders and borrow it only where the buffer is replayed.
quote: on the Dioxus bridge, by rendering the vdom into a crate-private `MutationBuffer` with the document's `RefCell` unborrowed, then borrowing the document once and replaying the buffer through one `MutationWriter` (packages/dioxus-native-dom/src/dioxus_document.rs:167-172; packages/dioxus-native-dom/src/dioxus_document.rs:315-320)
@carried n=3 run=2 label=architecture fact=fct-ps4gqbg4
quote: borrow conflicts worked around by taking a child list or layout out and restoring it
@carried n=4 label=architecture/conventions fact=fct-zd5gb466
why: The handler-coverage check v030-07's integration acceptance owes holds a Session on the stand, so it must share through session_common and be recorded in this list of Session-holding checks.
quote: the checks that hold a `Session` — the session checks, the settle check `stand_settle`, the sixteen driver-action checks `stand_act_*`, since 2026-10-10-mcp-surface `stand_mcp_tools`, since 2026-10-12-headless-screenshot, `stand_screenshot_repeat` and, since 2026-10-14-stand-requirement-sweep, the driver sweep `stand_requirement_sweep`, the served-page checks `stand_cells_served` and `stand_load_page` and `stand_graph`, which also reads `common`'s `GRAPH_DRIVES`, and the actions check `stand_actions`, which also reads `common`'s `ACTIONABLE_AT_BOOT` — keep what they share in a second such module, `tests/blitz-tests/tests/session_common/mod.rs`, read by `mod session_common;` (tests/blitz-tests/tests/session_common/mod.rs:1-7)
@carried n=5 label=architecture/conventions fact=fct-iemqu3j3
why: The owed replay test that v030-06's verified state waits on lands in recording_tests.rs, compiled only on unix, so its witness is the linux test leg and the macos matrix, never the windows job.
quote: `recording::tests` in `recording_tests.rs` (`#[cfg(all(test, unix))]`) and `recording::off_unix_tests` in `recording_off_unix_tests.rs` (`#[cfg(all(test, not(unix)))]`, which the windows job alone runs)
@carried n=5 run=2 label=architecture/conventions fact=fct-zd5gb466
quote: eight unit-test modules sit in files of their own, each a `#[cfg(test)]` child declared through `#[path]` at its parent
@carried n=6 label=architecture/conventions fact=fct-qr7xkzw2
why: A new verb adds a row to the MCP listing test and any sweep row added to stand_requirement_sweep.rs must be asserted through the private track_caller helpers, their messages naming the row and index only.
quote: escher-driver's MCP listing test and blitz-tests' 0.2.0 requirement-sweep test keep their outer assertions and row walk in the test function and assert each row through private `#[track_caller]` helpers whose messages name the row and index only (packages/escher-driver/src/mcp_tests.rs:421-502; tests/blitz-tests/tests/stand_requirement_sweep.rs:1050-1093 — as measured at escher-0.3.0/chunks/2026-10-26-codebase-health/report.md)
@carried n=7 label=architecture/conventions fact=fct-m3flvoi7
why: Every stand task registers its signals, so a stand-only test cannot witness the exercised record holding on an app with no registry; the plan must add a registry-less app or fixture to witness it.
quote: seven_guis' tasks register each signal through `tasks::use_graph_signal(name, writers, init)`, wrapping `use_signal` and `use_graph_state` (examples/seven_guis/src/tasks/mod.rs:4-15), `use_signal` left to the stand shell's `active` and `stand.rs`'s `linked`
@binding n=1
**conv-feature-gating ↔ contract-dioxus-dom-bridge**: the graph is read under `accessibility` while the registry and `handle_event`'s dispatch setting stand ungated and act only where a registry stands; the new exercised record must be written on the ungated event path for every app, registry or not, and read beside the gated graph without changing its shape (architecture/conventions fct-bhxvo7z2) (architecture fct-srb7slak).
@binding n=2
**conv-feature-gating ↔ contract-driver-session**: a coverage verb, if added, rides the hand-built ungated `command` and `tool_call` spans, moves the schema from fourteen verbs to fifteen, and neither its span nor any log carries the coverage list's ids, component or event names (architecture/conventions fct-bhxvo7z2) (architecture fct-ksgwc2yv) (architecture fct-tev7ieys).
@binding n=3
**conv-unsafe-and-borrowing ↔ contract-dioxus-dom-bridge**: the (element, event) record written where DOM events reach the vdom through `data-dioxus-id` must keep the buffered render's single borrow of the document (architecture/conventions fct-knw5nmxh) (architecture fct-srb7slak).
@binding n=4
**conv-tests ↔ contract-headless-stand**: the handler-coverage stand check and the replay test record and replay on a fresh session of the same served stand app, the stand check sharing through `session_common` and entered in its list of Session-holding checks (architecture/conventions fct-zd5gb466) (architecture fct-tewgiusr).
@binding n=5
**conv-tests ↔ dec-driver-session**: the owed replay test in the unix-only `recording::tests` keeps a check through the recording medium and, compared across linux and macOS, holds no `screenshot` step (architecture/conventions fct-iemqu3j3) (architecture fct-2okzb4io) (architecture fct-tk4aeobh).
@binding n=6
**conv-tests ↔ contract-driver-session**: a fifteenth verb's MCP listing row and any added sweep row are asserted through private `#[track_caller]` helpers naming row and index only (architecture/conventions fct-qr7xkzw2) (architecture fct-ksgwc2yv).
@binding n=7
**conv-apps-and-examples ↔ contract-dioxus-dom-bridge**: the stand registers every task signal through `use_graph_signal`, so the registry-less case of the exercised record is witnessed on an app that registers no state (architecture/conventions fct-m3flvoi7) (architecture fct-srb7slak).
