@task_prose part=session
The session closed the select list's debts: the open list's highlighted row is now read at its painted pair, rows take the tallest row's height, the highlight is read in the tree and the snapshot, and Cells' current cell states selected.
The accessibility lint reads the highlighted row of an open list at the engine's highlight text on highlight background through a new read-only harness reader; plain rows still read their own computed pair (highlight 12.56, plain 2.32 measured).
Every row of an open single select's list now takes the tallest row's line height, the select's own one-line height as the floor, so a larger authored font is no longer clipped; the rows keep one height.
While a single select's list stands open, the accessibility tree names the highlighted option as the select's active descendant, focus stays on the select, and a page's aria-activedescendant is still unread; the snapshot gains a node key highlighted, a closed bool written only when true, and node keys move from fourteen to fifteen, the readings from twelve to thirteen.
A highlight move to another row, by key or pointer, now marks the select changed so a press diff names highlighted on both rows; a move to the same row or a wheel move inside the list marks nothing.
Retired: the limit that the highlight is unread and the clause that a row with a larger authored font clips; a single select's size attribute stays unrendered as a list box, routed to its own entry by the founder's fork.
@task_file n=1 path=packages/blitz-test-harness/src/contrast.rs
what: edited — the test harness's text contrast readers; gains the highlight contrast reader for the open list's highlighted row
@task_file n=2 path=packages/escher-driver/src/lint.rs
what: edited — the driver's accessibility lint and its contrast check, which now reads the highlighted row's painted pair first
@task_file n=3 path=packages/blitz-dom/src/document.rs
what: edited — the engine document's open select list overlay: its row measuring (row height) and the key-driven highlight move
@task_file n=4 path=packages/blitz-dom/src/events/pointer.rs
what: edited — pointer events over the open select list, the hover that moves the highlight between rows
@task_file n=5 path=packages/blitz-dom/src/accessibility.rs
what: edited — the accessibility tree builder of a select, now naming the highlighted option as the active descendant while open
@task_file n=6 path=packages/dioxus-native-dom/src/snapshot.rs
what: edited — the snapshot's node state, gaining the highlighted field read from the tree
@task_file n=7 path=packages/dioxus-native-dom/src/snapshot_text.rs
what: edited — the snapshot text writer and its state tokens, gaining the highlighted token after clickable
@task_file n=8 path=packages/dioxus-native-dom/src/snapshot_diff.rs
what: edited — the snapshot diff and its list of readings, gaining the highlighted reading before bounds
@task_file n=9 path=packages/escher-driver/src/schema.rs
what: edited — the driver's schema table of node keys (now fifteen) and its help strings for snapshot text and acting changed
@task_file n=10 path=packages/escher-driver/src/schema_tests.rs
what: edited — checks of the driver's schema table and the node key count
@task_file n=11 path=packages/escher-driver/src/json/snapshot.rs
what: edited — the snapshot's JSON node writer, writing highlighted true only when true
@task_file n=12 path=packages/escher-driver/src/json/diff.rs
what: edited — the diff's JSON node writer for changed and added nodes
@task_file n=13 path=packages/escher-driver/src/json_tests.rs
what: edited — checks of the driver's JSON snapshot and diff output and the readings list
@task_file n=14 path=packages/escher-driver/src/describe.rs
what: edited — the driver's help and describe answer, its node_keys list and counts
@task_file n=15 path=examples/seven_guis/src/tasks/cells.rs
what: edited — the Cells task of the seven_guis stand, whose current data cell now states aria-selected
@task_file n=16 path=tests/blitz-tests/tests/select_popup.rs
what: edited — checks of the open select list overlay: row height, the tree's active descendant, highlight marks
@task_file n=17 path=tests/blitz-tests/tests/select_popup_painted.rs
what: edited — painted checks of the open select list: the highlight pair pinned to the chosen row pair, the highlight reader, a larger font painted whole inside its row
@task_file n=18 path=tests/blitz-tests/tests/stand_lint.rs
what: edited — checks of the driver's accessibility lint on the stand, the open list's highlighted row read at its painted pair
@task_file n=19 path=tests/blitz-tests/tests/stand_act_popup.rs
what: edited — checks of the driver's acting verbs on an open select list, a press returning the highlight move in its diff
@task_file n=20 path=tests/blitz-tests/tests/stand_cells_served.rs
what: edited — checks of the served Cells task, the current cell alone reading selected and an arrow moving it
@task_file n=21 path=tests/blitz-tests/tests/stand_cross_os_readings.rs
what: edited — the cross-OS readings check, Cells' re-pinned digests
@task_file n=22 path=tests/blitz-tests/tests/stand_diff.rs
what: edited — the snapshot diff check of the stand, its per-reading comparisons in readings order
@task_file n=23 path=docs/capabilities.md
what: edited — the capability statement: the fifteen node keys, the select list's limits, How the highlight is read replacing The highlight is unread
@task_item n=1 part=touches
contract/select-accessibility/highlighted-row: the highlighted row is now read, as the tree's active descendant while the list is open and as the snapshot's highlighted token on that one row; any cell saying the highlight is unread or absent from the snapshot is stale.
@task_item n=2 part=touches
decision/select-list-colours-engine-constants: the highlight text and background constants stay engine constants, now pinned by a check to the default sheet's chosen row pair (black on c8c8c8), and the lint reads them as the highlighted row's painted pair.
@task_item n=3 part=touches
decision/select-chosen-row-chromium-pair: the open list's highlight is pinned to share the default sheet's chosen row pair; a row naming the chosen row pair gains that tie.
@task_item n=4 part=touches
contract/select-list-events/highlight-move: a highlight move to another row by key or pointer now marks the select changed so the acting diff names highlighted on both rows; a same-row move and a wheel move mark nothing.
@task_item n=5 part=touches
contract/select-accessibility/open-select: while a single select's list is open the tree names the highlighted option as the select's active descendant, none when closed, focus staying on the select.
@task_item n=6 part=touches
contract/snapshot/select-list-row-bounds: the open list's rows in the snapshot now carry the highlighted state on the highlighted row alone, and the arrow on Cells diffs as selected and bounds.
@task_item n=7 part=touches
contract/screenshot/select-list-row and decision/select-list-engine-overlay: every row of the open list takes the tallest row's line height (a 700 weight header included), the select's one-line height its floor, so a larger font paints whole inside its row; the clause that a larger authored font clips is retired.
@task_item n=8 part=touches
decision/select-list-placed-in-viewport: indirect, through the row height; a list with a larger font is now taller, which the placement inside the viewport bounds.
@task_item n=9 part=touches
decision/select-list-wheel-and-key-scroll: key moves of the highlight now mark the select changed while wheel scroll inside the list still marks nothing.
@task_item n=10 part=touches
contract/escher-session-commands/snapshot and contract/escher-session-mcp/snapshot-result: the snapshot text gains the bare token highlighted after clickable and the JSON node writes highlighted true only when true.
@task_item n=11 part=touches
contract/escher-session-commands/help and contract/escher-session-mcp/help-result: help's node_keys count moves from fourteen to fifteen, and the help strings for snapshot text and acting changed name highlighted; MCP tool descriptions read from the same table.
@task_item n=12 part=touches
contract/escher-session-tasks/cells: Cells' current data cell alone now states aria-selected true and an arrow moves it; its cross-OS digests are re-pinned.
@task_item n=13 part=touches
decision/selects-that-open-no-list: a single select's size attribute stays unread and unrendered as a list box by the founder's fork, now owed to its own route entry.
@task_item n=14 part=touches
decision/engine-colours-hardcoded and absence/engine-color-tokens: indirect, the highlight colours stay hardcoded engine constants, no token added; only their pin to the chosen row pair and their read by the lint are new.
@task_item n=15 part=touches
decision/escher-session-agent-read-surface and decision/driver-output-for-agents: indirect, the agent-read snapshot and diff carriers gain one closed bool key, with no new carrier, verb, argument, exit or session error.
@task_fact n=1 slug=surface
fact: While a single select's list stands open, the accessibility tree names the highlighted option as the select's active descendant and keeps focus on the select; a page's aria-activedescendant is not read.
@task_fact n=2 slug=surface
fact: The snapshot's node key highlighted is a closed bool, a bare token after clickable in the text and true in JSON, written only on the open list's highlighted row; node keys are fifteen and the diff's readings thirteen.
@task_fact n=3 slug=surface
fact: A highlight move to another row of an open list, by key or pointer, marks its select changed; a move to the same row or a wheel move inside the list marks nothing.
@task_fact n=4 slug=spacing
fact: Every row of an open select list takes one height, the tallest row's line (headers at weight 700 included) with the select's own one-line height as the floor, plus the row padding, so no authored font clips.
@task_fact n=5 slug=color-palette
fact: The accessibility lint reads an open list's highlighted row at the engine's painted highlight pair, which shares the default sheet's chosen row pair, and reads plain rows at their own computed pair.
@task_item n=1 part=untouched
Typography's font family, fallback, system fonts, WASM font context, lang and root-relative unit rows: the session read rows' existing font sizes, it loaded or resolved no font.
@task_item n=2 part=untouched
Motion's virtual clock, advance, animation, smooth scroll and overlay scrollbar fade rows, and the list opening without motion: no clock or animation behaviour moved.
@task_item n=3 part=untouched
Depth strategy's paint order and hit-testing rows, including the open list answering hits first and being painted after the root: unchanged.
@task_item n=4 part=untouched
escher-session exit statuses and exit codes, stderr, usage and session errors, remedies and the MCP err ending: no exit, cause or error added.
@task_item n=5 part=untouched
The actions, coverage, UI graph and layout lines and their arguments, and the screenshot refusals: no change to those answers.
@task_item n=6 part=untouched
Iconography and cursor rows, the favicon, the shell provider's window controls, the JS window sizes and the WPT runner's output and result colours: untouched.
@task_item n=7 part=untouched
Other escher-session tasks (counter, crud, flight-booker, load, order, timer) and the select list's open, close and choose events: their behaviour is unchanged; the Counter checks replay byte-equal.
