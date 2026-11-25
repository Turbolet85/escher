@task_prose part=chunk
Lets an app declare virtual nodes inside a canvas widget painted from Rust (the custom widget, behind blitz-dom's custom-widget feature): each with an author key (non-empty, no slash), a role from a closed set, a non-empty name, and a region inside the canvas's content box.
A declaration that breaks a rule is not taken and nothing panics; two nodes with one key in one canvas are refused; each taken node gets the reserved virtual id form owner, separator, key.
The declared nodes, validated and with their ids, can be read in process; nothing new crosses any driver, session, command-line or MCP carrier.
Every founder fork (the declaration's form, the names' crossing, the role set, where the declaration lives, the region's frame) is written into the chunk's founder-forks evidence, and the phase stops at P4 with no default.
Two carried items ride the entry to its wrap: the one-host screenshot clause of the no-screenshot-step decision, and two security-summary gaps.
@task_file n=1 path=packages/dioxus-native-dom/src/element_id.rs
what: the stable element id component and its reserved virtual node id form — the virtual key separator and the function that spells a virtual node id from its owner's id and key
@task_file n=2 path=packages/blitz-dom/src/node/custom_widget.rs
what: the custom widget trait an app paints a canvas from Rust through, behind the custom-widget feature, with its commented per-widget accessibility tree reserved for multiple nodes per widget
@task_item n=1 part=needs
The virtual node declaration on a canvas widget: author key, role, name and region, and the rules each must meet (key non-empty with no slash, role from a closed set, name non-empty, region inside the canvas's content box).
@task_item n=2 part=needs
Refusal of a broken declaration: no node stands for it and no panic path is reached on untrusted input — the no-panic security rule binds the validation code and each refused case owes its own test.
@task_item n=3 part=needs
The reserved owner-key id form of a virtual node and the element-id virtual form contract: each taken node's id is the canvas element's stable id, the separator, then the key, built through the existing virtual id function and never spelled by hand.
@task_item n=4 part=needs
Duplicate keys within one canvas refused, so one id names one node only.
@task_item n=5 part=needs
The region held in the canvas's content-box frame, the frame the custom widget paints in and hears pointer events in; a region outside it is refused.
@task_item n=6 part=needs
The in-process read of what a canvas declares now, validated and with its ids — the only new reading this chunk adds.
@task_item n=7 part=needs
Nothing crosses a carrier: no driver verb, argument kind, refusal cause, session error, exit status, carrier, log field or span is added, and no declared name or key reaches a log line, stderr line, recording or answer — names and ids stay out of stand and driver logs, which hold fixed words and counts only.
@task_item n=8 part=needs
The founder's forks written to the chunk's founder-forks evidence, each with what crosses, the rule it meets, each option's measured cost, its dependences, whose it is and the most conservative option that still builds the entry; the phase stops at P4 with no default taken.
@task_item n=9 part=needs
Authored text on a ratified carrier is a widening (the founder's surfaces rule): whether a declared name crosses is a fork, and a plan that crosses nothing leaves it to the later entry.
@task_item n=10 part=needs
Unit tests of the declaration and its validation reached through the custom-widget feature, with the harness's accessibility turned on only where a test reads the tree; tests of the new code live with that code and add no new integration target unless the plan says so.
@task_item n=11 part=needs
Each new test is controlled red-before-green, and each control enters the plan with its red reading; the control edit leaves earlier test targets green.
@task_item n=12 part=needs
Measurement probes run in a worktree outside the build directory, so the target sweep never reaches it, and no plan step deletes that worktree.
@task_item n=13 part=needs
No raw log carrying a temporary or home path goes into evidence; a guard's refusal of a write is a stop, never worked around.
@task_item n=14 part=needs
The new code meets the per-crate coverage floor, clippy over the workspace and upstream blitz-dom, the cognitive-complexity ceiling and the minimum supported Rust version.
@task_item n=15 part=needs
The carried screenshot clause: the no-screenshot-step decision still says screenshot repeatability is asserted on one host only, while the cross-OS canvas check holds canvas-only shot digests equal on three hosts; the plan names it so this chunk's wrap report carries the row's update, never edited by hand.
@task_item n=16 part=needs
The carried security-summary gaps (OS drag-and-drop as an untrusted input source, the dropped-file reader with no path restriction) are written only by the wrap's leaf pass; the plan names them and adds no code step for them.
@task_item n=17 part=needs
The HEAD commit's CI verdict is not yet available and is not read as green; the chunk's base reads green on every check — per-package green is not the workspace green.
@task_item n=18 part=needs
The premises to close at P3: nothing produces a virtual node id yet, the widget trait's commented per-widget accessibility tree, and AccessKit carrying nodes with no DOM behind them with bounds and a transform on the parent.
@task_standing n=1
convention: a new unit test sits in an inline test module in the source file it tests
home: 4-unit-test-strategy · unit-test-strategy
@task_standing n=2
convention: a new test is controlled red-before-green, its expected value pinned as a literal, a table-driven test asserting its row count, and a failure printing no argument value
home: 4-unit-test-strategy · unit-test-strategy
@task_standing n=3
convention: test data is stated in code, hand-written, nothing seeded and no mocking library
home: 7-test-data-fixtures · test-data-fixtures
@task_standing n=4
convention: a new Rust test runs on the built-in test harness and drives the engine through the headless harness where it needs a document
home: 3-test-harness-contract
@task_item n=1 part=not_needed
The accessibility snapshot and its diff, click on a virtual node, and the covered, off-screen, disabled and stale refusals: owned by the later entry that makes virtual nodes elements.
@task_item n=2 part=not_needed
The UI graph, the actions answer, handler coverage, layout explained and exploration: owned by the later entry that brings virtual nodes into every read.
@task_item n=3 part=not_needed
The Circle Drawer stand boot and its circles as nodes, and draw commands grouped by node: owned by the Circle Drawer entry.
@task_item n=4 part=not_needed
Canvas keyboard reach and keyboard input helpers: owned by the canvas keyboard path entry.
@task_item n=5 part=not_needed
The session wire, session host lifecycle, session state directory and session errors: the chunk adds no reading on the session socket.
@task_item n=6 part=not_needed
The escher-session command line and MCP role contracts (tools list, help, exit codes, protocol revision, snapshot budget): no command, tool or exit status is added.
@task_item n=7 part=not_needed
The lint defects and their finding order: the canvas lint chunk's, untouched here.
@task_item n=8 part=not_needed
The virtual clock, advance, timer ticks and settle: the declaration involves no time.
@task_item n=9 part=not_needed
The cold-agent pipe and the agent-run verbs, events, selections and exit codes: no agent run is added or changed.
@task_item n=10 part=not_needed
The WPT runner, its suites, expectations, status set and pinned commit: no conformance reading changes.
@task_item n=11 part=not_needed
Session recording and committed recorded checks, their replay and byte-equality: no check is recorded or replayed.
@task_item n=12 part=not_needed
The capability statement and capability claims: this chunk claims no capability and v040-11 stays pooled.
