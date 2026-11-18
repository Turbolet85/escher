@history_item n=1 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** — set the cli surface's output form: one JSON line per command on stdout (answer, refusal or session error object in the schema's words), usage error one fixed stderr line, no colour, no ESC byte, no terminal test, exit 0 accepted · 1 refused · 2 usage · 3 session error. It is the origin of decision/escher-session-agent-read-surface, decision/escher-session-no-colour, decision/driver-output-for-agents and the exit-status and stdout contracts. The seventh app's answers, `caused` included, must keep that form, with no new exit.

@history_item n=2 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — added the `mcp` role: newline-delimited JSON-RPC 2.0 lines only on stdout, a tool result's text is the JSON line (the screen text for snapshot), no ESC byte checked on every reply by the shared `Speaker`, an Err ending on stderr with status 3. It is the source of the contract/escher-session-mcp/* rows. The served app has to be reachable over the mcp role, its `start` enum growing, with replies in this form.

@history_item n=3 marker=2026-10-12-self-description
**2026-10-12-self-description** — exit statuses are now read from the schema's `EXIT_STATUSES` table, whose meanings `help` serves (decision/exit-statuses-from-schema-table), and a session error carries its fixed `remedy`. The chunk adds no exit, so the table and the `help` text stay as they are.

@history_item n=4 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — a screenshot whose font context resolves no `sans-serif` family is refused `fonts-unavailable`, and the stand's bundled font passes. This is the why of decision/no-generic-family-lays-out-sans. The new app's boot must register the bundled font as the other boots do, or a shot of it is refused.

@history_item n=5 marker=2026-10-06-headless-stand
**2026-10-06-headless-stand** — the headless stand registers bundled DejaVu Sans for every generic, with system fonts off, through `HarnessOptions.font_ctx`. It is the precedent for a new `boot_*` taking the same font context. The claim later moved to architecture's Headless stand (see 2026-10-27-readme-and-records).

@history_item n=6 marker=2026-10-13-upstream-sync-polish-and-ship
**2026-10-13-upstream-sync-polish-and-ship** — every document gets the default user-agent sheet once, through `DocumentConfig::default()` (decision/default-ua-sheet-via-default-config). The new app's document must take the sheet through that default config, not by a call of its own.

@history_item n=7 marker=2026-10-19-session-recording
**2026-10-19-session-recording** — added the recording's two session errors to the exit-3 class with no new exit. It is the precedent for widening a class without adding a status. Here the scope adds no session error at all, so the exit-3 class line stays unmoved.

@history_item n=8 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — set the paged `snapshot` line `{"text"}` / `{"text","part","parts"}` and the mcp paged result (contract/escher-session-commands/snapshot, contract/escher-session-mcp/snapshot-result). The new app's snapshot must stay under `SNAPSHOT_TEXT_BUDGET` and answer unpaged.

@history_item n=9 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** — the direct precedent: the stand grew from the lean four to six served tasks (Cells and the load page) in the lean four's `TaskShell`, the usage line's `apps:` and the MCP `start` enum listing them, an unknown task a usage ending. The load page's styling, keyed elements and `LoadNet` delay were first recorded here. This is the origin of the contract/escher-session-tasks/* rows that the seventh app extends by one, in the same three places.

@history_item n=10 marker=2026-10-27-readme-and-records
**2026-10-27-readme-and-records** (three entries: radii/shadows/padding, colours, font registration; PROVISIONAL, later ratified) — the stand's radii, card shadows, padding, accent, success colours and select border moved to layout-templates §Surface: desktop-native → Primary screens (`values · desktop-native-screens`), and its font registration moved to architecture's Headless stand. Design-system keeps pointer rows only. Trap: the new app's colours, spacing and fonts belong in layout-templates and architecture, not in a design-system row.

