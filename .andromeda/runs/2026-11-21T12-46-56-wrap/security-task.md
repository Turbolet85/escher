@task_prose part=session
The engine now gives focus up, after each layout, from a focused control that no longer shows (display none on it or an ancestor, no computed style, or visibility not visible); the clear is silent, no blur or focusout is fired, and the changed set marks the node.
Tab and Shift+Tab now skip a node that does not show, an engine change for every document on the founder's word; one stated limit remains: a control whose own focus-within rule hides its container still takes Tab and loses focus at the next layout.
A script document can now be seeded from its building code (with_seed, a u64): the page's Math.random is replaced by a native seeded function; an unseeded document reads the host's random; no carrier, env read, log site or panic path on page input is added.
The Dioxus bridge's error-boundary registry now records each boundary's scope and is pruned of dropped boundaries before each render, so it stays bounded (measured length 3 over 20 remounts) instead of growing for the document's life.
The stated limit that a focused control which no longer shows keeps focus and takes keys is retired; a key on a boxless control now acts only after a re-focus; the a11y CI leg runs nineteen files (reading 19 · 177 · 0 · 0).
No IPC method, endpoint, event, socket, port, env var, dependency or workspace crate is added; CI on the pre-CI commit reads green.
@task_file n=1 path=tests/blitz-tests/tests/focus_fixup_unshown.rs
what: new integration test of the post-layout focus fixup and of the Tab walk skipping a control that does not show
@task_file n=2 path=packages/blitz-vibey-script/src/seeded_random.rs
what: new crate-private seeded random source (splitmix64) behind the script engine's Math.random
@task_file n=3 path=packages/blitz-vibey-script/tests/seeded_random.rs
what: new integration test of the seeded Math.random: one sequence per seed, an unseeded document reads the host's
@task_file n=4 path=packages/blitz-dom/src/node/node.rs
what: the DOM node; gains the under-display-none and shows readings a focus check reads
@task_file n=5 path=packages/blitz-dom/src/layout_reason.rs
what: the layout-reason reader; its box check now reads the shared under-display-none reading, answers unchanged
@task_file n=6 path=packages/blitz-dom/src/document.rs
what: the engine document: focus handling, the Tab walk (focus next and previous) and the new silent post-layout focus clear
@task_file n=7 path=packages/blitz-dom/src/resolve.rs
what: the document resolve pass; now runs the post-layout focus fixup right after the select list reconcile
@task_file n=8 path=packages/blitz-vibey-script/src/lib.rs
what: the script engine crate root; declares the seeded random module
@task_file n=9 path=packages/blitz-vibey-script/src/state.rs
what: the script runtime state; holds the optional seeded random state
@task_file n=10 path=packages/blitz-vibey-script/src/runtime.rs
what: the script runtime; installs the native seeded Math.random into the page realm
@task_file n=11 path=packages/blitz-vibey-script/src/document.rs
what: the script document API (ScriptDocument); gains the with_seed builder
@task_file n=12 path=packages/dioxus-native-dom/src/boundary.rs
what: the Dioxus bridge's error-boundary registry and boundary marks; now records scopes and prunes dropped boundaries
@task_file n=13 path=packages/dioxus-native-dom/src/dioxus_document.rs
what: the Dioxus document's poll and buffered render; now prunes the boundary registry before each render
@task_file n=14 path=tests/blitz-tests/tests/key_press_after_container_lost_its_box.rs
what: integration test of a key on a control whose container lost its box (dropped anonymous block); now re-focuses before the key
@task_file n=15 path=tests/blitz-tests/tests/dioxus_runtime_probe.rs
what: integration test of the Dioxus runtime: a task awaiting a mounted handle, focus during and after a render
@task_file n=16 path=.github/scripts/ci-leg.sh
what: the CI leg script; the a11y leg's list of test files gains one
@task_file n=17 path=.github/scripts/test_ci_workflows.py
what: the CI workflow check; pins the a11y leg's file list and count
@task_file n=18 path=.github/scripts/test_capability_statement.py
what: the capability statement check; pins the a11y leg's file count as a literal (18 to 19)
@task_file n=19 path=docs/capabilities.md
what: the committed capability statement: focus limits, the a11y leg's count and reading, the buffered-render limit
@task_item n=1 part=touches
decision/inert-focus-clear-silent: a second silent focus clear now exists, after each layout, for a focused control that does not show; it fires no blur or focusout like the inert clear.
@task_item n=2 part=touches
decision/programmatic-focus-events: its except list (focus changes that fire no focus events) must gain the post-layout clear of an unshown control.
@task_item n=3 part=touches
decision/inert-subtree: the focus fixup no longer reads inert alone; it also clears focus from a control that does not show after a layout.
@task_item n=4 part=touches
decision/dropped-anonymous-block-clears-parent: stale where it says a key on the boxless control still acts; focus is now given up after the layout and a key acts only after a re-focus.
@task_item n=5 part=touches
decision/seeded-source-in-process-seed-only: a script document's Math.random seed now comes from in-process building code alone through with_seed; the row gains this seeded source.
@task_item n=6 part=touches
decision/enter-activates-focused-control and decision/space-keyup-activates-focused-control: a control that no longer shows is no longer focused after a layout, so a key no longer reaches or activates it.
@task_item n=7 part=touches
decision/button-click-focuses and decision/accessibility-focus-built-nodes-only: the focus a click or the accessibility adapter gives is now taken off again after a layout when the node does not show.
@task_item n=8 part=touches
decision/accessibility-tree-refresh-on-change and decision/snapshot-state-readings: the changed set now marks a node whose focus was given up after a layout, so the tree and the focus reading move with no event.
@task_item n=9 part=touches
decision/select-list-closes-when-select-unfit and decision/select-readers-resolve-node-closed-select-none: a focused select that stops showing now loses focus at the next layout, after the select list reconcile.
@task_item n=10 part=touches
decision/bootstrap-native-helper-removed and decision/search-params-native-parser-hidden: a new native function (the seeded Math.random, writable, configurable, not enumerable) is placed in the page realm when seeded; rules on native functions exposed to pages reach it.
@task_item n=11 part=touches
decision/root-boundary-err-crossing: the boundary marks it reads come from a registry now pruned before each render; a boundary dropped in a step reads in that step's marks until the next render.
@task_item n=12 part=touches
decision/dioxus-render-into-buffer and decision/try-doc-none-while-borrowed: the Dioxus poll now prunes the boundary registry right before the buffered render; a task that wakes itself during a render livelocks inside it (a stated limit, no check).
@task_item n=13 part=touches
decision/skip-mounted-event-of-removed-element: a task polled in the render that removes its target leaves focus off it (measured: focus on the root, target unresolved).
@task_item n=14 part=touches
decision/no-panic-on-call-input and decision/untrusted-bytes-decoded-without-panic: the new focus check and seeded random add no panic path on page input; rows that hold that guarantee cover the new code.
@task_item n=15 part=touches
decision/engine-target-safe-fields and decision/accessible-names-not-logged: no log site is added by the fixup, the Tab skip, the seeded random or the registry prune.
@task_item n=16 part=touches
decision/capability-statement-mcp-crossing: the committed capability statement was edited (focus limits retired and re-stated, a11y leg count and reading); its MCP crossing text is unchanged.
@task_fact n=1 slug=input-validation
fact: Tab and Shift+Tab skip a node that does not show (display none on it or an ancestor, no computed style, or visibility not visible); the focus walk is focussable and shows.
@task_fact n=2 slug=input-validation
fact: Stated limit: a control whose own focus-within rule hides its container still takes Tab and loses focus at the next layout; no check asserts otherwise.
@task_fact n=3 slug=input-validation
fact: After each layout a focused control that does not show gives focus up silently; display contents and a hidden sibling keep focus.
@task_fact n=4 slug=error-handling
fact: The Dioxus bridge's error-boundary registry is pruned of dropped boundaries before each render, so it stays bounded instead of growing for the document's life.
@task_fact n=5 slug=input-validation
fact: A seeded script document replaces the page's Math.random with a native seeded function; an unseeded document reads the host's random; each value lies in [0, 1).
@task_item n=1 part=untouched
Driver session socket, MCP server and their reply, request, refusal and exit-code contracts: no verb, method, field, cause or bound moved.
@task_item n=2 part=untouched
Authentication and authorization rows: nothing about access or tokens changed.
@task_item n=3 part=untouched
Data protection rows (history store, HTTP cache, kept check files, coverage line): no stored data changed.
@task_item n=4 part=untouched
Dependency security rows: no dependency, lockfile, pin, audit ignore or licence moved.
@task_item n=5 part=untouched
Secret management rows and environment variable contracts: no env read or secret added.
@task_item n=6 part=untouched
Outbound network rows (fetch methods, schemes, per-host limits, CORS and same-origin absences, request and response bounds): no request path changed.
@task_item n=7 part=untouched
Value masking, password painting and stable element id rows: masks, ids and their carriers are unchanged.
@task_item n=8 part=untouched
Recording, replay and cold-agent rows: no recorder, check file or agent run changed.
@task_item n=9 part=untouched
Logging and span contract rows (command span, tool_call span, network tracing log): no log line or field changed.
