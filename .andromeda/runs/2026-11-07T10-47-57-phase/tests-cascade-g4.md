@carried n=1 label=test-plan/test-strategy fact=fct-yivrglnx
why: the inverted pins and the stand checks must reach the buffered poll through the harness's pump, settle, input synthesis and inspection, and the buffered poll must return the answer settle reads.
quote: the harness wraps any blitz_dom Document, such as HtmlDocument or DioxusDocument, with deterministic construction defaults, a pump/tick loop, `settle` (that pass repeated until no work is due, returning `Settled` or `NotSettled` naming the `Busy` class), inspection helpers and input synthesis (packages/blitz-test-harness/src/lib.rs:1-19)
@carried n=1 run=2 label=test-plan fact=fct-ky3gzvfl
quote: deterministic pseudo-random template swaps (an LCG) run 1000 steps to shake out slab/ElementId reuse bugs (tests/blitz-tests/tests/stale_node_mapping.rs:176-217)
@carried n=2 label=test-plan/test-strategy fact=fct-5d4ttzku
why: stand_replay must stay green over the buffered render, each recorded step's acceptance and answer unchanged.
quote: run as an ordinary `cargo test` target (`stand_replay`) through `Check::replay` — every step re-validated through `Session::run`, red at the first step whose acceptance or kept answer differs, the answers compared as JSON values
@carried n=3 label=test-plan/integration-test-strategy fact=fct-322qvjpg
why: incremental_oracle, which the chunk re-reads, must stay green as the witness that the buffer leaves incremental and non-incremental layout identical.
quote: `incremental_oracle`, green beside it, is the layout-identity witness (§2 Differential oracle)
@carried n=3 run=2 label=test-plan fact=fct-hzkxjxb5
quote: the same fixture and mutation sequence runs in an incremental and a non-incremental document, comparing every node's `final_layout()`, `layout_children` and paint tree after each step and asserting no damage remains
@carried n=4 label=test-plan/integration-test-strategy fact=fct-t7rg232i
why: the stand session checks must read the same settled or NotSettled outcome and the same diffs in both layout modes over the buffered poll.
quote: a step run through `Session::act` returns settled — the Timer's delayed update present, an idle step changing nothing, a write made while mounting present — or `SessionError::NotSettled(Busy::Render)` at the bound with the instance still readable, in both layout modes
@carried n=4 run=2 label=test-plan fact=fct-gq2zcc44
quote: a call run through `Session::run` returns after settle with the diff of the snapshots taken before and after it
@carried n=5 label=test-plan/integration-test-strategy fact=fct-adkwyvky
why: the buffer holds ids, tag and attribute names and text, and must never write any of them to stdout, stderr or the state directory, nor add a log line.
quote: `stand_session_quiet` proves the session library and the harness path write nothing of the screen — typed text, a stable id, an accessible name — to stdout, to stderr or into the state directory
@carried n=5 run=2 label=test-plan fact=fct-lm4npbmj
quote: each call run through `Session::run` leaves exactly one `escher_driver` line holding fixed words and counts only, none of the ids, names, values or typed text it handled, and at the default level no driver line
@carried n=6 label=test-plan/integration-test-strategy fact=fct-6jloyoi7
why: stand_act_select must stay green in both layout modes over the buffered render, as part of the stand_* witness.
quote: `stand_act_select` drives a `multiple` select through the driver, in both layout modes — its name, role and rows read each with its own bounds, an accepted row click choosing the row
@carried n=7 label=test-plan/integration-test-strategy fact=fct-gkuwtzih
why: stand_act_popup must stay green in both layout modes over the buffered render, as part of the stand_* witness.
quote: `stand_act_popup` (12) drives the list through a held session, in both layout modes — a `click` opening it and a `click` on a row choosing
@carried n=8 label=test-plan/integration-test-strategy fact=fct-xs6og5lj
why: the buffer must replay listener creation and removal into MutationWriter so the listener attributes read exactly as the direct write left them, link_2 staying green in both layout modes.
quote: dioxus-core 0.7.10 reaches `remove_event_listener` only when a listener leaves an element's attribute list (a spread, the `toggled_listener` page), and it lowers the per-kind count alone (packages/dioxus-native-dom/src/mutation_writer.rs:355-360), so a listener removed on a re-render leaves its attribute and the mark standing while a click no longer acts, and a `mounted` listener writes none; the vdom walk's dynamic attributes, `VNode.dynamic_attrs` with `VNode::mounted_dynamic_attribute`, read each element's current listeners exactly, `mounted` included — pinned by `dioxus_runtime_probe`'s `link_2_a_removed_listeners_attribute_stays`, in both layout modes
@carried n=8 run=2 label=test-plan fact=fct-5mwtk4y2
quote: `keyed_nodes_do_not_crash` runs a real `VirtualDom` against a `DioxusDocument` through 100 inserts, then checks that `<main>` has 100 children (packages/dioxus-native-dom/src/dioxus_document.rs:467-483)
@carried n=9 label=test-plan/integration-test-strategy fact=fct-3vcgyjbv
why: links_3_and_4 must stay green in both layout modes, the writer's click changing exactly the readers' nodes through the buffered render.
quote: the writer's click changes exactly the readers' nodes in the step's snapshot diff — `dioxus_runtime_probe`'s `links_3_and_4_a_held_signals_subscribers_are_its_readers` over the `shared_state` page (`Writer`, `FirstReader`, `SecondReader`), in both layout modes
@carried n=10 label=test-plan/e2e-test-strategy fact=fct-kgita7a3
why: this is the hazard the chunk removes: poll must no longer hold the document's borrow across render_immediate, and the pin is inverted here.
quote: a `MountedData` call in a task spawned from a key handler panics with `RefCell already borrowed` at `events.rs:162`: `DioxusDocument::poll` holds `inner.borrow_mut()` (`dioxus_document.rs:295`) across `vdom.render_immediate` (`:297`), where dioxus-core 0.7.10 first polls the queued task through `handle_task_wakeup` (`virtual_dom.rs:611`), and the task's `set_focus` (`events.rs:295`) borrows through `doc_mut` and panics, in both layout modes, pinned by `dioxus_runtime_probe` (below); its fix is routed to the route entry «MountedData buffered render», which ports upstream `286874cc1`'s buffered render onto the 0.7 bridge and inverts the pin
@carried n=11 label=test-plan/e2e-test-strategy fact=fct-mealwlke
why: the two should_panic pins must become tests that focus moves with no panic, one per layout mode, hazard_ready still asserted first, and the no-task sibling must stay green.
quote: `mounted_data_in_a_task_a_key_handler_spawned_panics_non_incremental` and `mounted_data_in_a_task_a_key_handler_spawned_panics_incremental`, two `should_panic` tests, one per layout mode, pin the panic, their precondition asserted first by `hazard_ready` (the handle held, the key target focused by Tab); beside them `mounted_data_asked_at_once_in_a_key_handler_acts`, in both layout modes: the same handler calling `set_focus` with no task moves focus and does not panic
@binding n=1
**e2e-test-strategy ↔ integration-test-strategy**: when the hazard pins are inverted (test-plan/e2e-test-strategy fct-mealwlke), the hazard's mechanism (test-plan/e2e-test-strategy fct-kgita7a3) must be re-stated in the plan as fixed. The dioxus_runtime_probe tests that stay, link_2 (test-plan/integration-test-strategy fct-xs6og5lj) and links_3_and_4 (test-plan/integration-test-strategy fct-3vcgyjbv), must stay green in both layout modes beside them.
@binding n=2
**integration-test-strategy ↔ test-strategy**: the buffered poll must keep the answer that the harness's settle reads (test-plan/test-strategy fct-yivrglnx), so that the stand session checks keep their settled readings and diffs (test-plan/integration-test-strategy fct-t7rg232i) and stand_replay keeps every recorded answer (test-plan/test-strategy fct-5d4ttzku). The layout identity is witnessed by incremental_oracle (test-plan/integration-test-strategy fct-322qvjpg) and element-id reuse by stale_node_mapping (test-plan fct-ky3gzvfl).
@binding n=3
**integration-test-strategy ↔ the scope's boundary**: the mutations the buffer holds must not reach any output stream or log line. stand_session_quiet and the driver's one-line rule stand witness to this (test-plan/integration-test-strategy fct-adkwyvky; test-plan fct-lm4npbmj).
