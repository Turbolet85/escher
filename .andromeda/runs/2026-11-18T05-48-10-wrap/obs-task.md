@task_prose part=session
The session rewrote the capability statement for Escher 0.3.0 on its build branch: what it can do, its stated limits, its accessibility reach and where each claim is proven.
It added three sections with their witnesses (acting-verb errors and their caused items, seeded randomness, bundled font families and cross-OS readings) and four 0.3.0 limits, and retired the false line that the layout read has no carrier.
It re-measured the a11y leg (eighteen files, 18 · 163 · 0 · 0) and the stand-repeat leg (seventeen named targets, 64 files, 273 · 1467 · 0 · 30) on one green CI run, and restated the witness tables (49 and 74 witnesses).
The capability statement's CI-script pin now also holds the a11y leg's file count and stand-repeat's named-target count, read from the CI leg script, so a change of either leg's membership reddens the ci-scripts leg until the page is restated.
The README's 0.3.0 status now reads built with its cold-agent run to come, 16 of 17 capabilities verified headlessly; the advisory ignore list's owner clauses now name residual rows; four harness leaf pages lost stale counts.
No product code changed: no Rust, no carrier, verb, cause, field, span, crossing, sink or log line.
@task_file n=1 path=docs/capabilities.md
what: the capability statement — what Escher can do, its limits, its a11y reach and the claim map naming where each claim is proven; edited (rewritten for 0.3.0)
@task_file n=2 path=README.md
what: the project README and its version status section; edited
@task_file n=3 path=deny.toml
what: the cargo-deny policy, its advisories ignore list and each ignore's owner clause; edited
@task_file n=4 path=.github/scripts/test_capability_statement.py
what: the capability statement's pin, a CI-script check run by the ci-scripts leg that holds the page's counts to the driver schema's tables and now to the CI leg script; edited
@task_file n=5 path=.claude/rules/a11y.md
what: the accessibility rule page of the harness; edited (counts removed)
@task_file n=6 path=.claude/docs/a11y-summary.md
what: the harness's accessibility summary page; edited (counts removed)
@task_file n=7 path=.claude/docs/tests-summary.md
what: the harness's tests summary page, listing the test targets and legs; edited (counts removed)
@task_file n=8 path=.claude/docs/services/seven_guis.md
what: the harness's service page for the seven_guis example app and its stand; edited (counts removed)
@task_item n=1 part=touches
The decision that the capability statement is updated by hand: its pin now also reads two counts (the a11y leg's file count and stand-repeat's named-target count) from the CI leg script, so less of it rests on hand upkeep; check whether the row's why or its claim of what no check reads is now stale.
@task_item n=2 part=touches
The capability statement now states as a 0.3.0 limit that the caused items of an acting verb carry an app's own text as its author wrote it, its scrub pinned by a stand check and the crossing asserted by no check; rows on scrubbing by field name, the content-named set redacted on escher targets and the telemetry scrub set's error field rule over what that text may reach.
@task_item n=3 part=touches
The capability statement now states as a limit that engine-sourced items in caused appear only on a tracing build; rows on engine tracing behind a cargo feature or set by the build's invocation rule over that build.
@task_item n=4 part=touches
The capability statement now states a seed held in process only: no verb, argument, flag, MCP tool field, kept-check key or span field carries it; rows on the closed fields of the command span and the tool_call span and on driver spans carrying no content sit beside that statement.
@task_item n=5 part=touches
The capability statement retired its line that the layout read has no carrier (false since layout moved onto the carriers); the decision that the layout read opens no span and the command span's layout field describe that same read.
@task_item n=6 part=touches
The README's 0.3.0 status now reads built with its cold-agent run to come and names the graph, actions and layout commands; rows on the cold-agent verdict, its counted verbs and graph, actions and layout not being counted for a cold agent rule over that pending run.
@task_item n=7 part=touches
The capability statement and the README now state the version as 0.3.0; the sink's service.version resource key and log-line field hold the product version, which is the value the page now names.
@task_item n=8 part=touches
The ci-scripts CI leg gained a check; rows on the per-leg CI log uploaded on failure and on CI on GitHub workflows rule over that leg, which now reddens on a leg membership change until the page is restated.
@task_fact n=1 slug=pii-scrubbing-compliance
fact: The caused items of an acting verb carry an app's own messages and fields as their author wrote them; only the field-name scrub applies, pinned by a stand check, and no check asserts the crossing of app text itself.
@task_fact n=2 slug=span-trace-coverage
fact: A randomness seed is held in process only; no span field, verb argument, MCP tool field or kept-check key carries it.
@task_item n=1 part=untouched
The telemetry sink's line format, stream, RUST_LOG handling, target allowlist and escaping: no sink, subscriber or log line changed.
@task_item n=2 part=untouched
The driver's command and tool_call span fields, their order and their ending values: no span or field was added, renamed or removed.
@task_item n=3 part=untouched
Refusal cause names, the escher-session exit codes, host wire and MCP role stdout: no verb, cause or command changed.
@task_item n=4 part=untouched
The cold-agent and agent-run harness events, transcripts, kept checks and recording directory rules: no cold-agent run was made and no harness event changed.
@task_item n=5 part=untouched
Script error surfaces, window error events, WPT report messages and the panic hooks: no engine or script code changed.
@task_item n=6 part=untouched
The absences of a telemetry backend, metrics exporters, metric emission and an error-reporting service: none was added.
@task_item n=7 part=untouched
Snapshot, select, optgroup, graph, actions and layout answer content rows: no answer shape or content changed.
