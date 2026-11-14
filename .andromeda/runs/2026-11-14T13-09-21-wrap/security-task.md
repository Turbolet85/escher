@task_prose part=session
The session adds a start in a named fixture state to the driver: a session reaches a named state of its app, in process, by running a fixed list of acting calls (click, type, press, advance) through the session's validated run path, in order.
A state name is checked whole against the app's closed table before anything boots; any other name is refused as an unknown state whose fixed text holds nothing the caller supplied.
A reach stops at the first refused or unsettled step, naming the step's index and its closed refusal cause or busy reading; the steps before it stand.
The stand app gains three named states (two on the CRUD task, one on the flight booker) and a start-in-a-state entry; the shared test fixtures gain a hold in a named state; a recorded session started in a state replays from boot with the check form unchanged.
No carrier changed: the command line, MCP tools, the session socket wire, the kept check reader and the session errors are untouched, and the capability statement now says a state is in process only, ratified on the founder's own word (option C).
No dependency, CI leg, span field, log site, print or environment read was added; one fix spelled the standard result type where the Android prelude shadows it.
@task_file n=1 path=packages/escher-driver/src/fixture.rs
what: the driver's in-process start-in-a-state API: the fixture step list, the closed fixture table lookup by whole name, the reach error set with fixed texts, and the session's reach through the validated run path
@task_file n=2 path=packages/escher-driver/src/fixture_tests.rs
what: the driver's unit checks of the fixture lookup, the step-to-call conversion, the reach stop at a refused step, the unsettled reading and the reach errors' fixed text
@task_file n=3 path=packages/escher-driver/src/lib.rs
what: the driver library's crate root and public exports, now exporting the fixture, fixture step and reach error
@task_file n=4 path=examples/seven_guis/src/stand.rs
what: the stand app of the seven GUI tasks, holding each task's closed table of named fixture states and the start-in-a-state entry that checks the name before it boots a session
@task_file n=5 path=tests/blitz-tests/tests/fixtures/mod.rs
what: the shared test fixtures module stating the fixture boot and hold once, now also a hold in a named state
@task_file n=6 path=tests/blitz-tests/tests/stand_start_in_state.rs
what: the integration check of a start in a state: each named state reads as its by-hand path from boot in both layout modes, an unknown name is refused, a refused step stops the reach, a recorded session replays from boot
@task_file n=7 path=docs/capabilities.md
what: the committed capability statement of the driver and its carriers, now stating that a fixture state is in process only and crosses no carrier
@task_item n=1 part=touches
One validation path for every carrier and the single command validation gate: each fixture step's call now runs through the session's run path and its validate, the same gate every carrier uses; a new in-process caller of that gate.
@task_item n=2 part=touches
Closed driver verb schema: a fixture step spells the verb table's call with its arguments by schema name in schema order; no verb or argument added.
@task_item n=3 part=touches
A refusal holds nothing a call supplied, no echo of caller input: the reach errors (unknown state, refused at a step, not settled at a step) have fixed texts naming no name tried, only a step index, a closed cause or a busy reading.
@task_item n=4 part=touches
Closed refusal cause set and the driver refusal causes: a reach refused at a step carries the step's closed refusal cause unchanged; no cause added.
@task_item n=5 part=touches
Session errors from a fixed table: the start-in-a-state entry wraps a session error or a reach error; the session error table itself is unchanged.
@task_item n=6 part=touches
One command span per call and command span holds no content: reaching a state opens no span of its own; each step is logged only by the command span its run call opens, with no field added.
@task_item n=7 part=touches
Driver outcomes never logged, driver installs no subscriber and reads no env, session output holds no caller input: the reach adds no log site, print or environment read.
@task_item n=8 part=touches
Session label form: every fixture state name is held to the label form and named once in its table.
@task_item n=9 part=touches
Boundary widening rule and a closed-value field in a ratified carrier: the founder chose in-process only (option C), so no carrier carries a state, no boundary moves and no widening escalation is owed; the absence lands ratified.
@task_item n=10 part=touches
Seeded source takes its seed from in-process code only and the absence of a URL-taking driver verb: the precedent beside which the new in-process-only state absence stands.
@task_item n=11 part=touches
Committed statement of the MCP crossing and the session socket carries calls: the capability statement now says no fixture state crosses the command line, MCP, the session socket or a kept check; those carriers are unchanged.
@task_item n=12 part=touches
Replay runs each step through the session, the recorded check file start record and its arguments: a recorded session started in a state replays from boot with the check form unchanged, the reach's steps recorded as ordinary calls.
@task_item n=13 part=touches
Advance moves the virtual clock exactly and time moves only on advance: an advance step of a fixture moves time only through the advance call.
@task_item n=14 part=touches
Driver command answer settled: a reach reads the settled flag off an acting or advancing outcome and stops as not settled naming the busy reading.
@task_item n=15 part=touches
Unknown id refused as stale or not-found and a call on an unknown or unfit element refused: a fixture step aimed at an absent element stops the reach with that refusal at its index.
@task_fact n=1 slug=input-validation
fact: A fixture state name is compared whole against the app's closed table before any session starts; any other name is refused as an unknown state and nothing is booted.
@task_fact n=2 slug=input-validation
fact: A session reaches a named fixture state only in process, each step's call run in order through the session's one validated run path, stopping at the first refused or unsettled step with the steps before it standing, not rolled back.
@task_fact n=3 slug=input-validation
fact: Absence, ratified: no fixture state crosses the command line, MCP, the session socket or a kept check; starting in a state is an in-process library capability only.
@task_fact n=4 slug=error-handling
fact: Reach errors are a closed set (unknown state, refused at a step with the step index and closed cause, not settled at a step with the busy reading) with fixed texts holding nothing a caller supplied.
@task_item n=1 part=untouched
Dependency security: no dependency, lockfile, audit ignore, licence or pin changed.
@task_item n=2 part=untouched
Authentication, CI tokens, publish jobs and secret management: no CI step, leg, secret or workflow changed.
@task_item n=3 part=untouched
Outbound requests and fetch: script fetchers, file and data URLs, CORS and same-origin, per-host limits, timeouts and response size absences are not reached.
@task_item n=4 part=untouched
Data protection storage: history database, cookies, HTTP cache, clipboard, file dialog and TLS are not reached.
@task_item n=5 part=untouched
Rendering and markup handling: select lists, accessibility adapter, stable element id derivation, password masking, range inputs, CSS and HTML parsing, serialization are unchanged.
@task_item n=6 part=untouched
Session socket bounds, idle expiry, owner-only state and recording directories and the socket wire grammar: no wire, bound or directory rule changed.
@task_item n=7 part=untouched
Cold-agent, agent-run, apt-install, coverage floor and WPT runner scripts and their exit codes: none changed.
@task_item n=8 part=untouched
Telemetry sink allowlists, engine log targets and network tracing log fields: no target or field added.
