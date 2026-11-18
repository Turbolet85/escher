@task_prose part=session
The session re-asserted SC 2.1.1, 2.4.3 and 1.4.3 on the stand and its served pages after the engine debts, the select list and bundled fonts, with every check in the a11y CI leg.
The a11y leg widened in place from fourteen to eighteen test targets: the select list's three checks (select_popup, select_popup_painted, stand_act_popup) and the key-after-dropped-anonymous-block file joined; no new job, required check or workflow edit; stand-repeat and embolden unchanged.
The leg now reads 18 result lines, 163 passed, 0 failed, 0 ignored, on CI and locally (was 14 lines, 115 passed).
It pinned the hidden-focus limit as a named SC 2.4.3 gap: a focused control that no longer shows (hidden, display none, or display none by focus-within) keeps engine focus, the accessibility tree's focus names the window, and the next Tab goes to the following control, in both render modes.
It measured that the stand's one focusable node a page can unmount while focused, the Cells editor focused by a click and unmounted by Enter, drops engine focus to the root and tree focus to the window, never reading the limit.
No engine code, public symbol, capability or dependency changed; only tests and the leg's target list.
@task_file n=1 path=.github/scripts/ci-leg.sh
what: the CI leg dispatcher; its a11y leg's one cargo test line naming the accessibility integration test targets, now eighteen
@task_file n=2 path=.github/scripts/test_ci_workflows.py
what: the CI scripts' own checks; the a11y leg's pinned tuple of target words (16 to 20 words) in the leg-dispatch check
@task_file n=3 path=tests/blitz-tests/tests/key_press_after_container_lost_its_box.rs
what: the key-after-dropped-anonymous-block checks; gains the witness that a focused control that no longer shows keeps focus with the tree naming the window, and the next Tab reaches the following control
@task_file n=4 path=tests/blitz-tests/tests/stand_keyboard.rs
what: the stand keyboard checks (focus order, Tab, Cells grid keys); gains the witness that the Cells editor unmounted by Enter while focused drops focus and never reads the hidden-focus limit
@task_item n=1 part=touches
decision/focus-fixup-reads-inert-alone: its stated limit (a focused control that no longer shows keeps focus) is now pinned; its guard gains a_focused_control_that_no_longer_shows_keeps_focus_the_tree_naming_the_window, measured as engine focus on the control, tree focus on the window, next Tab to the following control, both modes, all three ways of hiding.
@task_item n=2 part=touches
decision/removed-focused-node-drops-focus: newly witnessed on the stand by a_focused_cells_editor_unmounted_by_enter_drops_focus_and_never_reads_the_limit; the Cells editor unmounted by Enter leaves engine focus on the root and tree focus on the window; reddens when the focus drop is removed.
@task_item n=3 part=touches
decision/a11y-test-leg: the slow-tier accessibility leg now runs eighteen targets, 163 passed; touch only where the row states membership or a count.
@task_item n=4 part=touches
decision/a11y-ci-leg: the a11y CI leg's target list widened from fourteen to eighteen in place, the select list's three checks and the key-after-dropped-anonymous-block file joining; any stated count or membership is stale.
@task_item n=5 part=touches
decision/a11y-ci-gate: the checks that gate CI under the Accessibility (a11y) tests job grew to eighteen files with no new required check.
@task_item n=6 part=touches
decision/select-list-coverage-headless: the select list's headless checks (select_popup, select_popup_painted, stand_act_popup) now run in the a11y leg.
@task_item n=7 part=touches
contract/accessibility-tree/focus: the tree's focus is measured naming the window while engine focus stays on a control that no longer shows, and after the focused Cells editor is unmounted.
@task_item n=8 part=touches
decision/stand-contrast-asserted: SC 1.4.3 re-asserted on the stand through the widened leg; a true claim sharing the leg's name, touch only if it states the leg's count.
@task_item n=9 part=touches
contract/cells-grid/editor-enter: Enter in the Cells editor unmounts the focused editor; the new witness reads focus dropping to the root rather than lingering.
@task_item n=10 part=touches
decision/tab-and-shift-tab-move-focus: under the hidden-focus gap, Tab from a focused control that no longer shows moves to the next focusable control.
@task_item n=11 part=touches
decision/stand-shared-tab-order-table and decision/stand-checks-repeated-three-rounds: the stand keyboard file they guard gains one test (23 to 24); touch only where a row holds that count.
@task_fact n=1 slug=keyboard-navigation
fact: The stand's only focusable node a page can unmount while focused is the Cells editor (focused by a click, unmounted by Enter); it drops engine focus to the root and tree focus to the window, so no stand screen reaches the hidden-focus limit.
@task_fact n=2 slug=keyboard-navigation
fact: Named SC 2.4.3 gap: a focused control that stops showing (hidden, display none, or hidden by its own focus-within) keeps engine focus while the accessibility tree's focus names the window, and the next Tab moves on to the following control, in both render modes.
@task_item n=1 part=untouched
Contrast lint rows (lint contrast finding contract, contrast formula, exempt disabled controls, scrollbar thumb stroke, WCAG ratio helper): no contrast code or threshold changed.
@task_item n=2 part=untouched
ARIA property reads, accessibility tree states and role mapping rows: no role, name or state reading changed.
@task_item n=3 part=untouched
Snapshot, snapshot text, snapshot diff, actions answer, driver screenshot and MCP result contracts: no driver or snapshot surface changed.
@task_item n=4 part=untouched
Range input, select open and closed list key contracts, multiple select and CRUD list contracts: their behaviour is unchanged and stays pinned by standing tests.
@task_item n=5 part=untouched
Platform adapter, accessibility feature defaults and the assistive-technology reach absence: no adapter, feature flag or dependency changed.
@task_item n=6 part=untouched
Cognitive accessibility, UI graph state registration and Dioxus reached-handler contracts: nothing in those surfaces moved.
