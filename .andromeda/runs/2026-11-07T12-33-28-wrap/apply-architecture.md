@apply_row n=1 ref_file=fanout-architecture--cascade-g1.md ref_n=1 v=applied edit=E1
@apply_row n=2 ref_file=fanout-architecture--cascade-g1.md ref_n=2 v=applied edit=E2
@apply_row n=3 ref_file=fanout-architecture--cascade-g1.md ref_n=3 v=applied edit=E3
@apply_row n=4 ref_file=fanout-architecture--cascade-g1.md ref_n=4 v=applied edit=E4
@apply_row n=5 ref_file=fanout-architecture--cascade-g1.md ref_n=5 v=applied edit=E5
@apply_row n=6 ref_file=fanout-architecture--cascade-g1.md ref_n=6 v=applied edit=E6
@apply_row n=7 ref_file=fanout-architecture--cascade-g1.md ref_n=7 v=applied edit=E7
@apply_row n=8 ref_file=fanout-architecture--cascade-g2.md ref_n=1 v=applied edit=E8
@apply_row n=9 ref_file=fanout-architecture--cascade-g2.md ref_n=3 v=applied edit=E9
@apply_row n=10 ref_file=fanout-architecture--cascade-g2.md ref_n=4 v=applied edit=E10
@apply_row n=11 ref_file=fanout-architecture--cascade-g2.md ref_n=5 v=applied edit=E11
@apply_row n=12 ref_file=fanout-architecture--cascade-g3.md ref_n=1 v=applied edit=E12
@apply_row n=13 ref_file=fanout-architecture--cascade-g3.md ref_n=2 v=applied edit=E13
@apply_row n=14 ref_file=fanout-architecture--cascade-g3.md ref_n=3 v=applied edit=E14
@fact id=fct-apdputpm expect_rev=2
old: A measured hazard of a task spawned from a key handler, no engine change: `DioxusDocument::poll` holds the document's `RefCell` across `render_immediate`, where dioxus-core 0.7.10 first polls a queued task through `handle_task_wakeup`, so such a task that writes a signal and calls `set_focus` borrows through `doc_mut` and panics (`RefCell already borrowed`)
new: The bridge's buffered render, a direct engine change for every Dioxus document — the stand, the windowed `dioxus-native` launch, any app on the bridge — ratified: `DioxusDocument::initial_build` and `Document::poll` render through the crate-private `MutationBuffer` (`buffered_mutations`) with the document unborrowed, then take `inner.borrow_mut()` once and replay through one `MutationWriter`, so a task dioxus-core 0.7.10 polls inside `render_immediate` that writes a signal and calls `set_focus` reaches the document through `MountedData` and `doc_mut` with no `RefCell already borrowed` panic
old: events.rs:295), so Cells calls its handles synchronously. Any app meets it; measured headlessly through the harness's pump in both layout modes and pinned by `dioxus_runtime_probe` (test-plan §6 E2E Test Strategy), the windowed shell's same lines read and not run (as measured at escher-0.3.0/chunks/2026-11-04-dioxus-runtime-probe/report.md); its fix, a port of upstream `286874cc1`'s buffered render onto the Dioxus 0.7 bridge, is owned by the route entry «MountedData buffered render».
new: events.rs:295); a port of upstream `286874cc1`'s buffered render onto the Dioxus 0.7 bridge, flagged not upstreamable — upstream holds it in its 0.8 form — and nothing sent; Cells still calls its handles synchronously, kept, the hazard not needing it. Its witness on Dioxus 0.7 is `dioxus_runtime_probe`'s two `mounted_data_in_a_task_a_key_handler_spawned_acts_{non_incremental,incremental}` tests, measured headlessly through the harness's pump in both layout modes (test-plan §6 E2E Test Strategy), the windowed shell's same lines read and not run (as measured at escher-0.3.0/chunks/2026-11-07-mounteddata-buffered-render/report.md); its merge surface — `dioxus_document.rs`, `mutation_buffer.rs` (which upstream's 0.8 file replaces whole at the 0.8 move) and `lib.rs` — is owned by residual `res-3v42legm`, absorbed into `v030-03`, its re-read owned by the CARRY on «Upstream sync ahead of determinism and exploration».

@fact after=fct-apdputpm as=n1
- The buffered render's limit, by construction: a task polled during render acts on the document as the previous render left it, so a node created in the same render is not yet in it and `set_focus_to` returns `false` for it; the witness and the probe's pins act on nodes an earlier render mounted; owned by an open `residual` row, origin 2026-11-07-mounteddata-buffered-render (target: next).

@fact after=@n1 as=n2
- On dioxus-core 0.7.10 a task spawned from a component body during `render_immediate`'s `Work::RerunScope` arm is first polled by the next `poll`'s `wait_for_work` → `process_events` → `poll_tasks`, before the bridge takes any borrow, no `queue_events` following a `RerunScope`; a task spawned from an event handler that also dirties a scope is first polled inside `render_immediate`'s `Work::PollTask` arm, the one path on which a borrowed render meets the borrow (dioxus-core 0.7.10 `src/virtual_dom.rs:445-519` and `src/virtual_dom.rs:610-620`, read in the registry copy, and measured — as measured at escher-0.3.0/chunks/2026-11-07-mounteddata-buffered-render/evidence/mutation-control.md).

@fact id=fct-j4ls6r2g expect_rev=2
old: is owned by residual `res-3v42legm`, re-read row by row, each row given a named outcome and nothing sent (as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/evidence/readings.md), its owner «Upstream sync ahead of determinism and exploration».
new: is owned by residual `res-3v42legm`, absorbed into `v030-03`, its re-read owned by the CARRY on «Upstream sync ahead of determinism and exploration», re-read row by row, each row given a named outcome and nothing sent (as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/evidence/readings.md).

@fact id=fct-hff77mh6 expect_rev=2
old: owned by the same residual, `res-3v42legm`, whose owner is «Upstream sync ahead of determinism and exploration».
new: owned by the same residual, `res-3v42legm`, absorbed into `v030-03`, its re-read owned by the CARRY on «Upstream sync ahead of determinism and exploration».

@fact id=fct-2fe67avo expect_rev=2
old: is owned by the same residual, `res-3v42legm`, whose owner is «Upstream sync ahead of determinism and exploration».
new: is owned by the same residual, `res-3v42legm`, absorbed into `v030-03`, its re-read owned by the CARRY on «Upstream sync ahead of determinism and exploration».

@fact id=fct-knw5nmxh expect_rev=1
old: packages/blitz-dom/src/layout/inline/layout.rs:245-251); let-chains
new: packages/blitz-dom/src/layout/inline/layout.rs:245-251), and, on the Dioxus bridge, by rendering the vdom into a crate-private `MutationBuffer` with the document's `RefCell` unborrowed, then borrowing the document once and replaying the buffer through one `MutationWriter` (packages/dioxus-native-dom/src/dioxus_document.rs:166-171; packages/dioxus-native-dom/src/dioxus_document.rs:302-307); let-chains

