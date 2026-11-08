@apply_row n=1 ref_file=fanout-architecture--cascade-g2.md ref_n=1 v=applied edit=E1
@apply_row n=2 ref_file=fanout-architecture--cascade-g2.md ref_n=2 v=folded_into to_file=fanout-architecture--cascade-g2.md to_n=1
@apply_row n=3 ref_file=fanout-architecture--cascade-g2.md ref_n=3 v=folded_into to_file=fanout-architecture--cascade-g2.md to_n=1
@apply_row n=4 ref_file=fanout-architecture--cascade-g2.md ref_n=4 v=folded_into to_file=fanout-architecture--cascade-g2.md to_n=1
@apply_row n=5 ref_file=fanout-architecture--cascade-g2.md ref_n=5 v=folded_into to_file=fanout-architecture--cascade-g2.md to_n=1
@apply_row n=6 ref_file=fanout-architecture--cascade-g2.md ref_n=6 v=folded_into to_file=fanout-architecture--cascade-g2.md to_n=1
@apply_row n=7 ref_file=fanout-architecture--cascade-g2.md ref_n=7 v=folded_into to_file=fanout-architecture--cascade-g2.md to_n=1
@apply_row n=8 ref_file=fanout-architecture--cascade-g2.md ref_n=8 v=folded_into to_file=fanout-architecture--cascade-g2.md to_n=1
@apply_row n=9 ref_file=fanout-architecture--cascade-g2.md ref_n=9 v=folded_into to_file=fanout-architecture--cascade-g2.md to_n=1
@apply_row n=10 ref_file=fanout-architecture--cascade-g2.md ref_n=10 v=folded_into to_file=fanout-architecture--cascade-g2.md to_n=1
@apply_row n=11 ref_file=fanout-architecture--cascade-g2.md ref_n=11 v=folded_into to_file=fanout-architecture--cascade-g2.md to_n=1
@apply_row n=12 ref_file=fanout-architecture--cascade-g2.md ref_n=12 v=applied edit=E2
@apply_row n=13 ref_file=fanout-architecture--cascade-g2.md ref_n=13 v=applied edit=E3
@apply_row n=14 ref_file=fanout-architecture--cascade-g2.md ref_n=14 v=applied edit=E4
@apply_row n=15 ref_file=fanout-architecture--cascade-g2.md ref_n=15 v=applied edit=E5
@apply_row n=16 ref_file=fanout-architecture--cascade-g3.md ref_n=1 v=applied edit=E6
@apply_row n=17 ref_file=fanout-architecture--cascade-g3.md ref_n=2 v=applied edit=E7
@apply_row n=18 ref_file=fanout-architecture--cascade-g3.md ref_n=3 v=applied edit=E8
@apply_row n=19 ref_file=fanout-architecture--cascade-g3.md ref_n=6 v=applied edit=E9
@apply_row n=20 ref_file=fanout-architecture--cascade-g3.md ref_n=7 v=applied edit=E10
@apply_row n=21 ref_file=fanout-architecture--cascade-g3.md ref_n=8 v=folded_into to_file=fanout-architecture--cascade-g3.md to_n=7
@apply_row n=22 ref_file=fanout-architecture--cascade-g3.md ref_n=9 v=applied edit=E11
@apply_row n=23 ref_file=fanout-architecture--cascade-g3.md ref_n=10 v=applied edit=E12
@apply_row n=24 ref_file=fanout-architecture--cascade-g3.md ref_n=11 v=applied edit=E13
@apply_row n=25 ref_file=fanout-architecture--cascade-g3.md ref_n=12 v=applied edit=E14
@apply_row n=26 ref_file=fanout-architecture--cascade-g3.md ref_n=15 v=applied edit=E15
@apply_row n=27 ref_file=fanout-architecture--cascade-g3.md ref_n=16 v=applied edit=E16
@apply_row n=28 ref_file=fanout-architecture--cascade-g3.md ref_n=17 v=applied edit=E17
@apply_row n=29 ref_file=fanout-architecture--cascade-g3.md ref_n=18 v=applied edit=E18
@apply_row n=30 ref_file=fanout-architecture--cascade-g4.md ref_n=1 v=applied edit=E19
@apply_row n=31 ref_file=fanout-architecture--cascade-g4.md ref_n=2 v=folded_into to_file=fanout-architecture--cascade-g4.md to_n=1
@fact id=fct-t7zq5feb expect_rev=1
old: `VERBS` lists twelve verbs in three levels
new: `VERBS` lists thirteen verbs in three levels
old: first the eight asked of the held instance, in order:
new: first the nine asked of the held instance, in order:
old: (packages/escher-driver/src/schema.rs:355-380; packages/escher-driver/src/schema.rs:550-555); then the three asked of the session itself
new: (packages/escher-driver/src/schema.rs:355-380; packages/escher-driver/src/schema.rs:550-555), and `graph` (no argument, an argument refused `malformed` with nothing run; result `nodes` (`FieldKind::GraphNodes`) and `states` (`FieldKind::GraphStates`)), the screen's UI graph read whole, a further widening of the validated verb set (§Established Decisions → [Driver session]) (packages/escher-driver/src/schema.rs:382-427); then the three asked of the session itself
old: each keyed by the thirteen keys `NODE_FIELDS` names: `id` · `parent` · `role` · `name` · `enabled` · `checked` · `value` · `selected` · `pressed` · `invalid` · `focused` · `bounds` · `differs`
new: each keyed by the fourteen keys `NODE_FIELDS` names: `id` · `parent` · `role` · `name` · `enabled` · `checked` · `value` · `selected` · `pressed` · `invalid` · `focused` · `clickable` · `bounds` · `differs`
old: since that chunk the constant is `[NodeFieldSpec; 13]`, each row
new: the constant is `[NodeFieldSpec; 14]`, each row
old: `bounds` the same however far a box's content is scrolled;
new: `bounds` the same however far a box's content is scrolled, `clickable` true where the element's current listeners hold `click` and it is no disabled form control;
old: since 2026-10-12-headless-screenshot — ten kinds.
new: since 2026-10-12-headless-screenshot, then `GraphNodes` (`graph-nodes`) and `GraphStates` (`graph-states`), the kinds of `graph`'s `nodes` and `states` fields — twelve kinds.
old: 12 verbs, 13 node keys,
new: 13 verbs, 14 node keys,
old: packages/escher-driver/src/command.rs:179-180), with
new: packages/escher-driver/src/command.rs:179-180) · `Graph` (validated from `("graph", [])`), with
old: (packages/escher-driver/src/execute.rs:88-92) (`Debug`
new: (packages/escher-driver/src/execute.rs:88-92) · `Graphed { graph: UiGraph }` (`Debug`
old: so a lint dispatches nothing, runs no step and settles nothing;
new: so a lint dispatches nothing, runs no step and settles nothing, and the graph is dioxus-native-dom's `UiGraph`, read by the read-only `DioxusDocument::graph`, so a graph read dispatches nothing, runs no step and settles nothing too;
old: which takes no argument, and from every snapshot a call takes
new: which takes no argument, from the node ids of the graph an accepted `graph` reads, and from every snapshot a call takes
old: as do an accepted `lint` and a part read;
new: as do an accepted `lint`, a part read and an accepted `graph`;
old: a lint's findings or their count,
new: a lint's findings or their count, a graph, its ids or its names,
old: `tools/list` answers the twelve verbs of `VERBS` in order (`screenshot` after `scroll`, then `lint`), `help` last
new: `tools/list` answers the thirteen verbs of `VERBS` in order (`screenshot` after `scroll`, then `lint`, then `graph`), `help` last
old: and `lint`, no field added)
new: and `lint` and `graph`, no field added)
old: `focused` and `bounds` always written — `invalid` a boolean on every node,
new: `focused`, `clickable` and `bounds` always written — `invalid` and `clickable` each a boolean on every node,
old: for a grammar or spelling error" and the help of
new: for a grammar or spelling error", naming `clickable` among the state it compares, and the help of
old: pressed or unpressed, invalid and focused; a mixed toggle
new: pressed or unpressed, invalid, focused and clickable; a mixed toggle

