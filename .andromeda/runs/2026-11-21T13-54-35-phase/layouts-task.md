@task_prose part=chunk
A single select (no `multiple`) whose `size` reads a valid value above 1 is shown as HTML renders it: an in-flow list box `size` rows tall, its rows the options and group headers, the chosen row marked, the rest scrolling.
Keys move the choice in the box, a click on a row chooses it, and no engine-painted popup list opens for it; residual res-ysz67imk closes.
The founder's F8 falls in its last clause only (a single select's `size` stays unread); its `multiple` reading stands as built, HTML's default of 4 rows not adopted.
Re-verifies at take-up the residual's text and its route entry; an unsized, `size="1"`, `size="0"` or invalid-sized single select stays a closed drop-down box with its popup.
@task_file n=1 path=packages/blitz-dom/src/node/select_list.rs mark=edits
what: the select's list model: the `size` reading in rows (parsed as an unsigned integer, capped at 65534) — edits: size_rows
@task_file n=2 path=packages/blitz-dom/src/node/select.rs mark=edits
what: the select control's node: its key model for a single select and for a multiple select and its row click — edits: press_select_key, press_multiple_select_key, click_select_row
@task_file n=3 path=packages/blitz-dom/src/document.rs mark=edits
what: the document's popup gate deciding whether a select opens the engine-painted popup list — edits: select_list_allowed
@task_file n=4 path=packages/blitz-dom/src/events/keyboard.rs mark=edits
what: the engine's keyboard event dispatch that routes keys to a focused select
@task_file n=5 path=packages/blitz-dom/src/events/pointer.rs mark=edits
what: the engine's pointer event dispatch that routes a click to a select's face or row
@task_file n=6 path=packages/blitz-dom/src/stylo/presentational_hints.rs mark=edits
what: the presentational hints that turn HTML attributes into style — edits: the select `size` height hint (size rows of 1.2em, today on a multiple select alone)
@task_file n=7 path=packages/blitz-dom/src/accessibility.rs mark=edits
what: the accessibility tree builder: a select's role, list box versus combo box, and its states
@task_file n=8 path=tests/blitz-tests/tests/select_interaction.rs mark=edits
what: the select interaction checks (keys and clicks on a select), in both layout modes
@task_file n=9 path=tests/blitz-tests/tests/select_popup.rs mark=edits
what: the select popup list checks: which select opens the popup and how its rows are placed
@task_file n=10 path=tests/blitz-tests/tests/stand_act_select.rs mark=edits
what: the stand checks that act on a select through the driver
@task_file n=11 path=tests/blitz-tests/tests/stand_act_popup.rs mark=edits
what: the stand checks that act on an open popup list through the driver
@task_item n=1 part=needs
A sized single select renders as an in-flow list box `size` rows tall whose rows are real boxes, like a multiple select's; snapshot, diff, accessibility tree, lint, actions, layout and screenshot readings of it follow from the engine, and a reading that would differ from a multiple list box's or from HTML's is a founder fork.
@task_item n=2 part=needs
No popup opens for a sized single select: the popup gate refuses it, and the open-list rows that read their own bounds exist only for an unsized single select.
@task_item n=3 part=needs
Keys move the single choice within the list box (one chosen row, never several), scrolling the chosen row into view inside the box; a click on a row chooses it; change and input events fire as for a choice made in the popup.
@task_item n=4 part=needs
A select whose `size` is absent, 1, 0 or invalid, and every select already on the stand or a served task without a `size` above 1, keeps its closed drop-down face and popup, so no existing page moves; the flight-type select's boot reading of its two options stays as it is.
@task_item n=5 part=needs
Any stand or served-task control that is a single select carrying a `size` above 1 changes face to a list box under this chunk; its keyboard region, its ids, its lint at boot and its accessibility names and states are re-checked against the decisions that rule that control.
@task_item n=6 part=needs
The accessibility role of a sized single select is a list box, not a combo box, with no expanded state; the stand's names, roles and states still come from attributes alone.
@task_item n=7 part=needs
The driver's readers (snapshot `expanded` and `highlighted`, `type` on a select, lint, actions, layout's placing words, screenshot) read the list box through the same session answers: the change crosses no new carrier, verb, argument, key name, result field, cause, session error, exit code, log field or span unless the founder rules it.
@task_item n=8 part=needs
Typing into a select through the session's `type` and choosing through actions must reach the list box's rows without opening a popup; an off-screen row is brought into view through the scrollers that hold it and read through the clipping boxes' visible region.
@task_item n=9 part=needs
The list box's look comes from the UA stylesheet and the painted face of the select in the paint package; a sized single select is painted with no drop-down arrow, like a multiple list box.
@task_item n=10 part=needs
It is an engine change for every document, remote pages included: it rides the security plan's select rules' ratification, whose stated limit that a single select's `size` is not read falls.
@task_item n=11 part=needs
Every fork that turns what an agent or a page sees beyond HTML's own rendering of a sized single select, and F8's partial reversal, goes to the founder in founder-forks.md with its measurement and every option's cost; technical forks are the operator's.
@task_item n=12 part=needs
New checks of the sized single select (rendering height, key moves, row click, no popup, role, driver readings) sit beside the existing select checks and run in both layout modes, through the headless stand's pinned viewport where they go through the stand.
@task_item n=13 part=needs
A session recording replayed as a check must still replay with the same steps for any page whose selects carry no `size` above 1.
@task_item n=1 part=not_needed
The shell provider's window chrome controls (drag, maximize, minimize, close, decorations): the chunk touches no window chrome.
@task_item n=2 part=not_needed
The agent-run.sh test contract and the cold-agent.sh run pipe, and the cold agent's prompt, pipe and MCP decisions: the chunk adds no script, prompt or pipe.
@task_item n=3 part=not_needed
The multi_doc and paint_bench command lines: no benchmark argument, output or environment variable changes.
@task_item n=4 part=not_needed
The load page's network provider and the order page's controls built to cause: no network or fault behaviour changes.
@task_item n=5 part=not_needed
The session's coverage role, graph, help, MCP protocol revisions, stdio and recording-directory privacy rows: nothing of the session's roles, protocol or record files changes.
@task_item n=6 part=not_needed
The stand's accent colour, its fixed document skeleton with a main mount, and the held instance reading as a fresh boot: the stand's page structure and boot stay as they are.
@task_item n=7 part=not_needed
The absence of arbitrary index.html templates, and the mobile-native and web-spa surfaces: no template, mobile layout or SPA surface is reached.
