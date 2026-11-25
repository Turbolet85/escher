@task_prose part=chunk
Makes a canvas's taken virtual nodes (id {owner}///{key}) read by every driver read that today holds DOM elements alone: the UI graph, actions, handler coverage, the layout explanation and exploration.
Lifts the delegate's provisional limits this entry owns: the graph, actions, lint and exploration holding no virtual node (F9 A), only click taking a virtual id with layout and the other id verbs reading it not-found (F10 B), and a virtual node's clickable reading false (F12 A).
Sweeps the literal pins with the words: counts, help texts, the describe() self-description, VERBS.len(), the capabilities doc's counts and their pin, and the guards that assert no virtual id.
Proves it on a fixture canvas a test declares, in both layout modes; adds no new carrier, verb, exit, session error or span field.
@task_file n=1 path=packages/dioxus-native-dom/src/graph.rs mark=edits
what: the UI graph builder, one graph node per snapshot node — edits: the virtual-node exclusion (F9 A), now one graph node per virtual snapshot node with its owner
@task_file n=2 path=packages/escher-driver/src/actions.rs mark=edits
what: the actions answer, each node with the verbs it takes and the cause each would be refused with — edits: the listing of a virtual node
@task_file n=3 path=packages/escher-driver/src/lint.rs mark=edits
what: the driver's lint over the snapshot — edits: its exclusion of virtual nodes
@task_file n=4 path=packages/escher-driver/src/explore.rs mark=edits
what: exploration, the candidates it acts on — edits: a virtual node as a candidate
@task_file n=5 path=packages/escher-driver/src/coverage.rs mark=edits
what: handler coverage, the coverage role, reach read where a kept check acts on a node — edits: a virtual node's reach
@task_file n=6 path=packages/escher-driver/src/execute.rs mark=edits
what: the verbs' target resolution — edits: resolve, locate and check_target, the arm that reads a virtual id not-found for every verb but click (F10 B)
@task_file n=7 path=packages/dioxus-native-dom/src/snapshot.rs mark=edits
what: the snapshot and its accessibility fields — edits: a virtual node's clickable (F12 A)
@task_file n=8 path=packages/dioxus-native-dom/src/reached.rs mark=edits
what: the reached record, what a verb's action reached
@task_file n=9 path=docs/capabilities.md mark=edits
what: the capabilities doc, its counts and the pin that checks them — edits: the counts, their pin, the Pinned by a check list
@task_file n=10 path=tests/blitz-tests/tests/stand_graph.rs mark=edits
what: the stand's UI graph check — edits: its guard asserting no virtual id in the graph
@task_file n=11 path=tests/blitz-tests/tests/stand_actions.rs mark=edits
what: the stand's actions check — edits: its guard asserting no virtual id in the actions answer
@task_file n=12 path=tests/blitz-tests/tests/virtual_nodes_declared.rs mark=edits
what: the declared-virtual-nodes check — edits: its guard asserting no virtual id
@task_file n=13 path=tests/blitz-tests/tests/virtual_nodes_as_elements.rs mark=edits
what: the virtual-nodes-as-elements check, a fixture canvas in both layout modes — edits: the_graph_actions_and_lint_hold_no_virtual_node
@task_item n=1 part=needs
The UI graph: one graph node per snapshot node, now including one per virtual snapshot node with its owner, in process and as the graph verb; the F9 A carve-out from that rule is removed.
@task_item n=2 part=needs
The UI graph line and the graph answer and its arguments on escher-session: what a virtual graph node's line carries and its id {owner}///{key}.
@task_item n=3 part=needs
The actions answer and the actions line: a virtual node listed with the verbs it takes and the refusal cause each would meet, decided by the verbs' own checks, not a second rule.
@task_item n=4 part=needs
The ratified refusal-cause set: a virtual node's refusals in actions and in the verbs must use causes already in the set; a new cause is a fork, not a plan step.
@task_item n=5 part=needs
Handler coverage: the coverage answer, the coverage line and the coverage check files, with a virtual node's reach read where a kept check acts on it, within the check files' read bound and refusal.
@task_item n=6 part=needs
The layout explanation: the layout verb and Session::layout answer a virtual id rather than reading it not-found; the layout answer and arguments contract carries that answer.
@task_item n=7 part=needs
The verbs' target resolution (F10 B): which verbs take a virtual id — today click only, with type, scroll, screenshot with an id, layout and canvas reading it not-found; the screenshot [--id] and canvas verb contracts are read against any change.
@task_item n=8 part=needs
The snapshot's clickable field for a virtual node (F12 A, reading false today) and the accessibility fields a virtual snapshot node carries.
@task_item n=9 part=needs
Exploration: a virtual node as a candidate it acts on, through the same verbs and checks as an element candidate.
@task_item n=10 part=needs
The lint: virtual nodes now held by the lint read, alongside the canvas lint over a label past its region.
@task_item n=11 part=needs
The escher-session help and the describe() self-description: help texts, counts and VERBS.len() swept as literal pins with the words, the help contract and the mcp help tool result kept in step.
@task_item n=12 part=needs
The capabilities doc's counts and their pin: re-measured after the change, the pin matching the swept counts.
@task_item n=13 part=needs
Indirect, through the founder's ratified F3 A: a virtual id or name added to a carrier rides today's exits only; no new carrier, verb, exit status, session error or span field, and a new crossing class is a fork.
@task_item n=14 part=needs
Indirect, through the founder's ratified F11 A: a declaration change marks and signals nothing; the actions stale marker and the graph read take no new signal from it.
@task_item n=15 part=needs
The security plan's virtual-node crossing decision (graph, actions answer, lint, canvas answer and exploration hold none) is the rule each lifted limit meets; every limit lifted is a fork written into the founder-forks evidence with what crosses, each option's measured cost, the dependences, whose it is and the most conservative option that still builds the entry.
@task_item n=16 part=needs
The guards asserting no virtual id (stand graph, stand actions, declared virtual nodes, the graph-actions-and-lint-hold-no-virtual-node check) are flipped by the change; each new control enters the plan with its red reading, on a guard some input can turn red.
@task_item n=17 part=needs
The proving surface: a fixture canvas a test declares, in both layout modes, as the virtual-nodes-as-elements check proved on; the stand declares no virtual node in this chunk.
@task_item n=18 part=needs
Escher-session output is read by an agent, not a person: a virtual node's lines in graph, actions, coverage and layout follow the driver's agent-read line forms, with no colour.
@task_item n=19 part=needs
Evidence hygiene: no raw log with a temp or home path into evidence; a probe worktree lives outside target/ and no plan step deletes it; a guard refusal of a write is a stop.
@task_standing n=1
convention: each new driver test proves on a fixture canvas the test declares, in both layout modes, and asserts the escher-session answer lines in the agent-read form with no colour
home: surface
@task_standing n=2
convention: each new or changed check that pins a count, help text or verb list is swept with the words, and the capabilities doc's Pinned by a check list names it
home: surface
@task_item n=1 part=not_needed
Color palette, typography, spacing, border radius and iconography rows: the chunk paints, sizes and styles nothing and changes no cursor or favicon.
@task_item n=2 part=not_needed
Depth strategy rows on stacking, paint clip and hit-testing order: the chunk reads virtual nodes into driver reads and changes no paint or hit-test order.
@task_item n=3 part=not_needed
Motion rows on the virtual clock, scroll animation and overlay scrollbar timing: no clock, advance or scroll behaviour changes.
@task_item n=4 part=not_needed
Select list rows (its overlay, events, colours, accessibility and screenshot row): the chunk touches no select element.
@task_item n=5 part=not_needed
WPT runner output and result colour rows: the WPT runner is not reached.
@task_item n=6 part=not_needed
Shell provider, JS window, MountedData-during-render and dioxus-native configuration rows: no window, script runtime or config surface changes.
@task_item n=7 part=not_needed
The escher-session tasks rows (counter, crud, timer, cells and the others): the stand declares no virtual node until Circle Drawer's circles, so no task's answer changes.
@task_item n=8 part=not_needed
Circle Drawer canvas colour rows and the web-spa canvas surface: Circle Drawer's circles as virtual nodes and the WASM pages are another entry's.
@task_item n=9 part=not_needed
Mobile target, buffered render replay and logical viewport rows: no build target, render path or viewport size changes.
