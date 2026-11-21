@task_prose part=chunk
Researches four engine debts left by 0.1.0–0.3.0 and puts each as a founder's fork, every option costed, into one forks file in the run dir; technical forks go to the operator.
Then fixes each debt the founder rules fixed, proven headless on the stand or as a fixture in both layout modes, or keeps it as a residual with his reason, a wait on a dioxus-core release named as such.
Debt 1: a focused control hidden by the hidden attribute, inline display:none or CSS keeps focus and takes keys; the fork is a focus fixup for a control that no longer renders after layout.
Debt 2: a task polled during a buffered Dioxus render cannot reach nodes made in that render (set_focus does nothing); debt 3: a page script's Math.random is unseeded by the run's seeded source.
Debt 4: the error-boundary registry grows with every mount for a document's life; the fork is prune by a reachable signal, wait on dioxus-core, or keep.
@task_file n=1 path=packages/blitz-dom/src/mutator.rs mark=edits
what: the engine's document mutator and its flush — edits: DocumentMutator::flush's focus clear (the focus fixup)
@task_file n=2 path=packages/blitz-dom/src/document.rs mark=edits
what: the engine's document: focus state and inertness — edits: clear_focus, focus, is_inert
@task_file n=3 path=packages/dioxus-native-dom/src mark=edits
what: the Dioxus native document bridge: the buffered render and mutation buffer, MountedData's set_focus, the seeded source and the error-boundary registry — edits: the buffered render, set_focus / set_focus_to, SeededRandom and DEFAULT_SEED, the boundary registry and root_error_mark
@task_file n=4 path=packages/blitz-vibey-script mark=edits
what: the page-script runtime (Boa) and its set-up — edits: Boa's Math.random host hook, the runtime's set-up
@task_file n=5 path=tests/blitz-tests/tests/inert_subtree.rs mark=re-reads
what: the headless inert-subtree focus checks, beside which new focus-fixup checks stand
@task_item n=1 part=needs
The focus fixup in the engine's mutation flush: a focused control that stops rendering (hidden attribute, display:none inline or by CSS) gives up focus after the next layout, if the founder rules it built; a behaviour change for every document, remote pages included.
@task_item n=2 part=needs
A task polled during a buffered Dioxus render acting on that render's own new nodes (a deferred or replayed act), so set_focus on a node made in the same render takes effect, if the founder rules it built; the mutation buffer's contents still stay in memory only.
@task_item n=3 part=needs
Seeding the page script runtime's Math.random from a document-level seed, if the founder rules it: it crosses the script runtime's boundary and the seeded source's rule that it reads no environment variable, file or carrier.
@task_item n=4 part=needs
Pruning the error-boundary registry by a reachable signal (a weak handle or a generation), if the founder rules it; the registry still holds marks, not values.
@task_item n=5 part=needs
A fix crosses no new carrier, verb, argument, field, cause, log field or span unless the founder rules it: any new engine diagnostic or span added for these fixes needs his ruling first.
@task_item n=6 part=needs
Any log site the fixes add in the engine, the Dioxus bridge or the script runtime writes a fixed message, carries no page content (no focused control's text, no script values, no random values or seed derived from content) and stays behind the engine's tracing cargo feature.
@task_item n=7 part=needs
The page script's error and diagnostic surfaces (the embedder's drain of script messages and errors, the window error event, error source labels) stay as they are when the runtime's set-up changes for a seeded Math.random.
@task_item n=8 part=needs
The proof runs headless on the stand, which installs no telemetry sink; the new checks stay silent and assert behaviour, not log output.
@task_item n=9 part=needs
Debts the founder keeps are recorded as residuals with his reason; a wait on a dioxus-core release that exposes ErrorContext is named as such, not as a fix.
@task_standing n=1
convention: every new test or fixture is silent: it prints nothing on success, and a check that asserts empty stderr unsets RUST_LOG
home: 3-observability-harness-contract
@task_standing n=2
convention: every new engine log site writes a fixed message, prints safe fields only and carries no page content
home: 8-pii-scrubbing-compliance · pii-scrubbing-compliance
@task_standing n=3
convention: tracing in engine and upstream crates compiles only under the tracing cargo feature
home: 2-telemetry-strategy · telemetry-strategy
@task_item n=1 part=not_needed
The driver's command and tool_call spans and their fields, the refusal cause names and the escher-session roles, exit codes, host wire and coverage line: the driver debts are done and this chunk adds no verb, cause or field.
@task_item n=2 part=not_needed
The cold-agent pipe and agent-run harness: their events, verdict fields, counted verbs, transcripts, kept checks and cleanup.
@task_item n=3 part=not_needed
Session recording and state: the recording file, recording directory privacy, session lifecycle and idle expiry.
@task_item n=4 part=not_needed
CI artifacts and diagnostics: per-leg log and coverage uploads, the package-install diagnostic line, the build-directory sweep line; dependency and CI debts are another chunk.
@task_item n=5 part=not_needed
Select list and snapshot answers: the open list shot, optgroup labels, select chosen set and size attribute, multiple select form entries and the actions, layout and graph answers; the select debts are another chunk.
@task_item n=6 part=not_needed
WPT reports and WPT score publishing, and the absences of metrics, exporters, telemetry backends and error-reporting services.
@task_item n=7 part=not_needed
The telemetry sink's line format, resource keys, scrub set, RUST_LOG reading and panic hook in the native binaries: no sink is installed or changed by this chunk.
