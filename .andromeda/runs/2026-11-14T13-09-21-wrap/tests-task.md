@task_prose part=session
The session added a start in a state: a driver session reaches a named fixture state of its app in process, through the ordinary validated run path, and its first screen reads as that state reached by hand from boot.
The driver gained a fixture type (a name and a fixed list of click, type, press and advance steps), a lookup that compares a name whole against the app's closed table and refuses any other as UnknownState, a reach error with fixed text, and a reach call that stops at the first refused or unsettled step naming its index, the steps before it standing.
The seven_guis headless stand gained its fixture states (Crud fourteen-people and person-selected, Flight Booker return-flight, Counter and Timer none) and a start-in call that checks the name before it boots; the shared test fixtures gained a hold in a named state beside the hold.
One new integration target checks the start in a state in both layout modes, including a recorded session started in a state that replays from boot with the check form unchanged (unix only); no carrier changed: no command line, MCP tool, socket wire, kept check form, session error, cause or schema entry.
The founder ratified that no fixture state crosses the driver's carriers (in process only); CI was red once on Android only, a bare two-parameter Result under the dioxus_native prelude glob, fixed by spelling std::result::Result, then green 17 of 17.
@task_file n=1 path=packages/escher-driver/src/fixture.rs
what: the driver session's fixture states: the fixture and step types, the whole-name lookup in the app's closed table, the reach error and the session's reach through the validated run path
@task_file n=2 path=packages/escher-driver/src/fixture_tests.rs
what: the driver's unit tests of fixture states, a split test module of the driver crate: step to call conversion, whole-name lookup, label-form names, fixed reach error text, stop at the first refused step, unsettled outcomes
@task_file n=3 path=packages/escher-driver/src/lib.rs
what: the escher_driver crate root: its module list, its public re-exports and its crate documentation
@task_file n=4 path=examples/seven_guis/src/stand.rs
what: the seven_guis headless stand: boot, options and timer step of each task, now its fixture state tables, the start in a state and its error, with a unit test of the state names
@task_file n=5 path=tests/blitz-tests/tests/fixtures/mod.rs
what: the shared fixtures module of the integration checks, stating once the fixture boot and hold, now also the hold in a named state
@task_file n=6 path=tests/blitz-tests/tests/stand_start_in_state.rs
what: the stand check of a start in a state, one blitz-tests target per file, run in both layout modes and joining the stand-repeat leg by its stand prefix
@task_file n=7 path=docs/capabilities.md
what: the capability statement pinned to the driver schema, its claims naming their checks; it gained the start in a state claim and an in-process-only claim, its pinned counts unchanged
@task_item n=1 part=touches
decision/shared-fixtures-in-one-module and decision/shared-fixture-pages: the shared fixtures module now states a hold in a named state beside the fixture boot and hold; the row should name it.
@task_item n=2 part=touches
decision/session-checks-share-one-module and decision/shared-test-modules-not-targets: the new stand check reads both the shared session module and the shared fixtures module as modules, not targets; the readers' counts moved (fixtures 34 to 35, session 37 to 38) but no row states a count.
@task_item n=3 part=touches
decision/one-test-target-per-file, decision/blitz-tests-one-target-per-file and decision/one-test-target-per-behaviour: a new blitz-tests target for one behaviour, the start in a state.
@task_item n=4 part=touches
decision/stand-checks-in-both-layout-modes and decision/layout-tests-in-both-layout-modes: every new stand check runs in both layout modes.
@task_item n=5 part=touches
decision/stand-repeat-three-rounds, decision/zero-failure-stand-repeat, decision/stand-checks-zero-failure-budget and decision/joining-file-keeps-its-legs: the new stand check joins the test, coverage and stand-repeat legs by its file and stand prefix, no leg list changed.
@task_item n=6 part=touches
contract/stand-boot (seven_guis headless stand, boot, options, timer step): the stand gains its per-task fixture state tables, a start in a state that checks the name before booting, and its error of session or reach.
@task_item n=7 part=touches
contract/driver-session/start-label, contract/driver-session/run-validation and contract/driver-session/run-carriers: Session::start is unchanged, and a new in-process reach runs each step through run's validation; no carrier carries a state.
@task_item n=8 part=touches
contract/driver-session/not-settled-result, contract/driver-session/acting-verbs and contract/harness-settle/notsettled: a reach reads settled off acting and advanced outcomes and stops NotSettled naming the step and what is busy.
@task_item n=9 part=touches
decision/refusal-leaves-instance-unchanged: a refused step changes nothing, but a reach is not rolled back, the steps before the refused one stand; a refused name starts no session.
@task_item n=10 part=touches
decision/session-error-messages-fixed and decision/assertions-print-no-argument-value: the reach error's texts are fixed and hold nothing a caller supplied, no name tried.
@task_item n=11 part=touches
decision/test-data-is-code and decision/test-data-in-code: fixture states are code tables of steps, nothing seeded or loaded.
@task_item n=12 part=touches
decision/recorded-checks-replayed-as-tests, decision/replay-through-the-call-path, decision/recording-checks-unix-gated and contract/session-recording/check-read: a recorded session started in a state replays from boot as ordinary calls, the check form unchanged, the check unix-gated.
@task_item n=13 part=touches
decision/capability-statement-pinned, decision/capabilities-doc-pinned-to-schema, decision/capability-claims-name-their-checks and decision/capability-check-names-held-to-schema: the capability statement gained a start in a state claim naming its checks and an in-process-only claim; the pinned schema counts did not move.
@task_item n=14 part=touches
decision/table-tests-count-their-rows and decision/call-assertions-carry-row-index: the new driver unit tests are table-driven with row-indexed assertions.
@task_item n=15 part=touches
decision/mutation-controlled-tests: two remove-the-guard controls held for the new checks (the reach stop and the twelve creates).
@task_item n=16 part=touches
decision/inline-unit-test-modules, decision/unit-tests-in-source-file and decision/test-module-split-keeps-paths: the driver's fixture tests are a split cfg(test) child module by path, and the stand's state-name test sits inline beside its table.
@task_item n=17 part=touches
decision/no-ids-or-names-in-stand-logs, decision/driver-log-fixed-words-only and contract/command-span/verb: the reach adds no span, log site or print; each step opens the existing command span.
@task_item n=18 part=touches
decision/native-platform-ci-matrix, decision/local-pre-push-gate and decision/ci-legs-host-reproducible: the local gates were green while CI's Android test job alone failed to compile; the dev host builds no Android target.
@task_fact n=1 slug=session-lifecycle
fact: A driver session reaches a named fixture state in process: each step (click, type, press, advance) runs through the ordinary validated run path in order, the reach stops at the first refused or unsettled step naming its index and its cause or what is busy, and the steps before it stand.
@task_fact n=2 slug=session-lifecycle
fact: A fixture state name is compared whole against the app's closed table; any other name is refused UnknownState with a fixed text holding nothing supplied, and a refused name starts no session.
@task_fact n=3 slug=session-lifecycle
fact: No fixture state crosses the driver's carriers: not the command line, MCP, the session socket or a kept check; a start in a state is in process only, ratified by the founder.
@task_fact n=4 slug=session-lifecycle
fact: A recorded session started in a state replays from boot through the ordinary call path, the reach's steps kept as ordinary calls and the check form unchanged.
@task_fact n=5 slug=3-test-harness-contract
fact: The seven_guis headless stand states each task's fixture states in code (Crud fourteen-people and person-selected, Flight Booker return-flight, Counter and Timer none) and its start in a state checks the name before it boots.
@task_fact n=6 slug=3-test-harness-contract
fact: The shared fixtures module states once a hold in a named state beside the fixture boot and hold.
@task_fact n=7 slug=integration-test-strategy
fact: A start in a state is checked in both layout modes: each named state reads as its by-hand path from boot and takes the next call as the by-hand session does, and an unknown name is refused with the session reading as boot.
@task_fact n=8 slug=ci-integration
fact: In a file that globs the dioxus_native prelude, a two-parameter result is spelled std::result::Result: on Android the glob brings a one-parameter Result alias, and only CI's Android test job sees it.
@task_item n=1 part=untouched
WPT conformance, the WPT runner, its statuses, expectations and reports: no WPT suite, runner or status was touched.
@task_item n=2 part=untouched
agent-run and cold-agent contracts, events, verbs, exit codes and verdicts: no agent-run or cold-agent behaviour moved.
@task_item n=3 part=untouched
the escher-session command line, MCP role and tool results, session wire, session host, exit codes and stderr: no carrier, tool, wire message or exit changed.
@task_item n=4 part=untouched
the session error set and the refusal cause set and their printed forms: no session error or cause was added; the reach error carries an existing cause.
@task_item n=5 part=untouched
CI workflow mechanics: actions pinning, secrets, permissions, caches, artifacts, package installs, coverage floors and their exit codes, clippy, audit, MSRV: no CI step, leg, script or floor changed.
@task_item n=6 part=untouched
lint defects, text contrast, screenshot, pixel and shot checks: nothing of rendering or reading the screen changed.
@task_item n=7 part=untouched
handler coverage and the graph and actions answers: no coverage line, graph or actions answer changed.
@task_item n=8 part=untouched
the engine harness's construction, input helpers, core and settle internals: no harness surface changed.
@task_item n=9 part=untouched
mocking and stand-in discipline, the virtual clock and the advance command's contract: no stand-in was added and advance is reused as it stands.
@task_item n=10 part=untouched
quality gates on mutation runs, complexity ceilings and the keyboard compatibility exclusion: unchanged.
