@task_prose part=session
Chunk 2026-11-07-ui-graph adds one read of the screen's UI graph: a new driver verb graph that answers each snapshot node's id, owner, component, handlers, declared and confirmed writes and the states it reads, plus every registered state with its declared writers, confirmed or flagged writers and its reading components.
dioxus-native-dom gains two modules: graph (gated accessibility; DioxusDocument::graph returning UiGraph) and signal_registry (not gated; use_graph_state, SignalRegistry, DeclaredWrite). The bridge's event dispatch now sets the registry's dispatch around runtime.handle_event wherever a registry exists, and changes nothing where none does.
The snapshot gains NodeState.clickable, true where an element's current listeners hold click and it is no disabled form control. The text token clickable follows focused, and the reading means the click reading is now built.
The stable-id grammar states a fifth, reserved form {owner}///{key} through public VIRTUAL_KEY_SEPARATOR and virtual_node_id, which no caller produces yet.
Counts moved: VERBS 12 to 13, instance verbs 8 to 9, NODE_FIELDS 13 to 14, FieldKind 10 to 12, Reading::ALL 11 to 12, describe row [13,14,12,3,8,4,15], stand files 51 to 52, run stand 244/0/7, dioxus-native-dom units 83 to 100, Cells screen 53,859 B in 6 parts. Measured: the Cells graph answer is 293,869 B, a stated limit (An unpaged graph) with residual res-7cc5bi4k, target next.
No dependency, manifest, blitz-dom or dioxus-native change. CI green on 2ce2820b.
@task_file n=1 path=packages/dioxus-native-dom/src/graph.rs
@task_file n=2 path=packages/dioxus-native-dom/src/signal_registry.rs
@task_file n=3 path=tests/blitz-tests/tests/stand_graph.rs
@task_file n=4 path=packages/dioxus-native-dom/src/lib.rs
@task_file n=5 path=packages/dioxus-native-dom/src/element_id.rs
@task_file n=6 path=packages/dioxus-native-dom/src/snapshot.rs
@task_file n=7 path=packages/dioxus-native-dom/src/snapshot_text.rs
@task_file n=8 path=packages/dioxus-native-dom/src/snapshot_diff.rs
@task_file n=9 path=packages/dioxus-native-dom/src/dioxus_document.rs
@task_file n=10 path=packages/dioxus-native-dom/src/dioxus_document_tests.rs
@task_file n=11 path=packages/escher-driver/src/schema.rs
@task_file n=12 path=packages/escher-driver/src/schema/spec.rs
@task_file n=13 path=packages/escher-driver/src/schema_tests.rs
@task_file n=14 path=packages/escher-driver/src/command.rs
@task_file n=15 path=packages/escher-driver/src/execute.rs
@task_file n=16 path=packages/escher-driver/src/json.rs
@task_file n=17 path=packages/escher-driver/src/describe.rs
@task_file n=18 path=packages/escher-driver/src/lib.rs
@task_file n=19 path=packages/escher-driver/src/mcp.rs
@task_file n=20 path=packages/escher-driver/src/mcp_tests.rs
@task_file n=21 path=packages/escher-driver/src/recording_tests.rs
@task_file n=22 path=examples/seven_guis/src/tasks/mod.rs
@task_file n=23 path=examples/seven_guis/src/tasks/counter.rs
@task_file n=24 path=examples/seven_guis/src/tasks/temp_converter.rs
@task_file n=25 path=examples/seven_guis/src/tasks/flight_booker.rs
@task_file n=26 path=examples/seven_guis/src/tasks/timer.rs
@task_file n=27 path=examples/seven_guis/src/tasks/crud.rs
@task_file n=28 path=examples/seven_guis/src/tasks/circle_drawer.rs
@task_file n=29 path=examples/seven_guis/src/tasks/cells.rs
@task_file n=30 path=examples/seven_guis/tests/cli_commands.rs
@task_file n=31 path=examples/seven_guis/tests/mcp_server.rs
@task_file n=32 path=tests/blitz-tests/tests/common/mod.rs
@task_file n=33 path=tests/blitz-tests/tests/session_common/mod.rs
@task_file n=34 path=tests/blitz-tests/tests/stand_non_lean_tasks.rs
@task_file n=35 path=tests/blitz-tests/tests/stand_mcp_tools.rs
@task_file n=36 path=tests/blitz-tests/checks/counter-cli.json
@task_file n=37 path=tests/blitz-tests/checks/counter-mcp.json
@task_file n=38 path=docs/capabilities.md
@task_file n=39 path=README.md
@task_file n=40 path=scripts/cold-agent.sh
@task_item n=1 part=touches
Driver session contract (escher-driver): the schema's twelve verbs are now thirteen. graph is the ninth instance verb, after lint, with no argument and fields nodes and states. Command::Graph and Outcome::Graphed are added, and Graphed runs no step and records verb alone on its command span. Every row giving the current verb count is stale, including standard-contracts.md:199, which reads 12 verbs, 13 node keys.
@task_item n=2 part=touches
Instance-verb count 8 to 9: the rows reading the eight instance verbs (standard-contracts.md:234 and :235) are stale. The row calling lint the eighth instance verb (standard-contracts.md:237) was re-read and stays true, because graph is the ninth.
@task_item n=3 part=touches
Node-key count 13 to 14: clickable joins after focused in NODE_FIELDS, the snapshot's text form and the written node JSON. Every architecture row giving thirteen node keys is stale.
@task_item n=4 part=touches
FieldKind 10 to 12 (graph-nodes, graph-states) and Reading::ALL 11 to 12 (Reading::Clickable): any architecture row counting field kinds or readings moves. No master site was found for either.
@task_item n=5 part=touches
Historical verb counts were re-read and are unmoved: project-intent.md:47 (the twelve MCP tools of the 2026-10-25 live run) and standard-contracts.md:354 (the escher server's twelve tools of that run) stay as dated readings. project-intent.md:29, which states the current verb count, is stale.
@task_item n=6 part=touches
Dioxus DOM bridge contract: the stable id's four tiers now come with a fifth, reserved form {owner}///{key}, through the public VIRTUAL_KEY_SEPARATOR and virtual_node_id, re-exported. The walk never produces it, and no served id holds ///. element_ids and element_id read as before.
@task_item n=7 part=touches
Dioxus DOM bridge contract: under accessibility, a read-only graph reader DioxusDocument::graph now sits beside the snapshot, its text form and its diff, and holds no NodeId. The snapshot gains NodeState.clickable, read from the vdom walk's current listeners together with ElementData::is_disabled.
@task_item n=8 part=touches
Bridge event path, every Dioxus document: DioxusEventHandler::handle_event now sets and clears the SignalRegistry dispatch around runtime.handle_event when a registry exists at ScopeId::ROOT. Without one it is inert. The buffered render, initial_build, poll and flush_queued_mounted_events are unchanged. This touches the Dioxus integration decision and the cross-cutting event-processing pattern.
@task_item n=9 part=touches
Driver session decision (what crosses): the socket, the command line, MCP and kept checks now also carry the UI graph. That is ids, component names, registered state names, handler event names, closed flag words and booleans, never a value and never a NodeId. This is a boundary widening, ratified by the founder's word (inputs#I4, I9).
@task_item n=10 part=touches
Logging rule (no log or event carries the snapshot, text or diff): the graph joins this set. host_log and mcp_log read 0 id, name and graph needles, the new modules hold no log site, print or env read, and escher-driver still has 2 info_span! sites.
@task_item n=11 part=touches
CLIs contract: the escher-session usage line gains graph after lint. The MCP tools/list lists 13 tools, graph after lint, because VERBS orders it.
@task_item n=12 part=touches
CI contracts and the cold-agent pipe: the cold-agent VERBS set gains graph (thirteen), and the fake server answers graph with an empty graph. The capability-unused verdict set is unchanged.
@task_item n=13 part=touches
Front page and capability statement: the README count of driver commands an MCP client gets as tools is now thirteen. The capability statement reads thirteen verbs and fourteen node keys, and the capability-statement pin reads them green. The statement's Not built line for a click snapshot field was retired.
@task_item n=14 part=touches
Existing scopes: the snapshot model's scope (existing-scopes.md:76) gains the graph and the signal registry. The Cells screen row (existing-scopes.md:137) moves from 47,089 to 53,859 bytes and from 5 to 6 parts, still 740 lines.
@task_item n=15 part=touches
Config management (accessibility gating the dioxus-native-dom snapshot modules): graph is gated accessibility, but signal_registry is not gated.
@task_item n=16 part=touches
Headless stand counts: stand_* files 51 to 52 (stand_graph joins, and the stand-repeat glob takes it), the run stand tally 237/0/7 to 244/0/7, and dioxus-native-dom unit tests 83 to 100. The 83 percent coverage floor rows share the token but are unmoved.
@task_item n=17 part=touches
Apps and examples convention (reactive state uses signal and store hooks): every seven_guis task's signals now register through tasks::use_graph_signal with authored write declarations. Plain use_signal remains only in the stand shell's active and in stand.rs linked.
@task_item n=18 part=touches
Residuals: res-gvelrxja (the click reading owed) has its disposition met. res-7cc5bi4k is newly minted for the unpaged Cells graph size, target next.
@task_item n=19 part=touches
Committed kept checks: counter-cli.json and counter-mcp.json were re-recorded by record_checks.sh, and stand_replay replays them green. The two-committed-checks exception is unchanged.
@task_item n=20 part=touches
dioxus-native re-exports: DeclaredWrite, SignalRegistry, use_graph_state and the graph types now reach apps through dioxus-native's pub use dioxus_native_dom::*. The dioxus-native crate itself is unedited.
@task_fact n=1 slug=standard-contracts
fact: DioxusDocument::graph(&self) -> UiGraph (behind accessibility) is read-only. UiGraph {nodes, states} has one GraphNode {id, owner, component, handlers, reads} per snapshot node in pre-order, GraphHandler {event, writes}, GraphWrite {state, declared, confirmed, flag}, GraphState {name, component, declared, writers, readers}, GraphWriter {id, event, declared, confirmed, flag}, and WriteFlag::{Undeclared, Unconfirmed} with word().
@task_fact n=2 slug=standard-contracts
fact: Graph semantics: owner is the stable id of BaseDocument::select_of for an option or optgroup. A repeated (name, component) registration reads {name}:{k}. readers names each reading component once, in the pre-order of its first node, and a state only the app root reads appears in its nodes' reads alone. writers lists only writes a run confirmed or flagged, while declared keeps the authored declaration. A write resolves to the first chain element carrying a listener of its event, or to nothing when an earlier chain node is gone. unconfirmed fires only after a dispatch of the handler's event passed through its element and no run wrote the state.
@task_fact n=3 slug=standard-contracts
fact: use_graph_state(name: &'static str, writers: &'static [DeclaredWrite], subscribers: Subscribers) takes dioxus-core types alone. The first registering hook provides a SignalRegistry context at ScopeId::ROOT. Each entry holds a ReactiveContext::new_with_callback that records one write against the standing dispatch (chain head first, its event name) and the writing scope, and leaves with its scope via use_drop. DeclaredWrite {id, event} names a stable id or a prefix ending in *, and an event without on. The registry's Debug prints counts alone.
@task_fact n=4 slug=standard-contracts
fact: The Signal-returning use_graph_signal(name, writers, init) lives in seven_guis tasks/mod.rs, not in dioxus-native-dom, because dioxus-native-dom names no crate defining Signal and the manifest freeze forbids adding one. tasks::writes(id, event) is a const fn.
@task_fact n=5 slug=standard-contracts
fact: The graph verb's JSON answer is written by hand in the json module as {"nodes":[{id,owner?,component?,handlers:[{event,writes:[{state,declared,confirmed,flag?}]}],reads}],"states":[{name,component?,declared:[{id,event}],writers:[{id,event,declared,confirmed,flag?}],readers}]}. An argument is refused malformed with nothing run, and the answer feeds the record of ids from the graph's node ids.
@task_fact n=6 slug=standard-contracts
fact: The graph is unpaged: graph takes no argument and answers the whole screen at once. Cells reads 293,869 bytes (547,685 at first build and 522,692 after the readers narrowing), under the answer bound held by stand_graph. The remainder, each cell's declared click writes, is the stated limit An unpaged graph, owned by residual res-7cc5bi4k.
@task_fact n=7 slug=standard-contracts
fact: stand_graph runs in both layout modes. It holds the verb equal to DioxusDocument::graph, every graph node to a snapshot node, no served id to the /// form, a graph read to change nothing and hold no typed text, and the Cells answer to the bound. GRAPH_DRIVES in tests/blitz-tests/tests/common/mod.rs drives each conditional write once, so no stand write reads a flag.
@task_fact n=8 slug=standard-contracts
fact: CLICKABLE counts per served task, in both layout modes (stand_non_lean_tasks): Counter, Flight Booker, Timer, CRUD and Temperature Converter 0 each, Circle Drawer 1, Cells 676. The listener mark's count stays 677.
@task_fact n=9 slug=cross-cutting-patterns
fact: In Dioxus 0.7.10 an async event handler's spawned task runs inside the dispatch that started it, so its signal writes are recorded against that event. dioxus-signals keeps a reactive context subscribed while mark_dirty returns true, so the registry's watcher needs no re-subscribe.
@task_fact n=10 slug=conventions
fact: A &'static [DeclaredWrite] slice built from const fn calls is written const { &[...] }. A bare &[writes(..)] is a temporary that is not promoted to 'static and errors E0716.
@task_item n=1 part=untouched
blitz-dom engine behaviour, the select model, focus, inert, scrolling, hit testing, layout, painting and the accessibility tree builder: no engine file moved, and the git diff gate over blitz-dom is green.
@task_item n=2 part=untouched
Dependency pinning, inherited defaults and stack versions (Stylo, Taffy, Dioxus 0.7.3, winit), every manifest, Cargo.lock and deny.toml: unchanged. No crate was added or removed.
@task_item n=3 part=untouched
The buffered render and the vendored mutation writer: byte-identical to upstream 434e0661, and the runtime probe's 10 tests are green.
@task_item n=4 part=untouched
Occupied resources: no new listener, port, outbound host, filesystem area, env variable, thread or process-wide state. The session socket and the MCP stdio carry the new verb over the existing carriers.
@task_item n=5 part=untouched
Telemetry bootstrap, SAFE_FIELDS and CONTENT_FIELDS, and the logging and timing pattern's sink rules: unchanged. The graph adds no field to any span beyond verb.
@task_item n=6 part=untouched
The MCP listing and result modules and the recording module: unedited, because they route every verb through VERBS and Outcome::to_json.
@task_item n=7 part=untouched
JavaScript runtime, the WPT runner, the browser app, rdme, the shell loop and launch paths: untouched by the chunk.
@task_item n=8 part=untouched
CI workflow jobs, the leg runner, coverage floors (the 83 percent floor rows included), the cache and permissions: no job or leg added. The stand-repeat glob picks up the new stand file unchanged.
@task_item n=9 part=untouched
Upstream sync merge base, the take of e42ee1f25 and the direct-engine-change merge surfaces: no upstream take or merge in this chunk.
@task_item n=10 part=untouched
The unkeyed_actionable rule, roles, names and Tab orders of the served stand tasks, and the served screens' markup: unchanged, and stand_lint at boot reads as before.
