@task_prose part=session
Added a seventh served stand app, order, an order page whose controls each cause something a driver reads in caused: Submit with an out-of-rule quantity returns an error under the page's own error boundary (with Retry), a good Submit logs one info record, and Export panics.
The served-task set moved from six to seven (order after load) in the session table, the command line usage, the MCP start enum, the cold-agent prompt (seven apps, goal 7) and the capability statement and front page.
The cold-agent verdict gained caused_by_kind, counts of acting answers carrying a panic, error or log item; capability-unused now also fires when any of the three reads 0.
Measured: a caused item from the order page reads no owner, since the UI graph reads no owner for any of its controls; the plan's claim that each item is tied to its owner was disproved and the graph's reading pinned.
Measured on CI: on android the Dioxus prelude glob brings a one-parameter Result alias into scope, so the page's Submit handler spells std::result::Result whole.
seven_guis now depends on tracing (workspace dependency, no new package); the stand's cross-OS reading table grew from 48 to 54 rows.
@task_file n=1 path=examples/seven_guis/src/tasks/order.rs
what: the order page, the seventh served stand task: its components, element ids, registered graph states, the fixed error, log and panic texts and its CSS
@task_file n=2 path=examples/seven_guis/src/tasks/mod.rs
what: the seven_guis task module list
@task_file n=3 path=examples/seven_guis/src/stand.rs
what: the seven_guis headless stand, its boot functions per task (boot_order added) and its offline options
@task_file n=4 path=examples/seven_guis/src/session_host.rs
what: the escher-session served-task table and its apps, read by the usage line and the MCP start enum
@task_file n=5 path=examples/seven_guis/Cargo.toml
what: the seven_guis crate manifest and its workspace dependencies
@task_file n=6 path=Cargo.lock
what: the workspace lockfile
@task_file n=7 path=examples/seven_guis/tests/cli_commands.rs
what: the escher-session command line tests, its served task list and usage line assertion
@task_file n=8 path=examples/seven_guis/tests/host_log.rs
what: the session host's log tests, what the served host writes to stderr at trace
@task_file n=9 path=tests/blitz-tests/tests/stand_order_served.rs
what: the stand check of the order page served: keyed within budget, no lint finding at boot, what each control caused, the same caused across serve and MCP, a kept check replay
@task_file n=10 path=tests/blitz-tests/tests/common/mod.rs
what: the shared stand-check helpers and tables: actionable at boot, handlers at boot, graph drives, unexercised after drives, stated tab orders
@task_file n=11 path=tests/blitz-tests/tests/stand_actions.rs
what: the stand check of the actions verb over every served app
@task_file n=12 path=tests/blitz-tests/tests/stand_contrast.rs
what: the stand check that asserts each served screen's contrast ratios
@task_file n=13 path=tests/blitz-tests/tests/stand_cross_os_readings.rs
what: the cross-OS stand readings table, answer digests identical on every CI OS
@task_file n=14 path=tests/blitz-tests/tests/stand_explore.rs
what: the stand check of exploration over every served app
@task_file n=15 path=tests/blitz-tests/tests/stand_graph.rs
what: the stand check of the UI graph verb and its drives
@task_file n=16 path=tests/blitz-tests/tests/stand_handler_coverage.rs
what: the stand check of the handler coverage reader
@task_file n=17 path=tests/blitz-tests/tests/stand_keyboard.rs
what: the stand check that walks each screen's stated tab order forward and back
@task_file n=18 path=tests/blitz-tests/tests/stand_layout.rs
what: the stand check of the layout verb over every served app
@task_file n=19 path=scripts/cold-agent.sh
what: the cold-agent client and judge: its prompt, done conditions per app, verdict fields and verdict reasons
@task_file n=20 path=.github/scripts/test_cold_agent.py
what: the cold-agent contract tests and their fake escher-session
@task_file n=21 path=docs/capabilities.md
what: the capability statement, the served stand it is proven on and its counts
@task_file n=22 path=README.md
what: the root front page, its status and served-app slugs
@task_item n=1 part=touches
decision/cold-agent-six-apps is stale: the cold-agent prompt now names seven apps and a seventh goal (read what an action caused to find out why).
@task_item n=2 part=touches
contract/cold-agent-verdict/passed: all apps met is now seven apps, and a passing run also needs caused_by_kind with each of panic, error and log at least 1.
@task_item n=3 part=touches
contract/cold-agent-verdict-reasons/capability-unused: its set gains caused_by_kind; the reason fires when any of panic, error or log reads 0.
@task_item n=4 part=touches
contract/cold-agent-verdict/refused-by-cause and the other verdict count rows: a new verdict field caused_by_kind stands beside them.
@task_item n=5 part=touches
contract/cold-agent-events/run-end: the run.end event now carries caused_by_kind as the verdict does.
@task_item n=6 part=touches
contract/cold-agent-cli/run-task: the cold-agent tasks gain order with its own done condition.
@task_item n=7 part=touches
contract/escher-session-cli/usage, contract/escher-session/invocation and contract/escher-session-exit-codes/2: the usage line's apps and the served tasks are seven, order after load.
@task_item n=8 part=touches
contract/escher-session/mcp and contract/mcp-role/tool-listing: the MCP start tool's task enum names order.
@task_item n=9 part=touches
decision/session-apps-one-table: the one served-task table gains order, and the usage line and MCP enum follow it with no command line edit.
@task_item n=10 part=touches
contract/headless-stand/boot and decision/stand-offline: the stand gains boot_order, offline with options as given, rendering the order page in the task shell.
@task_item n=11 part=touches
decision/stand-readings-same-on-every-ci-os: the readings table covers seven apps, 54 rows (six order rows), green on linux, windows and macOS.
@task_item n=12 part=touches
contract/acting-verb-results/caused: the order page is where a driver and a cold agent meet caused items of kind error (boundary own), log (source app) and panic (file and line).
@task_item n=13 part=touches
contract/ui-graph/owner and decision/owning-component-via-public-api: a caused item's owner is the UI graph's, and the graph reads no owner for any order page control, so its items carry none.
@task_item n=14 part=touches
decision/std-result-under-dioxus-prelude: a new instance, the order page's Submit handler spells std::result::Result because android's prelude glob brings a one-parameter Result alias.
@task_item n=15 part=touches
decision/seven-guis-graph-signals and decision/graph-signal-hook-in-seven-guis: the order page registers graph states quantity and placed with their declared writers.
@task_item n=16 part=touches
decision/actionable-elements-keyed and contract/unkeyed-actionable/unkeyed-actionable: the order page's actionable controls are keyed by HTML id, unkeyed_actionable empty.
@task_item n=17 part=touches
decision/examples-panic-on-error: the order page's Export panics on purpose as a stand fixture, a panic that is the page's behaviour, not an error path.
@task_item n=18 part=touches
decision/workspace-dependencies and name/crate/seven-guis: seven_guis takes tracing as a workspace dependency, no new package locked.
@task_item n=19 part=touches
decision/telemetry-target-prefix-drop and contract/telemetry-scrub/other-targets: the order page's info record under target seven_guis::tasks::order is captured into caused but dropped by the stderr sink.
@task_item n=20 part=touches
decision/cold-agent-run-carries-stand-screen and decision/model-provider-only-on-live-cold-agent-run: on the live run the order page's authored caused texts (panic payload, error message, log message) cross to the model provider, a widening on the founder's ruling.
@task_item n=21 part=touches
decision/fixed-program-text-on-carriers: the order page's fixed rule, log and panic texts ride the carriers as program text, no field written.
@task_item n=22 part=touches
decision/capability-statement and contract/capability-statement/sections: the statement names seven served apps, the order page as where caused is met, and boot actionable counts with a seventh (4).
@task_item n=23 part=touches
decision/readme-is-escher-own: the front page's served-app slugs gain order.
@task_item n=24 part=touches
contract/ci-leg-legs/stand-repeat and decision/stand-repeat-zero-failure-budget: the stand-repeat leg takes the new order stand check by its stand prefix, no leg-script edit.
@task_item n=25 part=touches
decision/both-layout-modes and decision/scenarios-both-layout-modes: the order page is walked and checked in both layout modes, its tab order and its failure tab order with Retry stated.
@task_item n=26 part=touches
decision/cold-agent-judge-stdlib-python: caused_by_kind is read with stdlib json, a non-JSON answer skipped, nothing of an item but kind read.
@task_fact n=1 slug=standard-contracts
fact: The cold-agent verdict carries caused_by_kind, an object of the closed words panic, error and log to counts of accepted acting answers (click, type, press, advance, scroll) whose caused list holds an item of that kind, each kind counted once per answer; it is on verdict.json and the run.end event, and capability-unused fires when any of the three reads 0.
@task_fact n=2 slug=standard-contracts
fact: escher-session serves seven tasks, order the seventh after load: an order page whose Submit returns an error for a quantity not a whole number from 10 to 20 under the page's own error boundary with Retry, logs one info record on a good Submit, and whose Export panics, the stand app where a driver and a cold agent meet each caused kind.
@task_fact n=3 slug=standard-contracts
fact: A caused item's owner is the UI graph's owner of the control that caused it; a component keyed by HTML ids under no keyed element reads no owner in the graph, so its caused items carry no owner.
@task_fact n=4 slug=conventions
fact: An app crate that logs takes tracing itself as a workspace dependency; dioxus-native's prelude re-exports no tracing.
@task_item n=1 part=untouched
The engine areas, style, layout, paint, hit testing, scrolling, select list, forms and accessibility tree readers: no engine code moved.
@task_item n=2 part=untouched
The driver verb set, argument kinds, refusal causes and faults, session errors, exit codes other than the usage line naming apps, node keys and answer JSON shapes: none changed.
@task_item n=3 part=untouched
The session socket wire, the MCP server protocol, request and result bounds, idle expiry and the session host process model: unchanged.
@task_item n=4 part=untouched
The telemetry bootstrap, its scrub sets, span fields and line format: unchanged; the page's texts write no field.
@task_item n=5 part=untouched
The kept check file format, recording directory and replay rules: unchanged; the order page's kept check replays as any served app's.
@task_item n=6 part=untouched
The WPT runner, browser app, rdme, todomvc and other examples and their CLIs: untouched.
@task_item n=7 part=untouched
CI workflow jobs, action pins, coverage floors, cargo-deny audit and toolchain or MSRV decisions: no workflow or leg script changed.
@task_item n=8 part=untouched
contract/driver-forms/is-select: the carried correction is not this session's; neither the select target reader nor its callers moved.
