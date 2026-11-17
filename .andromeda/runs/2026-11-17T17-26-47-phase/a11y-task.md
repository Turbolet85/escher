@task_prose part=chunk
Splits the driver's answer writer, the hand-written output-only JSON writer every carrier's answers pass through, into private child modules by answer family: the caused forms, the layout answer, the snapshot node forms and the snapshot diff forms, with the shared primitives kept in the parent.
The writer's inline unit tests move with their family or to a sibling tests file; every test is kept, none weakened or dropped, and the test count is recorded before and after.
No behaviour change: no function body is edited, callers' paths keep resolving through the parent by re-export, and every carrier's answer stays byte for byte as it was.
It re-reads the existing witnesses of the written forms: the writer's and describe's unit tests, the committed Counter replay checks, the MCP and command-line checks, and the cross-OS reading digests.
The engine's document file is left whole and untouched.
@task_file n=1 path=packages/escher-driver/src/json.rs mark=edits
what: the driver's answer writer, the output-only JSON writer of every answer form: the snapshot node, the snapshot diff, the caused list, the layout answer, the UI graph, the actions answer, the lint finding, the screenshot, handler coverage, refusals, session errors and session lines — edits: its top-level items moved into family child modules, its inline mod tests moved out, re-exports for moved items
@task_file n=2 path=.claude/docs/services/escher-driver.md mark=edits
what: the driver service's doc, the pointer rows that name the answer writer's file — edits: the rows naming the writer, made to follow the split files
@task_file n=3 path=CLAUDE.md mark=edits
what: the project's standing instructions — edits: the Driver command line row that points at the answer writer
@task_file n=4 path=packages/escher-driver/src/describe.rs mark=re-reads
what: describe's unit tests over the written answer forms, a caller of the writer whose paths must keep resolving
@task_file n=5 path=tests/blitz-tests/tests/stand_replay.rs mark=re-reads
what: the stand replay check, replaying the committed Counter checks and comparing each answer whole and by its 64-bit digest
@task_file n=6 path=tests/blitz-tests/checks/counter-cli.json mark=re-reads
what: the committed Counter check over the command-line carrier, its kept answers read back byte for byte
@task_file n=7 path=tests/blitz-tests/checks/counter-mcp.json mark=re-reads
what: the committed Counter check over the MCP carrier, its kept answers read back byte for byte
@task_file n=8 path=tests/blitz-tests/tests/stand_mcp_tools.rs mark=re-reads
what: the MCP tools check over the stand, reading MCP tool results
@task_file n=9 path=examples/seven_guis/tests/cli_commands.rs mark=re-reads
what: the command-line carrier's commands check
@task_file n=10 path=examples/seven_guis/tests/cli_flow.rs mark=re-reads
what: the command-line carrier's flow check
@task_file n=11 path=tests/blitz-tests/tests/stand_cross_os_readings.rs mark=re-reads
what: the cross-OS readings check and its answer digests
@task_item n=1 part=needs
The restructure's invariant: the answer writer's forms move by family into private child modules — the snapshot node and diff forms (node parts, reading word, node, nodes, acted), the caused forms, the layout answer, and the remaining forms (UI graph, actions, lint finding with bounds and ratio, screenshot, handler coverage, outcome, refusal, session error, session lines) to their own module or the parent — each writing the same bytes it writes today; held by the writer's own unit tests, describe's tests, the Counter replay checks, the MCP and command-line checks and the cross-OS answer digests.
@task_item n=2 part=needs
The writer stays output only: it reads no JSON, and no serde writer or other library replaces the hand-written one.
@task_item n=3 part=needs
Every new module is private to the driver crate (crate-visible at most); no public API moves, the library's public re-export list is unchanged and the doc leg stays green.
@task_item n=4 part=needs
Callers' paths into the writer (the object type, the list and string primitives, the started, status and stopped session lines, the screenshot form without image) keep resolving through the parent module, by re-export where an item moves.
@task_item n=5 part=needs
The moved inline tests keep every test, none weakened or dropped, reaching moved items through the module path; the test count before and after is recorded, and the tests follow the crate's own child-module and sibling tests-file precedent.
@task_item n=6 part=needs
The move is a move only: no function body is edited, so no written form's bytes shift — no carrier, verb, argument, key name, result field, cause, session error, exit, log field or span added, removed or reordered.
@task_item n=7 part=needs
The driver's verb list, node field list and help self-description stay unchanged, as does every answer form's field order and key spelling.
@task_item n=8 part=needs
No new crate, dependency, port, environment variable or listener; the lock file is unchanged.
@task_item n=9 part=needs
The engine's document file and every other upstream-owned file stay untouched, so later upstream syncs take no new conflict.
@task_item n=10 part=needs
The gates the chunk is read by: the fast and doc CI legs, the stand checks in both layout modes, and fork CI green on the pushed commit.
@task_item n=11 part=needs
The doc pointer rows that name the answer writer's file follow the new files, as the wrap's leaf work.
@task_standing n=1
convention: every check of the stand's snapshot, diff, actions and MCP answers reads the answer through the one snapshot model and its stated expectation tables, never a second reader
home: screen-reader-support · 7-screen-reader-support
@task_standing n=2
convention: stand checks run three rounds and gate CI through the accessibility test leg
home: ci-integration · 9-ci-integration
@task_item n=1 part=not_needed
Keyboard navigation and activation rows (Tab order, Enter and Space, range input keys, select list keys, click focus): the chunk changes no input handling or focus behaviour.
@task_item n=2 part=not_needed
ARIA role mapping, hidden-content exclusion and ARIA property and state reads into the accessibility tree: the chunk builds no tree and reads no attribute.
@task_item n=3 part=not_needed
Visual design rows (contrast thresholds, scrollbar thumbs, select list colours and motion, scheme and zoom): no paint, colour or layout value changes.
@task_item n=4 part=not_needed
Accessibility feature defaults, the AccessKit platform adapter and the stand's lack of one: no feature flag, dependency or adapter is touched.
@task_item n=5 part=not_needed
Cognitive accessibility rows (invalid-state presentation, shared tab-order table): no stand component or its markup changes.
