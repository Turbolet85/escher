@task_prose part=session
The session changed engine focus: after each layout, a focused control that no longer shows gives focus up silently, with no blur or focusout, and the changed set marks it.
Tab and Shift+Tab now skip a node that does not show, an engine change for every document on the founder's word; a control whose own :focus-within rule hides its container still takes Tab and loses it at the next layout.
It retired the stated limit that a focused control that no longer shows keeps focus and takes keys; a boxless focused select now gives focus up and a key reaches it only after a re-focus.
It re-stated the MountedData-during-render remnant to the reachable case and measured that a task waking its own waker during a render livelocks inside that render, now a stated limit.
It added a per-document seed for the script document's Math.random, and pruned dropped error boundaries before each render so the boundary registry stays bounded.
The a11y CI leg grew from eighteen to nineteen files, reading 19 · 177 · 0 · 0; CI on the pre-CI commit read green.
@task_file n=1 path=tests/blitz-tests/tests/focus_fixup_unshown.rs
what: new integration test of the post-layout focus fixup and the Tab and Shift+Tab skip of controls that do not show, in both layout modes
@task_file n=2 path=packages/blitz-vibey-script/src/seeded_random.rs
what: new crate-private module of the script runtime holding the seeded random sequence behind Math.random
@task_file n=3 path=packages/blitz-vibey-script/tests/seeded_random.rs
what: new test that one seed reads one Math.random sequence, two seeds two, and an unseeded document the host's
@task_file n=4 path=packages/blitz-dom/src/node/node.rs
what: edited the engine DOM node, gaining the under-display-none and shows readings
@task_file n=5 path=packages/blitz-dom/src/layout_reason.rs
what: edited the engine's layout-reason reading of whether a node has a box
@task_file n=6 path=packages/blitz-dom/src/document.rs
what: edited the engine document: the post-layout giving-up of focus by a control that does not show, and the Tab and Shift+Tab focus walk
@task_file n=7 path=packages/blitz-dom/src/resolve.rs
what: edited the engine's resolve pass, which now runs the focus fixup right after reconciling the open select list
@task_file n=8 path=packages/blitz-vibey-script/src/lib.rs
what: edited the script runtime crate root, declaring the seeded random module
@task_file n=9 path=packages/blitz-vibey-script/src/state.rs
what: edited the script runtime state, gaining the random state
@task_file n=10 path=packages/blitz-vibey-script/src/runtime.rs
what: edited the script runtime, which installs the seeded Math.random in the page realm
@task_file n=11 path=packages/blitz-vibey-script/src/document.rs
what: edited the script document API, gaining the with_seed builder
@task_file n=12 path=packages/dioxus-native-dom/src/boundary.rs
what: edited the dioxus-native error boundary registry and its boundary marks, now pruned of dropped boundaries
@task_file n=13 path=packages/dioxus-native-dom/src/dioxus_document.rs
what: edited the dioxus-native document's poll, which prunes the boundary registry right before each render
@task_file n=14 path=tests/blitz-tests/tests/key_press_after_container_lost_its_box.rs
what: edited the test of a key pressed on a focused control whose container lost its box, now re-focusing before the key
@task_file n=15 path=tests/blitz-tests/tests/dioxus_runtime_probe.rs
what: edited the dioxus runtime probe test of MountedData calls during a render, with a stored-waker wait
@task_file n=16 path=.github/scripts/ci-leg.sh
what: edited the CI leg script; the a11y leg's test list gains one file
@task_file n=17 path=.github/scripts/test_ci_workflows.py
what: edited the check of the CI workflows, pinning the a11y leg's file count
@task_file n=18 path=.github/scripts/test_capability_statement.py
what: edited the check of the capability statement, pinning the a11y leg's file count
@task_file n=19 path=docs/capabilities.md
what: edited the capability statement: its focus limits, the MountedData-during-render remnant, the busy-yield limit and the a11y leg's count and reading
@task_item n=1 part=touches
decision/boxless-focused-select-keeps-focus is now stale: a focused select with no box gives focus up after the next layout, and a key reaches it only after it is re-focused.
@task_item n=2 part=touches
contract/mounteddata-during-render/document-state is stale: the remnant that a node created in the same render cannot be focused is re-stated to the reachable case; a task polled in the render that removes its target leaves focus off it.
@task_item n=3 part=touches
contract/mounteddata-during-render/set-focus-to: a task awaiting the handle of a node its handler shows now focuses it; the row stands but gains the busy-yield limit beside it.
@task_item n=4 part=touches
decision/buffered-render-replay: the render buffered then replayed stands, but a task that wakes its own waker while it waits livelocks inside that render, a limit now stated beside it.
@task_item n=5 part=touches
Any row holding that a focused control which no longer shows keeps focus and takes keys is now false: focus is given up silently after each layout, with no blur or focusout.
@task_item n=6 part=touches
contract/driver-verbs-and-virtual-clock/press and contract/driver-verbs-and-virtual-clock/type, indirectly: a key or text sent after a layout that hid the focused control no longer reaches that control, since focus has left it.
@task_item n=7 part=touches
decision/single-select-open-gestures, indirectly: Enter or Space on a select that lost its box no longer opens or acts on it until it is re-focused.
@task_item n=8 part=touches
decision/select-list-engine-overlay and decision/selects-that-open-no-list, indirectly: the focus fixup now runs right after the open select list is reconciled in the resolve pass, so a hidden focused select gives focus up after its list is reconciled.
@task_item n=9 part=touches
contract/escher-session-commands/snapshot and the contract/select-accessibility rows, indirectly: the focused node the snapshot and accessibility tree report moves off a control that stopped showing after the next layout.
@task_item n=10 part=touches
Keyboard focus order: Tab and Shift+Tab now skip a node that does not show, an engine change for every document, with the :focus-within exception.
@task_fact n=1 slug=surface
fact: After each layout, a focused control that no longer shows (out of the document, under display: none, or not visibility: visible) gives focus up silently, with no blur or focusout, and the changed set marks it; a display: contents control and one beside a hidden sibling keep focus.
@task_fact n=2 slug=surface
fact: Tab and Shift+Tab skip a node that does not show, for every document, on the founder's word; the exception is a control whose own :focus-within rule hides its container, which still takes Tab and loses focus at the next layout.
@task_fact n=3 slug=surface
fact: A task that wakes its own waker while it waits during a render livelocks inside that render; a task waiting for a later render must leave its waker for the producer, a limit stated and asserted by no check.
@task_item n=1 part=untouched
Brand identity, color palette, engine colours, select list colours and color-scheme rows: no colour value or token moved.
@task_item n=2 part=untouched
Typography, font fallback, font context and spacing rows: no font, type or spacing rule changed.
@task_item n=3 part=untouched
Depth, stacking, paint order and hit-testing rows: no paint or hit-test path changed.
@task_item n=4 part=untouched
Motion rows (virtual clock, scroll, scrollbar fades, smooth scroll, animation): the seeded random touches no clock or animation.
@task_item n=5 part=untouched
escher-session exit statuses, stdout, stderr, MCP role, tasks, actions, coverage, graph and layout answers: no command, output or exit code changed.
@task_item n=6 part=untouched
WPT runner output and result colours, shell provider, cursor and favicon, JS window sizes and the web-spa canvas surface: none reached.
@task_item n=7 part=untouched
Select list placement, rows, colours, events and screenshot or snapshot row bounds: the select list itself is unchanged.
