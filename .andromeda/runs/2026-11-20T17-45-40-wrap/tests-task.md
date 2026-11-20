@task_prose part=session
The session settled the open select list's debts on the founder's forks: the lint now reads the highlighted row of an open list at its painted pair through a new read-only harness reader, highlight_contrast, and falls back to text_contrast elsewhere.
The open list's rows all take the tallest row's height, so a larger authored font is painted whole inside its row; the clause that such a row clips is retired.
The engine's accessibility tree names the highlighted option as the open single select's active descendant, and a highlight move to another row (key or pointer) now marks the select changed.
The snapshot and diff gain a node key highlighted (a bare token after clickable, JSON true only when true): node keys 14 to 15, Reading::ALL 12 to 13, NodeState fields 8 to 9.
The stand's Cells task marks its current cell aria-selected; Cells' two cross-OS digests were re-pinned and its arrow diff measured as differs selected and bounds.
One painted check pins a glyph's top inked row as a stated band 7..=8 rather than a literal, because font-embolden moves it one pixel; the wholly-inside assertions stay exact. CI green on all three OS.
@task_file n=1 path=packages/blitz-test-harness/src/contrast.rs
what: the harness's contrast reading (text_contrast, required_ratio, the one contrast formula); edited to add the read-only highlight_contrast reader
@task_file n=2 path=packages/escher-driver/src/lint.rs
what: the driver's lint and its contrast defect; edited so the contrast finding reads the highlight's painted pair first, text contrast otherwise
@task_file n=3 path=packages/blitz-dom/src/document.rs
what: the engine document's select list (measurement of its rows, the highlight move by keys); edited
@task_file n=4 path=packages/blitz-dom/src/events/pointer.rs
what: the engine's pointer event handling (hover over an open list's rows); edited
@task_file n=5 path=packages/blitz-dom/src/accessibility.rs
what: the engine's accessibility tree builder; edited to name the open list's highlighted option as active descendant
@task_file n=6 path=packages/dioxus-native-dom/src/snapshot.rs
what: the snapshot node and its node state readings; edited (highlighted field)
@task_file n=7 path=packages/dioxus-native-dom/src/snapshot_text.rs
what: the snapshot text and its state tokens; edited (highlighted token)
@task_file n=8 path=packages/dioxus-native-dom/src/snapshot_diff.rs
what: the snapshot diff and its reading list; edited (Highlighted reading)
@task_file n=9 path=packages/escher-driver/src/schema.rs
what: the driver schema's node-key table read by help and the capability statement; edited (fifteen node keys)
@task_file n=10 path=packages/escher-driver/src/schema_tests.rs
what: the driver schema's inline unit tests, a table-driven test counting its rows; edited
@task_file n=11 path=packages/escher-driver/src/json/snapshot.rs
what: the driver's JSON answers for snapshot nodes; edited
@task_file n=12 path=packages/escher-driver/src/json/diff.rs
what: the driver's JSON answers for diff nodes; edited
@task_file n=13 path=packages/escher-driver/src/json_tests.rs
what: the driver's JSON answer unit tests; edited
@task_file n=14 path=packages/escher-driver/src/describe.rs
what: the driver's help description and its inline test pinning the schema's counts; edited
@task_file n=15 path=examples/seven_guis/src/tasks/cells.rs
what: the seven_guis stand's Cells task; edited (current cell aria-selected)
@task_file n=16 path=tests/blitz-tests/tests/select_popup.rs
what: the select popup integration test target, run in both layout modes; edited (row height and active descendant tests)
@task_file n=17 path=tests/blitz-tests/tests/select_popup_painted.rs
what: the select popup's pixel tests on a CPU buffer; edited (painted pair, highlight reader and a band-pinned glyph row)
@task_file n=18 path=tests/blitz-tests/tests/stand_lint.rs
what: the stand's lint integration test target; edited (highlighted row read at its painted pair)
@task_file n=19 path=tests/blitz-tests/tests/stand_act_popup.rs
what: the stand's acting-on-a-popup integration test target; edited (a press's diff names highlighted)
@task_file n=20 path=tests/blitz-tests/tests/stand_cells_served.rs
what: the stand's served Cells integration test target; edited (current cell alone reads selected)
@task_file n=21 path=tests/blitz-tests/tests/stand_cross_os_readings.rs
what: the cross-OS readings of the served tasks by answer digest; edited (two Cells digests re-pinned)
@task_file n=22 path=tests/blitz-tests/tests/stand_diff.rs
what: the stand's snapshot diff integration test target with its per-reading list; edited
@task_file n=23 path=docs/capabilities.md
what: the capability statement pinned to the driver schema; edited (fifteen node keys, how the highlight is read, the retired limit)
@task_item n=1 part=touches
Harness contrast contracts (text_contrast, required_ratio, blend-mode): a new public read-only reader highlight_contrast joins them, answering only the highlighted row of an open list and None elsewhere; text_contrast is unchanged.
@task_item n=2 part=touches
Contrast reader changes nothing, and one contrast formula: the new reader reads only and goes through the one contrast_ratio; both rules now cover a second reader.
@task_item n=3 part=touches
The harness's text contrast no-reading contract: the lint now falls back to text contrast only where the highlight reader answers None.
@task_item n=4 part=touches
Lint defect contrast and lint findings order: the contrast finding reads the highlighted row at its painted pair (no finding there); shape, order and written form unchanged.
@task_item n=5 part=touches
Chosen select row colour pair: the highlight's text and background are pinned by test to the default sheet's chosen row pair, black on c8c8c8.
@task_item n=6 part=touches
Snapshot node clickable and the snapshot node keys: a new node key highlighted stands after clickable, written only when true; the snapshot text, JSON and diff carriers name it.
@task_item n=7 part=touches
Capability statement pinned to the driver schema and claim section names and counts held to the schema: the node-key count moved 14 to 15 and the highlight limit was replaced by How the highlight is read.
@task_item n=8 part=touches
Verb-count and test-rename sweeps: a node-key table length is also pinned by a hand count list in the help description's test and a hand per-reading list in the stand diff check; a sweep on the table names missed both.
@task_item n=9 part=touches
Table-driven tests assert their row count: NODE_FIELDS length 14 to 15 and Reading::ALL 12 to 13 moved in the counting tests.
@task_item n=10 part=touches
Expected value pinned as a literal: an exception, a glyph's top inked row pinned as the band 7..=8 with its reason in a comment, since font-embolden moves it one pixel.
@task_item n=11 part=touches
Embolden local witness and embolden leg without a CI job: the fast leg's embolden step read a plain-build pixel pin red, mended by the band.
@task_item n=12 part=touches
Cross-OS readings of the served tasks by answer digest: Cells' two digests re-pinned (boot and after the step), held on windows and macos in CI.
@task_item n=13 part=touches
Stand checks and layout tests in both layout modes: every new select popup, lint, act popup and Cells test runs in both modes.
@task_item n=14 part=touches
Acting call result's changed node states and the run's diff: a highlight move to another row now marks the select changed, so a press diff names highlighted on both rows; a same-row or wheel move marks nothing.
@task_item n=15 part=touches
Committed checks byte-equal to the recording: the Counter checks replay byte-equal with the new key (recording and stand_replay green).
@task_item n=16 part=touches
Select list painted by the scene painter alone: the open list's row height is now the tallest row's line with the select's one-line height as floor.
@task_item n=17 part=touches
Stand boot of Cells: Cells' current cell now carries aria-selected, so its snapshot reads selected on that cell alone and an arrow's diff names selected and bounds.
@task_item n=18 part=touches
Oversize integration-test targets kept whole: select_popup grew by 130 lines and select_popup_painted by 141.
@task_fact n=1 slug=3-test-harness-contract
fact: Harness::highlight_contrast is a public read-only reader that answers only for the highlighted row of an open select list, with the list's highlight text and background through the one contrast formula and the option's own font size and weight, and None for any other node.
@task_fact n=2 slug=unit-test-strategy
fact: The lint's contrast reading takes the highlight reader first and the text contrast where it answers None, so an open list's highlighted row is judged at its painted pair and plain rows at their own.
@task_fact n=3 slug=unit-test-strategy
fact: A pixel-row pin that font-embolden moves is pinned as a stated band with its reason in a comment where the test crate cannot see the feature, while its wholly-inside assertions stay exact; on the operator's word.
@task_fact n=4 slug=test-strategy
fact: A count or rename sweep over the driver's node-key table must also reach the hand-held count list in the help description's test and the stand diff check's per-reading list, which pin the table's length outside its name.
@task_fact n=5 slug=unit-test-strategy
fact: The snapshot node key highlighted is a closed bool written as a bare token after clickable and as JSON true only when true, read from the tree's active descendant of an open single select.
@task_item n=1 part=untouched
The agent-run and cold-agent contracts, events, exit codes and verdicts: no agent-run, CI or status shape changed.
@task_item n=2 part=untouched
The session wire, session host, session errors, state directory and recording contracts: no new carrier, verb, argument, exit, cause or session error.
@task_item n=3 part=untouched
Driver refusal causes, their order and printed form, and the escher-session exit codes: unchanged.
@task_item n=4 part=untouched
The MCP server and MCP role contracts (protocol, tools list order, tool results): only descriptions read from the node-key table carry the new key; no tool or argument moved.
@task_item n=5 part=untouched
WPT runner, its CLI, status set, expectations and conformance decisions: no WPT run or suite changed.
@task_item n=6 part=untouched
CI workflow decisions (actions pinned, caches, triggers, package installs, legs and their membership, MSRV, clippy, audit): no workflow or leg script changed and no dependency moved.
@task_item n=7 part=untouched
Coverage floors, mutation testing and complexity ceilings: no floor or threshold changed.
@task_item n=8 part=untouched
Virtual clock, advance, settle and timer decisions: no time or settle behaviour changed.
@task_item n=9 part=untouched
Mocking discipline, network stand-ins and offline stand boot: no stand-in changed.
@task_item n=10 part=untouched
Handler coverage, UI graph and actions command decisions: unchanged.
@task_item n=11 part=untouched
Password masking and typed-password rules and the log content rules: no log field, span or typed-text path changed.
