@task_prose part=session
Restructured three functions toward a cognitive complexity of 15 with behaviour unchanged: the engine's click default actions (handle_click 97 to 2), the driver's command executor (Session::execute 32 to 1) and the Dioxus document's UI graph builder (DioxusDocument::graph 50 to 4), each split into private per-arm or per-step functions with signatures and callers kept.
handle_click is an engine function, restructured on the operator's word with upstream's arms included, so a later upstream sync that edits one of those arms must be re-applied by hand.
Added one new blitz-tests integration target, click_default_actions: six checks run in both layout modes (a link leaving the document navigates and a fragment link does not, a link with no href walks on to its label, a click no element takes clears focus, a disabled submit button stops the walk, a label of a multiple select focuses it and chooses nothing, two clicks with no time between them are one double click), over a hand-written recording NavigationProvider and an HtmlDocument wrapped by Harness::wrap with a VirtualClock; the check is silent (no tracing, print, env read or sink).
Measured: each new check red under its own mutation control (6 holds); coverage readings escher-driver 98.01 %, dioxus-native-dom 90.09 %, blitz-test-harness 95.74 %, every per-crate floor held; functions over 15 in the three files 6 to 3; CI green 17/17. No public symbol, dependency, span field, refusal cause, verb or schema changed; nothing retired.
@task_file n=1 path=packages/blitz-dom/src/events/pointer.rs
what: the engine's pointer event handling — the click default actions (link navigation, label re-dispatch, checkbox and radio checkable change events, summary toggle, select and option choice, file input, focus clearing, the double-click dispatch)
@task_file n=2 path=packages/escher-driver/src/execute.rs
what: the driver's command executor — Session::execute running each verb (snapshot, click, type, press, advance, scroll, screenshot, graph) over the held instance, feeding the session's record of ids
@task_file n=3 path=packages/dioxus-native-dom/src/graph.rs
what: the Dioxus document's UI graph — task state readings, node handlers and listeners from the vdom walk, declared and confirmed handler writes, the graph answer
@task_file n=4 path=tests/blitz-tests/tests/click_default_actions.rs
what: a new blitz-tests integration test target checking the engine's click default actions in both layout modes, with a hand-written recording navigation stand-in and a virtual clock
@task_item n=1 part=touches
decision/complexity-ceiling-host-only: its clause that engine functions over the ceiling are left as they are is now stale; the engine function handle_click was restructured (97 to 2) on the operator's word, the other engine functions still left (handle_pointermove 77, handle_pointerdown 39, touch_action_pan_axes 17).
@task_item n=2 part=touches
decision/per-crate-coverage-floors and decision/coverage-leg-exits-with-floor: the coverage readings moved (escher-driver 98.04 to 98.01 %, dioxus-native-dom 86.71 to 90.09 %, blitz-test-harness 96.06 to 95.74 %), each floor held; a row holding a floor value is to be read against these readings.
@task_item n=3 part=touches
decision/one-test-target-per-behaviour, decision/blitz-tests-one-target-per-file, decision/one-test-target-per-file and decision/blitz-tests-single-test-target / decision/blitz-tests-manifest-difference: a new blitz-tests target was added for one behaviour, auto-discovered with the blitz-tests manifest and upstream's aggregate test file unedited.
@task_item n=4 part=touches
decision/stand-checks-in-both-layout-modes and decision/layout-tests-in-both-layout-modes: the new click check loops every case over both layout modes (incremental off and on).
@task_item n=5 part=touches
decision/two-clicks-without-advance-double-click: the double-click dispatch moved inside the restructured click handling and a new check now asserts two clicks with no time between them are one double click.
@task_item n=6 part=touches
decision/change-event-from-engine-only: the checkbox and radio checkable change-event dispatches moved into their own per-arm functions, behaviour unchanged; the checkable_change_event checks still green (114 passed).
@task_item n=7 part=touches
decision/harness-virtual-clock, decision/wrapped-document-keeps-own-clock and contract/harness-construction/wrap: the new check builds an HtmlDocument with its own VirtualClock in its config and wraps it with Harness::wrap.
@task_item n=8 part=touches
decision/hand-written-stand-ins and decision/network-stand-ins-counted: the new check uses a hand-written recording NavigationProvider stand-in, no mocking library.
@task_item n=9 part=touches
decision/test-data-in-code, decision/test-data-is-code and decision/shared-fixture-pages: the new check states its fixture HTML page inline in the test file.
@task_item n=10 part=touches
decision/mutation-controlled-tests: each of the six new cases read red under its own control; one control neutralised a loop's advance to the loop's exit rather than removing the line, which hangs the test binary.
@task_item n=11 part=touches
decision/driver-executor-integration-tested and decision/driver-instance-verbs-tested-by-stand: the driver's command executor was restructured into one function per verb; escher-driver tests (104) and every stand check green.
@task_item n=12 part=touches
contract/session-run/type, contract/session-run/type-on-select and contract/driver-session/type-replaces: the type verb's acting order (select focus or click, select-all and backspace, typing that skips a leading space on a select) now stands as its own named step, behaviour unchanged.
@task_item n=13 part=touches
contract/driver-session/screenshot, contract/session-run/screenshot and contract/harness-screenshot/fonts-unavailable: the screenshot verb's crop and its ScreenshotError mapping moved into their own function, behaviour unchanged.
@task_item n=14 part=touches
contract/driver-session/graph, contract/driver-session/id-record, contract/session-run/graph and contract/driver-verbs/graph: the graph verb and its feeding of the session's record of ids moved into their own function; the command span with its fields stays opened in run.
@task_item n=15 part=touches
decision/graph-signal-registration, decision/listeners-read-from-vdom-walk, decision/unrun-declared-write-reads-unconfirmed and decision/graph-answer-holds-no-value: the UI graph builder was split into state readings, node handlers, handler write and node reads functions, behaviour unchanged; stand_graph and stand_handler_coverage green.
@task_item n=16 part=touches
decision/handler-coverage-by-replay and the handler-coverage rows: they read the UI graph whose builder was restructured; the handler coverage check stayed green.
@task_item n=17 part=touches
decision/log-silence-read-on-a-sink-host and decision/no-ids-or-names-in-stand-logs: the new check is silent, holding no tracing, print, env read or sink, like the checkable_change_event checks; no log site was added in the three restructured files.
@task_item n=18 part=touches
decision/harness-click-waits-on-nothing and contract/harness-input/input-helpers: the new check drives the engine's click default actions through the harness click and focus readings.
@task_fact n=1 slug=quality-gates-coverage-targets
fact: The cognitive-complexity ceiling has one engine exception: handle_click was restructured below the ceiling, upstream's arms included, on the operator's word, at the price of a hand re-apply when an upstream sync edits one of its arms; every other engine function over the ceiling is still left as it is.
@task_fact n=2 slug=3-test-harness-contract
fact: With no node focused the harness's focused reading answers the root html element, never none, so a check that a click clears focus asserts the root element.
@task_fact n=3 slug=unit-test-strategy
fact: A red-before-green control on a loop's advance step neutralises it to the loop's exit instead of removing the line, since removing it loops forever.
@task_item n=1 part=untouched
WPT conformance runner, its CLI, statuses, outputs and pinned commit: no WPT suite or expectation moved.
@task_item n=2 part=untouched
CI workflow, legs, caches, actions pinning, package installs and their exit codes: no CI script or workflow was edited.
@task_item n=3 part=untouched
Agent-run and cold-agent pipe contracts, events, verdicts and exit codes: untouched.
@task_item n=4 part=untouched
Session wire, socket, state directory, host lifecycle, idle expiry, session errors and escher-session exit codes and MCP role: no wire, error, exit code or protocol changed.
@task_item n=5 part=untouched
Session recording and committed recorded checks, their replay and byte-equality: no recording or committed check changed.
@task_item n=6 part=untouched
Lint defects, text contrast reading and the contrast formula: untouched.
@task_item n=7 part=untouched
Mutation tooling rows (cargo-mutants, nextest profile, baseline skipped) and the keyboard_compat coverage exclusion: unchanged.
@task_item n=8 part=untouched
Refusal causes, their order and printed form, the snapshot byte budget and the verb table: no cause, verb or budget added or moved.
