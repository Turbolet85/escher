@task_prose part=chunk
A behaviour-preserving restructure of three functions toward the cognitive-complexity ceiling of 15: blitz-dom's click handler, escher-driver's executor and dioxus-native-dom's UI graph builder.
Helpers are extracted inside the file that holds each function today; no file split, no new module, no other function restructured.
Nothing observable changes: dispatch order, event sequences, default actions, refusal order, outcomes, span fields, snapshot and graph content stay as they are.
It re-measures each function and every extracted helper before and after on the dev host, and re-runs the existing click, driver-act, graph and handler-coverage checks that hold the moved arms.
@task_file n=1 path=packages/blitz-dom/src/events/pointer.rs mark=edits
what: blitz-dom's pointer event handling, the click default actions (checkbox and radio toggle with input and change, button focus and submit, the select arms, links, summary, labels) — edits: the function handle_click
@task_file n=2 path=packages/escher-driver/src/execute.rs mark=edits
what: escher-driver's executor, the per-verb dispatch of driver calls, the target check and the refusal order — edits: the function execute
@task_file n=3 path=packages/dioxus-native-dom/src/graph.rs mark=edits
what: dioxus-native-dom's UI graph, the read-only graph the Dioxus document builds from its nodes and handlers — edits: the function DioxusDocument::graph
@task_item n=1 part=needs
The click default action for a native checkbox or radio: the click toggles it, fires input then change, focuses it, and makes one engine change per click — kept exactly through the restructure.
@task_item n=2 part=needs
A click focuses a button, and a button click's submit default action, kept in the same order.
@task_item n=3 part=needs
A disabled form control stops the click walk: the restructured click handler keeps that stop at the same point.
@task_item n=4 part=needs
The select arms of the click handler: a click opens or closes the single select's engine overlay list, focus stays on the select while its list is open, list choices share one write path, and a multiple select's row clicks keep their behaviour.
@task_item n=5 part=needs
Label clicks: a label click focuses its select and opens nothing, a hidden checkable's label still toggles its native state.
@task_item n=6 part=needs
The range input click: a click gives focus and not value, while a main-button pointerdown sets the value.
@task_item n=7 part=needs
Link and summary click default actions, and the details open state a summary click drives.
@task_item n=8 part=needs
Double click read on the document's clock with its threshold, as the click path carries it.
@task_item n=9 part=needs
Keyboard activation: Enter and Space activate as a click, with the handler order unchanged, since they reach the same click default actions.
@task_item n=10 part=needs
The driver's synthetic click is a primary click at the node's centre, and a driver click in a shown scrollbar band drags — the executor's click dispatch must keep both.
@task_item n=11 part=needs
Driver refusal causes and their order: a covered target, a disabled target, and the executor's target check, unchanged in order and cause.
@task_item n=12 part=needs
The read-only driver verbs keep changing nothing: the actions verb and the screenshot verb as the executor dispatches them, and driver type focusing a select with no click.
@task_item n=13 part=needs
The UI graph mirrors the snapshot one node per snapshot node, carries no values, and its content is unchanged by the restructure.
@task_item n=14 part=needs
UI graph state registration as the graph reads it: use_graph_state, unconfirmed, undeclared and no registered state.
@task_item n=15 part=needs
The Dioxus document's reached-handler record and the handler coverage answer, read through the graph and held by the handler-coverage check.
@task_item n=16 part=needs
Listeners read from the vdom walk, which the graph builder uses to attach handlers to nodes.
@task_item n=17 part=needs
The actionable rule the graph and snapshot share: one actionable rule, the actionable-key rule and interactive nodes carrying a name, unchanged.
@task_item n=18 part=needs
Any click arm no existing check reaches gets a check before the move, so each moved arm is held by a test that could fail.
@task_standing n=1
convention: each new check for a click default action is a Rust test asserting against the stand's stated expectations, repeated across the stand's rounds
home: 3-a11y-assertion-harness-contract · ci-integration
@task_item n=1 part=not_needed
Accessibility tree building and the AccessKit adapter: tree states, role mapping, hidden content exclusion and adapter creation order are not touched by the three functions.
@task_item n=2 part=not_needed
ARIA name and property reads and accessible-name precedence: no read of an ARIA attribute changes.
@task_item n=3 part=not_needed
Visual design verification: contrast thresholds, the lint's contrast finding, scrollbar thumb strokes, select list colours, row heights and motion, scheme and zoom.
@task_item n=4 part=not_needed
Keyboard key contracts for select lists, the range input's arrow, Home and End keys, the Cells grid and the CRUD list: keyboard key handling outside click activation is not restructured.
@task_item n=5 part=not_needed
Tab order and focus traversal rules: Tab and Shift+Tab focus moves, focusability recomputation, focus fixup on inertness and removed-node focus loss.
@task_item n=6 part=not_needed
Cognitive accessibility of the stand's content: invalid state stated beyond colour or class.
@task_item n=7 part=not_needed
Accessibility scope and feature wiring: tier 0 reach, the accessibility feature defaults, platform adapter choice and stand binaries building no adapter.
@task_item n=8 part=not_needed
Password masking and snapshot text form tokens: no reading or serialised form of the snapshot changes.
