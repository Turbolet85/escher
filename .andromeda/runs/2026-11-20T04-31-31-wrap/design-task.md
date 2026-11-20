@task_prose part=session
The session moved the stand checks' repeated boot, act and answer-read setup into shared test helpers; each check keeps one target, the same tests and the same messages.
The shared session module gained the mcp tool-call and served-exchange helpers, the tool-result reader, the held-task step helper, the geometry helpers (edges, centre, area, inside, viewport) and the recorded-check helper; the shared common module gained the element-mark reader.
Private copies of those helpers were removed from the stand checks; no product source, manifest, CI script or committed check changed, and no test was added, renamed or removed.
Measured: the duplication reading fell from 4.05 to 3.74 percent and the stand checks' own reading from 75 pairs and 870 lines to 42 pairs and 358 lines; the upstream-taken tests' share is untouched and the founder's question on keeping them stays unanswered.
Gates green: fast, stand-repeat 3 of 3 rounds, a11y, the fold guard, and the operator pass's CI read 17 of 17 including the windows leg.
@task_file n=1 path=tests/blitz-tests/tests/session_common/mod.rs
what: the stand checks' shared session helpers: booting a task under escher-session, the mcp role exchange and its tool result read, the held task stepping through driver verbs, element geometry against the viewport, and the recorded check
@task_file n=2 path=tests/blitz-tests/tests/common/mod.rs
what: the integration tests' shared harness helpers, now with the element marks read from the accessibility roles
@task_file n=3 path=tests/blitz-tests/tests/stand_act_error_text.rs
what: the stand check of the session error text a failed act answers over the mcp role
@task_file n=4 path=tests/blitz-tests/tests/stand_act_errors.rs
what: the stand check of act refusals and the session error object with its remedy
@task_file n=5 path=tests/blitz-tests/tests/stand_order_served.rs
what: the stand check of the order task served over the mcp role
@task_file n=6 path=tests/blitz-tests/tests/stand_mcp_masked.rs
what: the stand check of masked values in the mcp role's tool results
@task_file n=7 path=tests/blitz-tests/tests/stand_snapshot_parts.rs
what: the stand check of the snapshot answer and its parts
@task_file n=8 path=tests/blitz-tests/tests/stand_mcp_tools.rs
what: the stand check of the escher-session mcp role's tools, their results and stdout
@task_file n=9 path=tests/blitz-tests/tests/stand_masks_on_new_answers.rs
what: the stand check that masks hold on new answers
@task_file n=10 path=tests/blitz-tests/tests/stand_session_state.rs
what: the stand check of session state held across commands, focus among it
@task_file n=11 path=tests/blitz-tests/tests/stand_actionable_keys.rs
what: the stand check of the actionable keys the actions answer lists
@task_file n=12 path=tests/blitz-tests/tests/stand_non_lean_tasks.rs
what: the stand check of the escher-session tasks served without the lean path
@task_file n=13 path=tests/blitz-tests/tests/stand_actions.rs
what: the stand check of the actions line and its answer
@task_file n=14 path=tests/blitz-tests/tests/stand_layout.rs
what: the stand check of the layout answer
@task_file n=15 path=tests/blitz-tests/tests/stand_act_diff.rs
what: the stand check of the snapshot diff an act answers, incremental and full modes agreeing
@task_file n=16 path=tests/blitz-tests/tests/stand_act_states.rs
what: the stand check of the states a task passes through under driver verbs
@task_file n=17 path=tests/blitz-tests/tests/stand_act_obstructed.rs
what: the stand check of a click on an obstructed element, hit-testing against what paints over it
@task_file n=18 path=tests/blitz-tests/tests/stand_act_scroll.rs
what: the stand check of driver scroll into view before an act
@task_file n=19 path=tests/blitz-tests/tests/stand_act_transformed.rs
what: the stand check of a click landing on a transformed element
@task_file n=20 path=tests/blitz-tests/tests/stand_load_page.rs
what: the stand check of the load task: the load wait, the click and the press, and the virtual clock
@task_file n=21 path=tests/blitz-tests/tests/stand_replay.rs
what: the stand check of a recorded check replayed, kept verbs and the step it stopped at
@task_item n=1 part=touches
The escher-session mcp role contracts (help, snapshot, screenshot and other tool results, the Err ending, stdout): their stand witnesses now read tool results through one shared reader; target names and claims unchanged, no guard renamed.
@task_item n=2 part=touches
The escher-session commands answers (actions, layout, snapshot parts, coverage, graph): witnessed by stand checks whose setup folded into shared helpers; the answers themselves are unchanged.
@task_item n=3 part=touches
The escher-session tasks contracts (order, load, counter and the others): the stand checks serving them now boot through the shared served-exchange and hold helpers; the task list and its cells are unchanged.
@task_item n=4 part=touches
The driver verbs and virtual clock contract items (click, press, type, advance) and the load-wait-moves-no-clock decision: the load stand check's shared tail of assertions moved into a private helper with the same messages; no clock behaviour changed.
@task_item n=5 part=touches
The session error object's remedy and the stderr session error contract: the act error stand checks dropped their private exchange copies for the shared one; the error text and remedy asserted are the same.
@task_item n=6 part=touches
The actions line contracts and the actions-share-the-verbs-checks decision: the actions and actionable-keys stand checks now hold their task through the shared hold helper; no actions content changed.
@task_item n=7 part=touches
The hit-testing decisions (hit-testing stops at a paint clip, stacking-context groups paint and hit-testing): the obstructed and transformed click stand checks now read element edges, centre, area and the viewport from shared helpers; hit behaviour unchanged.
@task_item n=8 part=touches
The driver-scroll-instant and into-view-scroll decisions: the scroll stand check now takes inside and viewport from the shared helpers and keeps its own edges and centre; scroll behaviour unchanged.
@task_item n=9 part=touches
The one design-system row the citation sweep found naming a stand target as a guard or witness: it names the target by name, no line, and every target stands one to a file with the same tests, so no re-point and no claim change.
@task_item n=10 part=touches
The logical-viewport decision: the stand checks' viewport bounds now come from one shared viewport helper; the viewport the session reports is unchanged.
@task_item n=1 part=untouched
Colour palette, typography, spacing, depth, border radius, brand and iconography rows: no colour, font, spacing, elevation, radius or cursor behaviour or value moved.
@task_item n=2 part=untouched
The select list rows (its events, accessibility tree, overlay, placement, motion and colours): no select behaviour or test of it changed.
@task_item n=3 part=untouched
The WPT runner output and result colour contracts and its secondary text decision: the WPT runner was not reached.
@task_item n=4 part=untouched
The shell provider, JS window and dioxus-native configuration contracts and the font context rows: no product configuration or provider changed.
@task_item n=5 part=untouched
The escher-session exit status and exit code contracts and the exit-statuses-from-schema-table decision: no exit status, schema or command line argument moved.
@task_item n=6 part=untouched
The motion decisions on animation, overlay scrollbar fade and smooth scroll: no animation, settle or scroll timing changed.
@task_item n=7 part=untouched
The duplication reading and the shared-module reader count: no design-system row holds either value, so their movement touches no row.
