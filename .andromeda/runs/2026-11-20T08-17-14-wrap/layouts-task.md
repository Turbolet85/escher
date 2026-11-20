@task_prose part=session
A codebase-health pass, behaviour unchanged: escher's own files over 800 code lines and functions over cognitive 15 were split under the audit's bars.
No public item, signature, verb, field, answer, refusal, exit code, span or log site changed; every workspace test kept its path (1631 listed, 0 gone, 0 new).
The driver's key enum moved whole into a private child module and is re-exported under the same name; the command-line role reading and the MCP server's run loop were each split into private helpers.
The snapshot reader's unit tests moved out of line to a test file of their own, and the JSON answer tests' fixture builders moved into a test-only child.
The stand checks for the interaction graph, the actionable-node listing and the accessible names were split into assertion helpers, each message as written; the Dioxus runtime probe's walk was split the same way.
Measured: files over 800 25 to 22, functions over cognitive 15 101 to 94 (ours 7 to 0); the founder ruled upstream's oversize files, complex functions and test clones kept as upstream's and no route entry for escher's seven oversize test targets.
@task_file n=1 path=packages/escher-driver/src/json_tests.rs
what: the unit tests of the escher-session JSON answers — the answer shapes of snapshot, actions, layout, lint, graph and the refusals
@task_file n=2 path=packages/escher-driver/src/json_tests/fixtures.rs
what: test-only fixture builders for those JSON answer tests — caused items, findings, action nodes, a layout explanation, an interaction graph
@task_file n=3 path=packages/escher-driver/src/command.rs
what: the escher-session command parser — the command form, its verbs and the key names a press takes
@task_file n=4 path=packages/escher-driver/src/command/key.rs
what: the named keys the escher-session command form accepts (the key enum, its list of twelve and their names)
@task_file n=5 path=packages/dioxus-native-dom/src/snapshot.rs
what: the snapshot reader — the nodes, ids, names, roles, states and values the escher-session snapshot answer serves
@task_file n=6 path=packages/dioxus-native-dom/src/snapshot_tests.rs
what: the snapshot reader's unit tests — labels, masked password and file values, checked, pressed, selected, invalid, disabled, select options, clickable
@task_file n=7 path=packages/escher-driver/src/cli.rs
what: the escher-session command line — reading its role (host, MCP, coverage) and the task address, its usage and exit codes
@task_file n=8 path=packages/escher-driver/src/mcp.rs
what: the escher-session MCP server over local stdio — its instance and session loop, tools and end
@task_file n=9 path=tests/blitz-tests/tests/dioxus_runtime_probe.rs
what: the probe check walking the Dioxus runtime's roots, listeners and dynamic nodes
@task_file n=10 path=tests/blitz-tests/tests/stand_graph.rs
what: the stand check that every interaction-graph node is a snapshot node, no served id holds the reserved form, and each task's declarations agree with its code
@task_file n=11 path=tests/blitz-tests/tests/stand_actions.rs
what: the stand check that each served screen lists its actionable nodes with the verbs table
@task_file n=12 path=tests/blitz-tests/tests/stand_accessibility_ids.rs
what: the stand check that every interactive-role element and every region reads a name, by the stand's stable element ids
@task_item n=1 part=touches
The escher-session MCP server rows (stdio, end, tools, result bound): the server's run loop was split into an instance and a session helper, behaviour unchanged — a picker confirms no row holds the old shape.
@task_item n=2 part=touches
The escher-session command line rows (command form, coverage role usage, exit codes): the role and address reading was split into helpers, with no usage line, form or exit code changed.
@task_item n=3 part=touches
The key names the command form takes: the key enum moved into a private child module and is re-exported under the same name; the list of twelve and each name are unchanged.
@task_item n=4 part=touches
The escher-session snapshot answer and its part option: the snapshot reader's unit tests moved out of line, the reader itself unchanged.
@task_item n=5 part=touches
The decision stand ids as attributes alone and the reserved-separator rule under it: the stand graph check asserting no served id holds the reserved form was split into helpers, each message as written.
@task_item n=6 part=touches
The decision stand names, roles and states by attributes alone, and the stand stable element ids contract items: the accessible-names check over interactive elements and regions was split into helpers, by the same ids.
@task_item n=7 part=touches
The escher-session actions answer and the escher-session graph answer: the checks that read them over the stand (every actionable node listed with the verbs table; declarations agree with code once handlers run) were restructured, assertions unchanged.
@task_item n=8 part=touches
The escher-session served tasks rows: the stand checks iterate each served task as before; no task was added, renamed or dropped.
@task_item n=9 part=touches
Indirect: the decision session roles share the stand boot and the decision held instance reads as a fresh boot rule over the MCP and command-line roles whose reading code was split, unchanged in effect.
@task_item n=10 part=touches
Masked values: the snapshot tests holding that a password or file input reads a masked value moved file whole, unchanged — a row on caller input or masked values is ruled over code the session reached.
@task_item n=11 part=untouched
The ShellProvider window chrome contract items: no window, decoration or maximise control was reached.
@task_item n=12 part=untouched
The agent-run, cold-agent, multi_doc and paint_bench command-line contracts and the cold-agent decisions: none of those scripts or binaries was reached.
@task_item n=13 part=untouched
The stand's page and layout decisions (document skeleton, pinned viewport, accent colour, list keyboard region, open list row bounds, scrolling, clipping boxes, network provider, order page controls, flight type select, stand lints at boot): no stand screen, style or task code changed.
@task_item n=14 part=untouched
The escher-session recording, screenshot, layout, lint and host-log rows: no recording, screenshot, layout or log behaviour changed.
@task_item n=15 part=untouched
The absence of arbitrary index.html templates and the not-yet-measured surfaces (mobile-native, web-spa): nothing of theirs was reached.
