@task_prose part=chunk
Four engine debts left by 0.1.0-0.3.0, each researched with every option's cost and put to the founder as a fork in one file, then either fixed and proven headless on the stand (or as a fixture in both layout modes) or kept as a residual by his word.
Item 1: a focus fixup for a focused control that no longer renders (hidden attribute, display:none, CSS alone), which today keeps focus and takes keys because the fixup reads inertness alone — or keep the limit.
Item 2: a task polled during a Dioxus buffered render acting on the render's own new nodes (set_focus on a node created in that render), by a deferred or replayed act — or keep the limit.
Item 3: seeding a page script's Math.random from a document-level seed — or keep the limit; item 4: pruning the growing error-boundary registry by a reachable signal, waiting on a dioxus-core release, or keeping it.
A fix to items 1-3 is an engine change for every document, remote pages included, and crosses no new carrier, verb, argument, field, cause, log field or span unless the founder rules it.
@task_file n=1 path=packages/blitz-dom/src/mutator.rs mark=edits
what: the engine's document mutator, whose flush runs the focus fixup that reads inertness alone — edits: DocumentMutator::flush's focus clear
@task_file n=2 path=packages/blitz-dom/src/document.rs mark=edits
what: the engine document holding focus, its clearing and the inertness read — edits: clear_focus, focus, is_inert
@task_item n=1 part=needs
The focus fixup that reads inertness alone: item 1 may widen it so a focused control that stops rendering (hidden attribute, display: none, CSS alone) gives up focus, possibly only after the next layout — a changed behaviour if the founder chooses the fix.
@task_item n=2 part=needs
Removing or losing the focused node drops focus with blur steps: a focus given up by the new fixup must run the same blur and focus-event steps, and the keys it no longer takes must not reach the hidden control.
@task_item n=3 part=needs
What is focusable and how Tab and Shift+Tab walk focus: a control no longer rendered must not be left focused nor be a stop the fix reintroduces; focusability recomputed on attribute writes stays the trigger set unless the fork rules otherwise.
@task_item n=4 part=needs
Hidden content kept out of the accessibility tree (hidden, display: none, visibility: hidden, aria-hidden) and the inert subtree excluded: the focus fixup's notion of a no-longer-rendered control must agree with what the tree already drops, so the tree's focus never points at a node the tree omits.
@task_item n=5 part=needs
The accessibility tree's focus value and its refresh on change: a focus change made by the fixup or by a deferred set_focus must mark the changed set so the tree and snapshot report the new focus.
@task_item n=6 part=needs
The Dioxus render into a mutation buffer: item 2 may let a task polled during a render act on nodes that render created (a deferred or replayed act), changing what MountedData set_focus does on such a node, whose result set_focus_to now reports as false.
@task_item n=7 part=needs
Hidden and inert nodes in hit-testing and the accessibility tree: an inert element takes no hit; the fix must not change hit or tree readings of hidden or inert nodes beyond the focus it gives up.
@task_item n=8 part=needs
The page script runtime's randomness (item 3): seeding Boa's Math.random from a document-level seed reaches the script runtime's crossing and the seeded source's rule that it reads no environment variable, file or carrier — a fix must stay inside both.
@task_item n=9 part=needs
The error-boundary registry and its root error mark (item 4): pruning dropped boundaries by a weak handle or generation, or naming the wait on a dioxus-core release, must leave boundary marks reported as they are for live boundaries.
@task_item n=10 part=needs
No new carrier, verb, argument, field, cause, log field or span: the driver schema, snapshot fields, actions answer and log lines stay as they are unless the founder rules otherwise.
@task_item n=11 part=needs
Proof of each fix headless on the stand or as a fixture in both layout modes: a new focus check beside the inert-subtree checks, and the spawned-acts probe pins for item 2 re-read against the changed render.
@task_standing n=1
convention: each new accessibility or focus check is a Rust test asserting over the AccessKit tree or the engine document, headless
home: 3-a11y-assertion-harness-contract
@task_standing n=2
convention: each new accessibility test joins the accessibility test leg in the slow tier, which gates CI
home: 9-ci-integration · bootstrap-phases-derive-for-route-setup-project
@task_standing n=3
convention: each new stand check runs repeated three rounds
home: ci-integration
@task_item n=1 part=not_needed
Single-choice and multiple select rows — closed and open list keys, typeahead, optgroups, list colours, row heights, list motion, list rows in the snapshot: the single select sized as a list box is out of this chunk's scope.
@task_item n=2 part=not_needed
Contrast and visual lint rows — WCAG contrast ratio, thresholds, stand accent colour, lint contrast finding fields, scrollbar thumb stroke, author thumb state: the chunk changes no paint or colour.
@task_item n=3 part=not_needed
Range input, seven_guis Cells grid and CRUD list key contracts and the double-click threshold: no stand control's key handling changes.
@task_item n=4 part=not_needed
Role mapping, accessible name precedence, ARIA property and state reads (aria-checked, aria-pressed, aria-labelledby and the rest): the chunk reads no new role, name or ARIA state.
@task_item n=5 part=not_needed
Accessibility scope rows — tier 0 reach, AccessKit and accesskit_xplat choice, the feature defaults, no platform adapter on the stand: no fix changes the accessibility feature or adapter set.
@task_item n=6 part=not_needed
Cognitive accessibility rows (invalid state not by colour or class alone, shared tab order table for the stand): no invalid-state or stand form presentation changes.
@task_item n=7 part=not_needed
Password masking, password caret, screenshot, driver click refusals in a scrollbar band and the UI graph state registration rows: the driver debts are done and out of scope.
