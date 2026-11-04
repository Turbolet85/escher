@task_prose part=chunk
Builds a probe of the UI graph's four links (node to the component that rendered it, node to its handlers and their event names with click as the "reacts to a click" answer, handler to the state it writes, state to the nodes or components that read it), read only through what the Dioxus 0.7.3 runtime and escher's bridge dioxus-native-dom already expose, each link given one verdict: exposed, exposed with a cost, or not exposed without a fork-level change.
Each verdict rests on a run on the headless seven_guis stand (a lean task whose handler writes state another node reads, Counter or CRUD, and one served task beside it), in both layout modes where layout touches the reading; a verdict read from the code and not run says so; the probe's code is test or example code that measures, and what is kept of it is the plan's choice.
Re-verifies at HEAD, after the 2026-11-01 upstream sync, the MountedData hazard's mechanism and coordinates: a task spawned from a key handler that writes a signal is first polled inside render_immediate while the document's RefCell is held in the bridge's poll, and a MountedData call in it panics RefCell already borrowed at events.rs:162, measured headlessly through the harness's poll(None); the windowed path is read, not run.
Writes one plain-words reading for the founder (each link's verdict with its evidence, the cost class of building the UI graph on what is exposed, the forks needing his word) and names the MountedData hazard there with its fix brought as his fork with its options, not decided; no Dioxus crate is patched, vendored or re-pinned and no engine change is made.
@task_file n=1 path=packages/dioxus-native-dom/src/events.rs mark=re-reads
what: the MountedData panic's coordinate, RefCell already borrowed at events.rs:162, re-verified at HEAD
@task_file n=2 path=packages/dioxus-native-dom/src/dioxus_document.rs mark=re-reads
what: poll, the render_immediate path holding the document's RefCell when the spawned task is first polled, re-verified at HEAD
@task_file n=3 path=.andromeda/cache/chunks/2026-10-31-engine-debts/evidence/readings.md mark=re-reads
what: the section A hazard measured beside step 15, the headless measurement of the MountedData mechanism read back against HEAD
@task_item n=1 part=needs
The blitz-test-harness Dioxus-backed construction the probe boots through: Harness::from_vdom, from_component, and the stand's seven_guis::stand::boot(LeanTask, HarnessOptions) for Counter or Crud with stand::options(incremental) as the pinned options, each a fresh VirtualDom rendering the task in TaskShell.
@task_item n=2 part=needs
The served task beside the lean one: the stand's boot_cells and boot_load (the load page's options carrying the LoadNet stand-in, every other field kept), the only boots that reach the served pages.
@task_item n=3 part=needs
The harness's pump and settle semantics the hazard's reading rests on: pump polls the document with no waker context, resolves at the harness time and is the poll(None) path the panic was measured through; dispatch and dispatch_recorded do not pump; settle repeats the pass, advances no time and logs nothing.
@task_item n=4 part=needs
The harness input helpers that drive a handler on the stand: click, press and press_with (the key path the hazard needs, keyboard-types Key, Modifiers and NamedKey re-exported by the harness), type_text, each pumping after dispatch, and the inspection helpers query, node, text_content, attr and dom_string to read which nodes changed.
@task_item n=5 part=needs
dispatch_recorded returns the dispatched event names but drives the BaseDocument directly, bypassing Dioxus VirtualDom forwarding: a value bearing on how link (2) and the click reading are measured, since a recorded name is not a Dioxus handler run.
@task_item n=6 part=needs
The dioxus-native-dom bridge surfaces the links are read through: DioxusDocument, the stable element id and element_ids(), the listener map and event names, the document's events, and the existing unit-test build of a DioxusDocument from DocumentConfig::default() with initial_build, mark_dirty, poll(None) and a GlobalSignal flipped inside vdom.in_runtime to re-render.
@task_item n=7 part=needs
The bridge_tests rule over DioxusEventHandler: its two fields are private to dioxus_document and it has no constructor, so a probe reading a handler from inside the crate sits in a cfg(test) child of that module, and no function under test gains a constructor, a pub or a cfg(test) shim; a reading that needs one is an exposed-with-a-cost verdict to name, not to take.
@task_item n=8 part=needs
The unit-level limit of the dioxus-native-dom test helper: DocumentConfig::default() carries the no-op HTML parser, so dangerous_inner_html renders nothing at unit level and parsed markup is read in blitz-tests where the harness has the real parser; a snapshot or layout read at unit level calls resolve(0.0) through inner_mut() first.
@task_item n=9 part=needs
Both layout modes: the stand's options take incremental true and false, and a reading layout touches runs in each, as the stand's session and snapshot checks do.
@task_item n=10 part=needs
The headless-stand font: the bundled DejaVu Sans with system fonts off and no skip path, so a probe run on the stand measures the same on every runner and in the package-alone build.
@task_item n=11 part=needs
Time on the stand comes from the harness's VirtualClock (tick, advance_time, TimerTicks for the Timer): a probe check uses no sleep and no wall-clock read, the session checks' wall-time rule.
@task_item n=12 part=needs
Values masked, never logged, over what the probe reads: a component name, a handler or a signal's value is not printed, logged or put in an assertion message or a field; a kept probe check's assertions carry fixed words or a row index only, as escher-driver's command assertions do.
@task_item n=13 part=needs
The MountedData hazard as a measured reading: reproducing a known panic in a check (whether by should_panic, catch_unwind or a run read and not kept) is the plan's choice inside the boundary that the fix is not made here and no Dioxus or engine code changes.
@task_item n=14 part=needs
A kept probe check in tests/blitz-tests/tests/ joins the agent-run harness's selections (run stand when it carries the stand_ prefix, else run all and run by name) and moves the newest proof reading, run stand 237 over 51 files and run all 733, which a kept target's chunk re-counts.
@task_item n=15 part=needs
A kept stand_ file joins the stand-repeat leg and its zero-failure budget (three rounds, no retry), so a probe check kept there must be deterministic on every round in both layout modes.
@task_item n=16 part=needs
The engine-features-by-runner rule: the workspace build and the -p blitz-tests build compile the same test file against different engine features (tracing and writing-mode on in the workspace only), so a probe check's green per-package run is not the workspace leg's green.
@task_item n=17 part=needs
The coverage leg's per-crate floor on packages/dioxus-native-dom (83 percent, keyboard_compat.rs excluded) still binds if any probe code is kept in that crate; a cfg(test) child file is no record of the per-crate reading.
@task_item n=18 part=needs
The stand's process-free reach: the probe runs in process on the headless stand, never through a session host, socket or the windowed seven_guis_native binary, the windowed path being read from the code only.
@task_standing n=1
convention: unit tests sit in a #[cfg(test)] mod tests inside the source file, or in a file of their own declared #[cfg(test)] with #[path] at its parent, their test paths unchanged
home: unit-test-strategy
@task_standing n=2
convention: blitz-tests keeps one test-target file per behaviour under tests/blitz-tests/tests/, picked up by autotests with no [[test]] table added beside the unbuilt all, as BlitzTestsTargetsTest pins
home: §2. Test Strategy (Directory pattern) and §4. Unit Test Strategy (blitz-tests targets)
@task_standing n=3
convention: a stand check declares mod common; after its use block and reads the shared tables and helpers of tests/common/mod.rs, a fixture page more than one check reads lives in tests/fixtures/mod.rs, and a page with one reader stays in its check's file
home: §3. Test Harness Contract (Crate-local test helpers) and test-data-fixtures
@task_standing n=4
convention: a regression test is named for the bug it guards and documents the bug and what exposed it
home: §2. Test Strategy (Test function naming)
@task_standing n=5
convention: a headless-stand check asserts unconditionally under the bundled DejaVu Sans with system fonts off and has no skip path
home: §2. Test Strategy (Font-dependent tests) and mocking-stubbing-discipline
@task_standing n=6
convention: a check reads no wall clock and sleeps not, time moving only through the harness's virtual clock, bar the checks whose subject is wall time
home: integration-test-strategy
@task_standing n=7
convention: no third-party test framework or mocking library is added; Rust checks run under the standard built-in harness, with hand-written stand-ins only
home: §3. Test Harness Contract (Third-party test frameworks) and §8. Mocking & Stubbing Discipline
@task_standing n=8
convention: every stand_ check holds the zero-failure budget, run three rounds with no retry in the stand-repeat leg
home: quality-gates-coverage-targets
@task_standing n=9
convention: a check that reads a sink's capture counts lines by target, never in total, since the workspace and package-alone builds differ in engine features
home: ci-integration
@task_item n=1 part=not_needed
The WPT runner, its reftests, scoring, quarantine, workflows and Thai-font install: the probe reads no CSS conformance and runs no WPT suite.
@task_item n=2 part=not_needed
The apps/browser tests (history, suggestions, persistence, favicon, tab): the chunk reaches no browser crate.
@task_item n=3 part=not_needed
blitz-vibey-script's script tests and the Preact TodoMVC end-to-end flow: the probe is over the Dioxus runtime, not the script engine.
@task_item n=4 part=not_needed
The cold-agent run pipe, its verdict fields, its judge and its live run: the chunk adds no verb, tool or MCP change and runs no cold agent.
@task_item n=5 part=not_needed
The escher-driver session surfaces: its verbs, refusals, Session::run diff, MCP role, session host and socket lifecycle and the capability statement's counts: the probe adds no verb, argument, cause, exit or result field and builds no driver reading.
@task_item n=6 part=not_needed
The snapshot model's own checks (snapshot builder, text form, parts, diff, stand_snapshot files): the UI graph is not built here and no snapshot field is added.
@task_item n=7 part=not_needed
Recorded-check replay and the committed checks under tests/blitz-tests/checks/ with record_checks.sh: the probe records no session.
@task_item n=8 part=not_needed
Pixel tests, the harness's screenshot and text_contrast, and the visual lint: no verdict reads a frame or a colour.
@task_item n=9 part=not_needed
The blitz-dom engine unit tests (style bridge, layout, list markers, text-transform, select model, clock, net, form) and the differential incremental oracle: no engine change is made, so their coverage is not this chunk's.
@task_item n=10 part=not_needed
The CI Python script tests of the WPT diff, the area changes, apt-install, the agent-run and cold-agent scripts: no script, workflow or install step changes.
@task_item n=11 part=not_needed
The paint_tree_bench benchmarks, paint_bench, the screenshot example and the manual iframe and rowspan fixtures under examples/assets: none bears on the runtime's exposure.
@task_item n=12 part=not_needed
The CI take-up reading of the last wrap (17 of 17 green) and the eight local records commits: no red is folded into this chunk.
@task_item n=13 part=not_needed
The verification of v030-05 and the UI graph's acceptance (one read naming each node's component): this chunk advances only the probe-reached-the-founder half; the graph and its claim belong to the later chunk.
