@history_item n=1 marker=2026-10-06-headless-stand
**2026-10-06-headless-stand** — added the headless stand's `task_in_shell` TaskShell mount at the pinned 800 × 600 viewport and re-pointed `app.rs` citations after a line shift. This is the mount that decision/held-instance-reads-as-fresh-boot and decision/session-roles-share-stand-boot rest on. An extraction in the stand-test functions (`stand_graph.rs`, `stand_actions.rs`, `stand_accessibility_ids.rs`) has to keep booting through it unchanged.

@history_item n=2 marker=2026-10-06-stable-element-ids
**2026-10-06-stable-element-ids** — the lean tasks' controls got author ids (counter-*, flight-*, timer-*, crud-*) as attributes only, with no CSS selecting by them. This is the origin of the contract/stand-element-ids/* rows and decision/stand-ids-as-attributes-alone. The test functions this chunk splits read these ids, so the id set must come through the extraction unchanged.

@history_item n=3 marker=2026-10-06-id-persistence
**2026-10-06-id-persistence** — CRUD rows' Dioxus key changed from index `{i}` to `{person.id}` so that a row id follows its person under filter, Create and Delete. This is the precedent behind contract/stand-element-ids/crud-person-id. A test helper extracted from the graph or actions checks must keep reading person-keyed rows, not indices.

@history_item n=4 marker=2026-10-06-accessibility-tree-identity
**2026-10-06-accessibility-tree-identity** — the six stand inputs were named by attributes alone (`for` on labels, `aria-label` "Departure date" / "Return date"), and citations were re-pointed by a measured line map. This is the origin of decision/stand-a11y-by-attributes-alone. It is what `every_interactive_role_element_reads_a_name` (`stand_accessibility_ids.rs`, 16) asserts, so a split of that function keeps every name check. It is also the precedent for re-pointing citations after lines move.

@history_item n=5 marker=2026-10-06-id-stability-across-code-edits
**2026-10-06-id-stability-across-code-edits** — added the `crud-person-{person.id}` author id beside the key, and Home's seven `task-card-{slug}` ids, as attributes only, on the founder's rule that every actionable element reads an author key. This is the origin of decision/every-stand-control-keyed and the task-card-* and crud-person-id rows. The keyed and unkeyed readings in `stand_actions.rs` and `stand_graph.rs` assert this rule.

@history_item n=6 marker=2026-10-07-driver-session
**2026-10-07-driver-session** — the session host became a third consumer of the TaskShell mount. It boots through `seven_guis::stand` only and adds no markup. A held instance reads as a fresh boot, with the mount and viewport holding after commands. This is the origin of decision/session-roles-share-stand-boot, decision/held-instance-reads-as-fresh-boot (guard `stand_session_fresh`) and contract/escher-session-host/log-service-name. Moves in `packages/escher-driver/src/command.rs` and `cli.rs` must keep that single boot path.

@history_item n=7 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** — two citations into `examples/seven_guis/src/session_host.rs` were re-pointed after lines were added, with no claim changed. This is a precedent for a no-behaviour line shift that still owes re-pointed citations into session-host and driver code.

@history_item n=8 marker=2026-10-07-refusal-detection
**2026-10-07-refusal-detection** — the stand's nested scroller (`crud-list` clipping its rows): after twelve Creates the last row is refused `off-screen` until scrolled. This is the behaviour behind contract/escher-session-refusals/off-screen. Refusal paths in `command.rs` and `json_tests.rs` must keep it exactly.

@history_item n=9 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** — `escher-session` became the driver's command line, `escher-session <verb> [arguments] --session <dir>`, with four endings: 0 accepted JSON, 1 refused, 2 usage on stderr, 3 session error. The old two-argument form now reads `unknown-verb`, and the citations were replaced by the reader's and the endings'. This is the origin of contract/escher-session-command/form, the exit-codes rows and contract/escher-session-refusals/unknown-verb. It is the behaviour `read` in `packages/escher-driver/src/cli.rs:156` (cognitive complexity 25) implements, so an extraction there must keep every ending and the usage line byte for byte.

@history_item n=10 marker=2026-10-10-scrolling-box-bounds-and-hit
**2026-10-10-scrolling-box-bounds-and-hit** — Home's `#home` is a scrolling box, and the seventh card, `task-card-cells`, shows and is hit only once scrolled into view. A standing check that clicked it unscrolled went red. This is a trap for any shared test helper this chunk extracts that acts on the task-card ids.

@history_item n=11 marker=2026-10-10-snapshot-states
**2026-10-10-snapshot-states** — roles and states were stated by attributes alone (`aria-invalid`, `role="listbox"` / `"option"` with `aria-selected`), and the styling classes stayed. This is the origin of decision/stand-a11y-by-attributes-alone. These states are what the snapshot reads, so a move in `packages/dioxus-native-dom/src/snapshot.rs` (913 lines) must keep the role and state output unchanged.

