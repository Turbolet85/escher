@task_prose part=session
The chunk 2026-11-04-dioxus-runtime-probe added one blitz-tests target, dioxus_runtime_probe (10 tests, 8 looping both layout modes, two should_panic hazard pins), reading Dioxus 0.7.10 public API only for link 1 (component), link 2 (handlers), links 3 and 4 (written state and its readers); nothing under packages/ or examples/ and no Cargo.toml moved.
It measured the MountedData hazard at HEAD (a task spawned from a key handler that wrote a signal panics RefCell already borrowed in set_focus) and pinned it; the fix is routed to a new chunk, MountedData buffered render, porting upstream 286874cc1, which inverts the pin.
It measured that the bridge's listener attributes over-report (a removed listener's attribute and data-dioxus-id mark stay; a mounted listener writes none) while the vdom walk's dynamic attributes read current listeners exactly.
Counts moved: blitz-tests 158 .rs files and 157 test targets (157 and 156 before); run all passed 743 · failed 0 · ignored 12 (733 before); run stand unmoved at 237 over 51 files.
An operator fix commit 9b3b29714 bumped pulldown-cmark 0.13.5 to 0.13.6 in Cargo.lock (RUSTSEC-2026-0433) and added RUSTSEC-2026-0429 (zip 2.4.2) to the deny.toml ignore list, now eight IDs; CI#41914044942 green 17/17.
Nothing was retired; no harness, script, CI step or leg list changed.
@task_file n=1 path=tests/blitz-tests/tests/dioxus_runtime_probe.rs
@task_file n=2 path=Cargo.lock
@task_file n=3 path=deny.toml
@task_item n=1 part=touches
The blitz-tests target count in test-strategy (fct-7m7lxsmy, 157 .rs files and 156 test targets) moves to 158 .rs files directly under tests/blitz-tests/tests/ and 157 test targets, dioxus_runtime_probe the latest.
@task_item n=2 part=touches
The §3 Proof, the newest reading (fct-glytb3w3): run all's run.end now reads passed 743 · failed 0 · ignored 12, the new target dioxus_runtime_probe (no stand_ prefix) picked up by run all with no script change, as measured at escher-0.3.0/chunks/2026-11-04-dioxus-runtime-probe; run stand stays at passed 237 · failed 0 · ignored 7 over 51 files.
@task_item n=3 part=touches
The MountedData record in e2e-test-strategy (fct-kgita7a3) is re-measured at HEAD: DioxusDocument::poll holds inner.borrow_mut() at packages/dioxus-native-dom/src/dioxus_document.rs:295 across vdom.render_immediate at :297; dioxus-core 0.7.10 polls a queued task there via handle_task_wakeup (virtual_dom.rs:611); set_focus (events.rs:295) borrows through doc_mut (events.rs:162) and panics; now pinned by dioxus_runtime_probe in both layout modes, its fix routed to the port chunk MountedData buffered render on the founder's word.
@task_item n=4 part=touches
test-scope-summary's naming of upstream test task_polled_during_render_can_access_document as left out (fct-e2zfwpza) changes: it is now the port chunk's witness, the test that inverts dioxus_runtime_probe's hazard pin.
@task_item n=5 part=touches
The tests/blitz-tests coverage entry of test-scope-summary (one target per behavior, coverage spanning engine, stand, driver and telemetry) gains a Dioxus runtime probe target reading the vdom walk, listeners, signal subscribers and the MountedData hazard.
@task_item n=6 part=touches
The dioxus-native-dom actionable-key check named in test-scope-summary: its listener mark (data-dioxus-id) still holds an element whose listener was removed, so the check over-includes (conservative) — read from the code, not run.
@task_item n=7 part=touches
The §5 boundary Dioxus VirtualDom ↔ DioxusDocument (only keyed_nodes_do_not_crash today) now also has a blitz-tests check driving a real VirtualDom through Harness::from_vdom and reading its public walk (VirtualDom::base_scope, ScopeState::try_root_node, VNode roots, dynamic nodes and attributes, VComponent::mounted_scope).
@task_item n=8 part=touches
The Harness contract readers in §3 (from_vdom, press, click, pump, focused) gain one more caller, dioxus_runtime_probe; the contract itself is unchanged.
@task_item n=9 part=touches
The §3 agent-run run selection: dioxus_runtime_probe is reached by run all and run dioxus_runtime_probe (exit 0), not by run stand.
@task_item n=10 part=touches
The «reacts to a click» reading is measured by link 2 (676 Cells cells reacting to click, cells-grid to keydown, of the 677 that stand_non_lean_tasks pins; counter-increment click, counter-value none; crud-list keydown) with no snapshot field added, NODE_FIELDS still thirteen, the field still deferred.
@task_item n=11 part=touches
Mentions of upstream 286874cc1 as left out with the 0.8 move in quality-gates-coverage-targets (fct-fx4qekph) and test-scope-summary (fct-ybmlkzsq) are re-read and still true; the port's planned shape is new text.
@task_item n=12 part=touches
The CI Dependency audit and the ci-leg.sh audit leg: CI#41911351081 on 53ae48a65 went red on two new advisories with Cargo.lock and deny.toml unchanged; after the fix, ci-leg.sh audit reads advisories ok, licenses ok locally and in CI#41914044942, green 17/17, wall 1531 s.
@task_item n=13 part=touches
The pulldown-cmark resolution moves 0.13.5 to 0.13.6 (reached only through rdme), and the zip 2.4.2 ignores count four (RUSTSEC-2026-0367 · 0368 · 0369 · 0429); any test-plan line citing the deny.toml ignore list or those versions moves (the list now deny.toml:20-30, eight IDs).
@task_item n=14 part=touches
The workspace test leg (cargo test --workspace --locked) and the per-OS test matrix each run one more blitz-tests target.
@task_fact n=1 slug=test-scope-summary
fact: tests/blitz-tests/tests/dioxus_runtime_probe.rs (10 tests, 8 looping for incremental in [false, true], two should_panic hazard pins, one per mode) reads Dioxus 0.7.10 public API only to measure link 1 (the component that rendered each node, a deleted CRUD row resolving to nothing), link 2 (each node's handlers), link 3 (a handler's effect as its step's snapshot diff) and link 4 (a held signal's readers), with no fork-level change.
@task_fact n=2 slug=e2e-test-strategy
fact: The MountedData hazard is pinned by mounted_data_in_a_task_a_key_handler_spawned_panics_non_incremental and _incremental (should_panic), their precondition asserted first by hazard_ready (the handle held, the key target focused by Tab), beside mounted_data_asked_at_once_in_a_key_handler_acts (synchronous set_focus moves focus, no panic); get_scroll_offset and get_scroll_size (shared borrow through doc, events.rs:152) and the windowed path are read, not run.
@task_fact n=3 slug=integration-test-strategy
fact: The bridge's listener attributes (an attribute named the event, valued <rust func>, written by create_event_listener, mutation_writer.rs:331-352) over-report: a listener removed on a re-render (reached only when it leaves the attribute list, a spread) leaves its attribute and the data-dioxus-id mark while a click no longer acts, and a mounted listener writes none; VNode.dynamic_attrs with VNode::mounted_dynamic_attribute read current listeners exactly — pinned by link_2_a_removed_listeners_attribute_stays.
@task_fact n=4 slug=integration-test-strategy
fact: For a GlobalSignal, subscribers() visited in VirtualDom::in_runtime give through origin_scope() exactly its two reader components' scopes and the writer's click changes exactly the readers' nodes in the snapshot diff (links_3_and_4_a_held_signals_subscribers_are_its_readers); for hook-local use_signal state links 3 and 4 are not exposed without a fork-level change (Scope.hooks pub(crate)) — read, not run.
@task_fact n=5 slug=test-data-fixtures
fact: dioxus_runtime_probe keeps its one-reader pages in its own file: toggled_listener (a Vec<Attribute> spread in and out), shared_state (Writer, FirstReader, SecondReader over a GlobalSignal) and hazard (hazard_spawned, hazard_synchronous).
@task_fact n=6 slug=test-scope-summary
fact: The planned port of upstream 286874cc1's buffered render onto the 0.7 bridge writes its own buffer recording 0.7's WriteMutations calls and replays them into the existing MutationWriter, keeping mutation_writer.rs byte-identical to upstream at 434e0661, flush_queued_mounted_events unchanged and its two callers (dioxus_document.rs:167, :300) changing shape — read, not run.
@task_fact n=7 slug=ci-integration
fact: A CI Dependency audit can go red on advisories published after the base's green audit with Cargo.lock and deny.toml unchanged (CI#41911351081 against CI#41346357110), and the local fast leg runs no audit; bash .github/scripts/ci-leg.sh audit is the local reproduction.
@task_item n=1 part=untouched
§9 stand-repeat leg, its seventeen named targets and its figures (fct-qjdo4c7k), and the a11y leg: the new target has no stand_ prefix and is named in neither.
@task_item n=2 part=untouched
The agent-run contract's invocation, exit grammar, boot, status, cleanup, logs, events and state: scripts/agent-run.sh is unchanged.
@task_item n=3 part=untouched
The run stand Proof history and the stand reader counts of the shared common, session_common and fixtures modules: run stand is unmoved at 237 over 51 files.
@task_item n=4 part=untouched
§10 coverage floors, flakiness budget and mutation tier: no floor, profile or leg changed and no coverage was re-read.
@task_item n=5 part=untouched
§8 Mocking and stubbing: no stand-in added; HeldNet, LateNet, LoadNet and TimerTicks unchanged.
@task_item n=6 part=untouched
The cold-agent run pipe, its verdict, events, state and proof: not touched.
@task_item n=7 part=untouched
blitz-test-harness's API, settle, screenshot, contrast and their unit tests: no function added or changed.
@task_item n=8 part=untouched
dioxus-native-dom's unit, bridge, element_id and snapshot tests and its crate-local helpers: nothing under packages/ changed.
@task_item n=9 part=untouched
escher-driver's unit tests, verbs (twelve), session errors and the stand log format and telemetry checks: no driver or telemetry change.
@task_item n=10 part=untouched
WPT runner, workflows, scoring and fixtures, and the browser app's tests: not touched.
@task_item n=11 part=untouched
Third-party test frameworks: no crate or dev-dependency added; test-that stays 0.6.0.
