@task_prose part=session
The session makes a single select whose size attribute parses above 1 an in-flow list box in every document: its rows laid out in the page with their own boxes, no popup list ever opened by a click, Enter or Space.
On a single list box the arrows, Home, End and typeahead choose a row and scroll it into view; a click on a row chooses that row alone; it may stand with nothing chosen; it reads role ListBox with no expanded state.
A size write moves a select between list box and drop-down and closes an open list; a drop-down (display size 1, including size "1", "0", "-3", empty, non-numeric or absent) is unchanged, and a multiple select reads as before.
The driver's snapshot reads ListBox and the rows' bounds, actions lists the rows in view with no cause, click is accepted with value and rows' selected in its diff, press and type choose with no list open; only the click and press help texts are re-worded, no verb, argument, key name, field, cause, exit, log field or span added.
Measured: a page whose size-3 list box scrolls two of five rows out lints 4 findings at boot (those rows clipped, truncated, off-screen), no overlap; and a node a restyle turns display none keeps its last client rect.
Retired: the stated limit that a single select's size is unread; select test counts moved select_interaction 18 to 24, select_popup 27 to 28, stand_act_select 11 to 17.
@task_file n=1 path=packages/blitz-dom/src/node/select.rs
what: the engine's select model: a select's display size, list box or drop-down reading, and the settled chosen option
@task_file n=2 path=packages/blitz-dom/src/node/select_list.rs
what: the engine's open-list gate for a select's popup list, now open for a drop-down alone
@task_file n=3 path=packages/blitz-dom/src/stylo/presentational_hints.rs
what: the engine's presentational hints, the size attribute making a select rows tall and the list box rows and chosen-row colours
@task_file n=4 path=packages/blitz-dom/src/mutator.rs
what: the engine's DOM mutator, a size or multiple attribute write restyling the select's rows and closing its list
@task_file n=5 path=packages/blitz-dom/src/document.rs
what: the engine document, its public read of whether a select is a list box
@task_file n=6 path=packages/blitz-dom/src/events/keyboard.rs
what: the engine's key handling for form controls, the keys of a select list box and of a drop-down
@task_file n=7 path=packages/blitz-dom/src/events/pointer.rs
what: the engine's click handling, a click on a select opening its list and a click on a list box row choosing it
@task_file n=8 path=packages/blitz-dom/src/accessibility.rs
what: the accessibility tree, a select's role (ListBox or ComboBox) and its expanded state
@task_file n=9 path=packages/blitz-dom/src/layout_reason.rs
what: the layout answer's placing reason for a select's option rows
@task_file n=10 path=packages/dioxus-native-dom/src/snapshot.rs
what: the snapshot builder and its closed-select bridge, which answers none for a list box so its rows read their own bounds
@task_file n=11 path=packages/escher-driver/src/schema.rs
what: the driver's verb schema, the click and press help texts
@task_file n=12 path=packages/escher-driver/src/schema_tests.rs
what: the driver's checks of its verb schema and help texts
@task_file n=13 path=docs/capabilities.md
what: the capability statement, the select model and its stated limits
@task_file n=14 path=tests/blitz-tests/tests/select_interaction.rs
what: the select interaction check, keys, clicks, size writes and roles of selects, both layout modes
@task_file n=15 path=tests/blitz-tests/tests/select_popup.rs
what: the select popup list check, which selects open a list
@task_file n=16 path=tests/blitz-tests/tests/stand_act_select.rs
what: the driver-session check of selects, snapshot, click, actions, press, type and boot lint on select pages
@task_item n=1 part=touches
The flight booker's flight type select reads ComboBox at boot: it is a drop-down with no size, and it stays a ComboBox with its list opening as before; a picker checks the boot reading still names role ComboBox and the list it opens.
@task_item n=2 part=touches
Open list rows read their own bounds: a single list box's rows now read their own bounds too, always, with no list open, since the closed-select bridge answers none for a list box.
@task_item n=3 part=touches
Into-view scroll moves every holding scroller: a key or typeahead on a single list box scrolls the reached row into view inside the box (nearest, instant), a new caller of that scroll.
@task_item n=4 part=touches
Visible region through clipping boxes: rows of a list box scrolled out of its box read clipped, truncated or off-screen to the lint and actions, the rows clipped by the box itself.
@task_item n=5 part=touches
Stand lints clean at boot: a page holding a scrolling list box lints its hidden rows at boot (4 findings measured on the five-row size-3 page, no overlap); the stand's own pages, which hold no sized single select, still lint clean.
@task_item n=6 part=touches
Stand names, roles and states by attributes alone: a size attribute above 1 on a single select now changes its role to ListBox and drops its expanded state.
@task_item n=7 part=touches
CRUD list as one keyboard region: where the CRUD list is a select, its keys and row clicks follow the list box or drop-down reading by its display size; a picker checks what element that list is.
@task_item n=8 part=touches
escher-session snapshot answer: a single list box reads role ListBox, no expanded, and its rows with their own bounds.
@task_item n=9 part=touches
escher-session actions answer: a single list box's rows in view are listed with no cause (not covered).
@task_item n=10 part=touches
escher-session help answer: the click and press help texts are re-worded for the list box (press: the arrows, home and end choose a row and nothing opens); verbs, arguments, fields, causes and counts unchanged.
@task_item n=11 part=touches
escher-session lint answer: rows a list box scrolls out read clipped, truncated or off-screen, as any scrolling box's hidden rows.
@task_item n=12 part=touches
Session recording as a replayable check: a recorded click, press or type on a sized single select now replays as a row choice with no list open, its diff naming value and the rows' selected.
@task_item n=13 part=touches
escher-session served tasks flight-booker and crud: their select elements keep the reading their size gives; a served page with a sized single select would now serve a list box.
@task_fact n=1 slug=surface-desktop-native
fact: A single select whose size parses above 1 is an in-flow list box: its rows laid out in the page with their own bounds, no popup list, role ListBox with no expanded, keys and row clicks choosing one row, and it may stand with nothing chosen; size 1, 0, negative, empty, non-numeric or absent keeps the drop-down.
@task_fact n=2 slug=surface-cli
fact: On a single list box the driver's click on a row is accepted and chooses it alone, press arrow, home and end choose a row with no list open, type chooses by the text's start, and actions lists the rows in view with no cause; no verb, argument, key name, field, cause or exit was added for it.
@task_fact n=3 slug=surface-desktop-native
fact: A page with a list box whose rows scroll out of its box does not lint clean at boot: the hidden rows read clipped, truncated or off-screen, with no overlap.
@task_item n=1 part=untouched
The ShellProvider window chrome controls (drag, maximize, minimize, close, decorations) are not reached.
@task_item n=2 part=untouched
The stand stable element ids: no id was added, renamed or removed; the list-box pages are test pages, not the stand.
@task_item n=3 part=untouched
The agent-run, cold-agent, multi_doc and paint_bench command lines, their usage, stdout, exit codes and pass-throughs are not reached.
@task_item n=4 part=untouched
The escher-session exit codes, refusals (unknown-verb, off-screen, fonts-unavailable), recording files, host log service name and command form: none moved.
@task_item n=5 part=untouched
The MCP server over stdio, its tools list, protocol revisions, result bound and screenshot result: unchanged.
@task_item n=6 part=untouched
The stand's skeleton, accent colour, pinned headless viewport, held-instance boot, shared stand boot and the load page's network provider: not reached.
@task_item n=7 part=untouched
The cold-agent decisions (pipe builds nothing, prompt names no id, drives through MCP) and caller input never written back: not reached; the list box read carries no text out.
@task_item n=8 part=untouched
Multiple selects: their rows, keys, row clicks and role read as before.
