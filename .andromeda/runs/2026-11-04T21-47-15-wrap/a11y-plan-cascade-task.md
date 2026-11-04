@task_prose part=session
The session added one blitz-tests target, dioxus_runtime_probe (10 tests, both layout modes), that reads what Dioxus 0.7 exposes of component, handlers, written state and readers through public API alone, with no change under packages/ or examples/ and no Cargo.toml change.
It measured the reacts-to-a-click reading as link 2: on Cells the 677 marked no-role elements are 676 cells reacting to click and cells-grid to keydown; on Counter counter-increment carries click and counter-value none; CRUD's crud-list carries keydown; no snapshot field was added (NODE_FIELDS stays thirteen).
It measured that the bridge's listener attributes over-report: a removed listener leaves its attribute and the data-dioxus-id mark standing, a mounted listener writes none, while the vdom walk's dynamic attributes read each element's current listeners exactly.
It re-measured the MountedData hazard at HEAD and pinned it with two should_panic tests: set_focus in a task spawned by a key handler that wrote a signal panics RefCell already borrowed; set_focus called at once in a key handler moves focus and does not panic; the fix is routed to a new chunk, MountedData buffered render, on the founder's word.
The founder answered the probe's three forks: link 2 reads the vdom walk's dynamic attributes, the reacts-to-a-click reading becomes a snapshot field with the UI graph, and hook-local state is read through an escher-side signal registry.
An operator fix moved pulldown-cmark 0.13.5 to 0.13.6 in Cargo.lock (RUSTSEC-2026-0433) and added RUSTSEC-2026-0429 to the deny.toml ignore list, now eight IDs; the zip residual now lives as a store residual row, no .andromeda/residuals.md file stands.
@task_file n=1 path=tests/blitz-tests/tests/dioxus_runtime_probe.rs
@task_file n=2 path=Cargo.lock
@task_file n=3 path=deny.toml
@task_item n=1 part=touches
§7 actionable-key rule (fct-77ewqepn) and screen-reader-support sr-actionable-keys: the reacts-to-a-click reading is now measured by link 2 (Cells 676 cells click and cells-grid keydown, Counter counter-increment click and counter-value none, CRUD crud-list keydown); the snapshot field is still not added, and the founder decided it comes as a snapshot field with the UI graph.
@task_item n=2 part=touches
screen-reader-support sr-actionable-keys (fct-pfylw46r): the listener mark that DioxusDocument::unkeyed_actionable reads still holds an element whose listener was removed, so the actionable-key check over-includes (conservative), and a mounted listener writes no attribute; read, not run.
@task_item n=3 part=touches
keyboard-navigation (fct-gmhtpdel): the MountedData record re-measured at HEAD, its coordinates now DioxusDocument::poll holding inner.borrow_mut() at packages/dioxus-native-dom/src/dioxus_document.rs:295 across vdom.render_immediate at :297, set_focus at packages/dioxus-native-dom/src/events.rs:295 borrowing through doc_mut at events.rs:162, dioxus-core 0.7.10 polling the task through handle_task_wakeup (virtual_dom.rs:611); measured in both layout modes, its fix routed to the chunk MountedData buffered render.
@task_item n=4 part=touches
keyboard-navigation (fct-6luzygdm): its synchronous MountedData::scroll_to and NodeHandle claims were re-read and hold unmoved; the synchronous set_focus in a key handler was measured acting with no panic.
@task_item n=5 part=touches
a11y-scope-summary (fct-r6zswm2d): its move of pulldown-cmark to 0.13.5 is superseded; pulldown-cmark now resolves 0.13.6 in Cargo.lock, fixing RUSTSEC-2026-0433.
@task_item n=6 part=touches
Every a11y-plan claim owned by a .andromeda/residuals.md line (the Dioxus crates adapter fct-kbjv4usk, the tree lifecycle fct-zyrpeqlm, the ARIA states fct-cixbakze, the Cells role and cell semantics in §5 Per-surface keyboard shortcuts, the reacts-to-a-click reading fct-77ewqepn): residuals now live as residual rows in the store and no .andromeda/residuals.md file stands, so the path named is stale.
@task_item n=7 part=touches
Any blitz-tests count a11y-plan carries: 158 .rs files directly under tests/blitz-tests/tests/ and 157 test targets (157 and 156 before); run all passed 743, failed 0, ignored 12 (733 before); run stand unmoved at 237 over 51 files.
@task_item n=8 part=touches
keyboard-navigation keyboard-shortcuts and screen-reader-support: Cells' cells-grid is now measured as the one Cells element reacting to keydown and the 676 cells to click; CRUD's crud-list measured carrying keydown, its rows carrying no listener reading.
@task_item n=9 part=touches
Any a11y-plan statement on Dioxus listener removal or the bridge's listener attributes: remove_event_listener (packages/dioxus-native-dom/src/mutation_writer.rs:355-360) is reached only when a listener attribute leaves the element's attribute list, lowering the per-kind count alone; create_event_listener (mutation_writer.rs:331-352) writes the event-named attribute beside the mark.
@task_item n=10 part=touches
screen-reader-support sr-snapshot: NODE_FIELDS re-read at thirteen, unmoved; the founder's fork 3 answer schedules the reacts-to-a-click field with the UI graph, not now.
@task_fact n=1 slug=screen-reader-support
fact: The reacts-to-a-click reading, measured by dioxus_runtime_probe link 2 in both layout modes: on Cells the 677 elements carrying the listener mark and no role are 676 cells reacting to click and cells-grid reacting to keydown; on Counter counter-increment carries click and counter-value none; CRUD's crud-list carries keydown; no snapshot field added.
@task_fact n=2 slug=screen-reader-support
fact: The bridge's listener attributes over-report: a listener removed on a re-render leaves its event attribute and the data-dioxus-id mark while a click no longer acts, and a mounted listener writes none; the vdom walk's dynamic attributes (VNode.dynamic_attrs, VNode::mounted_dynamic_attribute, public dioxus-core 0.7.10 API) read each element's current listeners exactly, mounted included.
@task_fact n=3 slug=screen-reader-support
fact: The founder's answers to the probe's forks (2026-11-04, inputs#I6): link 2 reads the vdom walk's dynamic attributes, the reacts-to-a-click reading becomes a snapshot field with the UI graph, and links 3 and 4 for hook-local state go through an escher-side signal registry.
@task_fact n=4 slug=keyboard-navigation
fact: dioxus_runtime_probe (tests/blitz-tests/tests/dioxus_runtime_probe.rs, 10 tests, 8 looping both layout modes) pins the MountedData hazard as two should_panic tests, one per mode, their precondition asserted first (the handle held, the key target focused by Tab), and mounted_data_asked_at_once_in_a_key_handler_acts shows a synchronous set_focus moving focus; the port chunk MountedData buffered render inverts the pin; the target runs in the workspace test leg and run all, in neither the a11y leg nor stand-repeat.
@task_item n=1 part=untouched
§2 a11y-strategy: the tree lifecycle, changed_nodes, role derivation and ARIA state reads are untouched; no engine or builder code changed.
@task_item n=2 part=untouched
§4 aria-patterns-roles: the HTML-to-AccessKit mapping, ARIA state mapping and hidden-content rules are untouched.
@task_item n=3 part=untouched
§6 visual-design-verification: contrast tables, text_contrast, the select list colours and the visual lint are untouched; no stand page or harness reader changed.
@task_item n=4 part=untouched
§9 ci-integration: the a11y leg's fourteen files and the eight in stand-repeat are unmoved; the new target has no stand_ prefix and joins neither list, and no CI step or leg moved.
@task_item n=5 part=untouched
§5 focus order, Enter and Space activation, range and select key handling, and the Tab-order walks of stand_keyboard are untouched; no engine key or focus path changed.
@task_item n=6 part=untouched
The boolean-attribute rule (fct-fi2um7zb) and the bridge's attribute paths are untouched; mutation_writer.rs is unchanged.
@task_item n=7 part=untouched
§8 cognitive-accessibility, §3 harness helpers and the obs log format in §3 are untouched; the new target prints nothing (silence census 0).
@task_item n=8 part=untouched
The Dioxus crates' accessibility feature and platform adapter claims (fct-kbjv4usk, sr-platform-adapter) hold in substance: Dioxus stays at the 0.7.3 requirement, no manifest changed and no adapter built; only the residual's address moved.
