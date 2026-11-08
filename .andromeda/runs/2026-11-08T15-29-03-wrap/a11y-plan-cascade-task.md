@task_prose part=session
The session added the driver verb actions: one read of every actionable node on the screen, each with the verbs it can take now (from the schema table's instance rows that take an id: click, type, scroll, screenshot) and the refusal cause each would meet.
In dioxus-native-dom it added DioxusDocument::actionable_nodes and the ActionableNode record (id, owner, keyed), and made the crate-private Readings the ONE actionable rule read by both unkeyed_actionable and actionable_nodes: focusable, or one of the unchanged 27 interactive roles, or current listeners holding an event other than mounted.
It retired unkeyed_actionable's reading of the data-dioxus-id mark as its listener leg; the leg now reads the exact current listeners from element_facts. The mark itself stays: the bridge writes it and event dispatch reads it.
The four id-taking arms of Session::execute now decide refusals through one shared check_target (click and type: disabled, then off-screen, then covered; scroll none; screenshot with an id: in view, empty crop, fonts-unavailable), which the actions read also calls.
Counts moved: VERBS 13 to 14, instance verbs 9 to 10, FieldKind 12 to 13, stand files 52 to 53, run stand 244 to 253 passed, dioxus-native-dom actionable unit tests 4 to 8 (crate 100 to 104), escher-driver unit tests 100 to 102.
Measured: ACTIONABLE_AT_BOOT counter 2, flight-booker 7, timer 3, crud 11, cells 678, load 2; the Cells answer is 106,088 bytes.
@task_file n=1 path=packages/escher-driver/src/actions.rs
@task_file n=2 path=tests/blitz-tests/tests/stand_actions.rs
@task_file n=3 path=packages/dioxus-native-dom/src/actionable.rs
@task_file n=4 path=packages/dioxus-native-dom/src/lib.rs
@task_file n=5 path=packages/escher-driver/src/execute.rs
@task_file n=6 path=packages/escher-driver/src/schema.rs
@task_file n=7 path=packages/escher-driver/src/schema/spec.rs
@task_file n=8 path=packages/escher-driver/src/schema_tests.rs
@task_file n=9 path=packages/escher-driver/src/command.rs
@task_file n=10 path=packages/escher-driver/src/json.rs
@task_file n=11 path=packages/escher-driver/src/describe.rs
@task_file n=12 path=packages/escher-driver/src/lib.rs
@task_file n=13 path=packages/escher-driver/src/mcp.rs
@task_file n=14 path=packages/escher-driver/src/mcp_tests.rs
@task_file n=15 path=packages/escher-driver/src/cli_tests.rs
@task_file n=16 path=packages/escher-driver/src/recording_tests.rs
@task_file n=17 path=tests/blitz-tests/tests/session_common/mod.rs
@task_file n=18 path=tests/blitz-tests/tests/common/mod.rs
@task_file n=19 path=tests/blitz-tests/tests/stand_act_spans.rs
@task_file n=20 path=tests/blitz-tests/tests/stand_act_scroll.rs
@task_file n=21 path=tests/blitz-tests/tests/stand_mcp_tools.rs
@task_file n=22 path=tests/blitz-tests/tests/stand_requirement_sweep.rs
@task_file n=23 path=examples/seven_guis/tests/cli_commands.rs
@task_file n=24 path=examples/seven_guis/tests/mcp_server.rs
@task_file n=25 path=examples/seven_guis/tests/host_log.rs
@task_file n=26 path=examples/seven_guis/tests/mcp_log.rs
@task_file n=27 path=examples/seven_guis/tests/common/mod.rs
@task_file n=28 path=scripts/cold-agent.sh
@task_file n=29 path=.github/scripts/test_cold_agent.py
@task_file n=30 path=docs/capabilities.md
@task_file n=31 path=README.md
@task_item n=1 part=touches
The actionable-key rule (sr-actionable-keys): any statement that unkeyed_actionable reads the data-dioxus-id listener mark, or that the mark over-includes an element whose listener was removed (conservative), is now stale; its listener leg reads the exact current listeners, and a removed listener is read by neither reader.
@task_item n=2 part=touches
Any statement of which readers use the vdom walk's exact listener reading (element_facts): it now also feeds unkeyed_actionable and actionable_nodes through the one Readings rule, beside the snapshot's clickable.
@task_item n=3 part=touches
Any statement that unkeyed_actionable is the only reader of the actionable rule, or that the rule has a single caller: actionable_nodes now reads the same rule, with the shared crate-private roles() and no second role mapping.
@task_item n=4 part=touches
The driver's disabled refusal for click and type: it is now decided by the shared check_target (disabled, then off-screen, then covered) for all four id-taking arms, and the actions read reports the same cause without acting.
@task_item n=5 part=touches
The snapshot model's list of consumers (text form, parts, diff, MCP results, node-key help, screenshot, the UI graph): the actions read now also reads one snapshot per call and its element ids.
@task_item n=6 part=touches
Any count of driver verbs, tools or commands (thirteen to fourteen) or of instance verbs (nine to ten), for example where the lint verb is said to run on every carrier; actions is the tenth instance verb, after graph.
@task_item n=7 part=touches
Any count of stand files (stand_*.rs 52 to 53) or of the run stand tally (244 passed, 0 failed, 7 ignored to 253, 0, 7); the new stand file enters the stand-repeat leg through the existing glob.
@task_item n=8 part=touches
Any count of dioxus-native-dom actionable unit tests (4 to 8) or of the crate's unit tests (100 to 104), and escher-driver unit tests (100 to 102).
@task_item n=9 part=touches
Any count of session_common declarers (33 to 34) or fixtures declarers (31 to 32, their stand files 23 to 24).
@task_item n=10 part=touches
The statement that stand_actionable_keys keeps unkeyed_actionable empty on every stand task: it still holds, now measured with the exact listener leg.
@task_item n=11 part=touches
Any citation of line ranges in the actionable module of dioxus-native-dom or its crate root re-exports: the module gained the Readings rule, actionable_nodes and four unit tests, so earlier line citations there may have moved.
@task_item n=12 part=touches
The unpaged-answer residual res-7cc5bi4k, where cited: it is widened to own the unpaged actions answer beside the unpaged graph (target: next).
@task_item n=13 part=touches
Coverage of new surfaces: the actionable rule and actionable_nodes are marked a11y-relevant against the actionable-key rule of section 7; the actions verb itself has no UI element.
@task_fact n=1 slug=screen-reader-support
fact: DioxusDocument::actionable_nodes, read-only, lists the UI graph's nodes in snapshot pre-order that the actionable rule holds for or whose graph owner is set, as ActionableNode { id, owner, keyed }, keyed true where the id holds no slash.
@task_fact n=2 slug=screen-reader-support
fact: The crate-private Readings in dioxus-native-dom is the one actionable rule (focusable, or an interactive role of the 27, or current listeners holding an event other than mounted), read by both unkeyed_actionable and actionable_nodes.
@task_fact n=3 slug=screen-reader-support
fact: The keyed reading on the actions answer is the actionable-key check's first wire form; the keyed field on the carriers is PROVISIONAL pending the founder's own word (overseer's call, 2026-11-08).
@task_fact n=4 slug=screen-reader-support
fact: The actions verb answers, for every actionable node, each verb of the table's instance rows taking an id with the cause check_target would refuse it with; it changes nothing (no focus, list, scroll or diff change) and writes ids, owner ids, verb and cause words and booleans only, never a value, typed text, MASKED_VALUE or NodeId.
@task_fact n=5 slug=screen-reader-support
fact: ACTIONABLE_AT_BOOT is measured as counter 2, flight-booker 7, timer 3, crud 11, cells 678, load 2 actionable nodes, and the four lean tasks hold 3 disabled controls at boot.
@task_fact n=6 slug=screen-reader-support
fact: A flight booker option reads covered with owner flight-type until its list opens, flight-return-date reads disabled, and crud-person-14 reads off-screen until a scroll (stand_actions, both layout modes).
@task_fact n=7 slug=screen-reader-support
fact: A fixture's unkeyed button reads keyed false in the actions answer and is the one unkeyed_actionable reports; an inert or hidden element is not listed by actionable_nodes.
@task_fact n=8 slug=screen-reader-support
fact: A listener removed on a re-render is read by neither reader, and the unit test also asserts that data-dioxus-id outlives the listener, the in-tree witness that the old mark leg would have counted it.
@task_fact n=9 slug=screen-reader-support
fact: On Cells 461 of 678 cells read off-screen for click, type and screenshot and all 678 take scroll; the unpaged actions answer is 106,088 bytes, asserted above 100,000 and within 1,048,576 bytes.
@task_item n=1 part=untouched
The ci.yml a11y leg and its fourteen named files, and the eight files named for the stand-repeat leg: no CI step, leg or named target changed.
@task_item n=2 part=untouched
Role derivation, the HTML-to-AccessKit mapping, ARIA state reads, accname labels and descriptions, and hidden-content exclusion in the engine: blitz-dom was not changed.
@task_item n=3 part=untouched
Keyboard navigation: focusability, focus fixup, Enter and Space activation, select and range keys, Tab orders and the stand_keyboard walks are unchanged.
@task_item n=4 part=untouched
Contrast verification (Harness::text_contrast, stand_contrast tables, the lint's contrast reading), the focus ring, motion and select list colours: unchanged.
@task_item n=5 part=untouched
The platform adapter, the accessibility tree lifecycle and changed_nodes, and residual res-llkgqygv: unchanged.
@task_item n=6 part=untouched
The snapshot's clickable field, MASKED_VALUE masking of password and file inputs, author_id carriage and the bridge's boolean attribute handling: unchanged; event routing through the data-dioxus-id mark is unchanged.
@task_item n=7 part=untouched
The accessibility_* and inert_subtree test inventories and their counts, and the cognitive error-recovery cues: unchanged.
