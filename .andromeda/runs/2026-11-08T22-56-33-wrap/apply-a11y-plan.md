@apply_row n=1 ref_file=fanout-a11y-plan--cascade-g1.md ref_n=1 v=applied edit=E1
@apply_row n=2 ref_file=fanout-a11y-plan--cascade-g1.md ref_n=3 v=applied edit=E2
@apply_row n=3 ref_file=fanout-a11y-plan--cascade-g1.md ref_n=4 v=applied edit=E3
@apply_row n=4 ref_file=fanout-a11y-plan--cascade-g2.md ref_n=1 v=applied edit=E4
@fact id=fct-d5bs73pt expect_rev=3
old: while its `signal_registry` module (`use_graph_state`, `DeclaredWrite`, `SignalRegistry`) is not gated;
new: while its `signal_registry` module (`use_graph_state`, `DeclaredWrite`, `SignalRegistry`), its `reached` module (`ReachedHandler`, re-exported from the crate root, `DioxusDocument::record_reached`, `DioxusDocument::reached_handlers`) and the crate-private listener reader `element_listener_ids` the record reads are not gated, `element_facts` staying behind the feature;
old: packages/dioxus-native-dom/src/lib.rs:38)
new: packages/dioxus-native-dom/src/lib.rs:38; packages/dioxus-native-dom/src/lib.rs:28; packages/dioxus-native-dom/src/lib.rs:45; packages/dioxus-native-dom/src/reached.rs:85-105; packages/dioxus-native-dom/src/element_id.rs:304-320)

@fact after=fct-wfj657us as=n1
- Beside the registry arm, `DioxusEventHandler::handle_event` writes an ungated reached record once `DioxusDocument::record_reached` has switched it on (a crate-private `ReachedRecord` context at `ScopeId::ROOT`, a second call keeping the one record): after the event-kind and dioxus-id early returns and before `runtime.handle_event`, each node of the chain whose current listeners hold the event's name inserts its stable id with that name, and `DioxusDocument::reached_handlers` reads them back in first-reached order, deduplicated by (id, event), empty when the record was never switched on; the id is resolved before the run, so a handler whose own run removes its element still counts; every other document pays one `has_context` lookup per dispatch (packages/dioxus-native-dom/src/dioxus_document.rs:448-449; packages/dioxus-native-dom/src/dioxus_document.rs:452-460; packages/dioxus-native-dom/src/reached.rs:85-95; packages/dioxus-native-dom/src/reached.rs:97-105) — as measured at escher-0.3.0/chunks/2026-11-08-handler-coverage/report.md

@fact after=fct-5o7fbnn6 as=n2
- `stand_handler_coverage` (8 tests) asserts in both layout modes, through `escher_driver::handler_coverage`, that a refused step exercises nothing and a keyboard activation exercises its control's click (`a_refused_step_exercises_nothing_and_a_keyboard_activation_its_click`); it runs in the `stand-repeat` leg by its `stand_` prefix and is in no `a11y` leg list (tests/blitz-tests/tests/stand_handler_coverage.rs:298-330) — as measured at escher-0.3.0/chunks/2026-11-08-handler-coverage/report.md

@fact after=fct-gg5bgkx2 as=n3
- `escher_driver::handler_coverage` is one more reader of `DioxusDocument::graph`: per check it reads the graph at boot and after each replayed step, `mounted` left out, deduplicated by (id, event), and lists as unexercised the graph's handlers that no check reached; its answer holds ids, owner ids, event names and two counts (`checks`, `held`) only, no role or name among them (packages/escher-driver/src/coverage.rs:44-62; packages/escher-driver/src/coverage.rs:64-102; packages/escher-driver/src/json.rs:419-442) — as measured at escher-0.3.0/chunks/2026-11-08-handler-coverage/report.md
