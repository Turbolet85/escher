@task_prose part=session
The chunk 2026-11-07-ui-graph added a thirteenth driver verb, graph, an instance verb with no argument that answers the screen's UI graph: each snapshot node's id, owner, component, handlers with declared and confirmed writes, and the states it reads.
The graph answer carries ids, component names, registered state names, handler event names, closed flag words (undeclared, unconfirmed) and booleans only, never a value. It crosses the socket, the command line, MCP and kept checks. This is a boundary widening, ratified by the founder's P4 fork 1 (inputs#I4) and halted on one Validate card before apply.
dioxus-native-dom gained a graph module (behind accessibility) and a signal_registry module (use_graph_state, SignalRegistry, DeclaredWrite). The bridge's event dispatch now sets the registry's dispatch on every Dioxus document, but only where a registry exists, so it is inert without one.
The snapshot gained NodeState.clickable, a closed bool written after focused, so NODE_FIELDS went from 13 to 14. VERBS went from 12 to 13 and instance verbs from 8 to 9. The stable-id grammar gained a reserved fifth form {owner}///{key}, spelled by the public virtual_node_id, which no walk calls.
Measured: the Cells graph answer is 293,869 bytes, within the answer bound. The unpaged size stays a stated limit, with residual res-7cc5bi4k (target next). No dependency, manifest, Cargo.lock, deny.toml or blitz-dom change was made, and mutation_writer.rs is byte-identical.
Retired: the capability statement's Not built line for a snapshot field saying an element reacts to a click.
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
@task_file n=26 path=examples/seven_guis/tests/host_log.rs
@task_file n=27 path=examples/seven_guis/tests/mcp_log.rs
@task_file n=28 path=examples/seven_guis/tests/cli_commands.rs
@task_file n=29 path=examples/seven_guis/tests/mcp_server.rs
@task_file n=30 path=examples/seven_guis/tests/common/mod.rs
@task_file n=31 path=tests/blitz-tests/checks/counter-cli.json
@task_file n=32 path=tests/blitz-tests/checks/counter-mcp.json
@task_file n=33 path=tests/blitz-tests/tests/common/mod.rs
@task_file n=34 path=tests/blitz-tests/tests/session_common/mod.rs
@task_file n=35 path=tests/blitz-tests/tests/stand_mcp_tools.rs
@task_file n=36 path=tests/blitz-tests/tests/stand_non_lean_tasks.rs
@task_file n=37 path=docs/capabilities.md
@task_file n=38 path=README.md
@task_file n=39 path=scripts/cold-agent.sh
@task_file n=40 path=.github/scripts/test_cold_agent.py
@task_item n=1 part=touches
The driver command schema's closed table of twelve verbs is now thirteen. graph is checked by validate as a verb with no argument, and an argument is refused malformed with nothing run. Any count of the verbs, the MCP tools or the driver commands as tools moves from twelve to thirteen.
@task_item n=2 part=touches
The instance verbs went from eight to nine: graph is the ninth, after lint. Any row reading the eight instance verbs is stale. A row naming lint the eighth instance verb still holds.
@task_item n=3 part=touches
The node keys went from thirteen to fourteen: clickable, a closed bool, joins after focused in the snapshot, its text token, diffs and the written node. It falls under the widening rule as a closed-value field inside an already ratified carrier.
@task_item n=4 part=touches
The carriers of the driver session socket, the MCP stdio server and kept checks (ids, names, control values, snapshot text, diffs, screenshots, a lint's findings) now also carry a UI graph. That graph holds ids, component names, registered state names, handler event names, closed flag words and booleans. This reaches the threat-model vectors for local IPC and local stdio, and the input-validation and API-security rows that list what a reply carries.
@task_item n=5 part=touches
The stable element id contract gained a reserved fifth form {owner}///{key}. The public virtual_node_id returns None for an empty or slash-bearing key and is called by no walk. The id grammar states why no tier can hold the triple slash, and no served id holds it.
@task_item n=6 part=touches
The two committed checks of the scripted stand-Counter sessions were re-recorded (the CLI check +4/-4 lines, the MCP check +6/-6) and still replay through the strict reader. A kept check may now hold a graph step and its answer.
@task_item n=7 part=touches
The Dioxus bridge's event dispatch, on every Dioxus document, now sets and clears the registry's dispatch around runtime.handle_event, but only when a SignalRegistry context exists at the root scope. This is an engine-path change, inert without a registry. The buffered render, initial_build, poll and the mounted-event flush are untouched.
@task_item n=8 part=touches
The MCP server answers the graph tool's call with the text block of its JSON line. graph is listed after lint because VERBS orders it. The listing and result code did not change.
@task_item n=9 part=touches
escher-session's command-line usage line gains graph after lint. The command line answers graph with one uncoloured JSON line, status 0, so the stdout rule of one JSON line per command now covers graph.
@task_item n=10 part=touches
Logging: graph's command span records verb alone, and its tool_call parents one command span. The driver still has 2 info_span sites. host_log and mcp_log read 0 id, name and graph needles at trace in both builds. The new files hold no log site, print or env read.
@task_item n=11 part=touches
The cold-agent pipe's VERBS gains graph (thirteen), and its fake server answers graph with an empty nodes and states object. The verdict's capability-unused set is unchanged: graph is not required of a cold agent. A cold-agent check's steps are validated against the thirteen-verb schema.
@task_item n=12 part=touches
Password and file-input masking: a sentinel typed into crud-name occurs 0 times in the graph answer, and the graph holds names only, never a control value. The masked-value rule is unmoved, and the graph is a carrier it must now cover.
@task_item n=13 part=touches
Reply bounds: the graph answer is unpaged. Cells reads 293,869 bytes, within the answer bound, after it read 547,685 at first build and 522,692 after the readers narrowing. The plan's forecast of under 200 KB was disproved. The stated limit An unpaged graph and residual res-7cc5bi4k own the remaining size.
@task_item n=14 part=touches
Line citations into dioxus-native-dom's snapshot and diff code moved: snapshot.rs gained lines at 77-80, 112-116, 134-135 and 199, ahead of a 278-305 citation, and snapshot_diff.rs gained lines at 55-56, 63 and 74, ahead of a :111 citation. The file-input masking row's citations shift.
@task_item n=15 part=touches
Environment-read citations moved. tests/blitz-tests/tests/session_common/mod.rs gained lines at 19 and 134-147, ahead of the :349 citation. examples/seven_guis/tests/host_log.rs gained lines 4-5, ahead of the :28 citation. packages/escher-driver/src/mcp_tests.rs gained lines at 399, 408 and 413-416, ahead of the 654-655 citation. The claims themselves are unmoved: the new code reads no env var.
@task_item n=16 part=touches
Re-read unmoved: packages/dioxus-native-dom/src/mutation_writer.rs is byte-identical to upstream 434e0661, so its try_element_to_node_id and element_to_node_id citations (41-49) and the panic-path claims on it stand.
@task_item n=17 part=touches
The new in-process app API (use_graph_state, SignalRegistry, DeclaredWrite) is exported through dioxus-native's re-export of dioxus-native-dom. It takes dioxus-core types alone. Its Debug prints counts only, it holds no NodeId beyond the dispatch chain, and it has no unwrap on app input. This reaches any no-NodeId-kept, no-panic-on-input or no-log-site rule over dioxus-native-dom.
@task_item n=18 part=touches
The graph reader DioxusDocument::graph is read-only: a graph read changes nothing and holds no typed text (stand_graph, both layout modes). This reaches rules that a read verb runs no step and records ids only from what it answers.
@task_fact n=1 slug=input-validation
fact: Driver command schema (escher-driver) — graph is the thirteenth verb and ninth instance verb, taking no argument (an argument is refused malformed with nothing run); its answer carries ids, owner ids, component names, registered state names, handler event names (without on), the closed flag words undeclared and unconfirmed, and booleans, never a control value or typed text.
@task_fact n=2 slug=threat-model-summary
fact: The UI graph crossing the session socket, the command line, MCP and kept checks is a boundary widening, ratified by the founder's P4 fork 1 (Новый глагол graph, inputs#I4) and halted on one Validate card with one confirm before apply.
@task_fact n=3 slug=input-validation
fact: Markup attributes — the stable element id reserves a fifth form {owner}///{key} for virtual nodes; virtual_node_id spells it and returns None for an empty or slash-bearing key, no walk calls it, and no served id holds the triple slash.
@task_fact n=4 slug=input-validation
fact: Dioxus bridge dispatch — on every Dioxus document the event handler sets the SignalRegistry's dispatch around runtime.handle_event and clears it after, only where a registry context exists at the root scope; a page with no registered state reaches no registry and dispatches as before, an engine change inert without a registry.
@task_fact n=5 slug=input-validation
fact: Signal registry (dioxus-native-dom use_graph_state) — an in-process app API taking dioxus-core types alone; each entry leaves with its scope, its Debug prints counts only, it holds no NodeId beyond the dispatch chain, and it has no log site, print, env read or unwrap on app input.
@task_fact n=6 slug=input-validation
fact: Snapshot reading clickable — a closed bool, true where an element's current listeners hold click and it is no disabled form control, written after focused and in the text only when true; a closed-value field inside a ratified carrier, so no widening.
@task_fact n=7 slug=api-security
fact: The graph answer is unpaged and takes no part argument; Cells reads 293,869 bytes within the answer bound, its size the stated limit An unpaged graph, owned by residual res-7cc5bi4k (target next).
@task_fact n=8 slug=logging-monitoring
fact: The graph verb's command span records verb alone and its MCP tool_call parents one such span; host_log and mcp_log read 0 id, name and graph needles at trace with one graph call each, in both builds.
@task_fact n=9 slug=input-validation
fact: Cold-agent pipe — its VERBS holds graph (thirteen) and its fake server answers graph with empty nodes and states; graph is not required of a cold agent, and the capability-unused set is unchanged.
@task_item n=1 part=untouched
Authentication and authorization: no handshake, token, secret or peer check was added. The socket's filesystem access (0700 directory, 0600 socket) and the MCP server's spawning-client trust are unchanged.
@task_item n=2 part=untouched
Dependency security and supply chain: no dependency, manifest, lockfile, deny.toml or CI workflow changed. The audit tool, pinning, serde_json, png and color rows stand.
@task_item n=3 part=untouched
Secret management storage and what counts as secret: no secret source, credential or CI secret was added or read.
@task_item n=4 part=untouched
Remote web content and script execution: blitz-dom, blitz-net, blitz-html and blitz-vibey-script did not change. URL, data and file URL, stylesheet, font, image, SVG, CSS, selector and form input rows are unmoved.
@task_item n=5 part=untouched
The browser app, rdme, the WPT runner, the example CLI tools, apt-install.sh, coverage_floor.py and agent-run.sh argument handling are untouched.
@task_item n=6 part=untouched
escher's own log sink: the escher-telemetry target allowlist, its redaction and drop rules, and the RUST_LOG read are unchanged. No new log target was added.
@task_item n=7 part=untouched
Data at rest for the browser (the sqlite history, the HTTP cache), the clipboard, the file dialog and history deletion are untouched.
@task_item n=8 part=untouched
The driver session state directory and recording directory permission rules, the session label rule and the idle-expiry rule are unchanged.
@task_item n=9 part=untouched
Engine panic paths and graceful degradation in blitz-dom, blitz-net, blitz-shell, stylo_taffy and the WPT runner are untouched.
@task_item n=10 part=untouched
The Bootstrap phases, Compliance Controls, Security Anti-Patterns and Security Decisions Log sections gain nothing from this chunk.
