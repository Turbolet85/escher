@task_prose part=session
A behaviour-preserving restructure of three functions to bring their cognitive complexity under 15: the engine's click handler (97 to 2), the driver session's command dispatch (32 to 1) and the Dioxus document's UI graph builder (50 to 4).
The click handler now walks through one private function per element arm (checkbox, radio, summary, select, option, label, link, file input, the element match and the walk itself); the label arm still re-dispatches the click to its bound control; the double-click dispatch and the focus clear stay in the handler.
The driver's command dispatch is now one match with one private function per verb (snapshot, click, type with its acting step, press, advance, scroll, screenshot, graph); the command span, its fields, the target checks and the refusals are unedited.
The UI graph builder now splits into private functions for state readings, node handlers, handler writes, node reads and graph states, still compiled under the accessibility gate.
One new integration check of click default actions was added, six cases in both layout modes; no public symbol, span field, refusal cause, verb, dependency or capability was added, removed or retired.
Measured: every gate green, CI green 17 of 17, coverage floors held (driver 98.01, dioxus-native-dom 90.09, test harness 95.74 percent).
@task_file n=1 path=packages/blitz-dom/src/events/pointer.rs
what: the engine's pointer and click handling: the click walk up the ancestors and each element's click default action (checkbox and radio toggle with input and change, label re-dispatch to its control, select and option choice, summary toggle, link navigation, button and submit, file input), the focus clear and the double-click dispatch
@task_file n=2 path=packages/escher-driver/src/execute.rs
what: the driver session's command execution: one arm per verb (snapshot, click, type, press, advance, scroll, screenshot, graph, lint, actions), the type verb's acting order on a select or a text field, the screenshot crop and its error map
@task_file n=3 path=packages/dioxus-native-dom/src/graph.rs
what: the Dioxus document's UI graph builder: per-node handlers from the listeners, the reached-handler record, registered state readings, handler writes with their declared and unconfirmed flags, node reads, behind the accessibility feature
@task_file n=4 path=tests/blitz-tests/tests/click_default_actions.rs
what: an integration check of click default actions: a link leaving the document navigates and a fragment link does not, a link with no href walks on to its label, a click no element takes clears focus, a disabled submit button stops the walk, a label of a multiple select focuses it and chooses nothing, two clicks with no time between them are one double click
@task_item n=1 part=touches
decision/disabled-control-stops-click-walk: the click walk moved into its own function, behaviour unchanged; the new click check now asserts a disabled submit button stops the walk.
@task_item n=2 part=touches
decision/checkable-click-toggles-and-focuses and decision/one-engine-change-per-checkable-click: the checkbox and radio arms moved into their own functions, the input and change dispatches and the focus step unchanged.
@task_item n=3 part=touches
The rows whose verified evidence cites click-handler line numbers (the change-event decision naming the checkable_change_event tests, and the security twin): those dispatches moved, the checkbox change now further down and the radio change behind its was-checked test; the evidence cells' line numbers are stale, the rule true.
@task_item n=4 part=touches
decision/native-checkable-toggle and decision/hidden-checkable-label-toggle: the label arm's re-dispatch to its bound control moved into its own function, still recursing into the click handler.
@task_item n=5 part=touches
decision/label-click-focuses-select-only: the label arm moved; the new click check asserts a label of a multiple select focuses it and chooses nothing.
@task_item n=6 part=touches
decision/click-focuses-buttons and the button and submit default action: the button arm now calls the existing button-click function from the element match, unchanged.
@task_item n=7 part=touches
decision/double-click-threshold and decision/double-click-on-document-clock: the double-click dispatch stays in the click handler; the new click check asserts two clicks with no time between them are one double click.
@task_item n=8 part=touches
contract/keyboard-activation/handler-order: a true claim sharing the click handler's name; Enter and Space still activate through the same click path, no change.
@task_item n=9 part=touches
decision/select-list-choice-one-write-path, decision/disabled-optgroup-one-rule, contract/multiple-select/keys-and-row-clicks and contract/single-select/optgroup-options: the select, option and optgroup arms moved into their own functions, the optgroup arm still through the existing row-landing test.
@task_item n=10 part=touches
decision/open-pseudo-class-on-details-dialog and the summary toggle: the summary arm (first summary of its details only) moved into its own function, unchanged.
@task_item n=11 part=touches
contract/snapshot/file-input-value: the file input's click arm moved, under its feature, unchanged.
@task_item n=12 part=touches
decision/driver-type-focuses-select-no-click: the type verb's acting order (select focus or click, select-all and backspace, the typing that skips a leading space on a select) is now a named step function, unchanged.
@task_item n=13 part=touches
decision/synthetic-click-at-node-centre and the driver click verb: the click arm moved into its own function, the target check and the refusals unedited.
@task_item n=14 part=touches
decision/actions-verb-changes-nothing, decision/screenshot-changes-nothing, contract/driver-screenshot/element-shot and contract/driver-screenshot/row-shot: the screenshot verb with its crop and error map moved into its own function; actions stays a one-line arm.
@task_item n=15 part=touches
The snapshot verb rows (decision/snapshot-reads-the-tree, contract/mcp-result/snapshot, the snapshot text form): the snapshot arm with its text budget and parts moved into its own function, unchanged.
@task_item n=16 part=touches
decision/ui-graph-carries-no-values and decision/ui-graph-one-node-per-snapshot-node: the graph builder split into private functions and the graph verb feeds the session's record of ids as before.
@task_item n=17 part=touches
contract/ui-graph-state rows (use_graph_state, undeclared, unconfirmed, no registered state): the handler-write flags and the state readings moved into their own functions, unchanged.
@task_item n=18 part=touches
contract/handler-coverage/answer, contract/dioxus-document/reached-handlers, contract/dioxus-document/record-reached and decision/listeners-from-vdom-walk: the per-node handler listing and the ran test moved out of the graph builder, unchanged.
@task_item n=19 part=touches
decision/agent-readers-behind-accessibility and decision/accessibility-feature-gate: the graph builder's new functions are still compiled under the accessibility gate.
@task_item n=20 part=touches
decision/removed-focused-node-drops-focus and the focus clear on a click: the clear stays in the click handler after the walk; the new click check asserts it.
@task_fact n=1 slug=keyboard-navigation
fact: A click on an element no click arm takes clears focus, after which the focused reading is the root element, never none.
@task_item n=1 part=untouched
The cognitive-accessibility areas: the session's cognitive figure is a code-complexity metric, not cognitive accessibility for users.
@task_item n=2 part=untouched
Contrast, the visual lint, the stand accent colour, scrollbar thumbs and select-list colours: no paint, style or lint code moved.
@task_item n=3 part=untouched
Role mapping, ARIA property reads, accessible-name precedence and the accessibility tree states and hidden-content rows: the tree build is unedited.
@task_item n=4 part=untouched
Keyboard key contracts (select keys open and closed, Cells grid, CRUD list, range keys, Tab order): key handling is unedited.
@task_item n=5 part=untouched
Accessibility tier, the platform adapter, AccessKit and the feature defaults: no feature, dependency or adapter changed.
@task_item n=6 part=untouched
Password masking and the snapshot node's keys and wire form: no reader or schema changed.
@task_item n=7 part=untouched
The CI legs and the stand checks repeated three rounds: no leg or round changed, all ran green.
