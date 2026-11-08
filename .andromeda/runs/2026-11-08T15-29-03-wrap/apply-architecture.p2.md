@apply_row n=1 ref_file=fanout-architecture--cascade-g2.md ref_n=1 v=applied edit=E1
@apply_row n=2 ref_file=fanout-architecture--cascade-g3.md ref_n=1 v=applied edit=E2
@apply_row n=3 ref_file=fanout-architecture--cascade-g3.md ref_n=2 v=folded_into to_file=fanout-architecture--cascade-g3.md to_n=1
@apply_row n=4 ref_file=fanout-architecture--cascade-g3.md ref_n=3 v=folded_into to_file=fanout-architecture--cascade-g3.md to_n=1
@apply_row n=5 ref_file=fanout-architecture--cascade-g3.md ref_n=4 v=folded_into to_file=fanout-architecture--cascade-g3.md to_n=1
@apply_row n=6 ref_file=fanout-architecture--cascade-g3.md ref_n=5 v=folded_into to_file=fanout-architecture--cascade-g3.md to_n=1
@apply_row n=7 ref_file=fanout-architecture--cascade-g3.md ref_n=6 v=folded_into to_file=fanout-architecture--cascade-g3.md to_n=1
@apply_row n=8 ref_file=fanout-architecture--cascade-g3.md ref_n=7 v=folded_into to_file=fanout-architecture--cascade-g3.md to_n=1
@apply_row n=9 ref_file=fanout-architecture--cascade-g3.md ref_n=8 v=folded_into to_file=fanout-architecture--cascade-g3.md to_n=1
@apply_row n=10 ref_file=fanout-architecture--cascade-g3.md ref_n=9 v=folded_into to_file=fanout-architecture--cascade-g3.md to_n=1
@apply_row n=11 ref_file=fanout-architecture--cascade-g3.md ref_n=10 v=folded_into to_file=fanout-architecture--cascade-g3.md to_n=1
@apply_row n=12 ref_file=fanout-architecture--cascade-g3.md ref_n=11 v=folded_into to_file=fanout-architecture--cascade-g3.md to_n=1
@apply_row n=13 ref_file=fanout-architecture--cascade-g3.md ref_n=12 v=applied edit=E3
@apply_row n=14 ref_file=fanout-architecture--cascade-g3.md ref_n=13 v=applied edit=E4
@apply_row n=15 ref_file=fanout-architecture--cascade-g3.md ref_n=14 v=applied edit=E5
@apply_row n=16 ref_file=fanout-architecture--cascade-g3.md ref_n=15 v=applied edit=E6
@apply_row n=17 ref_file=fanout-architecture--cascade-g3.md ref_n=16 v=applied edit=E7
@apply_row n=18 ref_file=fanout-architecture--cascade-g3.md ref_n=17 v=folded_into to_file=fanout-architecture--cascade-g3.md to_n=16
@apply_row n=19 ref_file=fanout-architecture--cascade-g4.md ref_n=1 v=applied edit=E8
@apply_row n=20 ref_file=fanout-architecture--cascade-g4.md ref_n=2 v=applied edit=E9
@apply_row n=21 ref_file=fanout-architecture--cascade-g4.md ref_n=7 v=applied edit=E10
@apply_row n=22 ref_file=fanout-architecture--cascade-g4.md ref_n=8 v=folded_into to_file=fanout-architecture--cascade-g4.md to_n=7
@apply_row n=23 ref_file=fanout-architecture--cascade-g4.md ref_n=9 v=applied edit=E11
@apply_row n=24 ref_file=fanout-architecture--cascade-g4.md ref_n=10 v=applied edit=E12
@apply_row n=25 ref_file=fanout-architecture--cascade-g4.md ref_n=11 v=applied edit=E13
@apply_row n=26 ref_file=fanout-architecture--cascade-g4.md ref_n=12 v=applied edit=E14
@apply_row n=27 ref_file=fanout-architecture--cascade-g5.md ref_n=1 v=applied edit=E15
@fact id=fct-7zhttwuk expect_rev=1
old: `is_select`, the driver's select test, called by `execute.rs` `locate` alone,
new: `is_select`, the driver's select test, read within the workspace by `execute.rs`'s crate-private `target_of` alone (packages/escher-driver/src/execute.rs:443), which `locate` ends in and `Session::actions` also calls,

