@task_prose part=chunk
Builds a keyboard path into Circle Drawer's canvas: the canvas becomes a Tab stop, keys move a current circle among its virtual nodes, and the current circle is selected and its diameter dialog reached by keyboard as the double click reaches it.
Makes engine focus, the accessibility tree's focus and the snapshot's focused name one consistent target for the current circle; how a virtual node is named as focus while the engine holds the canvas is settled at P3/P4.
Adds the founder's B1 selection: a virtual node declaration gains a selected state, written on the circle's accessibility node and read as the snapshot's selected, so a click inside a circle selects it and adds none, and its diff names that node; a click outside every circle still adds one.
Enters Circle Drawer's Tab order into the stand's shared focus-order table and the stand keyboard walk, and reads v040-11's remaining stand readings on a circle: refused covered, off-screen and disabled where the stand can make one so.
Fixes two windows build warnings in two test files, and re-reads the stand checks and the stated sizes and counts the selection's repaint moves, sweeping literal pins.
@task_file n=1 path=examples/seven_guis/src/tasks/circle_drawer.rs mark=edits
what: the seven_guis Circle Drawer task and its drawing canvas — edits: the canvas's focusability, its key handling, the selection, and the canvas's virtual-node declarations gaining selected
@task_file n=2 path=packages/blitz-dom/src/node/custom_widget.rs mark=edits
what: the custom widget's virtual node declaration, an escher addition behind the custom-widget feature — edits: the virtual node declaration gains selected
@task_file n=3 path=packages/dioxus-native-dom/src/virtual_node.rs mark=edits
what: a canvas's virtual node as an accessibility tree node — edits: its selected state, and its focus
@task_file n=4 path=packages/escher-driver/src/execute.rs mark=edits
what: the driver's command execution — edits: press on a focused canvas, and the covered refusal reading
@task_file n=5 path=packages/blitz-dom/src/accessibility.rs mark=edits
what: the engine's accessibility tree build, where the tree's focus is written from the engine's focused node, else the window — edits: the tree's focus naming a virtual node, if P4 settles it there
@task_file n=6 path=tests/blitz-tests/tests/common/mod.rs mark=edits
what: the stand's shared reader and focus-order tables — edits: the shared Tab order table gains Circle Drawer's order
@task_file n=7 path=tests/blitz-tests/tests/stand_keyboard.rs mark=edits
what: the stand's keyboard walk over the stated Tab orders — edits: reads Circle Drawer's order beside the others and the keyboard reach of a circle
@task_file n=8 path=tests/blitz-tests/tests/stand_circle_nodes.rs mark=edits
what: the stand check of Circle Drawer's circles as virtual nodes — edits: a click on a circle selects it with a diff naming it; covered, off-screen and disabled refusals on a circle
@task_file n=9 path=tests/blitz-tests/tests/stand_accessibility_ids.rs mark=re-reads
what: the stand's accessibility id check, its Tab order unchanged
@task_file n=10 path=tests/blitz-tests/tests/stand_circle_drawer_canvas.rs mark=re-reads
what: the stand check of Circle Drawer's canvas and its committed pins
@task_file n=11 path=tests/blitz-tests/tests/stand_canvas_commands.rs mark=re-reads
what: the stand check of the canvas's draw commands and their literal pins
@task_file n=12 path=tests/blitz-tests/tests/stand_canvas_cross_os.rs mark=re-reads
what: the stand's cross-OS canvas pixel reading
@task_file n=13 path=tests/blitz-tests/tests/stand_graph.rs mark=re-reads
what: the stand's UI graph check
@task_file n=14 path=tests/blitz-tests/tests/stand_handler_coverage.rs mark=re-reads
what: the stand's handler coverage check, its graph drives whose double click lands on the circle a first drive added
@task_file n=15 path=tests/blitz-tests/tests/stand_act_error_text.rs mark=edits
what: a stand test of a driver act's error text — edits: the windows build warning at its import line
@task_file n=16 path=examples/seven_guis/tests/host_binary.rs mark=edits
what: the seven_guis host binary test — edits: the windows build warning at its import line
@task_file n=17 path=docs/capabilities.md mark=edits
what: the capability statement, edited by hand — edits: its stated sizes and counts the selection moves
@task_item n=1 part=needs
A keyboard route to a canvas's virtual nodes, today stated absent (Circle Drawer's circles reached by click, tree and snapshot alone): the chunk builds it, so that absence is lifted or rewritten.
@task_item n=2 part=needs
What is focusable and how focusability is recomputed when tabindex is written: the canvas becomes a Tab stop by the chunk's own markup.
@task_item n=3 part=needs
Tab and Shift+Tab move focus with focus events, and the stated Tab order is walked: the canvas joins that walk in Circle Drawer.
@task_item n=4 part=needs
Keyboard activation by Enter and Space as a click, and its handler order: the current circle is acted on by keyboard, and its diameter dialog reached as the double click reaches it.
@task_item n=5 part=needs
The double click threshold and double click read on the document's clock: the diameter dialog's keyboard route must reach what the double click reaches today, and the graph drive's double click re-read under B1.
@task_item n=6 part=needs
The accessibility tree's focus, written from the engine's focused node or else the window, and the snapshot's focused: both must name one consistent target for the current circle while the engine holds the canvas.
@task_item n=7 part=needs
aria-activedescendant kept unread is a stated limit that stays unless P4 forks on it; the chunk's focus reading must not rely on the engine reading it.
@task_item n=8 part=needs
The snapshot's selected state and the accessibility tree's selected state (the aria-selected reading): a circle's virtual node now carries selected, read through the snapshot's existing key, not a new one.
@task_item n=9 part=needs
The snapshot diff's differs and its entries: a driver click on a circle's node settles with a diff naming that node and differs selected.
@task_item n=10 part=needs
The snapshot text form's state tokens and the snapshot node's fields: selected on a virtual node is read through the existing token and field.
@task_item n=11 part=needs
Driver click refusals covered, off-screen and disabled, read on a circle where the stand can make one so; covered under B1 reads the top circle unless a later-declared sibling reads as covering it, an open question that rides the click clause.
@task_item n=12 part=needs
The driver's press on a focused canvas: key input reaches the canvas's key handler and moves the current circle.
@task_item n=13 part=needs
Circle Drawer's drawing area as one named canvas node, its circles as virtual nodes: a circle's node keeps its id and name, gaining selected and focus.
@task_item n=14 part=needs
The UI graph mirrors the snapshot, one graph node per snapshot node, and carries no values: a circle's selection or focus changes no graph shape beyond what the snapshot carries.
@task_item n=15 part=needs
Actionable nodes and the actions answer: a circle's node, now focusable through its canvas and selectable, is listed as actionable by the one actionable rule.
@task_item n=16 part=needs
Handler coverage and the Dioxus document's reached-handler record: the canvas's new key handler is reached and recorded, and the graph drive's first click now selects.
@task_item n=17 part=needs
The stand's shared Tab order table: Circle Drawer's sequence enters it, and the stand's keyboard walk reads it beside the others.
@task_item n=18 part=needs
Stand expectation tables stated once: Circle Drawer's expected focus order, roles and names stated in the shared table, never repeated in a test.
@task_item n=19 part=needs
The 7GUIs selection repaints the chosen circle filled, which moves the committed canvas pins and the capability statement's stated sizes and counts, edited by hand; literal pins are swept.
@task_item n=20 part=needs
The selected field is an escher addition behind the custom-widget feature, and the accessibility tree build stays behind the accessibility feature: a virtual node as the tree's focus is an engine change for every document behind the feature, or none.
@task_item n=21 part=needs
Inert or unshown focus is given up by the focus fixup, and a removed focused node drops focus with blur steps: a current circle that is deleted or undone must not leave focus on a node that no longer exists.
@task_item n=22 part=needs
The two windows build warnings are fixed where they stand and witnessed by the CI windows job's own log, read only after its Compiling line is seen.
@task_item n=23 part=needs
A control enters with its red reading; no key twice on a line; no raw log with a temp or home path in evidence.
@task_standing n=1
convention: every stand focus sequence is asserted against the shared Tab order table, and a new app's order joins that table
home: cognitive-accessibility
@task_standing n=2
convention: expected roles, names and orders are stated once in the stand's expectation tables and read by every check
home: screen-reader-support · 7-screen-reader-support
@task_standing n=3
convention: accessibility assertions are written as Rust tests over the AccessKit tree
home: 3-a11y-assertion-harness-contract
@task_standing n=4
convention: every stand check runs in the accessibility test leg of the slow tier and is repeated three rounds
home: ci-integration · 9-ci-integration
@task_item n=1 part=not_needed
A platform accessibility adapter on the stand binaries, active assistive technology reach and the adapter's creation order: the chunk builds no adapter.
@task_item n=2 part=not_needed
Keyboard or assistive-technology drag and drop: no drag route is built.
@task_item n=3 part=not_needed
Single-choice and multiple select contracts, open list rows, optgroups and select colours: no select is touched.
@task_item n=4 part=not_needed
Range input keys, pointerdown and value contracts: no range input is touched.
@task_item n=5 part=not_needed
Text contrast, the lint's contrast finding and its fields, scrollbar thumb contrast and stand accent colour: the lint reading virtual nodes belongs to another entry and no colour pair is re-measured here.
@task_item n=6 part=not_needed
Password value masking, file input value and invalid date state: no such control is touched.
@task_item n=7 part=not_needed
MCP tool results and the session library: Circle Drawer stays served by no session role.
@task_item n=8 part=not_needed
Hidpi, zoom and scheme changes, viewport scale: a rotated or scaled canvas's virtual nodes belong to another entry.
@task_item n=9 part=not_needed
Hidden content exclusion by aria-hidden, display none, hidden and visibility hidden: no hidden-content rule changes.
@task_item n=10 part=not_needed
ARIA name reads for aria-describedby, aria-description, aria-level and aria-expanded: no such attribute is written.
