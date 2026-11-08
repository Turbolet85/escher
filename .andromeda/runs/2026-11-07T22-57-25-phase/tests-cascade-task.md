@task_prose part=chunk
Builds the UI graph (v030-05): one read that gives, for the current screen of a held instance, each node, the component that rendered it, its handlers, the state they write and the nodes that read that state.
Link 1 by the dioxus-core vdom walk the stable ids run; link 2 by the walk's dynamic attributes, `mounted` included; links 3 and 4 through a new escher-side signal registry in dioxus-native-dom, the seven_guis stand's signals moved onto it.
Adds the closed boolean «reacts to a click» snapshot field, absorbing res-gvelrxja: NODE_FIELDS moves past thirteen and the stand_non_lean_tasks starting reading moves.
Values the graph carries read masked as on every carrier, and no log site, span field or print carries a node, component, handler, state or value.
Reserves a virtual-node id form that collides with none of the four id forms, keys the graph on the agent-visible node with an optional owner element, and lets an owner's handler serve sub-targets, producing no virtual node.
@task_file n=1 path=packages/dioxus-native-dom/src/element_id.rs
@task_file n=2 path=tests/blitz-tests/tests/stand_non_lean_tasks.rs mark=edits
what: the pinned starting reading of the «reacts to a click» field (0 on the four lean tasks, Home and Temperature Converter, Circle Drawer 1, Cells 676, both layout modes)
@task_file n=3 path=tests/blitz-tests/tests/dioxus_runtime_probe.rs mark=re-reads
what: the probe's measured link-1, link-2 and subscriber mechanisms the graph reader is built on
@task_item n=1 part=needs
dioxus-native-dom unit tests of the snapshot builder, its text form, its parts and its diff, and of the stable element id: the new graph reader, the signal registry and the new snapshot field each get unit tests there.
@task_item n=2 part=needs
The NODE_FIELDS count (thirteen today), the schema's help texts, describe() and every pin that counts the snapshot's node fields move together with the new boolean field, in the snapshot and diff carriers.
@task_item n=3 part=needs
The element_id unit-test pattern: build a DioxusDocument from a plain fn app, re-render by flipping a signal inside vdom.in_runtime, and resolve(0.0) through inner_mut() before reading a snapshot; the default config's no-op HTML parser means parsed markup is pinned in blitz-tests.
@task_item n=4 part=needs
The bridge tests' rule: no function under test gains a constructor, a pub or a cfg(test) shim for a test's sake; private module state is reached by a cfg(test) child module.
@task_item n=5 part=needs
The seven_guis headless stand (stand::boot, boot_cells, boot_load, boot_timer with TimerTicks, stand::options): the stand's signals move onto the registry, and every stand check over those tasks must stay green in both layout modes.
@task_item n=6 part=needs
The shared stand tables and helpers in tests/blitz-tests/tests/common/mod.rs, session_common/mod.rs and fixtures/mod.rs: stated expectation, never a second mapping; a fixture page more than one check reads joins fixtures/mod.rs.
@task_item n=7 part=needs
escher-driver's verb schema, JSON forms, MCP tools, recording and replay unit tests, and the twelve-verb count wherever it is pinned (VERBS, the MCP tool set, the cold-agent isolation check, test_capability_statement over docs/capabilities.md and README.md), should the read cross as a new verb or outcome.
@task_item n=8 part=needs
escher-driver depends on nothing that names a virtual DOM, so its unit tests cannot build a held instance: how the read is asked and answered is checked by integration checks over a Session booted through the stand.
@task_item n=9 part=needs
Session ↔ held instance: a call through Session::run returns after settle with the diff; a refused call leaves the instance unchanged and echoes none of the bytes it supplied; a password never leaves in an answer.
@task_item n=10 part=needs
Masking: a state or control value the graph carries reads MASKED_VALUE for a password or file input, as on every carrier, read by the stand checks that hold a secret (stand_mcp_masked and the per-file SECRET checks).
@task_item n=11 part=needs
Log scrub: escher-telemetry's target allowlist and redaction; host_log, mcp_log and stand_act_spans read 0 id, name and typed-text needles at trace under both builds; the driver's one escher_driver line per call holds fixed words and counts only; a new verb is a word of the command span's verb and tool_call's tool closed sets, with no span field added.
@task_item n=12 part=needs
Engine features by runner: workspace and -p blitz-tests builds differ (tracing, writing-mode), so a check reading a sink's capture counts lines by target, never in total.
@task_item n=13 part=needs
Recorded-check replay: committed checks under tests/blitz-tests/checks/ stay byte-equal to what escher-session records; a new recorded step form must keep them fresh.
@task_item n=14 part=needs
Coverage floors in the coverage leg: packages/dioxus-native-dom 83 %, packages/escher-driver 97 %, packages/blitz-test-harness 90 %, keyboard_compat.rs excluded; the new reader and registry code count toward them.
@task_item n=15 part=needs
The stand-repeat leg's zero-failure budget: the stand_*.rs files, the seventeen named blitz-tests targets and seven_guis' process checks run three rounds with no retry; a new non-stand_ target that must repeat joins that named list.
@task_item n=16 part=needs
The agent-run proof counts: run stand (237 · 0 · 7 over 51 files) and run all (743 · 0 · 12) are re-counted with the chunk's new tests, a new stand_ file picked up by its prefix with no script change.
@task_item n=17 part=needs
The headless-stand font rule: the stand checks assert unconditionally under the bundled DejaVu Sans with system fonts off, with no skip path.
@task_standing n=1
convention: unit tests sit in #[cfg(test)] mod tests in the source file, or in a #[path] file declared #[cfg(test)] at its parent, their test paths unchanged
home: §4 Unit Test Strategy (unit-test-strategy)
@task_standing n=2
convention: blitz-tests holds one test-target file per behaviour under tests/blitz-tests/tests/, no [[test]] table added (BlitzTestsTargetsTest), a stand check named with the stand_ prefix so run stand selects it
home: §2 Test Strategy → Directory pattern (test-strategy)
@task_standing n=3
convention: a regression test is named for the bug it guards and documents it
home: §2 Test Strategy → Test function naming (test-strategy)
@task_standing n=4
convention: a stand or session check runs in both layout modes, incremental and not
home: §5 Integration Test Strategy (integration-test-strategy)
@task_standing n=5
convention: no sleep and no wall-clock read in a session check unless wall time is its subject
home: §5 Integration Test Strategy → Wall time in the session checks (integration-test-strategy)
@task_standing n=6
convention: a check declares mod common; / mod session_common; / mod fixtures; after its use block and reads the shared tables and helpers rather than restating them
home: §3 Test Harness Contract → Crate-local test helpers
@task_standing n=7
convention: a check that needs a session process bounds every wait, kills and reaps its host by guard, removes its state directory, nulls the child's streams unless read, and is cfg(unix)-gated when it holds the socket
home: §2 Test Strategy → Process-lifecycle checks (test-strategy)
@task_standing n=8
convention: an assertion over a driver call carries a row index only, so a failing test's log prints no argument value
home: §4 Unit Test Strategy → escher-driver (unit-test-strategy)
@task_item n=1 part=not_needed
apps/browser's tests (about pages, history, favicon, URL suggestions, persistence, tab): the chunk touches no browser crate.
@task_item n=2 part=not_needed
WPT: the runner, its CLI, statuses, scoring, reftests, the Thai font, the wpt and post-results workflows and the WPT diff Python scripts.
@task_item n=3 part=not_needed
blitz-vibey-script's script DOM, Selection, URLSearchParams and Preact TodoMVC tests.
@task_item n=4 part=not_needed
blitz-paint's pixel tests, blend-layer gaps and gradient or css_box unit tests, and the harness's screenshot and text_contrast readers.
@task_item n=5 part=not_needed
blitz-dom engine tests of layout, style bridge, list markers, text-transform, select model and list, range rules, scrollbars and font-face loading: no blitz-dom engine change is expected.
@task_item n=6 part=not_needed
blitz-net's load-delivery tests and the HeldNet, LateNet and LoadNet network stand-ins' behaviour.
@task_item n=7 part=not_needed
The differential incremental oracle, the randomized template-swap stress test and the pure-restyle hover paths.
@task_item n=8 part=not_needed
The CI package-install bound (apt-install.sh), the cache configuration, failure-log uploads and the native ios/android build-only matrix description.
@task_item n=9 part=not_needed
The mutation tier (cargo-mutants under nextest), a dev-host run never in CI, and the paint_tree_bench ignored benchmarks.
@task_item n=10 part=not_needed
The cold-agent live run on the operator host and its cost, model and verdict reading.
@task_item n=11 part=not_needed
VirtualClock and WallClock unit tests and the Timer's time step: the chunk moves no clock.
@task_item n=12 part=not_needed
Example HTML fixtures under examples/assets and examples/preact, and the manual inspection tools (screenshot example, paint_bench).
@task_item n=13 part=not_needed
«What can I do here» (v030-06), «Handler coverage» (v030-07), virtual-node production, drag and a canvas renderer: out of scope.
