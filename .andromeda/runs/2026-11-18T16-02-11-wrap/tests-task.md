@task_prose part=session
The session added a seventh served stand app, the order page (served as order): its Submit returns an error under its own error boundary with a Retry, a good Submit logs one info record, and Export panics, so a driver and a cold agent meet each kind of caused item.
It added a stand boot for that page, the order entry in the served-task set (six to seven), new stand checks of the page in both layout modes, and the page in every shared stand table and served-app list.
The cold-agent pipe now counts the caused items of acting answers by kind (panic, error, log) into a verdict field caused_by_kind, and capability-unused also fires when any of the three reads 0; its prompt names seven apps and a goal 7.
Measured: a caused item from the order page carries no owner, because the UI graph reads no owner for any of its controls; the plan's owner clause was disproved and the graph's reading pinned.
After implement, the Submit handler's return type was spelled std::result::Result to compile on android, where the dioxus-native prelude glob brings a one-parameter Result alias; CI read green 17/17.
No test was retired; one cold-agent contract test was renamed six to seven, and a stand test name holding the count word six was kept because another check pins it.
@task_file n=1 path=examples/seven_guis/src/tasks/order.rs
what: the order page component of the seven_guis headless stand, a new served task with its controls, its error boundary and its fixed error, log and panic texts; edited after implement for its Submit return type
@task_file n=2 path=tests/blitz-tests/tests/stand_order_served.rs
what: a new stand check target for the order page, keyed within budget with no lint finding at boot, each control's caused answer, the same caused over serve mcp in process and a kept check replayed, in both layout modes
@task_file n=3 path=examples/seven_guis/Cargo.toml
what: the seven_guis example package manifest, edited to take tracing for the page's log record
@task_file n=4 path=Cargo.lock
what: the workspace lock file, edited by one dependency edge, no new package
@task_file n=5 path=examples/seven_guis/src/tasks/mod.rs
what: the seven_guis task module list, edited to add the order task
@task_file n=6 path=examples/seven_guis/src/stand.rs
what: the seven_guis headless stand boots, edited to add stand::boot_order offline with its inline boot test
@task_file n=7 path=examples/seven_guis/src/session_host.rs
what: the escher-session served-task set and session host, edited to serve order as the seventh task, named in the usage line and the MCP start enum
@task_file n=8 path=examples/seven_guis/tests/cli_commands.rs
what: the escher-session command-line checks run through the package's own binary, edited to start seven tasks and assert the usage line names them
@task_file n=9 path=examples/seven_guis/tests/host_log.rs
what: the session host stderr log checks, edited with a check that the order page served at trace logs neither its message nor its rule
@task_file n=10 path=tests/blitz-tests/tests/common/mod.rs
what: the shared stand tables module, edited with the order rows of actionable-at-boot, handlers-at-boot, graph drives and unexercised-after-drives, and a new stated order Tab order
@task_file n=11 path=tests/blitz-tests/tests/stand_actions.rs
what: the stand actions check over the served tasks, edited to seven served apps
@task_file n=12 path=tests/blitz-tests/tests/stand_contrast.rs
what: the stand contrast assertions, edited with the order page's ratios at boot, on failure, on Retry and on the placed status
@task_file n=13 path=tests/blitz-tests/tests/stand_cross_os_readings.rs
what: the cross-OS readings of the served tasks by answer digest, edited to 54 rows with six order rows
@task_file n=14 path=tests/blitz-tests/tests/stand_explore.rs
what: the stand exploration check over the served tasks, edited to seven served apps and their count assertion
@task_file n=15 path=tests/blitz-tests/tests/stand_graph.rs
what: the stand UI graph check of the shared graph drives, edited to drive the order page
@task_file n=16 path=tests/blitz-tests/tests/stand_handler_coverage.rs
what: the stand handler coverage by replay check, edited to cover seven tasks
@task_file n=17 path=tests/blitz-tests/tests/stand_keyboard.rs
what: the stand keyboard check walking each stated Tab order forward and back, edited with the order page and its failed-state order reaching Retry
@task_file n=18 path=tests/blitz-tests/tests/stand_layout.rs
what: the stand layout check over the served tasks, edited to seven served apps
@task_file n=19 path=scripts/cold-agent.sh
what: the cold-agent pipe, its prompt, its tasks and its verdict, edited with caused_by_kind, its capability-unused reading, seven apps and goal 7
@task_file n=20 path=.github/scripts/test_cold_agent.py
what: the cold-agent pipe's offline unittest contract tests with the fake escher-session, edited with a caused scenario key, the order app and a test counting caused by kind
@task_file n=21 path=docs/capabilities.md
what: the capability statement, edited to name seven served apps, the boot actionable counts and the order page as where caused is met
@task_file n=22 path=README.md
what: the README's 0.3.0 status line naming the served app slugs, edited to seven
@task_item n=1 part=touches
decision/cross-os-readings-by-answer-digest: the cross-OS readings now cover seven served tasks, the order page's rows added; six reads stale where seven now stand.
@task_item n=2 part=touches
decision/cold-agent-passes-on-positive-evidence: the pass now also requires caused_by_kind with each of panic, error and log at least 1 and the seventh app met; six reads stale.
@task_item n=3 part=touches
decision/cold-agent-prompt-names-no-ids: the prompt now names seven apps and adds goal 7, read what that action caused to find out why; still no ids or slugs.
@task_item n=4 part=touches
contract/cold-agent-tasks/stand: the stand task set gains order, seven apps.
@task_item n=5 part=touches
contract/cold-agent-verdict/capability-unused: its set gains caused_by_kind, firing when any of the three kinds reads 0.
@task_item n=6 part=touches
contract/cold-agent-verdict/reasons and contract/cold-agent-events/run-end: the verdict and the run.end event gain the caused_by_kind field.
@task_item n=7 part=touches
contract/cold-agent-verdict/tasks-met: the tasks met now count seven apps; the field's name itself stands.
@task_item n=8 part=touches
decision/cold-agent-events-content-free: caused_by_kind carries counts of closed kind words only, nothing of an item but its kind read.
@task_item n=9 part=touches
contract/session-host/served, contract/session-status/served and contract/escher-session-command-line/usage-line: the served task set and the usage line's apps gain order, seven.
@task_item n=10 part=touches
contract/escher-session-exit-status/2 and contract/escher-session-exit-codes/2: the usage line they print names seven apps.
@task_item n=11 part=touches
contract/mcp-server/tools-list: the MCP start tool's app enum gains order.
@task_item n=12 part=touches
contract/stand-boot rows: a new stand::boot_order sits beside boot, boot_alone, boot_cells, boot_load and boot_timer.
@task_item n=13 part=touches
decision/graph-drives-confirm-every-write: the shared graph drives gain the order page (type 12, click Submit), exercising its two registered writes.
@task_item n=14 part=touches
decision/table-tests-count-their-rows: the stand tables' row counts moved (cross-OS 54, served lists 7, actionable and handlers at boot 7, graph drives and unexercised 8).
@task_item n=15 part=touches
decision/count-and-rename-sweeps: a count sweep kept a test name holding six because another check pins it as a witness string, and renamed the six-app cold-agent test to seven.
@task_item n=16 part=touches
decision/root-boundary-err-on-own-session: the order page's Submit Err is caught by the page's own error boundary, read on its own session, as a boundary own error item.
@task_item n=17 part=touches
decision/host-stderr-no-screen-content and decision/sink-drops-third-party-targets: a new host_log check reads the order page served at trace with neither its message nor its rule on stderr, its info record dropped by the sink.
@task_item n=18 part=touches
decision/stand-checks-in-both-layout-modes and decision/stand-checks-bundled-font: the new order stand checks run in both layout modes under the bundled font.
@task_item n=19 part=touches
decision/stand-boots-offline: boot_order boots offline with the options given.
@task_item n=20 part=touches
decision/zero-failure-stand-repeat, decision/requirement-witness-inside-stand-repeat and decision/one-test-target-per-file: the new stand order target joins stand-repeat by its stand_ prefix as one target per file.
@task_item n=21 part=touches
decision/cold-agent-tests-offline and decision/mutation-controlled-tests: the offline fake gains a caused scenario key, and controls (a)-(c) on the unittest and (d) on the boundary were read red-before-green.
@task_item n=22 part=touches
decision/prompt-goal-control-phrase-once: goal 7's phrase is controlled, each of the three goal phrases asserted to stand once in the prompt.
@task_item n=23 part=touches
decision/capability-check-names-held-to-schema and decision/capabilities-doc-pinned-to-schema: the capability statement's served apps and boot actionable counts moved to seven; no driver schema changed.
@task_item n=24 part=touches
decision/engine-items-asserted-on-tracing-build: the order page's log caused item is asserted on the stand, its target seven_guis::tasks::order, fields empty.
@task_fact n=1 slug=3-test-harness-contract
fact: The cold-agent verdict and its run.end event carry caused_by_kind, an object of the three closed words panic, error and log to the count of accepted acting answers whose caused list holds an item of that kind; a non-JSON answer is skipped and nothing of an item but its kind is read.
@task_fact n=2 slug=3-test-harness-contract
fact: stand::boot_order boots the order page offline in the task shell with the options given, the seventh served task's stand boot.
@task_fact n=3 slug=integration-test-strategy
fact: A caused item carries the owner the UI graph reads and no other; a page keyed by HTML ids under no keyed element reads no owner, so its caused items carry none, and the check pins the graph's reading rather than an asserted owner.
@task_fact n=4 slug=test-strategy
fact: A count sweep keeps a test name holding a count word when another check pins that name as a witness string; the name is searched before any rename.
@task_fact n=5 slug=3-test-harness-contract
fact: The served order page is where a driver and a cold agent meet each caused kind: Submit with a quantity not a whole number from 10 to 20 answers one error item under its own boundary, a good Submit one info log item, Export one panic item with its file and line.
@task_item n=1 part=untouched
WPT runner, its CLI, outputs, statuses and conformance decisions: nothing of the WPT suites moved.
@task_item n=2 part=untouched
Coverage floors, mutation testing and the complexity ceiling: no floor, budget or mutation setting changed.
@task_item n=3 part=untouched
CI workflow jobs, triggers, caches, actions pins, package installs and leg membership: no workflow or leg script was edited; the macOS and iOS no-runner cancels were infra with no cause in the tree.
@task_item n=4 part=untouched
Harness construction, settle, virtual clock, input and screenshot contracts: no harness API changed.
@task_item n=5 part=untouched
Session wire, protocol, socket, recording, state directory and session error contracts: none moved.
@task_item n=6 part=untouched
Driver refusal causes, verbs, argument kinds, lint defects and snapshot node contracts: no cause, verb, argument, defect or node key changed.
@task_item n=7 part=untouched
Mocking discipline, network stand-ins, timer ticks and the advance command: untouched.
@task_item n=8 part=untouched
The windowed end-to-end absence and the mocking-library absence stand as they are.
@task_item n=9 part=untouched
Agent-run verbs, selections, events and exit codes: the agent-run contract is apart from the cold-agent pipe and did not move.
