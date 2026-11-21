@task_prose part=chunk
Builds a single select (no multiple) whose size reads a valid value above 1 as HTML renders it: an in-flow list box size rows tall, its rows the options and group headers, the chosen row marked, the rest scrolling.
Keys move the choice in the box, a click on a row chooses it, and no engine-painted popup list opens; size 1, 0 or an invalid value leave it a closed drop-down box with its popup.
Reverses in part the founder's F8: the clause that a single select's size stays unread and it is always a closed box and a list falls; the multiple select's size reading stands, with no default of 4 rows.
Closes the residual that a single select's size stays unread; re-verifies at take-up that residual and its route entry; any reading that differs from a multiple list box's or from HTML's goes to the founder as a fork.
@task_file n=1 path=packages/blitz-dom/src/node/select_list.rs mark=edits
what: the select list component: a select's size rows read through the unsigned-integer parser, capped at 65534, and the multiple select's row model — edits: size_rows
@task_file n=2 path=packages/blitz-dom/src/node/select.rs mark=edits
what: the select element component: its single-choice and multiple-choice state and options
@task_file n=3 path=packages/blitz-dom/src/document.rs mark=edits
what: the engine document: the gate on whether a select may open its popup list — edits: select_list_allowed
@task_file n=4 path=packages/blitz-dom/src/events/keyboard.rs mark=edits
what: the engine's keyboard handling: the single select's key model and the multiple select's key model — edits: press_select_key, press_multiple_select_key
@task_file n=5 path=packages/blitz-dom/src/events/pointer.rs mark=edits
what: the engine's pointer handling: a click on a select's row and the click that opens a single select's list — edits: click_select_row
@task_file n=6 path=packages/blitz-dom/src/stylo/presentational_hints.rs mark=edits
what: the presentational hints: a select's height of size times 1.2em, today on a multiple select alone
@task_file n=7 path=packages/blitz-dom/src/accessibility.rs mark=edits
what: the accessibility tree builder: a select's role, list box versus combo box, its expanded state and its options
@task_file n=8 path=packages/blitz-dom/assets/default.css mark=edits
what: the engine's UA stylesheet: a select's look as a list box or a drop-down box
@task_file n=9 path=packages/blitz-paint/src/render.rs mark=edits
what: the engine's painter: a single select's painted face and a list box's rows
@task_file n=10 path=packages/blitz-paint/src/render/select_list.rs
what: the painter of a select's list rows: the open popup list and a multiple select's rows
@task_file n=11 path=packages/escher-driver/src/json/snapshot.rs
what: the driver's snapshot reader: a select's expanded and highlighted readings, its rows and value
@task_file n=12 path=packages/escher-driver/src/lint.rs
what: the driver's lint reader: findings over a page's boxes, contrast included
@task_file n=13 path=packages/escher-driver/src/actions.rs
what: the driver's actions reader: the actionable nodes and their causes
@task_file n=14 path=packages/escher-driver/src/layout.rs
what: the driver's layout reader: its placing words for a page's boxes
@task_file n=15 path=tests/blitz-tests/tests/select_interaction.rs mark=edits
what: the select interaction checks: keys, clicks, input and change events on single and multiple selects, both layout modes
@task_file n=16 path=tests/blitz-tests/tests/select_popup.rs mark=edits
what: the select popup list checks: when a single select's list opens and how it behaves open
@task_file n=17 path=tests/blitz-tests/tests/stand_act_select.rs mark=edits
what: the stand's driver checks of acting on a select through type and click
@task_file n=18 path=tests/blitz-tests/tests/stand_act_popup.rs mark=edits
what: the stand's driver checks of acting on an open select list
@task_item n=1 part=needs
A single select with a valid size above 1 renders as an in-flow list box size rows tall, its options and group headers as rows, the chosen row marked, the rest scrolling; size 1, 0 or invalid keep the closed drop-down box.
@task_item n=2 part=needs
The single select's popup gate: a sized single select never opens the engine overlay list; the decision that a single select opens an engine overlay list now holds for an unsized single select only.
@task_item n=3 part=needs
Keys on a sized single select move the choice in the box (arrows, Home, End, typeahead), not the closed-list key model; each key change fires one input then one change through the one choice write path.
@task_item n=4 part=needs
A click on a row of a sized single select chooses that row and opens nothing; disabled rows and disabled optgroup rows stay unchoosable by the one disabled rule; a label click still focuses and opens nothing.
@task_item n=5 part=needs
The accessibility tree role of a sized single select: a list box as a multiple select's is, not a combo box, with its options as rows and their selected state; its expanded state follows from no list being open.
@task_item n=6 part=needs
The size reading of a select: parsed as an unsigned integer, capped at 65534, and the height hint of size times 1.2em extended from a multiple select to a sized single select; a multiple select's reading unchanged, with no HTML default of 4 rows.
@task_item n=7 part=needs
The founder's F8 partial reversal: its last clause falls, the rest stands, recorded as the founder's; F8's option B stays unchosen.
@task_item n=8 part=needs
Snapshot, snapshot diff, snapshot text state tokens and the snapshot's row bounds and value for a sized single select follow a multiple list box's: rows as real boxes, no highlighted open-list row, no expanded reading from a list; a reading that differs is a founder fork.
@task_item n=9 part=needs
Driver type on a sized single select, the actions answer, lint contrast and layout placing words read its rows as real boxes, as for a multiple select; the driver screenshot of an element or row reads them likewise.
@task_item n=10 part=needs
The look of a sized single select: the chosen-row colour pair and row heights as a multiple select's list box, painted in flow and not as the popup's engine constants; no motion.
@task_item n=11 part=needs
No new carrier, verb, argument, key name, result field, cause, session error, exit, log field or span crosses the driver unless the founder rules it.
@task_item n=12 part=needs
Every change is an engine change for every document, remote pages included, so it rides the security plan's ratification of the select rules' stated limits.
@task_item n=13 part=needs
Focus stays on the sized single select on key and click; Tab and Shift+Tab leave it as any focusable control, with no list to choose and close first.
@task_item n=14 part=needs
New checks of the sized single select run beside the select interaction, popup and stand act checks, in both layout modes, headless.
@task_standing n=1
convention: each new accessibility assertion is a Rust test over the AccessKit tree, its expected roles and names stated once in the shared tables
home: 3-a11y-assertion-harness-contract
@task_standing n=2
convention: select list coverage runs headless and each new stand check is repeated three rounds in the accessibility test leg
home: keyboard-navigation · ci-integration
@task_standing n=3
convention: a new stand fixture reads no lint finding at boot and keeps SC 1.4.3 contrast asserted on the stand
home: visual-design-verification
@task_item n=1 part=not_needed
Range input keys, pointerdown and click rows: the chunk touches no range input.
@task_item n=2 part=not_needed
The seven_guis Cells grid and CRUD list keyboard rows: no select reaches them.
@task_item n=3 part=not_needed
Password masking, password caret and file input value rows: no text or file input is touched.
@task_item n=4 part=not_needed
The Dioxus document reached-handler record, buffered Dioxus render and UI graph state registration rows: the chunk changes no Dioxus or UI graph state path.
@task_item n=5 part=not_needed
The accessibility feature defaults, AccessKit and accesskit_xplat adapter choice, platform adapter absence and tier-0 reach rows: the chunk adds no feature or adapter.
@task_item n=6 part=not_needed
Hidden, inert and display-none content exclusion rows: the chunk changes no hiding rule.
@task_item n=7 part=not_needed
Checkbox, radio and toggle state rows, aria-pressed, aria-checked and switch roles: no checkable control is touched.
@task_item n=8 part=not_needed
Details and dialog :open matching, scheme and zoom device change, hidpi reshaping and scrollbar thumb contrast rows: none is changed.
@task_item n=9 part=not_needed
Invalid date state and cognitive accessibility rows: no date or invalid state is touched.
