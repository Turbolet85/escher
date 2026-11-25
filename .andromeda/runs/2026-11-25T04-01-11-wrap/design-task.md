@task_prose part=session
The session added virtual nodes declared: an app's canvas widget may now declare keyed, named, role-bearing regions inside the canvas, through a defaulted widget-trait method that declares nothing unless a widget overrides it.
The dioxus bridge gained an in-process read of one canvas's declared nodes: it validates each declaration in order and refuses it with a closed reason (empty key, key holding a slash, duplicate key with the first kept, empty name, region outside the canvas's content box), and gives each taken node the reserved id form {owner}///{key}.
The region must lie inside the canvas's content box, its final layout size less border and padding; a stale or non-canvas node reads no canvas, and an unlaid canvas reads not laid out; there is no panic, log, changed-set mark or invalidation signal.
No carrier gains a reading: the snapshot, the UI graph, the accessibility tree, the actions line and the driver verbs carry no virtual node, and a test asserts a declaring canvas puts nothing on the snapshot or graph in both layout modes.
A declared node's role is AccessKit's own Role, with no second mapping; whether its name and key cross into the accessibility tree and the snapshot stays an open fork, deferred to the later entry of virtual nodes as elements, as do click on a virtual node and its changed-set mark.
Every rule this session rests on is PROVISIONAL, a delegate's answer and not the founder's; no dependency, config, gate or colour, type, spacing or motion value moved.
@task_file n=1 path=packages/blitz-dom/src/node/custom_widget.rs
what: the engine's custom widget trait that canvas widgets implement (paint, layout, pointer), now with the defaulted virtual-node declaration method and the declaration type
@task_file n=2 path=packages/blitz-dom/src/node/mod.rs
what: the engine's node module and its public re-exports of the widget types
@task_file n=3 path=packages/blitz-dom/src/lib.rs
what: the engine crate's root and its public re-export list
@task_file n=4 path=packages/dioxus-native-dom/src/lib.rs
what: the dioxus bridge crate's root, its module list and public re-export list
@task_file n=5 path=packages/dioxus-native-dom/src/element_id.rs
what: the stable element id of the dioxus bridge, which defines the reserved virtual-node id form {owner}///{key}
@task_file n=6 path=packages/dioxus-native-dom/src/virtual_node.rs
what: the dioxus bridge's new in-process read and validation of a canvas's declared virtual nodes, with its unit checks
@task_file n=7 path=tests/blitz-tests/tests/virtual_nodes_declared.rs
what: the integration check that a padded canvas reads its declared nodes in its content box and that a declaring canvas puts nothing on the snapshot or the UI graph
@task_item n=1 part=touches
contract/driver-verbs-and-virtual-clock/canvas: the canvas verb's canvas now may declare virtual nodes inside it; the verb itself is unchanged and reads none of them, which a picker may want beside it.
@task_item n=2 part=touches
decision/canvas-line-form-conventions: canvas lines are unchanged, but a canvas now holds declared regions in its content-box coordinates (border and padding excluded); no canvas line names them.
@task_item n=3 part=touches
contract/escher-session-commands/snapshot and contract/escher-session-mcp/snapshot-result: the snapshot now stands asserted to carry no virtual node and no declared name or id holding ///, while the in-process read holds them.
@task_item n=4 part=touches
contract/escher-session-commands/graph-answer and contract/ui-graph-line/content: the UI graph now stands asserted to carry no virtual node; a declaring canvas adds no line to it.
@task_item n=5 part=touches
contract/actions-line/nodes and decision/actions-share-the-verbs-checks: a virtual node is not an actionable node; actions list none and click on one is deferred to a later entry.
@task_item n=6 part=touches
contract/driver-verbs-and-virtual-clock/click: click still takes elements only; taking click on a virtual node, refused when covered, off-screen, disabled or stale, is the later entry's.
@task_item n=7 part=touches
contract/select-accessibility rows (open select, chosen option, highlighted row, open attribute): the accessibility tree gains no virtual node this session; a declared node carries an AccessKit Role but crosses into no tree.
@task_item n=8 part=touches
decision/circle-drawer-canvas-colours-in-code: indirect, through the widget trait the Circle Drawer canvas implements; the trait gained a defaulted method and no widget implementation, Circle Drawer's included, was edited.
@task_item n=9 part=touches
decision/escher-session-agent-read-surface and decision/driver-output-for-agents: indirect; what an agent reads of a canvas stays unchanged, the declared regions reach no agent-facing output this session.
@task_fact n=1 slug=surface
fact: An app's canvas widget may declare keyed, named, role-bearing virtual nodes inside the canvas; they are read in process only, each taken node gets the reserved id {owner}///{key}, its region must lie in the canvas's content box, and a refused declaration gets a closed reason (PROVISIONAL, a delegate's answer).
@task_fact n=2 slug=surface
fact: Virtual nodes reach no carrier: the snapshot, the UI graph, the accessibility tree, the actions line and the driver verbs hold no declared node, name or /// id; whether name and key cross is the founder's open fork, deferred to the virtual-nodes-as-elements entry (PROVISIONAL).
@task_item n=1 part=untouched
Colour, typography, spacing, border radius and depth rows: no colour, font, spacing, radius, elevation, paint order or hit-test rule moved; a declared region paints nothing and is not hit-tested.
@task_item n=2 part=untouched
Motion rows: the virtual clock, advance, settle, scroll and animation rules are unchanged; the declared-node read moves no clock.
@task_item n=3 part=untouched
Iconography rows: cursor and favicon rules are unchanged.
@task_item n=4 part=untouched
The select list rows (its events, overlay, open gestures, colours, placement, screenshot and snapshot row bounds): no select behaviour moved.
@task_item n=5 part=untouched
escher-session exit statuses and codes, stderr, session error, mcp role framing, help, tasks, coverage, layout and screenshot answers: no command, argument, exit status or output form changed.
@task_item n=6 part=untouched
Shell provider, window, JS window object, dioxus-native configuration, font context and MountedData rows: no configuration or shell behaviour changed.
@task_item n=7 part=untouched
WPT runner output and result colours, the web-spa surface and mobile-target rows: not reached.
