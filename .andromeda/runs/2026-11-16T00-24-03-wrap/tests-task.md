@task_prose part=session
The agent-run contract's run verb gained a fourth selection word, session, beside stand, all and a blitz-tests file name.
run session runs cargo test -p seven_guis --locked --test cli_flow: the seven_guis shell flows that drive the escher-session binary; every other selection still runs cargo test -p blitz-tests --locked as before.
session is a fixed word matched before the file-name arm; the argument count is checked first (an extra argument exits 2), and before boot it exits 3 with no cargo call.
No new verb, event, event field, state file or process: run session emits the existing run.start, test and run.end events with selection session; boot still builds blitz-tests alone; the Windows entry stays a pass-through.
Six contract cases were added under the cargo shim (session runs the flow target, before-boot exit 3, extra-argument exit 2, fixed word before a file of that name, failing flow carries no captured output, empty session run is never a pass).
The L1 boot-path smoke of escher-session, once driven by hand, is now reachable as agent-run run session and nameable as a plan's smoke entry.
@task_file n=1 path=scripts/agent-run.sh
what: the agent-run contract: the five-command harness entry (boot, run, status, logs, cleanup) whose run verb takes a selection and drives cargo test, with its usage line and run selections
@task_file n=2 path=.github/scripts/test_agent_run.py
what: the agent-run contract tests: the Python unittest cases that pin agent-run's verbs, selections, exit codes, events and status under a cargo shim
@task_item n=1 part=touches
contract/agent-run-verbs/run: its run {selection} runs cargo test -p blitz-tests --locked is now stale; the session selection runs cargo test -p seven_guis --locked --test cli_flow, so run reaches blitz-tests or seven_guis.
@task_item n=2 part=touches
The agent-run selection rows (agent-run run selection stand, all, {name}; agent-run selections run stand, run all, run {file}): still true, but a session selection row now belongs beside them.
@task_item n=3 part=touches
contract/agent-run-selection/name and contract/agent-run-selections/file: a file stem named session no longer selects that file; session is a fixed word matched before the file-name arm.
@task_item n=4 part=touches
contract/agent-run/invocation: the selections it names now include session (stand, all, session or a blitz-tests file name).
@task_item n=5 part=touches
decision/agent-run-holds-no-process: its exception gains the session flows' hosts, started and stopped by the cli_flow target's own checks, not by agent-run.
@task_item n=6 part=touches
The session-lifecycle overview: its line that the session is not part of the five-command agent-run contract is stale, since agent-run run session now drives the session binary's flows.
@task_item n=7 part=touches
contract/agent-run-exit-codes/2 and /3: run session with an extra argument exits 2 before any call past boot's; run session before boot exits 3 with no cargo call.
@task_item n=8 part=touches
decision/empty-run-is-never-a-pass and contract/agent-run-exit-codes/1: an empty session run (cli_flow running 0 tests, the off-unix shape) exits 1 with outcome failed; a failing session flow exits 1.
@task_item n=9 part=touches
decision/test-output-never-reaches-events and decision/agent-run-events-content-free: now pinned for the session selection too; a failing session flow's captured output reaches neither stdout nor the events file.
@task_item n=10 part=touches
contract/agent-run-events/run-start, /test, /run-end and /status: unchanged in shape, they now carry the selection value session, and status reads the session run.
@task_item n=11 part=touches
contract/agent-run-verbs/boot: unchanged, still builds blitz-tests alone; run session's own cargo call builds seven_guis.
@task_item n=12 part=touches
contract/agent-run-state/files: unchanged; the session flows' state directories are the cli_flow target's own under the cargo target tmp dir, not agent-run state.
@task_item n=13 part=touches
contract/agent-run/windows-entry: the Windows entry is an unchanged pass-through and carries the session word through.
@task_item n=14 part=touches
decision/one-agent-run-driver-at-a-time: now governs a session run as well as blitz-tests runs.
@task_item n=15 part=touches
decision/shell-flows-under-cargo-test and decision/shell-flows-are-cargo-test-assets: the seven_guis shell flows are now also reached through the agent-run contract's session selection.
@task_item n=16 part=touches
decision/seven-guis-cli-targets-unix-only and decision/cli-and-flow-targets-unix-gated: off unix the cli_flow target runs no test, which run session reads as a failed empty run.
@task_item n=17 part=touches
decision/ui-driven-through-session-binary: the binary's boot-path smoke is now an agent-run selection a plan names as its smoke entry.
@task_item n=18 part=touches
decision/cargo-legs-locked: run session's cargo call is locked like the other selections.
@task_item n=19 part=touches
decision/python-script-tests-under-unittest, decision/ci-scripts-unit-tested and decision/mutation-controlled-tests: six new agent-run contract cases, recorded red against the old script before green; no row holds the case count.
@task_fact n=1 slug=3-test-harness-contract
fact: agent-run run session runs cargo test -p seven_guis --locked --test cli_flow; session is a closed word matched before the file-name arm, the argument count checked first (exit 2) and the boot checked next (exit 3).
@task_fact n=2 slug=3-test-harness-contract
fact: the agent-run run verb reaches two packages: blitz-tests for stand, all and a file name, seven_guis for the session selection alone.
@task_fact n=3 slug=session-lifecycle
fact: the escher-session boot-path smoke (L1) is reachable as agent-run run session and is nameable as a plan's role=smoke entry, no longer driven by hand.
@task_item n=1 part=untouched
The escher-session binary itself: its command line, exit status, error object, stderr, recording and state directory rows; the binary was not changed, only reached.
@task_item n=2 part=untouched
Driver session, session host, session wire and session errors contracts; no protocol, bound or verb moved.
@task_item n=3 part=untouched
Harness construction, core, input, inspection, settle, screenshot and contrast contracts, and the seven_guis headless stand rows.
@task_item n=4 part=untouched
The cold-agent pipe rows: its verbs, events, exit codes, verdict and invocation; that pipe is a separate entry and was not changed.
@task_item n=5 part=untouched
MCP server and MCP role rows, refusal causes, lint defects and snapshot rows.
@task_item n=6 part=untouched
CI integration rows: legs, triggers, caches, artifacts, package installs and the leg script exit codes; no leg, job or trigger changed and no row holds the ci-scripts test count.
@task_item n=7 part=untouched
Coverage floors, mutation, complexity and stand-repeat quality gates; no threshold or budget moved.
@task_item n=8 part=untouched
WPT runner, status set and conformance rows, and test data, fixtures and mocking discipline rows.
