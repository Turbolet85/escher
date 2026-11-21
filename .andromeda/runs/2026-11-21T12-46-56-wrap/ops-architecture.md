@row_add n=1 row=decision/tab-walk-skips-unshown area=established-decisions fact=F1 detector=D-arch-decisions
name: Tab walk skips what does not show
rule: Tab and Shift+Tab skip a focusable node that does not show, meaning one not in the document, under display none, or not computing visibility visible; the skip is a direct engine change for every document.
why: ratified as the remedy of the Tab trap the post-layout focus fixup alone made
guard: focus_fixup_unshown::tab_and_shift_tab_skip_a_control_that_does_not_show
basis: fct-k5yrahcq, fct-amaxcqzi, fct-xvmt5uxa
because: The chunk made the Tab walk read whether a node shows, an engine decision ruled for every document that no row states.
@row_add n=2 row=decision/focus-within-hidden-control-takes-tab area=established-decisions fact=F3 detector=none
name: Control hidden by its own focus-within rule takes Tab
rule: A control whose own :focus-within rule hides its container still takes Tab, then loses focus at the next layout; the limit is stated, owned by no residual.
basis: fct-mryduozh, fct-k5yrahcq
because: The chunk states this remainder of the Tab skip and the post-layout fixup as a standing limit, which no row holds.
@row_set n=1 row=decision/focus-fixup-without-events cell=rule seen=1 fact=F2 detector=none
value: At flush, focus on a node that has become inert, a subtree moved under an inert box included, and after each layout, focus on a node that no longer shows, is cleared with no blur or focusout fired; a display contents control and a control beside a hidden sibling keep focus.
basis: fct-xvmt5uxa, fct-4mnmyqn6
because: The chunk added a second silent focus clear after each layout for a node that does not show, so the row naming the inert path alone is incomplete.
@row_set n=2 row=contract/dom-events/focus-change cell=promise seen=1 detector=none
value: A focus change dispatches blur, focusout, focus, focusin, except the focus fixups under inert and after a layout for a node that no longer shows, which clear focus with none.
basis: fct-xvmt5uxa, fct-mfew6peh
because: The post-layout fixup is a focus change that dispatches no event, so the promise that every focus change dispatches them is false.
@row_set n=3 row=contract/event-dispatch/focus-change cell=promise seen=2 detector=none
value: A focus change, a programmatic one included, dispatches blur, focusout, focus, focusin; only the focus fix-up under inert, the focus fix-up after a layout for a node that no longer shows, and the driver's select focus dispatch none.
basis: fct-xvmt5uxa, fct-mfew6peh
because: The post-layout fixup is a new silent focus path, so the closed list of focus changes that dispatch none must name it.
@row_set n=4 row=decision/changed-set-write-sites cell=except seen=3 detector=none
value: a page scroll or a move that closes an open select list marks its select at the next resolve; a focus given up after a layout marks its node
basis: fct-xvmt5uxa, fct-mfew6peh
because: The changed set now marks a node whose focus is given up after a layout, which the row's never by layout contradicts.
@row_set n=5 row=decision/hit-focus-form-engine-changes cell=rule seen=1 detector=D-arch-decisions
value: Visible region through transforms, no hit on a non-invertible transform, focus fixups under inert and after layout, Tab skipping what does not show, tree focus falling back to the window, native toggled, multiple select, optgroup and checkbox and radio change are engine changes for all documents.
basis: fct-xvmt5uxa, fct-mfew6peh
because: The chunk adds the post-layout focus fixup and the Tab walk's skip as direct engine changes for every document, so the tally must name them.
@row_set n=6 row=decision/resolve-reconciles-select-list cell=rule seen=1 detector=none
value: Resolve reconciles the select list after layout, then gives up the focus of a node that no longer shows, both before refreshing hover, and removing a select clears its list.
basis: fct-xvmt5uxa, fct-mfew6peh
because: Resolve now runs the focus fixup right after the select list reconcile, a step of resolve's order the row must state.
@row_set n=7 row=decision/bridge-buffered-render cell=except seen=1 fact=F4 detector=none
value: a Dioxus task that wakes its own waker while it waits livelocks inside the render, asserted by no check, so a task that waits for a later render leaves its waker to the producer
basis: fct-a4d7c4g3, fct-kabdmiwn, fct-mnilfhvh
because: The chunk measured the busy-yield livelock and states it as a limit of the buffered render, which the row does not hold.
@row_set n=8 row=decision/one-run-seed cell=except seen=1 fact=F5 detector=none
value: a script document's Math.random is seeded only from in-process building code through with_seed, one sequence per seed, and an unseeded script document reads the host's random source
basis: fct-xvmt5uxa, fct-4mnmyqn6, fct-mfew6peh
because: The chunk gave the script document its own seeded Math.random apart from the Dioxus root context, so the row's one-seed rule must name it.
@row_set n=9 row=decision/no-seed-on-carriers cell=rule seen=3 detector=none
value: No run seed crosses the driver's carriers: no verb, argument, command-line flag, MCP tool field, kept-check key or span field carries one, so each driver session and kept check's replay reads the default seed; a seed reaches a run only from in-process code building its VirtualDom or script document.
basis: fct-xvmt5uxa, fct-mfew6peh
because: A script document's seed now comes from its building code alone, a second in-process source the row must name.
@row_set n=10 row=contract/blitz-vibey-script-api/scriptdocument cell=promise seen=1 detector=D-arch-resources
value: Re-exported, one of the five items blitz-vibey-script re-exports; its consuming builder with_seed(u64) seeds the document's Math.random, one sequence per seed.
basis: fct-xvmt5uxa, fct-mfew6peh
because: The chunk added the public with_seed builder to ScriptDocument, a new item of its public API the contract must register.
@row_set n=11 row=contract/dioxus-native-dom-api/boundary-marks cell=promise seen=1 detector=D-arch-resources
value: Public and re-exported: the document provides a boundary registry at the root scope before its first build, every error boundary built under it registers, and boundary_marks reads them by identity; dropped boundaries are pruned before each render, so one dropped in a step reads till the next render.
basis: fct-xvmt5uxa, fct-hhfijknc, fct-4mnmyqn6
because: The registry no longer grows for the document's life, since the chunk prunes dropped boundaries before each render.
@row_set n=12 row=decision/anonymous-block-drop-clears-layout-parent cell=rule seen=1 detector=none
value: Dropping an anonymous block clears the layout parent of each child still naming it, so a key press on a control whose anonymous block was dropped does not panic, and once re-focused the key acts.
basis: fct-4mnmyqn6, fct-mfew6peh
because: A focused control whose container lost its box now gives focus up after the layout, so the row's claim that focus stays is false.
@row_set n=13 row=decision/integration-tests-in-blitz-tests cell=except seen=1 detector=D-arch-decisions
value: blitz-vibey-script keeps its own integration tests in its tests directory
basis: fct-zhup3qcq, fct-4iwuitrb
because: The chunk added an integration test in blitz-vibey-script's own tests directory beside the crate's standing ones, which the row's rule does not allow.
