@task_prose part=chunk
Adds a seventh served stand app, the served error page, in the seven_guis example, served by escher-session beside counter, flight-booker, timer, crud, cells and load.
Its controls, each reached by an ordinary driver step, make a handler panic, make a handler return an error the root error boundary takes, and make a handler log, so an acting call's answer on that app carries `caused`.
Adds its stand checks (headless boot in both layout modes, each control's `caused` items tied to its stable id, owner and event; listed and reached through the host and the mcp role), a goal in the cold-agent stand prompt and a `caused` count in the cold-agent verdict.
Moves the pipe's contract tests and the capability statement pins where the served-app count or the verdict's fields are held; the six served apps' answers stay byte for byte as they are.
@task_file n=1 path=examples/seven_guis/src/tasks/ mark=edits
what: the stand's task screens — edits: the new error page app, its controls and their stable element ids
@task_file n=2 path=examples/seven_guis/src/stand.rs mark=edits
what: the stand's headless boot shared by the session roles — edits: the new app's headless boot
@task_file n=3 path=examples/seven_guis/src/session_host.rs mark=edits
what: the escher-session host and its served tasks — edits: the served-task table, grown by one entry
@task_file n=4 path=examples/seven_guis/src/app.rs mark=edits
what: the windowed stand and its task cards — edits: the new app's task card, if the app joins the windowed stand
@task_file n=5 path=tests/blitz-tests/tests/ mark=edits
what: the stand checks — edits: the new app's stand checks
@task_file n=6 path=.github/scripts/test_blitz_tests_targets.py mark=edits
what: the blitz-tests target pin — edits: the new stand check target
@task_file n=7 path=.github/scripts/ci-leg.sh mark=edits
what: the CI leg runner — edits: the stand-repeat leg's named targets
@task_file n=8 path=examples/seven_guis/tests/ mark=edits
what: the escher-session binary's served-task checks — edits: the new served task listed and reached through the host and the mcp role
@task_file n=9 path=scripts/cold-agent.sh mark=edits
what: the cold-agent run pipe — edits: the stand prompt's goal, the verdict's `caused` count
@task_file n=10 path=scripts/cold-agent.ps1 mark=edits
what: the cold-agent run pipe's Windows pass-through
@task_file n=11 path=.github/scripts/test_cold_agent.py mark=edits
what: the cold-agent pipe's contract tests — edits: the prompt goal, the verdict count, the served-app count
@task_file n=12 path=docs/capabilities.md mark=edits
what: the capability statement — edits: the served-app counts
@task_file n=13 path=README.md mark=edits
what: the project README's status — edits: the served-app counts
@task_file n=14 path=.github/scripts/test_capability_statement.py mark=edits
what: the capability statement pins — edits: the served-app counts
@task_item n=1 part=needs
The escher-session served tasks: a seventh entry joins the six (counter, flight-booker, timer, crud, cells, load), listed and reached through the host and the mcp role as the others are.
@task_item n=2 part=needs
The new app's controls carry stable element ids as attributes alone, every stand control keyed, so each control's `caused` item is tied to its stable id, owner and event.
@task_item n=3 part=needs
The new app's names, roles and states are given by attributes alone, as every stand screen's are.
@task_item n=4 part=needs
The new app's boot reads 0 lint findings, as the lean stand screens do, and its snapshot stays under the snapshot text budget.
@task_item n=5 part=needs
The new app boots headlessly through the stand boot the session roles share, at the pinned headless viewport, in both layout modes; a held instance reads as a fresh boot.
@task_item n=6 part=needs
The escher-session actions answer on the new app: each acting call on a control carries the `caused` items that control is built to cause (a panic, an error the root error boundary takes, a log).
@task_item n=7 part=needs
No new driver verb, argument, carrier, cause, session error, exit code, node key, log field or span; the new app is reached by the driver's existing acting verbs and its existing MCP tools.
@task_item n=8 part=needs
The authored error text (panic messages, error text, log messages and fields) carries nothing a scrub would redact: no hidden-set field name, no host path, no secret, no user text; and no caller input is written back in it.
@task_item n=9 part=needs
The cold-agent stand prompt gains one goal, phrased once, asking the agent to meet the error page and make sense of what its controls cause; the prompt names no element id.
@task_item n=10 part=needs
The cold-agent verdict gains a count of the calls whose answer carried `caused`, read from the live server child's kept checks or transcript as bools and counts only, never the text, and whether the run is required to meet it beside the pipe's other required counts.
@task_item n=11 part=needs
The cold-agent pipe still builds nothing and drives through the driver's MCP server, local over stdio; its usage, exits, stdout, record dir and Windows pass-through stay as they are.
@task_item n=12 part=needs
The six served apps' answers stay byte for byte as they are, held by their kept checks and the stand cross-OS readings.
@task_item n=13 part=needs
The escher-session recording: the new app's acting calls land in the kept check files as the other served tasks' do, and those carry the `caused` items the verdict count reads.
@task_item n=14 part=needs
The escher-session MCP server's result bound and tools list hold for the new app's answers carrying `caused`.
@task_item n=15 part=needs
The stand accent colour and the fixed document skeleton with its main mount hold for the new app's screen.
@task_item n=16 part=needs
The new app has no network provider: the load page alone has one.
@task_item n=17 part=needs
The new app's stable element ids join the stand stable element ids register, one per control and its task card where it joins the windowed stand.
@task_standing n=1
convention: every stand control is keyed and carries a stable element id as an attribute alone, registered among the stand's stable element ids
home: surface-desktop-native
@task_standing n=2
convention: every served task is an entry of the escher-session served tasks, listed and reached through the host and the mcp role
home: surface-cli
@task_item n=1 part=not_needed
The ShellProvider window chrome controls (drag, maximise, minimise, close, decorations): the new app adds no window chrome behaviour.
@task_item n=2 part=not_needed
The agent-run test contract: the chunk changes the cold-agent pipe alone, not the agent-run script.
@task_item n=3 part=not_needed
The multi_doc and paint_bench command lines: benchmarks the chunk does not reach.
@task_item n=4 part=not_needed
The escher-session coverage role, graph, layout, lint, help and screenshot answers: their forms do not change; the new app is reached through them as any served task is.
@task_item n=5 part=not_needed
The escher-session refusals (fonts-unavailable, off-screen, unknown-verb) and exit codes: no new refusal, session error or exit.
@task_item n=6 part=not_needed
The CRUD list keyboard region, the flight type select's boot reading, open list rows' bounds, into-view scroll and the visible region through clipping boxes: behaviours of other stand screens and of the driver the new app does not change.
@task_item n=7 part=not_needed
Arbitrary index.html templates for a document: the new app uses the fixed skeleton.
@task_item n=8 part=not_needed
The mobile-native and web-spa surfaces: not measured and not reached.
