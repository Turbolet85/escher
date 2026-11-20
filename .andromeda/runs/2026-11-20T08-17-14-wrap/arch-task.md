@task_prose part=session
A codebase-health refactor with no change of behaviour: no public item, signature, verb, field, span, log site, dependency, crate or manifest changed, and every workspace test keeps its path.
Escher's own oversize files were split: the driver's JSON unit tests lost their fixture builders to a new test-only child module, the Key enum moved whole into a private child module of the driver's command module and is re-exported unchanged, and the snapshot reader's unit tests moved out of line into a path child.
Escher's own functions over cognitive complexity 15 were split into private sibling helpers: the command line's argument reading, the MCP server's run loop, the Dioxus runtime probe's walk and three stand checks, whose new assertion helpers are track-caller.
Measured: escher's own functions over cognitive 15 went 7 to 0; escher's own files over 800 code lines went 10 to 7, the seven left being integration-test targets; coverage read escher-driver 97.92 percent and dioxus-native-dom 89.91 percent, each at or above its floor.
The founder ruled on four forks: upstream's fifteen files over 800 code lines, its ninety-four functions over cognitive 15 and its cdp-server and vibey-script duplicated test pairs are kept as upstream's, and escher's seven oversize integration-test targets are kept as they are with no route entry.
A path test child not named with the tests suffix was found counted in its crate's coverage reading, which raised escher-driver by 0.15 points.
@task_file n=1 path=packages/escher-driver/src/json_tests.rs
what: edited — the driver's JSON answer writer's unit tests, the out-of-line json tests module of the hand-written answers
@task_file n=2 path=packages/escher-driver/src/json_tests/fixtures.rs
what: added — the shared fixture builders of the driver's JSON unit tests, a test-only child of the json tests module visible to its parent alone
@task_file n=3 path=packages/escher-driver/src/command.rs
what: edited — the driver's command module, the validated commands and the press verb's key, now re-exporting Key from its child
@task_file n=4 path=packages/escher-driver/src/command/key.rs
what: added — the Key enum, the closed set of key names the press verb and the key argument kind accept, with its name and from_name readers
@task_file n=5 path=packages/dioxus-native-dom/src/snapshot.rs
what: edited — dioxus-native-dom's snapshot reader, the compact semantic snapshot of the screen and its node state
@task_file n=6 path=packages/dioxus-native-dom/src/snapshot_tests.rs
what: added — the snapshot reader's unit tests, moved out of line into a path child keeping their test paths
@task_file n=7 path=packages/escher-driver/src/cli.rs
what: edited — the escher-session command line's reading of its roles (host, mcp, coverage) and session address
@task_file n=8 path=packages/escher-driver/src/mcp.rs
what: edited — the MCP server of the mcp role over stdio, its run loop over one instance and one session
@task_file n=9 path=tests/blitz-tests/tests/dioxus_runtime_probe.rs
what: edited — the blitz-tests probe of the Dioxus runtime walking the vdom's roots, listeners and dynamic nodes
@task_file n=10 path=tests/blitz-tests/tests/stand_graph.rs
what: edited — the stand check of the UI graph: every graph node a snapshot node, no served id in the reserved form, declarations agreeing with code once handlers run
@task_file n=11 path=tests/blitz-tests/tests/stand_actions.rs
what: edited — the stand check that each served screen's actions answer lists its actionable nodes with the verb table's verbs
@task_file n=12 path=tests/blitz-tests/tests/stand_accessibility_ids.rs
what: edited — the stand check that every interactive-role element and every region reads an accessible name
@task_item n=1 part=touches
The cognitive complexity ceiling of 15: escher's own functions now all sit under it (7 to 0), and upstream's ninety-four functions over it are kept as upstream's by the founder's ruling F2.
@task_item n=2 part=touches
Driver unit tests in path children: the json tests module gains a test-only fixtures child, and that child is not named with the tests suffix.
@task_item n=3 part=touches
Unit-test placement and unit tests at the end of the source file: the snapshot reader's tests module left its file for an out-of-line path child, as the driver's tests already had.
@task_item n=4 part=touches
Split test modules stay children of their module and moved unit-test modules keep their test paths: both moves keep every test path (1631 listed, none gone, none new).
@task_item n=5 part=touches
Per-crate coverage floors and the coverage leg's reading: dioxus-native-dom read 89.91 percent and escher-driver 97.92 percent; a row holding a crate's floor or a reading meets it here, and a path test child not named with the tests suffix is counted in the reading.
@task_item n=6 part=touches
Track-caller assertion helpers: three stand checks gain private track-caller assertion helpers, each message as written.
@task_item n=7 part=touches
Stand-check helpers in non-target modules: the new stand assertion helpers stay private to their own target file rather than moving into the shared non-target modules.
@task_item n=8 part=touches
One test target per file in blitz-tests and one behaviour per integration test file: seven integration-test targets stay over 800 code lines, kept as they are by the founder's ruling F4.
@task_item n=9 part=touches
The key names contract and the key argument kind: the Key enum behind them moved into a private child module and is re-exported, its twelve names, order and readers unchanged.
@task_item n=10 part=touches
The escher-session command line's roles, usage and exit statuses: its argument reading was split into role and address helpers with no change of shape, refusal or exit status.
@task_item n=11 part=touches
The MCP server and the mcp role over stdio: the run loop was split into an instance run and a session run, initialize, tools listing, tool calls, idle expiry and bounds unchanged.
@task_item n=12 part=touches
The snapshot contract and its node state readers: the reader's file changed only by its tests leaving it; no node key, state or membership changed.
@task_item n=13 part=touches
Restricted crate internals: the fixtures child is visible to its parent alone and the key module is private, its type reached only through the re-export.
@task_item n=14 part=touches
Listeners read from the vdom walk: the runtime probe's walk was split into roots, listeners and dynamic-node visitors that recurse as before.
@task_item n=15 part=touches
The UI graph and actions verb checks on the stand: the stand checks of graph agreement, reserved id form and actionable listing were restructured, their assertions unchanged.
@task_fact n=1 slug=conventions
fact: Escher's own source files stay at or under 800 code lines, the exception being its seven oversize integration-test targets, kept as they are by the founder's ruling; upstream-authored files over the bar are kept as upstream's.
@task_fact n=2 slug=stack-and-technologies
fact: Upstream-authored functions over the cognitive complexity ceiling of 15 are kept as upstream's by the founder's ruling; the ceiling binds escher's own functions alone.
@task_fact n=3 slug=conventions
fact: Upstream's duplicated test pairs in the cdp-server and vibey-script crates are kept as upstream's by the founder's ruling; escher does not deduplicate upstream tests.
@task_fact n=4 slug=stack-and-technologies
fact: A crate's coverage reading counts a path test child whose file name lacks the tests suffix; only files named with that suffix are left out of the reading.
@task_item n=1 part=untouched
Engine behaviour of style, layout, paint, hit testing, scrolling, select lists, text and fonts: no engine code changed.
@task_item n=2 part=untouched
The session socket's wire, the session host process model and the session errors: no wire version, reply, bound or error changed.
@task_item n=3 part=untouched
Telemetry, spans and logging rows: no span, field or log site changed and the touched files stay silent.
@task_item n=4 part=untouched
Stack and dependency choices, toolchain, Nix flake, cargo-deny and CI workflow and leg contracts: no dependency, manifest, tool version, workflow or leg changed.
@task_item n=5 part=untouched
Occupied names of crates, binaries, servers and environment variables: none taken or freed.
@task_item n=6 part=untouched
The browser app, the WPT runner, the example CLIs, the script runtime and the cold-agent and agent-run scripts: none reached.
@task_item n=7 part=untouched
Absences of capabilities: nothing added to or removed from what escher can do.
