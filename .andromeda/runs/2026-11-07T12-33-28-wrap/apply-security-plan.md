@apply_row n=1 ref_file=fanout-security-plan--cascade.md ref_n=5 v=refused
why: main's own — its target lies outside this writer's files
@apply_row n=2 ref_file=fanout-security-plan--cascade.md ref_n=6 v=refused
why: main's own — its target lies outside this writer's files
@apply_row n=3 ref_file=fanout-security-plan--cascade.md ref_n=7 v=refused
why: main's own — its target lies outside this writer's files
@apply_row n=4 ref_file=fanout-security-plan--cascade.md ref_n=1 v=applied edit=E1
@apply_row n=5 ref_file=fanout-security-plan--cascade.md ref_n=2 v=applied edit=E2
@apply_row n=6 ref_file=fanout-security-plan--cascade.md ref_n=3 v=applied edit=E3
@apply_row n=7 ref_file=fanout-security-plan--cascade.md ref_n=4 v=applied edit=E4
@fact id=fct-7ni72vcj expect_rev=2
old: `DioxusDocument::poll` holds `inner.borrow_mut()` (packages/dioxus-native-dom/src/dioxus_document.rs:305) across `vdom.render_immediate` (packages/dioxus-native-dom/src/dioxus_document.rs:297), where dioxus-core 0.7.10 first polls a task spawned from a key handler that wrote a signal (`handle_task_wakeup`, dioxus-core `virtual_dom.rs:611`), so that task's `set_focus` (packages/dioxus-native-dom/src/events.rs:295), borrowing through `doc_mut` (packages/dioxus-native-dom/src/events.rs:162), panics `RefCell already borrowed`, any app able to meet it — measured headlessly in both layout modes and pinned by `dioxus_runtime_probe`'s two `should_panic` tests, one per layout mode, its fix routed to the route entry «MountedData buffered render», the windowed path read, not run;
new: `DioxusDocument::initial_build` and `Document::poll` render into the crate-private `MutationBuffer` with the document unborrowed, then take `inner.borrow_mut()` once, replay through one `MutationWriter`, drop the borrow and flush the queued mounted events (packages/dioxus-native-dom/src/dioxus_document.rs:305; packages/dioxus-native-dom/src/dioxus_document.rs:297), so a task dioxus-core 0.7.10 first polls inside `render_immediate`, one spawned from a key handler that wrote a signal (`Work::PollTask`, dioxus-core `virtual_dom.rs:610-613`), reaches the document through `MountedData`, its `set_focus` (packages/dioxus-native-dom/src/events.rs:295) borrowing through `doc_mut` (packages/dioxus-native-dom/src/events.rs:162) with no `RefCell already borrowed` panic, an engine change for every Dioxus document — measured headlessly in both layout modes and pinned by `dioxus_runtime_probe`'s two `mounted_data_in_a_task_a_key_handler_spawned_acts_*` tests, one per layout mode, which fail `RefCell already borrowed` with the borrowed render restored, the windowed path read, not run;
old: as measured at escher-0.3.0/chunks/2026-11-04-dioxus-runtime-probe/report.md
new: as measured at escher-0.3.0/chunks/2026-11-07-mounteddata-buffered-render/report.md

@fact after=fct-7ni72vcj as=n1 file=error-handling gid=new:render-path-recorder kind=values subject=errors
- Render-path recorder: dioxus-native-dom's crate-private `MutationBuffer` holds no `unwrap`, `expect` or `panic!`; its text spans are `Range<usize>` into one arena, each read back by `strings.get(span).unwrap_or_default()`, so it has no slicing panic path (packages/dioxus-native-dom/src/mutation_buffer.rs:91-95; packages/dioxus-native-dom/src/mutation_buffer.rs:97-140) — as measured at escher-0.3.0/chunks/2026-11-07-mounteddata-buffered-render/report.md

@fact after=@n1 as=n2 file=error-handling gid=new:render-task-limit kind=values subject=errors
- Mounted handles from a task polled during render, a limit by construction: such a task acts on the document as the previous render left it, so `set_focus_to` returns `false` for a node created in that same render, and the probe's pins act on nodes an earlier render mounted; owned by an open `residual` row (origin 2026-11-07-mounteddata-buffered-render, target: next) — as measured at escher-0.3.0/chunks/2026-11-07-mounteddata-buffered-render/report.md

@fact after=fct-ksl3fslx file=logging-monitoring gid=new:render-path-recorder-silence kind=values subject=logging
- The render-path recorder's silence: dioxus-native-dom's crate-private `MutationBuffer` holds no log site, print, env read or listener; its `Debug` prints `len` alone (packages/dioxus-native-dom/src/mutation_buffer.rs:82-88); it holds ids, tag and attribute names, attribute values and text in process only, and its replay re-fires `MutationWriter`'s own `trace!` calls, so the log stream is the one the direct write gives — as measured at escher-0.3.0/chunks/2026-11-07-mounteddata-buffered-render/report.md
@pattern id=sec-01
fixed: panics `RefCell already borrowed`, any app able to meet it
@pattern id=sec-02
fixed: two `should_panic` tests, one per layout mode
