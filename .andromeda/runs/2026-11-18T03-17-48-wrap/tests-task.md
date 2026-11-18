@task_prose part=session
The a11y CI leg widens in place from fourteen to eighteen named test targets: the select list's three checks (select_popup, select_popup_painted, stand_act_popup) and the dropped-anonymous-block key witness join it; no new job, required check or workflow edit; stand-repeat and embolden arms unchanged.
SC 2.1.1, 2.4.3 and 1.4.3 re-asserted on the stand and its served pages: the leg reads 18 result lines, 163 passed, 0 failed, 0 ignored, locally and on CI (was 14 lines, 115 passed).
Added a test pinning the hidden-focus limit as a named SC 2.4.3 gap: a focused control that no longer shows keeps engine focus, the accessibility tree's focus names the window, and the next Tab moves to the following control; in both layout modes and three hiding ways (hidden, display none, focus-within).
Added a stand test: the Cells editor, focused by a click and unmounted by Enter, drops focus to the root html and the tree's focus to the window, so the stand never reaches the limit.
Both new tests were controlled red-before-green (tree-focus control, removed-node focus-drop control; both held). The CI scripts' a11y target-word pin moved 16 to 20 words. Nothing retired; no public symbol, dependency or config changed.
@task_file n=1 path=.github/scripts/ci-leg.sh
what: edited — the CI leg script every CI leg runs through; its a11y leg's one cargo test line gains four --test targets (fourteen to eighteen)
@task_file n=2 path=.github/scripts/test_ci_workflows.py
what: edited — the CI Python scripts' unittest checks; the a11y leg's target tuple pin in the gate-legs dispatch test grows to the eighteen targets
@task_file n=3 path=tests/blitz-tests/tests/key_press_after_container_lost_its_box.rs
what: edited — the integration test target for a key press after a dropped anonymous block; gains the hidden-focus limit witness and two private helpers, and now sits in the a11y leg as well
@task_file n=4 path=tests/blitz-tests/tests/stand_keyboard.rs
what: edited — the stand's keyboard integration checks run in both layout modes inside stand-repeat; gains the Cells editor unmount-while-focused check
@task_item n=1 part=touches
ci-integration decision/a11y-leg-membership goes stale: it names the tier-0 witnesses and the five tree-reading checks; the select list's three checks and the dropped-anonymous-block witness now join, eighteen files.
@task_item n=2 part=touches
ci-integration decision/joining-file-keeps-its-legs: four files joined the a11y leg while keeping the legs they already ran in.
@task_item n=3 part=touches
ci-integration decision/tally-move-attributed-by-test-name: the a11y leg's tally moved 115 to 163 passed and 14 to 18 result lines, attributed by file and test name.
@task_item n=4 part=touches
test-scope-summary decision/key-after-dropped-anonymous-block: its witness file gains a fourth test pinning the hidden-focus limit; the row's guard file holds a new test.
@task_item n=5 part=touches
test-scope-summary decision/stand-checks-in-both-layout-modes and integration-test-strategy decision/layout-tests-in-both-layout-modes: both new tests loop over both layout modes.
@task_item n=6 part=touches
unit-test-strategy decision/mutation-controlled-tests: both new tests were controlled red-before-green, one control per test, the control's case naming the test function.
@task_item n=7 part=touches
4-unit-test-strategy decision/ci-legs-through-leg-script and 1-test-scope-summary decision/ci-scripts-unit-tested: the leg script's a11y line and its unittest tuple pin both moved.
@task_item n=8 part=touches
test-strategy decision/requirement-witness-inside-stand-repeat, 10-quality-gates-coverage-targets decision/stand-checks-zero-failure-budget and quality-gates-coverage-targets decision/zero-failure-stand-repeat: the stand keyboard target inside stand-repeat gains a test (23 to 24), still held to zero failures.
@task_item n=9 part=touches
1-test-scope-summary decision/capability-statement-pinned, test-scope-summary decision/capabilities-doc-pinned-to-schema and unit-test-strategy decision/capability-statement-from-schema: the capability statement's Accessibility reach section is stale (fourteen files, an old run reading, the hidden-focus limit unnamed); a carry is written, no claim changed in this session.
@task_item n=10 part=touches
3-test-harness-contract contract/harness-inspect/focused: the new tests read the harness's focused reading after a focused node is hidden or unmounted (stays on the control; drops to the root html).
@task_item n=11 part=touches
2-test-strategy decision/stand-checks-bundled-font-no-skip and test-strategy decision/stand-checks-bundled-font: the a11y leg now runs the select list's painted check and the stand popup check on the bundled font.
@task_item n=12 part=touches
test-scope-summary decision/one-test-target-per-file and test-strategy decision/one-test-target-per-file: no new target; the new tests sit in their behaviour's existing files.
@task_fact n=1 slug=integration-test-strategy
fact: The hidden-focus limit is pinned as a named SC 2.4.3 gap: a focused control that no longer shows keeps engine focus, the accessibility tree's focus names the window, and the next Tab reaches the following control, asserted in both layout modes and for hidden, display none and focus-within hiding.
@task_fact n=2 slug=test-data-fixtures
fact: The stand's one focusable node a page can unmount while focused is the Cells editor; unmounted by Enter, focus drops to the root html and the tree's focus to the window, so no stand screen reaches the hidden-focus limit.
@task_item n=1 part=untouched
Session host, session wire, session errors, escher-session command line, MCP role and CLI contracts: no session, protocol, exit code or answer shape changed.
@task_item n=2 part=untouched
Driver verbs, refusal causes, lint defects, snapshot and driver JSON answers: no driver surface changed.
@task_item n=3 part=untouched
Agent-run and cold-agent contracts, events, verdicts and exit codes: untouched.
@task_item n=4 part=untouched
WPT runner, WPT status set and WPT CI decisions: untouched.
@task_item n=5 part=untouched
Coverage floors, coverage-floor exit codes, mutation baseline and complexity ceiling rows: no floor, threshold or exclusion moved.
@task_item n=6 part=untouched
CI job topology, triggers, caches, action pinning, permissions, secrets, package installs, disk freeing and MSRV: no workflow or job changed.
@task_item n=7 part=untouched
Harness construction, settle, virtual clock, screenshot and contrast contracts: no harness API or behaviour changed.
@task_item n=8 part=untouched
Mocking and stand-in discipline, timer ticks and the advance command: untouched.