@fact after=fct-62w2i7iy as=n1
- The UI graph verb: `graph`, the ninth instance verb, takes no argument — an argument is refused `malformed` with nothing run — and answers `Outcome::Graphed`, the screen's graph read through `DioxusDocument::graph` (§Dioxus DOM bridge), its node ids feeding the session's record of ids; its written form, by hand in the crate-private `json` module, is `{"nodes":[…],"states":[…]}` — a node keyed `id` · `owner` · `component` · `handlers` · `reads`, `owner` and `component` left out when absent, a handler `event` · `writes`, a write `state` · `declared` · `confirmed` · `flag`, `flag` left out when none; a state keyed `name` · `component` · `declared` · `writers` · `readers`, `component` left out when absent, a declaration `id` · `event`, a writer `id` · `event` · `declared` · `confirmed` · `flag` — holding ids, component, state and event names, closed flag words and booleans only, never a value (packages/escher-driver/src/json.rs:240-308).

@fact after=@n1 as=n2
- The graph is unpaged: `graph` answers the whole screen at once, and Cells' answer reads 293,869 bytes, under the 1,048,576-byte answer bound, which `stand_graph` holds in both layout modes (tests/blitz-tests/tests/stand_graph.rs:381-394; as measured at escher-0.3.0/chunks/2026-11-07-ui-graph/report.md); the remainder, each cell node's declared `click` writes, is the stated limit «An unpaged graph» (docs/capabilities.md:252-255), owned by residual `res-7cc5bi4k` (target: next).

