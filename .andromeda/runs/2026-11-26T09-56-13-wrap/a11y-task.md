@task_prose part=session
The session gave Circle Drawer's canvas a keyboard path: the canvas is one Tab stop, the arrows move a selected circle in declared order (clamped), and Enter opens the adjust dialog on that circle. A key pressed while the dialog is open changes nothing.
A click on a circle now selects the topmost circle under the point and adds none; a click off every circle still adds one. A canvas's virtual node declaration gains a selected state, which the accessibility tree and the snapshot read as selected or unselected.
The engine no longer traps Tab on a focused custom-widget element: the widget hears the key first, then Tab and Shift+Tab move focus. This is PROVISIONAL on the operator's word and flagged upstreamable. Measured: the engine focuses a custom-widget element on pointer down, so the app's own focus call was dropped.
The driver's covered refusal now also covers a canvas node when a node its canvas declares later has a region holding the target's region centre. Measured: with the dialog open every other circle reads disabled, and the canvas keeps 700 by 400.
Circle Drawer's Tab order (back button, then canvas) joins the walked Tab order tables. The a11y leg reading moved from 20 · 199 · 0 · 0 to 20 · 201 · 0 · 0. Two stand readings were retired: a click on a circle's node adds a circle, and another circle never covers one.
@task_file n=1 path=packages/blitz-dom/src/node/custom_widget.rs
what: edited — the custom widget's virtual node declaration, which gains a selected state (none, selected or unselected)
@task_file n=2 path=packages/blitz-dom/src/events/mod.rs
what: edited — the engine's event dispatch and its widget arm, which now passes Tab and Shift+Tab on to focus movement after a focused widget hears the key
@task_file n=3 path=packages/blitz-dom/src/events/pointer.rs
what: edited — the engine's pointer and click synthesis for widgets, which now holds the Tab pass-through for a focused custom-widget element
@task_file n=4 path=packages/dioxus-native-dom/src/dioxus_document.rs
what: edited — the Dioxus document's accessibility tree build with virtual nodes, which now writes a virtual node's declared selected state
@task_file n=5 path=packages/dioxus-native-dom/src/snapshot.rs
what: edited — the snapshot, whose virtual node branch now reads selected from the tree node just as the element branch does
@task_file n=6 path=packages/dioxus-native-dom/src/virtual_node.rs
what: edited — the reading of a canvas's virtual nodes, whose unit declaration builder now takes the selected state
@task_file n=7 path=packages/escher-driver/src/execute.rs
what: edited — the driver's click execution and its refusal checks, where covered now also counts a node its canvas declares later
@task_file n=8 path=packages/escher-driver/src/actions.rs
what: edited — the actions verb's listing of actionable nodes and their causes, which shares one target check with the run path
@task_file n=9 path=packages/escher-driver/src/refusal.rs
what: edited — the driver refusal causes and their fixed meanings, where the meaning of covered is restated
@task_file n=10 path=examples/seven_guis/src/tasks/circle_drawer.rs
what: edited — seven_guis Circle Drawer: its canvas with circles as virtual nodes, now a Tab stop with arrow and Enter keys, a selected circle, click-to-select, selected and disabled declarations, and a selected graph state
@task_file n=11 path=examples/seven_guis/tests/host_binary.rs
what: edited — the seven_guis host binary check
@task_file n=12 path=tests/blitz-tests/tests/common/mod.rs
what: edited — the shared stand expectation tables, among them the stated Tab order tables (Circle Drawer's order added) and the graph drive table
@task_file n=13 path=tests/blitz-tests/tests/fixtures/mod.rs
what: edited — the shared test fixtures, whose virtual node declarations state no selected state
@task_file n=14 path=tests/blitz-tests/tests/virtual_nodes_declared.rs
what: edited — the check of declared virtual nodes
@task_file n=15 path=tests/blitz-tests/tests/virtual_nodes_as_elements.rs
what: edited — the check that virtual nodes read as elements in the tree, the snapshot and a diff, now including a declared selected state
@task_file n=16 path=tests/blitz-tests/tests/stand_circle_nodes.rs
what: edited — the stand check of Circle Drawer's circle nodes: click-to-select, keys, covered by a later node, disabled while the dialog is open, and the focus events of a canvas click
@task_file n=17 path=tests/blitz-tests/tests/stand_keyboard.rs
what: edited — the stand keyboard check that walks each stated Tab order, now including Circle Drawer's canvas
@task_file n=18 path=tests/blitz-tests/tests/stand_canvas_click.rs
what: edited — the engine-level canvas click check, now also checking that Tab and Shift+Tab leave a focused widget that still hears them
@task_file n=19 path=tests/blitz-tests/tests/stand_circle_drawer_canvas.rs
what: edited — the stand check of Circle Drawer's canvas and its adjust dialog
@task_file n=20 path=tests/blitz-tests/tests/stand_act_error_text.rs
what: edited — the stand check of the driver's act error text
@task_file n=21 path=docs/capabilities.md
what: edited — the capability statement and its accessibility reach, edited by hand: the a11y leg reading, the current circle read from selected alone, and the covered meaning
@task_item n=1 part=touches
absence/keyboard-virtual-nodes: retired. Circle Drawer's circles are now reached by keyboard: Tab to the canvas, arrows to select, Enter to open the dialog.
@task_item n=2 part=touches
decision/stated-tab-order-walked and decision/stand-shared-tab-order-table: Circle Drawer's Tab order (back button, then canvas) joins the walked and shared tables, walked forward and back in both layout modes.
@task_item n=3 part=touches
decision/circle-drawer-canvas-pixels-only: the canvas is now a Tab stop with arrow and Enter keys. A click on a circle selects it rather than adding one, and the selected circle is declared selected.
@task_item n=4 part=touches
contract/driver-click-refusals/covered: a canvas's node now also reads covered where a node its canvas declares later has a region holding the target's centre. A region is the declared bounding box, so a later corner covers where its painted shape does not. PROVISIONAL.
@task_item n=5 part=touches
decision/tab-and-shift-tab-move-focus: now also holds on a focused custom-widget element. The widget hears the KeyDown first, then focus moves (engine change, PROVISIONAL, upstreamable).
@task_item n=6 part=touches
decision/focusable-elements and decision/click-focuses-buttons: the engine focuses a custom-widget element (a canvas with tabindex) on pointer down, so a canvas click focuses it with blur, focusout, focus and focusin in one step.
@task_item n=7 part=touches
decision/active-descendant-unread and decision/active-descendant-kept-unread: a matching residual on Circle Drawer. While the canvas holds focus, an assistive technology learns the current circle from selected alone, and no circle is the tree's focus.
@task_item n=8 part=touches
contract/accessibility-tree-states/aria-selected and decision/toggle-read-by-role: the tree's selected state is now also written from a virtual node's declaration, not only from ARIA or select options.
@task_item n=9 part=touches
contract/snapshot-diff/differs and contract/snapshot-text/state-tokens: a virtual node now reads selected or unselected in the snapshot and its text, and a flip reads differs selected.
@task_item n=10 part=touches
contract/accessibility-tree/focus: on Circle Drawer, the engine, snapshot and tree focus agree on the canvas and never on a circle.
@task_item n=11 part=touches
contract/driver-actions/disabled and contract/driver-refusal/disabled: while the adjust dialog is open, every circle but the adjusted one is declared disabled and refused disabled.
@task_item n=12 part=touches
contract/actions-answer/causes and decision/actions-verb-changes-nothing: actions lists covered for a node under a later one, through the same target check as the run path.
@task_item n=13 part=touches
contract/ui-graph-state/use-graph-state: Circle Drawer registers one more graph state, selected (written by canvas click and keydown), and the canvas gains a keydown handler.
@task_item n=14 part=touches
decision/keyboard-activation-enter-space: on Circle Drawer's canvas, Enter is an app key that opens the dialog on the selected circle.
@task_item n=15 part=touches
decision/capability-statement-reach-by-hand and decision/accessibility-reach-tier-0: the hand-edited reach now says the current circle is read from selected alone, and the a11y leg reading moved to 20 · 201 · 0 · 0.
@task_fact n=1 slug=keyboard-navigation
fact: A focused custom-widget element does not trap Tab. The widget hears the KeyDown, then Tab or Shift+Tab moves focus as on any element, so a widget cannot keep Tab for itself (PROVISIONAL, flagged upstreamable).
@task_fact n=2 slug=keyboard-navigation
fact: A pointer down on a focusable custom-widget element focuses it within the click's own step, with no app focus call.
@task_fact n=3 slug=5-keyboard-navigation
fact: seven_guis Circle Drawer canvas: one Tab stop. The arrows move the selection in declared circle order, clamped, and the first arrow with none selected selects the first circle. Enter opens the dialog on the selected circle, and a key pressed with the dialog open changes nothing.
@task_fact n=4 slug=5-keyboard-navigation
fact: seven_guis Circle Drawer canvas click: it selects the topmost circle whose disc holds the point and adds none, else it adds a circle.
@task_fact n=5 slug=keyboard-navigation
fact: After Circle Drawer's dialog closes, focus is dropped to the root, and a click on the canvas is needed before its keys act again. This is a stated limit, carried to the A11y re-read.
@task_fact n=6 slug=screen-reader-support
fact: A canvas's virtual node may declare a selected state. None states nothing, and true or false is written to the tree and read by the snapshot as selected or unselected.
@task_fact n=7 slug=screen-reader-support
fact: Residual: on Circle Drawer, an assistive technology learns the current circle from the selected state alone. The canvas keeps focus and no circle is the tree's focus.
@task_fact n=8 slug=visual-design-verification
fact: Residual: a later-declared square region's corner refuses covered on an earlier circle where the painted disc does not cover it.
@task_item n=1 part=untouched
Contrast, the visual lint and its contrast finding, and the stand accent and scrollbar colours: no colour pair or lint rule moved.
@task_item n=2 part=untouched
Single and multiple select, the open list, range input and the CRUD and Cells keyboard contracts: their keys and events are unchanged.
@task_item n=3 part=untouched
Hidden and inert content, aria-hidden and hit-testing on inert elements: unchanged.
@task_item n=4 part=untouched
Accessible name precedence, labels, passwords and the platform adapter (accesskit_xplat, its absence on stand binaries): unchanged.
@task_item n=5 part=untouched
Cognitive accessibility, invalid-state rules and the obs log format of the a11y assertion harness: unchanged.
@task_item n=6 part=untouched
The CI legs, their tiers and the three-round stand repeat: no gate, threshold or leg changed.
