@task_prose part=session
The agent-run contract's run verb took a fourth fixed selection word, session, beside stand, all and a blitz-tests file name.
run session runs the seven_guis cli_flow target, the escher-session binary's flows, so run now reaches seven_guis as well as blitz-tests.
The usage line now reads the selections stand, all, session or a blitz-tests file name; the header comment names the session binary's flows.
No new verb, event, event field, state file, env read or process: run session emits the existing run.start, test and run.end events.
The boot-path smoke of escher-session, once driven by hand, is now reachable as run session and nameable as a plan's smoke entry.
Six contract cases were added for the session selection (14 to 20 cases); nothing was retired.
@task_file n=1 path=scripts/agent-run.sh
what: the agent-run contract, the command line script agents drive to boot, run and read the test harness — its run selections, its usage line and the package its one cargo call tests
@task_file n=2 path=.github/scripts/test_agent_run.py
what: the agent-run contract's own tests, the check over its verbs, selections, exit statuses and events
@task_item n=1 part=touches
The Surface: cli area's Navigation Pattern overview (no slice recorded argument parsing or command structure for the CLI binaries): the agent-run command line gained a fixed selection word, session, matched before the file-name arm, its usage line naming it — check whether the overview's claim stands or is narrowed.
@task_item n=2 part=touches
decision/escher-session-agent-read-surface: escher-session's flows are now driven by an agent through the agent-run contract's run session — check whether the decision's reading surface or its except holds this path.
@task_item n=3 part=touches
decision/driver-output-for-agents: run session's output is the agent-run contract's existing JSON events read by agents, with no captured output of a failing flow carried into stdout or the events — check whether a rule over agent-read command line output reaches it.
@task_item n=4 part=touches
The escher-session exit codes and exit status rows (0, 1, 2, 3): run session reads the cli_flow test target's outcome, not the binary's exit codes directly; an empty session run exits 1 as failed and run session before boot exits 3, an extra argument exits 2 — check whether any row ties those statuses to the agent-run contract.
@task_item n=5 part=touches
The escher-session tasks rows (counter, crud, timer, flight-booker, cells, load): the cli_flow flows they back are now one selection of the agent-run contract — a row naming how those flows are run or reached may be stale.
@task_item n=6 part=touches
decision/escher-session-no-colour: its no-terminal-test stance — run session drives the flows under plain sh through cargo test, with no terminal; check that the decision still reads true.
@task_item n=1 part=untouched
Colour, typography, spacing, depth, border radius, iconography and brand rows: no paint, font, colour token or cursor changed.
@task_item n=2 part=untouched
Both motion areas and the driver verbs and virtual clock rows: no clock, scroll or animation behaviour changed.
@task_item n=3 part=untouched
The select list, snapshot, screenshot and accessibility rows of the desktop-native surface: no engine or select behaviour changed.
@task_item n=4 part=untouched
The escher-session commands, mcp role, stderr, session error, actions line, coverage line and UI graph line rows: the binary itself and its outputs did not change.
@task_item n=5 part=untouched
The WPT runner output and result colour rows, the shell provider, JS window, MountedData and dioxus-native configuration rows, and the web-spa surface: none was reached.
