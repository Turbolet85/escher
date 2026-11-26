@task_prose part=session
Circle Drawer's canvas became one Tab stop with a keyboard path: arrows move a selected circle in declared order, clamped, Enter opens the diameter dialog on it, and a press with the dialog open changes nothing.
A click on a circle now selects the topmost circle whose disc holds the point and adds none; a click off every circle adds one as before; the selected circle is filled with a new widget colour constant SELECTED_FILL (#3a5ce5 at alpha 96/255).
A canvas's virtual node can declare a selected state, read as selected in the accessibility tree, the snapshot and a diff; with the dialog open every circle but the adjusted one is declared disabled.
The engine no longer traps Tab on a focused custom-widget element: the widget hears the keydown, then the engine's Tab and Shift+Tab focus order runs (PROVISIONAL, flagged upstreamable); the engine focuses a custom-widget element on pointer down, so the app's own focus call was written and removed.
The driver's covered refusal now also counts a node its canvas declares later whose bounding region holds the target's region centre, so a later square's corner covers where its disc does not; the meaning text served by help and MCP was restated, its name, remedy and order unchanged.
No new verb, argument, key name, cause, session error, exit status, log field or dependency; the a11y leg's reading moved 20 · 199 · 0 · 0 to 20 · 201 · 0 · 0.
@task_file n=1 path=packages/blitz-dom/src/node/custom_widget.rs
what: edited — the custom widget's virtual node declaration (VirtualNodeDeclaration), which gains a selected state beside its region and disabled state
@task_file n=2 path=packages/blitz-dom/src/events/mod.rs
what: edited — the engine's DOM event dispatch, whose custom-widget arm now hands a Tab keydown on to the engine's focus order instead of trapping it
@task_file n=3 path=packages/blitz-dom/src/events/pointer.rs
what: edited — the engine's pointer handling and click synthesis for custom widgets, now holding the Tab pass-through for a focused widget element (moved here after implement)
@task_file n=4 path=packages/dioxus-native-dom/src/dioxus_document.rs
what: edited — the accessibility tree built with a canvas's virtual nodes, now writing a declared selected state on each node
@task_file n=5 path=packages/dioxus-native-dom/src/snapshot.rs
what: edited — the snapshot's reading of a virtual node, now reading selected from the tree as an element does
@task_file n=6 path=packages/dioxus-native-dom/src/virtual_node.rs
what: edited — the virtual nodes read from a canvas widget, and the unit builder of their declarations
@task_file n=7 path=packages/escher-driver/src/execute.rs
what: edited — the driver verbs' run path and target checks, where the covered refusal now reads later-declared nodes of a canvas
@task_file n=8 path=packages/escher-driver/src/actions.rs
what: edited — the actions answer, decided by the verbs' own checks, now carrying a virtual node's identity into the covered check
@task_file n=9 path=packages/escher-driver/src/refusal.rs
what: edited — the driver's refusal causes and their fixed meanings served by help and the MCP descriptions; the covered meaning restated
@task_file n=10 path=examples/seven_guis/src/tasks/circle_drawer.rs
what: edited — the Circle Drawer task: its canvas widget, its circles as virtual nodes, their colours, its click and keyboard handlers, its selected graph state and its diameter dialog
@task_file n=11 path=examples/seven_guis/tests/host_binary.rs
what: edited — the seven_guis host binary's check
@task_file n=12 path=tests/blitz-tests/tests/common/mod.rs
what: edited — the stand's shared helpers and per-task tables: the graph drives, the handlers left unexercised and the stated Tab orders
@task_file n=13 path=tests/blitz-tests/tests/fixtures/mod.rs
what: edited — the stand's shared fixture pages, among them a canvas with declared virtual nodes
@task_file n=14 path=tests/blitz-tests/tests/virtual_nodes_declared.rs
what: edited — the check that a canvas's declared virtual nodes are read as declared
@task_file n=15 path=tests/blitz-tests/tests/virtual_nodes_as_elements.rs
what: edited — the check that virtual nodes read as elements in the tree, the snapshot and a diff; now the selected state
@task_file n=16 path=tests/blitz-tests/tests/stand_circle_nodes.rs
what: edited — the stand check of Circle Drawer's circles as nodes: click selects, covered by a later node, disabled under the open dialog, keys, focus events
@task_file n=17 path=tests/blitz-tests/tests/stand_keyboard.rs
what: edited — the keyboard stand check walking each task's stated Tab order forward and back
@task_file n=18 path=tests/blitz-tests/tests/stand_canvas_click.rs
what: edited — the stand check of clicks on a widget canvas, now with the engine check that Tab and Shift+Tab leave a focused widget
@task_file n=19 path=tests/blitz-tests/tests/stand_circle_drawer_canvas.rs
what: edited — the stand check of Circle Drawer's canvas drawing, its dialog and its slider resizing a circle
@task_file n=20 path=tests/blitz-tests/tests/stand_act_error_text.rs
what: edited — the stand check of the act error text the driver prints
@task_file n=21 path=docs/capabilities.md
what: edited — the capability document: accessibility reach, the a11y leg's reading, and the stated limits of the canvas sample and of click landing
@task_item n=1 part=touches
decision/circle-drawer-canvas-colours-in-code: a new canvas colour constant SELECTED_FILL (#3a5ce5 at alpha 96/255) fills the selected circle, kept in code beside the canvas's other colours.
@task_item n=2 part=touches
contract/driver-verbs-and-virtual-clock/click: a click on a Circle Drawer circle node now selects it and adds none, its diff naming the node with differs holding selected; the click also focuses the canvas within its own settled step (engine focus on pointer down).
@task_item n=3 part=touches
contract/driver-verbs-and-virtual-clock/press: press now acts on Circle Drawer's focused canvas — arrows move the selection, Enter opens the dialog; Tab and Shift+Tab leave a focused custom-widget element.
@task_item n=4 part=touches
contract/driver-verbs-and-virtual-clock/canvas: Circle Drawer's canvas gains the data-selected and data-adjusting attributes beside the unchanged data-circles form; a data-circles write sets no app state.
@task_item n=5 part=touches
decision/actions-share-the-verbs-checks: the covered check now reads a canvas's later-declared nodes, and the actions answer and the run path share that one check.
@task_item n=6 part=touches
contract/actions-line/nodes: a virtual node under a later-declared node of its canvas now lists as refused covered in the actions answer.
@task_item n=7 part=touches
contract/escher-session-commands/help and contract/escher-session-mcp/help-result: the covered cause's fixed meaning text they serve was restated (the later-node clause); name, remedy and order unchanged.
@task_item n=8 part=touches
contract/escher-session-commands/snapshot and contract/escher-session-mcp/snapshot-result: a virtual node now carries a selected or unselected reading in the snapshot when its declaration states one.
@task_item n=9 part=touches
contract/escher-session-commands/graph-answer and contract/ui-graph-line/content: Circle Drawer's graph gains the state selected and a keydown handler on circle-canvas; dialog_idx and dialog_r add circle-canvas keydown to their writes.
@task_item n=10 part=touches
contract/escher-session-commands/coverage-answer and contract/coverage-line/content: Circle Drawer's drives grew 9 to 13 and its handlers left unexercised fell 4 to 2, where a row holds those counts.
@task_item n=11 part=touches
decision/canvas-line-form-conventions and decision/canvas-line-node-tag: indirect — the canvas's virtual node declarations they rule over gain a selected state and a selected fill in the painted commands.
@task_item n=12 part=touches
depth-strategy rows on hit-testing and paint order (decision/hit-test-stops-at-paint-clip, decision/hit-testing-stops-at-paint-clip, decision/stacking-context-groups-paint-and-hit): indirect — within one canvas a later-declared node now counts as over an earlier one for covered, by bounding region, not painted shape.
@task_item n=13 part=touches
decision/escher-session-agent-read-surface and decision/driver-output-for-agents: indirect — the agent-read text of the covered meaning changed.
@task_fact n=1 slug=surface
fact: Circle Drawer's canvas is one Tab stop; arrows move the selected circle in declared order, clamped, the first arrow with none selected selecting the first; Enter opens the diameter dialog on the selected circle; a press while the dialog is open changes nothing (PROVISIONAL).
@task_fact n=2 slug=surface
fact: A click on Circle Drawer's canvas selects the topmost circle whose disc holds the point and adds none; only a click on no circle adds a circle.
@task_fact n=3 slug=surface
fact: A focused custom-widget element never traps Tab: the widget hears the keydown, then the engine's Tab and Shift+Tab focus order runs, so a widget cannot keep Tab for itself (PROVISIONAL, upstreamable).
@task_fact n=4 slug=surface
fact: The engine focuses a custom-widget element on pointer down, so a click on a canvas focuses it with no app focus call.
@task_fact n=5 slug=surface
fact: A canvas's virtual node may declare a selected state; Some(true) reads selected, Some(false) unselected and None no word, alike in the accessibility tree, the snapshot and a diff, and an assistive technology learns Circle Drawer's current circle from selected alone.
@task_fact n=6 slug=surface
fact: While Circle Drawer's diameter dialog is open every circle but the adjusted one is declared disabled and refused disabled by the driver.
@task_fact n=7 slug=depth-strategy
fact: A canvas's virtual node reads covered when a node its canvas declares later has a region holding the target's region centre; a region is the declared bounding box, so a later shape's corner covers where its painted shape does not.
@task_item n=1 part=untouched
Typography, spacing, border-radius and motion rows: no font, unit, margin, radius, animation, scroll or virtual clock behaviour moved.
@task_item n=2 part=untouched
Color-palette rows on engine colours, select list colours, the color-scheme recascade, base_color and colour tokens: only a Circle Drawer canvas constant was added.
@task_item n=3 part=untouched
escher-session exit statuses and exit codes, stderr usage and session errors, the session error remedy and the mcp Err ending: no new cause, error or exit.
@task_item n=4 part=untouched
escher-session tasks rows for cells, counter, crud, flight-booker, timer, load, order and unknown task: no task was added, renamed or reordered.
@task_item n=5 part=untouched
Single-select rows (select list events, select accessibility, select list overlay, its paint and hit order): the select element was not changed.
@task_item n=6 part=untouched
Shell provider, cursor, favicon and iconography rows, js-window and dioxus-native configuration rows, font context rows: not reached.
@task_item n=7 part=untouched
WPT runner output and result colour rows: not reached.
@task_item n=8 part=untouched
contract/mounteddata-during-render/set-focus-to and document-state: the MountedData calls are unchanged; the app's focus call was written and removed.
@task_item n=9 part=untouched
Brand identity, anti-patterns, self-validation and design decisions log areas, and the web-spa surface: nothing recorded there is reached.
