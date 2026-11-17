@history_item n=1 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** — set the driver command line's output form: one line of JSON on stdout per command (answer, refusal or a session error's object, keyed by the schema's own words), nothing else there, no colour, no ESC byte, no terminal test, exit class 0/1/2/3. It is the origin of decision/escher-session-agent-read-surface, decision/escher-session-no-colour, decision/driver-output-for-agents and contract/escher-session-stdout/command-line-line; the split of `json.rs` must leave every byte of these written lines as they are.

@history_item n=2 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — added the `mcp` role to the cli surface: newline-delimited JSON-RPC 2.0 lines only on stdout, a tool result's text the screen text for `snapshot` and the JSON line otherwise, no ESC byte asserted on every reply by the shared `Speaker`, an `Err` ending's message on stderr with status 3. Origin of contract/escher-session-mcp/stdout, other-results, err-ending and contract/escher-session-stdout/mcp-line; `mcp.rs` and `mcp/framing.rs` call into `json.rs`, so their paths must keep resolving after the split.

@history_item n=3 marker=2026-10-12-self-description
**2026-10-12-self-description** — exit statuses read from the schema's `EXIT_STATUSES` table that `help` serves; `help` answers one uncoloured line reaching no session; a session error's object carries its fixed `remedy`; over `mcp` the help result is the self-description's line. Origin of decision/exit-statuses-from-schema-table, contract/escher-session-commands/help, contract/escher-session-mcp/help-result and contract/escher-session-session-error/remedy; the `impl SessionError` writer and the `help` self-description move or stay without a byte changed.

@history_item n=4 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — the shot's answer line `{"width","height","png"}`, the MCP image block then a text block without `png`, and the `fonts-unavailable` refusal with nothing painted. Origin of contract/escher-session-commands/screenshot, contract/escher-session-mcp/screenshot-result, contract/driver-screenshot/fonts-unavailable and decision/screenshot-never-skips-text; the `shot` / `shot_without_image` writers (the latter called from `mcp.rs`) are among the forms the scope leaves unplaced, and must keep their callers' paths.

@history_item n=5 marker=2026-10-19-session-recording
**2026-10-19-session-recording** — named the recording's `record-dir-not-private` and `recording-failed` among the exit-3 session errors, no new exit. Origin of contract/escher-session-exit-status/3; `recording.rs` is a caller of `json.rs` whose session-error text must stay unchanged.

@history_item n=6 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — the paged `snapshot` answer `{"text"}` / `{"text","part","parts"}` and over `mcp` a paged result as that JSON line. Origin of contract/escher-session-commands/snapshot and contract/escher-session-mcp/snapshot-result; the snapshot family the split creates writes these forms.

@history_item n=7 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (what the tree, the snapshot and a shot read of an open list; and the citations re-pointed into the driver's `schema.rs`) — a row of an open list carries its own `bounds` in the snapshot and a shot of a row crops to it. Origin of contract/snapshot/select-list-row-bounds, a snapshot-node form the split moves; the same chunk's coordinates-only entry is a precedent for re-pointing cli-surface citations after a source move.

@history_item n=8 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the cli surface's graph answer line; the unpaged graph; citation ranges re-read) — added `graph` (no argument, one line `{"nodes","states"}` on every carrier, names, closed flag words and booleans only) and its unpaged limit; and re-pointed the `escher-session` item's written-JSON citation `json.rs:317-373` → `json.rs:400-462`. Origin of contract/escher-session-commands/graph-answer, graph-arguments and contract/ui-graph-line/content; the re-point is the direct precedent: the master's citations into `json.rs` must follow the moved code to its new family module, claims unchanged.

@history_item n=9 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the `actions` answer line; the unpaged `actions` answer) — added `actions` (no argument, one line of nodes with verbs and causes, ids/verb words/cause words/booleans only, `stale` for an unresolved node), decided by the verbs' own `validate` and `check_target`, unpaged. Origin of contract/escher-session-commands/actions-answer, actions-arguments, contract/actions-line/content, nodes, stale and decision/actions-share-the-verbs-checks; `action_node` is an unplaced form of the split.

@history_item n=10 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (the coverage role's answer line, endings and refusals) — `coverage <app> <check>...` answers one line `{"unexercised","checks","held"}`, ids/owner ids/event names/two counts only, a bad check file refused `malformed` holding nothing of the path. Origin of contract/escher-session-commands/coverage-answer, contract/coverage-line/content and contract/coverage-check-files/refusal; `impl Coverage` in `json.rs` is an unplaced form the split must move unchanged.

@history_item n=11 marker=2026-11-15T05-21-42-wrap
**2026-11-15T05-21-42-wrap** (ratified — the graph answer line; ratified — the coverage role's answer line) — the PROVISIONAL graph and coverage entries now ratified by the founder's own word, text unchanged. Both answer forms are settled contracts a no-behaviour-change split may not touch.

@history_item n=12 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** — `+ contract/escher-session-commands/layout-answer`, `+ contract/escher-session-commands/layout-arguments`: `layout --id` answers one line `{id, bounds, laid_out, placed, shifts, width, height}` on every carrier, a missing or bad id refused `malformed`. The `layout` answer writer (`placer_word` … `layout_answer`) is one of the four named families of this split, added last epoch and a main source of `json.rs`'s growth.

@history_item n=13 marker=2026-11-13-upstream-sync-determinism-and-exploration
**2026-11-13-upstream-sync-determinism-and-exploration** (Typography: 2 rows) — `+ decision/unavailable-families-use-default-font`, ratified as upstream's behaviour. A picked typography row; it bears on this chunk only as an upstream-owned engine fact the split must not touch (no upstream file edited, `document.rs` left whole).

@history_item n=14 marker=2026-10-27-readme-and-records
**2026-10-27-readme-and-records** (the stand's font registration leaves for architecture's Headless stand) — the stand's bundled DejaVu Sans registration moved to architecture §Standard Contracts → Headless stand. It explains why the stand passes the `fonts-unavailable` check in the picked typography rows; the stand checks gating this chunk run on that font.

@history_item n=15 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — the master's text became rows (137 rows, each verified against the code); the picked surface and typography rows come from this conversion, divergences settled «as the code shows», so the rows describe the written forms as they are at HEAD and a pure move keeps them true.
