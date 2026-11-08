@proposal n=1 detector=D-cascade severity=warning file=standard-contracts anchor=fct-tofwjjiv basis_doc=doc:2026-11-07-ui-graph/report basis_fact=fct-yr4375o3
section: ## Standard Contracts
change: Both readings of «the eight instance verbs» become «the nine instance verbs»: a call outside them is not kept, and Check::from_bytes reads the nine. A graph step is kept like every other step, its answer whole.
sidecar: standard-contracts recording: the instance verbs are now nine, with graph (2026-11-07-ui-graph)
rationale: The report's Schema / config moves the instance verbs from 8 to 9, and its Counts mark standard-contracts.md:234-235 stale.
@proposal n=2 detector=D-cascade severity=warning file=standard-contracts anchor=fct-6m5fkd33 basis_doc=doc:2026-11-07-ui-graph/report basis_fact=fct-yr4375o3
section: ## Standard Contracts
change: «`VERBS` holds twelve» becomes «`VERBS` holds thirteen, `graph` listed after `lint`». The phrase «the eighth instance verb» stays as it is.
sidecar: standard-contracts lint: VERBS is now thirteen (2026-11-07-ui-graph)
rationale: The report moves VERBS from 12 to 13. Its Counts say the row naming lint the eighth instance verb is still true, since graph is the ninth.
@proposal n=3 detector=D-cascade severity=warning file=standard-contracts anchor=fct-g75i5t3p basis_doc=doc:2026-11-07-ui-graph/report basis_fact=fct-zklbw7ne
section: ## Standard Contracts
change: «`snapshot`, `screenshot` and `lint`, which run no step, wait on nothing» becomes «`snapshot`, `screenshot`, `lint` and `graph`, which run no step, wait on nothing».
sidecar: standard-contracts load wait: graph joins the verbs that run no step (2026-11-07-ui-graph)
rationale: The report states Outcome::Graphed runs no step.
@proposal n=4 detector=D-new-detail severity=warning file=standard-contracts anchor=fct-g75i5t3p fact=F5 basis_doc=doc:2026-11-07-ui-graph/report basis_fact=fct-zklbw7ne
section: ## Standard Contracts
change: - The `graph` verb, the ninth instance verb (`schema::GRAPH` after `LINT`, no argument, fields `nodes` `graph-nodes` · `states` `graph-states`; `Command::Graph` from `("graph", [])`; `Outcome::Graphed { graph: UiGraph }`, its `command` span recording `verb` alone): its answer is written by hand in the `json` module as `{"nodes":[{id,owner?,component?,handlers:[{event,writes:[{state,declared,confirmed,flag?}]}],reads}],"states":[{name,component?,declared:[{id,event}],writers:[{id,event,declared,confirmed,flag?}],readers}]}`. An argument is refused `malformed` with nothing run, and the answer feeds the record of ids from the graph's node ids.
sidecar: standard-contracts driver session: the graph verb's written form and its refusal (2026-11-07-ui-graph)
rationale: These are the report's escher-driver symbols and the JSON it gives for the graph verb.
@proposal n=5 detector=D-new-detail severity=warning file=standard-contracts anchor=fct-g75i5t3p fact=F6 basis_doc=doc:2026-11-07-ui-graph/report basis_fact=fct-reiinlg2
section: ## Standard Contracts
change: - The graph is unpaged: `graph` takes no argument and answers the whole screen at once. Cells reads 293,869 bytes (547,685 at first build, 522,692 after the readers narrowing), under the answer bound `stand_graph` holds. The remainder, each cell's declared `click` writes, is the stated limit «An unpaged graph», owned by residual `res-7cc5bi4k`.
sidecar: standard-contracts driver session: the unpaged graph, its Cells size and its residual (2026-11-07-ui-graph)
rationale: The report's Insufficient fixes and its Counts give the measured 293,869 B and the residual that owns the rest.
@proposal n=6 detector=D-arch-resources severity=warning file=standard-contracts anchor=fct-zmumd7nc basis_doc=doc:2026-11-07-ui-graph/report basis_fact=fct-zklbw7ne
section: ## Standard Contracts
change: dioxus_native's re-export of `dioxus_native_dom` also carries the graph registry's `DeclaredWrite`, `SignalRegistry` and `use_graph_state`.
sidecar: standard-contracts high-level launch: dioxus_native re-exports the graph registry (2026-11-07-ui-graph)
rationale: The report says these three are exported through dioxus-native's `pub use dioxus_native_dom::*`.
@proposal n=7 detector=D-cascade severity=warning file=standard-contracts anchor=fct-kllkxz3d basis_doc=doc:2026-11-07-ui-graph/report basis_fact=fct-zklbw7ne
section: ## Standard Contracts
change: `NodeState` carries eight fields, with `clickable: bool` after `focused`. It is true where the vdom walk's current listeners hold `click` and the element is no `ElementData::is_disabled` form control. The text form writes the token ` clickable` after `focused`, only when true. `Reading` has twelve variants, `Clickable` after `Focused` (`Reading::ALL` twelve).
sidecar: standard-contracts bridge: NodeState.clickable, its text token and Reading twelve (2026-11-07-ui-graph)
rationale: The report's Symbols list `NodeState.clickable` and `Reading::Clickable`, and its Schema / config moves `Reading::ALL` from 11 to 12.
@proposal n=8 detector=D-arch-resources severity=warning file=standard-contracts anchor=fct-kllkxz3d basis_doc=doc:2026-11-07-ui-graph/report basis_fact=fct-zklbw7ne
section: ## Standard Contracts
change: Beside the four tiers, the id grammar states a fifth, reserved form, `{owner}///{key}`. `VIRTUAL_KEY_SEPARATOR` (`"///"`) and `virtual_node_id(owner, key) -> Option<String>` are public and re-exported; the function answers `None` for an empty or `/`-bearing key. No tier can hold `///`, and no caller produces the form, since the walk does not call it. This is the founder's fork 2.
sidecar: standard-contracts bridge: the reserved virtual-node id form and virtual_node_id (2026-11-07-ui-graph)
rationale: The report's Symbols give the reserved id form. Outcome (arch): no served id holds `///`.
@proposal n=9 detector=D-arch-resources severity=warning file=standard-contracts anchor=fct-kllkxz3d basis_doc=doc:2026-11-07-ui-graph/report basis_fact=fct-zklbw7ne
section: ## Standard Contracts
change: `DioxusEventHandler::handle_event`: when `runtime.has_context::<SignalRegistry>(ROOT)` holds, it sets the registry's dispatch around `runtime.handle_event` and clears it after. A page with no registered state reaches no registry and dispatches as before. A test-only `DioxusDocument::dispatch_to_vdom` (`#[cfg(test)]`) was added.
sidecar: standard-contracts bridge: the event dispatch now sets the graph registry's dispatch (2026-11-07-ui-graph)
rationale: The report says the bridge dispatch change touches every Dioxus document's event path and is inert without a registry.
@proposal n=10 detector=D-new-detail severity=warning file=standard-contracts anchor=fct-kllkxz3d fact=F1 basis_doc=doc:2026-11-07-ui-graph/report basis_fact=fct-zklbw7ne
section: ## Standard Contracts
change: - The UI graph: `DioxusDocument::graph(&self) -> UiGraph` (behind `accessibility`) is read-only. `UiGraph { nodes, states }` holds one `GraphNode { id, owner, component, handlers, reads }` per snapshot node, in pre-order. A handler is `GraphHandler { event, writes }` and a write is `GraphWrite { state, declared, confirmed, flag }`. A state is `GraphState { name, component, declared, writers, readers }` and a writer is `GraphWriter { id, event, declared, confirmed, flag }`. `WriteFlag::{Undeclared, Unconfirmed}` has `word()`.
sidecar: standard-contracts bridge: the read-only UI graph and its types (2026-11-07-ui-graph)
rationale: The report's Symbols list the graph reader and its types.
@proposal n=11 detector=D-new-detail severity=warning file=standard-contracts anchor=fct-kllkxz3d fact=F2 basis_doc=doc:2026-11-07-ui-graph/report basis_fact=fct-zklbw7ne
section: ## Standard Contracts
change: - Graph semantics:
  - `owner` is the stable id of `BaseDocument::select_of` for an option or optgroup.
  - A repeated (name, component) registration reads `{name}:{k}`.
  - `readers` names each reading component once, in the pre-order of its first node. A state only the app root reads appears in its nodes' `reads` alone.
  - `writers` lists only writes a run confirmed or flagged, while `declared` keeps the authored declaration.
  - A write resolves to the first chain element carrying a listener of its event, or to nothing when an earlier chain node is gone.
  - `unconfirmed` fires only after a dispatch of the handler's event passed through its element and no run wrote the state.
