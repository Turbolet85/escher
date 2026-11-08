@task_prose part=chunk
Builds one handler-coverage command (v030-07): after a set of recorded kept checks has run on the stand through Check::from_bytes and Check::replay, it lists exactly the UI graph's handlers (graph node id and handler event name, as DioxusDocument::graph reads them) that none of those checks exercised.
Adds a record of which (element, event) handlers a run reached that holds on every app, registry or not, read beside the UI graph without changing its shape.
States the canvas limit in the acceptance: only DOM listeners on agent-visible nodes are counted, one per (node, event); handlers a canvas dispatches inside one listener are not, the count keyed on the graph's node id with its optional owner so 0.4.0 can extend it to virtual nodes.
Writes the owed replay test (CARRY cry-sxrivdk3): a session holding an actions step is recorded, kept, read back and replayed on a fresh session of the same app, the replay holding, and a differing actions answer naming the step and the differing key; v030-06 reads verified only once it passes.
@task_file n=1 path=packages/dioxus-native-dom/src/dioxus_document.rs
@task_file n=2 path=packages/dioxus-native-dom/src/signal_registry.rs
@task_file n=3 path=packages/dioxus-native-dom/src/graph.rs
@task_file n=4 path=packages/escher-driver/src/replay.rs
@task_file n=5 path=packages/escher-driver/src/recording.rs
@task_file n=6 path=packages/escher-driver/src/recording_tests.rs
@task_file n=7 path=packages/escher-driver/src/schema.rs
@task_file n=8 path=packages/escher-driver/src/command.rs
@task_file n=9 path=packages/escher-driver/src/execute.rs
@task_file n=10 path=packages/escher-driver/src/describe.rs
@task_file n=11 path=packages/escher-driver/src/json.rs
@task_file n=12 path=packages/escher-driver/src/mcp.rs
@task_file n=13 path=packages/escher-driver/src/mcp/listing.rs
@task_file n=14 path=packages/escher-driver/src/schema_tests.rs
@task_file n=15 path=packages/escher-driver/src/cli_tests.rs
@task_file n=16 path=.github/scripts/test_capability_statement.py
@task_file n=17 path=docs/capabilities.md
@task_file n=18 path=README.md
@task_file n=19 path=tests/blitz-tests/tests/stand_requirement_sweep.rs
@task_item n=1 part=needs
The UI graph read DioxusDocument::graph, its UiGraph nodes, GraphNode.handlers and GraphHandler.event, and a node's optional owner: the coverage command reads them and keeps their shapes.
@task_item n=2 part=needs
The graph-state registry (use_graph_state, DeclaredWrite, SignalRegistry and its dispatches record) and the bridge's handle_event setting the registry's dispatch only where a registry stands: the new exercised record must hold on apps with no registry.
@task_item n=3 part=needs
The Dioxus DOM bridge's event path, DOM events reaching the vdom through data-dioxus-id, where the (element, event) handlers a run reached would be recorded.
@task_item n=4 part=needs
Stable element ids (four tiers and the reserved owner form spelled by virtual_node_id): the coverage list is keyed on the graph's node id with its owner, never a NodeId, keeping the 0.4.0 virtual-node extension open.
@task_item n=5 part=needs
The driver session: Session::run as the one path every call takes, validation first, nothing public running a Command not from validate; any coverage read or replay goes through it.
@task_item n=6 part=needs
Kept checks and the recording: a session's answered calls kept as one owner-only replayable check, Check::from_bytes reading it back, Check::replay running every instance-verb step through Session::run; the owed test measures that a kept actions step replays and holds.
@task_item n=7 part=needs
The actions verb and its answer (actionable_nodes, ActionableNode id, owner, keyed; owner ids, verb words, closed cause words and booleans on the wire): read by the replay test and compared key by key, its shape unchanged.
@task_item n=8 part=needs
The driver's wire boundary: no NodeId crosses, nothing logs an id, a name or a value, every answer written by hand in the crate-private json module; the coverage answer carries only ids and component and event names the graph verb already carries, no control value or typed text.
@task_item n=9 part=needs
The command and refusal schema stated once as static data, fourteen verbs and eight closed causes, carried by the v3 wire, the command line, the MCP server and the command and tool_call spans: a new verb would move it to fifteen and every pin that holds that count.
@task_item n=10 part=needs
The escher-session command line and MCP tool listing over the six served stand tasks, exit codes 0, 1, 2, 3: where a new verb or argument lands.
@task_item n=11 part=needs
The recording directory and kept-check medium: a path the launcher names by --record, owner-only, never named by a tool call; a call naming a check path on disk would be a new input class, a boundary widening that is the founder's.
@task_item n=12 part=needs
A new carrier, exit, argument kind, result field or carrier content widens the driver session boundary and escalates at P4.
@task_item n=13 part=needs
The headless stand (seven_guis::stand, its boots, Cells and the load page, pinned viewport and font): where the recorded checks run and the replay test records and replays on a fresh session of the same app.
@task_item n=14 part=needs
The two committed stand checks under tests/blitz-tests/checks/ replayed by stand_replay, the one exception to owner-only kept checks.
@task_item n=15 part=needs
A shot's PNG bytes differ across linux x86_64 and macOS aarch64: a kept check the replay test compares across OSes holds no screenshot step.
@task_item n=16 part=needs
The capability statement docs/capabilities.md and README's count of driver commands an MCP client gets as tools, pinned by the capability-statement test: they move only if a verb is added.
@task_item n=17 part=needs
The snapshot's clickable reading and the one actionable rule (focusable, an interactive role, or current listeners holding an event other than mounted): the coverage read stays consistent with what the graph counts as a handler, mounted excluded.
@task_item n=18 part=needs
Telemetry and logging: escher-driver builds its spans by hand, ungated, installs no subscriber; no log or event carries the snapshot or the coverage list's names.
@task_standing n=1
convention: each integration test file covers one behavior and opens with a //! doc naming it; tests first assert the fixture produces the condition; scenarios run in both layout modes; unit tests in a cfg(test) mod tests at the file's end or a #[path] child file
home: Conventions (conventions file, conv-tests)
@task_standing n=2
convention: stand checks share helpers through modules under tests/ that are no test target; blitz-tests keeps one test target per file
home: Conventions (conventions file, conv-tests)
@task_standing n=3
convention: modules and files open with //! docs, public items carry ///, doc comments state the rationale for constants
home: Conventions (conventions file, conv-documentation)
@task_standing n=4
convention: cargo fmt, clippy -D warnings and rustdoc -D warnings over every library crate pass, each run through ci-leg.sh; expect_used and unwrap_used allowed only beside a justification
home: Conventions (conventions file, conv-formatting-and-lints)
@task_standing n=5
convention: crate internals pub(crate) or pub(super); consuming builders with_ and without_
home: Conventions (conventions file, conv-visibility-and-naming)
@task_standing n=6
convention: the per-crate coverage floor and the stand-repeat leg's zero-failure budget hold for new code and stand tests
home: Infrastructure Patterns, CI/CD; occupied-resources file, res-ci-infrastructure
@task_item n=1 part=not_needed
Engine layout, style, Taffy, Parley, CSS approximations and unsupported features: the chunk reads the graph and replays checks, it changes no layout or styling.
@task_item n=2 part=not_needed
The select model, open select list, its painting and reconcile_select_list: no select behaviour changes.
@task_item n=3 part=not_needed
Painting, renderer backends, blitz-paint, the password mask and the shell loop and windowing: no canvas renderer and no painting is built.
@task_item n=4 part=not_needed
JavaScript on Boa (blitz-vibey-script), its timers, module loader and document_may_load: no script path is touched.
@task_item n=5 part=not_needed
Networking (blitz-net, User-Agent, per-host semaphore, HTTP cache) and outbound hosts: the command makes no request.
@task_item n=6 part=not_needed
The browser app, rdme, its history store and the about pages: not reached.
@task_item n=7 part=not_needed
The WPT runner, its outputs and the WPT, publish-browser and post-results workflows: not reached.
@task_item n=8 part=not_needed
Dependency pinning, build profiles, release versioning, Nix packaging, cross-compilation and the upstream sync's merge base: no dependency or version moves.
@task_item n=9 part=not_needed
HTML parsing, the fixture forms (google, graphite) and HTML fixture conventions: no HTML fixture or parse path changes.
@task_item n=10 part=not_needed
The document clock and VirtualClock, LOAD_WAIT and the harness's tick and settle: the coverage read moves no time.
@task_item n=11 part=not_needed
Accessibility adapter accesskit_xplat and the platform-tree refresh: no platform tree work.
@task_item n=12 part=not_needed
Drag, auto-exploration (v030-09) and error capture (v030-10): out of this chunk's scope.
@task_item n=13 part=not_needed
The cold-agent pipe's verb list and the Cold-agent bar's verb count: that entry's call, not this chunk's.
