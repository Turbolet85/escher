@task_prose part=session
The session re-asserted keyboard reach, focus order and contrast (SC 2.1.1, 2.4.3, 1.4.3) on the stand and its served pages; it changed tests and the CI test list only, no engine, driver or page code.
The a11y CI leg widened in place from fourteen to eighteen named accessibility integration tests: the select list's three checks (the open list, the painted list, the stand acting on the list) and the keys-after-a-lost-box witness joined; no new job or required check; it read 18 result lines, 163 passed, 0 failed on CI.
It added a witness pinning a named focus-order limit: a focused control that no longer shows (hidden, display none, or hidden by its own focus-within) keeps engine focus and its keys, while the accessibility tree's focus names the window, and the next Tab moves to the control after it, in both render modes.
It measured the stand's one focusable node a page can unmount while focused: the Cells editor input, focused by a click and unmounted by Enter, drops engine focus to the root and the tree's focus to the window, and never reaches that limit.
Nothing was retired; no capability was claimed or removed.
@task_file n=1 path=.github/scripts/ci-leg.sh
what: the CI leg runner; edited the a11y leg's one cargo test line, its list of named accessibility integration tests, from fourteen to eighteen
@task_file n=2 path=.github/scripts/test_ci_workflows.py
what: the CI scripts' own tests; edited the pin on the a11y leg's target words in the legs-dispatch-to-cargo check
@task_file n=3 path=tests/blitz-tests/tests/key_press_after_container_lost_its_box.rs
what: the integration test of keys pressed after a focused control's container lost its box; edited to add the witness that a focused control that no longer shows keeps focus while the tree's focus names the window
@task_file n=4 path=tests/blitz-tests/tests/stand_keyboard.rs
what: the stand's keyboard reach tests over the seven GUIs pages; edited to add the check that the Cells editor unmounted by Enter drops focus and never reads the hidden-focus limit
@task_item n=1 part=touches
decision/boxless-focused-select-keeps-focus: a focused control with no box keeps focus and keys; the new witness widens that past the select to a button, pins the accessibility tree's focus on the window while engine focus stays, and the next Tab to the following control; its guard may gain the new test
@task_item n=2 part=touches
The hidden-focus limit (a focused control that no longer shows keeps focus, the tree naming the window) is now a named, tested SC 2.4.3 gap; any row stating focus or accessibility-tree focus of a control that lost its box is read against it
@task_item n=3 part=touches
contract/escher-session-tasks/cells and any row over the Cells page's editor: the editor input focused by a click and unmounted by Enter is measured to drop engine focus to the root and tree focus to the window
@task_item n=4 part=touches
contract/select-accessibility rows (open select, highlighted row, chosen option, open attribute) and decision/select-list-engine-overlay: the select list's accessibility checks now run in the a11y CI leg; their claims hold, a guard naming those checks stands
@task_item n=5 part=touches
contract/screenshot/select-list-row, contract/snapshot/select-list-row-bounds and decision/select-list-painted-after-root: the painted select list check joins the a11y leg; no claim moves
@task_item n=6 part=touches
Any row whose cell states the a11y leg's membership or its count (fourteen named accessibility tests) or its reading is stale: eighteen tests, 163 passed
@task_fact n=1 slug=surface
fact: A focused control that no longer shows (hidden, display none, or hidden by its own focus-within) keeps engine focus and its keys while the accessibility tree's focus names the window, and the next Tab moves to the control after it; a named SC 2.4.3 limit, in both render modes.
@task_fact n=2 slug=surface
fact: A focused node a page unmounts (the Cells editor input unmounted by Enter) drops engine focus to the root and the accessibility tree's focus to the window, never reaching the hidden-focus limit.
@task_item n=1 part=untouched
Color palette rows: engine colours, select list colours, colour scheme and base colour; no colour moved and contrast was re-asserted, not changed
@task_item n=2 part=untouched
Typography, spacing, depth and border radius rows: fonts, font fallback, type scale, margins, stacking and hit-testing are unchanged
@task_item n=3 part=untouched
Motion rows: the virtual clock, driver advance, scroll behaviour and the select list's open and wheel scroll are unchanged
@task_item n=4 part=untouched
Iconography rows: cursor icons and the favicon are untouched
@task_item n=5 part=untouched
The escher-session command, exit status, exit code, stdout, stderr and mcp role contracts, the actions, coverage and UI graph lines: no driver output changed
@task_item n=6 part=untouched
The select list's events (open, close, choose, highlight move) and its open gestures: no engine behaviour changed
@task_item n=7 part=untouched
WPT runner output and result colours, the shell provider, the JS window object, dioxus-native config and the web-spa surface: none reached
