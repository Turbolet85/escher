@task_prose part=session
The per-step capture behind an answer's caused items now hides fields by its own set of eleven names, the twelve content names less error, so a caused item's error field crosses every answer carrier (command line, socket call, MCP, a kept check's kept answer) as its author wrote it, cut at the text bound.
The sink's lines are unchanged: its twelve content names, error among them, still print [redacted], and a failed fetch's error text reads 0 times on the sink's lines.
The caused result-field help served by help and as each acting MCP tool's description was rewritten, six lines replacing six, error no longer among the names that read [redacted].
The error-text stand check grew from 7 to 8 tests (2 ignored): an app parse-warn step and an engine failed-fetch case on the tracing build, the carrier cases and the kept check's calls extended to it; the telemetry lib tests 25 to 26.
CI read red once at the Documentation job (a public doc comment linking a crate-private item), reproduced on the host by the doc leg and fixed; the local gate block had run no doc leg, as the plan listed none.
No verb, argument, cause, exit, carrier, log field, span, target or crate was added or retired.
@task_file n=1 path=packages/escher-telemetry/src/capture.rs
what: edited — the telemetry per-step capture that reads each logged record into an answer's caused fields and judges a field by name; now holds the crate-private answer hidden-field set and its unit table test
@task_file n=2 path=packages/escher-telemetry/src/format.rs
what: edited — the telemetry sink's line formatter, its event rule and span-field store reading the content-field names that print [redacted]
@task_file n=3 path=packages/escher-driver/src/schema.rs
what: edited — the driver schema: the verb table and the help text served by help and as each MCP tool's description, here the caused result-field help
@task_file n=4 path=packages/escher-driver/src/caused.rs
what: edited — the driver's caused items of an acting call result, the records a step logged; doc text only
@task_file n=5 path=tests/blitz-tests/tests/stand_act_error_text.rs
what: edited — the stand integration check that each failure answers what failed, where and from which source, across command line, MCP in process, the session socket and a kept check, with a failing network stand-in
@task_item n=1 part=touches
A row holding that a caused item or an acting answer reads every content-named field as [redacted], or echoes no typed or logged text, now has error as its exception: error crosses as written, possibly holding a failed fetch's URL; the other eleven names still read [redacted].
@task_item n=2 part=touches
decision/engine-items-asserted-on-tracing-build: a new engine caused-item case (a failed fetch's error text in the engine's error field) is asserted only on the tracing build, the package-alone build reading no engine item.
@task_item n=3 part=touches
decision/mcp-result-decoded-whole: the MCP-in-process caused comparison now also compares the parse-warn item's error text.
@task_item n=4 part=touches
contract/driver-session/act, contract/session-run/changed-node-states, contract/driver-session/run-carriers: the caused field of an acting call result now carries error text as written on every carrier.
@task_item n=5 part=touches
contract/mcp-server/help, contract/mcp-help/parity, contract/cli-help/output, contract/escher-session-command-line/help, contract/mcp-server/tools-list, contract/mcp-tools-list/order, contract/tool-call-span/help: the help text and the acting tools' descriptions changed in content (same line count); help and the tool listing still the same bytes on both surfaces.
@task_item n=6 part=touches
decision/capability-statement-pinned, decision/capabilities-doc-pinned-to-schema, decision/capability-statement-from-schema, decision/capability-check-names-held-to-schema: the schema's help text moved; a row pinning a statement to the schema's text is reached indirectly.
@task_item n=7 part=touches
Rows over the sink and stand logs (decision/telemetry-target-allowlist, decision/sink-drops-third-party-targets, decision/no-ids-or-names-in-stand-logs, decision/session-logs-no-screen-content, decision/host-stderr-no-screen-content, decision/log-silence-read-on-a-sink-host): the sink stays as it was, now asserted to hold error=[redacted] and the error text 0 times, while the answer carries the text — capture and sink no longer share one name set.
@task_item n=8 part=touches
decision/sink-capture-counted-by-target: the failed-fetch check finds the engine's sink line by its target blitz_dom::document and the error=[redacted] pair, never by the message.
@task_item n=9 part=touches
decision/recorded-checks-replayed-as-tests, contract/check-replay/stop: the kept check's calls went 3 to 4 and its changed-cut stop moved from step 2 to step 3, the kept text holding the parse error.
@task_item n=10 part=touches
decision/hand-written-stand-ins, decision/network-stand-ins-counted, decision/real-provider-inside-stand-in: a hand-written failing network provider stand-in was added for the failed-fetch case.
@task_item n=11 part=touches
decision/failed-load-ends-settle-wait, contract/harness-settle/failed-load: the failed-fetch case reads waiting_nodes 0 after a failed load.
@task_item n=12 part=touches
decision/stand-checks-in-both-layout-modes, decision/layout-tests-in-both-layout-modes: the parse-warn and failed-fetch cases run in both layout modes.
@task_item n=13 part=touches
decision/table-tests-count-their-rows, decision/expected-value-pinned-as-literal: the capture's name table test holds 12 literal rows, eleven names [redacted] and error its text.
@task_item n=14 part=touches
decision/mutation-controlled-tests: three named controls held red-before-green, one of them showing the tracing build ran the engine branch.
@task_item n=15 part=touches
decision/typed-password-never-leaves, decision/typed-password-never-returned: password stays among the eleven hidden names; no change, but the hidden set it sits in was renamed for the answer.
@task_item n=16 part=touches
decision/local-pre-push-gate, decision/ci-legs-host-reproducible, decision/workspace-tests-in-fast-ci-leg: the fast leg runs no rustdoc, so a private intra-doc link in a public doc comment read green on every local gate and red only in CI's Documentation job; the doc leg reproduced it on the host.
@task_item n=17 part=touches
decision/one-test-target-per-behaviour, decision/one-test-target-per-file, decision/blitz-tests-one-target-per-file: the new tests joined an existing target; no target added.
@task_item n=18 part=touches
decision/assertions-print-no-argument-value, decision/checks-hold-no-private-data: the new assertions check sentinel values absent from lines without printing them.
@task_fact n=1 slug=9-ci-integration
fact: A chunk that edits a public doc comment lists the doc leg in its local gate block, since the fast leg runs no rustdoc and a broken or private intra-doc link otherwise reads red only in CI's Documentation job.
@task_item n=1 part=untouched
WPT runner, its CLI, statuses, pinned commit and conformance reporting: no change reaches them.
@task_item n=2 part=untouched
Cold-agent pipe, its events, verdict, exit codes and isolation: untouched.
@task_item n=3 part=untouched
agent-run verbs, selections, events, state and exit codes: untouched.
@task_item n=4 part=untouched
Coverage floors, mutation baseline, complexity ceiling and the zero-failure stand-repeat budget: no threshold moved; stand-repeat read 3 of 3 rounds green.
@task_item n=5 part=untouched
CI workflow shape: actions pinning, secrets, permissions, caches, apt installs, trigger branches and artifacts are unchanged.
@task_item n=6 part=untouched
Harness construction, input helpers, inspection, contrast and screenshot contracts: untouched.
@task_item n=7 part=untouched
Session wire, protocol versions and bounds, session state directory, session errors and exit status: untouched.
@task_item n=8 part=untouched
Lint defects and findings, snapshot node and snapshot budget, handler coverage: untouched.
@task_item n=9 part=untouched
Committed recorded check files and their byte-equality to recordings: no recording changed; cross-OS digests did not move.
