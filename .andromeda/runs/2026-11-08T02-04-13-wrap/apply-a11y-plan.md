@apply_row n=1 ref_file=fanout-a11y-plan--cascade-g2.md ref_n=12 v=applied edit=E1
@apply_row n=2 ref_file=fanout-a11y-plan.md ref_n=1 v=applied edit=E2
@apply_row n=3 ref_file=fanout-a11y-plan--cascade-g2.md ref_n=11 v=folded_into to_file=fanout-a11y-plan.md to_n=1
@apply_row n=4 ref_file=fanout-a11y-plan--cascade-g1.md ref_n=1 v=applied edit=E3
@apply_row n=5 ref_file=fanout-a11y-plan--cascade-g1.md ref_n=2 v=applied edit=E4
@apply_row n=6 ref_file=fanout-a11y-plan--cascade-g2.md ref_n=1 v=applied edit=E5
@apply_row n=7 ref_file=fanout-a11y-plan--cascade-g2.md ref_n=2 v=applied edit=E6
@apply_row n=8 ref_file=fanout-a11y-plan--cascade-g2.md ref_n=3 v=applied edit=E7
@apply_row n=9 ref_file=fanout-a11y-plan--cascade-g2.md ref_n=4 v=applied edit=E8
@apply_row n=10 ref_file=fanout-a11y-plan--cascade-g2.md ref_n=5 v=applied edit=E9
@apply_row n=11 ref_file=fanout-a11y-plan--cascade-g2.md ref_n=6 v=applied edit=E10
@apply_row n=12 ref_file=fanout-a11y-plan--cascade-g2.md ref_n=7 v=applied edit=E11
@apply_row n=13 ref_file=fanout-a11y-plan--cascade-g2.md ref_n=8 v=applied edit=E12
@apply_row n=14 ref_file=fanout-a11y-plan--cascade-g2.md ref_n=9 v=applied edit=E13
@apply_row n=15 ref_file=fanout-a11y-plan--cascade-g2.md ref_n=10 v=applied edit=E14
@fact id=fct-uz4kdss3 expect_rev=1
old: else a checkable role's `toggled`;
new: else a checkable role's `toggled`, and `clickable` from the vdom walk's current `click` listeners on an element that is no disabled form control;
old: read that model and nothing else — never
new: read that model and nothing else, and the read-only UI graph (`DioxusDocument::graph`) holds one node per snapshot node, names and closed words only — never

@fact id=fct-77ewqepn expect_rev=2
old: no snapshot field says an element reacts to a click — that reading measured by link 2, its field owed to the route entry «The UI graph»
new: the snapshot field `clickable` says an element reacts to a click, read from the vdom walk's current listeners and not the listener mark

@fact id=fct-d5bs73pt expect_rev=1
old: all five of which its crate doc names, and dioxus-native forwards
new: all five of which its crate doc names, and its read-only UI graph reader (`DioxusDocument::graph`, a `UiGraph`, its `graph` module), while its `signal_registry` module (`use_graph_state`, `DeclaredWrite`, `SignalRegistry`) is not gated; and dioxus-native forwards

@fact after=fct-v4en4yup as=n1
- Apps register state for the UI graph through dioxus-native-dom's `use_graph_state` with authored `DeclaredWrite` declarations — an element's stable id or a prefix ending in `*`, and an event named without `on` — wrapped in seven_guis as `use_graph_signal`; where a `SignalRegistry` is registered, the bridge's `DioxusEventHandler::handle_event` sets the registry's dispatch (its chain and its event) around `runtime.handle_event` and clears it after, so each signal write is recorded against that dispatch and the writing scope, and resolves to the first element of its chain carrying a listener of its event, to nothing where a node of the chain before that element no longer resolves; a write is flagged `undeclared` where no declaration names it, and a declared write `unconfirmed` only once a dispatch of the handler's event passed through its element and no run wrote the state; a page with no registered state reaches no registry and dispatches every event as before (packages/dioxus-native-dom/src/signal_registry.rs:47-63; packages/dioxus-native-dom/src/graph.rs:294-320; examples/seven_guis/src/tasks/mod.rs:4-15) — as measured at escher-0.3.0/chunks/2026-11-07-ui-graph/report.md

@fact after=fct-kf4qrij2 as=n2
- `stand_graph` asserts in both layout modes that every graph node is a snapshot node, that no served id holds the reserved separator `///` (the reserved `{owner}///{key}` form, spelled by `virtual_node_id`, which the walk never calls), and that a graph read changes nothing and holds no typed text (tests/blitz-tests/tests/stand_graph.rs:282-339; tests/blitz-tests/tests/stand_graph.rs:341-379) — as measured at escher-0.3.0/chunks/2026-11-07-ui-graph/report.md

