@task_prose part=chunk
A driver session can start in a named fixture state of its app instead of at boot: the app's own table names its states, the session's first screen is that state's, and every verb after it acts on it as on any held screen.
The state is one optional, bounded, validated argument of a closed value set per app, carried where start already rides (command line start {task} --state {name}, the MCP start tool, the socket's hello); a name the app's table does not hold is refused with nothing started, no panic path, nothing the caller typed written back.
The seven_guis served tasks name at least the states a stand check today reaches by clicking from boot, built on 0.2.0's shared fixtures' boot and hold rather than a second boot path.
A kept check (--record) of a session started in a state replays from that state, or such a recording is refused (a P4 fork).
The proof: a new stand_start_in_state check reads, for each named state, the first screen's snapshot text (by answer digest or whole) against the same app's screen reached by hand from boot through the driver's own verbs, in both layout modes.
@task_file n=1 path=packages/escher-driver/src/schema.rs mark=edits
what: the driver schema, the one statement of the driver's verbs and their arguments — edits: START, the verb table, describe()
@task_file n=2 path=packages/escher-driver/src/command.rs mark=edits
what: the driver's command, a call parsed and checked against the schema before it runs
@task_file n=3 path=packages/escher-driver/src/cli.rs mark=edits
what: the escher-session command line — the start command form, its usage line and refused calls
@task_file n=4 path=packages/escher-driver/src/mcp.rs mark=edits
what: the escher-session MCP role — the start tool, tools/list and refused input
@task_file n=5 path=packages/escher-driver/src/session.rs mark=edits
what: the driver session — Session::start and the held state every verb acts on
@task_file n=6 path=packages/escher-driver/src/host.rs mark=edits
what: the session host — start and attach of the process holding one headless app instance
@task_file n=7 path=packages/escher-driver/src/wire.rs mark=edits
what: the session wire — the socket protocol's hello and its request bounds
@task_file n=8 path=packages/escher-driver/src/recording.rs mark=edits
what: the session recording — the kept check file, its closed key set (record fields) and Check::from_bytes
@task_file n=9 path=packages/escher-driver/src/replay.rs mark=edits
what: check replay — a kept check replayed through the ordinary call path from its start
@task_file n=10 path=packages/escher-driver/src/coverage.rs mark=edits
what: the coverage role, handler coverage measured by replay — edits: a check's replay boot
@task_file n=11 path=examples/seven_guis/src/stand.rs mark=edits
what: the seven_guis headless stand — the served tasks and their boot functions
@task_file n=12 path=examples/seven_guis/src/session_host.rs mark=edits
what: the seven_guis session host — the served tasks' boot behind the session binary
@task_file n=13 path=tests/blitz-tests/tests/fixtures/mod.rs mark=edits
what: 0.2.0's shared fixtures, the one statement of what the checks beside the stand share — boot, hold, pages and the held network stand-in
@task_file n=14 path=tests/blitz-tests/tests/session_common/mod.rs mark=edits
what: the shared module of the session checks — hosts started and held behind a guard
@task_file n=15 path=tests/blitz-tests/tests/stand_start_in_state.rs
what: a new stand check — a session started in each named state reads as that state reached by hand from boot, in both layout modes
@task_file n=16 path=.github/scripts/test_blitz_tests_targets.py mark=edits
what: the blitz-tests target pin — the list of the blitz-tests test targets, one per file
@task_file n=17 path=docs/capabilities.md mark=edits
what: the capability statement pinned to the driver schema — edits: its verb and argument counts, where they move
@task_file n=18 path=.github/scripts/test_capability_statement.py mark=edits
what: the capability statement's pin — claim section names and counts held to the schema
@task_item n=1 part=needs
The start verb gains one optional state argument of a closed value set per app, on the carriers start already rides: the command line's start command form and usage line, the MCP start tool in tools/list, and the session wire's hello; the carrier set and whether a new argument on a ratified carrier is a boundary widening is a P4 fork, the most conservative option provisional.
@task_item n=2 part=needs
The driver schema states the new argument: the START entry, the verb table and describe() carry it, and help on both surfaces (command line and MCP) reads it from the schema alone.
@task_item n=3 part=needs
The state name is validated by the schema before anything runs: an argument kind with a closed value set and a length bound; a name the app's table does not hold is a refused call (malformed) with nothing started, the instance and state directory unchanged, and no panic path on what the call supplies.
@task_item n=4 part=needs
Nothing the caller typed is written back: a refused start's refusal line, error object, stderr and MCP tool result carry fixed text only, never the state name supplied.
@task_item n=5 part=needs
No new socket, port, listener, env read, log field or span field: the command span and tool_call span record no state name, and the driver's log line keeps fixed words and counts only.
@task_item n=6 part=needs
Session start (Session::start, the session host's start and the socket's hello) yields as first screen the named state's, and every verb after it — reading and acting — acts on it as on any held state; with no state named, start boots as today.
@task_item n=7 part=needs
The app's own table names its states: the seven_guis served tasks name at least the states a stand check today reaches by clicking from boot, so the observed gap is closed where it was observed.
@task_item n=8 part=needs
The fixture states build on the shared fixtures' boot and hold — no second boot path; whether they live in the served app, in the test fixtures, or both is closed by P3.
@task_item n=9 part=needs
How a state is reached — an app-supplied initial state carried the way the seed rides a root context, or steps run from boot inside start — is measured by P3 and decided by P4; either way the stand still boots offline and its network stand-in keeps its loads counted.
@task_item n=10 part=needs
A kept check (--record) of a session started in a state replays from that state: the kept check's closed key set and Check::from_bytes take the state name, or a recording of a session started in a state is refused — a P4 fork changing the kept check's at-rest content; committed checks stay byte-equal to the binary's recording either way.
@task_item n=11 part=needs
Check replay and the coverage role's replay boot start a replayed check from the state it was recorded in (or refuse it, per the fork), through the ordinary call path.
@task_item n=12 part=needs
The proof check: for each named state, the first screen's snapshot text of a session started in it equals, by answer digest or whole, the same app's screen reached by hand from boot through the driver's own verbs, in both layout modes; byte comparisons across OSes hold no screenshot step.
@task_item n=13 part=needs
The session wire's hello carries the optional state within its request bound and version; a hello naming an unknown state is refused as any out-of-grammar request, the host not started.
@task_item n=14 part=needs
The escher-session exit status of a refused start (refused call) and of an accepted start keep their meanings; a start in a state already running answers as start does today.
@task_item n=15 part=needs
The blitz-tests target pin lists the new stand_start_in_state target, one target per file.
@task_item n=16 part=needs
The capability statement and its pin move with the schema where the verb or argument counts move; claim names stay held to the schema and the claim names its checks.
@task_item n=17 part=needs
A deferral of any part is a residual plus its line in the 0.4.0 incubator's deferred-from-0.3.0 list; a controls file's case names the test alone.
@task_standing n=1
convention: one integration test target per file, a shared module (fixtures, session_common) never a target
home: test-strategy · 2-test-strategy
@task_standing n=2
convention: a stand check runs in both layout modes
home: test-scope-summary · integration-test-strategy
@task_standing n=3
convention: a session check owns its host, held behind a guard and reaped on failure, its child streams sent to null and host pipes drained
home: session-lifecycle · test-strategy
@task_standing n=4
convention: socket and recording session checks are unix-gated
home: session-lifecycle
@task_standing n=5
convention: stand checks run on the bundled pinned font, never skipped
home: test-strategy · 8-mocking-stubbing-discipline
@task_standing n=6
convention: no sleep or wall-clock read in a session check
home: integration-test-strategy · 5-integration-test-strategy
@task_standing n=7
convention: cross-OS readings of served tasks by answer digest, no screenshot step in a byte-compared check
home: test-strategy · session-lifecycle
@task_standing n=8
convention: new tests controlled red-before-green; table-driven tests assert their row count; a failure prints no argument value
home: unit-test-strategy
@task_standing n=9
convention: test data stated in code, shared fixture pages stated once in the shared fixtures module, nothing seeded
home: 3-test-harness-contract · 7-test-data-fixtures
@task_standing n=10
convention: stand checks repeated three rounds under a zero-failure budget
home: 9-ci-integration · quality-gates-coverage-targets
@task_item n=1 part=not_needed
The WPT conformance runner, its command line, outputs, status set and pinned commit: the chunk touches no web-platform suite.
@task_item n=2 part=not_needed
The cold-agent pipe's events, verdict, exit codes, invocation and isolation: the chunk runs and changes no cold-agent run.
@task_item n=3 part=not_needed
The five-command agent-run contract (its verbs, selections, events, exit codes and state files): the session lies outside it and the chunk adds nothing to it.
@task_item n=4 part=not_needed
Lint defects and findings, and the harness's text contrast reading and its WCAG formula: no lint or contrast reading changes.
@task_item n=5 part=not_needed
Engine paint and layout rows — select list painting, scrollbar fade, box-shadow expectations, the incremental layout oracle, buffered render on the Dioxus bridge: the chunk changes no engine.
@task_item n=6 part=not_needed
The apt-install and coverage-floor scripts' exit codes and the CI log and artifact names: no CI workflow or install script changes.
@task_item n=7 part=not_needed
Auto-exploration, a seed on the driver's carriers and a component alone with props: later entries, out of this chunk.
