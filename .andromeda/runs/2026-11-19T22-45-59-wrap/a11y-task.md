@task_prose part=session
The session merged upstream main 547d7514b into the branch with two parents: Stylo 0.24, parley 0.13, winit beta.4 and Dioxus held at 0.7.3. CI, the a11y leg and the stand-repeat leg passed.
It replaced the dispatch callback in every engine event path with upstream's event queue. Programmatic focus changes now fire blur, focusout, focus and focusin, and script-dispatched events read isTrusted false.
It moved where the accessibility tree is refreshed. The shell's redraw now pushes the tree after style resolve whenever the engine's changed set is non-empty, then clears that set. Poll no longer refreshes the tree.
It took the founder's widenings W1 to W9: an opt-in devtools CDP listener with no default feature turning it on; visibility collapse hidden from the accessibility tree; a cancelled pointerup that still clicks; a measured SVG mask attribute fetch.
It fixed an upstream inline-box placeholder defect in the engine. A private fallback font family is now registered in every font context and holds the bullet font.
It retired the old focus-event generator and the runtime change synthesis (which stays retired), and it removed the parley code editor crate.
@task_file n=1 path=packages/blitz-dom/src/accessibility.rs mark=merge
what: the engine's accessibility tree build, including the rule that keeps hidden content out of the tree
@task_file n=2 path=packages/blitz-dom/src/document.rs mark=merge
what: the base document, which holds focus setting and clearing, the changed set, the pending event queue and font registration
@task_file n=3 path=packages/blitz-dom/src/events/pointer.rs mark=merge
what: pointer down, up and click handling, which covers the click walk, double click, checkable toggles, range pointerdown and scrollbar drag
@task_file n=4 path=packages/blitz-dom/src/events/keyboard.rs mark=merge
what: keyboard handling, which covers Tab focus moves, Enter and Space activation, select list keys and range input keys
@task_file n=5 path=packages/blitz-dom/src/events/mod.rs mark=merge
what: the engine's event entry and dispatch, including the default-prevented handling
@task_file n=6 path=packages/blitz-dom/src/events/wheel.rs mark=merge
what: wheel gesture axis locking and wheel scrolling (upstream's, new)
@task_file n=7 path=packages/blitz-dom/src/events/focus.rs mark=merge
what: the old focus-event generator, deleted by upstream
@task_file n=8 path=packages/blitz-dom/src/mutator.rs mark=merge
what: the document mutator, whose flush runs the inert focus fix-up
@task_file n=9 path=packages/blitz-dom/src/node/select_list.rs mark=merge
what: the single select's engine overlay list state
@task_file n=10 path=packages/blitz-dom/src/node/node.rs mark=merge
what: the engine node, which holds focusability, scroll container and tree navigation reads
@task_file n=11 path=packages/blitz-dom/src/scrolling.rs mark=merge
what: engine scrolling and smooth scroll progress on the document's clock
@task_file n=12 path=packages/blitz-dom/src/layout/inline/build.rs mark=merge
what: inline layout construction, including the inline-box placeholder and its fallback font family
@task_file n=13 path=packages/blitz-dom/src/layout/inline/layout.rs mark=merge
what: inline line breaking and float placement
@task_file n=14 path=packages/blitz-dom/src/lib.rs mark=merge
what: the engine crate root and its exports, including the private placeholder font family name
@task_file n=15 path=packages/blitz-dom/src/stylo.rs mark=merge
what: the engine's Stylo bridge, which held selector matching, user-agent stylesheet and prefs (deleted, split by upstream into a module)
@task_file n=16 path=packages/blitz-dom/src/stylo/presentational_hints.rs mark=merge
what: the presentational hints from legacy and SVG attributes
@task_file n=17 path=packages/blitz-dom/assets/default.css mark=merge
what: the engine's user-agent stylesheet
@task_file n=18 path=packages/blitz-dom/src/config.rs mark=merge
what: the document configuration, which now carries the devtools provider
@task_file n=19 path=packages/blitz-dom/src/document_inline_box_placeholder_tests.rs
what: the witness check for the inline-box placeholder fallback (new)
@task_file n=20 path=packages/blitz-shell/src/window.rs mark=merge
what: the shell's view, whose poll and redraw drive resolve and the accessibility tree push
@task_file n=21 path=packages/blitz-test-harness/src/contrast.rs mark=merge
what: the harness contrast reader that computes the WCAG contrast ratio
@task_file n=22 path=packages/blitz-paint/src/render/select_list.rs mark=merge
what: the paint of the open select list
@task_file n=23 path=packages/blitz-paint/src/render/form_controls.rs mark=merge
what: the paint of form controls such as checkboxes, radios and range inputs
@task_file n=24 path=packages/blitz-paint/src/text.rs mark=merge
what: text paint, including glyph runs and synthetic emboldening
@task_file n=25 path=packages/blitz-paint/src/render.rs mark=merge
what: the paint traversal over the document
@task_file n=26 path=packages/escher-driver/src/execute.rs mark=merge
what: the session driver's command execution, including the type command's select focus
@task_file n=27 path=packages/dioxus-native-dom/src/dioxus_document.rs mark=merge
what: the Dioxus document, which flushes pending DOM events to the Dioxus handlers
@task_file n=28 path=packages/dioxus-native-dom/src/events.rs mark=merge
what: Dioxus event data conversion, which now includes the wheel phase and source
@task_file n=29 path=packages/blitz-vibey-script/src/runtime/events.rs mark=merge
what: the script runtime's event dispatch, the source of the trusted flag on script events
@task_file n=30 path=packages/blitz-vibey-script/tests/dom.rs mark=merge
what: the script DOM checks
@task_file n=31 path=examples/seven_guis/src/app.rs mark=merge
what: the seven_guis stand app, kept as the branch had it
@task_file n=32 path=examples/seven_guis/src/tasks/cells.rs mark=merge
what: the seven_guis Cells grid, kept as the branch had it
@task_file n=33 path=tests/blitz-tests/tests/all.rs mark=merge
what: the blitz-tests target manifest
@task_file n=34 path=tests/blitz-tests/tests/svg_mask_attribute_fetch.rs
what: the measuring check for the SVG mask attribute fetch (new)
@task_file n=35 path=Cargo.toml mark=merge
what: the workspace manifest, which holds the members and the engine dependency pins
@task_file n=36 path=Cargo.lock mark=merge
what: the workspace lock
@task_file n=37 path=deny.toml mark=merge
what: the dependency audit configuration, with its licence table and advisory ignores
@task_file n=38 path=.github/workflows/ci.yml mark=merge
what: the CI workflow and its legs
@task_file n=39 path=packages/dioxus-native/Cargo.toml mark=merge
what: the Dioxus native crate's features, including its default set
@task_file n=40 path=apps/browser/Cargo.toml mark=merge
what: the browser app's features, including its default set
@task_item n=1 part=touches
Accessibility tree refreshed on change: the refresh moved from the shell's poll to its redraw, after style resolve, whenever the changed set is non-empty. The redraw then clears the changed set.
@task_item n=2 part=touches
What the engine's changed set marks and Harness settle leaves the changed set: a new clear-changes call empties the set, and the shell's redraw now clears it after each tree push. The take of changed nodes is unchanged.
@task_item n=3 part=touches
Accessibility tree through the document trait and Accessibility tree built through the document's own method: the shell now builds the tree from the wrapping document after resolve.
@task_item n=4 part=touches
Tab and Shift+Tab move focus with focus events and Removing the focused node drops focus with blur steps: setting and clearing focus now queue blur, focusout, focus and focusin through the engine's event queue (W5). Upstream's focus-event generator is gone.
@task_item n=5 part=touches
Focus fixup reads inertness alone and Inert subtree excluded: the inert fix-up now clears focus through a silent clear that queues no blur or focusout. The rule stands, through a new path.
@task_item n=6 part=touches
Driver type focuses a select with no click: the driver now focuses the select through a new silent focus call that queues no focus event.
@task_item n=7 part=touches
Keyboard activation (Enter, Space, handler order), Select key change fires one input then one change, and the single-choice select input and change events: the select choice, key-up and generated input paths now queue user-agent events instead of calling a dispatch callback. Event order is in the rows' reach.
@task_item n=8 part=touches
Range input keys, Range input pointerdown sets value, Range input click gives focus not value, and the range input contract items (input event, pointerdown, click): the range move path now queues its events through the event queue.
@task_item n=9 part=touches
Checkbox or radio click toggles, fires input, focuses; One engine change per checkable click; A click focuses a button; A disabled form control stops the click walk: pointerdown, pointerup and click now follow upstream's split with default-prevented handling. A pointerup cancelled by a handler still clicks (W7).
@task_item n=10 part=touches
Driver click in a shown scrollbar band drags: a cancelled pointerup still ends a scrollbar drag (upstream's, W7).
@task_item n=11 part=touches
Double click threshold and Double click read on the document's clock: a cancelled pointerdown now still counts towards a double click without selecting text (upstream's). Smooth scroll progress reads the document's clock.
@task_item n=12 part=touches
Hidden content kept out of the tree, Hidden content excluded from the accessibility tree, and the accessibility tree hidden content contract (visibility hidden, display none, hidden, aria-hidden): an element with visibility collapse is now kept out of the tree too (W9).
@task_item n=13 part=touches
AccessKit requests reach one event handler and Synthetic click is a primary click at the centre: the engine paths these requests and clicks enter now queue events instead of dispatching them inline.
@task_item n=14 part=touches
accesskit_xplat as the platform adapter: the adapter crate gains an iOS platform implementation (upstream's).
@task_item n=15 part=touches
Accessibility as a default-on feature, Browser accessibility feature off by default, and Dioxus accessibility feature taken only by name: both default feature sets changed. The browser app's default is now skia, cookies, cache, screenshot, apple-font-embolden, scrollbars and mimalloc, with no devtools. Dioxus native's default no longer has clipboard.
@task_item n=16 part=touches
WCAG contrast ratio matching Chrome and Lint contrast shares the harness reader: the harness contrast reader was edited in the merge, which reached its imports and its colour reads.
@task_item n=17 part=touches
Open select list colours are engine constants and Open select list rows take one height: the open select list's paint and its overlay list state were reached by the merge.
@task_item n=18 part=touches
Password caret placed by bullets and Password value masked: the bullet font is now also registered under a private placeholder family in every font context, including a context the document was given.
@task_item n=19 part=touches
:focus-visible never matches, :focus does, and :open matches an open details or dialog: selector matching moved into upstream's split Stylo module on Stylo 0.24 (selectors 0.43).
@task_item n=20 part=touches
Scheme change counts as a device change and Hidpi or zoom change reshapes text: device change detection moved into upstream's Stylo device module.
@task_item n=21 part=touches
The a11y CI leg, Accessibility test leg in the slow tier, Accessibility checks gate CI, and Stand checks repeated three rounds: the a11y leg and stand-repeat (3 of 3 rounds) were green on the merge. The CI workflow gains the Android NDK steps.
@task_item n=22 part=touches
Dioxus render into a mutation buffer and the Dioxus document reached-handler record: the Dioxus document now flushes the engine's pending DOM events. Dioxus is held at 0.7.3.
@task_item n=23 part=touches
Text runs label their parent element: text paint and inline layout were rebuilt on parley 0.13. An inline-box placeholder now shapes with a fallback family when no font can shape it.
@task_fact n=1 slug=keyboard-navigation
fact: A programmatic focus change fires blur, focusout, focus and focusin through the engine's event queue. The inert fix-up and the driver's select focus are the exceptions: they change focus silently and fire none.
@task_fact n=2 slug=keyboard-navigation
fact: Events dispatched by script read isTrusted false. Events the user agent dispatches read true.
@task_fact n=3 slug=keyboard-navigation
fact: A pointerup cancelled by a handler still produces the click and still ends a scrollbar drag.
@task_fact n=4 slug=keyboard-navigation
fact: A pointerdown cancelled by a handler still counts towards a double click but selects no text.
@task_fact n=5 slug=aria-patterns-roles
fact: An element with visibility collapse is kept out of the accessibility tree, as visibility hidden is.
@task_fact n=6 slug=a11y-strategy
fact: The shell pushes the accessibility tree in its redraw, after style resolve, only when the changed set is non-empty. It then clears the changed set, and poll pushes no tree.
@task_item n=1 part=untouched
The snapshot, snapshot text form, snapshot diff, UI graph, served node id, MCP result and actions answer contracts: their shapes and readers did not change.
@task_item n=2 part=untouched
Invalid state not by colour alone, invalid date state, and the cognitive accessibility rows: no form validation or invalid styling changed.
@task_item n=3 part=untouched
The UI graph state registration contract (use_graph_state, undeclared, unconfirmed, no registered state): no state registration changed.
@task_item n=4 part=untouched
The SLO and budgets, anti-patterns, and decisions-log sections: they hold no recorded intent, and the session adds none to them.
