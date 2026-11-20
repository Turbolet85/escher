@task_prose part=session
The session closed the select list's and the selection's debts on the founder's forks (F1 A, F2 A, F3 A, F4 C, F5 A).
The lint now reads an open list's highlighted row at its painted pair through a new read-only harness reader, the highlight reader; the highlight pair is pinned to the default sheet's chosen-row pair, black on #c8c8c8, measured 12.56 against plain rows at 2.32.
Open list rows now take the tallest row's height, with the select's one-line height as the floor, so a larger authored font paints whole inside its row; the clause that such a row clips is retired.
While a single select's list is open the accessibility tree names the highlighted option as the select's active descendant, focus staying on the select; the snapshot reads it as a new closed bool node key, highlighted, so node keys go 14 to 15, readings 12 to 13, snapshot node state fields 8 to 9.
A highlight move to another row, by key or by pointer, now marks the select in the engine's changed set; a move to the same row and a wheel move inside the list mark nothing.
On the stand, the Cells grid's current data cell alone states aria-selected, and an arrow moves it; the limit that the highlight is unread is retired from the capability statement.
@task_file n=1 path=packages/blitz-test-harness/src/contrast.rs
what: edited — the harness contrast reader, the one contrast formula, now with the highlight reader for an open list's highlighted row
@task_file n=2 path=packages/escher-driver/src/lint.rs
what: edited — the visual lint and its contrast check, now reading the highlight reader first and the text reader elsewhere
@task_file n=3 path=packages/blitz-dom/src/document.rs
what: edited — the engine document: the open select list's row height and the highlight move that marks the changed set
@task_file n=4 path=packages/blitz-dom/src/events/pointer.rs
what: edited — pointer handling: hovering an open list's row moves the highlight and marks the select
@task_file n=5 path=packages/blitz-dom/src/accessibility.rs
what: edited — the accessibility tree builder, now naming an open single select's highlighted option as its active descendant
@task_file n=6 path=packages/dioxus-native-dom/src/snapshot.rs
what: edited — the snapshot model and its node state, now with the highlighted reading taken from the tree
@task_file n=7 path=packages/dioxus-native-dom/src/snapshot_text.rs
what: edited — the snapshot text form and its state tokens, now with the highlighted token after clickable
@task_file n=8 path=packages/dioxus-native-dom/src/snapshot_diff.rs
what: edited — the snapshot diff and its reading list, now with the Highlighted reading before Bounds
@task_file n=9 path=packages/escher-driver/src/schema.rs
what: edited — the driver schema's node-key table, now fifteen keys with highlighted after clickable
@task_file n=10 path=packages/escher-driver/src/schema_tests.rs
what: edited — the driver schema's checks, the node-key count
@task_file n=11 path=packages/escher-driver/src/json/snapshot.rs
what: edited — the snapshot's JSON carrier, writing highlighted true only when true
@task_file n=12 path=packages/escher-driver/src/json/diff.rs
what: edited — the snapshot diff's JSON carrier for changed and added nodes
@task_file n=13 path=packages/escher-driver/src/json_tests.rs
what: edited — the driver's JSON carrier checks over the reading list
@task_file n=14 path=packages/escher-driver/src/describe.rs
what: edited — the driver's help description and its node-key counts
@task_file n=15 path=examples/seven_guis/src/tasks/cells.rs
what: edited — the stand's seven_guis Cells grid, its current data cell now stating aria-selected
@task_file n=16 path=tests/blitz-tests/tests/select_popup.rs
what: edited — the single select popup checks: the row height and the active descendant while open
@task_file n=17 path=tests/blitz-tests/tests/select_popup_painted.rs
what: edited — the select list's painted checks: the highlight pair, the highlight reader and a larger font's glyphs inside their row
@task_file n=18 path=tests/blitz-tests/tests/stand_lint.rs
what: edited — the stand's lint checks: an open list's highlighted row read at its painted pair
@task_file n=19 path=tests/blitz-tests/tests/stand_act_popup.rs
what: edited — the stand's driver acting checks on an open list: a press returns the highlight move in its diff
@task_file n=20 path=tests/blitz-tests/tests/stand_cells_served.rs
what: edited — the stand's served Cells checks: the current cell alone reads selected and an arrow moves it
@task_file n=21 path=tests/blitz-tests/tests/stand_cross_os_readings.rs
what: edited — the stand's cross-OS readings check, Cells' digests re-pinned
@task_file n=22 path=tests/blitz-tests/tests/stand_diff.rs
what: edited — the stand's snapshot diff checks, one comparison per reading
@task_file n=23 path=docs/capabilities.md
what: edited — the capability statement: the limit that the highlight is unread replaced by how the highlight is read, node keys fifteen
@task_item n=1 part=touches
decision/highlighted-row-contrast-unread is retired: the lint now reads an open list's highlighted row at its painted pair, the engine-constant text on its background, with font size and weight from the option's computed font.
@task_item n=2 part=touches
decision/lint-contrast-shares-one-reader and decision/lint-contrast-single-formula: the lint now asks the harness's highlight reader first and the text reader where it answers none; both go through the one contrast formula.
@task_item n=3 part=touches
decision/select-list-colours-engine-constants and decision/multiple-select-chosen-row-pair: the highlight's engine constants are now pinned to the default sheet's chosen-row pair, black on #c8c8c8.
@task_item n=4 part=touches
decision/text-contrast-required-ratio and decision/wcag-contrast-thresholds, indirectly: the highlighted row's required ratio is now judged from the option's own computed font size and weight.
@task_item n=5 part=touches
contract/lint-contrast-finding rows and contract/visual-lint-finding/content: the finding's shape, order and written form are unchanged, but an open list's highlighted row now reads no contrast finding while plain rows still do.
@task_item n=6 part=touches
decision/select-list-rows-one-height: rows still take one height, now the tallest row's line (a header at weight 700 included) with the select's one-line height as the floor; a larger authored font no longer clips.
@task_item n=7 part=touches
contract/snapshot/open-list-row-bounds and contract/driver-screenshot/row-shot, indirectly: an open list's row bounds follow the new row height.
@task_item n=8 part=touches
decision/changed-set-marks: a highlight move to another row, by key or pointer, now marks the list's select; a move to the same row and a wheel move inside the list still mark nothing.
@task_item n=9 part=touches
decision/active-descendant-unread and decision/active-descendant-kept-unread: the engine now writes the active descendant for an open single select; a page's aria-activedescendant stays unread, so whichever row says the active descendant is unread in the tree is stale.
@task_item n=10 part=touches
contract/snapshot/open-list-highlighted-row and contract/snapshot/select-list-rows: the highlighted row is now read in the tree as the select's active descendant and in the snapshot by the highlighted node key.
@task_item n=11 part=touches
contract/accessibility-tree/single-select and decision/focus-stays-on-select-with-list-open: while open with a current row the tree names that option as active descendant, none when closed, and the tree's focus stays on the select.
@task_item n=12 part=touches
contract/snapshot-text/state-tokens: a new bare token, highlighted, written after clickable only when true.
@task_item n=13 part=touches
contract/snapshot-diff/differs and contract/snapshot-diff/entries: the diff can now name highlighted, and a press in an open list names it on both rows.
@task_item n=14 part=touches
decision/option-labels-leave-only-as-pixels: the highlighted row now leaves an open list as a snapshot reading as well as pixels; check whether the row still holds.
@task_item n=15 part=touches
decision/boolean-state-limits, decision/snapshot-reads-the-tree and decision/readers-of-one-snapshot-model, indirectly: the snapshot model gains a ninth node state field, a closed bool read from the tree, carried by every reader.
@task_item n=16 part=touches
absence/snapshot-list-open-state: there is still no expanded node key; highlighted names a row, not whether a list is open.
@task_item n=17 part=touches
contract/accessibility-tree-states/aria-selected and the contract/cells-grid rows (arrows above all): the Cells grid's current data cell alone now states aria-selected true, and an arrow moves it, the arrow also bringing the cell into view.
@task_item n=18 part=touches
decision/stand-contrast-asserted and decision/stand-lint-clean-at-boot, indirectly: the stand's contrast and keyboard checks stay green with Cells' new selected state and the highlight reading.
@task_fact n=1 slug=screen-reader-support
fact: The snapshot node carries highlighted, a closed bool written only when true, true on an open single select's highlighted row alone; it is read from the tree as the node a tree node names as its active descendant, and the diff's differs can name it.
@task_fact n=2 slug=screen-reader-support
fact: While a single select's list is open with a current row, the engine names that option as the select's active descendant in the accessibility tree and names none when closed; focus stays on the select and a page's aria-activedescendant is still never read.
@task_fact n=3 slug=visual-design-verification
fact: The harness has a read-only highlight reader that answers only for an open list's highlighted row, giving its painted text and background pair through the one contrast formula; the lint reads it before the text reader.
@task_fact n=4 slug=visual-design-verification
fact: The open list's highlight colour pair is the default sheet's chosen-row pair, black on #c8c8c8, and a check pins the two equal.
@task_fact n=5 slug=5-keyboard-navigation
fact: In the seven_guis Cells grid the current data cell alone states aria-selected true, and an arrow key moves that state to the new current cell.
@task_item n=1 part=untouched
Range input keys, pointer and value rows: no range input behaviour changed.
@task_item n=2 part=untouched
The closed single select's keys, typeahead and the open list's Escape, Tab, Enter and Space rows: no key's effect changed, only what a highlight move marks.
@task_item n=3 part=untouched
Multiple select keys, row clicks and rows: the multiple select is unchanged.
@task_item n=4 part=untouched
Hidden content, inert subtree, aria-hidden and hit-testing rows: nothing hidden or inert changed.
@task_item n=5 part=untouched
Password masking, file input value and accessible name precedence rows: no name or value reading changed.
@task_item n=6 part=untouched
The CRUD list's keys and Tab stop, and keyboard activation by Enter and Space: unchanged.
@task_item n=7 part=untouched
The accessibility feature defaults, the platform adapter rows and assistive-technology reach: no adapter was added and reach stays tier 0.
@task_item n=8 part=untouched
UI graph state registration and the Dioxus reached-handler record: untouched.
@task_item n=9 part=untouched
Scrollbar thumb, author thumb, scheme and zoom rows: no change to them.
@task_item n=10 part=untouched
CI legs, tiers and the three-round repeat: no leg, tier or status shape changed.
@task_item n=11 part=untouched
Cognitive accessibility and the invalid-state rows: untouched.
