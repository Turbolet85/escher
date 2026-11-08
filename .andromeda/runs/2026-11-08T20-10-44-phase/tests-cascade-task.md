@task_prose part=chunk
Builds one handler-coverage command (v030-07): after a set of recorded checks (kept checks read through Check::from_bytes and run through Check::replay) has run on the seven_guis stand, it lists exactly the UI graph's handlers (a graph node id and a handler event name, as DioxusDocument::graph reads them) that none of those checks exercised.
States in the acceptance that handlers a canvas dispatches internally, inside one listener, are not counted; 0.4.0 extends the count to virtual nodes, so the command stays keyed on the graph node id with its optional owner, never on a NodeId.
Adds a record of which (element, event) handlers a run reached that holds on every app, registry or not, read with the signal registry, the bridge's dispatch record and the graph reader in dioxus-native-dom.
Writes the owed replay test (CARRY cry-sxrivdk3): a recorded session holding an actions step, kept as a check, read back and replayed on a fresh session of the same app, the replay holding, and a differing actions answer naming the step and the differing key; v030-06 reads verified only once it passes.
The coverage read is read-only over the screen: no step of its own, no control value, typed text or NodeId, and the UI graph, snapshot, stable-id grammar and actions verb keep their shapes.
@task_file n=1 path=packages/dioxus-native-dom/src/dioxus_document.rs
@task_file n=2 path=packages/dioxus-native-dom/src/signal_registry.rs
@task_file n=3 path=packages/dioxus-native-dom/src/graph.rs
@task_item n=1 part=needs
The UI graph reader DioxusDocument::graph, its GraphNode.handlers and GraphHandler.event, the graph verb and the stand_graph check that reads it on every served screen in both layout modes, with the GRAPH_DRIVES table and Drive enum in tests/blitz-tests/tests/common/mod.rs.
@task_item n=2 part=needs
The bridge's dispatch record, SignalRegistry::dispatches() and the graph's confirmed flag, with the dioxus-native-dom unit tests over the bridge, the signal registry and the graph reader, the bridge_tests child module and its cfg(test) shim dispatch_to_vdom; the new exercised record must hold on an app that registers no state.
@task_item n=3 part=needs
Recorded-check replay: kept checks committed under tests/blitz-tests/checks, run as cargo test targets through Check::replay, red at the first step whose acceptance or kept answer differs, and kept fresh byte-for-byte against what escher-session records; the stand_replay check and seven_guis recording target that embed them.
@task_item n=4 part=needs
The owed actions-step replay test: recording a session with an actions step (stand_session_recording, session_common's kept and shot_prefix readers), reading it back through Check::from_bytes, replaying on a fresh session, and asserting a differing actions answer names the step and the key.
@task_item n=5 part=needs
The actions verb and its stand_actions check (each served screen's actionable nodes, every listed cause what the call returns, both layout modes, changing nothing), whose answer the replay test keeps and compares.
@task_item n=6 part=needs
Session to held instance integration: an escher_driver Session booted through seven_guis::stand, calls run through Session::run returning after settle, both layout modes, and the session_common builders (hold, act, graphed, refused) a coverage check would use.
@task_item n=7 part=needs
The read-only boundary of the coverage read: it changes nothing on screen and carries no control value, typed text, page text beyond ids, component and event names, or NodeId, checked as the graph and actions verbs are (snapshot unchanged before and after).
@task_item n=8 part=needs
The canvas limit and the reserved virtual form of the stable element id: the count is DOM listeners on agent-visible nodes, one per (node, event); canvas-internal handlers are not counted and no virtual node is produced.
@task_item n=9 part=needs
If the command is a new driver verb: VERBS fourteen to fifteen and every pin of that count — test_capability_statement.py over docs/capabilities.md and README's tool count, schema_tests.rs, cli_tests.rs, stand_mcp_tools — plus the verb's word in the closed tool_call and command span sets; the cold-agent pipe's verb list and isolated tool set are named to «Cold-agent bar», not counted here.
@task_item n=10 part=needs
The driver's log discipline: one escher_driver command line per call with fixed words and counts only, and host_log and mcp_log reading 0 id, name and graph needles at RUST_LOG=trace under both builds for any new verb.
@task_item n=11 part=needs
The escher-driver unit tests of the verb schema, command validation, JSON forms, command line, MCP, recording and replay, which a new verb, argument kind or result field reaches.
@task_item n=12 part=needs
Per-crate coverage floors in the coverage leg: packages/escher-driver 97 % and packages/dioxus-native-dom 83 %, which the new code and its tests must keep.
@task_item n=13 part=needs
The stand-repeat leg's zero-failure flakiness budget (stand_*.rs files, the seventeen named targets, seven_guis process checks, three rounds, no retry), which the new checks join.
@task_item n=14 part=needs
The agent-run contract's run stand count (newest 244 · 0 · 7 over 52 files): a new stand_ file is picked up with no script change and the recount is stated in the report.
@task_item n=15 part=needs
v030-07's acceptance by integration method (after a set of recorded checks has run on the stand, one command lists exactly the unexercised handlers), and v030-06 flipping from implemented to verified on the owed test's pass.
@task_standing n=1
convention: a new integration check is one test-target file per behavior under tests/blitz-tests/tests, a stand check carrying the stand_ prefix so agent-run's run stand selects it, with no new [[test]] table in the manifest
home: §2 Test Strategy (Test directory + naming conventions) and §4 Unit Test Strategy (blitz-tests targets)
@task_standing n=2
convention: a check that holds a session declares mod session_common; and reads the shared builders, and a page more than one check reads is stated once in tests/fixtures/mod.rs; tables are stated expectation, never a second mapping
home: §3 Test Harness Contract (Crate-local test helpers)
@task_standing n=3
convention: a session check runs in both layout modes, incremental and not
home: §5 Integration Test Strategy (Session ↔ held instance)
@task_standing n=4
convention: no sleep and no wall-clock read in a session check, and no pass in a Timer flow but the driver's
home: §5 Integration Test Strategy (Wall time in the session checks)
@task_standing n=5
convention: unit tests sit in #[cfg(test)] mod tests in the source file, or in a #[path] file declared at its parent with test paths unchanged
home: §4 Unit Test Strategy (Conventions)
@task_standing n=6
convention: a check writes and logs nothing of the screen — no stable id, accessible name or typed text on stdout, stderr or a state directory; assertion messages carry a row index only
home: §5 Integration Test Strategy (Session host ↔ what it writes) and §3 Stand log format
@task_standing n=7
convention: a check that needs a session process bounds every wait, holds the guard that kills and reaps the host, removes its state directory and is cfg(unix)-gated
home: §2 Test Strategy (Process-lifecycle checks)
@task_standing n=8
convention: a recorded check committed as data is kept fresh by a byte-for-byte comparison with what the binary records and holds no typed text, host path, secret or screenshot step
home: §7 Test Data & Fixtures (tests/blitz-tests/checks) and §2 Test Strategy (Recorded-check replay)
@task_standing n=9
convention: a chunk adding stand checks restates agent-run's run stand run.end count, as measured in its report
home: §3 Test Harness Contract (Agent-run contract, Proof)
@task_standing n=10
convention: a check reading a sink's capture counts lines by target, never in total, since the workspace and package-alone builds differ in engine features
home: §9 CI Integration (Engine features by runner)
@task_item n=1 part=not_needed
WPT conformance: the runner, its scoring, ports, reftests and the upstream-only wpt workflows; the chunk adds nothing rendered against WPT.
@task_item n=2 part=not_needed
The apps/browser tests (about pages, history, favicon, URL suggestions, persistence, tab).
@task_item n=3 part=not_needed
blitz-vibey-script's script DOM, Selection, URLSearchParams and Preact TodoMVC tests.
@task_item n=4 part=not_needed
Pixel and paint tests: blitz-paint's units, blend and inline-box gaps, password mask painting, screenshot and text_contrast readers.
@task_item n=5 part=not_needed
blitz-dom's layout, style bridge, node, select model, clock, mutator and net unit tests, and the differential oracle, randomized stress and pure-restyle checks.
@task_item n=6 part=not_needed
The select popup and select list targets (select_interaction, select_popup, select_popup_painted, stand_act_popup) and the font-embolden witness leg.
@task_item n=7 part=not_needed
Example fixtures under examples/assets and examples/preact, the iframe click-through pages and the manual paint_bench and screenshot tools.
@task_item n=8 part=not_needed
The paint_tree_bench ignored benchmarks and the dev-host mutation tier (cargo-mutants under nextest).
@task_item n=9 part=not_needed
CI package-install bounding (apt-install.sh), rust-cache keys and the WPT diff and area-change Python scripts.
@task_item n=10 part=not_needed
The accessibility states as handed to an assistive technology (residual res-llkgqygv) and upstream's accessibility_label and accessibility_description checks.
@task_item n=11 part=not_needed
The network stand-ins HeldNet, LateNet and LoadNet and the load-wait behaviour, which the coverage read neither adds nor changes.