@fact after=fct-ge4n5uhw as=n3
- `NodeState.clickable` is a `bool` after `focused`, true where the vdom walk's current listeners hold `click` and the element is no disabled form control (`ElementData::is_disabled`), read from the one snapshot model and the walk with no second role mapping; `Snapshot::to_text` writes the token `clickable` after `focused` only when true, the written JSON node always carries `clickable` beside `focused`, and `Snapshot::diff` reads it as `Reading::Clickable`, one of `Reading::ALL`'s twelve (packages/dioxus-native-dom/src/snapshot_text.rs:128-130) — as measured at escher-0.3.0/chunks/2026-11-07-ui-graph/report.md

@fact after=@n3 as=n4
- `DioxusDocument::graph`, behind `accessibility`, is one more read-only reader beside the snapshot: a `UiGraph` holding one `GraphNode` per snapshot node in pre-order — its id, its `owner` (for an option or an optgroup, the stable id of its select), its component, its handlers with their declared writes, and the states it reads — and per registered state its declared writes, the writes a run confirmed or flagged (`undeclared`, `unconfirmed`) and its reading components; the answer carries ids, component names, state names, handler event names, closed flag words and booleans only, never a value (packages/dioxus-native-dom/src/graph.rs:24-48; packages/dioxus-native-dom/src/graph.rs:136-284) — as measured at escher-0.3.0/chunks/2026-11-07-ui-graph/report.md

@fact id=fct-yfrcl4qo expect_rev=1
old: `NODE_FIELDS` reads thirteen, so
new: `NODE_FIELDS` reads fourteen, so
old: 2026-10-31-select-popup/report.md
new: 2026-10-31-select-popup/report.md, the count at escher-0.3.0/chunks/2026-11-07-ui-graph/report.md

@fact id=fct-pfylw46r expect_rev=2
old: no snapshot field says that an element reacts to a click (`NODE_FIELDS` thirteen) — that reading measured by link 2, its snapshot field owed to the route entry «The UI graph» on the founder's C3 «Замерить сейчас, поле позже (Рекомендую)» (2026-10-14), this count its starting reading
new: the snapshot field `NodeState.clickable` (`NODE_FIELDS` fourteen) says that an element reacts to a click, read from the one snapshot model and the vdom walk's current listeners, not the listener mark
old: Cells' count at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md
new: Cells' count at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md, the click field at escher-0.3.0/chunks/2026-11-07-ui-graph/report.md

@fact after=fct-pfylw46r as=n5
- `stand_non_lean_tasks` asserts a `CLICKABLE` table of the elements that read `clickable` and state no role, per task in both layout modes — Counter, Temperature Converter, Flight Booker, Timer and CRUD 0 each, Circle Drawer 1, Cells 676 — beside the listener mark's count, Cells 677 (tests/blitz-tests/tests/stand_non_lean_tasks.rs:66-75; tests/blitz-tests/tests/stand_non_lean_tasks.rs:117-127) — as measured at escher-0.3.0/chunks/2026-11-07-ui-graph/report.md

@fact id=fct-7vfy7k5v expect_rev=1
old: no snapshot field is added (`NODE_FIELDS` thirteen)
new: the snapshot field `NodeState.clickable` holds that reading (`NODE_FIELDS` fourteen, as measured at escher-0.3.0/chunks/2026-11-07-ui-graph/report.md)

@fact after=fct-7vfy7k5v as=n6
- On the stand `clickable` reads true on `counter-increment` and on each of Cells' 676 data cells, and false on `counter-value` and `cells-grid`, from the one snapshot model and not the listener mark — as measured at escher-0.3.0/chunks/2026-11-07-ui-graph/report.md

@fact id=fct-cbzuzrms expect_rev=1
old: `mounted` included (packages
new: `mounted` included, and the bridge's vdom walk yields them per element (`element_listeners`), the reading `NodeState.clickable` and the graph's handlers take, so neither over-reports as the mark does (packages/dioxus-native-dom/src/element_id.rs:304-311; packages
old: dioxus-runtime-probe/report.md
new: dioxus-runtime-probe/report.md, the walk's listeners at escher-0.3.0/chunks/2026-11-07-ui-graph/report.md

@fact id=fct-bvkme5jk expect_rev=1
old: which measures link 2's «reacts to a click» reading (§7), is
new: which measures link 2's «reacts to a click» reading, the snapshot field `NodeState.clickable` that `stand_graph` and `stand_non_lean_tasks` read (§7), is
old: 2026-11-04-dioxus-runtime-probe/report.md)
new: 2026-11-04-dioxus-runtime-probe/report.md; the field at escher-0.3.0/chunks/2026-11-07-ui-graph/report.md)

@pattern id=a11y-01
fixed: no snapshot field says
@pattern id=a11y-02
fixed: field owed to the route entry «The UI graph»
@pattern id=a11y-03
fixed: no snapshot field is added
@pattern id=a11y-04
regex: NODE_FIELDS` (reads )?thirteen
