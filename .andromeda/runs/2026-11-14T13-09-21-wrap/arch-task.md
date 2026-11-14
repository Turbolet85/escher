@task_prose part=session
The session adds starting a driver session in a named fixture state of its app, in process only: escher-driver gains Fixture, FixtureStep, ReachError and Session::reach, and each step runs as an ordinary validated call through Session::run.
A fixture name is compared whole against the app's closed table, and any other name is refused UnknownState with a fixed text that holds nothing the caller supplied. A reach stops at the first refused or unsettled step, and the steps before it stand.
The seven_guis headless stand gains a closed fixture table per lean task (Crud fourteen-people and person-selected, Flight Booker return-flight, Counter and Timer none) and start_in, which checks the name before it boots, then starts and reaches.
No carrier changed: the start schema, command line, MCP tools, session socket wire, kept-check form, session errors, verbs, causes, node keys and spans are all unchanged. By the founder's own word (option C) a fixture state crosses no carrier, and that absence is ratified.
The shared blitz-tests fixtures module gains hold_in beside hold, and a new check target covers a start in a state in both layout modes, including a recorded session that replays from boot.
CI went red on Android only, because a bare two-parameter Result under the dioxus_native prelude glob resolves to manganis's one-parameter alias. The fix spells std::result::Result, and CI is green on every OS.
@task_file n=1 path=packages/escher-driver/src/fixture.rs
what: the driver session library's fixture-state reach, a named state as a closed list of acting steps run through the validated run path, with its refusal type
@task_file n=2 path=packages/escher-driver/src/fixture_tests.rs
what: the driver's unit tests of the fixture reach, a path child of its module
@task_file n=3 path=packages/escher-driver/src/lib.rs
what: the escher-driver crate root, its module list, public re-exports and crate docs
@task_file n=4 path=examples/seven_guis/src/stand.rs
what: the seven_guis headless stand, the stand's boot of a lean task, its pinned environment, and now its fixture-state tables and start in a state
@task_file n=5 path=tests/blitz-tests/tests/fixtures/mod.rs
what: the shared blitz-tests fixtures module, which states the fixture boot and hold once
@task_file n=6 path=tests/blitz-tests/tests/stand_start_in_state.rs
what: the blitz-tests integration check of a start in a state on the stand, one target per file, run by the stand-repeat leg by its stand prefix
@task_file n=7 path=docs/capabilities.md
what: the capability statement, the agent-facing list of what the driver does and does not do, held to the driver schema
@task_item n=1 part=touches
decision/no-seed-on-carriers: its sibling absence lands beside it, that no fixture state crosses the driver's carriers (command line, MCP, session socket, kept check). The seed row itself does not change.
@task_item n=2 part=touches
decision/new-carrier-widens-boundary and decision/validated-verb-set-widened-by-ratification: no carrier, verb or argument is added. The state stays in process, so no boundary widens, and the absence is ratified by the founder's word.
@task_item n=3 part=touches
decision/validated-commands-only: a reach runs each fixture step as a validated Call through Session::run, so a fixture adds no unvalidated path.
@task_item n=4 part=touches
contract/driver-session/schema-carriers, contract/driver-verbs/start and contract/escher-session/start: start's schema and carriers are unchanged, and no state argument is added. A start in a state exists only as an in-process library call.
@task_item n=5 part=touches
contract/headless-stand/boot: seven_guis::stand gains fixtures(task), start_in and StartInError beside its boot. start_in checks the name before it boots.
@task_item n=6 part=touches
decision/session-apps-one-table: each lean task's fixture states sit in a closed per-task table on the stand.
@task_item n=7 part=touches
decision/fixtures-test-module and decision/shared-fixtures-stated-once: the shared fixtures module now states hold_in, a hold in a named state, beside the fixture boot and hold.
@task_item n=8 part=touches
decision/driver-spans-only, decision/driver-library-side-effect-free and decision/driver-logs-no-content: the reach adds no span, log site or print. Each step opens only the command span that every run call opens.
@task_item n=9 part=touches
decision/error-enums and decision/typed-results: two new error enums. ReachError is UnknownState, Refused with step and cause, and NotSettled with step and busy, all with fixed texts. StartInError is Session or Reach.
@task_item n=10 part=touches
decision/recorded-session-kept-check and decision/kept-check-replays-through-run: a recorded session started in a state keeps the reach's steps as ordinary calls and replays from boot. The check form is unchanged.
@task_item n=11 part=touches
decision/capability-statement and decision/capability-statement-matches-schema: the capability statement gains a start-in-a-state section and the line that a state is in process only. Its pinned counts are unchanged.
@task_item n=12 part=touches
decision/driver-unit-tests-in-path-children and decision/split-test-module-as-child: the new driver module's unit tests sit in a path child.
@task_item n=13 part=touches
decision/blitz-tests-one-target-per-file and decision/stand-repeat-zero-failure-budget: a new stand-prefixed target joins the test, coverage and stand-repeat legs by its file and prefix. No leg list changed.
@task_item n=14 part=touches
decision/both-layout-modes and decision/scenarios-both-layout-modes: every start-in-a-state check runs in both layout modes.
@task_item n=15 part=touches
decision/shared-test-modules: the new check also reads the session_common module. No row states the reader count.
@task_item n=16 part=touches
decision/justified-expect-unwrap: hold_in carries one expect with its written justification.
@task_item n=17 part=touches
name/crate/escher-driver: the crate's public API gains Fixture, FixtureStep and ReachError, plus Session::reach. No crate or binary name is taken.
@task_item n=18 part=touches
decision/android-entry-point and decision/platform-gating: the stand's prelude glob brings manganis's one-parameter Result alias on Android only. Code there must spell std::result::Result.
@task_fact n=1 slug=standard-contracts
fact: A driver session reaches a named fixture state in process by Session::reach. It runs each of the state's acting steps (click, type, press, advance) as a validated call through the one run path, and stops at the first refused or unsettled step with the earlier steps standing, not rolled back.
@task_fact n=2 slug=standard-contracts
fact: A fixture name is compared whole against the app's closed fixture table. Any other name is refused UnknownState, and every ReachError text is fixed and holds nothing a caller supplied.
@task_fact n=3 slug=standard-contracts
fact: Absence, ratified by the founder's word: no fixture state crosses the driver's carriers (the command line, MCP, the session socket or a kept check). A start in a state exists in process only.
@task_fact n=4 slug=standard-contracts
fact: The headless stand holds a closed fixture table per lean task: Crud has fourteen-people and person-selected, Flight Booker has return-flight, and Counter and Timer have none. Its start in a state refuses an unknown name before anything boots.
@task_fact n=5 slug=standard-contracts
fact: A recorded session started in a state keeps the reach's steps as ordinary calls, and its kept check replays from boot in the unchanged check form.
@task_fact n=6 slug=conventions
fact: The shared blitz-tests fixtures module states the fixture boot, the hold and the hold in a named state once.
@task_fact n=7 slug=conventions
fact: A file that globs the dioxus_native prelude spells a two-parameter result as std::result::Result. Exception and reason: on Android the glob brings manganis's one-parameter Result alias, which only the CI Android job sees.
@task_item n=1 part=untouched
Engine rows (style, layout, paint, hit testing, scrolling, select list, text, fonts and the accessibility tree): the session reaches no engine code.
@task_item n=2 part=untouched
The verb table, argument kinds, refusal causes and faults, session errors, node keys, snapshot and diff contracts: all unchanged. A fixture step reuses the existing verbs.
@task_item n=3 part=untouched
The session socket wire, the session host, the MCP server and role, and the escher-session command line contracts: no wire, tool, flag or exit code changed.
@task_item n=4 part=untouched
Telemetry rows (the escher-telemetry bootstrap, scrub, line and span-field contracts): no span field, event, target or env read is added.
@task_item n=5 part=untouched
The WPT runner, the browser app, rdme, the cold-agent and agent-run scripts, and the CI workflow and leg contracts: none changed.
@task_item n=6 part=untouched
Stack and dependency rows and the seeded randomness rows: no dependency, tool or seed behaviour moved.
@task_item n=7 part=untouched
Occupied resources (listeners, state and recording directories, the model provider) and the names sheet's binaries and others: nothing new is occupied.