@fact after=@n2 as=n3
- `stand_graph`, in both layout modes, holds the `graph` verb's answer equal to `DioxusDocument::graph`, every graph node a snapshot node, no served id holding the reserved `///` form, a graph read changing nothing and holding no typed text, and the whole Cells answer within the answer bound; its drive table `GRAPH_DRIVES` drives each conditional write once, so no stand write reads a flag (tests/blitz-tests/tests/stand_graph.rs:106-394; tests/blitz-tests/tests/common/mod.rs:243-323).

@fact after=fct-dl75efux as=n4
- The stand's graph hooks: `tasks::use_graph_signal(name, writers, init) -> Signal<T>` wraps `use_signal` and `use_graph_state` (§Dioxus DOM bridge) in `tasks/mod.rs`, the `Signal` form living in seven_guis because dioxus-native-dom names no crate that defines `Signal`, and the crate-private `tasks::writes(id, event)` is a `const fn` building a `DeclaredWrite`, a task's declaration slice written `const { &[…] }` (examples/seven_guis/src/tasks/mod.rs:4-20).

@fact id=fct-tofwjjiv expect_rev=1
old: a verb outside the eight instance verbs, a `lint` step kept like every other with its answer whole
new: a verb outside the nine instance verbs, a `lint` or a `graph` step kept like every other with its answer whole
old: closed keys, the eight instance verbs,
new: closed keys, the nine instance verbs,

@fact id=fct-6m5fkd33 expect_rev=1
old: and `VERBS` holds twelve;
new: and `VERBS` holds thirteen, `graph` listed after `lint`;

@fact id=fct-g75i5t3p expect_rev=1
old: `snapshot`, `screenshot` and `lint`, which run no step
new: `snapshot`, `screenshot`, `lint` and `graph`, which run no step

@fact id=fct-zmumd7nc expect_rev=1
old: re-exports `dioxus_native_dom` plus
new: re-exports `dioxus_native_dom` — the graph registry's `DeclaredWrite`, `SignalRegistry` and `use_graph_state` among it — plus

