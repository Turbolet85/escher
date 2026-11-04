@apply_row n=1 ref_file=fanout-test-plan.md ref_n=1 v=applied edit=E1
@apply_row n=2 ref_file=fanout-test-plan--cascade-g1.md ref_n=1 v=applied edit=E2
@apply_row n=3 ref_file=fanout-test-plan--cascade-g1.md ref_n=2 v=applied edit=E3
@apply_row n=4 ref_file=fanout-test-plan--cascade-g3.md ref_n=1 v=applied edit=E4
@apply_row n=5 ref_file=fanout-test-plan--cascade-g3.md ref_n=2 v=applied edit=E5
@apply_row n=6 ref_file=fanout-test-plan--cascade-g3.md ref_n=3 v=applied edit=E6
@apply_row n=7 ref_file=fanout-test-plan--cascade-g3.md ref_n=4 v=applied edit=E7
@apply_row n=8 ref_file=fanout-test-plan--cascade-g3.md ref_n=5 v=applied edit=E8
@apply_row n=9 ref_file=cascade-compare-test-plan.md ref_n=1 v=applied edit=E9
@apply_row n=10 ref_file=fanout-test-plan--cascade-new-integration-test-strategy.md ref_n=1 v=applied edit=E10
@apply_row n=11 ref_file=fanout-test-plan--cascade-new-integration-test-strategy.md ref_n=2 v=applied edit=E11
@fact id=fct-glytb3w3 expect_rev=1
old: `run all`'s reads passed 733 · failed 0 · ignored 12, `key_press_after_container_lost_its_box`, with no `stand_` prefix, picked up by `run all` with no script change — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md
new: `run all`'s reads passed 743 · failed 0 · ignored 12, `dioxus_runtime_probe`, with no `stand_` prefix, picked up by `run all` with no script change — as measured at escher-0.3.0/chunks/2026-11-04-dioxus-runtime-probe/report.md

@fact after=fct-2ha2za3s file=test-scope-summary gid=new:probe-target kind=procedure subject=scope-entities
- **dioxus-native-dom (Dioxus runtime) — `dioxus_runtime_probe`:** a blitz-tests target of 10 tests, 8 looping `for incremental in [false, true]` and the MountedData hazard pinned as two `should_panic` tests, one per layout mode, with no `stand_` prefix, so `run all` and `run dioxus_runtime_probe` run it and `run stand` does not; it reads Dioxus 0.7.10 public API alone, with no fork-level change, and measures link 1 (the component that rendered each node, a deleted CRUD row resolving to nothing), link 2 (each node's handlers), link 3 (a handler's effect as its step's snapshot diff) and link 4 (a held signal's readers) (tests/blitz-tests/tests/dioxus_runtime_probe.rs:199-607) — as measured at escher-0.3.0/chunks/2026-11-04-dioxus-runtime-probe/report.md

@fact id=fct-e2zfwpza expect_rev=1
old: has no counterpart in the fork's dioxus_document.rs test module — as measured
new: has no counterpart in the fork's dioxus_document.rs test module; it is the witness of the route entry «MountedData buffered render», which ports `286874cc1`'s buffered render onto the Dioxus 0.7 bridge and inverts `dioxus_runtime_probe`'s hazard pin — as measured

@fact id=fct-7m7lxsmy expect_rev=1
old: 157 `.rs` files, 156 test targets and the unbuilt `all.rs`, as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md, the latest `key_press_after_container_lost_its_box.rs`, then
new: 158 `.rs` files, 157 test targets and the unbuilt `all.rs`, as measured at escher-0.3.0/chunks/2026-11-04-dioxus-runtime-probe/report.md, the latest `dioxus_runtime_probe.rs`, then `key_press_after_container_lost_its_box.rs`, then

@fact id=fct-kgita7a3 expect_rev=1
old: and not run; a `MountedData` call
new: and not run (as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/evidence/readings.md); a `MountedData` call
old: `events.rs:162` (as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/evidence/readings.md)
new: `events.rs:162`: `DioxusDocument::poll` holds `inner.borrow_mut()` (`dioxus_document.rs:295`) across `vdom.render_immediate` (`:297`), where dioxus-core 0.7.10 first polls the queued task through `handle_task_wakeup` (`virtual_dom.rs:611`), and the task's `set_focus` (`events.rs:295`) borrows through `doc_mut` and panics, in both layout modes, pinned by `dioxus_runtime_probe` (below); its fix is routed to the route entry «MountedData buffered render», which ports upstream `286874cc1`'s buffered render onto the 0.7 bridge and inverts the pin — as measured at escher-0.3.0/chunks/2026-11-04-dioxus-runtime-probe/report.md

@fact after=fct-kgita7a3 file=e2e-test-strategy gid=new:mounted-pins kind=procedure subject=e2e
- **The MountedData hazard's pins:** `mounted_data_in_a_task_a_key_handler_spawned_panics_non_incremental` and `mounted_data_in_a_task_a_key_handler_spawned_panics_incremental`, two `should_panic` tests, one per layout mode, pin the panic, their precondition asserted first by `hazard_ready` (the handle held, the key target focused by Tab); beside them `mounted_data_asked_at_once_in_a_key_handler_acts`, in both layout modes: the same handler calling `set_focus` with no task moves focus and does not panic; `get_scroll_offset` and `get_scroll_size`, a shared borrow through `doc` (`events.rs:152`), and the windowed path are read from the code, not run (tests/blitz-tests/tests/dioxus_runtime_probe.rs:557-573; tests/blitz-tests/tests/dioxus_runtime_probe.rs:580-584; tests/blitz-tests/tests/dioxus_runtime_probe.rs:586-590; tests/blitz-tests/tests/dioxus_runtime_probe.rs:592-607) — as measured at escher-0.3.0/chunks/2026-11-04-dioxus-runtime-probe/report.md

