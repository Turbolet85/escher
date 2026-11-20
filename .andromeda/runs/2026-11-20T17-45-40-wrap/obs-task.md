@task_prose part=session
The session settled the select list's debts on the founder's forks: the open list's highlighted row is now read by the tree, the snapshot and the lint.
The accessibility tree names the highlighted option as the open single select's active descendant; focus stays on the select and a page's aria-activedescendant stays unread.
The snapshot node gains a ninth state, highlighted, a closed bool written only when true (text token after clickable, JSON true only when true); node keys 14 to 15, Reading::ALL 12 to 13, NodeState fields 8 to 9.
A highlight move to another row (key or pointer) now marks the select in the changed set, so an acting diff names highlighted; a move to the same row or a wheel move marks nothing.
The lint's contrast reads the highlighted row at its painted pair through a new read-only harness reader, highlight_contrast, which prints and logs nothing; list rows take the tallest row's height so a larger font no longer clips.
Cells' current cell states aria-selected; the single select's size stays unread (kept as its own route entry). No carrier, verb, exit, cause, session error, log field or span was added; CI green on all three OSes.
@task_file n=1 path=packages/blitz-test-harness/src/contrast.rs
what: the test harness's contrast reader, with the new read-only highlight_contrast for the open list's highlighted row
@task_file n=2 path=packages/escher-driver/src/lint.rs
what: the driver's lint verb and its contrast check, which now reads the highlighted row's painted pair first
@task_file n=3 path=packages/blitz-dom/src/document.rs
what: the engine document's select list: row measurement and the key move of the highlight, which now marks the changed set
@task_file n=4 path=packages/blitz-dom/src/events/pointer.rs
what: the engine's pointer events, the hover of an open list row, which now marks the changed set on a highlight move
@task_file n=5 path=packages/blitz-dom/src/accessibility.rs
what: the engine's accessibility tree builder, naming the open single select's highlighted option as active descendant
@task_file n=6 path=packages/dioxus-native-dom/src/snapshot.rs
what: the snapshot node and its NodeState states, gaining highlighted
@task_file n=7 path=packages/dioxus-native-dom/src/snapshot_text.rs
what: the snapshot text and its state tokens, gaining the highlighted token
@task_file n=8 path=packages/dioxus-native-dom/src/snapshot_diff.rs
what: the snapshot diff and its readings list, Reading::ALL, gaining Highlighted
@task_file n=9 path=packages/escher-driver/src/schema.rs
what: the driver's node-key schema table NODE_FIELDS and the help strings for snapshot text and the acting changed set
@task_file n=10 path=packages/escher-driver/src/schema_tests.rs
what: the driver's schema unit checks, pinning the node-key count
@task_file n=11 path=packages/escher-driver/src/json/snapshot.rs
what: the driver's JSON writer for the snapshot node, writing highlighted only when true
@task_file n=12 path=packages/escher-driver/src/json/diff.rs
what: the driver's JSON writer for the diff's changed and added nodes
@task_file n=13 path=packages/escher-driver/src/json_tests.rs
what: the driver's JSON unit checks over node keys and readings
@task_file n=14 path=packages/escher-driver/src/describe.rs
what: the driver's describe and help answer, its node_keys and its pinned counts
@task_file n=15 path=examples/seven_guis/src/tasks/cells.rs
what: the stand's Cells task, its current data cell now stating aria-selected
@task_file n=16 path=tests/blitz-tests/tests/select_popup.rs
what: the select popup checks: row height of a larger font, the tree's active descendant, the highlight move marks
@task_file n=17 path=tests/blitz-tests/tests/select_popup_painted.rs
what: the painted select popup checks: the highlight pair, the highlight reader, a larger font painted whole in its row
@task_file n=18 path=tests/blitz-tests/tests/stand_lint.rs
what: the stand's lint checks, the open list's highlighted row read at its painted pair
@task_file n=19 path=tests/blitz-tests/tests/stand_act_popup.rs
what: the stand's acting checks on the popup, a press returning the highlight move in its diff
@task_file n=20 path=tests/blitz-tests/tests/stand_cells_served.rs
what: the stand's served Cells checks, the current cell alone reading selected and an arrow moving it
@task_file n=21 path=tests/blitz-tests/tests/stand_cross_os_readings.rs
what: the stand's cross-OS readings check, Cells' re-pinned digests
@task_file n=22 path=tests/blitz-tests/tests/stand_diff.rs
what: the stand's diff checks, its per-reading list in Reading::ALL order
@task_file n=23 path=docs/capabilities.md
what: the capability statement, updated by hand: fifteen node keys, How the highlight is read, the retired limit The highlight is unread
@task_item n=1 part=touches
The snapshot node's state set moved: a ninth state, highlighted, a closed bool written only when true beside clickable; rows over the snapshot node and the snapshot text's clickable token stand beside it.
@task_item n=2 part=touches
The snapshot and diff carriers: the diff's differs can now name highlighted and a press in an open list returns the highlight move in changed; rows on the diff's open list row bounds and what the snapshot and diff carry stand beside it.
@task_item n=3 part=touches
The changed set: a highlight move to another row now marks the select, so the command span's changed field counts it where before nothing was marked; same-row and wheel moves still mark nothing.
@task_item n=4 part=touches
Select list privacy rows: the highlighted row's identity now crosses into the snapshot and the diff as a closed bool; rows that the list row label and the chosen set are never logged, and that the open list shot carries labels as pixels, stand beside it.
@task_item n=5 part=touches
Session logs carry no page content and the state registry holds no values: the new highlighted state is a closed bool inside the ratified carriers, with no log field or span added.
@task_item n=6 part=touches
The select size attribute read: on the founder's fork a single select's size stays unread and is kept as its own route entry; a row on reading size must still hold.
@task_item n=7 part=touches
The command answers (graphed, nodes and states) and the graph answer's content: the node states an answer carries gain highlighted.
@task_item n=8 part=touches
The capability statement updated by hand: it now reads fifteen node keys and How the highlight is read, the limit The highlight is unread retired.
@task_item n=9 part=touches
The lint verb's contrast check now reads the highlighted row's painted pair through a read-only harness reader that prints and logs nothing; rows that the lint answer and findings are never logged stand beside it, the finding's shape unchanged.
@task_item n=10 part=touches
The driver's MCP tool descriptions and help node_keys, read from the node-key table, now list fifteen keys; the help answer counts it.
@task_item n=11 part=touches
Cells' current cell states aria-selected, so the stand's snapshot of Cells reads selected on one cell; the cross-OS digest guard of Cells was re-pinned.
@task_fact n=1 slug=pii-scrubbing-compliance
fact: The open list's highlighted row crosses only as the snapshot node's highlighted, a closed bool written only when true inside the snapshot and diff answers; it is never logged and no span or log field carries it.
@task_fact n=2 slug=pii-scrubbing-compliance
fact: The engine names an open single select's highlighted option as the tree's active descendant from its own state; a page's aria-activedescendant stays unread.
@task_item n=1 part=untouched
Telemetry strategy: the tracing crate, feature gating of engine tracing and the escher crates' ungated tracing are unchanged; no event macro was added.
@task_item n=2 part=untouched
Span coverage: the command and tool_call spans' fields, the span line format and its scrub are unchanged; no span was added.
@task_item n=3 part=untouched
Metrics and backends: no metric, exporter, OTel or error-reporting service was added; the absences hold.
@task_item n=4 part=untouched
Error capture: panic hooks, script error surfaces, window error events and WPT reports are untouched.
@task_item n=5 part=untouched
The cold-agent pipe and the agent-run harness: their events, verdict fields, counted verbs and reasons are untouched.
@task_item n=6 part=untouched
CI integration: workflows, artifacts, the build-directory sweep line and the package-install diagnostic are unchanged; no CI or status shape moved.
@task_item n=7 part=untouched
escher-session's roles, exit codes, refusal causes, host wire, recording file and session lifecycle are unchanged; no verb, argument, exit, cause or session error was added.
@task_item n=8 part=untouched
escher's telemetry sink, its resource keys, RUST_LOG handling, target allowlist and stderr are untouched.
