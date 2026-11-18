@task_prose part=session
The session re-asserted SC 2.1.1, 2.4.3 and 1.4.3 on the stand and its served pages, with tests and a CI leg list alone; no engine, driver or app code changed.
The a11y CI leg widened in place from fourteen to eighteen named accessibility integration test targets: the select list's three checks (select popup, painted select popup, stand popup acting) and the key-press-after-dropped-anonymous-block file joined; no new job, required check or workflow edit; stand-repeat and embolden arms unchanged.
The leg read 18 result lines, 163 passed, 0 failed, 0 ignored, locally and on CI.
A named SC 2.4.3 gap is now pinned by a test: a focused control whose box goes away (hidden, display none, or hidden by a focus-within rule) keeps engine focus while the accessibility tree's focus names the window, and the next Tab moves on to the next control; in both layout modes.
A stand check measured that the Cells editor input, focused by a click and unmounted by Enter, drops focus to the root element with the tree's focus on the window, and that removing the focus drop on removal of a focused node reddens it.
The capability statement's accessibility reach is now stale (fourteen files, a 0.2.0 CI reading, the hidden-focus limit unnamed); a carry was written for it, not edited here.
@task_file n=1 path=.github/scripts/ci-leg.sh
what: the one CI leg runner, its a11y leg holding the list of named accessibility integration test targets run on one locked cargo test line (edited: four targets added)
@task_file n=2 path=.github/scripts/test_ci_workflows.py
what: the CI workflow contract tests run by the ci-scripts leg, pinning each leg's cargo invocation and the a11y leg's target tuple (edited: the tuple pin widened to the eighteen targets)
@task_file n=3 path=tests/blitz-tests/tests/key_press_after_container_lost_its_box.rs
what: the blitz-tests integration check that a key press after a container's anonymous block is dropped is safe, now also the gap witness that a focused control that no longer shows keeps focus with the accessibility tree's focus naming the window (edited: one test and two private helpers added)
@task_file n=4 path=tests/blitz-tests/tests/stand_keyboard.rs
what: the seven_guis stand's keyboard-navigation and focus-order accessibility checks (SC 2.1.1, 2.4.3), in both layout modes (edited: one test added for the Cells editor unmounted by Enter while focused)
@task_item n=1 part=touches
contract/ci-leg-legs/a11y states fourteen named accessibility integration tests; the leg now runs eighteen (select popup, painted select popup, stand popup acting and the key-press-after-dropped-anonymous-block file joined).
@task_item n=2 part=touches
decision/a11y-leg-widens-without-new-job holds and gains a second instance: the leg widened in place again with no new job, required check or workflow edit; its guard names the CI workflow tuple test, whose a11y pin moved from 16 to 20 words.
@task_item n=3 part=touches
decision/one-ci-leg-runner, contract/ci-leg-script/exit-status and contract/ci-leg-script/leg-log rule over the runner whose a11y arm changed; its other arms (stand-repeat, embolden, fast, ci-scripts) are unchanged and the log now carries 18 result lines.
@task_item n=4 part=touches
decision/ci-contract-pinned and contract/ci-leg-legs/ci-scripts: the CI workflow contract tests changed their a11y target pin; the pin still holds the leg's whole invocation.
@task_item n=5 part=touches
decision/locked-cargo-legs and decision/cargo-legs-locked: the a11y leg still runs its eighteen targets on one cargo test --workspace --locked line.
@task_item n=6 part=touches
contract/accessibility-tree/focus: measured that the tree's focus names the window (TreeNodeId u64::MAX) while engine focus stays on a control whose box went away, and names the window when a focused node is unmounted; a limit the row may need to state.
@task_item n=7 part=touches
decision/focus-fixup-without-events, decision/inert-focus-fixup and contract/dom-events/inert-focus-fixup: focus fixup reads inert alone, so a focused control that loses its box (hidden or display none) keeps engine focus; the new test pins that limit as a named SC 2.4.3 gap, the next Tab moving on to the following control.
@task_item n=8 part=touches
decision/removed-node-state-cleared, decision/interaction-state-cleared-on-removal, decision/removed-node-interaction-state and decision/interaction-teardown-webkit-blink: a new stand witness shows a focused node removed from the document drops focus to the root element; removing the focus drop on removal reddens it.
@task_item n=9 part=touches
decision/cells-grid-single-tab-stop: the Cells grid's editor input, focused by a click and unmounted by Enter, is the stand's one focusable node a page can unmount while focused; it now has a keyboard check that focus drops and the hidden-focus limit is never reached on the stand.
@task_item n=10 part=touches
decision/anonymous-block-drop-clears-layout-parent, decision/anonymous-block-dealloc-detaches and decision/anonymous-blocks-in-layout-children-only: the integration check for a key press after a dropped anonymous block gained a test and joined the a11y leg.
@task_item n=11 part=touches
decision/integration-test-one-behavior, decision/blitz-tests-one-target-per-file and decision/integration-tests-in-blitz-tests: the key-press-after-dropped-anonymous-block file now also holds the hidden-focus gap witness; still one target per file.
@task_item n=12 part=touches
decision/fixture-precondition-first and decision/tests-assert-fixture-first: an exception measured — under the focus-within way the control's box goes as Tab focuses it, so no shown-state precondition exists; that way asserts the anonymous block and Tab focusing the control instead.
@task_item n=13 part=touches
decision/scenarios-both-layout-modes, decision/both-layout-modes and decision/layout-modes-identical: both new tests run in both layout modes and read the same.
@task_item n=14 part=touches
decision/capability-statement, contract/capability-statement/sections, decision/capabilities-doc-beside-upstream and decision/measured-limits-stated-in-the-tool: the capability statement's accessibility reach is stale (fourteen files, a 0.2.0 CI reading) and does not name the hidden-focus limit; a carry is owed on it.
@task_item n=15 part=touches
decision/select-list-painted-in-scene, decision/select-list-painted-overlay and contract/paint/select-list-order: the painted select list's check and the select popup checks now run in the a11y leg.
@task_item n=16 part=touches
decision/stand-typography-dejavu-sans and decision/stand-uses-bundled-font: SC 1.4.3 contrast re-asserted on the stand's served pages with its bundled font, now inside the a11y leg.
@task_item n=17 part=touches
decision/accesskit-accessibility-tree and decision/a11y-rebuild-on-nonempty-changed-set: tests now read the accesskit tree's focus directly (accesskit was already a blitz-tests dev-dependency).
@task_item n=18 part=touches
decision/stand-repeat-zero-failure-budget, decision/stand-repeat-zero-failure, contract/ci-leg-legs/stand-repeat and contract/ci-leg-legs/embolden: their arms of the runner are unchanged; the rows hold.
@task_fact n=1 slug=established-decisions
fact: A focused control that no longer shows (its box gone by visibility hidden, display none, or a focus-within rule) keeps engine focus while the accessibility tree's focus names the window, and the next Tab moves on to the following control; this SC 2.4.3 gap is a named, test-pinned limit.
@task_fact n=2 slug=established-decisions
fact: A focused node removed from the document drops engine focus to the root element and the accessibility tree's focus to the window; on the stand only the Cells editor input can be unmounted while focused, so the hidden-focus limit is never reached there.
@task_item n=1 part=untouched
Driver, session socket, MCP role and command-line contracts (verbs, answers, refusal causes, snapshot keys, kept checks): no driver or carrier changed.
@task_item n=2 part=untouched
Engine layout, paint, style and scrolling decisions: no engine code changed, only tests and a CI leg list.
@task_item n=3 part=untouched
WPT runner, browser app, rdme and example contracts and decisions: none reached.
@task_item n=4 part=untouched
Telemetry, logging and scrubbing rows: nothing logged or installed changed.
@task_item n=5 part=untouched
Stack, dependency, toolchain, coverage-floor and publish rows: no dependency, tool version, coverage floor or manifest changed.
@task_item n=6 part=untouched
Occupied names and resources (binaries, crates, environment variables, listeners, state directories): no name taken or freed.
@task_item n=7 part=untouched
Cold-agent run and recording rows: not exercised or changed.