@fact id=fct-t7zq5feb expect_rev=2
old: the crate re-exports 57 names under
new: the crate re-exports 59 names under
old: `Finding` and `Reason` from private modules
new: `Finding` and `Reason` and the actions read's `ActionNode` and `VerbReading` from private modules, the last from the private module `actions`
old: `VERBS` lists thirteen verbs in three levels
new: `VERBS` lists fourteen verbs in three levels
old: first the nine asked of the held instance
new: first the ten asked of the held instance
old: (packages/escher-driver/src/schema.rs:382-427); then the three asked of the session itself
new: (packages/escher-driver/src/schema.rs:382-427), and `actions` (no argument, an argument refused `malformed` with `Fault::Unnamed`; result `nodes` (`FieldKind::ActionNodes`)), what each actionable node can take now, a further widening of the validated verb set (§Established Decisions → [Driver session]) (packages/escher-driver/src/schema.rs:429-452); then the three asked of the session itself
old: `nodes` and `states` fields — twelve kinds.
new: `nodes` and `states` fields, then `ActionNodes` (`action-nodes`), the kind of `actions`' `nodes` field, which shares `graph`'s field name and not its kind — thirteen kinds.
old: 13 verbs, 14 node keys
new: 14 verbs, 14 node keys
old: · `Graph` (validated from `("graph", [])`), with
new: · `Graph` (validated from `("graph", [])`) · `Actions` (validated from `("actions", [])`), with
old: `Graphed { graph: UiGraph }` (`Debug`, `Clone`, `PartialEq`), the diff being
new: `Graphed { graph: UiGraph }` (`Debug`, `Clone`, `PartialEq`) · `Actions { nodes: Vec<ActionNode> }` (packages/escher-driver/src/execute.rs:99-104), the diff being
old: settles nothing too;
new: settles nothing too, and the `Command::Actions` arm calls `Session::actions`, `record_result` listing it among the no-step outcomes, so an actions read dispatches nothing, runs no step and settles nothing either — `ActionNode { id, owner: Option<String>, keyed: bool, verbs: Vec<VerbReading> }` (`Debug`, `Clone`, `PartialEq`, `Eq`) and `VerbReading { verb: &'static str, cause: Option<Cause> }` (`Copy` too) (packages/escher-driver/src/actions.rs:14-37);
old: from the node ids of the graph an accepted `graph` reads, and from every snapshot
new: from the node ids of the graph an accepted `graph` reads, from the snapshot an `actions` read takes, every id it lists recorded, and from every snapshot
old: a part read and an accepted `graph`;
new: a part read, an accepted `graph` and an accepted `actions`;
old: a graph, its ids or its names,
new: a graph, its ids or its names, an actions answer, its ids or its verb readings,
old: `tools/list` answers the thirteen verbs of `VERBS` in order (`screenshot` after `scroll`, then `lint`, then `graph`)
new: `tools/list` answers the fourteen verbs of `VERBS` in order (`screenshot` after `scroll`, then `lint`, then `graph`, then `actions`)
old: and `lint` and `graph`, no field added)
new: and `lint`, `graph` and `actions`, no field added, an `actions` tool call parenting one `command`)
old: packages/escher-driver/src/json.rs:473-477); a paged
new: packages/escher-driver/src/json.rs:473-477); an `actions` outcome `{"nodes":[{"id":…,"owner":…,"keyed":…,"verbs":[{"verb":…,"cause":…}]}]}`, written by the crate-private `json::action_node`, `owner` and `cause` left out when absent, a cause by `Cause::name`, holding ids, owner ids, the table's verb words, closed cause words and booleans only, never a value, typed text or `NodeId` (packages/escher-driver/src/json.rs:251-272); a paged

