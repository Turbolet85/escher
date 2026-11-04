@task_prose part=session
The session added one blitz-tests target, tests/blitz-tests/tests/dioxus_runtime_probe.rs (10 tests, 8 looping both layout modes), reading what Dioxus 0.7.10 public API exposes of a node's component, its handlers, the state a handler writes and that state's readers, with no fork-level change and nothing under packages/ or examples/ and no Cargo.toml moved.
It re-measured the MountedData hazard at HEAD and pinned it as two should_panic tests: DioxusDocument::poll holds inner.borrow_mut() at packages/dioxus-native-dom/src/dioxus_document.rs:295 across vdom.render_immediate at :297, and a task spawned from a key handler that calls set_focus (events.rs:295) borrows through doc_mut (events.rs:162) and panics; the fix is routed by the founder to its own port chunk, MountedData buffered render.
It measured that the bridge's listener attributes over-report (a removed listener's attribute and data-dioxus-id mark stay, a mounted listener writes none) while the vdom walk's dynamic attributes read current listeners exactly.
An operator fix commit bumped pulldown-cmark 0.13.5 to 0.13.6 in Cargo.lock alone (RUSTSEC-2026-0433) and added RUSTSEC-2026-0429 (zip 2.4.2) to the deny.toml advisories ignore list, now eight IDs at deny.toml:20-30.
Counts moved: blitz-tests 158 .rs files and 157 test targets; run all 743 passed, 0 failed, 12 ignored; run stand unmoved at 237 over 51 files. CI green 17/17 on 9b3b29714.
The founder answered the three forks for The UI graph: an escher-side signal registry for hook-local state, the vdom walk as the link-2 reader, and the reacts-to-a-click reading becoming a snapshot field with the graph.
@task_file n=1 path=tests/blitz-tests/tests/dioxus_runtime_probe.rs
@task_file n=2 path=Cargo.lock
@task_file n=3 path=deny.toml
@task_item n=1 part=touches
The MountedData record in Established Decisions (dec-dioxus-integration area, fct-apdputpm): its coordinates re-measured at HEAD — borrow_mut at dioxus_document.rs:295 across render_immediate at :297, dioxus-core 0.7.10 polling the queued task through handle_task_wakeup (virtual_dom.rs:611), set_focus at events.rs:295 borrowing through doc_mut at events.rs:162 — now pinned by dioxus_runtime_probe in both layout modes, its fix routed to a separate port chunk.
@task_item n=2 part=touches
The pulldown-cmark resolution in Stack and Technologies (fct-kjoxjobc): resolved 0.13.5 becomes 0.13.6, fixing RUSTSEC-2026-0433, reached only through rdme, which never sets ENABLE_WIKILINKS.
@task_item n=3 part=touches
Any architecture text naming the zip 2.4.2 advisories ignored in deny.toml: three IDs become four (RUSTSEC-2026-0367, 0368, 0369, 0429), zip reached only through anyrender_serialize 0.8.0 under the browser capture feature, which never calls ZipArchive::extract.
@task_item n=4 part=touches
The cargo-deny advisory gate in Infrastructure Patterns, build system: the cited deny.toml range (deny.toml:1-30) — one line added at 26, the ignore list now eight IDs at deny.toml:20-30.
@task_item n=5 part=touches
Any count of blitz-tests files or test targets in Existing Scopes or Conventions: 158 .rs files and 157 test targets (157 and 156 before), dioxus_runtime_probe.rs the latest; the report found no architecture site carrying it, so a classifier should confirm rather than assume.
@task_item n=6 part=touches
Any run all total carried in architecture: 743 passed, 0 failed, 12 ignored (733 before); run stand unmoved.
@task_item n=7 part=touches
The Tests convention (conv-tests): a new integration test file with one target, both layout modes, the fixture precondition asserted first (hazard_ready); no stand_ prefix, so it is in neither run stand, stand-repeat nor the a11y leg, reached by the workspace test leg, run all and run dioxus_runtime_probe.
@task_item n=8 part=touches
The Dioxus DOM bridge contract (contract-dioxus-dom-bridge): the listener attributes create_event_listener writes (mutation_writer.rs:331-352) over-report — remove_event_listener (mutation_writer.rs:355-360) lowers only the per-kind count, leaving the attribute and the data-dioxus-id mark; create_event_listener and remove_event_listener keep 0 workspace callers, re-read unmoved.
@task_item n=9 part=touches
The actionable-element reading over the listener mark (unkeyed_actionable and any rule reading the data-dioxus-id mark as listener evidence): indirectly, through the over-report, an element whose listener was removed still reads as carrying a listener, so such a reading over-includes (conservative).
@task_item n=10 part=touches
Project Intent's upstream sync line naming 286874cc1 among the commits left out with the Dioxus 0.8 move (fct-4ncs7geh): still true, but its buffered render is now routed for a 0.7 port in a chunk of its own, upstream's test task_polled_during_render_can_access_document its witness.
@task_item n=11 part=touches
The state convention (state via use_signal, conv-apps-and-examples and fct-2wv4ux4q): re-read true; the session measured that for hook-local use_signal state links 3 and 4 (written state, its readers) are not exposed by public API, Scope.hooks being pub(crate) in dioxus-core scope_context.rs:53.
@task_item n=12 part=touches
The Dioxus integration decisions (dec-dioxus-integration): the founder's answers set how The UI graph will read a node's component, listeners and state readers — the vdom walk, an escher-side signal registry, a snapshot field for the click reading.
@task_item n=13 part=touches
The dependency audit leg (CI contracts and the cargo-deny gate): a CI Dependency audit went red on advisories published after the base's green audit with Cargo.lock and deny.toml unchanged; the local fast leg runs no audit, ci-leg.sh audit is the local reproduction.
@task_fact n=1 slug=established-decisions
fact: dioxus_runtime_probe pins the MountedData hazard as two should_panic tests, one per layout mode, mounted_data_in_a_task_a_key_handler_spawned_panics_non_incremental and _incremental, while mounted_data_asked_at_once_in_a_key_handler_acts shows the same set_focus called synchronously from a key handler moves focus with no panic; the port chunk is to invert the pin.
@task_fact n=2 slug=established-decisions
fact: Dioxus 0.7.10 public API reads a node's owning component with no fork-level change — VirtualDom::base_scope, ScopeState::try_root_node, VNode.template.roots, VNode.dynamic_nodes, VNode::mounted_root, VComponent::mounted_scope and VComponent.name — a deleted CRUD row resolving to no component.
@task_fact n=3 slug=standard-contracts
fact: The vdom walk's dynamic attributes (VNode.dynamic_attrs, VNode::mounted_dynamic_attribute) read an element's current listeners exactly, mounted included, where the bridge's listener attributes (an attribute named the event valued rust func beside the data-dioxus-id mark) keep a removed listener and omit mounted.
@task_fact n=4 slug=standard-contracts
fact: dioxus-core 0.7.10 reaches remove_event_listener only when a listener attribute leaves an element's attribute list (a spread); a listener whose value turns to AttributeValue::None under the same name is written through set_attribute as an on{event} attribute, so the bridge's event attribute stays either way (diff/node.rs:416-536).
@task_fact n=5 slug=established-decisions
fact: For a GlobalSignal, Readable::subscribers() visited inside VirtualDom::in_runtime give through ReactiveContext::origin_scope exactly its reader components' scopes, and a writer's click changes exactly the readers' nodes in the snapshot diff; for hook-local use_signal state neither the readers nor the written state are reachable through public API.
@task_fact n=6 slug=project-intent
fact: The MountedData fix is a port of upstream 286874cc1's buffered render onto the Dioxus 0.7 bridge in a chunk of its own (founder's word): a buffer of its own recording 0.7's WriteMutations calls, replayed into the existing MutationWriter so mutation_writer.rs stays byte-identical to upstream at 434e0661, flush_queued_mounted_events unchanged and its two callers (dioxus_document.rs:167, :300) changing shape — an engine change for every Dioxus document.
@task_fact n=7 slug=established-decisions
fact: The founder's answers for The UI graph: links 3 and 4 for hook-local state through an escher-side signal registry, link 2 read by the vdom walk's dynamic attributes, and the reacts-to-a-click reading becoming a snapshot field with the graph.
@task_fact n=8 slug=existing-scopes
fact: blitz-tests holds the dioxus_runtime_probe target, 10 tests over the stand's Counter, CRUD and Cells and its own pages (toggled_listener, shared_state, hazard), reading public API only and carrying no print, env read, subscriber or listener.
@task_item n=1 part=untouched
Dioxus versions and pins: the requirement stays 0.7.3 (dioxus-core and dioxus-signals 0.7.10 in the lock); no Cargo.toml, so Inherited Defaults' Cargo.toml citations and dependency pinning are unmoved.
@task_item n=2 part=untouched
Snapshot and driver tables: NODE_FIELDS thirteen, VERBS twelve, CAUSES eight and SessionError fifteen are unmoved; no snapshot field was added, the click field still deferred.
@task_item n=3 part=untouched
Synchronous MountedData uses — scroll_to calls and NodeHandle backing in Standard Contracts (fct-qmowki7i, fct-kllkxz3d, fct-2ylcpw3b): true claims sharing the token, untouched.
@task_item n=4 part=untouched
The data-dioxus-id mark's parse and use as an id, and element_id or element_ids' four-tier stable id (fct-srb7slak): unchanged; only the mark's reading as listener evidence is touched.
@task_item n=5 part=untouched
CI shape: no job, leg, script, status shape or leg list moved; the thirteen linux jobs, stand-repeat, the a11y leg and cache rules stand as written.
@task_item n=6 part=untouched
Engine crates under packages/ (blitz-dom, blitz-paint, blitz-shell, dioxus-native-dom source): no code changed; every packages/ line citation is unmoved.
@task_item n=7 part=untouched
rdme's two renderer backends and its feature selection (fct-ljblw6mf): true, only the pulldown-cmark resolution in Stack moves.
@task_item n=8 part=untouched
downcast_doc_mut and other borrow sites sharing the coordinate tokens (security-plan's fct-6rk5sgno class): a different symbol, not re-measured.
@task_item n=9 part=untouched
Occupied Resources: no port, listener, env var, filesystem path, thread or name was added; the new target runs in process and binds nothing.
@task_item n=10 part=untouched
Telemetry and logging patterns: the new file installs no subscriber and prints nothing (silence census 0).
@task_item n=11 part=untouched
The headless stand contract (seven_guis::stand): its boots and pages are read by the probe, not changed; the 677 listener-marked elements on Cells remain the count stand_non_lean_tasks pins.
