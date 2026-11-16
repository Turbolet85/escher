@task_prose part=session
The session gave the agent-run.sh test contract's run verb a fourth selection word, session, beside stand, all and a blitz-tests file name.
run session runs the escher-session binary's flow target as cargo test -p seven_guis --locked --test cli_flow; every other selection still runs cargo test -p blitz-tests, and boot still builds blitz-tests alone.
session is a closed word matched before the file-stem arm; an extra argument exits 2, a run before boot exits 3 with no cargo call, an empty session run exits 1 as failed.
No new verb, event, event field, state file or process: run session emits the existing run.start, test and run.end events, and no captured output of a failing flow reaches stdout or the events.
The usage line now reads run selections: stand · all · session · <blitz-tests file name>; the Windows pass-through script is unchanged.
The contract's tests gained six cases (14 to 20), and the boot-path smoke of escher-session is now reachable as run session and nameable as a plan's smoke entry.
@task_file n=1 path=scripts/agent-run.sh
what: the agent-run.sh test contract — the harness script's boot, run and status verbs, its usage line, its run selections and the cargo test call run makes
@task_file n=2 path=.github/scripts/test_agent_run.py
what: the agent-run.sh test contract's own contract tests — its cases under a cargo shim, the guard of the contract's selections, exit codes and stdout
@task_item n=1 part=touches
contract/agent-run-cli/usage — stale: the usage line now names the selections stand, all, session or a blitz-tests file name.
@task_item n=2 part=touches
contract/agent-run-cli/exit-2 — run session with an extra argument exits 2, the argument count checked before the selection; session decided before any file of that name.
@task_item n=3 part=touches
contract/agent-run-cli/stdout — run session prints the existing run.start, test and run.end events with selection session; a failing flow's captured output never reaches stdout.
@task_item n=4 part=touches
contract/agent-run-cli/run-log — the cargo output the run log holds now comes from -p seven_guis --test cli_flow under session, from blitz-tests under every other selection.
@task_item n=5 part=touches
contract/agent-run-cli/windows-pass-through — the pass-through carries the new session selection unchanged; its row's wording may need session where it lists selections.
@task_item n=6 part=touches
decision/no-caller-input-echoed — holds over the new selection: no captured output of a session flow is written back into stdout or the events, pinned by a new contract case.
@task_item n=7 part=touches
Any row stating that agent-run run drives cargo test -p blitz-tests alone, or that the escher-session binary is outside the agent-run contract, is now stale: run session reaches the seven_guis cli_flow target.
@task_item n=8 part=touches
Rows whose guard cell names the agent-run contract tests — the guard keeps its name; six cases joined it (session runs the flow target, before boot exits 3, extra argument exits 2, fixed word before a file, failing flow carries no output, empty run never a pass).
@task_fact n=1 slug=surface-cli
fact: agent-run.sh run session runs the escher-session binary's flows as cargo test -p seven_guis --locked --test cli_flow, the one run selection outside blitz-tests; session is a closed word matched before a blitz-tests file of that name, before boot it exits 3 with no cargo call, and an empty session run exits 1 as failed.
@task_item n=1 part=untouched
The Surface: desktop-native decisions and contracts — stand element ids, ShellProvider window chrome, stand boot, viewport, accent, lints and scrolling — no stand or window code changed.
@task_item n=2 part=untouched
The escher-session binary's own contracts — command form, exit codes, actions, snapshot, screenshot, graph, lint, coverage, help, MCP server, recording, refusals, served tasks, host log — the binary and its flows are unchanged; only how a run reaches them moved.
@task_item n=3 part=untouched
The cold-agent.sh run pipe contracts and the cold-agent decisions — the cold-agent script and its pipe are unchanged.
@task_item n=4 part=untouched
The multi_doc and paint_bench command lines — neither example's arguments, environment or output changed.
@task_item n=5 part=untouched
The Surface: mobile-native and Surface: web-spa areas, the Decisions Log, and the absence of arbitrary index.html templates — no layout, page or template code moved.
