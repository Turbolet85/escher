@task_prose part=chunk
Circle Drawer's canvas becomes a Tab stop and keys move a current circle among its virtual nodes; the current circle is selected by keyboard and its diameter dialog is reachable by keyboard as the double click reaches it.
A click inside a circle selects it and adds none (the founder's B1): the virtual node declaration gains a selected field, written on the circle's accessibility node and read as the snapshot's selected, so the click's diff names that node; a click outside every circle still adds one.
Engine focus, the accessibility tree's focus and the snapshot's focused agree on the current circle; Circle Drawer's Tab order joins the stand's shared focus tables and its keyboard walk.
The stand reads v040-11's remaining readings on Circle Drawer (a circle refused covered, off-screen and disabled where it can make one so); two windows build warnings are fixed; literal pins (canvas pins, stated sizes and counts) are swept and re-measured.
@task_file n=1 path=examples/seven_guis/src/tasks/circle_drawer.rs mark=edits
what: the Circle Drawer task, its canvas widget and its circles' virtual nodes — edits: the canvas's focusability, its key handling, the click selection, CircleCanvas::virtual_nodes declaring selected and any focus reading
@task_file n=2 path=packages/blitz-dom/src/node/custom_widget.rs mark=edits
what: the custom widget boundary behind the custom-widget feature, holding the virtual node declaration — edits: VirtualNodeDeclaration gains selected
@task_file n=3 path=packages/dioxus-native-dom/src/virtual_node.rs mark=edits
what: a virtual node's accessibility tree node and its snapshot reading — edits: the node's selected and focus
@task_file n=4 path=packages/escher-driver/src/execute.rs mark=edits
what: the driver verbs' execution — edits: press on a focused canvas, the covered reading
@task_file n=5 path=packages/blitz-dom/src/accessibility.rs mark=edits
what: the engine's accessibility tree build — edits: where the tree's focus is written, when focus is a virtual node
@task_file n=6 path=tests/blitz-tests/tests/common/mod.rs mark=edits
what: the stand's shared readers and focus sequences — edits: the focused reader, tab_order and the shared Tab order tables, which gain Circle Drawer's
@task_file n=7 path=tests/blitz-tests/tests/stand_circle_nodes.rs mark=edits
what: the stand check of Circle Drawer's circles as virtual nodes — edits: the click on a circle selecting it, its refused covered, off-screen and disabled readings
@task_file n=8 path=tests/blitz-tests/tests/stand_keyboard.rs mark=edits
what: the stand's keyboard walk — edits: Circle Drawer's walk read beside the others
@task_file n=9 path=tests/blitz-tests/tests/stand_accessibility_ids.rs mark=edits
what: the stand check of accessibility ids and Tab order — edits: tab_order_is_unchanged
@task_file n=10 path=tests/blitz-tests/tests/stand_circle_drawer_canvas.rs mark=edits
what: the stand check of Circle Drawer's canvas and its committed pins — edits: literal pins swept
@task_file n=11 path=tests/blitz-tests/tests/stand_canvas_commands.rs mark=edits
what: the stand check of a canvas's draw commands — edits: literal pins swept
@task_file n=12 path=tests/blitz-tests/tests/stand_canvas_cross_os.rs mark=edits
what: the stand check of canvas output across operating systems — edits: literal pins swept
@task_file n=13 path=tests/blitz-tests/tests/stand_graph.rs mark=edits
what: the stand check of the UI graph — edits: literal pins swept
@task_file n=14 path=tests/blitz-tests/tests/stand_handler_coverage.rs mark=re-reads
what: the stand check of handler coverage, GRAPH_DRIVES whose double click lands on the circle its first drive added
@task_file n=15 path=tests/blitz-tests/tests/stand_act_error_text.rs mark=edits
what: the stand check of act error text — edits: the windows build warning at its line 27
@task_file n=16 path=examples/seven_guis/tests/host_binary.rs mark=edits
what: the seven_guis host binary test — edits: the windows build warning at its line 10
@task_file n=17 path=docs/capabilities.md mark=edits
what: the capabilities document — edits: its stated sizes and counts
@task_item n=1 part=needs
The canvas widget's keyboard path: the canvas takes focus in the Tab walk and keys move a current circle among its virtual nodes; Enter or a key acts on the current circle (selects it, reaches its diameter dialog).
@task_item n=2 part=needs
The click selection (the founder's B1): a click inside a circle selects it and adds none; a click outside every circle adds one; the driver click on a circle's node settles with a diff naming that node, differs selected.
@task_item n=3 part=needs
The virtual node declaration's new selected field, an escher addition behind the custom-widget feature, written on the circle's accessibility node and read as the snapshot's existing selected; every existing declaration literal takes the new field.
@task_item n=4 part=needs
The selected circle repaints filled, so Circle Drawer's canvas colours (kept as widget constants) gain or use a selected fill, and the committed canvas pins move.
@task_item n=5 part=needs
Focus agreement: the engine's focus on the canvas element, the accessibility tree's focus and the snapshot's focused name one consistent target; how a virtual node is named as focus while the engine holds the canvas is open at P3/P4, with the engine reading no aria-activedescendant as its stated limit.
@task_item n=6 part=needs
The accessibility tree's focus is written only where the focused node is among the built nodes, else the window; a virtual node as the tree's focus is an engine change for every document behind the feature, or none.
@task_item n=7 part=needs
The driver's press verb on a focused canvas, settling under the virtual clock, and the click verb on a virtual node.
@task_item n=8 part=needs
The driver's refusals on a circle — covered (a later-declared sibling read as covering a circle, F4 B, an open question), off-screen and disabled — read on the stand where it can make one so.
@task_item n=9 part=needs
The actions line, the UI graph line and actions decided by the verbs' own checks, as they read a selectable, focusable circle.
@task_item n=10 part=needs
Circle Drawer's Tab order enters the stand's shared focus tables and the stand's keyboard walk reads it beside the others; the unchanged-Tab-order check is re-read for the new stop.
@task_item n=11 part=needs
The handler coverage drive whose double click lands on the circle its first drive added is re-read: under B1 that first click selects.
@task_item n=12 part=needs
Literal pins swept: canvas pins, the stand's counts and the capabilities document's stated sizes and counts re-measured, none left stale.
@task_item n=13 part=needs
Two windows build warnings fixed where they stand and witnessed by the CI windows job's own log, read only after its Compiling line is seen.
@task_item n=14 part=needs
Circle Drawer stays served by no escher-session role; no escher-session task is added for it.
@task_item n=15 part=needs
Evidence rules: no raw log with a temp or home path enters evidence; no key twice on a line; a control enters with its red reading.
@task_standing n=1
convention: each new stand check reads the engine through the driver verbs and their settle under the virtual clock, never a real-time wait
home: motion-calibrated-to-expression-level-unrecorded
@task_standing n=2
convention: each new test or check output line is read by an agent, plain and colourless
home: surface
@task_item n=1 part=not_needed
Typography rows (fonts, type scale, text locale, font fallback): the chunk changes no text or font.
@task_item n=2 part=not_needed
Spacing rows (body margins, table cell spacing, select list placement): no layout spacing changes.
@task_item n=3 part=not_needed
Depth strategy rows on stacking, paint levels and hit-testing at paint clips: the canvas's circles are hit by their own declared geometry, not by a new paint order.
@task_item n=4 part=not_needed
Iconography rows (cursor icons, cursor resolution, favicon): no cursor or icon changes.
@task_item n=5 part=not_needed
Select element rows (select list overlay, its events, accessibility, colours and gestures): the chunk touches no select.
@task_item n=6 part=not_needed
WPT runner output and result colour rows: no WPT runner change.
@task_item n=7 part=not_needed
Shell provider window rows (maximize, minimize, decorations, drag) and the JS window object's sizes: no window chrome change.
@task_item n=8 part=not_needed
Web-spa surface rows: the WASM pages are not reached.
@task_item n=9 part=not_needed
Motion rows on scrollbars, smooth scroll and into-view scroll: the canvas keyboard path scrolls nothing.
@task_item n=10 part=not_needed
Engine colour scheme rows (color-scheme recascade, base colour, engine colour tokens): only Circle Drawer's own canvas colours move.
