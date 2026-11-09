@task_prose part=session
The session added handler coverage: one command lists the UI graph's handlers that no recorded check exercised.
dioxus-native-dom gained an ungated reached record (new module reached: ReachedHandler, record_reached, reached_handlers, crate-private ReachedRecord), written in DioxusEventHandler::handle_event beside the unchanged SignalRegistry arm, and a crate-private ungated reader element_listener_ids.
escher-driver gained the coverage module (handler_coverage, Coverage, HandlerRef, Coverage::to_json) and a third command-line role, coverage <app> <check>..., in the escher-session binary; Check::replay now runs through a crate-private replay_each.
A kept actions step now replays under test (stand_replay 3 to 4 tests), discharging the owed actions replay; a new stand target stand_handler_coverage (8 tests) brings the stand reading to 262 passed, 0 failed, 7 ignored over 54 files.
Measured: no log site, span, event, env read, clock read or print added; escher-driver keeps 2 info_span! sites; the host_log coverage run reads 0 needles at trace under both builds; no log field, span field or scrub set moved; nothing retired.
@task_file n=1 path=packages/dioxus-native-dom/src/reached.rs
@task_file n=2 path=packages/escher-driver/src/coverage.rs
@task_file n=3 path=tests/blitz-tests/tests/stand_handler_coverage.rs
@task_file n=4 path=packages/dioxus-native-dom/src/dioxus_document.rs
@task_file n=5 path=packages/dioxus-native-dom/src/dioxus_document_tests.rs
@task_file n=6 path=packages/dioxus-native-dom/src/element_id.rs
@task_file n=7 path=packages/dioxus-native-dom/src/lib.rs
@task_file n=8 path=packages/escher-driver/src/replay.rs
@task_file n=9 path=packages/escher-driver/src/lib.rs
@task_file n=10 path=packages/escher-driver/src/json.rs
@task_file n=11 path=packages/escher-driver/src/cli.rs
@task_file n=12 path=packages/escher-driver/src/cli_tests.rs
@task_file n=13 path=examples/seven_guis/src/session_host.rs
@task_file n=14 path=examples/seven_guis/tests/cli_commands.rs
@task_file n=15 path=examples/seven_guis/tests/common/mod.rs
@task_file n=16 path=examples/seven_guis/tests/host_log.rs
@task_file n=17 path=tests/blitz-tests/tests/stand_replay.rs
@task_file n=18 path=tests/blitz-tests/tests/common/mod.rs
@task_file n=19 path=docs/capabilities.md
@task_item n=1 part=touches
The count of checks that read the stand's session_common shared module moves from thirty-four to thirty-five: stand_handler_coverage is the new reader (the module thirty-four checks read, in the stand silence fact of section 6).
@task_item n=2 part=touches
The count of stand checks that read the shared module tests/blitz-tests/tests/common/mod.rs moves by one (blitz-tests mod common declarers 23 to 24), stand_handler_coverage the new reader; that module gains HANDLERS_AT_BOOT, UNEXERCISED_AFTER_DRIVES and grid_cells, adding no print, log or env read.
@task_item n=3 part=touches
The escher-session logging-stack fact in section 3 names the command line's roles (the host role and the mcp role); a third role, coverage, now runs in the same main under the same single install, calling the same boot closure once per check, installing no subscriber and reading no env var.
@task_item n=4 part=touches
The escher-driver silence statement listing its modules (recording the fifteenth, replay the sixteenth) gains a new module, coverage, and replay gains the crate-private replay_each with an after-step hook; the escher-driver print set (command line answers on stdout, usage on stderr) gains the coverage line and its refusals.
@task_item n=5 part=touches
The emitter fact for examples (escher-session's own lines are its one eprintln! and the endings its command line writes) now covers the coverage role's endings: the coverage line on stdout status 0, malformed status 1, usage status 2, a session error status 3, none holding anything the caller typed or any path.
@task_item n=6 part=touches
Section 6 facts that say the bridge sets the registry's dispatch around runtime.handle_event still hold; the reached record now sits beside it on the same path, written before runtime.handle_event where the root context holds, adding no log site.
@task_item n=7 part=touches
The escher-driver span facts re-read unmoved: no new span or field; info_span! stays at 2 sites; each step a coverage run replays leaves its own command span through Session::run, so a coverage run over a sink-installing binary at info writes command span lines and nothing else of its own.
@task_item n=8 part=touches
The host_log statements that name its tests and their ordinals (the fourth test on cells, and others) gain a new test: host_log tests 5 to 6, the new one a coverage run at trace.
@task_item n=9 part=touches
The seven_guis shared test module's statement of the log checks' needle readers (Needles, stand_needles, stamped_lines, found, occurrences, recorded_needles, actions_needles) gains coverage_needles, called by host_log alone; any line citation into that module's needle range may have shifted.
@task_item n=10 part=touches
The session-recording silence fact for stand_replay (replaying the two committed checks) now also covers a recorded Counter session with two actions steps replayed in both layout modes and a changed copy stopping at step 3; the committed checks themselves are unchanged.
@task_item n=11 part=touches
The cli_commands target (counted 17 on linux in other masters) gains one test, a coverage run through the binary, and its usage-row table one row; any section 6 mention of cli_commands spawning the sink-installing binary now includes the coverage role.
@task_item n=12 part=touches
The stand-target count that the silence probes span moves: stand_*.rs files 53 to 54; the run stand reading moves to 262 passed, 0 failed, 7 ignored over 54 files.
@task_item n=13 part=touches
The section 8 statement that the sink's lines read 0 id, name and typed-text needles in the escher-session host now also holds over a coverage run at trace (0 id, name, coverage, sentinel, path and recorded needles).
@task_item n=14 part=touches
The local-state statement in section 9 (recording directories and two committed kept checks) re-read unmoved: the committed checks are byte-identical, and the coverage role only reads kept checks the caller names; it writes no file.
@task_item n=15 part=touches
The dioxus-native-dom crate-root re-exports gain ReachedHandler (35 to 36) and escher-driver's gain Coverage, HandlerRef and handler_coverage (59 to 62); any obs-plan line naming those crates' public types or modules is a site.
@task_fact n=1 slug=log-coverage
fact: dioxus-native-dom's ungated module reached (ReachedHandler, record_reached, reached_handlers, crate-private ReachedRecord, an Arc<Mutex<Vec>> at the root scope's context) and the crate-private element_listener_ids hold no log site, print, env read or failure path; the record holds stable ids and event names only, no NodeId, and ReachedRecord's Debug prints its count alone.
@task_fact n=2 slug=log-coverage
fact: escher-driver's module coverage (handler_coverage, Coverage, HandlerRef) holds no span, event, env read, clock read or print; Coverage::to_json writes {"unexercised":[{"id","owner","event"}],"checks":n,"held":m} by the hand writer as a command line answer, owner left out when absent, serde_json writing nothing, never logged or fielded.
@task_fact n=3 slug=log-coverage
fact: the escher-session coverage role (coverage <app> <check>..., no --session or --record) reads each named kept check through a bounded read of 1,048,576 + 1 bytes and Check::from_bytes, writes one uncoloured JSON coverage line on stdout, and refuses an unreadable, over-bound, non-check or other-app file as malformed holding nothing of the path or file.
@task_fact n=4 slug=log-coverage
fact: host_log's a_coverage_run_logs_no_id_no_name_and_nothing_of_its_check_at_trace runs coverage crud over a CRUD session kept through the binary with a typed sentinel and a row click, reading the host's stderr at trace: 0 id, name, coverage, sentinel, path and recorded needles, green under the package and the workspace builds.
@task_fact n=5 slug=log-coverage
fact: stand_handler_coverage (8 tests, both layout modes, a common and session_common reader) installs no subscriber, reads no env var and prints nothing, its readings going into assertions only; the silence census reads last line 0 over the three new files of the chunk.
@task_fact n=6 slug=pii-scrubbing-compliance
fact: the coverage line holds stable ids, owner ids, handler event names and two counts only; a typed sentinel occurs 0 times in it (stand_handler_coverage a_typed_sentinel_never_reaches_the_coverage_line).
@task_item n=1 part=untouched
escher-telemetry's sink, its allowlist formatter, the engine and escher target prefix sets, the content-named redaction set and the panic hook: unchanged, no log field or scrub set moved.
@task_item n=2 part=untouched
The MCP server, the tool_call span and the cold-agent pipe and its server.log counting: no socket, MCP or cold-agent surface moved, VERBS still fourteen verbs.
@task_item n=3 part=untouched
The agent-run harness log format and its events: no change to the harness scripts or CI legs; a new stand target is picked up by its prefix only.
@task_item n=4 part=untouched
Section 2 telemetry strategy and feature wiring, section 5 metric coverage, section 7 error capture: no dependency, manifest, Cargo.lock or feature changed.
@task_item n=5 part=untouched
Engine and upstream emitters (blitz-dom, blitz-net, blitz-html, blitz-shell, blitz-paint, dioxus-native, blitz-vibey-script), the upstream apps' stdout subscribers and the WPT runner's env_logger: untouched.
@task_item n=6 part=untouched
Section 9 CI artifact handling and the apt-install diagnostic: no workflow, leg list or artifact changed.
@task_item n=7 part=untouched
Sections 10, 11 and 12 (no recorded intent) and the NOT YET MEASURED notes: the session measures none of their subjects.
