@proposal n=1 detector=D-cascade severity=warning file=existing-scopes anchor=fct-kaaobbkf basis_doc=doc:2026-11-07-ui-graph/report basis_fact=fct-z2vdklsi
section: ## Existing Scopes
change: In dioxus-native-dom's module list, add graph (crate-private, compiled only under `accessibility`: the read-only reader `DioxusDocument::graph` and its `UiGraph`, `GraphNode`, `GraphHandler`, `GraphWrite`, `GraphState`, `GraphWriter` and `WriteFlag` types, re-exported) and signal_registry (crate-private, under no `accessibility` gate: the `use_graph_state` hook, `DeclaredWrite` and `SignalRegistry`, re-exported). Also add that element_id holds the reserved `{owner}///{key}` form's `VIRTUAL_KEY_SEPARATOR` and `virtual_node_id` (re-exported) and the walk's crate-private `element_facts` and `element_listeners`.
sidecar: existing-scopes dioxus-native-dom: the graph and signal_registry modules and element_id's reserved-form and walk additions joined (2026-11-07-ui-graph)
rationale: The report's Crates/modules says dioxus-native-dom gains `graph` (gated `accessibility`) and `signal_registry` (not gated). Its Symbols/APIs lists the graph types, the registry exports, and element_id's `virtual_node_id` and the `ElementFacts`/`element_listeners` walk.
@proposal n=2 detector=D-cascade severity=warning file=existing-scopes anchor=fct-kaaobbkf basis_doc=doc:2026-11-07-ui-graph/report basis_fact=fct-twghj2ei
section: ## Existing Scopes
change: dioxus_document's `bridge_tests` child in `dioxus_document_tests.rs` holds eight unit tests of the bridge (seven until 2026-11-07-ui-graph, which added a dispatch recorded where a state is registered and acting as before where none is), and dioxus_document gains the `cfg(test)` helper `DioxusDocument::dispatch_to_vdom`.
sidecar: existing-scopes dioxus-native-dom: the bridge tests seven to eight, and dispatch_to_vdom (2026-11-07-ui-graph)
rationale: New text, by line, shows `dioxus_document_tests.rs` 269-322 adding the test `a_dispatch_is_recorded_where_a_state_is_registered_and_acts_as_before_where_none_is`. The Symbols/APIs bullet names the test-only `dispatch_to_vdom`.
@proposal n=3 detector=D-cascade severity=warning file=existing-scopes anchor=fct-mzrxx2a3 basis_doc=doc:2026-11-07-ui-graph/report basis_fact=fct-zklbw7ne
section: ## Existing Scopes
change: In escher-driver's text, after the lint clause, add «and, since 2026-11-07-ui-graph, the `graph` verb (schema's `GRAPH` and its two `FieldKind`s, command's `Command::Graph`, execute holding `Outcome::Graphed`, and json the graph's hand-written node and state forms)». The seventeen private modules stay as they are.
sidecar: existing-scopes escher-driver: the graph verb's homes across schema, command, execute and json (2026-11-07-ui-graph)
rationale: The report's Symbols/APIs (escher-driver) lists `schema::GRAPH`, `Command::Graph`, `Outcome::Graphed` and the JSON form written in json.rs (`graph_node`, `graph_state`, json.rs 249-308). Crates/modules says no crate or module was added.
@proposal n=4 detector=D-cascade severity=warning file=existing-scopes anchor=fct-4zmppehn basis_doc=doc:2026-11-07-ui-graph/report basis_fact=fct-zkn5zjmx
section: ## Existing Scopes
change: Add to blitz-tests' list «since 2026-11-07-ui-graph `stand_graph`, 6 tests in both layout modes». Its tests: the verb equal to `DioxusDocument::graph`; Counter's declared write at boot, confirmed after a click; every stand task's declarations agreeing with its code once its handlers have run; the served screens' handlers, owners and clicks; every graph node a snapshot node, with no served id holding `///`; a graph read changing nothing and holding no typed text; the whole Cells answer within the answer bound. It runs in `stand-repeat` by its `stand_` prefix.
sidecar: existing-scopes blitz-tests: the stand_graph target joined (2026-11-07-ui-graph)
rationale: Changes/Files lists the new `tests/blitz-tests/tests/stand_graph.rs`. Gates reads `--test stand_graph` green (6 passed). Harness/gate says it is a `stand_*` file the stand-repeat glob takes. New text, by line, lists its tests at 106-394.
@proposal n=5 detector=D-cascade severity=warning file=existing-scopes anchor=fct-4zmppehn basis_doc=doc:2026-11-07-ui-graph/report basis_fact=fct-ej52fywd dependent_of=D-cascade
section: ## Existing Scopes
change: The stand checks' shared `tests/common/mod.rs` also holds, since 2026-11-07-ui-graph, `Drive` and `GRAPH_DRIVES`, the stand's drive table that drives each conditional write once. It is read through `mod common;` by 22 stand checks, `stand_graph` the twenty-second.
sidecar: existing-scopes blitz-tests: common/mod.rs gains Drive and GRAPH_DRIVES, its readers 21 to 22 (2026-11-07-ui-graph)
rationale: New text, by line, shows `tests/blitz-tests/tests/common/mod.rs` 234-241 `pub enum Drive` and 243-323 `GRAPH_DRIVES`. Decisions says `stand_graph` is driven by it, and stand_graph.rs 77-84 has `call_of(drive: Drive)`.
@proposal n=6 detector=D-cascade severity=warning file=existing-scopes anchor=fct-4zmppehn basis_doc=doc:2026-11-07-ui-graph/report basis_fact=fct-ubggv3nh dependent_of=D-cascade
section: ## Existing Scopes
change: In `stand_non_lean_tasks`, add after the role-less count «and, since 2026-11-07-ui-graph, the per-task `CLICKABLE` count of elements whose snapshot reads `clickable` (Counter, Flight Booker, Timer, CRUD and Temperature Converter 0 each, Circle Drawer 1, Cells 676), the listener mark's count staying 677».
sidecar: existing-scopes blitz-tests: stand_non_lean_tasks gains the CLICKABLE count (2026-11-07-ui-graph)
rationale: The report's Counts/qualifiers moved gives the new `CLICKABLE` table from `stand_non_lean_tasks` in both layout modes, with the mark's count staying 677. New text, by line, shows stand_non_lean_tasks.rs 117-127 `const CLICKABLE`.
@proposal n=7 detector=D-cascade severity=warning file=existing-scopes anchor=fct-4zmppehn basis_doc=doc:2026-11-07-ui-graph/report basis_fact=fct-dxcpaddh dependent_of=D-cascade
section: ## Existing Scopes
change: In the MCP session clause of `stand_mcp_tools`, add «and, since 2026-11-07-ui-graph, a `graph` tool call answering the readers' graph and parenting one `command` span that records its verb alone». In the session_common clause, add «and since 2026-11-07-ui-graph the `graph` builder and the `graphed` reader», with `stand_graph` among its readers.
sidecar: existing-scopes blitz-tests: stand_mcp_tools' graph tool call and session_common's graph builder and reader (2026-11-07-ui-graph)
rationale: New text, by line, shows stand_mcp_tools.rs 771-843 adding the graph tool-call test, session_common/mod.rs 134-146 `graph()` and `graphed()`, and stand_graph.rs 26-28 `use session_common::{`.
@proposal n=8 detector=D-cascade severity=warning file=existing-scopes anchor=fct-qkp2micb basis_doc=doc:2026-11-07-ui-graph/report basis_fact=fct-zklbw7ne
section: ## Existing Scopes
change: seven_guis' tasks module also holds, since 2026-11-07-ui-graph, `use_graph_signal(name, writers, init) -> Signal<T>` (`use_signal` plus `use_graph_state`) and the `const fn` `writes(id, event)`. Every task's signals register through it with declarations read off the code, while `use_signal` stays the stand shell's `active` and `stand.rs`'s `linked`.
sidecar: existing-scopes seven_guis: tasks/mod.rs holds use_graph_signal and writes (2026-11-07-ui-graph)
rationale: The report's Symbols/APIs (seven_guis) puts `tasks::use_graph_signal` and the const fn `tasks::writes` in tasks/mod.rs, says every task's signals register, and names the remaining `use_signal` callers. New text, by line, shows tasks/mod.rs 4-20.
@proposal n=9 detector=D-cascade severity=warning file=existing-scopes anchor=fct-qkp2micb basis_doc=doc:2026-11-07-ui-graph/report basis_fact=fct-vzxdnwb7 dependent_of=D-cascade
section: ## Existing Scopes
change: seven_guis' shared `tests/common/mod.rs` holds `graph_needles()` beside `cells_needles(part)` among its readers (since 2026-11-07-ui-graph).
sidecar: existing-scopes seven_guis: tests/common/mod.rs gains graph_needles (2026-11-07-ui-graph)
rationale: New text, by line, shows examples/seven_guis/tests/common/mod.rs 425-452 adding `pub fn graph_needles()`. host_log and mcp_log read its needles (Coverage of new surfaces, PII).
@proposal n=10 detector=D-cascade severity=warning file=existing-scopes anchor=fct-okim3jaw basis_doc=doc:2026-11-07-ui-graph/report basis_fact=fct-ubggv3nh
section: ## Existing Scopes
change: `stand_cells_served`: Cells' screen is 53,859 bytes in 740 lines, walked in 6 parts of `SNAPSHOT_TEXT_BUDGET` through the driver (47,089 bytes in 5 parts until 2026-11-07-ui-graph added the ` clickable` token).
sidecar: existing-scopes stand_cells_served: Cells 47,089 to 53,859 bytes, 5 to 6 parts (2026-11-07-ui-graph)
rationale: The report's cells-parts pattern marks existing-scopes.md:137 stale: 53,859 B, 740 lines, parts 6 (`stand_cells_served` `PARTS` 6).
@proposal n=11 detector=D-cascade severity=warning file=existing-scopes anchor=fct-o6vmxcuj basis_doc=doc:2026-11-07-ui-graph/report basis_fact=fct-fvf7qs7b
section: ## Existing Scopes
change: blitz-tests' shared modules: `tests/common/mod.rs` is read through `mod common;` by 22 checks and `tests/session_common/mod.rs` by 33, `stand_graph` joining both at 2026-11-07-ui-graph.
sidecar: existing-scopes blitz-tests: the common and session_common reader counts 21 to 22 and 32 to 33 (2026-11-07-ui-graph)
rationale: The new stand_graph.rs imports session_common (26-28) and drives `GRAPH_DRIVES` and `Drive` from tests/common/mod.rs (77-84; Decisions, GRAPH_DRIVES).
@remark
The @task_fact records F1-F8 (standard-contracts), F9 (cross-cutting-patterns) and F10 (conventions) belong to other categories, so this group carries none. The 22 and 33 reader counts assume stand_graph declares both `mod common;` and `mod session_common;`; the report shows the session_common import, GRAPH_DRIVES and Drive use, but not the mod lines, so the orchestrator should confirm.
