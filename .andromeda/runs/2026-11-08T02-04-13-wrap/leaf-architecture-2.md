@leaf_group group=architecture-2
@leaf_read leaf=.claude/docs/services/dioxus-native-dom.md lines=44
last: - `.andromeda/architecture.md` · `.claude/docs/services/blit
@leaf_read leaf=.claude/docs/stack.md lines=59
last: 4. Commit as `chore: bump {crate} to {version}`.
@leaf_read leaf=.claude/docs/workflow.md lines=47
last: A reference-based CLAUDE.md: a thin index, with depth in `.c
@leaf_edit n=1 leaf=.claude/docs/services/dioxus-native-dom.md line=14 class=omission
leaf-text: `NodeHandle` backing `MountedData`.
source: fct-srb7slak «the UI graph (`DioxusDocument::graph`, a `UiGraph` of nodes and registered states) read the screen read-only»
replace-with: `NodeHandle` backing `MountedData`; under `accessibility`, the read-only UI graph `DioxusDocument::graph() -> UiGraph` (`UiGraph { nodes, states }`: one `GraphNode { id, owner, component, handlers, reads }` per snapshot node in pre-order, `GraphState { name, component, declared, writers, readers }`, `WriteFlag::{Undeclared, Unconfirmed}`), holding no `NodeId`; ungated, the graph registry `use_graph_state(name, writers, subscribers)` (`DeclaredWrite`, `SignalRegistry`), whose first hook provides a `SignalRegistry` context at `ScopeId::ROOT`.
@leaf_edit n=2 leaf=.claude/docs/services/dioxus-native-dom.md line=15 class=omission
leaf-text: an anchored path is the only id holding `//`.
source: fct-kllkxz3d «beside the four tiers the grammar states a fifth, reserved form, `{owner}///{key}`, which no tier can hold and no caller produces, the walk not calling it»
replace-with: an anchored path is the only id holding `//`; beside the four tiers a fifth, reserved form `{owner}///{key}`, which no tier holds and no walk writes, spelled by the public, re-exported `VIRTUAL_KEY_SEPARATOR` and `virtual_node_id(owner, key) -> Option<String>` (`None` for an empty or `/`-bearing key).
@leaf_edit n=3 leaf=.claude/docs/services/dioxus-native-dom.md line=17 class=stale
leaf-text: invalid, focused }` — seven fields
source: fct-kllkxz3d «`invalid: bool` · `focused: bool` · `clickable: bool`»
replace-with: invalid, focused, clickable }` — eight fields
@leaf_edit n=4 leaf=.claude/docs/services/dioxus-native-dom.md line=17 class=stale
leaf-text: keeping its thirteen keys
source: fct-h4l33tnz «`NODE_FIELDS` keeping the fourteen keys §Driver session names»
replace-with: keeping its fourteen keys
@leaf_edit n=5 leaf=.claude/docs/services/dioxus-native-dom.md line=17 class=stale
leaf-text: names all seven)
source: fct-kllkxz3d «so a struct literal without a spread names all eight»
replace-with: names all eight)
@leaf_edit n=6 leaf=.claude/docs/services/dioxus-native-dom.md line=17 class=omission
leaf-text: `focused` is the tree's focus.
source: fct-kllkxz3d «`clickable` true where the vdom walk's current listeners hold `click` and the element is no `ElementData::is_disabled` form control»
replace-with: `focused` is the tree's focus; `clickable` is true where the vdom walk's current listeners (the crate-private `element_listeners`, never the listener mark) hold `click` and the element is no `ElementData::is_disabled` form control, so a disabled `button` with a `click` listener reads not clickable and a `<div disabled>` with one reads clickable.
@leaf_edit n=7 leaf=.claude/docs/services/dioxus-native-dom.md line=18 class=stale
leaf-text: `invalid` (only when true) · `focused`, then
source: fct-kllkxz3d «`invalid` · `focused` · `clickable` (a token absent where the model holds nothing for it; `invalid` and `clickable` written only when true»
replace-with: `invalid` · `focused` · `clickable` (`invalid` and `clickable` only when true), then
@leaf_edit n=8 leaf=.claude/docs/services/dioxus-native-dom.md line=19 class=stale
leaf-text: (eleven variants, `Parent` · `Role` · `Name` · `Enabled` · `Checked` · `Value` · `Selected` · `Pressed` · `Invalid` · `Focused` · `Bounds`, with `Reading::ALL`)
source: fct-kllkxz3d «twelve variants, `Parent` · `Role` · `Name` · `Enabled` · `Checked` · `Value` · `Selected` · `Pressed` · `Invalid` · `Focused` · `Clickable` · `Bounds`, with `Reading::ALL`»
replace-with: (twelve variants, `Parent` · `Role` · `Name` · `Enabled` · `Checked` · `Value` · `Selected` · `Pressed` · `Invalid` · `Focused` · `Clickable` · `Bounds`, with `Reading::ALL`)
@leaf_edit n=9 leaf=.claude/docs/services/dioxus-native-dom.md line=22 class=omission
leaf-text: DOM events route to the vdom via the nearest `data-dioxus-id` attribute (parsed as `usize`);
source: fct-56u6r4qr «`DioxusEventHandler::handle_event` sets the registry's dispatch — the chain head first, and its event name — around `runtime.handle_event` and clears it after»
replace-with: DOM events route to the vdom via the nearest `data-dioxus-id` attribute (parsed as `usize`); where a `SignalRegistry` stands at `ScopeId::ROOT` (`runtime.has_context`), `DioxusEventHandler::handle_event` sets its dispatch — the chain head first, and its event name — around `runtime.handle_event` and clears it after, so a signal write is recorded against the dispatch that caused it, while a page with no registered state dispatches as before;
@leaf_edit n=10 leaf=.claude/docs/services/dioxus-native-dom.md line=38 class=omission
leaf-text: `src/{actionable,dioxus_document,element_id,mutation_writer,mutation_buffer,events,snapshot,snapshot_diff,snapshot_text,write_once_attr,keyboard_compat}.rs`
source: fct-kaaobbkf «signal_registry (crate-private, under no `accessibility` gate: the `use_graph_state` hook, `DeclaredWrite` and `SignalRegistry`, re-exported)»
replace-with: `src/{actionable,dioxus_document,element_id,graph,mutation_writer,mutation_buffer,events,signal_registry,snapshot,snapshot_diff,snapshot_text,write_once_attr,keyboard_compat}.rs` (`graph`, crate-private and compiled only under `accessibility`, the reader behind `DioxusDocument::graph`, its types re-exported; `signal_registry`, crate-private under no `accessibility` gate, `use_graph_state`, `DeclaredWrite` and `SignalRegistry` re-exported)
@leaf_edit n=11 leaf=.claude/docs/services/dioxus-native-dom.md line=41 class=stale
leaf-text: seven `bridge_tests`
source: fct-kaaobbkf «eight unit tests of the bridge, beside the `cfg(test)` helper `DioxusDocument::dispatch_to_vdom`»
replace-with: eight `bridge_tests` (beside the `cfg(test)` helper `DioxusDocument::dispatch_to_vdom`)
