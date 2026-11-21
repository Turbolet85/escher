@history_item n=1 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — set the paged `snapshot [--part <part>]` answer line: `{"text":…}` whole, `{"text":…,"part":n,"parts":m}` in parts, one uncoloured line, status 0, and over the mcp role a paged result became the JSON line (was the screen text). This is the precedent `graph --part N` is ruled to copy ("paged as `snapshot` is"). It underlies contract/escher-session-commands/snapshot and contract/escher-session-mcp/snapshot-result, so graph's paged line and its MCP result should match this shape.

@history_item n=2 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the cli surface's graph answer line) — added the `graph` item: no argument, an argument refused `malformed` with nothing run; one line `{"nodes":[…],"states":[…]}` on the socket, the command line, MCP (a text block) and in a kept check; the content limited to ids, names, closed flag words and booleans. It is the source of contract/escher-session-commands/graph-arguments, graph-answer and contract/ui-graph-line/content. `graph --part N` must amend graph-arguments, which today refuses every argument. The crossing was a boundary widening, PROVISIONAL at the time.

@history_item n=3 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the unpaged graph, a pending limit) — recorded the unpaged graph as a pending Component Patterns item, the subject `cli-unpaged-graph`: Cells at boot answers 293,869 bytes, the stated limit «An unpaged graph» in docs/capabilities.md, owned by residual `res-7cc5bi4k`. This chunk discharges that residual, so the pending item and the subject retire or change when `--part` lands.

@history_item n=4 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the unpaged `actions` answer, a stated limit) — widened residual `res-7cc5bi4k` to own the unpaged `actions` answer too (Cells at boot 106,088 bytes for 678 nodes), the subject `cli-unpaged-actions` beside the unpaged graph. Trap: the residual this chunk absorbs covers `actions` as well as `graph`. The plan must say whether `actions` gets a `--part` or its limit stays with a new owner.

