@task_prose part=chunk
Splits escher-driver's answer writer by answer family into private child modules: the caused forms, the layout answer, the snapshot node forms and the diff forms, beside the shared primitives; where each form the entry does not name lands is left to P4.
A corrective restructure with no behaviour change: every carrier's answers (the socket's call, the command line, MCP, a kept check) stay byte for byte as they are.
Moves the writer's inline tests with their family or into a sibling tests file by the crate's #[path] precedent, every test kept and the test count recorded before and after.
Re-reads the witnesses that already hold the written forms: the writer's and describe's unit tests, the committed Counter kept checks replayed by the stand, the MCP and command-line checks, and the cross-OS answer digests.
Leaves the upstream engine document file whole, adds no requirement line and claims no capability.
@task_file n=1 path=packages/escher-driver/src/json.rs mark=edits
what: the driver's answer writer, output only — the hand-written JSON of every command answer (snapshot node, diff, caused, layout, UI graph, actions, lint findings, shot, coverage line, outcome, refusal, session error) and the session lines started, status and stopped — edits: its top-level items moved by family into private child modules, its inline mod tests moved, re-exports kept for callers' paths
@task_file n=2 path=CLAUDE.md mark=edits
what: the project's pointer table — edits: the Driver command line row that names the answer writer's file
@task_file n=3 path=.claude/docs/services/escher-driver.md mark=edits
what: the escher-driver service description — edits: the pointer that names the answer writer's file
@task_file n=4 path=tests/blitz-tests/checks/counter-cli.json mark=re-reads
what: the committed Counter kept check over the command line, replayed by stand_replay, each answer kept whole and as a 64-bit digest
@task_file n=5 path=tests/blitz-tests/checks/counter-mcp.json mark=re-reads
what: the committed Counter kept check over MCP, replayed by stand_replay, each answer kept whole and as a 64-bit digest
@task_file n=6 path=examples/seven_guis/tests/cli_commands.rs mark=re-reads
what: the command-line check of the driver commands' written answers
@task_file n=7 path=examples/seven_guis/tests/cli_flow.rs mark=re-reads
what: the command-line flow check of the driver's written answers
@task_item n=1 part=needs
The restructure's invariant: the answer writer's units move by family into private child modules — the shared primitives (string, number, Object, list, texts), the snapshot node and diff forms, the caused forms, the layout answer, and the UI graph, actions, lint finding, shot, coverage line, outcome, refusal, session error and session-line forms — each still writing exactly the bytes it writes today on every carrier; held by the writer's own unit tests, describe's tests, the Counter kept checks replayed by stand_replay with their answer digests, stand_mcp_tools, the command-line checks and stand_cross_os_readings' answer digests.
@task_item n=2 part=needs
The keeping is done by moving code with no edit to any function body; P3 measures which of the existing witnesses would read a changed byte, and that measure is what the restructure is proved by.
@task_item n=3 part=needs
The writer stays output only: it reads no JSON, and no serde writer replaces the hand-written one in any moved module.
@task_item n=4 part=needs
Every new child module is private to escher-driver (pub(crate) at most); no public API moves, the crate's public re-export list stays unchanged, and the doc leg stays green.
@task_item n=5 part=needs
Callers' paths into the writer (the Object type, list, string, the started, status and stopped session lines, shot_without_image) keep resolving through the parent module, by re-export where an item moves, so the describe, MCP, MCP framing, command-line and recording code compile unedited.
@task_item n=6 part=needs
The moved tests: each inline test moves with its family or into a sibling tests file by the crate's #[path] precedent; every test kept, none weakened or dropped, and the test count before and after recorded.
@task_item n=7 part=needs
The moved writer code adds no logging: the driver emits no event and installs no subscriber, answer-bearing types are never logged or Debug-printed, and no span is opened or given a field by the move.
@task_item n=8 part=needs
No carrier, verb, argument, key name, result field, refusal cause, session error, exit status, log field or span is added, removed or reordered; the node field list, the verb list and the help self-description stay as they are.
@task_item n=9 part=needs
No new crate, dependency, port, environment variable or listener; the lockfile stays unchanged.
@task_item n=10 part=needs
The upstream engine document file and every other upstream-owned file stay untouched, so later upstream syncs meet no conflict from this chunk.
@task_item n=11 part=needs
The gates: the CI fast and doc legs, the stand checks in both layout modes, and fork CI green on the pushed commit; the previous chunk commit's CI verdict, still in progress, is re-read at P3 and before this chunk's push and never folded as green.
@task_item n=12 part=needs
The measure the split is read by: the writer's size, 1899 lines at HEAD and 1861 code lines by the code audit's M1, against one writer per answer family.
@task_item n=13 part=needs
The masters' pointer rows that name the writer's file (the project pointer table, the escher-driver service description, the architecture's standard contracts) follow the moved files as the wrap's leaf work.
@task_standing n=1
convention: each test file or test module the move creates is a silent check — it prints nothing on success and shares no noisy helper module
home: 3-observability-harness-contract
@task_item n=1 part=not_needed
Engine-side script error rows — the window error event, window.onerror, script error source labels, the embedder's drain of script diagnostics and the WPT report message — the chunk reaches no script runtime.
@task_item n=2 part=not_needed
Engine page behaviour rows — page script events, select and optgroup labels, multiple select form entries, the select size attribute, the mutation buffer and the state registry — the chunk changes no engine or DOM code.
@task_item n=3 part=not_needed
Engine loading and exploration rows — a failed stylesheet's pending set, file: loads from non-file documents, exploration catching a panicking call — none is reached by a move inside the driver's writer.
@task_item n=4 part=not_needed
The cold-agent pipe's and the agent-run harness's events, verdict fields, counted verbs, cleanup scope and transcript handling — no harness or pipe is changed.
@task_item n=5 part=not_needed
The telemetry sink's installation, its panic hook, its resource keys and its target allowlist — no binary's main or the sink crate is touched.
@task_item n=6 part=not_needed
The recording directory and session state directory privacy and the session's idle expiry — the session's lifecycle and storage are not changed.
@task_item n=7 part=not_needed
CI packaging rows — the package-install diagnostic line, per-leg log and coverage artifact uploads, WPT publishing from upstream — no CI workflow or script is changed.
