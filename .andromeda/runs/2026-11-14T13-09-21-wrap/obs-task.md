@task_prose part=session
A driver session can now start in a named fixture state of its app: escher-driver gains Fixture, FixtureStep, ReachError and Session::reach, in process only.
Session::reach runs each step of the state as an ordinary acting call (click, type, press, advance) through the session's run path, stops at the first refused or unsettled step, and opens no span, log site or print of its own: each step opens the command span every run call opens.
A fixture name is compared whole against the app's closed table; any other name is refused UnknownState; ReachError texts are fixed and hold nothing the caller supplied.
The headless stand gains three named states (fourteen-people, person-selected, return-flight) and start_in, which checks the name before it boots; the shared test fixtures module gains hold_in beside hold; a new silent check, stand_start_in_state, covers both layout modes and a recorded session started in a state replaying from boot with the check form unchanged.
No verb, argument, cause, session error, exit, log field or span field was added; the command line, MCP tools, the host wire and the kept check file are unchanged, and by the founder's own word no fixture state crosses any carrier (ratified, not provisional).
The capability statement was updated by hand with the start in a state and its in-process-only absence; one Android compile fix spelled std::result::Result in the stand.
@task_file n=1 path=packages/escher-driver/src/fixture.rs
what: the escher-driver library's in-process reach of a named fixture state — the fixture table type, its four acting steps, the reach error with fixed texts, and the session's reach that runs each step through the session's run path
@task_file n=2 path=packages/escher-driver/src/fixture_tests.rs
what: escher-driver's unit tests of the fixture reach — step-to-call conversion, whole-name lookup, fixed reach-error text, stop at the first refused step, busy read from an unsettled outcome
@task_file n=3 path=packages/escher-driver/src/lib.rs
what: the escher-driver library root — its module list, public exports and crate documentation
@task_file n=4 path=examples/seven_guis/src/stand.rs
what: the headless stand of the seven GUIs tasks — the tasks' boot for driver sessions, now with each task's named fixture states and the start in a state
@task_file n=5 path=tests/blitz-tests/tests/fixtures/mod.rs
what: the shared fixtures test module — the fixture boot and hold stated once, now with a hold in a named state
@task_file n=6 path=tests/blitz-tests/tests/stand_start_in_state.rs
what: the check of a start in a state — a silent integration check on the stand in both layout modes, including a recorded session's replay from boot
@task_file n=7 path=docs/capabilities.md
what: the capability statement, updated by hand — its start-in-a-state entry and its absence of a fixture state on the driver's carriers
@task_item n=1 part=touches
decision/one-span-per-command and the command span contracts: a reach opens no span of its own; each of its steps is one command and opens the one command span its run call opens, with the same fields — no field added, field count unchanged.
@task_item n=2 part=touches
decision/hosted-call-through-session-run: the reach of a fixture state runs every step through the session's run path alone, so validation and the command span apply to each step as to a hosted call.
@task_item n=3 part=touches
decision/driver-spans-carry-no-content and decision/command-span-no-call-content: neither the fixture name nor its steps' arguments enter any span field.
@task_item n=4 part=touches
decision/driver-emits-no-event and decision/driver-library-installs-no-subscriber: the new driver module adds no event, log site, print or subscriber.
@task_item n=5 part=touches
decision/fixed-diagnostics, decision/escher-session-fixed-diagnostics and decision/answers-echo-nothing-typed: the new reach error reads fixed texts (unknown state, refused at step n with its cause, not settled at step n) naming no name the caller tried.
@task_item n=6 part=touches
contract/refusal-cause rows: a refused reach step carries the existing closed refusal cause and its step index; no cause name added or renamed.
@task_item n=7 part=touches
decision/checks-silent: a new silent check of the start in a state, and the shared fixtures module gains a hold in a named state beside its boot and hold.
@task_item n=8 part=touches
decision/headless-stand-no-sink: the headless stand gains named fixture states and a start in a state; it still installs no sink.
@task_item n=9 part=touches
decision/recording-bounds, contract/recording-file rows and contract/kept-check rows: a recorded session that reaches a state records each step as an ordinary check and replays from boot; the recording and kept check form are unchanged.
@task_item n=10 part=touches
decision/advance-accepted-every-session: advance is one of the four step kinds a fixture state is reached by.
@task_item n=11 part=touches
contract/host-wire/v3, contract/escher-session-commands rows and contract/mcp-role-stdout/json-rpc: unchanged — no fixture state crosses the command line, MCP, the host wire or a kept check; the new absence belongs beside these carrier rows.
@task_item n=12 part=touches
decision/capability-statement-updated-by-hand: the capability statement was edited by hand for the start in a state and its in-process-only absence; its pinned counts are unchanged.
@task_item n=13 part=touches
decision/session-logs-carry-no-page-content and decision/state-registry-holds-no-values: a reached state is read through the same snapshot and answers as a by-hand path; the reach logs and stores nothing of the page.
@task_fact n=1 slug=span-trace-coverage
fact: Reaching a named fixture state opens no span of its own; each step of the reach is one driver command and opens the one command span its run call opens.
@task_fact n=2 slug=log-coverage
fact: The fixture reach error is fixed text (unknown state, refused at a step index with its closed cause, not settled at a step index) and holds no fixture name or other value the caller supplied.
@task_fact n=3 slug=3-observability-harness-contract
fact: Absence, ratified by the founder: no fixture state crosses the command line, MCP, the session socket or a kept check; a session starts in a named state in process only.
@task_fact n=4 slug=ci-integration
fact: A recorded session started in a named state keeps its reach steps as ordinary checks and replays from boot; the kept check form is unchanged.
@task_item n=1 part=untouched
Telemetry strategy rows (tracing crate choice, engine tracing feature gate, ungated escher crates): no dependency or feature changed.
@task_item n=2 part=untouched
Metric coverage rows and the metrics, telemetry backend and error-reporting absences: no metric, exporter or backend added.
@task_item n=3 part=untouched
escher's sink log line, sink configuration, scrub set and resource key rows: the sink, its format and its scrub were not changed.
@task_item n=4 part=untouched
Cold-agent and agent-run harness rows (events, verdict fields, transcripts, cleanup): no harness or its events changed.
@task_item n=5 part=untouched
Script error, window error event, WPT report and panic hook rows: no engine error path changed.
@task_item n=6 part=untouched
CI integration rows on workflows, artifacts and the package-install diagnostic: no CI step, leg, script or artifact changed; the new check joins the existing legs by its name.
@task_item n=7 part=untouched
PII rows on select lists, optgroups, open-list shots, page events and snapshot node values: no markup, id or class was added to any task and no snapshot shape changed.
@task_item n=8 part=untouched
escher-session exit codes, coverage role, error object and tool_call span rows: no exit, coverage line, session error or MCP tool changed.
