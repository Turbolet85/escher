@task_prose part=session
The agent-run harness contract's run verb takes a fourth fixed selection word, session, beside stand, all and a blitz-tests file name.
run session runs the escher-session boot-path flows: cargo test -p seven_guis --locked --test cli_flow, so the packages run reaches move from blitz-tests alone to blitz-tests or seven_guis (seven_guis under session alone).
session is a closed word matched before the file-name arm; an extra argument exits 2, a run before boot exits 3, a failing flow exits 1 with no captured output in stdout or events.jsonl, and an empty session run (0 tests) ends with outcome failed, never a pass.
No new verb, event, event field, state file, env read, listener or process: run session emits the existing run.start, test and run.end events, and status reads the session run; boot is unchanged and still builds blitz-tests alone.
The agent-run contract tests grew from 14 to 20 cases (six session cases) and the ci-scripts leg's unittest count from 112 to 118; the live run session became the first smoke a plan can name as a role=smoke entry (L1's boot-path smoke).
@task_file n=1 path=scripts/agent-run.sh
what: the agent-run harness contract (boot, run, status, cleanup) — its run selections, its usage line, the one cargo test call a run makes and the harness events and state it writes
@task_file n=2 path=.github/scripts/test_agent_run.py
what: the agent-run harness's contract tests — the check that guards the agent-run decisions (events with no captured output, raw output never printed, cleanup scoped) under a cargo shim
@task_item n=1 part=touches
contract/agent-run-events/run-start and contract/agent-run-events/run-end: their selection value now also takes session, and run.end's counts and outcome come from the seven_guis cli_flow target under it; no field is added.
@task_item n=2 part=touches
contract/agent-run-events/status: status now reads a run whose selection is session (state passed with passed 2, failed 0, ignored 0 in the smoke).
@task_item n=3 part=touches
contract/agent-run-events/test and contract/agent-run-test-event/file: under session a test event's file is the seven_guis cli_flow target, not a blitz-tests file; any row saying the file is always a blitz-tests test file is stale.
@task_item n=4 part=touches
decision/agent-run-no-captured-output: holds for the session flows too, now pinned by a contract case that a failing session flow carries no captured output into stdout or events.jsonl; its guard cell (the agent-run contract tests) gains that case.
@task_item n=5 part=touches
decision/agent-run-raw-output-never-printed and decision/agent-run-log-no-user-content: their reach now spans the cargo output of the seven_guis cli_flow run, which drives escher-session; the rule is unchanged.
@task_item n=6 part=touches
decision/agent-run-raw-log-unscrubbed: the raw log now also holds the seven_guis cli_flow cargo output when the selection is session.
@task_item n=7 part=touches
decision/agent-run-cleanup-scoped and decision/session-state-dir-private: the session flows' own state dirs are the cli_flow target's cf-* dirs under CARGO_TARGET_TMPDIR, as before; agent-run adds no state file of its own for them — check that no row says agent-run state covers only blitz-tests runs.
@task_item n=8 part=touches
decision/harness-json-via-python-json and decision/harness-log-not-telemetry-sink: the agent-run harness, which they rule over, changed (a new selection and a package variable); no event encoding or sink changed.
@task_item n=9 part=touches
Any row stating that agent-run drives cargo test -p blitz-tests alone, or that the session binary is not reachable through the agent-run contract, is now stale: run session reaches seven_guis and its escher-session flows.
@task_fact n=1 slug=6-log-coverage
fact: The agent-run harness events take session as a run selection: run.start, test, run.end and status name it for a run of the seven_guis cli_flow escher-session flows, with the same fields as every other selection and no captured output.
@task_fact n=2 slug=6-log-coverage
fact: An agent-run session run that ran 0 tests ends run.end with outcome failed and exit 1; an empty session run is never a pass.
@task_item n=1 part=untouched
The escher-driver command and tool_call spans, their fields and endings, and the refusal cause names: no driver code changed.
@task_item n=2 part=untouched
escher's telemetry sink and its log line format, scrub set, target allowlist, RUST_LOG handling and resource keys (service.name, service.version): no sink code changed.
@task_item n=3 part=untouched
escher-session's own contracts — its exit codes, stdout, mcp and coverage roles, host wire, stderr and recording files: the binary is run by its existing flows, not changed.
@task_item n=4 part=untouched
The cold-agent pipe, its events, verdict fields, counted verbs and kept checks: not reached.
@task_item n=5 part=untouched
Metric emission, metrics exporters, telemetry backends and the WPT scores: no capability added or removed; the moved test counts sit in no row.
@task_item n=6 part=untouched
CI integration rows (GitHub workflows, artifacts on failure or success, apt-install diagnostics, local state never uploaded): the ci-scripts leg only ran more cases; no workflow or artifact changed.
@task_item n=7 part=untouched
Script error capture, the window error event, WPT reports, snapshot, select and page-event PII rows, and engine tracing feature gating: no engine or page code changed.
