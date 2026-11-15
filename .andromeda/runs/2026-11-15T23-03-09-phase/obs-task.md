@task_prose part=chunk
Adds a boot-path smoke of the session binary escher-session that the agent-run harness contract reaches: one documented command that builds the binary, boots a session on a named stand task, runs at least one call against it, stops it, and reports a JSON-line verdict under the contract's exit grammar 0, 1, 2, 3.
Its shape, a harness verb or a committed flow script, is this phase's choice; a plan can then name it as its role=smoke gate entry instead of driving the binary by hand.
Adds its contract test to the CI scripts' unittest leg, pinned the way the five agent-run verbs are pinned today, through a cargo shim, with nothing live in CI.
Names the new smoke in the harness documents that describe the agent-run contract; the test-plan master itself is amended only at the wrap.
Re-verifies at HEAD the measured L1 mechanism: the harness drives only blitz-tests targets and no verb boots the session binary.
@task_file n=1 path=scripts/agent-run.sh mark=edits
what: the agent-run harness, its five verbs boot, run, status, cleanup and logs each driving cargo test of blitz-tests, and the events it writes — edits: a new smoke verb or entry that builds and boots escher-session
@task_file n=2 path=.github/scripts/test_agent_run.py mark=edits
what: the CI scripts' unittest leg's contract test of the agent-run harness, pinning its verbs through a cargo shim — edits: the new smoke's contract tests
@task_file n=3 path=examples/seven_guis/tests/flows/counter.sh
what: a committed POSIX flow that boots escher-session from the binary and a fresh state directory (start, snapshot, click, refused click, stop, session error), run as an asset of the seven_guis cli_flow target; a candidate the smoke may reuse
@task_item n=1 part=needs
The new smoke behaviour: one command builds escher-session, boots a session on a named stand task, makes at least one call, stops the session and prints one JSON-line verdict, exiting 0, 1, 2 or 3 under the agent-run contract's exit grammar.
@task_item n=2 part=needs
The agent-run harness events the smoke writes or extends: events.jsonl and its run.start, boot, test, status, cleanup and run.end events, and the test event's file field; a smoke event takes the same shape.
@task_item n=3 part=needs
Agent-run events hold no captured output and the harness never prints the raw output of what it drives; the smoke's verdict and events carry fixed fields only, never the session's answers.
@task_item n=4 part=needs
The agent-run harness log carries no user content and its raw log is left unscrubbed; the smoke's log of the session it boots follows the same rule.
@task_item n=5 part=needs
Harness JSON lines are encoded by python's json module, and the harness log is not a telemetry sink; the smoke's verdict line is written the same way.
@task_item n=6 part=needs
Agent-run cleanup touches only its own state: the smoke stops its session and removes only the state directory it created.
@task_item n=7 part=needs
The session's state directory: caller-named, relative under target/tmp so the socket path stays inside the 94-byte bound, owner-only and empty of content, with the one registered Unix socket; the smoke adds no listener, port, IPC endpoint, env read or credential path.
@task_item n=8 part=needs
The session id record and the session's state are never printed or copied by the harness; answers the smoke reads back echo nothing the caller typed.
@task_item n=9 part=needs
The escher-session stdout command answer and exit codes 0 to 3 are read by the smoke as they stand to decide its verdict; the smoke adds no verb, carrier, refusal cause, session error, log field or span.
@task_item n=10 part=needs
escher-session installs its telemetry sink first in main and writes its startup line to stderr under RUST_LOG; a check of the smoke that expects empty stderr unsets RUST_LOG, and the headless stand runs with no sink.
@task_item n=11 part=needs
The contract test runs in the CI scripts' unittest leg through a cargo shim or equivalent: silent checks with shared modules, nothing live in CI that the leg does not already run, and log check failures naming a kind and a count.
@task_item n=12 part=needs
No CI leg of its own unless an existing leg must run the smoke; the stand-repeat and test legs already run cli_flow over the committed flows, and local session state is never uploaded.
@task_item n=13 part=needs
The harness documents describing the agent-run contract name the new smoke and its role=smoke entry form; the test-plan's agent-run contract is amended only at the wrap.
@task_standing n=1
convention: each new harness event or verdict line is a JSON line encoded by python's json module, with fixed fields and no captured output or user content
home: 3-observability-harness-contract · 6-log-coverage
@task_standing n=2
convention: each new check or test in the CI scripts' unittest leg is silent on success, uses the shared modules, and names a kind and a count when it fails
home: 3-observability-harness-contract
@task_standing n=3
convention: each new harness log carries no user content and holds no page content of the session it drives
home: 8-pii-scrubbing-compliance
@task_standing n=4
convention: each new harness entry cleans up only its own state, keeps its session state directory private and local, and uploads none of it
home: ci-integration
@task_item n=1 part=not_needed
Telemetry strategy rows on the tracing crate, engine tracing behind a cargo feature and escher crates taking tracing ungated: the chunk adds no tracing and changes no crate's features.
@task_item n=2 part=not_needed
The driver's command span and tool_call span rows, their fields, endings and lines: the smoke adds no span and changes no driver code.
@task_item n=3 part=not_needed
The cold-agent pipe's rows: its events, verdict fields, counted verbs, reasons, transcript, server log and kept checks; the chunk leaves that pipe untouched.
@task_item n=4 part=not_needed
Refusal cause names and the driver's verbs and answers for graph, actions, lint and screenshot: the smoke drives the session as it stands and adds none.
@task_item n=5 part=not_needed
Metric coverage rows, metric emission, exporters, telemetry backends and OTel export: the chunk emits no metric and adds no backend.
@task_item n=6 part=not_needed
Error capture rows on script errors, the window error event, window.onerror, WPT reports and the WASM panic hook: engine and page error surfaces the chunk does not touch.
@task_item n=7 part=not_needed
Page-content PII rows on select lists, snapshot nodes, multiple select values, page events, the state registry and the mutation buffer: engine and driver content the smoke neither reads into its output nor changes.
