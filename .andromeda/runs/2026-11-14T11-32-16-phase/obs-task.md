@task_prose part=chunk
A driver session can start in a named fixture state of its app instead of at boot: the app's own table names its states, the session's first screen is that state's, and every verb after it acts on it as on any held screen.
The state is one optional, bounded, validated argument of start (command line start {task} --state {name}, the MCP start tool, the socket's hello), checked by the driver's schema before anything runs; a name the app does not hold is refused with nothing started, no panic path, nothing typed written back.
A kept check recorded from a session started in a state either replays from that state (its closed key set takes the state name) or is refused at record time; P4 picks.
The stand's served tasks name at least the states a stand check today reaches by clicking from boot, and a new check proves each state's first screen reads (snapshot text, by answer digest or whole) as that screen reached by hand from boot, in both layout modes.
@task_file n=1 path=packages/escher-driver/src/schema.rs mark=edits
what: the driver schema, its verb table and describe() — edits: START, the verb table, describe()
@task_file n=2 path=packages/escher-driver/src/command.rs mark=edits
what: the driver command, the call a verb runs as, under the one command span
@task_file n=3 path=packages/escher-driver/src/cli.rs mark=edits
what: escher-session's command line, its commands, exit codes and fixed diagnostics — edits: start {task} --state {name}
@task_file n=4 path=packages/escher-driver/src/mcp.rs mark=edits
what: escher-session's mcp role, its tools and the tool_call span — edits: the start tool
@task_file n=5 path=packages/escher-driver/src/session.rs mark=edits
what: the driver session and its lifecycle, from start to idle expiry
@task_file n=6 path=packages/escher-driver/src/host.rs mark=edits
what: escher-session's host role, the session host behind the socket
@task_file n=7 path=packages/escher-driver/src/wire.rs mark=edits
what: the escher-session host wire (v3) — edits: hello
@task_file n=8 path=packages/escher-driver/src/recording.rs mark=edits
what: the session recording and the kept check file, its closed key set
@task_file n=9 path=packages/escher-driver/src/replay.rs mark=edits
what: the kept check's replay, Check::from_bytes — edits: Check::from_bytes
@task_file n=10 path=packages/escher-driver/src/coverage.rs mark=edits
what: escher-session's coverage role and its coverage line — edits: a check's replay boot
@task_file n=11 path=examples/seven_guis/src/stand.rs mark=edits
what: the seven_guis stand, its served tasks — edits: the served tasks' named states
@task_file n=12 path=examples/seven_guis/src/session_host.rs mark=edits
what: the served tasks' boot, the headless stand host escher-session runs
@task_file n=13 path=tests/blitz-tests/tests/fixtures/mod.rs mark=edits
what: 0.2.0's shared fixtures — boot, hold, pages and the held network stand-in, the one statement of what the checks beside the stand share
@task_file n=14 path=tests/blitz-tests/tests/session_common/mod.rs mark=edits
what: the shared session module the driver session checks use
@task_file n=15 path=tests/blitz-tests/tests/stand_start_in_state.rs
what: the new stand_start_in_state check — a session started in each named state reads as that state reached by hand from boot, in both layout modes
@task_file n=16 path=.github/scripts/test_blitz_tests_targets.py mark=edits
what: the blitz-tests target pin, the list of test targets CI runs — edits: the new stand_start_in_state target
@task_file n=17 path=docs/capabilities.md mark=edits
what: the capability statement, updated by hand where verb or argument counts move
@task_file n=18 path=.github/scripts/test_capability_statement.py mark=edits
what: the capability statement's pin, the counts it holds the statement to
@task_item n=1 part=needs
The start verb gains one optional state argument of a closed value set per app, on the carriers start already rides: the command line, the MCP start tool and the socket's hello on the host wire; whether a new argument on a ratified carrier widens the boundary, and its carrier set, is a P4 fork.
@task_item n=2 part=needs
A state name the app's table does not hold, or a malformed one, is refused by the driver schema before anything runs, with nothing started and a refusal cause from the closed set of refusal cause names; the escher-session exit code and error object for that refusal stay within their fixed sets.
@task_item n=3 part=needs
Nothing the caller typed is written back: neither the refusal nor any answer or diagnostic echoes the state name; escher-session diagnostics stay fixed strings.
@task_item n=4 part=needs
The driver's command span for start records no state name — a call-supplied argument is call content; the span keeps its closed field set and field count, and no span field is added.
@task_item n=5 part=needs
The MCP tool_call span for a start carrying a state keeps its tool and ending fields only, its ending (accepted, refused, error) as for any call, and the command span its child.
@task_item n=6 part=needs
No new log field, env read, socket, port or listener; the driver library still emits no event and installs no subscriber, and the headless stand host still runs with no sink.
@task_item n=7 part=needs
The host wire's hello carries the state to the session host; a change to the hello's shape is a change to the v3 host wire contract and its version.
@task_item n=8 part=needs
A kept check of a session started in a state: either the kept check file's closed key set and its arguments take the state name and replay boots from it, or recording such a session is refused — a P4 fork over the kept check's at-rest content; committed kept checks still hold no private data.
@task_item n=9 part=needs
escher-session's coverage role boots a check's replay; if a kept check carries a state, the replay boot starts in it, the coverage line's shape, its accepted, malformed and boot-failure outcomes unchanged, and replay children stay uninstrumented.
@task_item n=10 part=needs
The session lifecycle gains a start in a state: the first screen is the state's, every later verb acts on it as on any held screen, and idle expiry and the session state directory's privacy are unchanged.
@task_item n=11 part=needs
describe() and the escher-session help name the new argument; the capability statement is updated by hand where verb or argument counts move, and its pin follows.
@task_item n=12 part=needs
The seven_guis stand's served tasks name their fixture states in the app's own table, at least the states a stand check today reaches by clicking from boot; how a state is reached (an app-supplied initial state or steps from boot inside start) is left to P3 and P4.
@task_item n=13 part=needs
The proof compares snapshot text, by answer digest or whole, of a session started in a state with the same screen reached by hand from boot through the driver's verbs, in both layout modes; a mismatch is reported without printing either answer.
@task_item n=14 part=needs
The new check builds on the shared fixtures' boot and hold rather than a second boot path, and joins the blitz-tests target pin.
@task_standing n=1
convention: each new check runs silent and shares its helpers through the shared modules, printing nothing on success
home: 3-observability-harness-contract
@task_standing n=2
convention: a check that asserts empty stderr unsets RUST_LOG for the child it runs
home: 3-observability-harness-contract
@task_standing n=3
convention: a check failure names a kind and a count, never an answer, a snapshot text or a typed value
home: 3-observability-harness-contract
@task_standing n=4
convention: driver span fields carry no call content; only closed-value fields print
home: 4-span-trace-coverage · span-trace-coverage
@task_item n=1 part=not_needed
Metric coverage: the chunk adds no counter, gauge, histogram or timing, and publishes no score.
@task_item n=2 part=not_needed
Script error capture: window error events, window.onerror, script error source labels, the script diagnostics drain and the WPT report; the chunk changes no engine and no script path.
@task_item n=3 part=not_needed
Form-control behaviour rows: select, optgroup, multiple select form entries, open list rows and page change events; the chunk changes no control and no engine.
@task_item n=4 part=not_needed
The agent-run harness and the cold-agent pipe: their events, verdict fields, reasons, counted verbs, transcripts and cleanup; the chunk changes neither harness.
@task_item n=5 part=not_needed
Engine tracing behind a cargo feature and the WASM console panic hook; the chunk reaches neither the engine's tracing nor a WASM build.
@task_item n=6 part=not_needed
The sink's log line layout, value escaping, resource keys service.name and service.version and the sink target allowlist; the chunk adds no log line and no target.
@task_item n=7 part=not_needed
CI platform rows: the package-install diagnostic line, the fork CI artifacts, the coverage and per-leg log uploads and WPT publishing from upstream.
@task_item n=8 part=not_needed
Stylesheet loading and file: loads from non-file documents; the chunk loads no resource.
