@task_prose part=session
Circle Drawer's canvas became one Tab stop with its own keys: arrows move a selection across its circles in declared order, clamped; Enter opens the diameter dialog on the selected circle; with the dialog open no key acts and every other circle reads disabled.
A click on a circle now selects it and adds none; a click off every circle adds one as before; the engine itself focuses a custom-widget element on pointer down, so the app's own focus call was written and removed.
The engine's widget event arm now lets Tab and Shift+Tab move focus on after a focused custom-widget element hears the key, so such an element no longer traps Tab; PROVISIONAL on the operator's word, flagged upstreamable, moved into a fork-owned function after implement with no behaviour change.
A virtual node declaration gains a selected field (None states nothing, Some written as the node's selected state), read into the accessibility tree and the snapshot as an element's is; the tree's focus never lands on a virtual node.
The covered refusal's meaning widened (PROVISIONAL): a canvas's virtual node also reads covered where a node its canvas declares later has a region holding the target's region centre, the region being the declared bounding box; one target check still serves the run path and the actions read; no new verb, argument, cause, session error, exit, log field or span.
@task_file n=1 path=packages/blitz-dom/src/node/custom_widget.rs
what: the custom widget boundary and its virtual node declaration, which gains the selected field
@task_file n=2 path=packages/blitz-dom/src/events/mod.rs
what: the engine's DOM event dispatch, its custom-widget arm now calling the Tab pass-through after the widget hears a key
@task_file n=3 path=packages/blitz-dom/src/events/pointer.rs
what: the engine's pointer and custom-widget event synthesis, beside the click synthesis on a custom-widget target, now holding the Tab pass-through
@task_file n=4 path=packages/dioxus-native-dom/src/dioxus_document.rs
what: the accessibility tree built with virtual nodes, now writing a declared selected state on a virtual node
@task_file n=5 path=packages/dioxus-native-dom/src/snapshot.rs
what: the snapshot's state readings, its virtual-node branch now reading selected from the tree node
@task_file n=6 path=packages/dioxus-native-dom/src/virtual_node.rs
what: the virtual node declarations and their validation, its unit builder taking the selected field
@task_file n=7 path=packages/escher-driver/src/execute.rs
what: the driver's single target check for acting verbs, adding the covered-by-a-later-node reading for a canvas's virtual node
@task_file n=8 path=packages/escher-driver/src/actions.rs
what: the actions read, which shares the verbs' target check and now marks a virtual-node target
@task_file n=9 path=packages/escher-driver/src/refusal.rs
what: the closed refusal cause set with each cause's fixed meaning and remedy, the covered meaning restated
@task_file n=10 path=examples/seven_guis/src/tasks/circle_drawer.rs
what: the Circle Drawer sample app, its canvas widget, its graph states and its declared writes
@task_file n=11 path=examples/seven_guis/tests/host_binary.rs
what: the seven_guis host-binary test, a test only
@task_file n=12 path=tests/blitz-tests/tests/common/mod.rs
what: the stand tests' shared helpers and Circle Drawer's drives and stated tab order
@task_file n=13 path=tests/blitz-tests/tests/fixtures/mod.rs
what: the stand tests' fixture pages, their virtual node declarations taking selected none
@task_file n=14 path=tests/blitz-tests/tests/virtual_nodes_declared.rs
what: the check of declared virtual nodes, a test only
@task_file n=15 path=tests/blitz-tests/tests/virtual_nodes_as_elements.rs
what: the check that a virtual node reads as an element in the tree, the snapshot and a diff, now with selected
@task_file n=16 path=tests/blitz-tests/tests/stand_circle_nodes.rs
what: the stand checks of Circle Drawer's circle nodes: selection, covered, disabled, keys and focus events
@task_file n=17 path=tests/blitz-tests/tests/stand_keyboard.rs
what: the stand's keyboard walk of each sample's stated tab order
@task_file n=18 path=tests/blitz-tests/tests/stand_canvas_click.rs
what: the stand's canvas click check, now with the engine-level check that Tab and Shift+Tab leave a focused widget
@task_file n=19 path=tests/blitz-tests/tests/stand_circle_drawer_canvas.rs
what: the stand checks of Circle Drawer's canvas and its dialog
@task_file n=20 path=tests/blitz-tests/tests/stand_act_error_text.rs
what: the stand check of acting-error text, a test only
@task_file n=21 path=docs/capabilities.md
what: the committed capability statement, its accessibility reach and its click and covered limits
@task_item n=1 part=touches
Driver refusal cause covered: its fixed meaning is restated — a node a canvas declares reads covered by an element over its region's centre, or by a node its canvas declares later whose region holds that centre, a region being the declared bounding box (PROVISIONAL); the row stating covered only by an element, never by another node of its canvas, is stale.
@task_item n=2 part=touches
Refusal JSON meaning and the MCP server help: both serve the covered meaning text, which changed; name, remedy and order of the cause unchanged.
@task_item n=3 part=touches
Fixed order of target refusals and the closed refusal cause set: the later-node reading runs after the unchanged element-hit reading inside the same cause; no cause added, so the order and the set hold.
@task_item n=4 part=touches
Actions read and verbs share one target check: the target now carries a private virtual id set both by the run path and by the actions read, so the two stay one check including the later-node covered reading.
@task_item n=5 part=touches
Click synthesized on a custom-widget target: the same engine widget arm now also passes Tab and Shift+Tab on to the engine's focus moving after the widget hears the key, where before it returned before every default action (PROVISIONAL, upstreamable).
@task_item n=6 part=touches
Virtual node crossing and declared virtual nodes validated by a closed refusal set: a declaration gains a selected field, a closed Option of bool, crossing into the tree and the snapshot; no new input to validate.
@task_item n=7 part=touches
Snapshot state words selected and unselected, the diff node key selected and three state readings per control: a virtual node now reads selected, unselected or no word from its declaration, and a flip reads differs holding selected.
@task_item n=8 part=touches
Control states cross as closed values, and a closed-value field in a ratified carrier is no widening: the virtual node's selected state is such a closed value on existing carriers.
@task_item n=9 part=touches
Accessibility focus names a built node: the tree's focus stays on the canvas element and never lands on a circle's virtual node; the three focus readings agree.
@task_item n=10 part=touches
Programmatic focus changes fire focus events and a button click focuses the button: a click on a custom-widget element is now measured to focus it on pointer down, firing blur and focusout on the prior node, then focus and focusin on the canvas, in one step.
@task_item n=11 part=touches
Cells grid keyboard model and Enter activates the focused control: a second canvas keyboard model now stands beside Cells — Circle Drawer's arrows and Enter on its focused canvas.
@task_item n=12 part=touches
Graph answer crossing, the graph answer's nodes and states and a DeclaredWrite's event and target: Circle Drawer registers one more graph state, selected, and declares canvas keydown writes; no new field of the answer.
@task_item n=13 part=touches
Canvas content crossing and attribute text leaving only by ratified carriers: Circle Drawer's canvas gains two attributes, data-selected and data-adjusting, each an id or empty, beside the unchanged data-circles.
@task_item n=14 part=touches
Custom widget initial attribute replay: the Circle Drawer canvas widget now reads the two new attributes through it to declare selected and disabled per circle.
@task_item n=15 part=touches
Committed statement of the MCP crossing: the capability statement restates the covered limit and adds that the current circle is read from selected alone.
@task_fact n=1 slug=input-validation
fact: A focused custom-widget element never traps Tab: after the widget hears a KeyDown of Tab the engine's widget arm runs the Tab default action, Shift reading backwards, so a widget cannot keep Tab for itself; PROVISIONAL, flagged upstreamable.
@task_fact n=2 slug=input-validation
fact: The engine focuses a custom-widget element on pointer down, so a click on a canvas focuses it within the click's own settled step with no app focus call.
@task_fact n=3 slug=input-validation
fact: A virtual node declaration's selected field is a closed Option of bool — None states nothing, Some is written as the node's selected state in the tree and read by the snapshot as an element's is; the tree's focus never names a virtual node.
@task_fact n=4 slug=input-validation
fact: Circle Drawer's canvas keyboard model: one Tab stop; arrows move the selection in declared order, clamped, the first arrow with none selected selecting the first; Enter opens the dialog on the selected circle; with the dialog open no key acts and every circle but the adjusted one is declared disabled.
@task_fact n=5 slug=error-handling
fact: A canvas's region is its declared bounding box, so a later node's corner refuses covered where the shape it paints does not cover — a recorded residual of the covered reading (PROVISIONAL).
@task_item n=1 part=untouched
Dependency security: no dependency, lockfile, manifest, audit, licence or pin moved.
@task_item n=2 part=untouched
Secret management and environment variables: nothing reads a new variable or secret.
@task_item n=3 part=untouched
Authentication and authorization, the session socket's grammar, bounds, replies, errors and exit codes, and the MCP protocol errors: no new verb, argument, key name, session error, exit or carrier.
@task_item n=4 part=untouched
Logging and monitoring: no new log field, span or target; the command span's cause is unchanged.
@task_item n=5 part=untouched
Data protection and kept or recorded check files: their contents, lifecycle and masking are unchanged.
@task_item n=6 part=untouched
Outbound network, fetch, CORS, iframe, stylesheet and HTML parsing rows: no network or document-loading path moved.
@task_item n=7 part=untouched
Stable element id derivation, its virtual-node form and its carriers: circle ids and the canvas id are unchanged.
@task_item n=8 part=untouched
Select list, range input, password masking and multiple select rows: no form control behaviour moved.
