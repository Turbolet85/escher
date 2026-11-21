@task_prose part=session
A single select whose size attribute parses above 1 is now shown as an in-flow list box of that many rows, the rest scrolling, and opens no popup list; display size 1 (absent, 0, 1, empty, negative, non-numeric) keeps the drop-down.
Its arrows, Home, End and typeahead choose a row and scroll it into view instantly; a row click chooses that row alone whatever modifier is held; Enter and Space do nothing; the select may stand with nothing chosen.
Its rows take presentational hints, the chosen row the Chromium inactive selected pair black on #c8c8c8 (12.56:1 measured); the tree reads ListBox with no expanded state and no active descendant.
The snapshot bridge treats a list box as no closed select, so its rows keep their own bounds and clicks land; the driver's click and press help texts are re-worded, no verb, argument, field, cause or exit added.
Retired: the stated limit that a single select's size is unread. Measured: the driver lint at boot reports a scrolling box's hidden rows as clipped, truncated or off-screen, no overlap.
@task_file n=1 path=packages/blitz-dom/src/node/select.rs
what: the select model — display size, list box or drop-down reading, the settled chosen option of a single select
@task_file n=2 path=packages/blitz-dom/src/node/select_list.rs
what: the open select list overlay of a single select — its gate, now a drop-down alone
@task_file n=3 path=packages/blitz-dom/src/stylo/presentational_hints.rs
what: the presentational hints — the select size hint and the list box rows and chosen row colour pair
@task_file n=4 path=packages/blitz-dom/src/mutator.rs
what: the DOM mutator — a size or multiple write restyles the select and its rows, and closes an open list
@task_file n=5 path=packages/blitz-dom/src/document.rs
what: the engine document — the public list box reading of a select and the settle of a select's choice
@task_file n=6 path=packages/blitz-dom/src/events/keyboard.rs
what: keyboard handling of selects — drop-down keys, the list box's arrows, Home, End and typeahead
@task_file n=7 path=packages/blitz-dom/src/events/pointer.rs
what: pointer handling of selects — click opens a drop-down's list, a list box row click chooses the row
@task_file n=8 path=packages/blitz-dom/src/accessibility.rs
what: the accessibility tree — the select role (ListBox or ComboBox) and its expanded state and active descendant
@task_file n=9 path=packages/blitz-dom/src/layout_reason.rs
what: the layout reason words — an option of a list box reads its own placing
@task_file n=10 path=packages/dioxus-native-dom/src/snapshot.rs
what: the snapshot bridge — closed select of a node, which folds a closed select's rows into it
@task_file n=11 path=packages/escher-driver/src/schema.rs
what: the driver schema — the click and press verbs' help texts
@task_file n=12 path=packages/escher-driver/src/schema_tests.rs
what: the driver schema's unit checks over the verbs' help texts
@task_file n=13 path=docs/capabilities.md
what: the capability statement — the select model and its stated limits
@task_file n=14 path=tests/blitz-tests/tests/select_interaction.rs
what: the select interaction checks — keys, clicks, choice and the list box's rows, both layout modes
@task_file n=15 path=tests/blitz-tests/tests/select_popup.rs
what: the select popup list checks — opening, closing and the open gate
@task_file n=16 path=tests/blitz-tests/tests/stand_act_select.rs
what: the stand's driver checks of selects — snapshot, click, actions, press, type and lint at boot
@task_item n=1 part=touches
Selects that open no list: a single select with size above 1 now joins the multiple select as a select that opens no list, on click, Enter or Space.
@task_item n=2 part=touches
Click, Enter or Space opens a single select: now a drop-down alone (display size 1); a single list box opens nothing on those gestures.
@task_item n=3 part=touches
Single select box shows its chosen option: holds for the drop-down only; a single list box shows its rows, the chosen one painted, and may show none chosen.
@task_item n=4 part=touches
Select chosen row takes Chromium's inactive selected pair: now also painted on a single list box's chosen row, black on #c8c8c8, 12.56:1.
@task_item n=5 part=touches
Select list colours as engine constants and engine colours hardcoded: the list box row pair is another hardcoded engine constant, no token.
@task_item n=6 part=touches
Select list as an engine overlay and the open list hit first and painted after the root: the overlay is now reached by a drop-down alone; a list box lays out in flow with no overlay.
@task_item n=7 part=touches
Open select list placed inside the viewport and wheel and key scroll the open select list: their reach narrows to a drop-down; a list box scrolls as its own box.
@task_item n=8 part=touches
Select list opens with no motion and into-view scroll instant: a list box's key-reached row is scrolled into view instantly, nearest edge.
@task_item n=9 part=touches
The snapshot and accessibility tree of a select (open select, open attribute, chosen option, highlighted row): a single list box reads ListBox with no expanded and no active descendant, and ignores an author aria-expanded.
@task_item n=10 part=touches
A single select's list events (open, close, choose, highlight move, close on its select's change): a list box raises no list events; a size write that makes a drop-down a list box closes its open list.
@task_item n=11 part=touches
The snapshot and screenshot of a row of an open select list: a list box's rows carry their own bounds in the snapshot with no list open.
@task_item n=12 part=touches
The actions line content and nodes: a list box's rows are listed in view with no covered cause, as the bridge no longer folds them into the select.
@task_item n=13 part=touches
actions decided by the verbs' own checks: a click on a list box row is accepted and its diff names value and the rows' selected.
@task_item n=14 part=touches
escher-session commands help and the driver verbs click and press (virtual clock): the click and press help texts are re-worded for the list box; no verb, argument, key name or field added.
@task_item n=15 part=touches
The driver verb type: on a list box it chooses by the text's start, typeahead, no list opened.
@task_item n=16 part=touches
Driver command line output read by agents: the lint at boot on a page with a scrolling list box reports its hidden rows as clipped, truncated or off-screen findings.
@task_fact n=1 slug=surface
fact: A single select whose size parses above 1 is an in-flow list box of that many rows, the rest scrolling, opening no list; display size 1, from size absent, 0, 1, empty, negative or non-numeric, keeps the drop-down; a size write moves a select between the two.
@task_fact n=2 slug=surface
fact: A single list box may stand with nothing chosen; only a drop-down falls back to its first enabled option.
@task_fact n=3 slug=surface
fact: A single list box's arrows, Home, End and typeahead choose a row, a row click chooses that row alone whatever modifier is held, and Enter, Space without typed text and other keys do nothing; a disabled select or row changes nothing.
@task_fact n=4 slug=surface
fact: A single list box reads role ListBox in the snapshot and tree, with no expanded state, no active descendant and no multiselectable, and an author aria-expanded is not read.
@task_fact n=5 slug=color-palette
fact: A single list box's chosen row takes Chromium's inactive selected pair, black on #c8c8c8, 12.56:1, as row presentational hints.
@task_fact n=6 slug=surface
fact: The driver lint reports the rows of a scrolling list box scrolled out of its box as clipped, truncated or off-screen findings, never overlap.
@task_item n=1 part=untouched
Typography: font faces, families, root-relative units, locale and the screenshot text rules are not reached.
@task_item n=2 part=untouched
Iconography: cursor icons, cursor resolution and the favicon are not reached.
@task_item n=3 part=untouched
The multiple select: its default stylesheet rules, keys, row clicks and size rows are unchanged.
@task_item n=4 part=untouched
escher-session exit statuses and exit codes, the mcp role results, stderr, the session error remedy and the tasks contracts: no exit, cause, error or task added.
@task_item n=5 part=untouched
The WPT runner output and result colours, the shell provider window controls, the JS window object and the dioxus-native config rows are not reached.
@task_item n=6 part=untouched
Stacking-context paint and hit grouping, auto z-index and hit-testing at a paint clip are not reached.
@task_item n=7 part=untouched
Animation on the virtual clock, settle, advance, smooth scroll-behavior and overlay scrollbar timings are not reached.
@task_item n=8 part=untouched
Spacing rows other than the select list placement: body margins and table cell spacing hints are not reached.
