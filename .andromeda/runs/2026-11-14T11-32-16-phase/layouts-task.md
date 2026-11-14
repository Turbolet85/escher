@task_prose part=chunk
Lets a driver session start in a named fixture state of its app instead of at boot: the app's own table of served tasks names its states, and the session's first screen is that state's.
The state is one optional, bounded, validated argument of a closed value set per app, carried where start already rides: the command line, the MCP start tool and the socket hello.
A name the app does not hold is refused by the driver schema before anything runs, with nothing started and nothing the caller typed written back.
A kept check recorded from a session started in a state replays from that state, or such a recording is refused (a fork P4 settles).
A new stand check proves, for each named state and in both layout modes, that the first screen's snapshot text reads as the same screen reached by hand from boot through the driver's own verbs.
@task_file n=1 path=packages/escher-driver/src/schema.rs mark=edits
what: the driver schema, the verb table and its description of each verb — edits: the START entry, the verb table, describe()
@task_file n=2 path=packages/escher-driver/src/command.rs mark=edits
what: the driver's command form, the verbs and their arguments as one command
@task_file n=3 path=packages/escher-driver/src/cli.rs mark=edits
what: the escher-session command line, its argument parsing and usage
@task_file n=4 path=packages/escher-driver/src/mcp.rs mark=edits
what: the escher-session MCP server over stdio, its tools and the start tool's arguments
@task_file n=5 path=packages/escher-driver/src/session.rs mark=edits
what: the driver session, its start and the held screen every verb acts on
@task_file n=6 path=packages/escher-driver/src/host.rs mark=edits
what: the session host that boots the served task and holds its instance
@task_file n=7 path=packages/escher-driver/src/wire.rs mark=edits
what: the socket wire between the client and the host, its hello message
@task_file n=8 path=packages/escher-driver/src/recording.rs mark=edits
what: the session recording that writes a kept check, its closed key set
@task_file n=9 path=packages/escher-driver/src/replay.rs mark=edits
what: the replay of a kept check, reading a check file back from bytes
@task_file n=10 path=packages/escher-driver/src/coverage.rs mark=edits
what: the coverage role, which replays a kept check from its boot — edits: a check's replay boot
@task_file n=11 path=examples/seven_guis/src/stand.rs mark=edits
what: the stand app's served tasks, the table of tasks a session can start
@task_file n=12 path=examples/seven_guis/src/session_host.rs mark=edits
what: the stand's session host boot, how a served task is booted for a session
@task_file n=13 path=tests/blitz-tests/tests/fixtures/mod.rs mark=edits
what: the shared stand fixtures, boot, hold, pages and the held network stand-in, the one statement of what the checks beside the stand share
@task_file n=14 path=tests/blitz-tests/tests/session_common/mod.rs mark=edits
what: the shared helpers the session checks use to start and drive a session
@task_file n=15 path=tests/blitz-tests/tests/stand_start_in_state.rs
what: the new stand check that a session started in a named state reads as that state reached by hand from boot
@task_file n=16 path=.github/scripts/test_blitz_tests_targets.py mark=edits
what: the blitz-tests target pin, the list of test targets the suite must hold
@task_file n=17 path=docs/capabilities.md mark=edits
what: the capability statement, its verb and argument counts
@task_file n=18 path=.github/scripts/test_capability_statement.py mark=edits
what: the pin of the capability statement's counts
@task_item n=1 part=needs
The escher-session command line form gains one optional argument on start, start {task} --state {name}; its usage and help answer name it.
@task_item n=2 part=needs
The escher-session MCP server's start tool gains the same optional state argument, so its tools list changes; the server stays local over stdio.
@task_item n=3 part=needs
The socket hello carries the state name to the host as one optional field of a closed value set; whether a new argument on a ratified carrier is a boundary widening, and the carrier set, is a fork P4 settles, the most conservative option provisional.
@task_item n=4 part=needs
The state argument is checked by the driver schema before anything runs: a name the app's table does not hold is refused with nothing started, under the escher-session refusals and exit codes, and no panic path is reachable from what the call supplies.
@task_item n=5 part=needs
Caller input is never written back: the refusal of an unknown state name, and every answer, carries no text the caller typed.
@task_item n=6 part=needs
No new socket, port, listener, env read, log field or span field; the command span records no state name, as a call-supplied argument.
@task_item n=7 part=needs
The escher-session served tasks (cells, counter, crud, flight-booker, load, timer) each name their states in the app's own table, at least the states a stand check today reaches by clicking from boot.
@task_item n=8 part=needs
A session started in a state holds that state's screen as its first screen, and every verb after it acts on it as on any held screen; the held instance still reads as a fresh boot of that state.
@task_item n=9 part=needs
The session roles share the stand boot: a state is reached on the shared fixtures' boot and hold, never a second boot path; whether states live in the served app, in the test fixtures, or both, is left to P3.
@task_item n=10 part=needs
A session recording is a replayable check: a kept check of a session started in a state replays from that state through its check file's closed key set and its reading back from bytes, or such a recording is refused; which, is a fork P4 settles.
@task_item n=11 part=needs
The escher-session coverage role replays a check file from its boot, so a check recorded in a state is replayed from that state, or refused as the recording is.
@task_item n=12 part=needs
The proof reads the escher-session snapshot answer (snapshot text, by answer digest or whole) of the first screen in a state against the screen reached by hand from boot through the driver's own verbs, in both layout modes, under the headless stand's pinned viewport.
@task_item n=13 part=needs
The new check file joins the blitz-tests target pin, and the capability statement's verb and argument counts, with their pin, move where the new argument moves them.
@task_item n=14 part=needs
No change to the engine is expected; the state is supplied at the driver and served-app layer only.
@task_standing n=1
convention: each new stand check boots through the shared stand boot and its held instance, never a boot of its own
home: surface-desktop-native
@task_standing n=2
convention: each new driver argument is bounded and validated before anything runs, its refusals closed names with their exit codes, and nothing the caller typed written back
home: surface-cli
@task_item n=1 part=not_needed
The ShellProvider window chrome controls (drag, maximize, minimize, close, decorations): a session's start state touches no window chrome.
@task_item n=2 part=not_needed
The agent-run.sh test contract and the cold-agent.sh run pipe, with the cold-agent decisions on prompts and pipes: auto-exploration and agent runs are out of this chunk.
@task_item n=3 part=not_needed
The multi_doc and paint_bench command lines and their environment knobs: neither is the session driver.
@task_item n=4 part=not_needed
The stand's accent colour, scroll-into-view, visible region through clipping boxes, open list rows' bounds and the CRUD list's keyboard region: layout and interaction behaviour the chunk does not change.
@task_item n=5 part=not_needed
The escher-session screenshot answer and the screenshot result of the MCP server: the proof reads snapshot text, not pixels.
@task_item n=6 part=not_needed
The absence of arbitrary index.html templates and the fixed document skeleton: a state changes no document skeleton.
@task_item n=7 part=not_needed
The mobile-native and web-spa surfaces: unmeasured and not reached by a driver session's start.
