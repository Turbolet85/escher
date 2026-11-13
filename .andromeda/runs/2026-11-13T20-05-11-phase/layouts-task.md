@task_prose part=chunk
A behaviour-preserving restructure of three functions toward the cognitive-complexity ceiling of 15: blitz-dom's click handler handle_click, escher-driver's executor execute, and dioxus-native-dom's UI graph builder graph.
Extracted helpers stay in the file that holds each function today: no file split, no new module, no other function restructured.
Each function's reading drops; one left above 15 names why, and no new helper reads over 15.
It re-measures the three functions and every extracted helper before and after with rust-code-analysis-cli on the dev host, and re-runs the existing checks that hold each moved arm, adding a check first for any arm none reaches.
@task_file n=1 path=packages/blitz-dom/src/events/pointer.rs mark=edits
what: blitz-dom's pointer event handling, home of the click default actions (checkbox and radio input and change, button focus and submit, the select arms, links, summary, labels) — edits: the function handle_click and the helpers extracted from it
@task_file n=2 path=packages/escher-driver/src/execute.rs mark=edits
what: escher-driver's executor, the per-verb dispatch, the target check and the refusal order — edits: the function execute and the helpers extracted from it
@task_file n=3 path=packages/dioxus-native-dom/src/graph.rs mark=edits
what: dioxus-native-dom's UI graph of a document, DioxusDocument::graph — edits: the function graph and the helpers extracted from it
@task_item n=1 part=needs
The click default actions of the stand controls stay the same: checkbox and radio fire input then change, buttons take focus and submit, select controls open their list and pick an option, links, summary and labels act as before.
@task_item n=2 part=needs
The select arms that the restructure moves: an open select list's rows keep their own bounds, and the flight type select still reads its two options at boot.
@task_item n=3 part=needs
Keyboard and focus behaviour around click (the CRUD list as one keyboard region, focus moved by a button click) stays the same, held by the existing keyboard checks.
@task_item n=4 part=needs
The executor's per-verb dispatch and refusal order stay the same, so the escher-session actions answer and its refusals (off-screen, unknown-verb, fonts-unavailable) come back unchanged, in the same order.
@task_item n=5 part=needs
The escher-session graph answer, built from the UI graph, carries the same content, key names and fields after the restructure.
@task_item n=6 part=needs
The escher-session snapshot answer and its parts, and the coverage role's answer that reads handler coverage from the graph, stay the same.
@task_item n=7 part=needs
The escher-session exit codes and session errors stay the same: no new exit, cause, session error, verb, argument, key name, field, log field or span is added.
@task_item n=8 part=needs
Stand names, roles and states read by attributes alone and stand ids read as attributes alone still read the same in the graph and snapshot the restructured code builds.
@task_item n=9 part=needs
Into-view scroll that moves every holding scroller and the visible region read through clipping boxes, which the executor's target check leans on before an act, keep their results.
@task_item n=10 part=needs
The stand boot the checks use (pinned headless viewport, a held instance reading as a fresh boot, session roles sharing one boot, lints clean at boot) is how the re-run checks reach each moved arm.
@task_item n=11 part=needs
Every stand control keyed by its stable element id is how a check added for an arm no existing check reaches drives that arm before the move.
@task_standing n=1
convention: a check the plan adds for an arm no existing check reaches drives the stand's controls by their stable element ids, through the shared stand boot at the pinned viewport, with names, roles and states read by attributes alone
home: surface-desktop-native
@task_item n=1 part=not_needed
The shell provider's window chrome controls (drag, maximize, minimize, close, decorations): the chunk changes no window chrome.
@task_item n=2 part=not_needed
The agent-run and cold-agent test pipes, their usage, exits, logs, record directories and Windows pass-through: the chunk adds and changes no script.
@task_item n=3 part=not_needed
The multi_doc and paint_bench command lines, their arguments, environment variables and output: the chunk touches no bench.
@task_item n=4 part=not_needed
The MCP server's transport, protocol revisions, tools list, result bound and screenshot result: the restructure adds no tool and changes no transport.
@task_item n=5 part=not_needed
The session recording, its check files and recording refusals, and the cold agent's prompt and pipe rules: none is changed by a restructure that keeps every outcome.
@task_item n=6 part=not_needed
The served tasks list and the stand's stable element id register: no task, control or id is added, renamed or removed.
@task_item n=7 part=not_needed
The stand's accent colour, the fixed document skeleton with its main mount, arbitrary index.html templates and the load page's network provider: no layout, page or template changes.
@task_item n=8 part=not_needed
The mobile-native and web-spa surfaces: the chunk reaches neither.
