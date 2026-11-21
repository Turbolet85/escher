@task_prose part=chunk
A single select (no `multiple`) whose `size` reads a valid value above 1 is shown as HTML renders it: an in-flow list box `size` rows tall, its rows the options and group headers, the chosen row marked, the rest scrolling.
In that box keys move the choice and a click on a row chooses it; no popup list opens for it.
It reverses in part the founder's F8: the clause that a single select's `size` stays unread falls; the `multiple` reading stands as built.
It re-verifies at take-up that residual res-ysz67imk (a single select's `size` stays unread) is owned here, and closes it.
Snapshot, diff, accessibility tree, lint, actions, layout and screenshot readings of the sized single select follow from the engine like a `multiple` list box's; any reading that would differ is a founder fork.
@task_file n=1 path=packages/blitz-dom/src/node/select_list.rs mark=edits
what: the select list component and a select's `size` reading — edits: size_rows, click_select_row, press_multiple_select_key
@task_file n=2 path=packages/blitz-dom/src/node/select.rs mark=edits
what: the select element's state: single or multiple, its chosen option, its options and group headers
@task_file n=3 path=packages/blitz-dom/src/document.rs mark=edits
what: the document's select handling, the gate on which selects open a list — edits: select_list_allowed
@task_file n=4 path=packages/blitz-dom/src/events/keyboard.rs mark=edits
what: keyboard events on a focused select, the single select's key model — edits: press_select_key
@task_file n=5 path=packages/blitz-dom/src/events/pointer.rs mark=edits
what: pointer events on a select: the click that opens a single select, the click on a list box row
@task_file n=6 path=packages/blitz-dom/src/stylo/presentational_hints.rs mark=edits
what: presentational hints, the list box height of `size` rows from a select's `size` — edits: the `size` hint on a select
@task_file n=7 path=packages/blitz-dom/src/accessibility.rs mark=edits
what: the accessibility tree of a select, its list box or combo box role
@task_file n=8 path=packages/blitz-dom/assets/default.css mark=edits
what: the default user-agent stylesheet, a list box's look
@task_file n=9 path=packages/blitz-paint mark=edits
what: the painter, a single select's painted face (the closed box that shows its chosen option)
@task_file n=10 path=packages/escher-driver mark=edits
what: the driver's readers of a select: snapshot expanded and highlighted, diff, `type` on a select, lint, actions, layout's placing words
@task_file n=11 path=tests/blitz-tests/tests/select_interaction.rs
what: the select interaction checks, keys and clicks on a select, both layout modes
@task_file n=12 path=tests/blitz-tests/tests/select_popup.rs
what: the select popup list checks, both layout modes
@task_file n=13 path=tests/blitz-tests/tests/stand_act_select.rs
what: the driver stand checks of acting on a select
@task_file n=14 path=tests/blitz-tests/tests/stand_act_popup.rs
what: the driver stand checks of acting on an open select list
@task_item n=1 part=needs
A sized single select (no `multiple`, a valid `size` above 1) renders as an in-flow list box `size` rows tall: its rows the options and group headers, the chosen row marked, the rows beyond `size` scrolling inside the box.
@task_item n=2 part=needs
The `size` reading: parsed as an unsigned integer, capped at 65534; on a single select `size="1"`, `size="0"` or an invalid value leave it a drop-down box with its popup list, per HTML's list box versus drop-down box rule.
@task_item n=3 part=needs
The list box height presentational hint (`size` rows at 1.2em each), today on a `multiple` select alone, extends to the sized single select.
@task_item n=4 part=needs
The popup gate: a sized single select opens no list; the click, Enter or Space gestures that open a single select do not open one for it, so it joins the selects that open no list.
@task_item n=5 part=needs
Keys move the choice in the sized box (the single select's choose semantics: one chosen option, its input and change events), and a click on a row chooses it; how the key model and row click are shared with the `multiple` list box is the chunk's to settle.
@task_item n=6 part=needs
A single select's list events (open, close, highlight move, choose, choosing the chosen option, choosing a disabled row): which of them a sized single select fires, read against a `multiple` list box and HTML; a difference is a founder fork.
@task_item n=7 part=needs
The snapshot and accessibility tree of a select: role list box versus combo box, the chosen option, the highlighted row, the open attribute and `expanded` for a sized single select, matching a `multiple` list box's.
@task_item n=8 part=needs
The single select box that shows its chosen option: a sized single select is no longer that closed box; its painted face is the list box of rows instead.
@task_item n=9 part=needs
The chosen row in the sized box is marked with the same chosen-row colours a list box's chosen row takes (engine constants, Chromium's inactive selected pair).
@task_item n=10 part=needs
Driver readings of the sized single select (snapshot, diff, actions line, lint, layout's placing words, `type` on a select, screenshot of its rows) follow from the engine as a `multiple` list box's rows are real boxes; a reading that differs from the `multiple` list box or HTML is a founder fork.
@task_item n=11 part=needs
Keeping the chosen row in view when keys move the choice scrolls the box's rows instantly, with no animation.
@task_item n=12 part=needs
The founder's F8 is reversed in part: the `multiple` reading stands (a valid `size` that many rows tall, no `size` all rows), and HTML's default 4-row height for a `multiple` select is not adopted.
@task_item n=13 part=needs
An engine change for every document, remote pages included: the security plan's stated limit that a single select's `size` is not read changes and rides its ratification.
@task_item n=14 part=needs
The change crosses no new driver carrier, verb, argument, key name, result field, cause, session error, exit, log field or span unless the founder rules it.
@task_item n=15 part=needs
Checks beside the select interaction, select popup and stand act select and popup checks, in both layout modes: the sized single select's box height, rows, key moves, row click, no popup, and the drop-down kept for `size` 1, 0 or invalid.
@task_standing n=1
convention: each new engine test is built with system fonts
home: typography
@task_standing n=2
convention: each new driver output a test asserts is read by an agent, not a person: plain text, no colour
home: surface-cli
@task_item n=1 part=not_needed
The WPT runner's output and result colours: the chunk adds no runner output.
@task_item n=2 part=not_needed
The escher-session tasks, coverage and UI graph lines and their arguments: the chunk adds no task, coverage or graph reading.
@task_item n=3 part=not_needed
The shell provider's window controls, cursor icons and favicon: no window or cursor change.
@task_item n=4 part=not_needed
The viewport's color scheme, the dioxus-native base colour and alpha mode, and engine colour tokens: no colour-scheme or configuration change.
@task_item n=5 part=not_needed
Fonts, locale, type scale and the WASM font context: no text or font change.
@task_item n=6 part=not_needed
The JS window object, MountedData calls during a render and the web-spa canvas surface: untouched by the select's rendering.
@task_item n=7 part=not_needed
Animation, the virtual clock, smooth scroll-behavior and overlay scrollbar fade timings: the sized box has no motion of its own.
@task_item n=8 part=not_needed
Table cell spacing hints, font element hints and body margins: other presentational hints the chunk does not touch.
@task_item n=9 part=not_needed
Stacking-context paint levels and auto z-index: the sized box is in-flow and opens no overlay.
@task_item n=10 part=not_needed
Dependency and CI debts, and any canvas work: out of scope.
