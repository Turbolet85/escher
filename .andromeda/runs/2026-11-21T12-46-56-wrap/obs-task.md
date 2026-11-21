@task_prose part=session
The engine now gives focus up, silently, after each layout when the focused control no longer shows (display none on it or an ancestor, or visibility not visible); no blur or focusout is queued and the changed set marks the node.
Tab and Shift+Tab now skip a node that does not show, an engine change for every document on the founder's word; a control whose own focus-within rule hides its container is a stated limit.
A script document can be seeded: a consuming builder gives the page's Math.random a fixed in-process seed (splitmix64), an unseeded document still draws from the host; no env read, carrier or log site is added.
The dioxus bridge's error-boundary registry now records each boundary's scope and is pruned of dropped boundaries before each render, so it stays bounded (measured length 3 over 20 remounts).
The a11y CI leg runs nineteen files (was eighteen), reading 19 · 177 · 0 · 0; the capability statement retires the limit that a focused control which no longer shows keeps focus, and states the busy-yield livelock of a self-woken task inside a render as a limit no check asserts.
CI on the pushed pre-CI commit read green, 17 of 17 checks; nothing changed after implement.
@task_file n=1 path=tests/blitz-tests/tests/focus_fixup_unshown.rs
what: added; the integration test of the post-layout focus fixup and the Tab walk's skip of controls that do not show, a member of the a11y CI leg
@task_file n=2 path=packages/blitz-vibey-script/src/seeded_random.rs
what: added; the script engine's crate-private seeded random source behind a seeded page Math.random
@task_file n=3 path=packages/blitz-vibey-script/tests/seeded_random.rs
what: added; the integration test that one seed reads one Math.random sequence and an unseeded document draws from the host
@task_file n=4 path=packages/blitz-dom/src/node/node.rs
what: edited; the engine's DOM node, gaining the readings of whether a node sits under display none and whether it shows
@task_file n=5 path=packages/blitz-dom/src/layout_reason.rs
what: edited; the engine's layout-reason reading of whether a node has a box, answers unchanged
@task_file n=6 path=packages/blitz-dom/src/document.rs
what: edited; the engine document's focus handling, the Tab and Shift+Tab walk and the new silent post-layout focus give-up
@task_file n=7 path=packages/blitz-dom/src/resolve.rs
what: edited; the engine's resolve pass (style and layout), now running the focus give-up after the select list reconcile
@task_file n=8 path=packages/blitz-vibey-script/src/lib.rs
what: edited; the script engine crate root, declaring the seeded random module
@task_file n=9 path=packages/blitz-vibey-script/src/state.rs
what: edited; the script runtime's state, gaining the seeded random state
@task_file n=10 path=packages/blitz-vibey-script/src/runtime.rs
what: edited; the page script runtime, installing a seeded native Math.random in the realm
@task_file n=11 path=packages/blitz-vibey-script/src/document.rs
what: edited; the script document embedders build, gaining the with_seed builder
@task_file n=12 path=packages/dioxus-native-dom/src/boundary.rs
what: edited; the dioxus bridge's error-boundary registry and the boundary marks the driver reads, now pruned of dropped boundaries
@task_file n=13 path=packages/dioxus-native-dom/src/dioxus_document.rs
what: edited; the dioxus document's poll and buffered render, pruning the boundary registry before each render
@task_file n=14 path=tests/blitz-tests/tests/key_press_after_container_lost_its_box.rs
what: edited; the integration test of a key on a focused control whose container lost its box, now reached through a re-focus
@task_file n=15 path=tests/blitz-tests/tests/dioxus_runtime_probe.rs
what: edited; the dioxus runtime probe test, gaining a stored-waker task that focuses a shown node and a task polled in the render that removes its target
@task_file n=16 path=.github/scripts/ci-leg.sh
what: edited; the CI leg script holding each leg's test list, the a11y leg's list gaining one file
@task_file n=17 path=.github/scripts/test_ci_workflows.py
what: edited; the CI workflow check pinning the legs' test lists and the a11y leg's file count
@task_file n=18 path=.github/scripts/test_capability_statement.py
what: edited; the capability statement check, pinning the a11y leg's file count as a literal (18 to 19)
@task_file n=19 path=docs/capabilities.md
what: edited by hand; the capability statement, its focus limits, the a11y leg's count and reading, and the buffered-render limits
@task_item n=1 part=touches
decision/checks-silent: the post-layout focus give-up, the Tab skip, the seeded Math.random and the boundary prune each add no log site, by this rule; the session leans on it and must not break it.
@task_item n=2 part=touches
decision/capability-statement-updated-by-hand: the capability statement was edited by hand for the a11y leg's count (nineteen files), its reading 19 · 177 · 0 · 0, the retired focus limit and the new focus-within and busy-yield limits.
@task_item n=3 part=touches
decision/engine-sites-fixed-messages and decision/engine-target-safe-fields: the engine gains focus code (fixup, Tab skip) with no log site, so no new engine message or field.
@task_item n=4 part=touches
decision/layout-read-opens-no-span: the layout pass now also gives focus up after it runs, opening no span and writing no event.
@task_item n=5 part=touches
decision/mutation-buffer-memory-only: the buffered dioxus render is re-stated to its reachable case plus the busy-yield livelock limit; the sweep reads this row true, the buffer still in memory only.
@task_item n=6 part=touches
decision/runtime-probe-leaks-no-read-value: the runtime probe test gains tasks reading focus after a shown or removed node; they assert focus readings and print no read value.
@task_item n=7 part=touches
contract/script-diagnostics-drain/take-js-errors, take-messages and error-retention: the page script runtime now installs a native seeded Math.random; it adds no diagnostic, message or panic path on page input, so the drain is unchanged.
@task_item n=8 part=touches
contract/script-error-surface/error-handler-exception and the script error source labels: the seeded Math.random is a native function with no error path of its own; the surface is unchanged.
@task_item n=9 part=touches
contract/page-events rows: a control given up after a layout queues no blur or focusout, a second silent focus path beside the inert one, so page script sees no focus event for it.
@task_item n=10 part=touches
contract/snapshot-diff and contract/snapshot-node rows: the snapshot's focus now leaves a control that no longer shows after the next layout, and the diff's changed set marks that node.
@task_item n=11 part=touches
decision/state-registry-holds-no-values and decision/no-state-page-skips-registry: the dioxus bridge's error-boundary registry (a different registry) now holds each boundary's scope and error context and is pruned before each render; the state registry is unchanged.
@task_item n=12 part=touches
decision/driver-emits-no-event: the boundary marks keep their signature and their two driver callers; the driver gains no event and no subscriber.
@task_item n=13 part=touches
decision/select-chosen-set-never-logged and decision/select-list-label-never-logged: a boxless focused select now gives focus up after a layout and acts on a key only re-focused; no log site reads its choice.
@task_item n=14 part=touches
decision/ci-platform-github-workflows and decision/ci-log-artifact-on-failure: the a11y leg runs one more file on the same workflows; the per-leg log artifact is unchanged and no row holds the leg's count.
@task_item n=15 part=touches
decision/headless-stand-no-sink: the engine change is proven headless by stand-repeat, whose list is unchanged and which still runs with no sink.
@task_item n=16 part=touches
decision/telemetry-via-tracing and decision/engine-tracing-feature-gated: no tracing call, span or feature is added by the engine, script or bridge changes.
@task_fact n=1 slug=pii-scrubbing-compliance
fact: A script document's Math.random seed comes only from the embedder's building code (with_seed), never from page input, an env var or a carrier, and the seed and its draws are never logged.
@task_fact n=2 slug=pii-scrubbing-compliance
fact: The dioxus bridge's error-boundary registry holds each boundary's scope and error context in memory only, pruned of dropped boundaries before each render, and is never logged.
@task_item n=1 part=untouched
The telemetry backend, OTel export and metric absences: nothing exported, emitted or counted is added.
@task_item n=2 part=untouched
The driver's command and tool_call span contracts, their fields and the refusal cause names: no span or field changed.
@task_item n=3 part=untouched
The telemetry sink, its log line format, scrub set, target allowlist, RUST_LOG handling and resource keys: no sink code changed.
@task_item n=4 part=untouched
The cold-agent and agent-run harness rows, their events, verdict fields and kept checks: no harness changed.
@task_item n=5 part=untouched
escher-session's commands, exit codes, host wire, coverage role, recording files and session lifecycle: no session code changed.
@task_item n=6 part=untouched
WPT score publishing, the build-directory sweep line and the package-install diagnostic line: untouched.
@task_item n=7 part=untouched
The panic hooks of escher-telemetry and WASM builds and the window error event contract: unchanged.