@history_item n=11 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (F2 ratified; stand colours, type and select padding leave; the load page's group leaves) — this ratified the stand-row move by the founder's word «Все десять, общие разделить», removed the stand-only rows and the `desktop-native-stand-load-page` surface group, and folded served-screen facts into layout-templates' served-screens bullet. Kept: surface's cli item naming the six served tasks and the driver's load-wait items. A served app's screen facts go to layout-templates; only the served-task list lives here.

@history_item n=12 marker=2026-10-07-settle-detection
**2026-10-07-settle-detection** — settle returns `Settled` for the keeps-animating set and reads animation time only through the harness clock, never advancing it. This is part of decision/animation-driven-by-virtual-clock, which the new app's acting steps rest on.

@history_item n=13 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** — the driver's `advance` moves app time, while `click`, `type` and `press` leave the clock where it was. It is the precursor of the contract/driver-verbs-and-virtual-clock/* rows. The new app's controls, reached by ordinary click/press steps, must not move the clock.

@history_item n=14 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** — the harness drives animation from the documents' `VirtualClock`, and `advance` moves it by exactly its ms together with the app's own time (decision/driver-advance-moves-one-clock, decision/animation-driven-by-virtual-clock). It was PROVISIONAL here.

@history_item n=15 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (Epoch 2 close ratification) — ratified the one-virtual-clock PROVISIONAL mark among others. The motion rows this chunk rests on stand ratified.

@history_item n=16 marker=2026-10-20-load-wait
**2026-10-20-load-wait** — the driver's load wait (at most `LOAD_WAIT`, 5 s) moves no clock. The served load page is the precedent for a stand app whose acting step waits. If the new app does async work, the wait is read the same way.

@history_item n=17 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the chosen-row pair; the accent's use in Cells) — added `blitz-dom default chosen row` `#c8c8c8`/`#000000`, hardcoded since the engine holds no tokens (part of decision/engine-colours-hardcoded). The stand accent row stays a pointer holding no value. A new app reusing the accent adds a use, not a token.

@history_item n=18 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the open select list's colours are engine constants) — the `SelectListView` constants for highlight, disabled and border are hardcoded, not in the default sheet, and add no token. This is the source of decision/engine-colours-hardcoded and the colour-token absence the chunk must not break.

@history_item n=19 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** — `MountedData` calls during a render no longer panic with `RefCell already borrowed`; a call acts on the previous render's document. Trap for an app built to panic in handlers: its panics must be the app's own authored ones, not borrow panics of the render path.

@history_item n=20 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (graph answer line; unpaged graph limit) — `graph` answers one uncoloured JSON line `{"nodes","states"}` on every carrier, carrying names, closed flag words and booleans only (contract/escher-session-commands/graph-answer, contract/ui-graph-line/content). The unpaged-graph limit (`res-7cc5bi4k`) is measured on Cells. The new app's graph must stay within that content set and its handlers must appear as events.

@history_item n=21 marker=2026-11-15T05-21-42-wrap
**2026-11-15T05-21-42-wrap** — ratified the graph answer line and the coverage role's answer line, endings and refusals on the founder's own word. Both rows the chunk touches now stand ratified, not PROVISIONAL.

@history_item n=22 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (actions answer line; unpaged actions limit) — added `actions`: every actionable node in snapshot pre-order with its id-taking verbs and causes, decided by the same `validate` and `check_target` the verbs meet, `stale` for an unresolved node, and content limited to ids, verb words, cause words and booleans. This is the origin of decision/actions-share-the-verbs-checks and the contract/actions-line/* rows. The new app's controls must be listed by it, and `caused` on acting answers must not leak values into the actions line.

@history_item n=23 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** — the `coverage <app> <check>...` role replays kept checks of a served app and adds no exit status. The new app becomes a valid `<app>` for it, and its kept checks must parse as checks of that app. Its stated limits (bubbling counts, canvas) apply.

@history_item n=24 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — turned the master into rows verified against the code. This is where the picked rows (the twin surface and motion areas, the tasks, exit and mcp contracts) got their handles. An amendment this chunk makes names rows by handle (`+`/`~`).

@history_item n=25 marker=2026-11-13-upstream-sync-determinism-and-exploration
**2026-11-13-upstream-sync-determinism-and-exploration** (Typography: 2 rows) — added decision/unavailable-families-use-default-font, ratified as upstream's. The new app's text lays out in the bundled default font when no listed family is available.

@history_item n=26 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** — added contract/escher-session-commands/layout-answer and layout-arguments to Surface: desktop-native, the most recent precedent for adding answer rows there. The new app's screen answers `layout` like the six. The task-table row change (+ the seventh task) follows this entry's row form.
