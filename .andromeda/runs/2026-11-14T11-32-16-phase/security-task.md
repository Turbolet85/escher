@task_prose part=chunk
A driver session can start in a named fixture state of its app instead of at boot: the app's own table names its states, the session's first screen is that state's, and every verb after it acts on it as on any held screen.
The state is one optional, bounded, validated argument of a closed value set per app, carried where start already rides (the command line start, the MCP start tool, the session socket's hello), checked by the driver schema before anything runs; an unknown name is refused with nothing started, no panic, nothing typed echoed.
A kept check recorded from a session started in a state either replays from that state (its start record and strict reader take the state name) or is refused; which one is a fork left to P4.
The served tasks of the stand name at least the states a stand check today reaches by clicking from boot, built on the shared test fixtures' boot and hold rather than a second boot path.
The proof: a new stand check reads, for each named state, the first screen of a session started in it as the same app's screen reached by hand from boot through the driver's verbs, in both layout modes.
@task_file n=1 path=packages/escher-driver/src/schema.rs mark=edits
what: the closed driver verb schema and the single validation gate, its argument kinds and the verb listing — edits: START, the verb table, describe()
@task_file n=2 path=packages/escher-driver/src/command.rs mark=edits
what: the driver command, its parse and validation into a closed call, and the command span
@task_file n=3 path=packages/escher-driver/src/cli.rs mark=edits
what: the escher-session command line, its closed argv reader, flags and roles
@task_file n=4 path=packages/escher-driver/src/mcp.rs mark=edits
what: the MCP server over stdio, its tool listing, tool arguments and results
@task_file n=5 path=packages/escher-driver/src/session.rs mark=edits
what: the driver session, its start and the held screen every verb acts on
@task_file n=6 path=packages/escher-driver/src/host.rs mark=edits
what: the session host behind the session socket, its boot of the app and its idle expiry
@task_file n=7 path=packages/escher-driver/src/wire.rs mark=edits
what: the session socket wire, its closed request grammar (hello, call, stop) and its replies
@task_file n=8 path=packages/escher-driver/src/recording.rs mark=edits
what: the recorder of kept checks, the recorded check file and its start record
@task_file n=9 path=packages/escher-driver/src/replay.rs mark=edits
what: replay of a kept check, its strict reader of untrusted check bytes and each step run through the session
@task_file n=10 path=packages/escher-driver/src/coverage.rs mark=edits
what: the coverage role reading kept checks as untrusted and replaying them — edits: a check's replay boot
@task_file n=11 path=examples/seven_guis/src/stand.rs mark=edits
what: the stand's served tasks — edits: the served tasks and their named states
@task_file n=12 path=examples/seven_guis/src/session_host.rs mark=edits
what: the served tasks' session host and their boot
@task_file n=13 path=tests/blitz-tests/tests/fixtures/mod.rs mark=edits
what: the shared fixtures of the checks beside the stand: boot, hold, pages and the held network stand-in
@task_file n=14 path=tests/blitz-tests/tests/session_common/mod.rs mark=edits
what: the shared session helpers of the stand's session checks
@task_file n=15 path=tests/blitz-tests/tests/stand_start_in_state.rs
what: a new stand check: a session started in each named state reads as that state reached by hand from boot, in both layout modes
@task_file n=16 path=.github/scripts/test_blitz_tests_targets.py mark=edits
what: the blitz-tests target pin, the list of test targets
@task_file n=17 path=docs/capabilities.md mark=edits
what: the committed capability statement, including the MCP crossing, where verb or argument counts move
@task_file n=18 path=.github/scripts/test_capability_statement.py mark=edits
what: the pin of the committed capability statement
@task_item n=1 part=needs
The closed driver verb schema and its single validation gate: the state is a new optional argument of start, of a closed value set per app, checked before anything runs, the same one validation path for every carrier.
@task_item n=2 part=needs
The driver argument kinds: the state name is a bounded name of a closed set; a length bound and closed match on what the call supplies.
@task_item n=3 part=needs
An unknown or malformed state name is refused with nothing started, through the closed refusal cause set and the refusal JSON, the refusal holding nothing the call supplied.
@task_item n=4 part=needs
No panic on call-supplied input and no echo of caller input: nothing typed for the state is written back on stdout, stderr, a refusal, a session error or a reply.
@task_item n=5 part=needs
The command line carrier: the closed argv reader and the escher-session flags gain the state flag on start; its usage messages and exit codes stay a closed set.
@task_item n=6 part=needs
The MCP carrier: the start tool's arguments and value kinds, the fixed-text tool listing, bounded requests and results, protocol errors fixed with no data.
@task_item n=7 part=needs
The session socket carrier: the hello request of the closed grammar carries the state, with its bounds, version and refused-malformed replies.
@task_item n=8 part=needs
The boundary widening rule and the closed-value field carve-out: whether a new argument on a ratified carrier widens a boundary is a fork for P4, its most conservative option provisional, recorded for the founder.
@task_item n=9 part=needs
The command span and the tool_call span stay content-free: no state name in a span or log line; no new log field, span field, env read, socket, port or listener; one command span per call.
@task_item n=10 part=needs
The kept check: its start record, closed key set and strict untrusted reader either take the state name or refuse a recorded session started in a state; its contents and at-rest class change only as P4 rules; coverage refuses an unusable check as malformed.
@task_item n=11 part=needs
Replay runs each step through the session from the recorded start state, on a replay boot that starts in that state.
@task_item n=12 part=needs
The session lifecycle: the session host boots the app in the named state, the held screen is that state's, and the host's idle expiry, socket ownership and one bounded connection hold as before.
@task_item n=13 part=needs
The served stand tasks gain named states (an app-supplied initial state or steps from boot, settled by P4), in-process only, never from a caller beyond the closed name.
@task_item n=14 part=needs
The snapshot text the proof reads (state words, id and name fields, masked values) and the stable element id, which must read the same on a state started and a state reached by hand.
@task_item n=15 part=needs
The committed capability statement of the MCP crossing and its pin move where the verb or argument count moves.
@task_item n=16 part=needs
The session errors from a fixed table and the escher-session exit codes for a start refused on its state.
@task_standing n=1
convention: each new stand check runs under the locked test harness, takes no tracing and no style, and joins the blitz-tests target pin
home: dependency-security
@task_standing n=2
convention: each new check's kept checks and recordings stay owner-only and uncommitted, and its output holds no caller input or screen content
home: data-protection · logging-monitoring
@task_standing n=3
convention: each new refusal or error a test asserts is a cause from the closed set, logged by cause name only
home: error-handling
@task_item n=1 part=not_needed
Outbound HTTP, script fetches, CORS, same-origin, response limits and request timeouts: starting in a state fetches nothing.
@task_item n=2 part=not_needed
Engine rows on CSS parsing, layout, hit testing, painting, fonts and HTML serialization: no change to the engine is expected.
@task_item n=3 part=not_needed
Select list, range input and password painting behaviour rows: the chunk changes no control's interaction or painting.
@task_item n=4 part=not_needed
The browser app's history store, cookies, HTTP cache, TLS, clipboard, file dialog and JavaScript opt-in: not reached.
@task_item n=5 part=not_needed
CI secrets, signing material, release publishing and the repository guards of the publish job: not reached.
@task_item n=6 part=not_needed
The cold-agent and agent-run scripts, their prompts, recordings and events: not reached.
@task_item n=7 part=not_needed
The WPT runner, its results, quarantine and post-results workflow: not reached.
@task_item n=8 part=not_needed
Auto-exploration, a seed on the driver's carriers and a component alone with props: out of this chunk.
