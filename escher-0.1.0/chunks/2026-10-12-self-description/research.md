# Codebase Research — 2026-10-12-self-description

## Scope
- **Depth:** deep · **Reads:** 19 · **Globs/Greps:** 14 (plus 1 code-graph query, 2 runs of the built binary)
- **Harness rules consulted:** `.claude/rules/testing.md` — 6 additions applied (the table-pin search before calling a check unaffected; hand mutation controls saved and restored by sha256; `--no-fail-fast` counts; no deleting key) · `.claude/rules/a11y.md` — read for the node-key help's words · none — no live leg in this chunk (no cold-agent or windowed run is planned)
- **Platform issues consulted:** the MCP specification, fetched 2026-10-12: `modelcontextprotocol.io/specification/2025-06-18/server/tools` — a Tool is `name` · `title` · `description` · `inputSchema` · `outputSchema` (optional) · `annotations`; "If an output schema is provided: Servers **MUST** provide structured results that conform to this schema", and "a tool that returns structured content SHOULD also return the serialized JSON in a TextContent block" · `…/2025-11-25/changelog` — no change to `initialize`, `tools/list` or `tools/call` that this server's behaviour misses; minor change 5: "input validation errors should be returned as Tool Execution Errors rather than Protocol Errors" (this server already returns a refusal as an `isError` result) · `…/2026-07-28/changelog` — "Make MCP stateless: remove the `initialize`/`notifications/initialized` handshake", "Add `server/discover`: servers MUST implement this RPC … Clients MAY call it … as a backward-compatibility probe on STDIO", "Remove `ping`", "All results now carry a required `resultType` field", list results require `ttlMs` and `cacheScope`.
- **External inputs:** `inputs#I1` — who decides this entry's forks (the operator's arguments to the take-up).

## Files inspected
- `packages/escher-driver/src/schema.rs` (1-400) — the one table: `MAX_*`, `KEY_NAMES` (12), `BUSY_CLASSES` (3), `NODE_FIELDS` (12 names, no help), `ArgKind`/`ArgSpec`/`FieldKind`/`FieldSpec`/`Level`/`VerbSpec`, nine `VerbSpec`s, `VERBS` (schema.rs:388-390), `verb()`. Every field of every verb has `help`; `settled`, `served`, `idle_expiry_s` have help (schema.rs:168-173, :337-350) — served nowhere.
- `packages/escher-driver/src/refusal.rs` (full) — `Cause` (8), `CAUSES`, `name`/`meaning`/`remedy`, `Fault` (5), `Refusal`; unit tests pin all three texts per cause. `Cause::Covered`'s doc comment (refusal.rs:20-22) still says "A hit reaches content scrolled out of a scrolling box", while its `meaning()` (refusal.rs:71-75) says `contain: paint` — the doc comment was left behind at 2026-10-10-scrolling-box-bounds-and-hit.
- `packages/escher-driver/src/error.rs` (full) — `SessionError`, 13 variants, `kind()` and a fixed `Display` message; no remedy; `StateDirTooLong`'s message states no bound (error.rs:77-79).
- `packages/escher-driver/src/cli.rs` (1-340, 739) — the exit statuses are four private constants (cli.rs:21-27) and their meanings live only in `command_line`'s doc comment (cli.rs:60-71); `usage()` is built from `VERBS` and the apps (cli.rs:281-304); the reader's order: `serve`, `mcp`, verb lookup, `validate`/`validate_session`, then `--session` exactly once (cli.rs:118-163) — a session-level verb with no `--session` ends as usage.
- `packages/escher-driver/src/mcp.rs` (1-560) — `PROTOCOL_VERSIONS` (mcp.rs:34-37); `initialized()` answers `protocolVersion` · `capabilities` (`tools`) · `serverInfo` and no `instructions` (mcp.rs:321-339); `tool()` writes `name` · `description` (the verb's help) · `inputSchema` (each argument's help with its bound) and no result field (mcp.rs:343-390); `handle` answers `initialize` · `ping` · `tools/list` · `tools/call`, every other method `-32601` (mcp.rs:530-550); a tool result is one text block and `isError`, no `structuredContent` (mcp.rs:482-490).
- `packages/escher-driver/src/json.rs` (grep) — `Refusal::to_json` writes `cause` · `meaning` · `remedy` · `fault` (json.rs:255-262), so every served refusal already names its remedy; `SessionError::to_json` writes `{"error":{"kind","message"}}` (json.rs:272-277); a node is written in `NODE_FIELDS` order (json.rs:112, :162).
- `packages/escher-driver/src/client.rs` (225-250) · `host.rs` (118-135) — `StateDirTooLong` is `std`'s `InvalidInput` on connect or bind of `state_dir.join("session.sock")` (wire.rs:12); the path is used as given, so a relative path counts its own bytes.
- `packages/dioxus-native-dom/src/snapshot_diff.rs` (1-120) — `DiffNode { id, parent, role, name, state, bounds }`; `changed` when role, name, state, bounds or parent differ (snapshot_diff.rs:73-79), written as it reads after; nothing records which differed.
- `tests/blitz-tests/tests/stand_mcp_tools.rs` (1-100) — `serve_mcp` in process with a `boot` closure over the lean tasks; its own `Recorder` reads spans.
- `examples/seven_guis/tests/mcp_server.rs` (full) — pins `VERBS.len() == 9` and the listing equal to the table's names (mcp_server.rs:120-125), `server/discover` → `-32601` (mcp_server.rs:182-183), `initialize` answering the revision asked (mcp_server.rs:100-109).
- `tests/blitz-tests/tests/stand_act_filled.rs` (1-60, 199-263) — the password fixture `password_fixture()` (`fx-secret`, `aria-label: "Secret"`) held by `Session::start("filled", …)`; its written form holds the mask, never the typed text.
- `tests/blitz-tests/tests/session_common/mod.rs` (grep) — `hold()` boots the lean tasks only; a fixture is held by `Session::start` directly.
- `escher-0.1.0/chunks/2026-10-10-mcp-surface/evidence/live-trial.md` (16, 24) — `protocol_version 2025-11-25 · discover sent`; the client fell back to `initialize` after `-32601`.
- `README.md` (48) — "The help the driver serves about itself and its screenshot are still to come".

