@task_prose part=session
The session re-asserted SC 2.1.1, 2.4.3 and 1.4.3 on the stand and its served pages: the a11y CI leg widens in place from fourteen to eighteen named accessibility integration test targets, joining the select list's three checks and the dropped-anonymous-block key-press witness; no new job, required check or workflow edit.
It added two tests and no production code: no public symbol, verb, endpoint, port, socket, env var, dependency, schema or config moved, and nothing was retired.
It measured and pinned a named SC 2.4.3 limit: a focused control that no longer shows (hidden, display none, or hidden by focus-within) keeps engine focus while the accessibility tree's focus names the window, and the next Tab moves on to the following control, in both incremental and full modes.
It measured the stand's one focusable node a page can unmount while focused (the Cells editor input, focused by a click and unmounted by Enter): engine focus falls to the root html element and the tree's focus to the window; dropping the focused-node clear on removal reddens it.
The a11y leg read 18 result lines, 163 passed, 0 failed, 0 ignored, locally and on CI (17 of 17 checks green); the CI workflow test's a11y tuple pin grows from 16 to 20 words.
@task_file n=1 path=.github/scripts/ci-leg.sh
what: edited — the CI leg dispatcher; its a11y leg, the locked cargo test line of named accessibility integration tests, widened from fourteen to eighteen targets; the stand-repeat and embolden arms unchanged
@task_file n=2 path=.github/scripts/test_ci_workflows.py
what: edited — the CI workflow and leg-script checks; the pin of each gate leg's cargo dispatch, its a11y tuple now the eighteen target words
@task_file n=3 path=tests/blitz-tests/tests/key_press_after_container_lost_its_box.rs
what: edited — the integration test of a key press after a focused control's container lost its box (the dropped anonymous block clearing its children's parent); gains the test that a focused control that no longer shows keeps focus with the accessibility tree's focus naming the window
@task_file n=4 path=tests/blitz-tests/tests/stand_keyboard.rs
what: edited — the stand's keyboard integration tests (the Cells grid keyboard model and focus on the stand's pages); gains the test that the Cells editor input unmounted by Enter drops focus to the root and never reads the hidden-focus limit
@task_item n=1 part=touches
decision/dropped-anonymous-block-clears-parent — its witness test file gains a fourth test pinning what follows a dropped box under a focused control (focus kept, tree focus on the window, next Tab moves on); the guard's file is unchanged by name, the claim holds, but the row may take the new test beside its guard.
@task_item n=2 part=touches
decision/accessibility-focus-built-nodes-only — indirect, through the accessibility tree's focus reading: the session measured that when the focused control has no built box the tree's focus names the window (TreeNodeId u64::MAX) while engine focus stays on the control; a row stating focus always names a built node gets this stated limit beside it.
@task_item n=3 part=touches
decision/accessibility-tree-refresh-on-change — indirect: the new tests read the tree's focus after a box is dropped and after a focused node is unmounted, both refreshes on change; the row's guard may take them.
@task_item n=4 part=touches
decision/cells-grid-keyboard — indirect, through the stand's keyboard tests: Enter on the Cells editor input unmounts the focused node and focus falls to the root html element, the tree's focus to the window; a row on the Cells grid keyboard model gets that fact or the new test beside its guard.
@task_item n=5 part=touches
decision/stale-node-ids-never-panic — indirect: the unmounted focused node is the removed-focused-node path, whose clear of the document's focused node the control run proved load-bearing; a row whose guard names the stand keyboard tests may take the new test.
@task_item n=6 part=touches
decision/inert-focus-clear-silent and decision/inert-subtree — indirect, through focus fix-up: the hidden-focus limit is the case the inert-only fix-up does not cover (a control that no longer shows is not cleared); a row stating when focus is cleared gets that limit beside it.
@task_item n=7 part=touches
decision/select-list-open-gate, decision/select-list-keys-and-painted-labels, decision/select-list-live-reader and the other select list rows — indirect: the select list's three integration checks (select popup, select popup painted, stand act popup) now run on every push in the a11y leg; a row whose guard or why names those tests or where they run may record the leg.
@task_item n=8 part=touches
decision/enter-activates-focused-control and decision/button-click-focuses — indirect: the hidden-focus test drives a button control by Tab, and the Cells test focuses by click and activates by Enter; their guards may take the new tests.
@task_fact n=1 slug=input-validation
fact: A focused control that no longer shows (hidden, display none, or hidden through focus-within) keeps engine focus while the accessibility tree's focus names the window, and the next Tab moves to the following control; this is a named SC 2.4.3 limit, not a fault, in both incremental and full modes.
@task_fact n=2 slug=input-validation
fact: Removing a focused node clears the document's focused node: engine focus falls to the root html element and the accessibility tree's focus to the window, so no stale focus reference outlives the node.
@task_item n=1 part=untouched
The driver session socket, its closed request grammar, replies, bounds, idle expiry and owner-only access: no verb, reply, refusal cause or bound moved.
@task_item n=2 part=untouched
The MCP server over stdio, its protocol errors, tool results, size bounds and tool_call span: nothing in the MCP role changed.
@task_item n=3 part=untouched
Masking of password and file values, the snapshot text, the stable element id and its carriers, the actions and graph answers: no carrier or field changed.
@task_item n=4 part=untouched
Outbound requests, script fetchers, file and data URL rules, CORS and same-origin absences, nesting limits and navigation schemes: no network or loading path changed.
@task_item n=5 part=untouched
Kept checks, recorded checks, recording directories, cold-agent runs and the coverage answer line: no recorder, check file or coverage role changed.
@task_item n=6 part=untouched
Dependency choices and pins, the audit and licence gates, and locked builds as a rule: no dependency was added or moved (accesskit was already a test dev-dependency).
@task_item n=7 part=untouched
Secret management, CI tokens, publish and signing guards, and the telemetry and log sinks and their allowlists: no secret, token, log target or field changed.
@task_item n=8 part=untouched
Authentication and authorization absences, data protection of history, cookies and cache, and the error and refusal tables: none reached.
