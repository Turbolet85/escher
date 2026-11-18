@task_prose part=session
The session added a seventh served stand app, the order page, served by escher-session as order, whose controls on purpose return an error under the page's own error boundary, panic, and write one info log record, so a driver and a cold agent meet each kind of caused item.
The order page's texts are fixed program text and write no field: the quantity rule as the error message, order placed as the log message, export is not built yet as the panic payload.
The cold-agent pipe now counts caused items by kind (panic, error, log) from each accepted acting answer, reading nothing of an item but its kind, and the verdict and its run.end event carry these counts as caused_by_kind. capability-unused also fires when any of the three counts is 0. The fixed prompt names seven apps and a goal 7.
seven_guis now takes tracing as a workspace-inherited dependency. The lockfile gains one edge and no new package.
Measured: a caused item from the order page carries no owner, because the UI graph reads no owner for any of its controls. A trace-level served run logs neither the error message nor the rule on stderr.
After implement, the Submit handler's return type was spelled std::result::Result, because an android-only Result alias reached through the dioxus-native prelude broke the android build. CI then read green on every OS.
@task_file n=1 path=examples/seven_guis/src/tasks/order.rs
what: the order page component, a stand app whose controls return an error, panic and log fixed texts; its own error boundary and the order-* element ids (new, then edited after implement for the Submit return type)
@task_file n=2 path=examples/seven_guis/src/tasks/mod.rs
what: the stand apps' module list (edited: the order module added)
@task_file n=3 path=examples/seven_guis/src/stand.rs
what: the headless stand boot functions, each served app booted offline (edited: boot_order added)
@task_file n=4 path=examples/seven_guis/src/session_host.rs
what: the escher-session host's served-task table, which the usage line and the MCP start tool's task enum are read from (edited: six to seven tasks, order added)
@task_file n=5 path=examples/seven_guis/Cargo.toml
what: the seven_guis app crate's manifest, its dependency set (edited: tracing taken from the workspace)
@task_file n=6 path=Cargo.lock
what: the workspace lockfile under locked builds (edited: one tracing edge for seven_guis, no new package)
@task_file n=7 path=examples/seven_guis/tests/cli_commands.rs
what: the escher-session command-line tests, covering the served tasks and the usage line (edited: seven tasks)
@task_file n=8 path=examples/seven_guis/tests/host_log.rs
what: the session host's stderr log tests, covering the telemetry sink's dropping and redaction (edited: the order page served at trace logs neither its message nor its rule)
@task_file n=9 path=tests/blitz-tests/tests/stand_order_served.rs
what: the order page's stand checks, covering each control's caused answer across the session, serve and MCP carriers, and kept-check replay (new)
@task_file n=10 path=tests/blitz-tests/tests/common/mod.rs
what: the stand tests' shared tables of actionable counts, handlers, graph drives and stated tab orders (edited: order rows)
@task_file n=11 path=tests/blitz-tests/tests/stand_actions.rs
what: the actions answer stand check over every served app (edited: seven apps)
@task_file n=12 path=tests/blitz-tests/tests/stand_contrast.rs
what: the stand contrast check (edited: the order page's ratios)
@task_file n=13 path=tests/blitz-tests/tests/stand_cross_os_readings.rs
what: the cross-OS stand readings guard, comparing answer digests on every CI OS (edited: 48 to 54 rows)
@task_file n=14 path=tests/blitz-tests/tests/stand_explore.rs
what: the exploration stand check over every served app (edited: seven apps)
@task_file n=15 path=tests/blitz-tests/tests/stand_graph.rs
what: the graph answer stand check (edited: an order drive)
@task_file n=16 path=tests/blitz-tests/tests/stand_handler_coverage.rs
what: the reached-handler coverage stand check (edited: order handlers)
@task_file n=17 path=tests/blitz-tests/tests/stand_keyboard.rs
what: the stated tab order keyboard walk (edited: the order page and its failure state)
@task_file n=18 path=tests/blitz-tests/tests/stand_layout.rs
what: the layout answer stand check over every served app (edited: seven apps)
@task_file n=19 path=scripts/cold-agent.sh
what: the cold-agent pipe, which holds the fixed prompt, the call and task allowlist, the verdict and its reasons, and the run metadata events (edited: caused_by_kind counted, capability-unused extended, seven apps)
@task_file n=20 path=.github/scripts/test_cold_agent.py
what: the cold-agent pipe's contract tests and its fake escher-session (edited: caused scenario key, seven apps, 42 tests)
@task_file n=21 path=docs/capabilities.md
what: the committed capability statement (edited: seven served apps, the order page as where caused is met)
@task_file n=22 path=README.md
what: the README's 0.3.0 status and its served-app slugs (edited: order added)
@task_item n=1 part=touches
decision/cold-agent-screen-to-provider: on the live run the order page's authored caused texts (the panic payload, the error message and the log message) now go to the model provider beside the screen. This is the founder's F5 widening, and the row owes it.
@task_item n=2 part=touches
decision/caused-text-crossing: the order page brings a new source of caused text. The error item's message is the quantity rule, the log item's message is order placed and the panic item's payload is export is not built yet. All are fixed program text with no field.
@task_item n=3 part=touches
decision/boundary-widening-rule: the widening (a seventh served app whose authored error text crosses to the model provider) rests on the founder's F5 relay.
@task_item n=4 part=touches
decision/cold-agent-events-metadata-only: the run.end event now holds caused_by_kind, which maps each closed kind word to a count.
@task_item n=5 part=touches
decision/cold-agent-logs-metadata-only: the verdict the pipe writes gains caused_by_kind, still counts of closed words only.
@task_item n=6 part=touches
decision/cold-agent-check-reader-counts-only and decision/cold-agent-checks-read-for-counts-only: the new caused-by-kind reader decodes each accepted acting answer and reads only each caused item's kind. A non-JSON answer is skipped.
@task_item n=7 part=touches
decision/cold-agent-fixed-prompt: the fixed prompt now names seven apps and adds goal 7, to read what an action caused to find out why.
@task_item n=8 part=touches
decision/cold-agent-call-validation: the task allowlist gains order, so a seventh app's done state is judged.
@task_item n=9 part=touches
decision/cold-agent-wrong-call-bar: the verdict's capability-unused reason now also fires when panic, error or log reads 0. This is a rule over the same verdict.
@task_item n=10 part=touches
decision/other-log-targets-dropped, decision/outside-target-dropped and decision/log-sink-redacts-and-allowlists: the order page writes one info record under the app target seven_guis::tasks::order. It is captured into caused and dropped whole by the stderr sink, and this is pinned at trace level.
@task_item n=11 part=touches
decision/no-screen-content-in-logs and decision/session-output-holds-no-caller-input: a served order run at trace logs neither the error message nor the rule on stderr. The panic payload appears only on std's own panic line.
@task_item n=12 part=touches
decision/root-boundary-err-crossing: a returned Err now also crosses from a page's own error boundary (boundary own), with a Retry that clears it, beside the root boundary.
@task_item n=13 part=touches
decision/workspace-inherited-dependency-versions, decision/tracing-for-engine-logging and decision/tracing-feature-propagates: an app crate now takes tracing from the workspace itself.
@task_item n=14 part=touches
decision/locked-builds: the lockfile gains one edge and no package. Locked builds stay green.
@task_item n=15 part=touches
contract/escher-session-exit-codes/2 and contract/cli-exit-codes/2: the usage message's apps list now names seven, order included.
@task_item n=16 part=touches
decision/mcp-listing-fixed-text and contract/mcp-server/help: the MCP start tool's task enum now names order, and the listing stays fixed program text.
@task_item n=17 part=touches
contract/actions-answer/owner and decision/caused-text-crossing: it was measured that a caused item carries no owner where the UI graph reads none. Every order-page control reads no owner. The rows' first reached handler stands.
@task_item n=18 part=touches
contract/stable-element-id/author-key and decision/stable-id-exits: six new author ids are read as stable ids on every carrier: order-quantity, order-submit, order-retry, order-failed, order-export and order-status.
@task_item n=19 part=touches
decision/ci-matrix-runs-every-job: the CI matrix's android job caught a compile red that the host OSes never see. The macOS job went five attempts with no runner before attempt 6 ran it green.
@task_fact n=1 slug=threat-model-summary
fact: The cold-agent live run sends the order page's authored caused texts to the model provider: the panic payload export is not built yet, the error message (the quantity rule) and the log message order placed. Each is fixed program text with no field, admitted by the founder's F5 widening.
@task_fact n=2 slug=logging-monitoring
fact: The cold-agent verdict and its run.end event carry caused_by_kind, a count for each closed kind word panic, error and log, taken from the accepted acting answers, and nothing of a caused item but its kind is read.
@task_fact n=3 slug=dependency-security
fact: An app crate that logs declares tracing itself, workspace-inherited, because the dioxus-native prelude re-exports none.
@task_fact n=4 slug=error-handling
fact: A served stand page may hold its own error boundary. A returned Err there crosses as an error caused item with boundary own, and Retry clears it. A control's panic crosses as a panic caused item with its file and line.
@task_item n=1 part=untouched
Authentication and authorization: no auth, token, publish guard or CI secret moved.
@task_item n=2 part=untouched
API security's outbound side (blitz-net and reqwest fetches, per-host limits, load waits, timeouts, CORS and CSP absences): the order page fetches nothing and adds no listener.
@task_item n=3 part=untouched
The driver verb schema, argument kinds, refusal causes, session errors, exit-code meanings, node keys, span fields and MCP protocol errors: none changed.
@task_item n=4 part=untouched
Masked control values, password painting, select list crossing and select keys: the order page has no masked or select control.
@task_item n=5 part=untouched
Data protection's browser stores (history in sqlite, cookies, the HTTP cache, TLS, clipboard, file dialog) and kept check file contents and lifecycle: unchanged.
@task_item n=6 part=untouched
Secret management (RUST_LOG as the only environment read, live runs on the operator's login, no credentials in evidence): unchanged. The doc secret search read no hit.
@task_item n=7 part=untouched
WPT runner, HTML parsing and CSS parsing rows: not reached.
