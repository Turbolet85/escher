@task_prose part=session
The session re-asserted SC 2.1.1, 2.4.3 and 1.4.3 on the stand and its served pages; the CI accessibility leg widened in place from fourteen to eighteen named integration tests, adding the select list's three checks and the dropped-box key check, with no new job.
It added a test pinning a named SC 2.4.3 gap: a focused control that no longer shows keeps engine focus while the accessibility tree names the window, and the next Tab moves on, in both modes and three ways of hiding.
It measured the stand's one focusable node a page can unmount while focused: the Cells editor cell-input, focused by a click and unmounted by Enter, leaves focus on the root html and the tree's focus on the window, and never reads the hidden-focus limit; a new stand keyboard test pins it.
No stand markup, id, layout, colour, viewport, driver answer, CLI or public symbol changed; tests and the leg list alone.
@task_file n=1 path=.github/scripts/ci-leg.sh
what: the CI leg runner; its a11y leg names the accessibility integration tests run on one cargo test line (now eighteen)
@task_file n=2 path=.github/scripts/test_ci_workflows.py
what: the CI scripts check that pins each leg's cargo targets, the a11y tuple among them
@task_file n=3 path=tests/blitz-tests/tests/key_press_after_container_lost_its_box.rs
what: the engine check for a key press after a focused control's container lost its box (the dropped anonymous block), now also the hidden-focus limit's witness
@task_file n=4 path=tests/blitz-tests/tests/stand_keyboard.rs
what: the stand keyboard check that drives every stand control by key in both modes, now also the Cells editor unmounted while focused
@task_item n=1 part=touches
contract/stand-element-ids/cell-input: the Cells editor is now measured as the stand's one focusable node a page unmounts while focused (click focuses it, Enter unmounts it); focus drops to the root html and the tree's focus to the window, pinned by a new stand keyboard test; something belongs beside this row.
@task_item n=2 part=touches
decision/every-stand-control-keyed: the stand keyboard check it rests on gains a test (23 to 24) driving the Cells editor by click and Enter; touch only where its guard or cell names that check or holds its count.
@task_item n=3 part=touches
contract/stand-element-ids/cells-grid: Enter on the grid's editor is now pinned as unmounting the focused editor and dropping focus to the root, not keeping it on a hidden control.
@task_item n=4 part=touches
decision/stand-a11y-by-attributes-alone: the stand's tree focus reading after the focused editor unmounts is now measured as the window; touch only where the row states what the tree names for focus.
@task_item n=5 part=touches
decision/flight-type-select-boot-reading: the select list's checks now run in the CI accessibility leg; touch only where its guard or cell names the leg or the tests it runs in.
@task_item n=6 part=touches
decision/open-list-rows-own-bounds: the open select list's painted and popup checks join the CI accessibility leg; touch only where the row names where those checks run.
@task_fact n=1 slug=surface-desktop-native
fact: The stand's one focusable node a page can unmount while focused is the Cells editor cell-input (focused by a click, unmounted by Enter); its removal moves focus to the root html and the accessibility tree's focus to the window, so the stand never meets the hidden-focus limit.
@task_item n=1 part=untouched
The CLI surface rows: agent-run, cold-agent, escher-session actions, coverage, exit codes, graph, help, host log, layout, lint, MCP server, recording, refusals, screenshot, served tasks and snapshot answers; no driver answer, verb, exit code or bound moved.
@task_item n=2 part=untouched
The ShellProvider window chrome controls; no window chrome call changed.
@task_item n=3 part=untouched
The stand stable element ids other than cell-input and cells-grid; no id was added, renamed or freed.
@task_item n=4 part=untouched
The stand accent colour, pinned viewport, fixed document skeleton, lints-clean-at-boot, held-instance boot, session-roles boot and load-page network decisions; no markup, style or boot path changed.
@task_item n=5 part=untouched
The scroll-into-view, visible-region-through-clipping and CRUD list keyboard region decisions; no scroll, clip or list behaviour changed.
@task_item n=6 part=untouched
The multi_doc and paint_bench command lines and the absence of arbitrary index.html templates; nothing there moved.
@task_item n=7 part=untouched
The mobile-native and web-spa surfaces and the decisions log; the session reached none.
