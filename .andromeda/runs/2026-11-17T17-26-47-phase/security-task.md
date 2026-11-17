@task_prose part=chunk
Splits the driver's answer writer into private child modules by answer family: the caused forms, the layout answer, the snapshot node forms and the diff forms, beside the shared primitives; the parent stays and re-exports what moves.
A corrective restructure: no function body is edited, no behaviour changes, no new requirement line, no answer form added.
Its inline tests move with their family or to a sibling test file by the crate's #[path] precedent, every test kept, the test count recorded before and after.
Re-reads every carrier's written answers byte for byte (socket call, command line, MCP, kept checks) through the witnesses that already hold them, and re-reads the last chunk's CI verdict at P3 and before the push.
@task_file n=1 path=packages/escher-driver/src/json.rs mark=edits
what: the driver's answer writer, output only — the hand-written JSON forms of the snapshot and diff answers, the caused forms, the layout answer, the graph, actions, lint findings, shot, coverage line, refusal and session error forms, and the session lifecycle lines — edits: split into private family child modules, its inline mod tests moved
@task_file n=2 path=packages/escher-driver/src/lib.rs mark=re-reads
what: the driver crate's public pub use list, read back unchanged, held by the doc leg
@task_file n=3 path=tests/blitz-tests/checks/counter-cli.json mark=re-reads
what: committed stand-Counter kept check for the command line, replayed by stand_replay, each answer kept whole and as a 64-bit digest
@task_file n=4 path=tests/blitz-tests/checks/counter-mcp.json mark=re-reads
what: committed stand-Counter kept check for the MCP server, replayed by stand_replay, each answer kept whole and as a 64-bit digest
@task_file n=5 path=examples/seven_guis/tests/cli_commands.rs mark=re-reads
what: the command-line checks of the escher-session commands and their answer lines
@task_file n=6 path=examples/seven_guis/tests/cli_flow.rs mark=re-reads
what: the command-line flow checks of an escher-session run and its answer lines
@task_file n=7 path=CLAUDE.md mark=edits
what: the masters' pointer rows to the driver — edits: the Driver command line row that names the answer writer's file
@task_file n=8 path=.claude/docs/services/escher-driver.md mark=edits
what: the driver service document — edits: its pointer to the answer writer's file
@task_item n=1 part=needs
The restructure's invariant: the units moved — the shared primitives (string, number, Object, list, texts), the snapshot node and diff forms, the caused forms, the layout answer, and the graph, actions, lint, shot, coverage, refusal, session error and session-line forms wherever P4 lands them — each still writes its answer byte for byte as before, with no carrier, verb, argument, key name, result field, refusal cause, session error, exit, log field or span added, removed or reordered, and NODE_FIELDS, VERBS and the help self-description unchanged; held by the writer's own unit tests, describe's tests, the Counter checks replayed by stand_replay whole and by digest, stand_mcp_tools, the seven_guis command-line checks and stand_cross_os_readings' answer digests.
@task_item n=2 part=needs
The writer stays output only: it reads no JSON, and no serde writer replaces the hand-written one; serde_json stays the read-only reader of untrusted input, never a writer of answers.
@task_item n=3 part=needs
Every new family module is private to the driver crate, pub(crate) at most; no public API moves, the crate's pub use list stays unchanged and the doc leg stays green.
@task_item n=4 part=needs
Callers keep their paths: crate::json and its Object, list, string, started, status, stopped and shot_without_image still resolve for the describe, MCP server, MCP framing, command-line and recording code, by re-export from the parent where an item moves.
@task_item n=5 part=needs
Moved tests follow the crate's precedent (a private child module beside the parent, tests reached through a #[path] test file); every test is kept, none weakened or dropped, and the test count before and after is recorded.
@task_item n=6 part=needs
No new crate, dependency, port, environment variable or listener; the lockfile stays unchanged, so locked builds and the dependency audit and licence gates read the same graph.
@task_item n=7 part=needs
No upstream-owned file is touched: the engine's document file stays whole, so later upstream syncs take upstream's files without conflict.
@task_item n=8 part=needs
Gates the chunk must pass: the fast and doc CI legs run locally, the stand checks in both layout modes, and fork CI green on the pushed commit.
@task_item n=9 part=needs
The prior chunk commit's CI verdict is still unread and is never folded as green; it is re-read at P3 and before this chunk's push.
@task_item n=10 part=needs
The masters' pointer rows that name the answer writer's file follow the new files, as the wrap's leaf work.
@task_standing n=1
convention: a test file or test module the plan moves takes no tracing and no style dependency into the test harness
home: dependency-security
@task_item n=1 part=not_needed
Engine-side input handling — select lists, range inputs, password painting, CSS and selector parsing, attribute readers, accessibility adapter rows — the chunk moves only the driver's written answers and touches no engine code.
@task_item n=2 part=not_needed
Outbound requests, script fetchers, navigation, iframe and import limits, CORS and same-origin absences — no fetch or navigation code is reached.
@task_item n=3 part=not_needed
Browser app data at rest: cookie store, HTTP cache, history database, clipboard and file dialog, TLS — none are reached.
@task_item n=4 part=not_needed
CI secrets, signing material, publish job guards and read-only CI tokens — no workflow or release file changes.
@task_item n=5 part=not_needed
Cold-agent pipe and agent-run harness rows (call allowlists, fixed prompt, transcripts, recordings, wrong-call bar) — the cold-agent bar is a later entry that reads the split code, not this chunk.
@task_item n=6 part=not_needed
WPT runner rows and its result kinds, quarantine entries and PR lookup — the test-suite runner is not reached.
@task_item n=7 part=not_needed
Engine-side error handling rows (Dioxus mounted elements, CSSOM script errors, JS module loading, error pages, history store errors) — no engine error path is reached.
@task_item n=8 part=not_needed
Network tracing and resource load failure log rows and the engine logging stack choice — the chunk adds no log line and touches no engine logging.