@fact after=fct-q7ubtgz3 as=n1
- The actions read: `Session::actions(&mut self) -> Vec<ActionNode>` reads one snapshot and one `element_ids` map per call and records every id it reads in the session's record of ids; for each node it lists and each verb of the crate-private `target_verbs` — the table's `Level::Instance` rows with an `ArgKind::Id` argument, in table order, today `click` · `type` · `scroll` · `screenshot`, no verb name written in the read's code — it builds the call the crate-private `call_naming` spells, a required non-target argument given a fixed in-bound value by its `ArgKind` (`Text` the empty text), runs it through `validate` and then `check_target`, and reads `stale` for a node that resolves to no element (packages/escher-driver/src/actions.rs:40-102; packages/escher-driver/src/actions.rs:105-111; packages/escher-driver/src/actions.rs:113-136).

@fact after=@n1 as=n2
- The shared per-verb target check: the crate-private `check_target(harness, &Command, &Target) -> Result<Cleared, Refusal>`, with the crate-private `Cleared` `Point` · `Rect` · `Free`, is the one per-verb target check — `click` and `type` meet `action_point` (`disabled`, then `off-screen`, then `covered`), `scroll` meets none, and a `screenshot` given an `id` meets `in_view`, then an empty crop, then `can_paint_text` false (`fonts-unavailable`), before anything is painted; its callers are the four id-taking arms of `Session::execute`, which keep no check of their own, and `Session::actions`; `Target` is `pub(crate)`, and `locate` ends in the crate-private `target_of`, which `Session::actions` also calls (packages/escher-driver/src/execute.rs:498-526; packages/escher-driver/src/execute.rs:478-496; packages/escher-driver/src/execute.rs:430-439).

@fact after=@n2 as=n3
- The actions read changes nothing: after it `has_changes` stays false, the snapshot, both focus readings and the diff are unchanged and an open select list stays open, and `actions.rs` makes no `pump`, `tick`, `dispatch`, `scroll_into_view`, `close_select_list` or `focus_select` call (tests/blitz-tests/tests/stand_actions.rs:486-544; tests/blitz-tests/tests/stand_actions.rs:306-360).

@fact after=@n3 as=n4
- `actions` answers whether a call would be accepted, not what it would do — the stated limit «Accepted is not acting» (docs/capabilities.md:259-261) — and takes no argument, answering the whole screen at once, unpaged: Cells' answer reads 106,088 bytes for 678 nodes, under the 1,048,576-byte answer bound, which `stand_actions` holds in both layout modes (tests/blitz-tests/tests/stand_actions.rs:546-560; as measured at escher-0.3.0/chunks/2026-11-08-what-can-i-do-here/report.md); the unpaged answer is the stated limit «An unpaged actions answer» (docs/capabilities.md:274-276), owned by residual `res-7cc5bi4k` (target: next).

@fact id=fct-tofwjjiv expect_rev=2
old: a verb outside the nine instance verbs, a `lint` or a `graph` step kept like every other with its answer whole,
new: a verb outside the ten instance verbs, a `lint`, a `graph` or an `actions` step kept like every other with its answer whole,
old: closed keys, the nine instance verbs,
new: closed keys, the ten instance verbs,

@fact id=fct-6m5fkd33 expect_rev=2
old: and `VERBS` holds thirteen, `graph` listed after `lint`
new: and `VERBS` holds fourteen, `graph` listed after `lint` and `actions` after `graph`

@fact id=fct-g75i5t3p expect_rev=2
old: and `snapshot`, `screenshot`, `lint` and `graph`, which run no step
new: and `snapshot`, `screenshot`, `lint`, `graph` and `actions`, which run no step

