@apply_row n=1 ref_file=fanout-a11y-plan--cascade-g1.md ref_n=5 v=applied edit=E1
@apply_row n=2 ref_file=fanout-a11y-plan--cascade-g1.md ref_n=6 v=applied edit=E2
@apply_row n=3 ref_file=fanout-a11y-plan--cascade-g1.md ref_n=7 v=applied edit=E3
@apply_row n=4 ref_file=fanout-a11y-plan--cascade-g1.md ref_n=8 v=applied edit=E4
@apply_row n=5 ref_file=fanout-a11y-plan--cascade-g1.md ref_n=1 v=applied edit=E5
@apply_row n=6 ref_file=fanout-a11y-plan--cascade-g1.md ref_n=2 v=applied edit=E6
@apply_row n=7 ref_file=fanout-a11y-plan--cascade-g1.md ref_n=3 v=applied edit=E7
@apply_row n=8 ref_file=fanout-a11y-plan--cascade-g1.md ref_n=4 v=applied edit=E8
@apply_row n=9 ref_file=fanout-a11y-plan--cascade-g2.md ref_n=1 v=applied edit=E9
@fact id=fct-zyrpeqlm expect_rev=2
old: owned by residual `res-llkgqygv`, an active
new: owned by residual `res-llkgqygv`, re-carried at 0.3.0 (target: next), an active

@fact id=fct-cixbakze expect_rev=2
old: owned by residual `res-llkgqygv`, an active
new: owned by residual `res-llkgqygv`, re-carried at 0.3.0 (target: next), an active

@fact id=fct-5i3imozy expect_rev=2
old: owned by residual `res-wvatzhpe`.
new: owned by residual `res-wvatzhpe`, re-carried at 0.3.0 (target: next).

@fact id=fct-kbjv4usk expect_rev=2
old: owned by residual `res-llkgqygv`, an active
new: owned by residual `res-llkgqygv`, re-carried at 0.3.0 (target: next), an active

@fact id=fct-gmhtpdel expect_rev=2
old: panics with `RefCell already borrowed`: `DioxusDocument::poll` holds `inner.borrow_mut()` across `vdom.render_immediate`, dioxus-core 0.7.10 first polls the task there through `handle_task_wakeup`, and its `set_focus`, borrowing through `doc_mut`, panics, while the same handler calling `set_focus` with no task moves focus and does not panic — measured headlessly in both layout modes by `dioxus_runtime_probe`'s two `should_panic` pins, their precondition asserted first (the handle held, the key target focused by Tab), and the windowed path read and not run (packages/dioxus-native-dom/src/dioxus_document.rs:295-297; packages/dioxus-native-dom/src/events.rs:162; packages/dioxus-native-dom/src/events.rs:295; tests/blitz-tests/tests/dioxus_runtime_probe.rs:556-631); so Cells calls its handles synchronously, each acting at the call; the hazard's fix routed to the route entry «MountedData buffered render» — as measured at escher-0.3.0/chunks/2026-11-04-dioxus-runtime-probe/report.md
new: acts and does not panic, on every Dioxus document: `DioxusDocument::initial_build` and `Document::poll` render into a mutation buffer (`buffered_mutations`, a `MutationBuffer`) with the document unborrowed, then take `inner.borrow_mut()` once and replay the buffer through one `MutationWriter`, so the task dioxus-core 0.7.10 first polls inside `render_immediate` reaches the document through `doc_mut`, as the same handler calling `set_focus` with no task does — measured headlessly in both layout modes by `dioxus_runtime_probe`'s two pins `mounted_data_in_a_task_a_key_handler_spawned_acts_{non_incremental,incremental}`, their precondition asserted first (the handle held, the key target focused by Tab), each asserting after the spawned task's `set_focus` that the engine's focus, the snapshot's one `focused` node and the tree's focus name `focus-target`, both reading FAILED with the borrowed render restored, and the windowed path read and not run (packages/dioxus-native-dom/src/dioxus_document.rs:295-297; packages/dioxus-native-dom/src/events.rs:162; packages/dioxus-native-dom/src/events.rs:295; tests/blitz-tests/tests/dioxus_runtime_probe.rs:556-631); Cells calls its handles synchronously, each acting at the call, no longer needed for the hazard and kept — as measured at escher-0.3.0/chunks/2026-11-07-mounteddata-buffered-render/report.md

@fact after=fct-gmhtpdel as=n1
- The limit, by construction: a task polled during a Dioxus render acts on the document as the previous render left it, so a node created in the same render is not yet in it and `set_focus_to` returns `false` for it; the probe's pins act on nodes mounted by an earlier render; the limit is owned by an open `residual` row (origin 2026-11-07-mounteddata-buffered-render, target: next) — as measured at escher-0.3.0/chunks/2026-11-07-mounteddata-buffered-render/report.md

@fact id=fct-p7eqwwyx expect_rev=2
old: is owned by residual `res-llkgqygv`, the witness
new: is owned by residual `res-llkgqygv`, re-carried at 0.3.0 (target: next), the witness

@fact id=fct-ngdl2hcv expect_rev=2
old: is owned by residual `res-llkgqygv`, which this version
new: is owned by residual `res-llkgqygv`, re-carried at 0.3.0 (target: next), which this version

@fact id=fct-pofw2owv expect_rev=2
old: stay out by residual `res-wvatzhpe`, and
new: stay out by residual `res-wvatzhpe`, re-carried at 0.3.0 (target: next), and
@pattern id=a11y-01
fixed: a key handler that writes a signal panics
@pattern id=a11y-02
fixed: two `should_panic` pins
@pattern id=a11y-03
fixed: the hazard's fix routed to the route entry
