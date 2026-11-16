@task_prose part=session
The agent-run contract's run verb gained a fourth closed selection word, session, matched before the file-stem arm like stand and all.
run session makes the run's one cargo call cargo test -p seven_guis --locked --test cli_flow, so it runs the seven_guis CLI flow target that drives the escher-session binary; every other selection still calls blitz-tests.
The package run reaches is now a variable: blitz-tests for stand, all and a file stem, seven_guis under session alone; the usage line reads stand · all · session · a blitz-tests file name.
No new verb, event, event field, exit code, state file, env read, listener or process of the script's own; run session emits the existing run.start, test and run.end events, and boot still builds blitz-tests alone.
Six contract cases were added to the agent-run contract tests (14 to 20), lifting the ci-scripts leg's unittest count from 112 to 118; the boot-path smoke of escher-session is now reachable as run session and nameable as a plan's smoke entry.
@task_file n=1 path=scripts/agent-run.sh
what: the agent-run contract script, the five-command agent harness (boot, run, status, cleanup) over cargo test, its JSON-line events, exit codes and local state directory; edited
@task_file n=2 path=.github/scripts/test_agent_run.py
what: the agent-run contract tests, stdlib unittest cases run by the ci-scripts leg that pin the agent-run script's events, exit codes and selections under a cargo shim; edited
@task_item n=1 part=touches
The agent-run contract's run verb takes a new closed selection word session; any row naming agent-run's run selections or its usage gains session beside stand, all and a blitz-tests file name.
@task_item n=2 part=touches
The packages agent-run's run reaches moved from blitz-tests alone to blitz-tests or seven_guis (seven_guis under session alone, its cli_flow target); a row stating agent-run drives cargo test -p blitz-tests alone is now stale.
@task_item n=3 part=touches
agent-run.sh JSON-line events run.start, test and run.end now also carry selection session; no event or field was added, so the event rows stay true but may name the selection values.
@task_item n=4 part=touches
agent-run.sh exit codes: run session before boot exits 3, run session with an extra argument exits 2, a failing or empty session run exits 1 with outcome failed; codes unchanged, their causes now include the session selection.
@task_item n=5 part=touches
agent-run.sh state directory and the agent-run and cold-agent state areas kept local: unchanged; the session flows' own session state dirs stay the cli_flow target's under CARGO_TARGET_TMPDIR, none in agent-run's state.
@task_item n=6 part=touches
Raw run output never printed: a failing session flow carries no captured output into stdout or the events file, now pinned by a contract case.
@task_item n=7 part=touches
Occupied resources: run session builds and runs the seven_guis cli_flow target, which starts escher-session hosts with their Unix socket and state dir; agent-run itself adds no listener or process of its own.
@task_item n=8 part=touches
CLI flow tests under plain sh and the seven_guis cli_flow target: now invoked by the agent-run contract, not only by the test leg.
@task_item n=9 part=touches
Windows script entries are pass-throughs: agent-run.ps1 unchanged and still a pass-through, so the session selection reaches Windows callers through it.
@task_item n=10 part=touches
The ci-scripts leg's unittest count moved 112 to 118 and the agent-run contract tests 14 to 20; a touch only where a row's cell holds either count.
@task_item n=11 part=touches
CI Python tests use unittest and CI helper scripts on the stdlib: the six new agent-run contract cases follow both, adding no dependency.
@task_item n=12 part=touches
The escher-session binary's boot-path smoke (the session lifecycle's L1) moved from a hand drive of the debug binary to the agent-run run session call; a row saying the session is outside the agent-run contract is now stale.
@task_fact n=1 slug=standard-contracts
fact: agent-run's run verb reaches two packages: blitz-tests for stand, all and a blitz-tests file name, and seven_guis's cli_flow target alone for the closed word session, matched before the file arm.
@task_item n=1 part=untouched
Engine rows (DOM, style, layout, paint, hit testing, select list, accessibility tree, scrolling, scripting): no engine code changed.
@task_item n=2 part=untouched
The escher-session command line, its roles, exit codes, session wire, session errors and MCP server: the binary is unchanged, only reached by a new harness call.
@task_item n=3 part=untouched
Driver verbs, snapshot, diff, lint, actions, graph, kept checks and stable element ids: no driver surface moved.
@task_item n=4 part=untouched
Telemetry, cold-agent harness, its verdict and events: untouched.
@task_item n=5 part=untouched
CI workflow jobs, the ci-leg.sh leg list, coverage floors, cargo-deny, fmt and clippy gates: no leg added or reshaped.
@task_item n=6 part=untouched
Crate and binary names, dependencies, toolchain, stack choices and upstream sync rows: no crate, dependency or name taken or freed.
@task_item n=7 part=untouched
Browser, WPT runner, rdme and example CLIs: not reached.
