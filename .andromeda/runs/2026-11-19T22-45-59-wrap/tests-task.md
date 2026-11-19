@task_prose part=session
The session merged upstream/main at 547d7514b into the build branch as a two-parent merge, CI green 17 of 17 on the first attempt.
It bumped the engine dependencies (Stylo 0.22 to 0.24, selectors 0.43, parley 0.13, skrifa 0.49, winit =0.31.0-beta.4, svgtypes 0.16.2), held Dioxus at 0.7.3, moved blitz-net to rustls, and made four precise lock updates to clear eight audit advisories.
It took upstream's event-queue model in place of the dispatch callback, programmatic focus now queueing blur, focusout, focus and focusin, with two silent focus paths kept for the inert fix-up and the driver's select focus.
It added the opt-in devtools CDP server crate as a workspace member with its listener and two env reads, retired parley_code_editor, split blitz-dom's stylo module, and moved the accessibility refresh to the shell's redraw after resolve.
It fixed an upstream inline-box placeholder defect in the engine with its own controlled blitz-dom witness, red on an upstream clone, and added a measuring check that a mask attribute URL is fetched through the stamping constructor.
Nine new blitz-tests targets stand: eight upstream files and this run's svg_mask_attribute_fetch; build, fast, doc, a11y, audit and stand-repeat legs all exit 0, stand-repeat 3 of 3 rounds green.
@task_file n=1 path=Cargo.toml mark=merge
what: the workspace manifest — its members and the engine dependency pins
@task_file n=2 path=Cargo.lock mark=merge
what: the workspace lockfile the locked cargo legs build from
@task_file n=3 path=deny.toml mark=merge
what: the dependency audit's configuration — the licence table and the advisory ignores with their reasons
@task_file n=4 path=.github/workflows/ci.yml mark=merge
what: the CI workflow — its jobs, the native platform matrix with the Android job, SHA-pinned actions, the wasm job on the leg script
@task_file n=5 path=.github/workflows/wpt.yml mark=merge
what: the WPT conformance workflow, run on upstream only
@task_file n=6 path=.github/scripts/test_cargo_lock.py mark=merge
what: a new CI Python script test under unittest, riding the ci-scripts leg, checking the lockfile's prettyplease and syn pairing
@task_file n=7 path=.github/scripts/wpt_diff_to_pr.py mark=merge
what: the CI script that turns a WPT diff into a pull request, now skipping results on a revision mismatch
@task_file n=8 path=tests/blitz-tests/tests/all.rs mark=merge
what: the blitz-tests shared module set the per-file test targets draw on
@task_file n=9 path=tests/blitz-tests/tests/svg_mask_attribute_fetch.rs
what: a new blitz-tests target measuring that an SVG mask attribute URL is fetched stamped with its document, in both layout modes
@task_file n=10 path=tests/blitz-tests/tests/event_queue.rs mark=merge
what: a new upstream blitz-tests target for the event queue — programmatic focus events, dropped-node discard, flush bound, scroll coalescing
@task_file n=11 path=tests/blitz-tests/tests/clipboard.rs mark=merge
what: a new upstream blitz-tests target for paste, copy and cut input events
@task_file n=12 path=tests/blitz-tests/tests/pointerup_cancelled.rs mark=merge
what: a new upstream blitz-tests target — a cancelled pointerup still clicks and still ends a scrollbar drag
@task_file n=13 path=tests/blitz-tests/tests/pointerdown_cancelled.rs mark=merge
what: a new upstream blitz-tests target — a cancelled pointerdown counts towards a double click without selecting
@task_file n=14 path=tests/blitz-tests/tests/wheel_axis_lock.rs mark=merge
what: a new upstream blitz-tests target for wheel gesture axis locking
@task_file n=15 path=tests/blitz-tests/tests/native_wheel_data.rs mark=merge
what: a new upstream blitz-tests target — an onwheel handler reads the wheel phase and source through the harness
@task_file n=16 path=tests/blitz-tests/tests/overscroll_behavior.rs mark=merge
what: a new upstream blitz-tests target for overscroll-behavior scroll chaining
@task_file n=17 path=tests/blitz-tests/tests/pseudo_document_order.rs mark=merge
what: a new upstream blitz-tests target for pseudo-element document order
@task_file n=18 path=tests/blitz-tests/tests/accessibility_hidden.rs mark=merge
what: the blitz-tests target for nodes hidden from the accessibility tree, now with visibility collapse
@task_file n=19 path=tests/blitz-tests/tests/fragment_navigation.rs mark=merge
what: the blitz-tests target for fragment navigation and smooth scroll, now across a viewport resize
@task_file n=20 path=packages/blitz-dom/src/document_inline_box_placeholder_tests.rs
what: a new inline unit test child module of the document — the controlled witness that an inline-block in text no font can shape is placed once its own font loads, in both layout modes
@task_file n=21 path=packages/blitz-dom/src/document.rs mark=merge
what: the engine document — focus, the event queue, the changed set, font context set-up, its inline unit test modules
@task_file n=22 path=packages/blitz-dom/src/layout/inline/build.rs mark=merge
what: the engine's inline layout build, where the inline-box placeholder takes its fallback font family
@task_file n=23 path=packages/blitz-dom/src/lib.rs mark=merge
what: the engine crate root, holding the placeholder font family name and its re-exports
@task_file n=24 path=packages/blitz-dom/src/events/pointer.rs mark=merge
what: the engine's pointer event handling — pointerdown, pointerup, click — on the event queue
@task_file n=25 path=packages/blitz-dom/src/events/focus.rs
what: deleted — the engine's old focus event generation
@task_file n=26 path=packages/blitz-dom/src/stylo.rs
what: deleted — the engine's style module, split upstream into a stylo directory
@task_file n=27 path=packages/blitz-dom/src/stylo/prefs.rs mark=merge
what: the Stylo prefs set once per process, now by the Stylo 0.24 names
@task_file n=28 path=packages/blitz-dom/src/accessibility.rs mark=merge
what: the engine's accessibility tree build, now leaving out visibility collapse
@task_file n=29 path=packages/blitz-dom/src/mutator.rs mark=merge
what: the document mutator, its flush's inert fix-up clearing focus silently, and devtools mutation records
@task_file n=30 path=packages/blitz-dom/src/scrolling.rs mark=merge
what: the engine's scrolling, its smooth scroll progress read on the document clock
@task_file n=31 path=packages/blitz-shell/src/window.rs mark=merge
what: the windowed shell's view, now refreshing the accessibility tree in redraw after resolve
@task_file n=32 path=packages/blitz-test-harness/src/contrast.rs mark=merge
what: the harness's text contrast reading
@task_file n=33 path=packages/escher-driver/src/execute.rs mark=merge
what: the driver executor's select focus step, now focusing with no focus event
@task_file n=34 path=packages/blitz-vibey-script/tests/dom.rs mark=merge
what: the vibey-script DOM integration tests
@task_file n=35 path=packages/dioxus-native-dom/src/dioxus_document_tests.rs mark=merge
what: the bridge tests, a child module of the dioxus document
@task_file n=36 path=packages/blitz-vibey-cdp-server/Cargo.toml
what: the new devtools CDP server crate, a workspace member behind the devtools feature, with upstream's own integration tests
@task_file n=37 path=packages/parley_code_editor
what: deleted — the parley code editor package, gone from the workspace
@task_file n=38 path=apps/browser/Cargo.toml mark=merge
what: the browser app's manifest, skia now among its default features
@task_file n=39 path=wpt/runner/src/main.rs mark=merge
what: the WPT runner CLI — suite arguments now matched through the WPT manifest, renderers chosen by argument, skia in its defaults
@task_file n=40 path=AGENTS.md
what: deleted — upstream's agents file, kept out
@task_item n=1 part=touches
The one-target-per-file and one-target-per-behaviour decisions and the blitz-tests manifest's difference from upstream: nine new blitz-tests targets stand, eight upstream files and svg_mask_attribute_fetch; the manifest's shape is unchanged.
@task_item n=2 part=touches
The dependency audit decision: the licence table gains CDLA-Permissive-2.0, the six ignores' reasons re-worded to winit 0.31.0-beta.4 and accesskit_unix 0.24.1, and four precise lock updates (pulldown-cmark, rustls, rustls-webpki, webpki-root-certs) cleared eight advisories.
@task_item n=3 part=touches
Cargo legs run locked: the lock was settled by cargo metadata, not an unlocked check, then checked locked.
@task_item n=4 part=touches
CI actions pinned to commit SHAs: the Android job gains nttld/setup-ndk pinned to ed92fe6cadad69be94a966a7ee3271275e62f779.
@task_item n=5 part=touches
The native platform CI matrix: the Android job gains upstream's NDK steps; the wasm job stays on the leg script.
@task_item n=6 part=touches
CI Python scripts unit-tested under unittest: a new upstream lockfile script test and the WPT diff script's revision-mismatch tests ride the ci-scripts leg.
@task_item n=7 part=touches
WPT workflow upstream-only and the WPT conformance runner: the WPT workflow and runner changed; the runner CLI's suite arguments now select through the WPT manifest with glob matching, renderer kind taken by argument.
@task_item n=8 part=touches
Pixel tests on a CPU buffer, indirectly: the browser app and the WPT runner now build skia by default beside the CPU renderer.
@task_item n=9 part=touches
blitz-dom builds requests through one stamping constructor: a mask attribute URL reaches the fetch only through it, and a fragment-only mask URL is fetched as a stamped data URL, measured by a new target.
@task_item n=10 part=touches
Expected value pinned as a literal: the mask check pins the measured fragment fetch, not what the path should do.
@task_item n=11 part=touches
Tests controlled red-before-green and regression tests named for their bug: the inline-box placeholder witness read red on an upstream clone, renamed, flagged upstreamable.
@task_item n=12 part=touches
Font-metric tests never skip and pre-existing font tests skip without a font, indirectly: the engine now registers a fallback placeholder family in every font context, a given one included; stand cross-OS readings unmoved in fast and in all three stand-repeat rounds.
@task_item n=13 part=touches
Stand checks and layout assertions in both layout modes and the incremental differential oracle: the new witness and mask checks run in both modes; the oracle green.
@task_item n=14 part=touches
Stand checks repeated three rounds and the zero-failure stand-repeat budget: stand-repeat 3 of 3 rounds green on the merge.
@task_item n=15 part=touches
The a11y leg and harness graph tests that turn accessibility on, indirectly: the accessibility tree refresh moved from poll to redraw after resolve, read from the changed set; visibility collapse now hides a node from the tree.
@task_item n=16 part=touches
change events come from the engine alone, indirectly: every engine event path now queues through the user-agent event queue with no dispatch callback; the runtime change synthesis stays retired.
@task_item n=17 part=touches
Recorded dispatch bypasses document handling and the harness core dispatch, indirectly: the engine's pointer, click, key-up, select, range and generated text-input paths take no dispatch callback and queue events instead.
@task_item n=18 part=touches
Two clicks with no advance are a double click, indirectly: a cancelled pointerdown now counts towards a double click without selecting, and a cancelled pointerup still clicks.
@task_item n=19 part=touches
One virtual clock and the scrollbar fade on the document clock: smooth scroll progress now reads the document's clock.
@task_item n=20 part=touches
keyboard_compat left out of coverage and the buffered render on the Dioxus 0.7 bridge: Dioxus held at 0.7.3; the 0.8 deletes of mutation_writer, events and keyboard_compat restored to HEAD's, the exclusion re-read at the next sync.
@task_item n=21 part=touches
Per-crate coverage floors and the workspace tested in the fast leg, indirectly: the workspace gains the CDP server crate as a member and loses the parley code editor package.
@task_item n=22 part=touches
The telemetry target allowlist and the sink dropping third-party targets, indirectly: the CDP server logs on an engine target, its message redacted by escher's sink; capture checks green in fast and stand-repeat.
@task_item n=23 part=touches
Bridge tests a child of the dioxus document: the bridge tests and the dioxus document were re-applied on HEAD's dioxus-native-dom.
@task_item n=24 part=touches
Inline unit test modules and unit tests in their source file: the witness is a new child test module of the document in its own file, and upstream's change-tracking tests join the document's inline modules.
@task_fact n=1 slug=test-strategy
fact: An upstream engine defect found during a sync is fixed in escher with its own blitz-dom witness, run red first on an upstream clone at the merge base, and flagged upstreamable.
@task_fact n=2 slug=test-strategy
fact: The devtools CDP server crate is tested by upstream's own integration tests alone, kept as written; escher adds none, as the crate is opt-in behind the devtools feature and no default feature set builds it.
@task_item n=1 part=untouched
The cold-agent pipe's events, exit codes, verdict and isolation rules: nothing of the cold-agent run changed.
@task_item n=2 part=untouched
The agent-run script's verbs, selections, events and exit codes: the leg script and agent-run are unchanged.
@task_item n=3 part=untouched
The session wire protocol, session host and session state directory contracts: the socket, its bounds and its versions did not move.
@task_item n=4 part=untouched
The escher-session command line, its MCP role, exit status and recording contracts: no verb, flag or output changed.
@task_item n=5 part=untouched
Mutation testing on the dev host and the cognitive-complexity ceiling: no mutation or complexity reading was taken or moved.
@task_item n=6 part=untouched
The build directory sweep and the target-sweep safety rules: the sweep ran as before, its rules unchanged.
@task_item n=7 part=untouched
Committed recorded checks and handler coverage: no recorded check, replay or coverage line changed.