## Graph impact (from the code-graph query; plane rust, regenerated, 129 rows)
- **VERBS** — read in escher-driver by `cli.rs` (3), `command.rs` (2, its test pins `table.len() == 9` and six instance rows, command.rs:789-797), `execute.rs` (2, test only: eight argument names, execute.rs:446-451), `mcp.rs` (5), `schema.rs` (9); outside it by `examples/seven_guis/tests/cli_commands.rs:100` (`VERBS.len() == 9`), `examples/seven_guis/tests/mcp_server.rs:120-121` (the listing, `== 9`), `tests/blitz-tests/tests/stand_act_scroll.rs:505-509` (`VERBS.len() == 9`, the sixth verb named by the off-screen remedy), `tests/blitz-tests/tests/stand_act_spans.rs:391-399` (instance rows only, `== 6`). A tenth verb moves four `== 9` pins; a Session-level one leaves `stand_act_spans` and `execute.rs`'s test as they are (no argument added).
- **CAUSES** · **remedy** · **meaning** — json.rs (the written form), refusal.rs, `stand_act_refused.rs`, `stand_act_spans.rs`, `stand_act_scroll.rs:507`; no check reads `remedy` from a served (JSON) refusal.
- **NODE_FIELDS** — json.rs (2), lib.rs, schema.rs (2); no reader outside the crate.
- **usage** · **tool_list** · **tool** · **initialized** — private to cli.rs and mcp.rs; callers are their own tests.
- **kind** (SessionError) — written by json.rs; pinned whole once in cli.rs:739 and by prefix in `examples/seven_guis/tests/flows/counter.sh:79` (`'{"error":{"kind":"no-session",'*`), so a field added after `kind` leaves the flow's pattern matching.

## Patterns detected
- **The served form is derived from the table** (cli.rs:283-304, mcp.rs:343-390): the usage line and each MCP tool are written by walking `VERBS`; no verb's text is a literal in either file.
- **Stated-table tests with counts** (refusal.rs:221-290, schema.rs tests, cli.rs:747-756): each table is restated in its test and its row count asserted; a failing message names a row index, never a value.
- **A server-answered verb** (mcp.rs:58-63, the `serve_mcp` doc): `start` · `status` · `stop` are tools the server answers itself, leaving a `tool_call` span and no `command` child.
- **A fixture held as a session** (stand_act_filled.rs:203-210): `Session::start(label, || boot of a VirtualDom)` with the stand's options; the same closure can be handed to `serve_mcp` as its `boot`.

## Conventions to follow
- **No text outside the table**: a served text is a field of a `'static` table in `schema.rs` / `refusal.rs` / `error.rs`, and `cli.rs` / `mcp.rs` only walk it (arch extract; schema.rs:1-2).
- **Hand-written JSON**: every served document is written by `json`'s `Object` / `string` (mcp.rs:21, json.rs); `serde_json` reads only.
- **Doc comments**: `#![deny(missing_docs)]`; a new public item carries `///` and the re-export count (41) moves explicitly.

