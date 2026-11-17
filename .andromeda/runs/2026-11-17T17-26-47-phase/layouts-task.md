@task_prose part=chunk
Splits the driver's hand-written answer writer into private child modules by answer family: caused, layout, snapshot and diff, the shared primitives kept for all, and each unnamed form (UI graph, actions, lint, shot, coverage, outcome/refusal/session error, session lines) placed where P4 settles.
A corrective restructure: no behaviour change, no new function, no new requirement line; every carrier's answers (socket call, command line, MCP, kept checks) stay byte for byte.
The writer's inline tests move with their family or to a sibling tests file, every test kept, the count before and after recorded.
Re-reads the existing witnesses of the written forms (unit tests, the Counter replay checks, the MCP and command-line checks, the cross-OS answer digests) and re-reads the last chunk commit's CI verdict at P3 and before push.
Blitz's upstream document file is left whole.
@task_file n=1 path=packages/escher-driver/src/json.rs mark=edits
what: the driver's answer writer, output-only JSON for every answer form (snapshot nodes and diffs, caused, layout, graph, actions, lint, shot, coverage, refusals and session errors, session lines) with its inline unit tests — edits: the writer's top-level items moved into family child modules and re-exported, the inline tests module moved
@task_file n=2 path=tests/blitz-tests/checks/counter-cli.json mark=re-reads
what: the committed Counter command-line check replayed by stand_replay, each answer kept whole and as a 64-bit digest
@task_file n=3 path=tests/blitz-tests/checks/counter-mcp.json mark=re-reads
what: the committed Counter MCP check replayed by stand_replay, each answer kept whole and as a 64-bit digest
@task_file n=4 path=examples/seven_guis/tests/cli_commands.rs mark=re-reads
what: the driver command-line check of its commands' answers
@task_file n=5 path=examples/seven_guis/tests/cli_flow.rs mark=re-reads
what: the driver command-line flow check of its answers across a session
@task_file n=6 path=CLAUDE.md mark=edits
what: the project's master pointer table — edits: the «Driver command line» row naming the answer writer's file
@task_file n=7 path=.claude/docs/services/escher-driver.md mark=edits
what: the escher-driver service doc's pointers to the answer writer's file
@task_item n=1 part=needs
The restructure's invariant: the writer's units move unchanged with no edit to any function body — the shared primitives (string, number, Object, list, texts), the snapshot node and diff forms, the caused forms, the layout answer, the UI graph, actions, lint, shot, coverage, outcome/refusal/session-error forms and the started/status/stopped session lines — so every answer of the escher-session snapshot, layout, graph, actions, lint, screenshot, coverage, help, refusals, exit codes, MCP server results and recording check files keeps its bytes, key names, field order, causes and errors; held by the writer's own unit tests, describe's tests, the Counter CLI and MCP replay checks, stand_mcp_tools, the seven_guis cli_commands and cli_flow tests and stand_cross_os_readings' answer digests.
@task_item n=2 part=needs
Module shape follows the driver crate's own precedent (mcp with private children and tests at a #[path] sibling; schema with its spec child and schema tests): the writer's file stays the parent and every caller's path into it (the Object type, list, string, the session lines, shot_without_image from describe, mcp, mcp framing, cli and recording) keeps resolving, by re-export where an item moves.
@task_item n=3 part=needs
Every new module is private to escher-driver, pub(crate) at most; no public API moves, the crate's lib pub use list is unchanged and the doc CI leg stays green.
@task_item n=4 part=needs
The writer stays output-only: it reads no JSON and no serde writer replaces the hand-written one.
@task_item n=5 part=needs
The inline tests (about 1060 lines) move with their family or to a sibling tests file by the #[path] precedent; none weakened or dropped, the test count before and after recorded.
@task_item n=6 part=needs
The driver's self-description stays unchanged: NODE_FIELDS, VERBS and the help answer; no carrier, verb, argument, key name, result field, cause, session error, exit code, log field or span is added, removed or reordered.
@task_item n=7 part=needs
No upstream-owned file is touched: Blitz's document file is not split, nor any other file the code audit listed.
@task_item n=8 part=needs
No new crate, dependency, port, env var or listener; Cargo.lock unchanged.
@task_item n=9 part=needs
Gates: the fast and doc CI legs, the stand checks in both layout modes, and fork CI green on the pushed commit; the unread CI verdict of the previous chunk commit is re-read at P3 and before this chunk's push, never folded as green.
@task_item n=10 part=needs
The masters' pointer rows naming the answer writer's file (the project pointer table's driver command line row, the escher-driver service doc, the architecture doc's standard contracts section) follow the moved files as the wrap's leaf work.
@task_item n=11 part=needs
No capability is expected to be claimed and no new requirement line is written; the later stand requirement sweep and cold-agent bar read the split code but are not built here.
@task_item n=1 part=not_needed
The ShellProvider window chrome controls (drag, maximize, minimize, close, decorations): window chrome of the desktop shell, not the driver's answer writing.
@task_item n=2 part=not_needed
The stand's stable element ids and stand-control decisions (keyed controls, ids and accessibility by attributes, accent colour, lint-clean boot, CRUD keyboard region, flight type select, open list rows' bounds): what the served Seven GUIs pages render, which this chunk does not change.
@task_item n=3 part=not_needed
Stand page layout and boot decisions (fixed document skeleton with main mount, pinned headless viewport, held instance as fresh boot, session roles sharing the stand boot, load page network provider, into-view scrolling, visible region through clipping boxes): engine and stand behaviour the writer only reports.
@task_item n=4 part=not_needed
The agent-run.sh and cold-agent.sh pipes and the cold-agent decisions (prompt names no id, pipe builds nothing, drives through the driver's MCP server): test-harness scripts outside the driver crate, not touched.
@task_item n=5 part=not_needed
The multi_doc and paint_bench command lines (arguments, env vars, backend, stdout): benchmark examples with their own output, not the driver's answers.
@task_item n=6 part=not_needed
The absence of arbitrary index.html templates for a document: a document-templating question the restructure never reaches.
@task_item n=7 part=not_needed
The mobile-native and web-spa surfaces: they hold no rows and the driver crate is not part of them.
