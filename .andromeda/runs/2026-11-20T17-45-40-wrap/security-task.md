@task_prose part=session
The session settled the open select list's selection debts on the founder's forks (F1 A, F2 A, F3 A, F4 C, F5 A).
The highlighted row of an open single select is now read: the accessibility tree names the highlighted option as the select's active descendant while the list is open (focus stays on the select; a page's aria-activedescendant is still never read), and the snapshot and diff carry a new closed bool node key highlighted, written only when true.
The driver's node keys moved 14 to 15, the diff readings (Reading::ALL) 12 to 13, NodeState fields 8 to 9; a press or pointer move of the highlight to another row now marks the select changed, so the diff names highlighted on both rows.
The lint reads the highlighted row's contrast at its painted pair through a new read-only test-harness reader, highlight_contrast; open-list rows take the tallest row's height so a larger font no longer clips.
The stand Cells' current data cell now states aria-selected true; the limit «The highlight is unread» was retired from the capability statement; a single select's size attribute stays unread, carried to its own route entry.
No new carrier, verb, argument, exit, refusal cause, session error, log field or span; no dependency moved; CI green on all three OSes.
@task_file n=1 path=packages/blitz-test-harness/src/contrast.rs
what: the test harness's text-contrast reader; gains a read-only highlight contrast reader for the open select list's highlighted row (engine constants, no mutation, no log)
@task_file n=2 path=packages/escher-driver/src/lint.rs
what: the driver's lint and its contrast finding; reads the highlighted row's painted pair first, finding shape and closed content unchanged
@task_file n=3 path=packages/blitz-dom/src/document.rs
what: the engine document holding the open select list (its measure and its highlight move); rows take the tallest row's height, a highlight move marks the select in the changed set
@task_file n=4 path=packages/blitz-dom/src/events/pointer.rs
what: pointer handling over an open select list; a hover moving the highlight to another row marks the select changed
@task_file n=5 path=packages/blitz-dom/src/accessibility.rs
what: the accessibility tree builder on the accessibility adapter crossing; an open single select names its highlighted option as active descendant from engine state alone
@task_file n=6 path=packages/dioxus-native-dom/src/snapshot.rs
what: the snapshot's node state (the state readings per control); gains the highlighted bool read from the tree's active descendant
@task_file n=7 path=packages/dioxus-native-dom/src/snapshot_text.rs
what: the snapshot text and its state words; gains the highlighted token after clickable, written only when true
@task_file n=8 path=packages/dioxus-native-dom/src/snapshot_diff.rs
what: the snapshot diff and its reading list; gains the Highlighted reading so differs can name highlighted
@task_file n=9 path=packages/escher-driver/src/schema.rs
what: the driver's closed node-key schema table; fifteen keys with highlighted after clickable, help strings name it
@task_file n=10 path=packages/escher-driver/src/schema_tests.rs
what: unit checks pinning the node-key table's length and order
@task_file n=11 path=packages/escher-driver/src/json/snapshot.rs
what: the JSON snapshot node writer; writes highlighted true only when true
@task_file n=12 path=packages/escher-driver/src/json/diff.rs
what: the JSON diff node writer for changed and added nodes
@task_file n=13 path=packages/escher-driver/src/json_tests.rs
what: unit checks of the JSON snapshot and diff carriers and the reading list
@task_file n=14 path=packages/escher-driver/src/describe.rs
what: the driver's help and self-description (node_keys listing and its counts)
@task_file n=15 path=examples/seven_guis/src/tasks/cells.rs
what: the stand Cells grid; its current data cell states aria-selected true
@task_file n=16 path=tests/blitz-tests/tests/select_popup.rs
what: checks of the open select list (row height, active descendant, changed-set marks)
@task_file n=17 path=tests/blitz-tests/tests/select_popup_painted.rs
what: painted checks of the open select list (highlight colour pair, unclipped larger font)
@task_file n=18 path=tests/blitz-tests/tests/stand_lint.rs
what: checks of the driver's lint findings, including the highlighted row's contrast
@task_file n=19 path=tests/blitz-tests/tests/stand_act_popup.rs
what: checks of acting verbs on an open select list; a press returns the highlight move in its diff
@task_file n=20 path=tests/blitz-tests/tests/stand_cells_served.rs
what: checks of the served Cells stand; the current cell alone reads selected and an arrow moves it
@task_file n=21 path=tests/blitz-tests/tests/stand_cross_os_readings.rs
what: the cross-OS readings guard; Cells' digests re-pinned
@task_file n=22 path=tests/blitz-tests/tests/stand_diff.rs
what: checks of the snapshot diff per reading
@task_file n=23 path=docs/capabilities.md
what: the committed capability statement (fifteen node keys, «How the highlight is read», aria-activedescendant unread from a page)
@task_item n=1 part=touches
decision/select-list-crossing: the select list's crossing, limited to bounds and pixels, now also carries the highlight as a closed bool (snapshot highlighted, tree active descendant) on the founder's F3 A; row labels still cross only as pixels.
@task_item n=2 part=touches
decision/snapshot-state-readings: three state readings per control becomes four with the highlight, a closed bool inside a ratified carrier.
@task_item n=3 part=touches
decision/control-states-closed-values-only and decision/closed-value-field-no-widening: the new highlighted key is a closed bool in the ratified snapshot and diff carriers; a rule these rows govern, applied again.
@task_item n=4 part=touches
contract/diff-node keys (clickable, differs, invalid, pressed, selected): a new sibling diff node key highlighted; differs can now name highlighted.
@task_item n=5 part=touches
contract/snapshot-text/state-words and its per-word rows: a new state word highlighted after clickable, written only when true.
@task_item n=6 part=touches
contract/driver-session-reply/state-readings, contract/graph-answer/nodes-states and contract/mcp-result rows: the state readings that leave by the session reply and MCP results gain highlighted; the MCP tool descriptions read from the node-key table now list it.
@task_item n=7 part=touches
decision/accessibility-adapter-content-classes and decision/descriptions-adapter-only: the accessibility adapter crossing gains an active descendant (a node reference, no content) on an open single select.
@task_item n=8 part=touches
decision/select-list-live-reader, decision/select-readers-resolve-node-closed-select-none, decision/visible-region-open-list-row: the open list is now also read by the tree builder and a test-harness contrast reader through the same live list view.
@task_item n=9 part=touches
decision/select-list-label-never-logged and decision/lint-findings-closed-content: the lint now reads the highlighted row's painted pair; finding shape and written form unchanged, nothing logged.
@task_item n=10 part=touches
decision/select-list-engine-colours: the highlight colour constants are pinned equal to the default sheet's chosen-row pair (#000000 on #c8c8c8).
@task_item n=11 part=touches
decision/select-list-hostile-markup-finite and decision/select-size-hint-cap: row height is now the tallest row's own layout height (floor the select's one-line height), so an authored large font sizes every row.
@task_item n=12 part=touches
decision/accessibility-tree-refresh-on-change: a highlight move to another row (key or pointer) now inserts the select in the changed set; a same-row or wheel move marks nothing.
@task_item n=13 part=touches
decision/cells-grid-keyboard: the stand Cells' current data cell now states aria-selected true, moved by the arrow keys; the arrow's diff names selected and bounds.
@task_item n=14 part=touches
decision/capability-statement-mcp-crossing: the committed statement now says fifteen node keys and how the highlight is read; its limit «The highlight is unread» retired.
@task_item n=15 part=touches
decision/open-list-shot-pixels: the open list's labels still cross as pixels only; the new highlight reading adds no label text to any carrier.
@task_fact n=1 slug=threat-model-summary
fact: The snapshot and diff carry a closed bool node key highlighted, written only when true, on the one highlighted row of an open single select's list; a diff's differs can name it.
@task_fact n=2 slug=input-validation
fact: While a single select's list is open the accessibility tree names the highlighted option as the select's active descendant, from engine state alone; a page's aria-activedescendant stays unread and the tree's focus stays on the select.
@task_fact n=3 slug=input-validation
fact: The snapshot text writes the state word highlighted after clickable, only when true.
@task_item n=1 part=untouched
Authentication, authorization, CI tokens and publish secrets: no workflow, secret or permission moved.
@task_item n=2 part=untouched
Dependency security: no manifest or lockfile moved, no crate added or removed, the audit and licence gates unchanged.
@task_item n=3 part=untouched
Session socket, MCP stdio server, request grammar, bounds, idle expiry, exit codes and refusal causes: no verb, argument, exit, cause or session error added.
@task_item n=4 part=untouched
Logging and monitoring: no log field, span or sink target added; the command span unchanged.
@task_item n=5 part=untouched
Outbound network, fetch, CORS, CSP, URL schemes and iframe or import limits: no network or loading path reached.
@task_item n=6 part=untouched
Data protection, kept and recorded checks, cookies, history, cache and cold-agent recordings: the committed Counter checks replay byte-equal; no on-disk format moved.
@task_item n=7 part=untouched
Masked password and file values and the stable element id: no masking or id derivation moved.
