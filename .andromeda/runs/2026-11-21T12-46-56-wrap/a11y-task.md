@task_prose part=session
The engine's focus fixup now also runs after each layout: a focused control that no longer shows (out of the document, under display: none, or not visibility: visible) gives focus up silently, with no blur or focusout, and the changed set marks it; display: contents and a hidden sibling keep focus.
Tab and Shift+Tab now skip a focusable node that does not show, an engine change for every document on the founder's word, made because the fixup alone turned a hidden focusable into a Tab trap; one stated limit remains: a control whose own :focus-within rule hides its container still takes Tab and loses it at the next layout.
A boxless focused control now gives focus up after the layout, and a key reaches it only after a re-focus; the mounted-during-render set_focus residual is re-stated to the reachable case, and a task that wakes its own waker during a render livelocks it (stated as a limit).
A script document can be seeded (with_seed) so page-script Math.random reads one sequence per seed, and the Dioxus error-boundary registry is pruned of dropped boundaries before each render, staying bounded.
The a11y CI leg runs nineteen files, up from eighteen, reading 19 · 177 · 0 · 0; the stand-repeat list is unchanged; CI on the pre-CI commit read green (17/17).
@task_file n=1 path=tests/blitz-tests/tests/focus_fixup_unshown.rs
what: new a11y leg test - a focused control that stops showing gives focus up with no blur, display: contents and a hidden sibling keep focus, the :focus-within case, and Tab and Shift+Tab skipping a control that does not show
@task_file n=2 path=packages/blitz-vibey-script/src/seeded_random.rs
what: new crate-private seeded random source of the page-script runtime (splitmix64), behind Math.random
@task_file n=3 path=packages/blitz-vibey-script/tests/seeded_random.rs
what: new tests of the script document's seeded Math.random - one sequence per seed, an unseeded document draws from the host
@task_file n=4 path=packages/blitz-dom/src/node/node.rs
what: edited - the engine's DOM node, gains the reading of whether a node shows and whether it sits under display: none
@task_file n=5 path=packages/blitz-dom/src/layout_reason.rs
what: edited - the engine's layout-reason reading of whether a node has a box, now through the display: none reading (answers unchanged)
@task_file n=6 path=packages/blitz-dom/src/document.rs
what: edited - the engine document, its focus fixup (post-layout give-up of an unshown focused control) and its Tab and Shift+Tab focus walk
@task_file n=7 path=packages/blitz-dom/src/resolve.rs
what: edited - the engine's resolve pass, which now runs the post-layout focus fixup right after reconciling the select list
@task_file n=8 path=packages/blitz-vibey-script/src/lib.rs
what: edited - the page-script runtime crate root, registers the seeded random module
@task_file n=9 path=packages/blitz-vibey-script/src/state.rs
what: edited - the page-script runtime state, holds the seeded random state
@task_file n=10 path=packages/blitz-vibey-script/src/runtime.rs
what: edited - the page-script runtime, installs a seeded Math.random in the realm
@task_file n=11 path=packages/blitz-vibey-script/src/document.rs
what: edited - the script document's public builder, gains with_seed
@task_file n=12 path=packages/dioxus-native-dom/src/boundary.rs
what: edited - the Dioxus document's error-boundary registry and its boundary marks, now pruned of dropped boundaries
@task_file n=13 path=packages/dioxus-native-dom/src/dioxus_document.rs
what: edited - the Dioxus document (buffered Dioxus render), prunes the boundary registry before each render
@task_file n=14 path=tests/blitz-tests/tests/key_press_after_container_lost_its_box.rs
what: edited - a11y leg test of a key on a control whose container lost its box, now re-focused before the key
@task_file n=15 path=tests/blitz-tests/tests/dioxus_runtime_probe.rs
what: edited - the Dioxus runtime probe tests, a task awaiting a shown node's handle focuses it and a task polled in the removing render leaves focus off
@task_file n=16 path=.github/scripts/ci-leg.sh
what: edited - the CI leg script, the a11y leg's test list gains one file
@task_file n=17 path=.github/scripts/test_ci_workflows.py
what: edited - the CI workflow check, pins the a11y leg's file count (nineteen)
@task_file n=18 path=.github/scripts/test_capability_statement.py
what: edited - the capability statement check, pins the a11y leg's file count (nineteen)
@task_file n=19 path=docs/capabilities.md
what: edited - the capability statement, its focus limits, the a11y leg's count and reading, and the mounted-during-render residual
@task_item n=1 part=touches
Focus fixup reads inertness alone: now false; the fixup also runs after each layout and gives up focus on a control that no longer shows, silently.
@task_item n=2 part=touches
What the engine's changed set marks: a focus given up after a layout now marks its node, so layout is no longer outside the changed set's write sites.
@task_item n=3 part=touches
Tab and Shift+Tab move focus: the walk's filter goes from focusable to focusable and shown; a hidden focusable is skipped both ways.
@task_item n=4 part=touches
Stated Tab order walked and the stand's shared Tab order table: the order now omits any focusable that does not show (indirect, through the Tab walk change).
@task_item n=5 part=touches
What is focusable: focusability itself is unchanged, but Tab reachability now also needs the node to show; a row equating focusable with Tab-reachable is stale.
@task_item n=6 part=touches
The a11y CI leg and the accessibility test leg (and the gate over them): the leg runs nineteen files, reading 19 · 177 · 0 · 0, where a row holds the count or reading.
@task_item n=7 part=touches
Removing the focused node drops focus with blur steps: unchanged, but a second, silent focus loss now sits beside it (after layout, no blur or focusout).
@task_item n=8 part=touches
Inert subtree excluded: the fixup that cleared focus on inertness now also clears it on not showing.
@task_item n=9 part=touches
The accessibility tree's focus contract: after a layout give-up the engine reads focus on the root and the tree reads the window.
@task_item n=10 part=touches
Dioxus render into a mutation buffer: the mounted-during-render set_focus residual is re-stated to the reachable case, and a task that wakes its own waker during a render livelocks it, a limit no check asserts.
@task_item n=11 part=touches
Focus on a select whose container lost its box (driver type focuses a select, focus stays on a select with its list open): a boxless select now gives focus up after the layout and takes a key only once re-focused (indirect).
@task_item n=12 part=touches
Hidden content kept out of the tree and the hidden-content contracts (display: none, visibility: hidden, hidden): the same hiding now also takes focus away and removes the node from the Tab walk (indirect, beside them).
@task_fact n=1 slug=keyboard-navigation
fact: Tab and Shift+Tab skip a focusable node that does not show (out of the document, under display: none, or not visibility: visible), for every document, on the founder's word of 2026-11-21.
@task_fact n=2 slug=keyboard-navigation
fact: After each layout a focused control that no longer shows gives focus up with no blur or focusout; a display: contents control and one beside a hidden sibling keep focus.
@task_fact n=3 slug=keyboard-navigation
fact: Limit: a control whose own :focus-within rule hides its container still takes Tab and loses focus at the next layout.
@task_item n=1 part=untouched
ARIA roles, states, property reads and accessible-name rows: no role, state or name reading moved.
@task_item n=2 part=untouched
Contrast, colour, scrollbar, zoom and visual lint rows: no paint or contrast reading moved.
@task_item n=3 part=untouched
Snapshot, snapshot text, snapshot diff, actions answer, MCP result and driver refusal contracts: no field or answer changed.
@task_item n=4 part=untouched
The key contracts of the range input, select lists (open and closed), CRUD list and Cells grid: their keys act as before on a shown control.
@task_item n=5 part=untouched
Platform adapter and accessibility feature scope rows (AccessKit, accesskit_xplat, tier 0, default-on feature): nothing added or removed.
@task_item n=6 part=untouched
The stand-repeat three-round check: its list is unchanged.
@task_item n=7 part=untouched
Cognitive accessibility rows (invalid state not by colour or class alone): untouched.
