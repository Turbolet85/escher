@task_prose part=chunk
Builds one read-only driver read of the current screen's actionable nodes (capability v030-06): for each node its stable id, its owner where its handler sits on another element, its verbs, and for each verb whether a call would be accepted now or the one refusal cause it would return (disabled, off-screen, covered).
The node model is the UI graph's GraphNode with an optional owner; the verbs are read from the schema's verb table VERBS, with no closed list of verb names in the code, which may need the verb table to state more (a contract change, the founder's fork).
The read and the executor decide a target's refusal through one shared path (today action_point in execute.rs), so the listed reason agrees with a real call; like snapshot, lint and graph it runs no step and changes nothing.
The actionable-key check (residual L12) may fold in, each node saying whether its id is an author key or a positional path; proven on the stand in both layout modes, with a minimal fixture where the stand lacks a covered, off-screen or owner-served case.
@task_file n=1 path=packages/escher-driver/src/schema.rs mark=edits
what: the verb table VERBS and its VerbSpec rows
@task_file n=2 path=packages/escher-driver/src/schema/spec.rs mark=edits
what: VerbSpec, which may state which verbs act on a target, their refusals and the elements they apply to
@task_file n=3 path=packages/escher-driver/src/execute.rs mark=edits
what: action_point, the shared refusal path (disabled, then off-screen, then covered)
@task_file n=4 path=packages/escher-driver/src/json.rs mark=edits
what: the written form of the read's answer
@task_file n=5 path=packages/escher-driver/src/cli.rs mark=edits
what: the command line's command for the read
@task_file n=6 path=packages/escher-driver/src/mcp/listing.rs mark=edits
what: the MCP tool listing
@task_file n=7 path=packages/escher-driver/src/mcp/result.rs mark=edits
what: the MCP result of the read
@task_file n=8 path=packages/escher-driver/src/recording.rs mark=edits
what: recording the read as a step of a kept check
@task_file n=9 path=packages/escher-driver/src/replay.rs mark=edits
what: replaying the read's step of a kept check
@task_file n=10 path=packages/escher-driver/src/describe.rs mark=edits
what: the self-description of the verbs
@task_file n=11 path=docs/capabilities.md mark=edits
what: the verb and cause names and their counts in words
@task_file n=12 path=.github/scripts/test_capability_statement.py mark=re-reads
what: the capability statement's pin of docs/capabilities.md and README.md to the driver schema's tables
@task_file n=13 path=packages/dioxus-native-dom/src/graph.rs
@task_file n=14 path=packages/dioxus-native-dom/src/snapshot.rs
@task_file n=15 path=packages/dioxus-native-dom/src/actionable.rs
@task_item n=1 part=needs
The escher-driver verb schema and its unit tests (verb schema, command validation, JSON forms, command line, MCP, recording, replay), which a new verb or a widened VerbSpec must keep green and extend.
@task_item n=2 part=needs
The executor's refusals (disabled, off-screen, covered) and the integration checks that read them through the driver, against which the read's per-verb reasons must agree with the refusal an actual call returns.
@task_item n=3 part=needs
The UI graph reader (DioxusDocument::graph, GraphNode, its handlers) and the stand_graph check, the node model the read uses, including an owner-served node such as a select's options and groups.
@task_item n=4 part=needs
The snapshot builder's clickable reading and the stable element id, author key versus positional path, which the read's ids and actionable set draw on.
@task_item n=5 part=needs
The actionable-key check unkeyed_actionable and its stand check stand_actionable_keys, which the read may answer for in a listed form.
@task_item n=6 part=needs
The session_common module's call builders and readers (graph, lint, graphed, linted, refused, act), where a read builder and its answer reader would join, and its readers count.
@task_item n=7 part=needs
The shared fixtures module tests/blitz-tests/tests/fixtures/mod.rs and its covered page, where a covered, off-screen or owner-served fixture for the read would go, and its count of declaring checks.
@task_item n=8 part=needs
The seven_guis headless stand boots in both layout modes (stand::boot, boot_cells, boot_load, stand::options) on which the read is proven.
@task_item n=9 part=needs
The stand log format: a new verb joins the closed sets of the command span's verb and the tool_call span's tool, its command line recording verb and, refused, cause only, with no field for the read's content or size, read by host_log and mcp_log at trace with 0 id, name and graph needles under both builds.
@task_item n=10 part=needs
The MCP surface's thirteen verbs as counted by stand_mcp_tools and the cold-agent pipe's isolated tool set and valid_step verb list, which a new verb on the MCP carrier would move.
@task_item n=11 part=needs
The command-line checks cli_commands and the recorded-check replay stand_replay and the committed checks under tests/blitz-tests/checks, where the read on the command line and in a kept check is exercised.
@task_item n=12 part=needs
The escher-driver per-crate coverage floor of 97 percent and the dioxus-native-dom floor of 83 percent, which the new code must stay inside.
@task_item n=13 part=needs
The agent-run proof count: run stand's run.end, passed 244 over 52 files at the last reading, moved by any new stand_ file or test.
@task_item n=14 part=needs
The stand-repeat leg's zero-failure flakiness budget over the stand_ files, three rounds with no retry, which a new stand check joins.
@task_item n=15 part=needs
The driver's log line rule: each call run through Session::run leaves exactly one escher_driver line of fixed words and counts only, and the stand_act_spans in-process capture of it.
@task_item n=16 part=needs
The windows and macOS CI legs: a session check holding a socket is cfg(unix)-gated, and a shot is never compared byte for byte across OSes.
@task_standing n=1
convention: a stand check sits in its own tests/blitz-tests/tests/stand_*.rs target, picked up by the stand_ prefix with no script change, declaring mod common, mod session_common or mod fixtures after its use block
home: test-strategy
@task_standing n=2
convention: a fixture page more than one check reads is stated once in tests/fixtures/mod.rs and booted through fixtures::boot with the stand's options; a page with one reader stays in its check's file
home: test-data-fixtures
@task_standing n=3
convention: a session check is proven in both layout modes, incremental and not, with no sleep or wall-clock read and every wait bounded
home: integration-test-strategy
@task_standing n=4
convention: a unit test sits in a #[cfg(test)] module in the source file, or a #[path] test file declared at its parent, its assertions over a call carrying a row index only so a failing log prints no argument value
home: unit-test-strategy
@task_standing n=5
convention: a check that holds a session process starts and stops its own host with bounded waits and a guard that kills and reaps it, its child's streams to null unless read
home: test-strategy
@task_standing n=6
convention: a new span or log line names fixed words and counts only, never an id, name, value or the read's content
home: ci-integration
@task_standing n=7
convention: a new check carries a regression-style name stating the behaviour it guards
home: test-strategy
@task_item n=1 part=not_needed
The WPT conformance runner, its CLI, outputs, scoring and workflows: the chunk changes no rendering and no conformance reading.
@task_item n=2 part=not_needed
The browser app's tests (about pages, history, favicon, url suggestions, persistence, tab): the chunk does not reach apps/browser.
@task_item n=3 part=not_needed
blitz-vibey-script's script tests and the Preact TodoMVC flow: no script engine is touched.
@task_item n=4 part=not_needed
blitz-paint's gradient, box, blend-layer and password-mask tests, and pixel tests against Chromium renders: the read paints nothing.
@task_item n=5 part=not_needed
blitz-dom's style bridge, list-marker, text-transform, clock and font-face tests: no engine change is expected.
@task_item n=6 part=not_needed
The CI Python scripts for WPT diff and area changes, and the apt-install script's bounded package installs.
@task_item n=7 part=not_needed
The mutation tier under cargo-mutants and nextest: it runs on the dev host only and the scope asks no mutation run.
@task_item n=8 part=not_needed
The harness's screenshot and text_contrast readers and the visual lint's contrast: the read takes no shot and reads no colour.
@task_item n=9 part=not_needed
Drag, virtual-node production, canvas and two-target verbs: they stay in 0.4.0.
@task_item n=10 part=not_needed
Handler coverage (v030-07) and auto-exploration (v030-09): each is its own entry.
@task_item n=11 part=not_needed
The benchmarks paint_tree_bench and paint_bench, run ignored or by hand.
@task_item n=12 part=not_needed
The accessibility platform adapter's states as handed to an assistive technology, owed under residual res-llkgqygv.
