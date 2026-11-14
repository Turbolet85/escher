@task_prose part=chunk
A driver session can start in a named fixture state of its app instead of at boot: the app's own table names its states, the session's first screen is that state's, and every verb after it acts on it as on any held screen.
The state is one optional, bounded, validated argument on the carriers start already rides (command line, MCP start tool, socket hello), checked by the driver's schema before anything runs; a name the app's table does not hold is refused with nothing started, no panic path, nothing the caller typed written back.
The proof: for each named state, the first screen of a session started in it reads (snapshot text, by answer digest or whole) as the same app's screen reached by hand from boot through the driver's own verbs, in both layout modes.
The stand's served tasks name at least the states a stand check today reaches by clicking from boot; a kept check of a session started in a state replays from that state or is refused (a fork for P4).
@task_file n=1 path=packages/escher-driver/src/schema.rs mark=edits
what: the driver schema, its verb table and its self-description — edits: START, the verb table, describe()
@task_file n=2 path=packages/escher-driver/src/command.rs mark=edits
what: the driver's command form a verb is parsed into, and the command span
@task_file n=3 path=packages/escher-driver/src/cli.rs mark=edits
what: the driver's command-line carrier (start {task} --state {name})
@task_file n=4 path=packages/escher-driver/src/mcp.rs mark=edits
what: the driver's MCP carrier and its start tool
@task_file n=5 path=packages/escher-driver/src/session.rs mark=edits
what: the driver session, its start and its held screen
@task_file n=6 path=packages/escher-driver/src/host.rs mark=edits
what: the session host that boots an app for a session
@task_file n=7 path=packages/escher-driver/src/wire.rs mark=edits
what: the socket wire form and its hello
@task_file n=8 path=packages/escher-driver/src/recording.rs mark=edits
what: the kept check written by --record, its closed key set
@task_file n=9 path=packages/escher-driver/src/replay.rs mark=edits
what: the kept check's replay and Check::from_bytes
@task_file n=10 path=packages/escher-driver/src/coverage.rs mark=edits
what: handler coverage of a check — edits: a check's replay boot
@task_file n=11 path=examples/seven_guis/src/stand.rs mark=edits
what: the seven_guis stand's served tasks — edits: the served tasks and their named states
@task_file n=12 path=examples/seven_guis/src/session_host.rs mark=edits
what: the seven_guis stand's session host — edits: the served tasks' boot
@task_file n=13 path=tests/blitz-tests/tests/fixtures/mod.rs mark=edits
what: 0.2.0's shared fixtures — boot, hold, pages and the held network stand-in, the one statement of what the checks beside the stand share
@task_file n=14 path=tests/blitz-tests/tests/session_common/mod.rs mark=edits
what: the shared session helpers of the stand's driver checks
@task_file n=15 path=tests/blitz-tests/tests/stand_start_in_state.rs
what: the new stand check that a session started in a named state reads as that state reached by hand from boot
@task_file n=16 path=.github/scripts/test_blitz_tests_targets.py mark=edits
what: the blitz-tests target pin, the list of test targets
@task_file n=17 path=docs/capabilities.md mark=edits
what: the capability statement, where verb or argument counts move
@task_file n=18 path=.github/scripts/test_capability_statement.py mark=edits
what: the capability statement's pin, where verb or argument counts move
@task_item n=1 part=needs
The driver schema's start verb gains one optional state argument of a closed value set per app, checked before anything runs; the schema still defines no role or name set of its own.
@task_item n=2 part=needs
The state crosses on the carriers start already rides — command line, MCP start tool and socket hello — as one argument with one meaning on each carrier, the same way a keyed field rides the carriers.
@task_item n=3 part=needs
A refused state name (one the app's table does not hold) starts nothing and echoes nothing the caller typed; it rides the driver's existing refusal path with no panic.
@task_item n=4 part=needs
The session's first screen in a state is read through the same snapshot model and the same snapshot text form (parts, state tokens, clickable) as any held screen; the proof compares that text, by answer digest or whole, against the screen reached by hand from boot.
@task_item n=5 part=needs
The by-hand path to each state is driven through the driver's own verbs (click, type, keys) on the stand, so the accessibility tree, the snapshot and the actionable nodes it reads are the ones those verbs already rely on.
@task_item n=6 part=needs
The proof holds in both layout modes, and each named state's expected screen is stated once, beside the stand's other expectation tables, not restated per check.
@task_item n=7 part=needs
A session started in a state still builds its accessibility tree and snapshot through the session library, which always enables accessibility; the state is applied before the first snapshot, not after it.
@task_item n=8 part=needs
The seven_guis served tasks name their states in the app's own table, covering at least the states a stand check today reaches by clicking from boot; served node ids and the actions answer of each state's screen stay as the stand's contracts state them.
@task_item n=9 part=needs
A kept check (--record) of a session started in a state replays from that state — its closed key set and Check::from_bytes take the state name — or is refused; which one is a fork for P4, and the replay's handler coverage reads the state's boot.
@task_item n=10 part=needs
No new log field or span field: the command span records no state name, the state being a call-supplied argument; the obs allowlist and redaction stay as they are.
@task_item n=11 part=needs
The new check joins the stand checks, repeated three rounds like the rest, and runs in the accessibility test leg of the slow tier where the stand checks run.
@task_item n=12 part=needs
The UI graph of a session started in a state mirrors that state's snapshot one node per node and carries no values, as for any held screen.
@task_standing n=1
convention: each new stand check states its expected roles, names and screen once in the shared stand expectation tables and asserts against them
home: screen-reader-support · 7-screen-reader-support
@task_standing n=2
convention: each new stand check is repeated three rounds and runs in the slow-tier accessibility leg that gates CI
home: ci-integration · 9-ci-integration
@task_standing n=3
convention: each new driver argument or answer reads through the one snapshot model and its text form, never a second reader
home: screen-reader-support · 7-screen-reader-support
@task_item n=1 part=not_needed
ARIA role mapping, accessible-name precedence and ARIA state or property reads: the chunk changes no engine code and no element's markup, so how roles, names and states are derived does not move.
@task_item n=2 part=not_needed
Keyboard contracts of the Cells grid, CRUD list, range input and single or multiple select (key handling, open and closed list behaviour, Tab order): the chunk adds no key handling and changes no control's behaviour.
@task_item n=3 part=not_needed
Hidden and inert content handling and hit-testing of inert elements: engine behaviour the chunk neither changes nor names.
@task_item n=4 part=not_needed
The platform accessibility adapter, accesskit_xplat and the accessibility tier and reach rows: the chunk builds no adapter and changes no feature default.
@task_item n=5 part=not_needed
WCAG contrast ratio formulas, the lint's contrast finding fields and colour pairs of controls and select lists: no style, colour or lint rule changes.
@task_item n=6 part=not_needed
Cognitive accessibility rows (invalid state not by colour or class alone): no form validation or date state is touched.
@task_item n=7 part=not_needed
Password masking, IME and caret rows: no text input handling changes.
@task_item n=8 part=not_needed
Screenshot rows (element and row shots): the proof reads snapshot text, not pixels, and the screenshot verb is untouched.
