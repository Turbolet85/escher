@apply_row n=1 ref_file=fanout-architecture--cascade-g1.md ref_n=1 v=applied edit=E1
@apply_row n=2 ref_file=fanout-architecture--cascade-g1.md ref_n=2 v=applied edit=E2
@apply_row n=3 ref_file=fanout-architecture--cascade-g1.md ref_n=4 v=applied edit=E3
@apply_row n=4 ref_file=fanout-architecture--cascade-g1.md ref_n=5 v=applied edit=E4
@apply_row n=5 ref_file=fanout-architecture--cascade-g1.md ref_n=7 v=applied edit=E5
@apply_row n=6 ref_file=fanout-architecture--cascade-g1.md ref_n=8 v=applied edit=E6
@apply_row n=7 ref_file=fanout-architecture--cascade-g1.md ref_n=9 v=applied edit=E7
@apply_row n=8 ref_file=fanout-architecture--cascade-g1.md ref_n=11 v=applied edit=E8
@apply_row n=9 ref_file=fanout-architecture--cascade-g2.md ref_n=1 v=applied edit=E9
@fact id=fct-kjoxjobc expect_rev=1
old: resolved 0.13.5, past RUSTSEC-2026-0414;
new: resolved 0.13.6, past RUSTSEC-2026-0414 and RUSTSEC-2026-0433;

@fact id=fct-apdputpm expect_rev=1
old: such a task that writes a signal is first polled inside `render_immediate`, which holds the document's `RefCell`, and a `MountedData` call in it panics at `events.rs:162` (`RefCell already borrowed`), so Cells calls its handles synchronously. Any app meets it; measured headlessly through the harness's `poll(None)`, the windowed shell's same lines read and not run (as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/evidence/readings.md).
new: `DioxusDocument::poll` holds the document's `RefCell` across `render_immediate`, where dioxus-core 0.7.10 first polls a queued task through `handle_task_wakeup`, so such a task that writes a signal and calls `set_focus` borrows through `doc_mut` and panics (`RefCell already borrowed`) (packages/dioxus-native-dom/src/dioxus_document.rs:295-297; packages/dioxus-native-dom/src/events.rs:162; packages/dioxus-native-dom/src/events.rs:295), so Cells calls its handles synchronously. Any app meets it; measured headlessly through the harness's pump in both layout modes and pinned by `dioxus_runtime_probe` (test-plan §6 E2E Test Strategy), the windowed shell's same lines read and not run (as measured at escher-0.3.0/chunks/2026-11-04-dioxus-runtime-probe/report.md); its fix, a port of upstream `286874cc1`'s buffered render onto the Dioxus 0.7 bridge, is owned by the route entry «MountedData buffered render».

@fact after=fct-apdputpm file=established-decisions gid=new:dioxus-public-api kind=values subject=dec-dioxus-integration as=n1
- A node's owning component reads through Dioxus 0.7.10's public API with no fork-level change: a walk from `VirtualDom::base_scope` through `ScopeState::try_root_node`, `VNode.template.roots`, `VNode.dynamic_nodes`, `VNode::mounted_root` and `VComponent::mounted_scope`, naming each component by the field `VComponent.name`, names a component for every task-body element on Counter and CRUD, and a deleted CRUD row resolves to none, in both layout modes (tests/blitz-tests/tests/dioxus_runtime_probe.rs:199-280).

@fact after=@n1 as=n2
- A held signal's readers read through public API: a `GlobalSignal`'s `Readable::subscribers()`, visited inside `VirtualDom::in_runtime`, give through `ReactiveContext::origin_scope` exactly its reader components' scopes, and a writer's click changes exactly the readers' nodes in the snapshot diff, in both layout modes (tests/blitz-tests/tests/dioxus_runtime_probe.rs:472-513); for hook-local `use_signal` state neither its readers nor the state a handler writes is reachable without a fork-level change, dioxus-core's `Scope.hooks` being `pub(crate)` — read from the code, not run.

@fact after=fct-2ylcpw3b file=standard-contracts gid=new:bridge-listeners kind=values subject=contract-dioxus-dom-bridge as=n3
- An element's listeners read two ways: the vdom walk's dynamic attributes (`VNode.dynamic_attrs`, `VNode::mounted_dynamic_attribute`) read its current listeners exactly, `mounted` included, while the bridge's listener attributes — an attribute named the event, valued `"<rust func>"`, beside the `data-dioxus-id` mark, written by `create_event_listener` — over-report: a listener a re-render removes leaves its attribute and the mark standing while a click no longer acts, `remove_event_listener` lowering the per-kind count alone, and a `mounted` listener writes none, in both layout modes (packages/dioxus-native-dom/src/mutation_writer.rs:331-352; packages/dioxus-native-dom/src/mutation_writer.rs:355-360; tests/blitz-tests/tests/dioxus_runtime_probe.rs:387-429).

@fact after=@n3 as=n4
- dioxus-core 0.7.10 reaches `remove_event_listener` only when a listener attribute leaves an element's attribute list (a spread); a listener whose value turns to `AttributeValue::None` under the same name is written through `set_attribute` as an `on{event}` attribute, so the bridge's event attribute stays either way (dioxus-core 0.7.10 `diff/node.rs:416-536`, read in the registry copy); `create_event_listener` and `remove_event_listener` have no workspace caller, dioxus-core calling them.

@fact id=fct-kllkxz3d expect_rev=1
old: every listener but `onmounted` — with ids
new: every listener but `onmounted` and that stays, with its event attribute, on an element whose listener a re-render removed, so the check over-includes such an element by its listener, the conservative side (read, not run) — with ids

@fact id=fct-4ncs7geh expect_rev=1
old: Dioxus 0.8.0-alpha.3 (`286874cc1`), which wait
new: Dioxus 0.8.0-alpha.3 (`286874cc1`, whose buffered render, the MountedData hazard's fix, is to be ported onto the Dioxus 0.7 bridge by the route entry «MountedData buffered render», the 0.8 move itself still waiting), which wait

@fact after=fct-6d5l2imz as=n5
- blitz-tests' Dioxus runtime probe, one file and one target, in both layout modes: `dioxus_runtime_probe`, 10 tests — 8 looping both layout modes and the MountedData hazard pinned as two `should_panic` tests, one per mode — over the stand's Counter, CRUD and Cells and its own pages `toggled_listener`, `shared_state` and `hazard`, reading Dioxus 0.7.10's public API alone: a node's owning component, its listeners and a held signal's readers; it holds no print, env read, subscriber or listener, and has no `stand_` prefix, so it is in neither `run stand`, `stand-repeat` nor the `a11y` leg, run by the workspace `test` leg and `run all` (tests/blitz-tests/tests/dioxus_runtime_probe.rs:199-607).
@pattern id=arch-01
fixed: resolved 0.13.5
