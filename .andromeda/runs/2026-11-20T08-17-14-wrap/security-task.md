@task_prose part=session
A codebase-health pass, refactor only: escher's own files over 800 code lines and functions over cognitive complexity 15 were split under those bars, and no behaviour changed.
No public item, signature, verb, field, span, log site, refusal or wire shape changed. No dependency, schema, config, CI job or gate changed, and nothing was retired.
The driver's Key enum (the closed key-name table) moved whole into a private child module and is re-exported unchanged. The command-line argv reader and the MCP server's run loop were each split into private helpers with the same behaviour.
The snapshot's unit tests and the driver JSON tests' fixture builders moved out of line into their own files, with no test name or body changed. Four stand checks had their bodies split into assert helpers.
Coverage was measured with every floor held: escher-driver went from 97.77 to 97.92 percent, dioxus-native-dom from 90.72 to 89.91 percent, and blitz-test-harness stayed at 96.07 percent.
The founder ruled that upstream-authored oversize files, upstream's complex functions and upstream's duplicated test pairs stay as upstream wrote them, and that no route entry is opened for escher's seven oversize integration-test targets.
@task_file n=1 path=packages/escher-driver/src/json_tests.rs
what: the driver's JSON answer tests, covering how the driver's answers, refusals, caused records, graph, actions and layout answers serialize to JSON lines
@task_file n=2 path=packages/escher-driver/src/json_tests/fixtures.rs
what: the test-only fixture builders for the driver's JSON answer tests (caused items, findings, action nodes, layout explanation, graph), moved out of that test module
@task_file n=3 path=packages/escher-driver/src/command.rs
what: the driver's command schema and single command validation gate, covering the closed driver verb schema, the argument kinds and the target checks
@task_file n=4 path=packages/escher-driver/src/command/key.rs
what: the driver's closed key-name table (the Key enum with its ALL list, name and from_name), which the key argument kind and the --key flag are checked against
@task_file n=5 path=packages/dioxus-native-dom/src/snapshot.rs
what: the snapshot reader, which reads each element's stable id, accessible name, state readings and masked control values into the snapshot
@task_file n=6 path=packages/dioxus-native-dom/src/snapshot_tests.rs
what: the snapshot's unit tests, covering accessible names, state readings, masked password and file values, range and select values, and the never-a-node rule for text runs, moved out of line
@task_file n=7 path=packages/escher-driver/src/cli.rs
what: the escher-session command line, with the closed argv reader for the roles (serve, mcp, coverage) and the flags
@task_file n=8 path=packages/escher-driver/src/mcp.rs
what: the MCP server over stdio, which serves the driver's verbs as tools under bounded request lines and results
@task_file n=9 path=tests/blitz-tests/tests/dioxus_runtime_probe.rs
what: the Dioxus runtime probe test, which walks the virtual DOM's roots, listeners and dynamic nodes
@task_file n=10 path=tests/blitz-tests/tests/stand_graph.rs
what: the stand graph check, which checks that every graph node is a snapshot node, that no served id holds the reserved form, and that the declarations agree with the code
@task_file n=11 path=tests/blitz-tests/tests/stand_actions.rs
what: the stand actions check, which checks that each served screen lists its actionable nodes with the table's verbs
@task_file n=12 path=tests/blitz-tests/tests/stand_accessibility_ids.rs
what: the stand accessibility check, which checks that every interactive-role element and region reads a name
@task_item n=1 part=touches
Closed argv reader and the escher-session roles and flags: the argv reader was split into role and address helpers with the same closed grammar, the same roles and the same refusals. No role or flag changed.
@task_item n=2 part=touches
The command line echoes nothing typed, and its usage and error messages: the reader that refuses bad argv was restructured but prints the same fixed texts with no caller input.
@task_item n=3 part=touches
MCP server over stdio, with no print or panic and bounded request lines and results: its run loop was split into per-instance and per-session helpers. Transport, bounds, protocol errors and idle expiry are unchanged.
@task_item n=4 part=touches
Closed driver verb schema, the single command validation gate and the key argument kind: the closed key-name table moved into its own private module and is re-exported. The set of key names and the --key flag are unchanged.
@task_item n=5 part=touches
The snapshot's masked control values (password and file inputs masked, the MASKED_VALUE stand-in, the mask holding on every carrier): the tests that guard this mask moved out of line, unchanged, and still pass.
@task_item n=6 part=touches
The snapshot's state readings and the snapshot text state words (selected, pressed, invalid, clickable, checked, disabled): the unit tests that guard these readings moved out of line unchanged.
@task_item n=7 part=touches
Stable element id, the reserved separator form and no NodeId crossing: the stand graph check that guards these was split into assert helpers, with every assertion and message kept.
@task_item n=8 part=touches
The actions answer (nodes, verbs, keyed) and the one actionable rule for both readings: the stand actions check that guards them was split into helpers, with its assertions kept.
@task_item n=9 part=touches
Accessible-name exits and region naming: the stand accessibility check that guards them was split into helpers, with its assertions kept.
@task_item n=10 part=touches
Per-crate line coverage floors: every crate stayed at or above its floor. escher-driver read 97.92 percent and dioxus-native-dom 89.91 percent. Name this only where a row's cell holds a floor or a reading.
@task_item n=11 part=touches
cargo-llvm-cov as the coverage tool: a path-declared test child whose file name does not end in _tests.rs is counted in its crate's coverage reading. The JSON test fixtures child raised escher-driver by 0.15 points.
@task_item n=12 part=touches
The JSON answer crossings (graph, actions, caused text, coverage answer): the fixture builders their serialization tests read moved into a test-only child module. No answer shape changed.
@task_item n=1 part=untouched
Dependency security, covering cargo-deny, licence gates, pins, locked builds and CI action pins: no dependency or manifest changed.
@task_item n=2 part=untouched
Secret management and environment variables: no secret, credential or environment read changed.
@task_item n=3 part=untouched
Data protection, covering the history database, HTTP cache, cookies, TLS and kept check files: nothing that stores or keeps data was reached.
@task_item n=4 part=untouched
Outbound HTTP, fetch methods, CORS and same-origin absences, iframe and import limits, and script fetchers: no network or engine-loading code was reached.
@task_item n=5 part=untouched
The session socket, its wire grammar, its replies, the recording directory and the state directory: no session host or socket code changed.
@task_item n=6 part=untouched
The telemetry sink, log target allowlists and span field contracts: no log site or span changed.
@task_item n=7 part=untouched
Authentication and authorization, CI token scopes and publish jobs: no CI workflow or publishing path changed.
@task_item n=8 part=untouched
Select lists, range inputs, password painting and the engine's event model: no engine or rendering behaviour changed. Only tests and helper structure moved.
@task_item n=9 part=untouched
Cold-agent and agent-run scripts and their checks: not reached.
