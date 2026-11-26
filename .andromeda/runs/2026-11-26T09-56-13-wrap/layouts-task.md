@task_prose part=session
The session gave Circle Drawer's canvas a keyboard path. The canvas is now one Tab stop. Arrow keys move a selected circle in declared order and stop at the ends, and Enter opens the diameter dialog on the selected circle.
A click on a circle now selects the topmost circle under the point and adds none. A click elsewhere adds a circle as before. The canvas carries two new attributes, data-selected and data-adjusting, beside the unchanged data-circles.
Virtual node declarations gained an optional selected state. The accessibility tree and the snapshot read it, and a diff that flips it reads differs selected.
The driver's covered refusal now also refuses a canvas node when a node its canvas declares later has a region holding the target's region centre (PROVISIONAL). The meaning served by help and MCP descriptions was restated. The cause's name, remedy and order are unchanged.
The engine no longer traps Tab on a focused custom-widget element: the widget hears the key, then focus moves on (PROVISIONAL, flagged upstreamable). An app focus call on canvas click was measured inert and removed, because the engine focuses a widget element on pointer down.
The canvas keeps 700 x 400 (flex-shrink 0), so Close lies below the 600 px stand view and is refused off-screen until scrolled. Two stand readings were retired and replaced: a click on a circle adds one, and another circle never covers one.
@task_file n=1 path=packages/blitz-dom/src/node/custom_widget.rs
what: the custom-widget boundary — the virtual node declaration a canvas widget hands the engine, now with an optional selected state
@task_file n=2 path=packages/blitz-dom/src/events/mod.rs
what: the engine's event dispatch — the widget arm that hands keys and pointer events to a custom widget, now calling the Tab pass-through
@task_file n=3 path=packages/blitz-dom/src/events/pointer.rs
what: the engine's pointer and focus handling for widget elements — the click synthesis, and now the Tab pass-through that moves focus past a focused widget
@task_file n=4 path=packages/dioxus-native-dom/src/dioxus_document.rs
what: the document's accessibility tree builder with virtual nodes — writes each declared node's selected state into the tree
@task_file n=5 path=packages/dioxus-native-dom/src/snapshot.rs
what: the snapshot read — a virtual node's states (now selected) as the element branch reads them
@task_file n=6 path=packages/dioxus-native-dom/src/virtual_node.rs
what: the virtual node read and its unit builder — declarations a canvas declares, read in declaration order
@task_file n=7 path=packages/escher-driver/src/execute.rs
what: the driver's act path and target check — the covered reading, now also by a later-declared node of the same canvas
@task_file n=8 path=packages/escher-driver/src/actions.rs
what: the driver's actions answer — lists each target's refusal by the same check the run path uses
@task_file n=9 path=packages/escher-driver/src/refusal.rs
what: the refusal causes and their fixed meanings served by help and MCP descriptions — covered restated
@task_file n=10 path=examples/seven_guis/src/tasks/circle_drawer.rs
what: the Circle Drawer stand task — circles painted on the circle-canvas, its click, keys, selection, dialog and canvas attributes
@task_file n=11 path=examples/seven_guis/tests/host_binary.rs
what: the stand host binary check — the served stand booted as a session host
@task_file n=12 path=tests/blitz-tests/tests/common/mod.rs
what: the stand harness's shared helpers — the stated Tab orders and the graph drives for each task
@task_file n=13 path=tests/blitz-tests/tests/fixtures/mod.rs
what: the shared test fixtures — virtual node declarations used by the checks
@task_file n=14 path=tests/blitz-tests/tests/virtual_nodes_declared.rs
what: the check of virtual nodes as declared by a widget
@task_file n=15 path=tests/blitz-tests/tests/virtual_nodes_as_elements.rs
what: the check that virtual nodes read as elements in the tree, the snapshot and a diff — now the selected state
@task_file n=16 path=tests/blitz-tests/tests/stand_circle_nodes.rs
what: the stand check of Circle Drawer's circles as nodes — select by click, covered by a later node, disabled while the dialog is open, keys, focus events
@task_file n=17 path=tests/blitz-tests/tests/stand_keyboard.rs
what: the stand keyboard check — each task's stated Tab order walked forward and back, now Circle Drawer's
@task_file n=18 path=tests/blitz-tests/tests/stand_canvas_click.rs
what: the engine-level canvas click check on a fixture canvas — now Tab and Shift+Tab leaving a focused widget
@task_file n=19 path=tests/blitz-tests/tests/stand_circle_drawer_canvas.rs
what: the stand check of Circle Drawer's canvas — centre clicks open the dialog, the slider resizes, Close after a scroll
@task_file n=20 path=tests/blitz-tests/tests/stand_act_error_text.rs
what: the stand check of act error texts — the refusal meanings as an agent reads them
@task_file n=21 path=docs/capabilities.md
what: the capabilities document — what the driver and stand can and cannot do, the accessibility reach and its a11y leg reading
@task_item n=1 part=touches
decision/circle-drawer-circles-painted-on-canvas: the canvas now carries data-selected and data-adjusting beside the unchanged data-circles form x,y,d,id. A click on a circle now selects it instead of adding one. The canvas keeps 700 x 400 (flex-shrink 0). The selected circle is filled with its own colour.
@task_item n=2 part=touches
contract/stand-element-ids/circle-canvas: the element now takes tabindex 0 and a keydown handler. It is one Tab stop, and it carries the data-selected and data-adjusting attributes.
@task_item n=3 part=touches
contract/stand-element-ids/circle-close: the id is unchanged. With the canvas at full size, Close now lies at y 607 to 639, below the 600 px stand view, and a driver click on it is refused off-screen until a scroll. Removing the dialog drops focus to the root.
@task_item n=4 part=touches
decision/headless-stand-pinned-viewport: the pinned 600 px view now puts Circle Drawer's Close below the fold. The viewport bound itself is unchanged.
@task_item n=5 part=touches
decision/scroll-into-view-moves-every-scroller: checks now reach Close through scroll circle-close before clicking it.
@task_item n=6 part=touches
decision/crud-list-keyboard-region: Circle Drawer's canvas now follows the same one-keyboard-region pattern (one Tab stop, with arrows inside it). Nothing about the CRUD list itself changed.
@task_item n=7 part=touches
decision/stand-a11y-by-attributes-alone: Circle Drawer's circles now state selected and disabled through their virtual node declarations, not through element attributes. This is a possible exception to names, roles and states by attributes alone.
@task_item n=8 part=touches
decision/stand-ids-as-attributes-alone and decision/every-stand-control-keyed: the canvas's circles stay virtual nodes under circle-canvas. No new stand element id was added.
@task_item n=9 part=touches
contract/escher-session-help/answer: the covered meaning served by help is restated. A canvas node also reads covered when a node its canvas declares later has a region (bounding box) holding the target's region centre. PROVISIONAL.
@task_item n=10 part=touches
contract/escher-session-mcp/tools: the MCP tool descriptions carry the restated covered meaning. No tool, verb or argument was added.
@task_item n=11 part=touches
contract/escher-session-actions/answer: the actions answer now lists a circle under a later-declared one as refused covered, by the same check as the run path. With the dialog open, it lists every other circle as disabled.
@task_item n=12 part=touches
contract/escher-session-snapshot/answer: a virtual node now reads selected or unselected from its declaration, and a flip reads as differs selected in a diff.
@task_item n=13 part=touches
contract/escher-session-graph/answer: Circle Drawer gains the graph state selected, written by circle-canvas click and keydown. Its dialog states also add circle-canvas keydown to their declared writes.
@task_item n=14 part=touches
contract/escher-session-refusals/off-screen: Circle Drawer's Close is now a standing case of an off-screen refusal on the stand.
@task_item n=15 part=touches
decision/visible-region-through-clipping-boxes: the covered reading for canvas nodes now also uses declared regions of later nodes, not only an element hit. A later square's corner covers where its painted disc does not.
@task_fact n=1 slug=surface-desktop-native
fact: Circle Drawer's canvas is one Tab stop. Arrows move the selected circle in declared order and stop at the ends; the first arrow with none selected selects the first circle. Enter opens the dialog on the selected circle, and a key pressed while the dialog is open changes nothing.
@task_fact n=2 slug=surface-desktop-native
fact: A click on Circle Drawer's canvas selects the topmost circle whose disc holds the point and adds none. Only a click on no circle adds a circle.
@task_fact n=3 slug=surface-desktop-native
fact: While Circle Drawer's diameter dialog is open, every circle but the one being adjusted declares disabled and is refused disabled.
@task_fact n=4 slug=surface-desktop-native
fact: Circle Drawer's canvas keeps 700 x 400 (flex-shrink 0), so on the 600 px stand view Close lies below the fold and must be scrolled into view before a click.
@task_fact n=5 slug=surface-desktop-native
fact: After Circle Drawer's dialog closes, focus drops to the root, and a click on the canvas is needed before its keys act again. This is an open limit, carried on A11y re-read.
@task_fact n=6 slug=surface-desktop-native
fact: An assistive technology learns Circle Drawer's current circle from the selected state alone. This is a recorded limit beside the CRUD list's.
@task_fact n=7 slug=new
fact: A focused custom-widget element hears a Tab keydown, and then the engine moves focus forward, or backward with Shift. A widget cannot keep Tab for itself. PROVISIONAL, flagged upstreamable.
@task_fact n=8 slug=new
fact: The engine focuses a custom-widget element on pointer down, so an app needs no focus call of its own on a canvas click.
@task_fact n=9 slug=surface-cli
fact: A canvas's virtual node is refused covered by an element over its region's centre, or by a node its canvas declares later whose declared bounding box holds that centre. PROVISIONAL.
@task_fact n=10 slug=surface-cli
fact: A virtual node's declared selected state (stated true, stated false, or not stated) reads as selected, unselected or no word in the tree, the snapshot and its text.
@task_item n=1 part=untouched
The ShellProvider window chrome controls are unchanged. The session touched no window decorations, maximize, minimize, close or drag.
@task_item n=2 part=untouched
The stable ids of the other stand tasks are unchanged: Cells, Counter, CRUD, Flight Booker, Temperature Converter, Timer, Load, Order and the task cards. So are the circle-undo, circle-redo and circle-diameter ids.
@task_item n=3 part=untouched
The agent-run, cold-agent, multi_doc and paint_bench command lines are unchanged: no argument, exit, stdout or Windows pass-through moved.
@task_item n=4 part=untouched
The escher-session command form, exit codes, coverage role, recording, layout, lint, screenshot, host log service name, and the MCP protocol revisions, stdio, end, result bound and canvas tool are unchanged. No verb, argument, key name, cause, session error, exit, carrier or log field was added.
@task_item n=5 part=untouched
The fonts-unavailable and unknown-verb refusals are unchanged, and so is the list of served tasks.
@task_item n=6 part=untouched
The cold-agent decisions (no id in the prompt, the pipe builds nothing, drives through MCP), MCP local over stdio, caller input never echoed, and the replayable session recording are unchanged.
@task_item n=7 part=untouched
The document skeleton, the main mount, held-instance fresh boot, session roles sharing the stand boot, the load page network provider, open list row bounds, the order page, the flight type select, stand lints clean at boot and the stand accent colour are unchanged.
@task_item n=8 part=untouched
The mobile-native and web-spa surfaces are unchanged, and so is the absence of arbitrary index.html templates.
