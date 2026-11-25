@task_prose part=chunk
Circle Drawer's canvas widget declares one virtual node per circle it paints (author key, role, non-empty name, region in its content box), so each circle reads as a node, owner///key, in the accessibility tree, snapshot, diff, UI graph, actions, layout and exploration of a served app for the first time.
A click on a circle's node acts on its circle and its diff names it; a covered, off-screen, disabled or removed circle node is refused with that cause, checked on the stand in both layout modes.
The canvas reading (the canvas verb) groups a canvas's draw commands under the virtual node that covers them, and a changed command reads with an identity in the reading's line diff.
Seven carried items ride along: the label region read against the shape or virtual node, covered among overlapping circles, the binary-level reading of a node's id and name through the socket host, command-line stdout and a cold-agent live run, a windows-only unused-import warning, handler coverage of a click on a virtual node, and the lint reading no virtual node.
Every committed stand pin that reads Circle Drawer's screen is swept, literal pins and the capability statement's stated sizes and counts included.
@task_file n=1 path=examples/seven_guis/src/tasks/circle_drawer.rs mark=edits
what: Circle Drawer's canvas widget, the served app's canvas that paints circles and carries an onclick listener — edits: the canvas widget's virtual-node declaration (key, role, name and region of each circle)
@task_file n=2 path=packages/escher-driver/src/canvas.rs mark=edits
what: the canvas reading behind the canvas verb and its line diff — edits: the grouping of draw commands under a virtual node, a draw command's identity in the diff
@task_file n=3 path=packages/escher-driver/src/lint.rs
what: the lint and its findings, which read no virtual node today
@task_file n=4 path=packages/escher-driver/src/canvas_lint.rs
what: the canvas lint and the region it reads a canvas label against
@task_file n=5 path=packages/escher-driver/src/coverage.rs
what: handler coverage and its coverage line, counting a click on a virtual node as the canvas's own listener under the canvas's id
@task_file n=6 path=packages/escher-driver/src/execute.rs
what: the driver's command execution, click on a node and its refusal causes (covered, off-screen, disabled, not found)
@task_file n=7 path=packages/dioxus-native-dom/src/virtual_node.rs
what: the declared virtual node read in process and its covered reading by an element over its region's centre
@task_file n=8 path=tests/blitz-tests/tests/virtual_nodes_in_every_read.rs mark=edits
what: the check of virtual nodes in every read (graph, actions, layout, exploration) on fixture canvases — edits: the snapshot import, put behind a unix-only gate
@task_file n=9 path=examples/seven_guis/tests/cli_commands.rs mark=edits
what: seven_guis' process check of the escher-session command line and its stdout answers
@task_file n=10 path=examples/seven_guis/tests/mcp_server.rs mark=edits
what: seven_guis' process check of the escher-session mcp role and its JSON-RPC stdout
@task_file n=11 path=docs/capabilities.md mark=edits
what: the capability statement, updated by hand — edits: its stated sizes and counts that read Circle Drawer's screen, the section on a virtual node's covered
@task_item n=1 part=needs
The canvas read opens no span, and the declared-node read logs nothing: grouping draw commands under a virtual node and reading Circle Drawer's declared circles add no span, event or log line.
@task_item n=2 part=needs
The driver's command span for a click on a circle's node and for the canvas verb keeps its closed field set (verb, settled, passes, busy, cause and the added, changed and removed counts) and carries no node id, circle name or call content.
@task_item n=3 part=needs
Refusal cause names: a refused click on a circle node names covered, off-screen, disabled, or not-found or stale for a removed circle, from the existing closed set — no new cause name.
@task_item n=4 part=needs
The tool_call span and its ending (accepted, refused, error) for an MCP call that clicks or reads a circle's node through the hosted session's run alone.
@task_item n=5 part=needs
The UI graph, actions and layout answers now carry Circle Drawer's virtual nodes by owner///key id and name: their content and logging rules hold — the answers are never logged and their nodes' names stay in the answer.
@task_item n=6 part=needs
The handler coverage line names no virtual node and counts a click on one as the canvas's own listener under the canvas's id; the coverage line is never logged.
@task_item n=7 part=needs
The canvas command answer and the canvas reading's line diff: a changed draw command now reads with an identity under its virtual node instead of one line removed and one added; the answer stays unlogged.
@task_item n=8 part=needs
The lint and canvas-lint findings answer: the lint still reads no virtual node (no overlap, clip or off-screen finding on a declared region) with overlapping circles on the stand, and a label's region is read against the shape or virtual node under it.
@task_item n=9 part=needs
The snapshot and its diff carry a circle's node as clickable, and a click's diff names the circle's node; the snapshot node's clickable flag holds for a virtual node.
@task_item n=10 part=needs
Binary-level reading of a virtual node's id and name: the escher-session host wire answer, the command line's stdout answer and the mcp role's stdout each carry it through one session run, and the escher-session exit codes and mcp role exit status hold.
@task_item n=11 part=needs
The cold-agent live run against Circle Drawer: its counted verbs, verdict fields and reasons stay closed and hold no content; its server log is read only to count by span name; its transcript and kept checks are never printed or copied into evidence unmasked.
@task_item n=12 part=needs
Answers echo nothing the caller typed and a part's answer is never logged, for the new process checks that read a circle's node on the binary.
@task_item n=13 part=needs
Auto-exploration of Circle Drawer reaches its virtual nodes, adds no span of its own and still catches a panicking call.
@task_item n=14 part=needs
The capability statement is updated by hand for the swept stated sizes and counts and the virtual node's covered reading.
@task_item n=15 part=needs
No raw log with a temp or home path enters committed evidence; committed transcript copies are masked and committed checks hold no private data.
@task_item n=16 part=needs
Probe worktrees and recordings stay local and private: a probe worktree lives outside the build directory, cleanup touches only its own state, and session state and recording directories stay private.
@task_standing n=1
convention: each new process check is silent, unsets RUST_LOG where it asserts an empty stderr, and its failure names a kind and a count, never an answer
home: 3-observability-harness-contract
@task_standing n=2
convention: each new test or kept check reads answers in the match only and never logs or prints a graph, actions, layout, canvas or coverage answer
home: log-coverage · 6-log-coverage
@task_standing n=3
convention: each new committed check, fixture or evidence file holds no private data, no secret and no temp or home path
home: ci-integration · 9-ci-integration
@task_standing n=4
convention: each new test's names, labels and decoded screen text stay in the match and are never stored in a span field or log line
home: pii-scrubbing-compliance · 8-pii-scrubbing-compliance
@task_item n=1 part=not_needed
Telemetry backends, OTel export, metrics exporters and metric emission: the chunk adds no counter, gauge, histogram or exporter.
@task_item n=2 part=not_needed
WPT score publishing and the WPT report's messages: the chunk runs no WPT.
@task_item n=3 part=not_needed
Script error surfaces, window error events, onerror arguments and script error source labels: Circle Drawer's circles involve no page script error path.
@task_item n=4 part=not_needed
Select, optgroup, multiple-select form entries, open-list shots and checkbox or radio page events: the chunk touches no form control.
@task_item n=5 part=not_needed
The package-install diagnostic line and the build-directory sweep's stdout line: the chunk changes no CI install step or sweep.
@task_item n=6 part=not_needed
escher-telemetry's sink installation, panic hook, resource keys and engine tracing feature gating: the chunk installs no sink and changes no tracing setup.
@task_item n=7 part=not_needed
Stylesheet loading and file: loads from non-file documents: the chunk loads no resource.
@task_item n=8 part=not_needed
The agent-run harness events and its test event: the chunk's live run is the cold-agent pipe, not the agent-run harness.
@task_item n=9 part=not_needed
Embedder drain of script diagnostics (take_js_errors, take_messages): no script diagnostics are read.
