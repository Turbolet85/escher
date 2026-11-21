@task_prose part=chunk
Renders a single select (no multiple) whose size reads a valid value above 1 as HTML does: an in-flow list box size rows tall, its rows the options and group headers, the chosen row marked, the rest scrolling.
In that box keys move the choice, a click on a row chooses it, and no engine-painted popup list opens; size 1, 0 or an invalid value leave it a closed drop-down box.
Partly reverses the founder's F8: the clause that a single select's size stays unread falls; the multiple-select size reading stands, with no 4-row default.
Closes residual res-ysz67imk, re-verified at take-up; snapshot, diff, accessibility, lint, actions, layout and screenshot readings follow from the engine like a multiple list box's.
@task_file n=1 path=packages/blitz-dom/src/node/select_list.rs mark=edits
what: the select list node component, the select's size reading — edits: size_rows, click_select_row, press_multiple_select_key
@task_file n=2 path=packages/blitz-dom/src/node/select.rs mark=edits
what: the select element node component, the single select's state and key model — edits: press_select_key
@task_file n=3 path=packages/blitz-dom/src/document.rs mark=edits
what: the engine document, the gate that lets a select open its popup list — edits: select_list_allowed
@task_file n=4 path=packages/blitz-dom/src/events/keyboard.rs mark=edits
what: the engine's keyboard event handling, the keys that move a select's choice
@task_file n=5 path=packages/blitz-dom/src/events/pointer.rs mark=edits
what: the engine's pointer event handling, a click on a select or a list box row
@task_file n=6 path=packages/blitz-dom/src/stylo/presentational_hints.rs mark=edits
what: the presentational hints, a select's height of size rows from its size attribute — edits: the select size height hint
@task_file n=7 path=packages/blitz-dom/src/accessibility.rs mark=edits
what: the engine accessibility tree, a select's role as list box or combo box
@task_file n=8 path=tests/blitz-tests/tests/select_interaction.rs mark=edits
what: the select interaction checks, keys and clicks on a select, both layout modes
@task_file n=9 path=tests/blitz-tests/tests/select_popup.rs mark=edits
what: the select popup list checks, when a popup opens and when none does
@task_file n=10 path=tests/blitz-tests/tests/stand_act_select.rs mark=edits
what: the stand checks of driver acts on a select, snapshot, actions and layout readings
@task_file n=11 path=tests/blitz-tests/tests/stand_act_popup.rs mark=edits
what: the stand checks of driver acts on an open popup list, its rows and bounds
@task_item n=1 part=needs
The select size attribute read: the rule that size is read on a multiple select alone changes, a valid size above 1 now read on a single select too, the multiple reading (size rows tall, no size all rows) kept as it is.
@task_item n=2 part=needs
The select's chosen set and a list row's label stay out of every log line and span field, for the sized single select's rows as for the multiple list box's.
@task_item n=3 part=needs
Any engine log site the chunk's select code writes carries a fixed message and safe fields only, never an option label, value or the chosen row.
@task_item n=4 part=needs
No new span, span field, log field, metric, verb, argument, key name, result field, refusal cause, session error or exit: the change crosses none unless the founder rules it; the driver's command and tool_call spans stay as they are.
@task_item n=5 part=needs
The snapshot and diff readings of a sized single select: its rows as real boxes with row bounds like a multiple list box's, its value, and the expanded and highlighted readings it gives with no popup.
@task_item n=6 part=needs
The actions answer and the snapshot node's clickable reading for the rows of a sized single select, a click on a row choosing it.
@task_item n=7 part=needs
The layout answer and the graph answer for a sized single select: an in-flow list box size rows tall, its content never logged.
@task_item n=8 part=needs
The screenshot of a sized single select carries its row labels as pixels only, the shot image leaving only in its answer; group headers shown as rows like the optgroup label crossing.
@task_item n=9 part=needs
Page script events on a sized single select: one change event per choice moved by key or click, as a single select fires today.
@task_item n=10 part=needs
The open popup list is not opened for a sized single select; the unsized single select's popup list keeps its own behaviour.
@task_item n=11 part=needs
The accessibility role of a sized single select reads as a list box, not a combo box, as HTML maps it.
@task_standing n=1
convention: every new check runs silent, sharing modules, and prints no answer content
home: 3-observability-harness-contract
@task_standing n=2
convention: every new engine log line writes a fixed message with safe fields only
home: 8-pii-scrubbing-compliance · pii-scrubbing-compliance
@task_item n=1 part=not_needed
Metric coverage: no counter, gauge, histogram or exporter is added and no timing figure changes.
@task_item n=2 part=not_needed
The cold-agent pipe and the agent-run harness: their events, verdict fields, counted verbs and reasons are not touched.
@task_item n=3 part=not_needed
CI integration: artifacts, the package-install diagnostic, the build-directory sweep, recording and session state directories; dependency and CI debts are out of scope.
@task_item n=4 part=not_needed
Error capture and reporting: script error sources, the window error event, onerror and the WPT report do not change.
@task_item n=5 part=not_needed
The telemetry strategy and the sink's line format, resource keys and RUST_LOG configuration: no tracing feature or sink change.
@task_item n=6 part=not_needed
escher-session's commands, exit codes, error object, host wire and coverage role: the chunk adds no command or exit.
