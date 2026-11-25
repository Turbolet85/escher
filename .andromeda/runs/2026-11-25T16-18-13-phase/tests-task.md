@task_prose part=chunk
Circle Drawer's canvas widget declares one virtual node per circle it paints (author key, role, non-empty name, region in its content box), making Circle Drawer the first served app whose circles read as nodes with the reserved owner-key id form in the accessibility tree, snapshot, diff, UI graph, actions, layout and exploration.
A click on a circle's node acts on its circle and the diff names it; a covered, off-screen, disabled or removed circle node is refused with that cause, checked on the stand.
The canvas reading groups a canvas's draw commands under the virtual node that covers them, and a changed command reads with an identity in the reading's line diff instead of one line removed and one added.
Stand checks run in both layout modes, and the virtual node's id and name are read at the binary level (command line stdout, MCP server, cold-agent pipe).
All seven folded CARRYs are re-verified here (command identity, label region, virtual-node covered, binary-level reading, windows unused-import warning, handler coverage of a canvas, the lint reading no virtual node), and every literal stand pin that reads Circle Drawer's screen is re-measured and swept.
@task_file n=1 path=examples/seven_guis/src/tasks/circle_drawer.rs mark=edits
what: the Circle Drawer served task and its canvas widget that paints the circles — edits: the canvas widget's virtual_nodes declaration (each circle's key, role, name and region)
@task_file n=2 path=packages/escher-driver/src/canvas.rs mark=edits
what: the driver's canvas reading (the canvas verb) and its line diff — edits: the grouping of draw commands under a virtual node, and a draw command's identity in the reading's diff
@task_file n=3 path=packages/escher-driver/src/lint.rs
what: the driver's lint — its overlap, clip and off-screen findings, which read no virtual node
@task_file n=4 path=packages/escher-driver/src/canvas_lint.rs
what: the canvas lint — a canvas label read against its region (content box cut by clip, layer or mask paths)
@task_file n=5 path=packages/escher-driver/src/coverage.rs
what: handler coverage — the coverage line, counting a click on a virtual node as its canvas's own listener under the canvas's id
@task_file n=6 path=packages/escher-driver/src/execute.rs
what: the driver's executor of acting verbs — click on a virtual node and the refusal causes (covered, off-screen, disabled, not-found, stale)
@task_file n=7 path=packages/dioxus-native-dom/src/virtual_node.rs
what: the virtual node component — a canvas's declared regions, their validation (a region outside the content box refused) and their covered reading
@task_file n=8 path=tests/blitz-tests/tests/virtual_nodes_in_every_read.rs mark=edits
what: the blitz-tests target reading virtual nodes in every read (UI graph, actions, layout, exploration) on fixture canvases — edits: the snapshot import, put behind a unix-only cfg
@task_file n=9 path=examples/seven_guis/tests/cli_commands.rs mark=edits
what: seven_guis' command-line process checks running the session binary — edits: a binary-level reading of a virtual node's id and name on stdout
@task_file n=10 path=examples/seven_guis/tests/mcp_server.rs mark=edits
what: seven_guis' MCP server process checks — edits: a binary-level reading of a virtual node's id and name in the MCP answer
@task_file n=11 path=docs/capabilities.md mark=edits
what: the capability statement pinned to the driver schema — edits: its stated sizes and counts that read Circle Drawer's screen, and the claim on a virtual node's covered
@task_item n=1 part=needs
The reserved owner-key id form of a virtual node (owner, three slashes, key) as Circle Drawer's circle nodes carry it in every read: element ids, snapshot nodes, the UI graph answer, the actions answer and the layout answer.
@task_item n=2 part=needs
Click on a circle's virtual node: the acting call result and its diff name the circle node; the refusal causes covered, off-screen, disabled, not-found and stale apply to a circle node with their fixed order and printed form, and a refusal changes nothing.
@task_item n=3 part=needs
A virtual node reads covered only by an element over its region's centre, never by another virtual node of its canvas — overlapping circles put this rule on the stand (CARRY 3).
@task_item n=4 part=needs
The canvas command line verb and the canvas reading: draw commands grouped under the virtual node that covers them, and a changed command read with an identity in the line diff (CARRY 1); naming a node's id in the canvas answer changes that answer's shape.
@task_item n=5 part=needs
Handler coverage over a canvas handler: a virtual node carries no handler of its own, so the coverage line lists none for it and counts a click on it as the canvas's own listener under the canvas's id; Circle Drawer's canvas carries onclick (CARRY 6).
@task_item n=6 part=needs
The lint reads no virtual node: no overlap, clip or off-screen finding on a declared region, held by the guard that the lint holds no virtual node, now on Circle Drawer's overlapping circles (CARRY 7); the lint's defect kinds and finding order stay as they are for elements.
@task_item n=7 part=needs
The canvas lint's label region: a label read as the shape or the virtual node under it, against the content box cut by each clip, layer or mask path (CARRY 2).
@task_item n=8 part=needs
A declared region must lie inside the canvas's content box (a region outside it is refused), so a circle near the canvas edge bounds the region the chunk's code declares.
@task_item n=9 part=needs
Binary-level reading of a virtual node's id and name through the session host's answer, the command line's stdout, the MCP server's tool result and the cold-agent pipe (CARRY 4), each by the package's own binary.
@task_item n=10 part=needs
Every committed stand pin that reads Circle Drawer's screen is re-measured and swept, literal pins included: the cross-OS answer-digest tables of the served tasks and of the canvas, the canvas commands table across hosts, the canvas lint and non-lean task checks and the Circle Drawer canvas check.
@task_item n=11 part=needs
The capability statement's stated sizes and counts that move when the circles become nodes, held to the driver schema by the capability-statement check and its claim names.
@task_item n=12 part=needs
The windows build's unused-import warning in the virtual-nodes-in-every-read target, fixed by a unix-only cfg and checked by a cargo check of the tests, locked, for the windows gnu target of blitz-tests (CARRY 5); socket session checks stay unix-gated.
@task_item n=13 part=needs
Circle Drawer's stand boot (the stand's Circle Drawer boot) as the home of the new stand checks, which run in both layout modes, on the bundled font without skip, and inside stand-repeat with a zero-failure budget.
@task_item n=14 part=needs
Snapshot within its text byte budget and the session wire's answer bound, as circle nodes add lines to Circle Drawer's snapshot and graph answers.
@task_item n=15 part=needs
Exploration and committed recorded checks over Circle Drawer: recorded checks stay byte-equal to the binary's recording and are kept fresh where the circle nodes change what a step reads.
@task_item n=16 part=needs
Cross-OS byte comparisons of the new readings hold no screenshot step, and the canvas shots stay compared across hosts by digest.
@task_item n=17 part=needs
Evidence rules for this take-up: every new test is controlled red-before-green and a control enters with its red reading; no raw log carrying a temp or home path enters evidence; a probe worktree lives outside the build directory and no step deletes it.
@task_item n=18 part=needs
Stand and session logs carry no ids or names, so a circle node's id and name appear only in answers, never in the command span or log lines.
@task_standing n=1
convention: each new stand check runs in both layout modes
home: test-scope-summary
@task_standing n=2
convention: each new stand check runs on the bundled font with no skip, and is witnessed inside stand-repeat
home: test-strategy
@task_standing n=3
convention: each new test file is one test target, and shared stand tables and fixtures live in their one shared module rather than as targets
home: test-strategy · 3-test-harness-contract
@task_standing n=4
convention: each new test is controlled red-before-green, pins its expected value as a literal and prints no argument value on failure
home: unit-test-strategy
@task_standing n=5
convention: each table-driven test asserts its row count
home: unit-test-strategy
@task_standing n=6
convention: each new command-line or MCP process check runs the package's own binary, owns its host, drains its pipes and is unix-gated
home: test-strategy · e2e-test-strategy
@task_standing n=7
convention: each new session check uses no sleep or wall-clock read and keeps no screen content in logs
home: integration-test-strategy
@task_item n=1 part=not_needed
WPT conformance rows — the runner, its CLI, statuses, outputs, pinned commit, Thai font and upstream-only workflow: the chunk adds no WPT suite or expectation.
@task_item n=2 part=not_needed
Typed password rows and the password caret, the select list painting and chosen select row colours: Circle Drawer's circles hold no password or select.
@task_item n=3 part=not_needed
The virtual clock, advance and timer-tick rows: the chunk's circle nodes and canvas grouping drive no timer or time.
@task_item n=4 part=not_needed
Network stand-ins, the load page, settle on busy loads and the file-URL rule: no load is involved in Circle Drawer's canvas.
@task_item n=5 part=not_needed
The agent-run five-verb contract, its events, exit codes, selections and state: the chunk adds no verb or selection to it.
@task_item n=6 part=not_needed
The apt-install and coverage-floor script exit codes, CI action pinning, cache save branches and trigger branches: the chunk adds no CI job or script.
@task_item n=7 part=not_needed
Session wire versioning and lifecycle rows (hello, version refusal, stop, idle expiry, connection): the protocol's form is unchanged by circle nodes.
@task_item n=8 part=not_needed
Keyboard reach of the circles and focus rows: keyboard paths over a canvas are the next entry's, not built here.
@task_item n=9 part=not_needed
Pan and zoom of a rotated or scaled canvas: its virtual nodes are another entry's.
@task_item n=10 part=not_needed
Upstream buffered-render and mounted-flush test rows and upstream oversize file or clone-pair keeps: the chunk touches no upstream test.
