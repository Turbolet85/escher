@task_prose part=session
The session rewrote the committed capability statement for Escher 0.3.0: what it can do, its stated limits, its a11y reach and where each claim is proven.
It added three sections with their witnesses (acting-verb errors and their caused text, seeded randomness, bundled font families and cross-OS readings) and four 0.3.0 limits, each with its standing and owner, and retired the stale layout-carrier line from Not built.
It re-measured the a11y leg (eighteen files) and the stand-repeat leg (seventeen named targets) on CI and pinned both counts in a new CI-script check, so a change of either leg's membership reddens the ci-scripts leg until the page is re-stated.
It re-pointed the cargo-deny advisory ignores' owner clauses from a line of the residuals file to the owning residual rows by id; the six ignore IDs are unchanged and the audit reads advisories ok, licenses ok.
It flipped the README's 0.3.0 status to built, its cold-agent run to come, and removed stale counts from four harness leaf pages; no product code, carrier, verb, cause, field, span or crossing changed.
@task_file n=1 path=docs/capabilities.md
what: the committed capability statement, the page that states the MCP crossing, the carried set, the limits and where each claim is proven
@task_file n=2 path=README.md
what: the project README, its status section naming each version's stages and the cold-agent run
@task_file n=3 path=deny.toml
what: the cargo-deny configuration, its advisory ignores by ID with a written reason and owner, and the licence gate
@task_file n=4 path=.github/scripts/test_capability_statement.py
what: the CI-script check that pins the capability statement's counts and names to the driver schema and now to the CI legs
@task_file n=5 path=.claude/rules/a11y.md
what: the accessibility rule page for the harness, stating the a11y leg's reach
@task_file n=6 path=.claude/docs/a11y-summary.md
what: the harness summary of accessibility checks and the a11y leg
@task_file n=7 path=.claude/docs/tests-summary.md
what: the harness summary of the test legs and the stand checks, including the select list checks
@task_file n=8 path=.claude/docs/services/seven_guis.md
what: the harness page for the seven_guis stand service and its checks
@task_item n=1 part=touches
decision/capability-statement-mcp-crossing: the committed statement of the MCP crossing was rewritten; its carried set now names an acting verb's caused items, with no carrier added.
@task_item n=2 part=touches
decision/caused-text-crossing: the page now states as a limit that caused carries an app's own text as its author wrote it, its scrub pinned by a stand check, and that engine items reach caused only on a tracing build.
@task_item n=3 part=touches
decision/advisory-ignores-by-id and decision/audit-ignores-per-id-with-reason: each advisory ignore's reason now names its owner as a residual row by id rather than a line of the residuals file; IDs and other words unchanged.
@task_item n=4 part=touches
decision/cargo-deny-audit-tool, decision/dependency-audit-cargo-deny, decision/dependency-audit-ci-gate and decision/licence-gate: the audit tool's configuration was edited and the audit leg re-read green (advisories ok, licenses ok).
@task_item n=5 part=touches
decision/no-git-dependencies and decision/no-git-dependency: the statement's stack sentence now says Taffy comes from upstream's git source pinned to one revision, as the manifest pins it; a picker should weigh this against the no-git-dependency rows.
@task_item n=6 part=touches
decision/seeded-source-in-process-seed-only: the page now states a seed in process only as a limit: no verb, argument, flag, MCP tool field, kept-check key or span carries a seed.
@task_item n=7 part=touches
decision/dioxus-render-into-buffer: the page now states the buffered render's same-render limit, asserted by no check and owned by a residual row targeted next.
@task_item n=8 part=touches
decision/inert-subtree and decision/inert-focus-clear-silent: the page now names the hidden-focus gap (a focused control that no longer shows keeps focus) as a stated limit beside the inert subtree, re-carried as a residual for next.
@task_item n=9 part=touches
decision/cold-agent-wrong-call-bar and decision/cold-agent-screen-to-provider: the README now states 0.3.0 built with its cold-agent run still to come; no 0.3.0 cold-agent result is claimed.
@task_item n=10 part=touches
contract/driver-verbs/graph, contract/driver-verbs/actions and contract/driver-verbs/layout: the README's stage lines now name the graph, actions and layout commands; the verbs themselves are unchanged.
@task_item n=11 part=touches
decision/session-layout-read-in-process: the page retired its Not built line calling a carrier for the layout read provisional, false since layout went on the carriers.
@task_item n=12 part=touches
decision/select-list-crossing and decision/open-list-shot-pixels: the select list checks are now among the a11y leg's files and the stand-repeat named targets; the crossing claims stand unchanged.
@task_fact n=1 slug=dependency-security
fact: Each cargo-deny advisory ignore's reason names its owner as a residual row by its id, and no product file names the residuals file.
@task_fact n=2 slug=threat-model-summary
fact: The capability statement states each 0.3.0 limit with its standing and owner: caused carries app text as authored, engine items in caused only on a tracing build, the buffered render's same-render limit, and a seed in process only.
@task_item n=1 part=untouched
The driver session socket, its request grammar, bounds, replies and lifecycle: no product code changed.
@task_item n=2 part=untouched
The MCP server's transport, request and result bounds, protocol errors and tool results.
@task_item n=3 part=untouched
Refusal causes, refusal JSON, session error tables and exit codes.
@task_item n=4 part=untouched
Stable element ids, snapshot text, masked control values and the value mask on every carrier.
@task_item n=5 part=untouched
Kept check files, recorded checks, the recording directory and the cold-agent harness scripts.
@task_item n=6 part=untouched
Logging and telemetry: the stderr sink, target allowlists, span lines and content scrubs.
@task_item n=7 part=untouched
Data protection: history in sqlite, cookies, the HTTP cache, TLS, clipboard and file dialogs.
@task_item n=8 part=untouched
Outbound HTTP, script fetchers, URL schemes, per-host limits and the absences of CORS, CSP and response limits.
@task_item n=9 part=untouched
CI tokens, publish secrets, signing material and WPT workflows.