@fact id=fct-kllkxz3d expect_rev=3
old: or the `data-dioxus-id` attribute the bridge writes for every listener but `onmounted` and that stays, with its event attribute, on an element whose listener a re-render removed, so the check over-includes such an element by its listener, the conservative side (read, not run)
new: or its current listeners, read from `element_facts` (the vdom walk's exact reading), holding an event other than `mounted` — the crate-private `Readings`, which `actionable_nodes` also reads, so neither reader counts an element whose listener a re-render removed, while the `data-dioxus-id` mark stays, written by the bridge and read by event dispatch
old: `interactive_role` and `listener` (each `bool`)
new: `interactive_role` and `listener` (each `bool`, `listener` true where the element's current listeners hold an event other than `mounted`)
old: the check has no wire form, and no driver, CLI or MCP command exposes it yet;
new: the check's first wire form is `ActionableNode`'s `keyed` reading (true where the id holds no `/`), which the `actions` verb carries on every carrier, PROVISIONAL until the founder's own word;

@fact id=fct-ie5oemgh expect_rev=2
old: the UI graph's handlers and `NodeState.clickable` read these, never the listener mark
new: the UI graph's handlers, `NodeState.clickable` and, through the crate-private `Readings`, `unkeyed_actionable`'s listener leg and `actionable_nodes` read these, never the listener mark

@fact after=fct-ie5oemgh as=n5
- The one actionable rule: the crate-private `Readings` holds an element actionable when `Node::is_focussable()` holds, or its role is one of the 27 interactive roles (`is_interactive`), or its current listeners, from `element_facts` (the vdom walk's exact reading), hold an event other than `mounted`; `unkeyed_actionable` and `actionable_nodes` both read it and share the crate-private `fn roles` (packages/dioxus-native-dom/src/actionable.rs:62-81).

@fact after=fct-xabeutlj as=n6
- The actionable nodes: `DioxusDocument::actionable_nodes(&self) -> Vec<ActionableNode>`, behind `accessibility`, is read-only: it returns `ActionableNode { id: String, owner: Option<String>, keyed: bool }` (`Debug`, `Clone`, `PartialEq`, re-exported from the crate root beside `UnkeyedActionable`) over the UI graph's nodes in snapshot pre-order, keeping each node the actionable rule holds for or whose graph `owner` is `Some`, `keyed` true where the id holds no `/` — the actionable-key check's first wire form, PROVISIONAL until the founder's own word (packages/dioxus-native-dom/src/actionable.rs:49-60; packages/dioxus-native-dom/src/actionable.rs:118-146; packages/dioxus-native-dom/src/lib.rs:37).

@fact id=fct-c5e4c7ao expect_rev=2
old: the verbs are the thirteen of `VERBS`
new: the verbs are the fourteen of `VERBS`
old: `lint`, `graph`, `start <task> [--record <dir>]`
new: `lint`, `graph`, `actions`, `start <task> [--record <dir>]`

@fact id=fct-jelmr2gx expect_rev=2
old: only the escher server's thirteen tools
new: only the escher server's fourteen tools
old: the thirteen `mcp__escher__{verb}` tools
new: the fourteen `mcp__escher__{verb}` tools
@pattern id=arch2-01
fixed: called by `execute.rs` `locate` alone
@pattern id=arch2-02
fixed: re-exports 57 names
@pattern id=arch2-03
fixed: lists thirteen verbs
@pattern id=arch2-04
fixed: the nine asked of the held instance
@pattern id=arch2-05
fixed: — twelve kinds
@pattern id=arch2-06
fixed: 13 verbs, 14 node keys
@pattern id=arch2-07
fixed: answers the thirteen verbs of `VERBS`
@pattern id=arch2-08
fixed: the nine instance verbs
@pattern id=arch2-09
fixed: `VERBS` holds thirteen
@pattern id=arch2-10
fixed: `lint` and `graph`, which run no step
@pattern id=arch2-11
fixed: over-includes such an element by its listener
@pattern id=arch2-12
fixed: the check has no wire form
@pattern id=arch2-13
fixed: the verbs are the thirteen of `VERBS`
@pattern id=arch2-14
fixed: escher server's thirteen tools
@pattern id=arch2-15
fixed: the thirteen `mcp__escher__{verb}` tools
