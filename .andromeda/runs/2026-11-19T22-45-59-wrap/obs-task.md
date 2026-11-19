@task_prose part=session
The session merged upstream/main at 547d7514b into the 0.4.0 build branch as a two-parent merge, CI green 17/17 on the first attempt, with each upstream commit left out named with its reason.
The merge brought an opt-in DevTools protocol server crate (devtools feature, off in every default feature set): a TcpListener on 127.0.0.1 at BLITZ_CDP_PORT, no Origin check and no authentication, and an env read BLITZ_DEVTOOLS_FRONTEND_DIR; its log lines take an engine target, redacted by escher's sink.
The event-queue model replaced the dispatch callback in every escher event path: programmatic focus now queues blur, focusout, focus and focusin, while the inert fix-up and the driver's select focus stay silent; script-dispatched events read isTrusted false; a cancelled pointerup still clicks; the runtime change synthesis stays retired.
Engine pins moved (Stylo 0.24, selectors 0.43, parley 0.13, skrifa 0.49, winit beta.4, svgtypes 0.16.2; Dioxus held at 0.7.3), blitz-net moved to rustls, four precise lock updates cleared eight audit advisories, Skia became a default renderer of the browser and the WPT runner, and the parley_code_editor crate and AGENTS.md were retired.
The accessibility tree is now pushed after resolve when the changed set is non-empty, an inline-box placeholder got an engine font fallback, and a mask attribute URL was measured to fetch only through the stamped request constructor.
@task_file n=1 path=Cargo.toml
what: edited; the workspace manifest: its members (the DevTools protocol server added, the code-editor crate removed) and the engine dependency pins
@task_file n=2 path=Cargo.lock
what: edited; the workspace lockfile, with the four ruled precise updates past upstream's lock
@task_file n=3 path=deny.toml
what: edited; the dependency audit's licence table and advisory ignores
@task_file n=4 path=.github/workflows/ci.yml
what: edited; the fork CI workflow and its jobs, the Android job gaining SHA-pinned NDK steps
@task_file n=5 path=.github/workflows/wpt.yml mark=merge
what: merged; the WPT workflow that runs the suite and publishes its scores
@task_file n=6 path=.github/scripts/wpt_diff_to_pr.py mark=merge
what: merged; the WPT diff-to-PR script, now skipping results on a revision mismatch
@task_file n=7 path=apps/browser/Cargo.toml
what: edited; the browser app's default feature set (skia added, devtools left off)
@task_file n=8 path=packages/blitz-vibey-cdp-server/src/transport.rs mark=merge
what: merged; the DevTools protocol server's TCP listener and its connection loop
@task_file n=9 path=packages/blitz-vibey-cdp-server/src/frontend.rs mark=merge
what: merged; the DevTools frontend file server and its frontend-directory env read
@task_file n=10 path=packages/blitz-vibey-cdp-server/src/lib.rs mark=merge
what: merged; the DevTools protocol server: its targets, sessions and message processing
@task_file n=11 path=packages/blitz-shell/src/devtools.rs mark=merge
what: merged; the shell's DevTools state, starting the protocol server from the port env read
@task_file n=12 path=packages/blitz-shell/src/window.rs
what: edited; the shell's window view: redraw pushes the accessibility tree after resolve and clears the changed set
@task_file n=13 path=packages/blitz-dom/src/document.rs
what: edited; the engine's base document: focus with and without focus events, the changed set, the event queue, font registration
@task_file n=14 path=packages/blitz-dom/src/mutator.rs
what: edited; the document mutator: the inert focus fix-up and mutation records handed to a devtools provider
@task_file n=15 path=packages/blitz-dom/src/events/pointer.rs
what: edited; the engine's pointer event handling: pointerdown, pointerup, click and select choice, queued through the event queue
@task_file n=16 path=packages/blitz-dom/src/events/mod.rs
what: edited; the engine's event driver entry and its default-prevented handling
@task_file n=17 path=packages/blitz-dom/src/events/keyboard.rs
what: edited; the engine's keyboard event handling, queued through the event queue
@task_file n=18 path=packages/blitz-dom/src/events/focus.rs mark=merge
what: deleted; the engine's old focus event generator
@task_file n=19 path=packages/blitz-dom/src/stylo.rs
what: deleted; the engine's single Stylo glue module, replaced by upstream's split stylo modules
@task_file n=20 path=packages/blitz-dom/src/stylo/prefs.rs mark=merge
what: merged; the Stylo prefs set once per process, at Stylo 0.24's pref names
@task_file n=21 path=packages/blitz-dom/src/stylo/presentational_hints.rs
what: edited; the presentational hints for legacy HTML and SVG presentation attributes
@task_file n=22 path=packages/blitz-dom/src/layout/inline/build.rs
what: edited; the inline layout builder, with the inline-box placeholder's font fallback
@task_file n=23 path=packages/blitz-dom/src/lib.rs
what: edited; the engine crate root, naming the private placeholder font family
@task_file n=24 path=packages/blitz-dom/src/document_inline_box_placeholder_tests.rs
what: added; the engine unit check of an inline block placed once its own font loads
@task_file n=25 path=packages/blitz-dom/src/accessibility.rs
what: edited; the accessibility tree builder, hiding visibility collapse
@task_file n=26 path=packages/blitz-dom/src/net.rs
what: edited; the engine's resource fetch path and its stamped request
@task_file n=27 path=packages/blitz-net/src/lib.rs
what: edited; the network provider and its TLS stack, now rustls
@task_file n=28 path=packages/blitz-paint/src/debug_overlay.rs
what: edited; the paint debug overlay
@task_file n=29 path=packages/escher-driver/src/execute.rs
what: edited; the driver's command execution, its select focus now set without a queued focus event
@task_file n=30 path=packages/blitz-vibey-script/src/runtime/events.rs
what: edited; the script runtime's event dispatch to page listeners
@task_file n=31 path=packages/dioxus-native-dom/src/events.rs
what: edited; the Dioxus native event conversion, wheel phase and source added
@task_file n=32 path=packages/dioxus-native/Cargo.toml
what: edited; the Dioxus native default feature set, clipboard no longer default
@task_file n=33 path=wpt/runner/src/main.rs
what: edited; the WPT runner: its test manifest load and renderer choice
@task_file n=34 path=wpt/runner/src/test_runners/ref_test.rs
what: edited; the WPT reftest runner and its report messages
@task_file n=35 path=tests/blitz-tests/tests/all.rs
what: edited; the engine integration test manifest
@task_file n=36 path=tests/blitz-tests/tests/svg_mask_attribute_fetch.rs
what: added; the check that a mask attribute URL is fetched stamped with its document
@task_file n=37 path=packages/parley_code_editor/Cargo.toml
what: deleted; the parley code editor crate, retired from the workspace
@task_file n=38 path=AGENTS.md
what: deleted; upstream's agent instructions file, kept out
@task_item n=1 part=touches
The rule that engine and upstream crates compile tracing only behind the tracing cargo feature, and the telemetry-strategy overview stating it: a new upstream workspace crate, the DevTools protocol server, joins those crates and logs.
@task_item n=2 part=touches
The sink target allowlist and the capture that counts framework targets at warn and above: the DevTools protocol server brings a new log target; the capture checks read green inside fast and stand-repeat.
@task_item n=3 part=touches
The rules that engine log sites write fixed messages, that engine targets print safe fields only, and the reach of the scrubbing sink: the new crate's log lines take an engine target, their messages redacted by escher's sink.
@task_item n=4 part=touches
Rows stating a surface reads no env var, or naming RUST_LOG as the sink's configuration read: two new env reads arrive, BLITZ_CDP_PORT and BLITZ_DEVTOOLS_FRONTEND_DIR, under the devtools feature only.
@task_item n=5 part=touches
Mutation buffer contents stay in memory: the mutator now hands mutation records (node inserted, removed, attribute and character data changed) to a devtools provider when one is configured under the devtools feature.
@task_item n=6 part=touches
The metric-coverage overview naming the in-app overlay, blitz-paint's counters and the WPT runner's figures: the paint debug overlay was edited and the WPT runner gained a renderer choice (cpu, tiny-skia, skia) and a manifest loader.
@task_item n=7 part=touches
WPT scores and results published from upstream only: the WPT workflow and its diff-to-PR script took upstream's edits (results skipped on a revision mismatch).
@task_item n=8 part=touches
The WPT report message and subtest message contracts: the reftest and crash-test runners were edited by the merge.
@task_item n=9 part=touches
CI on GitHub workflows and the fork CI artifacts (the per-leg log on failure, the coverage report on success, two artifacts): the CI workflow was edited, the Android job gaining SHA-pinned NDK steps; the run read 17 of 17 checks.
@task_item n=10 part=touches
The page script event contracts for change on a checkbox, change on a radio and the change count: every escher event path now queues through the user-agent event queue in place of the dispatch callback; the runtime change synthesis stays retired.
@task_item n=11 part=touches
Script error source labels for event listeners and event microtasks: the script runtime's event dispatch was edited, script-dispatched events now reading isTrusted false and programmatic focus now firing blur, focusout, focus and focusin.
@task_item n=12 part=touches
The driver emits no event and installs no subscriber, and one span per driver command: the driver's command execution was edited so its select focus calls the silent focus; no span field changed.
@task_item n=13 part=touches
Select chosen set and select list row label never logged, and the open list shot: select choice and row gestures now queue their events through the event queue.
@task_item n=14 part=touches
No file loads from non-file documents and a failed stylesheet leaves the pending critical set: the engine fetch path was edited (an import with an unparsable URL is refused) and a mask attribute URL now fetches through the same stamped request, a fragment-only one as a data URL.
@task_item n=15 part=touches
Snapshot rows that read the accessibility tree (clickable nodes and text, open list row bounds): the tree is now pushed after resolve when the changed set is non-empty, and an element with visibility collapse is hidden from it.
@task_item n=16 part=touches
The state registry and page-without-registered-state rows: the Dioxus native document and its signal registry test sources were re-applied over the merge, with Dioxus held at 0.7.3.
@task_fact n=1 slug=pii-scrubbing-compliance
fact: The DevTools protocol server, built only with the devtools feature that no default feature set turns on, logs under an engine target, its messages redacted by escher's sink.
@task_item n=1 part=untouched
The cold-agent pipe rows (its events, verdict fields and reasons, counted verbs, kept checks, transcript and server log): no part of the pipe was reached.
@task_item n=2 part=untouched
The agent-run harness rows (its events, raw output, cleanup and log content): the harness is unchanged.
@task_item n=3 part=untouched
The escher-session rows (its roles, exit codes, stdout, stderr, host wire, error object, coverage role and coverage line): no session code moved.
@task_item n=4 part=untouched
escher's own sink log line format, its resource keys service.name and service.version, its output stream, and the escher-telemetry panic hook and events: the sink and its installation are unchanged.
@task_item n=5 part=untouched
The recording file, recording bounds, recording and session state directories and the session lifecycle: not reached.
@task_item n=6 part=untouched
The build-directory sweep's stdout line: the sweep ran as before and its line did not change.
@task_item n=7 part=untouched
The command, graph, layout, actions and shot answer contracts and the refusal cause names: no answer shape or refusal cause moved.
@task_item n=8 part=untouched
The absences of a telemetry backend, metric emission, metrics exporters and an error-reporting service: the merge brings a document inspector, no telemetry, metric or error export.
@task_item n=9 part=untouched
The WASM console panic hook: the wasm CI job stays on its leg, unchanged.
