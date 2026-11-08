@carried n=1 label=architecture/standard-contracts fact=fct-6m5fkd33
why: A new coverage verb must move `VERBS` from fourteen to fifteen in its listed order, with every pin that holds that count.
quote: its `command` span recording `verb` and, refused, `cause` alone, no field added, and `VERBS` holds fourteen, `graph` listed after `lint` and `actions` after `graph`
@carried n=1 run=2 label=architecture fact=fct-ksgwc2yv
quote: the command and refusal schema stated once as `'static` data — fourteen verbs, eight closed causes, no refusal holding what a call supplied; the `v3` wire, the command line, the MCP server and the `command` and `tool_call` spans carry it; no `NodeId` crosses, and nothing logs an id, a name or a value.
@carried n=2 label=architecture/standard-contracts fact=fct-g75i5t3p
why: A coverage read runs no step and must wait on nothing, and the owed replay test must record and replay on an app that meets no load so its kept answers hold.
quote: and `snapshot`, `screenshot`, `lint`, `graph` and `actions`, which run no step, wait on nothing.
@carried n=2 run=2 label=architecture/standard-contracts fact=fct-g75i5t3p
quote: A kept check whose app meets a load can read `settled` differently on a replay whose load lands at another time (recorded, not established); the two committed Counter checks meet no load
@carried n=3 label=architecture/standard-contracts fact=fct-mi62uuno
why: The coverage command must read node ids and handler event names as the `graph` verb carries them and carry nothing beyond them, its answer written by hand in the `json` module.
quote: `graph`, the ninth instance verb, takes no argument — an argument is refused `malformed` with nothing run — and answers `Outcome::Graphed`, the screen's graph read through `DioxusDocument::graph` (§Dioxus DOM bridge), its node ids feeding the session's record of ids; its written form, by hand in the crate-private `json` module, is `{"nodes":[…],"states":[…]}` — a node keyed `id` · `owner` · `component` · `handlers` · `reads`, `owner` and `component` left out when absent, a handler `event` · `writes`
@carried n=3 run=2 label=architecture fact=fct-h6zygr3f
quote: Every answer is written by hand in the crate-private `json` module — no serialization crate writes one, `serde_json` only reading MCP requests and kept checks — and no command's ending writes back anything the caller typed.
@carried n=4 label=architecture/standard-contracts fact=fct-ahl2enqw
why: An unpaged coverage answer over Cells' handlers must stay within the same answer bound the graph's answer is held to.
quote: `graph` answers the whole screen at once, and Cells' answer reads 293,869 bytes, under the 1,048,576-byte answer bound, which `stand_graph` holds in both layout modes
@carried n=5 label=architecture/standard-contracts fact=fct-q7ubtgz3
why: The chunk's edits to the graph and registry files must keep `stand_graph` holding the graph's answer, its read-only reading and its bound.
quote: `stand_graph`, in both layout modes, holds the `graph` verb's answer equal to `DioxusDocument::graph`, every graph node a snapshot node, no served id holding the reserved `///` form, a graph read changing nothing and holding no typed text, and the whole Cells answer within the answer bound
@carried n=6 label=architecture/standard-contracts fact=fct-z3b2jzgi
why: The owed replay test replays a kept `actions` step through `Check::replay`, so the actions read it re-asserts must read and record its ids as stated.
quote: `Session::actions(&mut self) -> Vec<ActionNode>` reads one snapshot and one `element_ids` map per call and records every id it reads in the session's record of ids
@carried n=6 run=2 label=architecture fact=fct-elnqiowc
quote: The recording keeps a session's answered calls as a kept check, a masked `type` as its stand-in and a shot without its pixels; `Check::replay` runs them again through `Session::run`
@carried n=7 label=architecture/standard-contracts fact=fct-kllkxz3d
why: The record of reached (element, event) handlers must sit on the bridge's `data-dioxus-id` event path, and the coverage list must be keyed on stable ids so the reserved virtual-node form stays open for 0.4.0.
quote: DOM events route to the vdom via the nearest `data-dioxus-id` attribute parsed as `usize` (packages/dioxus-native-dom/src/dioxus_document.rs:49-57; packages/dioxus-native-dom/src/dioxus_document.rs:438-444)
@carried n=7 run=2 label=architecture/standard-contracts fact=fct-kllkxz3d
quote: beside the four tiers the grammar states a fifth, reserved form, `{owner}///{key}`, which no tier can hold and no caller produces, the walk not calling it — `VIRTUAL_KEY_SEPARATOR` (`"///"`) and `virtual_node_id(owner, key) -> Option<String>`, public and re-exported, spell it, answering `None` for an empty or `/`-bearing key (packages/dioxus-native-dom/src/element_id.rs:39-48)
@carried n=8 label=architecture/standard-contracts fact=fct-uzcbujjs
why: The exercised record must hold on every app, registry or not, while the registry's dispatch stays set only where a registry stands.
quote: where `runtime.has_context::<SignalRegistry>(ROOT)` holds, `DioxusEventHandler::handle_event` sets the registry's dispatch around `runtime.handle_event` and clears it after; a page with no registered state reaches no registry and dispatches as before — a change on every Dioxus document's event path, inert without a registry
@carried n=8 run=2 label=architecture fact=fct-srb7slak
quote: an app registers graphed state through the ungated `use_graph_state` (`DeclaredWrite`, `SignalRegistry`), and the bridge's `handle_event` sets the registry's dispatch around the vdom's only where a registry stands.
@carried n=9 label=architecture/standard-contracts fact=fct-ie5oemgh
why: The coverage count must read handlers through the graph's exact listener reading, never the over-reporting listener mark.
quote: The exact reading is the crate-private `element_listeners`, `mounted` included, beside `element_facts`, each element's owning scope and component name; the UI graph's handlers, `NodeState.clickable` and, through the crate-private `Readings`, `unkeyed_actionable`'s listener leg and `actionable_nodes` read these, never the listener mark
@carried n=10 label=architecture/standard-contracts fact=fct-ljxizw75
why: Which handlers the coverage count holds must stay consistent with the one actionable rule, which leaves `mounted` out.
quote: the crate-private `Readings` holds an element actionable when `Node::is_focussable()` holds, or its role is one of the 27 interactive roles (`is_interactive`), or its current listeners, from `element_facts` (the vdom walk's exact reading), hold an event other than `mounted`
@carried n=10 run=2 label=architecture fact=fct-d5zuqqfb
quote: both readers reading the one actionable rule — focusable, an interactive role, or current listeners holding an event other than `mounted`.
@carried n=11 label=architecture/standard-contracts fact=fct-xabeutlj
why: The command must read `GraphNode.handlers` and `GraphHandler.event` from a read-only graph whose shape stays as stated.
quote: `DioxusDocument::graph(&self) -> UiGraph`, behind `accessibility`, is read-only: `UiGraph { nodes, states }` holds one `GraphNode { id, owner, component, handlers, reads }` per snapshot node, in pre-order; a handler is `GraphHandler { event, writes }`
@carried n=12 label=architecture/standard-contracts fact=fct-vvyrpx4f
why: The replay test must compare the kept `actions` answer of `ActionableNode` fields key by key, their shape unchanged.
quote: `DioxusDocument::actionable_nodes(&self) -> Vec<ActionableNode>`, behind `accessibility`, is read-only: it returns `ActionableNode { id: String, owner: Option<String>, keyed: bool }`
@carried n=13 label=architecture/standard-contracts fact=fct-z43gfgk7
why: The exercised record must leave the graph's `unconfirmed` reading, which rides the registry's dispatch, as stated, and the coverage key must read the graph's `owner`.
quote: its `writers` lists only writes a run confirmed or flagged, while `declared` keeps the declaration as authored; a write resolves to the first element of its chain carrying a listener of its event, or to nothing when a node of the chain before that element no longer resolves; `unconfirmed` fires only once a dispatch of the handler's event passed through its element and no run wrote the state
@carried n=13 run=2 label=architecture/standard-contracts fact=fct-z43gfgk7
quote: `owner` is the stable id of `BaseDocument::select_of` for an option or an optgroup
@carried n=14 label=architecture/standard-contracts fact=fct-b7zqmrdo
why: Edits to `signal_registry.rs` must keep the registry recording writes against the standing dispatch and its `Debug` printing counts alone.
quote: the first registering hook provides a `SignalRegistry` context at `ScopeId::ROOT`; each entry holds a `ReactiveContext::new_with_callback` whose callback records one write against the standing dispatch — its chain head first, and its event name — and the writing scope, and leaves with its scope through `use_drop`; `DeclaredWrite { id, event }` names an element's stable id or a prefix ending in `*`, and an event without `on`; the registry's `Debug` prints counts alone
@carried n=15 label=architecture/standard-contracts fact=fct-c5e4c7ao
why: A coverage verb must land in `escher-session`'s verb set and answer one line of JSON under the four exit codes.
quote: the verbs are the fourteen of `VERBS` — `snapshot`, `click --id`, `type --id --text`, `press --key [--shift]`, `advance --ms`, `scroll --id`, `screenshot [--id <id>]`, `lint`, `graph`, `actions`, `start <task> [--record <dir>]`, `status`, `stop` and, since 2026-10-12-self-description, `help`
@carried n=15 run=2 label=architecture/standard-contracts fact=fct-c5e4c7ao
quote: a command answers one uncoloured line of JSON on stdout and exits `0` accepted · `1` refused · `2` usage error
@carried n=15 run=3 label=architecture fact=fct-fukxpbvx
quote: `escher-session <verb> [arguments] --session <dir>`, the driver's command line over six served stand tasks
@binding n=1
**UI graph ↔ bridge dispatch**: the coverage list is keyed on the graph's node id and owner and a handler's event (architecture/standard-contracts fct-xabeutlj), (architecture/standard-contracts fct-z43gfgk7), and its exercised record is read on the bridge's event path, holding on every app while the registry's dispatch stays set only where a registry stands (architecture/standard-contracts fct-uzcbujjs), (architecture/standard-contracts fct-b7zqmrdo).
@binding n=2
**verb count ↔ CLIs ↔ driver session body**: a new coverage verb moves `VERBS` from fourteen to fifteen in the one schema the wire, command line, MCP server and spans carry, and in `escher-session`'s verb list (architecture/standard-contracts fct-6m5fkd33), (architecture/standard-contracts fct-c5e4c7ao), (architecture fct-ksgwc2yv).
@binding n=3
**actions answer ↔ load wait ↔ replay**: the owed replay test compares a kept `actions` answer key by key on a fresh session (architecture/standard-contracts fct-vvyrpx4f), (architecture fct-elnqiowc), on an app that meets no load so `settled` cannot differ (architecture/standard-contracts fct-g75i5t3p).
@binding n=4
**listener reading ↔ actionable rule**: the handlers the coverage counts are the graph's from `element_listeners`, `mounted` included there, while the actionable rule leaves `mounted` out; the count must settle that fork consistently (architecture/standard-contracts fct-ie5oemgh), (architecture/standard-contracts fct-ljxizw75).
