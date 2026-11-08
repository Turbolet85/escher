@apply_row n=1 ref_file=fanout-architecture.md ref_n=1 v=applied edit=E1
@apply_row n=2 ref_file=fanout-architecture--cascade-g3.md ref_n=19 v=folded_into to_file=fanout-architecture.md to_n=1
@apply_row n=3 ref_file=fanout-architecture.md ref_n=2 v=applied edit=E2
@apply_row n=4 ref_file=fanout-architecture--cascade-g3.md ref_n=20 v=folded_into to_file=fanout-architecture.md to_n=2
@apply_row n=5 ref_file=fanout-architecture.md ref_n=3 v=applied edit=E3
@apply_row n=6 ref_file=fanout-architecture--cascade-g1.md ref_n=5 v=folded_into to_file=fanout-architecture.md to_n=3
@apply_row n=7 ref_file=fanout-architecture.md ref_n=4 v=refused
why: its place, fct-jzxi3spj, stands in architecture's occupied-resources category file, which is none of this part's three targets (the body, established-decisions, conventions), so its text and rev were not read here; it goes to the part that holds occupied-resources
@apply_row n=8 ref_file=fanout-architecture--cascade-g1.md ref_n=1 v=applied edit=E4
@apply_row n=9 ref_file=fanout-architecture--cascade-g1.md ref_n=2 v=applied edit=E5
@apply_row n=10 ref_file=fanout-architecture--cascade-g1.md ref_n=3 v=applied edit=E6
@apply_row n=11 ref_file=fanout-architecture--cascade-g1.md ref_n=4 v=applied edit=E7
@apply_row n=12 ref_file=fanout-architecture--cascade-g1.md ref_n=6 v=applied edit=E8
@apply_row n=13 ref_file=fanout-architecture--cascade-g1.md ref_n=7 v=applied edit=E9
@apply_row n=14 ref_file=fanout-architecture--cascade-g1.md ref_n=8 v=applied edit=E10
@apply_row n=15 ref_file=fanout-architecture--cascade-g1.md ref_n=9 v=applied edit=E11
@fact id=fct-ksgwc2yv expect_rev=1
old: twelve verbs, eight closed causes
new: thirteen verbs, eight closed causes

@fact id=fct-srb7slak expect_rev=1
old: written to neither DOM nor vdom; under `accessibility` the snapshot, its text form and its diff read the screen read-only, holding no `NodeId`.
new: written to neither DOM nor vdom, beside a reserved fifth form `{owner}///{key}` that the public `virtual_node_id` spells and no walk writes; under `accessibility` the snapshot, its text form, its diff and the UI graph (`DioxusDocument::graph`, a `UiGraph` of nodes and registered states) read the screen read-only, holding no `NodeId`, the snapshot's `clickable` reading an element's own current `click` listener on no disabled form control; an app registers graphed state through the ungated `use_graph_state` (`DeclaredWrite`, `SignalRegistry`), and the bridge's `handle_event` sets the registry's dispatch around the vdom's only where a registry stands.

@fact id=fct-nb3g6bqn expect_rev=1
old: diffs, shots and lint findings cross, never a `NodeId`
new: diffs, shots, lint findings and a UI graph's names, closed flag words and booleans cross, never a `NodeId`

@fact id=fct-nl4nv5ny expect_rev=1
old: the Enter and Space activation gates read it, so
new: the Enter and Space activation gates and the snapshot's `NodeState.clickable` (true where the bridge's vdom walk holds a current `click` listener and the element is no form control `ElementData::is_disabled` reports) read it, so a disabled `button` with a `click` listener reads not clickable and a `<div disabled>` with one reads clickable, and so

