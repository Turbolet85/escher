@task_prose part=session
Added a seventh served stand app, order (OrderPage), whose controls cause the three kinds a caused list holds: Submit with a bad quantity returns an error caught by the page's own ErrorBoundary, a good Submit writes one info log record with no field under the target seven_guis::tasks::order, and Export panics with a fixed message.
The served-task set moved six to seven: escher-session's served tasks, its usage line and the MCP start enum name order; the headless stand gains boot_order.
The cold-agent pipe gains the verdict field caused_by_kind (counts per closed kind word panic, error and log, read from the acting answers, nothing of an item but its kind), also on the run.end event; capability-unused now also fires when any of the three reads 0; the prompt names seven apps and a goal 7.
Measured: a caused item from the order page reads no owner, as the UI graph reads no owner for any of its controls; the order page's log record is dropped by the stderr sink and logs neither its message nor its rule at trace.
The example app crate now takes the tracing crate itself, as no prelude re-exports it; a CI-found android compile red was fixed by spelling std::result::Result in Submit's handler.
@task_file n=1 path=examples/seven_guis/src/tasks/order.rs
what: the order page, the seventh served stand app, new: its controls panic, return an error under its own error boundary, and write one fixed-text info log record under its own target
@task_file n=2 path=tests/blitz-tests/tests/stand_order_served.rs
what: the order page's stand checks, new: keyed at boot with no lint finding, each control's caused answer by kind, the same caused across the MCP role in process, and a kept check replaying
@task_file n=3 path=examples/seven_guis/Cargo.toml
what: the example app crate's manifest; gains the tracing crate as a dependency
@task_file n=4 path=Cargo.lock
what: the workspace lock; one dependency edge added, no new package
@task_file n=5 path=examples/seven_guis/src/tasks/mod.rs
what: the stand's task module list; registers the order page
@task_file n=6 path=examples/seven_guis/src/stand.rs
what: the headless stand's boot functions; gains boot_order for the order page in the task shell
@task_file n=7 path=examples/seven_guis/src/session_host.rs
what: escher-session's served-task table, which the usage line and the MCP start enum name; six to seven with order
@task_file n=8 path=examples/seven_guis/tests/cli_commands.rs
what: the escher-session command checks; the served-task list and the usage line's apps assertion to seven
@task_file n=9 path=examples/seven_guis/tests/host_log.rs
what: the sink-installing host's log checks read at trace; a new check that the order page logs neither its message nor its rule on stderr
@task_file n=10 path=tests/blitz-tests/tests/common/mod.rs
what: the stand's shared tables: actionable and handlers at boot, graph drives, unexercised after drives, and the stated tab orders; each gains the order page
@task_file n=11 path=tests/blitz-tests/tests/stand_actions.rs
what: the actions answer check over every served app; its served list to seven
@task_file n=12 path=tests/blitz-tests/tests/stand_contrast.rs
what: the stand's asserted contrast check; gains the order page's ratios
@task_file n=13 path=tests/blitz-tests/tests/stand_cross_os_readings.rs
what: the cross-OS stand readings by answer digest; rows 48 to 54 with the order page's
@task_file n=14 path=tests/blitz-tests/tests/stand_explore.rs
what: the auto-exploration check over every served app; its served list to seven
@task_file n=15 path=tests/blitz-tests/tests/stand_graph.rs
what: the UI graph answer check over the stand; gains the order page's drive
@task_file n=16 path=tests/blitz-tests/tests/stand_handler_coverage.rs
what: the handler coverage check over the stand; its doc to seven apps
@task_file n=17 path=tests/blitz-tests/tests/stand_keyboard.rs
what: the stated tab order walk; gains the order page's order and its failure-shown order reaching Retry
@task_file n=18 path=tests/blitz-tests/tests/stand_layout.rs
what: the layout answer check over every served app; its served list to seven
@task_file n=19 path=scripts/cold-agent.sh
what: the cold-agent pipe: its counted verbs, its verdict and reasons, its run.end event and its prompt; gains caused_by_kind, capability-unused over it, seven apps and goal 7
@task_file n=20 path=.github/scripts/test_cold_agent.py
what: the cold-agent pipe's contract tests and its fake escher-session; a caused scenario key, the caused-by-kind count test, seven apps
@task_file n=21 path=docs/capabilities.md
what: the capability statement, updated by hand; seven served apps and the order page as where a driver and a cold agent meet caused
@task_file n=22 path=README.md
what: the project README's status; the served apps' slugs to seven
@task_item n=1 part=touches
The cold-agent verdict's capability-unused, in both the span-trace and the log coverage areas: its set gains caused_by_kind, so it also fires when any of panic, error or log reads 0.
@task_item n=2 part=touches
The cold-agent verdict's closed field set: a new field caused_by_kind, an object of the three closed kind words to counts, beside refused_by_cause and the call counts.
@task_item n=3 part=touches
The cold-agent pipe's run.end event: it now carries caused_by_kind beside the verdict's other counts.
@task_item n=4 part=touches
The cold-agent verdict holds no content and the cold-agent events carry no user content: caused_by_kind reads nothing of a caused item but its kind, counts of closed words only, under the same rule.
@task_item n=5 part=touches
escher-session's exit code 2, the usage line naming the served apps: it now names seven, order the seventh; the help command's listing of apps likewise.
@task_item n=6 part=touches
The MCP role's start tool: its app enum gains order, with no command-line edit.
@task_item n=7 part=touches
Escher crates and app crates taking tracing: the example app crate now takes the tracing crate ungated itself, since no prelude re-exports it, and writes one info event under its own target.
@task_item n=8 part=touches
The sink's target allowlist and the stderr sink: the order page's info record under seven_guis::tasks::order is dropped by the sink, and the sink-installing host read at trace logs neither its message nor its rule.
@task_item n=9 part=touches
The capture that counts records into caused: it reads the order page's info record from the app's own target into a caused log item, beside the framework targets read at warn and above.
@task_item n=10 part=touches
The headless stand runs with no sink and its boot list: the stand gains a seventh boot, the order page.
@task_item n=11 part=touches
Exploration catches a panicking call and the panic event: the order page's Export panics by design with a fixed message, answered as a panic item with its file and line.
@task_item n=12 part=touches
The actions answer's owner and the UI graph answer: a page keyed by HTML ids under no keyed element reads no owner, so its caused items carry none.
@task_item n=13 part=touches
Fixed diagnostics and fixed messages at log sites: the order page's error, log and panic texts are fixed strings with no field; the scrub sets are unchanged.
@task_item n=14 part=touches
The capability statement updated by hand: it now names seven served apps, the boot actionable counts with the order page's 4, and where caused is met.
@task_item n=15 part=touches
The cold-agent transcript kept raw: the pipe now decodes each accepted acting answer in it to count caused kinds, skipping a non-JSON answer.
@task_item n=16 part=touches
Cross-OS stand readings by answer digest: the readings table moved 48 to 54 rows with six order rows, green on linux, windows and macOS.
@task_fact n=1 slug=6-log-coverage
fact: The cold-agent verdict holds caused_by_kind, an object of the closed kind words panic, error and log to the count of accepted acting answers (click, type, press, advance, scroll) whose caused list holds an item of that kind, each kind counted once per answer; it is also on the run.end event.
@task_fact n=2 slug=log-coverage
fact: The cold-agent pipe reads nothing of a caused item but its kind, and an acting answer that is not JSON is skipped, not counted.
@task_fact n=3 slug=3-observability-harness-contract
fact: The order page writes one info record with no field under its own target seven_guis::tasks::order on a good Submit; the capture reads it into a caused log item, and the stderr sink drops it.
@task_fact n=4 slug=pii-scrubbing-compliance
fact: A caused item from a component keyed by HTML ids under no keyed element carries no owner, as the UI graph reads no owner for it; the item's owner is the graph's.
@task_fact n=5 slug=telemetry-strategy
fact: An app crate that logs takes the tracing crate itself; no tracing re-export reaches it through the native renderer's prelude.
@task_item n=1 part=untouched
The driver's command span and the MCP tool_call span, their fields, endings and lines: no span, field or verb changed.
@task_item n=2 part=untouched
Refusal cause names and the session error object: no cause or session error was added or changed.
@task_item n=3 part=untouched
Metric emission, metrics exporters, a telemetry backend, OTel export and the sink's resource keys: none added.
@task_item n=4 part=untouched
The script error surface, the window error event, script diagnostics drains and WPT reporting: the order page raises no script error.
@task_item n=5 part=untouched
Recording bounds, the recording file, session state and recording directory privacy, and the session lifecycle: unchanged.
@task_item n=6 part=untouched
CI integration: per-leg log and coverage artifacts and the package-install diagnostic line are unchanged; the new stand checks ride the existing legs by their prefix.
@task_item n=7 part=untouched
Select, optgroup and form-entry rows, snapshot and diff contracts and page script events: the order page holds no select and no script.
@task_item n=8 part=untouched
The escher-session host wire, its exit codes other than the usage line's and the coverage role's line: unchanged.
