@history_item n=1 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** — set the escher-session output form: one JSON line per command on stdout (answer, refusal or session error's object in the schema's words), a fixed stderr line for usage and session errors, no colour, no ESC byte, no terminal test, exit classes 0 · 1 · 2 · 3. This is the origin of decision/escher-session-agent-read-surface, decision/escher-session-no-colour, decision/driver-output-for-agents and the exit-status/stdout/stderr contracts. The split of `packages/escher-driver/src/command.rs` must keep every one of them byte-for-byte; the guard is the seven_guis checks' shared reader and the shared Speaker.

@history_item n=2 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — added the `mcp` role to the escher-session item: stdout carries newline-delimited JSON-RPC 2.0 lines only, one reply per request that is not a notification, no ESC byte, an Err ending's fixed message on stderr with status 3. These are contract/escher-session-mcp/* and contract/escher-session-stdout/mcp-line. The command module the chunk splits feeds the mcp tool results, so the split must not change them.

@history_item n=3 marker=2026-10-12-self-description
**2026-10-12-self-description** — exit statuses are read from the schema's `EXIT_STATUSES` table, `help` answers the self-description line without reaching a session, a session error's object carries its `remedy`, and the mcp `help` result is that line. These are decision/exit-statuses-from-schema-table, contract/escher-session-commands/help, contract/escher-session-session-error/remedy and contract/escher-session-mcp/help-result. Command dispatch moved by the split must still route to them unchanged.

@history_item n=4 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — `screenshot [--id]` answers `{"width","height","png"}`, the mcp role answers two blocks (image, then text without png), and a shot with no sans-serif family is refused `fonts-unavailable` with nothing painted. These are decision/screenshot-never-skips-text, contract/driver-screenshot*/fonts-unavailable, contract/escher-session-commands/screenshot and contract/escher-session-mcp/screenshot-result. They are command-module behaviour the split must preserve, and their checks belong in the plan's list of proofs.

@history_item n=5 marker=2026-10-19-session-recording
**2026-10-19-session-recording** — put the recording's `record-dir-not-private` and `recording-failed` errors in the exit-3 class with no exit of their own (contract/escher-session-exit-status/3). The `--record` path lives in the command layer, so the split must keep these endings. It is also a precedent: the chunk may add no new cause or exit.

@history_item n=6 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — set the paged `snapshot [--part]` answer line `{"text"}` / `{"text","part","parts"}` and its mcp JSON-line result (contract/escher-session-commands/snapshot, contract/escher-session-mcp/snapshot-result). The part-argument parsing is command code that moves in the split.

@history_item n=7 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** — has three sub-entries: the graph answer line (PROVISIONAL, later ratified), the unpaged-graph pending limit owned by `res-7cc5bi4k`, and citation ranges re-read into `schema.rs`/`json.rs`/`snapshot.rs` after added lines. These are the origin of contract/escher-session-commands/graph-* and contract/ui-graph-line/content. The re-read sub-entry is the precedent for this chunk: when lines move, a cited range is re-pointed and its claim stays as written.

@history_item n=8 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** — added `actions`: no argument, and the answer line lists nodes with their verbs and causes. Each node's verbs are decided by the same `validate` + `check_target` every id-taking verb meets, and an unresolved node reads `stale`. The entry also states the unpaged-actions limit. These are decision/actions-share-the-verbs-checks and contract/actions-line/*, contract/escher-session-commands/actions-*. Trap for the split: `actions` and the verbs must keep sharing one validation path, so a split that duplicates or separates `validate`/`check_target` breaks the "listed cause is the returned cause" rule.

