@task_prose part=session
The engine now gives focus up, silently and with no blur or focusout, after a layout when the focused control no longer shows (display none on it or an ancestor, or visibility not visible), and the changed set marks that node.
Tab and Shift+Tab now skip a node that does not show, an engine change for every document on the founder's word; a control whose own focus-within rule hides its container still takes Tab and loses it at the next layout, a stated limit.
A script document can be seeded in building code, and its page Math.random then reads one fixed sequence per seed; an unseeded document reads the host's.
The Dioxus error-boundary registry is pruned of dropped boundaries before each render, so it stays bounded (length 3 measured over 20 remounts) and a boundary dropped in a step still reads until the next render.
The a11y CI leg gains one integration target and goes from eighteen to nineteen files, its reading from 18 · 163 · 0 · 0 to 19 · 177 · 0 · 0; the capability statement and the two CI-script tests that pin that count moved with it.
A test task that waits for a later render on the Dioxus bridge was measured to livelock when it wakes its own waker; the bridge probe now waits on a stored waker woken by the producer, and the busy-yield hang is stated as a limit no check asserts.
@task_file n=1 path=tests/blitz-tests/tests/focus_fixup_unshown.rs
what: new blitz-tests integration target, one per behaviour, run in both layout modes: a focused control that stops showing gives focus up with no blur, display contents and a hidden sibling keep focus, a focus-within rule case, and the Tab and Shift+Tab skip of an unshown control; table-driven rows
@task_file n=2 path=packages/blitz-vibey-script/src/seeded_random.rs
what: new crate-private module of the vibey-script engine, the seeded random source for page Math.random, with its inline unit tests pinning seed zero's reference outputs as literals
@task_file n=3 path=packages/blitz-vibey-script/tests/seeded_random.rs
what: new vibey-script integration test target: one sequence per seed, two seeds two sequences, an unseeded document draws from the host, each in both layout modes
@task_file n=4 path=packages/blitz-dom/src/node/node.rs
what: the blitz-dom node: gains the readings of whether a node sits under display none and whether it shows
@task_file n=5 path=packages/blitz-dom/src/layout_reason.rs
what: the blitz-dom layout reason reader: its has-a-box reading now goes through the under-display-none reading, answers unchanged
@task_file n=6 path=packages/blitz-dom/src/document.rs
what: the blitz-dom document: the post-layout focus fixup that gives up focus on a control that no longer shows, and the Tab walk (focus next and previous) now skipping unshown nodes
@task_file n=7 path=packages/blitz-dom/src/resolve.rs
what: the blitz-dom resolve pass: runs the post-layout focus fixup right after it reconciles the select list
@task_file n=8 path=packages/blitz-vibey-script/src/lib.rs
what: the vibey-script crate root: declares the seeded random module
@task_file n=9 path=packages/blitz-vibey-script/src/state.rs
what: the vibey-script runtime state: gains the random state of a seeded document
@task_file n=10 path=packages/blitz-vibey-script/src/runtime.rs
what: the vibey-script runtime: installs a seeded native Math.random in the realm
@task_file n=11 path=packages/blitz-vibey-script/src/document.rs
what: the script document's public API: gains the with_seed builder, a seed from building code alone
@task_file n=12 path=packages/dioxus-native-dom/src/boundary.rs
what: the Dioxus bridge's error-boundary registry and boundary marks the UI graph's caused owner reads, now pruned of dropped boundaries, with its inline bridge unit tests (bounded over twenty remounts, a new mark after remounts, a dropped boundary reads until the next render)
@task_file n=13 path=packages/dioxus-native-dom/src/dioxus_document.rs
what: the Dioxus document's poll, the buffered render on the Dioxus 0.7 bridge: prunes the boundary registry right before each render
@task_file n=14 path=tests/blitz-tests/tests/key_press_after_container_lost_its_box.rs
what: blitz-tests target for a key press after a dropped anonymous block or a lost box: focus is now given up after the layout and the key reaches the control only after a re-focus, select choice moving, in both modes
@task_file n=15 path=tests/blitz-tests/tests/dioxus_runtime_probe.rs
what: blitz-tests probe of the Dioxus runtime on the buffered-render bridge: a task awaiting the handle of a node its handler shows focuses it, and a task polled in the render that removes its target leaves focus off it, through a stored-waker wait
@task_file n=16 path=.github/scripts/ci-leg.sh
what: the CI leg script: the a11y leg's test list gains the new focus fixup target, nineteen files
@task_file n=17 path=.github/scripts/test_ci_workflows.py
what: the CI Python script unit tests over the workflows and legs: the a11y leg's file list and count
@task_file n=18 path=.github/scripts/test_capability_statement.py
what: the CI Python script unit test holding the capability statement to the schema: its literal a11y leg file count, 18 to 19
@task_file n=19 path=docs/capabilities.md
what: the capability statement: the a11y leg's count and reading, the focus limits (the keeps-focus limit retired, the focus-within Tab limit and the busy-yield livelock added, the same-render set-focus remnant re-stated), each claim naming its check
@task_item n=1 part=touches
decision/a11y-leg-membership: the a11y leg now holds nineteen files, the focus fixup target added after the key-press-after-lost-box target.
@task_item n=2 part=touches
decision/key-after-dropped-anonymous-block is stale: a focused control whose container loses its box now gives focus up at the next layout, and a key reaches it only after a re-focus; the test target was re-aimed, a third test included.
@task_item n=3 part=touches
decision/buffered-render-on-dioxus-07-bridge and decision/buffered-render-unborrowed-document: the same-render remnant is re-stated to the reachable case, and a task that wakes its own waker livelocks inside the buffered render, a limit no check asserts.
@task_item n=4 part=touches
decision/upstream-buffered-render-test-kept-as-written and decision/upstream-mounted-flush-test-left-out: the bridge probe gained mounted-handle focus tests on the buffered render; check the rows still read true beside them.
@task_item n=5 part=touches
The capability statement rows (decision/capability-statement-pinned, decision/capabilities-doc-pinned-to-schema, decision/capability-statement-from-schema, decision/capability-check-names-held-to-schema, decision/capability-claims-name-their-checks): the statement's a11y count and reading moved, one stated limit retired and two limits added, the busy-yield one asserted by no check.
@task_item n=6 part=touches
decision/count-and-rename-sweeps: a sweep over the leg script's names missed the CI-script test that pins the a11y file count as a literal; it moved 18 to 19 as a companion edit.
@task_item n=7 part=touches
decision/ci-scripts-unit-tested and decision/python-script-tests-under-unittest: both CI-script unit tests were edited for the leg's new count and list.
@task_item n=8 part=touches
decision/joining-file-keeps-its-legs and decision/requirement-witness-inside-stand-repeat: the new focus fixup target joins the a11y leg alone and no stand-repeat list; stand-repeat is unchanged and read 3 of 3 rounds green.
@task_item n=9 part=touches
decision/stand-checks-in-both-layout-modes and decision/layout-tests-in-both-layout-modes: every new focus, probe and seeded-random test runs in both layout modes.
@task_item n=10 part=touches
decision/one-test-target-per-behaviour, decision/blitz-tests-one-target-per-file and decision/one-test-target-per-file: two new test targets, one for the post-layout focus fixup and Tab skip, one for the seeded script random.
@task_item n=11 part=touches
decision/blitz-tests-single-test-target and decision/blitz-tests-manifest-difference: a new blitz-tests file was added; check the manifest rows still hold.
@task_item n=12 part=touches
decision/mutation-controlled-tests and decision/no-change-read-control-needs-pending-work: four new controls (fixup uncalled, visibility unread, seed unread, prune uncalled) and the Tab-skip pair each held red-before-green, six holds.
@task_item n=13 part=touches
decision/table-tests-count-their-rows: the new focus fixup tests are row tables of six, two and three rows.
@task_item n=14 part=touches
decision/expected-value-pinned-as-literal: seed zero's three reference outputs and the measured registry length 3 are pinned as literals.
@task_item n=15 part=touches
decision/inline-unit-test-modules, decision/inline-cfg-test-modules and decision/unit-tests-in-source-file: the seeded random module and the boundary registry carry their new unit tests inline.
@task_item n=16 part=touches
decision/bridge-tests-child-module and decision/bridge-tests-without-widening: the boundary prune is crate-private and tested in the bridge's own test module.
@task_item n=17 part=touches
decision/root-boundary-err-on-own-session and decision/caused-owner-pinned-as-graph-reads: boundary marks are now read from a registry pruned before each render; a boundary dropped in a step reads in that step's marks until the next render.
@task_item n=18 part=touches
decision/test-data-is-code and decision/test-data-in-code (nothing seeded): a script document may now be seeded, but only from building code, never from page input, env or a carrier.
@task_item n=19 part=touches
contract/harness-inspect/focused and contract/harness-core/pump: after the next pump's layout the harness's focused reading reads the root for a control that stopped showing, and Tab helpers no longer land on an unshown control.
@task_item n=20 part=touches
contract/session-run/changed-node-states: a focus given up after a layout now marks its node in the changed set, a new write site the acting call result can carry.
@task_item n=21 part=touches
decision/tally-move-attributed-by-test-name: the a11y reading moved 163 to 177 tests, attributed to the new and re-aimed tests by name.
@task_item n=22 part=touches
decision/ci-legs-through-leg-script and decision/stand-repeat-three-rounds: the a11y leg list in the leg script moved; fast, doc and stand-repeat legs stayed green and unchanged.
@task_fact n=1 slug=unit-test-strategy
fact: Once the post-layout focus fixup runs, an assertion that a hidden control is not focused holds with or without the change under test; a focus test pins the node focus lands on instead.
@task_fact n=2 slug=e2e-test-strategy
fact: A test task that waits for a later render on the Dioxus bridge leaves its waker for the producer to wake; a task that wakes its own waker while waiting livelocks inside the buffered render, a limit asserted by no check.
@task_fact n=3 slug=test-strategy
fact: A count of a CI leg's files pinned as a literal in a CI-script test moves with the leg; a sweep over the leg script's names alone misses it.
@task_fact n=4 slug=test-data-fixtures
fact: A script-document test that reads page Math.random seeds the document from its building code and pins the sequence that seed reads; an unseeded document reads the host's random, never asserted as a value.
@task_item n=1 part=untouched
The session wire, session host, session errors, socket and state-directory contracts: no IPC method, socket or wire form changed.
@task_item n=2 part=untouched
The MCP server, escher-session command line, exit codes and stderr contracts: no command, flag, exit code or log line changed.
@task_item n=3 part=untouched
The cold-agent and agent-run contracts, verbs, events and verdicts.
@task_item n=4 part=untouched
The WPT runner, its suites, statuses, outputs and pinned commit.
@task_item n=5 part=untouched
Coverage floors, mutation baselines, complexity ceilings and oversize-target rows: no floor, threshold or budget moved.
@task_item n=6 part=untouched
Driver refusal causes, driver verbs, lint defects, snapshot budget and recorded-check files: no verb, cause or recording changed.
@task_item n=7 part=untouched
The virtual clock, advance and timer-tick rows: no time handling changed.
@task_item n=8 part=untouched
CI permissions, pinned actions, caches, package installs, trigger branches and the apt-install and coverage-floor scripts' exit codes.
@task_item n=9 part=untouched
Pixel, screenshot, contrast and font rows: no paint or font path changed.
