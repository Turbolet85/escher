@task_prose part=chunk
Adds a boot-path smoke of the session binary escher-session, reachable through the agent-run contract: one documented command that builds the binary, boots a session on a named stand task, runs at least one call against it, stops it, and reports a JSON-line verdict under the contract's exit grammar 0 · 1 · 2 · 3.
Its shape — a harness verb or a committed flow script — is this phase's to choose; either way a plan can name it as its role = 'smoke' gate entry instead of driving the binary by hand.
Adds its contract test to the CI scripts' unittest leg, pinned the way the five agent-run verbs are pinned today, through a cargo shim or equivalent, with nothing live in CI that the leg does not already run.
The harness documents that describe the agent-run contract name the new smoke; the test-plan master is left to the wrap.
@task_file n=1 path=scripts/agent-run.sh mark=edits
what: the agent-run.sh test contract, the harness that drives blitz-tests targets through its five verbs boot run status cleanup logs — edits: its verb list and usage, if the smoke takes the harness-verb shape
@task_file n=2 path=.github/scripts/test_agent_run.py mark=edits
what: the CI scripts' unittest leg that pins the agent-run.sh test contract's verbs through a cargo shim — edits: a new contract test for the smoke's shape
@task_file n=3 path=examples/seven_guis/tests/flows/counter.sh
what: the committed POSIX flow that boots escher-session on the counter task (start, snapshot, click, refused click, stop, session error), run as an asset of the seven_guis cli_flow target; a candidate for the smoke to reuse
@task_item n=1 part=needs
The new boot-path smoke command: builds escher-session, boots a session on a named stand task, runs at least one call against it, stops it, and prints one JSON-line verdict.
@task_item n=2 part=needs
The agent-run.sh test contract's exit grammar, stdout, usage and run log: the smoke's verdict and exits follow it (0 · 1 · 2 · 3), so a plan can read it like the other verbs.
@task_item n=3 part=needs
The escher-session command line form, driven as it stands: the smoke invokes the binary without adding verbs, carriers or arguments to it.
@task_item n=4 part=needs
The escher-session exit codes 0 · 1 · 2 · 3: the smoke reads the binary's exits to reach its own verdict.
@task_item n=5 part=needs
The escher-session served tasks: the smoke boots its session on one named stand task among them.
@task_item n=6 part=needs
The escher-session snapshot and actions answers: the at-least-one call the smoke runs against the booted session, and what it reads back as proof of boot.
@task_item n=7 part=needs
The session's state directory and socket: the one registered Unix socket in a caller-named, relative target/tmp state directory, owner-only as every session's is, inside the 94-byte socket path bound; no new listener, port, IPC endpoint, env read or credential path.
@task_item n=8 part=needs
The session's stop and teardown: the smoke stops the session it booted and reports a session error as a failing verdict, never leaving a session running.
@task_item n=9 part=needs
Session roles share the stand boot: the smoke's session boots the stand the same way every session role does, so its boot is the boot-path being proved.
@task_item n=10 part=needs
The smoke adds no refusal causes, session errors, log fields or spans to escher-session, and no driver verbs; it reports only through the agent-run contract's own outputs.
@task_item n=11 part=needs
Caller input never written back: the smoke's verdict line names the task and outcome without echoing caller-supplied input.
@task_item n=12 part=needs
The smoke's contract test in the CI scripts' unittest leg: it pins the smoke's usage, verdict line and exits through a cargo shim or equivalent and runs nothing live that the leg does not already run.
@task_item n=13 part=needs
The Windows pass-through agent-run.ps1: if the smoke becomes a harness verb, the pass-through carries it unchanged.
@task_item n=14 part=needs
The committed session flows and session recording as a replayable check: whether the smoke reuses a committed flow such as the counter flow or carries its own; the stand-repeat and test legs already run cli_flow over those flows, and no new CI leg is added unless an existing leg must run the smoke.
@task_item n=15 part=needs
The harness documents describing the agent-run contract name the new smoke and its role = 'smoke' use.
@task_standing n=1
convention: each new or changed harness command owes its contract rows — usage, stdout, exit codes and log file — and a contract test in the CI scripts' unittest leg that pins them without live builds
home: surface-cli
@task_standing n=2
convention: each POSIX harness script under scripts/ carries a Windows pass-through that forwards its arguments and exits unchanged
home: surface-cli
@task_item n=1 part=not_needed
The ShellProvider window chrome control rows: the smoke opens no window and drives no window decorations, maximise, minimise, drag or close.
@task_item n=2 part=not_needed
The browser layout decisions — into-view scroll across scrollers, visible region through clipping boxes, open list rows' bounds, the CRUD list keyboard region, the flight type select's boot reading: the smoke changes no layout or scroll behaviour.
@task_item n=3 part=not_needed
The stand's look and document shape — accent colour, the fixed document skeleton with main mount, and the absence of arbitrary index.html templates: the smoke changes no stand markup or styling.
@task_item n=4 part=not_needed
The cold-agent.sh run pipe contract rows and the cold-agent decisions (prompt names no id, builds nothing, drives through the MCP server): the cold-agent pipe is out of scope.
@task_item n=5 part=not_needed
The multi_doc and paint_bench command line contracts: those binaries are not on the session binary's boot path.
@task_item n=6 part=not_needed
The escher-session coverage role, graph, lint and screenshot answers: the smoke's call is a boot proof and reads none of them.
@task_item n=7 part=not_needed
The mobile-native and web-spa surfaces: they hold no rows and the smoke reaches no mobile or web layout.
@task_item n=8 part=not_needed
The load page's network provider decision: the smoke opens no network path.
