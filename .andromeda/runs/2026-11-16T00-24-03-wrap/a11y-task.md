@task_prose part=session
The session added a fourth selection word, session, to the agent-run contract's run verb: matched before the file-name arm like stand and all, it runs the seven_guis cli_flow test target (cargo test -p seven_guis --locked --test cli_flow) instead of a blitz-tests file.
The cargo call now takes its package from a variable, blitz-tests for every other selection, so stand, all and a file name run what they ran before; the usage line and header comment name the new selection.
No new verb, event, event field, state file, env read, listener or process: run session emits the existing run.start, test and run.end events; boot is unchanged and still builds blitz-tests alone.
Six contract cases were added to the agent-run contract tests (14 to 20 cases; the ci-scripts leg's unittest count 112 to 118), covering the session run, its refusal before boot (exit 3), an extra argument (exit 2), the fixed word before a file of that name, a failing flow carrying no captured output, and an empty run never passing.
The boot-path smoke of the session binary is now reachable as run session and nameable as a plan's smoke entry; no accessibility behaviour, tree, snapshot, ARIA reading, keyboard handling or contrast check changed.
@task_file n=1 path=scripts/agent-run.sh
what: edited — the agent-run contract script (the agent-runnable test harness's five-command boot, run, status contract) that the test and CI legs drive; its run selections and the package its cargo call reaches
@task_file n=2 path=.github/scripts/test_agent_run.py
what: edited — the agent-run contract's own unit tests (the ci-scripts leg's suite), over canned cargo outputs under a cargo shim
@task_item n=1 part=touches
Any row naming how accessibility tests are run through the agent-run contract or which selections its run verb takes (stand, all, a blitz-tests file name): session now joins them as a fixed word running the seven_guis cli_flow target.
@task_item n=2 part=touches
Any row stating that agent-run drives cargo test -p blitz-tests alone: run session reaches the seven_guis package instead.
@task_item n=3 part=touches
The accessibility test leg and CI gate decisions (the a11y CI leg, the accessibility test leg in the slow tier, accessibility checks gate CI) only where a cell names the agent-run selections or the packages a run reaches; the legs themselves and their thresholds are unchanged.
@task_item n=4 part=touches
Rows over the seven_guis example (Cells grid, CRUD list) only indirectly: its cli_flow session flows are now run by the agent-run contract's run session; no seven_guis behaviour or keyboard contract changed.
@task_item n=5 part=touches
Stand checks repeated three rounds and the stand-run decisions only where a cell names the run selection stand or all; those selections still call what they called.
@task_item n=6 part=touches
Any row whose guard cell names the agent-run contract tests: six cases were added, none renamed or removed.
@task_item n=1 part=untouched
The accessibility tree, AccessKit adapter, role mapping, accessible name and ARIA state and property contracts: no engine or tree code changed.
@task_item n=2 part=untouched
Keyboard navigation, focus, select list, range input and keyboard activation decisions and contracts: no input handling changed.
@task_item n=3 part=untouched
Visual design verification, contrast lint, contrast thresholds, stand colours and scrollbar decisions: nothing measured or drawn changed.
@task_item n=4 part=untouched
Snapshot, actions answer, driver refusal, screenshot, MCP result and UI graph contracts: the session binary's driver and its readers are unchanged; only how its flows are launched in tests moved.
@task_item n=5 part=untouched
The accessibility feature defaults, platform adapter absence on the stand binaries and tier 0 reach decisions: no feature, dependency or build configuration changed.
@task_item n=6 part=untouched
The obs log format in the a11y assertion harness overview: no event, event field or log line was added.
