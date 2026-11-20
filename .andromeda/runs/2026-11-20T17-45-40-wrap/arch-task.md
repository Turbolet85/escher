@task_prose part=session
The session closed the open single select list's debts on the founder's forks: the lint now reads the highlighted row at its painted pair through a new read-only test-harness reader, highlight_contrast, and plain rows still through text_contrast.
Open list rows take the tallest row's height (the select's own one-line height the floor), so a larger authored font no longer clips; the row height stays one value.
While a single select's list is open the accessibility tree names the highlighted option as the select's active descendant (focus stays on the select; a page's aria-activedescendant stays unread), and a highlight move to another row, by key or pointer, marks the select in the changed set.
The snapshot gains a ninth NodeState field and a fifteenth node key, highlighted, a closed bool written only when true (text token after clickable, JSON true only when true); Reading gains Highlighted before Bounds (12 to 13), and a diff's differs can name it.
The Cells stand's current cell now states aria-selected; Cells' cross-OS digests were re-pinned. The capability statement retired its limit that the highlight is unread and the clause that a larger-font row clips, replacing them with how the highlight is read.
No crate, dependency, carrier, verb, argument, exit, refusal cause, session error, log field or span was added or removed; a single select's size stays unread as a list box and is moved to its own route entry.
@task_file n=1 path=packages/blitz-test-harness/src/contrast.rs
what: edited — the test harness's contrast readers (text_contrast, contrast_ratio); gains the public read-only highlight_contrast for an open list's highlighted row
@task_file n=2 path=packages/escher-driver/src/lint.rs
what: edited — the lint verb's checks, its contrast check reading the highlighted row's painted pair first
@task_file n=3 path=packages/blitz-dom/src/document.rs
what: edited — BaseDocument's open select list machinery: list measurement (row height) and the highlight move that now writes the changed set
@task_file n=4 path=packages/blitz-dom/src/events/pointer.rs
what: edited — pointer default actions over the open select list, the pointermove row hover that now marks the select
@task_file n=5 path=packages/blitz-dom/src/accessibility.rs
what: edited — the blitz-dom accessibility tree builder, its single select arm setting the active descendant while the list is open
@task_file n=6 path=packages/dioxus-native-dom/src/snapshot.rs
what: edited — the snapshot's NodeState and SnapshotNode readers, gaining the highlighted field read from the tree's active descendant
@task_file n=7 path=packages/dioxus-native-dom/src/snapshot_text.rs
what: edited — the snapshot text form, its state tokens gaining highlighted after clickable
@task_file n=8 path=packages/dioxus-native-dom/src/snapshot_diff.rs
what: edited — the snapshot diff, its Reading list gaining Highlighted before Bounds and its differs comparison
@task_file n=9 path=packages/escher-driver/src/schema.rs
what: edited — the driver's command and node-key schema table NODE_FIELDS, and the help strings for snapshot text and acting changed
@task_file n=10 path=packages/escher-driver/src/schema_tests.rs
what: edited — unit checks of the driver schema table and its node-key count
@task_file n=11 path=packages/escher-driver/src/json/snapshot.rs
what: edited — the driver's own JSON writer for a snapshot node, writing highlighted true only when true
@task_file n=12 path=packages/escher-driver/src/json/diff.rs
what: edited — the driver's own JSON writer for a diff's changed and added nodes
@task_file n=13 path=packages/escher-driver/src/json_tests.rs
what: edited — unit checks of the driver's JSON answer writer and its reading words
@task_file n=14 path=packages/escher-driver/src/describe.rs
what: edited — the driver's one self-description for command line and MCP, its pinned counts of the schema tables
@task_file n=15 path=examples/seven_guis/src/tasks/cells.rs
what: edited — the seven_guis stand's Cells task, its grid's current cell now stating aria-selected
@task_file n=16 path=tests/blitz-tests/tests/select_popup.rs
what: edited — integration checks of the open select list: row height, accessibility tree active descendant and changed-set marks
@task_file n=17 path=tests/blitz-tests/tests/select_popup_painted.rs
what: edited — painted checks of the open select list: the highlight's pair against the default sheet's chosen row, the highlight reader, a larger font painted whole in its row
@task_file n=18 path=tests/blitz-tests/tests/stand_lint.rs
what: edited — stand checks of the lint verb, the open list's highlighted row read at its painted pair
@task_file n=19 path=tests/blitz-tests/tests/stand_act_popup.rs
what: edited — stand checks of acting verbs on an open select list, a press's diff naming the highlight move
@task_file n=20 path=tests/blitz-tests/tests/stand_cells_served.rs
what: edited — stand checks of the served Cells task, the current cell alone reading selected
@task_file n=21 path=tests/blitz-tests/tests/stand_cross_os_readings.rs
what: edited — the cross-OS byte check of stand readings, Cells' pinned digests re-pinned
@task_file n=22 path=tests/blitz-tests/tests/stand_diff.rs
what: edited — stand checks of the snapshot diff, its per-reading comparison list in Reading order
@task_file n=23 path=docs/capabilities.md
what: edited — the capability statement: node keys now fifteen, the limit that the highlight is unread replaced by how the highlight is read
@task_item n=1 part=touches
contract/snapshot/node-state: NodeState holds nine fields now (eight before), highlighted added.
@task_item n=2 part=touches
contract/snapshot-diff/reading: Reading gains Highlighted before Bounds; Reading::ALL is 13 (was 12), so a list ending Clickable, Bounds is stale.
@task_item n=3 part=touches
contract/snapshot-node-keys/expanded: fifteen node keys now (was fourteen), highlighted after clickable.
@task_item n=4 part=touches
contract/snapshot-text/state-tokens: a new bare token highlighted after clickable, written only when true.
@task_item n=5 part=touches
decision/select-list-crossing: the open list now also crosses its highlight as a closed bool inside the ratified snapshot and diff carriers, beyond bounds and pixels (founder's F3 A).
@task_item n=6 part=touches
decision/select-list-changed-set and decision/changed-set-write-sites: a highlight move to another row (key or pointer) now marks the select; a move to the same row and a wheel move inside the list still mark nothing.
@task_item n=7 part=touches
contract/snapshot-diff/differs, decision/differs-closed-set and contract/driver-diff/differs: the closed differs set can now name highlighted.
@task_item n=8 part=touches
contract/node-keys rows (clickable, selected, bounds) and contract/answer-json/node: a new highlighted key stands beside them, JSON writes true only when true.
@task_item n=9 part=touches
contract/acting-verb-results/changed and contract/snapshot-verb help: help strings for snapshot text and acting changed now name highlighted; contract/mcp-role/tool-listing reads the table so MCP tool descriptions name it too.
@task_item n=10 part=touches
contract/test-harness/text-contrast and contract/test-harness/contrast-ratio: a sibling reader highlight_contrast now stands beside text_contrast, through the one contrast_ratio; text_contrast unchanged.
@task_item n=11 part=touches
contract/lint-verb/contrast and contract/lint-reason-keys/contrast: the contrast check reads an open list's highlighted row at its painted pair first and falls back to text_contrast; finding shape, order and written form unchanged.
@task_item n=12 part=touches
decision/select-list-colours-engine-constants and contract/blitz-dom-crate-root/select-list-view-types: the highlight text and background constants are now read by the lint and pinned to the default sheet's chosen-row pair.
@task_item n=13 part=touches
contract/select-list/select-row-layout, contract/select-list/list-machinery and decision/select-list-rows-and-placement: each row takes the tallest row's layout height, the select's one-line height the floor, plus row padding; a larger authored font no longer clips.
@task_item n=14 part=touches
contract/accessibility-tree/select-expanded, contract/accessibility-tree/focus and decision/a11y-aria-states: an open single select names its highlighted option as active descendant; focus stays on the select; a page's aria-activedescendant stays unread.
@task_item n=15 part=touches
contract/snapshot/closed-select-option and contract/driver-snapshot/row-bounds: open list rows now also read highlighted on the one highlighted row.
@task_item n=16 part=touches
decision/capability-statement, contract/capability-statement/sections and decision/capability-statement-matches-schema: the statement's node-key count is fifteen and its limit that the highlight is unread is retired.
@task_item n=17 part=touches
decision/cells-grid-single-tab-stop and contract/accessibility-tree/aria-selected (indirect): the Cells grid's current cell alone now states aria-selected, and an arrow moves it (its diff naming selected and bounds).
@task_item n=18 part=touches
decision/stand-readings-same-on-every-ci-os (indirect): Cells' cross-OS digests re-pinned, held on linux, windows and macos.
@task_item n=19 part=touches
decision/a11y-rebuild-on-nonempty-changed-set (indirect): a highlight move now yields a non-empty changed set and so a tree rebuild.
@task_item n=20 part=touches
decision/fixed-program-text-on-carriers and decision/new-carrier-widens-boundary (indirect): a new key inside existing carriers, no new carrier; the boundary is not widened.
@task_fact n=1 slug=standard-contracts
fact: The test harness's highlight_contrast is a public read-only reader that answers only for an open select list's highlighted row, giving the engine's highlight text on highlight background through the one contrast_ratio with the option's computed font size and weight; it answers none elsewhere.
@task_fact n=2 slug=standard-contracts
fact: The snapshot's node key highlighted is a closed bool, true on an open single select list's highlighted option alone, read from the accessibility tree's active descendant; written only when true as a bare text token after clickable and as JSON true.
@task_fact n=3 slug=standard-contracts
fact: While a single select's list stands open with a current option, the accessibility tree sets that option as the select's active descendant; closed, none; the tree's focus stays on the select and nothing is read from a page's aria-activedescendant.
@task_fact n=4 slug=established-decisions
fact: The lint's contrast check reads an open list's highlighted row at its painted pair (the engine's highlight constants) and every other element through text_contrast, so the highlighted row reads no contrast finding where the shared pair passes.
@task_fact n=5 slug=established-decisions
fact: Open select list rows share one height, the tallest row's own layout height with the select's one-line height as the floor plus row padding, so no row's glyphs are clipped.
@task_fact n=6 slug=cross-cutting-patterns
fact: A highlight move to another row of an open select list, by key or pointer, writes the select into the changed set; a move to the same row and a wheel move inside the list write nothing.
@task_item n=1 part=untouched
The WPT runner, its suites, outputs, result kinds and CI artifacts: nothing in WPT moved.
@task_item n=2 part=untouched
The browser app, its history, persistence, about pages and toolbar: untouched.
@task_item n=3 part=untouched
The session socket wire, session host, session errors, refusal causes and faults, exit codes and the command line's roles and verbs: no carrier, verb, argument, exit or cause added.
@task_item n=4 part=untouched
Telemetry, command spans and log fields: no log field or span added, nothing new logged.
@task_item n=5 part=untouched
Kept check file format, recording and replay: the committed Counter checks replay byte-equal.
@task_item n=6 part=untouched
Stack and dependency rows, toolchain, cargo-deny, coverage floors and CI legs and workflow: no dependency, tool or leg changed.
@task_item n=7 part=untouched
Multiple select rows and their chosen-row colours, keys and clicks: only the single select's open list changed.
@task_item n=8 part=untouched
Occupied names (crates, binaries, servers, environment variables): no crate, binary or name added or freed.
@task_item n=9 part=untouched
Script, JS runtime, networking, fetch, layout engine (Taffy, Stylo) and paint scene rows: untouched.