@history_item n=9 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** — added the `coverage <app> <check>...` role: its answer line, its usage-only shape (no `--session`, no `--record`), the bounded strict read at 1,048,576 + 1 bytes, `malformed` refusals that hold nothing of the path, and three stated limits. These are contract/escher-session-commands/coverage-*, contract/coverage-check-files/* and contract/coverage-line/content. The argv parsing and check-file reader likely sit in or near `command.rs`, and their security refusals must survive the split unchanged.

@history_item n=10 marker=2026-11-15T05-21-42-wrap
**2026-11-15T05-21-42-wrap** — ratified the graph answer line and the coverage role's line, endings and refusals, which were PROVISIONAL (delegate). Their rows are now ratified contracts, so the restructure has no room to reinterpret them.

@history_item n=11 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** — added contract/escher-session-commands/layout-answer and layout-arguments: a missing id is refused `malformed` with status 1, not a usage ending. This is a later command-module addition the split must carry, with its distinct refusal class.

@history_item n=12 marker=2026-11-18-served-error-page
**2026-11-18-served-error-page** — has two sub-entries: the six escher-session-tasks rows changed to "seven tasks", and contract/escher-session-tasks/order was added. Together they make the served-task set seven, listed in the usage line's `apps:` and the MCP start tool's enum. The task table and usage line live in the command layer and must read the same after the split.

@history_item n=13 marker=2026-11-20-driver-debts
**2026-11-20-driver-debts** — changed contract/escher-session-commands/graph-answer, graph-arguments and contract/ui-graph-line/content for graph read in parts: `--part` 1..1000, and the `part`/`parts` numbers. This chunk added the 72 lines that pushed `command.rs` from 776 to 848 code lines (scope CARRY cry-p3m7lb6q). The paged-graph code is a natural seam for the split and must keep these three rows exact.

@history_item n=14 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** — the driver's `advance` moves app time through the caller's step, and `click`/`type`/`press` leave the animation clock alone. Superseded in part by 2026-10-18-one-virtual-clock. This is the origin of the driver-verbs-and-virtual-clock contracts that the command module's verb dispatch must keep.

@history_item n=15 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** — the harness drives animation from the documents' `VirtualClock`. The driver's `advance` moves that clock by exactly its ms and the app's time together, and click/type/press leave it. These are decision/driver-advance-moves-one-clock, decision/animation-driven-by-virtual-clock and contract/driver-verbs-and-virtual-clock/*. They are verb behaviour inside the module being split.

@history_item n=16 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (the PROVISIONAL batch ratified, Epoch 2 close) — ratified the one-virtual-clock `advance` clause. The clock rows the split must preserve are founder-ratified.

@history_item n=17 marker=2026-10-07-settle-detection
**2026-10-07-settle-detection** — settle answers every keeps-animating member the same way: nothing holds it open, it returns Settled with `animating` from the document flag, and it never advances the clock (decision/settle-does-not-wait-on-animation). This is driver step behaviour the restructure must leave untouched.

@history_item n=18 marker=2026-10-20-load-wait
**2026-10-20-load-wait** — while an acting step waits on a load for at most `LOAD_WAIT`, the `VirtualClock` stands still (decision/load-wait-moves-no-clock). This is driver step code that may sit in the split module.

@history_item n=19 marker=2026-10-07-refusal-detection
**2026-10-07-refusal-detection** — an into-view scroll never animates a nested box, and the driver's `scroll` is instant in every box and in the viewport inside one settled step. Founder-ratified 2026-10-07. This is the origin of decision/driver-scroll-instant, decision/into-view-scroll-instant-in-nested-boxes and decision/into-view-scroll-nested-boxes-instant.

@history_item n=20 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — has several sub-entries that bear on this chunk:
- How a single select's list opens, moves, chooses and closes: contract/select-list-events/*.
- The list is painted and opens on click, Enter or a Space with no typed text standing (decision/single-select-open-gestures). This supersedes the 2026-10-31-engine-debts "Enter and Space open nothing".
- The list's wheel and key, with no motion (decision/select-list-wheel-and-key-scroll, decision/select-list-opens-without-motion).
- Into view on a row of an open list (the except clause of decision/into-view-scroll-instant-in-nested-boxes).
- What the tree, snapshot and shot read of an open list (contract/select-accessibility/*).
- A `multiple` select's `size_rows` height hint.

These are escher's additions inside `handle_key_or_input_event` (keyboard.rs), `build_accessibility_node` (accessibility.rs) and `compute_presentational_hints`. The plan must find them by diff against upstream at `547d7514b` and keep each one's checks (`select_popup`, `select_popup_painted`, the stand_act_popup driver tests).

@history_item n=21 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (no select popup is painted, and who owns it) — an earlier state in which Enter and Space opened nothing on a single select. 2026-10-31-select-popup replaced it. It is history of the keyboard.rs open-key additions this chunk moves, and it shows that the open gestures are escher's code, not upstream's.

@history_item n=22 marker=2026-11-20-select-list-and-selection-debts
**2026-11-20-select-list-and-selection-debts** — changed three rows:
- contract/select-accessibility/highlighted-row: the snapshot's `highlighted` bool.
- contract/select-accessibility/open-select: focus stays on the select, the highlighted option is the active descendant, and a page's aria-activedescendant is not read.
- contract/select-list-events/highlight-move: a move to another row marks its select changed.

These are among the most recent escher additions to `build_accessibility_node` and the key path. They fed the score growth (47→50 and 56→58 per scope) and are exactly what must move into escher's own functions while these rows stay unchanged.

@history_item n=23 marker=2026-11-21-single-select-sized-as-a-list-box
**2026-11-21-single-select-sized-as-a-list-box** — changed decision/single-select-open-gestures and decision/selects-that-open-no-list: a single select sized above 1 opens no list. Changed contract/select-list-events/close-on-select-change: a size write that makes the select a list box closes the list. Changed decision/select-chosen-row-chromium-pair for list boxes, and retired decision/single-select-box-shows-chosen-label. These are the latest escher additions to the keyboard open gate and probably to the size/list-box hint path in presentational_hints.rs. They must keep holding after the move into helpers.

@history_item n=24 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (a focused select past its dropped block) — a focused select whose anonymous block was dropped keeps focus and takes a key with no panic. This is escher's fix on the key path. It is history for keyboard.rs's escher additions, and 2026-11-21-engine-debts retired the row.

@history_item n=25 marker=2026-11-21-engine-debts
**2026-11-21-engine-debts** — removed decision/boxless-focused-select-keeps-focus (no heir in this master). The post-layout focus fixup gives up unshown focus, and that fact lives in a11y-plan. A key-path behaviour this chunk's keyboard.rs move must not resurrect or alter; the proof for it is in a11y-plan's checks, not this master's rows.

@history_item n=26 marker=2026-11-13-upstream-sync-determinism-and-exploration
**2026-11-13-upstream-sync-determinism-and-exploration** — has two sub-entries:
- Typography: added decision/font-element-face-and-size, plus unavailable-families-use-default-font.
- Spacing: added decision/table-cell-spacing-presentational-hints.

Both hints are "taken from upstream, ratified as upstream's behaviour". Trap for this chunk: the font, cellspacing, cellpadding and valign hints in `compute_presentational_hints` are upstream's text and must stay in the upstream body. Only escher's additions move out, and the upstream score is 167.

@history_item n=27 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — the masters became rows: decision, contract and absence rows, each verified against the code. All the picked rows (decision/body-right-bottom-margin-ignored and the guarded font_element_attributes among them) date from this conversion. A row records behaviour, not code location, so moving code into escher's own functions changes no row. The scope says no master row lands in this chunk's plan.

@history_item n=28 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (citations follow the script runtime's split) — when a module was split, its cited paths were re-pointed to the new files with the claim text unchanged. This is the direct precedent for splitting `command.rs` and moving the three bodies: re-point citations, change no claim.

@history_item n=29 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (citations re-pointed by the chunk's line moves) — `schema.rs` and other driver citations were re-pointed only by their line numbers. A precedent of the same kind, which applies when the split moves cited driver lines.
