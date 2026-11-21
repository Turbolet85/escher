@task_prose part=chunk
A single select (no `multiple`) whose `size` reads a valid value above 1 now shows as HTML renders it: an in-flow list box `size` rows tall, its rows the options and group headers, the chosen row marked, the rest scrolling.
Keys move the choice inside the box, a click on a row chooses it, and no popup list opens; `size` of 1, 0 or an invalid value leaves it the closed drop-down box with its popup.
The founder's F8 is reversed in part: its clause that a single select's `size` stays unread falls; its `multiple` reading stands (no 4-row default adopted).
It closes residual res-ysz67imk, re-verified at take-up, and is an engine change for every document, remote pages included.
Snapshot, diff, accessibility tree, lint, actions, layout and screenshot readings of the sized single select follow from the engine, like a `multiple` list box's.
@task_file n=1 path=packages/blitz-dom/src/node/select_list.rs mark=edits
what: the select list component: a select's options and rows and its `size` reading — edits: size_rows and the list-box rows for a single select
@task_file n=2 path=packages/blitz-dom/src/node/select.rs mark=edits
what: the select element's state and its single and multiple choice model
@task_file n=3 path=packages/blitz-dom/src/document.rs mark=edits
what: the engine document: the gate that lets a select open its popup list — edits: select_list_allowed
@task_file n=4 path=packages/blitz-dom/src/events/keyboard.rs mark=edits
what: keyboard default actions of the engine — edits: the single select's key model (press_select_key) and the list box's (press_multiple_select_key)
@task_file n=5 path=packages/blitz-dom/src/events/pointer.rs mark=edits
what: pointer default actions of the engine — edits: the row click of a list box (click_select_row)
@task_file n=6 path=packages/blitz-dom/src/stylo/presentational_hints.rs mark=edits
what: presentational hints mapped from attributes to style — edits: the select `size` height hint of size times 1.2em, today on `multiple` alone
@task_file n=7 path=packages/blitz-dom/src/accessibility.rs mark=edits
what: the accessibility tree the engine builds — edits: a select's role, list box versus combo box, and its expanded state
@task_file n=8 path=packages/blitz-dom/assets/default.css mark=edits
what: the engine's user-agent stylesheet — edits: the look of a list box select
@task_file n=9 path=packages/blitz-paint/src/render/select_list.rs mark=edits
what: the scene painter's select list painting: a single select's painted face and a list box's rows
@task_file n=10 path=packages/escher-driver/src/json/snapshot.rs
what: the driver's snapshot reader of a node: a select's expanded and highlighted states
@task_file n=11 path=packages/escher-driver/src/lint.rs
what: the driver's lint reader of on-screen defects
@task_file n=12 path=packages/escher-driver/src/actions.rs
what: the driver's actions reader of what an element offers to act on
@task_file n=13 path=packages/escher-driver/src/layout.rs
what: the driver's layout reader and its placing words
@task_file n=14 path=tests/blitz-tests/tests/select_interaction.rs mark=edits
what: the engine's select interaction checks, keys and clicks on a select, beside which the sized single select's checks stand
@task_file n=15 path=tests/blitz-tests/tests/select_popup.rs mark=edits
what: the engine's select popup checks: when a select's popup list opens and closes
@task_file n=16 path=tests/blitz-tests/tests/stand_act_select.rs mark=edits
what: the stand checks of acting on a select through the driver session, in both layout modes
@task_file n=17 path=tests/blitz-tests/tests/stand_act_popup.rs mark=edits
what: the stand checks of acting on a select's popup list through the driver session, in both layout modes
@task_item n=1 part=needs
A single select with a valid `size` above 1 lays out as an in-flow list box `size` rows tall (height size times 1.2em), its rows the options and group headers, the chosen row marked, the rows past `size` scrolling.
@task_item n=2 part=needs
The popup gate: a sized single select opens no popup list on click or key; a single select with `size` 1, 0, an invalid value or none stays a closed drop-down box that opens its popup as before.
@task_item n=3 part=needs
Keys on a focused sized single select move the one chosen row inside the box (and scroll it into view), firing input and change from the engine alone, the change never synthesized by the driver.
@task_item n=4 part=needs
A click on a row of a sized single select chooses that row alone, the single-choice model held (no multi-choice, no toggle).
@task_item n=5 part=needs
`size` is read through the existing unsigned-integer parse capped at 65534; the F8 `multiple` reading stands: a valid `size` that many rows tall, no `size` all rows, HTML's 4-row default not adopted.
@task_item n=6 part=needs
Accessibility tree of a sized single select: a list box role, not a combo box, with no expanded state, its rows options with the chosen one selected, read like a `multiple` list box's.
@task_item n=7 part=needs
Snapshot and diff of a sized single select: its expanded and highlighted states and clickable rows read as a list box's do; a reading that differs from a `multiple` list box's or from HTML's is a founder fork.
@task_item n=8 part=needs
The driver's type on a select, its actions answer, lint findings (clipped, off-screen, overlap, truncated, zero-size, contrast) and layout placing words over a sized single select follow the engine's list box boxes.
@task_item n=9 part=needs
Paint: a sized single select's face is painted as a list box, its chosen row in the chosen select row colour pair, the select list painted by the scene painter alone; a never-scrolled list box paints no scrollbar thumb.
@task_item n=10 part=needs
A scrolled list box reads where it stands: a row scrolled out of the box is clipped or off-screen to the reader, and a click on a clipped row follows the harness's clipped-click rule.
@task_item n=11 part=needs
The change crosses no new carrier, verb, argument, key name, result field, refusal cause, session error, exit code, log field or span; the driver's verb table, refusal cause set and session wire stay as they are.
@task_item n=12 part=needs
An engine change reaching every document, remote pages included: the security plan's stated select limit (a single select's `size` not read) is changed and rides its ratification.
@task_item n=13 part=needs
Checks of the sized single select run in both layout modes, through the harness on the bundled font, beside the existing select interaction, popup and stand act checks.
@task_item n=14 part=needs
The unsized single select's popup list keeps its behaviour, and the `multiple` list box's key model and row click keep theirs, held by the existing select and popup checks.
@task_item n=15 part=needs
Residual res-ysz67imk is closed by this chunk; F8's reversed clause and what of it stands are quoted in the founder forks file, with every founder fork's measurement and options.
@task_standing n=1
convention: each new engine or stand check stands in its own integration test target, one target per file, shared fixtures and stand tables in their one shared module, never a target of their own
home: test-scope-summary · test-strategy
@task_standing n=2
convention: each new stand or layout check runs in both layout modes and inside stand-repeat, with a zero-failure budget
home: test-scope-summary · integration-test-strategy
@task_standing n=3
convention: each new test is controlled red-before-green, pins its expected value as a literal, prints no argument value on failure, and a table-driven test asserts its row count
home: unit-test-strategy
@task_standing n=4
convention: each new unit test sits in an inline test module of its source file
home: 4-unit-test-strategy · unit-test-strategy
@task_standing n=5
convention: each new engine test is harness-driven on the bundled font, never skipping, with no sleep or wall-clock read, no screenshot step in a committed check
home: test-strategy · integration-test-strategy
@task_standing n=6
convention: each new fixture page is test data stated in code, built offline, holding no private data
home: 7-test-data-fixtures · test-data-fixtures
@task_standing n=7
convention: each new regression test is named for and documents the bug or residual it closes
home: 2-test-strategy
@task_item n=1 part=not_needed
Cold-agent pipe, agent-run verbs, events, selections and exit codes: the chunk adds no agent run and changes no harness script.
@task_item n=2 part=not_needed
WPT runner CLI, outputs, status set, pinned commit and reftest fonts: no WPT run or expectation changes here.
@task_item n=3 part=not_needed
Virtual clock, advance, timer ticks and the stand timer: a list box's choice moves with no time step.
@task_item n=4 part=not_needed
Handler coverage, the coverage line and role, and the UI graph's caused owners and registration: no listener or graph reading changes.
@task_item n=5 part=not_needed
MCP role protocol revision, tools/list order, help parity and recording parity: no MCP surface changes.
@task_item n=6 part=not_needed
Typed-password masking and the password caret: the chunk touches no password field.
@task_item n=7 part=not_needed
apt-install script exit codes, CI actions pinning, CI secrets and permissions, and the dependency audit: dependency and CI debts are out of scope.
@task_item n=8 part=not_needed
Mocking library absence and network stand-ins, the load page stand-in and load waits: no network or load is involved.