sidecar: standard-contracts bridge: graph semantics for owner, ordinal, readers, writers, resolution and unconfirmed (2026-11-07-ui-graph)
rationale: These are the report's graph reader semantics, including the operator's fixes inputs#I7 and inputs#I8 and the stop at a gone node.
@proposal n=12 detector=D-new-detail severity=warning file=standard-contracts anchor=fct-kllkxz3d fact=F3 basis_doc=doc:2026-11-07-ui-graph/report basis_fact=fct-zklbw7ne
section: ## Standard Contracts
change: - The registry: `use_graph_state(name: &'static str, writers: &'static [DeclaredWrite], subscribers: Subscribers)` takes dioxus-core types alone.
  - The first registering hook provides a `SignalRegistry` context at `ScopeId::ROOT`.
  - Each entry holds a `ReactiveContext::new_with_callback`. The callback records one write against the standing dispatch (chain head first, and its event name) and the writing scope.
  - An entry leaves with its scope through `use_drop`.
  - `DeclaredWrite { id, event }` names a stable id or a prefix ending in `*`, and an event without `on`.
  - The registry's `Debug` prints counts alone.
sidecar: standard-contracts bridge: use_graph_state, SignalRegistry and DeclaredWrite (2026-11-07-ui-graph)
rationale: These are the report's registry symbols.
@proposal n=13 detector=D-new-detail severity=warning file=standard-contracts anchor=fct-kllkxz3d fact=F4 basis_doc=doc:2026-11-07-ui-graph/report basis_fact=fct-rn2ymeq6
section: ## Standard Contracts
change: - The `Signal`-returning `use_graph_signal(name, writers, init)` lives in seven_guis `tasks/mod.rs`, not in dioxus-native-dom: dioxus-native-dom names no crate that defines `Signal`, and the manifest freeze forbids adding one. `tasks::writes(id, event)` is a `const fn`.
sidecar: standard-contracts bridge: use_graph_signal lives in seven_guis, not dioxus-native-dom (2026-11-07-ui-graph)
rationale: These are the report's reverted API facts and Deviation 1.
@proposal n=14 detector=D-new-detail severity=warning file=standard-contracts anchor=fct-kllkxz3d fact=F7 basis_doc=doc:2026-11-07-ui-graph/report basis_fact=fct-ej52fywd
section: ## Standard Contracts
change: - `stand_graph` runs in both layout modes. It holds the verb equal to `DioxusDocument::graph`, every graph node to a snapshot node, no served id to the `///` form, a graph read to changing nothing and holding no typed text, and the Cells answer to the bound. `GRAPH_DRIVES` in `tests/blitz-tests/tests/common/mod.rs` drives each conditional write once, so no stand write reads a flag.
sidecar: standard-contracts bridge: what stand_graph holds, and GRAPH_DRIVES (2026-11-07-ui-graph)
rationale: These are the report's stand_graph tests (6 passed) and the GRAPH_DRIVES decision.
@proposal n=15 detector=D-cascade severity=warning file=standard-contracts anchor=fct-ie5oemgh basis_doc=doc:2026-11-07-ui-graph/report basis_fact=fct-zklbw7ne
section: ## Standard Contracts
change: The vdom walk's exact reading of current listeners is the crate-private `element_listeners`, `mounted` included. Beside it, `element_facts` gives each element's owning scope and component name. The graph's handlers and `NodeState.clickable` read these, not the listener mark. `element_ids` and `element_id` read as before.
sidecar: standard-contracts bridge: the walk's listener reading now feeds the graph and clickable (2026-11-07-ui-graph)
rationale: The report's Symbols say the vdom walk now also yields per-element facts and current listeners, and that `clickable` does not read the listener mark.
@proposal n=16 detector=D-new-detail severity=warning file=standard-contracts anchor=fct-ie5oemgh fact=F8 basis_doc=doc:2026-11-07-ui-graph/report basis_fact=fct-ubggv3nh
section: ## Standard Contracts
change: - `CLICKABLE` counts per served task, in both layout modes (`stand_non_lean_tasks`): Counter, Flight Booker, Timer, CRUD and Temperature Converter 0 each, Circle Drawer 1, Cells 676. The listener mark's count stays 677.
sidecar: standard-contracts bridge: CLICKABLE counts beside the listener mark's 677 (2026-11-07-ui-graph)
rationale: This is the report's new CLICKABLE table, which has no master site yet.
@proposal n=17 detector=D-cascade severity=warning file=standard-contracts anchor=fct-h4l33tnz basis_doc=doc:2026-11-07-ui-graph/report basis_fact=fct-yr4375o3
section: ## Standard Contracts
change: «`NODE_FIELDS` keeping the thirteen keys §Driver session names» becomes «`NODE_FIELDS` keeping the fourteen keys §Driver session names».
sidecar: standard-contracts select: NODE_FIELDS is now fourteen (2026-11-07-ui-graph)
rationale: The report moves NODE_FIELDS from 13 to 14 with `clickable`. Its Counts mark every node-keys row stale.
@proposal n=18 detector=D-cascade severity=warning file=standard-contracts anchor=fct-c5e4c7ao basis_doc=doc:2026-11-07-ui-graph/report basis_fact=fct-yr4375o3
section: ## Standard Contracts
change: «the verbs are the twelve of `VERBS`» becomes «the verbs are the thirteen of `VERBS`», with `graph` listed after `lint`; the usage line gains `| graph |` after `lint`.
sidecar: standard-contracts CLIs: thirteen verbs, graph after lint (2026-11-07-ui-graph)
rationale: The report's Schema / config moves VERBS from 12 to 13 and adds `| graph |` to the usage line.
@proposal n=19 detector=D-cascade severity=warning file=body anchor=fct-ksgwc2yv basis_doc=doc:2026-11-07-ui-graph/report basis_fact=fct-yr4375o3
section: ## Standard Contracts
change: «twelve verbs» becomes «thirteen verbs».
sidecar: architecture body Driver session: thirteen verbs (2026-11-07-ui-graph)
rationale: The report moves VERBS from 12 to 13.
@proposal n=20 detector=D-cascade severity=warning file=body anchor=fct-srb7slak basis_doc=doc:2026-11-07-ui-graph/report basis_fact=fct-zklbw7ne
section: ## Standard Contracts
change: The Dioxus DOM bridge line adds that the four tiers sit beside a reserved `{owner}///{key}` form that no walk writes. Under `accessibility` it also lists the read-only UI graph (`DioxusDocument::graph`) beside the snapshot, its text form and its diff. Apps register graphed state through `use_graph_state`.
sidecar: architecture body Dioxus DOM bridge: the reserved id form, the UI graph and its registry (2026-11-07-ui-graph)
rationale: The report's Symbols and its Expected amendments name the Dioxus DOM bridge for the graph reader, the registry and the reserved form.
@remark
F9 belongs to cross-cutting-patterns and F10 to conventions, so neither is mine. The Established Decisions widening for graph's names crossing the carriers is another group's, its halt card set by inputs#I9. 28.1 is near its size cap, so splitting it may be needed.
