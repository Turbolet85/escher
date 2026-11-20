@task_prose part=session
The session settled the open select list's debts and the Cells stand's selected cell, on the founder's forks F1 A, F2 A, F3 A, F4 C and F5 A.
The lint now reads an open list's highlighted row at its painted pair through a new read-only harness reader, highlight_contrast; plain rows keep their own reading, and the finding's shape is unchanged.
Every row of an open list now takes the tallest row's height, so a larger authored font is no longer clipped; the clause that such a row clips is retired.
While a single select's list is open, the engine names the highlighted option as the select's active descendant; a page's aria-activedescendant stays unread, and focus stays on the select.
The snapshot and the diff gain the node key highlighted, a closed bool written only when true (node keys 14 to 15, readings 12 to 13, node state fields 8 to 9), and a highlight move by key or pointer now marks the select as changed.
The Cells stand's current data cell now states aria-selected true, alone; an arrow moves it and its diff names selected and bounds; Cells' two cross-OS digests were re-pinned and its part count stays 6. A single select's size stays unread, kept as its own route entry.
@task_file n=1 path=packages/blitz-test-harness/src/contrast.rs
what: the harness's text contrast reading, now with the highlighted row's painted-pair reader that the stand lint reads
@task_file n=2 path=packages/escher-driver/src/lint.rs
what: the escher-session lint's contrast check, the answer of the lint verb
@task_file n=3 path=packages/blitz-dom/src/document.rs
what: the engine document's open select list, its row height and its highlight moves by key
@task_file n=4 path=packages/blitz-dom/src/events/pointer.rs
what: the engine's pointer events, the hover that moves an open list's highlight
@task_file n=5 path=packages/blitz-dom/src/accessibility.rs
what: the engine's accessibility tree builder, where roles, names and states are read from attributes and the open list's active descendant is written
@task_file n=6 path=packages/dioxus-native-dom/src/snapshot.rs
what: the snapshot's node state, the readings each node carries
@task_file n=7 path=packages/dioxus-native-dom/src/snapshot_text.rs
what: the snapshot's text form and its state tokens, the answer escher-session snapshot prints
@task_file n=8 path=packages/dioxus-native-dom/src/snapshot_diff.rs
what: the snapshot diff and its list of readings, behind the changed set an acting verb returns
@task_file n=9 path=packages/escher-driver/src/schema.rs
what: the driver's node-key table, read by help and by the MCP server's tool descriptions
@task_file n=10 path=packages/escher-driver/src/schema_tests.rs
what: the node-key table's checks, its count of keys
@task_file n=11 path=packages/escher-driver/src/json/snapshot.rs
what: the JSON form of a snapshot node, the snapshot answer's JSON
@task_file n=12 path=packages/escher-driver/src/json/diff.rs
what: the JSON form of a diff's changed and added nodes
@task_file n=13 path=packages/escher-driver/src/json_tests.rs
what: the checks of the driver's JSON snapshot and diff forms
@task_file n=14 path=packages/escher-driver/src/describe.rs
what: the escher-session help answer's description of the node keys and their counts
@task_file n=15 path=examples/seven_guis/src/tasks/cells.rs
what: the Cells task of the seven_guis stand, its cells grid and data cells cell-{A..Z}{0..25} with their attributes
@task_file n=16 path=tests/blitz-tests/tests/select_popup.rs
what: the open select list checks, its rows' bounds and heights and its active descendant
@task_file n=17 path=tests/blitz-tests/tests/select_popup_painted.rs
what: the open select list's painted checks, its highlight colour pair and glyphs inside their row
@task_file n=18 path=tests/blitz-tests/tests/stand_lint.rs
what: the stand lint checks, the stand lints clean and an open list's highlighted row reads its painted pair
@task_file n=19 path=tests/blitz-tests/tests/stand_act_popup.rs
what: the stand's acting checks on a served select's open list, a press returning its diff
@task_file n=20 path=tests/blitz-tests/tests/stand_cells_served.rs
what: the served Cells task's checks, its parts and its selected current cell
@task_file n=21 path=tests/blitz-tests/tests/stand_cross_os_readings.rs
what: the stand's cross-OS readings check, pinned digests of served tasks' snapshots
@task_file n=22 path=tests/blitz-tests/tests/stand_diff.rs
what: the stand's snapshot diff checks, one comparison per reading
@task_file n=23 path=docs/capabilities.md
what: the capability statement, its limits and its node-key count
@task_item n=1 part=touches
The stand names, roles and states by attributes alone: the Cells task's current data cell now states selected through aria-selected alone, an attribute written by the stand, so the rule gains one stated state.
@task_item n=2 part=touches
The Cells data cell ids cell-{A..Z}{0..25} and the cells grid: the cell with the current position now carries aria-selected true, and no other cell does; an arrow moves it.
@task_item n=3 part=touches
The escher-session served task cells: its snapshot reads selected on the current cell alone, its part count stays 6, and its two cross-OS digests at boot and after the step were re-pinned.
@task_item n=4 part=touches
The escher-session snapshot answer: a node gains the state token highlighted, after clickable, written only when true, and the JSON writes highlighted true only when true; node keys move from 14 to 15.
@task_item n=5 part=touches
The escher-session help answer: its node keys list now holds 15 entries, and the help for snapshot text and for acting changed names highlighted.
@task_item n=6 part=touches
The escher-session MCP server tools/list: the tool descriptions read from the node-key table now name highlighted.
@task_item n=7 part=touches
The escher-session actions answer: an acting verb's changed set now names highlighted, and a press in an open list returns the highlight move on both rows; a highlight move by key or pointer marks the select, a wheel move inside the list or a move to the same row marks nothing.
@task_item n=8 part=touches
The escher-session lint answer: the contrast check now reads an open list's highlighted row at its painted pair (about 12.56), so that row raises no contrast finding while plain rows still do; the finding's shape, order and written form are unchanged.
@task_item n=9 part=touches
Open list rows read their own bounds: every row now takes the tallest row's height with the select's one-line height as the floor, so a larger authored font is painted whole inside its row; rows still read their own bounds.
@task_item n=10 part=touches
The flight type select's open list: while open, the highlighted option is named as the select's active descendant and reads highlighted in the snapshot.
@task_item n=11 part=touches
A page's aria-activedescendant stays unread: the engine itself now writes the active descendant for an open single select, while a page's attribute is still not read; a row stating the active descendant is unread holds true only for the page's attribute.
@task_item n=12 part=touches
Stand lints clean at boot: the stand still lints clean, the lint's contrast now reading the highlighted row's painted pair.
@task_item n=13 part=touches
Into-view scroll moves every holding scroller: a Cells arrow brings the current cell into view, so its diff names selected and bounds on both cells.
@task_item n=14 part=touches
Session recording as a replayable check and the recording check file: highlighted is written only when true, so the committed Counter checks replay byte-equal.
@task_fact n=1 slug=surface-desktop-native
fact: The Cells task's current data cell alone states aria-selected true, set by the stand as an attribute, and an arrow key moves it to the next cell.
@task_item n=1 part=untouched
The ShellProvider window chrome controls: no window drag, close, decoration, maximize or minimize changed.
@task_item n=2 part=untouched
The agent-run.sh and cold-agent.sh pipes, their usage, exits, logs and Windows pass-through, and the cold agent's prompt and MCP use.
@task_item n=3 part=untouched
The multi_doc and paint_bench command lines, their arguments, environment variables and stdout.
@task_item n=4 part=untouched
The escher-session exit codes, refusals, command form, host log service name, coverage role, graph, layout and screenshot answers, and the MCP server's stdio, end, protocol revisions and result bound: no verb, argument, exit, cause, error, log field or span was added.
@task_item n=5 part=untouched
The stable element ids of the Counter, Circle Drawer, CRUD, Flight Booker, Load, Order, Temperature Converter and Timer tasks and the task cards: no id was added, renamed or freed.
@task_item n=6 part=untouched
The fixed document skeleton, the headless stand's pinned viewport, the held instance read as a fresh boot, the shared stand boot of session roles, the load page's network provider and the order page's controls.
@task_item n=7 part=untouched
The stand accent colour: the highlight's colours are engine constants pinned to the default sheet's chosen row, not the stand's accent.
@task_item n=8 part=untouched
The absence of arbitrary index.html templates, and the mobile-native, web-spa and decisions-log areas.
