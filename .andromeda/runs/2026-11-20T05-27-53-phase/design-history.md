@history_item n=1 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** — the driver command line's output form: one JSON line per command on stdout (answer, refusal or session error's object, the schema's words), usage error one fixed stderr line with stdout empty, session error its fixed stderr message, no colour, no ESC byte, no terminal test, exit 0/1/2/3. Origin of decision/escher-session-agent-read-surface, decision/escher-session-no-colour, decision/driver-output-for-agents and the exit-status/stdout/stderr contracts; extracting `read` in `packages/escher-driver/src/cli.rs` and splitting `command.rs` must keep every byte of this form unchanged (founder's word on stdout shape and exit codes).

@history_item n=2 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — the `mcp` role's stdout: newline-delimited JSON-RPC 2.0 lines only, a reply per non-notification request, no ESC byte asserted on every reply by the shared `Speaker`, an `Err` ending's fixed message on stderr with status 3. Source of contract/escher-session-mcp/stdout and err-ending and contract/escher-session-stdout/mcp-line; the extraction of `run` in `packages/escher-driver/src/mcp.rs` must keep reply-per-request, end-writes-nothing and the Err-to-3 ending exactly.

@history_item n=3 marker=2026-10-12-self-description
**2026-10-12-self-description** — exit statuses read from the schema's `EXIT_STATUSES` table that `help` serves; `help` one uncoloured line reaching no session; a session error's object carries its fixed `remedy`; mcp `help` result is the self-description line. Source of decision/exit-statuses-from-schema-table, contract/escher-session-commands/help, contract/escher-session-session-error/remedy, contract/escher-session-mcp/help-result; a cli/command split must keep the exit status read from that table, not re-coded.

@history_item n=4 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — `screenshot [--id]` answers `{"width","height","png"}` one line; over mcp two content blocks (image block, then text line without `png`). Source of contract/escher-session-commands/screenshot and contract/escher-session-mcp/screenshot-result (founder's fork 1); the mcp `run` extraction and any `json_tests.rs` split touching the shot's written form must keep the two-block reply and the line's keys.

@history_item n=5 marker=2026-10-19-session-recording
**2026-10-19-session-recording** — `record-dir-not-private` and `recording-failed` sit in the exit-3 session-error class with no exit of their own. Source of contract/escher-session-exit-status/3's wording; the `--record` handling in `cli.rs` `read` and `command.rs` must keep mapping these to status 3.

@history_item n=6 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — `snapshot [--part <part>]` answers `{"text"}` or `{"text","part","parts"}`; over mcp a paged snapshot's result is the JSON line, a whole one the screen text (founder's F1). Source of contract/escher-session-commands/snapshot and contract/escher-session-mcp/snapshot-result; `--part` parsing in `cli.rs` `read` and the mcp `run` branch must keep both forms.

