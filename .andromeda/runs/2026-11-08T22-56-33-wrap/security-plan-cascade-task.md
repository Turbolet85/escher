@task_prose part=session
Added a third escher-session command-line role, coverage <app> <check>..., that reads kept-check files the caller names on argv and prints one JSON line listing the UI graph handlers no check exercised.
Each path is read under a byte bound (MAX_CHECK_BYTES + 1, 1,048,576 + 1) through the strict Check::from_bytes; an unreadable, over-bound, non-check or other-app file is refused malformed (status 1) holding nothing of the path; a boot failure is status 3, usage status 2, accepted status 0.
Added escher_driver::handler_coverage, Coverage and HandlerRef (crate re-exports 59 to 62), and in dioxus-native-dom an ungated reached record (ReachedHandler, record_reached, reached_handlers, re-exports 35 to 36) written on the bridge's handle_event path beside the SignalRegistry dispatch.
The coverage line holds ids, owner ids, event names and two counts only; a typed sentinel never reaches it, and host_log reads no id, name, path or check content at trace; no new span, log site, verb, dependency, socket or MCP surface.
Discharged the owed kept actions replay test named for Handler coverage (stand_replay gains a_kept_actions_step_replays_and_a_changed_one_stops_naming_nodes); the founder ratified kept-check files on the role's argv as a boundary widening.
Retired nothing; VERBS stays fourteen, info_span sites stay 2, Cargo.toml and Cargo.lock byte-identical.
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
CLI arguments (escher-session) rule: command_line gains the coverage role, the only role that reads files named on argv; still the four fixed endings 0/1/2/3, nothing the caller typed written back, a --session or --record word among the paths is usage; the usage line gains a coverage role clause.
@task_item n=2 part=touches
Recorded check (Check::from_bytes) rule: a kept check now has a second reader, the command line's coverage role, reading caller-named files under the byte bound and refusing malformed with nothing of the path or file; every replayed step still passes validate inside Session::run.
@task_item n=3 part=touches
Committed checks exception: the strict reader is now reached from the command line too; the two committed Counter checks themselves are byte-unchanged and stay the one exception to owner-only.
@task_item n=4 part=touches
Kept checks file medium: a kept check may now be named to escher-session coverage; of it only ids, owner ids, event names and counts reach stdout, never its steps, typed text or path.
@task_item n=5 part=touches
Threat model CLI input vector: a new entry point, escher-session coverage reading arbitrary local files the caller names, an untrusted input bounded and parsed strictly; the attack surface list names only example and rdme CLI entry points today.
@task_item n=6 part=touches
Data-protection fact naming Handler coverage as the owner of the owed actions replay test (the discharged entry, fct-cl72c3yo): the test now exists and passes, so the CARRY is discharged.
@task_item n=7 part=touches
Dioxus bridge rule (fct-frlob5k5 and its detail, fct-3atajnay): holds as written for the SignalRegistry dispatch, and gains the reached record beside it on handle_event, active only where record_reached was called; every other document pays one has_context lookup per dispatch.
@task_item n=8 part=touches
Stable element id carriers: ids now also leave through the coverage answer line (ids and owner ids), still no NodeId crossing.
@task_item n=9 part=touches
Stdout output fact: escher-session writes a new line form on stdout, the coverage line {unexercised:[{id,owner,event}],checks,held}, uncoloured, holding nothing the caller typed.
@task_item n=10 part=touches
Refusal and SessionError format: the coverage role reuses the malformed cause (Refusal::new(Cause::Malformed)) and the fixed SessionError table; no new cause, the eight-cause set unchanged.
@task_item n=11 part=touches
Widening rule and ratified widenings: kept-check files named on the role's argv are a ratified boundary widening (pattern rul-izni57qv) on the founder's word relayed by the overseer (inputs#I4, Q1).
@task_item n=12 part=touches
Driver command schema: VERBS stays fourteen verbs; the coverage line is no verb, no Outcome variant and no node key; the command line still routes every replayed call through validate.
@task_item n=13 part=touches
serde_json dependency fact: still a reader only; Coverage::to_json is hand-written and serde_json writes nothing; no dependency added.
@task_item n=14 part=touches
Environment values read: escher-driver's unit tests reaching target/tmp through CARGO_MANIFEST_DIR now include the coverage role test's scratch files; no new env read.
@task_item n=15 part=touches
Logging: no new span or log site; each replayed step leaves its own command span through Session::run, info_span sites in escher-driver stay 2; host_log gains a sixth test reading the coverage run at trace.
@task_item n=16 part=touches
Test and count citations: stand files 53 to 54, run stand 262 passed 0 failed 7 ignored over 54 files, stand_replay 3 to 4, cli_commands linux 17 to 18, host_log 5 to 6, cli_tests 5 to 6, json tests 7 to 8, the line table 102 to 109 rows, any security fact citing these moves.
@task_fact n=1 slug=input-validation
fact: CLI arguments (escher-session coverage role) — one shape, coverage <app> <check>..., no --session or --record; each path read through File::open(..).take(MAX_CHECK_BYTES + 1) then Check::from_bytes; unreadable, over-bound, non-check or another app's check refused malformed status 1 holding nothing of the path; boot failure status 3; accepted line status 0.
@task_fact n=2 slug=input-validation
fact: Dioxus bridge reached record (dioxus-native-dom, ungated) — after record_reached, handle_event writes each chain node's stable id and event name into a root-context ReachedRecord before runtime.handle_event, deduplicated by (id, event); no log site, no NodeId, its Debug prints the count alone; the crate-private element_listener_ids is its one reader.
@task_fact n=3 slug=threat-model-summary
fact: Kept check named on the command line — escher-session coverage reads caller-named local files as untrusted input through the bounded strict Check::from_bytes, replaying each on a fresh session; its answer carries ids, owner ids, event names and two counts only.
@task_fact n=4 slug=data-protection
fact: Coverage answer line — holds ids, owner ids, event names and the checks and held counts only; a typed sentinel occurs 0 times (stand_handler_coverage), and host_log reads no id, name, coverage, sentinel, path or recorded needle at trace.
@task_item n=1 part=untouched
The driver session socket, its state directory and the MCP stdio server: no grammar, bound, permission or carrier changed; the coverage role binds nothing and no socket, MCP or cold-agent surface moved.
@task_item n=2 part=untouched
The recording directory rule under --record: recording is unchanged; the coverage role takes no --record and writes no file.
@task_item n=3 part=untouched
Authentication and authorization: still none; the new role adds no handshake, token or peer check.
@task_item n=4 part=untouched
Dependency security, cargo-deny, deny.toml, lockfile and supply chain: no manifest, lock or CI workflow changed.
@task_item n=5 part=untouched
Secret management storage and what counts as secret: no secret, CI secret or credential source added.
@task_item n=6 part=untouched
escher_telemetry sink allowlist and scrub set: no log field, span or scrub set moved.
@task_item n=7 part=untouched
The agent-run.sh, cold-agent.sh, apt-install.sh and coverage_floor.py argument rules: none of those scripts changed.
@task_item n=8 part=untouched
Engine input validation (URLs, file and data URLs, fetch, CSS, SVG, fonts, markup attributes, forms, password masking): no engine reader changed.
@task_item n=9 part=untouched
Error-handling panic paths and graceful degradation of the engine, shell and WPT runner: untouched.
