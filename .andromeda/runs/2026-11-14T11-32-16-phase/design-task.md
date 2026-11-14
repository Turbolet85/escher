@task_prose part=chunk
A driver session (escher-session) can start in a named fixture state of its app instead of at boot: the app's own table names its states, the session's first screen is that state's, and every verb after it acts on it as on any held screen.
The state is one optional argument of a closed value set per app, on the carriers start already rides (the command line start {task} --state {name}, the MCP start tool, the socket's hello), checked by the driver's schema before anything runs; a name the app's table does not hold is refused with nothing started, no panic path, nothing the caller typed written back.
A kept check recorded from a session started in a state replays from that state, or such a recording is refused (a fork P4 settles).
The served seven_guis tasks name at least the states a stand check today reaches by clicking from boot, building on the shared test fixtures' boot and hold rather than a second boot path.
The proof: for each named state, the first screen of a session started in it reads (snapshot text, by answer digest or whole) as the same app's screen reached by hand from boot through the driver's own verbs, in both layout modes.
@task_file n=1 path=packages/escher-driver/src/schema.rs mark=edits
what: the driver schema, the one statement of its verbs, their arguments, refusals and exit statuses — edits: START, the verb table, describe()
@task_file n=2 path=packages/escher-driver/src/command.rs mark=edits
what: the driver's parsed command, the verb and arguments a call carries into a session
@task_file n=3 path=packages/escher-driver/src/cli.rs mark=edits
what: the escher-session command line role, its arguments, stdout answers, usage errors and exit codes
@task_file n=4 path=packages/escher-driver/src/mcp.rs mark=edits
what: the escher-session mcp role, its tools (the start tool) and their results
@task_file n=5 path=packages/escher-driver/src/session.rs mark=edits
what: the driver session, its start, the held screen and its lifecycle
@task_file n=6 path=packages/escher-driver/src/host.rs mark=edits
what: the session host, the boot of a served task behind the driver
@task_file n=7 path=packages/escher-driver/src/wire.rs mark=edits
what: the socket's wire messages between client and session host — edits: hello
@task_file n=8 path=packages/escher-driver/src/recording.rs mark=edits
what: the kept check (--record), its closed key set and at-rest content — edits: Check::from_bytes, the closed key set
@task_file n=9 path=packages/escher-driver/src/replay.rs mark=edits
what: the replay of a kept check from its recorded start
@task_file n=10 path=packages/escher-driver/src/coverage.rs mark=edits
what: the coverage line and coverage check files — edits: a check's replay boot
@task_file n=11 path=examples/seven_guis/src/stand.rs mark=edits
what: the stand's served tasks (counter, timer, crud, flight-booker, cells, load) — edits: the table of each app's named states
@task_file n=12 path=examples/seven_guis/src/session_host.rs mark=edits
what: the served tasks' boot behind escher-session
@task_file n=13 path=tests/blitz-tests/tests/fixtures/mod.rs mark=edits
what: the shared fixtures of the checks beside the stand — boot, hold, pages and the held network stand-in
@task_file n=14 path=tests/blitz-tests/tests/session_common/mod.rs mark=edits
what: the shared helpers of the stand's session checks
@task_file n=15 path=tests/blitz-tests/tests/stand_start_in_state.rs
what: the new stand check: a session started in each named state reads as that state reached by hand from boot, in both layout modes
@task_file n=16 path=.github/scripts/test_blitz_tests_targets.py mark=edits
what: the blitz-tests target pin, the list of test targets the crate holds
@task_file n=17 path=docs/capabilities.md mark=edits
what: the capability statement, its verb and argument counts
@task_file n=18 path=.github/scripts/test_capability_statement.py mark=edits
what: the pin of the capability statement's verb and argument counts
@task_item n=1 part=needs
The escher-session start command and its arguments: a new optional state argument naming one of the task's states, on the command line, the MCP start tool and the socket's hello.
@task_item n=2 part=needs
The escher-session tasks (counter, timer, crud, flight-booker, cells, load): each served task now names a closed set of states a session can start in, and the unknown-task refusal stands beside a new unknown-state refusal.
@task_item n=3 part=needs
A state name the task's table does not hold is refused before anything starts: a usage error on stderr with its exit code and status from the schema's table, no panic path, and nothing the caller typed echoed back in the refusal or its remedy.
@task_item n=4 part=needs
The help answer and the MCP help tool result describe the new argument and its closed value set, read from the schema.
@task_item n=5 part=needs
The MCP start tool's result and Err ending for a refused state, and the mcp role's stdout staying one line per result.
@task_item n=6 part=needs
The session's first screen is the named state's: the snapshot (text and parts), the actions line, the UI graph line and the coverage line read it as any held screen, and every later verb acts on it.
@task_item n=7 part=needs
The session lifecycle: start in a state then the same verbs, settle and close as a session started at boot.
@task_item n=8 part=needs
The virtual clock of a session started in a state: reaching the state moves no clock the screen reached by hand from boot would not, so advance, click, press and type act on it alike.
@task_item n=9 part=needs
The kept check (--record) of a session started in a state: its closed key set and at-rest content carry the state name and replay from it, or the recording is refused; coverage check files' read bound and refusal hold for the new content.
@task_item n=10 part=needs
The driver's command line output stays agent-read: one line per answer, no colour, nothing the caller supplied written back.
@task_item n=11 part=needs
No new socket, port, listener, environment read, log field or span field; the command span records no state name.
@task_item n=12 part=needs
The proof's measure: snapshot text of the first screen, compared by answer digest or whole, against the same app reached by hand from boot through the driver's own verbs, in both layout modes.
@task_item n=13 part=needs
The capability statement's verb and argument counts, and their pin, move with the new argument.
@task_standing n=1
convention: each new stand check reaches a served task only through the driver's own verbs and reads the session's answers (snapshot, actions, exit status) as an agent would
home: surface
@task_standing n=2
convention: each new refusal the command line role answers is a usage error or session error with an exit code from the schema's table and a remedy, writing back nothing the caller typed
home: surface-cli
@task_item n=1 part=not_needed
Colour: engine colours, select list colour pairs, the viewport's color scheme and colour tokens; the chunk adds no colour and changes none.
@task_item n=2 part=not_needed
Typography and fonts: font faces, fallback, system or bundled fonts, font contexts and the screenshot's fonts-unavailable refusal; a state changes no font.
@task_item n=3 part=not_needed
Spacing and depth strategy: margins, cell spacing, z-index paint levels, stacking contexts and hit-testing at paint clips are engine behaviour the chunk does not change.
@task_item n=4 part=not_needed
The select list as an engine overlay: its open gestures, list events, rows in the snapshot and screenshot, colours and scroll.
@task_item n=5 part=not_needed
Scrolling motion: driver scroll, into-view scroll, smooth scroll-behavior, overlay scrollbar fade and thumb drag.
@task_item n=6 part=not_needed
Iconography: cursor icons, cursor resolution and favicons.
@task_item n=7 part=not_needed
The shell provider's window controls (maximize, minimize, decorations, drag, set_cursor), the dioxus-native window alpha mode and the JS window object's sizes and pixel ratio.
@task_item n=8 part=not_needed
The WPT runner's output and result colours, mobile targets and the web-spa surface of WASM pages.
@task_item n=9 part=not_needed
Auto-exploration, a seed on the driver's carriers, and a component started alone with props: out of this chunk.
