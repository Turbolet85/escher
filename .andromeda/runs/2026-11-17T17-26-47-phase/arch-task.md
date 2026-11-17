@task_prose part=chunk
Splits the escher-driver's hand-written JSON answer writer by answer family into private child modules: the caused forms, the layout answer, the snapshot node forms and the diff forms, beside the shared primitives (string, number, Object, list, texts). P4 decides from research where each unnamed form goes (UI graph, actions, lint finding, shot, coverage, outcome, refusal, session error, the started/status/stopped session lines): its own family module or the parent.
The writer stays the parent module, as the driver's mcp and schema modules are, and its callers' paths keep resolving through re-exports. Its inline unit tests (over half the file) move with their family or into a path-attached test child module. Every test is kept and the test count is recorded before and after.
This is a corrective chunk with no behaviour change. It re-asserts that every carrier's answers stay byte for byte the same (the socket call, the command line, MCP, kept checks). It re-reads the Counter kept checks, the MCP and command-line checks and the cross-OS answer digests. It re-reads the CI verdict of the last chunk commit before the push.
@task_file n=1 path=packages/escher-driver/src/json.rs mark=edits
what: the driver's own hand-written JSON answer writer (output only, reads no JSON), which writes every answer form for the socket, the command line, MCP and kept checks — edits: the module, which keeps the shared primitives and the module doc and declares the family child modules and re-exports the items that move; the inline mod tests, which move out
@task_file n=2 path=packages/escher-driver/src/json/caused.rs
what: the answer writer's caused family: the acting-verb results' caused and caused_past forms and the cut-text form
@task_file n=3 path=packages/escher-driver/src/json/layout.rs
what: the answer writer's layout family: the layout answer with its placing, shift, size-cause, length and fallback words
@task_file n=4 path=packages/escher-driver/src/json/snapshot.rs
what: the answer writer's snapshot family: the snapshot node form (the node keys, the reading words) and the paged snapshot
@task_file n=5 path=packages/escher-driver/src/json/diff.rs
what: the answer writer's diff family: the driver JSON diff answer (added, removed, changed, differs) and the acted outcome form
@task_file n=6 path=packages/escher-driver/src/json_tests.rs
what: the answer writer's unit tests, moved out of the parent as a path-attached test child module, with every test kept
@task_file n=7 path=tests/blitz-tests/checks/counter-cli.json mark=re-reads
what: the committed stand-Counter kept check for the command line. stand_replay replays it and must match each answer whole and by its 64-bit digest
@task_file n=8 path=tests/blitz-tests/checks/counter-mcp.json mark=re-reads
what: the committed stand-Counter kept check for MCP. stand_replay replays it and must match each answer whole and by its 64-bit digest
@task_file n=9 path=examples/seven_guis/tests/cli_commands.rs mark=re-reads
what: the command-line check of the escher-session verbs and their written answers
@task_file n=10 path=examples/seven_guis/tests/cli_flow.rs mark=re-reads
what: the command-line flow check run under plain sh, which reads the written answers end to end
@task_item n=1 part=needs
Restructure invariant. Every form the writer writes keeps its bytes: the answer JSON forms (node, paged snapshot, diff, actions, layout and its words, lint finding, screenshot, outcome, string escaping), the acting-verb results with caused and caused_past, the UI graph answer, coverage's written form, the refusal JSON, the session error JSON, and the started, status and stopped lines. No key, field, cause, session error or exit is added, removed or reordered. VERBS, NODE_FIELDS and the help self-description stay unchanged. The writer's unit tests, describe's tests, the Counter kept checks replayed by stand_replay, stand_mcp_tools, the command-line checks and stand_cross_os_readings' answer digests hold all of this.
@task_item n=2 part=needs
Answers stay written by the driver's own hand-written JSON writer, which only writes. No serde writer takes its place. serde_json stays the driver's reader only, and no serde_json type reaches the driver's public API.
@task_item n=3 part=needs
Every new family module is private to escher-driver (pub(crate) at most). The crate's restricted-internals rule applies. The driver's public API does not move, and the lib's pub use list stays unchanged.
@task_item n=4 part=needs
Module shape follows the driver's own precedent: the parent module with private children, as mcp has framing, listing and result. Callers keep their paths (json::Object, json::list, json::string, the session lines, shot_without_image from describe, mcp, mcp framing, the command line and recording) through re-exports from the parent where an item moves.
@task_item n=5 part=needs
Moved unit tests must reach the moved items: their test module stays a child of the module it tests, attached by path as the driver's other split tests are. Each test keeps its name, and none is weakened or dropped. The test count is recorded before and after the move.
@task_item n=6 part=needs
The measure: the writer's size. It is 1899 lines at the base commit, and the code audit's M1 measured its code lines growing from 1244 to 1861 over the epoch. The cognitive complexity ceiling still applies to every function in the new modules.
@task_item n=7 part=needs
A move with no edit to any function body is how bytes are kept. P3 measures which of the witnesses would see a changed byte.
@task_item n=8 part=needs
Gates: the ci-leg runner's fast and doc legs, with rustdoc warnings as errors and fmt and clippy clean. The stand checks run in both layout modes, and fork CI must be green on the pushed commit. The escher-driver per-crate coverage floor must still hold after the tests move.
@task_item n=9 part=needs
No new crate, dependency, port, environment variable or listener, and Cargo.lock stays unchanged (the writer keeps encoding base64 by hand, with no base64 crate).
@task_item n=10 part=needs
No upstream-owned file is touched, the upstream DOM document module included. Splitting an upstream file would turn every later cherry-picked upstream sync into a conflict.
@task_item n=11 part=needs
The driver adds no log field or span, and the move logs no answer content: the driver's spans-only, no-content logging rules hold over the moved code.
@task_item n=12 part=needs
The CI verdict of the last wrap's chunk commit was still running when the scope was pinned. It is never folded in as green: it is re-read at P3 and again before this chunk's push.
@task_item n=13 part=needs
The masters' pointer rows that name the writer's file follow the new files as the wrap's leaf work: the driver command-line row, the escher-driver service doc and the Standard Contracts rows.
@task_standing n=1
convention: each new module file opens with a module doc and documents its items; rustdoc warnings are errors
home: conventions
@task_standing n=2
convention: a unit-test module split out of its source file stays a child of that module, attached by a path attribute, and keeps its test paths
home: conventions · existing-scopes
@task_standing n=3
convention: clippy allows stay local to the item they excuse, and expect and unwrap carry a justification
home: conventions
@task_item n=1 part=not_needed
Engine behaviour in blitz-dom (style, layout, Taffy, Parley, damage, scrolling, hit testing, the select list, form controls, event dispatch): the chunk changes no engine code and only moves where answers are written.
@task_item n=2 part=not_needed
Paint, renderers, the GPU layer, the window shell, winit, Android, iOS and wasm targets: none is reached.
@task_item n=3 part=not_needed
The WPT runner, its result lines, its CI artifacts and the wpt diff-to-PR script: untouched.
@task_item n=4 part=not_needed
The browser app, its history store, persistence, about pages and toolbar: untouched.
@task_item n=5 part=not_needed
Script execution (Boa, the JS runtime, script globals, the script fetcher): untouched.
@task_item n=6 part=not_needed
The accessibility tree and the AccessKit adapter: the writer reads snapshot readings already built and does not change how they are built.
@task_item n=7 part=not_needed
The cold-agent and agent-run scripts, their events, exit codes, verdicts and state: these belong to later entries, which will read the split code.
@task_item n=8 part=not_needed
The session host's process model, the state and recording directories' ownership rules, and idle expiry: no host or file handling changes.
@task_item n=9 part=not_needed
The telemetry bootstrap, subscriber, scrubbing and line format: no telemetry is installed or changed.
@task_item n=10 part=not_needed
The names register (binaries, crates, other names): no crate, binary or environment name is added.
@task_item n=11 part=not_needed
How the UI graph registry, stable element ids and the Dioxus bridge produce what is read: the chunk does not change any producer.
@task_item n=12 part=not_needed
The package-install script, the Nix flake, the MSRV job and the dependency audit tooling: untouched.
