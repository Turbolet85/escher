@task_prose part=session
A codebase-health refactor: escher's own files over 800 code lines and functions over cognitive 15 were brought under the audit's bars, with no public item, signature, verb, field, span or log site changed.
The driver's key enum moved whole into a private child module of the command module and is re-exported, so every caller stands; the JSON answer tests' fixture builders moved into a test-only child module; the snapshot's unit tests moved out of line.
The session CLI's argument read, the MCP server's run loop, the Dioxus runtime probe's visit and three stand checks were split into private sibling helpers; every test kept its path (1631 listed, 0 gone, 0 new) and the touched files stay silent.
Measured: files over 800 fell 25 to 22 (ours 10 to 7), functions over cognitive 15 fell 101 to 94 (ours 7 to 0); coverage read escher-driver 97.92 %, dioxus-native-dom 89.91 %, blitz-test-harness 96.07 %, each at or above its floor.
The founder ruled upstream's oversize files, complex functions and duplicated test pairs kept as upstream's, and no route entry for escher's seven oversize stand test targets; nothing retired, no dependency, schema, harness or gate surface changed.
@task_file n=1 path=packages/escher-driver/src/json_tests.rs
what: edited — the driver's JSON answer unit tests: the command answer shapes (graph, actions, layout, lint, shot) and their field names
@task_file n=2 path=packages/escher-driver/src/json_tests/fixtures.rs
what: new — test-only fixture builders for the driver's answer tests: caused items, findings, action nodes, a layout explanation, a UI graph
@task_file n=3 path=packages/escher-driver/src/command.rs
what: edited — the driver's command module: one command per verb, the command span with its fields, refusal causes
@task_file n=4 path=packages/escher-driver/src/command/key.rs
what: new — the driver's key enum and its key names, moved whole out of the command module and re-exported from it
@task_file n=5 path=packages/dioxus-native-dom/src/snapshot.rs
what: edited — the snapshot: its nodes, ids, names, values, masked password and file values, clickable and states
@task_file n=6 path=packages/dioxus-native-dom/src/snapshot_tests.rs
what: new — the snapshot's unit tests, moved out of line: masked values, clickable, select values, aria states
@task_file n=7 path=packages/escher-driver/src/cli.rs
what: edited — escher-session's command line: the host, mcp and coverage roles read from the arguments, help, and exit codes
@task_file n=8 path=packages/escher-driver/src/mcp.rs
what: edited — escher-session's mcp role: the MCP server's JSON-RPC lines on stdout, the tool_call span and its ending, the hosted call through the session's run
@task_file n=9 path=tests/blitz-tests/tests/dioxus_runtime_probe.rs
what: edited — the Dioxus runtime probe check: walks roots, listeners and dynamic nodes and leaks no read value
@task_file n=10 path=tests/blitz-tests/tests/stand_graph.rs
what: edited — the stand's UI graph check: every graph node is a snapshot node, no served id holds the reserved form, declarations agree with the code
@task_file n=11 path=tests/blitz-tests/tests/stand_actions.rs
what: edited — the stand's actions check: each served screen lists its actionable nodes with the table's verbs
@task_file n=12 path=tests/blitz-tests/tests/stand_accessibility_ids.rs
what: edited — the stand's accessibility check: every interactive role element and region reads a name
@task_item n=1 part=touches
The rule that checks and their shared modules stay silent: new private assertion helpers were added inside three stand checks and the runtime probe; the silence census read unchanged, so a picker confirms the rule still holds over them.
@task_item n=2 part=touches
The driver's command span and one span per driver command: the command module was edited (the key enum moved out); no span, field or verb changed, so its rows are confirmed, not moved.
@task_item n=3 part=touches
The MCP tool_call span, its tool and ending fields and its command child, and the rule that a hosted call runs through the session's run alone: the server's run loop was split into a per-instance and a per-session helper with the span sites unchanged.
@task_item n=4 part=touches
escher-session's mcp role stdout of JSON-RPC lines and its exit statuses 0 and 3: reached through the split of the server's run loop; no status or line changed.
@task_item n=5 part=touches
escher-session's commands (help, mcp, coverage role command shape, malformed, boot failure) and its exit codes 0 to 3: the command line's role read was split into a role helper and an address helper; no answer or code changed.
@task_item n=6 part=touches
The driver emits no event and installs no subscriber, and escher-session diagnostics are fixed strings: the edited command line, server and command module added no log site.
@task_item n=7 part=touches
The runtime probe leaks no read value: the probe's visit was split into root, listener and dynamic-node visitors; no read value printed.
@task_item n=8 part=touches
The snapshot node and text clickable items, the masked password and file values and the multiple-select value: the snapshot's tests that pin them moved out of line, bodies unchanged.
@task_item n=9 part=touches
The refusal cause names, including unknown-verb, and the key names a command reads: the key enum and its name table moved to a child module and are re-exported, every name the same.
@task_item n=10 part=touches
The command answer shapes (graphed, linted, shot, actions, graph, layout) and the graph answer's shape: the JSON tests that pin them were split, their fixture builders moved into a test child; no test name or body changed.
@task_item n=11 part=touches
The coverage report uploaded on success: the coverage leg ran green with per-crate readings moved; no row holds a per-crate reading or a floor value, so only confirm.
@task_item n=1 part=untouched
Telemetry strategy: the tracing crate, the engine's tracing feature gate, escher crates taking tracing ungated — no manifest or dependency changed.
@task_item n=2 part=untouched
The absences of a telemetry backend, OTel export, metrics exporters, metric emission and an error-reporting service — none added.
@task_item n=3 part=untouched
escher's sink log line format, its scrub set, target allowlist, RUST_LOG configuration, resource keys and the telemetry-installed and panic events — the telemetry crate and sink were not reached.
@task_item n=4 part=untouched
The agent-run harness events and the cold-agent pipe: events, verdict fields, counted verbs, reasons, kept checks, transcripts and cleanup — none reached.
@task_item n=5 part=untouched
Script error capture: the window error event, window.onerror, script error source labels, the embedder drain of script diagnostics and the WPT report — no engine or script code changed.
@task_item n=6 part=untouched
CI integration: the workflows, the per-leg log and coverage artifacts' upload rules, the package-install diagnostic line and the build-directory sweep line — no harness or gate surface changed; CI ran green 17 of 17.
@task_item n=7 part=untouched
Recording and session state: the recording file, recording bounds, private recording and state directories, the host wire and the session lifecycle — none reached.
@task_item n=8 part=untouched
Select list labels, optgroup labels, the open list shot and the state registry's no-values rule — no select or registry code changed.