@fact id=fct-ivrm3he5 expect_rev=2
old: that witness owned by residual `res-llkgqygv`, at an active assistive technology
new: that witness owned by residual `res-llkgqygv`, re-carried at 0.3.0 (target: next), at an active assistive technology

@fact after=fct-kllkxz3d as=n3
- `DioxusDocument` holds `pub(crate) buffered_mutations: MutationBuffer`, set in `DioxusDocument::new`, its only constructor (packages/dioxus-native-dom/src/dioxus_document.rs:90-91; packages/dioxus-native-dom/src/dioxus_document.rs:155); `initial_build` and `Document::poll`, their signatures unmoved, render into it with the document unborrowed (`vdom.rebuild` / `vdom.render_immediate`, never into the borrowed document), then take `inner.borrow_mut()` once, build one `MutationWriter`, replay, and drop the writer and the borrow before `flush_queued_mounted_events`, so user code the vdom runs during a render reaches the document through `MountedData` with no `RefCell already borrowed` panic (packages/dioxus-native-dom/src/dioxus_document.rs:166-171; packages/dioxus-native-dom/src/dioxus_document.rs:302-307).

@fact id=fct-67g4co3e expect_rev=1
old: `create_event_listener` and `remove_event_listener` have no workspace caller, dioxus-core calling them.
new: dioxus-core calls `MutationBuffer`'s `create_event_listener` and `remove_event_listener`, which record the call, and `MutationWriter`'s two have one workspace caller, `MutationBuffer::replay`, which drains the recorded calls into it in order (packages/dioxus-native-dom/src/mutation_buffer.rs:97-140; packages/dioxus-native-dom/src/mutation_buffer.rs:213-216; packages/dioxus-native-dom/src/mutation_buffer.rs:218-221).

@fact id=fct-4ncs7geh expect_rev=3
old: is to be ported onto the Dioxus 0.7 bridge by the route entry «MountedData buffered render», the 0.8 move itself still waiting
new: is ported onto the Dioxus 0.7 bridge (§Established Decisions → [Dioxus integration]), the 0.8 move itself still waiting

@fact id=fct-kaaobbkf expect_rev=1
old: events, mutation_writer, snapshot (crate-private
new: events, mutation_writer, mutation_buffer (crate-private, outside every `accessibility` gate, no `pub use`: the `MutationBuffer` that `DioxusDocument::initial_build` and `Document::poll` render into with the document unborrowed, then replay through one `MutationWriter`), snapshot (crate-private

@fact id=fct-ktfizprz expect_rev=1
old: 8 looping both layout modes and the MountedData hazard pinned as two `should_panic` tests, one per mode
new: 8 looping both layout modes and two, one per mode, `mounted_data_in_a_task_a_key_handler_spawned_acts_{non_incremental,incremental}`, through the helper `spawned_focus_acts`, asserting that the focus a task spawned by a key handler sets acts — `#presses` reads 1, and the engine's focus, the snapshot's one `focused` node and the tree's focus are `focus-target` — the witness of the bridge's buffered render (§Established Decisions → [Dioxus integration])

@fact after=fct-ktfizprz as=n4
- dioxus-native-dom's `DioxusDocument` tests hold upstream's `task_polled_during_render_can_access_document` from `286874cc1`, as upstream wrote it, and it passes; on Dioxus 0.7 it also passes with the borrowed render restored, so it is insensitive to the borrow there, and the buffered render's discriminating witness on Dioxus 0.7 is `dioxus_runtime_probe`'s two `…_spawned_acts_*` tests, which fail with `RefCell already borrowed` under that mutation (packages/dioxus-native-dom/src/dioxus_document.rs:847-885 — as measured at escher-0.3.0/chunks/2026-11-07-mounteddata-buffered-render/evidence/mutation-control.md).

@pattern id=arch-01
fixed: holds the document's `RefCell` across `render_immediate`
@pattern id=arch-02
fixed: is owned by the route entry «MountedData buffered render»
@pattern id=arch-03
fixed: nothing sent (as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/evidence/readings.md), its owner «Upstream sync ahead of determinism and exploration»
@pattern id=arch-04
fixed: `res-3v42legm`, whose owner is «Upstream sync ahead of determinism and exploration»
@pattern id=arch-05
fixed: have no workspace caller, dioxus-core calling them
@pattern id=arch-06
fixed: is to be ported onto the Dioxus 0.7 bridge
@pattern id=arch-07
fixed: pinned as two `should_panic` tests
