@task_prose part=session
The session moved the stand checks' repeated setup (boot a task, serve MCP exchanges, act and settle a step, read element bounds and marks, record a replay check) into the shared test helper modules; every stand check stays one target with the same tests and messages.
No product source, manifest, CI script, seven_guis test or committed check changed; no file added, renamed or removed; the fold guard read 197 files, 930 tests and an equal literal set.
Measured: the duplication reading fell from 4.05 to 3.74 percent and the stand checks' own reading from 75 pairs / 870 lines to 42 / 358; the upstream-taken tests' share stays untouched, the founder's question on keeping them unanswered.
The fast leg, the stand-repeat leg (3 of 3 rounds green) and the a11y leg all returned green, and the operator's CI read green 17 of 17, the windows leg included.
Nothing retired; no row's claim changes — the stand targets each row names as a guard or witness all stand under their same names.
@task_file n=1 path=tests/blitz-tests/tests/session_common/mod.rs
what: the shared session test helpers of the stand checks — boot a held task, serve MCP exchanges and decode tool results, settle an acted step, compare incremental and full modes, read element edges, centre and viewport, record a replay check
@task_file n=2 path=tests/blitz-tests/tests/common/mod.rs
what: the shared harness test helpers of the stand checks — now also the element marks read off the accessibility tree (role per node)
@task_file n=3 path=tests/blitz-tests/tests/stand_act_error_text.rs
what: the stand check of the driver's act refusal text over MCP
@task_file n=4 path=tests/blitz-tests/tests/stand_act_errors.rs
what: the stand check of the driver's act refusals (refusal causes such as disabled) over the served session
@task_file n=5 path=tests/blitz-tests/tests/stand_order_served.rs
what: the stand check of served answer order over the MCP exchange
@task_file n=6 path=tests/blitz-tests/tests/stand_mcp_masked.rs
what: the stand check that a password value stays masked in MCP tool results
@task_file n=7 path=tests/blitz-tests/tests/stand_snapshot_parts.rs
what: the stand check of the snapshot text form's parts in MCP results
@task_file n=8 path=tests/blitz-tests/tests/stand_mcp_tools.rs
what: the stand check of the MCP tools (snapshot, act, actions, screenshot) and their results, the covered-click refusal and no-font exchanges included
@task_file n=9 path=tests/blitz-tests/tests/stand_masks_on_new_answers.rs
what: the stand check that masking holds on every new answer
@task_file n=10 path=tests/blitz-tests/tests/stand_session_state.rs
what: the stand check of the session's state and focus reading
@task_file n=11 path=tests/blitz-tests/tests/stand_actionable_keys.rs
what: the stand check of the actionable-key rule against the accessibility tree's element roles
@task_file n=12 path=tests/blitz-tests/tests/stand_non_lean_tasks.rs
what: the stand check of the non-lean stand tasks served through a session
@task_file n=13 path=tests/blitz-tests/tests/stand_actions.rs
what: the stand check of the actions answer (actionable nodes, causes, fields) per stand task
@task_file n=14 path=tests/blitz-tests/tests/stand_layout.rs
what: the stand check of the stand tasks' layout in the snapshot bounds
@task_file n=15 path=tests/blitz-tests/tests/stand_act_diff.rs
what: the stand check of the snapshot diff after each act, incremental and full modes agreeing
@task_file n=16 path=tests/blitz-tests/tests/stand_act_states.rs
what: the stand check of states (checked, expanded, value) read after each settled act step
@task_file n=17 path=tests/blitz-tests/tests/stand_act_obstructed.rs
what: the stand check of the driver click refused when covered, by element edges inside the viewport
@task_file n=18 path=tests/blitz-tests/tests/stand_act_scroll.rs
what: the stand check of act scrolling an element into the viewport
@task_file n=19 path=tests/blitz-tests/tests/stand_act_transformed.rs
what: the stand check that a driver click lands on a transformed element at its centre
@task_file n=20 path=tests/blitz-tests/tests/stand_load_page.rs
what: the stand check of a page load reached by a click or a key press
@task_file n=21 path=tests/blitz-tests/tests/stand_replay.rs
what: the stand check of recorded replay — the recorded check kept, holding, stopped at its step
@task_item n=1 part=touches
Stand expectations stated once and expected roles and names stated once: the shared test helpers now also hold the element marks read off the accessibility tree; the expectation tables themselves are unmoved — a picker checks the rule still reads true.
@task_item n=2 part=touches
Stand checks repeated three rounds: the guard leg stand-repeat ran green 3 of 3 over the changed checks; its round count unchanged.
@task_item n=3 part=touches
The a11y CI leg, the accessibility test leg in the slow tier and accessibility checks gating CI: the a11y leg ran green over the moved helpers; no leg, threshold or target renamed.
@task_item n=4 part=touches
Accessibility assertions as Rust tests over AccessKit: the stand checks asserting over the tree now draw their setup and element marks from shared helpers; still Rust tests over the same tree.
@task_item n=5 part=touches
Actionable-key rule and one actionable rule, and the actionable nodes listing and actions answer contract items (causes, fields): their stand guards had setup folded, same tests and messages.
@task_item n=6 part=touches
Password value masked and password never revealed, and the snapshot password value contract: their stand guards over MCP results now decode through the shared tool-result reader; same assertions.
@task_item n=7 part=touches
Snapshot text form parts and state tokens, and MCP tool results snapshot: their stand guards now read results through the shared MCP exchange and decoder.
@task_item n=8 part=touches
Driver click refusals covered and driver refusal causes disabled, driver actions disabled: the guards of the refusals had their edge, centre, viewport and serve copies removed for the shared ones.
@task_item n=9 part=touches
Snapshot diff differs and entries: the diff guard now settles each step through the shared held-task step, incremental and full modes compared by the shared check.
@task_item n=10 part=touches
Stand focus order asserted against a shared table and Tab and Shift+Tab move focus: the session state guard dropped its private focused reader for the shared module, same assertions.
@task_item n=11 part=touches
Synthetic click is a primary click at the centre: the transformed-element guard now asserts the landing through one private helper reading the shared centre.
@task_item n=12 part=touches
One snapshot model, many readers and UI graph mirrors the snapshot: guards over served answers moved to the shared serve and settle helpers; no reader changed.
@task_item n=13 part=touches
Any row naming a stand check target as its guard or witness: every target stands under its same name, one target a file, its test names unchanged — no guard renamed or moved.
@task_item n=1 part=untouched
Engine role mapping, accessible name precedence and ARIA property and state reads: no product source changed, so no mapping, read or state rule moved.
@task_item n=2 part=untouched
Contrast, colour, scrollbar and visual lint rows and their thresholds: no ratio, required value or finding field changed.
@task_item n=3 part=untouched
Keyboard contracts of the seven_guis cells grid, CRUD list, range input and single or multiple select: no seven_guis test or engine key handling changed.
@task_item n=4 part=untouched
Platform adapter, AccessKit feature defaults, accessibility tier and assistive technology reach: no manifest, feature or binary changed.
@task_item n=5 part=untouched
Cognitive accessibility and the absences rows (list motion, snapshot list-open state, adapter reach): no capability added or removed.
@task_item n=6 part=untouched
The obs log format and allowlist in the harness contract overview: the shared helpers add no print, subscriber, env read, log site or span.