## Measured at P3
- **The Book step's eight changed nodes only moved.** `target/debug/escher-session` (built 2026-10-11 12:00, after the last source commit, 10:10) on a fresh flight booker: `click --id flight-book` answered `settled` true, 1 `added` (a `Paragraph`), 0 `removed`, 8 `changed`; a snapshot before and after, compared line by line with the `@x,y wxh` bounds taken out, reads all eight equal but for their bounds: each moved up 27 px, the card's container grew 54 px. Run with a state directory under `target/`, removed after; ids and names were not printed (`scratchpad` scripts `book_diff.py`, `moved.py`).
- **The `sun_path` bound**: `std`'s socket address takes a path shorter than `sun_path` (108 bytes on Linux, 104 on macOS, NUL included), and the session's path is `{dir}/session.sock` — 13 bytes after the directory — so a directory of at most 94 bytes on Linux and 90 on macOS fits. Read from `std`'s documented `SocketAddr` limit and wire.rs:12; not run at the edge.

## New files to create
- `packages/escher-driver/src/describe.rs` — the self-description document written from the table: the verbs (each with level, help, arguments with kind, bound and help, result fields with kind, presence and help), the node keys with their help, the key names, the busy classes, the causes with meaning and remedy, the exit statuses with their meaning, and the session errors with kind, message and remedy
- `tests/blitz-tests/tests/stand_mcp_masked.rs` — a password typed through an MCP `type` tool call on the password fixture beside the stand, in process, both layout modes

## Files to modify
- `packages/escher-driver/src/schema.rs` — a help per node key; the exit statuses as a table; under fork F1-A a `help` verb in `VERBS`
- `packages/escher-driver/src/refusal.rs` — `Cause::Covered`'s doc comment brought to its meaning
- `packages/escher-driver/src/error.rs` — a remedy per session error, and the state directory's bound in `StateDirTooLong`'s text
- `packages/escher-driver/src/json.rs` — the session error's written form gains its remedy; a changed node names what differs, under fork F3-A
- `packages/escher-driver/src/cli.rs` — the help served on the command line; the exit statuses read from the table
- `packages/escher-driver/src/mcp.rs` — the help served over MCP; tool descriptions carry result fields; `2025-11-25` spoken, under fork F2-A
- `packages/escher-driver/src/command.rs` — `validate_session` takes `help`, under fork F1-A; its table test's counts
- `packages/escher-driver/src/lib.rs` — re-exports and crate docs
- `packages/dioxus-native-dom/src/snapshot_diff.rs` — a changed node records what differs, under fork F3-A
- `packages/dioxus-native-dom/src/lib.rs` — the re-export of the type that names what differs, under fork F3-A
- `tests/blitz-tests/tests/stand_diff.rs` — the differing readings on the stand, under fork F3-A
- `tests/blitz-tests/tests/stand_act_scroll.rs` — its `VERBS.len() == 9` pin, under fork F1-A
- `tests/blitz-tests/tests/stand_mcp_tools.rs` — every cause a served refusal names carries its remedy, on the stand and the overlay fixture
- `examples/seven_guis/tests/cli_commands.rs` — the help served by the binary; its `VERBS.len() == 9` pin
- `examples/seven_guis/tests/mcp_server.rs` — the help served by the `mcp` role equals the command line's; the listing's count; the revision answered
- `examples/seven_guis/tests/common/mod.rs` — a reader for the served document, if the checks share one
- `README.md` — the front page: self-description built

## Sweeps
- `VERBS` over the workspace's Rust (graph refs, 129 rows; re-derived by `grep -rn 'VERBS'` over `packages/`, `examples/`, `tests/`): 4 `== 9` pins (command.rs:790, cli_commands.rs:100, mcp_server.rs:121, stand_act_scroll.rs:505) · 2 no-change (`stand_act_spans.rs:391` and `execute.rs:446` read instance rows and argument names only).
- `"error":{"kind"` over `packages/`, `examples/`, `tests/` (`grep -rn`): 2 hits · cli.rs:739 changed (pins the whole line) · counter.sh:79 no-change (a prefix pattern).
- `usage: <command>` over `packages/`, `examples/`, `tests/`: 1 file (cli.rs) — no check outside the crate pins the usage line.
- `2025-06-18` over `packages/`, `examples/`, `tests/`, `scripts/`: mcp.rs, mcp_server.rs, mcp_log.rs, stand_mcp_tools.rs, scripts/cold_agent_stub.py (the stub speaks its own revision; no change) — a revision added keeps every request that asks `2025-06-18` answered so.

## Open questions
- F1 — is the help a tenth verb of the table (the operator's 2026-10-07 direction, CARRY 1), or served outside `VERBS`? The tenth verb changes v010-13's "exactly the nine verbs" → blocks: plan-decision (the founder's, `inputs#I1`)
- F2 — does the server take up `2025-11-25`, and does it answer `server/discover`? → blocks: plan-decision (the operator's for the revision; the founder's for a new method)
- F3 — does a changed node name what differs, or does the help say it only? → blocks: plan-decision (the operator's)
