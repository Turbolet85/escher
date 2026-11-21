@task_prose part=chunk
Researches four engine debts left by 0.1.0–0.3.0 and puts each to the founder as a fix-or-keep fork in one forks file; each debt is then fixed and proven headless on the stand (or as a fixture in both layout modes), or kept as a residual by his word.
Debt 1: a focused control hidden by the hidden attribute, display:none or CSS alone keeps focus and takes keys, because the document mutator's focus fixup reads inertness alone; the fork is a focus fixup for a control that no longer renders.
Debt 2: a task polled during a Dioxus render sees the document as the previous render left it, so setting focus on a node made in that render does nothing; the fork is a deferred or replayed act.
Debt 3: a page script's Math.random is the host's, unseeded; the fork is seeding Boa's Math.random from a document-level seed. Debt 4: the error-boundary registry grows for a document's life; the fork is pruning by a reachable signal, waiting on a dioxus-core release, or keeping it.
@task_file n=1 path=packages/blitz-dom/src/mutator.rs
what: the document mutator, whose flush clears focus from a control that turns inert (the inert focus clear)
@task_file n=2 path=packages/blitz-dom/src/document.rs
what: the engine document: focus, clearing focus, and the inert-subtree reading of a node
@task_file n=3 path=tests/blitz-tests/tests/inert_subtree.rs
what: the inert subtree checks, beside which the checks of a focus fixup for a hidden control stand
@task_file n=4 path=packages/dioxus-native-dom/src/
what: the Dioxus native bridge: the buffered render, mounted-element operations (set focus), the seeded random source and its default seed, the error-boundary registry and the root boundary error mark
@task_file n=5 path=packages/blitz-vibey-script/
what: the browser JavaScript runtime (Boa): its set-up and the Math.random host hook a page script reaches
@task_item n=1 part=needs
A focus fixup for a control that is focused and then stops rendering (hidden attribute, display:none, CSS): whether focus is given up, when (after the next layout), and how it sits beside the inert focus clear and the inert subtree rule.
@task_item n=2 part=needs
Indirect, through the focus clear the fixup extends: the existing inert focus clear fires no blur, and programmatic focus changes fire focus events; a new clear must take one of those rules deliberately and keep the inert clear as it is.
@task_item n=3 part=needs
Indirect, through the accessibility adapter: accessibility focus names only a built node and the tree is refreshed on change, so a control that loses its box must not stay the adapter's focus.
@task_item n=4 part=needs
A task polled during a Dioxus render made to see that render's own nodes (a deferred or replayed act), under the rule that Dioxus renders into a buffer before borrowing the document and that try_doc answers none while the document is borrowed.
@task_item n=5 part=needs
Mounted-element operations on a missing node: setting focus on a node not yet in the document answers false and acts on nothing; a deferred act must keep the missing-node and removed-element answers and never panic on a stale node id.
@task_item n=6 part=needs
Seeding a page script's Math.random from a document-level seed: the seeded source takes its seed from in-process code only and reads no environment variable, file or carrier, so the seed reaching the script runtime must come the same way.
@task_item n=7 part=needs
Indirect, through the script runtime's crossing: JavaScript stays opt-in in the browser, the default script fetcher takes local, file and data URLs only, and script errors are recorded, not propagated; a seeding hook changes none of these.
@task_item n=8 part=needs
The error-boundary registry and the root boundary error mark: pruning a dropped boundary by a reachable signal (a weak handle, a generation) must keep the root-boundary Err crossing as it is; a wait on a dioxus-core release is named as such.
@task_item n=9 part=needs
A fix crosses no new carrier, verb, argument, field, cause, log field or span unless the founder rules it: the boundary widening rule, the closed driver verb schema, the closed refusal cause set and the engine target field allowlist hold.
@task_item n=10 part=needs
A fix to debts 1 to 3 is an engine change for every document, remote pages included, and is its own ratification; untrusted page markup and scripts reach the new focus fixup, deferred act and seeded Math.random, so each stays finite and panic-free on hostile input.
@task_item n=11 part=needs
Each fixed debt is proven headless on the stand or as a fixture in both layout modes; each kept debt stays a named residual with the founder's reason.
@task_standing n=1
convention: every new log line goes through escher's allowlist telemetry sink, carries only allowlisted safe fields and no screen content, accessible name or stable id
home: logging-monitoring
@task_standing n=2
convention: every new failure path in engine code never panics on a stale node id or untrusted input and answers by a closed set
home: error-handling
@task_standing n=3
convention: the test harness takes no tracing and no style, and new engine tests stay in the unpublished test crate
home: dependency-security
@task_item n=1 part=not_needed
Authentication and authorization, CI tokens and publish guards: the chunk touches no credential, job or release path.
@task_item n=2 part=not_needed
Data protection of stored data: history in sqlite, cookies, the HTTP cache, clipboard, file dialog and TLS; no debt stores or reads them.
@task_item n=3 part=not_needed
Outbound HTTP, per-host limits, sub-resource fetching, CSS and font fetches, the CDP devtools server and iframe navigation: no debt changes what the engine fetches or serves.
@task_item n=4 part=not_needed
Recording and kept check files, the cold-agent harness, coverage floors and the agent-run and apt-install scripts: tooling no debt reaches.
@task_item n=5 part=not_needed
The single and multiple select controls and the open select list (keys, rows, painting, choice exits): the select list sized as a list box is out of scope.
@task_item n=6 part=not_needed
Password and file input masking and painted bullets, range input actions and values: no debt changes how a control's value is read or painted.
@task_item n=7 part=not_needed
WPT runner, result posting and quarantine workflows, and CI secrets and signing material: CI and dependency debts are out of scope.
@task_item n=8 part=not_needed
HTML and CSS parsing and serialization rules (invalid declarations and selectors, presentational hints, inline SVG, raw HTML passthrough in rdme): no debt changes parsing.
