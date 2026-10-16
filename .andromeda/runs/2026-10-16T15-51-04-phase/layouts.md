# layouts extract

## Relevance
partial — the chunk creates and moves no screen, region or focusable element, but splitting `packages/escher-driver/src/mcp.rs` and `packages/escher-driver/src/schema.rs` reaches the source of the `escher-session` cli surface's output structure (its tool listing, `help` line and usage line), which must read the same after the split.

## Constraints
- The `escher-session mcp` role's tool listing must keep the eleven verbs in verb-table order, `help` last, each tool's description the verb's help followed by `. Returns: ` and its result fields. Splitting `schema.rs` into a verb table and its help texts must not reorder or reword either (per layout-templates §Surface: cli → Primary screens).
- An accepted `screenshot` tool result must stay two content blocks in this order: an `image` block (`image/png`, base64), then a text block holding the line without `png`, with `isError` false; a refused shot stays one text block with `isError` true. The result-writing seam of `mcp.rs` carries this order (per layout-templates §Surface: cli → Primary screens).
- The `mcp` role's stdout must keep carrying protocol lines only, uncoloured and with no ESC byte, in the four handshake revisions (`2025-11-25` answered to any other, `server/discover` answered `-32601`). The framing and bounds seam of `mcp.rs` must not gain a write path (per layout-templates §Surface: cli → Primary screens).
- `escher-session help` must keep answering `describe()`'s one uncoloured JSON line with its seven keys in order (`verbs` · `node_keys` · `key_names` · `busy_classes` · `causes` · `exit_statuses` · `session_errors`), every text a fixed string of the schema's tables. Moving tables between schema files must change neither the order nor the text (per layout-templates §Surface: cli → Primary screens).
- The command line's four endings and the usage line must stay as they are. The usage line is built from the verb table and lists `help` after `stop`, then `; mcp role: mcp`; the exit statuses are read from `EXIT_STATUSES` (per layout-templates §Surface: cli → Primary screens).
- The stand surface must not move: the TaskShell mount (`main#main > #task-shell`, header over `#task-body`), the pinned 800 × 600 viewport, the author ids and the CRUD Tab sequence. A held instance must read as a fresh boot. The chunk touches no `examples/seven_guis/src/` markup (per layout-templates §Surface: desktop-native → Primary screens).

## Patterns to follow
- The plan's additive-nothing phrasing applies to this refactor: "adding no element, class, style, wrapper or order". A split that moves code adds no output and no order (per layout-templates §Surface: desktop-native → Primary screens).
- The plan's cli entries keep file:line evidence pointers into `packages/escher-driver/src/mcp.rs` (`:51-117`, `:325-345`, `:347-356`, `:358-407`) and `packages/escher-driver/src/cli.rs`. Once the split lands, those pointers go stale and are re-pointed by the wrap's reconcile, never hand-edited in this chunk (per layout-templates §Surface: cli → Primary screens).
- The cli entry points to test-plan §3 → Session lifecycle for the session contract rather than restating it. The split should do the same: leave the contract where it lives (per layout-templates §Surface: cli → Primary screens).

## Anti-patterns to avoid
- Do not reorder the verb table, the node keys or the tool listing while separating `schema.rs`'s help texts from its table. In the plan, order is a surface property: `help` comes last, and `screenshot` comes seventh, after `scroll` (per layout-templates §Surface: cli → Primary screens).
- Do not add any stdout write in the `mcp` role or on the command line, even a temporary debug print during the move. Stdout is the answer or protocol line alone (per layout-templates §Surface: cli → Primary screens).

## Contract bindings
- layouts §Surface: cli → `escher-session` ↔ test-plan §3 → Session lifecycle. The plan defers the session contract to that section, so the stand session and MCP checks are what witness that the surface reads unchanged.
- layouts' `mcp.rs` / `cli.rs` line citations ↔ the wrap's reconcile citation re-point. The split moves the cited lines, so the layout plan is one of the documents the reconcile re-points.
- layouts §Surface: desktop-native → CRUD Tab sequence ↔ a11y SC 2.4.3 (`stand_keyboard`'s tables). Nothing in scope moves it; it only has to stay green.

## Acceptance criteria contributions
- (layouts) The `escher-session mcp` `tools/list` answer is byte-identical before and after the split: eleven tools in verb-table order, `help` last, each description the help + `. Returns: ` + result fields. Whether an existing check pins the whole listing byte for byte is research's question (per layout-templates §Surface: cli → Primary screens).
- (layouts) `escher-session help`'s stdout line is byte-identical before and after the split, with its seven keys in their stated order (per layout-templates §Surface: cli → Primary screens).
- (layouts) The command line's usage line (stderr, exit 2) and an accepted `screenshot`'s MCP content-block order (`image` then text) are unchanged before and after the split (per layout-templates §Surface: cli → Primary screens).
- (layouts) `git diff` over `examples/seven_guis/src/tasks/` and `examples/seven_guis/src/app.rs` is empty for the chunk, so the TaskShell mount, ids and Tab sequences cannot have moved (per layout-templates §Surface: desktop-native → Primary screens).
