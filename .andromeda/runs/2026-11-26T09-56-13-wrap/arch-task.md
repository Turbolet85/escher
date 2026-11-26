@task_prose part=session
Circle Drawer's circles, declared as a canvas's virtual nodes, are now reached, selected and acted on by keyboard and click: the canvas is one Tab stop, arrows move a selection in declared order (clamped), Enter opens the diameter dialog on the selected circle, and a click on a circle selects it and adds none.
VirtualNodeDeclaration gains a selected field (None states nothing, Some(b) the node's selected state); the accessibility tree writes it on the virtual node and the snapshot's virtual branch reads selected from the tree node, so a flip reads differs selected; the tree's focus stays on the canvas element.
The driver's covered refusal now also reads a canvas virtual node covered by a node its canvas declares later whose region (a bounding box) holds the target's region centre; the run path and the actions verb share the one target check; the cause's name, remedy and order are unchanged, its fixed meaning restated (PROVISIONAL).
Engine change behind custom-widget, PROVISIONAL and flagged upstreamable: after a focused custom-widget element hears a Tab KeyDown, the engine runs its Tab default action (next or previous focus), so a widget element no longer traps Tab; written in an escher-own function beside widget click synthesis, one call in upstream's dispatch body.
Measured: the engine already focuses a custom-widget element on pointer down, so the app's own focus call on the canvas click was removed; two stand readings retired (a click on a circle's node adds a circle; another circle never covers one), each replaced; no verb, argument, key name, cause, session error, exit, carrier, log field, span, dependency or crate added or removed.
@task_file n=1 path=packages/blitz-dom/src/node/custom_widget.rs
what: blitz-dom nodes and widgets: the Widget trait and VirtualNodeDeclaration, edited to add the selected field
@task_file n=2 path=packages/blitz-dom/src/events/mod.rs
what: blitz-dom DOM event dispatch, handle_dom_event's custom-widget arm, edited to call the Tab pass-through after the widget hears the key
@task_file n=3 path=packages/blitz-dom/src/events/pointer.rs
what: blitz-dom pointer-derived events and click synthesis on a custom-widget target, edited to hold the new escher-own Tab pass-through function
@task_file n=4 path=packages/dioxus-native-dom/src/dioxus_document.rs
what: the Dioxus document and its accessibility tree with virtual nodes, edited to write a virtual node's declared selected state
@task_file n=5 path=packages/dioxus-native-dom/src/snapshot.rs
what: the snapshot reader, edited so a virtual node reads selected from its tree node as an element does
@task_file n=6 path=packages/dioxus-native-dom/src/virtual_node.rs
what: the Dioxus bridge's virtual node reader and its unit declaration builder, edited for the selected field
@task_file n=7 path=packages/escher-driver/src/execute.rs
what: the driver's acting step and per-verb target check, edited to read covered by a later-declared virtual node
@task_file n=8 path=packages/escher-driver/src/actions.rs
what: the actions verb's reader, edited so its targets carry the virtual id into the same target check
@task_file n=9 path=packages/escher-driver/src/refusal.rs
what: the refusal causes and their fixed meanings served by help and MCP, edited to restate covered's meaning
@task_file n=10 path=examples/seven_guis/src/tasks/circle_drawer.rs
what: the seven_guis Circle Drawer task: its canvas, graph states, handlers and circle declarations, edited for selection, keys and the new canvas attributes
@task_file n=11 path=examples/seven_guis/tests/host_binary.rs
what: the seven_guis host binary check, edited for the declaration change
@task_file n=12 path=tests/blitz-tests/tests/common/mod.rs
what: the shared stand-check helpers and the stated tab orders and graph drive tables, edited for Circle Drawer's order and drives
@task_file n=13 path=tests/blitz-tests/tests/fixtures/mod.rs
what: the shared blitz-tests fixtures, edited so their declarations take selected none
@task_file n=14 path=tests/blitz-tests/tests/virtual_nodes_declared.rs
what: the check of virtual node declarations, edited for the selected field
@task_file n=15 path=tests/blitz-tests/tests/virtual_nodes_as_elements.rs
what: the check that virtual nodes read as elements in tree, snapshot and diff, edited to add the selected reading and its local fixture
@task_file n=16 path=tests/blitz-tests/tests/stand_circle_nodes.rs
what: the stand check of Circle Drawer's circle nodes, edited for selection, covered by a later node, disabled while the dialog is open, keys and focus events
@task_file n=17 path=tests/blitz-tests/tests/stand_keyboard.rs
what: the stand keyboard check walking each app's stated tab order, edited to add Circle Drawer's walk
@task_file n=18 path=tests/blitz-tests/tests/stand_canvas_click.rs
what: the stand canvas click check, edited to add the engine-level Tab and Shift+Tab check on a focused widget
@task_file n=19 path=tests/blitz-tests/tests/stand_circle_drawer_canvas.rs
what: the stand check of Circle Drawer's canvas and dialog, edited for the centre click selecting and Close after a scroll
@task_file n=20 path=tests/blitz-tests/tests/stand_act_error_text.rs
what: the stand check of acting error text, edited for the restated covered meaning
@task_file n=21 path=docs/capabilities.md
what: the capability statement, edited with the keyboard path, the current circle read from selected, the covered meaning and the a11y leg reading
@task_file n=22 path=.andromeda/runs/2026-11-26T07-21-03-implement/gate-2026-11-26-canvas-keyboard-path.json
what: the implement run's gate trail, a bookkeeping record, rewritten by the pre-push gate firing; no code
@task_file n=23 path=.andromeda/runs/2026-11-26T07-21-03-implement/rec-2026-11-26-canvas-keyboard-path.json
what: the chunk's land trail, a bookkeeping record, edited in the operator pass; no code
@task_item n=1 part=touches
contract/nodes-widgets/virtual-nodes: VirtualNodeDeclaration gains a public selected field, Option of bool, None stating nothing and Some(b) the node's selected state, behind custom-widget.
@task_item n=2 part=touches
contract/accessibility-tree/virtual-nodes: the accessibility tree with virtual nodes now writes set_selected on a virtual node whose declaration states Some(b); focus untouched.
@task_item n=3 part=touches
contract/snapshot/virtual-node and contract/snapshot/selected, contract/node-keys/selected: a virtual node's snapshot reads selected from its tree node, as the element branch does, and its text and diff read selected, unselected or no word; a flip reads differs selected (contract/snapshot-diff/differs, decision/differs-closed-set: no new member).
@task_item n=4 part=touches
contract/refusal-causes/covered: its fixed meaning restated (PROVISIONAL) — a canvas's node reads covered by an element over its region's centre, or by a node its canvas declares later whose region holds that centre; a region is the declared bounding box, so a later corner covers where the painted shape does not; the old clause that another node of its canvas never covers is gone; name, remedy and order unchanged.
@task_item n=5 part=touches
contract/target-check/click-type, contract/driver-verbs/click-virtual-node, contract/actions-verb/virtual-node, decision/kept-check-replays-through-run: the run path and the actions verb stay one target check, both now reading covered by a later-declared node.
@task_item n=6 part=touches
decision/visibility-from-geometry-not-hit: the new covered clause reads declared regions in declaration order, not a hit, after the unchanged element-hit reading.
@task_item n=7 part=touches
decision/sub-document-event-forwarding, decision/widget-click-synthesis, decision/handlers-before-default-action, decision/default-action-event-order: the engine's custom-widget arm no longer returns before every default action for Tab — the widget hears the KeyDown, then the Tab default action moves focus (Shift backwards); PROVISIONAL.
@task_item n=8 part=touches
contract/event-dispatch/focus-change, contract/dom-events/focus-change, contract/key-names/tab: Tab and Shift+Tab now move focus off a focused custom-widget element.
@task_item n=9 part=touches
decision/fork-additions-in-own-functions: followed — the Tab pass-through is an escher-own function beside widget click synthesis with one call in upstream's dispatch body.
@task_item n=10 part=touches
decision/direct-engine-changes-flagged-upstreamable, decision/a11y-input-focus-engine-changes, decision/hit-focus-form-engine-changes, decision/upstream-behaviour-changes-ratified-per-take: a new direct engine change (the widget Tab pass-through) flagged upstreamable, PROVISIONAL.
@task_item n=11 part=touches
decision/feature-gated-widgets: both the selected field and the Tab pass-through sit behind custom-widget.
@task_item n=12 part=touches
decision/cells-grid-single-tab-stop: Circle Drawer's canvas becomes a second single Tab stop with arrow and Enter keys, beside Cells' grid.
@task_item n=13 part=touches
decision/canvas-placement-at-click-centre, decision/canvas-command-grouping: a click on a circle's node no longer adds a circle — it selects the topmost circle whose disc holds the point; a click off every circle still adds one at the click point.
@task_item n=14 part=touches
decision/graph-state-by-registration, decision/seven-guis-graph-signals, contract/ui-graph/writers, contract/handler-coverage/canvas: Circle Drawer registers a new graph state selected, written by circle-canvas click and keydown; the canvas gains a keydown handler that two existing states also declare as a writer.
@task_item n=15 part=touches
decision/one-disabled-reading, decision/disabled-read-on-form-control-only, contract/refusal-causes/disabled: while the dialog is open every circle but the adjusted one declares disabled and is refused disabled — a virtual node's declared disabled read beside form controls.
@task_item n=16 part=touches
contract/accessibility-tree/focus, contract/snapshot/focused, decision/interaction-state-on-dom-nodes: the canvas element keeps focus across arrows; no virtual node is ever the tree's focus; the current circle is conveyed by selected alone.
@task_item n=17 part=touches
contract/accessibility-tree/active-descendant: still not used for virtual nodes; the current circle is read from selected alone, beside the recorded active-descendant limit.
@task_item n=18 part=touches
decision/keyboard-activation-enter: Enter on the focused canvas is the app's own keydown handling (opens the dialog on the selected circle), not a click activation.
@task_item n=19 part=touches
contract/capability-statement/sections, decision/capability-statement, decision/measured-limits-stated-in-the-tool: the capability statement adds the keyboard path, the current circle read from selected alone, the restated covered meaning and the a11y leg reading 20 · 201 · 0 · 0 (was 20 · 199 · 0 · 0).
@task_item n=20 part=touches
decision/one-self-description-for-cli-and-mcp, contract/mcp-role/self-description: the restated covered meaning is served through help and MCP descriptions.
@task_item n=21 part=touches
decision/declaration-change-marks-nothing, decision/a11y-rebuild-on-nonempty-changed-set: a selected flip is a declaration change read through the canvas's attribute change; check the rule still holds as worded.
@task_item n=22 part=touches
decision/single-reader-page-stays-local, decision/shared-fixtures-stated-once: the new selecting fixture stays in its one reader's file; shared fixtures take selected none.
@task_fact n=1 slug=established-decisions
fact: A focused custom-widget element does not trap Tab: the widget hears the KeyDown, then the engine's Tab default action moves focus (Shift+Tab backwards); a widget cannot keep Tab for itself (PROVISIONAL, flagged upstreamable).
@task_fact n=2 slug=established-decisions
fact: The engine focuses a custom-widget element on pointer down, so a click on a canvas focuses it within the click's own settled step, with blur and focusout on the prior node and focus and focusin on the canvas; an app needs no focus call of its own.
@task_fact n=3 slug=standard-contracts
fact: Circle Drawer's canvas is one Tab stop: arrows move the selection over circles in declared order, clamped, the first arrow with none selected selecting the first; Enter opens the diameter dialog on the selected circle; a press with the dialog open changes nothing.
@task_fact n=4 slug=standard-contracts
fact: A canvas's current virtual node is conveyed to assistive technology by selected alone; focus stays on the canvas element and no active descendant names a virtual node.
@task_fact n=5 slug=standard-contracts
fact: A later-declared node's bounding-box corner refuses covered where the shape it paints does not cover; a recorded limit of the region-based covered reading.
@task_item n=1 part=untouched
The session socket, its wire versions, MCP protocol and server, session errors, exit codes and recording directory: no carrier, call, answer field or error changed.
@task_item n=2 part=untouched
The verb table, argument kinds and key names: no verb, argument or key name added or removed; the refusal faults are unchanged.
@task_item n=3 part=untouched
Telemetry, tracing and the telemetry line: no log field, span or event added.
@task_item n=4 part=untouched
Stack, dependencies and toolchain: no crate, dependency, feature or tool version added or removed.
@task_item n=5 part=untouched
Style, layout, text, paint and scrolling engine rows: the canvas paints and lays out as before (its size held at 700 by 400); no layout or paint rule moved.
@task_item n=6 part=untouched
CI legs, gates, coverage floors, mutation tier and stand-repeat budgets: no leg, threshold or floor changed.
@task_item n=7 part=untouched
WPT runner, the browser app, script runtime, networking and history rows.
@task_item n=8 part=untouched
Names: no crate, binary, server or environment name taken or freed.
@task_item n=9 part=untouched
Select list, range input and form-control rows: their keyboard and default actions are unchanged.
@task_item n=10 part=untouched
Stable element ids and virtual node id forms: ids and their tiers unchanged.
