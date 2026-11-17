@task_prose part=chunk
Splits the driver's hand-written JSON answer writer by answer family (caused, layout, snapshot, diff) into private child modules under the writer as their parent, following the crate's own parent-and-private-children precedent.
A corrective restructure: no behaviour change, no new answer form, no new requirement line, no capability newly claimed; every carrier's answers (socket call, command line, MCP, a kept check) stay byte for byte as they are.
The writer's inline unit tests move with their family or to a sibling test file by the path-attribute precedent, every test kept, none weakened or dropped, the test count recorded before and after.
Re-verifies the existing witnesses of the written forms: the writer's and describe's unit tests, the committed Counter checks replayed whole and by 64-bit digest, the MCP and command-line checks, and the cross-OS answer digests; re-reads the unread CI verdict of the last chunk commit at P3 and before push.
Upstream's blitz-dom document file is left whole and no upstream-owned file is touched.
@task_file n=1 path=packages/escher-driver/src/json.rs mark=edits
what: the driver's answer writer, output only — the hand-written JSON forms every carrier answers in: shared primitives, snapshot node and diff forms, caused forms, the layout answer, UI graph, actions, lint findings, the shot, coverage, outcome, refusal and session-error forms, and the session lines, with its inline unit tests — edits: split by answer family into private child modules, items re-exported where they move, the inline tests module moved
@task_file n=2 path=tests/blitz-tests/checks/counter-cli.json mark=re-reads
what: committed Counter check recorded over the command line, replayed by the stand replay check, each answer kept whole and as a 64-bit digest
@task_file n=3 path=tests/blitz-tests/checks/counter-mcp.json mark=re-reads
what: committed Counter check recorded over MCP, replayed by the stand replay check, each answer kept whole and as a 64-bit digest
@task_file n=4 path=tests/blitz-tests/tests/stand_replay.rs mark=re-reads
what: the stand replay check that replays the committed Counter checks and compares each answer whole and by digest
@task_file n=5 path=tests/blitz-tests/tests/stand_mcp_tools.rs mark=re-reads
what: the MCP tools stand check holding the MCP tool results' written forms
@task_file n=6 path=tests/blitz-tests/tests/stand_cross_os_readings.rs mark=re-reads
what: the cross-OS readings check holding the served tasks' answer digests
@task_file n=7 path=examples/seven_guis/tests/cli_commands.rs mark=re-reads
what: the escher-session command-line checks of each command's stdout answer
@task_file n=8 path=examples/seven_guis/tests/cli_flow.rs mark=re-reads
what: the escher-session command-line flow checks run under cargo test
@task_file n=9 path=CLAUDE.md mark=edits
what: the project guide's pointer rows — edits: the Driver command line row that names the answer writer, following the files
@task_file n=10 path=.claude/docs/services/escher-driver.md mark=edits
what: the escher-driver service doc — edits: the pointer rows that name the answer writer, following the files
@task_item n=1 part=needs
The restructure's invariant: the writer's units move by family — shared primitives (string, number, Object, list, texts), snapshot node and diff forms, caused forms, the layout answer, and the forms the entry does not name (UI graph, actions, lint findings, shot, coverage, outcome, refusal, session error, session started/status/stopped lines) — each writing the same bytes, with NODE_FIELDS, VERBS and the help self-description unchanged; held by the writer's own unit tests, describe's tests, the Counter CLI and MCP checks replayed whole and by 64-bit digest, the MCP tools stand check, the seven_guis command-line and flow checks, and the cross-OS answer digests.
@task_item n=2 part=needs
Every new child module private to escher-driver, pub(crate) at most; no public API moved, the crate root's pub use list unchanged and the doc leg green.
@task_item n=3 part=needs
Callers keep their paths: the writer stays the parent module and items that move are re-exported so crate::json::{self, Object}, json::list, json::string, json::started / status / stopped and shot_without_image still resolve from describe, MCP, MCP framing, the CLI and recording.
@task_item n=4 part=needs
The module shape follows the crate's precedent: a parent file with private children in a same-named directory, and its tests in a sibling test file reached by a path attribute, as the MCP and schema modules do.
@task_item n=5 part=needs
The moved inline unit tests: every test kept, none weakened or dropped, moved with its family or into a sibling test file by the path-attribute precedent, test paths kept, and the test count before and after recorded.
@task_item n=6 part=needs
The writer stays output only: it reads no JSON and no serde writer replaces the hand-written one.
@task_item n=7 part=needs
The move is made with no edit to any function body; P3 measures which of the existing witnesses would read a changed byte.
@task_item n=8 part=needs
Upstream's blitz-dom document file and every other upstream-owned file left untouched, no other file the audit listed split.
@task_item n=9 part=needs
No new crate, dependency, port, env var or listener; Cargo.lock unchanged.
@task_item n=10 part=needs
Gates: the leg script's fast and doc legs, the stand checks in both layout modes, and fork CI green on the pushed commit.
@task_item n=11 part=needs
The last chunk commit's CI verdict is unread and never folded as green; it is re-read at P3 and before this chunk's push.
@task_item n=12 part=needs
No capability newly claimed and no requirement line added; capability claims and the schema-pinned capability statement stay as they are.
@task_item n=13 part=needs
The masters' pointer rows that name the answer writer (the project guide's driver command line row, the escher-driver service doc, the architecture's standard contracts) follow the moved files.
@task_standing n=1
convention: unit tests sit in an inline test module of their source file, or a sibling test file the source reaches by a path attribute
home: unit-test-strategy · 4-unit-test-strategy
@task_standing n=2
convention: a split test module keeps its tests' paths and names
home: unit-test-strategy
@task_standing n=3
convention: Rust tests on the built-in test harness, one integration target per file, shared modules no target
home: 3-test-harness-contract
@task_item n=1 part=not_needed
The WPT conformance runner, its command line, outputs, status set, pinned commit and reftest fonts: the chunk touches no layout engine conformance.
@task_item n=2 part=not_needed
The cold-agent pipe — its events, verdict, exit codes, invocation, isolation and wrong-call bar: the cold-agent bar is a later entry, out of scope.
@task_item n=3 part=not_needed
The agent-run five-verb contract — its verbs, events, exit codes, selections and state files: no agent-run code changes.
@task_item n=4 part=not_needed
The engine harness's construction, input helpers, inspection, settle, screenshot and contrast rows: the chunk changes no harness surface.
@task_item n=5 part=not_needed
Mocking and stubbing discipline — hand-written stand-ins, network stand-ins, timer ticks, the virtual clock and advance: no stand-in or clock is touched.
@task_item n=6 part=not_needed
CI workflow mechanics — package installs and the apt-install script's exit codes, actions pinned to SHAs, caches, trigger branches, disk freeing, artifacts: no workflow or CI script is edited.
@task_item n=7 part=not_needed
Engine and upstream rendering rows — buffered render on the Dioxus bridge, select list painting, scrollbar fade, box-shadow expectations, incremental layout oracle, blitz-dom fixtures built by hand: no engine code changes.
@task_item n=8 part=not_needed
The session host's state directory, idle expiry, owner-only files and recording directory permissions: no host or recording behaviour changes.
@task_item n=9 part=not_needed
Example fixtures read by a human and the absence of windowed end-to-end tests: the chunk adds no fixture and launches no window.
@task_item n=10 part=not_needed
Test data and fixture pages — shared fixture pages, the load-page stand-in's fixed delay, graph drives exercising every stand write: no fixture or stand app changes.
