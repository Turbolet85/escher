@task_prose part=session
The engine now reads a single select's size in every document: a single select whose size parses above 1 is an in-flow list box of its rows, opens no popup list, and is chosen by its arrow, Home, End and typeahead keys and by a click on a row; Enter and Space open nothing on it.
A list box reads the ListBox role with no expanded state, may stand with nothing chosen, and paints its chosen row with the chosen-row pair black on #c8c8c8 (12.56:1 measured); display size 1 (size 1, 0, non-numeric, negative, empty or absent) keeps the drop-down.
A size write moves a select between list box and drop-down and closes an open list; the bridge's closed-select reading answers none for a list box, so a click on its row is accepted, and the driver's click and press help texts are re-worded with no verb, argument, cause, field or exit added.
The stated limit that a single select's size is unread is retired from the capability statement; the select interaction checks grew 18 to 24, the popup checks 27 to 28 and the stand select checks 11 to 17, every new case run in both layout modes.
Measured: the boot lint of a page with a scrolling size-3 list box reads 4 findings (its scrolled-out rows clipped, truncated, off-screen), not 0; a node a restyle turns display none keeps its last client rect; the popup gate's first control did not hold until its case was widened.
@task_file n=1 path=packages/blitz-dom/src/node/select.rs
what: the engine's select model: display size, list box versus drop-down readings, the settle of the chosen option, stepping and typeahead
@task_file n=2 path=packages/blitz-dom/src/node/select_list.rs
what: the engine's popup select list and the gate deciding whether a select may open one
@task_file n=3 path=packages/blitz-dom/src/stylo/presentational_hints.rs
what: the engine's presentational hints: the select size hint and the list box rows' and chosen row's colour hints
@task_file n=4 path=packages/blitz-dom/src/mutator.rs
what: the engine's DOM mutator: attribute writes and the restyle a size or multiple write triggers
@task_file n=5 path=packages/blitz-dom/src/document.rs
what: the engine's base document: the public list-box reading and the select choice settle
@task_file n=6 path=packages/blitz-dom/src/events/keyboard.rs
what: the engine's keyboard dispatch: select keys, the new list box key arm
@task_file n=7 path=packages/blitz-dom/src/events/pointer.rs
what: the engine's pointer dispatch: select and option clicks, the new list box row click
@task_file n=8 path=packages/blitz-dom/src/accessibility.rs
what: the engine's accessibility tree: the select role and expanded state
@task_file n=9 path=packages/blitz-dom/src/layout_reason.rs
what: the layout reader's placing words for select options
@task_file n=10 path=packages/dioxus-native-dom/src/snapshot.rs
what: the Dioxus bridge's snapshot builder and its closed-select reading behind the covered refusal
@task_file n=11 path=packages/escher-driver/src/schema.rs
what: the driver schema: the verb table and the click and press help texts
@task_file n=12 path=packages/escher-driver/src/schema_tests.rs
what: the driver schema's unit tests over the help texts
@task_file n=13 path=docs/capabilities.md
what: the capability statement: the select model and its stated limits, held to the schema
@task_file n=14 path=tests/blitz-tests/tests/select_interaction.rs
what: the select interaction integration check target, both layout modes
@task_file n=15 path=tests/blitz-tests/tests/select_popup.rs
what: the select popup list integration check target
@task_file n=16 path=tests/blitz-tests/tests/stand_act_select.rs
what: the stand's driver checks on selects: snapshot, click, actions, press, type and lint
@task_item n=1 part=touches
The chosen select row colour pair decision: the pair black on #c8c8c8 now also paints a single list box's chosen row, not only a multiple select's and the popup list's.
@task_item n=2 part=touches
The type-on-a-select contract of the driver's run: type on a list box chooses by the text's start with no list open.
@task_item n=3 part=touches
The covered refusal cause and the snapshot node clickable item: the bridge's closed-select reading answers none for a list box, so a click on a list box row off its centre is accepted and reads no covered.
@task_item n=4 part=touches
The driver's actions verb contract: a list box's rows are listed in view with no cause.
@task_item n=5 part=touches
The acting call result and the driver's act contract: a click on a list box row returns a diff naming the select's value and the rows' selected.
@task_item n=6 part=touches
The CLI help output and MCP help parity rows: the click and press help texts are re-worded, verb, argument, field, cause and count unchanged.
@task_item n=7 part=touches
The lint defect rows clipped, truncated and off-screen: a scrolling list box's hidden rows read as those findings at boot, and a stand check pins them with no overlap.
@task_item n=8 part=touches
The select list painted by the scene painter decision: the popup list now opens on a drop-down alone; a list box paints in flow and opens none.
@task_item n=9 part=touches
The layout reader words decision: an option of a single list box reads its own placing as a multiple select's does; no placing word is added.
@task_item n=10 part=touches
The capability statement pinned to the driver schema and capability claims naming their checks: the select limit is retired and the list box claim stands with its checks, counts unmoved.
@task_item n=11 part=touches
The stand checks and layout assertions in both layout modes decisions: every new list box case runs incremental and full layout.
@task_item n=12 part=touches
The controlled red-before-green decision: the popup gate's control did not hold on its first case and was widened until the guard's own reading showed.
@task_item n=13 part=touches
The expected value pinned as a literal decision: the list box's scroll offsets and the boot lint findings were measured then pinned as literals.
@task_item n=14 part=touches
The harness layout rect inspection contract: a node a restyle turns display none keeps its last client rect, so a box check reads the computed display beside it.
@task_item n=15 part=touches
The scrolled box reads where it stands decision: list box rows scrolled by keys and typeahead read their scrolled bounds.
@task_item n=16 part=touches
The oversize test targets kept decision and the a11y leg membership: the three select check targets grew by 415, 80 and 230 lines and stay in their legs.
@task_fact n=1 slug=3-test-harness-contract
fact: A node a restyle turns display none keeps the client rect it last laid out at, so a check asking whether a node has a box reads its computed display beside the rect.
@task_fact n=2 slug=integration-test-strategy
fact: The boot lint of a page holding a scrolling list box reads its scrolled-out rows as clipped, truncated or off-screen findings; a check over such a page pins those findings rather than zero.
@task_fact n=3 slug=integration-test-strategy
fact: A check of the covered refusal through the bridge clicks a list box row off the select's centre, since a click on the centre row lands on that row and is accepted either way.
@task_fact n=4 slug=unit-test-strategy
fact: A control's named case can be insensitive to its guard where other arms route around it; the case is widened until the guard's own reading shows, as the popup gate's case writes size on a page-fixed drop-down whose box the write does not move.
@task_fact n=5 slug=test-data-fixtures
fact: A Dioxus fixture's selected literal is re-written on any re-render of its component, so a fixture select whose choice a check moves is a controlled select.
@task_item n=1 part=untouched
The WPT runner, its CLI, statuses, outputs and conformance decisions: no conformance suite or expectation moved.
@task_item n=2 part=untouched
The cold-agent pipe and agent-run contracts, events, verdicts and exit codes: no harness or event change.
@task_item n=3 part=untouched
The session wire, session host, session errors, recording and state directory contracts: no protocol, error, socket or recording change.
@task_item n=4 part=untouched
CI integration, coverage floors, mutation testing and package install rows: no CI leg, job, floor or gate changed.
@task_item n=5 part=untouched
The escher-session exit codes and refusal cause set and order: no cause, exit or session error added or reordered.
@task_item n=6 part=untouched
The virtual clock, timer, advance, settle and mocking discipline rows: no time or stand-in change.
@task_item n=7 part=untouched
The handler coverage, graph and committed recorded check rows: no listener, graph or recording change.
