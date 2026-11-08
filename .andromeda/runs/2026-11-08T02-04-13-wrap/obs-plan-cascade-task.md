@task_prose part=session
The chunk 2026-11-07-ui-graph adds a thirteenth driver verb, graph, the ninth instance verb: one read of a screen's UI graph, giving each node's id, owner, component, handlers with declared and confirmed writes, and the states it reads. It answers over the socket, the command line, MCP and kept checks.
dioxus-native-dom gains two modules: graph (the read-only DioxusDocument::graph reader, UiGraph and its types, behind accessibility) and signal_registry (use_graph_state, SignalRegistry, DeclaredWrite). The bridge's event dispatch sets the registry's dispatch only where a registry is present.
The snapshot gains NodeState.clickable, a closed bool written after focused, so NODE_FIELDS goes 13 to 14, Reading::ALL 11 to 12 and FieldKind 10 to 12. The reserved id form {owner}///{key} and virtual_node_id are public.
seven_guis tasks register their signals through use_graph_signal with authored declarations, in all seven tasks. The stand gains stand_graph, so stand files go 51 to 52 and run stand goes from 244 · 0 · 7. The two committed counter checks were re-recorded.
Measured: the Cells graph answer is 293,869 bytes after two operator narrowings. The remainder is the stated limit An unpaged graph, residual res-7cc5bi4k. host_log and mcp_log read 0 needles with one graph call each, escher-driver still holds 2 info_span! sites, and CI is green on 2ce2820b.
Retired: the capability statement's Not built line for a snapshot field saying an element reacts to a click. No dependency, manifest, blitz-dom source or escher-telemetry file changed.
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
@task_file n=22 path=packages/escher-driver/src/cli_tests.rs
@task_file n=23 path=examples/seven_guis/src/tasks/mod.rs
@task_file n=24 path=examples/seven_guis/src/tasks/cells.rs
@task_file n=25 path=examples/seven_guis/src/tasks/circle_drawer.rs
@task_file n=26 path=examples/seven_guis/tests/common/mod.rs
@task_file n=27 path=examples/seven_guis/tests/host_log.rs
@task_file n=28 path=examples/seven_guis/tests/mcp_log.rs
@task_file n=29 path=examples/seven_guis/tests/cli_commands.rs
@task_file n=30 path=examples/seven_guis/tests/mcp_server.rs
@task_file n=31 path=tests/blitz-tests/tests/common/mod.rs
@task_file n=32 path=tests/blitz-tests/tests/session_common/mod.rs
@task_file n=33 path=tests/blitz-tests/tests/stand_non_lean_tasks.rs
@task_file n=34 path=tests/blitz-tests/tests/stand_mcp_tools.rs
@task_file n=35 path=tests/blitz-tests/tests/stand_act_spans.rs
@task_file n=36 path=tests/blitz-tests/checks/counter-cli.json
@task_file n=37 path=tests/blitz-tests/checks/counter-mcp.json
@task_file n=38 path=docs/capabilities.md
@task_file n=39 path=scripts/cold-agent.sh
@task_file n=40 path=.github/scripts/test_cold_agent.py
@task_item n=1 part=touches
The command span: graph is a new word of its verb field. A graph call through Session::run opens the one command span and records verb alone: it runs no step, so no settled, passes or diff counts. The span's eight fields and the 2 info_span! sites are unchanged.
@task_item n=2 part=touches
The tool_call span: its tool field gains graph, and the verb table is now thirteen words. graph is an instance verb, so its tool_call parents one command child. Rows naming twelve tools or eight instance verbs are stale.
@task_item n=3 part=touches
The session library's verb list in the logging-stack and log-coverage prose names scroll sixth, screenshot seventh and lint eighth. graph is now the ninth instance verb, and rows counting eight instance verbs are stale. Rows naming lint the eighth still hold.
@task_item n=4 part=touches
Outcome classes written by to_json alone and never logged or fielded: Outcome::Graphed joins Shot, Linted and Part. Its answer is the graph's JSON line on the command line and over MCP, and is kept whole in a recorded check. Outcome's Debug now covers the graph too.
@task_item n=5 part=touches
The snapshot's state readings, which log-coverage and pii text describe as three closed words and booleans per node: NodeState.clickable is a further closed bool. It is written after focused, carried in the diff, and adds the text token clickable. Any node-key count of thirteen is now fourteen.
@task_item n=6 part=touches
host_log and mcp_log: each now drives one graph call at trace under both builds and reads 0 id, name and graph needles. The seven_guis shared test module gains graph_needles, built from the stand's component and state names. Rows listing what the two log checks read and the shared module's readers move.
@task_item n=7 part=touches
The stand checks' shared modules: tests/blitz-tests/tests/common/mod.rs gains Drive and GRAPH_DRIVES, and session_common gains graph and graphed. stand_graph is a new stand_* target. Counts of each shared module's readers, and the silence census over the stand checks, may move.
@task_item n=8 part=touches
The stand file count goes 51 to 52, and the run stand tally from 244 · 0 · 7. Rows stating the current count are stale. Dated earlier readings stay historical.
@task_item n=9 part=touches
The cold-agent pipe: its VERBS gain graph (thirteen), and its fake server answers graph with an empty nodes and states object. The capability-unused set and the counted verbs (lint, screenshot, advance) are unchanged, so server.log counting and the verdict fields do not move.
@task_item n=10 part=touches
The two committed kept checks, counter-cli and counter-mcp, were re-recorded because each answer's node now carries clickable. They still hold no typed text, host path or secret, and stand_replay replays them green. The CI artifact text naming two committed checks still holds.
@task_item n=11 part=touches
The ratified answer crossing (ids, accessible names, control values, closed state words): the graph widens it with component names, registered state names, handler event names, the closed flag words undeclared and unconfirmed, and booleans. It carries no value. Sites located by the lint-crossing, obs-verb and cli-usage patterns in obs-plan carry it.
@task_item n=12 part=touches
The escher-session prose covering the usage line, the MCP tools listing and help: the usage line gains graph after lint, the MCP listing goes to 13 tools with graph after lint, and describe's count row moves to 13 verbs and 14 node keys.
@task_item n=13 part=touches
dioxus-native-dom as an emitter: its two new modules hold no log site, print, env read or subscriber. The silence census over them reads 0. The bridge dispatch change on every Dioxus document's event path adds no log site and is inert without a registry.
@task_item n=14 part=touches
The cited line ranges of escher-driver json.rs (the Shot and Linted answer citations), host_log.rs, mcp_log.rs and the seven_guis tests common module can be off now that lines were added to each. Re-check every line citation into them.
@task_fact n=1 slug=log-coverage
fact: The UI graph is silent: dioxus-native-dom's graph and signal_registry modules and the stand_graph check hold no print, log site, subscriber or env read, the silence census reads 0 over them, and Outcome::Graphed is written by to_json alone as {"nodes":[…],"states":[…]}, never logged or fielded, with escher-driver keeping its 2 info_span! sites.
@task_fact n=2 slug=span-trace-coverage
fact: A graph call's command span records verb alone, and a graph tools/call opens a tool_call with tool=graph that parents exactly one command child recording verb alone (stand_mcp_tools; stand_act_spans one graph call per layout mode).
@task_fact n=3 slug=pii-scrubbing-compliance
fact: The graph answer carries stable ids, component names, registered state names, handler event names, the closed flag words undeclared and unconfirmed, and booleans only, never a state's value: a sentinel typed into crud-name occurs 0 times in it, and host_log and mcp_log read 0 id, name and graph needles at trace under both builds.
@task_fact n=4 slug=pii-scrubbing-compliance
fact: SignalRegistry's Debug prints counts alone; the registry holds no value of a state, no NodeId beyond the standing dispatch chain, and no log site, print, env read or unwrap on app input.
@task_fact n=5 slug=log-coverage
fact: The bridge dispatch (DioxusEventHandler::handle_event) sets the registry's dispatch around runtime.handle_event only when a SignalRegistry context is present at the root scope, and adds no log site; a page with no registered state reaches no registry.
@task_fact n=6 slug=pii-scrubbing-compliance
fact: The snapshot's clickable reading is a closed bool (an element's own click listener, no disabled form control), carrying no page text.
@task_fact n=7 slug=log-coverage
fact: The cold-agent pipe's fake server answers graph with {"nodes":[],"states":[]}, and graph is not required of a cold agent: the capability-unused set and the counted verbs are unchanged.
@task_item n=1 part=untouched
escher-telemetry's sink, its allowlist formatter, scrub sets, service identity, line format and chained panic hook: no file of that crate changed.
@task_item n=2 part=untouched
Engine telemetry and feature wiring (the tracing and log-times features across blitz-dom, blitz-net, blitz-html, blitz-shell, blitz-paint, blitz, blitz-vibey-script and dioxus-native): no manifest, Cargo.lock or blitz-dom source moved.
@task_item n=3 part=untouched
dioxus-native-dom's mutation writer log sites, cited among the values logged as-is: the writer is byte-identical to upstream 434e0661.
@task_item n=4 part=untouched
Metric coverage: no metric, counter or timing was added. The in-process timings and the absence of metrics exporters stand.
@task_item n=5 part=untouched
Error capture and reporting (panic hooks, the error classes captured, the script error surface): nothing in the session reaches them.
@task_item n=6 part=untouched
The agent-run harness log format and its state area: only a run's tally moved, not the events or their fields.
@task_item n=7 part=untouched
The escher-driver mcp listing and result modules and the recording module: unedited, since each routes every verb through VERBS and Outcome::to_json. Their stated silence stands.
@task_item n=8 part=untouched
CI artifact handling, the publish builds' CARGO_LOG, the apt-install diagnostic and the WPT publishing: no workflow or CI script outside the cold-agent test changed.
@task_item n=9 part=untouched
The upstream apps' stdout subscribers, wasm_hello's tracing_wasm, the wpt runner's env_logger and blitz-vibey-script's script diagnostics drain: untouched.
@task_item n=10 part=untouched
SLO invariants, obs anti-patterns and the obs decisions log: no recorded intent, and the session adds none.
