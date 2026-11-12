@row_none n=1 rec=1 type=5
@row_none n=2 rec=2 type=5
@row_none n=3 rec=3 type=5
@row_none n=4 rec=4 type=5
@row_add n=1 row=decision/graph-answer-holds-no-value area=integration-test-strategy rec=5
name: Graph answer holds no value
rule: A graph answer carries ids, component names, handler event names, state names, closed flag words and booleans only, never a value such as typed text.
guard: stand_graph, cli_commands, mcp_server, stand_mcp_tools
@row_add n=2 row=contract/graph-command/argument area=integration-test-strategy rec=5
boundary: the graph command
item: argument
promise: An argument to graph is refused malformed with nothing run.
@row_add n=3 row=decision/actions-answer-holds-no-value area=integration-test-strategy rec=6
name: Actions answer holds no value
rule: An actions answer carries ids, owner ids, the table's verb words, cause words and booleans only, never a value, never a masked value and no internal node id field.
guard: stand_actions
@row_add n=4 row=contract/actions-command/argument area=integration-test-strategy rec=6
boundary: the actions command
item: argument
promise: An argument to actions is refused malformed.
@row_add n=5 row=decision/session-state-owner-only area=integration-test-strategy rec=7
name: Owner-only session state
rule: The session state directory is created with mode 0700 and its socket with mode 0600.
guard: stand_session_lifecycle
@row_add n=6 row=contract/session-errors/nosession area=integration-test-strategy rec=7
boundary: session errors
item: NoSession
promise: Returned when a session is reached and none is running.
@row_add n=7 row=contract/session-errors/alreadyrunning area=integration-test-strategy rec=7
boundary: session errors
item: AlreadyRunning
promise: Returned by start while a session is already up.
@row_add n=8 row=contract/session-errors/dead area=integration-test-strategy rec=7
boundary: session errors
item: Dead
promise: Returned after the session host has been killed.
@row_add n=9 row=contract/session-errors/unsupported area=integration-test-strategy rec=7
boundary: session errors
item: Unsupported
promise: Returned by the session arms on non-unix platforms.
@row_add n=10 row=decision/session-refuses-out-of-grammar area=integration-test-strategy rec=7
name: Out-of-grammar requests refused
rule: A session request outside the grammar or over the bound is refused with nothing run.
guard: stand_session_lifecycle
@row_add n=11 row=contract/session-protocol/hello area=integration-test-strategy rec=7
boundary: the session protocol
item: hello
promise: hello answers idle_expiry_s.
@row_add n=12 row=contract/session-status/served area=integration-test-strategy rec=7
boundary: the session status
item: served
promise: Every answered request, a refused or malformed one included, counts toward served and restarts the idle count.
@row_add n=13 row=decision/session-idle-expiry area=integration-test-strategy rec=7
name: Session idle expiry
rule: A session host that answers no request for its idle expiry ends exactly as on stop.
guard: stand_session_lifecycle
@row_add n=14 row=contract/cli-exit-codes/3 area=integration-test-strategy rec=7
boundary: CLI exit codes
item: 3
promise: A command that finds no session ends as a session error, status 3.
@row_add n=15 row=decision/session-logs-no-screen-content area=integration-test-strategy rec=8
name: Session logs nothing of the screen
rule: The session library, the session host and its mcp role write no typed text, stable id, accessible name or shot bytes to stdout, stderr or the state directory, at any log level.
guard: stand_session_quiet, host_log, mcp_log
@row_add n=16 row=decision/sink-drops-third-party-targets area=integration-test-strategy rec=8
name: Sink drops third-party targets
rule: The escher log sink writes nothing for a third-party target at any level, native and bridged, even under a RUST_LOG directive naming it.
guard: telemetry_drop
@row_add n=17 row=contract/session-state-directory/session-sock area=integration-test-strategy rec=8
boundary: the session state directory
item: session.sock
promise: The state directory holds session.sock alone; a kept recording lives in the launcher's recording directory.
@row_add n=18 row=contract/session-host-stderr/service-name area=integration-test-strategy rec=8
boundary: the session host's stderr
item: service.name
promise: Every stderr line carries service.name=seven_guis.
@row_add n=19 row=contract/session-host-stderr/command-span area=integration-test-strategy rec=8
boundary: the session host's stderr
item: command span
promise: Exactly one command span line is written per call.
@row_add n=20 row=contract/mcp-role-stdout/protocol-lines area=integration-test-strategy rec=8
boundary: the mcp role's stdout
item: protocol lines
promise: The mcp role writes protocol lines only on stdout.
@row_add n=21 row=contract/mcp-role-stderr/tool-call area=integration-test-strategy rec=8
boundary: the mcp role's stderr
item: tool_call
promise: One tool_call line per call, carrying only its stated field names.
@row_none n=5 rec=9 type=5
@row_none n=6 rec=10 type=5
@row_none n=7 rec=11 type=5
@row_none n=8 rec=12 type=5
@row_none n=9 rec=13 type=5
@row_none n=10 rec=14 type=5
@row_none n=11 rec=15 type=5
@row_none n=12 rec=16 type=5
@row_add n=22 row=decision/listeners-read-from-vdom-walk area=integration-test-strategy rec=17
name: Listeners read from the vdom walk
rule: A node's current listeners are read from the vdom walk's dynamic attributes, mounted included, not from the bridge's listener attributes.
why: The bridge's listener attributes over-report: a listener removed on a re-render leaves its attribute standing, and a mounted listener writes none.
guard: dioxus_runtime_probe link_2_a_removed_listeners_attribute_stays
@row_add n=23 row=decision/graph-signal-registration area=integration-test-strategy rec=18
name: Task state registered for the UI graph
rule: The stand registers each task's signals through a graph-state hook subscribed to the signal, so writers and reading components reach the UI graph through the fork's signal registry, with no dioxus-core change.
why: Hook-local signal state exposes neither link without a fork-level change, the scope's hooks being crate-private.
guard: graph::tests, stand_graph
@row_none n=13 rec=19 type=7
@row_none n=14 rec=20 type=5
@row_none n=15 rec=21 type=5
@row_none n=16 rec=22 type=5
@row_none n=17 rec=23 type=5
@row_none n=18 rec=24 type=5
@row_none n=19 rec=25 type=5
@row_none n=20 rec=26 type=5
@row_none n=21 rec=27 type=5
@row_add n=24 row=decision/stand-boots-offline area=mocking-stubbing-discipline rec=28
name: Headless stand boots offline
rule: Every headless-stand boot keeps no network provider, and no real network is used by the stand.
guard: the_stand_options_stay_offline
except: The load page's boot uses a private stand-in that answers one fixed sheet from its own thread after a delay.
@row_none n=22 rec=29 type=5
@row_none n=23 rec=30 type=5
@row_none n=24 rec=31 type=5
@row_none n=25 rec=32 type=5
@row_none n=26 rec=33 type=5
@row_none n=27 rec=34 type=5
@row_none n=28 rec=35 type=5
@row_none n=29 rec=36 type=5
@row_none n=30 rec=37 type=5
@row_none n=31 rec=38 type=5
@row_none n=32 rec=39 type=5
@row_add n=25 row=decision/timer-driven-by-delivered-ticks area=mocking-stubbing-discipline rec=40
name: Timer driven by delivered ticks
rule: In every headless-stand boot the Timer's real delay is replaced by delivered 100 ms ticks applied on the next pass; settle neither advances the timer nor waits on an undelivered tick.
guard: stand_timer, stand_settle
@row_add n=26 row=decision/one-virtual-clock area=mocking-stubbing-discipline rec=40
name: One virtual clock
rule: The session's clock is the harness's virtual clock, the one every harness-built document reads, and advance moves it by exactly ms inside one settled step.
guard: stand_act_timer
@row_add n=27 row=decision/advance-reports-clock-move area=mocking-stubbing-discipline rec=40
name: advance reports the clock's move
rule: advanced_ms is exactly the ms asked, the clock's move, not what the Timer's counter gained.
why: Ratified.
@row_none n=33 rec=41 type=5
@row_none n=34 rec=42 type=5
@row_none n=35 rec=43 type=5
@row_none n=36 rec=44 type=5
@row_none n=37 rec=45 type=5
@row_add n=28 row=decision/font-metric-tests-never-skip area=mocking-stubbing-discipline rec=46
name: Font-metric tests never skip
rule: A font-metric test asserts its text-size precondition and fails rather than skipping when text measures 0x0.
why: The skip came from the fixture's no-namespace element, not from missing system fonts.
guard: hover_state_tests
@row_add n=29 row=absence/mocking-library area=mocking-stubbing-discipline rec=47,48
capability: A mocking library
@row_none n=38 rec=49 type=7
@row_none n=39 rec=50 type=7
@row_none n=40 rec=51 type=7
@row_none n=41 rec=52 type=7
@row_none n=42 rec=53 type=7
@row_none n=43 rec=54 type=7
@row_none n=44 rec=55 type=5