@history_item n=12 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — added the `mcp` role: an MCP server over stdio, local, with no listener or auth. It reads JSON-RPC 2.0 on stdin, writes protocol lines only on stdout, echoes the admitted `id`, and ends with exit 0 on Ok or 3 on Err. It also scoped the ban on writing back caller input to the commands. This is the origin of decision/mcp-server-local-stdio, contract/escher-session-mcp/stdio, contract/escher-session-mcp/end and decision/no-caller-input-echoed. It is the behaviour `run` in `packages/escher-driver/src/mcp.rs:215` (18) implements.

@history_item n=13 marker=2026-10-12-self-description
**2026-10-12-self-description** — added `escher-session help` (seven keys of fixed table text, reaching no session), `--session` exactly once except for `help`, and the session error's `remedy`. The `mcp` role went to four revisions, with each tool's description being the verb's help plus its result fields. This is the origin of contract/escher-session-help/answer, contract/escher-session-mcp/protocol-revisions and contract/escher-session-mcp/tools. The `cli.rs` and `mcp.rs` extractions and the split of `json_tests.rs` must keep these fixed strings.

@history_item n=14 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — added the `screenshot` verb (`{"width","height","png"}`, `--id` crop) and the `fonts-unavailable` and `off-screen` refusals. Over MCP it answers an image block then a text block, with a 1,048,576-byte result bound. This is the origin of contract/escher-session-screenshot/answer, contract/escher-session-mcp/screenshot-result, contract/escher-session-mcp/result-bound and contract/escher-session-refusals/fonts-unavailable. The bound and the block order live on the `mcp.rs` `run` path being extracted.

@history_item n=15 marker=2026-10-13-stand-keyboard-harness
**2026-10-13-stand-keyboard-harness** — `crud-list` took `tabindex="0"` and an `onkeydown`, with arrow, Home and End selection, and CRUD gained a sixth Tab stop. No element was added. This bears on contract/stand-element-ids/crud-list. `stand_keyboard.rs` (1,063 lines) is one of the test files left for a later entry.

@history_item n=16 marker=2026-10-14-stand-a11y-assertions
**2026-10-14-stand-a11y-assertions** — `crud-list` was named "People" by `aria-label` alone. This is the origin of contract/stand-element-ids/crud-list's listbox named "People". It is one of the names `every_interactive_role_element_reads_a_name` (`stand_accessibility_ids.rs:256`) asserts.

