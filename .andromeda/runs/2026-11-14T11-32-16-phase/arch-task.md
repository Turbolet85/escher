@task_prose part=chunk
A driver session can be started in a named fixture state of its app instead of at boot: the app's own table names its states, the session's first screen is that state's, and every verb after it acts on it as on any held screen.
The state is one optional, bounded argument of a closed value set per app on the carriers start already rides (the command line, the MCP start tool, the socket's hello), checked by the driver's schema before anything runs; an unknown name is refused with nothing started and nothing typed echoed.
A kept check of a session started in a state replays from that state, or such a recording is refused (a P4 fork); the stand's served tasks name at least the states a stand check today reaches by clicking from boot.
Proof: for each named state, the first screen of a session started in it reads (snapshot text, by digest or whole) as the same screen reached by hand from boot through the driver's own verbs, in both layout modes.
It builds on 0.2.0's shared blitz-tests fixtures (their boot and hold), not a second boot path; the capability statement and its pin move only where verb or argument counts move.
@task_file n=1 path=packages/escher-driver/src/schema.rs mark=edits
what: the driver's command schema — the verb table, its argument kinds and the self-description shared by command line and MCP — edits: the START entry, the verb table, describe()
@task_file n=2 path=packages/escher-driver/src/command.rs mark=edits
what: the driver's validated command — a call's verb and arguments after validation, run only once validated
@task_file n=3 path=packages/escher-driver/src/cli.rs mark=edits
what: the escher-session command line — start <task>, its flags, usage line and exit statuses
@task_file n=4 path=packages/escher-driver/src/mcp.rs mark=edits
what: the mcp role — the MCP server over stdio, its tool listing and its start tool's call crossing
@task_file n=5 path=packages/escher-driver/src/session.rs mark=edits
what: the escher-driver session — starting an app from the session apps table and holding its screen for the verbs
@task_file n=6 path=packages/escher-driver/src/host.rs mark=edits
what: the session host — the one-process host behind the session socket that starts the session on hello
@task_file n=7 path=packages/escher-driver/src/wire.rs mark=edits
what: the session socket's wire — the hello, call and stop requests and their replies, versioned
@task_file n=8 path=packages/escher-driver/src/recording.rs mark=edits
what: session recording — the kept check file, its opening record and closed key set, written to the recording directory
@task_file n=9 path=packages/escher-driver/src/replay.rs mark=edits
what: kept check replay — Check::from_bytes, the strict kept check reader, and Check::replay through the one run path
@task_file n=10 path=packages/escher-driver/src/coverage.rs mark=edits
what: the command line's coverage role — handler coverage over named kept checks, each replayed from its own per-check boot — edits: a check's replay boot
@task_file n=11 path=examples/seven_guis/src/stand.rs mark=edits
what: the seven_guis headless stand — the served tasks and their boot
@task_file n=12 path=examples/seven_guis/src/session_host.rs mark=edits
what: the seven_guis session host — the session apps table the driver starts served tasks from, and their boot
@task_file n=13 path=tests/blitz-tests/tests/fixtures/mod.rs mark=edits
what: the shared blitz-tests fixtures — boot, hold, pages and the held network stand-in, the one statement of what the checks beside the stand share
@task_file n=14 path=tests/blitz-tests/tests/session_common/mod.rs mark=edits
what: the shared stand-check session helpers — a non-target module the stand checks share for driving a session
@task_file n=15 path=tests/blitz-tests/tests/stand_start_in_state.rs
what: a new stand check — a session started in each named state reads as that state reached by hand from boot, in both layout modes
@task_file n=16 path=.github/scripts/test_blitz_tests_targets.py mark=edits
what: the blitz-tests target pin — the CI-script test that holds the list of blitz-tests test targets
@task_file n=17 path=docs/capabilities.md mark=edits
what: the capability statement — its sections and the verb and argument counts it states, held to the driver schema
@task_file n=18 path=.github/scripts/test_capability_statement.py mark=edits
what: the capability statement pin — the CI-script test that holds the statement to the driver schema's verb and argument counts
@task_item n=1 part=needs
The start verb gains one optional argument naming a state: its argument kind, its place in the verb table's order, and the self-description command line and MCP share all carry it.
@task_item n=2 part=needs
The argument is validated before anything runs: a value of a closed set per app, bounded in length, refused with a schema refusal cause and fault when the app's table does not hold it, with nothing started and no panic path on what the call supplies.
@task_item n=3 part=needs
A refused or accepted state name is never echoed back: the refusal's content and the answer carry no typed input, and a call validation refuses records no ids.
@task_item n=4 part=needs
The state crosses only on the carriers start already rides — the command line's start <task> --state, the MCP start tool, the socket's hello request — and whether a new argument on a ratified carrier widens the session boundary is a fork P4 settles; the most conservative option is provisional.
@task_item n=5 part=needs
The validated verb set and the argument set may widen only by ratification; the new argument goes through that rule and through the boundary-widening rule for carriers.
@task_item n=6 part=needs
The socket's hello request and its wire version: carrying the state on hello may change the hello's shape and the version the wire states; the request line bound still holds.
@task_item n=7 part=needs
The session apps come from one table; each app's own table names its states, and the session library still depends on no app.
@task_item n=8 part=needs
The session's first screen is the named state's: every verb after start (snapshot, actions, graph, click, type, press, scroll, advance, screenshot, lint) acts on it as on any held screen, and the session clock, settle and the virtual clock behave as from a boot.
@task_item n=9 part=needs
How a state is reached — an app-supplied initial state riding a Dioxus root context the way the seed does, or steps run from boot inside start — is left open to P3 and P4; either way the stand still boots offline with its pinned environment and bundled font.
@task_item n=10 part=needs
The kept check: a recorded session started in a state must replay from that state, so the opening record's closed key set and Check::from_bytes's strict read take the state name, or a recording of such a session is refused — a P4 fork changing the kept check's at-rest content.
@task_item n=11 part=needs
Kept check replay and the coverage role's per-check boot replay from the recorded state through the one run path; replaying a check of another app is still refused.
@task_item n=12 part=needs
The command span records no state name; no new log field, span field, env read, socket, port or listener; the driver logs no content and emits only its hand-built spans.
@task_item n=13 part=needs
The seven_guis served tasks name at least the states a stand check today reaches by clicking from boot, so the observed gap is closed where it was observed.
@task_item n=14 part=needs
The fixture states build on the shared blitz-tests fixtures' boot and hold rather than a second boot path; whether they live in seven_guis (served), in the test fixtures (test-only) or both is left to P3.
@task_item n=15 part=needs
The proof compares a session started in a state with the same screen reached by hand from boot through the driver's own verbs, by snapshot text or its answer digest, in both layout modes, and the reading must be identical on every CI OS.
@task_item n=16 part=needs
Where the verb or argument counts move, the capability statement and its pin move with the driver schema, and the root README's count of driver commands an MCP client gets as tools stays true.
@task_item n=17 part=needs
The start verb's contract on each carrier — the driver verbs' start, the command line's start, the escher-session start line, the session client's start, the MCP tool listing — is restated with the new argument, and its refusal JSON shape and session errors stay as they are.
@task_item n=18 part=needs
The validation order: the first failure is reported, and the state argument takes its place in that order without changing which failure the existing calls report.
@task_item n=19 part=needs
The stand's no-telemetry and no-network rules hold for a session started in a state: no environment read, no network load beyond the held network stand-in.
@task_standing n=1
convention: a new blitz-tests integration test file is its own test target, states one behaviour, has a descriptive name, and joins the target pin
home: conventions
@task_standing n=2
convention: a new stand check runs its scenarios in both layout modes, asserts the fixture's condition first and shares its helpers through non-target modules
home: conventions
@task_standing n=3
convention: shared blitz-tests fixtures are stated once, in the shared fixtures module, and a stand check builds on them
home: conventions
@task_standing n=4
convention: new driver code is documented at module and item level, keeps crate internals restricted, returns typed errors as enums and justifies every expect or unwrap
home: conventions
@task_standing n=5
convention: new driver unit tests live in path children of the module they test, at the file's end
home: conventions · existing-scopes
@task_standing n=6
convention: stand readings are identical on every CI OS, the stand boots offline with a pinned environment, and the stand-repeat leg runs with a zero-failure budget
home: standard-contracts · inherited-defaults
@task_standing n=7
convention: every escher crate keeps its per-crate coverage floor, and fmt, clippy and rustdoc warnings gate the change
home: inherited-defaults
@task_item n=1 part=not_needed
The CSS, layout, paint, style, text, scrolling and hit-test decisions of the engine: the chunk expects no engine change.
@task_item n=2 part=not_needed
The WPT runner, its result kinds, statuses, outputs, CI artifacts and PR-body tooling: the chunk adds no web-platform test.
@task_item n=3 part=not_needed
The browser app, its history store, about pages, toolbar, persistence and publish workflow: the chunk reaches no browser code.
@task_item n=4 part=not_needed
The JavaScript runtime, script fetchers, script globals and Boa decisions: no stand task runs page script.
@task_item n=5 part=not_needed
The select list, form controls, range input and DOM event-dispatch rows: a started state is reached through the app, not through new engine default actions.
@task_item n=6 part=not_needed
The accessibility tree, AccessKit adapter and platform adapter rows: the snapshot readers are used as they are.
@task_item n=7 part=not_needed
The cold-agent and agent-run scripts, their events, exit codes, verdicts and state directories: the chunk changes no operator run.
@task_item n=8 part=not_needed
Auto-exploration, a seed crossing the driver's carriers and a component alone with props: they are other entries, out of this chunk.
@task_item n=9 part=not_needed
The lint verb's defect set and reason keys, the graph verb's answer and the actions verb's answer: their behaviour on a held screen is unchanged.
@task_item n=10 part=not_needed
The Nix flake, the package-install script, toolchain and MSRV, allocator and dependency-audit decisions: the chunk adds no dependency and no build change.
