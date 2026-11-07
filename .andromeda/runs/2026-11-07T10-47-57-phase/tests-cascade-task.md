@task_prose part=chunk
Ports upstream 286874cc1's buffered render onto the Dioxus 0.7 bridge in dioxus-native-dom: DioxusDocument's two render sites (initial_build's vdom.rebuild, poll's vdom.render_immediate) render into a buffer of 0.7 WriteMutations calls with the document unborrowed, then apply it once through the existing MutationWriter and flush the queued mounted events as before.
Adds a new buffer module (mutation_buffer.rs) and ports upstream's unit test task_polled_during_render_can_access_document into dioxus_document.rs's mod tests as the witness that a task polled during render can call MountedData (set_focus, get_client_rect) without a RefCell already borrowed panic.
Inverts the two should_panic hazard pins of dioxus_runtime_probe into tests that the key-handler-spawned task's set_focus moves focus and does not panic, in both layout modes, their precondition (handle held, key target focused by Tab) still asserted first.
An engine change for every Dioxus document; Dioxus stays at 0.7, no new log line, field or boundary crossing; behaviour otherwise unchanged, re-asserted by the existing stand_* checks, stale_node_mapping, incremental_oracle and dioxus_runtime_probe's other eight tests.
@task_file n=1 path=packages/dioxus-native-dom/src/dioxus_document.rs mark=edits
what: the two render sites initial_build and poll; the ported test task_polled_during_render_can_access_document in mod tests
@task_file n=2 path=packages/dioxus-native-dom/src/mutation_buffer.rs
@task_file n=3 path=packages/dioxus-native-dom/src/lib.rs mark=edits
what: the module list
@task_file n=4 path=packages/dioxus-native-dom/src/mutation_writer.rs mark=edits
what: DioxusState, if the buffer lives on it as upstream's does; otherwise kept byte-identical to upstream at 434e0661
@task_file n=5 path=tests/blitz-tests/tests/dioxus_runtime_probe.rs mark=edits
what: the two should_panic RefCell already borrowed pins (one per layout mode) inverted; the module text re-said
@task_file n=6 path=tests/blitz-tests/tests/stale_node_mapping.rs mark=re-reads
what: stale_node_mapping, the randomized template-swap stress over slab and ElementId reuse, as witness that element ids are unchanged
@task_file n=7 path=tests/blitz-tests/tests/incremental_oracle.rs mark=re-reads
what: incremental_oracle, the differential incremental/non-incremental layout identity
@task_item n=1 part=needs
The dioxus-native-dom unit tests in the scope summary: the document's events, the mutation writer, the bridge, the stable element id and the snapshot, all of which build a DioxusDocument whose render path this chunk changes and must stay green.
@task_item n=2 part=needs
The dioxus-native-dom crate-local test helper pattern: a DioxusDocument built with DocumentConfig::default(), initial_build, then mark_dirty and poll(None), the dioxus crate as dev-dependency; the ported unit test builds its document this way, and that config carries the no-op HTML parser.
@task_item n=3 part=needs
The bridge_tests child module of dioxus_document (file dioxus_document_tests.rs), which reaches doc.vdom and doc.vdom_state directly; a buffer living on vdom_state or DioxusState changes what those tests construct over.
@task_item n=4 part=needs
The Dioxus VirtualDom to DioxusDocument integration boundary, keyed_nodes_do_not_crash (100 inserts, main holding 100 children), a regression test over the same render path.
@task_item n=5 part=needs
The element_id unit tests that a re-render-removed node no longer resolves and stands in no pair of element_ids(): the buffered replay must keep removal and mapping clearing identical.
@task_item n=6 part=needs
The blitz-test-harness Dioxus constructors from_component and from_vdom and pump, which polls the document with no waker context: the inverted probe pins and every stand check reach the buffered poll through them.
@task_item n=7 part=needs
The harness input helpers press (Tab and the handler's key) and the inspection helper focused, which the inverted pins use to assert their precondition and that focus moved.
@task_item n=8 part=needs
The harness settle loop and its quiet-pass rule, which keeps poll's answer: the buffered poll must return the same answer so settle's Render, Loads and Layout readings do not move.
@task_item n=9 part=needs
The differential oracle incremental_oracle and the both-layout-modes rule: the inverted pins run once per layout mode and the buffer must leave incremental and non-incremental layout identical.
@task_item n=10 part=needs
The randomized stress stale_node_mapping (deterministic LCG, 1000 steps), the witness that buffered replay keeps slab and ElementId reuse correct.
@task_item n=11 part=needs
The headless stand checks (stand_*) and the session over the stand, which boot DioxusDocument through seven_guis::stand: their snapshots, diffs, element ids and mounted-event order are the witness of unchanged behaviour.
@task_item n=12 part=needs
The stand-repeat leg and its zero-failure flakiness budget: the stand checks and the seventeen named blitz-tests targets run three times with no retry over the new render path.
@task_item n=13 part=needs
The coverage floor for packages/dioxus-native-dom at 83 percent line coverage, read by coverage_floor.py, with keyboard_compat.rs excluded: the new buffer module counts toward it.
@task_item n=14 part=needs
The agent-run proof readings: run all's newest reading, 743 passed, 0 failed, 12 ignored, with dioxus_runtime_probe picked up by run all with no script change; the inverted pins change their outcome, not the count, unless P4 measures otherwise.
@task_item n=15 part=needs
The workspace test leg (cargo test --workspace --locked) and the windows and macos matrix test legs, which run the ported unit test and the inverted pins; the engine feature difference between the workspace and the per-package build (tracing, writing-mode) applies.
@task_item n=16 part=needs
The local pre-push gate, ci-leg.sh fast (fmt, clippy, test, ci-scripts, embolden), which the change must pass.
@task_item n=17 part=needs
The stand log format and the driver's one log line per call: the buffer holds ids, tag and attribute names and text in process only, so no new log line or field may appear and the line counts stay as measured.
@task_item n=18 part=needs
The wall-time rule for checks: no sleep and no wall-clock read in a check unless its subject is wall time; the ported task test and the inverted pins drive time through the vdom and harness only.
@task_standing n=1
convention: a new unit test sits in a #[cfg(test)] mod tests block inside the source file under test, or in a #[path] child file declared #[cfg(test)] at its parent with its test path unchanged
home: §4 Unit Test Strategy, Conventions (unit-test-strategy)
@task_standing n=2
convention: a regression test is named for the bug it guards and documents the bug it guards
home: §2 Test Strategy, Test directory and naming conventions (test-strategy)
@task_standing n=3
convention: a dioxus-native-dom unit test builds its DioxusDocument from DocumentConfig::default() with initial_build, drives updates with mark_dirty and poll(None), and gains no constructor, pub or cfg(test) shim on the code under test
home: §3 Test Harness Contract, Crate-local test helpers
@task_standing n=4
convention: a check over a Dioxus document runs in both layout modes, incremental and non-incremental
home: §5 Integration Test Strategy (integration-test-strategy)
@task_standing n=5
convention: a check holds no sleep and no wall-clock read unless wall time is its subject
home: §5 Integration Test Strategy, Wall time in the session checks
@task_standing n=6
convention: a new source module counts toward its crate's line coverage floor in the coverage leg
home: §10 Quality Gates and Coverage Targets (quality-gates-coverage-targets)
@task_item n=1 part=not_needed
The WPT conformance runner, its CLI, outputs, scoring, reftests and the wpt workflows: they run on upstream only and over HtmlDocument, not the Dioxus bridge.
@task_item n=2 part=not_needed
The cold-agent run pipe, its verdict, events and contract tests test_cold_agent.py: no script or verdict field changes and its live run is no CI check.
@task_item n=3 part=not_needed
The agent-run script's internals (exit grammar, verbs, events, state directory) and test_agent_run.py: scripts/agent-run.sh is not touched.
@task_item n=4 part=not_needed
The apps/browser tests (about pages, history, favicon, URL suggestions, persistence, tab): no browser code is reached.
@task_item n=5 part=not_needed
The blitz-vibey-script script tests, the Preact TodoMVC flow and the script Selection and URLSearchParams tests: ScriptDocument is not a Dioxus document.
@task_item n=6 part=not_needed
The blitz-dom unit tests of the style bridge, layout, list markers, text-transform, select model, range, clock and SVG parsing: the engine's own document is unchanged.
@task_item n=7 part=not_needed
The pixel tests, the harness screenshot and text_contrast readers and the password mask on frames: no paint path changes.
@task_item n=8 part=not_needed
The CI Python script tests (test_ci_workflows, test_wpt_diff_to_pr, test_wpt_area_changes, test_capability_statement) and BlitzTestsTargetsTest: no workflow, manifest, capability text or README count changes.
@task_item n=9 part=not_needed
The CI package-install bound and apt-install.sh, the cache keys and the failure-log uploads: no CI job is added or changed.
@task_item n=10 part=not_needed
The process-lifecycle and session-host socket conventions (bounded waits, the Host guard, state directories, cfg(unix) gating) and the command-line shell flows: the chunk adds no process check.
@task_item n=11 part=not_needed
The shared fixtures module tests/blitz-tests/tests/fixtures/mod.rs and the committed recorded checks under tests/blitz-tests/checks: no fixture page or recorded check is added.
@task_item n=12 part=not_needed
The network stand-ins HeldNet, LateNet and LoadNet and the TimerTicks time stand-in: the chunk adds no stand-in and changes no load.
@task_item n=13 part=not_needed
The escher-driver and escher-telemetry unit tests (wire grammar, verbs, MCP, recording, scrub decision): neither crate is edited.
@task_item n=14 part=not_needed
The mutation tier (cargo-mutants under the nextest mutants profile): a dev-host run, never a CI gate, that the scope does not call for.