@fact id=fct-kllkxz3d expect_rev=2
old: `NodeState` carries seven fields in the order
new: `NodeState` carries eight fields in the order
old: `invalid: bool` · `focused: bool` — four until
new: `invalid: bool` · `focused: bool` · `clickable: bool` — four until
old: names all seven
new: names all eight
old: which falls back to the root element) — and no field holds
new: which falls back to the root element), `clickable` true where the vdom walk's current listeners hold `click` and the element is no `ElementData::is_disabled` form control — and no field holds
old: · `invalid` · `focused` (a token absent where the model holds nothing for it; `invalid` written only when true;
new: · `invalid` · `focused` · `clickable` (a token absent where the model holds nothing for it; `invalid` and `clickable` written only when true;
old: eleven variants, `Parent` · `Role` · `Name` · `Enabled` · `Checked` · `Value` · `Selected` · `Pressed` · `Invalid` · `Focused` · `Bounds`
new: twelve variants, `Parent` · `Role` · `Name` · `Enabled` · `Checked` · `Value` · `Selected` · `Pressed` · `Invalid` · `Focused` · `Clickable` · `Bounds`
old: (`differing`, its `NodeState` destructured whole)
new: (`differing`, its `NodeState` destructured whole, so `clickable` is compared and a changed node's `differs` can name it)
old: so a key spelled like a component name stays distinct (`Item//b:0` beside `Item/b:0`);
new: so a key spelled like a component name stays distinct (`Item//b:0` beside `Item/b:0`); beside the four tiers the grammar states a fifth, reserved form, `{owner}///{key}`, which no tier can hold and no caller produces, the walk not calling it — `VIRTUAL_KEY_SEPARATOR` (`"///"`) and `virtual_node_id(owner, key) -> Option<String>`, public and re-exported, spell it, answering `None` for an empty or `/`-bearing key (packages/dioxus-native-dom/src/element_id.rs:39-48);

@fact after=fct-ycfl4hix as=n5
- The bridge's dispatch: where `runtime.has_context::<SignalRegistry>(ROOT)` holds, `DioxusEventHandler::handle_event` sets the registry's dispatch around `runtime.handle_event` and clears it after; a page with no registered state reaches no registry and dispatches as before — a change on every Dioxus document's event path, inert without a registry — and the test-only `DioxusDocument::dispatch_to_vdom` (`#[cfg(test)]`) drives it (packages/dioxus-native-dom/src/dioxus_document.rs:446-457; packages/dioxus-native-dom/src/dioxus_document.rs:240-251).

@fact after=fct-llnppo2j as=n6
- The UI graph: `DioxusDocument::graph(&self) -> UiGraph`, behind `accessibility`, is read-only: `UiGraph { nodes, states }` holds one `GraphNode { id, owner, component, handlers, reads }` per snapshot node, in pre-order; a handler is `GraphHandler { event, writes }`, a write `GraphWrite { state, declared, confirmed, flag }`; a state is `GraphState { name, component, declared, writers, readers }`, a writer `GraphWriter { id, event, declared, confirmed, flag }`; `WriteFlag::{Undeclared, Unconfirmed}` has `word()` (packages/dioxus-native-dom/src/graph.rs:24-125; packages/dioxus-native-dom/src/graph.rs:136-284).

@fact after=@n6 as=n7
- The graph's readings: `owner` is the stable id of `BaseDocument::select_of` for an option or an optgroup; a repeated (name, component) registration reads `{name}:{k}`; a state's `readers` names each reading component once, in the pre-order of its first node, and a state only the app root reads is named in its nodes' `reads` alone; its `writers` lists only writes a run confirmed or flagged, while `declared` keeps the declaration as authored; a write resolves to the first element of its chain carrying a listener of its event, or to nothing when a node of the chain before that element no longer resolves; `unconfirmed` fires only once a dispatch of the handler's event passed through its element and no run wrote the state (packages/dioxus-native-dom/src/graph.rs:294-331).

@fact after=@n7 as=n8
- The graph registry: `use_graph_state(name: &'static str, writers: &'static [DeclaredWrite], subscribers: Subscribers)` takes dioxus-core types alone; the first registering hook provides a `SignalRegistry` context at `ScopeId::ROOT`; each entry holds a `ReactiveContext::new_with_callback` whose callback records one write against the standing dispatch — its chain head first, and its event name — and the writing scope, and leaves with its scope through `use_drop`; `DeclaredWrite { id, event }` names an element's stable id or a prefix ending in `*`, and an event without `on`; the registry's `Debug` prints counts alone (packages/dioxus-native-dom/src/signal_registry.rs:20-121).

@fact id=fct-ie5oemgh expect_rev=1
old: tests/blitz-tests/tests/dioxus_runtime_probe.rs:386-428).
new: tests/blitz-tests/tests/dioxus_runtime_probe.rs:386-428). The exact reading is the crate-private `element_listeners`, `mounted` included, beside `element_facts`, each element's owning scope and component name; the UI graph's handlers and `NodeState.clickable` read these, never the listener mark, and `element_ids` and `element_id` read as before (packages/dioxus-native-dom/src/element_id.rs:253-311).

@fact after=fct-ie5oemgh as=n9
- `clickable` on the stand, in both layout modes: `stand_non_lean_tasks`' `CLICKABLE` table reads Counter, Flight Booker, Timer, CRUD and Temperature Converter 0 each, Circle Drawer 1 and Cells 676, while the listener mark's count stays 677 (tests/blitz-tests/tests/stand_non_lean_tasks.rs:66-75; tests/blitz-tests/tests/stand_non_lean_tasks.rs:117-127; as measured at escher-0.3.0/chunks/2026-11-07-ui-graph/report.md).

@fact id=fct-h4l33tnz expect_rev=1
old: `NODE_FIELDS` keeping the thirteen keys
new: `NODE_FIELDS` keeping the fourteen keys

@fact id=fct-c5e4c7ao expect_rev=1
old: the verbs are the twelve of `VERBS`
new: the verbs are the thirteen of `VERBS`
old: `screenshot [--id <id>]`, `lint`, `start
new: `screenshot [--id <id>]`, `lint`, `graph`, `start

@fact id=fct-jelmr2gx expect_rev=1
old: only the escher server's twelve tools
new: only the escher server's thirteen tools
old: the twelve `mcp__escher__{verb}` tools
new: the thirteen `mcp__escher__{verb}` tools

@pattern id=arch2-01
fixed: lists twelve verbs in three levels
@pattern id=arch2-02
fixed: first the eight asked of the held instance
@pattern id=arch2-03
regex: the thirteen keys (`NODE_FIELDS`|§Driver session) names
@pattern id=arch2-04
fixed: [NodeFieldSpec; 13]
@pattern id=arch2-05
fixed: — ten kinds
@pattern id=arch2-06
fixed: 12 verbs, 13 node keys
@pattern id=arch2-07
fixed: answers the twelve verbs of `VERBS`
@pattern id=arch2-08
fixed: the eight instance verbs
@pattern id=arch2-09
fixed: `VERBS` holds twelve
@pattern id=arch2-10
fixed: `NodeState` carries seven fields
@pattern id=arch2-11
fixed: eleven variants, `Parent`
@pattern id=arch2-12
fixed: the verbs are the twelve of `VERBS`
@pattern id=arch2-13
regex: (escher server's twelve tools|the twelve `mcp__escher__)