@fact after=fct-b7w6xfsd file=established-decisions gid=new:dioxus-graph-dispatch kind=values subject=dec-dioxus-integration
- The bridge's dispatch, a change on every Dioxus document's event path that does nothing without a registry: `DioxusEventHandler::handle_event` sets the `SignalRegistry` dispatch — the event's chain, head first, and its event name — around `runtime.handle_event` and clears it after, only where `runtime.has_context::<SignalRegistry>(ScopeId::ROOT)` holds, so a page that registers no state reaches no registry and dispatches as before; the buffered render, `initial_build`, `poll` and `flush_queued_mounted_events` are untouched, every manifest is unchanged and `mutation_writer.rs` stays byte-equal to upstream `434e0661` (as measured at escher-0.3.0/chunks/2026-11-07-ui-graph/report.md).

@fact id=fct-b7w6xfsd expect_rev=1
old: for hook-local `use_signal` state neither
new: for hook-local `use_signal` state an app does not register neither
old: — read from the code, not run.
new: — read from the code, not run; a state registered through `use_graph_state(name, writers, subscribers)`, which takes dioxus-core's public types alone, is reachable with no fork-level change: its readers are read from its `Subscribers`, its writes are recorded by a `ReactiveContext::new_with_callback` subscribed to the signal, against the standing dispatch and the writing scope, and `DioxusDocument::graph` reads both (as measured at escher-0.3.0/chunks/2026-11-07-ui-graph/report.md).

@fact after=fct-wgdl4cz6 as=n1
- The UI graph crosses the ratified carriers — the session socket's `call`, the command line and MCP, as a text block — and rests in a kept check as the `graph` verb's answer: `graph` takes no argument, one refused `malformed` with nothing run, and its answer holds stable ids, owner ids, component names, registered state names, handler event names, the declared writes the app authors, the closed flag words `undeclared` and `unconfirmed` and booleans, never a value, typed text or `NodeId`; each snapshot node carries the closed bool `clickable`; no new carrier, exit, cause, session error, span field or dependency: a ratified boundary widening — as measured at escher-0.3.0/chunks/2026-11-07-ui-graph/report.md.

@fact id=fct-7p2g4lnp expect_rev=1
old: diffs and a lint's findings at rest
new: diffs, a lint's findings and a UI graph's names, closed flag words and booleans at rest

@fact id=fct-bhxvo7z2 expect_rev=1
old: passes only the first two items (packages/dioxus-native-dom/src/lib.rs:80-83);
new: passes only the first two items (packages/dioxus-native-dom/src/lib.rs:80-83); dioxus-native-dom's `graph` module (`DioxusDocument::graph`, `UiGraph`) is gated by `accessibility`, beside the snapshot model it reads, and its `signal_registry` module (`use_graph_state`, `DeclaredWrite`, `SignalRegistry`) is not, so an app registers state whatever its features;

@fact id=fct-zd5gb466 expect_rev=1
old: `stand_requirement_sweep`, and the served-page checks `stand_cells_served` and `stand_load_page` — keep
new: `stand_requirement_sweep`, the served-page checks `stand_cells_served` and `stand_load_page` and `stand_graph`, which also reads `common`'s `GRAPH_DRIVES` — keep

@fact after=fct-2wv4ux4q file=conventions gid=new:graph-signal-slices kind=values subject=conv-apps-and-examples
- seven_guis' tasks register each signal through `tasks::use_graph_signal(name, writers, init)`, wrapping `use_signal` and `use_graph_state` (examples/seven_guis/src/tasks/mod.rs:4-15), `use_signal` left to the stand shell's `active` and `stand.rs`'s `linked`; its writers, a `&'static [DeclaredWrite]` built from the `const fn` `tasks::writes(id, event)` (examples/seven_guis/src/tasks/mod.rs:17-20), are written `const { &[…] }` or as a `const` item (examples/seven_guis/src/tasks/temp_converter.rs:5-9; examples/seven_guis/src/tasks/circle_drawer.rs:15-21), since a bare `&[writes(..)]` is a temporary not promoted to `'static` and errors `E0716` at the parameter.
@pattern id=arch1-01 ignore_case=true
fixed: twelve verbs
@pattern id=arch1-02
fixed: shots and lint findings cross
@pattern id=arch1-03
fixed: diffs and a lint's findings at rest
@pattern id=arch1-04
fixed: neither its readers nor the state a handler writes is reachable