@fact after=fct-zkjxcvz6 as=n1
| Dioxus runtime probe pages (`rsx!`, in their check's own file) | `dioxus_runtime_probe`'s `toggled_listener` (its click listener spread in and out as a `Vec<Attribute>`), `shared_state` (`Writer`, `FirstReader` and `SecondReader` over one `GlobalSignal`) and `hazard(spawned)` (`hazard_spawned`, `hazard_synchronous`) | (tests/blitz-tests/tests/dioxus_runtime_probe.rs:370-385; tests/blitz-tests/tests/dioxus_runtime_probe.rs:446-449; tests/blitz-tests/tests/dioxus_runtime_probe.rs:451-454; tests/blitz-tests/tests/dioxus_runtime_probe.rs:456-461; tests/blitz-tests/tests/dioxus_runtime_probe.rs:463-470; tests/blitz-tests/tests/dioxus_runtime_probe.rs:515-547; tests/blitz-tests/tests/dioxus_runtime_probe.rs:549-551; tests/blitz-tests/tests/dioxus_runtime_probe.rs:553-555) |

@fact after=fct-gemhwfdl file=ci-integration gid=new:audit-red kind=values subject=ci
- **Red on `Dependency audit`:** CI#41911351081 at `53ae48a65` ended red on `Dependency audit` (`advisories FAILED, licenses ok`: RUSTSEC-2026-0433 on pulldown-cmark 0.13.5, RUSTSEC-2026-0429 on zip 2.4.2), and CI#41914044942 at `9b3b29714` reads green 17/17; why an audit goes red with `Cargo.lock` and `deny.toml` unchanged, and its local reproduction, stand at security-plan §Dependency Security — as measured at escher-0.3.0/chunks/2026-11-04-dioxus-runtime-probe/report.md

@fact after=fct-nrwelml5 file=ci-integration gid=new:probe-legs kind=values subject=ci
- **Legs reaching `dioxus_runtime_probe`:** the blitz-tests target `dioxus_runtime_probe`, auto-discovered with no manifest line, runs in the workspace `test` leg and in `agent-run.sh`'s `run all` and `run dioxus_runtime_probe`, each taking it as one more target with no line of its script naming it; its 10 tests, 8 looping `for incremental in [false, true]` and the MountedData hazard pinned as two `should_panic` tests, one per layout mode; it holds no `stand_` prefix and is none of `stand-repeat`'s seventeen named `--test` words, so neither `stand-repeat` nor `run stand` runs it, and it is none of the `a11y` leg's fourteen files (§9 Legs above) — as measured at escher-0.3.0/chunks/2026-11-04-dioxus-runtime-probe/report.md

@fact after=fct-6g7tp3rl file=integration-test-strategy gid=new:probe-listeners kind=procedure subject=integration as=n2
- **Dioxus VirtualDom ↔ DioxusDocument, a node's listeners:** the bridge's listener attributes — an attribute named the event, valued `<rust func>`, beside the `data-dioxus-id` mark, written by `create_event_listener` (packages/dioxus-native-dom/src/mutation_writer.rs:331-352) — over-report: dioxus-core 0.7.10 reaches `remove_event_listener` only when a listener leaves an element's attribute list (a spread, the `toggled_listener` page), and it lowers the per-kind count alone (packages/dioxus-native-dom/src/mutation_writer.rs:355-360), so a listener removed on a re-render leaves its attribute and the mark standing while a click no longer acts, and a `mounted` listener writes none; the vdom walk's dynamic attributes, `VNode.dynamic_attrs` with `VNode::mounted_dynamic_attribute`, read each element's current listeners exactly, `mounted` included — pinned by `dioxus_runtime_probe`'s `link_2_a_removed_listeners_attribute_stays`, in both layout modes (tests/blitz-tests/tests/dioxus_runtime_probe.rs:370-385; tests/blitz-tests/tests/dioxus_runtime_probe.rs:387-429) — as measured at escher-0.3.0/chunks/2026-11-04-dioxus-runtime-probe/report.md

@fact after=@n2 file=integration-test-strategy gid=new:probe-held-signal kind=procedure subject=integration
- **Dioxus VirtualDom ↔ DioxusDocument, a held signal's readers:** a `GlobalSignal`'s `subscribers()`, visited inside `VirtualDom::in_runtime`, give through `ReactiveContext::origin_scope` exactly its two reader components' scopes, and the writer's click changes exactly the readers' nodes in the step's snapshot diff — `dioxus_runtime_probe`'s `links_3_and_4_a_held_signals_subscribers_are_its_readers` over the `shared_state` page (`Writer`, `FirstReader`, `SecondReader`), in both layout modes; hook-local `use_signal` state, where the stand keeps all its state, exposes neither link without a fork-level change, `Scope.hooks` being `pub(crate)` (dioxus-core 0.7.10 `scope_context.rs:53`), read from the code, not run (tests/blitz-tests/tests/dioxus_runtime_probe.rs:463-470; tests/blitz-tests/tests/dioxus_runtime_probe.rs:472-513) — as measured at escher-0.3.0/chunks/2026-11-04-dioxus-runtime-probe/report.md

@pattern id=tp-01
fixed: passed 733 · failed 0 · ignored 12
@pattern id=tp-02
fixed: 157 `.rs` files, 156 test targets
