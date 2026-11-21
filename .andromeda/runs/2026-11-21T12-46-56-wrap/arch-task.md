@task_prose part=session
The session paid four engine debts. First, after each layout a focused control that no longer shows (not in the document, under display none, or not visibility visible) gives up focus silently, with no blur or focusout, and the changed set marks it. This fixup runs in resolve right after the select list is reconciled.
Tab and Shift+Tab now skip a node that does not show. The filter went from focusable alone to focusable and shown. This is an engine change for every document, ruled by the founder.
A script document can now be seeded through a consuming with_seed builder, which replaces the page realm's Math.random with a native splitmix64 sequence. An unseeded document still reads the host's random source.
The Dioxus boundary registry records each boundary's scope and is pruned of dropped boundaries in poll before each render. It stays bounded, and a boundary dropped in a step reads in that step's marks until the next render.
The a11y CI leg runs nineteen files instead of eighteen, read as 19 · 177 · 0 · 0. The stated limit that a focused control that no longer shows keeps focus is retired. Two limits are added to the statement: a busy-yield livelock inside render_immediate, and a control hidden by its own focus-within rule.
Measured: a task that wakes its own waker while it waits livelocks inside the Dioxus render. A task waiting for a later render leaves its waker to the producer.
@task_file n=1 path=packages/blitz-dom/src/node/node.rs
what: the DOM node, its readers of whether it lies under display none and whether it shows
@task_file n=2 path=packages/blitz-dom/src/layout_reason.rs
what: the layout reason reader and its has-box test
@task_file n=3 path=packages/blitz-dom/src/document.rs
what: BaseDocument, its focus handling, the Tab and Shift+Tab focus walk and the post-layout focus fixup
@task_file n=4 path=packages/blitz-dom/src/resolve.rs
what: the document's resolve pass, style then layout, then the select list reconcile and now the focus fixup
@task_file n=5 path=packages/blitz-vibey-script/src/lib.rs
what: the blitz-vibey-script crate root and its module list
@task_file n=6 path=packages/blitz-vibey-script/src/state.rs
what: the script runtime's state, now holding the seeded random state
@task_file n=7 path=packages/blitz-vibey-script/src/runtime.rs
what: the Boa script runtime that installs the page realm's globals, now a seeded Math.random
@task_file n=8 path=packages/blitz-vibey-script/src/document.rs
what: ScriptDocument, the script-running document and its consuming builders, now with_seed
@task_file n=9 path=packages/blitz-vibey-script/src/seeded_random.rs
what: the script crate's own seeded random source, a splitmix64 step and its unit interval
@task_file n=10 path=packages/blitz-vibey-script/tests/seeded_random.rs
what: the script crate's integration check of seeded and unseeded Math.random
@task_file n=11 path=packages/dioxus-native-dom/src/boundary.rs
what: the Dioxus error boundary registry behind boundary_marks and BoundaryMark
@task_file n=12 path=packages/dioxus-native-dom/src/dioxus_document.rs
what: the Dioxus document, its poll and buffered render
@task_file n=13 path=tests/blitz-tests/tests/focus_fixup_unshown.rs
what: an integration check of the post-layout focus fixup and the Tab skip, in the a11y leg
@task_file n=14 path=tests/blitz-tests/tests/key_press_after_container_lost_its_box.rs
what: an integration check of a key on a control whose container lost its box, in the a11y leg
@task_file n=15 path=tests/blitz-tests/tests/dioxus_runtime_probe.rs
what: the integration probe of Dioxus runtime behaviour on the bridge, its tasks, mounted handles and focus
@task_file n=16 path=.github/scripts/ci-leg.sh
what: the one CI leg runner and its legs' test lists, the a11y leg's list among them
@task_file n=17 path=.github/scripts/test_ci_workflows.py
what: the tests that pin the CI workflow contract and the legs' file counts
@task_file n=18 path=.github/scripts/test_capability_statement.py
what: the test that holds the capability statement to its schema and pins the a11y leg's file count
@task_file n=19 path=docs/capabilities.md
what: the capability statement, its stated limits and measured readings
@task_item n=1 part=touches
decision/focus-fixup-without-events and decision/inert-focus-fixup: there is now a second silent focus fixup. After each layout it gives up focus from a control that does not show, and the inert path is no longer the only one.
@task_item n=2 part=touches
contract/dom-events/inert-focus-fixup, contract/dom-events/focus-change and contract/event-dispatch/focus-change: focus lost at the post-layout fixup queues no blur or focusout, which is a new exception to the focus-change events.
@task_item n=3 part=touches
decision/changed-set-write-sites: the changed set is now written at layout time too, because the focus cleared after a layout marks its node. Any row saying the changed set is never written by layout is stale. decision/changed-set-engine-touched and contract/document-core/changed-set may hold the same list.
@task_item n=4 part=touches
decision/hit-focus-form-engine-changes, decision/a11y-input-focus-engine-changes and decision/direct-engine-changes-flagged-upstreamable: the tally of direct engine changes gains the post-layout fixup and the Tab walk's skip of unshown nodes.
@task_item n=5 part=touches
decision/resolve-reconciles-select-list: resolve now runs the focus fixup right after it reconciles the select list.
@task_item n=6 part=touches
Rows on keyboard focus movement, such as contract/key-names/tab and the Tab focus order: Tab and Shift+Tab now skip a focusable node that does not show.
@task_item n=7 part=touches
contract/snapshot/focused, contract/node-keys/focused and contract/accessibility-tree/focus: after a layout, a hidden control that was focused reads unfocused. The engine reads focus on the root and the tree reads it on the window.
@task_item n=8 part=touches
decision/bridge-buffered-render, decision/dioxus-bridge-buffered-render and decision/render-with-document-unborrowed: the limit on a node created in the same render, where set_focus does nothing, is re-stated to the reachable case. The busy-yield livelock is added as a limit.
@task_item n=9 part=touches
decision/no-seed-on-carriers and decision/one-run-seed: a script document is now seeded from its building code alone, and no seed crosses a carrier.
@task_item n=10 part=touches
decision/no-prng-crate: seeded page randomness is the crate's own splitmix64, so no PRNG crate is added.
@task_item n=11 part=touches
contract/blitz-vibey-script-api/scriptdocument: it gains with_seed, a public consuming builder that takes a u64.
@task_item n=12 part=touches
Script global and bootstrap rows, such as contract/script/bootstrap and contract/script-globals: on a seeded document, Math.random is replaced by a native function that is writable, configurable and not enumerable.
@task_item n=13 part=touches
contract/dioxus-native-dom-api/boundary-marks: the registry no longer grows for the document's life. It is pruned before each render, and a boundary dropped in a step still reads in that step's marks until the next render. The signature and callers are unchanged.
@task_item n=14 part=touches
contract/dioxus-document/poll and contract/document-trait/poll: the Dioxus document's poll now prunes dropped boundaries right before its render.
@task_item n=15 part=touches
contract/ci-leg-legs/a11y and decision/a11y-leg-widens-without-new-job: the a11y leg's file count goes from eighteen to nineteen, and its reading goes from 18 · 163 to 19 · 177 · 0 · 0.
@task_item n=16 part=touches
decision/capability-statement, decision/capability-statement-matches-schema and decision/ci-contract-pinned: the statement's focus limits changed, and the a11y leg's file count is pinned as a literal in the CI-script tests.
@task_item n=17 part=touches
decision/consuming-builder-methods and decision/builder-method-naming: with_seed is a new consuming builder that falls under these conventions.
@task_fact n=1 slug=established-decisions
fact: Tab and Shift+Tab skip a focusable node that does not show, meaning it is not in the document, is under display none, or does not compute visibility visible. This is a direct engine change for every document, ruled by the founder.
@task_fact n=2 slug=established-decisions
fact: After each layout, a focused node that does not show gives up focus silently, with no blur or focusout, and the changed set marks it. A display contents control and a control beside a hidden sibling keep focus.
@task_fact n=3 slug=established-decisions
fact: Stated limit: a control whose own focus-within rule hides its container still takes Tab, then loses focus at the next layout.
@task_fact n=4 slug=established-decisions
fact: Stated limit, asserted by no check: a Dioxus task that wakes its own waker while it waits livelocks inside render_immediate. A task that waits for a later render must leave its waker to the producer.
@task_fact n=5 slug=cross-cutting-patterns
fact: A script document's Math.random is seeded only from in-process building code through with_seed, with one sequence per seed. An unseeded script document reads the host's random source.
@task_item n=1 part=untouched
Driver verbs, the session socket wire, the MCP role, kept checks and refusal causes: no verb, carrier, IPC method, socket or port is added.
@task_item n=2 part=untouched
Telemetry, logging and scrubbing rows: no log site, span, env read or panic path is added.
@task_item n=3 part=untouched
decision/removed-node-interaction-state and decision/removed-focused-node-drops-focus: these are unchanged. A removed focused node still leaves focus on the root.
@task_item n=4 part=untouched
Select list rows on its model, rows, painting, wheel and default actions: the select list machinery itself did not change.
@task_item n=5 part=untouched
Stack and dependency rows and the occupied names: no crate, binary, env var or dependency is added, and Dioxus and Boa versions stay where they were.
@task_item n=6 part=untouched
decision/stand-repeat-zero-failure-budget and the stand-repeat list: the new test joins no stand-repeat list.
@task_item n=7 part=untouched
Paint, hit testing, scrolling and WPT runner rows: none of their behaviour moved.
@task_item n=8 part=untouched
The browser app, its history and persistence rows: nothing there changed.