@history_item n=17 marker=2026-10-14-stand-requirement-sweep
**2026-10-14-stand-requirement-sweep** — the three non-lean tasks got author ids (temp-*, circle-* including the dialog's, `cell-{A..Z}{0..25}`, `cell-input`) as `id:` lines alone, and `unkeyed_actionable()` reads 0 on all seven tasks. This is the origin of the temp-*, circle-* and cell-* rows and of decision/every-stand-control-keyed's "an open dialog included". The keyed-count checks in the functions being extracted assert it.

@history_item n=18 marker=2026-10-19-session-recording
**2026-10-19-session-recording** — `--record <dir>` on `start`, the host role and `mcp` keeps a session as one owner-only replayable check (`0600` file in a `0700` directory). It adds the session errors `record-dir-not-private` and `recording-failed`, and the usage cases for a misplaced, doubled or valueless `--record`. This is the origin of decision/session-recording-replayable-check and the contract/escher-session-record/* rows. These usage and recording branches sit in the `cli.rs` `read` and `command.rs` code this chunk restructures.

@history_item n=19 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** — the `--record` check keeps, per call, the verb, arguments, acceptance, digest and masked answer (a shot without its `png`), read back by `Check::replay`. This is the decision/session-recording-replayable-check row as it stands now. Answer-shape tests split out of `json_tests.rs` must keep the masked forms.

@history_item n=20 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** (the gated part, `escher-session lint`) — the `lint` verb answers `{"findings":[…]}` (or is refused `fonts-unavailable`) and is kept under `--record`, and `mcp` lists it after `screenshot`. This is the origin of contract/escher-session-lint/answer. Its answer form is one of the `json/{answer}.rs` children that `json_tests.rs` would split along.

@history_item n=21 marker=2026-10-20-load-wait
**2026-10-20-load-wait** — each acting tool's `settled` field states the load wait of up to 5 s, as fixed table text inside a ratified carrier. This feeds contract/escher-session-mcp/tools and help's self-description. A move must keep the fixed text byte for byte.

@history_item n=22 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — added `snapshot --part`, a whole number from 1 to 1000 (outside that, `malformed`): `{"text"}` for a whole screen or `{"text","part","parts"}` in parts, with a part past the last giving empty text. This is the origin of contract/escher-session-snapshot/answer and contract/escher-session-snapshot/part. Paging and bound checks may sit in the `snapshot.rs`, `command.rs` and `cli.rs` code being split.

@history_item n=23 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** — six served tasks (adding cells and load, through `boot_cells` / `boot_load`) in the host and `mcp` roles and the `start` enum. The flight ids became `flight-type`, `flight-type-one-way` and `flight-type-return` (a select, with no `aria-pressed`), and `load-fetch` / `load-done` were added. This is the origin of contract/escher-session-served-tasks/* (cells, load), the flight-type* rows and the load-* rows. The usage line's `apps:` list is built from the task names that `cli.rs` `read` uses.

@history_item n=24 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (citations into the files the take moved) — when code moved from one file into new module files (`runtime.rs` → `runtime/window.rs`, `element.rs` → `element/geometry.rs`), the citation paths were rewritten, not only their line numbers. This is the direct precedent and trap for this chunk's file splits (`json_tests.rs` → per-answer children, `snapshot.rs`, `command.rs`): any master or row citing the moved code needs new paths.

@history_item n=25 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (citations re-pointed · the load page's Tab sequence · the Cells grid) — thirteen citations were re-pointed for the chunk's line moves. `load-fetch` was recorded as taking Enter and Space, and `cells-grid` became one Tab stop, a keyboard region named "Cells" with no role. This is the origin of contract/stand-element-ids/load-fetch and contract/stand-element-ids/cells-grid. It is also a precedent for a re-point-only entry when files in the changed set shift lines.

@history_item n=26 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the open list; the lint's `in_select_box`) — while a select's list is open, each row reads its own `bounds` in the snapshot. After a close, and while the list stays closed, an option reads its select's client rect. This bears on contract/stand-element-ids/flight-type*. It is snapshot behaviour that a move in `packages/dioxus-native-dom/src/snapshot.rs` must keep.

@history_item n=27 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (citations after the graph registrations · escher-session gains the graph verb) — added the `graph` verb, `{"nodes","states"}` with ids and closed words only and never a value, which is the origin of contract/escher-session-graph/answer. Source citations, including the driver's `schema.rs`, were re-pointed after the added lines. The two `stand_graph.rs` functions this chunk extracts (22 and 16) check this answer against the code.

@history_item n=28 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (tab-order citations · the actions verb) — added the `actions` verb (unpaged `{"nodes"}` of id, owner, keyed and verbs, never a value or `NodeId`), the origin of contract/escher-session-actions/answer. It also re-pointed `tests/blitz-tests/tests/common/mod.rs` citations after `ACTIONABLE_AT_BOOT` was added. This bears on `each_served_screen_lists_its_actionable_nodes_with_the_tables_verbs` (`stand_actions.rs:159`, 21). It is also a trap: lines added to the shared test module shift the master citations into it.

@history_item n=29 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (coverage role as a mount consumer · bridge notes re-cited · the coverage role) — added `escher-session coverage <app> <check>...`, which replays each check on a fresh boot through the same boot `main` hands the command line and answers `{"unexercised","checks","held"}`. Malformed paths are refused `malformed`, and a failed boot gives exit 3. This is the origin of the contract/escher-session-coverage/* rows and of decision/session-roles-share-stand-boot's coverage role. Citations into the command line and the binary moved after the added lines, so this chunk's `cli.rs` and `command.rs` moves owe the same.

@history_item n=30 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — the master text became rows (decisions and contracts), with 72 relied citations cut: a master holds what a planner must respect and the code holds the rest. This sets the form of every picked row. A pure move or extraction should owe no row change unless a row's promise or guard names the moved code.

@history_item n=31 marker=2026-11-15T05-21-42-wrap
**2026-11-15T05-21-42-wrap** (ratified — the graph verb · the coverage role as a mount consumer · the coverage role) — the graph and coverage entries went from PROVISIONAL to ratified by the founder's own word. The rows for graph, coverage and session-roles-share-stand-boot stand ratified, so this chunk must not reword them.

@history_item n=32 marker=2026-11-15-session-binary-smoke
**2026-11-15-session-binary-smoke** — `agent-run.sh` gained the `session` selection, which runs the escher-session binary's flows in seven_guis. This bears on keeping the test target layout: the scope leaves the integration-test file splits for a later entry because target names are pinned.

@history_item n=33 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** — `~ contract/escher-session-command/form` (fifteen verbs), `~ contract/escher-session-mcp/tools` (layout after actions, help last), `~ decision/no-caller-input-echoed` (the layout id exception) and `+ contract/escher-session-layout/answer`. These are the current wordings of the picked cli rows that the `cli.rs`, `mcp.rs` and `command.rs` restructuring must leave exactly true. The verb count and tool order are fixed.

@history_item n=34 marker=2026-11-17-a-component-in-isolation
**2026-11-17-a-component-in-isolation** — the delegate confirm of the layout-on-the-carriers rows was ratified by the founder's own word, so contract/escher-session-layout/answer and the no-caller-input-echoed exception stand ratified and unchanged.

@history_item n=35 marker=2026-11-18-served-error-page
**2026-11-18-served-error-page** (Surface: cli · Surface: desktop-native) — `+ contract/escher-session-served-tasks/order` (the seventh task, listed after load) and `+ contract/stand-element-ids/order-*` (quantity, submit, retry, failed, export, status), plus the order page's lint-clean-at-boot. These are the newest picked rows. The served-task list and the id set that the stand tests in this chunk read include them.