@history_item n=5 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the `actions` answer line) — added `actions`: no argument, refused `malformed` otherwise, with the same line on every carrier. Its nodes are listed in snapshot pre-order, and a node's verbs are decided by the verbs' own `validate` and `check_target`. It is the source of decision/actions-share-the-verbs-checks and contract/actions-line/*. A new `press` modifier changes the verb table that `actions` lists, and a part reading would follow the same pre-order.

@history_item n=6 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (the coverage role's answer line, endings and refusals) — added the `values · cli-coverage` group: `coverage <app> <check>...` replays kept checks on fresh sessions and answers `{"unexercised":[…],"checks":n,"held":m}`. Its one shape takes no `--session`/`--record`. Check files are read bounded at 1,048,576 + 1 bytes and refused `malformed` without echoing the path. It is the source of contract/escher-session-commands/coverage-* and contract/coverage-*. A fix that reads a stop in the reached record changes what this line lists, but its shape must stay. Kept checks that hold a new `press` modifier or a `graph --part` must parse here.

@history_item n=7 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (the coverage answer's three stated limits) — added the pending group `cli-coverage-limits`: a bubbling dispatch counts every ancestor listener it passed, with `stopPropagation` unread (owner: a residual row, target next — `res-4dr3wt62`, which this chunk absorbs). Its siblings are canvas handlers (owner 0.4.0) and an unreached screen (holds by construction). A fix retires only the third limit in this group and its capabilities.md line, and the other two stay.

@history_item n=8 marker=2026-11-15T05-21-42-wrap
**2026-11-15T05-21-42-wrap** (ratified — the graph answer line; ratified — the coverage role's answer line) — the founder ratified the PROVISIONAL graph and coverage entries, and their text stands. The graph-arguments and coverage-arguments rows that this chunk would change are now founder-ratified, so a change needs his word or the ruled `graph --part N` form.

@history_item n=9 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** — added contract/escher-session-commands/layout-answer and layout-arguments as rows. It is the nearest precedent for adding an argument contract to a command on every carrier, "beside those of actions and graph", with a missing id refused `malformed`, not a usage ending. `graph --part` and a `press` modifier should follow the same row form and refusal class.

@history_item n=10 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** — set the escher-session output form: one JSON line per command keyed by the driver schema's words, usage on stderr with stdout empty, no colour, no ESC byte, no terminal test, and exit 0/1/2/3. It is the source of decision/escher-session-agent-read-surface, decision/escher-session-no-colour, decision/driver-output-for-agents and the exit-status/stdout/stderr rows. A new argument shape (a modifier, `--part`) must keep the four exit statuses and send a bad shape to usage (2) or `malformed` (1) by the existing classes.

@history_item n=11 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — added the mcp role: stdout holds JSON-RPC 2.0 lines only, one uncoloured reply per request; a tool result's text is the screen text for `snapshot` and the JSON line otherwise; an Err ending goes to stderr with status 3. It is the source of contract/escher-session-mcp/*. New tool parameters (a `press` modifier, a graph `part`) reach this carrier, and its tool listing changes with them.

@history_item n=12 marker=2026-10-12-self-description
**2026-10-12-self-description** — exit statuses are read from the schema's `EXIT_STATUSES` table that `help` serves. `help` is one line, status 0, reaching no session, and a session error carries a fixed `remedy`. It is the source of decision/exit-statuses-from-schema-table and contract/escher-session-commands/help and mcp/help-result. `help` states today's `press` no-Ctrl limit and graph's argument set, so the self-description must change with either fix.

@history_item n=13 marker=2026-10-19-session-recording
**2026-10-19-session-recording** — named the recording's `record-dir-not-private` and `recording-failed` in the exit-3 class with no new exit (contract/escher-session-exit-status/3). This shows a new error enters an existing class rather than a new status. Kept checks are recorded shapes that a new `press` modifier or `graph --part` must round-trip through.

@history_item n=14 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — added `screenshot [--id]` with its answer line `{"width","height","png"}` and a two-block MCP result. It is a precedent of an optional argument on a ratified verb carried on every carrier while keeping the surface's one-line, no-ESC form.

@history_item n=15 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** — the driver's `advance` moves app time, and `click`, `type` and `press` leave the harness animation clock where it was. This is the origin of contract/driver-verbs-and-virtual-clock/press: a `press` with a Ctrl/Meta/Alt modifier must still leave the clock alone.

@history_item n=16 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** — the harness drives animation from the documents' `VirtualClock`. `advance` moves it by exactly its ms and the app's time together, and `click`, `type` and `press` leave it unchanged. It is the source of decision/animation-driven-by-virtual-clock and decision/driver-advance-moves-one-clock, ratified by 2026-10-18-engine-debts (amd-mgqkxn5f). A widened `press` stays a no-clock verb.

@history_item n=17 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (the PROVISIONAL batch ratified by the founder, Epoch 2 close) — ratified the one-virtual-clock clause that `advance` moves the clock and the app's time together, and with it the press-leaves-the-clock reading that the motion rows rest on.

@history_item n=18 marker=2026-10-07-settle-detection
**2026-10-07-settle-detection** — settle reads animation time only through the harness clock and never advances it. This is the root of decision/animation-driven-by-virtual-clock, which any `press` change that settles after the key must keep.

@history_item n=19 marker=2026-10-20-load-wait
**2026-10-20-load-wait** — the driver's load wait moves no clock: while an acting step waits on a load (at most `LOAD_WAIT`), the `VirtualClock` stands still. This is the source of decision/load-wait-moves-no-clock, which binds a `press` (an acting step) with a new modifier.

@history_item n=20 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (a `multiple` select's chosen-row pair; the default sheet's `multiple` rows and a select's groups) — built the `multiple` select: its chosen-row pair `#c8c8c8`/`#000000` and its rows' layout and padding in the default sheet. This is the chunk where `res-hfvjj77l` (no Ctrl on `press`, so a non-adjacent `multiple` set cannot be built) was kept as a residual on the founder's word. A Ctrl `press` reopens that select's selection path. These rows' visuals stay as they are.

@history_item n=21 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (how a single select's list opens, moves, chooses and closes) — a disabled, inert or hidden select, one with no row to show, and a `multiple` select open no list. This is the source of decision/selects-that-open-no-list. A Ctrl-modified `press` on a `multiple` select must still open nothing, so its pin `a_disabled_inert_hidden_singular_or_empty_select_opens_nothing` stays green.

@history_item n=22 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the open select list answers the hit before the page's walk) — the open list answers first in `hit_with_scrollbar`, so `hit`, `set_hover_to`, `handle_pointerdown` and the driver's `covered` all read it, and a button under the list is refused `covered`. This is the source of decision/open-select-list-hit-first. It bears on the dispatch and reached-record path (`stopPropagation`) and on any driver verb change that uses the covered check.

@history_item n=23 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (a single select's list is painted, an engine overlay) — `size` is read on a `multiple` select and unread on a single select. With the `multiple` select's `size` rows, this frames the select that the Ctrl `press` fork serves.

@history_item n=24 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (citation ranges re-read after the graph chunk's code) — re-pointed the `escher-session` item's citations: `EXIT_STATUSES` schema.rs:503-541 → :557-595, json.rs:317-373 → :400-462. Trap: this chunk edits schema.rs and json.rs ahead of those ranges, so the cited lines will move again and need re-pointing.

@history_item n=25 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (citations re-pointed by the chunk's line moves) — moved surface.md's `values · cli-output` citations into the driver's `schema.rs` by line shifts. This precedent shows that edits to the verb table move the cli surface's cited ranges.

@history_item n=26 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — the master became rows (137 rows: decision, contract, absence, overview), each verified against the code. Every picked row here dates from this change. Amendments are now `+`/`~`/`−` on row handles, so this chunk's expected amendments (graph-arguments, graph-answer, the mcp snapshot-style result, a `press` argument, the coverage limit) are row edits.

@history_item n=27 marker=2026-11-18-served-error-page
**2026-11-18-served-error-page** (Surface: desktop-native, 1 of 2 and 2 of 2) — moved the served-task rows from six to seven and added contract/escher-session-tasks/order. The usage line's `apps:` and the MCP `start` enum list the seven. Any usage or MCP tool-listing change this chunk makes must keep the seven tasks.

@history_item n=28 marker=2026-11-20-select-list-and-selection-debts
**2026-11-20-select-list-and-selection-debts** — the last debts chunk on the select. It changed contract/select-accessibility/highlighted-row (the closed bool `highlighted` in the snapshot), open-select (active descendant) and select-list-events/highlight-move (a move to another row marks its select changed). These are the select-side rows that a Ctrl `press` on selects rests beside, and the precedent for a debt fixed by `~` row edits on the operator's relay of the founder's word.