@history_item n=7 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** — (Cli item) `escher-session` serves six tasks listed in the usage line's `apps:` and the MCP `start` tool's `enum`, an unknown task a usage ending. Origin of contract/escher-session-tasks/* and unknown-task; a `cli.rs`/`command.rs`/`mcp.rs` refactor must keep the usage line and the enum listing one closed set.

@history_item n=8 marker=2026-11-18-served-error-page
**2026-11-18-served-error-page** — `~ contract/escher-session-tasks/{cells,counter,crud,flight-booker,load,timer}` six → seven and `+ contract/escher-session-tasks/order`: the served-task set is seven with order. The current reading of the task rows this chunk's cli/mcp moves must keep (usage `apps:` and the `start` enum both list seven).

@history_item n=9 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** — (graph answer line, later ratified) `graph` takes no argument, refused `malformed` with nothing run; one line `{"nodes","states"}` on every carrier, over mcp the text block of that line; the line carries names, closed flag words and booleans only. Source of contract/escher-session-commands/graph-answer, graph-arguments and contract/ui-graph-line/content; the test functions `every_graph_node_is_a_snapshot_node_and_no_served_id_holds_the_reserved_form` and `every_stand_tasks_declarations_agree_with_its_code_once_its_handlers_have_run` (`stand_graph.rs`) guard it and must keep asserting the same after extraction. Its sibling entry (the unpaged graph, residual `res-7cc5bi4k`) is a stated limit, not this chunk's.

@history_item n=10 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** — (citation ranges re-read) re-pointed citations into `snapshot.rs` (an open list's row bounds 152-155 → 163-166), `dioxus_document.rs`, `schema.rs` (`EXIT_STATUSES`) and `json.rs` (the written JSON) after added lines moved them. Precedent and trap: splitting `packages/dioxus-native-dom/src/snapshot.rs` and moving code out of the driver's files moves the ranges the design rows' citations name, so the wrap must re-point them (coordinates only, no claim change) or they read claim-false.

@history_item n=11 marker=2026-11-15T05-21-42-wrap
**2026-11-15T05-21-42-wrap** — ratified the graph answer line and the coverage role's answer line, endings and refusals (both were PROVISIONAL, delegate overseer); text unchanged. The graph and coverage contract rows this chunk's cli/mcp/command refactor must preserve stand on the founder's own word.

@history_item n=12 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** — (actions answer line) `actions` takes no argument (refused `malformed`), answers `{"nodes":[{"id","owner","keyed","verbs":[{"verb","cause"}]}]}` on every carrier, nodes in snapshot pre-order; causes decided by the same `validate` and `check_target` every id-taking verb meets; a node resolving to no element reads `stale`. Source of decision/actions-share-the-verbs-checks and contract/actions-line/*, contract/escher-session-commands/actions-answer and actions-arguments (founder's «Новый глагол actions»); the test `each_served_screen_lists_its_actionable_nodes_with_the_tables_verbs` (`stand_actions.rs`) guards it and an extraction must keep its assertions, and a `command.rs` split must keep actions sharing `validate`/`check_target` rather than a copy. Its sibling (unpaged actions) is a stated limit.

@history_item n=13 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** — (answer line, endings and refusals) `coverage <app> <check>...` one shape, any other a usage ending status 2; check paths read bounded at 1,048,576 + 1 bytes and parsed strictly; unreadable, oversize, non-check or other-app files refused `malformed` status 1 with nothing of the path; answer `{"unexercised","checks","held"}`; no exit status added. Source of contract/escher-session-commands/coverage-arguments, coverage-answer, contract/coverage-check-files/* and contract/coverage-line/content; `cli.rs` `read` (complexity 25) parses this shape, so its extraction must keep the shape rule, the bound and the refusal classes exactly.

@history_item n=14 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** — `+ contract/escher-session-commands/layout-answer` and `+ contract/escher-session-commands/layout-arguments`: `layout --id` one line on every carrier; a missing id, an id past the bound or a second argument refused `malformed` status 1, not a usage ending. Argument handling in `cli.rs` `read`/`command.rs` and the layout answer's tests in `json_tests.rs` must keep it so through the split.

@history_item n=15 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — (what the tree, the snapshot and a shot read of an open list) highlighted row in no snapshot or tree; open select `ComboBox`/`expanded`, chosen option `selected`; author's `open` steers nothing; a row of an open list carries its own `bounds` in the snapshot (the builder reads the row's list bounds before its client rect); a row's screenshot crops to it and leaves the list open. Source of contract/select-accessibility/*, contract/snapshot/select-list-row-bounds and contract/screenshot/select-list-row; splitting `packages/dioxus-native-dom/src/snapshot.rs` (913 lines) must keep that bounds order and these readings unchanged.

@history_item n=16 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — (cited source lines follow the chunk's edits) citation ranges into `document.rs`, `render.rs`, the driver's `schema.rs` and others re-pointed after added lines, claims unchanged. Precedent for the coordinates-only re-point a code-moving chunk owes the design rows.

@history_item n=17 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** — (citations re-pointed by the chunk's line moves) 19 lines' citations moved, among them `schema.rs` and `stand_cells_served.rs` in surface's `values · cli-output`. Precedent: moving lines in driver and stand test files shifts cli-surface citations, re-pointed with no claim change.

@history_item n=18 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** — every Dioxus document records a render into a mutation buffer with the document unborrowed, then replays under one borrow, so `MountedData` calls during a render do not panic; such a call sees the previous render's document and `set_focus_to` returns false for a node created in that render. Source of decision/buffered-render-replay and contract/mounteddata-during-render/*; `visit` in `tests/blitz-tests/tests/dioxus_runtime_probe.rs` (complexity 22) probes that runtime, so its extraction must keep what it asserts.

@history_item n=19 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** — (the Cells grid's into-view move carries no animation) the Cells arrow-key handler calls `MountedData::scroll_to(ScrollBehavior::Instant)` synchronously because the async form panicked `RefCell already borrowed`. The trap behind decision/buffered-render-replay; no refactor here should reintroduce a borrow across a render-time call.

@history_item n=20 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — the master became rows (364 records → 137 rows), each verified against the code, the code holding the rest. Every picked row is from this conversion; a behaviour-preserving split must leave each row true against the code, and a row's guard (e.g. the shared reader and `Speaker` asserting no ESC byte) must survive the test moves.
