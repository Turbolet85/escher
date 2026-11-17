@task_prose part=chunk
Splits the driver's answer writer by answer family (caused, layout, snapshot, diff) into private child modules, the writer file staying the parent and re-exporting what moves, so every caller's path keeps resolving.
A corrective restructure: no behaviour change, no new requirement line, no new answer form; every carrier's answers (socket call, command line, MCP, a kept check) stay byte for byte.
The writer's inline unit tests move with their family or into a sibling tests file, every test kept and the count before and after recorded.
Re-runs the witnesses that hold the written forms: the writer's and describe's unit tests, the committed Counter checks replayed, the MCP and command-line checks, and the cross-OS answer digests.
Upstream's document file in the DOM crate is left whole and untouched.
@task_file n=1 path=packages/escher-driver/src/json.rs mark=edits
what: the driver's answer writer, the hand-written output-only JSON writer of every answer the carriers write — edits: split into family child modules (caused forms, layout answer, snapshot node and diff forms, plus the shared primitives and the unnamed forms: UI graph, actions, lint, shot, coverage, outcome, refusal, session error, session lines), its inline tests module moved
@task_file n=2 path=CLAUDE.md mark=edits
what: the project's pointer rows — edits: the Driver command line row that names the answer writer's file
@task_file n=3 path=.claude/docs/services/escher-driver.md mark=edits
what: the escher-driver service doc — edits: its pointers to the answer writer's file
@task_file n=4 path=tests/blitz-tests/checks/counter-cli.json mark=re-reads
what: the committed Counter command-line check replayed by stand_replay, each answer kept whole and as a 64-bit digest
@task_file n=5 path=tests/blitz-tests/checks/counter-mcp.json mark=re-reads
what: the committed Counter MCP check replayed by stand_replay, each answer kept whole and as a 64-bit digest
@task_file n=6 path=examples/seven_guis/tests/cli_commands.rs mark=re-reads
what: the command-line check of the escher-session commands and their answers
@task_file n=7 path=examples/seven_guis/tests/cli_flow.rs mark=re-reads
what: the command-line flow check of escher-session's answers across a session
@task_item n=1 part=needs
The restructure's invariant: the answer writer's forms — the shared primitives, the snapshot node and diff forms, the caused forms, the layout answer, the UI graph, actions, lint findings, the shot with and without image, coverage, outcome, refusal and session error objects, and the started, status and stopped session lines — move into family modules and each writes exactly the bytes it wrote before, on every carrier (socket call, command line, MCP stdout, help and tool results, stderr session errors); held by the writer's own unit tests, describe's tests, the Counter checks replayed by stand_replay, stand_mcp_tools, the seven_guis command-line checks and stand_cross_os_readings' answer digests.
@task_item n=2 part=needs
The escher-session commands' answers (snapshot, layout, graph, actions, coverage, screenshot, help) and the mcp role's tool results are the surface whose written bytes must not move: no key name, result field, cause, order or session error added, removed or reordered, and the help self-description unchanged.
@task_item n=3 part=needs
Driver command line output is read by agents: the split keeps the writer's output an agent reads unchanged, with no colour and no terminal-dependent form introduced.
@task_item n=4 part=needs
The writer stays output only: it reads no JSON, and no serde writer replaces the hand-written one in any new module.
@task_item n=5 part=needs
Every new family module is private to the driver crate (crate-visible at most); no public API moves and the crate's public re-export list stays unchanged, the doc gate green.
@task_item n=6 part=needs
Module shape follows the crate's own precedent of a parent file with private child modules and a sibling tests file reached by a path attribute; callers' paths into the writer keep resolving by re-export where an item moves.
@task_item n=7 part=needs
Moved inline tests: every test kept, none weakened or dropped, reaching the moved units through the parent; the test count before and after recorded.
@task_item n=8 part=needs
A move with no edit to any function body is the mechanism that keeps the bytes; P3 measures which witness would read a changed byte.
@task_item n=9 part=needs
No new crate, dependency, port, env var, listener, log field or span; the lock file unchanged.
@task_item n=10 part=needs
Gates the restructure must pass: the fast and doc CI legs, the stand checks in both layout modes, and fork CI green on the pushed commit.
@task_item n=11 part=needs
The pointer rows that name the writer's file (the project's Driver command line row, the escher-driver service doc, the architecture doc's Standard Contracts) follow the files as the wrap's leaf work.
@task_item n=1 part=not_needed
Color palette, typography, spacing, depth strategy, border radius and iconography: engine painting, fonts, cursors and colour tokens are not reached by an output writer's restructure.
@task_item n=2 part=not_needed
Motion and the virtual clock: animation, scroll timing, advance and settle behaviour are engine and session behaviour the writer only reports, unchanged.
@task_item n=3 part=not_needed
The select list's events, overlay painting, gestures and accessibility rows: engine-side select behaviour, not answer serialisation.
@task_item n=4 part=not_needed
The WPT runner's output and result colours: a separate binary's terminal output.
@task_item n=5 part=not_needed
Shell provider, window decorations, compositing alpha mode, the JS window object, MountedData during a render and native font-context configuration: windowing and runtime contracts outside the driver's writer.
@task_item n=6 part=not_needed
The web-spa surface: WASM pages rendering into a canvas are not reached.
