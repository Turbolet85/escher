@apply_row n=1 ref_file=fanout-a11y-plan--cascade.md ref_n=15 v=applied edit=E1
@apply_row n=2 ref_file=fanout-a11y-plan--cascade.md ref_n=1 v=applied edit=E2
@apply_row n=3 ref_file=fanout-a11y-plan--cascade.md ref_n=9 v=applied edit=E3
@apply_row n=4 ref_file=fanout-a11y-plan--cascade.md ref_n=12 v=applied edit=E4
@apply_row n=5 ref_file=fanout-a11y-plan--cascade.md ref_n=13 v=applied edit=E5
@apply_row n=6 ref_file=fanout-a11y-plan--cascade.md ref_n=14 v=applied edit=E6
@apply_row n=7 ref_file=cascade-compare-a11y-plan.md ref_n=1 v=applied edit=E7
@fact id=fct-77ewqepn expect_rev=1
old: that reading owned by a `.andromeda/residuals.md` line (target: next)
new: that reading measured by link 2, its field owed to the route entry «The UI graph»; the listener mark over-includes an element whose listener was removed (conservative)

@fact id=fct-r6zswm2d expect_rev=1
old: the adapter on the stand binary and its windowed AT witness owned by a `.andromeda/residuals.md` line that reads
new: the adapter on the stand binary and its windowed AT witness owned by a `residual` row in the store that reads
old: so no stand binary builds a platform adapter still — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md
new: so no stand binary builds a platform adapter still — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md; re-read on `Cargo.lock`'s move of `pulldown-cmark` to 0.13.6, no crate added or removed and no `Cargo.toml` changed, Dioxus at the `"0.7.3"` requirement: no platform adapter joins the stand's graph, so no stand binary builds a platform adapter still — as measured at escher-0.3.0/chunks/2026-11-04-dioxus-runtime-probe/report.md

@fact id=fct-gmhtpdel expect_rev=1
old: panics with `RefCell already borrowed` (`events.rs:162`), the task first polled inside `render_immediate`, measured headlessly and the windowed path read and not run,
new: panics with `RefCell already borrowed`: `DioxusDocument::poll` holds `inner.borrow_mut()` across `vdom.render_immediate`, dioxus-core 0.7.10 first polls the task there through `handle_task_wakeup`, and its `set_focus`, borrowing through `doc_mut`, panics, while the same handler calling `set_focus` with no task moves focus and does not panic — measured headlessly in both layout modes by `dioxus_runtime_probe`'s two `should_panic` pins, their precondition asserted first (the handle held, the key target focused by Tab), and the windowed path read and not run (packages/dioxus-native-dom/src/dioxus_document.rs:295-297; packages/dioxus-native-dom/src/events.rs:162; packages/dioxus-native-dom/src/events.rs:295; tests/blitz-tests/tests/dioxus_runtime_probe.rs:557-607);
old: each acting at the call — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md
new: each acting at the call; the hazard's fix routed to the route entry «MountedData buffered render» — as measured at escher-0.3.0/chunks/2026-11-04-dioxus-runtime-probe/report.md, Cells' synchronous calls at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md

@fact id=fct-pfylw46r expect_rev=1
old: the listener mark `data-dioxus-id` (so a hidden control is still held)
new: the listener mark `data-dioxus-id` (so a hidden control is still held, and so is an element whose listener a re-render removed, its mark still standing, so the check over-includes, conservative, while a `mounted` listener writes none — read, not run)
old: owned by a `.andromeda/residuals.md` line (target: next) on the founder's C3
new: measured by link 2, its snapshot field owed to the route entry «The UI graph» on the founder's C3

@fact after=fct-pfylw46r as=n1
- The «reacts to a click» reading is measured by `dioxus_runtime_probe`'s link 2, in both layout modes: on Cells the 677 elements carrying the listener mark and stating no role are the 676 cells reacting to `click` and `cells-grid` reacting to `keydown`; on Counter `counter-increment` carries `click` and `counter-value` none; CRUD's `crud-list` carries `keydown`; no snapshot field is added (`NODE_FIELDS` thirteen) (tests/blitz-tests/tests/dioxus_runtime_probe.rs:282-317; tests/blitz-tests/tests/dioxus_runtime_probe.rs:340-368) — as measured at escher-0.3.0/chunks/2026-11-04-dioxus-runtime-probe/report.md

@fact after=@n1 as=n2
- The bridge's listener attributes over-report: a listener removed on a re-render leaves its event attribute and the `data-dioxus-id` mark standing while a click no longer acts, `remove_event_listener` lowering the per-kind count alone, and a `mounted` listener writes none; the vdom walk's dynamic attributes (`VNode.dynamic_attrs`, `VNode::mounted_dynamic_attribute`, public dioxus-core 0.7.10 API) read each element's current listeners exactly, `mounted` included (packages/dioxus-native-dom/src/mutation_writer.rs:331-352; packages/dioxus-native-dom/src/mutation_writer.rs:355-360; tests/blitz-tests/tests/dioxus_runtime_probe.rs:387-429) — as measured at escher-0.3.0/chunks/2026-11-04-dioxus-runtime-probe/report.md

@fact after=fct-3xv5s7f6 file=ci-integration gid=new:probe-leg kind=procedure subject=ci
`dioxus_runtime_probe` (10 tests, in both layout modes), which measures link 2's «reacts to a click» reading (§7), is none of the a11y leg's fourteen files and carries no `stand_` prefix, so it runs in neither the `a11y` nor the `stand-repeat` leg — the workspace `test` leg, `run all` and `run dioxus_runtime_probe` reach it; the a11y leg and its job are unchanged (as measured at escher-0.3.0/chunks/2026-11-04-dioxus-runtime-probe/report.md).
@pattern id=a11y-01
fixed: that reading owned by a `.andromeda/residuals.md` line
@pattern id=a11y-02
fixed: owned by a `.andromeda/residuals.md` line that reads `re-carried:0.2.0`
