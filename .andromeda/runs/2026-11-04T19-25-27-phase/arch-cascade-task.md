@task_prose part=chunk
Builds a probe of the four links of the UI graph (node to component, node to handlers, handler to state written, state to reading nodes), each read through what the Dioxus 0.7.3 runtime and escher's dioxus-native-dom bridge already expose, with no fork-level change.
Each link gets one of three verdicts (exposed, exposed with a cost, not exposed without a fork-level change), measured by runs on the headless seven_guis stand (a lean task such as Counter or CRUD plus one served task), in both layout modes where layout touches the reading.
Answers the deferred "reacts to a click" reading: whether a node's click listener is readable from the listener map today.
Re-verifies at HEAD, after the 2026-11-01 upstream sync, the MountedData hazard's mechanism and coordinates (render_immediate holding the document RefCell in poll, the panic at events.rs:162), re-reading the engine-debts reading.
Writes one plain-words reading for the founder: each verdict with its evidence, the cost class of building the UI graph, and the forks needing his word, the MountedData fix among them with its options, not decided here.
@task_file n=1 path=packages/dioxus-native-dom/src/dioxus_document.rs mark=re-reads
what: the MountedData hazard's mechanism, `poll` first polling a spawned task inside `render_immediate` while the document's RefCell is held, re-verified at HEAD
@task_file n=2 path=packages/dioxus-native-dom/src/events.rs mark=re-reads
what: the MountedData `RefCell already borrowed` panic coordinate `events.rs:162`, re-verified at HEAD
@task_file n=3 path=.andromeda/cache/chunks/2026-10-31-engine-debts/evidence/readings.md mark=re-reads
what: §A hazard measured beside step 15, the headless measurement through the harness's `poll(None)`
@task_file n=4 path=examples/seven_guis/
@task_file n=5 path=tests/blitz-tests/
@task_item n=1 part=needs
The Dioxus DOM bridge in dioxus-native-dom: `DioxusDocument`, DOM events reaching the vdom through `data-dioxus-id`, and `element_id` / `element_ids` with their four tiers (author key, anchored path, component path, document path); the probe reads these for link (1) and link (2) and writes nothing to DOM or vdom.
@task_item n=2 part=needs
The bridge's listener reads (the listener map and its event names) as link (2)'s surface, including whether a `click` listener is readable today.
@task_item n=3 part=needs
The Dioxus 0.7.3 runtime crates in the dependency graph, read only: components, handlers, signals and their readers are probed through public or bridge-held APIs, each API named in its verdict.
@task_item n=4 part=needs
Dioxus stays pinned at "0.7.3": no patch, vendored copy or pin move of any Dioxus crate (the Inherited Defaults and Upstream sync lines that hold Dioxus 0.8.0-alpha.3 back for the next version).
@task_item n=5 part=needs
The headless stand `seven_guis::stand`: the four lean tasks with a handler writing state another node reads (Counter, CRUD), its pinned 800x600 Light viewport with one bundled font, offline boots, no telemetry and no env read.
@task_item n=6 part=needs
The six served stand tasks behind `escher-session` and the driver `Session` as the home of the served task the probe runs beside a lean one; the probe adds no verb, argument, cause or exit to them.
@task_item n=7 part=needs
The test harness (`Harness`): pumping input and `poll(None)` without settling, the shared `VirtualClock` that only `tick` and `advance_time` move; the hazard was measured through its `poll(None)`.
@task_item n=8 part=needs
Both layout modes producing identical layouts: a verdict whose reading layout touches is run in both modes.
@task_item n=9 part=needs
The MountedData hazard: a task spawned from a key handler that writes a signal is first polled inside `render_immediate`, and a `MountedData` call in it panics `RefCell already borrowed`; named in the reading with its measured mechanism, the windowed path stated as read from the code, not run.
@task_item n=10 part=needs
Shell loop and dioxus-native launch path: every window event followed by a Poll, the windowed shell running the same `poll` lines; read, not run, unless a reading needs it.
@task_item n=11 part=needs
The rule on direct engine changes for every document (Established Decisions): a MountedData fix is one, so the chunk makes none; the fix is brought to the founder as a fork with its options, and where a chosen fix lands is part of that fork.
@task_item n=12 part=needs
Event processing: Dioxus skips event kinds with zero handlers, and handlers run before the default action; this bears on what the listener map shows for links (2) and (3).
@task_item n=13 part=needs
Service wiring: shared state is `Rc<RefCell<…>>` and Dioxus services are root contexts; this bears on the RefCell borrow the hazard trips and on what state link (3) and link (4) can read.
@task_item n=14 part=needs
Nothing crosses: the driver session's schema (twelve verbs, eight closed causes, the `v3` wire, `command` and `tool_call` spans) and the snapshot, its text and diff stay as they are; no node key, result field, session error, carrier or `NodeId` is added.
@task_item n=15 part=needs
Logging: no component name, handler or signal value the probe reads is printed, logged or put in a field; escher-telemetry's field rules (`SAFE_FIELDS`, `CONTENT_FIELDS` redacted) and the ban on a log carrying the snapshot hold over any probe output.
@task_item n=16 part=needs
Occupied resources: the probe adds no listener, port, env var, workspace crate, binary name or state directory.
@task_item n=17 part=needs
The `accessibility` feature, which gates the dioxus-native-dom snapshot modules, where a probe check reads the snapshot read-only.
@task_item n=18 part=needs
The CI gates any probe test or example code must pass: the `test` leg, `stand-repeat`'s zero-failure budget, the clippy, fmt and rustdoc gates, and the per-crate coverage floors.
@task_standing n=1
convention: each new integration test file covers one behavior, opens with a `//!` doc naming it, runs its scenario in both layout modes, first asserts the fixture produces the condition, and is one test target per file in blitz-tests
home: conventions
@task_standing n=2
convention: stand checks share helpers through modules under `tests/` that are no test target; `tests/blitz-tests/tests/fixtures/mod.rs` states the fixture boot and hold once, and a page with one reader stays in its check's file
home: conventions
@task_standing n=3
convention: new code passes `cargo fmt --all --check` and `cargo clippy --workspace --locked -- -D warnings`, with `clippy::expect_used` / `clippy::unwrap_used` only next to a justification
home: conventions
@task_standing n=4
convention: new modules and files open with `//!` docs, public items carry `///`, and an example opens with a `//!` comment describing what it demonstrates
home: conventions
@task_standing n=5
convention: a Dioxus example declares `#[component]` components with signal and store hooks, reactive state through `#[derive(Store)]`, its CSS in `const` raw strings, and fails with `.unwrap()`, `.expect(..)` or `panic!`
home: conventions
@task_standing n=6
convention: `unsafe` blocks carry `// SAFETY:` comments, and borrow conflicts are worked around by taking a child list or layout out and restoring it
home: conventions
@task_item n=1 part=not_needed
The select model and the open select list (`apply_select_choice`, `open_select`, `select_list_view`, `reconcile_select_list`, the select popup's merge surface): the probe reads no select.
@task_item n=2 part=not_needed
JavaScript on Boa (blitz-vibey-script, `ScriptDocument`, the JS bootstrap, timers and module loader): the probe reads a Dioxus app, not page scripts.
@task_item n=3 part=not_needed
The WPT runner, its outputs, bucketing and the WPT and post-results workflows.
@task_item n=4 part=not_needed
The browser app and rdme: history store, navigation provider, document loader, about pages, URL suggester.
@task_item n=5 part=not_needed
Painting and renderer backends: `paint_scene`, layer caps, password masking in paint, vello and skia choices, screenshot rendering.
@task_item n=6 part=not_needed
Networking crates and outbound hosts: blitz-net's provider, user agent, per-host semaphore, HTTP cache, and the hosts list.
@task_item n=7 part=not_needed
Deployment and packaging: publish-browser bundles, the Nix flake, Cross.toml and cross-compilation, signed builds.
@task_item n=8 part=not_needed
Release versioning, build profiles and the MSRV build job.
@task_item n=9 part=not_needed
CSS and layout decisions: CSS approximations, unsupported features, fonts, inline SVG, tables as grid, HTML parsing.
@task_item n=10 part=not_needed
The upstream sync merge surfaces of earlier engine changes (the eight upstreamable changes, `hit_inner`'s conflict with `0980cce5f`, the key-press panic fix): the probe merges nothing.
@task_item n=11 part=not_needed
Style threading and `PARALLEL_TRAVERSAL_LOCK`: the headless stand sets no `style_threading`.
@task_item n=12 part=not_needed
The accessibility adapter `accesskit_xplat` and its platform adapters.
@task_item n=13 part=not_needed
Licensing exceptions and the fixture forms of the google and graphite pages.
@task_item n=14 part=not_needed
Scrolling, hit testing and `visible_region` contracts: no link of the UI graph reads geometry.
